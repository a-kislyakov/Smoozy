//! **The render settings — the larger of the two panels over a server.**
//!
//! The page half is `web/render-settings.js`, and it draws inside piece 5's
//! panel mechanism: **the contract at the head of `web/a-server.js` is what
//! this piece obeys**, and nothing here builds a panel of its own.
//!
//! **This file is the surface and not the deciding.** Every judgement on this
//! page is `App/Render Settings/`'s and `App/Rendering/`'s, both of which are
//! whole below the page: the two objects that are never merged (J1), the three
//! refusals (J11) and what each states (J12), the frame parse and its tidy
//! (F1), the per-frame view-layer answer and the diamond (`Render Settings`
//! V11, `Render Settings` V12), the
//! samples imposition set (C1), the version pick and the mismatch material
//! (B2a, B8), the device warning across both of `Servers`' axes (D6), the
//! payload (Y1) and the missing-assets warning's cases (`Rendering` W2).
//! **Nothing here re-decides any of them.**
//!
//! ## What this piece draws, cited the way the runner ruled on 2026-09-12
//!
//! **The Design document and the heading**, and **the mockup state names**.
//! `Preparation/Design/Render Settings.md` is 314 lines and carries no clause
//! marker, so the headings are the citation:
//!
//! - `Design/Render Settings § Where it sits` — a panel over the server,
//!   opened by choosing a `.blend`, collapsing to a strip.
//! - `§ The header carries the Blender version`, `§ When there is no version to
//!   pick`, `§ The version mismatch warning`, `§ The one thing this page says
//!   about add-ons` — the header entire.
//! - `§ The body is three columns`, `§ The file's value against the user's`,
//!   `§ Before the file has been read`, `§ Reading the file`, `§ When the read
//!   gives nothing`.
//! - `§ Frames`, `§ View Layers`, `§ The sample count`, `§ The picture size`,
//!   `§ The render engine`, `§ What the render runs on`, `§ Warnings`,
//!   `§ Scenes`.
//! - `§ What it will write`, `§ The dependency warning`, `§ Submitting`.
//! - `Design/Whole app § Panels over a server` for the frame, which is piece
//!   5's and is not re-decided here.
//!
//! **The mockup states this piece draws** are [`THE_STATES_THIS_PIECE_DRAWS`],
//! read out of `Preparation/Design/Mockup/Smoozy.html`'s own state list by a
//! test below, in both directions. The account is
//! `Audits/Build/Step 10a/Piece 8 — what landed.md`.
//!
//! ## What can actually happen today, said plainly
//!
//! - **Everything the user answers on this page runs whole**: the frame field,
//!   the samples box, the percentage box, the scene ticks, the view-layer pins,
//!   the engine pick, the device choice, the version pick, every cross, the
//!   hand-everything-back control, the job's name and the three submission
//!   choices. All of them are store writes and a recomposition, and the store
//!   is process memory for the life of the run (J2), so nothing durable moves.
//! - **Reading the file happens, since step 10c's dependency wave piece 3.**
//!   The press drives `Render Settings`' own Pre-load over the server's own
//!   point through [`crate::reading_the_file`]: one request to the machine's
//!   supervisor, a file-inspection worker there, and the document read back
//!   through the worker's captured stream. **The outputs half is kept only
//!   where the file has one scene** (`Design/Render Settings § What it will
//!   write`), because a reading composed with no scene describes the scene the
//!   file was saved showing and does not say which (that file's head says
//!   why). *(Until that piece the act answered
//!   [`crate::surface::files::A_CONNECTION_IS_NOT_BUILT`] and recorded no
//!   ending, because no read was attempted.)* The four endings land through
//!   [`the_read_gave_nothing`], which hands each to the diary once — and a
//!   fifth beside them, where the server answered and ran no Blender at all,
//!   which says *"Could not read the file on the server."*
//!   **A file read on this page is submitted like any other**, and its
//!   manifest says no reading (`Job files and outputs` R7's second bullet),
//!   the render worker taking its own at the render's start. *(Until
//!   2026-09-29 it was refused at Submit, on the ground that nothing could take
//!   a reading; step 10c's four small things after *Read the file* took the
//!   refusal away on the runner's ruling.)*
//! - **Submitting happens, since step 10c piece 6.** The Submit control
//!   composes one payload per scene-job and hands the act to
//!   [`crate::submitting`], which runs `Rendering`'s whole act over the
//!   server's own point: the gate, the identities, the manifest, the
//!   provisional record with its kept setup (record version 25), the remap
//!   material placed, the material published by one rename, and the job
//!   record asked of the supervisor. **The machine's program does not answer
//!   that last ask** — step 10c piece 5 builds it — so today every submission
//!   ends with the page saying the program on the server does not accept
//!   jobs, the provisional record gone, and `Rendering` S4b's debt written for
//!   the material it published. **That is the truth of what happened and it is
//!   what the page says.**
//! - **The dependency band reads the check standing from the local record**,
//!   since step 10c's dependency wave piece 7 — [`the_check_the_band_reads`]
//!   — and not from the run's memory, so a check survives a restart and a mark
//!   written by a later act brings the warning back. **A standing does not
//!   stop the warning where Smoozy could not keep the stop true** — see
//!   [`TheCheckTheBandReads`], and piece 7b, which made it keepable for a
//!   project uploaded through Files and after a restart.
//! - **The Blender version list is read from the machine**, when this page is
//!   opened, through `Server Setup`'s own search (`Render Settings` B1,
//!   `Server Setup` B12), and held for the connected period only.
//! - **Two controls are drawn and dead** — see the section below.
//!
//! ## `What Build inherits` item 8(a), which is this page's
//!
//! **Five settings spellings have no read source** (`Render Settings` §13 item
//! 29): the property names C1 items 2, 3 and 5 assign against, the property C1
//! item 4's rule lives at, and D3's scene device setting. `App/Render
//! Settings/src/spellings.rs` holds each as an explicit absence a type can tell
//! from a value, so **no composition can turn one into a pair**.
//!
//! **What that costs on this page, exactly:**
//!
//! - **The samples box takes a typed count and the page draws it**, and the
//!   count rides the payload as one entry. **Every attempt of that job fails**
//!   at `Rendering` L8a's named failure status with nothing written, because
//!   [`smoozy_render_settings::samples::the_imposition_set`] cannot compose.
//! - **The CPU option takes the user's choice against a scene set to the
//!   cards**, and the page draws it. **Every attempt of that job fails** the
//!   same way, because
//!   [`smoozy_render_settings::device::the_processor_pair`] cannot compose.
//!
//! **Nothing here stubs a spelling, invents a rescue, or refuses at the
//! desktop.** The entry rules the loud, total failure the safe direction — a
//! guessed property name is handed in, reads back correctly and changes
//! nothing, which is the silent-wrong-render class this whole step exists to
//! avoid — and a desktop-side refusal would be this piece re-deciding a plan
//! that says where the failure lands. What is added is
//! [`tests::the_two_dead_controls_are_dead_and_this_is_the_line_that_says_when_they_are_not`],
//! which fails the day the lookup lands, so the controls coming alive is
//! something the build says rather than something somebody remembers.
//!
//! ## How a clause marker is written in this crate
//!
//! A marker with no document beside it means a clause of this crate's own plan,
//! `Preparation/Architecture/Desktop app and delivery.md`. **Every marker of
//! another document is written with that document** — at the site, or in the
//! list below. This page's own two plans collide with this crate's on several
//! letters, so `Rendering`'s and `Render Settings`' are written out at the site
//! wherever the letter is one both define.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Render Settings` B1, B2a, B8, B9, D3, D5,
//! D6, J1, J2, J3, J4, J5, J7, J10, J11, J12. `Job files and
//! outputs` I16a, I22, N8. `Server Setup` B11.

use std::collections::BTreeMap;
use std::sync::{Mutex, OnceLock};

use tauri::{Manager, State};

use smoozy_dependencies::standing::{Counts, WhatTheRecordSays};
use smoozy_job_files::holds::the_six_things;
use smoozy_job_files::inventory::{compose_the_inventory, WhatTheInventoryIs};
use smoozy_job_files::layout::{Area, TheLayout};
use smoozy_job_files::mapping::{compose_the_map, ADestinationToMap, TheMap};
use smoozy_job_files::reading::{
    the_per_frame_set_with_the_jobs_pins, AViewLayerTheJobPins, ArtistsText, Destination, TheOutputsHalf,
    WhereItComesFrom,
};
use smoozy_render_settings::device::{self, DeviceChoice, Presence, Tense, Warning};
use smoozy_render_settings::endings::Ending;
use smoozy_render_settings::engine::{OfferedEngine, WhatThePageShows};
use smoozy_render_settings::file_half::{version_text, BlenderIdentity, FileHalf, FileKey};
use smoozy_render_settings::frames::{self, FrameForm};
use smoozy_render_settings::names::{ExactName, NamedAnswer, Provenance};
use smoozy_render_settings::reading::{self, FrameAnswer, LayerSamplesRule, SceneReading};
use smoozy_render_settings::readings::HeldHardware;
use smoozy_render_settings::refusals::{self, Refusal, Statement};
use smoozy_render_settings::reporting::{self, ReportingChoice};
use smoozy_render_settings::samples;
use smoozy_render_settings::store::{self, Answer, PageInHand, SceneEntries, Setting, Store, Where};
use smoozy_render_settings::version::{
    self, HowItMatched, Resolution, TheListThisPageAsksFor, WhyNone,
};
use smoozy_render_settings::view_layers::{self, Diamond};
use smoozy_rendering::warning::{TheWarningsCase, THE_SUPPRESSION_IDENTITY, THE_SUPPRESSION_ROW};
use smoozy_state::identities::{Identity, IdentityKind};

use crate::events::{Event, Said, Subject, TheOneQueue};
use crate::start::Started;
use crate::surface::servers::Words;
use crate::surface::transfers::{
    the_files_the_choice_brings, the_folders_offered, the_format_box_from, AFormatKnown,
    TheFormatAnswer, TheFormatReading, WhatTheBoxKnows,
};
use crate::typography::{Prose, Verbatim};

// ════════════════════════════════════════════════════════════════════════════
// The names this piece and its page agree on
// ════════════════════════════════════════════════════════════════════════════

/// **The panel key, and it is the drawing's own.** The same string
/// `src/surface/a_server.rs` holds for the render settings panel; importing it
/// rather than typing it is what that piece's contract asks for.
pub const RENDER_SETTINGS: &str = crate::surface::a_server::RENDER_SETTINGS;

/// **The kind, declared beside the service that posts it.**
/// `web/render-settings.js` names the same string.
pub const AN_ACT_ON_THIS_PAGE: &str = "render-settings:an-act";

/// Requests can be served on this connection. The open panel asks for its
/// list even while collapsed; the earlier Connected reading is too soon.
pub const THE_CONNECTION_IS_LENT: &str = "render-settings:the-connection-is-lent";

/// **How many of an answer's leading parts say what went out**, rather than
/// what was refused — `Design values`: `--alert` is *"Refused or blocked"*
/// and nothing else, so *Submitted.* and *2 of 3 jobs submitted.* are drawn as
/// a plain note and only what follows them keeps the refusal colour. Carried
/// on the submission's answer alone; an answer without it is all refusal, as
/// every other answer on this page is. `web/render-settings.js` names the same
/// string. Added 2026-09-28, step 10c's tenth-pass fix piece 6: the tenth pass
/// saw *Submitted.* in the refusal colour.
pub const THE_PLAIN_PARTS: &str = "plainParts";

/// **A submission that went: the job the queue opens on** — the first job the
/// act sent, by its identity — carried on the submission's answer only where
/// every job went. **Its presence is what makes the answer the confirmation**
/// (`Design/Render Settings § Submitting`, added 2026-10-03, the Creator's
/// note 10 on the test copy): *Submitted to Vast 4090.* with *Show in Queue*
/// beside it. `web/render-settings.js` names the same two strings.
pub const THE_JOB_TO_SHOW: &str = "jobToShow";
/// The words of *Show in Queue*, beside the job it opens.
pub const SHOW_IN_QUEUE_SAID: &str = "showInQueue";

/// **The box `Render Settings` B1's list answers on when it comes back** — not the version
/// pick's, which is the user's act. The page keeps the confirmation of a
/// submission until something on the page changes, and the list arriving is
/// the one act on this page that is neither the user's nor the submission's.
pub const THE_LIST_CAME_BACK: &str = "version-list";

/// **The mockup states this piece draws**, exactly as
/// `Preparation/Design/Mockup/Smoozy.html`'s own `<select id="state">` names
/// them — its *Render settings* group entire. **The runner ruled on 2026-09-12
/// that the mockup state names are what this half is counted by**, and
/// [`tests::the_states_are_the_drawings_own`] reads the drawing and fails in
/// both directions. **The count is the array's own and is written nowhere
/// else**: a number in a name or in a sentence is the one fault this stage
/// keeps meeting.
pub const THE_STATES_THIS_PIECE_DRAWS: [&str; 55] = [
    // **`res-oneframe`: every job the Render control would send has one
    // frame**, so *Render one frame first* is greyed and cannot be pressed but
    // keeps its setting, and the dependency band leaves its first-frame way
    // out. Drawn and built 2026-10-27, the Creator's nineteenth notes, note 8
    // (`Design/Render Settings § Submitting`); held by
    // `the_first_frame_switch_is_greyed_where_every_job_has_one_frame`.
    "res-oneframe",
    // **`res-againlist`: the page opened by *Submit again* with the frames as
    // the entry holds them, typed.** Drawn 2026-10-24 by the eighteenth notes'
    // design pass, ruling 6 (note 8). **Not built yet**: it is owed to that
    // work's piece 7, which keeps the typed text in the job record and has
    // *Submit again* hand it to this page (`Render Settings` F1).
    "res-againlist",
    // **`§ How the three sit together`: the foot along the bottom, in two
    // lines**, every group aligned to the top. Drawn 2026-10-17 by the drawing
    // pass for the Creator's tenth notes, note 8, first in the drawing's
    // *Render settings* group; named here by their piece T1, which held this
    // file. The foot itself is their piece T8's, after the Creator's word;
    // built 2026-10-17 (`theFoot` in `web/render-settings.js`), and measured
    // against the drawing by `the_foot_is_two_lines_aligned_to_the_top_as_drawn`.
    "res-foot",
    "res-footdl",
    // The narrow form, drawn 2026-10-29 by the twenty-first notes' design
    // pass (note 4).
    "res-footdlnarrow",
    "res-tgoff",
    "resbad",
    "res-again",
    "res-nosrv",
    "res-simpon",
    "res-carry",
    "res-carryno",
    "res-find",
    "res-noread",
    "res-reading",
    "res-lost",
    "res-smp",
    "res-range",
    "res-eng",
    "res-gpuseen",
    "res-gpunocount",
    "res-multi",
    // **`§ Submitting`: the switch on and no folder anybody can name.** Drawn
    // 2026-09-20, and the one state on this page that had never been drawn at
    // all — the silence a job's frames leave when they have nowhere to come
    // home to.
    "res-nofolder",
    // **`§ Submitting`: a submission that went says so beside Render**, with
    // *Show in Queue*. Drawn 2026-10-03 by the drawing pass for the Creator's
    // notes on the test copy, their note 10, and built the same day by their
    // piece 3: [`a_submission_that_went`] and the page's
    // `theSubmissionThatWentIsDrawn`.
    "res-sent",
    // **`§ What it will write`: the line above the list says where the job's
    // folder falls back, and why.** Drawn 2026-10-04 by the drawing pass for
    // the Creator's second notes on the test copy, on their ruling on note 18.
    // Built by that map's piece 2 (the words) and piece 4 (the choice:
    // [`where_the_jobs_folder_sits`]).
    "res-fallback",
    // **`§ What it will write`: a second line where the server's fallback
    // folder is ruled out before submitting.** Drawn 2026-10-04 by the drawing
    // pass for the chosen fallback folder, shorter than the queue entry's line
    // because the line above it already names Smoozy's folder. Built by the
    // Creator's second notes, piece 4: [`the_second_folder_line`].
    "res-fallchosen",
    // **`Design/Whole app`: Smoozy draws its own dropdowns.** The version
    // list open, drawn as the right-click menu is, the picked build in bold.
    // Drawn 2026-10-08 by the drawing pass for the Creator's fifth notes, note
    // 2; built by that map's piece U.
    "res-verlist",
    // **`§ Submitting`: *A fourth choice, per job only: a picture every few
    // minutes*.** Telegram's list open upward with the new row third; the
    // choice picked, the control reading *A picture every* with the minutes
    // field and *minutes* after it; and minutes Smoozy will not take, the field
    // red and *Whole minutes, 1 to 60.* on the third line. Drawn 2026-10-19 by
    // the plan pass and the drawing pass for the Creator's want of that day;
    // built by piece N (`theTelegramLine` in `web/render-settings.js`).
    "res-tglist",
    "res-tgmin",
    "res-tgbad",
    // **`Design/Whole app`: the other panel waits as its strip under the open
    // one.** Render Settings open, ending above the dependency check's strip.
    // Drawn 2026-10-08 by the drawing pass for the Creator's fifth notes, note
    // 3; built by that map's piece V.
    "res-chkstrip",
    // **`Design/Whole app`: a panel never opened over that server still has
    // its strip while the other is open, its name alone.** Render Settings
    // open over a check never opened, and the check open over Render Settings
    // never opened. Drawn 2026-10-10 by the drawing pass for the Creator's
    // seventh notes, note 2; built by that map's piece Z6.
    "res-chkfresh",
    "chkfresh-set",
    "dep-never",
    "dep-unfound",
    "dep-ver",
    "dep-first",
    // **`§ Submitting`: the check's uploads still going**, counted above
    // Render. Drawn 2026-10-02, the Creator's note 6 on 0.1.8.
    "dep-sending",
    "ow-comp",
    "ow-comp2",
    "ow-vid",
    // **`§ What it will write`: a video and frames from a File Output node**,
    // named as two outputs with no twice line. Drawn 2026-10-12, the Creator's
    // eighth notes, note 4.
    "ow-vidfr",
    "ow-none",
    "ow-collide",
    "rf-srv",
    "rf-ver",
    "rf-part",
    "rf-stop",
    // **The one case beside `Render Settings` Q8's four**: the server answered
    // and ran no Blender. Drawn 2026-09-29, step 10c's four small things after
    // *Read the file*.
    "rf-norun",
    "bl-old",
    "bl-none",
    "bl-marks",
    "bl-below",
    "bl-mism",
    "bl-addon",
];

/// **One of them is drawn on another piece's page and this is where that is
/// said.** `res-find` is *submitting a job again on a server that is gone*, and
/// the mockup draws it as `#findbar` inside the file browser at line 6924 —
/// because what the user has to do is point at the `.blend` again, and pointing
/// at a file is `File Browser`'s. **This piece owns the state name and piece 6
/// owns the bar**, so it is recorded as owed rather than drawn here; the
/// account names what piece 6 needs.
pub const THE_ONE_STATE_DRAWN_ON_ANOTHER_PIECES_PAGE: &str = "res-find";

// ════════════════════════════════════════════════════════════════════════════
// Every word the user reads
// ════════════════════════════════════════════════════════════════════════════

/// **A figure the entry recorded when the job ended, rather than one read from
/// the server just now** — `Rendering` A9: *"a remembered figure is never
/// presented as a checked one."*
///
/// **This is the one home of that phrase and History takes it from here** —
/// step 10a, the screens round's loose ends, L6, carrying out must-do 2(c) of
/// that round's Verdict for the queue, history, settings and downloads: *"Keep
/// that sentence in one place and use it from both files."* ~~It stood in
/// `the_history.rs` as a second copy with a test to catch the two drifting~~,
/// because this file's words were private to it. It is a macro rather than a
/// constant so that each caller can build its own sentence around the phrase at
/// compile time.
macro_rules! the_recorded_figure {
    () => {
        "the frame recorded when the job ended"
    };
}
pub(crate) use the_recorded_figure;

pub(crate) mod words {
    // The header. `Design/Render Settings § The header carries the Blender
    // version`.
    pub const RENDER_SETTINGS: &str = "Render settings";
    /// The panel's title as the drawing writes it — *Render settings —
    /// kitchen_v14.blend* — built the way the check's title is built: the words
    /// with the dash, then the file's name as a value. Must-do 17.
    pub const RENDER_SETTINGS_TITLE: &str = "Render settings —";
    /// **The hover on the file's name in either panel's title**, which is a
    /// link into Files — `Design/Whole app § Panels over a server`, added
    /// 2026-10-12 (the Creator's ninth notes, note 5). The dependency check's
    /// title carries the same words, from here.
    pub const SHOW_IN_FILES: &str = "Show in Files";
    pub const BLENDER: &str = "Blender";
    pub const BLENDER_VERSION: &str = "Blender version";
    pub const READ_THE_FILE: &str = "Read the file";
    pub const READ_AGAIN: &str = "Read again";
    pub const READING_THE_FILE: &str = "Reading the file";
    pub const READ_AT: &str = "read at";
    pub const RESET_ALL: &str = "Reset all to file values";
    pub const BELOW_THE_FLOOR: &str = "— below 4.5";

    // `§ The header carries the Blender version` — the pick line's three, which
    // are the drawing's own words.
    pub const MATCHED_TO_THE_FILE: &str = "matched to the file";
    pub const NEAREST_ABOVE: &str = "nearest above the file’s";
    pub const NEAREST_BELOW: &str = "nearest below — nothing above it on this server";

    // `§ When there is no version to pick` — the three cases.
    pub const SMOOZY_DOES_NOT_CHOOSE_BELOW_THE_FLOOR: &str =
        "Versions below 4.5 are never picked automatically. Pick one.";
    pub const NOTHING_TO_MATCH_ON: &str =
        "The file does not record its Blender version. Pick one.";
    pub const NO_BLENDER_ON_THIS_SERVER: &str = "This server has no Blender.";
    pub const SET_IT_UP: &str = "Set it up";
    pub const NO_MACHINE_IN_HAND: &str =
        "Connect a server to list its Blender versions.";

    // `Server Setup` B11: of the search's limits, only a build that did not
    // answer is drawn. **The folders it could not read and the search going by
    // name are kept in its answer and the diary and not drawn** — Creator-stated
    // 2026-10-02, note 13 on 0.1.7: *"All this should just go — it's
    // unnecessary noise."*
    pub const DID_NOT_ANSWER: &str = "did not report its version.";

    // `§ The version mismatch warning`.
    pub const SAVED_WITH: &str = "Saved with";
    pub const RENDERING_WITH: &str = "rendering with";
    // **The line says something may not work as expected, and opens nothing**
    // — `Render Settings` B8 as rewritten 2026-10-02, the Creator's note 12 on
    // 0.1.7. *What changes* and its panel of break notes are gone.
    pub const SOME_THINGS_MAY_NOT_WORK: &str = "Some things may not work as expected.";

    // `§ The one thing this page says about add-ons`.
    pub const THE_ADD_ON_IS_MISSING_LEAD: &str = "lacks the add-on for this file’s engine.";
    pub const THE_ENGINE_READ_BACK_AS_EEVEE: &str =
        "The engine shown above is EEVEE, not the file’s.";

    // `§ Before the file has been read` — the band, said once at the top.
    pub const NOT_READ_LEAD: &str = "File not read.";
    pub const NOT_READ_REST: &str = "Values are the file’s own until changed. Scenes and View Layers appear after a read. You can still submit.";
    pub const THE_CONNECTION_ENDED_LEAD: &str =
        "Connection lost.";
    pub const THE_CONNECTION_ENDED_REST: &str = "Your changes are kept. Scenes and View Layers appear after a read. You can still submit.";

    // `§ Scenes`, *More than one ticked* — said once, in the same band.
    pub const SCENES_ARE_TICKED_LEAD: &str = "scenes ticked:";
    pub const SCENES_ARE_TICKED_REST: &str =
        "jobs. Each uses its own settings; only the frame range is shared.";

    // `§ When the read gives nothing` — Q8's four endings, each saying
    // something different, and only the second carrying a second sentence.
    pub const COULD_NOT_REACH_THE_SERVER: &str = "Could not reach the server.";
    pub const THE_SERVER: &str = "Server";
    pub const COULD_NOT_OPEN_THIS_FILE: &str = "This Blender could not open this file.";
    pub const PICK_A_DIFFERENT_BLENDER: &str = "Pick another Blender and read again.";
    pub const THE_READ_DID_NOT_FINISH: &str = "Reading the file did not finish.";
    pub const SMOOZY_STOPPED_THE_READ: &str = "Read stopped.";
    pub const BLENDER_OUTPUT: &str = "Blender output";
    pub const DISMISS: &str = "Dismiss";

    // The three columns.
    pub const SCENE: &str = "Scene";
    pub const ALL_SCENES: &str = "All scenes";
    pub const JOBS_ONE_EACH: &str = "jobs, one each";
    pub const FRAMES: &str = "Frames";
    pub const THE_FILES_RANGE: &str = "— the file’s";
    pub const ONE_RANGE_FOR_ALL: &str = "— one range for all";
    pub const RANGE: &str = "Range";
    pub const HOW_TO_TYPE_A_RANGE: &str = "How to type a range";
    pub const EVERY_FRAME: &str = "every frame";
    pub const EVERY_SECOND: &str = "every 2nd";
    pub const JUST_THOSE: &str = "just those";
    pub const MIXED: &str = "mixed";
    pub const FRAMES_WORD: &str = "frames";
    pub const FRAME_WORD: &str = "frame";
    pub const FRAMES_ACROSS: &str = "frames across";
    pub const SCENES_WORD: &str = "scenes";
    pub const VIEW_LAYERS: &str = "View Layers";
    pub const READ_AT_FRAME: &str = "— read at frame";
    pub const ANIMATED_KEYED_ON_FRAME: &str = "Animated — keyed on frame";
    pub const ANIMATED_NOT_KEYED_ON_FRAME: &str = "Animated — not keyed on frame";
    pub const SAMPLES_OF_ITS_OWN: &str = "samples of its own";
    pub const SAMPLES_OVERRIDDEN: &str = "samples, overridden";
    pub const NOT_KNOWN_WITHOUT_A_READ: &str = "unknown until read";
    pub const BACK_TO_THE_FILES_VALUE_AT_FRAME: &str = "Back to the file’s value at frame";
    pub const BACK_TO_THE_FILES_VALUE: &str = "Back to the file’s value:";

    // `§ The sample count`, `§ The picture size`, `§ The render engine`,
    // `§ What the render runs on`.
    pub const RENDER: &str = "Render";
    pub const SAMPLES: &str = "Samples";
    pub const STOPS_EARLY_LEAD: &str = "This file stops early at noise threshold";
    pub const STOPS_AFTER_A_TIME_LIMIT_OF: &str = "This file stops after a time limit of";
    pub const AND_AFTER_A_TIME_LIMIT_OF: &str = "and after a time limit of";
    pub const ON_EACH_FRAME: &str = "on each frame.";
    pub const OVERRIDES_LEAD: &str = "Overrides noise threshold";
    pub const OVERRIDES_A_TIME_LIMIT_OF: &str = "Overrides the time limit of";
    pub const A_TIME_LIMIT_OF: &str = "the time limit of";
    pub const AND_THE_TIME_LIMIT_OF: &str = "and the time limit of";
    pub const OVERRIDES_THE_OWN_COUNTS_OF: &str = "Overrides the sample counts of";
    pub const AND_THE_OWN_COUNTS_OF: &str = "and the sample counts of";
    pub const THE_COMMA: &str = ",";
    pub const EVERY_FRAME_NOW_RENDERS_ALL: &str =
        "Every frame renders all samples, so the job may take longer and cost more.";
    pub const RENDER_AT_THEIR_OWN_COUNT: &str =
        "keep their own sample counts.";
    pub const RESOLUTION: &str = "Resolution";
    pub const SIZE_NOT_KNOWN: &str = "size unknown";
    pub const ENGINE: &str = "Engine";
    pub const THE_FILES: &str = "— the file’s";
    pub const THE_FILES_NOT_READ_YET: &str = "the file’s — not read yet";
    pub const CYCLES: &str = "Cycles";
    pub const EEVEE: &str = "EEVEE";
    pub const LEAVE_IT_UNCHANGED_READ: &str =
        "If unchanged, the render uses the file’s engine.";
    pub const LEAVE_IT_UNCHANGED_UNREAD: &str =
        "If unchanged, the render uses whatever engine the file names.";
    pub const BACK_TO_THE_FILES_ENGINE: &str = "Back to the file’s engine:";
    pub const RENDERS_ON: &str = "Renders on";
    pub const GPU: &str = "GPU";
    pub const CPU: &str = "CPU";
    pub const GPUS_UNUSED: &str = "GPUs unused";
    pub const GPU_UNUSED: &str = "GPU unused";
    pub const LAST_SEEN: &str = "last seen";

    // `§ Warnings` — the refusal line's second half is J12's, per setting.
    pub const CORRECTED_SILENTLY: &str =
        "Blender would silently use the nearest allowed value.";
    pub const REFUSED_SO_THE_JOB_FAILS: &str =
        "Blender rejects this value; the job would fail.";
    pub const RENDERS_NOTHING_AND_REPORTS_SUCCESS: &str =
        "Blender would render nothing and still report success.";
    pub const A_STEP_OF_ZERO: &str = "A step of 0 renders the first frame once.";
    pub const AN_END_BELOW_THE_START: &str =
        "An end below the start renders nothing, and Blender still reports success.";
    /// **Where the file's regular output is a video, a list with gaps** —
    /// `Design/Render Settings`, *Where the file writes a video, the range
    /// takes one unbroken run*, word for word. Added 2026-10-17, the eleventh
    /// notes, the video-range piece.
    pub const A_VIDEO_CANT_SKIP_FRAMES: &str = "A video can’t skip frames. Type one unbroken range.";
    pub const TO: &str = "to";

    // `§ What it will write`.
    pub const OUTPUT_FILES: &str = "Output files";
    /// **The line above the list**, the Design's own words (`§ What it will
    /// write`, added 2026-10-04 on the Creator's note 18): *In
    /// /home/ubuntu/Render, in a folder for this job.*, or one of the two
    /// fallbacks. The Creator's second notes, piece 2.
    pub const THE_JOBS_FOLDER_IN: &str = "In";
    pub const IN_A_FOLDER_FOR_THIS_JOB: &str = ", in a folder for this job.";
    pub const THE_FOLDER_IS_ANOTHER_COMPUTERS: &str =
        "Your file’s output folder is on another computer, so the files go to Smoozy’s folder on the server.";
    pub const SMOOZY_CANT_USE_THE_FOLDER: &str =
        "Your file’s output folder can’t be used, so the files go to Smoozy’s folder on the server.";
    /// **Where the server has a fallback folder set and nothing known before
    /// submitting rules it out, either sentence ends with that folder's path**
    /// (`Design/Render Settings § What it will write`, the chosen fallback
    /// folder, 2026-10-04): *…so the files go to /mnt/renders.* The folder is
    /// a value and the full stop follows it.
    pub const THE_FOLDER_IS_ANOTHER_COMPUTERS_SO_THE_FILES_GO_TO: &str =
        "Your file’s output folder is on another computer, so the files go to";
    pub const SMOOZY_CANT_USE_THE_FOLDER_SO_THE_FILES_GO_TO: &str =
        "Your file’s output folder can’t be used, so the files go to";
    pub const THE_FOLDERS_FULL_STOP: &str = ".";
    /// **The second line, where the server's fallback folder is ruled out
    /// before submitting** — the drawing's `res-fallchosen`, shorter than the
    /// queue entry's because the sentence above already names Smoozy's folder.
    pub const THIS_SERVERS_FALLBACK_FOLDER_CANT_BE_USED: &str = "This server’s fallback folder can’t be used.";
    /// **A destination at the top of the job's folder**, in the list and in
    /// the download box alike — the drawing's own words, one string for both.
    pub const TOP_OF_THE_JOBS_FOLDER: &str = "top of the job’s folder";
    pub const FILES_WORD: &str = "files";
    pub const FILE_WORD: &str = "file";
    pub const ONE_FILE_MAY_GROW: &str = "1\u{a0}file, may grow";
    pub const READ_FROM_ONE_SCENE_LEAD: &str = "Outputs are listed for one scene at a time;";
    pub const READ_FROM_ONE_SCENE_REST: &str =
        "are ticked. See each job’s queue entry.";
    pub const WRITES_NOTHING_LEAD: &str = "This job writes no files.";
    pub const WRITES_NOTHING_REST: &str =
        "Fix the output settings in the file.";
    pub const COLLIDE_LEAD: &str = "Two of this job’s outputs write to the same file.";
    pub const COLLIDE_REST: &str =
        "One will overwrite the other.";
    pub const THE_SCENES_OUTPUT_PATH: &str = "— the scene’s output path";
    pub const A_FILE_OUTPUT_NODE_NAMED: &str = "— File Output node";
    pub const AN_ADDED_WRITE_PLACE: &str = "— an added write place";
    // **The duplicate-picture line names the places the walk found** — the
    // runner's ruling of 2026-10-12, *The duplicate-picture warning names what
    // it found*, and `Design/Render Settings § Warnings` that day: *Beauty pass
    // written twice — two File Output nodes.*, *— a File Output node and the
    // output path.*, and so on. Composed in parts by
    // [`super::the_beauty_written_more_than_once`]. The line read *Beauty pass
    // written twice — a file output node and the output path.* whatever the
    // places were.
    /// **How many times, two to nine**, as the line's opening words.
    pub const THE_BEAUTY_WRITTEN_TIMES: [&str; 8] = [
        "Beauty pass written twice",
        "Beauty pass written three times",
        "Beauty pass written four times",
        "Beauty pass written five times",
        "Beauty pass written six times",
        "Beauty pass written seven times",
        "Beauty pass written eight times",
        "Beauty pass written nine times",
    ];
    /// Past nine, the count is a figure between these two.
    pub const THE_BEAUTY_WRITTEN: &str = "Beauty pass written";
    pub const TIMES_WORD: &str = "times";
    pub const BEFORE_THE_PLACES: &str = "—";
    /// **A count of places, two to nine**; past nine, a figure.
    pub const HOW_MANY_WORDS: [&str; 8] = ["two", "three", "four", "five", "six", "seven", "eight", "nine"];
    pub const INPUTS_OF: &str = "inputs of";
    pub const A_FILE_OUTPUT_NODE: &str = "a File Output node";
    pub const A_FILE_OUTPUT_NODE_FULL_STOP: &str = "a File Output node.";
    pub const FILE_OUTPUT_NODES: &str = "File Output nodes";
    pub const FILE_OUTPUT_NODES_FULL_STOP: &str = "File Output nodes.";
    pub const AND_THE_OUTPUT_PATH: &str = "and the output path.";
    /// **A video and frames are two outputs, not one picture written twice**
    /// — `Design/Render Settings § What it will write` and `Design/Queue &
    /// History § What the job writes`, 2026-10-12, the Creator's eighth notes,
    /// note 4. The drawing's `ow-vidfr` and `q-marks`.
    pub const FRAMES_AND_A_VIDEO: &str = "Frames from a File Output node, and a video from the output path.";
    pub const PULLS_IN_LEAD: &str = "uses";
    pub const PULLS_IN_REST: &str =
        "’s render, so it renders too, at its own settings.";
    pub const PULLED_IN_IS_TICKED_LEAD: &str = "is also ticked, so it renders twice:";
    pub const PULLED_IN_IS_TICKED_REST: &str = "alone and inside this job.";
    pub const ONE_VIDEO_FILE_LEAD: &str =
        "This job writes one video file, not one file per frame.";
    pub const ONE_VIDEO_FILE_SPLITS: &str = "Blender may split a long render into several files.";
    /// **The cost is a re-run's and not a loss** — the interrupt Smoozy sends
    /// leaves the file holding the frames that were finished (`Job files and
    /// outputs` C34); what the whole range costs is the second run, I22 having
    /// switched the already-there machinery off for a video-shaped job.
    pub const ONE_VIDEO_FILE_REST: &str = "If stopped, the whole range renders again.";
    /// **`Job files and outputs` I9a: a scene that writes more than one view
    /// separately** — `Design/Render Settings § What it will write` and
    /// `Design/Queue & History § What the job writes`, one line in both places,
    /// so one string (the 38's piece 10, item 38).
    pub const MORE_THAN_ONE_VIEW: &str =
        "This scene renders several views, so its frames can't be counted or checked.";
    /// **A typed frame outside the frames the read covered** — `Design/Render
    /// Settings § What it will write`, added 2026-10-17: no count is shown,
    /// and this one line stands in its place, under the heading and in the
    /// download box. The Creator's tenth notes, note 1.
    pub const FRAMES_OUTSIDE_THE_FILES_RANGE: &str =
        "Frames outside the file’s range can’t be counted before rendering.";

    // `§ The dependency warning` — four sentences, one band, one switch.
    pub const DEPENDENCIES: &str = "Dependencies";
    pub const NOBODY_HAS_CHECKED_LEAD: &str = "Dependencies not checked on";
    pub const NOBODY_HAS_CHECKED_REST: &str =
        "Assets may be missing from the server.";
    pub const REPLACED_LEAD: &str = "File re-uploaded since its dependency check.";
    pub const UNFOUND_LEAD: &str = "assets this file needs are not on the server.";
    // **One asset is one asset** — the pass before 0.1.9 on Vast read *1
    // assets … are not … leave them there*.
    pub const UNFOUND_LEAD_ONE: &str = "asset this file needs is not on the server.";
    pub const VERSION_LEAD: &str = "Dependencies were checked with Blender";
    pub const VERSION_MIDDLE: &str = ", not the";
    pub const _VERSION_MIDDLE_IS_PUNCTUATION_FIRST: () = ();
    pub const VERSION_REST: &str = "this job renders with.";
    pub const CHECK_DEPENDENCIES: &str = "Check dependencies…";
    pub const OPEN_THE_CHECK: &str = "Open dependency check";
    // `§ Submitting`: the check's uploads, counted above Render. The
    // Creator's note 6 on 0.1.8.
    pub const UPLOADING_DEPENDENCIES: &str = "Uploading dependencies:";
    pub const FILES_FULL_STOP: &str = "files.";
    pub const FILE_FULL_STOP: &str = "file.";
    pub const RENDER_ONE_FRAME_FIRST: &str = "Render one frame first";
    pub const SUBMIT_ANYWAY: &str = "Submit anyway";
    pub const ONE_FRAME_FIRST_BUYS: &str =
        "Missing assets then cost one frame, not the whole job.";
    pub const DONT_WARN_ME_AGAIN: &str = "Don’t warn me again";
    pub const IT_COVERS_EVERY_FILE: &str =
        "Applies to all files. Turn it back on in Settings.";

    // `§ Submitting`.
    pub const NAME_THIS_JOB: &str = "Name this job";
    pub const WHERE_THE_NAME_SHOWS: &str = "Shows in the queue and names its folder";
    pub const DOWNLOAD_AS_THEY_RENDER: &str = "Download frames as they render";
    pub const AND: &str = "and";
    pub const OF: &str = "of";
    pub const TO_FOLDER: &str = "to";
    pub const WHERE_THEY_GO: &str = "Download folder";
    /// The drawing's own dash between the formats and the count, and between
    /// *Download folder* and the folder. Punctuation, and it is the sentence's.
    pub const THE_DASH: &str = "—";
    pub const FILES_AND_A_COMMA: &str = "files,";
    /// The same, after a count of one: *1 PNG file, to ~/Downloads*.
    pub const FILE_AND_A_COMMA: &str = "file,";
    /// The format box's own word for leaving it, used where the run's lock
    /// cannot be taken and the box has nothing else to offer.
    pub const CANCEL: &str = "Cancel";
    pub const FORMATS_COME_FROM_THE_FILE: &str =
        "Formats are listed after a read. Until then, everything downloads.";
    /// **`§ Submitting`: where Smoozy can name no folder at all.** The lead
    /// says what happens and the rest says what it costs and what to do — the
    /// drawing's `adwarn` line under the summary control, at `res-nofolder`.
    pub const NO_FOLDER_IS_CHOSEN: &str =
        "No download folder chosen.";
    pub const THE_FRAMES_STAY_ON_THE_SERVER: &str =
        "Frames will stay on the server. Choose a folder above.";
    pub const TELEGRAM: &str = "Telegram";
    pub const EVERYTHING: &str = "Everything";
    pub const PICTURES_ONLY: &str = "Pictures only";
    pub const OFF: &str = "Off";
    pub const ANSWER_IN_QUEUE: &str = "Answer the first frame in Queue.";
    pub const ONLY_WHAT_NEEDS_YOU: &str = "Alerts only";
    /// **`§ Submitting`, *A fourth choice, per job only*** (2026-10-19): the
    /// row in the list, the closed control once it is picked, the word after
    /// the field, the field's own name, and the one refusal the minutes have.
    pub const A_PICTURE_EVERY_FEW_MINUTES: &str = "A picture every few minutes";
    pub const A_PICTURE_EVERY: &str = "A picture every";
    pub const MINUTES: &str = "minutes";
    pub const MINUTES_BETWEEN_PICTURES: &str = "Minutes between pictures";
    pub const WHOLE_MINUTES_1_TO_60: &str = "Whole minutes, 1 to 60.";
    pub const JOBS_WORD: &str = "jobs,";

    // `Design/Render Settings § What this document left open`, third bullet —
    // Simplify, which is `Rendering`'s and rides the re-submission.
    pub const SIMPLIFY: &str = "Simplify";
    pub const SIMPLIFY_OFFER: &str =
        "Reduces how much of the scene Blender loads. Offered on every failed job.";
    pub const SIMPLIFY_ON_IN_THE_FILE: &str =
        "On in this file; values are the file’s.";
    pub const MAX_SUBDIVISION: &str = "Max Subdivision";
    pub const CHILD_PARTICLES: &str = "Child Particles";
    pub const TEXTURE_LIMIT: &str = "Texture Limit";
    // The Texture Limit pick's *off*, in the drawing's word for it.
    pub const NO_LIMIT: &str = "No Limit";

    // The band that says where a re-submitted page's values came from.
    // `Design/Render Settings § What this document left open`, first bullet,
    // settled at Design and drawn in the mockup.
    pub const SUBMITTING_AGAIN_LEAD: &str = "Submitting again.";
    pub const SUBMITTING_AGAIN_MIDDLE: &str = "These are the settings";
    pub const SUBMITTING_AGAIN_REST: &str = ", not the file’s.";
    pub const NO_SERVER_CONNECTED_LEAD: &str = "No server is connected.";
    pub const NO_SERVER_CONNECTED_REST: &str = "Connect a server to submit.";
    pub const CARRYING_ON_LEAD: &str = "Resuming";
    pub const CARRYING_ON_CHECKED: &str =
        "is the last on the server, so the range starts at";
    pub const CARRYING_ON_SHORT_FILE: &str = "A partly written frame counts as finished and may be skipped.";
    pub const CARRYING_ON_FRAME: &str = "Frame";
    pub const CARRYING_ON_UNREAD: &str = concat!(
        "Could not read the server’s output, so the range starts at ",
        the_recorded_figure!(),
        ":"
    );
    pub const THESE_ARE_THE_SETTINGS_IT_USED_ON: &str = "Settings from";
    pub const USED_ON: &str = "used on";
    /// Punctuation rather than a word, so that a part ending a sentence can
    /// close it against the value it follows.
    pub const THE_FULL_STOP: &str = ".";

    // What an act answers where it cannot happen. **L8a: the screen says the
    // failure rather than showing an emptied version of itself**, and Rule 8
    // makes going quiet a fault however the trouble arose.
    pub const THE_RECORD_WOULD_NOT_READ: &str = "Could not read Smoozy’s database.";
    // A switch the user ticked that the record would not take — must-do 2.
    pub const THE_SETTING_DID_NOT_SAVE: &str = "Could not save the setting.";

    // **What the Submit button answers, step 10c piece 6.** Design draws
    // nothing for the moment after Submit is pressed — `Design/Render Settings
    // § What this document left open` puts the three lists on the queue — so
    // these are the plain sentences `Rendering` S10 requires be said, per
    // scene-job, and each says one thing.
    pub const SUBMITTING: &str = "Submitting.";
    pub const SUBMITTED: &str = "Submitted.";
    pub const JOBS_SUBMITTED: &str = "jobs submitted.";
    // **A submission that went, said as a confirmation** — `Design/Render
    // Settings § Submitting` (2026-10-03, the Creator's note 10 on the test
    // copy): *Submitted to Vast 4090.*, or *3 jobs submitted to Vast 4090.*,
    // and *Show in Queue* beside it.
    pub const SUBMITTED_TO: &str = "Submitted to";
    pub const JOBS_SUBMITTED_TO: &str = "jobs submitted to";
    pub const SHOW_IN_QUEUE: &str = "Show in Queue";
    pub const NOT_SUBMITTED: &str = "Not submitted.";
    pub const NOT_SUBMITTED_OF: &str = "of";
    pub const THE_SERVER_DOES_NOT_TAKE_JOBS: &str =
        "Smoozy on this server does not accept jobs.";
    pub const NOT_KNOWN_WHETHER_IT_WAS_TAKEN: &str =
        "Could not tell whether the server accepted the job.";
    pub const THE_FILES_COULD_NOT_BE_PLACED: &str =
        "Could not upload the job’s files to the server.";
    /// **The job's files were placed and the server did not take the job** —
    /// the machine answered that it holds no record of it (`Rendering` S9).
    /// *Design/Render Settings* gives no words for it, so these pair with
    /// [`NOT_KNOWN_WHETHER_IT_WAS_TAKEN`] in the page's own style. Added
    /// 2026-09-27: until then this said the files could not be placed, which
    /// was untrue — they were on the server.
    pub const THE_SERVER_DID_NOT_TAKE_THE_JOB: &str = "The server did not accept the job.";
    pub const THE_RECORD_REFUSED_THE_JOB: &str = "Could not save the job to Smoozy’s database.";
    pub const THE_BUILD_LACKS_THE_ENGINE_LEAD: &str = "does not have";
    // ~~*Only the first frame was submitted. Smoozy cannot send the rest of
    // the range as a job that waits until you have seen it.*~~ **Struck at step
    // 10c's eleventh-pass fix piece 8**, with the one route that said it: the
    // whole range is sent now, held (`Rendering` S13 as amended 2026-10-02 —
    // the rest of the range until then), so the second
    // sentence is false, and where the rest does not go the act says so the
    // way it says any job that did not — *1 of 2 jobs submitted.* and the
    // reason.
    // **`Design/Render Settings § Submitting`, where the first frame cannot be
    // named** (added 2026-10-01): with nothing read, the first half; with a
    // reading of several scenes, none ticked and no range typed, both. ~~*Render
    // one frame first needs the frames to render, and this file has not been
    // read. Type the frames first.*~~ **Struck at step 10c's Sesterce-pass fix
    // piece 1**: it was said after every read, where it was false.
    pub const THE_FIRST_FRAME_IS_NOT_KNOWN: &str = "Type the frames.";
    pub const THE_FIRST_FRAME_IS_NOT_KNOWN_OF_SEVERAL_SCENES: &str = "Type the frames or tick a scene.";
    pub const STOPPED_BEHIND_AN_UNKNOWN_ONE: &str =
        "An earlier job’s status is unknown, so the rest were not sent.";
    pub const THE_FILE_ON_THE_SERVER_COULD_NOT_BE_READ: &str =
        "Could not read the file on the server.";
    pub const IT_COULD_NOT_BE_PREPARED: &str = "Could not prepare the job on this computer.";
    pub const NOT_SET_UP_ON_THIS_SERVER: &str = "This server is not set up.";
    pub const NO_SERVER_IS_CONNECTED: &str = "No server is connected.";

    // **`Server Setup` B12's search, while it runs and where it could not.**
    pub const SEARCHING_THIS_SERVER: &str = "Searching this server for Blender.";
    pub const THE_SERVER_COULD_NOT_BE_SEARCHED: &str =
        "Could not search this server for Blender.";
    // `§ The dependency warning`, where the server's name could not be read.
    pub const NOBODY_HAS_CHECKED_ON_THIS_SERVER: &str =
        "Dependencies not checked on this server.";
}

/// **One piece of the duplicate-picture line**: words, a figure where a count
/// passes nine, and the words after it. The shape both screens draw — this
/// page's [`Words`] and the queue's `APart` — so the two say one line.
pub(crate) type APieceOfTheLine = (&'static str, Option<usize>, &'static str);

/// **The duplicate-picture line, naming the places the walk found** — the
/// runner's ruling of 2026-10-12 and `Design/Render Settings § Warnings`.
/// *Beauty pass written twice — a File Output node and the output path.*;
/// *— two File Output nodes.*; *— two inputs of a File Output node.*; *Beauty
/// pass written three times — two File Output nodes and the output path.*
/// **Nothing where the picture reaches fewer than two places.**
pub(crate) fn the_beauty_written_more_than_once(
    places: &smoozy_job_files::main_output::ThePlacesTheBeautyIsWritten,
) -> Vec<APieceOfTheLine> {
    let how_many = places.how_many();
    if how_many < 2 {
        return Vec::new();
    }
    let a_count = |count: usize, then: &'static str| -> APieceOfTheLine {
        match count {
            2..=9 => (words::HOW_MANY_WORDS[count - 2], None, then),
            _ => ("", Some(count), then),
        }
    };
    let mut pieces = vec![
        match how_many {
            2..=9 => (words::THE_BEAUTY_WRITTEN_TIMES[how_many - 2], None, ""),
            _ => (words::THE_BEAUTY_WRITTEN, Some(how_many), words::TIMES_WORD),
        },
        (words::BEFORE_THE_PLACES, None, ""),
    ];
    let nodes = places.file_output_nodes;
    if places.their_inputs > nodes {
        pieces.push(a_count(places.their_inputs, words::INPUTS_OF));
    }
    // **The last place ends the line**, so its words carry the full stop.
    let last = !places.the_output_path;
    match nodes {
        0 => {}
        1 if last => pieces.push((words::A_FILE_OUTPUT_NODE_FULL_STOP, None, "")),
        1 => pieces.push((words::A_FILE_OUTPUT_NODE, None, "")),
        _ if last => pieces.push(a_count(nodes, words::FILE_OUTPUT_NODES_FULL_STOP)),
        _ => pieces.push(a_count(nodes, words::FILE_OUTPUT_NODES)),
    }
    if places.the_output_path {
        pieces.push((words::AND_THE_OUTPUT_PATH, None, ""));
    }
    pieces
}

// ════════════════════════════════════════════════════════════════════════════
// What the run holds, and it is process memory for the life of the run
// ════════════════════════════════════════════════════════════════════════════

/// **J2: the override store is volatile — process memory for the life of the
/// application run — and it lives in the volatile class `State, identity and
/// recovery` §1.1 already defines.** *Closing Smoozy takes everything* is the
/// spec's own grant; nothing here is kept on disk, carried into another sitting
/// or written into the local record. **The recorded reading is the same class**
/// and is additionally period-held, dying with the connected period (J10).
///
/// **It is a static rather than a value the shell manages**, because `main.rs`
/// is shared and a piece adds nothing to it. One run, one of these.
#[derive(Default)]
pub struct WhatTheRunHolds {
    /// J1's second object: the user's answers and only the user's answers.
    pub store: Store,
    /// J1's first object: the file's values and only the file's values.
    pub readings: FileHalf,
    /// **The outputs half of the same one read** — `Job files and outputs`
    /// R1's first payload, which is that document's and is carried here
    /// beside the settings half rather than being read a second time (R9).
    ///
    /// **With the scene it was read from** — the 38's piece 10, item 25: it is
    /// drawn only where that scene is the page's scene in hand
    /// ([`the_outputs_in_hand`]).
    ///
    /// **One half per scene** — `Job files and outputs` R4b, the Creator's
    /// seventh notes, note 1: the read scene's first, then every other
    /// scene's the reading holds. A reading from a machine program older than
    /// that clause holds the read scene's alone.
    pub outputs: BTreeMap<String, Vec<(Vec<u8>, TheOutputsHalf)>>,
    /// **Q9: a failed read changes nothing**, so the ending is held beside the
    /// page rather than written anywhere, and it is cleared by the cross.
    pub the_read_that_gave_nothing: BTreeMap<String, AReadThatGaveNothing>,
    /// Whether a read is in flight for that (server, file) — Q5's join.
    pub reading_now: BTreeMap<String, bool>,
    /// B1's list, per server, where a machine answered one.
    pub version_lists: BTreeMap<String, TheListThisPageAsksFor>,
    /// **Where B1's list is being asked for, or the machine could not be
    /// asked**, per server — so the header never says *no server is
    /// connected* about a server that is being searched. Period-held with the
    /// list (`Server Setup` B12).
    pub version_searches: BTreeMap<String, TheSearchStands>,
    /// `Server Setup` B12: an answer from before an install or removal ended is no longer
    /// current. Each search keeps its own mark until its answer lands.
    version_searches_in_flight: BTreeMap<String, std::sync::Arc<()>>,
    /// **Whether a submission is running** for that (server, file), so the
    /// Render control is not offered twice for one act (step 10c piece 6).
    pub submitting_now: BTreeMap<String, bool>,
    /// `Servers`' remembered hardware reading, per server.
    pub hardware: BTreeMap<String, HeldHardware>,
    /// `Rendering` W2's record state for the (file, machine, version) — what
    /// `Dependency Upload & Remap` V14 supplies as data.
    pub the_check_record: BTreeMap<String, WhatTheRecordSays>,
    /// The three things settled at submission, per (server, file).
    pub submission: BTreeMap<String, TheSubmissionChoices>,
    /// Where a page opened from a history entry came from, per (server, file).
    pub where_it_came_from: BTreeMap<String, ThePageCameFrom>,
    /// **The file's saved-with version, which is `File Browser`'s read taken
    /// without opening the file** — `Render Settings` B2 names that read as the match's one
    /// source, so this page holds what that module handed it rather than
    /// reading the file itself.
    pub the_saved_with: BTreeMap<String, String>,
    /// **The folder the system's panel gave the open download box**, per
    /// (server, file), before the box's *Done* — `Design/Render Settings`:
    /// *"Choosing Somewhere else… puts the system's folder panel up at once,
    /// and the folder chosen there is shown in the box as the choice, which
    /// Done then keeps like any other."* The box offers it first, so *Done*
    /// hands it back by its row like every other folder (H34), and the page
    /// never names a folder itself. **A fresh box starts without one**, and
    /// *Done* lets it go. Added 2026-09-30, step 10c's eleventh-pass fix
    /// piece 7.
    pub the_folder_the_panel_gave: BTreeMap<String, Vec<u8>>,
    /// **The scene the last stored reading named as the one the file was
    /// saved showing**, per (server, file), where it named one — kept at the
    /// landing so *Reset all to file values* can tick it again (`Render
    /// Settings` J5 as changed 2026-10-08, the Creator's fifth notes, note 4).
    /// Read only while a reading stands. Added 2026-10-08.
    pub the_saved_scene: BTreeMap<String, Vec<u8>>,
}

/// **Where `Render Settings` B1's list stands while there is none to show.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TheSearchStands {
    /// `Server Setup`'s search is running on the machine now.
    Searching,
    /// **`Render Settings`' own *the machine could not be asked***: the search
    /// did not run, or the machine would not name its own clock. Nothing is
    /// claimed about what it holds.
    TheMachineCouldNotBeAsked,
}

/// `Design/Render Settings § Submitting` — the job's name and the three
/// choices, all of them the user's own answers and none of them the file's.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheSubmissionChoices {
    pub job_name: String,
    pub download_as_it_renders: bool,
    pub the_first_frame_check: bool,
    /// One of the ~~three~~ four the drawing offers. `Telegram`'s, carried
    /// here. **`None` is no per-job answer**, and then the job starts from the
    /// stored default through `Telegram` V1a's hand-over (fix-here item 9 of the round
    /// *setting it up, and what the desktop keeps*).
    pub what_it_reports: Option<&'static str>,
    /// **The minutes between pictures, for *A picture every few minutes***
    /// (`Specs/Telegram`, *Fewer pictures, on a clock*, 2026-10-19): whole
    /// minutes from 1 to 60, 5 until the user types another. **A typed entry
    /// Smoozy will not take never reaches here** (`Render Settings` F1), so
    /// this is always the minutes that stood. Kept whatever the choice, so
    /// picking another row and coming back finds the minutes typed.
    /// **`None` is none typed for this job**, and then the minutes are the
    /// stored default's, or 5 (`Telegram` V1a as amended, 2026-10-31: the
    /// default carries minutes) — [`the_minutes_in_force`].
    pub the_minutes_between_pictures: Option<u8>,
    /// **Which formats come down, and `None` is every one** — `Download` N3:
    /// *everything* is no filter rather than a frozen list. A list holds the
    /// format names exactly as the reading recorded them (`Download` N1).
    /// `Design/Render Settings § How the three sit together`.
    pub the_formats_chosen: Option<Vec<Vec<u8>>>,
    /// **Where the files go, where the user chose in the box.** `None` is not
    /// chosen yet, and then the standing folder, or else §8's proposal, is what
    /// the box offers first and what the summary names.
    pub where_they_go: Option<Vec<u8>>,
}

impl Default for TheSubmissionChoices {
    fn default() -> Self {
        TheSubmissionChoices {
            job_name: String::new(),
            // `Specs/Download`: off unless the user turns it on.
            download_as_it_renders: false,
            the_first_frame_check: false,
            what_it_reports: None,
            the_minutes_between_pictures: None,
            the_formats_chosen: None,
            where_they_go: None,
        }
    }
}

/// **What the page reads out of the local record and the system before it
/// composes**, read once and let go of before the run's own lock is taken.
///
/// - `Telegram` V1a's stored default, and the switch —
///   `Design/Render Settings § How the three sit together`: *"Where Telegram is
///   switched off it is not there at all."*
/// - `Download` H2's standing folder, and `Desktop app and delivery` §8's
///   proposal, taken fresh (W3).
///
/// **A record that does not answer holds nothing here**: no default, Telegram
/// read as off, no standing folder. The line a record that does not answer
/// cannot vouch for is left out rather than guessed.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct WhatTheRecordHolds {
    pub the_reporting_default: Option<String>,
    /// **The stored default's minutes** (version 56, `Telegram` V1a as
    /// amended), nothing where none were typed or taken.
    pub the_default_minutes: Option<u8>,
    pub telegram_is_on: bool,
    pub the_standing_folder: Option<Vec<u8>>,
    pub the_proposed_folder: Option<Vec<u8>>,
    /// **What the user calls the server** — its label, or `username@host` —
    /// read from its row. **`None` where the record did not answer**, and then
    /// no sentence names the server at all rather than naming it by its
    /// identifier (must-do 5).
    pub the_servers_name: Option<String>,
    /// **Whether `Rendering` W4a's band is switched off**, read through the
    /// shared service (must-do 2). **A record that does not answer reads as
    /// that, never as *not switched off*** — the shared-layer Verdict's F-10.
    pub the_dependency_warning: TheSwitchReads,
    /// **`Rendering` W2's record state for this file on this server**, read
    /// from the local record by [`the_check_the_band_reads`]. **The default is
    /// *nobody has ever checked***, so a page composed with nothing read never
    /// stops the warning.
    pub the_check: TheCheckTheBandReads,
    /// **How far this file's dependency check's uploads on this server have
    /// got** — files uploaded, of files being sent — **read from what
    /// Transfers lists and never counted a second time**
    /// (`crate::surface::transfers::the_checks_uploads_on`). Not the record's,
    /// though read beside it. `None` where nothing of the check is going. The
    /// Creator's note 6 on 0.1.8.
    pub the_checks_uploads: Option<(usize, usize)>,
    /// **The server's layout as its row records it** (`Job files and outputs`
    /// L1 and `Job files and outputs` L4a), read beside the record for one
    /// server — what [`where_the_jobs_folder_sits`] reads. **`None` where no placement is
    /// recorded or the row will not read back**, and then no line says where
    /// the job's folder will sit, because Submit refuses that server anyway.
    /// The Creator's second notes, piece 2 (note 18).
    pub the_layout: Option<TheLayout>,
    /// **The server row's chosen fallback folder** (`Servers` D3, `Job files
    /// and outputs` P23), read with the layout from the same row — what the
    /// line above *Output files* names where the job's folder falls back, and
    /// what Submit carries. The Creator's second notes, piece 4.
    pub the_servers_fallback_folder: Option<String>,
}

/// **What the dependency band reads: the check standing the local record
/// holds for this file on this machine, and whether Smoozy can keep a stop
/// true** — step 10c's dependency wave pieces 7 and 7b.
///
/// **The case the band was first written for, and the ones it meets now.** It
/// was written at step 10a against a standing put in the run's memory by hand.
/// Piece 7 read it from the record, but let a stop stand only for a file whose
/// root `Dependency Upload & Remap` V13a's walk could find **and** whose check this run held, the kinds its
/// pairs need being kept nowhere else — so a project uploaded through Files,
/// which is every real project, never stopped the warning, and no project
/// stopped it after a restart. **Piece 7b kept the kinds in the record
/// (`Dependency Upload & Remap` K4's last limb) and had the walk take
/// `File Browser` N6's answer first**, and this reads the record alone.
///
/// **`a_stop_can_be_kept` is `false`, and the warning stands where the
/// standing alone would stop it** (`Rendering` W1: the warning is absent only
/// where a complete standing satisfying `Dependency Upload & Remap` V11
/// exists, and these are where Smoozy cannot keep that true), **wherever the
/// standing says something was confirmed on the machine and one of these
/// holds** — the safe side, which still binds:
///
/// - **The record holds no answer to point it at**: the answers are read under
///   the holding file a check of this file keys them by, and a standing that
///   counted a confirmed row over none means the key has moved since the check
///   (an upload record the user removed, `File Browser` N8).
/// - **An answer to point at carries no kind**, so no pair can be composed for
///   it (`Dependency Upload & Remap` K4's last limb, written by no reading yet).
/// - **The disturbance mark could never reach this file**:
///   `smoozy_dependencies::standing::the_walk_reaches_the_roots_own_answers`,
///   for a root the walk keys differently from the check — one that came up as
///   a dependency with no upload through Files behind it.
///
/// **A standing that confirmed nothing** — no reference, or every row set
/// aside — has neither a mark to miss nor a pair to compose, and stops.
///
/// **The statement drawn over a stop that cannot be kept is the one an
/// incomplete check is drawn under**, `Rendering`'s *nobody has checked*,
/// whose own mapping states that the words are wrong about the record there
/// too; **a true sentence for it is Design's** and is not written here.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheCheckTheBandReads {
    pub the_record_says: WhatTheRecordSays,
    pub a_stop_can_be_kept: bool,
}

impl Default for TheCheckTheBandReads {
    fn default() -> TheCheckTheBandReads {
        TheCheckTheBandReads {
            the_record_says: WhatTheRecordSays::NobodyHasEverChecked,
            a_stop_can_be_kept: false,
        }
    }
}

/// **`Rendering` W1 and W2 over what the band reads**: `Rendering`'s own
/// evaluation, and a stop Smoozy cannot keep is not let stand.
pub fn the_warnings_case(
    the_check: &TheCheckTheBandReads,
    picked: &BlenderIdentity,
) -> TheWarningsCase {
    match smoozy_rendering::warning::evaluate(&the_check.the_record_says, picked) {
        TheWarningsCase::Absent if !the_check.a_stop_can_be_kept => {
            TheWarningsCase::ACheckWasRunAndDidNotComplete
        }
        case => case,
    }
}

/// **What the band reads for `file` on `server`, from the local record
/// alone** — see [`TheCheckTheBandReads`]. **A record that does not answer is
/// an error**, which the caller draws as *nobody has checked*.
pub fn the_check_the_band_reads(
    connection: &rusqlite::Connection,
    machine: &smoozy_dependencies::record::MachineKey,
    file: &str,
) -> Result<TheCheckTheBandReads, crate::record::RecordFailure> {
    use crate::record::dependency_records::{TheDependencyRecord, WhatFilesSays};
    use smoozy_dependencies::bytes::MachineLocation;
    use smoozy_dependencies::record::InMemoryRecord;
    let root = MachineLocation::composed(file.as_bytes().to_vec());
    let keeper = TheDependencyRecord::over(connection);
    let the_record_says = match keeper.the_standing(&root, machine)? {
        Some(standing) => WhatTheRecordSays::AStanding(Box::new(standing)),
        None => WhatTheRecordSays::NobodyHasEverChecked,
    };
    let it_confirmed_something = matches!(
        &the_record_says,
        WhatTheRecordSays::AStanding(standing) if standing.counts.confirmed > 0
    );
    let files_says = WhatFilesSays::read(connection, machine);
    let holding = files_says.the_holding_file_a_check_keys(&root);
    // **What would be pointed at**: an answer resolved and not set aside.
    let to_point_at: Vec<_> = keeper
        .the_answers_of(&holding)?
        .into_iter()
        .filter(|answer| answer.resolution.is_some() && !answer.set_aside)
        .collect();
    let a_stop_can_be_kept = if !it_confirmed_something && to_point_at.is_empty() {
        true
    } else {
        !to_point_at.is_empty()
            && to_point_at.iter().all(|answer| !answer.read_as.is_empty())
            && smoozy_dependencies::standing::the_walk_reaches_the_roots_own_answers(
                &root,
                machine,
                &InMemoryRecord {
                    placements: keeper.the_placements_on(machine)?,
                    ..InMemoryRecord::default()
                },
                &holding,
                &|file| files_says.where_it_came_from(file),
            )
    };
    Ok(TheCheckTheBandReads {
        the_record_says,
        a_stop_can_be_kept,
    })
}

/// [`the_check_the_band_reads`], under the application's lock.
fn the_check_is_read_now(
    started: &State<'_, Mutex<Started>>,
    server: &str,
    file: &str,
) -> TheCheckTheBandReads {
    let Ok(started) = started.lock() else {
        return TheCheckTheBandReads::default();
    };
    let Some(record) = started.record.as_ref() else {
        return TheCheckTheBandReads::default();
    };
    // **No installation confirmed, no machine**: nothing was checked under one.
    let Some(installation) = crate::record::server_rows::all(record.connection())
        .ok()
        .and_then(|rows| rows.into_iter().find(|row| row.local_server_identifier == server))
        .and_then(|row| row.last_confirmed_installation_identity)
        .and_then(|text| Identity::read_back(IdentityKind::Installation, &text))
    else {
        return TheCheckTheBandReads::default();
    };
    let machine = smoozy_dependencies::record::MachineKey::of(an_identity(server), installation);
    the_check_the_band_reads(record.connection(), &machine, file).unwrap_or_default()
}

/// **What the shared service answered about the band's switch-off** —
/// [`crate::surface::said_once::WhetherItIsSwitchedOff`], in a shape this
/// page's value can carry. **The default is the read that failed**, so a page
/// composed with nothing read never claims to know.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum TheSwitchReads {
    NotSwitchedOff,
    SwitchedOff,
    #[default]
    TheReadFailed,
}

impl WhatTheRecordHolds {
    /// **Read now**, from the record the application opened and from §8's
    /// route. Nothing is kept.
    pub fn read_now(record: Option<&crate::record::LocalRecord>) -> WhatTheRecordHolds {
        let telegram = record.and_then(|record| {
            crate::record::settings::the_telegram_settings(record.connection()).ok()
        });
        let settings = record.and_then(|record| crate::record::settings::read(record.connection()).ok());
        let the_proposed_folder = match crate::platform::downloads_folder::the_folder_the_system_proposes() {
            crate::platform::downloads_folder::TheProposal::TheSystemNamesThisFolder(folder) => {
                Some(folder.as_os_str().as_encoded_bytes().to_vec())
            }
            crate::platform::downloads_folder::TheProposal::TheSystemNamesNoFolder(_) => None,
        };
        WhatTheRecordHolds {
            // **Read with the layout, from the row**, by the caller that knows
            // which server (`the_layout` below is read the same way).
            the_servers_fallback_folder: None,
            the_reporting_default: telegram.as_ref().and_then(|t| t.the_reporting_default.clone()),
            the_default_minutes: telegram.as_ref().and_then(|t| t.the_default_minutes),
            telegram_is_on: telegram.as_ref().is_some_and(|t| t.switched_on),
            the_standing_folder: settings.and_then(|s| s.the_chosen_download_folder),
            the_proposed_folder,
            // **Read beside the record, from the application's own state** —
            // see `the_record_now`. Nothing is claimed here.
            the_servers_name: None,
            the_dependency_warning: TheSwitchReads::TheReadFailed,
            // **Read beside the record, for one file** — see
            // `the_check_is_read_now`. Nothing is claimed here.
            the_check: TheCheckTheBandReads::default(),
            // **Read from what Transfers lists, for one file** — see the
            // command. Nothing is claimed here.
            the_checks_uploads: None,
            // **Read beside the record, for one server** — see
            // `the_record_now`. Nothing is claimed here.
            the_layout: None,
        }
    }

    /// §8's proposal in the shape piece 12's box takes it.
    pub(crate) fn the_proposal(&self) -> crate::platform::downloads_folder::TheProposal {
        match &self.the_proposed_folder {
            Some(folder) => crate::platform::downloads_folder::TheProposal::TheSystemNamesThisFolder(
                std::path::PathBuf::from(shown(folder)),
            ),
            None => crate::platform::downloads_folder::TheProposal::TheSystemNamesNoFolder(String::new()),
        }
    }
}

/// A folder's bytes as the page shows them. **A value**, never prose.
fn shown(folder: &[u8]) -> String {
    String::from_utf8_lossy(folder).to_string()
}

/// The five the drawing offers, in its own order — the value the
/// page hands back, and the words. **Settings offers the same five as the
/// default from 2026-10-31** (`crate::surface::telegram::TheReportingChoice`,
/// `Telegram` V1d); until then it kept three, the third and fifth here being
/// per job only.
pub const REPORTING_CHOICES: [(&str, &str); 5] = [
    ("everything", words::EVERYTHING),
    ("pictures-only", words::PICTURES_ONLY),
    (A_PICTURE_EVERY_FEW_MINUTES, words::A_PICTURE_EVERY_FEW_MINUTES),
    ("only-what-needs-you", words::ONLY_WHAT_NEEDS_YOU),
    ("nothing", words::OFF),
];

/// **The fourth choice's identifier**, which alone carries a value.
pub const A_PICTURE_EVERY_FEW_MINUTES: &str = "a-picture-every-few-minutes";

/// **The page identifier as a choice**, for the job: the row's own member, the
/// fourth carrying the minutes that stood. Nothing for an identifier the list
/// does not hold.
fn the_choice_named(id: &str, the_minutes: u8) -> Option<ReportingChoice> {
    let at = REPORTING_CHOICES.iter().position(|(value, _)| *value == id)?;
    Some(match ReportingChoice::ALL[at] {
        ReportingChoice::APictureEveryFewMinutes { .. } => ReportingChoice::APictureEveryFewMinutes { minutes: the_minutes },
        other => other,
    })
}

/// **A choice as the page names it, and its minutes for the fourth** — the
/// other way from [`the_choice_named`], for a choice that arrived in the
/// manifest's own form: `Telegram` V1d's phone's choice, taken as Settings'
/// default.
pub fn the_page_identifier_of(choice: ReportingChoice) -> (&'static str, Option<u8>) {
    let at = ReportingChoice::ALL
        .iter()
        .position(|one| std::mem::discriminant(one) == std::mem::discriminant(&choice))
        .unwrap_or(0);
    let minutes = match choice {
        ReportingChoice::APictureEveryFewMinutes { minutes } => Some(minutes),
        _ => None,
    };
    (REPORTING_CHOICES[at].0, minutes)
}

/// **The app-wide value, as Settings stored it**: any of the five, the third
/// with `the_minutes` (`Telegram` V1a as amended, 2026-10-31: *"the app-wide
/// item holds any of the five, minutes included, because a choice from the
/// phone becomes it"*). An identifier the list does not hold is no default.
/// *(Until that day it was held to Settings' three.)*
fn the_app_wide_choice_named(id: &str, the_minutes: u8) -> Option<ReportingChoice> {
    the_choice_named(id, the_minutes)
}

/// **The minutes a job's *a picture every few minutes* carries**: those typed
/// for the job, else the stored default's, else 5. So a default of *a picture
/// every 10 minutes* offers 10, and minutes typed on the page for this job win.
fn the_minutes_in_force(submission: &TheSubmissionChoices, record: &WhatTheRecordHolds) -> u8 {
    submission
        .the_minutes_between_pictures
        .or(record.the_default_minutes)
        .unwrap_or(reporting::THE_MINUTES_TO_BEGIN_WITH)
}

/// **Where a page opened from a history entry came from** — `Rendering` A1's
/// *submitting again opens this page filled from the history entry's kept
/// setup*, and A2's *carrying on*. The band is the one thing that says so:
/// italic grey and bold cannot, both kinds of value being the user's.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhereItCameFrom {
    /// The kept setup, and a server to send it to.
    SubmittingAgain { the_jobs_name: String, when: String },
    /// The kept setup, and nothing connected — the setup is not lost by there
    /// being no server to send it to.
    SubmittingAgainWithNothingConnected { the_jobs_name: String, when: String },
    /// The range moved, and what it moved to was checked on the machine.
    CarryingOn {
        the_jobs_name: String,
        when: String,
        the_last_frame_written: i64,
        the_range_now_starts_at: i64,
    },
    /// The range moved, and the figure is the one the entry recorded rather
    /// than one just checked — so the band says so.
    CarryingOnWithTheOutputsUnread {
        the_jobs_name: String,
        when: String,
        the_range_now_starts_at: i64,
    },
}

/// **Where a page opened from a history entry came from, and what that
/// entry decides on it** — the 38's piece 2.
///
/// - **`the_simplify_offer_is_drawn` is `Rendering` A1b's**: the offer is the
///   entry's, drawn where it ended failed, whichever of the routes opened the
///   page. Until this piece the group was drawn on every page a history entry
///   opened, which `Rendering` A1b and `Rendering` A1c forbid.
/// - **`the_entry` is the job the page was opened from**, so a carry-on's job
///   carries `Queue & History` N15's mark naming it, with the label as it
///   stood (`Queue & History` N20).
/// - **`the_moment` is when that job was submitted**, in seconds, for the page
///   to write as a date — this application writes no date of its own.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThePageCameFrom {
    pub band: WhereItCameFrom,
    pub the_simplify_offer_is_drawn: bool,
    pub the_entry: Identity,
    pub the_entrys_label: String,
    pub the_moment: Option<String>,
}

impl ThePageCameFrom {
    /// **`Queue & History` N15's second mark, for the job this page submits**
    /// — on a carry-on and on nothing else.
    pub fn the_carry_on_mark(&self) -> Option<smoozy_queue_history::record::CarriesOnFrom> {
        matches!(
            self.band,
            WhereItCameFrom::CarryingOn { .. } | WhereItCameFrom::CarryingOnWithTheOutputsUnread { .. }
        )
        .then(|| smoozy_queue_history::record::CarriesOnFrom {
            earlier_job: self.the_entry.clone(),
            the_label_as_it_stood: self.the_entrys_label.clone(),
        })
    }
}

/// **The page on (server, file) came from a history entry** — written by the
/// route in `crate::a_new_job_from_an_old_one`, at the open or at the pick.
pub(crate) fn the_page_came_from(held: &mut WhatTheRunHolds, server: &str, file: &str, came_from: ThePageCameFrom) {
    held.where_it_came_from.insert(keyed(server, file), came_from);
}

/// The one of these there is.
pub fn the_run_holds() -> &'static Mutex<WhatTheRunHolds> {
    static HELD: OnceLock<Mutex<WhatTheRunHolds>> = OnceLock::new();
    HELD.get_or_init(|| Mutex::new(WhatTheRunHolds::default()))
}

/// **`Render Settings` D5: the hardware reading `Servers` holds for this row,
/// in the state the row itself is drawn in** — taken from the server row and
/// from what this run knows about its connections, and **never from anything
/// this page remembers.**
///
/// **This exists because of step 10c's fifth pass on Vast, fault 4.** The map
/// [`WhatTheRunHolds::hardware`] is read at three places on this page — the
/// device warning, *Renders on*, and the card make that goes on the manifest
/// (`Job files and outputs` M1b) — and **nothing in the workspace had ever
/// written to it**, so all three read *never read* on a connected machine
/// whose row was showing its card. The submission then carried
/// `card_make: "NoReadingStood"` and the render worker ended the attempt
/// before the render instruction.
///
/// **The state is `Servers`' own judgement, called rather than repeated** —
/// [`crate::surface::servers::the_hardware_state_this_run_knows`] — so the
/// value the user was shown and the value the render names are one object,
/// which is exactly what D5 asks for.
pub fn the_hardware_the_row_stands_in(
    row: &smoozy_servers::store::ServerRow,
    read: Option<crate::servers::a_period::WhatTheLastConnectionRead>,
    of: smoozy_servers::condition::WhatIsTrueOfTheServer,
) -> HeldHardware {
    let held = row.remembered_hardware_reading.as_ref();
    let state = crate::surface::servers::the_hardware_state_this_run_knows(held, read, of);
    HeldHardware::from_servers(state, held.cloned())
}

/// **The reading held beside the page, for the server it was read for.**
///
/// **`Servers` H4: a reading replaces the held one whole**, so this is a
/// replacement and never a merge, and a row with no reading in the state it
/// stands in takes the map's entry away rather than leaving a stale one.
pub fn the_hardware_stands_at(
    held: &mut WhatTheRunHolds,
    server: &str,
    hardware: HeldHardware,
) {
    match hardware {
        HeldHardware::NeverRead => {
            held.hardware.remove(server);
        }
        hardware => {
            held.hardware.insert(server.to_string(), hardware);
        }
    }
}

/// **The same reading, taken for one server from the application's own state.**
/// **Nothing is claimed where the record will not read, or holds no row for
/// this server**: the map is left as it was rather than being emptied, because
/// a record that did not answer is not a row with no reading, and a page is not
/// drawn for a row that is gone.
fn the_hardware_is_read_now<R: tauri::Runtime>(
    app: &tauri::AppHandle<R>,
    started: &State<'_, Mutex<Started>>,
    server: &str,
) -> Option<HeldHardware> {
    let row = started
        .lock()
        .ok()?
        .record
        .as_ref()
        .and_then(|record| crate::record::server_rows::all(record.connection()).ok())?
        .into_iter()
        .find(|row| row.local_server_identifier == server)?;
    let periods = crate::surface::servers::the_periods_of(app);
    let periods = periods.as_deref();
    Some(the_hardware_the_row_stands_in(
        &row,
        periods.and_then(|periods| periods.what_the_last_connection_read(server)),
        crate::surface::servers::what_is_true_of(periods, server),
    ))
}

/// The one key every map above is keyed by. **A rendering and never a
/// meaning** — nothing parses one.
fn keyed(server: &str, file: &str) -> String {
    format!("{server}\u{1f}{file}")
}

/// **The file arrives as its exact bytes** — `File Browser` L1a, and must-do 22
/// of the step 10a round 1 audit of these pieces: every way into this panel
/// hands over the row's bytes, never its display name.
///
/// **The key it is held under is `Render Settings`' own `FileKey`, whose path
/// is text**, so the bytes become that text here and nowhere else. A path that
/// is valid text crosses exactly. **A path holding a byte no encoding carries
/// cannot be keyed exactly by that type**, and it is decoded with the byte
/// replaced — which is a limit of `Render Settings`' key and is recorded rather
/// than worked around here.
pub fn the_path_as_it_is_keyed(bytes: &[u8]) -> String {
    match std::str::from_utf8(bytes) {
        Ok(text) => text.to_string(),
        Err(_) => String::from_utf8_lossy(bytes).into_owned(),
    }
}

// ════════════════════════════════════════════════════════════════════════════
// What the page is handed
// ════════════════════════════════════════════════════════════════════════════

/// One entry in the version dropdown.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct AVersion {
    /// The build's path, which is what a pick hands back. A value.
    pub build: Verbatim,
    /// The version as the build answered it. A value.
    pub answered: Verbatim,
    /// **Below the floor, said as such.** Prose, and absent otherwise.
    pub below_the_floor: Option<Prose>,
    pub picked: bool,
}

/// **A setting with a value the user can hand back** — `Design/Render Settings
/// § The file's value against the user's`: italic grey is the file's, upright
/// bold is the user's, with a cross beside it.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct AField {
    /// The label. Prose.
    pub label: Prose,
    /// **Whose value is on screen** — `the-users`, `the-files`, or
    /// `the-promise-form`, which is the em dash. A value the page keys on.
    pub whose: Verbatim,
    /// The value itself, where there is one. A value.
    pub value: Option<Verbatim>,
    /// What the cross hands back to, where a cross is drawn. Prose beside a
    /// value, never joined.
    pub hand_back_to: Option<Words>,
    /// **Greyed, which is `§ Scenes`' *a greyed control shows no value*.**
    pub greyed: bool,
    /// **Refused, which is `§ Warnings`' *turns the field red*.**
    pub refused: bool,
}

/// One scene box, or the *All scenes* box beside them.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct ABox {
    /// The scene's own name, exactly as the reading produced it. A value.
    pub name: Verbatim,
    pub ticked: bool,
    /// The tag beside *All scenes*, and nothing on a scene.
    pub tag: Option<Words>,
}

/// One view-layer box — `§ View Layers`, the most marked setting on the page.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct AViewLayer {
    /// The layer's own name. A value.
    pub name: Verbatim,
    pub ticked: bool,
    /// **Whether the box is the user's pin or the file's reading at the stated
    /// frame** — `the-users` or `the-files`. A value the page keys on.
    pub whose: Verbatim,
    /// **The keyframe diamond** — `solid`, `hollow`, or absent. A value.
    pub diamond: Option<Verbatim>,
    /// What the diamond means, for anyone who cannot see it. Words with the
    /// frame beside them.
    pub diamond_said: Option<Words>,
    /// **`unknown until read`** — Q3's own answer where a driven switch
    /// is asked about outside the scene's own range. Prose.
    pub not_known: Option<Prose>,
    /// The layer's own sample count, where it has one. Words with the figure.
    pub tag: Option<Words>,
    /// What the cross hands back to, on a pinned box. Words with the frame.
    pub hand_back_to: Option<Words>,
}

/// The mismatch warning — `§ The version mismatch warning`: one line naming
/// the two versions and saying something may not work as expected, **and
/// nothing else** (B8, 2026-10-02).
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct TheMismatch {
    /// *Saved with 4.5, rendering with 5.1. Some things may not work as
    /// expected.* — two values in one line.
    pub line: ASentence,
}

/// One of Q8's four endings, in the one failure shape.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct TheReadFailure {
    /// Which ending, as a value the page keys on and a test counts.
    pub which: Verbatim,
    /// **The plain sentence saying what did not happen, and it is whole.**
    /// X7: a name, a version or a path never travels inside a sentence, and the
    /// one failure shape's own answer is that the values sit under it, each
    /// under its own name — so the sentence names none of them.
    pub what_did_not_happen: Prose,
    /// **The second sentence, only where Smoozy knows what to do next** —
    /// which is the second of the four and no other.
    pub what_to_do_about_it: Option<Prose>,
    /// The values that belong beside the sentence, each with its own name.
    pub values: Vec<AChoice>,
    /// **The raw text, where there is any**, and the control names whose words
    /// they are.
    pub the_raw_text: Option<Prose>,
    /// The cross. Every one of the four carries it: each describes something
    /// still true while the user is looking at it.
    pub clear_this: Prose,
}

/// The engine control's three rows — `§ The render engine`.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct TheEngine {
    pub label: Prose,
    /// **The file's own engine, by name**, where a reading stands and it is
    /// neither of the two offered. A value.
    pub the_files_own: Option<Verbatim>,
    /// The words beside it — *the file's* — where the row is drawn. Prose.
    pub the_files_own_said: Option<Prose>,
    /// **The promise form**, where nothing has been read. Prose.
    pub not_read_yet: Option<Prose>,
    pub cycles: Prose,
    pub eevee: Prose,
    /// Which row the setting is on — `the-files`, `cycles` or `eevee`. A value.
    pub on: Verbatim,
    /// What the cross hands back to, where the user has picked.
    pub hand_back_to: Option<Words>,
    /// The one line under the group.
    pub note: Prose,
    /// **`§ The one thing this page says about add-ons`** — the announcement,
    /// where the file makes one.
    pub the_add_on_case: Option<TheAddOnCase>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct TheAddOnCase {
    /// *Blender 5.1.0 does not have the add-on for the engine this file
    /// names.* — the version is a value beside the words.
    pub lead: Words,
    pub read_back_as_eevee: Prose,
}

/// `§ What the render runs on` — two buttons, and the tag's four faces.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct TheDevice {
    pub label: Prose,
    pub gpu: Prose,
    pub cpu: Prose,
    /// Which is in force — `the-cards` or `the-processor`. A value.
    pub on: Verbatim,
    /// **The promise form**, where nothing has been read.
    pub not_read_yet: Option<Prose>,
    /// D6's tag, where both halves of D5 stand and D6's table shows one.
    pub tag: ASentence,
    /// **The tag's *last seen* moment, as seconds** — the one value in the tag
    /// the page writes as a date or a time, the way every screen writes one
    /// (`web/servers.js`' `theMomentReads`), as the band's moment is. Until
    /// 2026-10-03 the page drew the seconds raw (an owed fault folded into
    /// the Creator's notes on the test copy, piece 3).
    pub the_moment: Option<Verbatim>,
    pub hand_back_to: Option<Words>,
    /// **Greyed with nothing on, where several scenes are ticked** — `§ Scenes`.
    pub greyed: bool,
}

/// `§ What it will write` — the destination list and the total.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct TheWrites {
    pub rows: Vec<AWriteRow>,
    /// The total, as *720 files*.
    pub total: Words,
    /// The total split by format, one line each.
    pub by_format: Vec<AWriteTotal>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct AWriteRow {
    /// Where it writes: its folder under the job's folder, as `PNG/`. A value.
    /// Empty where [`AWriteRow::the_top`] says it.
    pub place: Verbatim,
    /// ***top of the job's folder***, where the destination writes there.
    /// Words, so the page draws them as prose and not as a path.
    pub the_top: Option<Prose>,
    /// How many files. A value.
    pub how_many: Verbatim,
    /// Which format, where the destination has one. A value.
    pub format: Option<Verbatim>,
}

/// **A sentence, as a sequence of parts.**
///
/// **X10: where the orphan rule must hold inside a multi-sentence block, the
/// caller hands each sentence as its own string** — so a block of two sentences
/// is two parts here and never one joined string, and the typography point
/// works on each of them. **X7: a name, a figure, a path or a port never
/// travels inside a sentence**, so each part carries its words and its values
/// in two fields and the page sets them side by side.
pub type ASentence = Vec<Words>;

/// **One thing a control offers** — the value the page hands back, and the
/// words beside it. Two strings on two routes.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct AChoice {
    pub value: Verbatim,
    pub said: Prose,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct AWriteTotal {
    pub how_many: Verbatim,
    pub format: Verbatim,
}

/// One of the two warnings under `§ What it will write` that hold a job.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct TheOutputWarning {
    /// `writes-nothing` or `collide`. A value the page keys on.
    pub which: Verbatim,
    pub lead: Prose,
    pub rest: Prose,
    /// **Every colliding pair, in the order the module gives them** (must-do
    /// 14), each side naming where it comes from as well as where it lands.
    pub pairs: Vec<Vec<ASentence>>,
}

/// **`§ Submitting`: the line that says the frames have nowhere to come home
/// to** — the drawing's `adwarn`, a bold lead and the rest beside it, which is
/// the same two-part shape [`TheOutputWarning`] already uses on this page.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct TheNowhereLine {
    pub lead: Prose,
    pub rest: Prose,
}

/// `§ The dependency warning` — one band, four sentences, three ways on.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct TheDependencyBand {
    /// Which of the four, as a value a test counts.
    pub which: Verbatim,
    pub said: ASentence,
    /// The leftmost way on — *Check dependencies…* or *Open the check*.
    pub the_way_to_the_check: Prose,
    /// **Whether that way starts a check.** *Open the check* goes to the rows
    /// that are already there and starts nothing; *Check dependencies…* starts
    /// one (must-do 11 of the step 10a round 1 audit of these pieces).
    pub the_way_starts_a_check: bool,
    /// **Where the switch-off could not be read**, the band says so rather than
    /// drawing itself as though the answer were known — the shared-layer
    /// Verdict's F-10.
    pub the_switch_could_not_be_read: Option<Prose>,
    /// **Absent where the user has already asked for the first-frame check**,
    /// which is the spec's own *it is the same thing they already have*.
    pub render_one_frame_first: Option<Prose>,
    pub what_one_frame_first_buys: Option<Prose>,
    pub submit_anyway: Prose,
    pub switch_it_off: Prose,
    pub what_the_switch_covers: Prose,
}

/// `Design/Render Settings § What this document left open`, third bullet.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct TheSimplify {
    pub label: Prose,
    pub on: bool,
    /// The offer, or the statement where the file was saved with it on.
    pub said: Prose,
    /// The three, each with its own two faces.
    pub settings: Vec<AField>,
    /// **The Texture Limit pick's choices** — `Rendering` A1d's enumeration,
    /// each identifier with what the pick shows for it.
    pub texture_choices: Vec<AChoice>,
}

/// **The whole panel, in one answer** — H34's closed response type.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct TheRenderSettings {
    pub server: Verbatim,
    /// **The file as this page keys it** — the text [`the_path_as_it_is_keyed`]
    /// made of the bytes. An act's event carries the same text, which is how
    /// the page tells its own acts apart.
    pub file: Verbatim,
    /// **The file's exact bytes**, which is what every act hands back
    /// (`File Browser` L1a, must-do 22).
    pub file_bytes: Vec<u8>,
    pub the_files_name: Verbatim,
    pub heading: Prose,
    /// **The panel's title, built the way the check's is** — *Render settings
    /// — kitchen_v14.blend*: the words, then the file's name as a value
    /// (must-do 17).
    pub title: Words,
    /// **The file's name in the title is a link into Files** — the folder and
    /// the name it hands over, composed here so the page composes no path.
    /// The ninth notes, note 5.
    pub show_in_files: Option<TheFileInFiles>,

    // The header.
    pub blender: Prose,
    pub blender_version: Prose,
    pub the_versions: Vec<AVersion>,
    pub the_version_control_is_drawn: bool,
    pub no_blender: ASentence,
    pub set_it_up: Prose,
    pub the_pick_line: Option<Prose>,
    /// **Whether `the_pick_line` is the Blender search still running**, so the
    /// page puts the turning mark on the line that will answer it —
    /// `Design/Whole app § Anything that isn't instant shows that it's
    /// underway`: *"where the control that started the work is gone … the
    /// reading it produced carries the mark itself."* No control starts this
    /// search; opening the page does. Added 2026-09-27, step 10c's eighth-pass
    /// fix piece 5.
    pub the_search_is_underway: bool,
    pub the_search_marks: Vec<Words>,
    pub the_mismatch: Option<TheMismatch>,
    pub read_at: ASentence,
    pub read_the_file: Prose,
    pub read_again: Prose,
    pub reading_the_file: Prose,
    pub a_reading_stands: bool,
    pub the_read_is_offered: bool,
    pub the_read_is_underway: bool,
    pub reset_all: Prose,

    // The bands.
    pub the_band: ASentence,
    pub where_it_came_from: ASentence,
    pub where_it_came_from_is_waiting: bool,
    /// **The moment inside the band, in seconds**, which the page writes as
    /// a date wherever the band carries it as a value. Absent where the page
    /// came from no entry.
    pub where_it_came_from_moment: Option<String>,
    pub the_read_failure: Option<TheReadFailure>,

    // The left column.
    pub scene_heading: Prose,
    pub the_scenes: Vec<ABox>,
    pub all_scenes: Option<ABox>,
    pub the_scene_group_is_drawn: bool,
    pub frames_heading: Prose,
    /// **Whose range is showing**, where the heading says it — none where the
    /// user's own range stands on one scene (`Design/Render Settings
    /// § Frames`, 2026-10-01).
    pub frames_qualifier: Option<Words>,
    pub the_range: AField,
    pub the_frame_count: ASentence,
    pub the_range_forms: Vec<AChoice>,
    pub how_to_type_a_range: Prose,
    pub view_layers_heading: Prose,
    pub view_layers_qualifier: Option<Words>,
    pub the_view_layers: Vec<AViewLayer>,
    pub the_view_layer_group_is_drawn: bool,

    // The middle column.
    pub render_heading: Prose,
    pub the_render_group_is_greyed: bool,
    pub samples: AField,
    pub the_samples_lines: Vec<ASentence>,
    pub simplify: Option<TheSimplify>,
    pub resolution: AField,
    pub the_size_reading: Option<Verbatim>,
    pub the_size_is_not_known: Option<Prose>,
    pub the_refusal_line: ASentence,
    pub engine: TheEngine,
    pub renders_on: Option<TheDevice>,

    // The right column.
    pub output_heading: Prose,
    /// **`§ What it will write`: one line above the list says where the job's
    /// folder will sit**, or why it falls back — [`the_folder_line`]. Drawn
    /// only beside the list, and only where the server's layout is recorded.
    /// The Creator's second notes, piece 2 (note 18).
    pub the_folder_line: Option<ASentence>,
    /// **The second line under it, where the server's fallback folder is
    /// ruled out before submitting** — *This server's fallback folder can't
    /// be used.* (`Design/Render Settings § What it will write`, the drawing's
    /// `res-fallchosen`). The Creator's second notes, piece 4.
    pub the_second_folder_line: Option<ASentence>,
    pub what_it_will_write: Option<TheWrites>,
    pub the_output_warnings: Vec<TheOutputWarning>,
    pub the_output_lines: Vec<ASentence>,
    pub the_output_group_is_drawn: bool,
    pub one_scene_only: ASentence,
    pub dependencies_heading: Prose,
    pub the_dependency_band: Option<TheDependencyBand>,
    /// **The *Dependencies* heading is drawn only where the band is under it**
    /// (must-do 15).
    pub the_dependencies_group_is_drawn: bool,

    // The foot.
    pub name_this_job: Prose,
    pub the_job_name: Verbatim,
    pub where_the_name_shows: Prose,
    pub download_as_they_render: Prose,
    pub the_download_is_on: bool,
    /// **`Design/Render Settings § How the three sit together`: one control
    /// under the switch carries the summary and opens the panel** — *EXR and
    /// PNG — 600 of 720 files, to ~/Renders/kitchen*. Absent before a read and
    /// while the switch is off.
    pub the_format_summary: Option<ASentence>,
    /// **Before a read, where the files go is still chosen** (`§ Submitting`),
    /// so the control reads *Download folder — the folder*. Absent after a read,
    /// where the summary above carries the folder, and while the switch is off.
    pub where_they_go: Option<ASentence>,
    pub the_format_list_is_not_offered: Option<Prose>,
    /// **`§ Submitting`: the switch is on and Smoozy can name no folder** —
    /// nothing chosen for this job, no standing folder, and `Desktop app and
    /// delivery` W5's route proposing none. **`Download` R1's arrangement is
    /// then refused with *there is nowhere to put it*, and this is what says
    /// so** — the drawing's `res-nofolder`. Absent wherever a folder can be
    /// named and while the switch is off.
    pub the_frames_have_nowhere_to_go: Option<TheNowhereLine>,
    /// **`§ Submitting`: while the dependency check's uploads for this file on
    /// this server are running or waiting, one line directly above Render** —
    /// *Uploading dependencies: 4 of 12 files.* — a link that opens the check.
    /// **Render stays available**: submitting starts no transfer
    /// (`Specs/Rendering`), so the line says what is happening and holds
    /// nothing. Absent where nothing of the check is going. The Creator's note
    /// 6 on 0.1.8: *"there should be some indication of that where the user
    /// would start the render job."*
    pub the_dependencies_uploading: Option<ASentence>,
    pub render_one_frame_first: Prose,
    pub the_first_frame_is_asked_for: bool,
    pub the_first_frame_is_answered_in_queue: Option<Prose>,
    /// **`§ Submitting`: where every job the Render control would send has one
    /// frame** — one scene whose range is one frame, or several scenes of one
    /// frame each — *Render one frame first* is greyed and cannot be pressed,
    /// because the check has nothing to save. **It keeps its setting**:
    /// the saved submission keeps the choice; the page reads off while
    /// greyed and restores the choice when the range grows. What is sent does not
    /// change (`Rendering` S13). The drawing's `res-oneframe`; the Creator's
    /// nineteenth notes, note 8, 2026-10-27.
    pub the_first_frame_switch_is_greyed: bool,
    pub telegram: Prose,
    /// **`§ How the three sit together`: *"Where Telegram is switched off it
    /// is not there at all."*** The line is drawn only where the record says
    /// Telegram is on.
    pub the_telegram_line_is_drawn: bool,
    pub the_reporting_choices: Vec<AChoice>,
    pub the_reporting_choice: Verbatim,
    /// **What the closed control says** — the row picked, or *A picture
    /// every* where the fourth is (`§ Submitting`, 2026-10-19).
    pub the_reporting_control_reads: Prose,
    /// **The minutes field, drawn only while *A picture every few minutes* is
    /// picked**: the minutes that stood, 5 until the user types another.
    pub the_minutes_between_pictures: Option<Verbatim>,
    /// The word after the field, and the field's own name.
    pub minutes: Prose,
    pub minutes_between_pictures: Prose,
    pub the_render_control: ASentence,
    pub submitting_is_offered: bool,
    /// **Whether this page's submission is running**, so *Render* carries
    /// Design's *on the control* mark for its length — `Design/Whole app §
    /// Anything that isn't instant shows that it's underway`: *"the control the
    /// user pressed is the thing that is working and everything else on the
    /// screen still works … It is not dimmed."* It is the same fact
    /// `submitting_is_offered` already withdraws the control on, handed over
    /// so the page can tell *working* from *not offered*; on step 10c's eighth
    /// pass *Render* went grey for the twelve seconds a submission took and
    /// said nothing else. Added 2026-09-27, eighth-pass fix piece 5.
    pub the_submission_is_underway: bool,
    /// **Which of `State, identity and recovery` §8.7's durable subjects the
    /// machine's own words are asked for**, so that the page names none of them
    /// itself. The read failure's raw text is a server's.
    pub the_machines_words_subject: Verbatim,

    // The strip, while the panel is collapsed.
    pub the_strip_title: Words,
    pub the_strip_reading: Option<Words>,
}

// ════════════════════════════════════════════════════════════════════════════
// The commands
// ════════════════════════════════════════════════════════════════════════════

/// **The whole panel, in one answer.**
///
/// **A command rather than an event, and P6 is not being bent.** P6 is about a
/// module's *results*; this is the page asking for the facts it needs before it
/// can draw anything, which is the shape `the_rail`, `the_all_servers_screen`
/// and `the_servers_own_screen` already have. **A page that comes back asks
/// again** (P8).
#[tauri::command]
pub fn the_render_settings_panel<R: tauri::Runtime>(
    app: tauri::AppHandle<R>,
    started: State<'_, Mutex<Started>>,
    server: String,
    file: Vec<u8>,
) -> TheRenderSettings {
    // **`Telegram` V1a's stored default and the switch, and the standing
    // folder, read first and let go of** before the run's own lock is taken
    // (fix-here item 9). A record that does not answer stores no default, and
    // the first of the three is used.
    let file = the_path_as_it_is_keyed(&file);
    let mut record = the_record_now(&started, &server);
    record.the_check = the_check_is_read_now(&started, &server, &file);
    // **The check's uploads of this file, as Transfers lists them** — the
    // Creator's note 6 on 0.1.8. Read before the run's lock, like the rest.
    record.the_checks_uploads = crate::surface::transfers::the_checks_uploads_on(&app, &server, &file);
    // **D5's reading, taken with the page rather than remembered** — the
    // server row's own, in the state the row is drawn in. Read before the run's
    // lock is taken, like everything else this command reads.
    let hardware = the_hardware_is_read_now(&app, &started, &server);
    let a_connection_is_live = app
        .try_state::<std::sync::Arc<crate::servers::a_period::ThePeriodsThisRun>>()
        .and_then(|periods| periods.the_sessions_of(&server))
        .is_some_and(|point| point.a_connection_is_live());
    let held = the_run_holds().lock();
    match held {
        Ok(mut held) => {
            // **`Server Setup` B12: B1's list is period-held**, so with no
            // live connection there is none to draw.
            if !a_connection_is_live {
                the_period_ended_for(&mut held, &server);
            }
            if let Some(hardware) = hardware {
                the_hardware_stands_at(&mut held, &server, hardware);
            }
            the_panel_starting_from(&held, &server, &file, &record)
        }
        // **L8a rather than an emptied page.** A lock this run cannot take is
        // a read that did not answer, and the page says so.
        Err(_) => {
            let mut panel = the_panel(&WhatTheRunHolds::default(), &server, &file);
            panel.the_band = vec![plainly(words::THE_RECORD_WOULD_NOT_READ)];
            panel
        }
    }
}

/// [`WhatTheRecordHolds`], read with the application's lock held only for the
/// read.
fn the_record_now(started: &State<'_, Mutex<Started>>, server: &str) -> WhatTheRecordHolds {
    match started.lock() {
        Ok(started) => {
            use crate::surface::said_once::{the_warning_is_switched_off, WhetherItIsSwitchedOff};
            let mut record = WhatTheRecordHolds::read_now(started.record.as_ref());
            record.the_servers_name = crate::surface::files::what_the_server_is_called(&started, server);
            // **`Rendering` W4a's one identity, asked of the shared service
            // with `Rendering`'s own name** — never a direct read of the row.
            record.the_dependency_warning = match the_warning_is_switched_off(
                &started,
                THE_SUPPRESSION_ROW.0,
                THE_SUPPRESSION_IDENTITY,
            ) {
                WhetherItIsSwitchedOff::Yes => TheSwitchReads::SwitchedOff,
                WhetherItIsSwitchedOff::No => TheSwitchReads::NotSwitchedOff,
                WhetherItIsSwitchedOff::TheReadFailed => TheSwitchReads::TheReadFailed,
            };
            // **The row's own layout**, the one Submit composes the job's paths
            // from (`the_act_as_the_page_composed_it`), so the line above
            // *Output files* and the job's real folder read one source.
            let row = started
                .record
                .as_ref()
                .and_then(|record| crate::record::server_rows::all(record.connection()).ok())
                .and_then(|rows| rows.into_iter().find(|row| row.local_server_identifier == server));
            record.the_layout = row
                .as_ref()
                .and_then(crate::servers::installation::the_layout_recorded_for);
            record.the_servers_fallback_folder = row.and_then(|row| row.chosen_fallback_folder);
            record
        }
        Err(_) => WhatTheRecordHolds::read_now(None),
    }
}

/// **Which setting an answer is for.** H34: a closed request type, validated
/// before anything is touched.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum WhichSetting {
    FrameRange,
    SampleCount,
    ResolutionPercentage,
    EnginePick,
    Device,
    VersionPick,
    JobName,
    DownloadAsTheyRender,
    TheFirstFrameCheck,
    WhatItReports,
    /// **The minutes between pictures**, for *A picture every few minutes*
    /// (`Specs/Telegram`, *Fewer pictures, on a clock*, 2026-10-19): refused
    /// in place where they are not whole minutes from 1 to 60.
    TelegramMinutes,
    /// **`Rendering`'s Simplify switch on a re-submitted failed job**, held in
    /// `Render Settings`' own store as `SimplifySwitch`. Must-do 9 of the step
    /// 10a round 1 audit of these pieces: the page sent `simplify` and this list
    /// had no such value, so the call was refused before it ran.
    Simplify,
    /// **Simplify's three values, each its own entry** (`Rendering` A1d,
    /// `Render Settings` J3): the two typed boxes and the Texture Limit pick.
    /// The store held them and `Rendering`'s two boxes were built, and nothing
    /// on the page could reach any of them until 2026-10-03 (an owed fault
    /// folded into the Creator's notes on the test copy, piece 3).
    SimplifySubdivision,
    SimplifyChildParticles,
    SimplifyTextureLimit,
    /// Everything the page in hand is showing — `§ The file's value against the
    /// user's`, *one control hands everything back at once*.
    Everything,
}

/// **An answer the user gave.** The value arrives as text and **the box refuses
/// before anything reaches the store** (J11), which is why the parse is here
/// and not on the page.
#[tauri::command]
pub fn an_answer_was_given(
    queue: State<'_, std::sync::Arc<TheOneQueue>>,
    server: String,
    file: Vec<u8>,
    which: WhichSetting,
    text: String,
) {
    let file = the_path_as_it_is_keyed(&file);
    let Ok(mut held) = the_run_holds().lock() else {
        the_queue_is_told(&queue, an_act_said(&server, &file, "the-record", vec![plainly(words::THE_RECORD_WOULD_NOT_READ)]));
        return;
    };
    let said = what_the_answer_says(&mut held, &server, &file, which, &text);
    the_queue_is_told(&queue, an_act_said(&server, &file, the_box_of(which), said));
}

/// **An answer, taken or refused, and what its box says** — nothing where it
/// was taken. The whole of [`an_answer_was_given`] but the queue, so it can be
/// tested.
fn what_the_answer_says(
    held: &mut WhatTheRunHolds,
    server: &str,
    file: &str,
    which: WhichSetting,
    text: &str,
) -> ASentence {
    // **`Rendering` A1d's two boxes refuse in `Render Settings` F1's shape,
    // the line stating the range alone**, and nothing becomes an entry.
    if let Some(range) = what_a_simplify_box_refuses(which, text) {
        return what_the_range_alone_says(range);
    }
    // **The minutes refuse in the same shape, the range alone** —
    // `Design/Render Settings § Submitting`: *"the foot's third line says Whole
    // minutes, 1 to 60."* — and the minutes that stood go on standing.
    if which == WhichSetting::TelegramMinutes && reporting::the_minutes_typed(text).is_err() {
        return vec![plainly(words::WHOLE_MINUTES_1_TO_60)];
    }
    // **A list with gaps, where the file's regular output is a video**: refused
    // the way any unusable range is, and nothing becomes an entry, so the field
    // keeps its last good value.
    if a_gapped_list_on_a_video(held, server, file, which, text) {
        return vec![plainly(words::A_VIDEO_CANT_SKIP_FRAMES)];
    }
    match the_answer_is_taken(held, server, file, which, text) {
        Some(refusal) => what_the_refusal_says(&refusal),
        None => Vec::new(),
    }
}

/// **`Design/Render Settings`, *Where the file writes a video, the range takes
/// one unbroken run*** (added 2026-10-17, the eleventh notes, the video-range piece; the
/// spec's *The frame range*): **where the file's regular output is a video, a
/// typed list with gaps is refused.** A video holds frames one after another
/// and cannot skip any — started with a list, Blender wrote a video holding one
/// frame (`Rendering` L17a). **A step is not a gap**: a first, last and step is
/// accepted, and so is a list that comes to one evenly stepped run (`1-10,
/// 11-20` is one run; `1, 3, 5` is the run 1 to 5 at step 2), the same rule
/// the machine renders a video job by, so the page never refuses what the
/// machine would render.
///
/// **What the regular output is comes from the reading**: the halves of every
/// scene ticked (one with several ticked is enough, the group range standing
/// for all of them), and nothing before a read, when it is not known — the
/// machine refuses a gapped video job in words for whatever still arrives.
fn a_gapped_list_on_a_video(
    held: &WhatTheRunHolds,
    server: &str,
    file: &str,
    which: WhichSetting,
    text: &str,
) -> bool {
    if which != WhichSetting::FrameRange {
        return false;
    }
    let Ok(FrameForm::List(frames)) = frames::parse_the_frame_field(text) else {
        return false;
    };
    let evenly_stepped = match frames.get(1).zip(frames.first()) {
        Some((second, first)) => {
            let step = second - first;
            frames.windows(2).all(|pair| pair[1] - pair[0] == step)
        }
        None => true,
    };
    if evenly_stepped {
        return false;
    }
    the_outputs_the_box_reads(held, server, file).is_some_and(|halves| {
        halves.iter().any(|half| {
            half.destinations.iter().any(|destination| {
                matches!(destination.kind, smoozy_job_files::reading::DestinationKind::TheRegularOutputPath)
                    && destination.format.as_ref().is_some_and(|format| format.is_video_shaped)
            })
        })
    })
}

/// **A cross, or the hand-everything-back control.** J5: clearing is deletion,
/// and there is no *cleared* marker.
#[tauri::command]
pub fn a_value_is_handed_back(
    queue: State<'_, std::sync::Arc<TheOneQueue>>,
    server: String,
    file: Vec<u8>,
    which: WhichSetting,
) {
    let file = the_path_as_it_is_keyed(&file);
    let Ok(mut held) = the_run_holds().lock() else {
        the_queue_is_told(&queue, an_act_said(&server, &file, "the-record", vec![plainly(words::THE_RECORD_WOULD_NOT_READ)]));
        return;
    };
    let refusal = the_value_is_handed_back(&mut held, &server, &file, which);
    let event = match &refusal {
        Some(refusal) => an_act_said(
            &server,
            &file,
            the_box_of(which),
            what_the_refusal_says(refusal),
        ),
        None => an_act_said(&server, &file, the_box_of(which), Vec::new()),
    };
    the_queue_is_told(&queue, event);
}

/// **A scene box.** J3's *scenes ticked*, by name with J7's provenance.
#[tauri::command]
pub fn a_scene_is_ticked(
    queue: State<'_, std::sync::Arc<TheOneQueue>>,
    server: String,
    file: Vec<u8>,
    scene: String,
    on: bool,
) {
    let file = the_path_as_it_is_keyed(&file);
    let Ok(mut held) = the_run_holds().lock() else {
        the_queue_is_told(&queue, an_act_said(&server, &file, "the-record", vec![plainly(words::THE_RECORD_WOULD_NOT_READ)]));
        return;
    };
    let key = FileKey {
        server: an_identity(&server),
        path: file.clone(),
    };
    let name = ExactName::as_the_reading_produced_it(&scene);
    if on {
        // **J7: a name-shaped answer is held against the name's exact bytes
        // plus a recorded provenance**, and the page offers no way to type a
        // scene name — so where no reading stands there is nothing to tick and
        // nothing is written.
        let Some(provenance) = the_provenance(&held, &key) else {
            the_queue_is_told(&queue, an_act_said(&server, &file, "scene", Vec::new()));
            return;
        };
        held.store.tick_a_scene(key, NamedAnswer { name, provenance });
    } else {
        held.store.untick_a_scene(key, &name);
    }
    the_queue_is_told(&queue, an_act_said(&server, &file, "scene", Vec::new()));
}

/// **Every scene at once** — `§ Scenes`: *All scenes* is not a fourth scene,
/// it is a different act that submits one job per scene.
#[tauri::command]
pub fn every_scene_is_ticked(
    queue: State<'_, std::sync::Arc<TheOneQueue>>,
    server: String,
    file: Vec<u8>,
    on: bool,
) {
    let file = the_path_as_it_is_keyed(&file);
    let Ok(mut held) = the_run_holds().lock() else {
        the_queue_is_told(&queue, an_act_said(&server, &file, "the-record", vec![plainly(words::THE_RECORD_WOULD_NOT_READ)]));
        return;
    };
    let key = FileKey {
        server: an_identity(&server),
        path: file.clone(),
    };
    let names: Vec<ExactName> = the_standing_scenes(&held, &server, &file);
    let provenance = the_provenance(&held, &key);
    for name in names {
        match (on, provenance.clone()) {
            (true, Some(provenance)) => held
                .store
                .tick_a_scene(key.clone(), NamedAnswer { name, provenance }),
            (true, None) => {}
            (false, _) => held.store.untick_a_scene(key.clone(), &name),
        }
    }
    the_queue_is_told(&queue, an_act_said(&server, &file, "scene", Vec::new()));
}

/// **A view-layer box.** V4's pin — per layer name with J7's provenance.
#[tauri::command]
pub fn a_view_layer_is_pinned(
    queue: State<'_, std::sync::Arc<TheOneQueue>>,
    server: String,
    file: Vec<u8>,
    layer: String,
    on: bool,
) {
    let file = the_path_as_it_is_keyed(&file);
    let Ok(mut held) = the_run_holds().lock() else {
        the_queue_is_told(&queue, an_act_said(&server, &file, "the-record", vec![plainly(words::THE_RECORD_WOULD_NOT_READ)]));
        return;
    };
    let key = FileKey {
        server: an_identity(&server),
        path: file.clone(),
    };
    let Some(scene) = the_scene_in_hand(&held, &server, &file) else {
        // **A pin needs a name only a reading supplies** (J3), so there is
        // nothing to write and nothing is claimed.
        the_queue_is_told(&queue, an_act_said(&server, &file, "view-layer", Vec::new()));
        return;
    };
    let Some(provenance) = the_provenance(&held, &key) else {
        the_queue_is_told(&queue, an_act_said(&server, &file, "view-layer", Vec::new()));
        return;
    };
    held.store.pin(
        key,
        scene,
        NamedAnswer {
            name: ExactName::as_the_reading_produced_it(&layer),
            provenance,
        },
        on,
    );
    the_queue_is_told(&queue, an_act_said(&server, &file, "view-layer", Vec::new()));
}

/// **A pinned box handed back** — the cross beside it, which hands back to the
/// file's own answer at the frame the boxes are read at.
#[tauri::command]
pub fn a_pin_is_handed_back(
    queue: State<'_, std::sync::Arc<TheOneQueue>>,
    server: String,
    file: Vec<u8>,
    layer: String,
) {
    let file = the_path_as_it_is_keyed(&file);
    let Ok(mut held) = the_run_holds().lock() else {
        the_queue_is_told(&queue, an_act_said(&server, &file, "the-record", vec![plainly(words::THE_RECORD_WOULD_NOT_READ)]));
        return;
    };
    let key = FileKey {
        server: an_identity(&server),
        path: file.clone(),
    };
    let Some(scene) = the_scene_in_hand(&held, &server, &file) else {
        the_queue_is_told(&queue, an_act_said(&server, &file, "view-layer", Vec::new()));
        return;
    };
    let _ = held.store.clear_one(
        Where::AScene { file: key, scene },
        &Setting::Pin(ExactName::as_the_reading_produced_it(&layer)),
    );
    the_queue_is_told(&queue, an_act_said(&server, &file, "view-layer", Vec::new()));
}

/// **The file was pointed at** — `File Browser`'s own act, and it hands over
/// the two things this page cannot read for itself: which file, and **the
/// saved-with version read without opening it**, which `Render Settings` B2
/// names as the automatic match's one source.
///
/// **Where nothing is handed over, the reading is taken here, through
/// `File Browser`'s own** ([`crate::surface::files::the_saved_with_version_of`],
/// `File Browser` L7 to L10) — the row's tag fills in behind the listing, so a
/// press that came before it arrived hands none. It reads with someone waiting
/// (`Server protocol and transfers` §1.1 item 2), and the page opens when it
/// answers. **Where it is blank the page draws `§ When there is no version to
/// pick`'s second case** — a file that does not say which Blender saved it —
/// which is one of Design's three and is not a failure.
///
/// *Changed 2026-10-01, step 10c's twelfth-pass fix piece 6b.* **The case this
/// was first written for** is a version handed by the listing; **the case it
/// now also meets** is a press before the listing's tag arrived. Until then no
/// listing ever carried one, so this page said every file did not say which
/// Blender saved it, and no file was matched without a pick.
///
/// **`async`, so the reading waits on the machine off the drawing thread** (P5).
#[tauri::command]
pub async fn the_file_was_pointed_at<R: tauri::Runtime>(
    app: tauri::AppHandle<R>,
    server: String,
    file: Vec<u8>,
    saved_with: String,
) {
    let key = the_path_as_it_is_keyed(&file);
    let point = app
        .try_state::<std::sync::Arc<crate::servers::a_period::ThePeriodsThisRun>>()
        .and_then(|periods| periods.the_sessions_of(&server))
        .filter(crate::servers::sessions::ThisServersSessions::a_connection_is_live);
    let saved_with = the_saved_with_version_to_hold(point.as_ref(), &server, &file, &saved_with).await;
    let Some(queue) = app.try_state::<std::sync::Arc<TheOneQueue>>() else {
        return;
    };
    let Ok(mut held) = the_run_holds().lock() else {
        the_queue_is_told(
            &queue,
            an_act_said(&server, &key, "the-record", vec![plainly(words::THE_RECORD_WOULD_NOT_READ)]),
        );
        return;
    };
    the_saved_with_version_lands(&mut held, &server, &key, &saved_with);
    // **`File Browser` §12: a pick in hand takes the file** — the setup a
    // re-submission held waits for exactly this, and is written under the key
    // that now exists (`Render Settings` Y15's no-key route). An ordinary
    // choice of a file, with no pick in hand, changes nothing here.
    crate::a_new_job_from_an_old_one::the_pick_lands(&mut held, &server, &key);
    the_queue_is_told(&queue, an_act_said(&server, &key, "the-file", Vec::new()));
}

/// **The version handed, or `File Browser`'s reading where none was** — the
/// half of [`the_file_was_pointed_at`] that asks the machine. Blank where no
/// connection is live or the reading is blank (`File Browser` L8).
pub async fn the_saved_with_version_to_hold<C: crate::servers::sessions::AHeldConnection>(
    point: Option<&crate::servers::sessions::ThisServersSessions<C>>,
    server: &str,
    file: &[u8],
    handed: &str,
) -> String {
    if !handed.trim().is_empty() {
        return handed.trim().to_string();
    }
    match point {
        Some(point) => crate::surface::files::the_saved_with_version_of(point, server, file, true)
            .await
            .unwrap_or_default(),
        None => String::new(),
    }
}

/// **Where `File Browser`'s reading of the saved-with version lands** — the one
/// place this page and the dependency check resolve `Render Settings` B2 from.
/// Blank holds nothing, so the page draws the file that does not say which
/// Blender saved it.
pub fn the_saved_with_version_lands(held: &mut WhatTheRunHolds, server: &str, file: &str, saved_with: &str) {
    if saved_with.trim().is_empty() {
        held.the_saved_with.remove(&keyed(server, file));
    } else {
        held.the_saved_with
            .insert(keyed(server, file), saved_with.trim().to_string());
    }
}

/// **Reading the file** — `Design/Render Settings § Reading the file`, and
/// `Render Settings` Q4's Pre-load, **run since step 10c's dependency wave
/// piece 3**.
///
/// **The press asks the machine over the server's own point**: one request, *a
/// reading of a `.blend`*, taken by the file-inspection worker `Render
/// Settings` W1 declares, and the document read back through the worker's
/// captured stream — [`crate::reading_the_file`], which drives `Render
/// Settings`' own `pre_load` whole. **The act runs off the drawing thread**
/// (P5); while it runs the control carries the underway mark and nothing else
/// on the page moves (`Render Settings` Q5), and the page is told when it
/// ends. **A second press while one runs joins it** rather than starting a
/// second read (`Render Settings` Q5 again, `§ Reading the file`'s *"Pressing it twice does
/// nothing worth drawing"*).
///
/// *(Until this piece the press answered
/// [`crate::surface::files::A_CONNECTION_IS_NOT_BUILT`] and recorded no ending,
/// because nothing drove the operation and the machine did not run one.)*
///
/// **Where it cannot start, it says why in words the page already has**: no
/// live connection is `Render Settings` Q8's first ending, the channel's own, and lands through
/// [`the_read_gave_nothing`] like any other ending; a server with no placement
/// recorded has no package area to ask, and says Smoozy is not set up there.
#[tauri::command]
pub fn the_file_is_read<R: tauri::Runtime>(
    app: tauri::AppHandle<R>,
    started: State<'_, Mutex<Started>>,
    queue: State<'_, std::sync::Arc<TheOneQueue>>,
    server: String,
    file: Vec<u8>,
) {
    let file = the_path_as_it_is_keyed(&file);
    let point = app
        .try_state::<std::sync::Arc<crate::servers::a_period::ThePeriodsThisRun>>()
        .and_then(|periods| periods.the_sessions_of(&server))
        .filter(crate::servers::sessions::ThisServersSessions::a_connection_is_live);
    let Some(point) = point else {
        let said = the_read_landed(
            &app,
            &server,
            &file,
            crate::reading_the_file::WhatTheReadCameTo::ThePreLoad {
                went: smoozy_render_settings::acting::pre_load::HowThePreLoadWent::NoReading(
                    Ending::TheMachineCouldNotBeReached,
                ),
                operation: None,
            },
        );
        the_queue_is_told(&queue, an_act_said(&server, &file, "read", said));
        return;
    };
    // **The package area is the one the server row records** (`Job files and
    // outputs` L4a), read with the application's lock held for the read alone.
    let package_area = started
        .lock()
        .ok()
        .and_then(|started| {
            started
                .record
                .as_ref()
                .and_then(|record| crate::record::server_rows::all(record.connection()).ok())
        })
        .and_then(|rows| rows.into_iter().find(|row| row.local_server_identifier == server))
        .and_then(|row| match crate::servers::installation::where_the_package_area_is(&row) {
            crate::servers::installation::WhereThePackageAreaIs::At(area) => Some(area),
            _ => None,
        });
    let Some(package_area) = package_area else {
        the_queue_is_told(
            &queue,
            an_act_said(&server, &file, "read", vec![plainly(words::NOT_SET_UP_ON_THIS_SERVER)]),
        );
        return;
    };
    match the_run_holds().lock() {
        Ok(mut held) => {
            held.reading_now.insert(keyed(&server, &file), true);
        }
        Err(_) => {
            the_queue_is_told(
                &queue,
                an_act_said(&server, &file, "read", vec![plainly(words::THE_RECORD_WOULD_NOT_READ)]),
            );
            return;
        }
    }
    the_queue_is_told(&queue, an_act_said(&server, &file, "read", Vec::new()));
    let queue = std::sync::Arc::clone(queue.inner());
    std::thread::spawn(move || {
        let runtime = tokio::runtime::Builder::new_current_thread().enable_all().build();
        let came_to = match runtime {
            Ok(runtime) => runtime.block_on(crate::reading_the_file::the_file_is_read(
                crate::reading_the_file::ThisServersMachine {
                    sessions: &point,
                    package_area: &package_area,
                    server: &server,
                },
                the_run_holds(),
                &server,
                &file,
            )),
            Err(_) => crate::reading_the_file::WhatTheReadCameTo::ThePreLoad {
                went: smoozy_render_settings::acting::pre_load::HowThePreLoadWent::NoReading(
                    Ending::TheMachineCouldNotBeReached,
                ),
                operation: None,
            },
        };
        let said = the_read_landed(&app, &server, &file, came_to);
        the_queue_is_told(&queue, an_act_said(&server, &file, "read", said));
    });
}

/// **Where what a press came to lands, and what the read line says.**
///
/// - **A reading stored** (`Render Settings` Q6): a failure the page was
///   showing goes, the underway mark goes, and **the one act line `Render
///   Settings` §12 declares** is written through the diary service with `Render
///   Settings` Q18's own stamp. Nothing is said on the line: the page itself now holds the
///   file's values.
/// - **A reading discarded on landing** (`Render Settings` Q17): nothing is written and nothing
///   is said, and the page is where it was with the read offered again.
/// - **No reading** (`Render Settings` Q8): [`the_read_gave_nothing`], which holds the ending
///   beside the page and hands the sentence to the diary once.
/// - **No four values to start under**: said on the read line in the sentence
///   this page already uses for that same reading at submission, and no ending
///   is recorded, because no read ran.
fn the_read_landed<R: tauri::Runtime>(
    app: &tauri::AppHandle<R>,
    server: &str,
    file: &str,
    came_to: crate::reading_the_file::WhatTheReadCameTo,
) -> ASentence {
    // **The application's lock before the run's**, the order every act on
    // this page takes them in.
    let started = app.try_state::<Mutex<Started>>();
    let started = started.as_ref().and_then(|started| started.lock().ok());
    let Ok(mut held) = the_run_holds().lock() else {
        return vec![plainly(words::THE_RECORD_WOULD_NOT_READ)];
    };
    the_read_lands_in(&mut held, started.as_deref(), server, file, came_to)
}

/// **[`the_read_landed`]'s whole act, with both locks already taken** — split
/// out so a test lands a read exactly as the press does.
pub(crate) fn the_read_lands_in(
    held: &mut WhatTheRunHolds,
    started: Option<&Started>,
    server: &str,
    file: &str,
    came_to: crate::reading_the_file::WhatTheReadCameTo,
) -> ASentence {
    use crate::reading_the_file::WhatTheReadCameTo;
    use smoozy_render_settings::acting::pre_load::HowThePreLoadWent;
    use smoozy_render_settings::file_half::Landed;

    let mark = keyed(server, file);
    held.reading_now.remove(&mark);
    match came_to {
        WhatTheReadCameTo::TheFourValuesCouldNotBeRead => {
            vec![plainly(words::THE_FILE_ON_THE_SERVER_COULD_NOT_BE_READ)]
        }
        // **The one case beside Q8's four: the server answered and ran no
        // Blender.** Held and handed to the diary the way Q8's endings are.
        WhatTheReadCameTo::TheServerRanNoBlender => {
            let name = started
                .and_then(|started| crate::surface::files::what_the_server_is_called(started, server))
                .unwrap_or_default();
            let _ = the_read_gave_nothing(
                held,
                server,
                file,
                AReadThatGaveNothing::TheServerRanNoBlender,
                &TheDiaryInHand {
                    diary: started.and_then(|started| started.diary.as_ref()),
                    run: started.and_then(|started| started.run.as_ref()),
                    the_servers_name: &name,
                },
            );
            Vec::new()
        }
        WhatTheReadCameTo::ThePreLoad { went, operation } => match went {
            HowThePreLoadWent::AReading { landed: Landed::Stored, the_diary_act } => {
                held.the_read_that_gave_nothing.remove(&mark);
                if let (Some(act), Some(operation), Some(started)) = (the_diary_act, operation, started.as_ref()) {
                    let key = FileKey {
                        server: an_identity(server),
                        path: file.to_string(),
                    };
                    let stored_at = held
                        .readings
                        .standing(&key)
                        .map(|reading| reading.stored_at.taken_at());
                    if let (Some(service), Some(run), Some(stored_at)) =
                        (started.diary.as_ref(), started.run.as_ref(), stored_at)
                    {
                        let diary = crate::diary_service::ModulesDiary {
                            service,
                            declarations: &started.declarations,
                            run: run.clone(),
                            module: crate::reading_the_file::THE_MODULE,
                        };
                        let _ = diary.write(&act.line(
                            smoozy_render_settings::diary::Concerning {
                                run: run.clone(),
                                server_row: an_identity(server),
                                the_users_label_for_the_machine: crate::surface::files::what_the_server_is_called(
                                    started, server,
                                )
                                .unwrap_or_default(),
                                operation,
                            },
                            stored_at,
                        ));
                    }
                }
                Vec::new()
            }
            HowThePreLoadWent::AReading { .. } | HowThePreLoadWent::TheReadIsNotOffered => Vec::new(),
            HowThePreLoadWent::NoReading(ending) => {
                let name = started
                    .as_ref()
                    .and_then(|started| crate::surface::files::what_the_server_is_called(started, server))
                    .unwrap_or_default();
                let _ = the_read_gave_nothing(
                    held,
                    server,
                    file,
                    ending,
                    &TheDiaryInHand {
                        diary: started.as_ref().and_then(|started| started.diary.as_ref()),
                        run: started.as_ref().and_then(|started| started.run.as_ref()),
                        the_servers_name: &name,
                    },
                );
                Vec::new()
            }
        },
    }
}

/// **Where a real read's ending lands, and the one place it is handed to the
/// diary** — `Render Settings` §12's declaration, through `Desktop app and
/// delivery` §16.8 M32's second limb (the upstream must-do of the step 10a round 1 audit of
/// these pieces).
///
/// **Q9: a failed read changes nothing**, so the ending is held beside the page
/// and no record is raised. **What the user met still reaches the diary, once
/// for this request**: the sentence the page composes for this ending — its own
/// words, before the typography point — with the server row, the user's label
/// for it, and the file's recorded path. **It is called where the request's
/// ending first lands and never on a redraw**, which is what makes it once per
/// presentation. A reading is not an ending and hands nothing over.
///
/// **Nothing under `App/` drives a read yet**, so the one caller outside tests
/// is the read the connection step builds; this is where it lands.
pub fn the_read_gave_nothing(
    held: &mut WhatTheRunHolds,
    server: &str,
    file: &str,
    ending: impl Into<AReadThatGaveNothing>,
    the_diary: &TheDiaryInHand<'_>,
) -> crate::presentation::the_two_hooks::TheDiaryWasHanded {
    let ending = ending.into();
    use crate::presentation::the_two_hooks::{
        the_diary_is_handed_a_sentence_with_no_record, APresentationWithNoRecord, TheDiaryWasHanded,
        WhoseSentenceItIs,
    };
    let sentence = the_read_failures_sentence(&ending);
    held.the_read_that_gave_nothing.insert(keyed(server, file), ending);
    let Some(sentence) = sentence else {
        return TheDiaryWasHanded::Nothing {
            reported: "a reading is not an ending".to_string(),
        };
    };
    let server_row = an_identity(server);
    the_diary_is_handed_a_sentence_with_no_record(
        the_diary.diary,
        the_diary.run,
        &APresentationWithNoRecord {
            server_row: &server_row,
            the_users_label: the_diary.the_servers_name,
            the_files_recorded_path: file,
        },
        WhoseSentenceItIs::TheComposersOwn(sentence),
    )
}

/// **What a request's ending is handed to the diary with**: the diary service,
/// this run's identity, and what the user calls the server. Read from
/// [`Started`] by whoever drives the read.
pub struct TheDiaryInHand<'a> {
    pub diary: Option<&'a crate::diary_service::DiaryService>,
    pub run: Option<&'a Identity>,
    pub the_servers_name: &'a str,
}

/// **A read failure cleared** — the cross on the one failure shape. Q9 already
/// makes the failure change nothing, so clearing it changes nothing either.
#[tauri::command]
pub fn the_read_failure_is_cleared(
    queue: State<'_, std::sync::Arc<TheOneQueue>>,
    server: String,
    file: Vec<u8>,
) {
    let file = the_path_as_it_is_keyed(&file);
    let Ok(mut held) = the_run_holds().lock() else {
        the_queue_is_told(&queue, an_act_said(&server, &file, "the-record", vec![plainly(words::THE_RECORD_WOULD_NOT_READ)]));
        return;
    };
    held.the_read_that_gave_nothing.remove(&keyed(&server, &file));
    the_queue_is_told(&queue, an_act_said(&server, &file, "read", Vec::new()));
}

/// **The missing-assets warning switched off** — `Rendering` W4a's one
/// suppression identity, which covers the whole band in all four of its
/// statements. The write is the shared service's and the identity is that
/// document's.
///
/// **It goes through [`crate::surface::said_once::a_warning_was_switched_off`]**,
/// which checks the declaration before it writes (`Desktop app and delivery`
/// G5), **and a write that did not go down says so** on the band rather than
/// being taken as done — must-do 2 of the step 10a round 1 audit of these
/// pieces. *(Until that round this wrote the row directly and threw the answer
/// away.)*
#[tauri::command]
pub fn the_missing_assets_warning_is_switched_off(
    started: State<'_, Mutex<Started>>,
    queue: State<'_, std::sync::Arc<TheOneQueue>>,
    server: String,
    file: Vec<u8>,
) {
    let file = the_path_as_it_is_keyed(&file);
    let Ok(started) = started.lock() else {
        the_queue_is_told(&queue, an_act_said(&server, &file, "the-record", vec![plainly(words::THE_RECORD_WOULD_NOT_READ)]));
        return;
    };
    let saved = crate::surface::said_once::a_warning_was_switched_off(
        &started,
        THE_SUPPRESSION_ROW.0,
        THE_SUPPRESSION_IDENTITY,
    );
    let said = if saved {
        Vec::new()
    } else {
        vec![plainly(words::THE_SETTING_DID_NOT_SAVE)]
    };
    the_queue_is_told(&queue, an_act_said(&server, &file, "dependencies", said));
}

/// **Submitting** — `Design/Render Settings § Submitting`, and `Whole app.md`'s
/// ruling that submitting sits at the bottom of this panel, which is why the
/// control is this piece's although `Rendering` has no Design document.
///
/// **It does the whole act, since step 10c piece 6**: the page's answers
/// composed into one payload per scene-job here, and the act performed by
/// [`crate::submitting`] off the drawing thread (P5) — `Rendering`'s
/// identities and manifest, the provisional record with its kept setup,
/// `Rendering` S4's ordered unit over the server's own point, and what became
/// of each scene-job, said on the control that asked.
///
/// **`with_the_first_frame_check` is the band's *Render one frame first***:
/// `Rendering` W4's `SubmitWithTheFirstFrameCheck`, which sets the check and
/// submits **as one act** (must-do 10 of the step 10a round 1 audit of these
/// pieces). The check is set before the submission is tried, so it stands on
/// the page whatever the submission answers.
#[tauri::command]
pub fn a_job_is_submitted<R: tauri::Runtime>(
    app: tauri::AppHandle<R>,
    started: State<'_, Mutex<Started>>,
    queue: State<'_, std::sync::Arc<TheOneQueue>>,
    server: String,
    file: Vec<u8>,
    with_the_first_frame_check: Option<bool>,
) {
    let file = the_path_as_it_is_keyed(&file);
    if with_the_first_frame_check == Some(true) {
        if let Ok(mut held) = the_run_holds().lock() {
            let _ = the_answer_is_taken(&mut held, &server, &file, WhichSetting::TheFirstFrameCheck, "on");
        }
    }
    let (row, run, record_holds) = match started.lock() {
        Ok(held_started) => {
            let row = held_started
                .record
                .as_ref()
                .and_then(|record| crate::record::server_rows::all(record.connection()).ok())
                .and_then(|rows| rows.into_iter().find(|row| row.local_server_identifier == server));
            (row, held_started.run.clone(), None::<()>)
        }
        Err(_) => {
            the_queue_is_told(
                &queue,
                an_act_said(&server, &file, "submit", vec![plainly(words::THE_RECORD_WOULD_NOT_READ)]),
            );
            return;
        }
    };
    let _ = record_holds;
    // **What the confirmation names the server**: the name the rest of the
    // app shows for it.
    let the_servers_name = row
        .as_ref()
        .map(crate::surface::servers::what_a_server_is_called)
        .unwrap_or_default();
    let mut record = the_record_now(&started, &server);
    record.the_check = the_check_is_read_now(&started, &server, &file);
    let point = app
        .try_state::<std::sync::Arc<crate::servers::a_period::ThePeriodsThisRun>>()
        .and_then(|periods| periods.the_sessions_of(&server))
        .filter(crate::servers::sessions::ThisServersSessions::a_connection_is_live);
    // **`Job files and outputs` M1b: the card make the manifest carries is the
    // reading the row stands in at the moment of the act**, taken from the row
    // this act already holds rather than from whatever a drawing left behind.
    // **The fifth pass on Vast is why it is taken here as well as with the
    // page**: a submission is the one act that must not read a reading nobody
    // wrote.
    let periods = crate::surface::servers::the_periods_of(&app);
    let hardware = row.as_ref().map(|row| {
        the_hardware_the_row_stands_in(
            row,
            periods
                .as_deref()
                .and_then(|periods| periods.what_the_last_connection_read(&server)),
            crate::surface::servers::what_is_true_of(periods.as_deref(), &server),
        )
    });
    let composed = match (the_run_holds().lock(), row, run, point) {
        (Err(_), ..) | (_, None, ..) | (_, _, None, _) => {
            Err(vec![plainly(words::THE_RECORD_WOULD_NOT_READ)])
        }
        (_, _, _, None) => Err(vec![plainly(words::NOT_SUBMITTED), plainly(words::NO_SERVER_IS_CONNECTED)]),
        (Ok(mut held), Some(row), Some(run), Some(point)) => {
            if let Some(hardware) = hardware {
                the_hardware_stands_at(&mut held, &server, hardware);
            }
            match the_act_as_the_page_composed_it(&held, &server, &file, &record, &row, run) {
                Ok(mut given) => {
                    // **`Server Setup` R1c's readings, taken at the press.**
                    // Held-job piece A, 2026-10-12.
                    given.the_driver_at_submission =
                        crate::servers::the_driver_swap::the_driver_at_submission(&app, &server);
                    held.submitting_now.insert(keyed(&server, &file), true);
                    Ok((given, point))
                }
                Err(said) => Err(said),
            }
        }
    };
    let (given, point) = match composed {
        Ok(ready) => ready,
        Err(said) => {
            the_queue_is_told(&queue, an_act_said(&server, &file, "submit", said));
            return;
        }
    };
    // **Submitting is work under way and not a refusal**, so it is the plain
    // note too.
    the_queue_is_told(&queue, a_submission_said(&server, &file, vec![plainly(words::SUBMITTING)], 1));
    // **`Queue & History` C10: the job is in its server's line from the press
    // of *Submit***, reading *Submitting…* (the runner's ruling on C10) — the
    // Creator's seventh notes, piece Z7. The queue is drawn again at once.
    let in_flight = crate::submitting::an_act_is_begun(&given);
    crate::surface::the_queue::the_queue_is_told_again(&app);

    // **P5: the act runs off the drawing thread**, and what it came to is
    // posted when it ends.
    let queue = std::sync::Arc::clone(queue.inner());
    std::thread::spawn(move || {
        let the_run = app
            .try_state::<std::sync::Arc<crate::downloading::TheRunsTransfers>>()
            .map(|the_run| std::sync::Arc::clone(the_run.inner()));
        let door = TheApplicationsRecord { app: app.clone() };
        let how_many = given.scene_jobs.len();
        let (came_to, named) = match the_run {
            Some(the_run) => {
                // **`Rendering` S10: a several-scene act the connection broke
                // part-way is held for the queue's banner**, with each job it
                // sent named (the 38's piece 2).
                let handed = given.clone();
                let (came_to, named) = crate::submitting::the_act_is_performed_in_flight(
                    given, &point, &the_run, &door, in_flight,
                );
                crate::surface::the_queue::an_act_ended(&handed, &came_to, &named);
                (came_to, named)
            }
            None => {
                drop(in_flight);
                (
                    crate::submitting::WhatTheActCameTo::TheseJobs(vec![
                        crate::submitting::WhatBecameOfOneJob::ThisComputerCouldNotComposeIt,
                    ]),
                    Vec::new(),
                )
            }
        };
        if let Ok(mut held) = the_run_holds().lock() {
            held.submitting_now.remove(&keyed(&server, &file));
        }
        // **The *Submitting…* row has gone, or waits for the machine's own**:
        // the queue is drawn again now the act has ended (piece Z7).
        crate::surface::the_queue::the_queue_is_told_again(&app);
        // **A submission that went is the confirmation; anything else is what
        // the act came to**, said as before.
        let event = a_submission_that_went(&server, &file, &the_servers_name, how_many, &came_to, &named)
            .unwrap_or_else(|| {
                let (said, plain) = the_acts_answer(&came_to);
                a_submission_said(&server, &file, said, plain)
            });
        the_queue_is_told(&queue, event);
    });
}

/// **The application's own record, for the act that runs off the drawing
/// thread** — the lock taken for each write alone.
pub(crate) struct TheApplicationsRecord<R: tauri::Runtime> {
    app: tauri::AppHandle<R>,
}

/// [`TheApplicationsRecord`], for another act that submits — `Rendering`
/// `Rendering` S11's *Submit the remaining scenes* (the 38's piece 2).
pub(crate) fn the_applications_record<R: tauri::Runtime>(app: tauri::AppHandle<R>) -> TheApplicationsRecord<R> {
    TheApplicationsRecord { app }
}

impl<R: tauri::Runtime> crate::uploading::TheRecordAnUploadWrites for TheApplicationsRecord<R> {
    fn with_the_record(&self, act: &mut dyn FnMut(&rusqlite::Connection)) -> bool {
        let Some(started) = self.app.try_state::<Mutex<Started>>() else {
            return false;
        };
        let Ok(started) = started.lock() else {
            return false;
        };
        match started.record.as_ref() {
            Some(record) => {
                act(record.connection());
                true
            }
            None => false,
        }
    }
}

/// Called once when the primary connection starts serving requests, including
/// every restoration. Invalidate the old search before telling the page.
pub fn the_connection_is_lent<R: tauri::Runtime>(app: &tauri::AppHandle<R>, server: &str) {
    let Some(queue) = app.try_state::<std::sync::Arc<TheOneQueue>>() else {
        return;
    };
    if let Ok(mut held) = the_run_holds().lock() {
        the_queue_is_told(&queue, the_new_connection_for(&mut held, server));
    }
}

fn the_new_connection_for(held: &mut WhatTheRunHolds, server: &str) -> Event {
    the_blender_run_ended_for(held, server);
    Event::about(Subject::TheRun, THE_CONNECTION_IS_LENT)
        .saying("server", Said::Value(Verbatim::new(server.to_string())))
}

/// Drop the old connection's reading and search even if the page is collapsed
/// and never asks for a drawing while the connection is down
/// (`Render Settings` J10, `Server Setup` B12).
pub fn the_connection_ended(server: &str) {
    if let Ok(mut held) = the_run_holds().lock() {
        the_period_ended_for(&mut held, server);
    }
}

/// **`Render Settings` B1's list, asked for when this page wants it** —
/// `Server Setup` B12's *when `Render Settings` asks for the version list*,
/// run over the server's own point by [`crate::submitting::the_blenders_on_the_machine`].
///
/// **Period-held and never durable** (`Server Setup` B12): the answer is kept
/// in the run's own memory beside the page, and a page drawn with no live
/// connection shows none. **The search runs off the drawing thread** (P5) and
/// the page is told when it ends.
#[tauri::command]
pub fn the_version_list_is_asked_for<R: tauri::Runtime>(
    app: tauri::AppHandle<R>,
    started: State<'_, Mutex<Started>>,
    queue: State<'_, std::sync::Arc<TheOneQueue>>,
    server: String,
    file: Vec<u8>,
) {
    let file = the_path_as_it_is_keyed(&file);
    let point = app
        .try_state::<std::sync::Arc<crate::servers::a_period::ThePeriodsThisRun>>()
        .and_then(|periods| periods.the_sessions_of(&server))
        .filter(crate::servers::sessions::ThisServersSessions::a_connection_is_live);
    let Some(point) = point else {
        return;
    };
    // **`Server Setup` B9's own area is not a candidate**, at the exact path the
    // row's recorded layout composes, or nothing where no placement is recorded.
    let own_area = started
        .lock()
        .ok()
        .and_then(|started| {
            started
                .record
                .as_ref()
                .and_then(|record| crate::record::server_rows::all(record.connection()).ok())
        })
        .and_then(|rows| rows.into_iter().find(|row| row.local_server_identifier == server))
        .and_then(|row| crate::servers::installation::the_layout_recorded_for(&row))
        .map(|layout| smoozy_server_setup::blender::the_blender_area(&layout));
    let search = match the_run_holds().lock() {
        Ok(mut held) => match the_version_search_begins(&mut held, &server) {
            Some(search) => search,
            None => return,
        },
        Err(_) => return,
    };
    the_queue_is_told(&queue, an_act_said(&server, &file, THE_LIST_CAME_BACK, Vec::new()));
    let queue = std::sync::Arc::clone(queue.inner());
    std::thread::spawn(move || {
        let runtime = tokio::runtime::Builder::new_current_thread().enable_all().build();
        let asked = match runtime {
            Ok(runtime) => runtime.block_on(crate::submitting::the_blenders_on_the_machine(
                &point,
                &server,
                own_area.as_deref(),
            )),
            Err(_) => smoozy_render_settings::acting::versions::TheVersionListAsk::TheMachineCouldNotBeAsked,
        };
        // **`Server Setup` R1b's comparison, at `Server Setup` B12's moment of
        // `Render Settings` asking for the version list.** Added 2026-10-12.
        crate::servers::the_driver_swap::the_search_is_compared(&app, &server, &asked);
        if let Ok(mut held) = the_run_holds().lock() {
            if !the_version_search_came_back(&mut held, &server, &search, asked) {
                return;
            }
        }
        the_queue_is_told(&queue, an_act_said(&server, &file, THE_LIST_CAME_BACK, Vec::new()));
    });
}

/// One current search per server; an install's ending lets the next one begin.
fn the_version_search_begins(held: &mut WhatTheRunHolds, server: &str) -> Option<std::sync::Arc<()>> {
    if held.version_searches.get(server) == Some(&TheSearchStands::Searching) {
        return None;
    }
    let search = std::sync::Arc::new(());
    held.version_searches_in_flight.insert(server.to_string(), std::sync::Arc::clone(&search));
    held.version_searches.insert(server.to_string(), TheSearchStands::Searching);
    Some(search)
}

/// `Server Setup` B12: a search superseded by an install, removal or connection ending
/// cannot put its old list back, even if it answers after the fresh search.
fn the_version_search_came_back(
    held: &mut WhatTheRunHolds,
    server: &str,
    search: &std::sync::Arc<()>,
    asked: smoozy_render_settings::acting::versions::TheVersionListAsk,
) -> bool {
    if !held.version_searches_in_flight.get(server).is_some_and(|current| std::sync::Arc::ptr_eq(current, search)) {
        return false;
    }
    the_version_list_came_back(held, server, asked);
    true
}

/// **Where B1's answer lands.** A list replaces the searching mark; a machine
/// that could not be asked leaves no list and says so.
pub fn the_version_list_came_back(
    held: &mut WhatTheRunHolds,
    server: &str,
    asked: smoozy_render_settings::acting::versions::TheVersionListAsk,
) {
    held.version_searches_in_flight.remove(server);
    match asked.as_the_page_reads_it() {
        Some(list) => {
            held.version_searches.remove(server);
            held.version_lists.insert(server.to_string(), list);
        }
        None => {
            held.version_lists.remove(server);
            held.version_searches
                .insert(server.to_string(), TheSearchStands::TheMachineCouldNotBeAsked);
        }
    }
}

/// **`§ What it will write`'s material, stored in the same act as the reading
/// it came with**, with the scene it was read from — kept where the read named
/// its scene or the file has one, and dropped otherwise, so a stored reading
/// never stands beside another reading's outputs. *(Kept only for a file of one
/// scene until 2026-10-03, the runner's ruling of 2026-09-29, which hole 13 of
/// the map of the 38 promises reversed: the read now names its scene.)*
///
/// **With every other scene's half beside it** (`Job files and outputs` R4b),
/// in the same act, so a ticked scene that is not the read one has its own.
pub(crate) fn the_outputs_half_landed(
    held: &mut WhatTheRunHolds,
    server: &str,
    file: &str,
    outputs: Option<(Vec<u8>, TheOutputsHalf)>,
    the_other_scenes: Vec<(Vec<u8>, TheOutputsHalf)>,
) {
    let mark = keyed(server, file);
    let every: Vec<(Vec<u8>, TheOutputsHalf)> = outputs.into_iter().chain(the_other_scenes).collect();
    if every.is_empty() {
        held.outputs.remove(&mark);
    } else {
        held.outputs.insert(mark, every);
    }
}

/// **`Render Settings` J7's third naming act: the read's own tick** (added to
/// the plan 2026-10-03, the Creator's note 5 on the test copy), made in the act
/// that stores the reading. *"When a reading lands for a (server, file) and no
/// scene is ticked for it, the scene the reading names as the one the file was
/// saved showing is ticked — its bytes that reading's own, its provenance that
/// reading's identity … Where the reading names no saved scene, its first
/// scene is ticked."* **A read again keeps every tick naming a scene the new
/// reading holds and ticks nothing**; where none does, the read's tick takes
/// their place. **A scene that arrived from a history entry's kept setup is
/// never replaced by the read.**
///
/// **Why it matters** (Rule 8): with several scenes and none ticked the page
/// stood in the reading's first scene (`Render Settings` J3) while the job
/// carried no name and rendered the saved one (`Render Settings` Y10) — one
/// scene described, another rendered. With the saved scene ticked the job
/// carries its name by `Render Settings` F5's ticked branch.
///
/// **And `Render Settings` J3's last sentence, in the same act**: the pre-read
/// answer set becomes the entries of the scene the page then stands in —
/// re-keyed, never copied — so a range typed before the read stays in its
/// box. [`smoozy_render_settings::store::Store::a_reading_first_landed`] had
/// been built and nothing but its own test called it.
pub(crate) fn the_read_ticks_its_scene(
    held: &mut WhatTheRunHolds,
    server: &str,
    file: &str,
    the_scene_it_read: Option<&[u8]>,
) {
    match the_scene_it_read {
        Some(named) => {
            held.the_saved_scene.insert(keyed(server, file), named.to_vec());
        }
        None => {
            held.the_saved_scene.remove(&keyed(server, file));
        }
    }
    let key = FileKey {
        server: an_identity(server),
        path: file.to_string(),
    };
    let Some(reading) = held.readings.standing(&key) else {
        return;
    };
    let scenes = &reading.settings.scenes;
    let ticked: Vec<NamedAnswer> = held
        .store
        .per_file(&key)
        .map(|per_file| per_file.scenes_ticked.clone())
        .unwrap_or_default();
    let it_holds = |name: &ExactName| scenes.iter().any(|scene| scene.name.as_bytes() == name.as_bytes());
    let from_a_kept_setup = ticked
        .iter()
        .any(|tick| matches!(tick.provenance, Provenance::AKeptSetup { .. }));
    if from_a_kept_setup || ticked.iter().any(|tick| it_holds(&tick.name)) {
        return;
    }
    let Some(the_saved) = the_scene_named_as_saved(scenes, the_scene_it_read) else {
        return;
    };
    let provenance = Provenance::ARecordedReading(reading.identity());
    for gone in &ticked {
        held.store.untick_a_scene(key.clone(), &gone.name);
    }
    held.store.tick_a_scene(
        key.clone(),
        NamedAnswer {
            name: the_saved.clone(),
            provenance,
        },
    );
    held.store.a_reading_first_landed(key, the_saved);
}

/// **The scene a reading names as the one the file was saved showing, else
/// its first** — `Render Settings` J7's third act, and J5's re-tick, which
/// points at it.
fn the_scene_named_as_saved(
    scenes: &[smoozy_render_settings::reading::SceneReading],
    named: Option<&[u8]>,
) -> Option<ExactName> {
    named
        .and_then(|named| scenes.iter().find(|scene| scene.name.as_bytes() == named))
        .or_else(|| scenes.first())
        .map(|scene| scene.name.clone())
}

/// **`Render Settings` J5's re-tick, as changed 2026-10-08** (the Creator's
/// fifth notes, note 4: *"Reset all to file values doesn't reset the selected
/// scene. It should revert to the file's active scene."*). Where a reading
/// stands, the hand-everything-back control unticks every scene — a kept
/// setup's too, the control being *to file values* — and ticks the one the
/// reading names as saved, else its first, under that reading's provenance,
/// and deletes that scene's own entries so the page shows the file's values.
/// **With no reading there is no file scene to go back to**, and the ticks are
/// left as they are.
fn the_ticks_go_back_to_the_saved_scene(held: &mut WhatTheRunHolds, server: &str, file: &str) {
    let key = FileKey {
        server: an_identity(server),
        path: file.to_string(),
    };
    let Some(reading) = held.readings.standing(&key) else {
        return;
    };
    let named = held.the_saved_scene.get(&keyed(server, file)).map(Vec::as_slice);
    let Some(the_saved) = the_scene_named_as_saved(&reading.settings.scenes, named) else {
        return;
    };
    let provenance = Provenance::ARecordedReading(reading.identity());
    let ticked: Vec<ExactName> = held
        .store
        .per_file(&key)
        .map(|per_file| per_file.scenes_ticked.iter().map(|tick| tick.name.clone()).collect())
        .unwrap_or_default();
    for gone in &ticked {
        held.store.untick_a_scene(key.clone(), gone);
    }
    held.store.tick_a_scene(
        key.clone(),
        NamedAnswer {
            name: the_saved.clone(),
            provenance,
        },
    );
    held.store.hand_everything_back(key, PageInHand::OneScene(the_saved));
}

/// `Server Setup` B12: an install or removal ending drops this server's
/// version list and any older search, while keeping the file's reading.
pub fn the_blender_run_ended_for(held: &mut WhatTheRunHolds, server: &str) {
    held.version_lists.remove(server);
    held.version_searches.remove(server);
    held.version_searches_in_flight.remove(server);
}

/// The connected period also takes the file's reading with it (J10).
pub fn the_period_ended_for(held: &mut WhatTheRunHolds, server: &str) {
    the_blender_run_ended_for(held, server);
    // **`Render Settings` J10's first event: the connected period ends, and
    // the file's half goes with it, for every file of that server at once.**
    // Added at step 10c's dependency wave piece 3, when a reading could first
    // stand on this page.
    held.readings.the_period_ended(&an_identity(server));
}

/// **The act, as the page composed it** — `Render Settings` Y1's payload per
/// scene-job from the store and the resolved pick, and everything else the
/// driver needs, taken from the page, the run and the server row at the moment
/// of Submit.
///
/// **An answer the page cannot compose is said, never sent.**
pub fn the_act_as_the_page_composed_it(
    held: &WhatTheRunHolds,
    server: &str,
    file: &str,
    record: &WhatTheRecordHolds,
    row: &smoozy_servers::store::ServerRow,
    run: Identity,
) -> Result<crate::submitting::WhatThePageHandsOver, ASentence> {
    use crate::servers::installation::{the_layout_recorded_for, where_the_package_area_is, WhereThePackageAreaIs};
    let not_submitted = |why: &'static str| vec![plainly(words::NOT_SUBMITTED), plainly(why)];

    let key = FileKey {
        server: an_identity(server),
        path: file.to_string(),
    };
    let mark = keyed(server, file);
    let standing = held.readings.standing(&key);
    let per_file = held.store.per_file(&key);
    let ticked: Vec<NamedAnswer> = per_file.map(|per_file| per_file.scenes_ticked.clone()).unwrap_or_default();
    let scene_name = the_scene_in_hand(held, server, file);
    let scene: Option<&SceneReading> = standing.and_then(|reading| {
        let wanted = scene_name.as_ref()?;
        reading.settings.scenes.iter().find(|scene| scene.name.as_bytes() == wanted.as_bytes())
    });
    let saved_with = held
        .the_saved_with
        .get(&mark)
        .cloned()
        .or_else(|| scene.map(|scene| scene.saved_with.clone()))
        .and_then(|text| the_release_line(&text));
    let resolution = version::resolve(
        per_file.and_then(|per_file| per_file.version_pick.as_ref()),
        saved_with,
        held.version_lists.get(server),
    );
    // **`Render Settings` B5a: where no version resolves, submitting is not
    // offered**, and a press that arrives anyway composes nothing.
    let Some(picked) = resolution.the_payloads_pick() else {
        return Err(not_submitted(words::IT_COULD_NOT_BE_PREPARED));
    };

    // **`Render Settings` Y1: one payload per scene-job**, composed whole from
    // the store and the resolved pick. **`Rendering` S2**: several ticked is
    // one job per scene, every name the reading's own (S6). `named` is the
    // scene the job carries; `entries_of` is the scene whose entries it
    // composes from.
    let payload_for = |named: Option<&NamedAnswer>, entries_of: Option<&ExactName>| {
        let answers = held.store.the_answers_composition_reads(&key, entries_of);
        smoozy_render_settings::payload::compose(smoozy_render_settings::payload::Composition {
            version_pick: &picked,
            scene: named,
            answers: &answers,
            standing,
            stale: false,
        })
    };
    // **`Queue & History` P5b: the frame text the user typed, read from the
    // same entry the payloads compose from** (`Render Settings` F1 and
    // `Render Settings` F5) — the scene in hand's own, or the group's where
    // several are ticked, which is the one range every scene-job of the act
    // then carries (`Render Settings` J3). **It goes to the local record alone
    // and never into a payload**; where the entry holds no range the job
    // renders the file's own and nothing was typed. The eighteenth notes,
    // piece 7.
    let the_typed_frames = {
        let entries_of = match ticked.as_slice() {
            [] => scene_name.clone(),
            [first, ..] => Some(first.name.clone()),
        };
        let answers = held.store.the_answers_composition_reads(&key, entries_of.as_ref());
        answers.frame_range.and(answers.frame_range_text)
    };
    let scene_jobs = match ticked.as_slice() {
        // **`Render Settings` F5, the sentence settled 2026-10-01: where a
        // reading stands and no scene is ticked, a scene is in hand** — the one
        // the page stands in (`Render Settings` J3), [`the_scene_in_hand`], the same one
        // [`the_answer_is_taken`] writes under and the panel draws from — **and
        // the job composes from that scene's own entries.** Where the reading
        // holds one scene the job carries its name, with J7's provenance as a
        // tick would give it; where it holds several the job carries none
        // (`Render Settings` Y10), and so no pins (`Render Settings` C8). With
        // nothing read there is no scene in hand and the pre-read set is read,
        // as before.
        //
        // *Until step 10c's Sesterce-pass fix piece 1 this composed with no
        // scene in hand whenever none was ticked, so after* Read the file *it
        // read the pre-read set — re-keyed into the scene's own entry the
        // moment the reading landed (`Render Settings` J3), so empty — and a range the page drew
        // as typed reached the job as the file's own.*
        [] => {
            let the_one_scene = standing
                .filter(|reading| reading.settings.scenes.len() == 1)
                .and(scene_name.clone())
                .and_then(|name| {
                    Some(NamedAnswer {
                        name,
                        provenance: the_provenance(held, &key)?,
                    })
                });
            vec![payload_for(the_one_scene.as_ref(), scene_name.as_ref())]
        }
        several => several
            .iter()
            .map(|scene| payload_for(Some(scene), Some(&scene.name)))
            .collect(),
    };

    let submission = held.submission.get(&mark).cloned().unwrap_or_default();
    // **`Rendering` S5a's field**: the per-job answer where one was given,
    // else the stored default, composed by `Rendering`'s own composer. **The
    // fourth carries the minutes that stood** (`Telegram` V1a, 2026-10-19).
    // **`Telegram` V1a as amended: the default may be any of the five, minutes
    // included**, and minutes typed for this job win over the default's.
    let the_minutes = the_minutes_in_force(&submission, record);
    let reporting = smoozy_rendering::submission::the_reporting_field(
        submission
            .what_it_reports
            .and_then(|id| the_choice_named(id, the_minutes)),
        record
            .the_reporting_default
            .as_deref()
            .and_then(|id| the_app_wide_choice_named(id, the_minutes))
            .unwrap_or(ReportingChoice::ALL[0]),
    );

    let Some(layout) = the_layout_recorded_for(row) else {
        return Err(not_submitted(words::NOT_SET_UP_ON_THIS_SERVER));
    };
    let WhereThePackageAreaIs::At(package_area) = where_the_package_area_is(row) else {
        return Err(not_submitted(words::NOT_SET_UP_ON_THIS_SERVER));
    };
    // **A server with no installation this copy can use is not set up
    // either**, and the page says so before anything is sent — the same test
    // and the same words as the dependency check's
    // (`crate::surface::the_dependency_check`). The act's first step, the
    // remap material's placement (`crate::submitting`), refuses without one.
    //
    // *Written for a row either never set up (no root) or set up whole (a root
    // and a confirmed installation). It now also meets a row whose first
    // setup the machine deferred: the hand-over records the root at step 3,
    // and the installation is confirmed only once step 5's placement commits.
    // Until step 10c's RunPod-pass fix piece 1 such a row composed an act,
    // the act stopped at the remap material, and the page said* Smoozy could
    // not put the job's files on the server *about a server nothing had been
    // sent to.*
    let Some(installation) = row
        .last_confirmed_installation_identity
        .as_deref()
        .and_then(|text| Identity::read_back(IdentityKind::Installation, text))
    else {
        return Err(not_submitted(words::NOT_SET_UP_ON_THIS_SERVER));
    };
    let Some(local_server) = Identity::read_back(IdentityKind::LocalServer, &row.local_server_identifier) else {
        return Err(vec![plainly(words::THE_RECORD_WOULD_NOT_READ)]);
    };

    // **`Queue & History` N15's first mark: the record state `Rendering` W2
    // found at submission.**
    // **Read as the band reads it** — step 10c's dependency wave piece 7 —
    // so the mark says what the page said.
    let dependency_check = match the_warnings_case(&record.the_check, &picked) {
        TheWarningsCase::NobodyHasEverCheckedThisFileOnThisMachine
        | TheWarningsCase::ACheckWasRunAndDidNotComplete => {
            smoozy_queue_history::record::DependencyCheckAtSubmission::NoStandingAtAll
        }
        TheWarningsCase::AcheckWasRunAndTheFileHasBeenReplacedSince
        | TheWarningsCase::ACheckWasRunAndItsTreeHasBeenDisturbedSince => {
            smoozy_queue_history::record::DependencyCheckAtSubmission::AStandingCarryingTheReplacedSinceCheckOrDisturbedMark
        }
        TheWarningsCase::ACheckStandsAndDoesNotCoverThisBlender { .. } => {
            smoozy_queue_history::record::DependencyCheckAtSubmission::AStandingKeyedToAnotherBlenderVersion
        }
        TheWarningsCase::TheCheckLeftThingsUnfound(_) | TheWarningsCase::Absent => {
            smoozy_queue_history::record::DependencyCheckAtSubmission::ACompleteStandingStood
        }
    };

    Ok(crate::submitting::WhatThePageHandsOver {
        server: smoozy_queue_history::record::TheServerAsItStood {
            local_server,
            label: row.label.clone(),
            // **The row's badge is one of the eight** (`b1` to `b8`), and the
            // record keeps its number, the form `Queue & History`'s own type
            // holds it in. A colour that is not one of the eight keeps none.
            hue: row
                .colour
                .as_deref()
                .and_then(|colour| colour.strip_prefix('b'))
                .and_then(|number| number.parse::<u16>().ok())
                .filter(|number| (1..=8).contains(number))
                .map(smoozy_queue_history::record::Hue),
        },
        installation: Some(installation),
        layout,
        package_area,
        the_blend: file.to_string(),
        label: Some(submission.job_name.clone()).filter(|name| !name.is_empty()),
        reporting,
        the_version_origin: match resolution {
            version::Resolution::TheUsersOverride(_) => {
                smoozy_queue_history::record::AnswerOrigin::TheUsersOwnOverride
            }
            _ => smoozy_queue_history::record::AnswerOrigin::ResolvedForThem,
        },
        dependency_check: Some(dependency_check),
        // **`Rendering` §9 item 27: no read-back of the build's add-ons is held
        // on this computer**, so the engine gate has nothing to judge against
        // and `Render Settings` E2 decides at the attempt.
        what_the_desktop_holds_about_the_build:
            smoozy_rendering::submission::WhatTheDesktopHoldsAboutTheBuild::NothingButTheEntry,
        the_first_frame_check: submission.the_first_frame_check,
        // **`Download` R1's three: the strip's own switch, the folder it names
        // and the formats its box left chosen** — handed over with everything
        // else the act needs, because the act runs off this thread and never
        // reads the page back. **The folder is [`the_folder_named`]'s own
        // order** — chosen for this job, else the standing folder, else §8's
        // proposal — in the system's own path form rather than the shown one.
        the_download_as_it_renders_choice: submission.download_as_it_renders,
        where_the_downloads_go: submission
            .where_they_go
            .clone()
            .or_else(|| record.the_standing_folder.clone())
            .or_else(|| record.the_proposed_folder.clone()),
        the_formats_chosen: submission.the_formats_chosen.clone(),
        scene_jobs,
        // **The reading standing on the page goes with the act for the local
        // record alone** (`Queue & History` N14 and `Queue & History` E12a);
        // the manifest says no
        // reading whatever stands here (`Job files and outputs` R7's second
        // bullet, the runner's ruling of 2026-09-29).
        the_reading_in_force: standing.cloned(),
        // **`Job files and outputs` M1b: the card make goes on the manifest**,
        // composed from the hardware reading `Servers` holds for this row —
        // the same held reading this page already reads for the device
        // warning, so the value the user was shown and the value the render
        // names are one object. **`Rendering` L23 has no way to take the
        // reading on the machine.**
        the_card_make: smoozy_rendering::submission::the_card_make_field(
            held.hardware.get(server),
        ),
        run,
        // **`Queue & History` N15's second mark**, where this page was opened
        // by *Resume* (the 38's piece 2).
        carries_on_from: held
            .where_it_came_from
            .get(&mark)
            .and_then(ThePageCameFrom::the_carry_on_mark),
        the_temporary_area: crate::platform::locations::smoozys_own_temporary_area(),
        // **`Job files and outputs` P23: the row's chosen fallback folder as it
        // stands at Submit**, the one the manifest carries.
        the_servers_fallback_folder: row.chosen_fallback_folder.clone(),
        // **`Server Setup` R1c's readings are taken at the press**, where the
        // application is ([`crate::servers::the_driver_swap::the_driver_at_submission`]);
        // composed here, nothing is held.
        the_driver_at_submission: Default::default(),
        the_typed_frames,
    })
}

/// **What a read that gave nothing is held as, beside the page.**
///
/// **`Render Settings` Q8's own ending, or the one case beside the four**:
/// the server answered and ran no Blender at all — the Smoozy on it does not
/// know the request, or did not admit the reading. `Render Settings`' own
/// judgement puts that on its explicit-absence arm, which the page otherwise
/// says as *reading the file did not finish*; no read ran, so the page says
/// could not read the file on the server
/// (`Design/Render Settings § When the read gives nothing`, 2026-09-29).
#[derive(Debug, Clone, PartialEq)]
pub enum AReadThatGaveNothing {
    Ended(Ending),
    TheServerRanNoBlender,
}

impl From<Ending> for AReadThatGaveNothing {
    fn from(ending: Ending) -> AReadThatGaveNothing {
        AReadThatGaveNothing::Ended(ending)
    }
}

/// **What the act says, on the control that asked** — `Rendering` S10: which
/// scene-jobs went and which did not, and why, one sentence for each thing
/// that is true.
pub fn what_the_act_says(came_to: &crate::submitting::WhatTheActCameTo) -> ASentence {
    the_acts_answer(came_to).0
}

/// **What the act says, and how many of its leading parts say what went out**
/// ([`THE_PLAIN_PARTS`]). **Composed together**, so the words and which of
/// them are plain cannot drift apart. *Submitted.* is one plain part, and
/// *2 of 3 jobs submitted.* is two; *Not submitted.* and every reason are
/// none. Step 10c's tenth-pass fix piece 6. *(A fourth, *Only the first frame
/// was submitted…*, went with its route at the eleventh-pass fix piece 8.)*
pub fn the_acts_answer(came_to: &crate::submitting::WhatTheActCameTo) -> (ASentence, usize) {
    use crate::submitting::{WhatBecameOfOneJob as Became, WhatTheActCameTo as CameTo};
    let jobs = match came_to {
        // A first-frame refusal is handled by its own control.
        CameTo::TheFirstFrameIsNotKnown { a_scene_can_be_ticked } => {
            let why = if *a_scene_can_be_ticked {
                words::THE_FIRST_FRAME_IS_NOT_KNOWN_OF_SEVERAL_SCENES
            } else {
                words::THE_FIRST_FRAME_IS_NOT_KNOWN
            };
            return (vec![plainly(words::NOT_SUBMITTED), plainly(why)], 0);
        }
        CameTo::TheseJobs(jobs) => jobs,
    };
    let in_the_line = jobs.iter().filter(|job| **job == Became::ItIsInTheLine).count();
    let mut said: ASentence = Vec::new();
    let mut plain = 0;
    if in_the_line == jobs.len() {
        return (vec![plainly(words::SUBMITTED)], 1);
    } else if in_the_line > 0 {
        said.push(part(
            "",
            vec![in_the_line.to_string()],
            Some(words::NOT_SUBMITTED_OF),
            false,
        ));
        said.push(a_figure(jobs.len().to_string(), words::JOBS_SUBMITTED));
        plain = said.len();
    } else {
        said.push(plainly(words::NOT_SUBMITTED));
    }
    // **Each reason once**, in the order the act met them.
    let mut reasons: Vec<(&'static str, ASentence)> = Vec::new();
    for job in jobs {
        let (which, reason): (&'static str, ASentence) = match job {
            // **A first-frame check's second job not sent behind a first that
            // was not** says nothing of its own: the first job's reason is the
            // reason (`Rendering` S13).
            Became::ItIsInTheLine | Became::NotSentBehindAFirstFrameThatWasNot => continue,
            Became::TheServerDoesNotTakeJobs => ("program", vec![plainly(words::THE_SERVER_DOES_NOT_TAKE_JOBS)]),
            Became::NotKnown => ("not known", vec![plainly(words::NOT_KNOWN_WHETHER_IT_WAS_TAKEN)]),
            // **`Rendering` S4's unit stopped at its last step — the job
            // record — only where the machine answered that it does not hold
            // the job** (`Rendering` S9): the files were placed, and the server
            // did not take the job. Every earlier step is the files not placed.
            Became::ItStoppedBeforeTheJobRecord { the_step }
                if *the_step == smoozy_rendering::submission::THE_ORDERED_UNIT[2] =>
            {
                ("not taken", vec![plainly(words::THE_SERVER_DID_NOT_TAKE_THE_JOB)])
            }
            Became::ItStoppedBeforeTheJobRecord { .. } => {
                ("files", vec![plainly(words::THE_FILES_COULD_NOT_BE_PLACED)])
            }
            Became::TheRecordWasRefused => ("record", vec![plainly(words::THE_RECORD_REFUSED_THE_JOB)]),
            Became::TheBuildLacksTheEngine { engine, build } => (
                "engine",
                vec![
                    part("", vec![build.clone()], Some(words::THE_BUILD_LACKS_THE_ENGINE_LEAD), false),
                    part("", vec![engine.clone()], Some(words::THE_FULL_STOP), true),
                ],
            ),
            Became::NotAttemptedBehindAnUnaccountedOne => {
                ("behind", vec![plainly(words::STOPPED_BEHIND_AN_UNKNOWN_ONE)])
            }
            Became::TheFileOnTheServerCouldNotBeRead => {
                ("file", vec![plainly(words::THE_FILE_ON_THE_SERVER_COULD_NOT_BE_READ)])
            }
            Became::ThisComputerCouldNotComposeIt => ("computer", vec![plainly(words::IT_COULD_NOT_BE_PREPARED)]),
        };
        if !reasons.iter().any(|(held, _)| *held == which) {
            reasons.push((which, reason));
        }
    }
    let reasons: ASentence = reasons.into_iter().flat_map(|(_, reason)| reason).collect();
    said.extend(reasons);
    said.truncate(THE_PARTS_OWN_KEYS.len());
    let plain = plain.min(said.len());
    (said, plain)
}

// ════════════════════════════════════════════════════════════════════════════
// The acts, with the state handed in so the whole of each can be tested
// ════════════════════════════════════════════════════════════════════════════

/// Which box an answer belongs to, so the page can put a refusal on it.
fn the_box_of(which: WhichSetting) -> &'static str {
    match which {
        WhichSetting::FrameRange => "range",
        WhichSetting::SampleCount => "samples",
        WhichSetting::ResolutionPercentage => "resolution",
        WhichSetting::EnginePick => "engine",
        WhichSetting::Device => "device",
        WhichSetting::VersionPick => "version",
        WhichSetting::JobName => "job-name",
        WhichSetting::DownloadAsTheyRender => "download",
        WhichSetting::TheFirstFrameCheck => "first-frame",
        WhichSetting::WhatItReports => "reports",
        WhichSetting::TelegramMinutes => "telegram-minutes",
        WhichSetting::Simplify => "simplify",
        WhichSetting::SimplifySubdivision => "simplify-subdivision",
        WhichSetting::SimplifyChildParticles => "simplify-child-particles",
        WhichSetting::SimplifyTextureLimit => "simplify-texture-limit",
        WhichSetting::Everything => "everything",
    }
}

/// **J11, performed: the typed value never becomes a store entry where the box
/// refuses**, and the entry that stood goes on standing.
pub fn the_answer_is_taken(
    held: &mut WhatTheRunHolds,
    server: &str,
    file: &str,
    which: WhichSetting,
    text: &str,
) -> Option<Refusal> {
    let key = FileKey {
        server: an_identity(server),
        path: file.to_string(),
    };
    let scene = the_scene_in_hand(held, server, file);
    let ticked = held
        .store
        .per_file(&key)
        .map(|per_file| per_file.scenes_ticked.len())
        .unwrap_or(0);
    let at = match &scene {
        Some(scene) => Where::AScene {
            file: key.clone(),
            scene: scene.clone(),
        },
        None => Where::ThePreReadSet { file: key.clone() },
    };
    let the_pick = held
        .store
        .the_answers_composition_reads(&key, scene.as_ref())
        .engine_pick;

    match which {
        WhichSetting::FrameRange => match frames::parse_the_frame_field(text) {
            Ok(form) => {
                if ticked > 1 {
                    // **J3: the group entry is the one that stands while more
                    // than one scene is ticked**, and it shadows the per-scene
                    // ranges rather than being written into any scene's key.
                    let _ = held.store.answer_the_typed_group_range(key, form, text);
                } else {
                    held.store.answer_the_typed_range(at, form, text);
                }
                None
            }
            Err(refusal) => Some(Refusal::TheFrameField(refusal)),
        },
        WhichSetting::SampleCount => match refusals::samples_box(text, the_pick) {
            Ok(count) => {
                held.store.answer(at, Answer::SampleCount(count));
                None
            }
            Err(refusal) => Some(refusal),
        },
        WhichSetting::ResolutionPercentage => match refusals::percentage_box(text) {
            Ok(percentage) => {
                held.store
                    .answer(at, Answer::ResolutionPercentage(percentage));
                None
            }
            Err(refusal) => Some(refusal),
        },
        WhichSetting::EnginePick => {
            let pick = match text {
                "cycles" => OfferedEngine::Cycles,
                "eevee" => OfferedEngine::Eevee,
                _ => return None,
            };
            let standing = held
                .store
                .the_answers_composition_reads(&key, scene.as_ref())
                .sample_count;
            // **J11 item 3's fourth bullet: the engine control is held to the
            // standing count by the same test**, refused in place.
            match refusals::engine_pick_against_the_standing_count(pick, standing) {
                Ok(_) => {
                    held.store.answer(at, Answer::EnginePick(pick));
                    None
                }
                Err(refusal) => Some(refusal),
            }
        }
        WhichSetting::Device => {
            let choice = match text {
                "the-cards" => DeviceChoice::TheCards,
                "the-processor" => DeviceChoice::TheProcessor,
                _ => return None,
            };
            held.store.answer(at, Answer::Device(choice));
            None
        }
        WhichSetting::VersionPick => {
            // **A path that is not on the list this server answered picks
            // nothing** — `Render Settings` B3's *a version number does not identify a
            // build* is
            // why the pick is a path, and a path nothing listed is not a build.
            if let Some(build) = the_listed_build(held, server, text) {
                held.store.pick_a_version(key, build);
                the_version_change_drops_the_reading(held, server, file);
            }
            None
        }
        WhichSetting::JobName => {
            held.submission
                .entry(keyed(server, file))
                .or_default()
                .job_name = text.to_string();
            None
        }
        WhichSetting::DownloadAsTheyRender => {
            held.submission
                .entry(keyed(server, file))
                .or_default()
                .download_as_it_renders = text == "on";
            None
        }
        WhichSetting::TheFirstFrameCheck => {
            held.submission
                .entry(keyed(server, file))
                .or_default()
                .the_first_frame_check = text == "on";
            None
        }
        WhichSetting::WhatItReports => {
            if let Some((value, _)) = REPORTING_CHOICES.iter().find(|(value, _)| *value == text) {
                held.submission
                    .entry(keyed(server, file))
                    .or_default()
                    .what_it_reports = Some(value);
            }
            None
        }
        // **Minutes Smoozy will not take become nothing here** and are said
        // by [`what_the_answer_says`]; the minutes that stood go on standing.
        WhichSetting::TelegramMinutes => {
            if let Ok(minutes) = reporting::the_minutes_typed(text) {
                held.submission
                    .entry(keyed(server, file))
                    .or_default()
                    .the_minutes_between_pictures = Some(minutes);
            }
            None
        }
        WhichSetting::Simplify => {
            // **The switch's value is parsed like every other switch's**, and a
            // value that is neither is taken as nothing and changes nothing.
            let on = match text {
                "on" => true,
                "off" => false,
                _ => return None,
            };
            held.store.answer(at, Answer::SimplifySwitch(on));
            None
        }
        // **`Rendering` A1d: each of the three is its own entry**, under the
        // same key as the switch. A value a box refuses becomes nothing here
        // and is said by [`what_a_simplify_box_refuses`] at the press.
        WhichSetting::SimplifySubdivision => {
            if let Ok(value) = smoozy_rendering::again::the_max_subdivision_box(text) {
                held.store.answer(at, Answer::SimplifySubdivision(value));
            }
            None
        }
        WhichSetting::SimplifyChildParticles => {
            if let Ok(value) = smoozy_rendering::again::the_child_particles_box(text) {
                held.store.answer(at, Answer::SimplifyChildParticles(value));
            }
            None
        }
        WhichSetting::SimplifyTextureLimit => {
            if let Some(picked) = smoozy_rendering::again::the_texture_limit_pick(text) {
                held.store.answer(at, Answer::SimplifyTextureLimit(picked.to_string()));
            }
            None
        }
        WhichSetting::Everything => None,
    }
}

/// **What `Rendering` A1d's two typed boxes refuse**, where the answer is for
/// one of them; nothing for every other setting, the Texture Limit pick
/// included, which refuses nothing.
fn what_a_simplify_box_refuses(which: WhichSetting, text: &str) -> Option<smoozy_rendering::again::StateTheRange> {
    match which {
        WhichSetting::SimplifySubdivision => smoozy_rendering::again::the_max_subdivision_box(text).err(),
        WhichSetting::SimplifyChildParticles => smoozy_rendering::again::the_child_particles_box(text).err(),
        _ => None,
    }
}

/// **`Rendering` A1d: each refusal states the range alone**, *"because what
/// Blender does with an out-of-range assignment to either setting is
/// established nowhere"* — `Render Settings` J12's `TheRangeAlone` shape, worded as that one is.
fn what_the_range_alone_says(range: smoozy_rendering::again::StateTheRange) -> ASentence {
    use smoozy_rendering::again::StateTheRange;
    let upper = match range {
        StateTheRange::MaxSubdivisionTakesWholeNumbersFromZeroTo32767 => "32767",
        StateTheRange::ChildParticlesTakesANumberFromZeroToOne => "1",
    };
    vec![
        part("", vec!["0".to_string()], Some(words::TO), false),
        part("", vec![upper.to_string()], Some(words::THE_FULL_STOP), true),
    ]
}

/// **J5, performed.** The per-setting cross deletes that entry; the
/// hand-everything-back control deletes the entries the page is showing, plus
/// the version override, which is the one entry that is not per scene — and,
/// where a reading stands, ticks the scene the file was saved showing, alone
/// ([`the_ticks_go_back_to_the_saved_scene`]).
pub fn the_value_is_handed_back(
    held: &mut WhatTheRunHolds,
    server: &str,
    file: &str,
    which: WhichSetting,
) -> Option<Refusal> {
    let key = FileKey {
        server: an_identity(server),
        path: file.to_string(),
    };
    let scene = the_scene_in_hand(held, server, file);
    let ticked = held
        .store
        .per_file(&key)
        .map(|per_file| per_file.scenes_ticked.len())
        .unwrap_or(0);
    let at = match &scene {
        Some(scene) => Where::AScene {
            file: key.clone(),
            scene: scene.clone(),
        },
        None => Where::ThePreReadSet { file: key.clone() },
    };

    let setting = match which {
        WhichSetting::FrameRange => Setting::FrameRange,
        WhichSetting::SampleCount => Setting::SampleCount,
        WhichSetting::ResolutionPercentage => Setting::ResolutionPercentage,
        WhichSetting::EnginePick => Setting::EnginePick,
        WhichSetting::Device => Setting::Device,
        WhichSetting::Simplify => Setting::SimplifySwitch,
        WhichSetting::SimplifySubdivision => Setting::SimplifySubdivision,
        WhichSetting::SimplifyChildParticles => Setting::SimplifyChildParticles,
        WhichSetting::SimplifyTextureLimit => Setting::SimplifyTextureLimit,
        WhichSetting::VersionPick => {
            held.store.clear_the_version_pick(key);
            the_version_change_drops_the_reading(held, server, file);
            return None;
        }
        WhichSetting::Everything => {
            let page = match (&scene, ticked) {
                (_, many) if many > 1 => PageInHand::AGroup,
                (Some(scene), _) => PageInHand::OneScene(scene.clone()),
                (None, _) => PageInHand::NoSceneInHand,
            };
            held.store.hand_everything_back(key, page);
            // **Before the version check**, which may drop the reading the
            // re-tick is drawn from.
            the_ticks_go_back_to_the_saved_scene(held, server, file);
            the_version_change_drops_the_reading(held, server, file);
            return None;
        }
        WhichSetting::JobName
        | WhichSetting::DownloadAsTheyRender
        | WhichSetting::TheFirstFrameCheck
        | WhichSetting::WhatItReports
        | WhichSetting::TelegramMinutes => return None,
    };
    held.store.clear_one(at, &setting).err()
}

/// **`Render Settings` Q12: changing the version on this page drops the
/// file's half whole**, exactly as a reconnect does — J10's act. **Added at
/// step 10c's dependency wave piece 3**, when a reading could first stand on
/// this page: without it a reading taken by one Blender went on standing under
/// a pick of another. **The test is the one Q12 closes on** — *nothing anywhere
/// holds a read that outlives its version* — so a reading stands only while the
/// page's pick is the build that took it, and picking that same build again
/// changes nothing.
fn the_version_change_drops_the_reading(held: &mut WhatTheRunHolds, server: &str, file: &str) {
    let key = FileKey {
        server: an_identity(server),
        path: file.to_string(),
    };
    let pick = the_version_resolved(held, server, file).the_payloads_pick();
    let outlived = held
        .readings
        .standing(&key)
        .is_some_and(|reading| Some(&reading.taken_by) != pick.as_ref());
    if outlived {
        held.readings.drop_the_files_half(
            &key,
            smoozy_render_settings::file_half::WhatDroppedIt::TheVersionChanged,
        );
    }
}

/// The build a picked path names, out of the list this server answered.
fn the_listed_build(
    held: &WhatTheRunHolds,
    server: &str,
    path: &str,
) -> Option<BlenderIdentity> {
    let list = held.version_lists.get(server)?;
    let entry = list.entries.iter().find(|entry| entry.path == path)?;
    Some(BlenderIdentity {
        path: entry.path.clone(),
        version: entry.answered?,
    })
}

/// **J7's provenance, out of the reading the name was drawn from.** The page
/// offers no way to type a scene or a view-layer name, so every name in the
/// store is, by provenance, bytes some reading of some file produced — a
/// user's tick, a kept setup's, or since 2026-10-03 the read's own tick
/// ([`the_read_ticks_its_scene`], `Render Settings` J7's third act). Where no reading stands
/// there is no name to hold and nothing is written.
fn the_provenance(held: &WhatTheRunHolds, key: &FileKey) -> Option<Provenance> {
    Some(Provenance::ARecordedReading(
        held.readings.standing(key)?.identity(),
    ))
}

/// **The one scene in hand**, which is the ticked one where exactly one is
/// ticked, else the first the reading holds. **Where nothing has been read
/// there is no scene at all**, which is J3's pre-read state.
fn the_scene_in_hand(held: &WhatTheRunHolds, server: &str, file: &str) -> Option<ExactName> {
    let key = FileKey {
        server: an_identity(server),
        path: file.to_string(),
    };
    if let Some(per_file) = held.store.per_file(&key) {
        if per_file.scenes_ticked.len() == 1 {
            return Some(per_file.scenes_ticked[0].name.clone());
        }
        if per_file.scenes_ticked.len() > 1 {
            return None;
        }
    }
    held.readings
        .standing(&key)
        .and_then(|reading| reading.settings.scenes.first())
        .map(|scene| scene.name.clone())
}

/// **The outputs half, only where the scene it was read from is the page's
/// scene in hand** — `Design/Render Settings § What it will write`: *"With that
/// scene ticked, the group is drawn as on a one-scene file. With another scene
/// ticked … the group stays as it is before a read, because a list from the
/// read would be another scene's outputs."* The 38's piece 10, item 25. The
/// group, the format box and its *Done* read it through here alike.
///
/// **Since `Job files and outputs` R4b the reading holds every scene's half**
/// (the Creator's seventh notes, note 1), so whichever scene is in
/// hand has its own; only a reading from an older machine program, which
/// holds one scene, leaves another scene with none.
fn the_outputs_in_hand<'h>(held: &'h WhatTheRunHolds, server: &str, file: &str) -> Option<&'h TheOutputsHalf> {
    let in_hand = the_scene_in_hand(held, server, file)?;
    the_outputs_of_the_scene(held, server, file, in_hand.as_bytes())
}

fn the_outputs_of_the_scene<'h>(
    held: &'h WhatTheRunHolds,
    server: &str,
    file: &str,
    scene: &[u8],
) -> Option<&'h TheOutputsHalf> {
    held.outputs
        .get(&keyed(server, file))?
        .iter()
        .find(|(read_from, _)| read_from.as_slice() == scene)
        .map(|(_, outputs)| outputs)
}

/// **What the download box reads: the half of every scene ticked** — one
/// where one is ticked or none is (the scene in hand), and with several ticked
/// every one of theirs, so the box offers every format any of them writes
/// (`Job files and outputs` R4b). **Nothing where any ticked scene has no
/// half**: the box cannot say what all of them write.
fn the_outputs_the_box_reads<'h>(held: &'h WhatTheRunHolds, server: &str, file: &str) -> Option<Vec<&'h TheOutputsHalf>> {
    let key = FileKey {
        server: an_identity(server),
        path: file.to_string(),
    };
    held.readings.standing(&key)?;
    if let Some(per_file) = held.store.per_file(&key) {
        if per_file.scenes_ticked.len() > 1 {
            return per_file
                .scenes_ticked
                .iter()
                .map(|ticked| the_outputs_of_the_scene(held, server, file, ticked.name.as_bytes()))
                .collect();
        }
    }
    the_outputs_in_hand(held, server, file).map(|outputs| vec![outputs])
}

/// **`Render Settings` B2a's resolution for one (server, file), as the page
/// stands now** — the store's own pick, else the automatic match over B1's
/// list. **One composition**, read by the page it draws and by *Read the file*
/// at the press and at the landing (`Render Settings` B5a, `Render Settings` Q17), so the pick a
/// reading is started under and judged against is the pick the page shows.
pub fn the_version_resolved(held: &WhatTheRunHolds, server: &str, file: &str) -> Resolution {
    let key = FileKey {
        server: an_identity(server),
        path: file.to_string(),
    };
    let scene_name = the_scene_in_hand(held, server, file);
    let scene = held.readings.standing(&key).and_then(|reading| {
        let wanted = scene_name.as_ref()?;
        reading
            .settings
            .scenes
            .iter()
            .find(|scene| scene.name.as_bytes() == wanted.as_bytes())
    });
    the_resolution_of(held, server, file, held.store.per_file(&key), scene)
}

fn the_resolution_of(
    held: &WhatTheRunHolds,
    server: &str,
    file: &str,
    per_file: Option<&store::PerFile>,
    scene: Option<&SceneReading>,
) -> Resolution {
    version::resolve(
        per_file.and_then(|per_file| per_file.version_pick.as_ref()),
        the_saved_with_of(held, server, file, scene),
        held.version_lists.get(server),
    )
}

/// **`Render Settings` B2's source is `File Browser`'s read taken without
/// opening the file**, and the reading's own field is the same fact taken by
/// the read — used only where the browser handed nothing, so there is never a
/// third answer.
fn the_saved_with_of(
    held: &WhatTheRunHolds,
    server: &str,
    file: &str,
    scene: Option<&SceneReading>,
) -> Option<version::ReleaseLine> {
    held.the_saved_with
        .get(&keyed(server, file))
        .cloned()
        .or_else(|| scene.map(|scene| scene.saved_with.clone()))
        .and_then(|text| the_release_line(&text))
}

/// **The server's local identity, from the text the page hands back** — the
/// same parse every key on this page is made with, for a caller outside it.
pub fn the_servers_identity(server: &str) -> Identity {
    an_identity(server)
}

/// Every scene the standing reading holds.
fn the_standing_scenes(held: &WhatTheRunHolds, server: &str, file: &str) -> Vec<ExactName> {
    let key = FileKey {
        server: an_identity(server),
        path: file.to_string(),
    };
    held.readings
        .standing(&key)
        .map(|reading| {
            reading
                .settings
                .scenes
                .iter()
                .map(|scene| scene.name.clone())
                .collect()
        })
        .unwrap_or_default()
}

/// **An identity is never composed from anything else** (`State, identity and
/// recovery` Y5), and this parses one rather than making one — a server
/// identifier the page hands back is one the application gave it.
fn an_identity(server: &str) -> Identity {
    // A page handing back something that is not an identity keys nothing, and
    // the panel it gets is the one for a file nobody has read — **which is the
    // value that claims nothing**, not a made-up server.
    Identity::read_back(IdentityKind::LocalServer, server).unwrap_or_else(|| {
        Identity::read_back(IdentityKind::LocalServer, &"0".repeat(32))
            .expect("thirty-two zeroes are an identity's own rendering")
    })
}

// ════════════════════════════════════════════════════════════════════════════
// The event, which is what an act answers with
// ════════════════════════════════════════════════════════════════════════════

fn the_queue_is_told(queue: &TheOneQueue, event: Event) {
    let _ = queue.read_then_post::<Event, std::convert::Infallible>(
        || Ok(event.clone()),
        |event| event.clone(),
    );
}

/// **What an act says.** The page re-asks for the panel; this carries only what
/// the panel cannot: which box was acted on, and the refusal where there was
/// one.
pub fn an_act_said(server: &str, file: &str, the_box: &'static str, said: ASentence) -> Event {
    let mut event = Event::about(Subject::TheRun, AN_ACT_ON_THIS_PAGE)
        .saying("server", Said::Value(Verbatim::new(server.to_string())))
        .saying("file", Said::Value(Verbatim::new(file.to_string())))
        .saying("theBox", Said::Value(Verbatim::new(the_box)));
    let mut many = 0usize;
    for (at, part) in said.into_iter().take(THE_PARTS_OWN_KEYS.len()).enumerate() {
        let (lead, values, rest, joins, binds) = THE_PARTS_OWN_KEYS[at];
        many = at + 1;
        // **Worked out before the part is taken apart**, by the one rule every
        // `Words` is bound by — `Design/Whole app § Typography`.
        let the_rest_binds = part.the_rest_binds();
        event = event.saying(lead, Said::Prose(part.lead));
        for (which, value) in part.values.into_iter().take(values.len()).enumerate() {
            event = event.saying(values[which], Said::Value(value));
        }
        if let Some(said) = part.rest {
            event = event.saying(rest, Said::Prose(said));
        }
        if part.the_rest_joins {
            event = event.saying(joins, Said::Value(Verbatim::new("yes")));
        }
        if the_rest_binds {
            event = event.saying(binds, Said::Value(Verbatim::new("yes")));
        }
    }
    event.saying(
        "partCount",
        Said::Value(Verbatim::new(many.to_string())),
    )
}

/// **The submission's answer, with how many of its leading parts are plain**
/// ([`THE_PLAIN_PARTS`]) — the one act on this page whose answer can say that
/// something went out.
pub fn a_submission_said(server: &str, file: &str, said: ASentence, plain: usize) -> Event {
    an_act_said(server, file, "submit", said)
        .saying(THE_PLAIN_PARTS, Said::Value(Verbatim::new(plain.to_string())))
}

/// **A submission every job of which went, said as the confirmation** —
/// `Design/Render Settings § Submitting` (2026-10-03, the Creator's note 10 on
/// the test copy): *"A submission that went says so beside the Render control,
/// in ordinary ink at the page's own text size, with a check mark: Submitted to
/// Vast 4090., or 3 jobs submitted to Vast 4090., and Show in Queue beside
/// it, which opens the queue with that job, or that group, chosen."* **The
/// count is of scene-jobs** — what the user asked for, one per scene ticked —
/// so a first-frame check's two jobs of one scene read as one submission. The
/// queue opens on the first job the act sent, which is a group's first.
///
/// **Nothing where any job did not go**: a part-submission keeps its own
/// statement ([`the_acts_answer`]), and so does a refusal.
pub fn a_submission_that_went(
    server: &str,
    file: &str,
    the_servers_name: &str,
    how_many: usize,
    came_to: &crate::submitting::WhatTheActCameTo,
    named: &[crate::submitting::AJobAsSent],
) -> Option<Event> {
    use crate::submitting::{WhatBecameOfOneJob as Became, WhatTheActCameTo as CameTo};
    let CameTo::TheseJobs(jobs) = came_to else {
        return None;
    };
    if jobs.is_empty() || jobs.iter().any(|job| *job != Became::ItIsInTheLine) {
        return None;
    }
    let first = named.first()?;
    let said: ASentence = if how_many > 1 {
        vec![
            a_figure(how_many.to_string(), words::JOBS_SUBMITTED_TO),
            part("", vec![the_servers_name.to_string()], Some(words::THE_FULL_STOP), true),
        ]
    } else {
        vec![part(
            words::SUBMITTED_TO,
            vec![the_servers_name.to_string()],
            Some(words::THE_FULL_STOP),
            true,
        )]
    };
    let plain = said.len();
    Some(
        a_submission_said(server, file, said, plain)
            .saying(THE_JOB_TO_SHOW, Said::Value(Verbatim::new(first.job.as_text())))
            .saying(SHOW_IN_QUEUE_SAID, Said::Prose(Prose::new(words::SHOW_IN_QUEUE))),
    )
}

/// **The keys a sentence's parts travel under**, declared here beside the
/// service that posts them. `web/render-settings.js` reads the same names.
#[allow(clippy::type_complexity)]
const THE_PARTS_OWN_KEYS: [(&str, [&str; 3], &str, &str, &str); 4] = [
    (
        "lead0",
        ["value00", "value01", "value02"],
        "rest0",
        "joins0",
        "binds0",
    ),
    (
        "lead1",
        ["value10", "value11", "value12"],
        "rest1",
        "joins1",
        "binds1",
    ),
    (
        "lead2",
        ["value20", "value21", "value22"],
        "rest2",
        "joins2",
        "binds2",
    ),
    (
        "lead3",
        ["value30", "value31", "value32"],
        "rest3",
        "joins3",
        "binds3",
    ),
];

/// **J12, drawn: what a refusal states, per setting.** *A number Smoozy will
/// not take turns the field red and says the range, and says what Blender would
/// otherwise have done silently.*
///
/// **The exact wording is Design's and what may be claimed is the plan's**, so
/// each of the six statements has one sentence here and none of them is
/// composed out of another setting's evidence.
fn what_the_refusal_says(refusal: &Refusal) -> ASentence {
    match refusals::what_the_refusal_states(refusal) {
        Statement::CorrectedSilentlyAtAssignmentAndPaidFor { lower, upper } => vec![
            part(
                "",
                vec![lower.to_string()],
                Some(words::TO),
                false,
            ),
            part(
                "",
                vec![upper.to_string()],
                Some(words::THE_FULL_STOP),
                true,
            ),
            plainly(words::CORRECTED_SILENTLY),
        ],
        Statement::TheRangeAlone { lower, upper } => vec![
            part("", vec![lower.to_string()], Some(words::TO), false),
            part(
                "",
                vec![upper.to_string()],
                Some(words::THE_FULL_STOP),
                true,
            ),
        ],
        Statement::RefusedWithAnErrorNamingTheSettingSoTheAttemptWouldFail => {
            vec![plainly(words::REFUSED_SO_THE_JOB_FAILS)]
        }
        Statement::RendersNothingAtAllAndStillExitsZero => {
            vec![plainly(words::RENDERS_NOTHING_AND_REPORTS_SUCCESS)]
        }
        Statement::AStepOfZeroWritesTheFirstFrameOnce => vec![plainly(words::A_STEP_OF_ZERO)],
        Statement::AnEndBelowTheStartRendersNothingAndExitsZero => {
            vec![plainly(words::AN_END_BELOW_THE_START)]
        }
    }
}

// ════════════════════════════════════════════════════════════════════════════
// Composing the panel
// ════════════════════════════════════════════════════════════════════════════

/// ***Uploading dependencies: 4 of 12 files.*** — the two figures values,
/// the words around them the sentence's.
fn the_dependencies_uploading(done: usize, whole: usize) -> ASentence {
    vec![
        with_values(words::UPLOADING_DEPENDENCIES, vec![done.to_string()], Some(words::OF)),
        a_figure(
            whole.to_string(),
            if whole == 1 { words::FILE_FULL_STOP } else { words::FILES_FULL_STOP },
        ),
    ]
}

fn plainly(said: &str) -> Words {
    Words {
        lead: Prose::new(said),
        values: Vec::new(),
        rest: None,
        the_rest_joins: false,
    }
}

fn with_values(lead: &str, values: Vec<String>, rest: Option<&str>) -> Words {
    part(lead, values, rest, false)
}

/// **One part of a sentence**: some words, then the values that belong beside
/// them, then more words. The page sets them side by side and joins nothing.
fn part(lead: &str, values: Vec<String>, rest: Option<&str>, joins: bool) -> Words {
    Words {
        lead: Prose::new(lead),
        values: values.into_iter().map(Verbatim::new).collect(),
        rest: rest.map(Prose::new),
        the_rest_joins: joins,
    }
}

/// A part that is a value and the words that follow it with nothing between.
fn a_figure(value: impl Into<String>, rest: &str) -> Words {
    part("", vec![value.into()], Some(rest), false)
}

/// **`Servers` H2a's mark, as a number the page formats.** The three lines are
/// copied from `src/surface/servers.rs`, whose own reader is private and whose
/// file this piece may not edit; making it `pub(crate)` and deleting this is a
/// one-line tidy for whoever may edit that file.
fn the_moment(at: std::time::SystemTime) -> String {
    at.duration_since(std::time::UNIX_EPOCH)
        .map(|since| since.as_secs().to_string())
        .unwrap_or_default()
}

/// The last component of a path, which is what the header and the strip name
/// — and what a job sent with no name is called (`the_queue.rs`,
/// `what_the_job_is_called`). **Kept once, beside that rule, in
/// `smoozy_queue_history::naming`**, so the page's file and an unnamed job's
/// name cannot be cut two ways (Vast-pass fix piece 3).
pub(crate) fn the_files_name(path: &str) -> String {
    smoozy_queue_history::naming::the_files_name(path)
}

/// **Where Files is sent by the file's name in a panel's title** — the folder
/// the file is in and the file's own name, both as bytes, with the link's
/// hover. `Design/Whole app § Panels over a server`, added 2026-10-12 (the
/// Creator's ninth notes, note 5): pressing the name collapses the panel and
/// shows Files on that folder with the file chosen. **The application sends
/// the two and the page composes no path** (`File Browser` L1a).
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct TheFileInFiles {
    /// The folder Files opens at — the path up to the file's own name.
    pub folder: Vec<u8>,
    /// The row Files chooses, by its name, as Files holds a chosen row.
    pub name: Vec<u8>,
    /// *Show in Files*, the link's hover.
    pub hover: Prose,
}

/// **The folder and the name of a file's path, on its bytes.** Nothing where
/// the path names no folder or no file — a panel opened with no file — and
/// then the title's name is not a link. Both panels call this one.
pub fn the_file_in_files(file: &[u8]) -> Option<TheFileInFiles> {
    let at = file.iter().rposition(|byte| *byte == b'/')?;
    let name = &file[at + 1..];
    if name.is_empty() {
        return None;
    }
    // **The top of the server is a folder too**: a file straight under it is
    // in `/`, never in nothing.
    let folder = if at == 0 { b"/".to_vec() } else { file[..at].to_vec() };
    Some(TheFileInFiles { folder, name: name.to_vec(), hover: Prose::new(words::SHOW_IN_FILES) })
}

/// **The whole panel, composed from the two objects and from nothing else.**
pub fn the_panel(held: &WhatTheRunHolds, server: &str, file: &str) -> TheRenderSettings {
    the_panel_starting_from(held, server, file, &WhatTheRecordHolds::default())
}

/// **`Telegram` V1a: *"A submission reads that value once, at composition."***
/// The app-wide default is the page identifier Settings stored, or nothing
/// where none was chosen; the job's starting choice is composed from it
/// through `Render Settings`' own hand-over, and the first of the three is
/// used only where no default is stored. fix-here item 9 of the round *setting it up, and
/// what the desktop keeps*.
///
/// **The stored default is any of the five from 2026-10-31** (`Telegram` V1a
/// as amended, `Telegram` V1d); an identifier Settings does not offer is no
/// default. *(From 2026-10-19 until then it was one of Settings' three, and a
/// stored* a picture every few minutes *was no default.)* The minutes are not
/// this function's: [`the_minutes_in_force`].
pub fn the_reporting_choice_composed(the_per_job_answer: Option<&str>, the_stored_default: Option<&str>) -> &'static str {
    let the_minutes = reporting::THE_MINUTES_TO_BEGIN_WITH;
    let the_app_wide_value = the_stored_default
        .and_then(|id| the_app_wide_choice_named(id, the_minutes))
        .unwrap_or(ReportingChoice::ALL[0]);
    let handed = reporting::the_hand_over(
        the_per_job_answer.and_then(|id| the_choice_named(id, the_minutes)),
        the_app_wide_value,
    );
    let at = ReportingChoice::ALL.iter().position(|one| *one == handed).unwrap_or(0);
    REPORTING_CHOICES[at].0
}

/// **What the closed Telegram control reads**: the row picked, except *A
/// picture every few minutes*, which reads *A picture every* so that the
/// minutes field and *minutes* finish the sentence beside it
/// (`Design/Render Settings § Submitting`).
fn what_the_reporting_control_reads(the_choice: &str) -> &'static str {
    if the_choice == A_PICTURE_EVERY_FEW_MINUTES {
        return words::A_PICTURE_EVERY;
    }
    REPORTING_CHOICES
        .iter()
        .find(|(value, _)| *value == the_choice)
        .map_or(words::EVERYTHING, |(_, said)| *said)
}

/// **The panel, with the reporting default Settings stored.**
pub fn the_panel_starting_from(
    held: &WhatTheRunHolds,
    server: &str,
    file: &str,
    record: &WhatTheRecordHolds,
) -> TheRenderSettings {
    let the_reporting_default = record.the_reporting_default.as_deref();
    let key = FileKey {
        server: an_identity(server),
        path: file.to_string(),
    };
    let mark = keyed(server, file);
    let standing = held.readings.standing(&key);
    let per_file = held.store.per_file(&key);
    let ticked: Vec<ExactName> = per_file
        .map(|per_file| {
            per_file
                .scenes_ticked
                .iter()
                .map(|one| one.name.clone())
                .collect()
        })
        .unwrap_or_default();
    let several = ticked.len() > 1;
    let scene_name = the_scene_in_hand(held, server, file);
    let scene: Option<&SceneReading> = standing.and_then(|reading| {
        let wanted = scene_name.as_ref()?;
        reading
            .settings
            .scenes
            .iter()
            .find(|scene| scene.name.as_bytes() == wanted.as_bytes())
    });
    let answers: SceneEntries = held
        .store
        .the_answers_composition_reads(&key, scene_name.as_ref());
    let submission = held.submission.get(&mark).cloned().unwrap_or_default();
    let the_reporting_choice = the_reporting_choice_composed(submission.what_it_reports, the_reporting_default);

    // ── The version, and what it decides ────────────────────────────────────
    let list = held.version_lists.get(server);
    let saved_with = the_saved_with_of(held, server, file, scene);
    let resolution = the_resolution_of(held, server, file, per_file, scene);
    let picked = resolution.the_payloads_pick();

    // ── The read's own state ────────────────────────────────────────────────
    let underway = held.reading_now.get(&mark).copied().unwrap_or(false);
    let the_read_failure = held
        .the_read_that_gave_nothing
        .get(&mark)
        .and_then(|ending| the_read_failure(ending, record.the_servers_name.as_deref(), picked.as_ref()));

    // ── The two bands ───────────────────────────────────────────────────────
    let the_band: ASentence = if several {
        vec![
            a_figure(ticked.len().to_string(), words::SCENES_ARE_TICKED_LEAD),
            a_figure(ticked.len().to_string(), words::SCENES_ARE_TICKED_REST),
        ]
    } else if standing.is_none() {
        // **J10's three drops leave the page exactly as it was before any read
        // except that the user's own values are still standing in it**, and the
        // band's wording is the one thing that differs.
        match held.the_read_that_gave_nothing.get(&mark) {
            Some(AReadThatGaveNothing::Ended(Ending::TheMachineCouldNotBeReached))
                if held.hardware.contains_key(server) =>
            {
                vec![
                    plainly(words::THE_CONNECTION_ENDED_LEAD),
                    plainly(words::THE_CONNECTION_ENDED_REST),
                ]
            }
            _ => vec![plainly(words::NOT_READ_LEAD), plainly(words::NOT_READ_REST)],
        }
    } else {
        Vec::new()
    };

    let came_from = held.where_it_came_from.get(&mark);
    let (where_it_came_from, waiting) = the_where_it_came_from(came_from.map(|came_from| &came_from.band));

    let engine_in_force = the_engine_in_force(&answers, standing, scene);

    // ── The left column ─────────────────────────────────────────────────────
    let scenes: Vec<ABox> = standing
        .map(|reading| {
            reading
                .settings
                .scenes
                .iter()
                .map(|one| ABox {
                    name: Verbatim::new(one.name.as_text().to_string()),
                    ticked: ticked.iter().any(|name| name.as_bytes() == one.name.as_bytes()),
                    tag: None,
                })
                .collect()
        })
        .unwrap_or_default();
    let all_scenes = standing.map(|reading| ABox {
        name: Verbatim::new(words::ALL_SCENES),
        ticked: !scenes.is_empty() && scenes.iter().all(|one| one.ticked),
        tag: Some(with_values(
            "",
            vec![reading.settings.scenes.len().to_string()],
            Some(words::JOBS_ONE_EACH),
        )),
    });

    let the_range = the_range_field(&answers, scene, several);
    let the_frame_count = the_frame_count(&answers, standing, &ticked, scene, several);
    // **Where every job the Render control would send has one frame, the
    // first-frame check has nothing to save** — the switch is greyed and the
    // band leaves its way out (the Creator's nineteenth notes, note 8).
    let every_job_has_one_frame = every_job_has_one_frame(&answers, standing, &ticked, scene, several);

    let the_frame = scene.map(|scene| {
        view_layers::the_frame_the_untouched_box_states(answers.frame_range.as_ref(), scene)
    });
    let the_view_layers: Vec<AViewLayer> = match (scene, several) {
        (Some(scene), false) => scene
            .view_layers
            .iter()
            .map(|layer| the_view_layer(layer, &answers, scene, the_frame.unwrap_or(0), engine_in_force))
            .collect(),
        _ => Vec::new(),
    };

    // ── The middle column ───────────────────────────────────────────────────
    // **The engine in force is the user's pick, otherwise the file's own where
    // it is one of the two offered** — the same answer [`the_engine`] draws the
    // control from (must-do 6). Until the step 10a round 1 audit of these
    // pieces an unpicked page fell back to Cycles, so an EEVEE file drew
    // Cycles' sample count in the file's face and the GPU/CPU pair Design does
    // not show under EEVEE. **Where neither says, nothing is assumed**: no
    // file's count is drawn and the pair stays, which is the unread page's own
    // reading.
    let samples = the_samples_field(&answers, scene, engine_in_force, several);
    let the_samples_lines = the_samples_lines(
        &answers,
        scene,
        engine_in_force,
        standing.map(|reading| reading.taken_by.version),
        several,
    );
    let (resolution_field, size, size_not_known) = the_resolution(&answers, scene, several);
    let engine = the_engine(&answers, standing, scene);
    let renders_on = the_renders_on(&answers, scene, held.hardware.get(server), several, engine_in_force);

    // ── The right column ────────────────────────────────────────────────────
    // **Only beside the reading it came with**: a reading dropped (J10, Q12)
    // takes its outputs half off the page with it.
    let outputs = standing.and_then(|_| the_outputs_in_hand(held, server, file));
    // **What the count is composed against: where this file sits on that
    // server, and this page's own View Layer pins** (holes 2 and 5 of the map
    // of the Creator's second notes).
    let mut ground = the_ground_on_this_page(file, &answers);
    // **Where the job's folder will sit, from the one place it is chosen**
    // (`Job files and outputs` §6.6, the render worker's own choice asked with
    // nothing made) — and the page's map is composed there, so the collision
    // warning names the path the files will really land at. The Creator's
    // second notes, piece 4.
    let sits = outputs.zip(record.the_layout.as_ref()).map(|(outputs, layout)| {
        where_the_jobs_folder_sits(layout, outputs, &ground, record.the_servers_fallback_folder.as_deref())
    });
    if let Some(sits) = &sits {
        ground.the_jobs_folder = Some(sits.the_pages_job_folder());
    }
    let (what_it_will_write, the_output_warnings, the_output_lines) =
        the_output_group(outputs, scene, &ticked, several, &ground);
    let (the_folder_line, the_second_folder_line) = match sits.as_ref().filter(|_| what_it_will_write.is_some()) {
        Some(sits) => (Some(the_folder_line(sits)), the_second_folder_line(sits)),
        None => (None, None),
    };

    let the_dependency_band = the_dependency_band(
        record,
        picked.as_ref(),
        // **On a single-frame submission the band leaves the first-frame way
        // out too**, for the same reason as where the check is already asked
        // for (`§ The dependency warning`, nineteenth notes, note 8).
        submission.the_first_frame_check || every_job_has_one_frame,
    );

    // ── The foot ────────────────────────────────────────────────────────────
    let the_render_control =
        the_render_control(&the_frame_count, ticked.len(), standing.is_some());
    // The figure the strip repeats, taken before the count is handed over.
    let the_folder_named = the_folder_named(&submission, record);
    let (the_format_summary, where_they_go) = if submission.download_as_it_renders {
        the_download_summary(
            &what_the_format_box_knows(outputs, &submission, record, &ground),
            the_folder_named.as_deref(),
            standing.is_some(),
        )
    } else {
        (None, None)
    };
    // **`§ Submitting`: the switch on and no folder anybody can name.** All
    // three sources are empty at once — nothing chosen for this job, no
    // standing folder, and `Desktop app and delivery` W5's route proposing
    // none — so [`the_act_as_the_page_composed_it`] hands over no destination
    // and `Download` R1's arrangement is refused with *there is nowhere to put
    // it*. **Nothing said this, and a job then rendered with the switch on and
    // its frames never came home.** The job still goes: `§ Before the file has
    // been read` has submitting work anyway, and nothing on this strip stops a
    // submission.
    let the_frames_have_nowhere_to_go =
        (submission.download_as_it_renders && the_folder_named.is_none()).then(|| TheNowhereLine {
            lead: Prose::new(words::NO_FOLDER_IS_CHOSEN),
            rest: Prose::new(words::THE_FRAMES_STAY_ON_THE_SERVER),
        });
    let the_strips_frame_figure = the_frame_count
        .first()
        .and_then(|first| first.values.first())
        .map(|figure| figure.as_str().to_string())
        .unwrap_or_default();

    TheRenderSettings {
        server: Verbatim::new(server.to_string()),
        file: Verbatim::new(file.to_string()),
        file_bytes: file.as_bytes().to_vec(),
        the_files_name: Verbatim::new(the_files_name(file)),
        heading: Prose::new(words::RENDER_SETTINGS),
        title: with_values(words::RENDER_SETTINGS_TITLE, vec![the_files_name(file)], None),
        show_in_files: the_file_in_files(file.as_bytes()),

        blender: Prose::new(words::BLENDER),
        blender_version: Prose::new(words::BLENDER_VERSION),
        the_versions: the_versions(list, picked.as_ref()),
        the_version_control_is_drawn: list.is_some_and(|list| !list.entries.is_empty()),
        no_blender: the_no_blender_line(&resolution),
        set_it_up: Prose::new(words::SET_IT_UP),
        // **A server being searched, or one that could not be, is not a
        // server that is missing** — the line says which.
        the_pick_line: match (list, held.version_searches.get(server)) {
            (None, Some(TheSearchStands::Searching)) => Some(Prose::new(words::SEARCHING_THIS_SERVER)),
            (None, Some(TheSearchStands::TheMachineCouldNotBeAsked)) => {
                Some(Prose::new(words::THE_SERVER_COULD_NOT_BE_SEARCHED))
            }
            _ => the_pick_line(&resolution),
        },
        the_search_is_underway: list.is_none()
            && matches!(held.version_searches.get(server), Some(TheSearchStands::Searching)),
        the_search_marks: the_search_marks(list),
        the_mismatch: the_mismatch(saved_with, picked.as_ref()),
        read_at: standing
            .map(|_| vec![plainly(words::READ_AT)])
            .unwrap_or_default(),
        read_the_file: Prose::new(words::READ_THE_FILE),
        read_again: Prose::new(words::READ_AGAIN),
        reading_the_file: Prose::new(words::READING_THE_FILE),
        a_reading_stands: standing.is_some(),
        the_read_is_offered: resolution.the_read_is_offered(),
        the_read_is_underway: underway,
        reset_all: Prose::new(words::RESET_ALL),

        the_band,
        where_it_came_from,
        where_it_came_from_is_waiting: waiting,
        where_it_came_from_moment: came_from.and_then(|came_from| came_from.the_moment.clone()),
        the_read_failure,

        scene_heading: Prose::new(words::SCENE),
        the_scene_group_is_drawn: !scenes.is_empty(),
        the_scenes: scenes,
        all_scenes,
        frames_heading: Prose::new(words::FRAMES),
        // **`Design/Render Settings § Frames`: where the user's range stands,
        // *the file's* is not drawn** — the bold value already says whose it
        // is, and *the file's* beside it is false (step 10c's Sesterce-pass
        // fix piece 1). *One range for all 3* stays: it is true beside a typed
        // group range (`§ The body is three columns`).
        frames_qualifier: if several {
            Some(with_values(
                words::ONE_RANGE_FOR_ALL,
                vec![ticked.len().to_string()],
                None,
            ))
        } else if answers.frame_range.is_some() {
            None
        } else {
            Some(plainly(words::THE_FILES_RANGE))
        },
        the_range,
        the_frame_count,
        the_range_forms: the_range_forms(),
        how_to_type_a_range: Prose::new(words::HOW_TO_TYPE_A_RANGE),
        view_layers_heading: Prose::new(words::VIEW_LAYERS),
        view_layers_qualifier: the_frame.filter(|_| !several).map(|frame| {
            with_values(words::READ_AT_FRAME, vec![frame.to_string()], None)
        }),
        the_view_layer_group_is_drawn: !the_view_layers.is_empty(),
        the_view_layers,

        render_heading: Prose::new(words::RENDER),
        the_render_group_is_greyed: several,
        samples,
        the_samples_lines,
        simplify: the_simplify(came_from, &answers, scene),
        resolution: resolution_field,
        the_size_reading: size,
        the_size_is_not_known: size_not_known,
        the_refusal_line: Vec::new(),
        engine,
        renders_on,

        output_heading: Prose::new(words::OUTPUT_FILES),
        the_folder_line,
        the_second_folder_line,
        // **The *Output files* heading is drawn only where something is under
        // it** — the list, a warning, a line, or the one-scene line (must-do 15;
        // the drawing's own script: *"a page with none has nothing to put
        // there"*).
        the_output_group_is_drawn: what_it_will_write.is_some()
            || !the_output_warnings.is_empty()
            || !the_output_lines.is_empty()
            || several,
        what_it_will_write,
        the_output_warnings,
        the_output_lines,
        one_scene_only: if several {
            vec![part(
                words::READ_FROM_ONE_SCENE_LEAD,
                vec![ticked.len().to_string()],
                Some(words::READ_FROM_ONE_SCENE_REST),
                false,
            )]
        } else {
            Vec::new()
        },
        dependencies_heading: Prose::new(words::DEPENDENCIES),
        the_dependencies_group_is_drawn: the_dependency_band.is_some(),
        the_dependency_band,

        name_this_job: Prose::new(words::NAME_THIS_JOB),
        the_job_name: Verbatim::new(submission.job_name.clone()),
        where_the_name_shows: Prose::new(words::WHERE_THE_NAME_SHOWS),
        download_as_they_render: Prose::new(words::DOWNLOAD_AS_THEY_RENDER),
        the_download_is_on: submission.download_as_it_renders,
        the_format_summary,
        where_they_go,
        the_format_list_is_not_offered: (submission.download_as_it_renders && standing.is_none())
            .then(|| Prose::new(words::FORMATS_COME_FROM_THE_FILE)),
        the_frames_have_nowhere_to_go,
        the_dependencies_uploading: record
            .the_checks_uploads
            .map(|(done, whole)| the_dependencies_uploading(done, whole)),
        render_one_frame_first: Prose::new(words::RENDER_ONE_FRAME_FIRST),
        the_first_frame_is_asked_for: submission.the_first_frame_check && !every_job_has_one_frame,
        the_first_frame_is_answered_in_queue: (record.telegram_is_on
            && the_reporting_choice == "nothing"
            && submission.the_first_frame_check && !every_job_has_one_frame)
            .then(|| Prose::new(words::ANSWER_IN_QUEUE)),
        the_first_frame_switch_is_greyed: every_job_has_one_frame,
        telegram: Prose::new(words::TELEGRAM),
        the_telegram_line_is_drawn: record.telegram_is_on,
        the_reporting_choices: REPORTING_CHOICES
            .iter()
            .map(|(value, said)| AChoice {
                value: Verbatim::new(*value),
                said: Prose::new(*said),
            })
            .collect(),
        the_reporting_choice: Verbatim::new(the_reporting_choice),
        the_reporting_control_reads: Prose::new(what_the_reporting_control_reads(the_reporting_choice)),
        the_minutes_between_pictures: (the_reporting_choice == A_PICTURE_EVERY_FEW_MINUTES)
            .then(|| Verbatim::new(the_minutes_in_force(&submission, record).to_string())),
        minutes: Prose::new(words::MINUTES),
        minutes_between_pictures: Prose::new(words::MINUTES_BETWEEN_PICTURES),
        the_render_control,
        // **One act at a time from one page**: while a submission runs, the
        // control is not offered again (step 10c piece 6).
        submitting_is_offered: resolution.submitting_is_offered()
            && !held.submitting_now.get(&mark).copied().unwrap_or(false),
        the_submission_is_underway: held.submitting_now.get(&mark).copied().unwrap_or(false),
        the_machines_words_subject: Verbatim::new(
            crate::record::failure_records::DurableSubject::ServerRow(an_identity(server))
                .kind_as_text(),
        ),

        the_strip_title: with_values(
            words::RENDER_SETTINGS,
            vec![the_files_name(file)],
            None,
        ),
        // **What the strip says while the panel is down** — the scene in hand
        // and the frames it covers. **The drawing's own strip reads a third
        // part, how many settings the user changed**, and it is not composed
        // here: `theStripReads` takes one `Words`, which carries one run of
        // words and one run of values, and three parts separated by the
        // drawing's own middle dots do not fit in it. **The narrowing is this
        // piece's and the shape is piece 5's** — a strip reading that took a
        // sentence rather than one `Words` would carry all three, and that is
        // one line in a file this piece may not edit.
        the_strip_reading: scene.map(|scene| {
            with_values(
                "",
                vec![
                    scene.name.as_text().to_string(),
                    the_strips_frame_figure.clone(),
                ],
                Some(words::FRAMES_WORD),
            )
        }),
    }
}

fn the_release_line(saved_with: &str) -> Option<version::ReleaseLine> {
    let mut parts = saved_with.split('.');
    let major: u32 = parts.next()?.trim().parse().ok()?;
    let minor: u32 = parts.next().unwrap_or("0").trim().parse().ok()?;
    Some(version::ReleaseLine { major, minor })
}

fn the_versions(list: Option<&TheListThisPageAsksFor>, picked: Option<&BlenderIdentity>) -> Vec<AVersion> {
    let Some(list) = list else {
        return Vec::new();
    };
    list.entries
        .iter()
        .filter_map(|entry| {
            let answered = entry.answered?;
            let line = version::ReleaseLine::of(answered);
            Some(AVersion {
                build: Verbatim::new(entry.path.clone()),
                answered: Verbatim::new(version_text(answered)),
                below_the_floor: (line < version::ReleaseLine::of(smoozy_server_setup::blender::THE_FLOOR))
                    .then(|| Prose::new(words::BELOW_THE_FLOOR)),
                picked: picked.is_some_and(|picked| picked.path == entry.path),
            })
        })
        .collect()
}

/// **`§ The header carries the Blender version`: the line after the version
/// says how the pick was made, and only that** — three outcomes, and the third
/// does not share a word with the other two.
fn the_pick_line(resolution: &Resolution) -> Option<Prose> {
    match resolution {
        Resolution::Automatic { how, .. } => Some(Prose::new(match how {
            HowItMatched::TheExactLine => words::MATCHED_TO_THE_FILE,
            HowItMatched::TheNearestLineAbove => words::NEAREST_ABOVE,
            HowItMatched::TheNearestLineBelow => words::NEAREST_BELOW,
        })),
        Resolution::TheUsersOverride(_) => None,
        Resolution::NoVersionResolves(WhyNone::NothingButBuildsBelowTheFloor) => Some(Prose::new(
            words::SMOOZY_DOES_NOT_CHOOSE_BELOW_THE_FLOOR,
        )),
        Resolution::NoVersionResolves(WhyNone::NoSavedWithVersionToMatchOn) => {
            Some(Prose::new(words::NOTHING_TO_MATCH_ON))
        }
        Resolution::NoVersionResolves(WhyNone::NoMachineInHand) => {
            Some(Prose::new(words::NO_MACHINE_IN_HAND))
        }
        // **With no Blender there is no pick to describe and the line is not
        // drawn at all**, which is the drawing's own answer.
        Resolution::NoVersionResolves(WhyNone::TheListIsEmpty) => None,
    }
}

fn the_no_blender_line(resolution: &Resolution) -> ASentence {
    if matches!(
        resolution,
        Resolution::NoVersionResolves(WhyNone::TheListIsEmpty)
    ) {
        vec![plainly(words::NO_BLENDER_ON_THIS_SERVER)]
    } else {
        Vec::new()
    }
}

/// **`Server Setup` B11: a build that did not answer when asked is named, and
/// nothing else about the search is drawn.** On an ordinary machine there is
/// none and the line is not there at all. **The folders the search could not
/// read and the search going by name stay in its answer and the diary** —
/// Creator-stated 2026-10-02, note 13 on 0.1.7. *(Until then both were drawn
/// where true, and the by-name line was true on every machine.)*
fn the_search_marks(list: Option<&TheListThisPageAsksFor>) -> Vec<Words> {
    let Some(list) = list else {
        return Vec::new();
    };
    let mut out = Vec::new();
    for path in &list.answer.did_not_answer {
        out.push(Words {
            lead: Prose::new(String::new()),
            values: vec![Verbatim::new(path.clone())],
            rest: Some(Prose::new(words::DID_NOT_ANSWER)),
            the_rest_joins: false,
        });
    }
    out
}

/// **`§ The version mismatch warning`** — one line where the picked version and
/// the file's are different release lines, naming the two and saying something
/// may not work as expected. **It opens nothing** (`Render Settings` B8 as
/// rewritten 2026-10-02, the Creator's note 12 on 0.1.7).
fn the_mismatch(
    saved_with: Option<version::ReleaseLine>,
    picked: Option<&BlenderIdentity>,
) -> Option<TheMismatch> {
    let picked = picked?;
    let files = saved_with?;
    let rendering = version::ReleaseLine::of(picked.version);
    if files == rendering {
        return None;
    }
    Some(TheMismatch {
        line: vec![
            part(
                words::SAVED_WITH,
                vec![format!("{}.{}", files.major, files.minor)],
                Some(words::THE_COMMA),
                true,
            ),
            part(
                words::RENDERING_WITH,
                vec![format!("{}.{}", rendering.major, rendering.minor)],
                Some(words::THE_FULL_STOP),
                true,
            ),
            plainly(words::SOME_THINGS_MAY_NOT_WORK),
        ],
    })
}

/// **The plain sentence each of Q8's endings is said with**, before the
/// typography point. **One composition**, so the sentence the page shows and the
/// one the diary is handed (M32's second limb) cannot differ. A reading is not
/// an ending and has none.
fn the_read_failures_sentence(ending: &AReadThatGaveNothing) -> Option<&'static str> {
    let ending = match ending {
        // **The one case beside the four**: the sentence the page already says
        // when the file on the server cannot be read at Submit.
        AReadThatGaveNothing::TheServerRanNoBlender => {
            return Some(words::THE_FILE_ON_THE_SERVER_COULD_NOT_BE_READ)
        }
        AReadThatGaveNothing::Ended(ending) => ending,
    };
    match ending {
        Ending::AReading(_) => None,
        Ending::TheMachineCouldNotBeReached => Some(words::COULD_NOT_REACH_THE_SERVER),
        Ending::TheFileCouldNotBeOpenedByThisVersion => Some(words::COULD_NOT_OPEN_THIS_FILE),
        Ending::TheReadRanAndSomethingInItFailed
        | Ending::NotAReadingAndNoneOfTheThreeEndingsFits => Some(words::THE_READ_DID_NOT_FINISH),
        Ending::SmoozyStoppedIt => Some(words::SMOOZY_STOPPED_THE_READ),
    }
}

/// **`§ When the read gives nothing`: four endings, not one**, each saying
/// something different, and **only the second carries a second sentence**.
fn the_read_failure(
    gave: &AReadThatGaveNothing,
    the_servers_name: Option<&str>,
    picked: Option<&BlenderIdentity>,
) -> Option<TheReadFailure> {
    let what_did_not_happen = the_read_failures_sentence(gave)?;
    let ending = match gave {
        // **No Blender ran, so there is no Blender output to open and nothing
        // Smoozy knows the user should do next** — one sentence and the cross.
        AReadThatGaveNothing::TheServerRanNoBlender => {
            return Some(TheReadFailure {
                which: Verbatim::new("the-server-ran-no-blender"),
                what_did_not_happen: Prose::new(what_did_not_happen),
                what_to_do_about_it: None,
                values: Vec::new(),
                the_raw_text: None,
                clear_this: Prose::new(crate::presentation::the_failure_shape::CLEAR_THIS),
            })
        }
        AReadThatGaveNothing::Ended(ending) => ending,
    };
    let (which, what_to_do, values, raw) = match ending {
        Ending::AReading(_) => return None,
        // **Ending 1. This ending is not about the file at all**, and anything
        // said about the `.blend` would be a claim Smoozy cannot make. **The
        // value beside it is what the user calls the server** — never the
        // local identifier, which is a key (must-do 5) — and where the record
        // would not give the name, no value is drawn at all.
        Ending::TheMachineCouldNotBeReached => (
            "the-machine",
            None,
            the_servers_name
                .map(|name| AChoice {
                    value: Verbatim::new(name.to_string()),
                    said: Prose::new(words::THE_SERVER),
                })
                .into_iter()
                .collect(),
            None,
        ),
        // **Ending 2 is the one of the four where Smoozy knows what to do
        // next**, so it is the one that carries a second sentence. **The
        // version named is Smoozy's own record of which build it ran**, never
        // anything read out of Blender's words.
        Ending::TheFileCouldNotBeOpenedByThisVersion => (
            "the-version",
            Some(Prose::new(words::PICK_A_DIFFERENT_BLENDER)),
            vec![AChoice {
                value: Verbatim::new(
                    picked
                        .map(|picked| version_text(picked.version))
                        .unwrap_or_default(),
                ),
                said: Prose::new(words::BLENDER),
            }],
            None,
        ),
        // **Ending 3. No cause is offered and no second sentence is written**,
        // because Smoozy does not know what the user should do next; what
        // Blender printed is one control away, the same panel every other
        // screen opens.
        Ending::TheReadRanAndSomethingInItFailed
        | Ending::NotAReadingAndNoneOfTheThreeEndingsFits => (
            "the-read",
            None,
            Vec::new(),
            Some(Prose::new(words::BLENDER_OUTPUT)),
        ),
        // **A read Smoozy ended itself is none of the three**, and it exists as
        // its own ending because without it this case looks exactly like the
        // second — and the page would tell the user their file cannot be
        // opened, which is false.
        Ending::SmoozyStoppedIt => ("smoozy-stopped-it", None, Vec::new(), None),
    };
    Some(TheReadFailure {
        which: Verbatim::new(which),
        what_did_not_happen: Prose::new(what_did_not_happen),
        what_to_do_about_it: what_to_do,
        values,
        the_raw_text: raw,
        clear_this: Prose::new(crate::presentation::the_failure_shape::CLEAR_THIS),
    })
}

fn the_where_it_came_from(came_from: Option<&WhereItCameFrom>) -> (ASentence, bool) {
    let Some(came_from) = came_from else {
        return (Vec::new(), false);
    };
    match came_from {
        WhereItCameFrom::SubmittingAgain { the_jobs_name, when } => (
            vec![
                plainly(words::SUBMITTING_AGAIN_LEAD),
                part(
                    words::SUBMITTING_AGAIN_MIDDLE,
                    vec![the_jobs_name.clone()],
                    Some(words::USED_ON),
                    false,
                ),
                part(
                    "",
                    vec![when.clone()],
                    Some(words::SUBMITTING_AGAIN_REST),
                    true,
                ),
            ],
            false,
        ),
        WhereItCameFrom::SubmittingAgainWithNothingConnected { the_jobs_name, when } => (
            vec![
                plainly(words::NO_SERVER_CONNECTED_LEAD),
                part(
                    words::SUBMITTING_AGAIN_MIDDLE,
                    vec![the_jobs_name.clone()],
                    Some(words::USED_ON),
                    false,
                ),
                a_figure(when.clone(), words::THE_FULL_STOP),
                plainly(words::NO_SERVER_CONNECTED_REST),
            ],
            true,
        ),
        WhereItCameFrom::CarryingOn {
            the_jobs_name,
            when,
            the_last_frame_written,
            the_range_now_starts_at,
        } => (
            vec![
                part(
                    words::CARRYING_ON_LEAD,
                    vec![the_jobs_name.clone()],
                    Some(words::THE_FULL_STOP),
                    true,
                ),
                part(
                    words::THESE_ARE_THE_SETTINGS_IT_USED_ON,
                    vec![when.clone()],
                    Some(words::THE_FULL_STOP),
                    true,
                ),
                part(
                    words::CARRYING_ON_FRAME,
                    vec![the_last_frame_written.to_string()],
                    Some(words::CARRYING_ON_CHECKED),
                    false,
                ),
                part(
                    "",
                    vec![the_range_now_starts_at.to_string()],
                    Some(words::THE_FULL_STOP),
                    true,
                ),
                plainly(words::CARRYING_ON_SHORT_FILE),
            ],
            false,
        ),
        WhereItCameFrom::CarryingOnWithTheOutputsUnread {
            the_jobs_name,
            when,
            the_range_now_starts_at,
        } => (
            vec![
                part(
                    words::CARRYING_ON_LEAD,
                    vec![the_jobs_name.clone()],
                    Some(words::THE_FULL_STOP),
                    true,
                ),
                part(
                    words::THESE_ARE_THE_SETTINGS_IT_USED_ON,
                    vec![when.clone()],
                    Some(words::THE_FULL_STOP),
                    true,
                ),
                part(
                    words::CARRYING_ON_UNREAD,
                    vec![the_range_now_starts_at.to_string()],
                    Some(words::THE_FULL_STOP),
                    true,
                ),
            ],
            false,
        ),
    }
}

/// **`§ Frames`: one field, not two**, taking either form.
fn the_range_field(
    answers: &SceneEntries,
    scene: Option<&SceneReading>,
    several: bool,
) -> AField {
    let the_files = scene.map(|scene| {
        a_range_as_the_field_writes_it(scene.frame_start, scene.frame_end, scene.frame_step)
    });
    let the_users = answers.frame_range.as_ref().map(|form| {
        answers.frame_range_text.clone().unwrap_or_else(|| written_out(form))
    });
    // **`§ Scenes`, *More than one ticked*: the range field is the one control
    // offering something rather than reporting something, so it is live and
    // empty.**
    let whose = if several && the_users.is_none() {
        "the-users"
    } else {
        match store::shown(the_users.clone(), the_files.clone()) {
            store::Shown::TheUsers(_) => "the-users",
            store::Shown::TheFiles(_) => "the-files",
            store::Shown::PromiseForm => "the-promise-form",
        }
    };
    AField {
        label: Prose::new(words::RANGE),
        whose: Verbatim::new(whose),
        value: if several {
            the_users.map(Verbatim::new)
        } else {
            the_users.or(the_files.clone()).map(Verbatim::new)
        },
        hand_back_to: answers.frame_range.as_ref().and(the_files).map(|files| {
            Words {
                lead: Prose::new(words::BACK_TO_THE_FILES_VALUE),
                values: vec![Verbatim::new(files)],
                rest: None,
                the_rest_joins: false,
            }
        }),
        greyed: false,
        refused: false,
    }
}

/// **A range as the range field writes it** — `1-240`, or `1-240x2` where the
/// step is not one. **One writer for the field and for History's *Frames***,
/// for file values and older entries that carry no accepted display text.
pub(crate) fn a_range_as_the_field_writes_it(first: i64, last: i64, step: i64) -> String {
    if step == 1 {
        format!("{first}-{last}")
    } else {
        format!("{first}-{last}x{step}")
    }
}

/// **The canonical fallback for a range without display text** — the triple by
/// [`a_range_as_the_field_writes_it`], a list as its frames.
pub(crate) fn written_out(form: &FrameForm) -> String {
    match form {
        FrameForm::Triple { first, last, step } => {
            a_range_as_the_field_writes_it(*first, *last, *step)
        }
        FrameForm::List(frames) => frames
            .iter()
            .map(|frame| frame.to_string())
            .collect::<Vec<_>>()
            .join(","),
    }
}

/// **F2: the frame count reads the one parsed form**, and with more than one
/// scene ticked it is the group's.
fn the_frame_count(
    answers: &SceneEntries,
    standing: Option<&smoozy_render_settings::file_half::RecordedReading>,
    ticked: &[ExactName],
    scene: Option<&SceneReading>,
    several: bool,
) -> ASentence {
    if several {
        let each: Vec<u64> = standing
            .map(|reading| {
                reading
                    .settings
                    .scenes
                    .iter()
                    .filter(|one| ticked.iter().any(|name| name.as_bytes() == one.name.as_bytes()))
                    .map(|one| one.frame_count)
                    .collect()
            })
            .unwrap_or_default();
        let total = frames::the_group_count(answers.frame_range.as_ref(), &each);
        return vec![
            a_figure(total.to_string(), words::FRAMES_ACROSS),
            a_figure(ticked.len().to_string(), words::SCENES_WORD),
        ];
    }
    let count = match (&answers.frame_range, scene) {
        (Some(form), _) => form.count(),
        (None, Some(scene)) => scene.frame_count,
        (None, None) => return Vec::new(),
    };
    vec![a_figure(
        count.to_string(),
        if count == 1 {
            words::FRAME_WORD
        } else {
            words::FRAMES_WORD
        },
    )]
}

/// **`§ Submitting`: every job the Render control would send has one frame**
/// — one scene whose range is one frame, or several scenes of one frame each.
/// It reads the same counts as [`the_frame_count`]: with several scenes ticked,
/// a typed group range of one frame, or every ticked scene's own range one
/// frame where none was typed. **Where a count is not known it is not one
/// frame**, so the switch stays as it was. The Creator's nineteenth notes,
/// note 8, 2026-10-27.
fn every_job_has_one_frame(
    answers: &SceneEntries,
    standing: Option<&smoozy_render_settings::file_half::RecordedReading>,
    ticked: &[ExactName],
    scene: Option<&SceneReading>,
    several: bool,
) -> bool {
    if several {
        if let Some(form) = answers.frame_range.as_ref() {
            return form.count() == 1;
        }
        let each: Vec<u64> = standing
            .map(|reading| {
                reading
                    .settings
                    .scenes
                    .iter()
                    .filter(|one| ticked.iter().any(|name| name.as_bytes() == one.name.as_bytes()))
                    .map(|one| one.frame_count)
                    .collect()
            })
            .unwrap_or_default();
        return each.len() == ticked.len() && each.iter().all(|count| *count == 1);
    }
    match (&answers.frame_range, scene) {
        (Some(form), _) => form.count() == 1,
        (None, Some(scene)) => scene.frame_count == 1,
        (None, None) => false,
    }
}

/// The tooltip's four forms — the value and what it means.
fn the_range_forms() -> Vec<AChoice> {
    [
        ("1-240", words::EVERY_FRAME),
        ("1-240x2", words::EVERY_SECOND),
        ("1,5,9", words::JUST_THOSE),
        ("1-10, 40", words::MIXED),
    ]
    .into_iter()
    .map(|(form, said)| AChoice {
        value: Verbatim::new(form),
        said: Prose::new(said),
    })
    .collect()
}

/// **`§ View Layers`: the setting where what is shown and what the job does are
/// not the same thing, so it carries the most marking.**
fn the_view_layer(
    layer: &smoozy_render_settings::reading::ViewLayerReading,
    answers: &SceneEntries,
    scene: &SceneReading,
    frame: i64,
    engine_in_force: Option<OfferedEngine>,
) -> AViewLayer {
    let pinned = answers
        .pins
        .iter()
        .find(|pin| pin.layer.name.as_bytes() == layer.name.as_bytes());
    let answer = view_layers::the_box(layer, frame);
    let ticked = match pinned {
        Some(pin) => pin.on,
        None => match answer {
            FrameAnswer::Evaluated(renders)
            | FrameAnswer::Extrapolated(renders)
            | FrameAnswer::FromTheCurve { value: renders, .. } => renders,
            FrameAnswer::NotKnownWithoutARead => false,
        },
    };
    // **A pinned box carries no diamond and no reading of a later frame** (V4,
    // V6): the pin severs the file's animation, so the curve decides nothing.
    let diamond = match pinned {
        Some(_) => Diamond::None,
        None => view_layers::the_diamond(layer, frame),
    };
    let count = engine_in_force.map(|engine| {
        samples::the_count_in_force_for(answers.sample_count, engine, scene, layer)
    });
    AViewLayer {
        name: Verbatim::new(layer.name.as_text().to_string()),
        ticked,
        whose: Verbatim::new(if pinned.is_some() {
            "the-users"
        } else {
            "the-files"
        }),
        diamond: match diamond {
            Diamond::None => None,
            Diamond::Solid => Some(Verbatim::new("solid")),
            Diamond::Hollow => Some(Verbatim::new("hollow")),
        },
        diamond_said: match diamond {
            Diamond::None => None,
            Diamond::Solid => Some(with_values(
                words::ANIMATED_KEYED_ON_FRAME,
                vec![frame.to_string()],
                None,
            )),
            Diamond::Hollow => Some(with_values(
                words::ANIMATED_NOT_KEYED_ON_FRAME,
                vec![frame.to_string()],
                None,
            )),
        },
        not_known: matches!(answer, FrameAnswer::NotKnownWithoutARead)
            .then(|| Prose::new(words::NOT_KNOWN_WITHOUT_A_READ)),
        // **A View Layer's own sample count is not the switch, so that tag
        // stays on a pinned box like any other.**
        tag: layer.overrides_the_scenes_count.then(|| Words {
            lead: Prose::new(String::new()),
            values: vec![Verbatim::new(layer.own_sample_count.to_string())],
            rest: Some(Prose::new(if answers.sample_count.is_some() {
                words::SAMPLES_OVERRIDDEN
            } else {
                words::SAMPLES_OF_ITS_OWN
            })),
            the_rest_joins: false,
        }),
        hand_back_to: pinned.map(|_| {
            with_values(
                words::BACK_TO_THE_FILES_VALUE_AT_FRAME,
                vec![frame.to_string()],
                None,
            )
        }),
    }
    .with_count_unused(count)
}

impl AViewLayer {
    /// The count in force is read so that C6's split is exercised rather than
    /// only asserted; nothing on the box draws it beyond the tag above, which
    /// carries the layer's own figure by Design's own rule that the tag names
    /// the layer's number.
    fn with_count_unused(self, _count: Option<u64>) -> AViewLayer {
        self
    }
}

/// **`§ The sample count`: the box has two readings and one act tells them
/// apart — whether the user has typed.**
fn the_samples_field(
    answers: &SceneEntries,
    scene: Option<&SceneReading>,
    engine: Option<OfferedEngine>,
    several: bool,
) -> AField {
    let the_files = scene.zip(engine).map(|(scene, engine)| match engine {
        OfferedEngine::Cycles => scene.sample_counts.cycles,
        OfferedEngine::Eevee => scene.sample_counts.eevee,
    });
    AField {
        label: Prose::new(words::SAMPLES),
        whose: Verbatim::new(match store::shown(answers.sample_count, the_files) {
            store::Shown::TheUsers(_) => "the-users",
            store::Shown::TheFiles(_) => "the-files",
            store::Shown::PromiseForm => "the-promise-form",
        }),
        value: if several {
            None
        } else {
            answers
                .sample_count
                .or(the_files)
                .map(|count| Verbatim::new(count.to_string()))
        },
        hand_back_to: answers.sample_count.and(the_files).map(|files| Words {
            lead: Prose::new(words::BACK_TO_THE_FILES_VALUE),
            values: vec![Verbatim::new(files.to_string())],
            rest: None,
            the_rest_joins: false,
        }),
        greyed: several,
        refused: false,
    }
}

/// **The two lines under the box, and neither is drawn before the file has been
/// read** — whether there is anything to set aside is a fact about the file's
/// insides.
///
/// **`Render Settings` C3, and it is what the engine the page shows decides —
/// step 10c's dependency wave piece 3, fault 7 of the splash scenes.** The
/// early stop and the time limit are Cycles' alone, so they are named only
/// where the engine in force is Cycles: an EEVEE render carries neither,
/// whatever Cycles' own settings the file also holds, and `Design/Render
/// Settings § The sample count`'s *"Where the file carries neither, there is
/// no line"* is then the case. Until this piece the 4.5 splash scene, an EEVEE
/// file, was told it *stops early at noise threshold 0.01*. **A View Layer's
/// own count is not Cycles' alone**: EEVEE ignores it in 4.5 and obeys it from
/// 5.0 (`Render Settings` C3, observed since 2026-08-29), so under EEVEE it is named from 5.0 on
/// and not before. **The version is the one the reading was taken by**, which
/// `Render Settings` Q17 and `Render Settings` Q12 hold to the page's pick — the one that renders. **The engine is
/// [`the_engine_in_force`]'s**, the same answer the samples box and the device
/// pair are drawn from; where it names neither offered engine nothing is
/// claimed about the early stop.
fn the_samples_lines(
    answers: &SceneEntries,
    scene: Option<&SceneReading>,
    engine_in_force: Option<OfferedEngine>,
    taken_by: Option<smoozy_server_setup::blender::Version>,
    several: bool,
) -> Vec<ASentence> {
    let Some(scene) = scene else {
        return Vec::new();
    };
    if several {
        return Vec::new();
    }
    let under_cycles = engine_in_force == Some(OfferedEngine::Cycles);
    let the_layers_own_counts_count = match engine_in_force {
        Some(OfferedEngine::Eevee) => taken_by.is_some_and(|version| version.major >= 5),
        _ => true,
    };
    let the_early_stop_is_on = under_cycles && scene.early_stop.adaptive_sampling_is_on;
    let the_time_limit_is_set = under_cycles && scene.time_limit > 0.0;
    // **Composed afresh from the layers still in the job** — a layer a pin took
    // out is not in it, and neither is a layer the file itself keeps off at
    // every frame of the range in force, which is what makes the spec's three
    // endings one rule. **Whether a layer is in the job is `Render Settings`'
    // own judgement** ([`view_layers::renders_somewhere_in_the_range`],
    // `Render Settings` V12),
    // and this page calls it rather than keeping a second copy (must-do 8 of the
    // step 10a round 1 audit of these pieces).
    let overriding: Vec<String> = scene
        .view_layers
        .iter()
        .filter(|layer| layer.overrides_the_scenes_count)
        .filter(|layer| {
            let pinned = answers
                .pins
                .iter()
                .find(|pin| pin.layer.name.as_bytes() == layer.name.as_bytes())
                .map(|pin| pin.on);
            view_layers::renders_somewhere_in_the_range(layer, answers.frame_range.as_ref(), scene, pinned)
        })
        .map(|layer| layer.name.as_text().to_string())
        .collect();
    let the_layers_still_override = the_layers_own_counts_count
        && !overriding.is_empty()
        && scene.layer_samples_rule != LayerSamplesRule::Ignore;

    if answers.sample_count.is_some() {
        // **Typing a number replaces that line rather than adding to it**: the
        // typed line names what the number sets aside — the same settings by
        // the same names, plus any View Layer's own count — and then what that
        // costs.
        //
        // **What is set aside is gathered first**, then joined as one sentence:
        // commas between the parts and *and* before the last, as the untyped
        // line joins its own, and a full stop at the end. **The cost is said
        // only where something was set aside** — with nothing set aside the
        // typed count costs nothing the file's own setup did not (must-do 7).
        enum SetAside {
            TheThreshold(String),
            TheTimeLimit(String),
            TheLayersOwnCounts(Vec<String>),
        }
        let mut set_aside: Vec<SetAside> = Vec::new();
        if the_early_stop_is_on {
            set_aside.push(SetAside::TheThreshold(the_threshold(scene.early_stop.threshold)));
        }
        if the_time_limit_is_set {
            set_aside.push(SetAside::TheTimeLimit(the_time_limit(scene.time_limit)));
        }
        if the_layers_still_override {
            set_aside.push(SetAside::TheLayersOwnCounts(overriding));
        }
        if set_aside.is_empty() {
            return Vec::new();
        }
        let last = set_aside.len() - 1;
        let mut said: ASentence = Vec::new();
        for (at, one) in set_aside.into_iter().enumerate() {
            let (first_lead, and_lead, middle_lead, values) = match one {
                SetAside::TheThreshold(value) => (
                    words::OVERRIDES_LEAD,
                    words::OVERRIDES_LEAD,
                    words::OVERRIDES_LEAD,
                    vec![value],
                ),
                SetAside::TheTimeLimit(value) => (
                    words::OVERRIDES_A_TIME_LIMIT_OF,
                    words::AND_THE_TIME_LIMIT_OF,
                    words::A_TIME_LIMIT_OF,
                    vec![value],
                ),
                SetAside::TheLayersOwnCounts(names) => (
                    words::OVERRIDES_THE_OWN_COUNTS_OF,
                    words::AND_THE_OWN_COUNTS_OF,
                    words::AND_THE_OWN_COUNTS_OF,
                    names,
                ),
            };
            let lead = if at == 0 {
                first_lead
            } else if at == last {
                and_lead
            } else {
                middle_lead
            };
            let rest = if at == last {
                words::THE_FULL_STOP
            } else {
                words::THE_COMMA
            };
            said.push(part(lead, values, Some(rest), true));
        }
        said.push(plainly(words::EVERY_FRAME_NOW_RENDERS_ALL));
        return vec![said];
    }

    let mut out: Vec<ASentence> = Vec::new();
    // **What the file was going to do about stopping short is said either
    // way**, because whether a file stops itself short changes what a job costs
    // on a machine charged by the hour.
    if the_early_stop_is_on || the_time_limit_is_set {
        let mut said: ASentence = Vec::new();
        if the_early_stop_is_on {
            said.push(part(
                words::STOPS_EARLY_LEAD,
                vec![the_threshold(scene.early_stop.threshold)],
                Some(if the_time_limit_is_set {
                    words::THE_COMMA
                } else {
                    words::THE_FULL_STOP
                }),
                true,
            ));
        }
        if the_time_limit_is_set {
            // **The time limit says *a frame* because that is what it is**: it
            // is spent on each frame rather than across the job, so a long
            // animation pays it many times over.
            said.push(part(
                if said.is_empty() {
                    words::STOPS_AFTER_A_TIME_LIMIT_OF
                } else {
                    words::AND_AFTER_A_TIME_LIMIT_OF
                },
                vec![the_time_limit(scene.time_limit)],
                Some(words::ON_EACH_FRAME),
                false,
            ));
        }
        out.push(said);
    }
    // **`§ When the override warning goes`: a second line says which View
    // Layers do not render at the count on screen**, naming them rather than
    // repeating their numbers.
    if the_layers_still_override {
        out.push(vec![part(
            "",
            overriding,
            Some(words::RENDER_AT_THEIR_OWN_COUNT),
            false,
        )]);
    }
    out
}

/// **The noise threshold, written as the shortest decimal that is the same
/// single-precision number** — step 10c's dependency wave piece 3, fault 6 of
/// the splash scenes: the 0.01 every splash file holds was drawn
/// `0.009999999776482582`, where `Design/Render Settings § The sample count`'s
/// own example of this line reads *0.01*.
///
/// **Why single precision.** Blender's reference describes its floating-point
/// property as *"RNA floating-point number (single precision) property
/// definition"* (`bpy.types.FloatProperty`, the reference bundled with this
/// project's Blender tooling, read 2026-09-29), and the reading carries the
/// value as the nearest double to that single-precision number: 0.01 read back
/// as `0.009999999776482582` in all three splash readings (observed, the splash
/// scenes' account). **How to write a value Smoozy read is Smoozy's own
/// choice**, and the choice is the shortest text that reads back as the very
/// number the file holds — never a rounding to a figure the file does not hold.
fn the_threshold(threshold: f64) -> String {
    // A value outside single precision's range is written as it was read
    // rather than turned into infinity.
    let single = threshold as f32;
    if single.is_finite() {
        format!("{single}")
    } else {
        format!("{threshold}")
    }
}

/// **The time limit, written the same way and for the same reason** — the
/// same kind of number, so a limit of 0.1 s was drawn `0.10000000149011612s`.
fn the_time_limit(seconds: f64) -> String {
    format!("{}s", the_threshold(seconds))
}

/// **`§ The picture size`: the percentage is the setting; the pixels are a
/// reading beside it.**
fn the_resolution(
    answers: &SceneEntries,
    scene: Option<&SceneReading>,
    several: bool,
) -> (AField, Option<Verbatim>, Option<Prose>) {
    let the_files = scene.map(|scene| scene.resolution_percentage);
    let in_force = answers.resolution_percentage.or(the_files);
    let size = match (scene, in_force) {
        (Some(scene), Some(percentage)) if !several => {
            reading::the_size_at(scene.full_size, percentage)
        }
        _ => None,
    };
    let field = AField {
        label: Prose::new(words::RESOLUTION),
        whose: Verbatim::new(
            match store::shown(answers.resolution_percentage, the_files) {
                store::Shown::TheUsers(_) => "the-users",
                store::Shown::TheFiles(_) => "the-files",
                store::Shown::PromiseForm => "the-promise-form",
            },
        ),
        value: if several {
            None
        } else {
            in_force.map(|percentage| Verbatim::new(percentage.to_string()))
        },
        hand_back_to: answers
            .resolution_percentage
            .and(the_files)
            .map(|files| Words {
                lead: Prose::new(words::BACK_TO_THE_FILES_VALUE),
                values: vec![Verbatim::new(format!("{files}%"))],
                rest: None,
                the_rest_joins: false,
            }),
        greyed: several,
        refused: false,
    };
    match size {
        Some((across, down)) => (
            field,
            Some(Verbatim::new(format!("{across} × {down}"))),
            None,
        ),
        // **Where there is no size to work out, the reading says so rather than
        // going blank**, in the same words either way.
        None => (field, None, Some(Prose::new(words::SIZE_NOT_KNOWN))),
    }
}

/// **The engine in force**, which is what the samples field, the View Layer
/// counts and the device pair are drawn for: the user's pick, otherwise the
/// file's own where it is one of the two offered
/// ([`smoozy_render_settings::engine::what_the_page_shows`]). `None` where
/// neither says — nothing read, or a file naming a third engine.
fn the_engine_in_force(
    answers: &SceneEntries,
    standing: Option<&smoozy_render_settings::file_half::RecordedReading>,
    scene: Option<&SceneReading>,
) -> Option<OfferedEngine> {
    answers.engine_pick.or_else(|| match (scene, standing) {
        (Some(scene), Some(reading)) => match smoozy_render_settings::engine::what_the_page_shows(scene, reading) {
            WhatThePageShows::Offered(engine) => Some(engine),
            WhatThePageShows::TheFilesOwn(_) => None,
        },
        _ => None,
    })
}

/// **`§ The render engine`: Cycles and EEVEE are the choice, and a file naming
/// anything else is named beside them rather than refused.**
fn the_engine(
    answers: &SceneEntries,
    standing: Option<&smoozy_render_settings::file_half::RecordedReading>,
    scene: Option<&SceneReading>,
) -> TheEngine {
    let shows = match (scene, standing) {
        (Some(scene), Some(reading)) => {
            Some(smoozy_render_settings::engine::what_the_page_shows(scene, reading))
        }
        _ => None,
    };
    let on = match answers.engine_pick {
        Some(OfferedEngine::Cycles) => "cycles",
        Some(OfferedEngine::Eevee) => "eevee",
        None => match &shows {
            Some(WhatThePageShows::Offered(OfferedEngine::Cycles)) => "cycles",
            Some(WhatThePageShows::Offered(OfferedEngine::Eevee)) => "eevee",
            Some(WhatThePageShows::TheFilesOwn(_)) => "the-files",
            None => "the-files",
        },
    };
    let the_files_own = match &shows {
        Some(WhatThePageShows::TheFilesOwn(name)) => {
            Some(Verbatim::new(name.as_text().to_string()))
        }
        _ => None,
    };
    let add_on = standing
        .and_then(|reading| reading.settings.announced_add_on_case.as_ref())
        .map(|_| TheAddOnCase {
            lead: Words {
                lead: Prose::new(words::BLENDER),
                values: vec![Verbatim::new(
                    standing
                        .map(|reading| version_text(reading.taken_by.version))
                        .unwrap_or_default(),
                )],
                rest: Some(Prose::new(words::THE_ADD_ON_IS_MISSING_LEAD)),
                the_rest_joins: false,
            },
            read_back_as_eevee: Prose::new(words::THE_ENGINE_READ_BACK_AS_EEVEE),
        });
    TheEngine {
        label: Prose::new(words::ENGINE),
        the_files_own,
        the_files_own_said: shows
            .as_ref()
            .and_then(|shows| match shows {
                WhatThePageShows::TheFilesOwn(_) => Some(Prose::new(words::THE_FILES)),
                WhatThePageShows::Offered(_) => None,
            }),
        not_read_yet: standing
            .is_none()
            .then(|| Prose::new(words::THE_FILES_NOT_READ_YET)),
        cycles: Prose::new(words::CYCLES),
        eevee: Prose::new(words::EEVEE),
        on: Verbatim::new(on),
        hand_back_to: answers.engine_pick.and(shows.as_ref()).map(|shows| Words {
            lead: Prose::new(words::BACK_TO_THE_FILES_ENGINE),
            values: vec![Verbatim::new(match shows {
                WhatThePageShows::TheFilesOwn(name) => name.as_text().to_string(),
                WhatThePageShows::Offered(OfferedEngine::Cycles) => words::CYCLES.to_string(),
                WhatThePageShows::Offered(OfferedEngine::Eevee) => words::EEVEE.to_string(),
            })],
            rest: None,
            the_rest_joins: false,
        }),
        note: Prose::new(if standing.is_some() {
            words::LEAVE_IT_UNCHANGED_READ
        } else {
            words::LEAVE_IT_UNCHANGED_UNREAD
        }),
        the_add_on_case: add_on,
    }
}

/// **`§ What the render runs on`: GPU or CPU, as two buttons** — and **under
/// EEVEE the pair is not shown at all**, which is the page-visibility rule
/// `App/Render Settings/src/device.rs` says it does not own.
fn the_renders_on(
    answers: &SceneEntries,
    scene: Option<&SceneReading>,
    hardware: Option<&HeldHardware>,
    several: bool,
    engine_in_force: Option<OfferedEngine>,
) -> Option<TheDevice> {
    // **Under EEVEE the pair is not shown at all**: EEVEE only renders on the
    // cards, so there is nothing to choose, and a greyed pair would invite the
    // user to wonder what they had done wrong. `App/Render Settings/src/device.rs`
    // says in its own words that this is a page-visibility rule it does not own.
    // **It is the engine in force that decides**, the file's own as much as the
    // user's pick (must-do 6).
    if engine_in_force == Some(OfferedEngine::Eevee) {
        return None;
    }
    // **`§ Scenes`, *More than one ticked*: a control holding a value stays and
    // empties**, and the device pair holds a value — so it stays, greyed and
    // with nothing on (must-do 16). *(Until the step 10a round 1 audit of these
    // pieces it was taken away, which is what a group holding names does.)*
    if several {
        return Some(TheDevice {
            label: Prose::new(words::RENDERS_ON),
            gpu: Prose::new(words::GPU),
            cpu: Prose::new(words::CPU),
            on: Verbatim::new("nothing"),
            not_read_yet: None,
            tag: Vec::new(),
            the_moment: None,
            hand_back_to: None,
            greyed: true,
        });
    }
    // **`Render Settings` J1's three faces, and the user's comes first whether
    // or not a reading stands.** With a reading, `Render Settings` D1 resolves the choice from
    // the user's entry or the scene's own value. **With none, the user's own
    // entry is what is drawn** — `§ Before the file has been read`:
    // *"Everything the user can answer without a read, they can answer"*, and
    // *what it renders on* is on that list. Only where neither stands is the
    // setting on the promise form. **Until 2026-09-26 an unread page drew the
    // promise form whatever the user pressed**: step 10c's sixth pass pressed
    // *CPU* three times, the manifest carried the processor, and the page went
    // on showing *the file's — not read yet*, so a user would have believed the
    // job was going to the cards.
    let resolved = match scene {
        Some(scene) => Some(device::the_resolved_choice(answers.device, scene)),
        None => answers.device,
    };
    let (tag, the_moment) = the_device_tag(answers, scene, hardware);
    Some(TheDevice {
        label: Prose::new(words::RENDERS_ON),
        gpu: Prose::new(words::GPU),
        cpu: Prose::new(words::CPU),
        on: Verbatim::new(match resolved {
            Some(DeviceChoice::TheCards) => "the-cards",
            Some(DeviceChoice::TheProcessor) => "the-processor",
            None => "the-promise-form",
        }),
        not_read_yet: scene
            .is_none()
            .then(|| Prose::new(words::THE_FILES_NOT_READ_YET)),
        tag,
        the_moment: the_moment.map(Verbatim::new),
        hand_back_to: the_device_hand_back(answers, scene),
        greyed: false,
    })
}

/// **D5 and D6's tag, and the page never composes the judgement.** The warning
/// is shown only where both halves stand — the resolved device, which needs the
/// file read, and `Servers`' remembered hardware reading — and it is written
/// across both of that document's axes at once.
fn the_device_tag(
    answers: &SceneEntries,
    scene: Option<&SceneReading>,
    hardware: Option<&HeldHardware>,
) -> (ASentence, Option<String>) {
    match device::the_warning(
        answers.device,
        scene,
        hardware.unwrap_or(&HeldHardware::NeverRead),
    ) {
        Warning::NotShown(_) => (Vec::new(), None),
        Warning::Shown { presence, tense } => {
            let mut said: ASentence = Vec::new();
            match presence {
                Presence::TheCardsInFull { count } => said.push(a_figure(
                    count.to_string(),
                    if count == 1 {
                        words::GPU_UNUSED
                    } else {
                        words::GPUS_UNUSED
                    },
                )),
                // **No count is shown in either cannot-be-read state**:
                // `Servers` H5 makes the count the rows of a per-card query and
                // nothing else is read as one, so the tag states presence and
                // promises nothing.
                Presence::PresentAndCannotBeRead => said.push(plainly(words::GPUS_UNUSED)),
            }
            // **Never in the present tense off a *last seen* reading** (H2):
            // what is shown is what was last seen, with the reading's moment.
            let mut moment = None;
            if let Tense::LastSeen {
                the_readings_moment,
            } = tense
            {
                let seconds = the_moment(the_readings_moment);
                said.push(part(words::LAST_SEEN, vec![seconds.clone()], None, false));
                moment = Some(seconds);
            }
            (said, moment)
        }
    }
}

fn the_device_hand_back(answers: &SceneEntries, scene: Option<&SceneReading>) -> Option<Words> {
    answers.device.and(scene).map(|scene| {
        with_values(
            words::BACK_TO_THE_FILES_VALUE,
            vec![match scene.device {
                reading::DeviceSetting::TheCards => words::GPU,
                reading::DeviceSetting::TheProcessor => words::CPU,
            }
            .to_string()],
            None,
        )
    })
}

/// **`§ What it will write`: each destination, how many files, and which format
/// each writes — then the total, split by format.** With more than one scene
/// ticked the group's contents go and one line stands in their place.
#[allow(clippy::type_complexity)]
fn the_output_group(
    outputs: Option<&TheOutputsHalf>,
    scene: Option<&SceneReading>,
    ticked: &[ExactName],
    several: bool,
    ground: &TheGroundOfTheCount,
) -> (Option<TheWrites>, Vec<TheOutputWarning>, Vec<ASentence>) {
    if several {
        return (None, Vec::new(), Vec::new());
    }
    let Some(outputs) = outputs else {
        return (None, Vec::new(), Vec::new());
    };
    let AsTheJobRendersIt {
        outputs,
        map,
        inventory,
        a_typed_frame_lies_outside_the_read,
    } = the_inventory_of(outputs, ground);
    let outputs = &outputs;
    let six = the_six_things(outputs, &map, &inventory, ground.the_folder_composed_under());

    let mut warnings = Vec::new();
    if six.it_is_writing_nothing.is_some() {
        warnings.push(TheOutputWarning {
            which: Verbatim::new("writes-nothing"),
            lead: Prose::new(words::WRITES_NOTHING_LEAD),
            rest: Prose::new(words::WRITES_NOTHING_REST),
            pairs: Vec::new(),
        });
    }
    warnings.extend(the_collision_warning(&six.the_colliding_pairs));

    // **The destination list is not drawn at all where the job writes
    // nothing** — a heading with an empty list under it reads as a Smoozy that
    // failed to look.
    let writes = the_writes(
        &inventory,
        six.it_is_writing_nothing.is_some(),
        outputs,
        &map,
        ground.the_folder_composed_under(),
    );
    let (mut writes, warnings, mut lines) = the_rest_of_the_output_group(writes, warnings, &six, scene, ticked);
    // **A typed frame outside what the read stepped: no figures, and one line
    // in their place** — `Design/Render Settings § What it will write`, added
    // 2026-10-17 (the Creator's tenth notes, note 1). Where there would have
    // been no list anyway, there is nothing for the line to stand in for.
    if a_typed_frame_lies_outside_the_read && writes.is_some() {
        writes = None;
        lines.insert(0, vec![plainly(words::FRAMES_OUTSIDE_THE_FILES_RANGE)]);
    }
    // **`Job files and outputs` I9a: a scene that writes more than one view
    // separately has no inventory, and the page says so in one line** — the
    // same line its queue entry carries (the 38's piece 10, item 38).
    if matches!(
        inventory,
        WhatTheInventoryIs::TheFrameSetIsNotPredictable(
            smoozy_job_files::inventory::WhyTheFrameSetIsNotPredictable::TheSceneWritesMoreThanOneViewSeparately
        )
    ) {
        lines.push(vec![plainly(words::MORE_THAN_ONE_VIEW)]);
    }
    (writes, warnings, lines)
}

/// **The collision warning, naming every colliding pair** in the order
/// `Job files and outputs` gives them (must-do 14), or nothing where none
/// collides.
fn the_collision_warning(
    the_colliding_pairs: &[smoozy_job_files::inventory::ACollidingPair],
) -> Option<TheOutputWarning> {
    if the_colliding_pairs.is_empty() {
        return None;
    }
    {
        Some(TheOutputWarning {
            which: Verbatim::new("collide"),
            lead: Prose::new(words::COLLIDE_LEAD),
            rest: Prose::new(words::COLLIDE_REST),
            // **Each of the two names where it comes from as well as where it
            // lands** — I16a, and it is the whole of the warning's value: the
            // path alone is the same string twice. **Every pair, not the
            // first**: a user who fixed one and read again would otherwise meet
            // the next (must-do 14).
            pairs: the_colliding_pairs
                .iter()
                .map(|pair| {
                    pair.members
                        .iter()
                        .map(|side| {
                            let mut side_said = vec![Words {
                                lead: Prose::new(String::new()),
                                values: vec![Verbatim::new(
                                    String::from_utf8_lossy(&pair.predicted_path).to_string(),
                                )],
                                rest: None,
                                the_rest_joins: false,
                            }];
                            side_said.extend(where_it_comes_from(&side.origin));
                            side_said
                        })
                        .collect()
                })
                .collect(),
        })
    }
}

/// The destination list and its totals, where the job writes something.
///
/// **Each destination is its folder as it will sit under the job's folder** —
/// `PNG/`, `EXR/`, *top of the job's folder* — read off the map the page
/// composed (`Job files and outputs` P11 as corrected 2026-10-04), **never the
/// path the file records**: `//../Render/PNG` is not where the files go. Until
/// the Creator's second notes, piece 2, every row was [`the_place`], and the
/// Creator was told `//../Render/PNG` before a job whose files went to
/// `<job>/Render/PNG` (their note 18). **Formats are in Smoozy's words**
/// ([`a_format_in_smoozys_words`], their note 16 and Rule 9).
fn the_writes(
    inventory: &WhatTheInventoryIs,
    it_is_writing_nothing: bool,
    outputs: &TheOutputsHalf,
    map: &TheMap,
    job_folder: &str,
) -> Option<TheWrites> {
    match (inventory, it_is_writing_nothing) {
        (WhatTheInventoryIs::It(it), false) => {
            let rows: Vec<AWriteRow> = outputs
                .destinations
                .iter()
                .enumerate()
                .filter(|(_, destination)| destination.is_one_the_render_asked_for())
                .map(|(at, destination)| {
                    let (place, the_top) = match where_under_the_jobs_folder(map, job_folder, destination) {
                        WhereUnderTheJobsFolder::TheTop => {
                            (String::new(), Some(Prose::new(words::TOP_OF_THE_JOBS_FOLDER)))
                        }
                        WhereUnderTheJobsFolder::AFolder(folder) => (folder, None),
                        WhereUnderTheJobsFolder::NotUnderIt => (the_place(destination), None),
                    };
                    AWriteRow {
                        place: Verbatim::new(place),
                        the_top,
                        how_many: Verbatim::new(it.the_figure_for(at).to_string()),
                        format: destination
                            .format
                            .as_ref()
                            .map(|format| Verbatim::new(a_format_in_smoozys_words(format.as_the_file_records_it.as_bytes()))),
                    }
                })
                .collect();
            let by_format: Vec<AWriteTotal> = it
                .the_split_by_format(&outputs.destinations)
                .into_iter()
                .map(|(format, how_many)| AWriteTotal {
                    how_many: Verbatim::new(how_many.to_string()),
                    // **An entry whose format is not recorded falls in a group
                    // of its own that says so** rather than dropping out of the
                    // split, and the empty value is that group.
                    format: Verbatim::new(
                        format
                            .map(|name| a_format_in_smoozys_words(&name))
                            .unwrap_or_default(),
                    ),
                })
                .collect();
            Some(TheWrites {
                rows,
                total: Words {
                    lead: Prose::new(String::new()),
                    values: vec![Verbatim::new(it.the_count().to_string())],
                    // **One file is one file**: a one-file job read *1 files*,
                    // and a typed `1` makes one (the Creator's tenth notes,
                    // note 1).
                    rest: Some(Prose::new(if it.the_count() == 1 { words::FILE_WORD } else { words::FILES_WORD })),
                    the_rest_joins: false,
                },
                by_format,
            })
        }
        _ => None,
    }
}

/// The lines under the list, with the list and the warnings handed through.
#[allow(clippy::type_complexity)]
fn the_rest_of_the_output_group(
    writes: Option<TheWrites>,
    warnings: Vec<TheOutputWarning>,
    six: &smoozy_job_files::holds::TheSixThings,
    scene: Option<&SceneReading>,
    ticked: &[ExactName],
) -> (Option<TheWrites>, Vec<TheOutputWarning>, Vec<ASentence>) {
    // **Two more things the read found sit under that list, as lines rather
    // than warnings**: both are facts about what this job writes, neither asks
    // the user to do anything, so they belong with the list they qualify.
    let mut lines: Vec<ASentence> = Vec::new();
    let twice = the_beauty_written_more_than_once(&six.where_the_same_picture_is_written);
    if six.the_same_picture_reaches_more_than_one_place && !twice.is_empty() {
        lines.push(
            twice
                .into_iter()
                .map(|(lead, figure, rest)| match figure {
                    Some(count) => part(lead, vec![count.to_string()], Some(rest), false),
                    None if rest.is_empty() => plainly(lead),
                    None => part(lead, Vec::new(), Some(rest), false),
                })
                .collect(),
        );
    }
    // **A video and frames are two outputs, named as what they are**, never
    // the same picture written twice (the Creator's eighth notes, note 4).
    if six.the_render_reaches_frames_and_a_video {
        lines.push(vec![plainly(words::FRAMES_AND_A_VIDEO)]);
    }
    if let Some(other) = &six.the_tree_is_shared_with {
        let named = String::from_utf8_lossy(other.as_bytes()).to_string();
        lines.push(vec![
            part(
                "",
                vec![scene
                    .map(|scene| scene.name.as_text().to_string())
                    .unwrap_or_default()],
                Some(words::PULLS_IN_LEAD),
                false,
            ),
            part(
                "",
                vec![named.clone()],
                Some(words::PULLS_IN_REST),
                false,
            ),
        ]);
        // **Where both scenes are ticked a second line says the doubling** —
        // a different fact, and only true of that case.
        if ticked.iter().any(|one| one.as_bytes() == other.as_bytes()) {
            lines.push(vec![
                part(
                    "",
                    vec![named],
                    Some(words::PULLED_IN_IS_TICKED_LEAD),
                    false,
                ),
                plainly(words::PULLED_IN_IS_TICKED_REST),
            ]);
        }
    }
    if six.its_output_is_a_single_video_file {
        lines.push(vec![
            plainly(words::ONE_VIDEO_FILE_LEAD),
            plainly(words::ONE_VIDEO_FILE_SPLITS),
            plainly(words::ONE_VIDEO_FILE_REST),
        ]);
    }
    (writes, warnings, lines)
}

// ════════════════════════════════════════════════════════════════════════════
// Where the files go and which formats — the one control under the switch
// ════════════════════════════════════════════════════════════════════════════
//
// `Design/Render Settings § Submitting` and `§ How the three sit together`.
// **The panel it opens is piece 12's** — `crate::surface::transfers`'
// [`the_format_box_from`](crate::surface::transfers::the_format_box_from), the
// drawing's `fmt`, `fmtone` and `fmtnone` — and this page hands it what the
// read found and what the user already chose. The two figures the summary
// shows are the box's own sum, so the strip and the box never disagree.

/// **What the box knows, for a submission not yet made.** The formats are
/// `Download` N1's — read off the reading's destinations — counted by the one
/// inventory `§ What it will write` counts with (`Job files and outputs` I14a's
/// split). **No reading, no list** (`Download` N2).
///
/// **Each format says where in the job it is written** — every folder its
/// destinations take under the job's folder, from the same map the list under
/// `§ What it will write` reads, and *top of the job's folder* where one writes
/// there — never the path the file records (the Creator's second notes,
/// piece 2, their notes 16 and 18). **The name stays the identifier the file
/// records**, because it is what a narrowing keeps (`Download` N1) and what the
/// page hands back; the box shows it in Smoozy's words.
pub fn what_the_format_box_knows(
    outputs: Option<&TheOutputsHalf>,
    choices: &TheSubmissionChoices,
    record: &WhatTheRecordHolds,
    ground: &TheGroundOfTheCount,
) -> WhatTheBoxKnows {
    let read = outputs.and_then(|outputs| {
        let AsTheJobRendersIt {
            outputs,
            map,
            inventory,
            a_typed_frame_lies_outside_the_read,
        } = the_inventory_of(outputs, ground);
        // **A typed frame outside the read: no figures here either** — the
        // box has no list, and [`the_download_panel_from`] says why.
        if a_typed_frame_lies_outside_the_read {
            return None;
        }
        let WhatTheInventoryIs::It(it) = inventory else {
            return None;
        };
        let formats: Vec<AFormatKnown> = it
            .the_split_by_format(&outputs.destinations)
            .into_iter()
            // **`Download` N4: a file with no format is outside a narrowing**,
            // so it is never offered as one.
            .filter_map(|(format, files)| {
                let name = format?;
                let mut at_the_top = false;
                let mut folders: Vec<String> = Vec::new();
                for destination in outputs.destinations.iter().filter(|destination| {
                    destination.is_one_the_render_asked_for()
                        && destination
                            .format
                            .as_ref()
                            .is_some_and(|format| format.as_the_file_records_it.as_bytes() == name.as_slice())
                }) {
                    let folder = match where_under_the_jobs_folder(&map, ground.the_folder_composed_under(), destination) {
                        WhereUnderTheJobsFolder::TheTop => {
                            at_the_top = true;
                            continue;
                        }
                        WhereUnderTheJobsFolder::AFolder(folder) => folder,
                        WhereUnderTheJobsFolder::NotUnderIt => the_place(destination),
                    };
                    // **Each folder once**, in the reading's order.
                    if !folders.contains(&folder) {
                        folders.push(folder);
                    }
                }
                let chosen = choices
                    .the_formats_chosen
                    .as_ref()
                    .is_none_or(|chosen| chosen.contains(&name));
                Some(AFormatKnown {
                    name: shown(&name),
                    where_in_the_job: folders.join(THE_FOLDERS_ARE_LISTED_WITH),
                    at_the_top,
                    files,
                    chosen,
                })
            })
            .collect();
        Some((formats, it.the_count()))
    });
    let (formats, files_in_the_job) = match read {
        Some((formats, count)) => (Some(formats), count),
        None => (None, 0),
    };
    WhatTheBoxKnows {
        formats,
        files_in_the_job,
        // **Both in the system's own path form** (must-do 18): the box turns
        // each into text where it draws it and nowhere else.
        the_standing_folder: record.the_standing_folder.clone(),
        the_formats_chosen: choices.the_formats_chosen.clone(),
        the_folder_chosen_for_this_job: choices.where_they_go.clone(),
    }
}

/// **The folder the summary names**: the one chosen for this job, else the
/// standing folder, else §8's proposal — the order the box offers them in, and
/// the first is the one the box opens on. **Nothing where nobody named one**
/// (`Download` H3).
pub fn the_folder_named(choices: &TheSubmissionChoices, record: &WhatTheRecordHolds) -> Option<String> {
    choices
        .where_they_go
        .as_ref()
        .or(record.the_standing_folder.as_ref())
        .or(record.the_proposed_folder.as_ref())
        .map(|folder| shown(folder))
}

/// **The summary, or the folder alone.** `§ How the three sit together`:
/// *EXR and PNG — 600 of 720 files, to ~/Renders/kitchen*; on a job with one
/// format *240 EXR files, to ~/Renders/kitchen*; and with nothing read the
/// control is where the files go and nothing else. **Every format name, figure
/// and folder is a value** (X7), and the words between them are this page's.
pub fn the_download_summary(
    knows: &WhatTheBoxKnows,
    folder: Option<&str>,
    a_reading_stands: bool,
) -> (Option<ASentence>, Option<ASentence>) {
    // **One file is one file** — *1 PNG file, to ~/Downloads*, with `1` typed
    // (the Creator's tenth notes, note 1).
    let the_files_word = |how_many: usize| match (folder.is_some(), how_many == 1) {
        (true, true) => words::FILE_AND_A_COMMA,
        (true, false) => words::FILES_AND_A_COMMA,
        (false, true) => words::FILE_WORD,
        (false, false) => words::FILES_WORD,
    };
    match &knows.formats {
        Some(formats) if a_reading_stands && !formats.is_empty() => {
            let mut sentence: ASentence = Vec::new();
            if formats.len() == 1 {
                // **The count and the format name are one value**, *240 EXR*,
                // and the words after it are bound to it (`Design/Whole app §
                // Typography`), as `src/surface/transfers.rs` writes the same
                // figure.
                sentence.push(part(
                    "",
                    vec![crate::typography::a_figure_with_its_unit(
                        formats[0].files,
                        &a_format_in_smoozys_words(formats[0].name.as_bytes()),
                    )],
                    Some(the_files_word(formats[0].files)),
                    false,
                ));
            } else {
                let chosen: Vec<String> = formats
                    .iter()
                    .filter(|format| format.chosen)
                    // **In Smoozy's words**, as the box names them (Rule 9).
                    .map(|format| a_format_in_smoozys_words(format.name.as_bytes()))
                    .collect();
                match chosen.split_last() {
                    Some((last, [])) => sentence.push(part("", vec![last.clone()], None, false)),
                    Some((last, before)) => {
                        sentence.push(part("", before.to_vec(), None, false));
                        sentence.push(part(words::AND, vec![last.clone()], None, false));
                    }
                    None => {}
                }
                sentence.push(part(
                    words::THE_DASH,
                    vec![the_files_the_choice_brings(formats).to_string()],
                    Some(words::OF),
                    false,
                ));
                sentence.push(part("", vec![knows.files_in_the_job.to_string()], Some(the_files_word(knows.files_in_the_job)), false));
            }
            if let Some(folder) = folder {
                sentence.push(part(words::TO_FOLDER, vec![folder.to_string()], None, false));
            }
            (Some(sentence), None)
        }
        _ => {
            let mut sentence: ASentence = vec![plainly(words::WHERE_THEY_GO)];
            if let Some(folder) = folder {
                sentence.push(part(words::THE_DASH, vec![folder.to_string()], None, false));
            }
            (None, Some(sentence))
        }
    }
}

/// **The box a file's download control opens, and whether its read is
/// offered.** The box is piece 12's, shared with the queue's; what this page
/// adds is the one thing the box cannot know for itself.
///
/// **`Render Settings` B5a reaches the box's *Read it now* too**: it is a read
/// exactly as the header's *Read the file* is, and *"Not offered means the
/// control stays where it was and reads unavailable — Render and the read
/// alike"* (`Design/Render Settings`). **The case it was first written for**
/// is a file whose page had a Blender picked, where the box's read and the
/// header's are offered together. **The case it now also meets** is a page
/// with no version resolved: the header's read was drawn unavailable and the
/// box's was drawn live, and a press on it closed the box, went to the
/// application, came back *not offered* and said nothing — step 10c's
/// eleventh pass, fault 8, proved in the page harness on 2026-09-30.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct TheDownloadBox {
    #[serde(flatten)]
    pub the_box: TheFormatAnswer,
    pub the_read_is_offered: bool,
}

/// **The panel the control opens** — piece 12's box, composed for this file.
///
/// **A box opened is a fresh box**: a folder the system's panel gave an earlier
/// box that was cancelled is let go of here (`Design/Render Settings`:
/// *"Closing the panel leaves the box as it was"*, and a box closed with
/// *Cancel* keeps nothing).
#[tauri::command]
pub fn the_download_panel(started: State<'_, Mutex<Started>>, server: String, file: Vec<u8>) -> TheDownloadBox {
    let file = the_path_as_it_is_keyed(&file);
    let record = the_record_now(&started, &server);
    let Ok(mut held) = the_run_holds().lock() else {
        return TheDownloadBox {
            the_box: TheFormatAnswer::TheActIsNotBuiltYet {
                said: Prose::new(words::THE_RECORD_WOULD_NOT_READ),
                close: Prose::new(words::CANCEL),
            },
            the_read_is_offered: false,
        };
    };
    the_box_is_opened(&mut held, &server, &file, &record)
}

/// **[`the_download_panel`]'s whole**: a fresh box, and its read.
pub fn the_box_is_opened(
    held: &mut WhatTheRunHolds,
    server: &str,
    file: &str,
    record: &WhatTheRecordHolds,
) -> TheDownloadBox {
    held.the_folder_the_panel_gave.remove(&keyed(server, file));
    the_download_box(held, server, file, record)
}

/// **The box and its read**, from what the run holds — the command's whole, so
/// a test can stand where it stands.
pub fn the_download_box(
    held: &WhatTheRunHolds,
    server: &str,
    file: &str,
    record: &WhatTheRecordHolds,
) -> TheDownloadBox {
    TheDownloadBox {
        the_box: the_download_panel_from(held, server, file, record),
        // **`Render Settings` B5a**, from the one resolution the header's read
        // and the press both take.
        the_read_is_offered: the_version_resolved(held, server, file).the_read_is_offered(),
    }
}

/// **`Design/Render Settings`: *"Choosing Somewhere else… puts the system's
/// folder panel up at once, and the folder chosen there is shown in the box as
/// the choice, which Done then keeps like any other. Closing the panel leaves
/// the box as it was."*** Added 2026-09-30, from the Creator's press on step
/// 10c's eleventh pass (fault 11): the panel waited for *Done*, and they found
/// it *"a little confusing"*.
///
/// **The panel goes up the moment *Somewhere else…* is chosen**, through the
/// adapter every panel goes through (`Desktop app and delivery` K11), and
/// **`async`**, because a plain command runs on the drawing thread and a panel
/// put up there froze the window on the seventh pass on Hyperstack (see
/// [`crate::picker::asked`]).
///
/// **A folder chosen is held for the box, not kept for the job**: the box is
/// composed again with it offered first, and *Done* keeps it by its row, as it
/// keeps any other (H34 — the page never names a folder the box did not
/// offer). **The panel closed with nothing chosen answers nothing**, and the
/// page leaves the box as it was.
#[tauri::command]
pub async fn the_folder_is_asked_for_in_the_box<R: tauri::Runtime>(
    app: tauri::AppHandle<R>,
    server: String,
    file: Vec<u8>,
) -> Option<TheDownloadBox> {
    use crate::picker::{self, Asking, HowMany, Picked, WhatMayBeChosen};
    let file = the_path_as_it_is_keyed(&file);
    let picked = picker::asked(
        &app,
        Asking {
            what: WhatMayBeChosen::Folders,
            how_many: HowMany::One,
            starting_at: None,
            title: Prose::new(words::WHERE_THEY_GO),
        },
    )
    .await;
    let Picked::TheUserChose(chosen) = picked else {
        return None;
    };
    let folder = chosen
        .first()
        .map(|one| one.as_the_user_gave_it().as_os_str().as_encoded_bytes().to_vec())?;
    // **The record is read before the run's lock is taken**, the order every
    // act on this page takes them in.
    let record = match app.try_state::<Mutex<Started>>() {
        Some(started) => the_record_now(&started, &server),
        None => WhatTheRecordHolds::read_now(None),
    };
    let mut held = the_run_holds().lock().ok()?;
    Some(the_folder_the_panel_gave_is_held(&mut held, &server, &file, folder, &record))
}

/// **[`the_folder_is_asked_for_in_the_box`]'s landing**: the folder is held for
/// the open box, and the box is composed again with it as the choice.
pub fn the_folder_the_panel_gave_is_held(
    held: &mut WhatTheRunHolds,
    server: &str,
    file: &str,
    folder: Vec<u8>,
    record: &WhatTheRecordHolds,
) -> TheDownloadBox {
    held.the_folder_the_panel_gave.insert(keyed(server, file), folder);
    the_download_box(held, server, file, record)
}

/// The box, from what the run holds.
pub fn the_download_panel_from(
    held: &WhatTheRunHolds,
    server: &str,
    file: &str,
    record: &WhatTheRecordHolds,
) -> TheFormatAnswer {
    let mark = keyed(server, file);
    let submission = held.submission.get(&mark).cloned().unwrap_or_default();
    let halves = the_outputs_the_box_reads(held, server, file);
    let mut the_box = the_format_box_from(
        &what_the_open_box_knows_of(held, server, file, halves.as_deref(), &submission, record),
        &record.the_proposal(),
    );
    // **The box's third case: read, but not for the scene ticked** — it says
    // so rather than borrowing the *not read* sentence, which would be false.
    // **And a typed frame outside the read** (the Creator's tenth notes, note
    // 1): read, and nothing counted, so the line takes the figures' place.
    // **Each says what downloads as the box composed it** — the narrowing
    // that stands, where one does (the box ruling of 2026-10-17).
    if let TheFormatReading::NotRead { what_downloads, .. } = &the_box.reading {
        let what_downloads = what_downloads.clone();
        if a_typed_frame_lies_outside_what_the_box_reads(held, server, file, halves.as_deref()) {
            the_box.reading = the_box_says_the_frames_cant_be_counted(what_downloads);
        } else if let Some(several_ticked) = the_file_was_read_for_another_scene(held, server, file) {
            the_box.reading = TheFormatReading::read_for_another_scene(several_ticked, what_downloads);
        }
    }
    TheFormatAnswer::TheBox(the_box)
}

/// **Whether the box has no figures because a typed frame lies outside the
/// read** — of any scene ticked, against the same ground the box counts with
/// ([`the_ground_in_hand`]).
fn a_typed_frame_lies_outside_what_the_box_reads(
    held: &WhatTheRunHolds,
    server: &str,
    file: &str,
    halves: Option<&[&TheOutputsHalf]>,
) -> bool {
    let Some(halves) = halves else {
        return false;
    };
    let ground = the_ground_in_hand(held, server, file);
    halves
        .iter()
        .any(|outputs| a_typed_frame_the_read_did_not_step(&outputs.per_frame, ground.the_jobs_frames.as_deref()))
}

/// **The box's line where a typed frame lies outside the read**:
/// `Design/Render Settings § What it will write`'s one line, in place of the
/// figures, then what downloads — *Everything downloads.*, or the narrowing
/// that stands (the box ruling of 2026-10-17). **The box's shape for a file read that gives no list** —
/// the same as the read-for-another-scene case, and with no *Read it now* for
/// the same reason: a read steps the file's own range again, so it would
/// count no more.
fn the_box_says_the_frames_cant_be_counted(what_downloads: crate::surface::files::Words) -> TheFormatReading {
    TheFormatReading::ReadForAnotherScene {
        warning: Prose::new(words::FRAMES_OUTSIDE_THE_FILES_RANGE),
        what_downloads,
    }
}

/// **Whether a standing reading of a file of several scenes is for a scene
/// other than the one ticked** — `Design/Render Settings § What it will
/// write`: another scene ticked, several ticked, or a read that could not name
/// its scene. `Some(true)` where several are ticked. **A file of one scene is
/// never this case**: its read is always for the scene ticked. The Creator's
/// sixth notes, note 2.
fn the_file_was_read_for_another_scene(held: &WhatTheRunHolds, server: &str, file: &str) -> Option<bool> {
    let key = FileKey {
        server: an_identity(server),
        path: file.to_string(),
    };
    let reading = held.readings.standing(&key)?;
    if reading.settings.scenes.len() < 2 {
        return None;
    }
    let ticked = held.store.per_file(&key).map_or(0, |per_file| per_file.scenes_ticked.len());
    if ticked > 1 {
        return Some(true);
    }
    let read_from = held.the_saved_scene.get(&keyed(server, file));
    let in_hand = the_scene_in_hand(held, server, file);
    match (read_from, in_hand) {
        (Some(read_from), Some(in_hand)) if read_from.as_slice() == in_hand.as_bytes() => None,
        _ => Some(false),
    }
}

/// **[`what_the_open_box_knows`] over the half of every scene ticked**
/// (`Job files and outputs` R4b). With several, each format any of them
/// writes is offered once: its files added up, its folders each once, at the
/// top where any writes it there; and the job's count is every scene's.
fn what_the_open_box_knows_of(
    held: &WhatTheRunHolds,
    server: &str,
    file: &str,
    halves: Option<&[&TheOutputsHalf]>,
    submission: &TheSubmissionChoices,
    record: &WhatTheRecordHolds,
) -> WhatTheBoxKnows {
    let Some((first, rest)) = halves.and_then(<[&TheOutputsHalf]>::split_first) else {
        return what_the_open_box_knows(held, server, file, None, submission, record);
    };
    let mut knows = what_the_open_box_knows(held, server, file, Some(first), submission, record);
    for outputs in rest {
        let more = what_the_open_box_knows(held, server, file, Some(outputs), submission, record);
        knows.files_in_the_job += more.files_in_the_job;
        let (Some(formats), Some(adding)) = (knows.formats.as_mut(), more.formats) else {
            knows.formats = None;
            continue;
        };
        for format in adding {
            match formats.iter_mut().find(|known| known.name == format.name) {
                Some(known) => {
                    known.files += format.files;
                    known.at_the_top |= format.at_the_top;
                    let mut folders: Vec<&str> = known
                        .where_in_the_job
                        .split(THE_FOLDERS_ARE_LISTED_WITH)
                        .filter(|folder| !folder.is_empty())
                        .collect();
                    for folder in format.where_in_the_job.split(THE_FOLDERS_ARE_LISTED_WITH) {
                        if !folder.is_empty() && !folders.contains(&folder) {
                            folders.push(folder);
                        }
                    }
                    known.where_in_the_job = folders.join(THE_FOLDERS_ARE_LISTED_WITH);
                }
                None => formats.push(format),
            }
        }
    }
    knows
}

/// **What the open box knows**: [`what_the_format_box_knows`], with the folder
/// the system's panel gave this box, where it gave one, standing first as the
/// choice. **One composition for the box and for its *Done***, so a row is the
/// same folder in both (must-do 18: a folder goes back by its row).
///
/// **Counted against the same ground as the page** ([`the_ground_in_hand`]):
/// this file's folder on that server and the page's own View Layer pins, so
/// the box and the list beside it say one figure.
fn what_the_open_box_knows(
    held: &WhatTheRunHolds,
    server: &str,
    file: &str,
    outputs: Option<&TheOutputsHalf>,
    submission: &TheSubmissionChoices,
    record: &WhatTheRecordHolds,
) -> WhatTheBoxKnows {
    let mut knows = what_the_format_box_knows(outputs, submission, record, &the_ground_in_hand(held, server, file));
    if let Some(folder) = held.the_folder_the_panel_gave.get(&keyed(server, file)) {
        knows.the_folder_chosen_for_this_job = Some(folder.clone());
    }
    knows
}

/// **Which folder the user left chosen in the box.** H34: a closed request
/// type. A folder the page names must be one the box offered.
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[serde(tag = "is", rename_all = "kebab-case")]
pub enum TheFolderLeftChosen {
    /// **Which of the rows the box offered, by its place in that list** — and
    /// never the text the row showed (must-do 18 of the queue, history,
    /// settings and downloads round; step 10a, the screens round's loose ends,
    /// L6). ~~The page handed back the folder's displayed text and the
    /// application matched on it~~, which turns the folder into text and back
    /// again and cannot survive a name that is not valid text.
    ThisOne { at: usize },
    /// *Somewhere else…* left chosen at *Done* — the system's own picker,
    /// asked then (`Desktop app and delivery` K11). **A page that asks at the
    /// choice sends this no longer**: Render Settings' box puts the panel up
    /// the moment the row is chosen ([`the_folder_is_asked_for_in_the_box`])
    /// and hands the folder back by its row. It stays for a box that does not.
    SomewhereElse,
}

/// **Where the folder came from, once it is known.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TheFolderTaken {
    /// One of the rows the box offered, by its place in the offered list.
    Offered(usize),
    /// A path from the user, through the picker. **Held, not written into**:
    /// whatever lands there is judged by §7 when a download writes it (W7, K13).
    FromThePicker(Vec<u8>),
}

/// **The box's Done.** The two choices are kept on the submission (J2's volatile
/// store: nothing durable), and the panel is drawn again from them.
///
/// **`async`, because *Somewhere else…* puts the system's folder panel up**
/// (`Desktop app and delivery` §3.2, K11), and a plain command runs on the
/// drawing thread — see [`crate::picker::asked`]. It froze the window on the
/// seventh pass on Hyperstack.
#[tauri::command]
pub async fn the_download_choices_are_made<R: tauri::Runtime>(
    app: tauri::AppHandle<R>,
    server: String,
    file: Vec<u8>,
    formats: Vec<String>,
    folder: TheFolderLeftChosen,
) {
    use tauri::Manager;
    let file = the_path_as_it_is_keyed(&file);
    let record = match app.try_state::<Mutex<Started>>() {
        Some(started) => the_record_now(&started, &server),
        None => WhatTheRecordHolds::read_now(None),
    };
    let taken = match folder {
        TheFolderLeftChosen::ThisOne { at } => TheFolderTaken::Offered(at),
        TheFolderLeftChosen::SomewhereElse => {
            use crate::picker::{self, Asking, HowMany, Picked, WhatMayBeChosen};
            let picked = picker::asked(
                &app,
                Asking {
                    what: WhatMayBeChosen::Folders,
                    how_many: HowMany::One,
                    starting_at: None,
                    title: Prose::new(words::WHERE_THEY_GO),
                },
            )
            .await;
            let Picked::TheUserChose(chosen) = picked else {
                return;
            };
            let Some(folder) = chosen
                .first()
                .map(|one| one.as_the_user_gave_it().as_os_str().as_encoded_bytes().to_vec())
            else {
                return;
            };
            TheFolderTaken::FromThePicker(folder)
        }
    };
    let Some(queue) = app.try_state::<std::sync::Arc<TheOneQueue>>() else {
        return;
    };
    let Ok(mut held) = the_run_holds().lock() else {
        the_queue_is_told(&queue, an_act_said(&server, &file, "the-record", vec![plainly(words::THE_RECORD_WOULD_NOT_READ)]));
        return;
    };
    let _ = the_download_choices_are_taken(&mut held, &server, &file, &formats, taken, &record);
    the_queue_is_told(&queue, an_act_said(&server, &file, "download-choices", Vec::new()));
}

/// **The choices, checked and kept.** Answers whether anything was kept.
///
/// **A format the reading does not offer, or a folder the box did not offer,
/// changes nothing** (H34). Every offered format chosen is *everything*
/// (`Download` N3), which is kept as no list at all. **A box with no list
/// keeps the narrowing it names** (the box ruling of 2026-10-17), and **a list of one has nothing to
/// narrow** (`Design/Render Settings`), so it is everything.
pub fn the_download_choices_are_taken(
    held: &mut WhatTheRunHolds,
    server: &str,
    file: &str,
    formats: &[String],
    folder: TheFolderTaken,
    record: &WhatTheRecordHolds,
) -> bool {
    let mark = keyed(server, file);
    let mut submission = held.submission.get(&mark).cloned().unwrap_or_default();
    let halves = the_outputs_the_box_reads(held, server, file);
    // **The rows the open box offered**, the folder the system's panel gave it
    // first where it gave one — the composition the box was drawn from.
    let knows = what_the_open_box_knows_of(held, server, file, halves.as_deref(), &submission, record);

    let the_formats_chosen = match &knows.formats {
        // **No list: nothing can be ticked, and the box said the narrowing
        // that stands**, so Done keeps it — it is pressed to choose the folder.
        // ~~`None`~~ dropped it, and the box's own line with it (the box ruling of 2026-10-17).
        None if formats.is_empty() => submission.the_formats_chosen.clone(),
        None => return false,
        Some(offered) => {
            if formats.iter().any(|one| !offered.iter().any(|format| format.name == *one)) {
                return false;
            }
            // **A list of one draws no row to tick**, so the page sends none —
            // ~~which was kept as a narrowing to no format at all~~ (found
            // building the box ruling of 2026-10-17).
            if offered.len() < 2 || offered.iter().all(|format| formats.contains(&format.name)) {
                None
            } else {
                // The bytes the reading recorded, in the reading's order —
                // every ticked scene's, in the order they are ticked.
                let recorded: Vec<Vec<u8>> = halves
                    .as_deref()
                    .map(|halves| {
                        let mut names: Vec<Vec<u8>> = Vec::new();
                        for destination in halves.iter().flat_map(|outputs| &outputs.destinations) {
                            let Some(format) = &destination.format else { continue };
                            let name = format.as_the_file_records_it.as_bytes().to_vec();
                            if formats.contains(&shown(&name)) && !names.contains(&name) {
                                names.push(name);
                            }
                        }
                        names
                    })
                    .unwrap_or_default();
                Some(recorded)
            }
        }
    };

    let where_they_go = match folder {
        TheFolderTaken::FromThePicker(bytes) => bytes,
        // **A row of the box's own list, taken by its place** (must-do 18).
        // The list is composed here from the same three folders the box was
        // composed from, so a row the box never offered has no place in it.
        TheFolderTaken::Offered(at) => {
            let offered = the_folders_offered(&knows, &record.the_proposal());
            let Some((bytes, _)) = offered.into_iter().nth(at) else {
                return false;
            };
            bytes
        }
    };

    submission.the_formats_chosen = the_formats_chosen;
    submission.where_they_go = Some(where_they_go);
    // **Done keeps it like any other**, and the box it was given to is done.
    held.the_folder_the_panel_gave.remove(&mark);
    held.submission.insert(mark, submission);
    true
}

/// **What a count is composed against, beside the reading** — the two facts
/// that are the caller's and never the reading's: **where the `.blend` sits on
/// that server**, which the file's `//` paths resolve against, and **the View
/// Layers the job pins**. Holes 2 and 5 of the map of the Creator's second
/// notes: until piece 2 the page resolved every `//` against a made-up folder
/// and counted every layer the file switched on, so with *Box AO* and
/// *Mattress AO* pinned off it still said 7 files where 5 were written.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TheGroundOfTheCount {
    /// The folder the `.blend` sits in on the server, as bytes.
    pub the_folder_the_blend_sits_in: Vec<u8>,
    /// `Job files and outputs` R4 item 5's pins, in that crate's shape.
    pub the_jobs_pins: Vec<AViewLayerTheJobPins>,
    /// **Where the page composes its map**: the folder the job's folder will
    /// sit in, from [`where_the_jobs_folder_sits`], joined to a stand-in for
    /// the job's own folder — so a path the page shows whole (the collision
    /// warning) is where the files will really land. **Nothing where the
    /// layout is not known**, and then the page's own stand-in folder, which
    /// changes no folder shown under the job's folder.
    pub the_jobs_folder: Option<String>,
    /// **The frames the job covers, where a range was typed** — `Render
    /// Settings` F3's explicit list of the parsed form, each frame once.
    /// **Nothing where none was typed**, and then the job's frames are the
    /// file's own and the reading's whole per-frame list is counted (F4).
    /// `Design/Render Settings § What it will write`, added 2026-10-17: *the
    /// counts are for the frames the job will render.* The Creator's tenth
    /// notes, note 1: with `1` typed on a 300-frame file the page counted 300.
    pub the_jobs_frames: Option<Vec<i64>>,
}

/// The folder a `.blend` path sits in — everything before its last `/`, the
/// way the read before submitting takes it (`crate::reading_the_file`).
fn the_folder_a_blend_sits_in(the_blends_path: &str) -> Vec<u8> {
    the_blends_path
        .rsplit_once('/')
        .map(|(folder, _)| folder.as_bytes().to_vec())
        .unwrap_or_default()
}

/// **This page's ground**: the file it is over, and the pins it holds for the
/// scene in hand — `Render Settings` V4's pins, the ones Submit hands the
/// payload — **and the range typed**, the one entry Submit hands the payload
/// too (`Render Settings` F5: with several scenes ticked, the group's). **Only
/// the shape changes here**: the rule is `Job files and outputs`' one, applied
/// by the composition below.
fn the_ground_on_this_page(file: &str, answers: &SceneEntries) -> TheGroundOfTheCount {
    TheGroundOfTheCount {
        the_folder_the_blend_sits_in: the_folder_a_blend_sits_in(file),
        the_jobs_pins: answers
            .pins
            .iter()
            .map(|pin| AViewLayerTheJobPins {
                layer: ArtistsText::of(pin.layer.name.as_bytes().to_vec()),
                on: pin.on,
            })
            .collect(),
        the_jobs_folder: None,
        the_jobs_frames: answers.frame_range.as_ref().map(FrameForm::explicit_list),
    }
}

/// [`the_ground_on_this_page`], for a command that holds the run and not the
/// panel — the box and its *Done* read the same answers the panel does.
fn the_ground_in_hand(held: &WhatTheRunHolds, server: &str, file: &str) -> TheGroundOfTheCount {
    let key = FileKey {
        server: an_identity(server),
        path: file.to_string(),
    };
    let scene = the_scene_in_hand(held, server, file);
    the_ground_on_this_page(file, &held.store.the_answers_composition_reads(&key, scene.as_ref()))
}

/// **A submitted job's ground, for the box a queue entry opens**: the `.blend`
/// the job was submitted with, and **the pins it was submitted with** — the
/// payload's own `view_layer_pins`, read off the kept setup the local job
/// record has held beside every submission since record version 25. That is
/// the very payload Submit sent (one composition; the manifest and the record
/// both take it), so the queue's box counts the layers the job renders, as
/// Render Settings did before Submit. The Creator's second notes, piece 2b.
///
/// **No kept setup, no pins**: a record written before version 25, or by
/// something other than a submission, counts the layers the file switches on,
/// as this box did before — nothing is made up for it.
///
/// **And the range it was submitted with**, the payload's own `frame_range`,
/// the same way as the page's (the Creator's tenth notes, note 1): the box
/// counts the frames the job renders. No kept setup, or none typed, is the
/// file's own range.
pub(crate) fn the_ground_of_a_submitted_job(
    the_blends_path: Option<&str>,
    the_setup_it_was_submitted_with: Option<&smoozy_render_settings::payload::Payload>,
) -> TheGroundOfTheCount {
    TheGroundOfTheCount {
        the_folder_the_blend_sits_in: the_blends_path.map(the_folder_a_blend_sits_in).unwrap_or_default(),
        // **Only the shape changes here**, as on the page: the rule is
        // `Job files and outputs`' one, applied by the composition.
        the_jobs_pins: the_setup_it_was_submitted_with
            .map(|payload| {
                payload
                    .view_layer_pins
                    .iter()
                    .map(|pin| AViewLayerTheJobPins {
                        layer: ArtistsText::of(pin.layer.as_bytes().to_vec()),
                        on: pin.on,
                    })
                    .collect()
            })
            .unwrap_or_default(),
        the_jobs_folder: None,
        the_jobs_frames: the_setup_it_was_submitted_with
            .and_then(|payload| payload.frame_range.as_ref())
            .map(FrameForm::explicit_list),
    }
}

impl TheGroundOfTheCount {
    /// **The folder the page's map is composed under** — where the job's
    /// folder will sit, where that is known, or the page's own stand-in.
    pub fn the_folder_composed_under(&self) -> &str {
        self.the_jobs_folder.as_deref().unwrap_or(THE_PAGES_JOB_FOLDER)
    }
}

/// **The job output folder the page composes its map under.** The page shows
/// each folder *under* the job's folder and nothing that depends on the job's
/// own name, which no job has before it is submitted — so a stand-in is enough,
/// and it is the one the read before submitting composes under too.
const THE_PAGES_JOB_FOLDER: &str = "/ephemeral/smoozy/jobs/job";

/// **The map and the inventory of one reading, composed one way** — for the
/// destination list and the split by format under `§ What it will write`, and
/// for the formats the download panel offers. **One composition**, so the
/// figures the page shows in the two places cannot differ.
///
/// **The job's pins apply first**, by `Job files and outputs`' one rule
/// ([`the_per_frame_set_with_the_jobs_pins`], R4 item 5 as written
/// 2026-10-04), the way that crate's own composition applies them on the
/// server — so the page's count and the server's cannot disagree about a
/// layer. **The working copy is handed back** for whatever reads the per-frame
/// set after it (the six things); the reading the page holds stays the file's.
///
/// **The per-frame list is cut to the typed frames before anything is
/// composed** (the Creator's tenth notes, note 1; `Job files and outputs`
/// item 5, *"evaluated at every frame the job covers"*), so the list and the
/// download box both count the frames the job renders, from this one
/// composition. **Where a typed frame lies outside what the read stepped**,
/// that frame has no entry to count from, and a count of the rest would say
/// fewer files than the job writes: the reading is then composed whole, as
/// the file's own range, and
/// [`AsTheJobRendersIt::a_typed_frame_lies_outside_the_read`] says so, so no
/// caller shows a figure from it. **On the machine the job's own reading steps
/// the typed frames**, so its count is exact either way.
fn the_inventory_of(outputs: &TheOutputsHalf, ground: &TheGroundOfTheCount) -> AsTheJobRendersIt {
    let a_typed_frame_lies_outside_the_read =
        a_typed_frame_the_read_did_not_step(&outputs.per_frame, ground.the_jobs_frames.as_deref());
    let per_frame = match ground.the_jobs_frames.as_deref() {
        Some(typed) if !a_typed_frame_lies_outside_the_read => {
            let typed: std::collections::BTreeSet<i64> = typed.iter().copied().collect();
            outputs
                .per_frame
                .iter()
                .filter(|at| typed.contains(&at.frame))
                .cloned()
                .collect()
        }
        _ => outputs.per_frame.clone(),
    };
    let as_the_job_renders_it = TheOutputsHalf {
        per_frame: the_per_frame_set_with_the_jobs_pins(&per_frame, &ground.the_jobs_pins),
        ..outputs.clone()
    };
    let to_map: Vec<ADestinationToMap<'_>> = as_the_job_renders_it
        .destinations
        .iter()
        .map(|destination| ADestinationToMap {
            destination,
            // **N8's budget is measured against the predicted deepest file
            // path**, and the recipe that predicts it is `Job files and
            // outputs` §7's. The page draws the destinations and the counts
            // and measures no budget, so nothing here supplies one.
            the_longest_name_it_will_write: None,
        })
        .collect();
    let map = compose_the_map(
        &to_map,
        &ground.the_folder_the_blend_sits_in,
        ground.the_folder_composed_under(),
        as_the_job_renders_it.taken_by,
    );
    drop(to_map);
    let inventory = compose_the_inventory(&as_the_job_renders_it, &map);
    AsTheJobRendersIt {
        outputs: as_the_job_renders_it,
        map,
        inventory,
        a_typed_frame_lies_outside_the_read,
    }
}

/// **[`the_inventory_of`]'s answer**: the reading as the job renders it, its
/// map and its inventory, and whether a typed frame lies outside the read.
struct AsTheJobRendersIt {
    outputs: TheOutputsHalf,
    map: TheMap,
    inventory: WhatTheInventoryIs,
    /// **A typed frame the read did not step**: no figure is shown, and
    /// `Design/Render Settings § What it will write`'s one line stands in the
    /// figures' place ([`words::FRAMES_OUTSIDE_THE_FILES_RANGE`]).
    a_typed_frame_lies_outside_the_read: bool,
}

/// **Whether a typed frame has no entry in the reading's per-frame list** —
/// *Read the file* steps the file's own range, so a frame typed outside it
/// was never read. Nothing typed is never this case.
fn a_typed_frame_the_read_did_not_step(
    per_frame: &[smoozy_job_files::reading::AtOneFrame],
    the_jobs_frames: Option<&[i64]>,
) -> bool {
    let Some(typed) = the_jobs_frames else {
        return false;
    };
    let stepped: std::collections::BTreeSet<i64> = per_frame.iter().map(|at| at.frame).collect();
    typed.iter().any(|frame| !stepped.contains(frame))
}

/// **Where one destination lands under the job's folder.**
#[derive(Debug, Clone, PartialEq, Eq)]
enum WhereUnderTheJobsFolder {
    /// At the top of the job's folder itself.
    TheTop,
    /// In a folder under it, written as `PNG/`.
    AFolder(String),
    /// **Nowhere under it**: `Job files and outputs` P12's *not mappable*,
    /// which holds the job (`Job files and outputs` P13). The row then says
    /// what the file says, the one true thing there is to say of it.
    NotUnderIt,
}

/// **One destination's folder under the job's folder**, read off the map by
/// `Job files and outputs`' own relative reading
/// ([`TheMap::the_folders_of`]) — never composed a second time here.
fn where_under_the_jobs_folder(map: &TheMap, job_folder: &str, destination: &Destination) -> WhereUnderTheJobsFolder {
    match map
        .the_folders_of(job_folder, |one| one == destination)
        .first()
    {
        None => WhereUnderTheJobsFolder::NotUnderIt,
        Some(beneath) if beneath.is_empty() => WhereUnderTheJobsFolder::TheTop,
        Some(beneath) => WhereUnderTheJobsFolder::AFolder(format!("{}/", String::from_utf8_lossy(beneath))),
    }
}

/// **What sits between two folders where one format writes into several** —
/// the drawing's own *beauty/ · mist/*. A list, so punctuation and never a word.
const THE_FOLDERS_ARE_LISTED_WITH: &str = " · ";

/// **Formats in Smoozy's words, from one table** — hole 17 of the map of the
/// Creator's second notes, ruled by the build runner: *Smoozy's words for the
/// identifiers the research lists*. **A decision, not a Blender fact** (Rule
/// 2): which identifiers a file can record is the research's (`Comparison/
/// Images` question 3, `Comparison/File output nodes` question 9 — sixteen
/// across the four versions), and what Smoozy calls each is Smoozy's.
/// `Design/Render Settings § What it will write`: *PNG*, *JPEG*, *EXR*,
/// *Multilayer EXR*, *Video*. **Every word is its own**, so a word names one
/// identifier and a narrowing is never two formats under one name.
pub const THE_FORMATS_IN_SMOOZYS_WORDS: [(&str, &str); 16] = [
    ("PNG", "PNG"),
    ("JPEG", "JPEG"),
    ("OPEN_EXR", "EXR"),
    ("OPEN_EXR_MULTILAYER", "Multilayer EXR"),
    ("FFMPEG", "Video"),
    ("TIFF", "TIFF"),
    ("WEBP", "WebP"),
    ("AVIF", "AVIF"),
    ("JPEG2000", "JPEG 2000"),
    ("BMP", "BMP"),
    ("TARGA", "Targa"),
    ("TARGA_RAW", "Targa Raw"),
    ("CINEON", "Cineon"),
    ("DPX", "DPX"),
    ("HDR", "Radiance HDR"),
    ("IRIS", "Iris"),
];

/// **A format as the user reads it**: the table's word, or — for an
/// identifier the table does not hold — the identifier as the file records it.
pub fn a_format_in_smoozys_words(as_the_file_records_it: &[u8]) -> String {
    THE_FORMATS_IN_SMOOZYS_WORDS
        .iter()
        .find(|(identifier, _)| identifier.as_bytes() == as_the_file_records_it)
        .map(|(_, said)| said.to_string())
        .unwrap_or_else(|| shown(as_the_file_records_it))
}

/// **Where the job's folder will sit on the server — the one place the page
/// reads it from.** `Design/Render Settings § What it will write`: *"Above
/// the list, one line says where the job's folder will sit"*, or which of the
/// two fallbacks the spec names — and, from later on 2026-10-04, where the
/// fallback goes when the server has a folder chosen for it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhereTheJobsFolderSits {
    /// In this folder, in a folder of its own: *In /home/ubuntu/Render, in a
    /// folder for this job.*
    In(String),
    /// `Job files and outputs` P19's fallback, with the case, and where it
    /// goes (`Job files and outputs` P23).
    FallsBack {
        why: smoozy_job_files::where_the_folder_sits::WhyTheFilesOwnPlaceWasNotUsed,
        to: WhereTheFallbackGoes,
        /// The folder the job's folder sits in there — Smoozy's job area or
        /// the server's chosen folder.
        place: String,
    },
}

/// **Where a job that falls back goes, as the page can tell before
/// submitting** (`Job files and outputs` P23).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhereTheFallbackGoes {
    /// No folder is chosen for this server: Smoozy's own.
    SmoozysFolder,
    /// The server's chosen fallback folder, and nothing known before
    /// submitting rules it out.
    TheServersFolder(String),
    /// A folder is chosen and something known before submitting rules it out,
    /// so Smoozy's own — and the page says so on a second line.
    SmoozysFolderBecauseTheServersCantBeUsed,
}

/// **The stand-in for the job's own folder under the place the page names**,
/// which no job has a name for before it is submitted (`Job files and
/// outputs` N3 is the identity and the label). A collision warning reads
/// `/home/ubuntu/Render/…/PNG/001 - Box.png`: the real place, then a folder
/// for this job. Build's own choice.
const A_FOLDER_FOR_THIS_JOB: &str = "…";

impl WhereTheJobsFolderSits {
    /// **The folder the page composes its map under** — the place the job's
    /// folder will sit in, joined to [`A_FOLDER_FOR_THIS_JOB`].
    pub fn the_pages_job_folder(&self) -> String {
        format!("{}/{A_FOLDER_FOR_THIS_JOB}", self.the_place())
    }

    fn the_place(&self) -> &str {
        match self {
            WhereTheJobsFolderSits::In(folder) | WhereTheJobsFolderSits::FallsBack { place: folder, .. } => folder,
        }
    }
}

/// **Where the job's folder will sit — `Job files and outputs`' own choice,
/// asked with nothing made** ([`smoozy_job_files::where_the_folder_sits::where_it_would_sit`]):
/// the run the render's own destinations share, resolved against where this
/// file sits on the server (`Job files and outputs` P18); `Job files and
/// outputs` P19's fallback where that is another computer's or a folder
/// Smoozy cannot use; and the server's chosen fallback folder (`Job files and
/// outputs` P23) where one is set. **The render worker makes the same choice at
/// render start**, so the line says where the files will really go — save
/// what only the machine can tell (a folder it lacks at its top, a make it
/// refuses), which the queue entry says. The Creator's second notes, piece 4.
pub fn where_the_jobs_folder_sits(
    layout: &TheLayout,
    outputs: &TheOutputsHalf,
    ground: &TheGroundOfTheCount,
    the_servers_fallback_folder: Option<&str>,
) -> WhereTheJobsFolderSits {
    use smoozy_job_files::where_the_folder_sits::{the_run_of, where_it_would_sit, WhatTheChoiceIsMadeFrom, WhereItIs};
    let run = the_run_of(&outputs.destinations, &ground.the_folder_the_blend_sits_in, outputs.taken_by);
    let sits = where_it_would_sit(
        &WhatTheChoiceIsMadeFrom {
            the_run: &run,
            layout,
            the_job_output_folders_name: A_FOLDER_FOR_THIS_JOB,
            the_servers_fallback_folder,
        },
        &mut |_| true,
    );
    let place = sits.the_folder_it_sits_in().to_string();
    match sits.where_it_is {
        WhereItIs::WhereTheFilePoints => WhereTheJobsFolderSits::In(place),
        // **A job with no render destination at all** has no reason to give,
        // and its folder sits in the fallback's place.
        WhereItIs::TheFallback { why: None, .. } => WhereTheJobsFolderSits::In(place),
        WhereItIs::TheFallback { why: Some(why), the_servers_fallback_folder_was_passed_over } => {
            WhereTheJobsFolderSits::FallsBack {
                why,
                to: if the_servers_fallback_folder_was_passed_over {
                    WhereTheFallbackGoes::SmoozysFolderBecauseTheServersCantBeUsed
                } else if place == layout.area(Area::Job) {
                    WhereTheFallbackGoes::SmoozysFolder
                } else {
                    WhereTheFallbackGoes::TheServersFolder(place.clone())
                },
                place,
            }
        }
    }
}

/// **The one line above *Output files***, in the Design's words. **The folder
/// is a value** (X7), never joined into the words.
fn the_folder_line(sits: &WhereTheJobsFolderSits) -> ASentence {
    use smoozy_job_files::where_the_folder_sits::WhyTheFilesOwnPlaceWasNotUsed as Why;
    match sits {
        WhereTheJobsFolderSits::In(folder) => {
            vec![part(words::THE_JOBS_FOLDER_IN, vec![folder.clone()], Some(words::IN_A_FOLDER_FOR_THIS_JOB), true)]
        }
        WhereTheJobsFolderSits::FallsBack { why, to: WhereTheFallbackGoes::TheServersFolder(folder), .. } => {
            let lead = match why {
                Why::ItIsAnotherComputers => words::THE_FOLDER_IS_ANOTHER_COMPUTERS_SO_THE_FILES_GO_TO,
                Why::ItIsAFolderSmoozyCannotUse => words::SMOOZY_CANT_USE_THE_FOLDER_SO_THE_FILES_GO_TO,
            };
            vec![part(lead, vec![folder.clone()], Some(words::THE_FOLDERS_FULL_STOP), true)]
        }
        WhereTheJobsFolderSits::FallsBack { why: Why::ItIsAnotherComputers, .. } => {
            vec![plainly(words::THE_FOLDER_IS_ANOTHER_COMPUTERS)]
        }
        WhereTheJobsFolderSits::FallsBack { why: Why::ItIsAFolderSmoozyCannotUse, .. } => {
            vec![plainly(words::SMOOZY_CANT_USE_THE_FOLDER)]
        }
    }
}

/// **The second line, where the server's fallback folder is ruled out before
/// submitting** — the drawing's `res-fallchosen`. Nothing otherwise.
fn the_second_folder_line(sits: &WhereTheJobsFolderSits) -> Option<ASentence> {
    matches!(
        sits,
        WhereTheJobsFolderSits::FallsBack { to: WhereTheFallbackGoes::SmoozysFolderBecauseTheServersCantBeUsed, .. }
    )
    .then(|| vec![plainly(words::THIS_SERVERS_FALLBACK_FOLDER_CANT_BE_USED)])
}

fn the_place(destination: &Destination) -> String {
    destination
        .recorded_location
        .as_ref()
        .map(|location| String::from_utf8_lossy(location.as_bytes()).to_string())
        .unwrap_or_default()
}

/// **Where a colliding output comes from, as words and values** — and **a File
/// Output node's name is a value**, never joined into the words (X7, must-do 13
/// of the step 10a round 1 audit of these pieces). *(Until that round the name
/// was joined into prose with `format!`, so the typography point could change
/// characters inside it.)*
fn where_it_comes_from(origin: &WhereItComesFrom) -> ASentence {
    match origin {
        WhereItComesFrom::TheScenesOutputPath => vec![plainly(words::THE_SCENES_OUTPUT_PATH)],
        WhereItComesFrom::AFileOutputNodeNamed(name) => vec![with_values(
            words::A_FILE_OUTPUT_NODE_NAMED,
            vec![String::from_utf8_lossy(name.as_bytes()).to_string()],
            None,
        )],
        WhereItComesFrom::AnAddedWritePlaceNamed(_) => vec![plainly(words::AN_ADDED_WRITE_PLACE)],
    }
}

/// **`§ The dependency warning`: three sentences, and which one depends on why
/// the warning is there — plus the fourth, which replaces the other three.**
fn the_dependency_band(
    the_record: &WhatTheRecordHolds,
    picked: Option<&BlenderIdentity>,
    the_first_frame_is_asked_for: bool,
) -> Option<TheDependencyBand> {
    let picked = picked?;
    // **No standing is *nobody has ever checked*** — `Dependency Upload &
    // Remap` V14 and `standing::what_the_record_says` — and that is the case
    // the band exists for (must-do 3). *(Until the step 10a round 1 audit of
    // these pieces a missing entry drew no band at all.)*
    //
    // **The standing is the local record's**, read by
    // [`the_check_the_band_reads`] — step 10c's dependency wave piece 7. ~~It
    // was read from the run-held map~~ until then, with this line named as the
    // one that changes when the record's reader lands.
    let case = the_warnings_case(&the_record.the_check, picked);
    // **`Rendering` W4a's switch-off, as the shared service answered it**
    // (must-do 2). A read that failed is its own answer: the band is drawn and
    // says the switch could not be read, never taken as *not switched off*.
    let switched_off = the_record.the_dependency_warning == TheSwitchReads::SwitchedOff;
    let case = smoozy_rendering::warning::what_the_user_meets(case, switched_off)?;
    let (which, said, to_the_check) = match &case {
        TheWarningsCase::NobodyHasEverCheckedThisFileOnThisMachine
        | TheWarningsCase::ACheckWasRunAndDidNotComplete => (
            "never",
            vec![
                // **The server is named by what the user calls it**, never by
                // its local identifier (must-do 5); where the record would not
                // give the name, the sentence says *this server*.
                match &the_record.the_servers_name {
                    Some(name) => part(
                        words::NOBODY_HAS_CHECKED_LEAD,
                        vec![name.clone()],
                        Some(words::THE_FULL_STOP),
                        true,
                    ),
                    None => plainly(words::NOBODY_HAS_CHECKED_ON_THIS_SERVER),
                },
                plainly(words::NOBODY_HAS_CHECKED_REST),
            ],
            words::CHECK_DEPENDENCIES,
        ),
        TheWarningsCase::AcheckWasRunAndTheFileHasBeenReplacedSince
        | TheWarningsCase::ACheckWasRunAndItsTreeHasBeenDisturbedSince => (
            "replaced",
            vec![plainly(words::REPLACED_LEAD)],
            words::CHECK_DEPENDENCIES,
        ),
        TheWarningsCase::ACheckStandsAndDoesNotCoverThisBlender { the_standing_covers } => (
            // **One sentence rather than two, and it carries both halves**:
            // the check happened, and it does not cover this Blender. **Naming
            // both versions is what makes it act on**, the user picking the
            // version at the top of this same panel.
            "version",
            vec![
                part(
                    words::VERSION_LEAD,
                    vec![the_standing_covers.clone()],
                    Some(words::VERSION_MIDDLE),
                    true,
                ),
                part(
                    "",
                    vec![version_text(picked.version)],
                    Some(words::VERSION_REST),
                    false,
                ),
            ],
            words::CHECK_DEPENDENCIES,
        ),
        TheWarningsCase::TheCheckLeftThingsUnfound(counts) => (
            "unfound",
            the_unfound_sentence(counts),
            words::OPEN_THE_CHECK,
        ),
        TheWarningsCase::Absent => return None,
    };
    Some(TheDependencyBand {
        which: Verbatim::new(which),
        said,
        the_way_to_the_check: Prose::new(to_the_check),
        the_way_starts_a_check: to_the_check != words::OPEN_THE_CHECK,
        the_switch_could_not_be_read: (the_record.the_dependency_warning == TheSwitchReads::TheReadFailed)
            .then(|| Prose::new(words::THE_RECORD_WOULD_NOT_READ)),
        // **Not offered where the user has already asked for the check below**:
        // it is the same thing they already have, so the band drops to two ways
        // on and the note goes with the control it explained.
        render_one_frame_first: (!the_first_frame_is_asked_for)
            .then(|| Prose::new(words::RENDER_ONE_FRAME_FIRST)),
        what_one_frame_first_buys: (!the_first_frame_is_asked_for)
            .then(|| Prose::new(words::ONE_FRAME_FIRST_BUYS)),
        submit_anyway: Prose::new(words::SUBMIT_ANYWAY),
        switch_it_off: Prose::new(words::DONT_WARN_ME_AGAIN),
        what_the_switch_covers: Prose::new(words::IT_COVERS_EVERY_FILE),
    })
}

/// **The unfound case's one sentence**, the count leading it.
fn the_unfound_sentence(counts: &Counts) -> Vec<Words> {
    let how_many = the_unfound_count(counts);
    let lead = if how_many == 1 { words::UNFOUND_LEAD_ONE } else { words::UNFOUND_LEAD };
    vec![a_figure(how_many.to_string(), lead)]
}

/// **`§ The dependency warning`: it says how many, and says it about the server
/// rather than about the search** — the number covers four findings, and rows
/// the user set aside are not counted.
fn the_unfound_count(counts: &Counts) -> usize {
    // **`Dependency Upload & Remap` V12: the number covers four findings** —
    // not found, could not be sent, could not be fixed, and found on your
    // computer and never sent — **and the one thing true of all four is that
    // the render will not find the asset.** **Rows the user set aside are not
    // counted**: setting one aside is the user saying they know and do not
    // care, and counting it would leave the number standing for ever.
    counts.not_found + counts.could_not_be_sent + counts.could_not_be_fixed
        + counts.found_but_not_sent
}

/// `Design/Render Settings § What this document left open`, third bullet — the
/// Simplify switch, which belongs to `Rendering` and sits with the render
/// settings on a re-submitted failed job.
fn the_simplify(
    came_from: Option<&ThePageCameFrom>,
    answers: &SceneEntries,
    scene: Option<&SceneReading>,
) -> Option<TheSimplify> {
    // **`Rendering` A1b: drawn where the entry the page was opened from ended
    // failed, and on no other page.** Until the 38's piece 2 any page an entry
    // opened drew it.
    if !came_from?.the_simplify_offer_is_drawn {
        return None;
    }
    let on = answers
        .simplify
        .use_simplify
        .or(scene.map(|scene| scene.simplify.use_simplify))
        .unwrap_or(false);
    let files = scene.map(|scene| &scene.simplify);
    let field = |label: &'static str, users: Option<String>, files: Option<String>| AField {
        label: Prose::new(label),
        whose: Verbatim::new(match store::shown(users.clone(), files.clone()) {
            store::Shown::TheUsers(_) => "the-users",
            store::Shown::TheFiles(_) => "the-files",
            store::Shown::PromiseForm => "the-promise-form",
        }),
        value: users.clone().or(files.clone()).map(Verbatim::new),
        hand_back_to: users.and(files).map(|files| Words {
            lead: Prose::new(words::BACK_TO_THE_FILES_VALUE),
            values: vec![Verbatim::new(files)],
            rest: None,
            the_rest_joins: false,
        }),
        greyed: false,
        refused: false,
    };
    Some(TheSimplify {
        label: Prose::new(words::SIMPLIFY),
        on,
        said: Prose::new(if scene.is_some_and(|scene| scene.simplify.use_simplify) {
            words::SIMPLIFY_ON_IN_THE_FILE
        } else {
            words::SIMPLIFY_OFFER
        }),
        settings: vec![
            field(
                words::MAX_SUBDIVISION,
                answers
                    .simplify
                    .subdivision_render
                    .map(|value| value.to_string()),
                files.map(|files| files.subdivision_render.to_string()),
            ),
            field(
                words::CHILD_PARTICLES,
                answers
                    .simplify
                    .child_particles_render
                    .map(|value| format!("{value:.2}")),
                files.map(|files| format!("{:.2}", files.child_particles_render)),
            ),
            the_texture_limit(
                answers.simplify.texture_limit_render.clone(),
                files.map(|files| files.texture_limit_render.clone()),
            ),
        ],
        // **`Rendering` A1d: a pick over the file's own enumeration**, each
        // identifier with what the pick shows for it.
        texture_choices: smoozy_rendering::again::THE_TEXTURE_LIMITS
            .iter()
            .map(|one| AChoice {
                value: Verbatim::new(*one),
                said: Prose::new(the_texture_limit_said(one)),
            })
            .collect(),
    })
}

/// **The Texture Limit row**: the user's pick or the file's, by its
/// identifier, which the page shows through [`TheSimplify::texture_choices`];
/// the cross names the file's own value in the pick's own words.
fn the_texture_limit(users: Option<String>, files: Option<String>) -> AField {
    AField {
        label: Prose::new(words::TEXTURE_LIMIT),
        whose: Verbatim::new(match store::shown(users.clone(), files.clone()) {
            store::Shown::TheUsers(_) => "the-users",
            store::Shown::TheFiles(_) => "the-files",
            store::Shown::PromiseForm => "the-promise-form",
        }),
        value: users.clone().or(files.clone()).map(Verbatim::new),
        hand_back_to: users.and(files).map(|files| Words {
            lead: Prose::new(words::BACK_TO_THE_FILES_VALUE),
            values: Vec::new(),
            rest: Some(Prose::new(the_texture_limit_said(&files))),
            the_rest_joins: false,
        }),
        greyed: false,
        refused: false,
    }
}

/// **What the pick shows for an identifier**: *No Limit* for off, as the
/// drawing has it, and the pixel ceiling itself for the rest.
fn the_texture_limit_said(identifier: &str) -> &str {
    if identifier == smoozy_rendering::again::THE_TEXTURE_LIMITS[0] {
        words::NO_LIMIT
    } else {
        identifier
    }
}

/// **`§ Before the file has been read`: the Render control names no count
/// there** — and with several scenes ticked it names the act rather than one
/// job.
fn the_render_control(count: &ASentence, ticked: usize, a_reading_stands: bool) -> ASentence {
    let figure = || {
        count
            .first()
            .and_then(|first| first.values.first())
            .map(|figure| figure.as_str().to_string())
            .unwrap_or_default()
    };
    // **One frame is *frame*** — step 10c's Vast pass read *Render 1 frames*
    // on a one-frame range, while the count beside the range already took the
    // singular ([`the_frame_count`]).
    let frames = |figure: &str| if figure == "1" { words::FRAME_WORD } else { words::FRAMES_WORD };
    match (count.is_empty(), ticked > 1, a_reading_stands) {
        // **The Render control names the act rather than one job**: ticking
        // several scenes submits one job per scene, so it names the jobs and
        // the frames together.
        (false, true, _) => vec![
            part(words::RENDER, vec![ticked.to_string()], Some(words::JOBS_WORD), false),
            a_figure(figure(), frames(&figure())),
        ],
        (false, false, true) => vec![part(
            words::RENDER,
            vec![figure()],
            Some(frames(&figure())),
            false,
        )],
        // **`§ Before the file has been read`: the Render control names no
        // count here** — the count is worked out from the file's own range and
        // there is no read to work one out from.
        _ => vec![plainly(words::RENDER)],
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use smoozy_render_settings::reading::{
        DeviceSetting, EarlyStop, OfferedEngineIdentifiers, PerEngineCounts, SettingsHalf,
        SimplifyReadBack, SwitchAnimation, ViewLayerReading,
    };
    use smoozy_render_settings::spellings::{Spelling, THE_FIVE_NOT_YET_READ};
    use std::fs;
    use std::path::Path;

    /// This file's working half — everything before the tests. **The sweeps
    /// below read it rather than the whole file**, because the things they
    /// forbid are written out here in order to forbid them.
    fn the_working_half() -> &'static str {
        include_str!("render_settings.rs")
            .split("#[cfg(test)]")
            .next()
            .expect("the file has a working half")
    }

    /// **The Creator's note 12 on 0.1.7: the mismatch is one line naming the
    /// two versions and saying something may not work as expected, and it opens
    /// nothing** — `Render Settings` B8 as rewritten 2026-10-02. **And it sits
    /// directly under the version row, above the pick line.** Before the piece
    /// it carried *What changes*, a panel of break notes, and sat after the pick
    /// line.
    #[test]
    fn a_mismatch_is_one_line_with_no_control_above_the_pick_line() {
        let picked = BlenderIdentity {
            path: "/opt/blender-5.1.0/blender".to_string(),
            version: a_version(5, 1, 0),
        };
        let line = the_mismatch(Some(version::ReleaseLine { major: 4, minor: 5 }), Some(&picked))
            .expect("4.5 and 5.1 are different release lines");
        // **Compared as words**: the typography point binds some spaces.
        let printed = serde_json::to_string(&line).expect("it serialises").replace('\u{a0}', " ");
        for said in ["Saved with", "4.5", "rendering with", "5.1", "Some things may not work as expected."] {
            assert!(printed.contains(said), "{said}: {printed}");
        }
        assert!(!printed.contains("What changes") && !printed.contains("crossed"), "{printed}");
        assert_eq!(serde_json::to_value(&line).expect("a value").as_object().map(|it| it.len()), Some(1));
        assert!(the_mismatch(Some(version::ReleaseLine { major: 5, minor: 1 }), Some(&picked)).is_none());

        let page = the_page();
        let drawn = page
            .split("function theMismatchIsDrawn(mismatch, head) {")
            .nth(1)
            .and_then(|rest| rest.split("\n}\n").next())
            .expect("the mismatch line");
        assert!(!drawn.contains("aLink(") && !drawn.contains("addEventListener"), "the line carries a control");
        assert!(!page.contains("break-notes"), "the panel of break notes is still drawn");
        let mismatch = page.find("if (said.the_mismatch) theMismatchIsDrawn(said.the_mismatch, head);").expect("drawn");
        let answer = page.find("head.appendChild(anAnswerLine(\"read\"));").expect("the read's line");
        let pick = page.find("const theLine = said.the_pick_line").expect("the pick line");
        assert!(mismatch < answer && mismatch < pick, "the mismatch is not directly under the version row");
        assert_eq!(page.matches("theMismatchIsDrawn(said.the_mismatch, head)").count(), 1);
    }

    /// **The Creator's sixth notes, note 5: Render Settings draws no amber
    /// dot** — `Design/Render Settings`, the mismatch line: *"It is plain text,
    /// with no dot, and so is the add-on line below."* Before the piece both
    /// lines began with one.
    ///
    /// The page is drawn by `web/render-settings.js` under Node, beside the
    /// uploading line's stand-ins, on an answer carrying both lines, and every
    /// piece it draws is looked at. **Skipped, and says so, where no Node is
    /// installed.**
    #[test]
    fn render_settings_draws_no_amber_dot() {
        let Some(got) = the_page_drawn_with_the_mismatch_and_the_add_on_line("no-amber-dot") else {
            eprintln!("SKIPPED: no Node on this computer, so the render settings page was not run");
            return;
        };
        assert_eq!(got["mismatch"], true, "the mismatch line is not drawn: {got}");
        assert_eq!(got["addOn"], true, "the add-on line is not drawn: {got}");
        assert_eq!(got["dots"], 0, "Render Settings draws an amber dot: {got}");
    }

    /// **The page drawn under Node on an answer carrying the mismatch line and
    /// the add-on line**, and what was drawn; `None` where no Node is installed.
    ///
    /// **`which` names the test asking, and its scratch folder with it**: two
    /// tests draw this page, and with two test threads one folder per process
    /// let one delete the other's files mid-run (found by the tenth notes'
    /// piece that made the download box say what goes).
    fn the_page_drawn_with_the_mismatch_and_the_add_on_line(which: &str) -> Option<serde_json::Value> {
        let file = "/root/uploads/kitchen.blend";
        let held = WhatTheRunHolds::default();
        let panel = the_panel_starting_from(&held, &a_server(), file, &WhatTheRecordHolds::default());
        let mut answer = serde_json::to_value(&panel).expect("the panel serialises");
        let picked = BlenderIdentity { path: "/opt/blender-5.1.0/blender".to_string(), version: a_version(5, 1, 0) };
        let mismatch = the_mismatch(Some(version::ReleaseLine { major: 4, minor: 5 }), Some(&picked));
        answer["the_mismatch"] = serde_json::to_value(mismatch).expect("the mismatch serialises");
        let add_on = TheAddOnCase {
            lead: Words {
                lead: Prose::new(words::BLENDER),
                values: vec![Verbatim::new("5.1.0")],
                rest: Some(Prose::new(words::THE_ADD_ON_IS_MISSING_LEAD)),
                the_rest_joins: false,
            },
            read_back_as_eevee: Prose::new(words::THE_ENGINE_READ_BACK_AS_EEVEE),
        };
        answer["engine"]["the_add_on_case"] = serde_json::to_value(add_on).expect("the add-on line serialises");

        let folder = std::env::temp_dir().join(format!("smoozy-no-amber-dot-page-{which}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&folder);
        fs::create_dir_all(&folder).expect("a scratch folder");
        let web = Path::new(env!("CARGO_MANIFEST_DIR")).join("web");
        fs::copy(web.join("render-settings.js"), folder.join("render-settings.js")).expect("the page");
        for (name, text) in THE_UPLOADING_LINES_STAND_INS {
            fs::write(folder.join(name), text).expect("a stand-in");
        }
        fs::write(folder.join("answers.json"), answer.to_string()).expect("the answer");
        fs::write(folder.join("harness.mjs"), THE_NO_DOT_HARNESS.replace("SERVER", &a_server())).expect("the harness");
        let ran = std::process::Command::new("node").arg(folder.join("harness.mjs")).current_dir(&folder).output();
        let _ = fs::remove_dir_all(&folder);
        let ran = ran.ok()?;
        assert!(ran.status.success(), "the page code did not run: {}", String::from_utf8_lossy(&ran.stderr));
        Some(serde_json::from_slice(&ran.stdout).expect("the harness prints one answer"))
    }

    /// **The no-dot test's steps**: the page is opened on the one answer and
    /// drawn, and every piece under the head and the body is looked at.
    const THE_NO_DOT_HARNESS: &str = r#"import { everyPiece } from "./document.mjs";
import { readFileSync } from "node:fs";
const answer = JSON.parse(readFileSync("./answers.json", "utf8"));
globalThis.window = { addEventListener() {}, __TAURI__: { core: { invoke(command) {
  if (command === "the_render_settings_panel") return Promise.resolve(answer);
  return Promise.resolve(null);
} } } };
const { panels } = await import("./a-server.js");
const { theSettingsAreOpenedOn } = await import("./render-settings.js");
const settle = async () => { for (let at = 0; at < 8; at += 1) await new Promise((go) => setImmediate(go)); };
await theSettingsAreOpenedOn("SERVER", Array.from(Buffer.from(answer.file)), "");
const head = document.createElement("div");
const body = document.createElement("div");
panels[0].itIsDrawn({ head, body }, "SERVER");
await settle();
const all = [...everyPiece(head), ...everyPiece(body)];
const addOnLines = all.filter((one) => one.names.has("addon"));
console.log(JSON.stringify({
  mismatch: all.some((one) => one.names.has("vmm")),
  addOn: addOnLines.length > 0,
  addOnSays: addOnLines.map((one) => one.textContent),
  addOnControls: addOnLines.flatMap((one) => everyPiece(one))
    .filter((one) => one.tagName === "BUTTON" || one.tagName === "A" || one.names.has("lnkb")).length,
  dots: all.filter((one) => one.names.has("fdot")).length,
}));
"#;

    /// **The Creator's note 13 on 0.1.7: of the search's limits, only a build
    /// that did not answer is drawn** — `Server Setup` B11 and `Render
    /// Settings` B1 as rewritten 2026-10-02. **An answer with refused folders
    /// and the by-name mark draws neither**, though the answer keeps both.
    /// Before the piece both were drawn, and the by-name line on every machine.
    #[test]
    fn the_search_draws_only_a_build_that_did_not_answer() {
        let list = TheListThisPageAsksFor {
            entries: Vec::new(),
            answer: smoozy_server_setup::search::WhatTheAnswerSays {
                the_search_is_by_name: true,
                folders_refused: 1332,
                did_not_answer: vec!["/opt/blender-test/blender".to_string()],
            },
            at: std::time::SystemTime::now(),
        };
        let marks = the_search_marks(Some(&list));
        assert_eq!(marks.len(), 1, "{marks:?}");
        assert_eq!(marks[0].values, vec![Verbatim::new("/opt/blender-test/blender")]);
        assert_eq!(marks[0].rest, Some(Prose::new(words::DID_NOT_ANSWER)));
        let mut quiet = list.clone();
        quiet.answer.did_not_answer.clear();
        assert!(the_search_marks(Some(&quiet)).is_empty(), "an ordinary machine draws nothing");
    }

    fn the_page() -> String {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("web/render-settings.js");
        fs::read_to_string(path).expect("the page module is readable")
    }

    fn the_mockup() -> String {
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../Preparation/Design/Mockup/Smoozy.html");
        fs::read_to_string(path).expect("the drawing is readable")
    }

    pub(crate) fn a_server() -> String {
        "a".repeat(32)
    }

    pub(crate) fn a_layer(name: &str, renders: bool, overriding: bool) -> ViewLayerReading {
        ViewLayerReading {
            name: ExactName::as_the_reading_produced_it(name),
            renders_at: vec![(1, renders)],
            animation: SwitchAnimation::None { value: renders },
            own_sample_count: 1024,
            overrides_the_scenes_count: overriding,
        }
    }

    pub(crate) fn a_scene(name: &str) -> SceneReading {
        SceneReading {
            name: ExactName::as_the_reading_produced_it(name),
            frame_start: 1,
            frame_end: 240,
            frame_step: 1,
            frame_count: 240,
            full_size: (1920, 1080),
            resolution_percentage: 100,
            size_at_the_files_percentage: (1920, 1080),
            engine_as_read: ExactName::as_the_reading_produced_it("CYCLES"),
            sample_counts: PerEngineCounts {
                cycles: 2048,
                eevee: 64,
            },
            device: DeviceSetting::TheCards,
            early_stop: EarlyStop {
                adaptive_sampling_is_on: true,
                threshold: 0.01,
                minimum: 0,
            },
            time_limit: 30.0,
            layer_samples_rule: LayerSamplesRule::Use,
            saved_with: "4.5.3".to_string(),
            view_layers: vec![a_layer("Interior", true, false), a_layer("FX", true, true)],
            pulls_in: Vec::new(),
            simplify: SimplifyReadBack {
                use_simplify: false,
                subdivision_render: 6,
                child_particles_render: 1.0,
                texture_limit_render: "OFF".to_string(),
            },
        }
    }

    /// A run holding one reading of one file, which is what most of the 35
    /// states are drawn against.
    pub(crate) fn a_run_with_a_reading() -> WhatTheRunHolds {
        let mut held = WhatTheRunHolds::default();
        let key = FileKey {
            server: an_identity(&a_server()),
            path: "/ephemeral/smoozy/assets/kitchen_v14.blend".to_string(),
        };
        let picked = BlenderIdentity {
            path: "/opt/blender-5.1.0/blender".to_string(),
            version: a_version(5, 1, 0),
        };
        held.readings.a_reading_landed(
            key,
            &smoozy_render_settings::file_half::StartedUnder {
                version_pick: picked.clone(),
                file_identity: four_values(),
            },
            SettingsHalf {
                scenes: vec![a_scene("Interior"), a_scene("Exterior")],
                offered_engine_identifiers: OfferedEngineIdentifiers {
                    cycles: Some(ExactName::as_the_reading_produced_it("CYCLES")),
                    eevee: Some(ExactName::as_the_reading_produced_it("BLENDER_EEVEE")),
                },
                announced_add_on_case: None,
            },
            Some(&picked),
            &four_values(),
            std::time::SystemTime::now(),
        );
        held.version_lists.insert(
            a_server(),
            TheListThisPageAsksFor {
                entries: vec![smoozy_server_setup::search::ACandidate {
                    path: "/opt/blender-5.1.0/blender".to_string(),
                    answered: Some(a_version(5, 1, 0)),
                }],
                answer: smoozy_server_setup::search::WhatTheAnswerSays {
                    the_search_is_by_name: false,
                    folders_refused: 0,
                    did_not_answer: Vec::new(),
                },
                at: std::time::SystemTime::now(),
            },
        );
        held
    }

    pub(crate) fn a_version(major: u32, minor: u32, patch: u32) -> smoozy_server_setup::blender::Version {
        smoozy_server_setup::blender::Version {
            major,
            minor,
            patch,
        }
    }

    pub(crate) fn four_values() -> smoozy_server_protocol::transfers::SourceReading {
        smoozy_server_protocol::transfers::SourceReading {
            size: 4096,
            inode: "1".to_string(),
            modification_time: 1,
            change_time: 1,
        }
    }

    // ════════════════════════════════════════════════════════════════════════
    // What this half is counted by
    // ════════════════════════════════════════════════════════════════════════

    // ── The Creator's note 6 on 0.1.8: *Uploading dependencies* by Render ──

    fn the_uploading_words_of(sentence: &ASentence) -> String {
        sentence
            .iter()
            .map(|part| {
                let mut said = vec![part.lead.as_str().to_string()];
                said.extend(part.values.iter().map(|value| value.as_str().to_string()));
                said.extend(part.rest.as_ref().map(|rest| rest.as_str().to_string()));
                said.into_iter().filter(|one| !one.is_empty()).collect::<Vec<_>>().join(" ")
            })
            .collect::<Vec<_>>()
            .join(" ")
            .replace('\u{a0}', " ")
    }

    /// **`§ Submitting`: while the check's uploads for this file on this server
    /// run, one line above Render says how far they have got — *Uploading
    /// dependencies: 3 of 5 files.* — and Render stays offered.** Absent where
    /// nothing is going. The two figures are values, never inside the words.
    #[test]
    fn the_uploading_line_shows_while_the_checks_files_go_and_render_stays_offered() {
        let file = "/root/uploads/kitchen.blend";
        let held = WhatTheRunHolds::default();
        let none = the_panel_starting_from(&held, &a_server(), file, &WhatTheRecordHolds::default());
        assert!(none.the_dependencies_uploading.is_none());
        let going = WhatTheRecordHolds {
            the_checks_uploads: Some((3, 5)),
            ..WhatTheRecordHolds::default()
        };
        let panel = the_panel_starting_from(&held, &a_server(), file, &going);
        let line = panel.the_dependencies_uploading.as_ref().expect("the line, while two of five are going");
        assert_eq!(the_uploading_words_of(line), "Uploading dependencies: 3 of 5 files.");
        let values: Vec<&str> = line.iter().flat_map(|part| part.values.iter().map(Verbatim::as_str)).collect();
        assert_eq!(values, vec!["3", "5"]);
        assert!(!line.iter().any(|part| part.lead.as_str().contains('3') || part.lead.as_str().contains('5')));
        // **Render is not held by it**: submitting starts no transfer.
        assert_eq!(panel.submitting_is_offered, none.submitting_is_offered);
        assert_eq!(panel.the_render_control, none.the_render_control);
        let one = WhatTheRecordHolds {
            the_checks_uploads: Some((0, 1)),
            ..WhatTheRecordHolds::default()
        };
        let one = the_panel_starting_from(&held, &a_server(), file, &one);
        assert_eq!(the_uploading_words_of(one.the_dependencies_uploading.as_ref().expect("the line")), "Uploading dependencies: 0 of 1 file.");
        // The words are Design's own.
        let design = include_str!("../../../../Preparation/Design/Render Settings.md");
        assert!(design.contains("*Uploading dependencies: 4 of 12 files.*"));
    }

    /// **What the line counts is this file's check on this server, running or
    /// waiting, read from what Transfers lists** — the check's own sends, and
    /// nothing else: not another file's check, not another server's, not an
    /// ordinary upload, not a send that has ended.
    #[test]
    fn the_count_is_only_this_files_check_on_this_server_as_transfers_lists_it() {
        use crate::surface::transfers::{the_checks_uploads_in, the_requests_the_sends_hold, AServerKnown, WhatIsKnown};
        use crate::the_found_files_sent::{the_sends_this_run, AFileSent, ASend, HowItEnded};
        // Rows of their own, so no other test's sends are counted here, and
        // none of these is counted in another test.
        let (here, there) = ("c0ffee00c0ffee00c0ffee00c0ffee01", "c0ffee00c0ffee00c0ffee00c0ffee02");
        let (file, other) = ("/root/uploads/kitchen.blend", "/root/uploads/garden.blend");
        let a_send = |server: &str, file: &str, files: usize, published: usize, ended: usize| ASend {
            request: Identity::issue(IdentityKind::Operation).expect("randomness"),
            server: server.to_string(),
            file: file.to_string(),
            files: (0..files)
                .map(|at| AFileSent {
                    landing: format!("/root/smoozy/assets/{at}.png").into_bytes(),
                    size: 10,
                    transfer: None,
                    ended: if at < published {
                        Some(HowItEnded::Published)
                    } else if at < published + ended {
                        Some(HowItEnded::Failed)
                    } else {
                        None
                    },
                })
                .collect(),
        };
        let sends = [
            a_send(here, file, 5, 2, 1),
            a_send(here, file, 3, 1, 0),
            a_send(here, other, 7, 0, 0),
            a_send(there, file, 9, 0, 0),
            a_send(here, file, 4, 4, 0),
        ];
        let ours: Vec<String> = sends.iter().map(|send| send.request.as_text()).collect();
        the_sends_this_run().lock().expect("the register").extend(sends);
        let servers: Vec<AServerKnown> = [here, there]
            .iter()
            .map(|row| AServerKnown { row: row.to_string(), name: "box".into(), badge: None })
            .collect();
        let mut known = WhatIsKnown { requests: the_requests_the_sends_hold(&servers), servers, ..WhatIsKnown::default() };
        // An ordinary upload into the same server is not the check's.
        let mut an_upload = known.requests[0].clone();
        an_upload.kind = crate::surface::transfers::TheKind::Upload;
        known.requests.push(an_upload);
        the_sends_this_run()
            .lock()
            .expect("the register")
            .retain(|send| !ours.contains(&send.request.as_text()));
        // 2 + 1 uploaded, of 5 + 3; the finished send of four is not going.
        assert_eq!(the_checks_uploads_in(&known, here, file), Some((3, 8)));
        assert_eq!(the_checks_uploads_in(&known, there, file), Some((0, 9)));
        assert_eq!(the_checks_uploads_in(&known, here, "/root/uploads/nothing.blend"), None);
        // **And the panel's command reads it through Transfers' own reading**,
        // never a count of its own.
        let command = include_str!("render_settings.rs")
            .split("pub fn the_render_settings_panel<")
            .nth(1)
            .and_then(|rest| rest.split("\n}").next())
            .expect("the command")
            .to_string();
        assert!(command.contains("crate::surface::transfers::the_checks_uploads_on(&app, &server, &file)"));
    }

    /// **The page, under Node: the line sits on the foot's third line, under
    /// Render (the Creator's tenth notes, note 8; at the head of the foot,
    /// above Render, until then), its press opens the check on the rows already there, and the check's own
    /// event asks the page again, so the line goes when the last file lands.**
    /// Before the Creator's note 6 on 0.1.8 Render Settings said nothing while
    /// the check's files went (`Audits/Build/Step 10c/The Creator's notes on
    /// 0.1.8, piece 4 — what landed.md`). **Skipped, and says so, where no
    /// Node is installed.**
    #[test]
    fn the_page_draws_the_line_under_render_and_it_opens_the_check() {
        let file = "/root/uploads/kitchen.blend";
        let mut held = WhatTheRunHolds::default();
        let _ = the_answer_is_taken(&mut held, &a_server(), file, WhichSetting::DownloadAsTheyRender, "on");
        let going = the_panel_starting_from(
            &held,
            &a_server(),
            file,
            &WhatTheRecordHolds { the_checks_uploads: Some((4, 12)), ..WhatTheRecordHolds::default() },
        );
        let done = the_panel_starting_from(&held, &a_server(), file, &WhatTheRecordHolds::default());
        let answers = serde_json::json!({ "going": going, "done": done });
        let folder = std::env::temp_dir().join(format!("smoozy-uploading-line-page-{}", std::process::id()));
        let _ = fs::remove_dir_all(&folder);
        fs::create_dir_all(&folder).expect("a scratch folder");
        let web = Path::new(env!("CARGO_MANIFEST_DIR")).join("web");
        fs::copy(web.join("render-settings.js"), folder.join("render-settings.js")).expect("the page");
        for (name, text) in THE_UPLOADING_LINES_STAND_INS {
            fs::write(folder.join(name), text).expect("a stand-in");
        }
        fs::write(folder.join("answers.json"), answers.to_string()).expect("the answers");
        fs::write(folder.join("harness.mjs"), THE_UPLOADING_LINES_HARNESS.replace("SERVER", &a_server()))
            .expect("the harness");
        let ran = std::process::Command::new("node").arg(folder.join("harness.mjs")).current_dir(&folder).output();
        let _ = fs::remove_dir_all(&folder);
        let Ok(ran) = ran else {
            eprintln!("SKIPPED: no Node on this computer, so the render settings page was not run");
            return;
        };
        assert!(ran.status.success(), "the page code did not run: {}", String::from_utf8_lossy(&ran.stderr));
        let got: serde_json::Value = serde_json::from_slice(&ran.stdout).expect("the harness prints one answer");
        assert_eq!(
            got,
            serde_json::json!({
                "going": {
                    // **On the foot's third line, under Render** — the
                    // drawing's `.sfdep` — after the two lines under the
                    // switches this unread file with no folder also draws.
                    "theThirdLineHolds": ["bar-note", "adwarn", "adwarn sfdep"],
                    "itsLineIsTheThird": true,
                    "itReads": "Uploading dependencies: 4 of 12 files.",
                    "amber": true,
                    "renderIsThere": true
                },
                "pressed": [[a_server(), file]],
                // **The line goes from the third line, and the rest stay.**
                "afterTheLastFile": { "theLineIsThere": false, "theThirdLineHolds": ["bar-note", "adwarn"], "asked": 2 },
                "anotherFilesCheck": { "asked": 1 }
            })
        );
    }

    /// **The file's name in the panel's title is a link into Files** —
    /// `Design/Whole app § Panels over a server`, added 2026-10-12 (the
    /// Creator's ninth notes, note 5), drawn as the drawing's `.golink` inside
    /// the `h2` with the hover *Show in Files*.
    ///
    /// The page runs under Node over the panel this file composes. The name is
    /// a button in the title, the hover is the application's, and pressing it
    /// collapses the panel as its chevron does, hands Files **the folder and
    /// the name the application sent** — the page composes no path — and
    /// shows Files. **It fails if the name is plain text again, if the press
    /// leaves the panel up, or if what Files is handed is anything but the
    /// application's two.** Skipped, and says so, where no Node is installed.
    #[test]
    fn the_files_name_in_the_title_shows_it_in_files() {
        let file = "/home/ubuntu/kitchen/kitchen_v14.blend";
        let panel = the_panel(&WhatTheRunHolds::default(), &a_server(), file);
        let there = panel.show_in_files.clone().expect("the title's name has somewhere to go");
        assert_eq!(there.folder, b"/home/ubuntu/kitchen".to_vec(), "the folder is the path up to the name");
        assert_eq!(there.name, b"kitchen_v14.blend".to_vec(), "the name is the path's last part");
        assert_eq!(there.hover.as_str(), Prose::new(words::SHOW_IN_FILES).as_str());
        // **A file straight under the top of the server is in `/`**, and a
        // path naming no file sends Files nowhere.
        let top = the_file_in_files(b"/kitchen.blend").expect("a file at the top");
        assert_eq!((top.folder, top.name), (b"/".to_vec(), b"kitchen.blend".to_vec()));
        assert_eq!(the_file_in_files(b""), None);
        assert_eq!(the_file_in_files(b"/home/ubuntu/"), None);

        let answers = serde_json::json!({ "panel": panel });
        let folder = std::env::temp_dir().join(format!("smoozy-render-settings-name-link-{}", std::process::id()));
        let _ = fs::remove_dir_all(&folder);
        fs::create_dir_all(&folder).expect("a scratch folder");
        let web = Path::new(env!("CARGO_MANIFEST_DIR")).join("web");
        fs::copy(web.join("render-settings.js"), folder.join("render-settings.js")).expect("the page");
        for (name, text) in THE_UPLOADING_LINES_STAND_INS {
            fs::write(folder.join(name), text).expect("a stand-in");
        }
        fs::write(folder.join("answers.json"), answers.to_string()).expect("the answers");
        fs::write(folder.join("harness.mjs"), THE_NAME_LINK_HARNESS.replace("SERVER", &a_server()))
            .expect("the harness");
        let ran = std::process::Command::new("node").arg(folder.join("harness.mjs")).current_dir(&folder).output();
        let _ = fs::remove_dir_all(&folder);
        let Ok(ran) = ran else {
            eprintln!("SKIPPED: no Node on this computer, so the render settings page was not run");
            return;
        };
        assert!(ran.status.success(), "the page code did not run: {}", String::from_utf8_lossy(&ran.stderr));
        let got: serde_json::Value = serde_json::from_slice(&ran.stdout).expect("the harness prints one answer");
        let bytes = |text: &str| serde_json::json!(text.as_bytes());
        assert_eq!(
            got,
            serde_json::json!({
                "reads": format!("{} kitchen_v14.blend", Prose::new(words::RENDER_SETTINGS_TITLE).as_str()),
                "link": {
                    "looks": "golink",
                    "type": "button",
                    "hover": Prose::new(words::SHOW_IN_FILES).as_str(),
                    "says": "kitchen_v14.blend"
                },
                "pressed": [
                    ["collapsed", a_server()],
                    ["opened-at", a_server(), bytes("/home/ubuntu/kitchen"), bytes("kitchen_v14.blend")],
                    ["tab", "files"]
                ]
            }),
            "the file's name in the render settings title is not a link into Files"
        );
    }

    /// Draws the panel, reads its title, and presses the name.
    const THE_NAME_LINK_HARNESS: &str = r#"import { everyPiece } from "./document.mjs";
import { readFileSync } from "node:fs";
const answers = JSON.parse(readFileSync("./answers.json", "utf8"));
globalThis.window = { addEventListener() {}, __TAURI__: { core: { invoke(command) {
  return Promise.resolve(command === "the_render_settings_panel" ? answers.panel : null);
} } } };
const { panels } = await import("./a-server.js");
const { theSettingsAreOpenedOn } = await import("./render-settings.js");
const settle = async () => { for (let at = 0; at < 8; at += 1) await new Promise((go) => setImmediate(go)); };
await theSettingsAreOpenedOn("SERVER", Array.from(Buffer.from(answers.panel.file)), "");
const head = document.createElement("div");
const body = document.createElement("div");
panels[0].itIsDrawn({ head, body }, "SERVER");
await settle();
const heading = everyPiece(head).find((one) => one.tagName === "H2");
const link = heading ? everyPiece(heading).find((one) => one.tagName === "BUTTON") : null;
const out = {
  reads: heading ? heading.textContent : null,
  link: link ? { looks: link.className, type: link.type, hover: link.title, says: link.textContent } : null,
};
globalThis.calls = [];
if (link) link.press();
await settle();
out.pressed = globalThis.calls;
console.log(JSON.stringify(out));
"#;

    /// **The Creator's seventh notes, note 2: Render Settings opened from its
    /// strip while it was never opened over that server starts nothing.** The
    /// page's registration hands `web/a-server.js` the file it is open on, and
    /// its way in from the strip is `theSettingsAreOpenedOn` on the check's
    /// file. **It fails if the strip's way in asks the application for anything
    /// but opening the page on that file**, piece Z6.
    ///
    /// **Since step 10c's owed piece 2 it also hands the saved-with version
    /// Files read for that file** (`theSavedWithVersionReadFor` in
    /// `web/files.js`, stood in here), and blank where Files holds no read,
    /// leaving the application's own fallback ([`the_file_was_pointed_at`]).
    /// Opened before
    /// the read, then after it; the read is keyed by the server, so the same
    /// path on another server is handed nothing. **It fails if the strip's way
    /// in hands `""` where Files read a version**, which is what it did before.
    /// **Skipped, and says so, where no Node is installed.**
    #[test]
    fn opened_from_its_strip_the_page_starts_nothing() {
        let folder = std::env::temp_dir().join(format!("smoozy-settings-from-a-strip-{}", std::process::id()));
        let _ = fs::remove_dir_all(&folder);
        fs::create_dir_all(&folder).expect("a scratch folder");
        let web = Path::new(env!("CARGO_MANIFEST_DIR")).join("web");
        fs::copy(web.join("render-settings.js"), folder.join("render-settings.js")).expect("the page");
        for (name, text) in THE_UPLOADING_LINES_STAND_INS {
            fs::write(folder.join(name), text).expect("a stand-in");
        }
        fs::write(folder.join("harness.mjs"), THE_FROM_A_STRIP_HARNESS).expect("the harness");
        let ran = std::process::Command::new("node").arg(folder.join("harness.mjs")).current_dir(&folder).output();
        let _ = fs::remove_dir_all(&folder);
        let Ok(ran) = ran else {
            eprintln!("SKIPPED: no Node on this computer, so the page code was not run");
            return;
        };
        assert!(ran.status.success(), "the page code did not run: {}", String::from_utf8_lossy(&ran.stderr));
        let got: serde_json::Value = serde_json::from_slice(&ran.stdout).expect("the harness prints one answer");
        assert_eq!(
            got,
            serde_json::json!({
                "before": null,
                "asked": [
                    ["the_file_was_pointed_at", [1, 2, 3], ""],
                    ["the_version_list_is_asked_for", [1, 2, 3], null]
                ],
                "after": [1, 2, 3],
                "afterTheRead": [
                    ["the_file_was_pointed_at", [1, 2, 3], "4.5.3"],
                    ["the_version_list_is_asked_for", [1, 2, 3], null]
                ],
                "onAnotherServer": [
                    ["the_file_was_pointed_at", [1, 2, 3], ""],
                    ["the_version_list_is_asked_for", [1, 2, 3], null]
                ],
                "checksStarted": 0
            }),
            "Render Settings opened from its strip asked for more than its page"
        );
    }

    /// `Server Setup` B12: one completed install or removal asks once for the
    /// open page's server and exact file. Other setup news starts no search.
    #[test]
    fn piece_f_the_open_page_asks_once_after_a_blender_run() {
        let folder = std::env::temp_dir().join(format!("smoozy-piece-f-page-{}", std::process::id()));
        fs::create_dir_all(&folder).expect("a scratch folder");
        let web = Path::new(env!("CARGO_MANIFEST_DIR")).join("web");
        fs::copy(web.join("render-settings.js"), folder.join("render-settings.js")).expect("the page");
        for (name, text) in THE_UPLOADING_LINES_STAND_INS {
            fs::write(folder.join(name), text).expect("a stand-in");
        }
        fs::write(folder.join("harness.mjs"), THE_BLENDER_RUN_HARNESS).expect("the harness");
        let ran = std::process::Command::new("node").arg(folder.join("harness.mjs")).current_dir(&folder).output();
        let _ = fs::remove_dir_all(&folder);
        let ran = ran.expect("Node is required for piece F's page proof");
        assert!(ran.status.success(), "the page did not run: {}", String::from_utf8_lossy(&ran.stderr));
        let got: serde_json::Value = serde_json::from_slice(&ran.stdout).expect("the page's answer");
        let ask = serde_json::json!([["the_version_list_is_asked_for", { "server": "S", "file": [1, 2, 255] }]]);
        assert_eq!(got["installed"], ask, "the installed Blender was not asked for once");
        assert_eq!(got["removed"], ask, "the removed Blender was not asked for once");
        assert_eq!(got["otherNews"], serde_json::json!([]), "unrelated news started a search");
    }

    /// A search begun before an install ended must not replace the new list,
    /// whether it answers before or after the fresh search.
    #[test]
    fn piece_f_an_older_search_cannot_put_the_old_list_back() {
        use smoozy_render_settings::acting::versions::TheVersionListAsk;
        let server = a_server();
        let file = "/ephemeral/smoozy/assets/kitchen_v14.blend";
        for old_answers_first in [true, false] {
            let mut held = a_run_with_a_reading();
            let old_list = held.version_lists[&server].clone();
            let old = the_version_search_begins(&mut held, &server).expect("the first search");
            assert!(the_version_search_begins(&mut held, &server).is_none(), "a second search began");
            the_blender_run_ended_for(&mut held, &server);
            assert!(held.readings.standing(&FileKey { server: an_identity(&server), path: file.to_string() }).is_some());
            let fresh = the_version_search_begins(&mut held, &server).expect("a fresh search can begin");
            if old_answers_first {
                assert!(!the_version_search_came_back(&mut held, &server, &old, TheVersionListAsk::Answered(old_list.clone())));
                assert!(held.version_lists.get(&server).is_none());
                assert_eq!(held.version_searches.get(&server), Some(&TheSearchStands::Searching));
            }
            assert!(the_version_search_came_back(&mut held, &server, &fresh, TheVersionListAsk::Answered(a_list_of(&[(5, 4, 0)]))));
            assert!(!the_version_search_came_back(&mut held, &server, &old, TheVersionListAsk::Answered(old_list)));
            assert_eq!(held.version_lists[&server].entries[0].answered.as_ref().expect("a version").minor, 4);
            assert!(held.version_searches.get(&server).is_none());
            let next = the_version_search_begins(&mut held, &server).expect("another ask");
            the_period_ended_for(&mut held, &server);
            assert!(!the_version_search_came_back(&mut held, &server, &next, TheVersionListAsk::Answered(a_list_of(&[(5, 4, 0)]))));
            assert!(!held.version_lists.contains_key(&server));
        }
    }

    /// The panel that survived a break asks once when requests can be served,
    /// even in its strip. The answer restores Read and Render without replacing
    /// the user's committed answers or the text still being typed.
    #[test]
    fn reconnect_restores_the_open_and_collapsed_settings_panel() {
        let server = a_server();
        let file = "/ephemeral/smoozy/assets/kitchen_v14.blend";
        let mut held = a_run_with_a_reading();
        held.the_saved_with.insert(keyed(&server, file), "5.1.0".to_string());
        the_read_ticks_its_scene(&mut held, &server, file, Some(b"Interior"));
        assert!(the_answer_is_taken(&mut held, &server, file, WhichSetting::SampleCount, "321").is_none());
        assert!(the_answer_is_taken(&mut held, &server, file, WhichSetting::JobName, "My render").is_none());
        let list = held.version_lists[&server].clone();
        the_period_ended_for(&mut held, &server);
        let disconnected = the_panel(&held, &server, file);
        assert!(!disconnected.the_read_is_offered && !disconnected.submitting_is_offered);
        struct ThePage(std::sync::Arc<Mutex<Option<serde_json::Value>>>);
        impl crate::events::Delivers for ThePage {
            fn deliver(&self, delivery: &crate::events::Delivery) {
                *self.0.lock().unwrap() = Some(serde_json::to_value(delivery).unwrap());
            }
        }
        let arrived = std::sync::Arc::new(Mutex::new(None));
        let queue = TheOneQueue::new();
        queue.the_page_is_ready(Box::new(ThePage(arrived.clone())));
        the_queue_is_told(&queue, the_new_connection_for(&mut held, &server));
        let restoration = arrived.lock().unwrap().take().expect("the restoration reaches the page");
        the_version_list_came_back(&mut held, &server, smoozy_render_settings::acting::versions::TheVersionListAsk::Answered(list));
        let restored = the_panel(&held, &server, file);
        assert!(restored.the_read_is_offered && restored.submitting_is_offered);
        assert_eq!(restored.samples.value.as_ref().map(|value| value.as_str()), Some("321"));
        for collapsed in [false, true] {
            let folder = std::env::temp_dir().join(format!("smoozy-reconnect-settings-{}-{collapsed}", std::process::id()));
            fs::create_dir_all(&folder).expect("a scratch folder");
            let web = Path::new(env!("CARGO_MANIFEST_DIR")).join("web");
            for (name, text) in THE_UPLOADING_LINES_STAND_INS {
                fs::write(folder.join(name), text).expect("a stand-in");
            }
            for name in ["render-settings.js", "the-typed-text.js"] {
                fs::copy(web.join(name), folder.join(name)).expect("the page code");
            }
            fs::write(folder.join("the-click.mjs"), THE_CLICK_STAND_IN).expect("the document's focus");
            fs::write(folder.join("answers.json"), serde_json::json!({
                "disconnected": disconnected, "restored": restored, "restoration": restoration,
                "server": server, "collapsed": collapsed
            }).to_string()).expect("the page answers");
            fs::write(folder.join("harness.mjs"), THE_RECONNECT_HARNESS).expect("the harness");
            let ran = std::process::Command::new("node").arg(folder.join("harness.mjs")).current_dir(&folder).output();
            let _ = fs::remove_dir_all(&folder);
            let ran = ran.expect("Node is required for the reconnect proof");
            assert!(ran.status.success(), "collapsed={collapsed}: {}", String::from_utf8_lossy(&ran.stderr));
            let got: serde_json::Value = serde_json::from_slice(&ran.stdout).expect("the page's answer");
            assert_eq!(got, serde_json::json!({
                "unrelated": [], "searches": ["the_version_list_is_asked_for"],
                "readRestored": true, "renderRestored": true, "disconnectedLineGone": true,
                "samplesKept": true, "typingKept": true, "noEditsSent": true,
                "secondRestoration": ["the_version_list_is_asked_for"]
            }), "collapsed={collapsed}");
        }
    }

    /// A new usable connection rejects the old search even if no panel was
    /// drawn during the break. Its late answer is harmless in either order.
    #[test]
    fn reconnect_rejects_an_old_search_before_and_after_the_fresh_answer() {
        use smoozy_render_settings::acting::versions::TheVersionListAsk;
        let server = a_server();
        for old_answers_first in [true, false] {
            let mut held = a_run_with_a_reading();
            let old_list = held.version_lists[&server].clone();
            let other_server = "b".repeat(32);
            held.version_lists.insert(other_server.clone(), old_list.clone());
            let old = the_version_search_begins(&mut held, &server).expect("the old search");
            let _ = the_new_connection_for(&mut held, &server);
            assert!(!held.version_lists.contains_key(&server));
            assert!(held.version_lists.contains_key(&other_server));
            let fresh = the_version_search_begins(&mut held, &server).expect("the restored connection can search");
            assert!(the_version_search_begins(&mut held, &server).is_none(), "only one search runs");
            if old_answers_first {
                assert!(!the_version_search_came_back(&mut held, &server, &old, TheVersionListAsk::Answered(old_list.clone())));
                assert_eq!(held.version_searches.get(&server), Some(&TheSearchStands::Searching));
                assert!(!held.version_lists.contains_key(&server));
            }
            assert!(the_version_search_came_back(&mut held, &server, &fresh, TheVersionListAsk::Answered(a_list_of(&[(5, 4, 0)]))));
            assert!(!the_version_search_came_back(&mut held, &server, &old, TheVersionListAsk::Answered(old_list)));
            assert_eq!(held.version_lists[&server].entries[0].answered.as_ref().unwrap().minor, 4);
            assert!(!held.version_searches.contains_key(&server));
        }
    }

    const THE_RECONNECT_HARNESS: &str = r#"import { everyPiece } from "./document.mjs";
import { fire } from "./the-click.mjs";
import { readFileSync } from "node:fs";
const { disconnected, restored, restoration, server, collapsed } = JSON.parse(readFileSync("./answers.json", "utf8"));
const asked = [];
let panel = disconnected;
globalThis.window = { addEventListener() {}, __TAURI__: { core: { invoke(command) {
  asked.push(command);
  return Promise.resolve(command === "the_render_settings_panel" ? panel : null);
} } } };
const { panels } = await import("./a-server.js");
const { theSettingsAreOpenedOn } = await import("./render-settings.js");
const settle = async () => { for (let at = 0; at < 8; at++) await new Promise(go => setImmediate(go)); };
const tell = async (kind, said) => {
  for (const handler of globalThis.told[kind] || []) await handler({ said });
  await settle();
};
await theSettingsAreOpenedOn(server, Array.from(Buffer.from(disconnected.file)), "");
const head = document.createElement("div"), body = document.createElement("div");
head.inThePage = body.inThePage = true;
panels[0].itIsDrawn({ head, body }, server);
await settle();
head.hidden = body.hidden = collapsed;
const find = box => [...everyPiece(head), ...everyPiece(body)].find(one => one.dataset.box === box);
if (!find("read").disabled || !find("render").disabled) throw Error("the disconnected controls were enabled");
const name = find("job-name");
name.focus(); name.value = "Still typing"; name.setSelectionRange(5, 5, "none"); fire(name, "input");
asked.length = 0;
await tell("servers:what-is-observed", { theConditionsValue: "connected" });
await tell("render-settings:the-connection-is-lent", { server: "another-server" });
const out = { unrelated: asked.splice(0) };
await tell(restoration.kind, restoration.said);
out.searches = asked.splice(0);
// The real search answers on the normal page event; no panel is opened again.
panel = restored;
await tell("render-settings:an-act", { server, file: restored.file, theBox: "version-list" });
out.readRestored = !find("read").disabled;
out.renderRestored = !find("render").disabled;
out.disconnectedLineGone = !head.textContent.includes(disconnected.the_pick_line);
out.samplesKept = find("samples-field").value === "321";
out.typingKept = find("job-name").value === "Still typing" && find("job-name").selectionStart === 5;
out.noEditsSent = asked.every(command => command === "the_render_settings_panel");
asked.length = 0;
if (collapsed) { panels[0].itIsShowingAgain(server); await settle(); }
await tell("servers:what-is-observed", { theConditionsValue: "reconnecting" });
await tell("servers:what-is-observed", { theConditionsValue: "connected" });
asked.length = 0;
await tell(restoration.kind, restoration.said);
out.secondRestoration = asked.splice(0);
console.log(JSON.stringify(out));
"#;

    const THE_BLENDER_RUN_HARNESS: &str = r#"import "./document.mjs";
const asked = [];
globalThis.window = { addEventListener() {}, __TAURI__: { core: { invoke(command, args) {
  asked.push([command, args]);
  return Promise.resolve(null);
} } } };
const { theSettingsAreOpenedOn } = await import("./render-settings.js");
await theSettingsAreOpenedOn("S", [1, 2, 255], "");
asked.length = 0;
const tell = async (kind, said) => {
  for (const handler of globalThis.told[kind] || []) await handler({ said });
};
await tell("set-up:the-answer", { server: "S", the_run_is_under_way: "yes" });
await tell("set-up:the-answer", { server: "S", the_blenders_are_read_again: "yes" });
await tell("set-up:the-answer", { server: "T", the_render_versions_are_read_again: "yes" });
const out = { otherNews: asked.splice(0) };
await tell("set-up:the-answer", { server: "S", the_render_versions_are_read_again: "yes" });
out.installed = asked.splice(0);
await tell("set-up:the-answer", { server: "S", the_render_versions_are_read_again: "yes" });
out.removed = asked.splice(0);
console.log(JSON.stringify(out));
"#;

    const THE_FROM_A_STRIP_HARNESS: &str = r#"import "./document.mjs";
const asked = [];
globalThis.window = { addEventListener() {}, __TAURI__: { core: { invoke(command, args) {
  asked.push([command, (args && args.file) || null, args && "savedWith" in args ? args.savedWith : null]);
  return Promise.resolve(null);
} } } };
const { panels } = await import("./a-server.js");
await import("./render-settings.js");
const settle = async () => { for (let at = 0; at < 8; at += 1) await new Promise((go) => setImmediate(go)); };
const before = panels[0].theFileItIsOn("S");
const opened = async (server) => {
  asked.length = 0;
  await panels[0].itIsShownOn(server, [1, 2, 3]);
  await settle();
  return asked.slice().sort((one, two) => one[0].localeCompare(two[0]));
};
const out = { before };
out.asked = await opened("S");
out.after = panels[0].theFileItIsOn("S");
// **Files reads the file**: the stand-in's lookup now holds its version.
globalThis.theReads = { "S:1,2,3": "4.5.3" };
out.afterTheRead = await opened("S");
out.onAnotherServer = await opened("T");
out.checksStarted = globalThis.checksStarted || 0;
console.log(JSON.stringify(out));
"#;

    /// **What the page shows from the version the strip's way in hands** —
    /// step 10c's owed piece 2, the application's half of
    /// [`opened_from_its_strip_the_page_starts_nothing`]. A version Files read,
    /// handed over, is what the page matches on (*matched to the file*); a
    /// blank one, with no connection to read it again through `File Browser`'s
    /// reading, draws Design's *file does not say which Blender saved it* case
    /// (`§ When there is no version to pick`). On a file never read, so the
    /// handed version is the only source.
    #[test]
    fn the_version_files_read_is_what_the_page_matches_on_and_blank_is_designs_case() {
        let file = "/ephemeral/smoozy/assets/never_read_here.blend";
        let with_a_list = || {
            let mut held = WhatTheRunHolds::default();
            held.version_lists = a_run_with_a_reading().version_lists;
            held
        };
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("a runtime");
        let handed = |saved_with: &str| {
            runtime.block_on(the_saved_with_version_to_hold::<smoozy_server_protocol::transport::ssh::Connection>(
                None,
                &a_server(),
                file.as_bytes(),
                saved_with,
            ))
        };

        let mut held = with_a_list();
        the_saved_with_version_lands(&mut held, &a_server(), file, &handed("5.1.0"));
        assert_eq!(
            the_panel(&held, &a_server(), file).the_pick_line,
            Some(Prose::new(words::MATCHED_TO_THE_FILE)),
            "the version Files read is the one the page matches on"
        );

        let mut held = with_a_list();
        the_saved_with_version_lands(&mut held, &a_server(), file, &handed(""));
        assert_eq!(
            the_panel(&held, &a_server(), file).the_pick_line,
            Some(Prose::new(words::NOTHING_TO_MATCH_ON)),
            "before any read the page draws Design's existing case"
        );
    }

    const THE_UPLOADING_LINES_STAND_INS: [(&str, &str); 14] = [
        ("servers.js", "export function theMomentReads(seconds) { return `at ${seconds}`; }"),
        (
            "files.js",
            "export function theBrowserIsOpenedAt(...args) { (globalThis.calls ||= []).push(['opened-at', ...args]); } \
             export function theSavedWithVersionReadFor(server, file) { \
               return (globalThis.theReads || {})[`${server}:${file.join(',')}`] || ''; }",
        ),
        (
            "the-rail.js",
            "export const THE_QUEUE = 'queueview'; \
             export function theCurrentPlaceIs(place, which) { (globalThis.went ||= []).push([place, which]); }",
        ),
        (
            "a-server.js",
            "export const RENDER_SETTINGS = 'set'; export const SET_UP = 'setupview'; export const FILES = 'files'; \
             export const panels = []; \
             export function aPanelIsRegistered(panel) { panels.push(panel); } \
             export function thePanelIsOpenedOver() {} export function theStripReads() {} \
             export function theCurrentTabIs(tab) { (globalThis.calls ||= []).push(['tab', tab]); } \
             export function thePanelCollapsesAsItsChevronDoes(server) { (globalThis.calls ||= []).push(['collapsed', server]); }",
        ),
        (
            "the-dependency-check.js",
            "export async function theCheckIsOpenedOn() { globalThis.checksStarted = (globalThis.checksStarted || 0) + 1; } \
             export async function theCheckIsShownOn(server, file) { (globalThis.shown ||= []).push([server, String.fromCharCode(...file)]); }",
        ),
        ("transfers.js", "export function aTransferIsAskedFor() {} export async function theFormatBoxIsOpenedForAFile() {}"),
        ("event-route.js", "export function whenTold(kind, handler) { ((globalThis.told ||= {})[kind] ||= []).push(handler); }"),
        ("the-failure-shape.js", "export const EXPANDED = 'e'; export function theFailureShape() { return document.createElement('div'); }"),
        ("the-machines-own-words.js", "export function theMachinesOwnWordsFor() {}"),
        ("the-underway-shape.js", "export function theMarkOnAControl() {} export function theMarkOnARow() { return document.createElement('span'); }"),
        ("the-typed-text.js", "export function theTypingIsTakenFrom() { return null; } export function theTypingIsPutBackInto() {}"),
        (
            "the-value-drawn.js",
            "export function theValueIsDrawnInto(node, value) { node.textContent = value; } \
             export function theValueDrawn(value) { const node = document.createElement('span'); node.textContent = value; return node; }",
        ),
        ("package.json", r#"{ "type": "module" }"#),
        (
            "document.mjs",
            r#"class TextPiece {
  constructor(text) { this.data = String(text); this.parentNode = null; }
  get textContent() { return this.data; }
  remove() { if (this.parentNode) this.parentNode.removeChild(this); }
}
class Piece {
  constructor(tag) {
    this.tagName = tag.toUpperCase(); this.children = []; this.own = ""; this.title = ""; this.style = {};
    this.dataset = {}; this.hidden = false; this.type = ""; this.disabled = false; this.value = "";
    this.attributes = {}; this.parentNode = null; this.open = false; this.listeners = {};
    const names = new Set();
    this.names = names;
    this.classList = {
      add: (...more) => more.forEach((one) => names.add(one)), remove: (...more) => more.forEach((one) => names.delete(one)),
      toggle: (one, on) => { const want = on === undefined ? !names.has(one) : Boolean(on); if (want) names.add(one); else names.delete(one); return want; },
      contains: (one) => names.has(one),
    };
  }
  set className(value) { this.names.clear(); String(value).split(/\s+/).filter(Boolean).forEach((one) => this.names.add(one)); }
  get className() { return [...this.names].join(" "); }
  set textContent(value) { this.children.forEach((one) => { one.parentNode = null; }); this.children = []; this.own = String(value); }
  get textContent() { return this.own + this.children.map((one) => one.textContent).join(""); }
  appendChild(child) { if (child.parentNode) child.parentNode.removeChild(child); child.parentNode = this; this.children.push(child); return child; }
  append(...more) { more.forEach((one) => this.appendChild(one)); }
  removeChild(child) { const at = this.children.indexOf(child); if (at >= 0) this.children.splice(at, 1); child.parentNode = null; return child; }
  replaceChildren(...more) { this.textContent = ""; this.append(...more); }
  remove() { if (this.parentNode) this.parentNode.removeChild(this); }
  after(node) { const parent = this.parentNode; if (!parent) return; if (node.parentNode) node.parentNode.removeChild(node); node.parentNode = parent; parent.children.splice(parent.children.indexOf(this) + 1, 0, node); }
  contains(node) { for (let at = node; at; at = at.parentNode) if (at === this) return true; return false; }
  get parentElement() { return this.parentNode; }
  get firstChild() { return this.children[0] || null; }
  get childElementCount() { return this.children.filter((one) => one instanceof Piece).length; }
  setAttribute(name, value) { this.attributes[name] = String(value); }
  getAttribute(name) { return name in this.attributes ? this.attributes[name] : null; }
  removeAttribute(name) { delete this.attributes[name]; }
  addEventListener(kind, handler) { (this.listeners[kind] ||= []).push(handler); }
  press() { for (const handler of this.listeners.click || []) handler({ preventDefault() {}, target: this }); }
  focus() { document.activeElement = this; }
  closest() { return null; }
  querySelector() { return null; }
  querySelectorAll() { return []; }
  getBoundingClientRect() { return this.rect || { top: 0, bottom: 0, left: 0, right: 0, width: 0, height: 0 }; }
}
export function everyPiece(root, found = []) {
  if (!(root instanceof Piece)) return found;
  found.push(root);
  root.children.forEach((one) => everyPiece(one, found));
  return found;
}
globalThis.document = {
  createElement: (tag) => new Piece(tag), createTextNode: (text) => new TextPiece(text),
  activeElement: null, addEventListener() {}, querySelector() { return null; }, body: new Piece("body"),
};
globalThis.CSS = { escape: (text) => text };
"#,
        ),
    ];

    /// Opens the panel while the check's files go, presses the line, then
    /// tells the page the check moved — once for another file, once for this
    /// one after its last file landed.
    const THE_UPLOADING_LINES_HARNESS: &str = r#"import { everyPiece } from "./document.mjs";
import { readFileSync } from "node:fs";
const answers = JSON.parse(readFileSync("./answers.json", "utf8"));
let stage = "going";
let asked = 0;
globalThis.window = { addEventListener() {}, __TAURI__: { core: { invoke(command) {
  if (command === "the_render_settings_panel") { asked += 1; return Promise.resolve(answers[stage]); }
  return Promise.resolve(null);
} } } };
const { panels } = await import("./a-server.js");
const { theSettingsAreOpenedOn } = await import("./render-settings.js");
const settle = async () => { for (let at = 0; at < 8; at += 1) await new Promise((go) => setImmediate(go)); };
const file = Array.from(Buffer.from(answers.going.file));
await theSettingsAreOpenedOn("SERVER", file, "");
const head = document.createElement("div");
const body = document.createElement("div");
panels[0].itIsDrawn({ head, body }, "SERVER");
await settle();
asked = 1;
const foot = () => everyPiece(body).find((one) => one.names.has("sfoot"));
const line = () => everyPiece(body).find((one) => one.dataset.box === "dependencies-uploading");
const third = () => foot().children.find((one) => one.names.has("sfl3")) || null;
const held = line().parentNode;
const out = { going: {
  theThirdLineHolds: third() ? third().children.map((one) => one.className) : null,
  itsLineIsTheThird: held.parentNode === third(),
  itReads: line().textContent.replace(/ /g, " ").replace(/\s+/g, " ").trim(),
  amber: held.names.has("adwarn"),
  renderIsThere: Boolean(everyPiece(body).find((one) => one.dataset.box === "render")),
} };
line().press();
await settle();
out.pressed = globalThis.shown || [];
const tell = async (said) => { for (const handler of globalThis.told["the-dependency-check:an-act"] || []) await handler({ said }); await settle(); };
await tell({ server: "SERVER", file: "/root/uploads/garden.blend" });
out.anotherFilesCheck = { asked };
stage = "done";
await tell({ server: "SERVER", file: answers.going.file });
out.afterTheLastFile = { theLineIsThere: Boolean(line()), theThirdLineHolds: third() ? third().children.map((one) => one.className) : null, asked };
console.log(JSON.stringify(out));
"#;

    /// **The states are the drawing's own, in both directions.**
    ///
    /// The runner ruled on 2026-09-12 that the mockup state names are what this
    /// half is counted by, so **this is the line that fails if the drawing
    /// gains a state this piece does not draw, or loses one it does.**
    #[test]
    fn the_states_are_the_drawings_own() {
        let drawing = the_mockup();
        let list = drawing
            .split("<select id=\"state\"")
            .nth(1)
            .and_then(|rest| rest.split("</select>").next())
            .expect("the drawing has a state list");
        let group = list
            .split("Render settings\">")
            .nth(1)
            .and_then(|rest| rest.split("</optgroup>").next())
            .expect("the drawing has a Render settings group");
        let mut drawn: Vec<&str> = Vec::new();
        for (at, _) in group.match_indices("<option value=\"") {
            let rest = &group[at + "<option value=\"".len()..];
            drawn.push(&rest[..rest.find('"').expect("the value closes")]);
        }
        let mut ours: Vec<&str> = THE_STATES_THIS_PIECE_DRAWS.to_vec();
        drawn.sort_unstable();
        ours.sort_unstable();
        assert_eq!(
            drawn, ours,
            "the drawing's Render settings group and this piece's list have \
             drifted. **The mockup state names are what this half is counted \
             by**, so one of them is now wrong"
        );
        assert!(
            THE_STATES_THIS_PIECE_DRAWS.contains(&THE_ONE_STATE_DRAWN_ON_ANOTHER_PIECES_PAGE),
            "the one state whose drawing is piece 6's is not in this piece's \
             own list, so nothing counts it at all"
        );
    }

    /// **The page names this file's own command and the drawing's own panel
    /// key.** Nothing holds the two sides together at compile time.
    #[test]
    fn the_page_and_this_file_name_the_same_things() {
        let page = the_page();
        for named in [
            "the_render_settings_panel",
            "an_answer_was_given",
            "a_value_is_handed_back",
            "a_scene_is_ticked",
            "every_scene_is_ticked",
            "a_view_layer_is_pinned",
            "a_pin_is_handed_back",
            "the_file_is_read",
            "the_read_failure_is_cleared",
            "the_missing_assets_warning_is_switched_off",
            "a_job_is_submitted",
            AN_ACT_ON_THIS_PAGE,
        ] {
            assert!(
                page.contains(named),
                "web/render-settings.js does not name {named}, so the two \
                 sides have drifted and nothing else would say so"
            );
        }
        assert!(
            page.contains("RENDER_SETTINGS"),
            "the page does not import the drawing's own panel key from \
             web/a-server.js. **Import the name, never type it.**"
        );
    }

    /// **The check is opened on the file these settings are of, never on a
    /// bare server.** Piece 9's panel opened with no file says it does not know
    /// which file to check, and that is what the dependency band did until the
    /// repair of 2026-09-13.
    #[test]
    fn the_dependency_check_is_opened_on_this_panels_file() {
        let page = the_page();
        let code: String = page
            .lines()
            .filter(|line| {
                let line = line.trim_start();
                !(line.starts_with("//") || line.starts_with("*"))
            })
            .collect::<Vec<_>>()
            .join("\n");
        assert!(
            code.contains("import { theCheckIsOpenedOn, theCheckIsShownOn } from \"./the-dependency-check.js\""),
            "the page does not import the dependency check's two ways in"
        );
        assert!(
            !code.contains("THE_DEPENDENCY_CHECK"),
            "**the page opens the bare dependency check panel**, which hands it \
             a server and no file"
        );
        let helper = code
            .split("function theCheckIsOpened(server) {")
            .nth(1)
            .and_then(|rest| rest.split("\n}").next())
            .expect("the page has its one way to the check");
        assert!(
            helper.contains("theCheckIsOpenedOn(server, held ? held.file : [])"),
            "the way to the check does not hand over the file these settings are of"
        );
        assert!(
            code.matches("theCheckIsOpenedOn(").count() == 1,
            "every way that starts a check goes through the one helper"
        );
        let shown = code
            .split("function theCheckIsShown(server) {")
            .nth(1)
            .and_then(|rest| rest.split("\n}").next())
            .expect("the page has its way to the rows already there");
        assert!(shown.contains("theCheckIsShownOn(server, held ? held.file : [])"));
    }

    // ════════════════════════════════════════════════════════════════════════
    // The step 10a round 1 audit of these pieces: one test per must-do that a
    // test can catch, each one that would have failed before its fix
    // ════════════════════════════════════════════════════════════════════════

    /// The page's working lines, comment lines taken out.
    fn the_page_code() -> String {
        the_page()
            .lines()
            .filter(|line| {
                let line = line.trim_start();
                !(line.starts_with("//") || line.starts_with('*'))
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// One function of the page, by its opening line.
    fn the_pages_function(opening: &str) -> String {
        the_page_code()
            .split(opening)
            .nth(1)
            .and_then(|rest| rest.split("\n}").next())
            .unwrap_or_else(|| panic!("the page has {opening}"))
            .to_string()
    }

    /// **Must-do 1: an empty field that is not greyed takes typing.** Before
    /// the fix an empty field was a read-only dash, so nothing could be typed
    /// before a read and the several-scenes range field was locked.
    #[test]
    fn an_empty_field_that_is_not_greyed_is_never_drawn_read_only() {
        let field = the_pages_function("function theField(field, width, answered) {");
        assert!(!field.contains("readOnly"), "an empty field is drawn read-only");
        assert!(!field.contains("box.value = \"—\""), "the dash is written as the field's value");
        assert!(field.contains("box.placeholder = \"—\""), "the dash is not the empty field's placeholder");
        assert!(field.contains("field.whose === \"the-promise-form\""), "the dash is shown where nothing says it is the file's");
        // And the unread page's own fields are that kind, while the
        // several-scenes range field is live, empty and not the promise form.
        let unread = the_panel(&WhatTheRunHolds::default(), &a_server(), "/k.blend");
        assert!(unread.the_range.value.is_none() && !unread.the_range.greyed);
        assert_eq!(unread.the_range.whose.as_str(), "the-promise-form");
    }

    /// **Must-dos 2 and 3: the band is drawn where no check was ever run, the
    /// switch-off is honoured, and a switch-off nobody could read is said
    /// rather than taken as *not switched off*.**
    #[test]
    fn the_band_says_nobody_has_checked_where_nothing_is_recorded_and_honours_the_switch() {
        let held = a_run_with_a_reading();
        let file = "/ephemeral/smoozy/assets/kitchen_v14.blend";
        let on = WhatTheRecordHolds {
            the_servers_name: Some("the big box".to_string()),
            the_dependency_warning: TheSwitchReads::NotSwitchedOff,
            ..WhatTheRecordHolds::default()
        };
        let band = the_panel_starting_from(&held, &a_server(), file, &on)
            .the_dependency_band
            .expect("no check record is *nobody has ever checked*");
        assert_eq!(band.which.as_str(), "never");
        assert!(band.the_way_starts_a_check);
        assert!(band.the_switch_could_not_be_read.is_none());
        assert_eq!(band.said[0].values[0].as_str(), "the big box");
        assert!(!format!("{:?}", band.said).contains(&a_server()), "the band names the server by its identifier");

        let off = WhatTheRecordHolds {
            the_dependency_warning: TheSwitchReads::SwitchedOff,
            ..on.clone()
        };
        let panel = the_panel_starting_from(&held, &a_server(), file, &off);
        assert!(panel.the_dependency_band.is_none(), "the switch-off was not honoured");
        assert!(!panel.the_dependencies_group_is_drawn);

        let unread = WhatTheRecordHolds {
            the_servers_name: None,
            the_dependency_warning: TheSwitchReads::TheReadFailed,
            ..WhatTheRecordHolds::default()
        };
        let band = the_panel_starting_from(&held, &a_server(), file, &unread)
            .the_dependency_band
            .expect("a read that failed is not *switched off* either");
        assert!(band.the_switch_could_not_be_read.is_some());
        assert!(band.said[0].values.is_empty(), "with no name read, no value stands in for it");

        // The write goes through the shared service, and a refusal is said.
        let working = the_working_half();
        assert!(working.contains("crate::surface::said_once::a_warning_was_switched_off("));
        assert!(working.contains("vec![plainly(words::THE_SETTING_DID_NOT_SAVE)]"));
    }

    /// **Must-do 4: *Set it up* goes to the Set up tab**, folding the panel
    /// down, and never to the dependency check. **The add-on line carries no
    /// link** since the Creator parked add-ons on 2026-10-08 (`Design/Render
    /// Settings`, `§ The one thing this page says about add-ons`): drawn under
    /// Node, it still says the engine's add-on is missing, and holds no control.
    #[test]
    fn set_it_up_goes_to_the_set_up_tab_and_the_add_on_line_carries_no_link() {
        let code = the_page_code();
        assert!(code.contains("aLink(said.set_it_up, () => theSetUpTabIsShown(server), \"set-it-up\")"));
        assert!(!code.contains("install-it"), "the page still draws a link on the add-on line");
        assert!(!code.contains("Install it in Server Setup"), "the page still names the add-on line's link");
        if let Some(got) = the_page_drawn_with_the_mismatch_and_the_add_on_line("set-it-up") {
            assert_eq!(got["addOnControls"], 0, "the add-on line carries a control: {got}");
            // The page keeps a line's last words together with no-break
            // spaces; the words are the same either way.
            let says: Vec<String> = got["addOnSays"]
                .as_array()
                .map(|lines| lines.iter().map(|one| one.as_str().unwrap_or_default().replace('\u{a0}', " ")).collect())
                .unwrap_or_default();
            assert_eq!(
                says,
                vec![format!(
                    "{} 5.1.0 {} {}",
                    words::BLENDER,
                    words::THE_ADD_ON_IS_MISSING_LEAD,
                    words::THE_ENGINE_READ_BACK_AS_EEVEE
                )],
                "the add-on line no longer says the engine's add-on is missing: {got}"
            );
        } else {
            eprintln!("SKIPPED: no Node on this computer, so the add-on line was not drawn");
        }
        let shown = the_pages_function("function theSetUpTabIsShown(server) {");
        assert!(shown.contains("theCurrentTabIs(SET_UP)"));
        assert!(shown.contains(".collapse"), "the panel is not folded down first");
        assert_eq!(
            code.matches("theCheckIsOpened(server)").count()
                - code.matches("function theCheckIsOpened(server)").count(),
            1,
            "a way other than the band's own control reaches the check"
        );
    }

    /// **Must-do 5: the *could not reach* ending names the server by what the
    /// user calls it, and never by its identifier.**
    #[test]
    fn a_read_that_could_not_reach_the_server_names_it_by_its_name() {
        let drawn = the_read_failure(&Ending::TheMachineCouldNotBeReached.into(), Some("the big box"), None)
            .expect("drawn");
        assert_eq!(drawn.values.len(), 1);
        assert_eq!(drawn.values[0].value.as_str(), "the big box");
        let unnamed = the_read_failure(&Ending::TheMachineCouldNotBeReached.into(), None, None).expect("drawn");
        assert!(unnamed.values.is_empty());
    }

    /// **Must-do 6: an EEVEE file draws EEVEE's count and no device pair**, the
    /// engine in force being the file's own where the user has not picked.
    #[test]
    fn an_eevee_file_draws_eevees_count_and_no_device_pair() {
        let mut held = a_run_with_a_reading();
        let file = "/ephemeral/smoozy/assets/kitchen_v14.blend";
        let key = FileKey {
            server: an_identity(&a_server()),
            path: file.to_string(),
        };
        let picked = BlenderIdentity {
            path: "/opt/blender-5.1.0/blender".to_string(),
            version: a_version(5, 1, 0),
        };
        let mut eevee = a_scene("Interior");
        eevee.engine_as_read = ExactName::as_the_reading_produced_it("BLENDER_EEVEE");
        held.readings.a_reading_landed(
            key,
            &smoozy_render_settings::file_half::StartedUnder {
                version_pick: picked.clone(),
                file_identity: four_values(),
            },
            SettingsHalf {
                scenes: vec![eevee],
                offered_engine_identifiers: OfferedEngineIdentifiers {
                    cycles: Some(ExactName::as_the_reading_produced_it("CYCLES")),
                    eevee: Some(ExactName::as_the_reading_produced_it("BLENDER_EEVEE")),
                },
                announced_add_on_case: None,
            },
            Some(&picked),
            &four_values(),
            std::time::SystemTime::now(),
        );
        let panel = the_panel(&held, &a_server(), file);
        assert_eq!(panel.samples.value.as_ref().map(Verbatim::as_str), Some("64"));
        assert!(panel.renders_on.is_none(), "the GPU/CPU pair is drawn under the file's own EEVEE");
    }

    pub(crate) fn the_words_of_a_line(line: &ASentence) -> String {
        line.iter()
            .map(|part| {
                let mut out = part.lead.as_str().to_string();
                for value in &part.values {
                    out.push_str(" <");
                    out.push_str(value.as_str());
                    out.push('>');
                }
                if let Some(rest) = &part.rest {
                    if !part.the_rest_joins {
                        out.push(' ');
                    }
                    out.push_str(rest.as_str());
                }
                out
            })
            .collect::<Vec<_>>()
            .join(" ")
            .replace('\u{a0}', " ")
    }

    /// **Must-do 7: the typed-samples line is one sentence, commas and a final
    /// *and*, ending in a full stop — and with nothing set aside it says no
    /// cost at all.**
    #[test]
    fn the_typed_samples_line_is_one_sentence_and_names_a_cost_only_when_something_is_set_aside() {
        let mut scene = a_scene("Interior");
        let answers = SceneEntries {
            sample_count: Some(512),
            ..SceneEntries::default()
        };
        let lines = the_samples_lines(&answers, Some(&scene), Some(OfferedEngine::Cycles), Some(a_version(5, 1, 0)), false);
        assert_eq!(lines.len(), 1);
        let said = the_words_of_a_line(&lines[0]);
        assert!(said.contains("<0.01>,"), "the threshold part does not end in a comma: {said}");
        let plain = |word: &str| Prose::new(word).as_str().replace('\u{a0}', " ");
        assert!(said.contains(&plain(words::AND_THE_OWN_COUNTS_OF)), "{said}");
        assert!(said.contains("<FX>."), "the list does not end in a full stop: {said}");
        assert!(!said.contains(",  Every") && !said.contains(", Every"), "the cost runs on after a comma: {said}");
        assert!(said.ends_with(&plain(words::EVERY_FRAME_NOW_RENDERS_ALL)), "{said}");

        scene.early_stop.adaptive_sampling_is_on = false;
        scene.time_limit = 30.0;
        scene.view_layers = vec![a_layer("Interior", true, false)];
        let said = the_words_of_a_line(&the_samples_lines(&answers, Some(&scene), Some(OfferedEngine::Cycles), Some(a_version(5, 1, 0)), false)[0]);
        assert!(said.starts_with(&plain(words::OVERRIDES_A_TIME_LIMIT_OF)), "{said}");
        assert!(said.contains("<30s>."), "{said}");

        scene.time_limit = 0.0;
        assert!(
            the_samples_lines(&answers, Some(&scene), Some(OfferedEngine::Cycles), Some(a_version(5, 1, 0)), false).is_empty(),
            "with nothing set aside the typed line still describes a cost"
        );
    }

    /// **Must-do 8: a layer the file keeps off across the range is not named**
    /// as rendering at its own count, and the rule is `Render Settings`' own.
    #[test]
    fn a_layer_that_never_renders_in_the_range_is_not_named() {
        let mut scene = a_scene("Interior");
        scene.view_layers = vec![a_layer("FX", false, true)];
        let lines = the_samples_lines(&SceneEntries::default(), Some(&scene), Some(OfferedEngine::Cycles), Some(a_version(5, 1, 0)), false);
        assert!(
            lines.iter().all(|line| !the_words_of_a_line(line).contains("<FX>")),
            "a layer that renders at no frame of the range is named"
        );
        assert!(the_working_half().contains("view_layers::renders_somewhere_in_the_range("));
    }

    /// **Must-do 9: the Simplify switch is a setting the closed list has, and
    /// every setting the page sends is one it has.**
    #[test]
    fn every_setting_the_page_sends_is_one_the_closed_list_takes() {
        let code = the_page_code();
        let mut sent = Vec::new();
        for (at, _) in code.match_indices("anAnswerIsGiven(server, said, \"") {
            let rest = &code[at + "anAnswerIsGiven(server, said, \"".len()..];
            sent.push(rest[..rest.find('"').expect("closes")].to_string());
        }
        for (at, _) in code.match_indices("aValueIsHandedBack(server, said, \"") {
            let rest = &code[at + "aValueIsHandedBack(server, said, \"".len()..];
            sent.push(rest[..rest.find('"').expect("closes")].to_string());
        }
        assert!(sent.contains(&"simplify".to_string()));
        for name in &sent {
            let parsed: Result<WhichSetting, serde::de::value::Error> = serde::Deserialize::deserialize(
                serde::de::value::StrDeserializer::<serde::de::value::Error>::new(name),
            );
            assert!(parsed.is_ok(), "the page sends {name:?} and the closed list has no such setting");
        }
        let mut held = a_run_with_a_reading();
        let file = "/ephemeral/smoozy/assets/kitchen_v14.blend";
        assert_eq!(the_answer_is_taken(&mut held, &a_server(), file, WhichSetting::Simplify, "on"), None);
        let key = FileKey { server: an_identity(&a_server()), path: file.to_string() };
        let scene = the_scene_in_hand(&held, &a_server(), file);
        assert_eq!(held.store.the_answers_composition_reads(&key, scene.as_ref()).simplify.use_simplify, Some(true));
    }

    /// **Must-do 10: *Render one frame first* sets the check and submits, as
    /// one act.**
    #[test]
    fn render_one_frame_first_sets_the_check_and_submits() {
        let band = the_pages_function("function theDependencyBand(server, said) {");
        assert!(band.contains("aJobIsSubmitted(server, said, true)"));
        assert!(!band.contains("\"the-first-frame-check\", \"on\""), "the band only turns the switch on");
        let command = the_working_half()
            .split("pub fn a_job_is_submitted<")
            .nth(1)
            .and_then(|rest| rest.split("\n}").next())
            .expect("the command");
        assert!(command.contains("with_the_first_frame_check == Some(true)"));
        assert!(command.contains("WhichSetting::TheFirstFrameCheck, \"on\""));
    }

    /// **`§ Submitting` and `§ The dependency warning`: where every job the
    /// Render control would send has one frame, *Render one frame first* is
    /// greyed and cannot be pressed but keeps its setting, and the band leaves
    /// its first-frame way out** — the drawing's `res-oneframe`. The
    /// Creator's nineteenth notes, note 8, 2026-10-27. Nothing about what is
    /// sent changes, so only the page is held here.
    #[test]
    fn the_first_frame_switch_is_greyed_where_every_job_has_one_frame() {
        let record = WhatTheRecordHolds {
            the_dependency_warning: TheSwitchReads::NotSwitchedOff,
            ..WhatTheRecordHolds::default()
        };
        let (mut held, file) = a_run_that_read_and_can_submit(vec![a_scene("Scene")]);
        let page = |held: &WhatTheRunHolds| the_panel_starting_from(held, &a_server(), file, &record);
        let band_offers_it = |panel: &TheRenderSettings| {
            panel
                .the_dependency_band
                .as_ref()
                .expect("the band is drawn")
                .render_one_frame_first
                .is_some()
        };

        // The file's own 240 frames: the switch is live and the band offers it.
        let panel = page(&held);
        assert!(!panel.the_first_frame_switch_is_greyed);
        assert!(band_offers_it(&panel));

        // One frame typed: greyed, still off as it was set, and no band way.
        assert!(the_answer_is_taken(&mut held, &a_server(), file, WhichSetting::FrameRange, "7").is_none());
        let panel = page(&held);
        assert!(panel.the_first_frame_switch_is_greyed, "a one-frame range leaves the switch live");
        assert!(!panel.the_first_frame_is_asked_for);
        assert!(!band_offers_it(&panel), "the band offers a check that has nothing to save");

        // The saved choice is hidden while greyed and restored when the range grows.
        let _ = the_answer_is_taken(&mut held, &a_server(), file, WhichSetting::TheFirstFrameCheck, "on");
        let panel = page(&held);
        assert!(panel.the_first_frame_switch_is_greyed && !panel.the_first_frame_is_asked_for);
        assert!(the_answer_is_taken(&mut held, &a_server(), file, WhichSetting::FrameRange, "1-20").is_none());
        let panel = page(&held);
        assert!(!panel.the_first_frame_switch_is_greyed && panel.the_first_frame_is_asked_for);

        // The file's own range of one frame, nothing typed.
        let mut one = a_scene("Scene");
        one.frame_count = 1;
        let (held, file) = a_run_that_read_and_can_submit(vec![one]);
        let panel = the_panel_starting_from(&held, &a_server(), file, &record);
        assert!(panel.the_first_frame_switch_is_greyed);
        assert!(!band_offers_it(&panel));

        // Several scenes: a typed group range of one frame is one frame in
        // every job; two frames, or counts not known, are not.
        let ticked = [ExactName::as_the_reading_produced_it("A"), ExactName::as_the_reading_produced_it("B")];
        let typed = |form: FrameForm| SceneEntries {
            frame_range: Some(form),
            ..SceneEntries::default()
        };
        assert!(every_job_has_one_frame(&typed(FrameForm::List(vec![7])), None, &ticked, None, true));
        assert!(!every_job_has_one_frame(&typed(FrameForm::List(vec![7, 8])), None, &ticked, None, true));
        assert!(!every_job_has_one_frame(&SceneEntries::default(), None, &ticked, None, true));

        // **The page draws it**: the foot hands the greyed answer to the
        // switch, and a greyed switch is a disabled button.
        let foot = the_pages_function("function theFoot(");
        assert!(foot.contains("said.the_first_frame_switch_is_greyed"), "the foot does not grey the switch");
        let switch = the_pages_function("function aSwitch(label, on, pressed, which, greyed) {");
        assert!(switch.contains("if (greyed) control.disabled = true;"));
    }

    #[test]
    fn off_is_per_job_and_the_first_frame_note_follows_the_live_switch() {
        let file = "/ephemeral/smoozy/assets/kitchen_v14.blend";
        let mut held = a_run_with_a_reading();
        let record = WhatTheRecordHolds { telegram_is_on: true, ..WhatTheRecordHolds::default() };
        let row = a_row_on(&a_server(), true);
        let page = |held: &WhatTheRunHolds| the_panel_starting_from(held, &a_server(), file, &record);
        assert_eq!(REPORTING_CHOICES.last(), Some(&("nothing", "Off")));
        // **From 2026-10-31 Off may be the default** (`Telegram` V1d).
        assert_eq!(the_reporting_choice_composed(None, Some("nothing")), "nothing");
        assert!(the_answer_is_taken(&mut held, &a_server(), file, WhichSetting::WhatItReports, "nothing").is_none());
        assert!(page(&held).the_first_frame_is_answered_in_queue.is_none());
        assert!(the_answer_is_taken(&mut held, &a_server(), file, WhichSetting::TheFirstFrameCheck, "on").is_none());
        let panel = page(&held);
        assert_eq!(panel.the_reporting_control_reads.as_str(), "Off");
        assert_eq!(panel.the_first_frame_is_answered_in_queue.unwrap().as_str().replace('\u{a0}', " "), "Answer the first frame in Queue.");
        let given = the_act_as_the_page_composed_it(&held, &a_server(), file, &record, &row, a_run()).unwrap();
        assert_eq!(given.reporting.0, serde_json::json!("Nothing"));
        assert!(the_answer_is_taken(&mut held, &a_server(), file, WhichSetting::FrameRange, "7").is_none());
        assert!(!page(&held).the_first_frame_is_asked_for);
        assert!(page(&held).the_first_frame_is_answered_in_queue.is_none());
        assert!(the_answer_is_taken(&mut held, &a_server(), file, WhichSetting::FrameRange, "1-20").is_none());
        assert!(page(&held).the_first_frame_is_answered_in_queue.is_some());
        assert!(the_answer_is_taken(&mut held, &a_server(), file, WhichSetting::WhatItReports, "everything").is_none());
        assert!(page(&held).the_first_frame_is_answered_in_queue.is_none());
        // An off choice stays off after a single-frame range grows too.
        assert!(the_answer_is_taken(&mut held, &a_server(), file, WhichSetting::TheFirstFrameCheck, "off").is_none());
        for range in ["7", "1-20"] {
            assert!(the_answer_is_taken(&mut held, &a_server(), file, WhichSetting::FrameRange, range).is_none());
            assert!(!page(&held).the_first_frame_is_asked_for);
        }
        let foot = the_pages_function("function theFoot(");
        assert!(foot.contains("note.textContent = said.the_first_frame_is_answered_in_queue;"));
    }

    /// **Must-do 11: *Open the check* starts nothing; *Check dependencies…*
    /// starts one.**
    #[test]
    fn open_the_check_starts_nothing() {
        let band = the_pages_function("function theDependencyBand(server, said) {");
        assert!(band.contains("band.the_way_starts_a_check ? theCheckIsOpened(server) : theCheckIsShown(server)"));
        let check = include_str!("../../web/the-dependency-check.js");
        let shown = check
            .split("export async function theCheckIsShownOn(server, file) {")
            .nth(1)
            .and_then(|rest| rest.split("\n}").next())
            .expect("the second way in");
        assert!(!shown.contains("the_check_is_started"), "the second way in starts a check");
    }

    /// **Must-do 12: with no Blender, the read and *Reset all* are still
    /// drawn** — the read reading unavailable.
    #[test]
    fn with_no_blender_the_read_and_reset_all_stay() {
        let header = the_pages_function("function theHeaderIsDrawn(server, said, head) {");
        let after_the_branch = header
            .split("if (said.no_blender.length) {")
            .nth(1)
            .expect("the branch");
        let read_at = after_the_branch.find("\"read\"").expect("the read is drawn");
        let branch_ends = after_the_branch.find("} else if (said.read_at.length)").expect("the branch ends");
        assert!(read_at > branch_ends, "the read is drawn inside one branch only");
        assert!(after_the_branch.contains("if (!said.the_read_is_offered) read.disabled = true;"));
        assert!(after_the_branch.contains("\"reset-all\""));
        let mut held = a_run_with_a_reading();
        held.version_lists.get_mut(&a_server()).expect("a list").entries.clear();
        let panel = the_panel(&held, &a_server(), "/ephemeral/smoozy/assets/kitchen_v14.blend");
        assert!(!panel.no_blender.is_empty());
        assert!(!panel.the_read_is_offered, "the read reads unavailable with no Blender");
    }

    fn a_collider(origin: WhereItComesFrom, destination: usize) -> smoozy_job_files::inventory::AColliderInTheRecord {
        smoozy_job_files::inventory::AColliderInTheRecord { destination, origin }
    }

    /// **Must-dos 13 and 14: every colliding pair is named, and a File Output
    /// node's name is a value.**
    #[test]
    fn the_collision_warning_names_every_pair_and_a_nodes_name_is_a_value() {
        use smoozy_job_files::reading::ArtistsText;
        let pairs = vec![
            smoozy_job_files::inventory::ACollidingPair {
                predicted_path: b"/out/beauty_0001.exr".to_vec(),
                members: vec![
                    a_collider(WhereItComesFrom::TheScenesOutputPath, 0),
                    a_collider(WhereItComesFrom::AFileOutputNodeNamed(ArtistsText::of("Beauty -- x")), 1),
                ],
            },
            smoozy_job_files::inventory::ACollidingPair {
                predicted_path: b"/out/depth_0001.exr".to_vec(),
                members: vec![
                    a_collider(WhereItComesFrom::AFileOutputNodeNamed(ArtistsText::of("Depth")), 2),
                    a_collider(WhereItComesFrom::AFileOutputNodeNamed(ArtistsText::of("Depth.001")), 3),
                ],
            },
        ];
        let warning = the_collision_warning(&pairs).expect("a warning");
        assert_eq!(warning.pairs.len(), 2, "only the first colliding pair is named");
        let node = &warning.pairs[0][1];
        let named = node
            .iter()
            .find(|part| part.lead.as_str().replace('\u{a0}', " ").contains("File Output node"))
            .expect("the node part");
        assert_eq!(named.values[0].as_str(), "Beauty -- x");
        assert!(!named.lead.as_str().contains("Beauty"), "the node's name is joined into the words");
        assert!(the_collision_warning(&[]).is_none());
    }

    /// **Must-do 15: the two headings are drawn only where something is under
    /// them.**
    #[test]
    fn a_heading_with_nothing_under_it_is_not_drawn() {
        let unread = the_panel(&WhatTheRunHolds::default(), &a_server(), "/k.blend");
        assert!(!unread.the_output_group_is_drawn);
        let right = the_pages_function("function theRightColumn(server, said) {");
        assert!(right.contains("if (said.the_output_group_is_drawn)"));
        assert!(right.contains("if (said.the_dependencies_group_is_drawn)"));
    }

    /// **The 38's piece 10, item 38: a scene that writes more than one view
    /// separately says the one line under *What it will write*** — the line
    /// its queue entry carries — and one view says nothing of the kind.
    #[test]
    fn a_two_view_scene_says_its_frames_cannot_be_counted() {
        use smoozy_job_files::reading::HowManyViewsTheSceneWrites;
        let lines_for = |views| {
            let outputs = crate::servers::the_machines_work::tests::a_recorded_reading(
                "/ephemeral/smoozy/jobs/job",
                &[1, 2],
                views,
            )
            .reading
            .outputs;
            the_output_group(Some(&outputs), None, &[], false, &TheGroundOfTheCount::default()).2
        };
        // **As it travels**: the typography point has curled its apostrophe.
        let the_line = vec![plainly(words::MORE_THAN_ONE_VIEW)];
        assert!(lines_for(HowManyViewsTheSceneWrites::MoreThanOneWrittenSeparately { how_many: 2 })
            .contains(&the_line));
        assert!(!lines_for(HowManyViewsTheSceneWrites::One).contains(&the_line));
    }

    /// **The Creator's eighth notes, note 4, and their answer of 2026-10-12:
    /// a video is not a pass at all**, so frames from a File Output node and a
    /// video from the output path are named as two outputs and draw no twice
    /// line. **And the twice line names the places the walk found** — the
    /// runner's ruling that day, one shape each.
    #[test]
    fn frames_and_a_video_are_named_and_the_twice_line_names_its_places() {
        use smoozy_job_files::main_output::ThePlacesTheBeautyIsWritten;
        let outputs = crate::servers::the_machines_work::tests::a_recorded_reading(
            "/ephemeral/smoozy/jobs/job",
            &[1, 2],
            smoozy_job_files::reading::HowManyViewsTheSceneWrites::One,
        )
        .reading
        .outputs;
        let ground = TheGroundOfTheCount::default();
        let AsTheJobRendersIt { outputs, map, inventory, .. } = the_inventory_of(&outputs, &ground);
        let mut six = the_six_things(&outputs, &map, &inventory, ground.the_folder_composed_under());
        let lines_of = |six: &smoozy_job_files::holds::TheSixThings| -> Vec<String> {
            the_rest_of_the_output_group(None, Vec::new(), six, None, &[])
                .2
                .iter()
                .map(the_words_of_a_line)
                .collect()
        };

        // **A File Output node and a video at the output path.**
        six.the_same_picture_reaches_more_than_one_place = false;
        six.where_the_same_picture_is_written = ThePlacesTheBeautyIsWritten::default();
        six.the_render_reaches_frames_and_a_video = true;
        six.its_output_is_a_single_video_file = false;
        let lines = lines_of(&six);
        assert!(lines.iter().any(|line| line == words::FRAMES_AND_A_VIDEO), "{lines:?}");
        assert!(!lines.iter().any(|line| line.starts_with(words::THE_BEAUTY_WRITTEN)), "{lines:?}");

        // **The twice line, one shape each.**
        six.the_render_reaches_frames_and_a_video = false;
        six.the_same_picture_reaches_more_than_one_place = true;
        for (nodes, inputs, the_output_path, said) in [
            (1, 1, true, "Beauty pass written twice — a File Output node and the output path."),
            (2, 2, false, "Beauty pass written twice — two File Output nodes."),
            (1, 2, false, "Beauty pass written twice — two inputs of a File Output node."),
            (2, 2, true, "Beauty pass written three times — two File Output nodes and the output path."),
            (2, 3, true, "Beauty pass written four times — three inputs of two File Output nodes and the output path."),
            (12, 12, false, "Beauty pass written <12> times — <12> File Output nodes."),
        ] {
            six.where_the_same_picture_is_written =
                ThePlacesTheBeautyIsWritten { file_output_nodes: nodes, their_inputs: inputs, the_output_path };
            let lines = lines_of(&six);
            let twice: Vec<String> = lines
                .iter()
                .filter(|line| line.starts_with(words::THE_BEAUTY_WRITTEN))
                .map(|line| line.split_whitespace().collect::<Vec<_>>().join(" "))
                .collect();
            assert_eq!(twice, vec![said.to_string()], "{lines:?}");
            assert!(!lines.iter().any(|line| line == words::FRAMES_AND_A_VIDEO));
        }
        // **One place is no line.**
        six.where_the_same_picture_is_written =
            ThePlacesTheBeautyIsWritten { file_output_nodes: 1, their_inputs: 1, the_output_path: false };
        assert!(the_beauty_written_more_than_once(&six.where_the_same_picture_is_written).is_empty());
    }

    /// **Must-do 16: with several scenes ticked the device pair stays, greyed
    /// and with nothing on.**
    #[test]
    fn several_scenes_ticked_keeps_the_device_pair_greyed_and_empty() {
        let mut held = a_run_with_a_reading();
        let file = "/ephemeral/smoozy/assets/kitchen_v14.blend";
        for scene in ["Interior", "Exterior"] {
            let key = FileKey { server: an_identity(&a_server()), path: file.to_string() };
            let provenance = the_provenance(&held, &key).expect("a reading stands");
            held.store.tick_a_scene(
                key,
                NamedAnswer { name: ExactName::as_the_reading_produced_it(scene), provenance },
            );
        }
        let device = the_panel(&held, &a_server(), file).renders_on.expect("the pair stays");
        assert!(device.greyed);
        assert_eq!(device.on.as_str(), "nothing");
        assert!(device.hand_back_to.is_none() && device.tag.is_empty());
    }

    /// **Must-do 17: the title reads the way the drawing's does, built the way
    /// the check's is.**
    #[test]
    fn the_title_is_the_words_then_the_files_name() {
        let panel = the_panel(&WhatTheRunHolds::default(), &a_server(), "/a/kitchen_v14.blend");
        assert_eq!(panel.title.lead.as_str(), Prose::new(words::RENDER_SETTINGS_TITLE).as_str());
        assert_eq!(panel.title.values[0].as_str(), "kitchen_v14.blend");
        assert!(the_pages_function("function theHeaderIsDrawn(server, said, head) {").contains("said.title.lead"));
    }

    /// **Must-do 18: every control carries a `data-box`, so focus comes back.**
    #[test]
    fn every_control_the_page_draws_carries_its_own_box() {
        let code = the_page_code();
        for opening in [
            "function aBox(name, ticked, extra, pressed, which) {",
            "function aSwitch(label, on, pressed, which, greyed) {",
            "function aLink(label, pressed, which) {",
            "function theCross(handBackTo, pressed, which) {",
        ] {
            let body = the_pages_function(opening);
            assert!(body.contains(".dataset.box = which"), "{opening} gives its control no box");
        }
        for named in ["`scene-${scene.name}`", "\"all-scenes\"", "`layer-${layer.name}`", "\"download-switch\"", "\"first-frame-switch\"", "\"simplify\""] {
            assert!(code.contains(named), "the page gives no box named {named}");
        }
    }

    /// **Must-do 19 and the upstream must-do, as step 10c's dependency wave
    /// piece 3 left them: pressing *Read the file* drives the read and no longer
    /// answers *"Smoozy cannot do this yet"*; a real ending lands through
    /// [`the_read_gave_nothing`] alone and is handed to the diary once per
    /// request.** *(Until that piece the press answered the shared not-built
    /// sentence and recorded nothing, because no read was attempted.)*
    #[test]
    fn a_press_drives_the_read_and_a_real_ending_reaches_the_diary_once_per_request() {
        let command = the_working_half()
            .split("pub fn the_file_is_read<")
            .nth(1)
            .and_then(|rest| rest.split("\n}").next())
            .expect("the command");
        assert!(command.contains("crate::reading_the_file::the_file_is_read("), "the press drives no read");
        assert!(!command.contains("A_CONNECTION_IS_NOT_BUILT"), "the press says the read is not built");
        assert!(!command.contains("the_read_that_gave_nothing"), "an ending is recorded beside the one place it lands");
        assert!(!the_working_half().contains("crate::surface::files::A_CONNECTION_IS_NOT_BUILT;"));
        assert!(!the_working_half().contains("nothing is connected to the server"));

        let mut area = std::env::temp_dir();
        area.push(format!("smoozy-read-diary-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&area);
        let diary = crate::diary_service::DiaryService::open_at(area.clone()).expect("a diary");
        let run = Identity::issue(IdentityKind::Run).expect("randomness");
        let the_diary = TheDiaryInHand { diary: Some(&diary), run: Some(&run), the_servers_name: "the big box" };
        let mut held = WhatTheRunHolds::default();
        let file = "/ephemeral/smoozy/assets/kitchen_v14.blend";
        for _ in 0..2 {
            let _ = the_read_gave_nothing(&mut held, &a_server(), file, Ending::TheReadRanAndSomethingInItFailed, &the_diary);
        }
        // Redrawing the panel hands nothing over.
        let _ = the_panel(&held, &a_server(), file);
        let _ = the_panel(&held, &a_server(), file);
        let mut kept = String::new();
        for entry in std::fs::read_dir(&area).expect("the area").flatten() {
            kept.push_str(&std::fs::read_to_string(entry.path()).unwrap_or_default());
        }
        assert_eq!(kept.matches(words::THE_READ_DID_NOT_FINISH).count(), 2, "not once per request");
        assert!(kept.contains("the big box") && kept.contains("kitchen_v14.blend"));
        assert!(!kept.contains(crate::surface::files::A_CONNECTION_IS_NOT_BUILT));
        let drawn = the_panel(&held, &a_server(), file).the_read_failure.expect("rf-part stays drawable");
        assert_eq!(drawn.which.as_str(), "the-read");
        let _ = std::fs::remove_dir_all(&area);
    }

    /// **Must-do 22: every act hands the file's exact bytes back.**
    #[test]
    fn every_act_hands_back_the_files_bytes() {
        let code = the_page_code();
        assert!(!code.contains("file: said.file,") && !code.contains("file: said.file }"), "an act hands back the display text");
        assert!(code.contains("file: said.file_bytes"));
        let working = the_working_half();
        let mut commands = 0;
        for (at, _) in working.match_indices("#[tauri::command]") {
            let signature = &working[at..at + working[at..].find(") ").unwrap_or(0)];
            if signature.contains("file:") {
                commands += 1;
                assert!(signature.contains("file: Vec<u8>"), "a command takes the file as text: {signature}");
            }
        }
        assert!(commands >= 13);
    }

    // ════════════════════════════════════════════════════════════════════════
    // `What Build inherits` item 8(a): the two controls, alive since 2026-09-20
    // ════════════════════════════════════════════════════════════════════════

    /// **The samples box is alive.** `Render Settings` §13 item 29 left five
    /// property spellings with no read source until 2026-09-20, and until then
    /// every attempt of every typed-sample-count job failed at `Rendering`
    /// L8a's named failure status with nothing written. **The lookup of that
    /// date read all five off the four running Blenders** (C10), so C1's
    /// imposition set composes: the count pair plus the four displacer pairs,
    /// every one written out from the job's scene.
    ///
    /// This replaces the test that held the control dead, which was written to
    /// fail on the day the lookup landed.
    #[test]
    fn the_samples_box_composes_now_that_the_five_spellings_are_read() {
        let set = samples::the_imposition_set(OfferedEngine::Cycles, 512)
            .expect("C10 carries every name C1 items 1 to 5 assign against");
        let names: Vec<&str> = set.iter().map(|pair| pair.property).collect();
        assert_eq!(
            names,
            vec![
                "scene.cycles.samples",
                "scene.cycles.use_adaptive_sampling",
                "scene.cycles.time_limit",
                "scene.cycles.use_layer_samples",
                "scene.cycles.use_sample_subset",
            ],
            "C1's five pairs, in the clause's own order, against the job's scene"
        );
        // **C8: every pair is assigned against the job's scene and none walks
        // the file's scene list.**
        assert!(names.iter().all(|name| name.starts_with("scene.")));
        // **The near miss is not what any of them assigns against.**
        assert!(!names.iter().any(|name| name.contains("compositor")));
    }

    /// **The processor override is alive.** D3's pair is `scene.cycles.device`
    /// assigned to `CPU`, both read on 2026-09-20 (C10). **The sharpest near
    /// miss sits on the same scene and takes the same two values** —
    /// `scene.render.compositor_device` — so the name is checked exactly.
    #[test]
    fn the_processor_override_composes_and_is_not_the_compositors_device() {
        let pair = device::the_processor_pair(Some(DeviceChoice::TheProcessor), &a_scene("Interior"))
            .expect("C10 carries D3's spelling")
            .expect("a processor override against a scene set to the cards is a pair");
        assert_eq!(pair.property, "scene.cycles.device");
        assert_ne!(pair.property, "scene.render.compositor_device");
    }

    /// **Nothing in the spellings file is an absence any more, and the shape
    /// that held one is still there.** A spelling later found to differ where
    /// Smoozy renders comes back as an absence rather than as a guess, because
    /// **a guessed property name is handed in, reads back correctly and
    /// changes nothing** — the silent-wrong-render class this whole step
    /// exists to avoid. `Rendering` L10a's name check stands behind every one
    /// of them and is not removed.
    #[test]
    fn nothing_is_still_owed_and_the_absence_shape_remains() {
        assert!(THE_FIVE_NOT_YET_READ.is_empty());
        for spelling in THE_FIVE_NOT_YET_READ {
            assert!(matches!(spelling, Spelling::NotYetRead(_)));
        }
    }

    /// **fix-here item 9: a job's reporting choice starts from the stored default**, a
    /// per-job answer wins over it, and the first of the three is used only
    /// where nothing is stored.
    #[test]
    fn a_jobs_reporting_choice_starts_from_the_stored_default() {
        assert_eq!(the_reporting_choice_composed(None, None), "everything");
        assert_eq!(the_reporting_choice_composed(None, Some("only-what-needs-you")), "only-what-needs-you");
        assert_eq!(the_reporting_choice_composed(Some("pictures-only"), Some("only-what-needs-you")), "pictures-only");
        assert_eq!(the_reporting_choice_composed(None, Some("not one of the three")), "everything");
        let mut held = a_run_with_a_reading();
        let file = "/ephemeral/smoozy/assets/kitchen_v14.blend";
        let stored = WhatTheRecordHolds {
            the_reporting_default: Some("pictures-only".to_string()),
            ..WhatTheRecordHolds::default()
        };
        let panel = the_panel_starting_from(&held, &a_server(), file, &stored);
        assert_eq!(panel.the_reporting_choice.as_str(), "pictures-only");
        // A job name typed first is not a reporting answer.
        let _ = the_answer_is_taken(&mut held, &a_server(), file, WhichSetting::JobName, "kitchen");
        let panel = the_panel_starting_from(&held, &a_server(), file, &stored);
        assert_eq!(panel.the_reporting_choice.as_str(), "pictures-only");
        let _ = the_answer_is_taken(&mut held, &a_server(), file, WhichSetting::WhatItReports, "everything");
        let panel = the_panel_starting_from(&held, &a_server(), file, &stored);
        assert_eq!(panel.the_reporting_choice.as_str(), "everything");
    }

    /// **`Design/Render Settings § Submitting`, *A fourth choice, per job
    /// only* (2026-10-19, the Creator's want: *"a choice that the user can
    /// manually type in the number of minutes"*).** The list offers it third;
    /// picked, the control reads *A picture every* and the minutes field is
    /// drawn with 5 in it; typed minutes are taken and the job carries them in
    /// `Telegram` V1a's form; minutes Smoozy will not take are refused with
    /// the range alone, the minutes that stood are what the job carries, and
    /// Render still composes. From 2026-10-31 Settings' stored default may be
    /// the fourth, with its own minutes.
    #[test]
    fn a_picture_every_few_minutes_is_offered_third_and_the_job_carries_the_minutes_that_stood() {
        let file = "/ephemeral/smoozy/assets/kitchen_v14.blend";
        let mut held = a_run_with_a_reading();
        let record = WhatTheRecordHolds { telegram_is_on: true, ..WhatTheRecordHolds::default() };
        let row = a_row_on(&a_server(), true);
        let the_jobs_field = |held: &WhatTheRunHolds| {
            the_act_as_the_page_composed_it(held, &a_server(), file, &record, &row, a_run())
                .unwrap_or_else(|said| panic!("Render did not compose: {said:?}"))
                .reporting
                .0
        };
        // **The words as read**: the typography point binds each last word to
        // the one before it (`Design/Whole app § Typography`).
        let read = |said: &str| said.replace('\u{a0}', " ");
        let panel = the_panel_starting_from(&held, &a_server(), file, &record);
        let rows: Vec<(String, String)> = panel
            .the_reporting_choices
            .iter()
            .map(|one| (one.value.as_str().to_string(), read(one.said.as_str())))
            .collect();
        let rows: Vec<(&str, &str)> = rows.iter().map(|(value, said)| (value.as_str(), said.as_str())).collect();
        assert_eq!(
            rows,
            [
                ("everything", "Everything"),
                ("pictures-only", "Pictures only"),
                ("a-picture-every-few-minutes", "A picture every few minutes"),
                ("only-what-needs-you", "Alerts only"),
                ("nothing", "Off"),
            ],
            "the new row, third"
        );
        assert_eq!(read(panel.the_reporting_control_reads.as_str()), "Everything");
        assert!(panel.the_minutes_between_pictures.is_none(), "the field is there only for the fourth");

        assert_eq!(
            what_the_answer_says(&mut held, &a_server(), file, WhichSetting::WhatItReports, A_PICTURE_EVERY_FEW_MINUTES),
            Vec::new()
        );
        let panel = the_panel_starting_from(&held, &a_server(), file, &record);
        assert_eq!(panel.the_reporting_choice.as_str(), A_PICTURE_EVERY_FEW_MINUTES);
        assert_eq!(read(panel.the_reporting_control_reads.as_str()), "A picture every");
        assert_eq!(panel.the_minutes_between_pictures.as_ref().map(Verbatim::as_str), Some("5"), "5 to begin with");
        assert_eq!(
            (read(panel.minutes.as_str()), read(panel.minutes_between_pictures.as_str())),
            ("minutes".to_string(), "Minutes between pictures".to_string())
        );
        assert_eq!(the_jobs_field(&held), serde_json::json!({"APictureEveryFewMinutes": {"minutes": 5}}));

        assert_eq!(what_the_answer_says(&mut held, &a_server(), file, WhichSetting::TelegramMinutes, "12"), Vec::new());
        let panel = the_panel_starting_from(&held, &a_server(), file, &record);
        assert_eq!(panel.the_minutes_between_pictures.as_ref().map(Verbatim::as_str), Some("12"));
        assert_eq!(the_jobs_field(&held), serde_json::json!({"APictureEveryFewMinutes": {"minutes": 12}}));

        for typed in ["0", "61", "", "2.5", "five", "-3"] {
            let said = what_the_answer_says(&mut held, &a_server(), file, WhichSetting::TelegramMinutes, typed);
            assert_eq!(read(&the_words_of_a_line(&said)), "Whole minutes, 1 to 60.", "{typed:?} is refused with the range");
            let panel = the_panel_starting_from(&held, &a_server(), file, &record);
            assert_eq!(panel.the_minutes_between_pictures.as_ref().map(Verbatim::as_str), Some("12"), "{typed:?}: the minutes that stood");
            assert_eq!(the_jobs_field(&held), serde_json::json!({"APictureEveryFewMinutes": {"minutes": 12}}), "{typed:?}");
        }
        assert_eq!(the_box_of(WhichSetting::TelegramMinutes), "telegram-minutes");

        // **Another row picked and the fourth again**: the minutes typed are
        // still there, and another row carries no minutes.
        let _ = what_the_answer_says(&mut held, &a_server(), file, WhichSetting::WhatItReports, "pictures-only");
        let panel = the_panel_starting_from(&held, &a_server(), file, &record);
        assert!(panel.the_minutes_between_pictures.is_none());
        assert_eq!(read(panel.the_reporting_control_reads.as_str()), "Pictures only");
        assert_eq!(the_jobs_field(&held), serde_json::json!("ThePicturesWithoutTheRunningCommentary"));
        let _ = what_the_answer_says(&mut held, &a_server(), file, WhichSetting::WhatItReports, A_PICTURE_EVERY_FEW_MINUTES);
        assert_eq!(the_jobs_field(&held), serde_json::json!({"APictureEveryFewMinutes": {"minutes": 12}}));

        // **From 2026-10-31 the default may be the fourth, with its minutes**
        // (`Telegram` V1a as amended, `Telegram` V1d): a job with no answer of
        // its own offers it with the default's minutes, and minutes typed for
        // the job win. *(Until that day Settings kept three, and a stored
        // fourth was no default.)*
        assert_eq!(
            the_reporting_choice_composed(None, Some(A_PICTURE_EVERY_FEW_MINUTES)),
            A_PICTURE_EVERY_FEW_MINUTES
        );
        let mut fresh = a_run_with_a_reading();
        let stored = WhatTheRecordHolds {
            telegram_is_on: true,
            the_reporting_default: Some(A_PICTURE_EVERY_FEW_MINUTES.to_string()),
            the_default_minutes: Some(10),
            ..WhatTheRecordHolds::default()
        };
        let panel = the_panel_starting_from(&fresh, &a_server(), file, &stored);
        assert_eq!(panel.the_reporting_choice.as_str(), A_PICTURE_EVERY_FEW_MINUTES);
        assert_eq!(panel.the_minutes_between_pictures.as_ref().map(Verbatim::as_str), Some("10"), "the default's minutes");
        let given = the_act_as_the_page_composed_it(&fresh, &a_server(), file, &stored, &row, a_run()).expect("composed");
        assert_eq!(given.reporting.0, serde_json::json!({"APictureEveryFewMinutes": {"minutes": 10}}));
        assert_eq!(what_the_answer_says(&mut fresh, &a_server(), file, WhichSetting::TelegramMinutes, "7"), Vec::new());
        let given = the_act_as_the_page_composed_it(&fresh, &a_server(), file, &stored, &row, a_run()).expect("composed");
        assert_eq!(given.reporting.0, serde_json::json!({"APictureEveryFewMinutes": {"minutes": 7}}), "typed for the job");
        // **Off, and every other of the five, is carried whole** by a job with
        // no answer of its own.
        for (id, form) in [
            ("nothing", serde_json::json!("Nothing")),
            ("only-what-needs-you", serde_json::json!("OnlyWhatNeedsAnAnswer")),
        ] {
            let stored = WhatTheRecordHolds {
                telegram_is_on: true,
                the_reporting_default: Some(id.to_string()),
                ..WhatTheRecordHolds::default()
            };
            let given = the_act_as_the_page_composed_it(&a_run_with_a_reading(), &a_server(), file, &stored, &row, a_run())
                .expect("composed");
            assert_eq!(given.reporting.0, form, "{id}");
        }
    }

    /// **The page draws the fourth choice as the drawing does** —
    /// `res-tglist`, `res-tgmin`, `res-tgbad` — under Node: the list's five
    /// rows, the new one third; picked, the control reads *A picture every*,
    /// keeps the drawing's class, and the field (the drawing's
    /// `.fld.w-frame.mono`, in no `.set`, 5 in it) and *minutes* follow it in
    /// the Telegram line; a typed entry goes to the application as it is; the
    /// refusal turns the field red with what was typed still in it and puts
    /// the range on the foot's third line; the next redraw shows the minutes
    /// that stood; with another choice there is no field and no line; and at
    /// the window's foot the list opens upward. Skipped, and says so, where no
    /// Node is installed.
    #[test]
    fn the_page_draws_the_minutes_field_its_refusal_and_the_upward_list() {
        let file = "/ephemeral/smoozy/assets/kitchen_v14.blend";
        let mut held = a_run_with_a_reading();
        let record = WhatTheRecordHolds { telegram_is_on: true, ..WhatTheRecordHolds::default() };
        let plain = the_panel_starting_from(&held, &a_server(), file, &record);
        let _ = what_the_answer_says(&mut held, &a_server(), file, WhichSetting::WhatItReports, A_PICTURE_EVERY_FEW_MINUTES);
        let minutes = the_panel_starting_from(&held, &a_server(), file, &record);
        let refused = serde_json::json!({ "theBox": "telegram-minutes", "partCount": "1", "lead0": words::WHOLE_MINUTES_1_TO_60 });
        let answers = serde_json::json!({ "plain": plain, "minutes": minutes, "refused": refused });
        let folder = std::env::temp_dir().join(format!("smoozy-minutes-page-{}", std::process::id()));
        let _ = fs::remove_dir_all(&folder);
        fs::create_dir_all(&folder).expect("a scratch folder");
        let web = Path::new(env!("CARGO_MANIFEST_DIR")).join("web");
        for (name, text) in THE_UPLOADING_LINES_STAND_INS {
            fs::write(folder.join(name), text).expect("a stand-in");
        }
        fs::copy(web.join("render-settings.js"), folder.join("render-settings.js")).expect("the page");
        fs::write(folder.join("the-click.mjs"), THE_CLICK_STAND_IN).expect("the click");
        fs::write(folder.join("answers.json"), answers.to_string()).expect("the answers");
        fs::write(folder.join("harness.mjs"), THE_MINUTES_HARNESS.replace("SERVER", &a_server())).expect("the harness");
        let ran = std::process::Command::new("node").arg(folder.join("harness.mjs")).current_dir(&folder).output();
        let _ = fs::remove_dir_all(&folder);
        let Ok(ran) = ran else {
            eprintln!("SKIPPED: no Node on this computer, so the render settings page was not run");
            return;
        };
        assert!(ran.status.success(), "the page code did not run: {}", String::from_utf8_lossy(&ran.stderr));
        let got: serde_json::Value = serde_json::from_slice(&ran.stdout).expect("the harness prints one answer");
        assert_eq!(
            got,
            serde_json::json!({
                "plain": {
                    "rows": ["Everything", "Pictures only", "A picture every few minutes", "Alerts only", "Off"],
                    "closed": "Everything", "pick": "pick w-tg", "field": false, "after": false, "line": false
                },
                "picked": [["what-it-reports", "a-picture-every-few-minutes"]],
                "minutes": {
                    "closed": "A picture every", "pick": "pick w-tg",
                    "line": ["hintl:Telegram", "pick w-tg", "INPUT:fld mono w-frame:5", "hintl:minutes"],
                    "label": "Minutes between pictures", "inASet": false, "bad": false,
                    "answerLine": { "hidden": true, "inTheThirdLine": true }
                },
                "typed": [["telegram-minutes", "0"]],
                "refused": {
                    "bad": true, "value": "0",
                    "answerLine": { "hidden": false, "inTheThirdLine": true, "className": "alertnote", "said": "Whole minutes, 1 to 60." }
                },
                "redrawn": { "bad": false, "value": "5", "answerLine": { "hidden": true, "inTheThirdLine": true } },
                "upward": { "open": true, "up": true, "rows": 5 }
            })
        );
    }

    /// Draws the page with *Everything*, picks the fourth, draws it again
    /// picked, types 0, hands it the refusal, draws again, and opens the list
    /// at the window's foot.
    const THE_MINUTES_HARNESS: &str = r#"import { everyPiece } from "./document.mjs";
import { fire } from "./the-click.mjs";
import { readFileSync } from "node:fs";
const answers = JSON.parse(readFileSync("./answers.json", "utf8"));
const sent = [];
let current = "plain";
globalThis.window = { innerHeight: 800, addEventListener() {}, __TAURI__: { core: { invoke(command, asked) {
  if (command === "the_render_settings_panel") return Promise.resolve(answers[current]);
  if (command === "an_answer_was_given") sent.push([asked.which, asked.text]);
  return Promise.resolve(null);
} } } };
const { panels } = await import("./a-server.js");
const { theSettingsAreOpenedOn } = await import("./render-settings.js");
const settle = async () => { for (let at = 0; at < 8; at += 1) await new Promise((go) => setImmediate(go)); };
await theSettingsAreOpenedOn("SERVER", Array.from(Buffer.from(answers.plain.file)), "");
const head = document.createElement("div");
const body = document.createElement("div");
panels[0].itIsDrawn({ head, body }, "SERVER");
await settle();
const told = async (said) => {
  for (const handler of globalThis.told["render-settings:an-act"] || []) await handler({ said: { server: "SERVER", file: answers.plain.file, partCount: "0", ...said } });
  await settle();
};
const all = () => [...everyPiece(head), ...everyPiece(body)];
const box = (name) => all().find((one) => one.dataset.box === name);
const listOf = (face) => face.parentNode.children.find((one) => one.names && one.names.has("ddl"));
const sorted = (piece) => [...piece.names].sort().join(" ");
// The words as read: the typography point binds each last word to the one before.
const read = (text) => String(text).replace(/\u00a0/g, " ");
const answerLine = () => {
  const line = box("telegram-minutes-answer");
  if (!line) return false;
  const said = { hidden: line.hidden, inTheThirdLine: Boolean(line.parentNode && line.parentNode.names.has("sfl3")) };
  if (!line.hidden) { said.className = line.className; said.said = read(line.textContent); }
  return said;
};
const out = {};
let face = box("reports");
out.plain = {
  rows: listOf(face).children.map((one) => read(one.textContent)), closed: read(face.textContent), pick: face.parentNode.className,
  field: Boolean(box("telegram-minutes-field")), after: all().some((one) => one.names && one.names.has("hintl") && one.textContent === answers.minutes.minutes),
  line: answerLine(),
};
fire(face, "click");
fire(listOf(face).children[2], "click");
out.picked = sent.splice(0);
current = "minutes";
await told({ theBox: "reports" });
face = box("reports");
const field = box("telegram-minutes-field");
const telegram = face.parentNode.parentNode;
out.minutes = {
  closed: read(face.textContent), pick: face.parentNode.className,
  line: telegram.children.map((one) => one.tagName === "INPUT" ? `INPUT:${sorted(one)}:${one.value}` : one.names.has("hintl") ? `hintl:${read(one.textContent)}` : one.className),
  label: read(field.getAttribute("aria-label")),
  inASet: (() => { for (let at = field.parentNode; at; at = at.parentNode) if (at.names && at.names.has("set")) return true; return false; })(),
  bad: field.names.has("bad"),
  answerLine: answerLine(),
};
field.value = "0";
fire(field, "change");
out.typed = sent.splice(0);
await told(answers.refused);
out.refused = { bad: box("telegram-minutes-field").names.has("bad"), value: box("telegram-minutes-field").value, answerLine: answerLine() };
await told({ theBox: "job-name" });
out.redrawn = { bad: box("telegram-minutes-field").names.has("bad"), value: box("telegram-minutes-field").value, answerLine: answerLine() };
face = box("reports");
const list = listOf(face);
const at = (top, bottom) => ({ top, bottom, left: 0, right: 100, width: 100, height: bottom - top });
face.rect = at(760, 786);
list.rect = at(790, 930);
face.press();
out.upward = { open: !list.hidden, up: list.names.has("up"), rows: list.children.length };
console.log(JSON.stringify(out));
"#;

    /// **What a user gets today from each of the two, asserted rather than only
    /// written down.** The page takes the value and draws it: nothing is
    /// refused at the box and nothing is hidden, because neither is a refusal
    /// J11 states and hiding a control the drawing draws would make the app
    /// look different rather than closed.
    #[test]
    fn both_dead_controls_still_take_the_users_answer_and_draw_it() {
        let mut held = a_run_with_a_reading();
        let file = "/ephemeral/smoozy/assets/kitchen_v14.blend";
        assert_eq!(
            the_answer_is_taken(&mut held, &a_server(), file, WhichSetting::SampleCount, "512"),
            None,
            "the samples box refused a count inside Cycles' own range"
        );
        assert_eq!(
            the_answer_is_taken(&mut held, &a_server(), file, WhichSetting::Device, "the-processor"),
            None,
            "the device control refused the processor"
        );
        let panel = the_panel(&held, &a_server(), file);
        assert_eq!(panel.samples.whose.as_str(), "the-users");
        assert_eq!(panel.samples.value.as_ref().map(Verbatim::as_str), Some("512"));
        assert_eq!(
            panel
                .renders_on
                .as_ref()
                .expect("the pair is drawn under Cycles")
                .on
                .as_str(),
            "the-processor"
        );
    }

    /// **Step 10c's sixth pass, fault 5, taken as a class: on a file not yet
    /// read, every control that takes an answer draws the user's own.**
    /// `§ Before the file has been read` lists them — the frame range, the
    /// samples, the percentage, the engine and what it renders on — and
    /// `Render Settings` J1 puts the user's answer first whatever else stands.
    /// **The device pair drew the promise form whatever was pressed**, which
    /// is what the pass met; the other four already drew the answer, and this
    /// test holds all five so none of them drifts back.
    #[test]
    fn on_a_file_not_yet_read_every_answer_is_drawn_as_the_users() {
        let mut held = WhatTheRunHolds::default();
        let file = "/vast6-cube.blend";
        for (which, text) in [
            (WhichSetting::FrameRange, "1-3"),
            (WhichSetting::SampleCount, "1024"),
            (WhichSetting::ResolutionPercentage, "50"),
            (WhichSetting::EnginePick, "cycles"),
            (WhichSetting::Device, "the-processor"),
        ] {
            assert_eq!(
                the_answer_is_taken(&mut held, &a_server(), file, which, text),
                None,
                "{text} was refused on an unread page"
            );
        }
        let panel = the_panel(&held, &a_server(), file);
        assert!(!panel.a_reading_stands, "nothing was read");
        assert_eq!(panel.the_range.whose.as_str(), "the-users");
        assert_eq!(panel.samples.whose.as_str(), "the-users");
        assert_eq!(panel.samples.value.as_ref().map(Verbatim::as_str), Some("1024"));
        assert_eq!(panel.resolution.whose.as_str(), "the-users");
        assert_eq!(panel.engine.on.as_str(), "cycles");
        let device = panel.renders_on.expect("the pair is drawn with no engine read");
        assert_eq!(device.on.as_str(), "the-processor", "the user's CPU is the row that is on");
        assert!(device.not_read_yet.is_some(), "the promise row is still there to go back to");

        // **With no answer the promise form is what is on**, as before.
        let untouched = the_panel(&WhatTheRunHolds::default(), &a_server(), file);
        assert_eq!(
            untouched.renders_on.expect("the pair").on.as_str(),
            "the-promise-form"
        );

        // **The page puts *on* on the promise row only where the setting is on
        // it** — until the pass it drew that row on unconditionally.
        let page = the_pages_function("function theRendersOn(server, said) {");
        assert!(
            page.contains(r#"device.on === "the-promise-form" ? "rad on" : "rad""#),
            "the promise row is drawn on whatever the user pressed"
        );
    }

    // ════════════════════════════════════════════════════════════════════════
    // The two objects, and the boxes that refuse
    // ════════════════════════════════════════════════════════════════════════

    /// **J4: nothing ever converts one kind into the other.** A value the user
    /// has not touched has no store entry, and the page shows the reading's
    /// value in the file's own face — which is what the whole page rests on.
    #[test]
    fn an_untouched_setting_is_the_files_and_has_no_entry() {
        let held = a_run_with_a_reading();
        let panel = the_panel(
            &held,
            &a_server(),
            "/ephemeral/smoozy/assets/kitchen_v14.blend",
        );
        assert_eq!(panel.samples.whose.as_str(), "the-files");
        assert_eq!(panel.the_range.whose.as_str(), "the-files");
        assert_eq!(panel.resolution.whose.as_str(), "the-files");
        assert!(panel.samples.hand_back_to.is_none());
    }

    /// **J11 and J12: the page refuses in exactly three places, the typed value
    /// never becomes an entry, and the line says what Blender would otherwise
    /// have done silently.**
    #[test]
    fn a_refused_number_never_becomes_an_entry_and_the_line_says_what_would_have_happened() {
        let mut held = a_run_with_a_reading();
        let file = "/ephemeral/smoozy/assets/kitchen_v14.blend";
        let refusal = the_answer_is_taken(
            &mut held,
            &a_server(),
            file,
            WhichSetting::ResolutionPercentage,
            "40000",
        )
        .expect("40000 is outside 1 to 32767");
        let said = what_the_refusal_says(&refusal);
        assert!(
            said.len() >= 2,
            "the refusal states neither the range nor what would have happened"
        );
        // The entry that stood goes on standing — here, none at all.
        let panel = the_panel(&held, &a_server(), file);
        assert_eq!(panel.resolution.whose.as_str(), "the-files");

        // The frame field's own four, and J11 item 2's fifth.
        for typed in ["240-1", "1-240x0", "", "1-10, x", "1-2000000"] {
            assert!(
                the_answer_is_taken(
                    &mut held,
                    &a_server(),
                    file,
                    WhichSetting::FrameRange,
                    typed
                )
                .is_some(),
                "the frame field took {typed:?}"
            );
        }
    }

    /// **J11 item 3's fourth bullet, in both directions**: an act moving the
    /// engine in force while a count above the resulting engine's ceiling
    /// stands is refused in place.
    #[test]
    fn the_engine_pick_is_held_to_the_standing_count() {
        let mut held = a_run_with_a_reading();
        let file = "/ephemeral/smoozy/assets/kitchen_v14.blend";
        assert_eq!(
            the_answer_is_taken(&mut held, &a_server(), file, WhichSetting::EnginePick, "eevee"),
            None
        );
        assert_eq!(
            the_answer_is_taken(
                &mut held,
                &a_server(),
                file,
                WhichSetting::SampleCount,
                "20000000"
            ),
            None,
            "EEVEE's own range takes twenty million"
        );
        assert!(
            the_answer_is_taken(&mut held, &a_server(), file, WhichSetting::EnginePick, "cycles")
                .is_some(),
            "changing the pick to Cycles under a count above its ceiling was taken"
        );
        assert!(
            the_value_is_handed_back(&mut held, &a_server(), file, WhichSetting::EnginePick)
                .is_some(),
            "clearing the pick under a count above the no-pick ceiling was taken"
        );
    }

    // ════════════════════════════════════════════════════════════════════════
    // The states, drawn
    // ════════════════════════════════════════════════════════════════════════

    /// **`§ Before the file has been read`: the scene list and the View Layer
    /// boxes are absent rather than empty, and Render names no count.**
    #[test]
    fn a_page_nobody_has_read_says_so_once_and_keeps_its_controls() {
        let panel = the_panel(
            &WhatTheRunHolds::default(),
            &a_server(),
            "/ephemeral/smoozy/assets/kitchen_v14.blend",
        );
        assert!(!panel.the_band.is_empty(), "the band says nothing");
        assert!(!panel.the_scene_group_is_drawn);
        assert!(!panel.the_view_layer_group_is_drawn);
        assert_eq!(panel.the_render_control.len(), 1);
        assert!(panel.the_render_control[0].values.is_empty());
        // **Everything the user can answer without a read, they can answer.**
        assert_eq!(panel.samples.whose.as_str(), "the-promise-form");
        assert!(panel.the_size_is_not_known.is_some());
        assert!(!panel.the_download_is_on);
        assert!(panel.the_format_list_is_not_offered.is_none());
    }

    /// **Step 10c's Vast pass: the Render control read *Render 1 frames* for a
    /// one-frame range.** The frame count beside the range already took the
    /// singular; the control did not. Typed onto a read file, as the pass's
    /// *pv lion* was.
    #[test]
    fn a_one_frame_range_is_rendered_as_one_frame() {
        let mut held = a_run_with_a_reading();
        let file = "/ephemeral/smoozy/assets/kitchen_v14.blend";
        for (typed, said) in [("1", "Render <1> frame"), ("1-20", "Render <20> frames")] {
            assert_eq!(
                the_answer_is_taken(&mut held, &a_server(), file, WhichSetting::FrameRange, typed),
                None,
                "{typed} was refused"
            );
            let panel = the_panel(&held, &a_server(), file);
            assert!(panel.a_reading_stands);
            assert_eq!(the_words_of_a_line(&panel.the_render_control), said);
        }
    }

    // ── Step 10c piece 6: submitting, and the version list ─────────────────

    pub(crate) fn a_row_on(server: &str, with_a_root: bool) -> smoozy_servers::store::ServerRow {
        use smoozy_servers::identity::{Host, KeyReference, Port, Username};
        smoozy_servers::store::ServerRow {
            local_server_identifier: server.to_string(),
            host: Host::what_the_user_typed("203.0.113.9").expect("a host"),
            port: Port::STANDARD_SSH,
            username: Username::what_the_user_typed("root").expect("a user"),
            key: KeyReference {
                local_key_identifier: "key-1".into(),
                fingerprint: "SHA256:aaa".into(),
            },
            label: Some("the big box".to_string()),
            colour: Some("b3".to_string()),
            remembered_setup: None,
            remembered_host_key: None,
            remembered_hardware_reading: None,
            recorded_machine_root: with_a_root.then(|| smoozy_servers::store::RecordedMachineRoot {
                path: "/ephemeral/smoozy".to_string(),
                the_package_areas_own_location: None,
                read_under: "b".repeat(32),
            }),
            // **A row with a root is a row whose setup committed**, so its
            // installation is confirmed too (step 10c's RunPod-pass fix
            // piece 1: the page asks for both).
            last_confirmed_installation_identity: with_a_root.then(|| "b".repeat(32)),
            event_cursor: None,
            machine_password_subject: None,
            chosen_fallback_folder: None,
            the_driver_missed_the_optix_bar: false,
        }
    }

    pub(crate) fn a_run() -> Identity {
        Identity::issue(IdentityKind::Run).expect("randomness")
    }

    /// **Step 10c's dependency wave piece 7: the band and the Submit read one
    /// check, and a stop Smoozy cannot keep is not let stand.** A complete
    /// standing that left nothing unfound stops the warning only where
    /// [`TheCheckTheBandReads::a_stop_can_be_kept`]; where it is not, the band
    /// is drawn under *nobody has checked* and the Submit's `Queue & History`
    /// N15 mark says what the band said.
    #[test]
    fn the_band_and_the_submit_read_one_check_and_an_unkeepable_stop_does_not_stand() {
        use smoozy_dependencies::bytes::MachineLocation;
        use smoozy_dependencies::record::MachineKey;
        use smoozy_dependencies::standing::{CheckStanding, Completion};
        let file = "/ephemeral/smoozy/assets/kitchen_v14.blend";
        let mut held = a_run_with_a_reading();
        held.the_saved_with.insert(keyed(&a_server(), file), "4.5.3".to_string());
        let row = a_row_on(&a_server(), true);
        let machine = MachineKey::of(
            an_identity(&a_server()),
            Identity::issue(IdentityKind::Installation).expect("randomness"),
        );
        let root = MachineLocation::composed(file.as_bytes().to_vec());
        let picked = BlenderIdentity {
            path: "/opt/blender-5.1.0/blender".to_string(),
            version: a_version(5, 1, 0),
        };
        let standing = CheckStanding::at_the_sittings_end(
            root.clone(),
            machine.clone(),
            picked,
            std::time::SystemTime::now(),
            Completion::Completed,
            &[],
            four_values(),
        );
        let a_record = |a_stop_can_be_kept: bool| WhatTheRecordHolds {
            the_dependency_warning: TheSwitchReads::NotSwitchedOff,
            the_check: TheCheckTheBandReads {
                the_record_says: WhatTheRecordSays::AStanding(Box::new(standing.clone())),
                a_stop_can_be_kept,
            },
            ..WhatTheRecordHolds::default()
        };

        let kept = a_record(true);
        assert!(the_panel_starting_from(&held, &a_server(), file, &kept)
            .the_dependency_band
            .is_none());
        let given = the_act_as_the_page_composed_it(&held, &a_server(), file, &kept, &row, a_run())
            .unwrap_or_else(|said| panic!("{said:?}"));
        assert_eq!(
            given.dependency_check,
            Some(smoozy_queue_history::record::DependencyCheckAtSubmission::ACompleteStandingStood)
        );

        let not_kept = a_record(false);
        let band = the_panel_starting_from(&held, &a_server(), file, &not_kept)
            .the_dependency_band
            .expect("a stop Smoozy cannot keep does not stand");
        assert_eq!(band.which.as_str(), "never");
        let given =
            the_act_as_the_page_composed_it(&held, &a_server(), file, &not_kept, &row, a_run())
                .unwrap_or_else(|said| panic!("{said:?}"));
        assert_eq!(
            given.dependency_check,
            Some(smoozy_queue_history::record::DependencyCheckAtSubmission::NoStandingAtAll)
        );
    }

    /// **The act, as the page composed it** — `Render Settings` Y1's payload
    /// from the store and the resolved pick, the job's name as typed, the
    /// row's own label and badge, and the package area from piece 4's
    /// `where_the_package_area_is`. **One scene ticked or none is one job; two
    /// ticked are two, each carrying its own read name** (`Rendering` S2, S6).
    #[test]
    fn the_act_is_composed_from_the_page_the_row_and_the_resolved_pick() {
        let file = "/ephemeral/smoozy/assets/kitchen_v14.blend";
        let mut held = a_run_with_a_reading();
        // **`File Browser`'s saved-with read, handed over when the file was
        // pointed at** — what the automatic match reads with several scenes
        // ticked, when no one scene is in hand.
        held.the_saved_with.insert(keyed(&a_server(), file), "4.5.3".to_string());
        held.submission.insert(
            keyed(&a_server(), file),
            TheSubmissionChoices {
                job_name: "Kitchen, first pass".to_string(),
                ..TheSubmissionChoices::default()
            },
        );
        let row = a_row_on(&a_server(), true);
        let given = the_act_as_the_page_composed_it(
            &held,
            &a_server(),
            file,
            &WhatTheRecordHolds::default(),
            &row,
            a_run(),
        )
        .unwrap_or_else(|said| panic!("{said:?}"));
        assert_eq!(given.scene_jobs.len(), 1);
        assert!(given.scene_jobs[0].scene.is_none(), "no scene ticked names none (`Rendering` L15)");
        assert_eq!(given.scene_jobs[0].version_pick.version, "5.1.0");
        assert_eq!(given.label.as_deref(), Some("Kitchen, first pass"));
        assert_eq!(given.server.label.as_deref(), Some("the big box"));
        assert_eq!(given.server.hue, Some(smoozy_queue_history::record::Hue(3)));
        assert_eq!(given.package_area, "/ephemeral/smoozy/package");
        assert_eq!(given.the_blend, file);
        assert!(given.the_reading_in_force.is_some());
        assert_eq!(
            given.the_version_origin,
            smoozy_queue_history::record::AnswerOrigin::ResolvedForThem
        );
        assert_eq!(
            given.dependency_check,
            Some(smoozy_queue_history::record::DependencyCheckAtSubmission::NoStandingAtAll)
        );
        // **`Queue & History` N15's second mark rides only a carry-on's
        // page** (the 38's piece 2): none on an ordinary page or a submit
        // again, the entry's job and label on a carry-on.
        assert_eq!(given.carries_on_from, None);
        let earlier = Identity::issue(IdentityKind::Job).unwrap();
        let came_from = |band| ThePageCameFrom {
            band,
            the_simplify_offer_is_drawn: false,
            the_entry: earlier.clone(),
            the_entrys_label: "Kitchen, take one".to_string(),
            the_moment: None,
        };
        let mut again = a_run_with_a_reading();
        again.the_saved_with.insert(keyed(&a_server(), file), "4.5.3".to_string());
        the_page_came_from(
            &mut again,
            &a_server(),
            file,
            came_from(WhereItCameFrom::SubmittingAgain {
                the_jobs_name: "Kitchen, take one".to_string(),
                when: String::new(),
            }),
        );
        let given = the_act_as_the_page_composed_it(&again, &a_server(), file, &WhatTheRecordHolds::default(), &row, a_run())
            .unwrap_or_else(|said| panic!("{said:?}"));
        assert_eq!(given.carries_on_from, None);
        the_page_came_from(
            &mut again,
            &a_server(),
            file,
            came_from(WhereItCameFrom::CarryingOnWithTheOutputsUnread {
                the_jobs_name: "Kitchen, take one".to_string(),
                when: String::new(),
                the_range_now_starts_at: 60,
            }),
        );
        let given = the_act_as_the_page_composed_it(&again, &a_server(), file, &WhatTheRecordHolds::default(), &row, a_run())
            .unwrap_or_else(|said| panic!("{said:?}"));
        assert_eq!(
            given.carries_on_from,
            Some(smoozy_queue_history::record::CarriesOnFrom {
                earlier_job: earlier.clone(),
                the_label_as_it_stood: "Kitchen, take one".to_string(),
            })
        );

        for scene in ["Interior", "Exterior"] {
            let key = FileKey {
                server: an_identity(&a_server()),
                path: file.to_string(),
            };
            let provenance = the_provenance(&held, &key).expect("a reading stands");
            held.store.tick_a_scene(
                key,
                NamedAnswer {
                    name: ExactName::as_the_reading_produced_it(scene),
                    provenance,
                },
            );
        }
        let given = the_act_as_the_page_composed_it(
            &held,
            &a_server(),
            file,
            &WhatTheRecordHolds::default(),
            &row,
            a_run(),
        )
        .unwrap_or_else(|said| panic!("{said:?}"));
        let named: Vec<String> = given
            .scene_jobs
            .iter()
            .map(|payload| payload.scene.as_ref().expect("a named scene").name.as_text().to_string())
            .collect();
        assert_eq!(named, vec!["Interior", "Exterior"]);
    }

    /// **The card make the user was shown reaches the submission** —
    /// `Job files and outputs` M1b through `Render Settings` D5, and step 10c's
    /// fifth pass on Vast, fault 4.
    ///
    /// **The row draws its card and the manifest names it**, through the one
    /// judgement `Servers` makes about the row's state. Until 2026-09-23 the
    /// map this page reads was written by nothing at all, so a row reading
    /// *1 × NVIDIA GeForce GTX 1650, 4 GB* submitted `NoReadingStood` and the
    /// render worker ended the attempt before the render instruction.
    #[test]
    fn a_submission_from_a_row_showing_a_card_carries_that_card() {
        use smoozy_servers::hardware::{Card, CardMake, Cards, HardwareReading, MachineMemory};
        use std::time::{Duration, SystemTime};

        let file = "/ephemeral/smoozy/assets/kitchen_v14.blend";
        let mut held = a_run_with_a_reading();
        let mut row = a_row_on(&a_server(), true);
        row.remembered_hardware_reading = Some(HardwareReading::taken(
            Cards::TheCards(vec![Card {
                unique_identifier: "GPU-1".to_string(),
                bus_location: "00000000:01:00.0".to_string(),
                make: CardMake::from_the_drivers_own_query("NVIDIA GeForce GTX 1650"),
                total_memory_bytes: Some(4 * 1024 * 1024 * 1024),
            }]),
            MachineMemory::CouldNotBeEstablished,
            None,
            SystemTime::UNIX_EPOCH + Duration::from_secs(1_756_800_000),
            &a_server(),
            SystemTime::UNIX_EPOCH + Duration::from_secs(1_756_900_000),
            None,
        ));

        // **The state is the one the row is drawn in** — no connection in this
        // run has read the server, so `Servers` H1's own table governs and the
        // row shows its card as *last seen*. A held reading is a held reading
        // on both sides of the seam.
        let hardware = the_hardware_the_row_stands_in(
            &row,
            None,
            crate::surface::servers::what_is_true_of_a_server_with_no_period(),
        );
        the_hardware_stands_at(&mut held, &a_server(), hardware);

        let given = the_act_as_the_page_composed_it(
            &held,
            &a_server(),
            file,
            &WhatTheRecordHolds::default(),
            &row,
            a_run(),
        )
        .unwrap_or_else(|said| panic!("{said:?}"));
        // **The make's key and never the driver's name for the card** —
        // `Servers` H7. Step 10c's sixth pass: this test pinned the full name
        // while the machine's table was keyed on the maker, and every cards job
        // on every NVIDIA machine failed. **It now names the same constant the
        // machine's table is keyed by**, so the two halves read one fact.
        assert_eq!(
            given.the_card_make,
            smoozy_job_files::manifest::TheCardMakeAtSubmission::TheMakes(vec![
                smoozy_servers::hardware::NVIDIA.to_string()
            ])
        );

        // **And a row nothing has ever read still says so**, which is
        // `Rendering` L23's own
        // already-written ending rather than a gap.
        let mut nothing = a_run_with_a_reading();
        the_hardware_stands_at(
            &mut nothing,
            &a_server(),
            the_hardware_the_row_stands_in(
                &a_row_on(&a_server(), true),
                None,
                crate::surface::servers::what_is_true_of_a_server_with_no_period(),
            ),
        );
        let given = the_act_as_the_page_composed_it(
            &nothing,
            &a_server(),
            file,
            &WhatTheRecordHolds::default(),
            &a_row_on(&a_server(), true),
            a_run(),
        )
        .unwrap_or_else(|said| panic!("{said:?}"));
        assert_eq!(
            given.the_card_make,
            smoozy_job_files::manifest::TheCardMakeAtSubmission::NoReadingStood
        );
    }

    /// **What the page cannot compose is said and never sent**: a server with
    /// no placement recorded, and a page where no version resolves.
    #[test]
    fn an_act_the_page_cannot_compose_says_why_and_sends_nothing() {
        let file = "/ephemeral/smoozy/assets/kitchen_v14.blend";
        let held = a_run_with_a_reading();
        let said = the_act_as_the_page_composed_it(
            &held,
            &a_server(),
            file,
            &WhatTheRecordHolds::default(),
            &a_row_on(&a_server(), false),
            a_run(),
        )
        .err()
        .expect("refused");
        assert!(the_words_of(&said).contains(&words::NOT_SET_UP_ON_THIS_SERVER.to_string()));

        // **A root recorded and no installation confirmed is not set up
        // either** — step 10c's RunPod-pass fix piece 1. A setup's hand-over
        // records the root before its placement is sent, so a first setup the
        // machine deferred leaves exactly this row; the act then stopped at
        // its first step and said *could not put the job's files on the
        // server*, about a server nothing had been sent to.
        let mut deferred = a_row_on(&a_server(), true);
        deferred.last_confirmed_installation_identity = None;
        let said = the_act_as_the_page_composed_it(
            &held,
            &a_server(),
            file,
            &WhatTheRecordHolds::default(),
            &deferred,
            a_run(),
        )
        .err()
        .expect("a server with no installation confirmed is refused at the page");
        assert_eq!(
            the_words_of(&said),
            vec![words::NOT_SUBMITTED.to_string(), words::NOT_SET_UP_ON_THIS_SERVER.to_string()]
        );
        // **And one whose installation will not read back is the same.**
        deferred.last_confirmed_installation_identity = Some("not an identity".to_string());
        assert!(the_act_as_the_page_composed_it(
            &held,
            &a_server(),
            file,
            &WhatTheRecordHolds::default(),
            &deferred,
            a_run(),
        )
        .is_err());

        let mut no_list = a_run_with_a_reading();
        no_list.version_lists.clear();
        assert!(the_act_as_the_page_composed_it(
            &no_list,
            &a_server(),
            file,
            &WhatTheRecordHolds::default(),
            &a_row_on(&a_server(), true),
            a_run(),
        )
        .is_err());
    }

    /// **`Rendering` S10: what the act says, one sentence for each thing that
    /// is true**.
    #[test]
    fn what_the_act_says_is_what_became_of_each_scene_job() {
        use crate::submitting::{WhatBecameOfOneJob as Became, WhatTheActCameTo as CameTo};
        let words_of = |came_to: CameTo| the_words_of(&what_the_act_says(&came_to)).join(" ");
        assert_eq!(words_of(CameTo::TheseJobs(vec![Became::ItIsInTheLine])), words::SUBMITTED);
        let refused = words_of(CameTo::TheseJobs(vec![Became::TheServerDoesNotTakeJobs]));
        assert!(refused.contains(words::NOT_SUBMITTED) && refused.contains(words::THE_SERVER_DOES_NOT_TAKE_JOBS));
        let some = words_of(CameTo::TheseJobs(vec![
            Became::ItIsInTheLine,
            Became::NotKnown,
            Became::NotAttemptedBehindAnUnaccountedOne,
        ]));
        assert!(some.contains('1') && some.contains('3'), "{some}");
        assert!(some.contains(words::NOT_KNOWN_WHETHER_IT_WAS_TAKEN));
        assert!(some.contains(words::STOPPED_BEHIND_AN_UNKNOWN_ONE));
        // **`Rendering` S13's two jobs**: both in the line is *Submitted.*;
        // the rest not sent behind a first that was not says the first's
        // reason, once, and nothing of its own (step 10c's eleventh-pass fix
        // piece 8, where this read *Only the first frame was submitted…*).
        let both = words_of(CameTo::TheseJobs(vec![Became::ItIsInTheLine, Became::ItIsInTheLine]));
        assert_eq!(both, words::SUBMITTED);
        let neither = what_the_act_says(&CameTo::TheseJobs(vec![
            Became::TheServerDoesNotTakeJobs,
            Became::NotSentBehindAFirstFrameThatWasNot,
        ]));
        assert_eq!(
            the_words_of(&neither),
            vec![words::NOT_SUBMITTED, words::THE_SERVER_DOES_NOT_TAKE_JOBS]
        );
        // **Each reason once**, however many jobs met it.
        let twice = what_the_act_says(&CameTo::TheseJobs(vec![
            Became::TheServerDoesNotTakeJobs,
            Became::TheServerDoesNotTakeJobs,
        ]));
        assert_eq!(twice.len(), 2);
    }

    /// **Step 10c's tenth pass: *Submitted.* was drawn in dark red**, the
    /// colour `Design values` gives to *"Refused or blocked"* and nothing else,
    /// so a job that went out read as a failure. **What went out is a plain
    /// note, and only what was refused keeps `--alert`**: *Submitted.* and *2
    /// of 3 jobs submitted.* are plain; *Not submitted.* and every reason are
    /// not. *(So was the first-frame sentence, struck at the eleventh-pass fix
    /// piece 8.)*
    /// The page harness measured the colours (tenth-pass fix pieces 2 and 6's
    /// account); this holds the two halves to one another.
    #[test]
    fn what_went_out_is_a_plain_note_and_only_a_refusal_keeps_the_refusal_colour() {
        use crate::submitting::{WhatBecameOfOneJob as Became, WhatTheActCameTo as CameTo};
        let plain_of = |came_to: CameTo| the_acts_answer(&came_to).1;
        assert_eq!(
            the_acts_answer(&CameTo::TheseJobs(vec![Became::ItIsInTheLine, Became::ItIsInTheLine])),
            (vec![plainly(words::SUBMITTED)], 1)
        );
        let (some, plain) = the_acts_answer(&CameTo::TheseJobs(vec![
            Became::ItIsInTheLine,
            Became::NotKnown,
            Became::ItIsInTheLine,
        ]));
        assert_eq!(plain, 2, "{some:?}");
        let went = the_words_of(&some[..plain].to_vec()).join(" ");
        assert!(went.contains('2') && went.contains('3') && went.contains(words::JOBS_SUBMITTED), "{went}");
        assert_eq!(the_words_of(&some[plain..].to_vec()), vec![words::NOT_KNOWN_WHETHER_IT_WAS_TAKEN]);
        assert_eq!(plain_of(CameTo::TheseJobs(vec![Became::TheServerDoesNotTakeJobs])), 0);
        assert_eq!(plain_of(CameTo::TheFirstFrameIsNotKnown { a_scene_can_be_ticked: false }), 0);
        assert_eq!(plain_of(CameTo::TheFirstFrameIsNotKnown { a_scene_can_be_ticked: true }), 0);
        assert_eq!(
            plain_of(CameTo::TheseJobs(vec![
                Became::TheServerDoesNotTakeJobs,
                Became::NotSentBehindAFirstFrameThatWasNot
            ])),
            0
        );

        // **The submission's answer carries it, and so does *Submitting.***
        assert_eq!(
            a_submission_said("s", "f", vec![plainly(words::SUBMITTED)], 1),
            an_act_said("s", "f", "submit", vec![plainly(words::SUBMITTED)])
                .saying(THE_PLAIN_PARTS, Said::Value(Verbatim::new("1")))
        );
        let working = the_working_half();
        assert!(working.contains("a_submission_said(&server, &file, vec![plainly(words::SUBMITTING)], 1)"));
        assert!(working.contains("let (said, plain) = the_acts_answer(&came_to);"));

        // **The page reads it and draws the plain parts as the drawing's plain
        // note**, and everything else as the refusal.
        let page = the_page();
        assert!(page.contains(&format!("const THE_PLAIN_PARTS = \"{THE_PLAIN_PARTS}\"")));
        let at = page.find("function theRefusalIsPutOnItsBox(").expect("the answer is put on its line");
        let body = &page[at..at + page[at..].find("\n}\n").expect("it closes")];
        assert!(body.contains("said[THE_PLAIN_PARTS]"), "the page never reads which parts are plain");
        assert!(body.contains("plain === parts.length ? \"bar-note underval\" : \"alertnote\""));
    }

    /// **A job whose files were placed and which the machine says it does not
    /// hold is told as the server not taking it — never as its files not
    /// placed**, which was untrue: they were on the server. A unit stopped at
    /// an earlier step is still the files not placed. Added 2026-09-27, out of
    /// eighth-pass fix piece 4's resolver.
    #[test]
    fn a_job_the_machine_does_not_hold_is_told_as_not_taken_and_not_as_files_not_placed() {
        use crate::submitting::{WhatBecameOfOneJob as Became, WhatTheActCameTo as CameTo};
        use smoozy_rendering::submission::THE_ORDERED_UNIT;
        let words_of = |came_to: CameTo| the_words_of(&what_the_act_says(&came_to)).join(" ");
        let not_taken = words_of(CameTo::TheseJobs(vec![Became::ItStoppedBeforeTheJobRecord {
            the_step: THE_ORDERED_UNIT[2],
        }]));
        assert!(not_taken.contains(words::NOT_SUBMITTED), "{not_taken}");
        assert!(not_taken.contains(words::THE_SERVER_DID_NOT_TAKE_THE_JOB), "{not_taken}");
        assert!(
            !not_taken.contains(words::THE_FILES_COULD_NOT_BE_PLACED),
            "the files were placed, and the page said they were not: {not_taken}"
        );
        for earlier in &THE_ORDERED_UNIT[..2] {
            let files = words_of(CameTo::TheseJobs(vec![Became::ItStoppedBeforeTheJobRecord {
                the_step: earlier,
            }]));
            assert!(files.contains(words::THE_FILES_COULD_NOT_BE_PLACED), "{earlier}: {files}");
        }
    }

    /// **The Submit control no longer answers with the shared sentence** —
    /// the act is real now, and `A_CONNECTION_IS_NOT_BUILT` is read by the file
    /// read alone on this page.
    #[test]
    fn submitting_no_longer_says_the_act_is_not_built() {
        let command = the_working_half()
            .split("pub fn a_job_is_submitted<")
            .nth(1)
            .and_then(|rest| rest.split("\n}").next())
            .expect("the command");
        assert!(!command.contains("NEEDS_A_CONNECTION"));
        assert!(command.contains("crate::submitting::the_act_is_performed"));
        assert!(command.contains("std::thread::spawn"), "the act runs off the drawing thread (P5)");
    }

    /// **The Creator's seventh notes, piece Z7: the *Submitting…* row is
    /// begun at the press and the queue drawn at once**, before the act's
    /// thread starts; the act carries the row; and the queue is drawn again
    /// when the act ends, after the row has learned how.
    #[test]
    fn z7_the_press_begins_the_submitting_row_before_the_act_runs() {
        let command = the_working_half()
            .split("pub fn a_job_is_submitted<")
            .nth(1)
            .and_then(|rest| rest.split("\n}").next())
            .expect("the command")
            .to_string();
        let at = |what: &str| command.find(what).unwrap_or_else(|| panic!("{what} is in the command"));
        let begun = at("crate::submitting::an_act_is_begun(&given)");
        let spawned = at("std::thread::spawn");
        let drawn = at("crate::surface::the_queue::the_queue_is_told_again(&app)");
        assert!(begun < drawn && drawn < spawned, "begun, drawn, then the act's thread");
        let performed = at("crate::submitting::the_act_is_performed_in_flight(");
        let drawn_again = command.rfind("crate::surface::the_queue::the_queue_is_told_again(&app)").expect("again");
        assert!(spawned < performed && performed < drawn_again, "drawn again when the act has ended");
        assert!(command[performed..].contains("in_flight"), "the act carries the row begun at the press");
    }

    /// **`Server Setup` B12 and `Render Settings` B1: while the list is being
    /// asked for, the header says so, and where the machine could not be asked
    /// it says that** — never *no server is connected* about a connected one.
    #[test]
    fn the_header_says_the_server_is_being_searched_and_never_that_none_is_connected() {
        let file = "/ephemeral/smoozy/assets/kitchen_v14.blend";
        let mut held = a_run_with_a_reading();
        held.version_lists.clear();
        held.version_searches.insert(a_server(), TheSearchStands::Searching);
        let panel = the_panel(&held, &a_server(), file);
        assert_eq!(panel.the_pick_line, Some(Prose::new(words::SEARCHING_THIS_SERVER)));
        assert!(!panel.submitting_is_offered);

        the_version_list_came_back(
            &mut held,
            &a_server(),
            smoozy_render_settings::acting::versions::TheVersionListAsk::TheMachineCouldNotBeAsked,
        );
        let panel = the_panel(&held, &a_server(), file);
        assert_eq!(panel.the_pick_line, Some(Prose::new(words::THE_SERVER_COULD_NOT_BE_SEARCHED)));
        let list = a_run_with_a_reading().version_lists.remove(&a_server()).expect("a list");
        the_version_list_came_back(
            &mut held,
            &a_server(),
            smoozy_render_settings::acting::versions::TheVersionListAsk::Answered(list),
        );
        assert!(held.version_searches.is_empty());
        let panel = the_panel(&held, &a_server(), file);
        assert!(panel.submitting_is_offered);

        // **Period-held**: the period ends and the list goes with it.
        the_period_ended_for(&mut held, &a_server());
        let panel = the_panel(&held, &a_server(), file);
        assert_eq!(panel.the_pick_line, Some(Prose::new(words::NO_MACHINE_IN_HAND)));
    }

    /// **One act at a time from one page**: while a submission runs, the
    /// control is not offered again.
    #[test]
    fn the_control_is_not_offered_while_a_submission_runs() {
        let file = "/ephemeral/smoozy/assets/kitchen_v14.blend";
        let mut held = a_run_with_a_reading();
        assert!(the_panel(&held, &a_server(), file).submitting_is_offered);
        held.submitting_now.insert(keyed(&a_server(), file), true);
        assert!(!the_panel(&held, &a_server(), file).submitting_is_offered);
    }

    /// **`Design/Whole app § Anything that isn't instant shows that it's
    /// underway`: *Render* carries the *on the control* mark while its
    /// submission runs**, and the page is told that apart from a control that
    /// is simply not offered. Step 10c's eighth-pass fix piece 5: on the pass
    /// the control went grey for the whole submission and said nothing else.
    #[test]
    fn the_page_is_told_a_submission_is_running_and_not_only_that_render_is_withdrawn() {
        let file = "/ephemeral/smoozy/assets/kitchen_v14.blend";
        let mut held = a_run_with_a_reading();
        let before = the_panel(&held, &a_server(), file);
        assert!(!before.the_submission_is_underway);
        held.submitting_now.insert(keyed(&a_server(), file), true);
        let during = the_panel(&held, &a_server(), file);
        assert!(during.the_submission_is_underway);
        assert!(!during.submitting_is_offered);
        held.submitting_now.remove(&keyed(&a_server(), file));
        assert!(!the_panel(&held, &a_server(), file).the_submission_is_underway);
    }

    /// **The Blender search's line carries the mark while the search runs, and
    /// only then** — `Design/Whole app`: where no control started the work,
    /// *"the reading it produced carries the mark itself."* A server that
    /// could not be searched, and one whose list came back, carry none.
    #[test]
    fn the_search_line_is_marked_underway_only_while_the_search_runs() {
        let file = "/ephemeral/smoozy/assets/kitchen_v14.blend";
        let mut held = a_run_with_a_reading();
        assert!(!the_panel(&held, &a_server(), file).the_search_is_underway);
        let list = held.version_lists.remove(&a_server()).expect("a list");
        held.version_searches.insert(a_server(), TheSearchStands::Searching);
        assert!(the_panel(&held, &a_server(), file).the_search_is_underway);
        held.version_searches
            .insert(a_server(), TheSearchStands::TheMachineCouldNotBeAsked);
        assert!(!the_panel(&held, &a_server(), file).the_search_is_underway);
        the_version_list_came_back(
            &mut held,
            &a_server(),
            smoozy_render_settings::acting::versions::TheVersionListAsk::Answered(list),
        );
        assert!(!the_panel(&held, &a_server(), file).the_search_is_underway);
    }

    /// **`§ Scenes`, *More than one ticked*: a control holding a value stays
    /// and empties, a group holding names goes.**
    #[test]
    fn several_scenes_ticked_greys_the_values_and_takes_the_names_away() {
        let mut held = a_run_with_a_reading();
        let file = "/ephemeral/smoozy/assets/kitchen_v14.blend";
        for scene in ["Interior", "Exterior"] {
            let key = FileKey {
                server: an_identity(&a_server()),
                path: file.to_string(),
            };
            let provenance = the_provenance(&held, &key).expect("a reading stands");
            held.store.tick_a_scene(
                key,
                NamedAnswer {
                    name: ExactName::as_the_reading_produced_it(scene),
                    provenance,
                },
            );
        }
        let panel = the_panel(&held, &a_server(), file);
        assert!(panel.the_render_group_is_greyed);
        assert!(panel.samples.greyed && panel.samples.value.is_none());
        assert!(!panel.the_view_layer_group_is_drawn, "the names stayed");
        assert!(panel.the_scene_group_is_drawn, "the scene boxes went");
        // **The range field is live and empty**, and the count reads for the
        // group.
        assert!(!panel.the_range.greyed);
        assert_eq!(panel.the_frame_count.len(), 2);
        assert!(!panel.one_scene_only.is_empty());
        assert!(panel.what_it_will_write.is_none());
    }

    /// **`§ When the read gives nothing`: four endings, not one**, and only the
    /// second carries a second sentence.
    #[test]
    fn the_four_read_endings_each_say_something_different() {
        let picked = BlenderIdentity {
            path: "/opt/blender-5.1.0/blender".to_string(),
            version: a_version(5, 1, 0),
        };
        let mut said = Vec::new();
        for ending in [
            Ending::TheMachineCouldNotBeReached,
            Ending::TheFileCouldNotBeOpenedByThisVersion,
            Ending::TheReadRanAndSomethingInItFailed,
            Ending::SmoozyStoppedIt,
        ] {
            let drawn = the_read_failure(&ending.clone().into(), Some("the big box"), Some(&picked))
                .expect("an ending is drawn");
            said.push(drawn.what_did_not_happen.as_str().to_string());
            let second = drawn.what_to_do_about_it.is_some();
            assert_eq!(
                second,
                matches!(ending, Ending::TheFileCouldNotBeOpenedByThisVersion),
                "the second sentence is on the wrong ending. **The shape grants \
                 one only where Smoozy knows what the user should do next**"
            );
        }
        // **The one case beside the four**, where the server ran no Blender:
        // one sentence of its own, no second one and no Blender output.
        let no_blender = the_read_failure(
            &AReadThatGaveNothing::TheServerRanNoBlender,
            Some("the big box"),
            Some(&picked),
        )
        .expect("it is drawn");
        assert!(no_blender.what_to_do_about_it.is_none() && no_blender.the_raw_text.is_none());
        said.push(no_blender.what_did_not_happen.as_str().to_string());
        said.sort();
        said.dedup();
        assert_eq!(said.len(), 5, "two of the endings say the same thing");
        // **A read Smoozy ended itself is not diagnosed as any of the three**,
        // and it does not send the user to change the version.
        let stopped = the_read_failure(&Ending::SmoozyStoppedIt.into(), Some("the big box"), Some(&picked))
            .expect("it is drawn");
        assert!(stopped.what_to_do_about_it.is_none());
        assert!(stopped.values.is_empty());
    }

    /// **One asset is one asset**: *1 asset this file needs is not on the
    /// server.* The pass before 0.1.9 on Vast read *1 assets this file needs
    /// are not on the server. The check did not leave them there*.
    #[test]
    fn the_unfound_sentence_counts_one_as_one() {
        let counts = |not_found: usize| Counts {
            not_found,
            could_not_be_sent: 0,
            could_not_be_fixed: 0,
            found_but_not_sent: 0,
            set_aside: 0,
            confirmed: 0,
        };
        let read = |said: Vec<Words>| {
            said.iter()
                .map(|words| {
                    let mut all = words.lead.as_str().to_string();
                    for value in &words.values {
                        all.push(' ');
                        all.push_str(value.as_str());
                    }
                    if let Some(rest) = &words.rest {
                        all.push(' ');
                        all.push_str(rest.as_str());
                    }
                    all
                })
                .collect::<Vec<_>>()
                .join(" ")
                .replace('\u{a0}', " ")
        };
        let one = read(the_unfound_sentence(&counts(1)));
        assert!(one.contains("1 asset this file needs is not on the server."), "{one}");
        assert!(!one.contains("The render will not find"), "{one}");
        let two = read(the_unfound_sentence(&counts(2)));
        assert!(two.contains("2 assets this file needs are not on the server."), "{two}");
    }

    /// **`§ The dependency warning`: the four sentences must not read alike.**
    #[test]
    fn the_dependency_bands_four_sentences_are_told_apart() {
        // The four cases are `Rendering` W2's and W4's, and the page draws one
        // sentence for each; this asserts the four are four.
        let four = [
            "never", "replaced", "version", "unfound",
        ];
        let mut seen: Vec<&str> = four.to_vec();
        seen.sort_unstable();
        seen.dedup();
        assert_eq!(seen.len(), 4);
        // **The version case names both Blenders in one sentence**, which is
        // what makes it act on.
        let page = the_page();
        assert!(page.contains("dependencies-"));
    }

    // ════════════════════════════════════════════════════════════════════════
    // The rules the shared half holds every screen to
    // ════════════════════════════════════════════════════════════════════════

    /// **The strip keeps counting, and the counting is this panel's.**
    ///
    /// Piece 5's contract is explicit that the strip is that piece's and the
    /// figures are the panel's, so this is the line that fails if the panel
    /// stops saying anything. **It was not photographed**: the only route into
    /// this panel is `File Browser`'s, which is piece 6's and not built, and
    /// the temporary block that stood in for it is gone.
    #[test]
    fn the_strip_is_told_what_to_count() {
        let held = a_run_with_a_reading();
        let panel = the_panel(
            &held,
            &a_server(),
            "/ephemeral/smoozy/assets/kitchen_v14.blend",
        );
        assert_eq!(
            panel.the_strip_title.values.first().map(Verbatim::as_str),
            Some("kitchen_v14.blend"),
            "the strip does not name the file"
        );
        let reading = panel
            .the_strip_reading
            .expect("a panel with a reading tells its strip what to count");
        assert_eq!(
            reading
                .values
                .iter()
                .map(Verbatim::as_str)
                .collect::<Vec<_>>(),
            vec!["Interior", "240"],
            "the strip's reading is not the scene and its frames"
        );
    }

    /// **Every word the user reads is in `mod words` and nowhere else.**
    #[test]
    fn every_word_is_in_one_place() {
        let source = the_working_half();
        let after = source
            .split("mod words {")
            .nth(1)
            .expect("the words are a module");
        let after = after.split("\n}\n").nth(1).expect("the words module closes");
        let mut faults = Vec::new();
        for (at, _) in after.match_indices("Prose::new(\"") {
            let rest = &after[at + "Prose::new(\"".len()..];
            faults.push(rest[..rest.find('"').unwrap_or(0)].to_string());
        }
        assert!(
            faults.is_empty(),
            "a word is written into a composition rather than into `mod \
             words`: {faults:#?}"
        );
    }

    /// **Nothing in this piece rebuilds a shape the shared half already
    /// exports** — and the mockup's own copy of the failure shape on this very
    /// page is `.rdfail`, which is one of the five.
    #[test]
    fn no_shared_shape_is_rebuilt_here() {
        let page = the_page();
        for prefix in ["qfail", "svfail", "tffail", "rdfail", "cbad"] {
            assert!(
                !page.contains(&format!("\"{prefix}")),
                "web/render-settings.js writes {prefix}, which is one of the \
                 mockup's five copies of the one failure shape. **Use the shape \
                 that exists** — `theFailureShape` in web/the-failure-shape.js"
            );
        }
        for exported in [
            "theFailureShape",
            "theMarkOnAControl",
            "theMachinesOwnWordsFor",
            "theStripReads",
        ] {
            assert!(
                page.contains(exported),
                "the page does not call {exported}, which the shared half \
                 exports for exactly this drawing"
            );
        }
    }

    /// **The one place this page reaches the diary is M32's second limb, and
    /// it passes the composer's own sentence.**
    ///
    /// **The four read endings are not failure records and raise none** — Q9:
    /// a failed read changes nothing, and `State, identity and recovery` §8.7's
    /// list of durable subjects holds no recorded reading. **What the user met
    /// still reaches the diary** through `Desktop app and delivery` §16.8 M32's
    /// second limb, once per request (the Verdict's upstream must-do; until then this test asserted
    /// the page never reached the capture at all). A screen that passed a
    /// stand-in here would produce no error, no failing test and no visible
    /// fault.
    #[test]
    fn the_diary_is_reached_once_and_with_the_composers_own_sentence() {
        let working = the_working_half();
        assert_eq!(working.matches("WhoseSentenceItIs::TheComposersOwn(").count(), 1);
        assert!(!working.contains("AStandInThePlumbingComposed"));
        assert_eq!(working.matches("the_diary_is_handed_a_sentence_with_no_record(").count(), 1);
    }

    /// **The panel is drawn inside the server's own screen and never measured
    /// against the window.** `.app` has three grid rows whenever Smoozy is
    /// locked, and piece 5's own answer is that the panel is absolute inside
    /// `#machineview`.
    ///
    /// **One exception, and only one: a dropdown's list** — the Creator's
    /// thirteenth notes, note 1. A list with no room below opens upward, which
    /// only a measurement against the window can decide, as the right-click
    /// menus' does. **It lives in `theListIsKeptInsideTheWindow` and nowhere
    /// else**, and the panel is still never measured. *(Three measurements
    /// there from 2026-10-25, the Creator's eighteenth notes, note 11: the
    /// list, its control, and each box around the control that clips it, so
    /// the nearest edge that cuts the list off counts and not the window's
    /// alone.)*
    #[test]
    fn the_page_never_measures_itself_against_the_window() {
        let whole = the_page();
        let (before, rest) = whole
            .split_once("function theListIsKeptInsideTheWindow(")
            .expect("the dropdown's one measurement");
        let (the_exception, after) = rest.split_once("\n}\n").expect("the function ends");
        assert_eq!(the_exception.matches("getBoundingClientRect").count(), 3, "{the_exception}");
        assert!(the_exception.contains("window.innerHeight"), "{the_exception}");
        assert_eq!(whole.matches("theListIsKeptInsideTheWindow(face, list);").count(), 1, "the list is measured from more than its opening");
        let page = format!("{before}{after}");
        for reached in [
            "axlocked",
            "titlebar",
            "getBoundingClientRect",
            "window.inner",
            "documentElement",
        ] {
            assert!(
                !page.contains(reached),
                "web/render-settings.js reaches for {reached}. **The panel \
                 covers the server's own screen and is placed inside it**"
            );
        }
    }

    /// **A handler is given the whole delivery and reads `said` off it.**
    ///
    /// `whenTold` hands over `{ sequence, kind, subject, said }`, and a handler
    /// written against `said` itself reads every key as `undefined` — **with no
    /// error and nothing on the screen to say so.** It cost this piece the
    /// closed submit control and every refusal line before it was found by
    /// looking at the window rather than at a test.
    #[test]
    fn every_handler_reads_the_deliverys_said_and_never_the_delivery() {
        let page = the_page();
        let mut faults = Vec::new();
        for (at, _) in page.match_indices("whenTold(") {
            let rest = &page[at..];
            let line = rest.lines().next().unwrap_or_default();
            if line.contains("(delivery)") || line.contains("(delivery,") {
                continue;
            }
            // A handler taking nothing at all reads nothing and is fine.
            if line.contains("() =>") {
                continue;
            }
            faults.push(line.to_string());
        }
        assert!(
            faults.is_empty(),
            "a handler is written against the event's `said` rather than \
             against the delivery, so every key reads as nothing: {faults:#?}"
        );
    }

    /// **Nothing on this page polls and nothing ticks** — P9 and P10.
    #[test]
    fn nothing_here_repeats_on_a_timer() {
        let page = the_page();
        for started in ["setInterval", "setTimeout", "requestAnimationFrame"] {
            assert!(
                !page.contains(started),
                "web/render-settings.js starts {started}. **The one standing \
                 timer this application is allowed is the update check, and it \
                 is not built**"
            );
        }
    }

    /// **The mockup's own faults are not inherited.** 88 inline `style=`
    /// attributes and four uses of `innerHTML`, every one inert under this
    /// application's content policy.
    #[test]
    fn the_page_writes_no_markup_and_no_inline_style() {
        // **The comment lines are dropped first**, because the head of that
        // file names each of these in order to forbid it — the same reading the
        // shared half's own page sweep takes.
        let page: String = the_page()
            .lines()
            .filter(|line| {
                let line = line.trim_start();
                !(line.starts_with("//") || line.starts_with('*') || line.starts_with("/*"))
            })
            .collect::<Vec<_>>()
            .join("\n");
        for forbidden in ["innerHTML", "outerHTML", "insertAdjacentHTML", ".style."] {
            assert!(
                !page.contains(forbidden),
                "web/render-settings.js writes {forbidden}, which this \
                 application's content policy makes inert — silently"
            );
        }
    }

    /// **The store is process memory for the life of the run and nothing here
    /// writes it to disk** — J2, and `State, identity and recovery` §1.1's
    /// *only durable store* sentence.
    #[test]
    fn nothing_here_reaches_a_durable_store_but_the_one_suppression() {
        let source = the_working_half();
        // The one write there is: `Rendering` W4a's suppression, through the
        // shared service that checks the declaration first (must-do 2).
        assert_eq!(
            source.matches("a_warning_was_switched_off(").count(),
            1,
            "this piece writes to the record somewhere else"
        );
        assert!(
            !source.contains("the_user_switched_it_off"),
            "this piece writes the suppression row directly again, past the declaration check"
        );
        // **Submitting writes the job record since step 10c piece 6**, and it
        // does so in `crate::submitting` through `src/record/job_records.rs`
        // — never from this file, which still holds no row of its own.
        for reached in ["schema::", "migrations::", "INSERT ", "UPDATE ", "DELETE "] {
            assert!(
                !source.contains(reached),
                "this piece reaches the local record directly at {reached}. \
                 **The override store is volatile** (J2) and there is no \
                 durable row for anything on this page"
            );
        }
    }

    // ── The four gaps the screens gate found: R1 and R2 ─────────────────────

    fn the_words_of(sentence: &ASentence) -> Vec<String> {
        sentence
            .iter()
            .flat_map(|part| {
                let mut out = vec![part.lead.as_str().to_string()];
                out.extend(part.values.iter().map(|value| value.as_str().to_string()));
                out.extend(part.rest.iter().map(|rest| rest.as_str().to_string()));
                out
            })
            .filter(|one| !one.is_empty())
            // The typography point's hard space is the point's, not the words'.
            .map(|one| one.replace('\u{a0}', " "))
            .collect()
    }

    /// `Specs/Download`: a fresh page downloads nothing until the user turns
    /// it on, whether or not a reading or a standing folder exists.
    #[test]
    fn auto_download_stays_off_until_the_user_turns_it_on() {
        let file = "/ephemeral/smoozy/assets/kitchen_v14.blend";
        let record = WhatTheRecordHolds {
            the_standing_folder: Some(b"/Users/artist/Renders/kitchen".to_vec()),
            ..WhatTheRecordHolds::default()
        };
        assert!(!TheSubmissionChoices::default().download_as_it_renders);
        for mut held in [WhatTheRunHolds::default(), a_run_with_a_reading()] {
            let panel = the_panel_starting_from(&held, &a_server(), file, &record);
            assert!(!panel.the_download_is_on);
            assert!(panel.where_they_go.is_none() && panel.the_format_summary.is_none());
            assert!(panel.the_format_list_is_not_offered.is_none());
            assert!(panel.the_frames_have_nowhere_to_go.is_none());
            let _ = the_answer_is_taken(&mut held, &a_server(), file, WhichSetting::DownloadAsTheyRender, "on");
            let panel = the_panel_starting_from(&held, &a_server(), file, &record);
            assert!(panel.the_download_is_on);
            assert!(panel.where_they_go.is_some() || panel.the_format_summary.is_some());
            assert!(!the_panel_starting_from(&held, &a_server(), "/srv/another.blend", &record).the_download_is_on);
            let _ = the_answer_is_taken(&mut held, &a_server(), file, WhichSetting::DownloadAsTheyRender, "off");
            let panel = the_panel_starting_from(&held, &a_server(), file, &record);
            assert!(!panel.the_download_is_on);
            assert!(panel.where_they_go.is_none() && panel.the_format_summary.is_none());
        }
        let given = the_act_as_the_page_composed_it(
            &a_run_with_a_reading(), &a_server(), file, &record, &a_row_on(&a_server(), true), a_run(),
        ).unwrap_or_else(|said| panic!("{said:?}"));
        assert!(!given.the_download_as_it_renders_choice);
    }

    /// Fresh drawing states start off too; its download examples and its
    /// switch reveal the same choices the page offers after opting in.
    #[test]
    fn the_drawings_download_choices_follow_its_switch() {
        let drawing = the_mockup();
        assert!(drawing.contains("class=\"sws\" id=\"downloadsw\""));
        let start = drawing.find("  function setAutoDownload(").expect("the switch's drawing");
        let end = start + drawing[start..].find("  function setRead(").expect("the next function");
        let selection = drawing.lines().find(|line| line.trim_start().starts_with("setAutoDownload(v"))
            .expect("each drawing state resets its switch");
        let script = r#"
const elements = Object.fromEntries(["downloadsw", "pickfmt", "downloadnote", "folder", "warning"].map(id => [id, {}]));
let on;
elements.downloadsw.classList = { toggle(_name, value) { on = value; } };
globalThis.document = {
  getElementById(id) { return elements[id]; },
  querySelector(selector) { return elements[selector.includes("sfl1") ? "folder" : "warning"]; }
};
FUNCTION
function select(v) { SELECT }
function check(condition) { if (!condition) throw new Error("The download drawing disagrees with its switch"); }
for (const v of ["res-foot", "res-noread", "res-lost", "res-again", "res-nosrv", "res-carry", "res-carryno"]) {
  select("res-nofolder");
  select(v);
  check(!on && elements.pickfmt.hidden && elements.folder.hidden && elements.downloadnote.hidden && elements.warning.hidden);
}
for (const v of ["fmt", "fmtone", "res-nofolder"]) {
  select(v);
  check(on && !elements.pickfmt.hidden && elements.folder.hidden && elements.downloadnote.hidden);
  check(elements.warning.hidden === (v !== "res-nofolder"));
}
select("fmtnone");
check(on && elements.pickfmt.hidden && !elements.folder.hidden && !elements.downloadnote.hidden);
setAutoDownload(true, "res-noread");
check(on && !elements.folder.hidden && !elements.downloadnote.hidden);
setAutoDownload(false, "res-noread");
check(!on && elements.folder.hidden && elements.downloadnote.hidden);
setAutoDownload(true, "res-foot");
check(on && !elements.pickfmt.hidden && elements.folder.hidden);
"#.replace("FUNCTION", &drawing[start..end]).replace("SELECT", selection);
        let ran = std::process::Command::new("node").arg("-e").arg(script).output();
        let Ok(ran) = ran else {
            eprintln!("SKIPPED: no Node on this computer, so the download drawing was not run");
            return;
        };
        assert!(ran.status.success(), "{}", String::from_utf8_lossy(&ran.stderr));
    }

    /// **R1, before a read: where the files go is still chosen** —
    /// `Design/Render Settings § Submitting`. With the switch turned on and
    /// nothing read, the control says where the files go; the summary is
    /// not, because there is no format list to summarise.
    #[test]
    fn before_a_read_the_control_says_where_the_files_go() {
        let file = "/ephemeral/smoozy/assets/kitchen_v14.blend";
        let mut held = WhatTheRunHolds::default();
        let _ = the_answer_is_taken(&mut held, &a_server(), file, WhichSetting::DownloadAsTheyRender, "on");
        let panel = the_panel(&held, &a_server(), file);
        assert!(
            panel.where_they_go.is_some(),
            "the control that chooses where a job's downloads go is not drawn"
        );
        assert!(panel.the_format_summary.is_none());
        assert!(panel.the_format_list_is_not_offered.is_some());

        let record = WhatTheRecordHolds {
            the_standing_folder: Some(b"/Users/artist/Renders/kitchen".to_vec()),
            the_proposed_folder: Some(b"/Users/artist/Downloads".to_vec()),
            ..WhatTheRecordHolds::default()
        };
        let panel = the_panel_starting_from(&held, &a_server(), file, &record);
        assert_eq!(
            the_words_of(panel.where_they_go.as_ref().expect("the control")),
            vec![words::WHERE_THEY_GO, words::THE_DASH, "/Users/artist/Renders/kitchen"],
            "the standing folder is the one named, ahead of the proposal"
        );
    }

    /// **R1, after a read: one control carries the summary** — *EXR and PNG —
    /// 600 of 720 files, to ~/Renders/kitchen*, and *240 EXR files, to …* on a
    /// job with one format. **Every name, figure and folder is a value** (X7).
    #[test]
    fn after_a_read_the_control_summarises_the_formats_the_count_and_the_folder() {
        let a_format = |name: &str, files: usize, chosen: bool| AFormatKnown {
            name: name.to_string(),
            where_in_the_job: String::new(),
            at_the_top: false,
            files,
            chosen,
        };
        let several = WhatTheBoxKnows {
            formats: Some(vec![a_format("EXR", 480, true), a_format("PNG", 120, true), a_format("JPEG", 120, false)]),
            files_in_the_job: 720,
            the_standing_folder: None,
            the_formats_chosen: None,
            the_folder_chosen_for_this_job: None,
        };
        let (summary, where_alone) = the_download_summary(&several, Some("~/Renders/kitchen"), true);
        assert!(where_alone.is_none());
        let summary = summary.expect("a summary");
        assert_eq!(
            the_words_of(&summary),
            vec!["EXR", words::AND, "PNG", words::THE_DASH, "600", words::OF, "720", words::FILES_AND_A_COMMA, words::TO_FOLDER, "~/Renders/kitchen"]
        );
        for part in &summary {
            for value in &part.values {
                assert!(
                    !part.lead.as_str().contains(value.as_str()),
                    "a value was joined into the words"
                );
            }
        }

        let one = WhatTheBoxKnows {
            formats: Some(vec![a_format("EXR", 240, true)]),
            files_in_the_job: 240,
            the_standing_folder: None,
            the_formats_chosen: None,
            the_folder_chosen_for_this_job: None,
        };
        let (summary, _) = the_download_summary(&one, Some("~/Renders/kitchen"), true);
        assert_eq!(
            the_words_of(&summary.expect("a summary")),
            // One value, *240 EXR*, joined by U+00A0 — which this reading
            // shows as a space, like the point's own.
            vec!["240 EXR", words::FILES_AND_A_COMMA, words::TO_FOLDER, "~/Renders/kitchen"]
        );

        // No reading: the folder alone, whatever the box would otherwise hold.
        let (summary, where_alone) = the_download_summary(&several, None, false);
        assert!(summary.is_none());
        assert_eq!(the_words_of(&where_alone.expect("the folder control")), vec![words::WHERE_THEY_GO]);
    }

    /// **R1: the box's Done keeps the folder on the submission, and only a
    /// folder the box offered.** A format a file that was never read cannot
    /// offer is refused, and so is a folder nobody named.
    #[test]
    fn the_boxs_done_keeps_only_what_the_box_offered() {
        let file = "/ephemeral/smoozy/assets/kitchen_v14.blend";
        let record = WhatTheRecordHolds {
            the_standing_folder: Some(b"/Users/artist/Renders/kitchen".to_vec()),
            the_proposed_folder: Some(b"/Users/artist/Downloads".to_vec()),
            ..WhatTheRecordHolds::default()
        };
        let mut held = WhatTheRunHolds::default();
        // The box offers the standing folder, then §8's proposal.
        let offered = the_folders_offered(
            &what_the_format_box_knows(None, &TheSubmissionChoices::default(), &record, &TheGroundOfTheCount::default()),
            &record.the_proposal(),
        );
        assert_eq!(
            offered,
            vec![
                (b"/Users/artist/Renders/kitchen".to_vec(), false),
                (b"/Users/artist/Downloads".to_vec(), true)
            ]
        );
        // A row the box never offered.
        assert!(!the_download_choices_are_taken(
            &mut held,
            &a_server(),
            file,
            &[],
            TheFolderTaken::Offered(offered.len()),
            &record
        ));
        assert!(!the_download_choices_are_taken(
            &mut held,
            &a_server(),
            file,
            &["EXR".to_string()],
            TheFolderTaken::Offered(1),
            &record
        ));
        assert!(held.submission.is_empty(), "a refused Done changed nothing");

        let _ = the_answer_is_taken(&mut held, &a_server(), file, WhichSetting::DownloadAsTheyRender, "on");
        assert!(the_download_choices_are_taken(
            &mut held,
            &a_server(),
            file,
            &[],
            TheFolderTaken::Offered(1),
            &record
        ));
        let panel = the_panel_starting_from(&held, &a_server(), file, &record);
        assert_eq!(
            the_words_of(panel.where_they_go.as_ref().expect("the control")),
            vec![words::WHERE_THEY_GO, words::THE_DASH, "/Users/artist/Downloads"],
            "the folder chosen for this job is the one named"
        );
        // The box opens on it, and still offers the standing folder.
        let TheFormatAnswer::TheBox(the_box) = the_download_panel_from(&held, &a_server(), file, &record) else {
            panic!("a box");
        };
        let offered: Vec<&str> = the_box.folders.iter().map(|one| one.folder.as_str()).collect();
        assert_eq!(offered, vec!["/Users/artist/Downloads", "/Users/artist/Renders/kitchen"]);

        // A folder from the system's picker is held as the user gave it.
        assert!(the_download_choices_are_taken(
            &mut held,
            &a_server(),
            file,
            &[],
            TheFolderTaken::FromThePicker(b"/Volumes/Renders/kitchen".to_vec()),
            &record
        ));
        let panel = the_panel_starting_from(&held, &a_server(), file, &record);
        assert_eq!(
            the_words_of(panel.where_they_go.as_ref().expect("the control")).last().map(String::as_str),
            Some("/Volumes/Renders/kitchen")
        );

        // With the switch off nothing downloads, so there is nothing to choose.
        let _ = the_answer_is_taken(&mut held, &a_server(), file, WhichSetting::DownloadAsTheyRender, "off");
        let panel = the_panel_starting_from(&held, &a_server(), file, &record);
        assert!(panel.where_they_go.is_none() && panel.the_format_summary.is_none());
    }

    /// **`§ Submitting`: the switch on, no folder anybody can name, the job
    /// still submitted, and the strip saying so** — the drawing's
    /// `res-nofolder`.
    ///
    /// **This is the silence the state was drawn for.** All three sources of
    /// the folder are empty at once — nothing chosen for this job, no standing
    /// folder, and `Desktop app and delivery` W5's route proposing none — so
    /// `Download` R1's arrangement is refused with *there is nowhere to put
    /// it*, and until 2026-09-20 nothing anywhere said it: the job went to the
    /// machine with the switch on and its frames were never coming home.
    #[test]
    fn a_job_whose_frames_have_nowhere_to_go_is_still_sent_and_the_strip_says_so() {
        let file = "/ephemeral/smoozy/assets/kitchen_v14.blend";
        let mut held = a_run_with_a_reading();
        let _ = the_answer_is_taken(&mut held, &a_server(), file, WhichSetting::DownloadAsTheyRender, "on");
        // A record that answers nothing, which is also what a record that
        // would not read holds: no standing folder, and no proposal.
        let record = WhatTheRecordHolds::default();
        assert!(
            the_folder_named(&TheSubmissionChoices::default(), &record).is_none(),
            "nothing can name a folder, which is the state under test"
        );

        // **The telling**, on the strip, before the job goes.
        let panel = the_panel_starting_from(&held, &a_server(), file, &record);
        let line = panel
            .the_frames_have_nowhere_to_go
            .as_ref()
            .expect("the strip says nothing about frames that cannot come home");
        assert_eq!(line.lead, Prose::new(words::NO_FOLDER_IS_CHOSEN));
        assert_eq!(line.rest, Prose::new(words::THE_FRAMES_STAY_ON_THE_SERVER));

        // **And the job still goes.** The act composes, with the switch on and
        // no destination, which is exactly `Download` R1's refusal.
        let given = the_act_as_the_page_composed_it(
            &held,
            &a_server(),
            file,
            &record,
            &a_row_on(&a_server(), true),
            a_run(),
        )
        .unwrap_or_else(|said| panic!("{said:?}"));
        assert!(given.the_download_as_it_renders_choice);
        assert!(
            given.where_the_downloads_go.is_none(),
            "the act would otherwise carry a folder nobody named"
        );
        assert_eq!(given.scene_jobs.len(), 1, "the submission still goes");
        // **`Download` R1's own maker, asked with what the act hands over.**
        // The keeper refuses every call, so this also shows the refusal is
        // taken before anything is written down.
        use smoozy_download::arrangement::{AnArrangement, KeepsTheArrangement, WhatEndsIt};
        use smoozy_state::outcomes::Refused;
        struct NothingIsWrittenDown;
        impl KeepsTheArrangement for NothingIsWrittenDown {
            fn write_it_down(&mut self, _made: &AnArrangement) -> Result<(), Refused> {
                panic!("an arrangement with nowhere to put it was written down")
            }
            fn the_arrangement_for(
                &self,
                _job: &Identity,
            ) -> Result<Option<AnArrangement>, Refused> {
                Ok(None)
            }
            fn it_ended(&mut self, _job: &Identity, _how: WhatEndsIt) -> Result<(), Refused> {
                Ok(())
            }
        }
        assert!(
            matches!(
                crate::downloading::the_arrangement_is_made_at_submission(
                    &mut NothingIsWrittenDown,
                    given.the_download_as_it_renders_choice,
                    crate::downloading::WhatTheSubmissionChose {
                        job: Identity::issue(IdentityKind::Job).expect("randomness"),
                        server_row: given.server.local_server.clone(),
                        destination: given.where_the_downloads_go.clone().unwrap_or_default(),
                        narrowing: smoozy_download::arrangement::TheNarrowing::Everything,
                    },
                ),
                Err(crate::downloading::WhyNoArrangementWasMade::ThereIsNowhereToPutIt(_))
            ),
            "the line on the strip is saying something that is not true"
        );

        // **Where a folder can be named there is nothing to say**, and §8's
        // proposal alone is enough to name one.
        let proposed = WhatTheRecordHolds {
            the_proposed_folder: Some(b"/Users/artist/Downloads".to_vec()),
            ..WhatTheRecordHolds::default()
        };
        assert!(the_panel_starting_from(&held, &a_server(), file, &proposed)
            .the_frames_have_nowhere_to_go
            .is_none());

        // **And with the switch off nothing downloads by arrangement anyway**,
        // so there is nothing to warn about.
        let mut off = a_run_with_a_reading();
        let _ = the_answer_is_taken(
            &mut off,
            &a_server(),
            file,
            WhichSetting::DownloadAsTheyRender,
            "off",
        );
        assert!(the_panel_starting_from(&off, &a_server(), file, &record)
            .the_frames_have_nowhere_to_go
            .is_none());

        // **And the page draws it**, in the drawing's own amber line.
        let page = the_page();
        assert!(
            page.contains("said.the_frames_have_nowhere_to_go"),
            "web/render-settings.js never reads the line, so nothing reaches the screen"
        );
        assert!(
            page.contains("\"adwarn\""),
            "the line is not drawn in the drawing's amber"
        );
    }

    /// **R1 on the page: the control opens piece 12's box for this file and
    /// its Done comes back here.**
    #[test]
    fn the_page_draws_the_control_and_opens_the_box_from_it() {
        let page = the_page();
        assert!(
            page.contains("theFormatBoxIsOpenedForAFile("),
            "web/render-settings.js does not open the format box"
        );
        assert!(page.contains("\"the_download_choices_are_made\""));
        assert!(
            page.contains("said.the_format_summary || said.where_they_go"),
            "the one control is not drawn from the summary or the folder"
        );
        let registered = include_str!("mod.rs");
        for command in ["render_settings::the_download_panel,", "render_settings::the_download_choices_are_made,"] {
            assert!(registered.contains(command), "{command} is not registered");
        }
        let transfers = std::fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("web/transfers.js"))
            .expect("web/transfers.js");
        assert!(transfers.contains("export async function theFormatBoxIsOpenedForAFile("));
        assert!(transfers.contains("\"the_download_panel\""));
    }

    /// **R2: *"Where Telegram is switched off it is not there at all."*** The
    /// record says which, and a record that says nothing is read as off.
    #[test]
    fn the_telegram_line_is_there_only_where_telegram_is_on() {
        let file = "/ephemeral/smoozy/assets/kitchen_v14.blend";
        let off = the_panel(&WhatTheRunHolds::default(), &a_server(), file);
        assert!(!off.the_telegram_line_is_drawn);
        let on = WhatTheRecordHolds {
            telegram_is_on: true,
            ..WhatTheRecordHolds::default()
        };
        let on = the_panel_starting_from(&WhatTheRunHolds::default(), &a_server(), file, &on);
        assert!(on.the_telegram_line_is_drawn);
        assert!(
            the_page().contains("if (said.the_telegram_line_is_drawn)"),
            "web/render-settings.js draws the Telegram line whatever the switch says"
        );
    }

    /// **R2 against a real record**: the switch as `Settings` writes it is the
    /// switch this page reads.
    #[test]
    fn the_switch_is_read_from_the_record_settings_writes() {
        let folder = std::env::temp_dir().join(format!("smoozy-r2-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&folder);
        std::fs::create_dir_all(&folder).expect("a folder");
        let record = crate::record::LocalRecord::open_at(&folder.join("local-record.sqlite3")).expect("a record");
        assert!(!WhatTheRecordHolds::read_now(Some(&record)).telegram_is_on);
        crate::record::settings::telegram_was_switched(record.connection(), true).expect("switched on");
        assert!(WhatTheRecordHolds::read_now(Some(&record)).telegram_is_on);
        crate::record::settings::telegram_was_switched(record.connection(), false).expect("switched off");
        assert!(!WhatTheRecordHolds::read_now(Some(&record)).telegram_is_on);
        drop(record);
        let _ = std::fs::remove_dir_all(&folder);
    }

    /// **Text being typed is not lost to a redraw** — `Design/Whole app § The
    /// window`, and the fault step 10c's ninth pass met on this page: a job
    /// name typed into *Name this job* was gone 0.3 s later with the caret
    /// still in the field, because an answer arrived while it was typed and the
    /// page drew the field again from the last committed answer.
    ///
    /// **The rule is kept once, in `web/the-typed-text.js`**, and this runs
    /// that page code under Node beside a stand-in for the little of a document
    /// it reads: the typing and the caret come back into the field drawn in its
    /// place; the field taken out commits nothing; the typing put back still
    /// commits, once, on Enter or on being left; the window going behind
    /// another commits nothing; and a field whose own answer went is drawn from
    /// the answer. **Before 2026-09-28 the file did not exist**, and the page
    /// harness showed every case failing on this page, on Settings and on
    /// History (step 10c's ninth-pass fix piece 5, whose account holds the
    /// harness's results). **Skipped, and says so, where no Node is
    /// installed.**
    #[test]
    fn typed_text_and_its_caret_survive_a_redraw_and_still_commit_once() {
        let web = Path::new(env!("CARGO_MANIFEST_DIR")).join("web");
        for page in ["render-settings.js", "settings.js", "the-history.js"] {
            let text = fs::read_to_string(web.join(page)).expect("the page is readable");
            assert!(
                text.contains("from \"./the-typed-text.js\""),
                "web/{page} draws a text field again and does not keep what is typed in it"
            );
        }
        assert!(
            the_page().contains("theTypingIsTakenFrom(held.body)"),
            "web/render-settings.js no longer takes the typing before the page is drawn again"
        );
        let folder = std::env::temp_dir().join(format!("smoozy-typed-text-harness-{}", std::process::id()));
        let _ = fs::remove_dir_all(&folder);
        fs::create_dir_all(&folder).expect("a scratch folder");
        fs::copy(web.join("the-typed-text.js"), folder.join("the-typed-text.mjs"))
            .expect("the page module is copied");
        fs::write(folder.join("harness.mjs"), THE_TYPED_TEXT_HARNESS).expect("the harness is written");
        let ran = std::process::Command::new("node")
            .arg(folder.join("harness.mjs"))
            .current_dir(&folder)
            .output();
        let _ = fs::remove_dir_all(&folder);
        let Ok(ran) = ran else {
            eprintln!("SKIPPED: no Node on this computer, so the typed-text page code was not run");
            return;
        };
        assert!(
            ran.status.success(),
            "the page code did not run: {}",
            String::from_utf8_lossy(&ran.stderr)
        );
        assert_eq!(
            String::from_utf8_lossy(&ran.stdout).trim(),
            r#"{"typingIsMarked":true,"aFieldTakenOutCommitsNothing":true,"theTypingComesBack":true,"itIsStillBeingTypedIn":true,"enterCommitsItOnce":true,"leavingCommitsItOnce":true,"theWindowGoingBehindCommitsNothing":true,"aKeyAfterLeavesTheCommitToTheEngine":true,"anAnsweredFieldIsDrawnFromTheAnswer":true,"nothingIsTakenWhereTheFocusIsNot":true}"#
        );
    }

    /// **A click into a value box chooses its whole text** — `Design/Render
    /// Settings`, beside the value boxes: *"A click into a value box — the
    /// frames, the samples, the resolution, Simplify's figures — chooses its
    /// whole text, so what is typed replaces it; a second click places the
    /// caret."* A redraw that gives the box its focus back chooses nothing.
    /// The Creator's fifth notes, note 5.
    ///
    /// This runs the page itself under Node, with `web/the-typed-text.js` as it
    /// is, and clicks the way an engine does: the press, and — unless the page
    /// stopped it — the focus and the caret where the pointer is. The samples,
    /// the resolution and the range are each clicked twice; the range, focused
    /// with its caret placed, is drawn again; the samples are typed over and
    /// drawn again; and the job name is clicked. **Before 2026-10-08 the first
    /// click on each value box placed the caret and chose nothing**, so this
    /// failed on all three. **Skipped, and says so, where no Node is
    /// installed.**
    #[test]
    fn a_click_into_a_value_box_chooses_its_whole_text_and_a_redraw_chooses_nothing() {
        let panel = the_panel(&a_run_with_a_reading(), &a_server(), "/ephemeral/smoozy/assets/kitchen_v14.blend");
        for (name, field) in [("samples", &panel.samples), ("resolution", &panel.resolution), ("range", &panel.the_range)] {
            assert!(
                !field.greyed && field.value.as_ref().is_some_and(|value| value.as_str().chars().count() > 1),
                "the {name} box of the panel this runs on holds no value of two characters or more"
            );
        }
        let answers = serde_json::json!({ "panel": panel });
        let folder = std::env::temp_dir().join(format!("smoozy-a-click-chooses-page-{}", std::process::id()));
        let _ = fs::remove_dir_all(&folder);
        fs::create_dir_all(&folder).expect("a scratch folder");
        let web = Path::new(env!("CARGO_MANIFEST_DIR")).join("web");
        for (name, text) in THE_UPLOADING_LINES_STAND_INS {
            fs::write(folder.join(name), text).expect("a stand-in");
        }
        fs::copy(web.join("render-settings.js"), folder.join("render-settings.js")).expect("the page");
        fs::copy(web.join("the-typed-text.js"), folder.join("the-typed-text.js")).expect("the typed text");
        fs::write(folder.join("the-click.mjs"), THE_CLICK_STAND_IN).expect("the click");
        fs::write(folder.join("answers.json"), answers.to_string()).expect("the answers");
        fs::write(folder.join("harness.mjs"), THE_CLICK_HARNESS.replace("SERVER", &a_server())).expect("the harness");
        let ran = std::process::Command::new("node").arg(folder.join("harness.mjs")).current_dir(&folder).output();
        let _ = fs::remove_dir_all(&folder);
        let Ok(ran) = ran else {
            eprintln!("SKIPPED: no Node on this computer, so the render settings page was not run");
            return;
        };
        assert!(ran.status.success(), "the page code did not run: {}", String::from_utf8_lossy(&ran.stderr));
        let got: serde_json::Value = serde_json::from_slice(&ran.stdout).expect("the harness prints one answer");
        let both = serde_json::json!({ "theFirstClickChoosesItAll": true, "aSecondClickPlacesTheCaret": true });
        assert_eq!(
            got,
            serde_json::json!({
                "samples": both,
                "resolution": both,
                "range": both,
                "aRedrawsFocusReturnChoosesNothing": true,
                "typingSurvivesARedraw": true,
                "theJobNameIsLeftAlone": true
            })
        );
    }

    /// **What the click harness adds to the stand-in document**: listeners on
    /// the document that run first, as the capturing ones do; focus that moves
    /// with its own events; a field's chosen text; finding a box by its
    /// `data-box`; and whether a piece is still in the page.
    const THE_CLICK_STAND_IN: &str = r#"import { everyPiece } from "./document.mjs";
const piece = Object.getPrototypeOf(document.createElement("div"));
const first = {};
document.addEventListener = (kind, handler) => { (first[kind] ||= []).push(handler); };
export function fire(target, kind, extra) {
  let stopped = false;
  const event = Object.assign({
    type: kind, target, button: 0, defaultPrevented: false,
    preventDefault() { this.defaultPrevented = true; }, stopImmediatePropagation() { stopped = true; },
  }, extra || {});
  for (const handler of first[kind] || []) { handler(event); if (stopped) return event; }
  for (const handler of (target.listeners && target.listeners[kind]) || []) handler(event);
  return event;
}
piece.focus = function () {
  const was = document.activeElement;
  if (was === this) return;
  document.activeElement = this;
  if (was) fire(was, "focusout");
  fire(this, "focus");
};
piece.setSelectionRange = function (start, end, direction) {
  this.selectionStart = start; this.selectionEnd = end; this.selectionDirection = direction || "none";
};
piece.select = function () { this.setSelectionRange(0, String(this.value).length, "none"); };
piece.dispatchEvent = function (event) { fire(this, event.type); return true; };
piece.querySelector = function (selector) {
  const box = /^\[data-box="(.*)"\]$/.exec(selector);
  return box ? everyPiece(this).find((one) => one !== this && one.dataset.box === box[1]) || null : null;
};
Object.defineProperty(piece, "isConnected", {
  get() { for (let at = this; at; at = at.parentNode) if (at.inThePage) return true; return false; },
});
"#;

    /// Opens the panel, clicks each value box twice, draws the page again with
    /// the range focused, types over the samples and draws again, and clicks
    /// the job name.
    const THE_CLICK_HARNESS: &str = r#"import { everyPiece } from "./document.mjs";
import { fire } from "./the-click.mjs";
import { readFileSync } from "node:fs";
const answers = JSON.parse(readFileSync("./answers.json", "utf8"));
globalThis.window = { addEventListener() {}, __TAURI__: { core: { invoke(command) {
  if (command === "the_render_settings_panel") return Promise.resolve(answers.panel);
  return Promise.resolve(null);
} } } };
const { panels } = await import("./a-server.js");
const { theSettingsAreOpenedOn } = await import("./render-settings.js");
const settle = async () => { for (let at = 0; at < 8; at += 1) await new Promise((go) => setImmediate(go)); };
await theSettingsAreOpenedOn("SERVER", Array.from(Buffer.from(answers.panel.file)), "");
const head = document.createElement("div");
const body = document.createElement("div");
head.inThePage = true;
body.inThePage = true;
panels[0].itIsDrawn({ head, body }, "SERVER");
await settle();
const find = (box) => everyPiece(body).find((one) => one.dataset.box === box);
// A click as an engine makes one: unless the page stopped the press, the
// focus comes and the caret goes where the pointer is.
const click = (box, at) => {
  fire(box, "pointerdown");
  const press = fire(box, "mousedown");
  if (!press.defaultPrevented) { box.focus(); box.setSelectionRange(at, at, "none"); }
  fire(box, "pointerup");
  fire(box, "mouseup");
  fire(box, "click");
};
const allChosen = (box) => box.value.length > 1 && box.selectionStart === 0 && box.selectionEnd === box.value.length;
const theCaretAt = (box, at) => box.selectionStart === at && box.selectionEnd === at;
const drawnAgain = async () => {
  for (const handler of globalThis.told["the-dependency-check:an-act"] || []) {
    await handler({ said: { server: "SERVER", file: answers.panel.file } });
  }
  await settle();
};
const out = {};
for (const [name, which] of [["samples", "samples-field"], ["resolution", "resolution-field"], ["range", "range-field"]]) {
  const box = find(which);
  click(box, 1);
  const first = document.activeElement === box && allChosen(box);
  click(box, 1);
  out[name] = { theFirstClickChoosesItAll: first, aSecondClickPlacesTheCaret: document.activeElement === box && theCaretAt(box, 1) };
}
const range = find("range-field");
await drawnAgain();
const rangeAgain = find("range-field");
out.aRedrawsFocusReturnChoosesNothing = rangeAgain !== range && document.activeElement === rangeAgain && theCaretAt(rangeAgain, 1);
const samples = find("samples-field");
click(samples, 1);
const [from, to] = [samples.selectionStart, samples.selectionEnd];
samples.value = samples.value.slice(0, from) + "300" + samples.value.slice(to);
samples.setSelectionRange(from + 3, from + 3, "none");
fire(samples, "input");
await drawnAgain();
const samplesAgain = find("samples-field");
out.typingSurvivesARedraw = samplesAgain !== samples && document.activeElement === samplesAgain && samplesAgain.value === "300" && theCaretAt(samplesAgain, 3);
const name = find("job-name");
click(name, 1);
out.theJobNameIsLeftAlone = document.activeElement === name && theCaretAt(name, 1);
console.log(JSON.stringify(out));
"#;

    // ════════════════════════════════════════════════════════════════════════
    // The Creator's fifth notes, piece U: Smoozy's own dropdowns
    // ════════════════════════════════════════════════════════════════════════

    /// **`Design/Whole app § The window`: *"Smoozy draws its own dropdowns."***
    /// The Creator's fifth notes, note 2. The page, under Node, with three
    /// builds on the server (one below 4.5), Simplify on and Telegram on: no
    /// `select` is left; each dropdown opens the right-click menu's own panel
    /// with the picked row in bold; a mouse pick sends the answer and leaves
    /// the focus nowhere in the control, before and after the redraw; Down, Up
    /// and Return pick from the keyboard and the focus comes back to the
    /// control after the redraw; Escape closes the list first and marks the
    /// press handled; a press elsewhere closes it. **Before 2026-10-08 all
    /// three were `select`s, so this failed at its first line.** Skipped, and
    /// says so, where no Node is installed.
    #[test]
    fn the_pages_dropdowns_are_smoozys_own_and_a_pick_leaves_no_ring() {
        let file = "/ephemeral/smoozy/assets/kitchen_v14.blend";
        let mut held = a_run_with_a_reading();
        let list = held.version_lists.get_mut(&a_server()).expect("the run's list");
        for (major, minor, patch) in [(5, 2, 1), (4, 2, 9)] {
            list.entries.push(smoozy_server_setup::search::ACandidate {
                path: format!("/opt/blender-{major}.{minor}.{patch}/blender"),
                answered: Some(a_version(major, minor, patch)),
            });
        }
        // Simplify is offered only on a page opened from a failed entry.
        the_page_came_from(
            &mut held,
            &a_server(),
            file,
            ThePageCameFrom {
                band: WhereItCameFrom::SubmittingAgain { the_jobs_name: "Kitchen".to_string(), when: String::new() },
                the_simplify_offer_is_drawn: true,
                the_entry: Identity::issue(IdentityKind::Job).expect("randomness"),
                the_entrys_label: "Kitchen".to_string(),
                the_moment: None,
            },
        );
        assert_eq!(the_answer_is_taken(&mut held, &a_server(), file, WhichSetting::Simplify, "on"), None);
        let record = WhatTheRecordHolds { telegram_is_on: true, ..WhatTheRecordHolds::default() };
        let panel = the_panel_starting_from(&held, &a_server(), file, &record);

        // What each dropdown should say, from the panel itself.
        let versions: Vec<String> = panel
            .the_versions
            .iter()
            .map(|one| match &one.below_the_floor {
                Some(below) => format!("{} {}", one.answered.as_str(), below.as_str()),
                None => one.answered.as_str().to_string(),
            })
            .collect();
        assert!(
            versions.iter().any(|one| one.replace('\u{a0}', " ").contains("below 4.5")),
            "the panel this runs on has no build below the floor: {versions:?}"
        );
        let the_pick = panel.the_versions.iter().position(|one| one.picked).expect("a build is picked");
        assert!(the_pick + 2 < versions.len(), "the picked build has no two rows under it");
        let simplify = panel.simplify.as_ref().filter(|one| one.on).expect("Simplify is on");
        let texture = simplify.settings.get(2).expect("the Texture Limit row");
        let textures: Vec<String> = simplify.texture_choices.iter().map(|one| one.said.as_str().to_string()).collect();
        let the_texture = simplify
            .texture_choices
            .iter()
            .position(|one| texture.value.as_ref().is_some_and(|value| value.as_str() == one.value.as_str()));
        let reports: Vec<String> = panel.the_reporting_choices.iter().map(|one| one.said.as_str().to_string()).collect();
        let the_report = panel
            .the_reporting_choices
            .iter()
            .position(|one| one.value.as_str() == panel.the_reporting_choice.as_str())
            .expect("a report is picked");
        let looks = |rows: &[String], picked: Option<usize>, pick: &str| {
            let bold: Vec<&String> = picked.map(|at| &rows[at]).into_iter().collect();
            serde_json::json!({
                "face": "BUTTON", "pick": pick, "list": "ctx ddl", "role": "listbox", "hidden": true,
                "closed": picked.map(|at| rows[at].clone()).unwrap_or_else(|| {
                    texture.value.as_ref().map(|value| value.as_str().to_string()).unwrap_or_else(|| "—".to_string())
                }),
                "rows": rows, "bold": bold, "selected": bold,
            })
        };
        let not_picked = |picked: Option<usize>| if picked == Some(0) { 1 } else { 0 };

        let answers = serde_json::json!({ "panel": panel });
        let folder = std::env::temp_dir().join(format!("smoozy-own-dropdowns-page-{}", std::process::id()));
        let _ = fs::remove_dir_all(&folder);
        fs::create_dir_all(&folder).expect("a scratch folder");
        let web = Path::new(env!("CARGO_MANIFEST_DIR")).join("web");
        for (name, text) in THE_UPLOADING_LINES_STAND_INS {
            fs::write(folder.join(name), text).expect("a stand-in");
        }
        fs::copy(web.join("render-settings.js"), folder.join("render-settings.js")).expect("the page");
        fs::write(folder.join("the-click.mjs"), THE_CLICK_STAND_IN).expect("the click");
        fs::write(folder.join("answers.json"), answers.to_string()).expect("the answers");
        fs::write(folder.join("harness.mjs"), THE_DROPDOWN_HARNESS.replace("SERVER", &a_server())).expect("the harness");
        let ran = std::process::Command::new("node").arg(folder.join("harness.mjs")).current_dir(&folder).output();
        let _ = fs::remove_dir_all(&folder);
        let Ok(ran) = ran else {
            eprintln!("SKIPPED: no Node on this computer, so the render settings page was not run");
            return;
        };
        assert!(ran.status.success(), "the page code did not run: {}", String::from_utf8_lossy(&ran.stderr));
        let got: serde_json::Value = serde_json::from_slice(&ran.stdout).expect("the harness prints one answer");
        let build = |at: usize| panel.the_versions[at].build.as_str().to_string();
        assert_eq!(
            got,
            serde_json::json!({
                "noSelect": true,
                "version": looks(&versions, Some(the_pick), "pick w-ver"),
                "texture": looks(&textures, the_texture, "pick"),
                // Only as wide as its longest choice, by the drawing's own class
                // (the Creator's tenth notes, note 8).
                "reports": looks(&reports, Some(the_report), "pick w-tg"),
                "aMouseOpens": { "open": true, "expanded": "true" },
                "aMousePick": {
                    "closed": true,
                    "sent": [["version-pick", build(not_picked(Some(the_pick)))]],
                    "theFocusIsInIt": false,
                    "afterTheRedraw": false
                },
                "theKeys": {
                    "opensOnThePick": true,
                    "downMoves": true,
                    "upMoves": true,
                    "returnPicks": true,
                    "sent": [["version-pick", build(the_pick + 2)]],
                    "theFocusComesBack": true
                },
                "escape": { "closes": true, "handled": true, "focusOnTheControl": true, "sent": 0 },
                "aPressElsewhereCloses": true,
                "theOtherTwoSend": [
                    ["simplify-texture-limit", simplify.texture_choices[not_picked(the_texture)].value.as_str()],
                    ["what-it-reports", panel.the_reporting_choices[not_picked(Some(the_report))].value.as_str()]
                ]
            })
        );
    }

    /// Draws the page, reads the three dropdowns, then works the version one
    /// by mouse and by key, and picks once from each of the other two.
    const THE_DROPDOWN_HARNESS: &str = r#"import { everyPiece } from "./document.mjs";
import { fire } from "./the-click.mjs";
import { readFileSync } from "node:fs";
const answers = JSON.parse(readFileSync("./answers.json", "utf8"));
const sent = [];
globalThis.window = { addEventListener() {}, __TAURI__: { core: { invoke(command, asked) {
  if (command === "the_render_settings_panel") return Promise.resolve(answers.panel);
  if (command === "an_answer_was_given") sent.push([asked.which, asked.text]);
  return Promise.resolve(null);
} } } };
const piece = Object.getPrototypeOf(document.createElement("div"));
piece.blur = function () { if (document.activeElement === this) { document.activeElement = null; fire(this, "focusout"); } };
const { panels } = await import("./a-server.js");
const { theSettingsAreOpenedOn } = await import("./render-settings.js");
const settle = async () => { for (let at = 0; at < 8; at += 1) await new Promise((go) => setImmediate(go)); };
await theSettingsAreOpenedOn("SERVER", Array.from(Buffer.from(answers.panel.file)), "");
const head = document.createElement("div");
const body = document.createElement("div");
panels[0].itIsDrawn({ head, body }, "SERVER");
await settle();
// An event as an engine sends one: the capturing listeners on the document,
// then the target, then each piece it sits in.
const bubble = (target, kind, extra) => {
  const event = fire(target, kind, extra);
  for (let at = target.parentNode; at; at = at.parentNode) for (const handler of (at.listeners && at.listeners[kind]) || []) handler(event);
  return event;
};
const mouse = (target) => { bubble(target, "pointerdown"); bubble(target, "mousedown"); bubble(target, "pointerup"); bubble(target, "click", { detail: 1 }); };
const key = (target, name) => {
  const event = bubble(target, "keydown", { key: name });
  if (!event.defaultPrevented && (name === "Enter" || name === " ") && target.tagName === "BUTTON") bubble(target, "click", { detail: 0 });
  return event;
};
const drawnAgain = async (theBox) => {
  for (const handler of globalThis.told["render-settings:an-act"] || []) await handler({ said: { server: "SERVER", file: answers.panel.file, theBox } });
  await settle();
};
const all = () => [...everyPiece(head), ...everyPiece(body)];
const box = (name) => all().find((one) => one.dataset.box === name);
const listOf = (face) => face.parentNode.children.find((one) => one.names && one.names.has("ddl"));
const rowsOf = (face) => listOf(face).children;
const look = (face) => {
  const list = listOf(face);
  const rows = list ? list.children : [];
  return {
    face: face.tagName, closed: face.textContent, pick: face.parentNode.className,
    list: list ? [...list.names].sort().join(" ") : null, role: list ? list.getAttribute("role") : null, hidden: list ? list.hidden : null,
    rows: rows.map((one) => one.textContent),
    bold: rows.filter((one) => one.names.has("hd")).map((one) => one.textContent),
    selected: rows.filter((one) => one.getAttribute("aria-selected") === "true").map((one) => one.textContent),
  };
};
const out = { noSelect: !all().some((one) => one.tagName === "SELECT") };
out.version = look(box("version"));
out.texture = look(box("simplify-texture-limit-field"));
out.reports = look(box("reports"));
const notPicked = (face) => rowsOf(face).find((one) => !one.names.has("hd"));

let face = box("version");
mouse(face);
out.aMouseOpens = { open: !listOf(face).hidden, expanded: face.getAttribute("aria-expanded") };
mouse(notPicked(face));
out.aMousePick = { closed: listOf(face).hidden, sent: sent.splice(0), theFocusIsInIt: face.parentNode.contains(document.activeElement) };
await drawnAgain("version");
out.aMousePick.afterTheRedraw = document.activeElement === box("version");

face = box("version");
face.focus();
key(face, "ArrowDown");
const list = listOf(face);
const rows = list.children;
const at = rows.findIndex((one) => one.names.has("hd"));
const keys = { opensOnThePick: !list.hidden && document.activeElement === rows[at] };
key(document.activeElement, "ArrowDown");
keys.downMoves = document.activeElement === rows[at + 1];
key(document.activeElement, "ArrowUp");
keys.upMoves = document.activeElement === rows[at];
key(document.activeElement, "ArrowDown");
key(document.activeElement, "ArrowDown");
key(document.activeElement, "Enter");
keys.returnPicks = list.hidden && document.activeElement === face;
keys.sent = sent.splice(0);
await drawnAgain("version");
keys.theFocusComesBack = box("version") !== face && document.activeElement === box("version");
out.theKeys = keys;

face = box("version");
key(face, "ArrowDown");
const escape = key(document.activeElement, "Escape");
out.escape = { closes: listOf(face).hidden, handled: escape.defaultPrevented, focusOnTheControl: document.activeElement === face, sent: sent.splice(0).length };

mouse(face);
bubble(body, "pointerdown");
out.aPressElsewhereCloses = listOf(face).hidden;

for (const name of ["simplify-texture-limit-field", "reports"]) {
  const other = box(name);
  mouse(other);
  mouse(notPicked(other));
}
out.theOtherTwoSend = sent.splice(0);
console.log(JSON.stringify(out));
"#;

    // ════════════════════════════════════════════════════════════════════════
    // The Creator's thirteenth notes, note 1: a dropdown at the foot
    // ════════════════════════════════════════════════════════════════════════

    /// **A panel with Telegram on, so *What it reports* is drawn in the foot.**
    fn a_panel_with_the_reports_dropdown() -> serde_json::Value {
        let panel = the_panel_starting_from(
            &a_run_with_a_reading(),
            &a_server(),
            "/ephemeral/smoozy/assets/kitchen_v14.blend",
            &WhatTheRecordHolds { telegram_is_on: true, ..WhatTheRecordHolds::default() },
        );
        assert!(panel.the_telegram_line_is_drawn, "the panel draws no Telegram dropdown");
        serde_json::to_value(&panel).expect("the panel serialises")
    }

    /// **The Creator's thirteenth notes, note 1: a dropdown whose list would
    /// pass the window's bottom opens upward** where there is more room above
    /// it — `Design/Whole app`: *"A pressed one opens a list drawn as the
    /// right-click menu is"*, and that menu is kept inside the window. *What it
    /// reports* sits in the foot, and until then its list always opened
    /// downward, past the window's edge.
    ///
    /// Under Node, with the control and its list given places: at the foot the
    /// list carries `up`; opened again with room below it does not; and where
    /// it fits neither way it stays below if there is more room there. **It
    /// fails if the list opens downward at the foot**, or keeps `up` from an
    /// earlier opening. Skipped, and says so, where no Node is installed.
    #[test]
    fn a_dropdown_at_the_windows_foot_opens_upward() {
        let answers = serde_json::json!({ "panel": a_panel_with_the_reports_dropdown() });
        let folder = std::env::temp_dir().join(format!("smoozy-dropdown-upward-page-{}", std::process::id()));
        let _ = fs::remove_dir_all(&folder);
        fs::create_dir_all(&folder).expect("a scratch folder");
        let web = Path::new(env!("CARGO_MANIFEST_DIR")).join("web");
        for (name, text) in THE_UPLOADING_LINES_STAND_INS {
            fs::write(folder.join(name), text).expect("a stand-in");
        }
        fs::copy(web.join("render-settings.js"), folder.join("render-settings.js")).expect("the page");
        fs::write(folder.join("answers.json"), answers.to_string()).expect("the answers");
        fs::write(folder.join("harness.mjs"), THE_UPWARD_HARNESS.replace("SERVER", &a_server())).expect("the harness");
        let ran = std::process::Command::new("node").arg(folder.join("harness.mjs")).current_dir(&folder).output();
        let _ = fs::remove_dir_all(&folder);
        let Ok(ran) = ran else {
            eprintln!("SKIPPED: no Node on this computer, so the render settings page was not run");
            return;
        };
        assert!(ran.status.success(), "the page code did not run: {}", String::from_utf8_lossy(&ran.stderr));
        let got: serde_json::Value = serde_json::from_slice(&ran.stdout).expect("the harness prints one answer");
        assert_eq!(
            got,
            serde_json::json!({
                "atTheFoot": { "open": true, "up": true },
                "withRoomBelow": { "open": true, "up": false },
                "moreRoomBelowThanAbove": { "open": true, "up": false },
                "atTheFootAgain": { "open": true, "up": true }
            }),
            "a dropdown at the window's foot opened downward"
        );
    }

    /// Draws the page, then opens *What it reports* with its control and list
    /// placed in a window of the height given.
    const THE_UPWARD_HARNESS: &str = r#"import { everyPiece } from "./document.mjs";
import { readFileSync } from "node:fs";
const answers = JSON.parse(readFileSync("./answers.json", "utf8"));
globalThis.window = { innerHeight: 800, addEventListener() {}, __TAURI__: { core: { invoke(command) {
  return Promise.resolve(command === "the_render_settings_panel" ? answers.panel : null);
} } } };
const { panels } = await import("./a-server.js");
const { theSettingsAreOpenedOn } = await import("./render-settings.js");
const settle = async () => { for (let at = 0; at < 8; at += 1) await new Promise((go) => setImmediate(go)); };
await theSettingsAreOpenedOn("SERVER", Array.from(Buffer.from(answers.panel.file)), "");
const head = document.createElement("div");
const body = document.createElement("div");
panels[0].itIsDrawn({ head, body }, "SERVER");
await settle();
const face = [...everyPiece(head), ...everyPiece(body)].find((one) => one.dataset.box === "reports");
const list = face.parentNode.children.find((one) => one.names && one.names.has("ddl"));
const at = (top, bottom) => ({ top, bottom, left: 0, right: 100, width: 100, height: bottom - top });
const opens = (height, control, room) => {
  window.innerHeight = height;
  face.rect = control;
  list.rect = room;
  face.press();
  const said = { open: !list.hidden, up: list.names.has("up") };
  face.press();
  return said;
};
console.log(JSON.stringify({
  atTheFoot: opens(800, at(760, 786), at(790, 900)),
  withRoomBelow: opens(800, at(100, 126), at(130, 240)),
  moreRoomBelowThanAbove: opens(300, at(60, 86), at(90, 400)),
  atTheFootAgain: opens(800, at(760, 786), at(790, 900)),
}));
"#;

    /// **Note 1 in a real browser**: the page drawn in headless Chrome with
    /// the application's own stylesheets, the panel pinned to the window's
    /// foot, and *What it reports* opened — its list carries `up`, ends above
    /// the control and inside the window; the Blender version's list, near the
    /// top with room below, opens downward as drawn. **It fails if the list at
    /// the foot runs past the window's bottom.**
    ///
    /// **Where `SMOOZY_THE_UPWARD_PICTURE` names a file, a picture of the
    /// window with the list open is written there** — the builder's look at
    /// it, which no assertion stands in for. A computer with no Chrome skips
    /// it.
    #[test]
    fn a_dropdown_at_the_windows_foot_opens_upward_in_a_real_browser() {
        let Some(chrome) = a_chrome() else {
            eprintln!("SKIPPED: no Chrome on this computer, so the dropdown was not measured");
            return;
        };
        let answers = serde_json::json!({ "plain": a_panel_with_the_reports_dropdown() });
        let folder = std::env::temp_dir().join(format!("smoozy-dropdown-upward-browser-{}", std::process::id()));
        let _ = fs::remove_dir_all(&folder);
        fs::create_dir_all(&folder).expect("a scratch folder");
        let web = Path::new(env!("CARGO_MANIFEST_DIR")).join("web");
        for entry in fs::read_dir(&web).expect("the page folder") {
            let path = entry.expect("an entry").path();
            if path.extension().and_then(|one| one.to_str()) == Some("css") {
                fs::copy(&path, folder.join(path.file_name().expect("a name"))).expect("a stylesheet is copied");
            }
        }
        fs::copy(web.join("render-settings.js"), folder.join("render-settings.js")).expect("the page");
        for (name, text) in THE_UPLOADING_LINES_STAND_INS {
            fs::write(folder.join(name), text).expect("a stand-in");
        }
        fs::write(folder.join("answers.js"), format!("export default {answers};")).expect("the answers");
        fs::write(folder.join("index.html"), THE_FOOT_WITH_THE_LIST_OPEN.replace("SERVER", &a_server())).expect("the page");

        let run = |what: &str, out: Option<&Path>| {
            let mut command = std::process::Command::new(&chrome);
            command.args([
                "--headless",
                "--disable-gpu",
                "--no-first-run",
                "--hide-scrollbars",
                "--allow-file-access-from-files",
                "--virtual-time-budget=5000",
                "--window-size=1400,900",
            ]);
            command.arg(format!("--user-data-dir={}", folder.join(format!("profile-{what}")).display()));
            match out {
                Some(picture) => command.arg(format!("--screenshot={}", picture.display())),
                None => command.arg("--dump-dom"),
            };
            let dumped = folder.join(format!("{what}.html"));
            let mut chrome = command
                .arg(format!("file://{}", folder.join("index.html").display()))
                .stdout(fs::File::create(&dumped).expect("the dump is written"))
                .stderr(std::process::Stdio::null())
                .spawn()
                .expect("Chrome starts");
            // A fixed deadline, never an open wait.
            let deadline = std::time::Instant::now() + std::time::Duration::from_secs(60);
            while chrome.try_wait().expect("Chrome is watched").is_none() {
                if std::time::Instant::now() > deadline {
                    let _ = chrome.kill();
                    let _ = chrome.wait();
                    break;
                }
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
            fs::read_to_string(&dumped).unwrap_or_default()
        };
        let page = run("measured", None);
        if let Some(picture) = std::env::var_os("SMOOZY_THE_UPWARD_PICTURE") {
            run("picture", Some(Path::new(&picture)));
        }
        let _ = fs::remove_dir_all(&folder);
        let said = page
            .split("<pre id=\"out\">")
            .nth(1)
            .and_then(|rest| rest.split("</pre>").next())
            .filter(|said| !said.is_empty())
            .unwrap_or_else(|| panic!("the page drew no measurement: {page}"));
        let said: serde_json::Value = serde_json::from_str(said).expect("one JSON answer");
        let at = |which: &str, part: &str, edge: &str| {
            said[which][part][edge].as_f64().unwrap_or_else(|| panic!("no {which} {part} {edge}: {said}"))
        };
        let window = said["window"].as_f64().expect("the window's height");

        assert_eq!(said["foot"]["up"], true, "the list at the foot did not open upward: {said}");
        assert!(at("foot", "list", "bottom") <= at("foot", "face", "top"), "the list at the foot is not above its control: {said}");
        assert!(at("foot", "list", "top") >= 0.0, "the list at the foot runs past the window's top: {said}");
        assert!(at("foot", "list", "bottom") <= window, "the list at the foot runs past the window's bottom: {said}");
        assert!(at("foot", "face", "bottom") > window - 120.0, "the control is not at the window's foot: {said}");

        assert_eq!(said["top"]["up"], false, "a list with room below opened upward: {said}");
        assert!(at("top", "list", "top") >= at("top", "face", "bottom"), "the list near the top is not under its control: {said}");
        assert!(at("top", "list", "bottom") <= window, "the list near the top does not fit below: {said}");
    }

    /// **The Creator's eighteenth notes, note 11, on this panel: a list cut off
    /// by the panel's own scrolling sheet opens upward**, though the window has
    /// room under it. The note was met on Queue's View Layer list; this panel's
    /// dropdowns measured the same way, against the window's bottom alone. The
    /// panel stands at the window's top, shorter than the window, with *What
    /// it reports* in the foot along the sheet's bottom, and its list opens
    /// upward; the Blender version's, with room in the sheet, opens downward.
    /// A computer with no Chrome skips it.
    #[test]
    fn eighteenth_notes_11_a_list_cut_off_by_the_sheet_opens_upward_in_a_real_browser() {
        let Some(chrome) = a_chrome() else {
            eprintln!("SKIPPED: no Chrome on this computer, so the dropdown was not measured");
            return;
        };
        let answers = serde_json::json!({ "plain": a_panel_with_the_reports_dropdown() });
        let folder = std::env::temp_dir().join(format!("smoozy-eighteenth-11-sheet-{}", std::process::id()));
        let _ = fs::remove_dir_all(&folder);
        fs::create_dir_all(&folder).expect("a scratch folder");
        let web = Path::new(env!("CARGO_MANIFEST_DIR")).join("web");
        for entry in fs::read_dir(&web).expect("the page folder") {
            let path = entry.expect("an entry").path();
            if path.extension().and_then(|one| one.to_str()) == Some("css") {
                fs::copy(&path, folder.join(path.file_name().expect("a name"))).expect("a stylesheet is copied");
            }
        }
        fs::copy(web.join("render-settings.js"), folder.join("render-settings.js")).expect("the page");
        for (name, text) in THE_UPLOADING_LINES_STAND_INS {
            fs::write(folder.join(name), text).expect("a stand-in");
        }
        fs::write(folder.join("answers.js"), format!("export default {answers};")).expect("the answers");
        fs::write(folder.join("index.html"), THE_SHEET_ENDING_UNDER_THE_LIST.replace("SERVER", &a_server())).expect("the page");
        let dumped = folder.join("measured.html");
        let mut command = std::process::Command::new(&chrome);
        command.args([
            "--headless",
            "--disable-gpu",
            "--no-first-run",
            "--hide-scrollbars",
            "--allow-file-access-from-files",
            "--virtual-time-budget=5000",
            "--window-size=1400,900",
            "--dump-dom",
        ]);
        command.arg(format!("--user-data-dir={}", folder.join("profile").display()));
        let mut chrome = command
            .arg(format!("file://{}", folder.join("index.html").display()))
            .stdout(fs::File::create(&dumped).expect("the dump is written"))
            .stderr(std::process::Stdio::null())
            .spawn()
            .expect("Chrome starts");
        // A fixed deadline, never an open wait.
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(60);
        while chrome.try_wait().expect("Chrome is watched").is_none() {
            if std::time::Instant::now() > deadline {
                let _ = chrome.kill();
                let _ = chrome.wait();
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
        let page = fs::read_to_string(&dumped).unwrap_or_default();
        let _ = fs::remove_dir_all(&folder);
        let said = page
            .split("<pre id=\"out\">")
            .nth(1)
            .and_then(|rest| rest.split("</pre>").next())
            .filter(|said| !said.is_empty())
            .unwrap_or_else(|| panic!("the page drew no measurement: {page}"));
        let said: serde_json::Value = serde_json::from_str(said).expect("one JSON answer");
        assert!(said.get("error").is_none(), "{said}");
        let at = |which: &str, part: &str, edge: &str| {
            said[which][part][edge].as_f64().unwrap_or_else(|| panic!("no {which} {part} {edge}: {said}"))
        };
        let window = said["window"].as_f64().expect("the window's height");
        assert!(
            at("cut", "sheet", "bottom") - at("cut", "face", "bottom") < at("cut", "list", "height"),
            "the list would fit under its control inside the sheet, so nothing is measured: {said}"
        );
        assert!(at("cut", "sheet", "bottom") + 200.0 < window, "the window has no room under the sheet: {said}");
        assert_eq!(said["cut"]["up"], true, "a list the sheet cuts off opened downward: {said}");
        assert!(at("cut", "list", "bottom") <= at("cut", "face", "top") + 0.5, "the list is not above its control: {said}");
        assert_eq!(said["room"]["up"], false, "a list with room in the sheet opened upward: {said}");
    }

    /// **The panel at the window's top, 480 pixels tall in a window of about
    /// 800**, so its sheet — which clips what it holds — ends with *What it
    /// reports* in the foot along its bottom, and the window's room under it.
    /// The Blender version's list, near the top, has room in the sheet. Each
    /// opening is measured and shut.
    const THE_SHEET_ENDING_UNDER_THE_LIST: &str = r#"<!doctype html><html><head><meta charset="utf-8">
<link rel="stylesheet" href="smoozy.css">
<style>
  html, body { height: 100%; overflow: hidden; margin: 0; }
  .app { display: block; height: 100%; }
  .frame { width: 1100px; height: 480px; position: fixed; left: 0; top: 0; }
  #out { display: none; }
</style>
</head><body>
<div class="app" id="app"><div class="frame" data-panel="set"><div class="sheet"><div class="shead"><div class="htitle" id="head"></div></div><div class="suscroll" id="body"></div></div></div></div>
<pre id="out"></pre>
<script>
  window.__TAURI__ = { core: { invoke(command) {
    if (command === "the_render_settings_panel") return Promise.resolve(structuredClone(window.theAnswers.plain));
    return Promise.resolve(null);
  } } };
</script>
<script type="module">
import answers from "./answers.js";
import { panels } from "./a-server.js";
import { theSettingsAreOpenedOn } from "./render-settings.js";
window.theAnswers = answers;
try {
const settle = async () => { for (let at = 0; at < 8; at += 1) await new Promise((go) => setTimeout(go, 0)); };
const head = document.getElementById("head");
const body = document.getElementById("body");
await theSettingsAreOpenedOn("SERVER", Array.from(new TextEncoder().encode(answers.plain.file)), "");
panels[0].itIsDrawn({ head, body }, "SERVER");
await settle();
const edges = (part) => {
  const at = part.getBoundingClientRect();
  return { top: at.top, bottom: at.bottom, left: at.left, right: at.right, height: at.height };
};
const sheet = document.querySelector(".sheet");
const opened = (name) => {
  const face = document.querySelector(`[data-box="${name}"]`);
  face.click();
  const list = face.parentNode.querySelector(".ddl");
  const answer = { up: list.classList.contains("up"), face: edges(face), list: edges(list), sheet: edges(sheet) };
  face.click();
  return answer;
};
const out = { window: window.innerHeight };
out.cut = opened("reports");
out.room = opened("version");
document.getElementById("out").textContent = JSON.stringify(out);
} catch (error) { document.getElementById("out").textContent = JSON.stringify({ error: String(error && error.stack || error) }); }
</script></body></html>"#;

    /// The first Chrome this computer has, where it has one.
    fn a_chrome() -> Option<std::path::PathBuf> {
        [
            "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
            "/Applications/Chromium.app/Contents/MacOS/Chromium",
            "/usr/bin/google-chrome",
            "/usr/bin/chromium",
            "/usr/bin/chromium-browser",
            "C:\\Program Files\\Google\\Chrome\\Application\\chrome.exe",
        ]
        .into_iter()
        .map(std::path::PathBuf::from)
        .find(|one| one.exists())
    }

    /// **The panel pinned to the window's foot**, as Render Settings' foot
    /// sits at the bottom of the window. The Blender version's list is opened
    /// and measured with the panel's top at the window's top, then the panel
    /// is pinned to the foot and *What it reports* is opened, measured, and
    /// left open for the picture.
    const THE_FOOT_WITH_THE_LIST_OPEN: &str = r#"<!doctype html><html><head><meta charset="utf-8">
<link rel="stylesheet" href="smoozy.css">
<style>
  html, body { height: 100%; overflow: hidden; margin: 0; }
  .app { display: block; height: 100%; }
  .frame { width: 1100px; position: fixed; left: 0; top: 0; }
  .frame.atthefoot { top: auto; bottom: 0; }
  #out { display: none; }
</style>
</head><body>
<div class="app" id="app"><div class="frame" data-panel="set"><div class="sheet"><div class="shead"><div class="htitle" id="head"></div></div><div class="suscroll" id="body"></div></div></div></div>
<pre id="out"></pre>
<script>
  window.__TAURI__ = { core: { invoke(command) {
    if (command === "the_render_settings_panel") return Promise.resolve(structuredClone(window.theAnswers.plain));
    return Promise.resolve(null);
  } } };
</script>
<script type="module">
import answers from "./answers.js";
import { panels } from "./a-server.js";
import { theSettingsAreOpenedOn } from "./render-settings.js";
window.theAnswers = answers;
window.addEventListener("unhandledrejection", (event) => { document.getElementById("out").textContent = JSON.stringify({ error: String(event.reason && event.reason.stack || event.reason) }); });
const settle = async () => { for (let at = 0; at < 8; at += 1) await new Promise((go) => setTimeout(go, 0)); };
const head = document.getElementById("head");
const body = document.getElementById("body");
await theSettingsAreOpenedOn("SERVER", Array.from(new TextEncoder().encode(answers.plain.file)), "");
panels[0].itIsDrawn({ head, body }, "SERVER");
await settle();
const box = (name) => document.querySelector(`[data-box="${name}"]`);
const edges = (part) => {
  if (!part) return null;
  const at = part.getBoundingClientRect();
  return { top: at.top, bottom: at.bottom, left: at.left, right: at.right, height: at.height };
};
const opened = (face) => {
  face.click();
  const list = face.parentNode.querySelector(".ddl");
  return { up: list.classList.contains("up"), face: edges(face), list: edges(list) };
};
const out = { window: window.innerHeight };
out.top = opened(box("version"));
box("version").click();
document.querySelector(".frame").classList.add("atthefoot");
await settle();
out.foot = opened(box("reports"));
document.getElementById("out").textContent = JSON.stringify(out);
</script></body></html>"#;

    // ════════════════════════════════════════════════════════════════════════
    // A picture every few minutes (piece N, 2026-10-19): the three drawn states
    // ════════════════════════════════════════════════════════════════════════

    /// **`res-tglist`, `res-tgmin` and `res-tgbad` in a real browser**: the
    /// page drawn in headless Chrome with the application's own stylesheets,
    /// the panel pinned to the window's foot, as the drawing draws the three.
    ///
    /// - **The list open**: it opens upward, ends above its control and inside
    ///   the window, and its third row is the new one.
    /// - **Picked**: the control keeps its width and its place; the field and
    ///   *minutes* sit on the control's line after it; the field is 58 wide,
    ///   right-aligned, in ordinary ink, and nothing else on the line moves.
    /// - **Refused**: the field is red (the drawing's `--alert`), and *Whole
    ///   minutes, 1 to 60.* sits on the third line, under line 2, starting at
    ///   the line's left with no inset.
    ///
    /// **Where `SMOOZY_THE_MINUTES_PICTURES` names a folder, a picture of each
    /// state is written there** (`tglist.png`, `tgmin.png`, `tgbad.png`, at
    /// twice the pixels) — the builder's look beside the drawing, which no
    /// assertion stands in for. A computer with no Chrome skips it.
    #[test]
    fn the_minutes_are_drawn_as_the_drawing_draws_them_in_a_real_browser() {
        let Some(chrome) = a_chrome() else {
            eprintln!("SKIPPED: no Chrome on this computer, so the minutes were not measured");
            return;
        };
        let file = "/ephemeral/smoozy/assets/kitchen_v14.blend";
        let mut held = a_run_with_a_reading();
        let record = WhatTheRecordHolds {
            telegram_is_on: true,
            the_proposed_folder: Some(b"/Users/artist/Renders/kitchen".to_vec()),
            ..WhatTheRecordHolds::default()
        };
        let plain = the_panel_starting_from(&held, &a_server(), file, &record);
        let _ = what_the_answer_says(&mut held, &a_server(), file, WhichSetting::WhatItReports, A_PICTURE_EVERY_FEW_MINUTES);
        let minutes = the_panel_starting_from(&held, &a_server(), file, &record);
        assert!(minutes.the_minutes_between_pictures.is_some());
        let refused = serde_json::json!({ "theBox": "telegram-minutes", "partCount": "1", "lead0": words::WHOLE_MINUTES_1_TO_60 });
        let answers = serde_json::json!({ "plain": plain, "minutes": minutes, "refused": refused });

        let folder = std::env::temp_dir().join(format!("smoozy-minutes-browser-{}", std::process::id()));
        let _ = fs::remove_dir_all(&folder);
        fs::create_dir_all(&folder).expect("a scratch folder");
        let web = Path::new(env!("CARGO_MANIFEST_DIR")).join("web");
        for entry in fs::read_dir(&web).expect("the page folder") {
            let path = entry.expect("an entry").path();
            if path.extension().and_then(|one| one.to_str()) == Some("css") {
                fs::copy(&path, folder.join(path.file_name().expect("a name"))).expect("a stylesheet is copied");
            }
        }
        fs::copy(web.join("render-settings.js"), folder.join("render-settings.js")).expect("the page");
        for (name, text) in THE_UPLOADING_LINES_STAND_INS {
            fs::write(folder.join(name), text).expect("a stand-in");
        }
        fs::write(folder.join("answers.js"), format!("export default {answers};")).expect("the answers");
        fs::write(folder.join("index.html"), THE_MINUTES_AT_THE_FOOT.replace("SERVER", &a_server())).expect("the page");

        let run = |state: &str, out: Option<&Path>| {
            let mut command = std::process::Command::new(&chrome);
            command.args([
                "--headless",
                "--disable-gpu",
                "--no-first-run",
                "--hide-scrollbars",
                "--allow-file-access-from-files",
                "--virtual-time-budget=5000",
                "--window-size=1280,800",
            ]);
            command.arg(format!("--user-data-dir={}", folder.join(format!("profile-{state}")).display()));
            match out {
                Some(picture) => {
                    command.arg("--force-device-scale-factor=2");
                    command.arg(format!("--screenshot={}", picture.display()))
                }
                None => command.arg("--dump-dom"),
            };
            let dumped = folder.join(format!("{state}.html"));
            let mut chrome = command
                .arg(format!("file://{}#{state}", folder.join("index.html").display()))
                .stdout(fs::File::create(&dumped).expect("the dump is written"))
                .stderr(std::process::Stdio::null())
                .spawn()
                .expect("Chrome starts");
            // A fixed deadline, never an open wait.
            let deadline = std::time::Instant::now() + std::time::Duration::from_secs(60);
            while chrome.try_wait().expect("Chrome is watched").is_none() {
                if std::time::Instant::now() > deadline {
                    let _ = chrome.kill();
                    let _ = chrome.wait();
                    break;
                }
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
            fs::read_to_string(&dumped).unwrap_or_default()
        };
        let page = run("measured", None);
        if let Some(pictures) = std::env::var_os("SMOOZY_THE_MINUTES_PICTURES") {
            let pictures = Path::new(&pictures);
            for state in ["tglist", "tgmin", "tgbad"] {
                run(state, Some(&pictures.join(format!("{state}.png"))));
            }
        }
        let _ = fs::remove_dir_all(&folder);
        let said = page
            .split("<pre id=\"out\">")
            .nth(1)
            .and_then(|rest| rest.split("</pre>").next())
            .filter(|said| !said.is_empty())
            .unwrap_or_else(|| panic!("the page drew no measurement: {page}"));
        // **The words as read**: the dump writes the typography point's bound
        // space as an entity.
        let said: serde_json::Value = serde_json::from_str(&said.replace("&nbsp;", " ")).expect("one JSON answer");
        let at = |state: &str, part: &str, edge: &str| -> f64 {
            said[state][part][edge].as_f64().unwrap_or_else(|| panic!("no {edge} for {part} in {state}: {said}"))
        };
        let middle = |state: &str, part: &str| (at(state, part, "top") + at(state, part, "bottom")) / 2.0;
        let near = |a: f64, b: f64| (a - b).abs() <= 1.5;
        let window = said["window"].as_f64().expect("the window's height");

        // **The list open, upward, the new row third.**
        assert_eq!(said["tglist"]["up"], true, "the list did not open upward: {said}");
        assert!(at("tglist", "list", "bottom") <= at("tglist", "face", "top"), "the list is not above its control: {said}");
        assert!(at("tglist", "list", "top") >= 0.0 && at("tglist", "list", "bottom") <= window, "the list leaves the window: {said}");
        assert_eq!(
            said["tglist"]["rows"],
            serde_json::json!(["Everything", "Pictures only", "A picture every few minutes", "Alerts only", "Off"]),
            "{said}"
        );

        // **Picked: the control keeps its width and place, the field and
        // *minutes* after it on its line.**
        for edge in ["left", "right", "top", "bottom"] {
            assert!(near(at("tgmin", "face", edge), at("tglist", "face", edge)), "the control moved or changed width ({edge}): {said}");
        }
        assert_eq!(said["tgmin"]["face"]["text"], "A picture every", "{said}");
        assert!(near(middle("tgmin", "field"), middle("tgmin", "face")), "the field is not on the control's line: {said}");
        assert!(near(middle("tgmin", "after"), middle("tgmin", "face")), "minutes is not on the control's line: {said}");
        assert!(at("tgmin", "field", "left") > at("tgmin", "face", "right"), "the field is not after the control: {said}");
        assert!(at("tgmin", "after", "left") > at("tgmin", "field", "right"), "minutes is not after the field: {said}");
        assert!(near(at("tgmin", "field", "right") - at("tgmin", "field", "left"), 58.0), "the field is not 58 wide: {said}");
        assert_eq!(said["tgmin"]["field"]["align"], "right", "{said}");
        assert_eq!(said["tgmin"]["field"]["style"], "normal", "the value is upright: {said}");
        assert_eq!(said["tgmin"]["field"]["value"], "5", "{said}");
        assert_eq!(said["tgmin"]["field"]["color"], said["ink"], "the field is in ordinary ink: {said}");
        for edge in ["left", "top"] {
            assert!(near(at("tgmin", "firstFrameSwitch", edge), at("tglist", "firstFrameSwitch", edge)), "line 2 moved: {said}");
            assert!(near(at("tgmin", "render", edge), at("tglist", "render", edge)), "Render moved: {said}");
        }
        assert_eq!(said["tgmin"]["answer"], serde_json::Value::Null, "a line is shown with nothing to say: {said}");

        // **Refused: the field red, the range on the third line at its left.**
        assert_eq!(said["tgbad"]["field"]["color"], said["alert"], "the field is not red: {said}");
        assert_eq!(said["tgbad"]["field"]["border"], said["alert"], "the field's border is not red: {said}");
        assert_eq!(said["tgbad"]["field"]["value"], "0", "what was typed stays in it: {said}");
        assert_eq!(said["tgbad"]["answer"]["text"], "Whole minutes, 1 to 60.", "{said}");
        assert_eq!(said["tgbad"]["answer"]["color"], said["alert"], "the line is not in the refusal colour: {said}");
        assert!(at("tgbad", "answer", "top") >= at("tgbad", "firstFrameSwitch", "bottom"), "the line is not under line 2: {said}");
        assert!(near(at("tgbad", "answer", "left"), at("tgbad", "firstFrameSwitch", "left")), "the line does not start at the line's left: {said}");
        assert_eq!(said["tgbad"]["answer"]["padding"], "0px", "the line keeps the value column's inset: {said}");
    }

    /// **The foot drawn three ways, and measured** — or, where the address
    /// names one state, that state alone left on screen for its picture. The
    /// panel is pinned to the window's foot, as Render Settings' foot sits.
    const THE_MINUTES_AT_THE_FOOT: &str = r#"<!doctype html><html><head><meta charset="utf-8">
<link rel="stylesheet" href="smoozy.css">
<style>
  html, body { height: 100%; overflow: hidden; margin: 0; }
  .app { display: block; height: 100%; }
  .frame { width: 1100px; position: fixed; left: 0; bottom: 0; }
  #out { display: none; }
</style>
</head><body>
<div class="app" id="app"><div class="frame" data-panel="set"><div class="sheet"><div class="shead"><div class="htitle" id="head"></div></div><div class="suscroll" id="body"></div></div></div></div>
<pre id="out"></pre>
<script>
  window.theAnswer = "plain";
  window.__TAURI__ = { core: { invoke(command) {
    if (command === "the_render_settings_panel") return Promise.resolve(structuredClone(window.theAnswers[window.theAnswer]));
    return Promise.resolve(null);
  } } };
</script>
<script type="module">
import answers from "./answers.js";
import { panels } from "./a-server.js";
import { theSettingsAreOpenedOn } from "./render-settings.js";
window.theAnswers = answers;
window.addEventListener("unhandledrejection", (event) => { document.getElementById("out").textContent = JSON.stringify({ error: String(event.reason && event.reason.stack || event.reason) }); });
const settle = async () => { for (let at = 0; at < 8; at += 1) await new Promise((go) => setTimeout(go, 0)); };
const head = document.getElementById("head");
const body = document.getElementById("body");
await theSettingsAreOpenedOn("SERVER", Array.from(new TextEncoder().encode(answers.plain.file)), "");
panels[0].itIsDrawn({ head, body }, "SERVER");
await settle();
const tell = async (said) => {
  for (const handler of globalThis.told["render-settings:an-act"] || []) await handler({ said: { server: "SERVER", file: answers.plain.file, partCount: "0", ...said } });
  await settle();
};
const box = (name) => body.querySelector(`[data-box="${name}"]`);
const edges = (part) => {
  if (!part) return null;
  const at = part.getBoundingClientRect();
  return { top: at.top, bottom: at.bottom, left: at.left, right: at.right };
};
const looks = (part) => {
  if (!part) return null;
  const style = getComputedStyle(part);
  return { ...edges(part), color: style.color, border: style.borderTopColor, align: style.textAlign, style: style.fontStyle, value: part.value, text: part.textContent, padding: style.paddingLeft };
};
const measure = () => {
  const answer = box("telegram-minutes-answer");
  const field = box("telegram-minutes-field");
  return {
    face: looks(box("reports")),
    field: looks(field),
    after: field ? edges(field.nextElementSibling) : null,
    firstFrameSwitch: edges(box("first-frame-switch")),
    render: edges(box("render")),
    answer: answer && !answer.hidden ? looks(answer) : null,
  };
};
const probe = document.createElement("span");
document.body.appendChild(probe);
const colourOf = (token) => { probe.style.color = `var(${token})`; return getComputedStyle(probe).color; };
const state = location.hash.slice(1);
const out = { window: window.innerHeight, alert: colourOf("--alert"), ink: colourOf("--ink") };
// **The list open**, from the default.
box("reports").click();
const list = box("reports").parentNode.querySelector(".ddl");
out.tglist = { ...measure(), up: list.classList.contains("up"), list: edges(list), rows: [...list.children].map((one) => one.textContent) };
if (state !== "tglist") {
  box("reports").click();
  // **Picked**, 5 minutes.
  window.theAnswer = "minutes";
  await tell({ theBox: "reports" });
  out.tgmin = measure();
  if (state !== "tgmin") {
    // **Refused**: 0 typed, and the application's answer.
    const field = box("telegram-minutes-field");
    field.value = "0";
    field.dispatchEvent(new Event("change"));
    await settle();
    await tell(answers.refused);
    out.tgbad = measure();
  }
}
document.getElementById("out").textContent = JSON.stringify(out);
</script></body></html>"#;

    // ════════════════════════════════════════════════════════════════════════
    // Step 10c's dependency wave piece 3: the two slips the splash scenes found
    // ════════════════════════════════════════════════════════════════════════

    /// **A run holding one reading of one file, taken by `version`, with the
    /// scenes given** — the page as it stands after *Read the file* landed.
    pub(crate) fn a_run_that_read(
        scenes: Vec<SceneReading>,
        version: smoozy_server_setup::blender::Version,
    ) -> (WhatTheRunHolds, &'static str) {
        a_run_that_read_this(
            SettingsHalf {
                scenes,
                offered_engine_identifiers: OfferedEngineIdentifiers {
                    cycles: Some(ExactName::as_the_reading_produced_it("CYCLES")),
                    eevee: Some(ExactName::as_the_reading_produced_it("BLENDER_EEVEE")),
                },
                announced_add_on_case: None,
            },
            version,
        )
    }

    /// The same, with the whole settings half a reading carried.
    pub(crate) fn a_run_that_read_this(
        settings: SettingsHalf,
        version: smoozy_server_setup::blender::Version,
    ) -> (WhatTheRunHolds, &'static str) {
        let file = "/ephemeral/smoozy/assets/splash.blend";
        let mut held = WhatTheRunHolds::default();
        let picked = BlenderIdentity {
            path: format!("/opt/blender-{}/blender", version_text(version)),
            version,
        };
        held.readings.a_reading_landed(
            FileKey {
                server: an_identity(&a_server()),
                path: file.to_string(),
            },
            &smoozy_render_settings::file_half::StartedUnder {
                version_pick: picked.clone(),
                file_identity: four_values(),
            },
            settings,
            Some(&picked),
            &four_values(),
            std::time::SystemTime::now(),
        );
        (held, file)
    }

    /// **Fault 6 of the splash scenes, as a person meets it: the noise
    /// threshold the file holds as 0.01 is drawn `0.009999999776482582`.**
    /// `Design/Render Settings § The sample count`'s own example of this line
    /// reads *0.01*. Blender keeps both numbers as single-precision floats, so
    /// the reading carries the nearest single-precision value written out in
    /// full; **the time limit beside it is the same kind of number** and a
    /// limit of 0.1 s was drawn `0.10000000149011612s`.
    #[test]
    fn the_threshold_and_the_time_limit_are_drawn_as_the_file_holds_them() {
        let mut scene = a_scene("Scene");
        scene.early_stop.threshold = f64::from(0.01_f32);
        scene.time_limit = f64::from(0.1_f32);
        scene.view_layers = vec![a_layer("ViewLayer", true, false)];
        let (held, file) = a_run_that_read(vec![scene], a_version(5, 2, 2));
        let panel = the_panel(&held, &a_server(), file);
        let said = the_words_of_a_line(&panel.the_samples_lines[0]);
        assert!(said.contains("<0.01>"), "{said}");
        assert!(said.contains("<0.1s>"), "{said}");

        // **Typed into, the line that sets them aside names them the same
        // way.**
        let mut held = held;
        let _ = the_answer_is_taken(&mut held, &a_server(), file, WhichSetting::SampleCount, "512");
        let panel = the_panel(&held, &a_server(), file);
        let said = the_words_of_a_line(&panel.the_samples_lines[0]);
        assert!(said.contains("<0.01>") && said.contains("<0.1s>"), "{said}");
    }

    /// **Fault 7 of the splash scenes, as a person meets it: an EEVEE file is
    /// told it *stops early at noise threshold …*.** `Render Settings` C3: the
    /// early stop and the time limit are Cycles' alone, and the Design's own
    /// words are *"Where the file carries neither, there is no line"* — an
    /// EEVEE render carries neither, whatever Cycles' settings the file also
    /// holds. **The typed line's list of what a typed count sets aside is held
    /// to the same test**, and `Render Settings` C3's other half with it: a View Layer's own
    /// count is not Cycles' alone, and EEVEE ignores it in 4.5 and obeys it
    /// from 5.0.
    #[test]
    fn an_eevee_render_is_never_told_about_the_early_stop_or_the_time_limit() {
        let eevee = |name: &str| {
            let mut scene = a_scene(name);
            scene.engine_as_read = ExactName::as_the_reading_produced_it("BLENDER_EEVEE");
            scene
        };
        // 4.5, EEVEE, a layer with its own count: EEVEE ignores it there.
        let (mut held, file) = a_run_that_read(vec![eevee("Scene")], a_version(4, 5, 12));
        let panel = the_panel(&held, &a_server(), file);
        assert!(
            panel.the_samples_lines.is_empty(),
            "{:?}",
            panel.the_samples_lines.iter().map(the_words_of_a_line).collect::<Vec<_>>()
        );
        let _ = the_answer_is_taken(&mut held, &a_server(), file, WhichSetting::SampleCount, "256");
        let panel = the_panel(&held, &a_server(), file);
        assert!(
            panel.the_samples_lines.is_empty(),
            "{:?}",
            panel.the_samples_lines.iter().map(the_words_of_a_line).collect::<Vec<_>>()
        );

        // 5.1, EEVEE: the layer's own count counts, and it is the one thing a
        // typed count sets aside.
        let (mut held, file) = a_run_that_read(vec![eevee("Scene")], a_version(5, 1, 2));
        let panel = the_panel(&held, &a_server(), file);
        let lines: Vec<String> = panel.the_samples_lines.iter().map(the_words_of_a_line).collect();
        assert_eq!(lines.len(), 1, "{lines:?}");
        assert!(lines[0].contains("<FX>"), "{lines:?}");
        let _ = the_answer_is_taken(&mut held, &a_server(), file, WhichSetting::SampleCount, "256");
        let panel = the_panel(&held, &a_server(), file);
        let lines: Vec<String> = panel.the_samples_lines.iter().map(the_words_of_a_line).collect();
        assert_eq!(lines.len(), 1, "{lines:?}");
        let plain = |word: &str| Prose::new(word).as_str().replace('\u{a0}', " ");
        assert!(lines[0].starts_with(&plain(words::OVERRIDES_THE_OWN_COUNTS_OF)), "{lines:?}");
        assert!(!lines[0].contains("0.01") && !lines[0].contains("30s"), "{lines:?}");

        // **The engine the page shows is what decides, not the file's own**:
        // a Cycles file the user switched to EEVEE loses the line too, and an
        // EEVEE file switched to Cycles gains it.
        let (mut held, file) = a_run_that_read(vec![a_scene("Scene")], a_version(5, 2, 2));
        let _ = the_answer_is_taken(&mut held, &a_server(), file, WhichSetting::EnginePick, "eevee");
        let panel = the_panel(&held, &a_server(), file);
        assert!(panel
            .the_samples_lines
            .iter()
            .map(the_words_of_a_line)
            .all(|line| !line.contains("0.01")));
        let (mut held, file) = a_run_that_read(vec![eevee("Scene")], a_version(5, 2, 2));
        let _ = the_answer_is_taken(&mut held, &a_server(), file, WhichSetting::EnginePick, "cycles");
        let panel = the_panel(&held, &a_server(), file);
        assert!(the_words_of_a_line(&panel.the_samples_lines[0]).contains("<0.01>"));
    }

    /// **`Render Settings` Q12 and J10's first event, which a standing reading
    /// needs now that one can stand**: picking another Blender drops the
    /// file's half whole, and so does a page drawn after its server's period
    /// ended. Picking the build already in force changes nothing.
    #[test]
    fn a_version_change_or_the_periods_end_drops_the_reading() {
        let file = "/ephemeral/smoozy/assets/kitchen_v14.blend";
        let key = FileKey {
            server: an_identity(&a_server()),
            path: file.to_string(),
        };
        let mut held = a_run_with_a_reading();
        held.version_lists
            .get_mut(&a_server())
            .expect("the list")
            .entries
            .push(smoozy_server_setup::search::ACandidate {
                path: "/opt/blender-4.5.12/blender".to_string(),
                answered: Some(a_version(4, 5, 12)),
            });
        let _ = the_answer_is_taken(&mut held, &a_server(), file, WhichSetting::VersionPick, "/opt/blender-5.1.0/blender");
        assert!(held.readings.standing(&key).is_some(), "picking the build in force dropped the reading");
        let _ = the_answer_is_taken(&mut held, &a_server(), file, WhichSetting::VersionPick, "/opt/blender-4.5.12/blender");
        assert!(held.readings.standing(&key).is_none(), "a reading outlived its version");

        let mut held = a_run_with_a_reading();
        the_period_ended_for(&mut held, &a_server());
        assert!(held.readings.standing(&key).is_none(), "a reading outlived its period");
    }

    /// **The section's first desk check, against the three real splash
    /// readings** — what the page draws from each, as a person meets it.
    ///
    /// **Ignored, because the readings are not in the tree**: they are the
    /// splash tester's (`Audits/Build/Step 10c/The splash scenes at the desk —
    /// what was found.md`), 0.4 to 2.5 MB each. Run by hand with
    /// `SMOOZY_DESK_SPLASH_RESULTS` naming that tester's `results/` folder.
    #[test]
    #[ignore = "reads the splash tester's readings from outside the tree"]
    fn the_three_splash_readings_draw_what_blender_shows() {
        let folder = std::path::PathBuf::from(
            std::env::var("SMOOZY_DESK_SPLASH_RESULTS").expect("SMOOZY_DESK_SPLASH_RESULTS"),
        );
        let settings_of = |name: &str| -> SettingsHalf {
            let whole: serde_json::Value =
                serde_json::from_slice(&fs::read(folder.join(name)).expect("the reading")).expect("it reads");
            serde_json::from_value(whole["printed"]["settings"].clone()).expect("the settings half")
        };
        let the_page_of = |name: &str, version| {
            let (held, file) = a_run_that_read_this(settings_of(name), version);
            the_panel(&held, &a_server(), file)
        };
        let lines = |panel: &TheRenderSettings| -> Vec<String> {
            panel.the_samples_lines.iter().map(the_words_of_a_line).collect()
        };

        let five_two = the_page_of("reading-5.2-as.json", a_version(5, 2, 2));
        println!(
            "5.2: range {:?}, count {:?}, samples {:?}, lines {:?}",
            five_two.the_range.value,
            five_two.the_frame_count.iter().map(|part| part.values.clone()).collect::<Vec<_>>(),
            five_two.samples.value,
            lines(&five_two)
        );
        assert_eq!(five_two.the_range.value.as_ref().map(Verbatim::as_str), Some("1-250"));
        assert!(the_words_of_a_line(&five_two.the_frame_count).contains("<250>"));
        assert_eq!(five_two.samples.value.as_ref().map(Verbatim::as_str), Some("514"));
        assert!(lines(&five_two)[0].contains("<0.01>"), "{:?}", lines(&five_two));

        let five_one = the_page_of("reading-5.1-as.json", a_version(5, 1, 2));
        println!("5.1: samples {:?}, lines {:?}", five_one.samples.value, lines(&five_one));
        assert_eq!(five_one.samples.value.as_ref().map(Verbatim::as_str), Some("150"));
        assert!(lines(&five_one)[0].contains("<0.01>"), "{:?}", lines(&five_one));

        let four_five = the_page_of("reading-4.5-as.json", a_version(4, 5, 12));
        println!("4.5: samples {:?}, lines {:?}", four_five.samples.value, lines(&four_five));
        assert_eq!(four_five.samples.value.as_ref().map(Verbatim::as_str), Some("128"));
        assert!(lines(&four_five).is_empty(), "{:?}", lines(&four_five));
    }

    // ════════════════════════════════════════════════════════════════════════
    // Step 10c's eleventh-pass fix piece 7: the box's read, and *Somewhere
    // else…* asked at once
    // ════════════════════════════════════════════════════════════════════════

    fn a_list_of(versions: &[(u32, u32, u32)]) -> TheListThisPageAsksFor {
        TheListThisPageAsksFor {
            entries: versions
                .iter()
                .map(|(major, minor, patch)| smoozy_server_setup::search::ACandidate {
                    path: format!("/home/ubuntu/smoozy/blender/{major}.{minor}.{patch}/blender"),
                    answered: Some(a_version(*major, *minor, *patch)),
                })
                .collect(),
            answer: smoozy_server_setup::search::WhatTheAnswerSays {
                the_search_is_by_name: false,
                folders_refused: 0,
                did_not_answer: Vec::new(),
            },
            at: std::time::SystemTime::now(),
        }
    }

    const A_SPLASH_FILE: &str = "/home/ubuntu/splash/blender-5.1-splash.blend";

    /// **`Render Settings` B5a on both of the page's reads, in every case where
    /// no version resolves, and the user's pick offers both.** Step 10c's
    /// eleventh pass, fault 8: *Read the file* with no Blender picked did
    /// nothing and said nothing. **The header's read was already drawn
    /// unavailable** (the page harness, 2026-09-30); **the box's *Read it now*
    /// was drawn live**, and a press closed the box, went to the application,
    /// came back *not offered* and said nothing. The box now carries the
    /// answer the header takes.
    #[test]
    fn neither_read_is_offered_where_no_version_resolves_and_the_pick_offers_both() {
        let record = WhatTheRecordHolds::default();
        let mark = keyed(&a_server(), A_SPLASH_FILE);
        let saved = |held: &mut WhatTheRunHolds| {
            held.the_saved_with.insert(mark.clone(), "5.1".to_string());
        };
        let mut nothing_to_match_on = WhatTheRunHolds::default();
        nothing_to_match_on.version_lists.insert(a_server(), a_list_of(&[(5, 2, 0)]));
        let mut below_the_floor = WhatTheRunHolds::default();
        below_the_floor.version_lists.insert(a_server(), a_list_of(&[(4, 2, 0)]));
        saved(&mut below_the_floor);
        let mut being_searched = WhatTheRunHolds::default();
        being_searched.version_searches.insert(a_server(), TheSearchStands::Searching);
        saved(&mut being_searched);
        let mut could_not_be_searched = WhatTheRunHolds::default();
        could_not_be_searched
            .version_searches
            .insert(a_server(), TheSearchStands::TheMachineCouldNotBeAsked);
        saved(&mut could_not_be_searched);
        let mut no_blender = WhatTheRunHolds::default();
        no_blender.version_lists.insert(a_server(), a_list_of(&[]));
        saved(&mut no_blender);
        for (case, held) in [
            ("the file names no Blender", &nothing_to_match_on),
            ("only builds below the floor", &below_the_floor),
            ("the server is being searched", &being_searched),
            ("the server could not be searched", &could_not_be_searched),
            ("the server has no Blender", &no_blender),
        ] {
            assert!(!the_panel(held, &a_server(), A_SPLASH_FILE).the_read_is_offered, "{case}: the header's read");
            let the_box = the_download_box(held, &a_server(), A_SPLASH_FILE, &record);
            assert!(!the_box.the_read_is_offered, "{case}: the box's read");
            let drawn = serde_json::to_value(&the_box).expect("the box is drawn");
            assert_eq!(drawn["is"], "the-box", "{case}: the box is still the box");
            assert_eq!(drawn["the_read_is_offered"], false, "{case}: the page is told");
        }
        // **The withholding lasts exactly until the pick.**
        for mut held in [nothing_to_match_on, below_the_floor] {
            let build = held.version_lists[&a_server()].entries[0].path.clone();
            let _ = the_answer_is_taken(&mut held, &a_server(), A_SPLASH_FILE, WhichSetting::VersionPick, &build);
            assert!(the_panel(&held, &a_server(), A_SPLASH_FILE).the_read_is_offered, "the pick offers the header's read");
            assert!(
                the_download_box(&held, &a_server(), A_SPLASH_FILE, &record).the_read_is_offered,
                "the pick offers the box's read"
            );
        }
    }

    fn the_folders_of(the_box: &TheDownloadBox) -> Vec<String> {
        let TheFormatAnswer::TheBox(the_box) = &the_box.the_box else {
            panic!("a box");
        };
        the_box.folders.iter().map(|one| one.folder.as_str().to_string()).collect()
    }

    fn a_record_with_two_folders() -> WhatTheRecordHolds {
        WhatTheRecordHolds {
            the_standing_folder: Some(b"/Users/artist/Renders/kitchen".to_vec()),
            the_proposed_folder: Some(b"/Users/artist/Downloads".to_vec()),
            ..WhatTheRecordHolds::default()
        }
    }

    /// **`Design/Render Settings`: *"Choosing Somewhere else… puts the system's
    /// folder panel up at once, and the folder chosen there is shown in the
    /// box as the choice, which Done then keeps like any other. Closing the
    /// panel leaves the box as it was."*** The application's half: the folder
    /// the panel gave is held for the open box and offered first, *Done*
    /// keeps it by its row and lets it go, and a box opened afresh has none.
    #[test]
    fn the_folder_the_panel_gave_is_the_boxs_choice_and_done_keeps_it_by_its_row() {
        let record = a_record_with_two_folders();
        let mark = keyed(&a_server(), A_SPLASH_FILE);
        let mut held = WhatTheRunHolds::default();
        let opened = the_box_is_opened(&mut held, &a_server(), A_SPLASH_FILE, &record);
        assert_eq!(the_folders_of(&opened), vec!["/Users/artist/Renders/kitchen", "/Users/artist/Downloads"]);

        let again = the_folder_the_panel_gave_is_held(
            &mut held,
            &a_server(),
            A_SPLASH_FILE,
            b"/Volumes/Renders/kitchen".to_vec(),
            &record,
        );
        assert_eq!(
            the_folders_of(&again),
            vec!["/Volumes/Renders/kitchen", "/Users/artist/Renders/kitchen", "/Users/artist/Downloads"],
            "the folder the panel gave is the box's first row, which is its choice"
        );
        assert!(
            held.submission.get(&mark).is_none_or(|choices| choices.where_they_go.is_none()),
            "nothing is kept before Done"
        );

        assert!(the_download_choices_are_taken(
            &mut held,
            &a_server(),
            A_SPLASH_FILE,
            &[],
            TheFolderTaken::Offered(0),
            &record
        ));
        assert_eq!(
            held.submission[&mark].where_they_go.as_deref(),
            Some(b"/Volumes/Renders/kitchen".as_slice()),
            "Done keeps it like any other"
        );
        assert!(held.the_folder_the_panel_gave.is_empty(), "Done lets the box's folder go");

        // **A box closed with Cancel keeps nothing**: the next box opens on the
        // job's own folder, with nothing the panel gave the closed one.
        let _ = the_folder_the_panel_gave_is_held(&mut held, &a_server(), A_SPLASH_FILE, b"/Volumes/Other".to_vec(), &record);
        let fresh = the_box_is_opened(&mut held, &a_server(), A_SPLASH_FILE, &record);
        assert_eq!(
            the_folders_of(&fresh),
            vec!["/Volumes/Renders/kitchen", "/Users/artist/Renders/kitchen", "/Users/artist/Downloads"]
        );
    }

    /// **The page's half, run**: `web/transfers.js`'s box, opened the way
    /// `web/render-settings.js` opens it, under Node beside a stand-in
    /// document, and fed the answers the application composes above.
    ///
    /// - *Read it now* reads unavailable where the application says the read
    ///   is not offered (fault 8);
    /// - choosing *Somewhere else…* asks for the panel at once, before *Done*
    ///   (fault 11 — before this, the page asked nothing until *Done*);
    /// - the folder the panel gave is drawn as the choice, and *Done* hands it
    ///   back by its row, which the application then keeps;
    /// - the panel closed with nothing chosen leaves the row chosen before.
    ///
    /// **Skipped, and says so, where no Node is installed.**
    #[test]
    fn the_boxs_read_and_somewhere_else_as_the_page_draws_them() {
        let control = the_page()
            .split("control.dataset.box = \"download-choices\";")
            .nth(1)
            .and_then(|rest| rest.split("inner.appendChild(control);").next())
            .expect("the download control")
            .to_string();
        assert!(
            control.contains("\"the_folder_is_asked_for_in_the_box\""),
            "web/render-settings.js opens the box without asking for the panel at the choice"
        );

        let record = a_record_with_two_folders();
        let mut held = WhatTheRunHolds::default();
        held.version_lists.insert(a_server(), a_list_of(&[(5, 2, 0)]));
        let opened = the_box_is_opened(&mut held, &a_server(), A_SPLASH_FILE, &record);
        assert!(!opened.the_read_is_offered);
        let given = the_folder_the_panel_gave_is_held(
            &mut held,
            &a_server(),
            A_SPLASH_FILE,
            b"/Volumes/Renders/kitchen".to_vec(),
            &record,
        );
        // **The Creator's sixth notes, note 2: the box for a file read for
        // another scene**, drawn by the same page code.
        let mut for_another_scene = serde_json::to_value(&opened).expect("the box");
        for_another_scene["reading"] =
            serde_json::to_value(TheFormatReading::read_for_another_scene(
                false,
                crate::surface::transfers::what_downloads_without_a_list(None),
            ))
            .expect("the case");
        // **The box ruling of 2026-10-17: a narrowing that stands, drawn by the same page code** —
        // each format a value among the words.
        let mut narrowed = for_another_scene.clone();
        narrowed["reading"] = serde_json::to_value(TheFormatReading::read_for_another_scene(
            false,
            crate::surface::transfers::what_downloads_without_a_list(Some(&[b"OPEN_EXR".to_vec(), b"PNG".to_vec()])),
        ))
        .expect("the case");
        let answers = serde_json::json!({
            "the_download_panel": opened,
            "the_folder_is_asked_for_in_the_box": given,
            "the_download_panel_for_another_scene": for_another_scene,
            "the_download_panel_narrowed": narrowed,
        });

        let web = Path::new(env!("CARGO_MANIFEST_DIR")).join("web");
        let folder = std::env::temp_dir().join(format!("smoozy-the-box-harness-{}", std::process::id()));
        let _ = fs::remove_dir_all(&folder);
        fs::create_dir_all(&folder).expect("a scratch folder");
        fs::copy(web.join("transfers.js"), folder.join("transfers.js")).expect("the page module is copied");
        for (name, text) in THE_BOX_STAND_INS {
            fs::write(folder.join(name), text).expect("a stand-in is written");
        }
        fs::write(folder.join("package.json"), r#"{ "type": "module" }"#).expect("the package note");
        fs::write(folder.join("answers.json"), answers.to_string()).expect("the answers are written");
        fs::write(folder.join("harness.js"), THE_BOX_HARNESS).expect("the harness is written");
        let ran = std::process::Command::new("node")
            .arg(folder.join("harness.js"))
            .current_dir(&folder)
            .output();
        let _ = fs::remove_dir_all(&folder);
        let Ok(ran) = ran else {
            eprintln!("SKIPPED: no Node on this computer, so the box's page code was not run");
            return;
        };
        assert!(ran.status.success(), "the page code did not run: {}", String::from_utf8_lossy(&ran.stderr));
        let got: serde_json::Value =
            serde_json::from_str(String::from_utf8_lossy(&ran.stdout).trim()).expect("the harness prints one answer");
        assert_eq!(got["readItNowReadsUnavailable"], true, "{got}");
        assert_eq!(got["thePanelIsAskedAtOnce"], true, "{got}");
        assert_eq!(
            got["theRowsRead"],
            serde_json::json!([
                "/Volumes/Renders/kitchen",
                "/Users/artist/Renders/kitchen",
                "/Users/artist/Downloads — your download folder",
                "Somewhere else…"
            ]),
            "{got}"
        );
        assert_eq!(got["theFolderItGaveIsTheChoice"], true, "{got}");
        assert_eq!(got["closedLeavesTheBoxAsItWas"], true, "{got}");
        assert_eq!(
            got["anotherSceneSays"],
            format!(
                "{} {}",
                crate::surface::transfers::words::READ_ANOTHER_SCENE,
                crate::surface::transfers::words::EVERYTHING_DOWNLOADS
            ),
            "{got}"
        );
        assert_eq!(got["anotherSceneOffersNoRead"], true, "{got}");
        assert_eq!(
            got["narrowedSays"],
            format!("{} EXR and PNG files are downloaded.", crate::surface::transfers::words::READ_ANOTHER_SCENE),
            "{got}"
        );
        assert_eq!(got["done"]["folder"], serde_json::json!({ "is": "this-one", "at": 0 }), "{got}");

        // **And *Done*'s answer, taken as the application takes it, keeps the
        // folder the panel gave.**
        let at = got["done"]["folder"]["at"].as_u64().expect("a row") as usize;
        assert!(the_download_choices_are_taken(
            &mut held,
            &a_server(),
            A_SPLASH_FILE,
            &[],
            TheFolderTaken::Offered(at),
            &record
        ));
        assert_eq!(
            held.submission[&keyed(&a_server(), A_SPLASH_FILE)].where_they_go.as_deref(),
            Some(b"/Volumes/Renders/kitchen".as_slice())
        );
    }

    // ── Step 10c's Sesterce-pass fix piece 1: a typed range reaches the job ──

    /// **A run that read `scenes` of one file, with `Render Settings` B1's list standing so a
    /// pick resolves** — the page as the Sesterce pass stood on it after *Read
    /// the file*.
    fn a_run_that_read_and_can_submit(scenes: Vec<SceneReading>) -> (WhatTheRunHolds, &'static str) {
        let (mut held, file) = a_run_that_read(scenes, a_version(5, 1, 0));
        held.version_lists = a_run_with_a_reading().version_lists;
        (held, file)
    }

    /// The one job the page composes for the file as it stands.
    fn the_jobs_the_page_composes(held: &WhatTheRunHolds, file: &str) -> Vec<smoozy_render_settings::payload::Payload> {
        the_act_as_the_page_composed_it(
            held,
            &a_server(),
            file,
            &WhatTheRecordHolds::default(),
            &a_row_on(&a_server(), true),
            a_run(),
        )
        .unwrap_or_else(|said| panic!("{said:?}"))
        .scene_jobs
    }

    fn the_range_the_page_draws(held: &WhatTheRunHolds, file: &str) -> Option<String> {
        the_panel_starting_from(held, &a_server(), file, &WhatTheRecordHolds::default())
            .the_range
            .value
            .map(|value| value.as_str().to_string())
    }

    // `Render Settings` F1, the Creator's seventeenth notes, note 3.
    #[test]
    fn the_seventeenth_notes_keep_the_typed_range_and_count_the_parsed_frames() {
        for (text, count, expected) in [
            ("1,3-5", 4, vec![1, 3, 4, 5]),
            ("1,5-100", 97, std::iter::once(1).chain(5..=100).collect()),
            ("1-250x5", 50, (1..=250).step_by(5).collect()),
            ("5, 1,3-5, 1", 4, vec![1, 3, 4, 5]),
        ] {
            let (mut held, file) = a_run_that_read_and_can_submit(vec![a_scene("Scene")]);
            assert!(what_the_answer_says(&mut held, &a_server(), file, WhichSetting::FrameRange, text).is_empty());
            assert_eq!(the_range_the_page_draws(&held, file).as_deref(), Some(text));
            let panel = the_panel(&held, &a_server(), file);
            assert_eq!(the_words_of_a_line(&panel.the_frame_count).trim(), format!("<{count}> frames"));
            let jobs = the_jobs_the_page_composes(&held, file);
            assert_eq!(jobs[0].frame_range.as_ref().unwrap().explicit_list(), expected);
            assert!(serde_json::to_value(&jobs[0]).unwrap().get("frame_range_text").is_none());
            if text == "1,3-5" {
                assert_eq!(jobs[0].frame_range, Some(FrameForm::List(vec![1, 3, 4, 5])));
            }
            assert!(!what_the_answer_says(&mut held, &a_server(), file, WhichSetting::FrameRange, "5-1").is_empty());
            assert_eq!(the_range_the_page_draws(&held, file).as_deref(), Some(text));
        }
    }

    #[test]
    fn the_seventeenth_notes_keep_the_group_range_and_each_scenes_own_text() {
        let (mut held, file) = a_run_that_read_and_can_submit(vec![a_scene("Scene"), a_scene("Other")]);
        let key = FileKey { server: an_identity(&a_server()), path: file.into() };
        let provenance = the_provenance(&held, &key).unwrap();
        let scene = |name: &str| NamedAnswer { name: ExactName::as_the_reading_produced_it(name), provenance: provenance.clone() };
        held.store.tick_a_scene(key.clone(), scene("Scene"));
        assert!(what_the_answer_says(&mut held, &a_server(), file, WhichSetting::FrameRange, "1,3-5").is_empty());
        held.store.tick_a_scene(key.clone(), scene("Other"));
        assert!(what_the_answer_says(&mut held, &a_server(), file, WhichSetting::FrameRange, "1,5-100").is_empty());
        assert_eq!(the_range_the_page_draws(&held, file).as_deref(), Some("1,5-100"));
        let reading = held.readings.standing(&key).unwrap();
        let count = the_panel(&held, &a_server(), file).the_frame_count;
        assert_eq!(count[0].values[0].as_str(), "194");
        assert_eq!(count[1].values[0].as_str(), "2");
        let jobs: Vec<_> = held.store.per_file(&key).unwrap().scenes_ticked.iter().map(|scene| {
            let answers = held.store.the_answers_composition_reads(&key, Some(&scene.name));
            smoozy_render_settings::payload::compose(smoozy_render_settings::payload::Composition {
                version_pick: &reading.taken_by, scene: Some(scene), answers: &answers,
                standing: Some(reading), stale: false,
            })
        }).collect();
        assert_eq!(jobs.len(), 2);
        assert!(jobs.iter().all(|job| job.frame_range.as_ref().unwrap().explicit_list().len() == 97));
        held.store.untick_a_scene(key.clone(), &scene("Other").name);
        assert_eq!(the_range_the_page_draws(&held, file).as_deref(), Some("1,3-5"));
        held.store.tick_a_scene(key.clone(), scene("Other"));
        held.store.untick_a_scene(key.clone(), &scene("Scene").name);
        assert!(what_the_answer_says(&mut held, &a_server(), file, WhichSetting::FrameRange, "1-250x5").is_empty());
        held.store.tick_a_scene(key.clone(), scene("Scene"));
        held.store.untick_a_scene(key, &scene("Other").name);
        assert_eq!(the_range_the_page_draws(&held, file).as_deref(), Some("1,3-5"));
    }

    /// **`Queue & History` P5b and `Render Settings` F1: the typed text goes
    /// over with the act for the local record, and never into a payload** —
    /// the eighteenth notes, piece 7, note 8. One scene's own typed range, the
    /// group's where several are ticked (every scene-job carries it), and
    /// nothing where the range was not typed.
    #[test]
    fn the_eighteenth_notes_hand_the_typed_range_over_beside_the_payloads_and_never_in_them() {
        let the_act = |held: &WhatTheRunHolds, file: &str| {
            the_act_as_the_page_composed_it(
                held,
                &a_server(),
                file,
                &WhatTheRecordHolds::default(),
                &a_row_on(&a_server(), true),
                a_run(),
            )
            .unwrap_or_else(|said| panic!("{said:?}"))
        };
        // **One scene, typed as a list.**
        let (mut held, file) = a_run_that_read_and_can_submit(vec![a_scene("Scene")]);
        assert_eq!(the_act(&held, file).the_typed_frames, None, "nothing typed hands nothing over");
        assert!(what_the_answer_says(&mut held, &a_server(), file, WhichSetting::FrameRange, " 1,2-4 ").is_empty());
        let given = the_act(&held, file);
        assert_eq!(given.the_typed_frames.as_deref(), Some("1,2-4"));
        assert_eq!(given.scene_jobs[0].frame_range, Some(FrameForm::List(vec![1, 2, 3, 4])));
        for job in &given.scene_jobs {
            let carried = job.as_carried().0.to_string();
            assert!(!carried.contains("2-4"), "the typed text reached a payload: {carried}");
        }
        // **Cleared, nothing is handed over.**
        let key = FileKey { server: an_identity(&a_server()), path: file.into() };
        held.store
            .clear_one(
                Where::AScene { file: key, scene: ExactName::as_the_reading_produced_it("Scene") },
                &smoozy_render_settings::store::Setting::FrameRange,
            )
            .unwrap();
        assert_eq!(the_act(&held, file).the_typed_frames, None);

        // **Several scenes ticked: the group's text, for every scene-job.**
        let (mut held, file) = a_run_that_read_and_can_submit(vec![a_scene("Scene"), a_scene("Other")]);
        // With several ticked no scene is in hand, so the version is matched
        // against the file's own *saved with*, as the reading leaves it.
        held.the_saved_with.insert(keyed(&a_server(), file), "4.5.3".to_string());
        let key = FileKey { server: an_identity(&a_server()), path: file.into() };
        let provenance = the_provenance(&held, &key).unwrap();
        let scene = |name: &str| NamedAnswer { name: ExactName::as_the_reading_produced_it(name), provenance: provenance.clone() };
        held.store.tick_a_scene(key.clone(), scene("Scene"));
        assert!(what_the_answer_says(&mut held, &a_server(), file, WhichSetting::FrameRange, "1,3-5").is_empty());
        held.store.tick_a_scene(key.clone(), scene("Other"));
        assert_eq!(
            the_act(&held, file).the_typed_frames,
            None,
            "a range typed for one scene is not the group's, and the group renders the file's own"
        );
        assert!(what_the_answer_says(&mut held, &a_server(), file, WhichSetting::FrameRange, "1-60,121-300").is_empty());
        let given = the_act(&held, file);
        assert_eq!(given.scene_jobs.len(), 2);
        assert_eq!(given.the_typed_frames.as_deref(), Some("1-60,121-300"));
        assert!(given.scene_jobs.iter().all(|job| job.frame_range.as_ref().unwrap().count() == 240));
    }

    #[test]
    fn the_seventeenth_notes_draw_an_entry_without_text_as_before() {
        let (mut held, file) = a_run_that_read_and_can_submit(vec![a_scene("Scene")]);
        let key = FileKey { server: an_identity(&a_server()), path: file.into() };
        held.store.answer(
            Where::AScene { file: key, scene: ExactName::as_the_reading_produced_it("Scene") },
            Answer::FrameRange(FrameForm::List(vec![1, 3, 4, 5])),
        );
        assert_eq!(the_range_the_page_draws(&held, file).as_deref(), Some("1,3,4,5"));
    }

    #[test]
    fn the_seventeenth_notes_keep_a_pre_read_range_within_the_run() {
        let (mut held, file) = a_run_that_read_and_can_submit(vec![a_scene("Scene")]);
        let key = FileKey { server: an_identity(&a_server()), path: file.into() };
        held.store.answer_the_typed_range(
            Where::ThePreReadSet { file: key.clone() },
            FrameForm::List(vec![1, 3, 4, 5]),
            "1, 3-5",
        );
        held.store.a_reading_first_landed(key, ExactName::as_the_reading_produced_it("Scene"));
        assert_eq!(the_range_the_page_draws(&held, file).as_deref(), Some("1, 3-5"));
    }

    #[test]
    fn the_seventeenth_notes_page_does_not_put_a_refused_range_back() {
        let page = the_page();
        let function = page.split("function theRefusalIsPutOnItsBox(server, said) {").nth(1).unwrap()
            .split("/// **A sentence off the event").next().unwrap();
        let script = format!(r#"
const CSS = {{ escape: (text) => text }};
const THE_JOB_TO_SHOW = "job", THE_PLAIN_PARTS = "plain";
const field = {{ value: "1,3-5", classList: {{ add() {{}} }} }};
const row = {{ classList: {{ add() {{}} }} }};
const line = {{ replaceChildren() {{}}, hidden: true }};
const panels = new Map([["s", {{ head: {{ querySelector() {{ return null; }} }}, body: {{
    querySelector(selector) {{
        if (selector.includes("range-field")) return field;
        if (selector.includes("refusal")) return line;
        if (selector.includes('"range"')) return row;
        return null;
    }}
}} }}]]);
let theLastTyped = {{ which: "frame-range", text: "5-1" }};
function thePartsOf() {{ return [{{ lead: "refused" }}]; }}
function theSentence() {{}}
function theRefusalIsPutOnItsBox(server, said) {{{function}
theRefusalIsPutOnItsBox("s", {{ theBox: "range" }});
if (field.value !== "1,3-5") throw new Error("a refused range replaced the accepted text");
if (line.hidden) throw new Error("the refusal was not shown");
"#);
        let ran = std::process::Command::new("node").arg("-e").arg(script).output().expect("node runs the page function");
        assert!(ran.status.success(), "{}", String::from_utf8_lossy(&ran.stderr));
    }

    fn typed(text: &str) -> FrameForm {
        frames::parse_the_frame_field(text).expect("a range")
    }

    /// **The Sesterce pass's fault 1, as it happened: *Read the file*, then
    /// `1-20` typed with no scene ticked, then *Render*.** The field read back
    /// `1-20` and the button said *Render 20 frames*; the manifest said
    /// `TheFilesOwn` and Blender rendered the file's 1..250.
    /// (`Audits/Build/Step 10c/The pass before Vast — Sesterce.md`, fault 1.)
    ///
    /// The page writes a typed range under the scene in hand (`Render
    /// Settings` J3: once a reading lands, a scene is in hand); **the job must
    /// compose from that same entry** (`Render Settings` F5, the sentence
    /// settled 2026-10-01). On a one-scene reading the job names that scene;
    /// on a reading of several it names none (`Render Settings` Y10) and still
    /// carries the range.
    #[test]
    fn a_range_typed_after_a_read_reaches_the_job_as_the_page_draws_it() {
        // One scene, nothing ticked.
        let (mut held, file) = a_run_that_read_and_can_submit(vec![a_scene("Scene")]);
        assert!(the_answer_is_taken(&mut held, &a_server(), file, WhichSetting::FrameRange, "1-20").is_none());
        assert_eq!(the_range_the_page_draws(&held, file).as_deref(), Some("1-20"));
        let jobs = the_jobs_the_page_composes(&held, file);
        assert_eq!(jobs.len(), 1);
        assert_eq!(jobs[0].frame_range, Some(typed("1-20")), "the range the page drew");
        assert_eq!(
            jobs[0].the_manifests_frame_range(),
            smoozy_job_files::manifest::FrameRange::TheUsersParsedIntegers((1..=20).collect()),
            "the manifest carries the user's integers, never the file's own"
        );
        assert_eq!(
            jobs[0].scene.as_ref().map(|scene| scene.name.as_text().to_string()).as_deref(),
            Some("Scene"),
            "`Render Settings` F5: a one-scene reading names its one scene"
        );

        // Two scenes, nothing ticked.
        let (mut held, file) =
            a_run_that_read_and_can_submit(vec![a_scene("Interior"), a_scene("Exterior")]);
        assert!(the_answer_is_taken(&mut held, &a_server(), file, WhichSetting::FrameRange, "1-20").is_none());
        assert_eq!(the_range_the_page_draws(&held, file).as_deref(), Some("1-20"));
        let jobs = the_jobs_the_page_composes(&held, file);
        assert_eq!(jobs.len(), 1);
        assert_eq!(jobs[0].frame_range, Some(typed("1-20")));
        assert!(jobs[0].scene.is_none(), "`Render Settings` F5 and `Render Settings` Y10: several scenes, none named");
        assert!(jobs[0].view_layer_pins.is_empty(), "no pins on a job naming no scene");
    }

    /// **A range typed and then the scene ticked is carried**, and wherever
    /// the tick moves the page to, **the job carries what the page then
    /// draws**. The Sesterce tester believed the ticked case failed too;
    /// the pass's capture shows the box unticked (`shots/022`).
    #[test]
    fn a_range_typed_before_the_tick_is_carried_and_the_job_follows_the_page() {
        let tick = |held: &mut WhatTheRunHolds, file: &str, scene: &str| {
            let key = FileKey {
                server: an_identity(&a_server()),
                path: file.to_string(),
            };
            let provenance = the_provenance(held, &key).expect("a reading stands");
            held.store.tick_a_scene(
                key,
                NamedAnswer {
                    name: ExactName::as_the_reading_produced_it(scene),
                    provenance,
                },
            );
        };
        let (mut held, file) = a_run_that_read_and_can_submit(vec![a_scene("Scene")]);
        assert!(the_answer_is_taken(&mut held, &a_server(), file, WhichSetting::FrameRange, "5-9").is_none());
        tick(&mut held, file, "Scene");
        assert_eq!(the_range_the_page_draws(&held, file).as_deref(), Some("5-9"));
        let jobs = the_jobs_the_page_composes(&held, file);
        assert_eq!(jobs[0].frame_range, Some(typed("5-9")));
        assert_eq!(jobs[0].scene.as_ref().map(|scene| scene.name.as_text()), Some("Scene"));

        // **Two scenes: typed while the first stood in hand, then the second
        // ticked.** The page then stands in the second scene and draws its
        // own range, which is the file's — so the job carries no range, and
        // the two agree (`Render Settings` J3's per-scene keying).
        let (mut held, file) =
            a_run_that_read_and_can_submit(vec![a_scene("Interior"), a_scene("Exterior")]);
        assert!(the_answer_is_taken(&mut held, &a_server(), file, WhichSetting::FrameRange, "5-9").is_none());
        tick(&mut held, file, "Exterior");
        assert_eq!(the_range_the_page_draws(&held, file).as_deref(), Some("1-240"));
        let jobs = the_jobs_the_page_composes(&held, file);
        assert_eq!(jobs[0].frame_range, None, "the file's own, as the page draws it");
        assert_eq!(jobs[0].scene.as_ref().map(|scene| scene.name.as_text()), Some("Exterior"));
    }

    /// A reading of three scenes, landed as *Read the file* lands one, for a
    /// file saved showing *Interior* — so the read's own tick stands on it.
    /// **Interior is not the reading's first scene**, so the saved scene and
    /// the first are told apart.
    fn a_run_that_read_a_file_saved_showing_interior() -> (WhatTheRunHolds, &'static str, FileKey) {
        let (mut held, file) =
            a_run_that_read_and_can_submit(vec![a_scene("Exterior"), a_scene("Interior"), a_scene("Garden")]);
        the_read_ticks_its_scene(&mut held, &a_server(), file, Some(b"Interior".as_slice()));
        let key = FileKey {
            server: an_identity(&a_server()),
            path: file.to_string(),
        };
        (held, file, key)
    }

    /// Unticks everything, then ticks these, as the user's boxes would.
    fn only_these_are_ticked(held: &mut WhatTheRunHolds, key: &FileKey, scenes: &[&str]) {
        let provenance = the_provenance(held, key).expect("a reading stands");
        for gone in the_ticks(held, key) {
            held.store
                .untick_a_scene(key.clone(), &ExactName::as_the_reading_produced_it(&gone));
        }
        for scene in scenes {
            held.store.tick_a_scene(
                key.clone(),
                NamedAnswer {
                    name: ExactName::as_the_reading_produced_it(scene),
                    provenance: provenance.clone(),
                },
            );
        }
    }

    fn the_ticks(held: &WhatTheRunHolds, key: &FileKey) -> Vec<String> {
        held.store
            .per_file(key)
            .map(|per_file| {
                per_file
                    .scenes_ticked
                    .iter()
                    .map(|tick| tick.name.as_text().to_string())
                    .collect()
            })
            .unwrap_or_default()
    }

    fn its_entries_stand(held: &WhatTheRunHolds, key: &FileKey, scene: &str) -> bool {
        held.store
            .entries_for(key, &ExactName::as_the_reading_produced_it(scene))
            .is_some()
    }

    /// **The Creator's fifth notes, note 4**: *"Reset all to file values
    /// doesn't reset the selected scene. It should revert to the file's
    /// active scene."* `Render Settings` J5 as changed 2026-10-08: where a
    /// reading stands, the control unticks every scene and ticks the one the
    /// reading names as saved, under that reading's provenance, and deletes
    /// that scene's own entries. **Scenes the page is not showing keep
    /// theirs**, and the version override goes as before.
    #[test]
    fn reset_all_ticks_the_scene_the_file_was_saved_showing_and_clears_both_scenes() {
        let (mut held, file, key) = a_run_that_read_a_file_saved_showing_interior();
        assert_eq!(the_ticks(&held, &key), ["Interior"], "the read's own tick");
        for (scene, samples) in [("Garden", "64"), ("Interior", "128"), ("Exterior", "256")] {
            only_these_are_ticked(&mut held, &key, &[scene]);
            assert!(the_answer_is_taken(&mut held, &a_server(), file, WhichSetting::SampleCount, samples).is_none());
            assert!(its_entries_stand(&held, &key, scene), "{scene} was typed on");
        }
        let the_build = held.readings.standing(&key).expect("a reading stands").taken_by.clone();
        held.store.pick_a_version(key.clone(), the_build);

        assert_eq!(the_value_is_handed_back(&mut held, &a_server(), file, WhichSetting::Everything), None);

        assert_eq!(the_ticks(&held, &key), ["Interior"], "the scene the file was saved showing, alone");
        let tick = &held.store.per_file(&key).expect("ticks stand").scenes_ticked[0];
        assert!(
            matches!(tick.provenance, Provenance::ARecordedReading(_)),
            "under the reading's provenance"
        );
        assert!(!its_entries_stand(&held, &key, "Exterior"), "the scene the page was showing");
        assert!(!its_entries_stand(&held, &key, "Interior"), "the scene it goes back to shows the file's values");
        assert!(its_entries_stand(&held, &key, "Garden"), "a scene the page was not showing keeps its entries");
        assert!(held.store.per_file(&key).expect("ticks stand").version_pick.is_none(), "the version override");
    }

    /// **With several scenes ticked**, the control clears the group range and
    /// the saved scene is ticked alone. **A read that named no saved scene**
    /// gives its first, as the read's own tick does (`Render Settings` J7).
    #[test]
    fn reset_all_with_several_ticked_leaves_the_saved_scene_alone_and_the_group_range_gone() {
        let (mut held, file, key) = a_run_that_read_a_file_saved_showing_interior();
        only_these_are_ticked(&mut held, &key, &["Exterior", "Garden"]);
        assert!(the_answer_is_taken(&mut held, &a_server(), file, WhichSetting::FrameRange, "1-20").is_none());
        assert!(held.store.per_file(&key).and_then(|per_file| per_file.group_frame_range.as_ref()).is_some());

        assert_eq!(the_value_is_handed_back(&mut held, &a_server(), file, WhichSetting::Everything), None);

        assert_eq!(the_ticks(&held, &key), ["Interior"]);
        assert!(held.store.per_file(&key).and_then(|per_file| per_file.group_frame_range.as_ref()).is_none());

        let (mut held, file) = a_run_that_read_and_can_submit(vec![a_scene("Interior"), a_scene("Exterior")]);
        the_read_ticks_its_scene(&mut held, &a_server(), file, None);
        let key = FileKey {
            server: an_identity(&a_server()),
            path: file.to_string(),
        };
        only_these_are_ticked(&mut held, &key, &["Exterior"]);
        assert_eq!(the_value_is_handed_back(&mut held, &a_server(), file, WhichSetting::Everything), None);
        assert_eq!(the_ticks(&held, &key), ["Interior"], "the reading's first scene");
    }

    /// **With no reading standing there is no file scene to go back to**, so
    /// the ticks are left as they are — here after the connected period ended
    /// and took the reading with it (`Render Settings` J10).
    #[test]
    fn reset_all_with_no_reading_leaves_the_ticks_as_they_are() {
        let (mut held, file, key) = a_run_that_read_a_file_saved_showing_interior();
        only_these_are_ticked(&mut held, &key, &["Exterior"]);
        the_period_ended_for(&mut held, &a_server());
        assert!(held.readings.standing(&key).is_none());

        assert_eq!(the_value_is_handed_back(&mut held, &a_server(), file, WhichSetting::Everything), None);

        assert_eq!(the_ticks(&held, &key), ["Exterior"]);
    }

    /// **`Design/Render Settings § Submitting`'s two refusals, word for word**
    /// (2026-10-01): *Not submitted. Type the frames.* where nothing was read,
    /// and *Not submitted. Type the frames or tick a scene.* where several
    /// scenes were read and none ticked. **Never *this file has not been
    /// read*** — the sentence the Sesterce pass met after every read.
    #[test]
    fn the_first_frame_check_that_cannot_name_its_frame_says_what_would() {
        use crate::submitting::WhatTheActCameTo as CameTo;
        let said = |a_scene_can_be_ticked| {
            the_words_of(&what_the_act_says(&CameTo::TheFirstFrameIsNotKnown { a_scene_can_be_ticked }))
        };
        assert_eq!(said(false), vec!["Not submitted.", "Type the frames."]);
        assert_eq!(said(true), vec!["Not submitted.", "Type the frames or tick a scene."]);
    }

    /// **`Design/Render Settings § Frames`: where the user's range stands, the
    /// heading's *the file's* is not drawn** (2026-10-01). The Sesterce pass
    /// saw *Frames — the file's* beside a typed `1-2`.
    #[test]
    fn the_frames_heading_says_the_files_only_beside_the_files_range() {
        let (mut held, file) = a_run_that_read_and_can_submit(vec![a_scene("Scene")]);
        let qualifier = |held: &WhatTheRunHolds| {
            the_panel_starting_from(held, &a_server(), file, &WhatTheRecordHolds::default())
                .frames_qualifier
                .map(|words| the_words_of(&vec![words]).join(" "))
        };
        assert_eq!(qualifier(&held).as_deref(), Some(words::THE_FILES_RANGE));
        assert!(the_answer_is_taken(&mut held, &a_server(), file, WhichSetting::FrameRange, "1-2").is_none());
        assert_eq!(qualifier(&held), None, "the user's range stands");
    }

    // ── The Creator's notes on the test copy, piece 3 ───────────────────────

    /// **Note 10: a submission every job of which went is the confirmation**
    /// — *Submitted to Vast 4090.* or *3 jobs submitted to Vast 4090.*, plain,
    /// carrying the job *Show in Queue* opens and its words
    /// (`Design/Render Settings § Submitting`, 2026-10-03). **A first-frame
    /// check's two jobs of one scene are one submission.** A part-submission, a
    /// refusal, and a lock keep what they said before.
    #[test]
    fn a_submission_that_went_is_the_confirmation_and_nothing_else_is() {
        use crate::submitting::{AJobAsSent, WhatBecameOfOneJob as Became, WhatTheActCameTo as CameTo};
        let job = Identity::issue(IdentityKind::Job).expect("randomness");
        let sent = |many: usize| -> Vec<AJobAsSent> {
            (0..many).map(|at| AJobAsSent { job: if at == 0 { job.clone() } else { Identity::issue(IdentityKind::Job).expect("randomness") }, of_the_scene_job: at }).collect()
        };
        let all_went = |many: usize| CameTo::TheseJobs(vec![Became::ItIsInTheLine; many]);
        let expected = |said: ASentence| {
            let plain = said.len();
            a_submission_said("s", "f", said, plain)
                .saying(THE_JOB_TO_SHOW, Said::Value(Verbatim::new(job.as_text())))
                .saying(SHOW_IN_QUEUE_SAID, Said::Prose(Prose::new(words::SHOW_IN_QUEUE)))
        };
        let one = vec![part(words::SUBMITTED_TO, vec!["Vast 4090".to_string()], Some(words::THE_FULL_STOP), true)];
        assert_eq!(the_words_of_a_line(&one), "Submitted to <Vast 4090>.");
        assert_eq!(
            a_submission_that_went("s", "f", "Vast 4090", 1, &all_went(1), &sent(1)),
            Some(expected(one.clone()))
        );
        // A first-frame check: two jobs of one scene-job.
        assert_eq!(
            a_submission_that_went("s", "f", "Vast 4090", 1, &all_went(2), &sent(2)),
            Some(expected(one))
        );
        let three = vec![
            a_figure("3", words::JOBS_SUBMITTED_TO),
            part("", vec!["Vast 4090".to_string()], Some(words::THE_FULL_STOP), true),
        ];
        assert_eq!(the_words_of_a_line(&three), " <3> jobs submitted to  <Vast 4090>.");
        assert_eq!(
            a_submission_that_went("s", "f", "Vast 4090", 3, &all_went(3), &sent(3)),
            Some(expected(three))
        );
        for not_all in [
            CameTo::TheseJobs(vec![Became::ItIsInTheLine, Became::NotKnown]),
            CameTo::TheseJobs(vec![Became::TheServerDoesNotTakeJobs]),
            CameTo::TheseJobs(Vec::new()),
            CameTo::TheFirstFrameIsNotKnown { a_scene_can_be_ticked: true },
        ] {
            assert_eq!(a_submission_that_went("s", "f", "Vast 4090", 2, &not_all, &sent(2)), None, "{not_all:?}");
        }
        assert_eq!(a_submission_that_went("s", "f", "Vast 4090", 1, &all_went(1), &[]), None, "no job to show");
        // **The command says it**, and `Render Settings` B1's list answers on its own box.
        let working = the_working_half();
        let command = working.split("pub fn a_job_is_submitted<").nth(1).expect("the command");
        let command = &command[..command.find("\n}\n").expect("it closes")];
        assert!(
            command.contains("let event = a_submission_that_went(&server, &file, &the_servers_name, how_many, &came_to, &named)\n"),
            "the submission's answer is not the confirmation where every job went"
        );
        assert!(command.contains("the_queue_is_told(&queue, event);"), "the confirmation is never told");
        let list = working.split("pub fn the_version_list_is_asked_for<").nth(1).expect("the list's command");
        let list = &list[..list.find("\n}\n").expect("it closes")];
        assert_eq!(list.matches("an_act_said(&server, &file, THE_LIST_CAME_BACK, Vec::new())").count(), 2);
        assert!(!list.contains("\"version\""));
        let page = the_page();
        for (name, value) in [
            ("THE_JOB_TO_SHOW", THE_JOB_TO_SHOW),
            ("SHOW_IN_QUEUE_SAID", SHOW_IN_QUEUE_SAID),
            ("THE_LIST_CAME_BACK", THE_LIST_CAME_BACK),
        ] {
            assert!(page.contains(&format!("const {name} = \"{value}\"")), "the page does not name {name}");
        }
    }

    /// **Simplify's three values take answers that reach the payload** —
    /// `Rendering` A1d's boxes and pick, each its own entry, the boxes refusing
    /// with the range alone and the pick refusing nothing. Until 2026-10-03
    /// the page drew all three as text (an owed fault of the 38's piece 2).
    #[test]
    fn simplifys_three_values_take_answers_that_reach_the_payload() {
        let file = "/ephemeral/smoozy/assets/kitchen_v14.blend";
        let mut held = a_run_with_a_reading();
        held.the_saved_with.insert(keyed(&a_server(), file), "4.5.3".to_string());
        the_page_came_from(
            &mut held,
            &a_server(),
            file,
            ThePageCameFrom {
                band: WhereItCameFrom::SubmittingAgain { the_jobs_name: "Kitchen".to_string(), when: String::new() },
                the_simplify_offer_is_drawn: true,
                the_entry: Identity::issue(IdentityKind::Job).expect("randomness"),
                the_entrys_label: "Kitchen".to_string(),
                the_moment: None,
            },
        );
        let take = |held: &mut WhatTheRunHolds, which, text: &str| the_answer_is_taken(held, &a_server(), file, which, text);
        assert_eq!(take(&mut held, WhichSetting::Simplify, "on"), None);
        assert_eq!(take(&mut held, WhichSetting::SimplifySubdivision, "3"), None);
        assert_eq!(take(&mut held, WhichSetting::SimplifyChildParticles, "0.25"), None);
        assert_eq!(take(&mut held, WhichSetting::SimplifyTextureLimit, "1024"), None);
        // **Refused in place, the range alone**, and nothing becomes an entry.
        for (which, typed, said) in [
            (WhichSetting::SimplifySubdivision, "40000", " <0> to  <32767>."),
            (WhichSetting::SimplifySubdivision, "2.5", " <0> to  <32767>."),
            (WhichSetting::SimplifyChildParticles, "1.5", " <0> to  <1>."),
        ] {
            let refused = what_a_simplify_box_refuses(which, typed).expect("refused");
            assert_eq!(the_words_of_a_line(&what_the_range_alone_says(refused)), said);
            assert_eq!(take(&mut held, which, typed), None);
        }
        assert_eq!(what_a_simplify_box_refuses(WhichSetting::SimplifyTextureLimit, "16384"), None);
        assert_eq!(take(&mut held, WhichSetting::SimplifyTextureLimit, "16384"), None);
        let panel = the_panel(&held, &a_server(), file);
        let simplify = panel.simplify.expect("the offer is drawn");
        let shown: Vec<(String, Option<String>)> = simplify
            .settings
            .iter()
            .map(|one| (one.whose.as_str().to_string(), one.value.as_ref().map(|value| value.as_str().to_string())))
            .collect();
        assert_eq!(
            shown,
            vec![
                ("the-users".to_string(), Some("3".to_string())),
                ("the-users".to_string(), Some("0.25".to_string())),
                ("the-users".to_string(), Some("1024".to_string())),
            ]
        );
        assert_eq!(
            simplify
                .texture_choices
                .iter()
                .map(|one| (one.value.as_str().to_string(), one.said.as_str().replace('\u{a0}', " ")))
                .collect::<Vec<_>>(),
            [("OFF", words::NO_LIMIT), ("128", "128"), ("256", "256"), ("512", "512"), ("1024", "1024"), ("2048", "2048"), ("4096", "4096"), ("8192", "8192")]
                .iter()
                .map(|(value, said)| (value.to_string(), said.to_string()))
                .collect::<Vec<_>>()
        );
        // **The cross names the file's own value in the pick's words.**
        let back = simplify.settings[2].hand_back_to.clone().expect("a cross");
        assert_eq!(
            the_words_of_a_line(&vec![back]).replace('\u{a0}', " "),
            format!("{} {}", words::BACK_TO_THE_FILES_VALUE, words::NO_LIMIT)
        );
        let row = a_row_on(&a_server(), true);
        let given = the_act_as_the_page_composed_it(&held, &a_server(), file, &WhatTheRecordHolds::default(), &row, a_run())
            .unwrap_or_else(|said| panic!("{said:?}"));
        let simplify = &given.scene_jobs[0].simplify;
        assert_eq!(simplify.use_simplify, Some(true));
        assert_eq!(simplify.subdivision_render, Some(3));
        assert_eq!(simplify.child_particles_render, Some(0.25));
        assert_eq!(simplify.texture_limit_render.as_deref(), Some("1024"));
        // **Each has its own cross.**
        assert_eq!(the_value_is_handed_back(&mut held, &a_server(), file, WhichSetting::SimplifySubdivision), None);
        let given = the_act_as_the_page_composed_it(&held, &a_server(), file, &WhatTheRecordHolds::default(), &row, a_run())
            .unwrap_or_else(|said| panic!("{said:?}"));
        assert_eq!(given.scene_jobs[0].simplify.subdivision_render, None);
        assert_eq!(given.scene_jobs[0].simplify.texture_limit_render.as_deref(), Some("1024"));
    }

    /// **The device tag's *last seen* moment is handed beside the tag**, as
    /// the very value the tag carries, so the page can write it as a date or a
    /// time. Until 2026-10-03 it was drawn as raw seconds.
    #[test]
    fn the_last_seen_moment_is_handed_as_the_value_the_tag_carries() {
        use smoozy_render_settings::readings::HeldState;
        use smoozy_servers::hardware::{Card, CardMake, Cards, HardwareReading, MachineMemory};
        use smoozy_state::observed::{Clock, Reading};
        use std::time::{Duration, SystemTime};
        let reading = HardwareReading {
            cards: Cards::TheCards(vec![Card {
                unique_identifier: "GPU-1".to_string(),
                bus_location: "0000:01:00.0".to_string(),
                make: CardMake::from_the_drivers_own_query("NVIDIA GeForce RTX 4090"),
                total_memory_bytes: None,
            }]),
            machine_memory: MachineMemory::TheAllocation(1 << 30),
            free_space: None,
            the_machines_own_moment: Reading::taken(
                (),
                SystemTime::UNIX_EPOCH + Duration::from_secs(1_700_000_000),
                Clock::AMachine(smoozy_state::observed::MachineClock("box".to_string())),
            ),
            the_desktops_own_moment: Reading::taken(
                (),
                SystemTime::UNIX_EPOCH + Duration::from_secs(1_700_000_050),
                Clock::ThisComputer,
            ),
            read_under: None,
            the_driver: None,
        };
        let answers = SceneEntries { device: Some(DeviceChoice::TheProcessor), ..SceneEntries::default() };
        let scene = a_scene("Interior");
        let (tag, moment) = the_device_tag(
            &answers,
            Some(&scene),
            Some(&HeldHardware::Held { state: HeldState::LastSeen, reading: reading.clone() }),
        );
        assert_eq!(moment.as_deref(), Some("1700000050"));
        assert_eq!(the_words_of_a_line(&tag), " <1> GPU unused last seen <1700000050>");
        let (_, live) = the_device_tag(
            &answers,
            Some(&scene),
            Some(&HeldHardware::Held { state: HeldState::Live, reading }),
        );
        assert_eq!(live, None, "a live reading names no moment");
    }

    /// **The page, under Node: the confirmation, Simplify's three controls, and
    /// *last seen* as a time.** The confirmation stands beside Render, opens
    /// the queue on its job, outlives `Render Settings` B1's list arriving, and goes at the next
    /// act on the page or the next submission. **Skipped, and says so, where no
    /// Node is installed.**
    #[test]
    fn the_page_draws_the_confirmation_simplifys_controls_and_last_seen_as_a_time() {
        let file = "/ephemeral/smoozy/assets/kitchen_v14.blend";
        let mut held = a_run_with_a_reading();
        the_page_came_from(
            &mut held,
            &a_server(),
            file,
            ThePageCameFrom {
                band: WhereItCameFrom::SubmittingAgain { the_jobs_name: "Kitchen".to_string(), when: String::new() },
                the_simplify_offer_is_drawn: true,
                the_entry: Identity::issue(IdentityKind::Job).expect("randomness"),
                the_entrys_label: "Kitchen".to_string(),
                the_moment: None,
            },
        );
        assert_eq!(the_answer_is_taken(&mut held, &a_server(), file, WhichSetting::Simplify, "on"), None);
        assert_eq!(the_answer_is_taken(&mut held, &a_server(), file, WhichSetting::SimplifyTextureLimit, "1024"), None);
        let mut panel = the_panel(&held, &a_server(), file);
        panel.renders_on = Some(TheDevice {
            label: Prose::new(words::RENDERS_ON),
            gpu: Prose::new(words::GPU),
            cpu: Prose::new(words::CPU),
            on: Verbatim::new("the-processor"),
            not_read_yet: None,
            tag: vec![a_figure("1", words::GPU_UNUSED), part(words::LAST_SEEN, vec!["1700000050".to_string()], None, false)],
            the_moment: Some(Verbatim::new("1700000050")),
            hand_back_to: None,
            greyed: false,
        });
        let went = serde_json::json!({
            "theBox": "submit", "partCount": "1", "lead0": words::SUBMITTED_TO, "value00": "Vast 4090",
            "rest0": words::THE_FULL_STOP, "joins0": "yes", THE_PLAIN_PARTS: "1",
            THE_JOB_TO_SHOW: "job-1", SHOW_IN_QUEUE_SAID: words::SHOW_IN_QUEUE,
        });
        let answers = serde_json::json!({ "panel": panel, "went": went, "listCameBack": THE_LIST_CAME_BACK });
        let folder = std::env::temp_dir().join(format!("smoozy-the-confirmation-page-{}", std::process::id()));
        let _ = fs::remove_dir_all(&folder);
        fs::create_dir_all(&folder).expect("a scratch folder");
        let web = Path::new(env!("CARGO_MANIFEST_DIR")).join("web");
        fs::copy(web.join("render-settings.js"), folder.join("render-settings.js")).expect("the page");
        for (name, text) in THE_UPLOADING_LINES_STAND_INS {
            fs::write(folder.join(name), text).expect("a stand-in");
        }
        fs::write(folder.join("answers.json"), answers.to_string()).expect("the answers");
        fs::write(folder.join("harness.mjs"), THE_CONFIRMATION_HARNESS.replace("SERVER", &a_server())).expect("the harness");
        let ran = std::process::Command::new("node").arg(folder.join("harness.mjs")).current_dir(&folder).output();
        let _ = fs::remove_dir_all(&folder);
        let Ok(ran) = ran else {
            eprintln!("SKIPPED: no Node on this computer, so the render settings page was not run");
            return;
        };
        assert!(ran.status.success(), "the page code did not run: {}", String::from_utf8_lossy(&ran.stderr));
        let got: serde_json::Value = serde_json::from_slice(&ran.stdout).expect("the harness prints one answer");
        assert_eq!(
            got,
            serde_json::json!({
                "beforeASubmission": false,
                "confirmation": "✓Submitted to Vast 4090. Show in Queue",
                // **Drawn after Render**, so the keyboard reaches Render first —
                // piece 5's owed tidy, done by the Creator's second notes,
                // piece 4 — **on the foot's second line, under Render**, the
                // Creator's tenth notes, note 8.
                "afterRender": true,
                "onTheSecondLine": true,
                "shown": [["queueview", "job-1"]],
                "afterTheList": true,
                "afterAnAnswer": false,
                "afterASecondSubmit": false,
                "textureChoices": ["No Limit", "128", "256", "512", "1024", "2048", "4096", "8192"],
                "textureShows": ["1024"],
                "answered": [["simplify-subdivision", "3"], ["simplify-texture-limit", "2048"]],
                "lastSeen": "1 GPU unused last seen at 1700000050"
            })
        );
    }

    /// Opens the panel, tells it a submission went, presses *Show in Queue*,
    /// then tells it the list came back, an answer, and a second submission;
    /// then answers Simplify's box and pick, and reads the device tag.
    const THE_CONFIRMATION_HARNESS: &str = r#"import { everyPiece } from "./document.mjs";
import { readFileSync } from "node:fs";
const answers = JSON.parse(readFileSync("./answers.json", "utf8"));
const invoked = [];
globalThis.window = { addEventListener() {}, __TAURI__: { core: { invoke(command, args) {
  invoked.push([command, args || null]);
  if (command === "the_render_settings_panel") return Promise.resolve(answers.panel);
  return Promise.resolve(null);
} } } };
const { panels } = await import("./a-server.js");
const { theSettingsAreOpenedOn } = await import("./render-settings.js");
const settle = async () => { for (let at = 0; at < 8; at += 1) await new Promise((go) => setImmediate(go)); };
await theSettingsAreOpenedOn("SERVER", Array.from(Buffer.from(answers.panel.file)), "");
const head = document.createElement("div");
const body = document.createElement("div");
panels[0].itIsDrawn({ head, body }, "SERVER");
await settle();
const find = (box) => everyPiece(body).find((one) => one.dataset.box === box);
const words = (piece) => (piece ? piece.textContent.replace(/ /g, " ").replace(/\s+/g, " ").trim() : null);
const tell = async (said) => {
  for (const handler of globalThis.told["render-settings:an-act"] || []) await handler({ said: { server: "SERVER", file: answers.panel.file, ...said } });
  await settle();
};
const out = { beforeASubmission: Boolean(find("submitted")) };
await tell(answers.went);
out.confirmation = words(find("submitted"));
const inOrder = everyPiece(everyPiece(body).find((one) => one.names.has("sfoot")));
out.afterRender = inOrder.indexOf(find("submitted")) > inOrder.indexOf(find("render"));
out.onTheSecondLine = find("submitted").parentNode.names.has("sfl2");
find("show-in-queue").press();
out.shown = globalThis.went || [];
await tell({ theBox: answers.listCameBack });
out.afterTheList = Boolean(find("submitted"));
await tell({ theBox: "range" });
out.afterAnAnswer = Boolean(find("submitted"));
await tell(answers.went);
await tell({ theBox: "submit", partCount: "1", lead0: "Submitting.", plainParts: "1" });
out.afterASecondSubmit = Boolean(find("submitted"));
const subdivision = find("simplify-subdivision-field");
subdivision.value = "3";
for (const handler of subdivision.listeners.change || []) handler({});
// Smoozy's own dropdown (the Creator's fifth notes, piece U): the rows are
// the list's, the picked one in bold, and a pick is a press on a row.
const texture = find("simplify-texture-limit-field");
const textureRows = texture.parentNode.children.find((one) => one.names && one.names.has("ddl")).children;
out.textureChoices = textureRows.map((one) => one.textContent.replace(/\u00a0/g, " "));
out.textureShows = textureRows.filter((one) => one.names.has("hd")).map((one) => one.textContent);
texture.press();
textureRows.find((one) => one.textContent === "2048").press();
out.answered = invoked.filter(([command]) => command === "an_answer_was_given").map(([, args]) => [args.which, args.text]);
out.lastSeen = words(everyPiece(find("device-the-processor")).find((one) => one.names.has("tag")));
console.log(JSON.stringify(out));
"#;

    // ── The tenth notes, note 8: the foot as drawn ──

    /// **The foot is two lines, every group aligned to the top, as the drawing
    /// draws `res-foot`** — `Design/Render Settings § How the three sit
    /// together`, the Creator's tenth notes, note 8, approved 2026-10-17.
    /// Measured on a page a real browser draws with the application's own
    /// stylesheets and this panel's own script, at a wide panel and at the
    /// 1,180 step:
    ///
    /// - the name's field, the download switch and Render share line 1;
    /// - *Render one frame first* and Telegram sit on line 2, under it;
    /// - Telegram's dropdown is only as wide as its longest choice — narrower
    ///   than its column, by the drawing's class and never a written width;
    /// - a submission that went sits under Render, right-aligned in Render's
    ///   column, and **Render does not move when it appears**;
    /// - the third line is drawn only when something is wrong or under way,
    ///   and then under the switches, with *Uploading dependencies* under
    ///   Render;
    /// - at the 1,180 step the switches drop under the name and Render stays
    ///   on the name's line.
    ///
    /// A computer with no Chrome skips it, as one with no Node skips the page
    /// runs above.
    #[test]
    fn the_foot_is_two_lines_aligned_to_the_top_as_drawn() {
        let chrome = [
            "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
            "/Applications/Chromium.app/Contents/MacOS/Chromium",
            "/usr/bin/google-chrome",
            "/usr/bin/chromium",
            "/usr/bin/chromium-browser",
            "C:\\Program Files\\Google\\Chrome\\Application\\chrome.exe",
        ]
        .into_iter()
        .map(std::path::PathBuf::from)
        .find(|one| one.exists());
        let Some(chrome) = chrome else {
            eprintln!("SKIPPED: no Chrome on this computer, so the foot was not measured");
            return;
        };

        let file = "/ephemeral/smoozy/assets/kitchen_v14.blend";
        let mut held = a_run_with_a_reading();
        let _ = the_answer_is_taken(&mut held, &a_server(), file, WhichSetting::DownloadAsTheyRender, "on");
        // A folder Smoozy can name and Telegram on: nothing is wrong or under
        // way, so there is no third line.
        let plain = the_panel_starting_from(
            &held,
            &a_server(),
            file,
            &WhatTheRecordHolds {
                telegram_is_on: true,
                the_proposed_folder: Some(b"/Users/artist/Renders/kitchen".to_vec()),
                ..WhatTheRecordHolds::default()
            },
        );
        assert!(plain.the_telegram_line_is_drawn && plain.the_frames_have_nowhere_to_go.is_none());
        // No folder anywhere, and the check's files going: two lines for the
        // third line, one under the switches and one under Render.
        let third = the_panel_starting_from(
            &held,
            &a_server(),
            file,
            &WhatTheRecordHolds {
                telegram_is_on: true,
                the_checks_uploads: Some((4, 12)),
                ..WhatTheRecordHolds::default()
            },
        );
        assert!(third.the_frames_have_nowhere_to_go.is_some() && third.the_dependencies_uploading.is_some());
        let went = serde_json::json!({
            "theBox": "submit", "partCount": "1", "lead0": words::SUBMITTED_TO, "value00": "Vast 4090",
            "rest0": words::THE_FULL_STOP, "joins0": "yes", THE_PLAIN_PARTS: "1",
            THE_JOB_TO_SHOW: "job-1", SHOW_IN_QUEUE_SAID: words::SHOW_IN_QUEUE,
        });
        let mut off = serde_json::to_value(&plain).unwrap();
        off["the_download_is_on"] = serde_json::json!(false);
        off["the_format_summary"] = serde_json::Value::Null;
        off["where_they_go"] = serde_json::Value::Null;
        let mut one_frame = off.clone();
        one_frame["the_first_frame_is_asked_for"] = serde_json::json!(true);
        one_frame["the_first_frame_switch_is_greyed"] = serde_json::json!(true);
        let mut telegram_off = serde_json::to_value(&plain).unwrap();
        telegram_off["the_reporting_choice"] = serde_json::json!("nothing");
        telegram_off["the_reporting_control_reads"] = serde_json::json!("Off");
        telegram_off["the_first_frame_is_answered_in_queue"] = serde_json::json!(words::ANSWER_IN_QUEUE);
        let answers = serde_json::json!({ "plain": plain, "off": off, "oneFrame": one_frame,
            "telegramOff": telegram_off, "third": third, "went": went });

        let folder = std::env::temp_dir().join(format!("smoozy-the-foot-measured-{}", std::process::id()));
        let _ = fs::remove_dir_all(&folder);
        fs::create_dir_all(&folder).expect("a scratch folder");
        let web = Path::new(env!("CARGO_MANIFEST_DIR")).join("web");
        // **Every stylesheet the application ships**, through its own one
        // link, so the cascade measured is the application's.
        for entry in fs::read_dir(&web).expect("the page folder") {
            let path = entry.expect("an entry").path();
            if path.extension().and_then(|one| one.to_str()) == Some("css") {
                fs::copy(&path, folder.join(path.file_name().expect("a name"))).expect("a stylesheet is copied");
            }
        }
        fs::copy(web.join("render-settings.js"), folder.join("render-settings.js")).expect("the page");
        for (name, text) in THE_UPLOADING_LINES_STAND_INS {
            fs::write(folder.join(name), text).expect("a stand-in");
        }
        fs::write(folder.join("answers.js"), format!("export default {answers};"))
            .expect("the answers");
        fs::write(folder.join("index.html"), THE_MEASURED_FOOT.replace("SERVER", &a_server())).expect("the page");
        let dumped = folder.join("dumped.html");
        let mut chrome = std::process::Command::new(chrome)
            .args([
                "--headless",
                "--disable-gpu",
                "--no-first-run",
                "--allow-file-access-from-files",
                "--virtual-time-budget=5000",
                "--window-size=1400,900",
                "--dump-dom",
            ])
            .arg(format!("--user-data-dir={}", folder.join("profile").display()))
            .arg(format!("file://{}", folder.join("index.html").display()))
            .stdout(fs::File::create(&dumped).expect("the dump is written"))
            .stderr(std::process::Stdio::null())
            .spawn()
            .expect("Chrome starts");
        // A fixed deadline, never an open wait: a browser that has not drawn
        // the page in a minute is stopped.
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(60);
        while chrome.try_wait().expect("Chrome is watched").is_none() {
            if std::time::Instant::now() > deadline {
                let _ = chrome.kill();
                let _ = chrome.wait();
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
        let page = fs::read_to_string(&dumped).unwrap_or_default();
        let _ = fs::remove_dir_all(&folder);
        let said = page
            .split("<pre id=\"out\">")
            .nth(1)
            .and_then(|rest| rest.split("</pre>").next())
            .filter(|said| !said.is_empty())
            .unwrap_or_else(|| panic!("the page drew no measurement: {page}"));
        let said: serde_json::Value = serde_json::from_str(said).expect("one JSON answer");
        let at = |case: &str, part: &str, edge: &str| -> f64 {
            said[case][part][edge]
                .as_f64()
                .unwrap_or_else(|| panic!("no {edge} for {part} in {case}: {said}"))
        };
        let middle = |case: &str, part: &str| (at(case, part, "top") + at(case, part, "bottom")) / 2.0;
        let near = |a: f64, b: f64| (a - b).abs() <= 1.5;

        // Notes 2–4: the actual page, through every shipped stylesheet.
        for (on, off) in [("wide", "wideOff"), ("narrow", "narrowOff")] {
            assert!(near(at(on, "foot", "height"), at(off, "foot", "height")), "download grew the foot: {said}");
            for part in ["downloadSwitch", "firstFrameSwitch", "render"] {
                assert!(near(at(on, part, "top"), at(off, part, "top")), "download moved {part}: {said}");
            }
        }
        assert_eq!(said["oneFrame"]["firstFramePressed"], "false", "{said}");
        assert_eq!(said["oneFrame"]["firstFrameDisabled"], true, "{said}");
        assert_eq!(said["telegramOff"]["queueNote"], words::ANSWER_IN_QUEUE, "{said}");
        assert!(said["telegramOff"]["report"].as_str().unwrap().starts_with("Off"), "{said}");

        // **Line 1: the name's field, the download switch and Render.**
        for case in ["wide", "narrow"] {
            assert!(near(middle(case, "field"), middle(case, "render")), "{case}: the name's field and Render are not on one line: {said}");
        }
        assert!(near(middle("wide", "field"), middle("wide", "downloadSwitch")), "the download switch is not on the name's line: {said}");
        assert!(near(middle("wide", "downloadSwitch"), middle("wide", "summary")), "the summary is not beside its switch: {said}");
        // **Line 2: Render one frame first, then Telegram, under line 1.**
        for part in ["field", "downloadSwitch", "render"] {
            assert!(at("wide", "firstFrameSwitch", "top") >= at("wide", part, "bottom") - 1.0, "line 2 is not under the {part}: {said}");
        }
        assert!(near(middle("wide", "firstFrameSwitch"), middle("wide", "telegram")), "Telegram is not on the first-frame switch's line: {said}");
        assert!(at("wide", "telegram", "left") > at("wide", "firstFrameSwitch", "right"), "Telegram is not after the first-frame switch: {said}");
        // The two switches start in one column, beside the name.
        assert!(near(at("wide", "downloadSwitch", "left"), at("wide", "firstFrameSwitch", "left")), "the switches do not line up: {said}");
        assert!(at("wide", "downloadSwitch", "left") > at("wide", "field", "right"), "the switches are not beside the name: {said}");
        // **Telegram's dropdown is only as wide as its longest choice.**
        let dropdown = at("wide", "telegram", "right") - at("wide", "telegram", "left");
        let column = at("wide", "secondLine", "right") - at("wide", "secondLine", "left");
        assert!(dropdown < column / 2.0, "Telegram's dropdown is {dropdown} wide in a column of {column}: {said}");
        assert_eq!(said["wide"]["telegramWidthWritten"], false, "Telegram's dropdown carries a width written on it: {said}");
        assert_eq!(said["wide"]["telegramClass"], "pick w-tg", "Telegram's dropdown lost the drawing's class: {said}");
        // **No third line where nothing is wrong or under way.**
        assert_eq!(said["wide"]["thirdLine"], false, "a third line is drawn with nothing to say: {said}");

        // **A submission that went: under Render, right-aligned in Render's
        // column, and nothing moved.**
        assert!(near(at("sent", "sent", "right"), at("sent", "render", "right")), "the submitted line does not end at Render's right edge: {said}");
        assert!(at("sent", "sent", "top") >= at("sent", "render", "bottom") - 1.0, "the submitted line is not under Render: {said}");
        assert!(near(middle("sent", "sent"), middle("sent", "firstFrameSwitch")), "the submitted line is not on line 2: {said}");
        for edge in ["top", "left", "right", "bottom"] {
            assert!(near(at("sent", "render", edge), at("wide", "render", edge)), "Render moved when the submission went ({edge}): {said}");
        }
        assert!(near(at("sent", "foot", "height"), at("wide", "foot", "height")), "the foot grew when the submission went: {said}");

        // **The third line, when there is something to say.**
        assert_eq!(said["third"]["thirdLine"], true, "the third line is not drawn: {said}");
        assert!(at("third", "noFolder", "top") >= at("third", "firstFrameSwitch", "bottom"), "the no-folder line is not under line 2: {said}");
        assert!(near(at("third", "noFolder", "left"), at("third", "firstFrameSwitch", "left")), "the no-folder line is not under the switches: {said}");
        assert!(at("third", "uploading", "top") >= at("third", "firstFrameSwitch", "bottom"), "the uploading line is not under line 2: {said}");
        assert!(near(at("third", "uploading", "right"), at("third", "render", "right")), "the uploading line is not under Render: {said}");
        assert!(at("third", "foot", "height") > at("wide", "foot", "height"), "the third line takes no room: {said}");

        // **At the 1,180 step the switches drop under the name**, and Render
        // stays on the name's line.
        assert!(at("narrow", "downloadSwitch", "top") >= at("narrow", "field", "bottom"), "the switches did not drop under the name: {said}");
        assert!(at("narrow", "firstFrameSwitch", "top") >= at("narrow", "downloadSwitch", "bottom") - 1.0, "line 2 is not under line 1 at the step: {said}");
        assert!(near(at("narrow", "downloadSwitch", "left"), at("narrow", "label", "left")), "the switches do not start under the name: {said}");
    }

    /// **A page that draws the panel three ways and measures its foot**, and
    /// writes what it found into `#out`. The frame stands in for the panel's
    /// place in the window; `.app` carries the step mark `web/panes.js` writes.
    const THE_MEASURED_FOOT: &str = r#"<!doctype html><html><head><meta charset="utf-8">
<link rel="stylesheet" href="smoozy.css">
<style>
  html, body { height: auto; overflow: visible; }
  .app { display: block; }
  .frame { width: 1100px; }
  .app[data-narrow~="s1"] .frame { width: 940px; }
</style>
</head><body>
<div class="app" id="app"><div class="frame" data-panel="set"><div class="sheet"><div class="shead"><div class="htitle" id="head"></div></div><div class="suscroll" id="body"></div></div></div></div>
<pre id="out"></pre>
<script>
  window.theAnswer = "plain";
  window.__TAURI__ = { core: { invoke(command) {
    if (command === "the_render_settings_panel") return Promise.resolve(structuredClone(window.theAnswers[window.theAnswer]));
    return Promise.resolve(null);
  } } };
</script>
<script type="module">
// **Imported, never fetched by a call**: headless Chrome under a virtual
// clock left a module fetched by `import()` unanswered about half the time,
// and a static graph is loaded before the page is said to be loaded.
import answers from "./answers.js";
import { panels } from "./a-server.js";
import { theSettingsAreOpenedOn } from "./render-settings.js";
window.theAnswers = answers;
// What went wrong is written where the measurement would be, so the test's
// message says it.
window.addEventListener("unhandledrejection", (event) => { document.getElementById("out").textContent = JSON.stringify({ error: String(event.reason && event.reason.stack || event.reason) }); });
const settle = async () => { for (let at = 0; at < 8; at += 1) await new Promise((go) => setTimeout(go, 0)); };
const head = document.getElementById("head");
const body = document.getElementById("body");
await theSettingsAreOpenedOn("SERVER", Array.from(new TextEncoder().encode(answers.plain.file)), "");
panels[0].itIsDrawn({ head, body }, "SERVER");
await settle();
const tell = async (said) => {
  for (const handler of globalThis.told["render-settings:an-act"] || []) await handler({ said: { server: "SERVER", file: answers.plain.file, ...said } });
  await settle();
};
const box = (name) => body.querySelector(`[data-box="${name}"]`);
const edges = (part) => {
  if (!part) return null;
  const at = part.getBoundingClientRect();
  return { top: at.top, bottom: at.bottom, left: at.left, right: at.right, height: at.height };
};
const measure = () => {
  const foot = body.querySelector(".sfoot");
  const telegram = box("reports") ? box("reports").parentNode : null;
  return {
    foot: edges(foot),
    label: edges(foot.querySelector("label")),
    field: edges(box("job-name")),
    downloadSwitch: edges(box("download-switch")),
    summary: edges(box("download-choices")),
    render: edges(box("render")),
    firstFrameSwitch: edges(box("first-frame-switch")),
    firstFramePressed: box("first-frame-switch").getAttribute("aria-pressed"),
    firstFrameDisabled: box("first-frame-switch").disabled,
    queueNote: box("first-frame-in-queue")?.textContent || null,
    report: box("reports")?.textContent.replace(/\u00a0/g, " "),
    telegram: edges(telegram),
    telegramClass: telegram ? telegram.className : null,
    telegramWidthWritten: Boolean(telegram && telegram.getAttribute("style")),
    secondLine: edges(foot.querySelector(".sfl2")),
    sent: edges(box("submitted")),
    thirdLine: Boolean(foot.querySelector(".sfl3")),
    noFolder: edges(box("no-folder")),
    uploading: edges(box("dependencies-uploading") ? box("dependencies-uploading").parentNode : null),
  };
};
const out = {};
out.wide = measure();
await tell(answers.went);
out.sent = measure();
window.theAnswer = "third";
await tell({ theBox: "range" });
out.third = measure();
window.theAnswer = "plain";
document.getElementById("app").dataset.narrow = "s1";
await tell({ theBox: "range" });
out.narrow = measure();
window.theAnswer = "off";
await tell({ theBox: "range" });
out.narrowOff = measure();
document.getElementById("app").dataset.narrow = "";
await tell({ theBox: "range" });
out.wideOff = measure();
window.theAnswer = "oneFrame";
await tell({ theBox: "range" });
out.oneFrame = measure();
window.theAnswer = "telegramOff";
await tell({ theBox: "range" });
out.telegramOff = measure();
document.getElementById("out").textContent = JSON.stringify(out);
</script></body></html>"#;

    // ── The sixteenth notes, notes 3 and 4: the Telegram choice and the switch ──

    /// **The run the two output kinds are drawn against**: one file read over
    /// 300 frames, writing pictures, or — the same file with its output path
    /// set to FFmpeg — writing only a video.
    fn a_run_writing(video: bool) -> (WhatTheRunHolds, &'static str) {
        use smoozy_job_files::reading::{ArtistsText, DestinationKind, Format};
        let (mut held, file, _) = a_run_over_three_hundred_frames();
        if video {
            for (_, outputs) in held.outputs.get_mut(&keyed(&a_server(), file)).expect("the outputs") {
                for destination in &mut outputs.destinations {
                    if matches!(destination.kind, DestinationKind::TheRegularOutputPath) {
                        destination.format = Some(Format {
                            as_the_file_records_it: ArtistsText::of(b"FFMPEG".to_vec()),
                            the_ending_it_writes: Some(ArtistsText::of(b".mp4".to_vec())),
                            is_video_shaped: true,
                        });
                    }
                }
            }
        }
        (held, file)
    }

    /// **The sixteenth notes, notes 3 and 4**: *"There is no option to send
    /// anything via Telegram if a job only writes a video file"*, and *"I don't
    /// see options for Telegram reporting in Render settings for an
    /// image-outputting file, too."* `Design/Render Settings § How the three
    /// sit together`: the choice is there whenever Telegram is on, whatever the
    /// job writes, and *"Where Telegram is switched off it is not there at
    /// all."*
    ///
    /// **Traced from the record to the foot, in a real browser.** The record is
    /// set up as a user sets Telegram up — the bot checked and its proof
    /// recorded, its token stored, a server told, the switch on — then switched
    /// off as Settings switches it off, and set up again. Each state is read
    /// the way the page's command reads it (`WhatTheRecordHolds::read_now`)
    /// and composed for a job writing pictures and a job writing only a video.
    ///
    /// **The page is opened before the switch moves, and nothing on it is
    /// pressed after**, which is the case the page lost: Settings' Telegram
    /// section was told of the switch and the open page was not, so the foot
    /// went on showing the switch as it stood when the page was drawn — no
    /// choice after Telegram was switched on, and one after it was switched
    /// off — until something on the page itself was acted on. **It fails if the
    /// foot does not follow the switch when Telegram's section is told.**
    #[test]
    fn the_telegram_choice_follows_the_switch_for_pictures_and_a_video_in_a_real_browser() {
        use crate::record::telegram_bot::TheProvedBotRecord;
        use smoozy_telegram::proof::{KeepsTheProvedBot, TheBot, TheConversation};
        let Some(chrome) = a_chrome() else {
            eprintln!("SKIPPED: no Chrome on this computer, so the Telegram choice was not drawn");
            return;
        };

        // **The record, as a user sets Telegram up.**
        let place = std::env::temp_dir().join(format!("smoozy-telegram-choice-record-{}", std::process::id()));
        let _ = fs::remove_dir_all(&place);
        fs::create_dir_all(&place).expect("a folder");
        let record = crate::record::LocalRecord::open_at(&place.join("local-record.sqlite3")).expect("a record");
        let c = record.connection();
        let a_bot_is_kept = |id: i64| {
            let bot = TheBot { id, username: format!("smoozy_{id}_bot") };
            let keeper = TheProvedBotRecord::over(c);
            keeper.the_proof_was_recorded(&bot, TheConversation(296_269_070)).expect("the proof is recorded");
            keeper.the_token_was_stored_for(&bot).expect("the token is recorded as stored");
            crate::record::settings::telegram_was_switched(c, true).expect("switched on");
        };
        let before = WhatTheRecordHolds::read_now(Some(&record));
        a_bot_is_kept(8_720_107_572);
        crate::record::told_record::the_telling_was_acknowledged(
            c,
            &a_server(),
            &smoozy_telegram::told::WhatSmoozyTells::the_roles_desired_state(true),
            std::time::SystemTime::now(),
            "an-installation",
        )
        .expect("the server is told");
        let on = WhatTheRecordHolds::read_now(Some(&record));
        // **Switched off as Settings switches it off**: the switch alone (the
        // token is kept since 2026-10-31, the fifteenth notes, note 1).
        let _ = crate::surface::telegram::telegram_is_switched_off_in(c);
        let off = WhatTheRecordHolds::read_now(Some(&record));
        a_bot_is_kept(8_720_107_573);
        let again = WhatTheRecordHolds::read_now(Some(&record));
        drop(record);
        let _ = fs::remove_dir_all(&place);
        assert!(!before.telegram_is_on && on.telegram_is_on && !off.telegram_is_on && again.telegram_is_on);

        // **Composed for both output kinds**, through the page's own composer.
        let mut answers = serde_json::Map::new();
        for (kind, video) in [("pictures", false), ("video", true)] {
            for (state, read) in [("before", &before), ("on", &on), ("off", &off), ("again", &again)] {
                let (held, file) = a_run_writing(video);
                let panel = the_panel_starting_from(&held, &a_server(), file, read);
                assert_eq!(panel.the_telegram_line_is_drawn, read.telegram_is_on, "{kind} {state}");
                let writes = panel.what_it_will_write.as_ref().expect("the file is read, so the list is drawn");
                let formats: Vec<&str> = writes.rows.iter().filter_map(|row| row.format.as_ref().map(|one| one.as_str())).collect();
                assert_eq!(formats.iter().all(|one| *one == "Video"), video, "{kind}: the job writes {formats:?}");
                answers.insert(format!("{kind}-{state}"), serde_json::to_value(&panel).expect("the panel serialises"));
            }
        }
        let answers = serde_json::Value::Object(answers);

        let folder = std::env::temp_dir().join(format!("smoozy-telegram-choice-browser-{}", std::process::id()));
        let _ = fs::remove_dir_all(&folder);
        fs::create_dir_all(&folder).expect("a scratch folder");
        let web = Path::new(env!("CARGO_MANIFEST_DIR")).join("web");
        for entry in fs::read_dir(&web).expect("the page folder") {
            let path = entry.expect("an entry").path();
            if path.extension().and_then(|one| one.to_str()) == Some("css") {
                fs::copy(&path, folder.join(path.file_name().expect("a name"))).expect("a stylesheet is copied");
            }
        }
        fs::copy(web.join("render-settings.js"), folder.join("render-settings.js")).expect("the page");
        for (name, text) in THE_UPLOADING_LINES_STAND_INS {
            fs::write(folder.join(name), text).expect("a stand-in");
        }
        fs::write(folder.join("answers.js"), format!("export default {answers};")).expect("the answers");
        fs::write(folder.join("index.html"), THE_TELEGRAM_CHOICE_FOLLOWS_THE_SWITCH.replace("SERVER", &a_server()))
            .expect("the page");
        let dumped = folder.join("dumped.html");
        let mut browser = std::process::Command::new(chrome)
            .args([
                "--headless",
                "--disable-gpu",
                "--no-first-run",
                "--allow-file-access-from-files",
                "--virtual-time-budget=5000",
                "--window-size=1400,900",
                "--dump-dom",
            ])
            .arg(format!("--user-data-dir={}", folder.join("profile").display()))
            .arg(format!("file://{}", folder.join("index.html").display()))
            .stdout(fs::File::create(&dumped).expect("the dump is written"))
            .stderr(std::process::Stdio::null())
            .spawn()
            .expect("Chrome starts");
        // A fixed deadline, never an open wait.
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(60);
        while browser.try_wait().expect("Chrome is watched").is_none() {
            if std::time::Instant::now() > deadline {
                let _ = browser.kill();
                let _ = browser.wait();
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
        let page = fs::read_to_string(&dumped).unwrap_or_default();
        let _ = fs::remove_dir_all(&folder);
        let said = page
            .split("<pre id=\"out\">")
            .nth(1)
            .and_then(|rest| rest.split("</pre>").next())
            .filter(|said| !said.is_empty())
            .unwrap_or_else(|| panic!("the page drew nothing to read: {page}"));
        let said: serde_json::Value = serde_json::from_str(said).expect("one JSON answer");
        assert!(said.get("error").is_none(), "the page failed: {said}");

        for kind in ["pictures", "video"] {
            // **Drawn while Telegram was off: no choice.**
            assert_eq!(said[kind]["drawnOff"]["there"], false, "{kind}: a choice drawn while Telegram was off: {said}");
            // **Switched on, with the page open and nothing pressed on it.**
            let shown = &said[kind]["switchedOn"];
            assert_eq!(shown["there"], true, "{kind}: no Telegram choice after Telegram was switched on: {said}");
            assert_eq!(shown["seen"], true, "{kind}: the choice is not inside the foot's second line: {said}");
            assert_eq!(shown["label"], words::TELEGRAM, "{kind}: {said}");
            assert_eq!(shown["reads"], words::EVERYTHING, "{kind}: {said}");
            // **Switched off: gone. Set up again: back.**
            assert_eq!(said[kind]["switchedOff"]["there"], false, "{kind}: the choice stayed after Telegram was switched off: {said}");
            assert_eq!(said[kind]["onAgain"]["there"], true, "{kind}: no choice after Telegram was set up again: {said}");
            assert_eq!(said[kind]["onAgain"]["seen"], true, "{kind}: {said}");
            // **A page opened with Telegram on draws it at once.**
            assert_eq!(said[kind]["openedOn"]["there"], true, "{kind}: a page opened with Telegram on has no choice: {said}");
            assert_eq!(said[kind]["openedOn"]["seen"], true, "{kind}: {said}");
        }
    }

    /// **The page opened once on the server, Telegram's switch moved under it,
    /// and Telegram's section told — as `src/surface/telegram.rs` tells it
    /// after every switch** (`THE_SECTION`, `telegram:the-section`). Nothing
    /// on the page is pressed between, so only that can redraw the foot.
    const THE_TELEGRAM_CHOICE_FOLLOWS_THE_SWITCH: &str = r#"<!doctype html><html><head><meta charset="utf-8">
<link rel="stylesheet" href="smoozy.css">
<style>
  html, body { height: auto; overflow: visible; }
  .app { display: block; }
  .frame { width: 1100px; }
</style>
</head><body>
<div class="app" id="app"><div class="frame" data-panel="set"><div class="sheet"><div class="shead"><div class="htitle" id="head"></div></div><div class="suscroll" id="body"></div></div></div></div>
<pre id="out"></pre>
<script>
  window.theAnswer = "pictures-before";
  window.__TAURI__ = { core: { invoke(command) {
    if (command === "the_render_settings_panel") return Promise.resolve(structuredClone(window.theAnswers[window.theAnswer]));
    return Promise.resolve(null);
  } } };
</script>
<script type="module">
import answers from "./answers.js";
import { panels } from "./a-server.js";
import { theSettingsAreOpenedOn } from "./render-settings.js";
window.theAnswers = answers;
window.addEventListener("unhandledrejection", (event) => { document.getElementById("out").textContent = JSON.stringify({ error: String(event.reason && event.reason.stack || event.reason) }); });
const settle = async () => { for (let at = 0; at < 8; at += 1) await new Promise((go) => setTimeout(go, 0)); };
const head = document.getElementById("head");
const body = document.getElementById("body");
const file = Array.from(new TextEncoder().encode(answers["pictures-before"].file));
const telegramIsTold = async (answer) => {
  window.theAnswer = answer;
  for (const handler of globalThis.told["telegram:the-section"] || []) await handler({ said: {} });
  await settle();
};
const look = () => {
  const face = body.querySelector('[data-box="reports"]');
  if (!face) return { there: false };
  const line = face.closest(".sftg");
  const second = body.querySelector(".sfoot .sfl2");
  const at = line.getBoundingClientRect();
  const within = second.getBoundingClientRect();
  return {
    there: true,
    seen: second.contains(line) && at.width > 0 && at.height > 0 && getComputedStyle(line).visibility === "visible"
      && at.left >= within.left && at.right <= within.right && at.top >= within.top && at.bottom <= within.bottom,
    label: line.querySelector(".hintl").textContent,
    reads: face.textContent,
  };
};
const out = {};
let drawn = false;
for (const kind of ["pictures", "video"]) {
  window.theAnswer = `${kind}-before`;
  await theSettingsAreOpenedOn("SERVER", file, "");
  if (!drawn) { panels[0].itIsDrawn({ head, body }, "SERVER"); drawn = true; }
  await settle();
  const seen = { drawnOff: look() };
  await telegramIsTold(`${kind}-on`);
  seen.switchedOn = look();
  await telegramIsTold(`${kind}-off`);
  seen.switchedOff = look();
  await telegramIsTold(`${kind}-again`);
  seen.onAgain = look();
  window.theAnswer = `${kind}-on`;
  await theSettingsAreOpenedOn("SERVER", file, "");
  await settle();
  seen.openedOn = look();
  out[kind] = seen;
}
document.getElementById("out").textContent = JSON.stringify(out);
</script></body></html>"#;

    // ── The Creator's second notes, piece 2: Render Settings says it truly ──

    /// **A stand-in for the Creator's file of their second notes** (notes 14,
    /// 16 and 18), as its reading has it: an FFmpeg video at
    /// `//../Render/Animation`; a PNG node under `//../Render/PNG` with one
    /// output per view layer, `### - Hero - <layer>`; two multi-layer EXR nodes
    /// under `//../Render/EXR` fed by *Box* and *Mattress*; the bake output path
    /// at `//`, as a fresh file records it, and the other four added places
    /// with no value; four view layers, all four on in the file. One frame.
    pub(crate) fn the_creators_shape() -> TheOutputsHalf {
        use smoozy_job_files::reading::{
            AnAddedWritePlace, ArtistsText, AtOneFrame, Destination, DestinationKind, Format, RecordedLocation,
            WhichScene, EVERY_ADDED_WRITE_PLACE,
        };
        let text = |said: &str| ArtistsText::of(said.as_bytes().to_vec());
        let a_format = |name: &str, ending: &str, video: bool| Format {
            as_the_file_records_it: text(name),
            the_ending_it_writes: Some(text(ending)),
            is_video_shaped: video,
        };
        let layers = ["Box", "Mattress", "Box AO", "Mattress AO"];
        let mut outputs = crate::servers::the_machines_work::tests::a_recorded_reading(
            "/ephemeral/smoozy/jobs/job",
            &[1],
            smoozy_job_files::reading::HowManyViewsTheSceneWrites::One,
        )
        .reading
        .outputs;
        outputs.gating.a_compositing_tree_is_present = true;
        outputs.gating.a_live_final_output_node = true;
        let mut destinations = vec![Destination {
            kind: DestinationKind::TheRegularOutputPath,
            recorded_location: Some(RecordedLocation::of(b"//../Render/Animation".to_vec())),
            format: Some(a_format("FFMPEG", ".mp4", true)),
            origin: WhereItComesFrom::TheScenesOutputPath,
            scene: WhichScene::TheJobsOwnScene,
            the_view_layers_that_feed_it: layers.iter().map(|layer| text(layer)).collect(),
            the_location_with_its_folder_replaced: None,
        }];
        for layer in layers {
            destinations.push(Destination {
                kind: DestinationKind::AnOutputOfAFileOutputNode {
                    node: text("PNG"),
                    output: text(&format!("### - Hero - {layer}")),
                    the_nodes_base_file_name: None,
                    replaced: None,
                },
                recorded_location: Some(RecordedLocation::of(b"//../Render/PNG".to_vec())),
                format: Some(a_format("PNG", ".png", false)),
                origin: WhereItComesFrom::AFileOutputNodeNamed(text("PNG")),
                scene: WhichScene::TheJobsOwnScene,
                the_view_layers_that_feed_it: vec![text(layer)],
                the_location_with_its_folder_replaced: None,
            });
        }
        for layer in ["Box", "Mattress"] {
            destinations.push(Destination {
                kind: DestinationKind::AWholeFileOutputNodeGatheringIntoOneFile {
                    node: text(&format!("EXR {layer}")),
                    the_nodes_base_file_name: Some(text(&format!("### - Hero - {layer}"))),
                    replaced: None,
                },
                recorded_location: Some(RecordedLocation::of(b"//../Render/EXR".to_vec())),
                format: Some(a_format("OPEN_EXR_MULTILAYER", ".exr", false)),
                origin: WhereItComesFrom::AFileOutputNodeNamed(text(&format!("EXR {layer}"))),
                scene: WhichScene::TheJobsOwnScene,
                the_view_layers_that_feed_it: vec![text(layer)],
                the_location_with_its_folder_replaced: None,
            });
        }
        for place in EVERY_ADDED_WRITE_PLACE {
            destinations.push(Destination {
                kind: DestinationKind::AnAddedWritePlace(place),
                recorded_location: (place == AnAddedWritePlace::TheBakeOutputPath)
                    .then(|| RecordedLocation::of(b"//".to_vec())),
                format: None,
                origin: WhereItComesFrom::AnAddedWritePlaceNamed(place),
                scene: WhichScene::TheJobsOwnScene,
                the_view_layers_that_feed_it: Vec::new(),
                the_location_with_its_folder_replaced: None,
            });
        }
        outputs.destinations = destinations;
        outputs.per_frame = vec![AtOneFrame {
            frame: 1,
            view_layers_that_render: layers.iter().map(|layer| text(layer)).collect(),
            gating: outputs.gating.clone(),
            the_regular_output_paths_answered_path: Some(RecordedLocation::of(
                b"/home/ubuntu/Render/Animation0001-0001.mp4".to_vec(),
            )),
        }];
        outputs
    }

    /// The stand-in's ground: its `.blend` one folder below `Render/`, as the
    /// Creator's was, and the two AO layers pinned as `off` says.
    pub(crate) fn the_creators_ground(ao_off: bool) -> TheGroundOfTheCount {
        TheGroundOfTheCount {
            the_folder_the_blend_sits_in: b"/home/ubuntu/rendering".to_vec(),
            the_jobs_pins: if ao_off {
                ["Box AO", "Mattress AO"]
                    .iter()
                    .map(|layer| AViewLayerTheJobPins { layer: ArtistsText::of(layer.as_bytes().to_vec()), on: false })
                    .collect()
            } else {
                Vec::new()
            },
            the_jobs_folder: None,
            the_jobs_frames: None,
        }
    }

    /// **Notes 18 and 14: each destination is its folder under the job's
    /// folder — never the path the file records — and no cache folder is a
    /// row.** On the stand-in, the video sits at *top of the job's folder*, the
    /// PNG node's outputs in `PNG/` and the EXR nodes' in `EXR/`. **It fails
    /// against the rows as they stood**, which read `//../Render/Animation`,
    /// `//../Render/PNG` and `//../Render/EXR`.
    #[test]
    fn each_destination_is_its_folder_under_the_jobs_folder_never_the_files_path() {
        let outputs = the_creators_shape();
        let (writes, _, _) = the_output_group(Some(&outputs), None, &[], false, &the_creators_ground(true));
        let writes = writes.expect("the list");
        let rows: Vec<(String, bool, String)> = writes
            .rows
            .iter()
            .map(|row| {
                (
                    row.place.as_str().to_string(),
                    row.the_top.is_some(),
                    row.format.as_ref().map(|format| format.as_str().to_string()).unwrap_or_default(),
                )
            })
            .collect();
        let row = |place: &str, top: bool, format: &str| (place.to_string(), top, format.to_string());
        assert_eq!(
            rows,
            vec![
                row("", true, "Video"),
                row("PNG/", false, "PNG"),
                row("PNG/", false, "PNG"),
                row("PNG/", false, "PNG"),
                row("PNG/", false, "PNG"),
                row("EXR/", false, "Multilayer EXR"),
                row("EXR/", false, "Multilayer EXR"),
            ]
        );
        let top = writes.rows[0].the_top.as_ref().expect("the top's words");
        assert_eq!(top, &Prose::new(words::TOP_OF_THE_JOBS_FOLDER));
        let printed = serde_json::to_string(&writes).expect("it serialises");
        assert!(!printed.contains("//"), "a row says the file's own path: {printed}");
        for cache in ["bake", "pointcache", "geometry-nodes-bake", "sequencer-proxy", "fluid-cache", "rendering"] {
            assert!(!printed.contains(cache), "a cache folder is listed: {cache}");
        }
        // **The words are the Design's own.**
        let design = include_str!("../../../../Preparation/Design/Render Settings.md");
        assert!(design.contains("*top of the job's folder*"));
    }

    /// **Note 14 and hole 2: the page counts with its own View Layer pins**,
    /// by `Job files and outputs`' one rule. The stand-in with both AO layers
    /// pinned off writes 5 files — the video, two PNG and two EXR — and the
    /// box's PNG figure drops with it. **It fails against the page as it
    /// stood**, which said 7 whatever the pins.
    #[test]
    fn the_pages_count_takes_its_own_view_layer_pins() {
        let outputs = the_creators_shape();
        let total = |ao_off: bool| {
            let (writes, _, _) = the_output_group(Some(&outputs), None, &[], false, &the_creators_ground(ao_off));
            writes.expect("the list").total.values[0].as_str().to_string()
        };
        assert_eq!(total(true), "5");
        assert_eq!(total(false), "7");
        let png = |ao_off: bool| {
            what_the_format_box_knows(
                Some(&outputs),
                &TheSubmissionChoices::default(),
                &WhatTheRecordHolds::default(),
                &the_creators_ground(ao_off),
            )
            .formats
            .expect("formats")
            .into_iter()
            .find(|format| format.name == "PNG")
            .expect("PNG")
            .files
        };
        assert_eq!((png(true), png(false)), (2, 4));

        // **On the page and in its box: the pins the store holds for the scene
        // in hand**, and the file's own folder. *Box AO* pinned off: 6.
        let file = "/ephemeral/smoozy/assets/kitchen_v14.blend";
        let key = FileKey { server: an_identity(&a_server()), path: file.to_string() };
        let mut held = a_run_with_a_reading();
        held.outputs.insert(keyed(&a_server(), file), vec![(b"Interior".to_vec(), outputs.clone())]);
        let record = WhatTheRecordHolds::default();
        let the_pages_total = |held: &WhatTheRunHolds| {
            the_panel_starting_from(held, &a_server(), file, &record)
                .what_it_will_write
                .expect("the list")
                .total
                .values[0]
                .as_str()
                .to_string()
        };
        let the_boxs_png = |held: &WhatTheRunHolds| {
            let TheFormatAnswer::TheBox(the_box) = the_download_panel_from(held, &a_server(), file, &record) else {
                panic!("the box");
            };
            let crate::surface::transfers::TheFormatReading::SeveralFormats { formats, .. } = the_box.reading else {
                panic!("several formats");
            };
            let png = formats.iter().find(|one| one.name.as_str() == "PNG").expect("PNG");
            png.files.values().iter().map(|value| value.to_string()).collect::<Vec<String>>()
        };
        assert_eq!(the_pages_total(&held), "7");
        let provenance = the_provenance(&held, &key).expect("a reading stands");
        held.store.pin(
            key.clone(),
            ExactName::as_the_reading_produced_it("Interior"),
            NamedAnswer { name: ExactName::as_the_reading_produced_it("Box AO"), provenance },
            false,
        );
        assert_eq!(the_pages_total(&held), "6");
        assert_eq!(the_boxs_png(&held), vec!["3".to_string()]);
        let answers = held
            .store
            .the_answers_composition_reads(&key, Some(&ExactName::as_the_reading_produced_it("Interior")));
        let ground = the_ground_on_this_page(file, &answers);
        assert_eq!(ground.the_folder_the_blend_sits_in, b"/ephemeral/smoozy/assets".to_vec());
        assert_eq!(
            ground.the_jobs_pins,
            vec![AViewLayerTheJobPins { layer: ArtistsText::of(b"Box AO".to_vec()), on: false }]
        );
    }

    // ── The tenth notes, note 1: a typed range changes the counts ──────────

    /// **A 300-frame reading of one PNG output**, held for the page's scene in
    /// hand — the Creator's case in the tenth notes — with downloads turned
    /// on and a standing folder, so the strip's summary names one.
    fn a_run_over_three_hundred_frames() -> (WhatTheRunHolds, &'static str, WhatTheRecordHolds) {
        let file = "/ephemeral/smoozy/assets/kitchen_v14.blend";
        let frames: Vec<i64> = (1..=300).collect();
        let outputs = crate::servers::the_machines_work::tests::a_recorded_reading(
            "/ephemeral/smoozy/jobs/job",
            &frames,
            smoozy_job_files::reading::HowManyViewsTheSceneWrites::One,
        )
        .reading
        .outputs;
        let mut held = a_run_with_a_reading();
        held.outputs.insert(keyed(&a_server(), file), vec![(b"Interior".to_vec(), outputs)]);
        let _ = the_answer_is_taken(&mut held, &a_server(), file, WhichSetting::DownloadAsTheyRender, "on");
        let record = WhatTheRecordHolds {
            the_standing_folder: Some(b"/Users/artist/Downloads".to_vec()),
            ..WhatTheRecordHolds::default()
        };
        (held, file, record)
    }

    /// **What the page says of the count, in each place it says it**: the
    /// list under *What it will write* (each row's figure, the total, then the
    /// split by format), the lines under it, the strip's download summary, and
    /// the box that summary opens.
    #[derive(Debug)]
    struct TheCountsSaid {
        the_list: Option<Vec<String>>,
        the_lines: Vec<String>,
        the_strip: Option<Vec<String>>,
        the_box: String,
    }

    fn the_counts_said(held: &WhatTheRunHolds, file: &str, record: &WhatTheRecordHolds) -> TheCountsSaid {
        use crate::surface::transfers::TheFormatReading;
        let panel = the_panel_starting_from(held, &a_server(), file, record);
        let the_list = panel.what_it_will_write.as_ref().map(|writes| {
            let mut said: Vec<String> = writes.rows.iter().map(|row| row.how_many.as_str().to_string()).collect();
            said.extend(the_words_of(&vec![writes.total.clone()]));
            for one in &writes.by_format {
                said.push(one.how_many.as_str().to_string());
                said.push(one.format.as_str().to_string());
            }
            said
        });
        let TheFormatAnswer::TheBox(the_box) = the_download_panel_from(held, &a_server(), file, record) else {
            panic!("the box");
        };
        let the_box = match &the_box.reading {
            TheFormatReading::OneFormat { said } => format!("{} | {}", said.values().join(" "), said.said()),
            TheFormatReading::SeveralFormats { total, .. } => format!("several | {}", total.values().join(" ")),
            TheFormatReading::NotRead { warning, what_downloads, .. } => {
                format!("not read | {} {}", warning.as_str(), what_downloads.said())
            }
            TheFormatReading::ReadForAnotherScene { warning, what_downloads } => {
                format!("{} {}", warning.as_str(), what_downloads.said())
            }
        };
        TheCountsSaid {
            the_list,
            the_lines: panel.the_output_lines.iter().map(the_words_of_a_line).collect(),
            the_strip: panel.the_format_summary.as_ref().map(the_words_of),
            the_box: the_box.replace('\u{a0}', " "),
        }
    }

    fn typed_into_the_page(held: &mut WhatTheRunHolds, file: &str, typed: &str) {
        assert_eq!(
            the_answer_is_taken(held, &a_server(), file, WhichSetting::FrameRange, typed),
            None,
            "{typed} was refused"
        );
    }

    /// **The tenth notes, note 1: on a 300-frame file with `1` typed, the
    /// download box said *300 PNG files* and *What it will write* counted all
    /// 300.** `Design/Render Settings § What it will write`, added 2026-10-17:
    /// *the counts are for the frames the job will render: with a range typed,
    /// only those frames are counted, here and in the download box.* **With
    /// nothing typed, nothing changes**; `1` reads *1 PNG file* in both
    /// places, singular; `1-2x2` is frame 1 alone; `5,9` is two. **It fails
    /// against the page as it stood**, which said 300 whatever was typed.
    #[test]
    fn a_typed_range_changes_the_counts_in_both_places() {
        let (mut held, file, record) = a_run_over_three_hundred_frames();
        let folder = "/Users/artist/Downloads";

        // **Nothing typed: the file's own 300**, as before.
        let said = the_counts_said(&held, file, &record);
        assert_eq!(said.the_list, Some(vec!["300", "300", "files", "300", "PNG"].into_iter().map(String::from).collect()));
        assert_eq!(said.the_strip, Some(vec!["300 PNG".to_string(), "files,".into(), "to".into(), folder.into()]));
        assert!(said.the_box.starts_with("300 PNG |"), "{said:?}");
        assert!(said.the_box.contains("files are downloaded."), "{said:?}");

        // **`1` typed: one PNG file, here and in the download box.**
        typed_into_the_page(&mut held, file, "1");
        let said = the_counts_said(&held, file, &record);
        assert_eq!(said.the_list, Some(vec!["1", "1", "file", "1", "PNG"].into_iter().map(String::from).collect()), "{said:?}");
        assert_eq!(said.the_strip, Some(vec!["1 PNG".to_string(), "file,".into(), "to".into(), folder.into()]), "{said:?}");
        assert!(said.the_box.starts_with("1 PNG |"), "{said:?}");
        assert!(said.the_box.contains("file is downloaded."), "{said:?}");
        assert!(said.the_lines.is_empty(), "a range inside the read draws no line: {said:?}");

        // **`1-2x2` is frame 1 alone; `5,9` is two.**
        typed_into_the_page(&mut held, file, "1-2x2");
        let said = the_counts_said(&held, file, &record);
        assert_eq!(said.the_strip, Some(vec!["1 PNG".to_string(), "file,".into(), "to".into(), folder.into()]), "{said:?}");
        typed_into_the_page(&mut held, file, "5,9");
        let said = the_counts_said(&held, file, &record);
        assert_eq!(said.the_list, Some(vec!["2", "2", "files", "2", "PNG"].into_iter().map(String::from).collect()), "{said:?}");
        assert_eq!(said.the_strip, Some(vec!["2 PNG".to_string(), "files,".into(), "to".into(), folder.into()]), "{said:?}");
        assert!(said.the_box.starts_with("2 PNG |"), "{said:?}");

        // **The Render control already counted the typed frames**, and still does.
        let panel = the_panel_starting_from(&held, &a_server(), file, &record);
        assert_eq!(the_words_of_a_line(&panel.the_render_control), "Render <2> frames");
    }

    /// **The eleventh notes, the video-range piece: where the file's regular output is a
    /// video, a typed list with gaps is refused under the range field** with
    /// the Design's own line, and the field keeps its last good value, the way
    /// any unusable range is (`Design/Render Settings`, *Where the file writes
    /// a video, the range takes one unbroken run*). **A step is accepted**, and
    /// so is a list that comes to one evenly stepped run. **On a file writing
    /// pictures nothing changes**: the same list is taken.
    #[test]
    fn a_gapped_list_is_refused_where_the_files_regular_output_is_a_video() {
        use smoozy_job_files::reading::{ArtistsText, DestinationKind, Format};
        let (mut held, file, _) = a_run_over_three_hundred_frames();
        let say = |held: &mut WhatTheRunHolds, typed: &str| {
            the_words_of(&what_the_answer_says(held, &a_server(), file, WhichSetting::FrameRange, typed))
        };

        // **Pictures: a list with gaps is an ordinary list**, as before.
        assert_eq!(say(&mut held, "1-10, 20-30"), Vec::<String>::new());
        assert_eq!(
            the_range_the_page_draws(&held, file).as_deref(),
            Some("1-10, 20-30")
        );

        // **The same file writing a video at its output path.**
        let mut videos = 0;
        for (_, outputs) in held.outputs.get_mut(&keyed(&a_server(), file)).expect("the outputs") {
            for destination in &mut outputs.destinations {
                if matches!(destination.kind, DestinationKind::TheRegularOutputPath) {
                    destination.format = Some(Format {
                        as_the_file_records_it: ArtistsText::of(b"FFMPEG".to_vec()),
                        the_ending_it_writes: Some(ArtistsText::of(b".mp4".to_vec())),
                        is_video_shaped: true,
                    });
                    videos += 1;
                }
            }
        }
        assert_eq!(videos, 1, "the stand-in has one regular output path");

        assert_eq!(say(&mut held, "1-20"), Vec::<String>::new());
        let the_last_good = the_range_the_page_draws(&held, file);
        assert_eq!(the_last_good.as_deref(), Some("1-20"));
        let the_line = "A video can’t skip frames. Type one unbroken range.";
        let design = include_str!("../../../../Preparation/Design/Render Settings.md");
        assert!(design.contains(&the_line.replace('’', "'")), "the line is not the Design's own");
        for gapped in ["1-10, 20-30", "1, 2, 5", "1-3, 5, 7", "1, 3, 4"] {
            assert_eq!(say(&mut held, gapped), vec![the_line.to_string()], "{gapped}");
            assert_eq!(
                the_range_the_page_draws(&held, file),
                the_last_good,
                "{gapped}: the field does not keep its last good value"
            );
        }

        // **A step is no gap**, typed as a step or as a list evenly stepped —
        // two frames are a run at their own step — and a list that comes to
        // one unbroken run is that run.
        for accepted in ["1-10x2", "1-10, 11-20", "1, 3, 5, 7", "5, 9", "7", "20-30"] {
            assert_eq!(say(&mut held, accepted), Vec::<String>::new(), "{accepted}");
        }
        // **An unusable range is refused as before**, with its own line.
        assert_ne!(say(&mut held, "10-1"), vec![the_line.to_string()]);
    }

    /// **A typed frame outside what the read stepped: no figures, and one line
    /// in their place** — `Design/Render Settings § What it will write`: *Frames
    /// outside the file's own range can't be counted before the render.* The
    /// read stepped the file's own 1–300, so `301`, and `299-301` with two of
    /// its three inside, have no entry to count from; **a count of the frames
    /// it did step would say fewer files than the job writes.** The list goes,
    /// the strip names the folder alone, and the box says the line where it
    /// would have said the figure — and never *not read*, which would be false.
    #[test]
    fn a_typed_frame_outside_the_read_hides_the_figures_and_says_why() {
        let (mut held, file, record) = a_run_over_three_hundred_frames();
        let the_line = "Frames outside the file’s range can’t be counted before rendering.";
        let design = include_str!("../../../../Preparation/Design/Render Settings.md");
        assert!(design.contains(&the_line.replace('’', "'")), "the line is not the Design's own");

        for typed in ["301", "299-301", "0,5"] {
            typed_into_the_page(&mut held, file, typed);
            let said = the_counts_said(&held, file, &record);
            assert_eq!(said.the_list, None, "{typed}: the figures are drawn: {said:?}");
            assert_eq!(said.the_lines, vec![the_line.to_string()], "{typed}: {said:?}");
            assert_eq!(said.the_strip, None, "{typed}: the strip still counts: {said:?}");
            assert!(said.the_box.starts_with(the_line), "{typed}: {said:?}");
            assert!(!said.the_box.contains("not read"), "{typed}: the box says the file is not read: {said:?}");
            let panel = the_panel_starting_from(&held, &a_server(), file, &record);
            assert!(panel.the_output_group_is_drawn, "{typed}: the line has no group to sit in");
            assert_eq!(
                panel.where_they_go.as_ref().map(the_words_of),
                Some(vec![words::WHERE_THEY_GO.to_string(), words::THE_DASH.into(), "/Users/artist/Downloads".into()]),
                "{typed}: the strip does not name the folder alone"
            );
        }

        // **Back inside the read, the figures come back** and the line goes.
        typed_into_the_page(&mut held, file, "300");
        let said = the_counts_said(&held, file, &record);
        assert!(said.the_lines.is_empty(), "{said:?}");
        assert_eq!(said.the_strip.as_ref().map(|strip| strip[0].as_str()), Some("1 PNG"), "{said:?}");
    }

    /// **The queue's box for a submitted job takes the payload's range the same
    /// way** (`the_ground_of_a_submitted_job`). A job submitted with `1-10x3`
    /// typed on the 300-frame file counts 4 PNG files; one submitted with
    /// nothing typed counts the file's 300; and a record with no kept setup
    /// counts what it did before. **It fails against the box as it stood**,
    /// which counted 300 for the first.
    #[test]
    fn the_queues_box_counts_the_range_the_job_was_submitted_with() {
        use smoozy_render_settings::payload::{compose, Composition};
        let outputs = crate::servers::the_machines_work::tests::a_recorded_reading(
            "/ephemeral/smoozy/jobs/job",
            &(1..=300).collect::<Vec<i64>>(),
            smoozy_job_files::reading::HowManyViewsTheSceneWrites::One,
        )
        .reading
        .outputs;
        let submitted_with = |frame_range: Option<FrameForm>| {
            let answers = SceneEntries { frame_range, ..Default::default() };
            compose(Composition {
                version_pick: &BlenderIdentity {
                    path: "/opt/blender-5.1.0/blender".to_string(),
                    version: a_version(5, 1, 0),
                },
                scene: None,
                answers: &answers,
                standing: None,
                stale: false,
            })
        };
        let the_box_counts = |payload: Option<&smoozy_render_settings::payload::Payload>| {
            let knows = what_the_format_box_knows(
                Some(&outputs),
                &TheSubmissionChoices::default(),
                &WhatTheRecordHolds::default(),
                &the_ground_of_a_submitted_job(Some("/artist/hero.blend"), payload),
            );
            let png = knows.formats.as_ref().and_then(|formats| formats.first()).map(|format| format.files);
            (png, knows.files_in_the_job)
        };
        assert_eq!(the_box_counts(Some(&submitted_with(Some(typed("1-10x3"))))), (Some(4), 4));
        assert_eq!(the_box_counts(Some(&submitted_with(Some(typed("7"))))), (Some(1), 1));
        assert_eq!(the_box_counts(Some(&submitted_with(None))), (Some(300), 300));
        assert_eq!(the_box_counts(None), (Some(300), 300));
    }

    // ── The tenth notes: the download box says what will go ────────────────────────────

    /// **What a box's reading says, sentence by sentence** — each plain string
    /// it carries, and each sentence of words and values read in its order,
    /// with U+00A0 read as a space. The case's own tag comes first, apart.
    /// Read off the box as the page is handed it, so it reads any shape.
    pub(crate) fn what_a_reading_says(reading: &serde_json::Value) -> (String, Vec<String>) {
        fn walk(value: &serde_json::Value, said: &mut Vec<String>) {
            match value {
                serde_json::Value::String(text) => said.push(text.replace('\u{a0}', " ")),
                serde_json::Value::Array(all) => all.iter().for_each(|one| walk(one, said)),
                serde_json::Value::Object(map) => match map.get("pieces") {
                    Some(serde_json::Value::Array(pieces)) => said.push(
                        pieces
                            .iter()
                            .filter_map(|piece| {
                                piece.get("said").and_then(|one| one.as_str()).or_else(|| piece.get("value").and_then(|one| one.as_str()))
                            })
                            .collect::<Vec<_>>()
                            .join(" ")
                            .replace('\u{a0}', " "),
                    ),
                    _ => map.iter().filter(|(key, _)| key.as_str() != "reads").for_each(|(_, one)| walk(one, said)),
                },
                _ => {}
            }
        }
        let mut said = Vec::new();
        walk(reading, &mut said);
        (reading["reads"].as_str().unwrap_or_default().to_string(), said)
    }

    /// **What the Render Settings box says**, through [`what_a_reading_says`].
    fn what_the_box_says(held: &WhatTheRunHolds, file: &str, record: &WhatTheRecordHolds) -> (String, Vec<String>) {
        let TheFormatAnswer::TheBox(the_box) = the_download_panel_from(held, &a_server(), file, record) else {
            panic!("the box");
        };
        what_a_reading_says(&serde_json::to_value(&the_box.reading).expect("drawn"))
    }

    fn the_narrowing_on_the_page(held: &mut WhatTheRunHolds, file: &str, chosen: Option<Vec<Vec<u8>>>) {
        held.submission.entry(keyed(&a_server(), file)).or_default().the_formats_chosen = chosen;
    }

    /// **The box says what will actually go** (the runner's ruling of
    /// 2026-10-17). Where the box cannot
    /// count — the file not read, read for another scene, or a typed frame
    /// outside the read — it said *Everything downloads.* while a narrowing
    /// the user made earlier still went with the job at Submit (T1's ask 2,
    /// the runner's ruling of 2026-10-17; Rule 8). **Where a narrowing stands
    /// it is named**: the formats in Smoozy's words, as the strip's summary
    /// and the Transfers row name them, then the box's own *files are
    /// downloaded.* **Where none stands, *Everything downloads.* stays.** What
    /// Submit hands over is the narrowing, as before. **It fails against the
    /// box as it stood**, which said *Everything downloads.* in all three.
    #[test]
    fn the_box_names_a_narrowing_where_it_cannot_count() {
        let everything = crate::surface::transfers::words::EVERYTHING_DOWNLOADS;
        let exr_and_png: Vec<Vec<u8>> = vec![b"OPEN_EXR".to_vec(), b"PNG".to_vec()];
        let (mut outside_the_read, three_hundred, record) = a_run_over_three_hundred_frames();
        typed_into_the_page(&mut outside_the_read, three_hundred, "301");
        let cases = vec![
            ("not read", a_run_with_a_reading(), "/ephemeral/smoozy/assets/never_read.blend", "not-read"),
            (
                "another scene",
                a_run_with_a_reading(),
                "/ephemeral/smoozy/assets/kitchen_v14.blend",
                "read-for-another-scene",
            ),
            ("a frame outside the read", outside_the_read, three_hundred, "read-for-another-scene"),
        ];
        for (case, mut held, file, reads) in cases {
            // **No narrowing: everything downloads, and says so.**
            the_narrowing_on_the_page(&mut held, file, None);
            let (drawn, said) = what_the_box_says(&held, file, &record);
            assert_eq!(drawn, reads, "{case}: {said:?}");
            assert!(said.iter().any(|one| one == everything), "{case}, not narrowed: {said:?}");
            // **What Submit hands over**: the act as the page composes it. A
            // file never read does not compose in this harness, so there the
            // submission the act copies it from is read instead.
            let handed_over = |held: &WhatTheRunHolds| match the_act_as_the_page_composed_it(
                held,
                &a_server(),
                file,
                &record,
                &a_row_on(&a_server(), true),
                a_run(),
            ) {
                Ok(given) => given.the_formats_chosen,
                Err(said) if case == "not read" => {
                    let _ = said;
                    held.submission[&keyed(&a_server(), file)].the_formats_chosen.clone()
                }
                Err(said) => panic!("{case}: {said:?}"),
            };
            assert_eq!(handed_over(&held), None, "{case}");

            // **Narrowed to two, one, and none.**
            for (chosen, named) in [
                (exr_and_png.clone(), "EXR and PNG files are downloaded."),
                (vec![b"OPEN_EXR_MULTILAYER".to_vec()], "Multilayer EXR files are downloaded."),
                (Vec::new(), crate::surface::transfers::words::NO_FORMAT_IS_DOWNLOADED),
            ] {
                the_narrowing_on_the_page(&mut held, file, Some(chosen.clone()));
                let (drawn, said) = what_the_box_says(&held, file, &record);
                assert_eq!(drawn, reads, "{case}: {said:?}");
                assert!(
                    !said.iter().any(|one| one == everything),
                    "{case}: the box says everything downloads while a narrowing stands: {said:?}"
                );
                assert!(said.iter().any(|one| one == named), "{case}: the narrowing is not named: {said:?}");
                // **What Submit hands over is unchanged**: the narrowing.
                assert_eq!(handed_over(&held), Some(chosen), "{case}");
            }
        }
    }

    /// **Done in a box with no list keeps the narrowing the box names** — the
    /// page sends no format from a box that lists none, and Done is pressed to
    /// choose the folder. **It used to drop the narrowing**, so the box's line
    /// would have been false the moment Done was pressed. With none standing,
    /// none is made.
    #[test]
    fn done_in_a_box_with_no_list_keeps_the_narrowing_it_names() {
        let file = "/ephemeral/smoozy/assets/never_read.blend";
        let record = WhatTheRecordHolds {
            the_standing_folder: Some(b"/Users/artist/Downloads".to_vec()),
            ..WhatTheRecordHolds::default()
        };
        let mut held = a_run_with_a_reading();
        let exr: Vec<Vec<u8>> = vec![b"OPEN_EXR".to_vec()];
        the_narrowing_on_the_page(&mut held, file, Some(exr.clone()));
        assert!(the_download_choices_are_taken(&mut held, &a_server(), file, &[], TheFolderTaken::Offered(0), &record));
        assert_eq!(held.submission[&keyed(&a_server(), file)].the_formats_chosen, Some(exr), "Done dropped the narrowing");
        let (_, said) = what_the_box_says(&held, file, &record);
        assert!(said.iter().any(|one| one == "EXR files are downloaded."), "{said:?}");

        the_narrowing_on_the_page(&mut held, file, None);
        assert!(the_download_choices_are_taken(&mut held, &a_server(), file, &[], TheFolderTaken::Offered(0), &record));
        assert_eq!(held.submission[&keyed(&a_server(), file)].the_formats_chosen, None);
        // **A format named from a box with no list is still refused** (H34).
        assert!(!the_download_choices_are_taken(
            &mut held,
            &a_server(),
            file,
            &["PNG".to_string()],
            TheFolderTaken::Offered(0),
            &record
        ));
    }

    /// **Done in a box of one format narrows nothing** — `Design/Render
    /// Settings`: *on a job with only one format there is nothing to narrow,
    /// so nothing is offered.* The box lists no row to tick, so the page sends
    /// no format, **and Done kept that as a narrowing to no format at all**:
    /// the box went on saying *All 300 PNG files are downloaded.*, the strip
    /// *300 PNG files*, and Submit handed over a narrowing that downloads none
    /// of them. Found while building the box ruling.
    #[test]
    fn done_in_a_box_of_one_format_narrows_nothing() {
        let (mut held, file, record) = a_run_over_three_hundred_frames();
        let (drawn, _) = what_the_box_says(&held, file, &record);
        assert_eq!(drawn, "one-format");
        assert!(the_download_choices_are_taken(&mut held, &a_server(), file, &[], TheFolderTaken::Offered(0), &record));
        assert_eq!(
            held.submission[&keyed(&a_server(), file)].the_formats_chosen,
            None,
            "Done in a box of one format narrowed the job to no format"
        );
        let given =
            the_act_as_the_page_composed_it(&held, &a_server(), file, &record, &a_row_on(&a_server(), true), a_run())
                .unwrap_or_else(|said| panic!("{said:?}"));
        assert_eq!(given.the_formats_chosen, None);
    }

    /// **Hole 5: the page resolves the file's `//` against where the file
    /// sits on that server**, as the render does. The stand-in with its PNG
    /// node written as an absolute path to the same folder its `//..` paths
    /// reach: against the file's real folder the three share `Render/`, so the
    /// list reads as the render will write. **Against the made-up folder the
    /// page used before**, the run they share is the top of the disk and every
    /// row carries a path nobody will find.
    #[test]
    fn the_page_resolves_the_files_paths_against_where_the_file_sits() {
        let mut outputs = the_creators_shape();
        for destination in outputs.destinations.iter_mut().skip(1).take(4) {
            destination.recorded_location =
                Some(smoozy_job_files::reading::RecordedLocation::of(b"/home/ubuntu/Render/PNG".to_vec()));
        }
        let places = |folder: &[u8]| -> Vec<String> {
            let ground = TheGroundOfTheCount { the_folder_the_blend_sits_in: folder.to_vec(), the_jobs_pins: Vec::new(), the_jobs_folder: None, the_jobs_frames: None };
            let (writes, _, _) = the_output_group(Some(&outputs), None, &[], false, &ground);
            writes.expect("the list").rows.iter().map(|row| row.place.as_str().to_string()).collect()
        };
        assert_eq!(places(b"/home/ubuntu/rendering"), vec!["", "PNG/", "PNG/", "PNG/", "PNG/", "EXR/", "EXR/"]);
        assert_eq!(places(b"/ephemeral/smoozy/assets")[0], "ephemeral/smoozy/Render/");
    }

    /// **Hole 17: formats in Smoozy's words, from one table** — every word its
    /// own, and an identifier the table does not hold shown as recorded.
    #[test]
    fn formats_are_named_in_smoozys_words_from_one_table() {
        assert_eq!(a_format_in_smoozys_words(b"OPEN_EXR_MULTILAYER"), "Multilayer EXR");
        assert_eq!(a_format_in_smoozys_words(b"OPEN_EXR"), "EXR");
        assert_eq!(a_format_in_smoozys_words(b"FFMPEG"), "Video");
        assert_eq!(a_format_in_smoozys_words(b"PNG"), "PNG");
        assert_eq!(a_format_in_smoozys_words(b"JPEG"), "JPEG");
        assert_eq!(a_format_in_smoozys_words(b"SOMETHING_NEW"), "SOMETHING_NEW");
        let mut words: Vec<&str> = THE_FORMATS_IN_SMOOZYS_WORDS.iter().map(|(_, said)| *said).collect();
        words.sort_unstable();
        words.dedup();
        assert_eq!(words.len(), THE_FORMATS_IN_SMOOZYS_WORDS.len(), "two identifiers share a word");
        // **The five the Design names are the table's.**
        let design = include_str!("../../../../Preparation/Design/Render Settings.md");
        for said in ["*PNG*", "*JPEG*", "*EXR*", "*Multilayer EXR*", "*Video*"] {
            assert!(design.contains(said), "{said}");
        }

        // **The summary names them so too**, and the box keeps the identifier
        // to hand back.
        let knows = what_the_format_box_knows(
            Some(&the_creators_shape()),
            &TheSubmissionChoices::default(),
            &WhatTheRecordHolds::default(),
            &the_creators_ground(true),
        );
        let names: Vec<String> = knows.formats.as_ref().expect("formats").iter().map(|one| one.name.clone()).collect();
        assert_eq!(names, vec!["FFMPEG", "PNG", "OPEN_EXR_MULTILAYER"]);
        let (summary, _) = the_download_summary(&knows, Some("~/Renders"), true);
        let said: Vec<String> = summary
            .expect("a summary")
            .iter()
            .flat_map(|part| part.values.iter().map(|value| value.as_str().to_string()))
            .collect();
        assert_eq!(said[..3], ["Video".to_string(), "PNG".to_string(), "Multilayer EXR".to_string()]);
    }

    /// **The box says where in the job each format is written** — every
    /// folder once, and *top of the job's folder* in words — never the file's
    /// path. **It fails against the box as it stood**, which said
    /// `//../Render/PNG` beside PNG.
    #[test]
    fn the_box_says_where_in_the_job_each_format_is_written() {
        let knows = what_the_format_box_knows(
            Some(&the_creators_shape()),
            &TheSubmissionChoices::default(),
            &WhatTheRecordHolds::default(),
            &the_creators_ground(true),
        );
        let formats = knows.formats.expect("formats");
        let seen: Vec<(&str, &str, bool)> = formats
            .iter()
            .map(|one| (one.name.as_str(), one.where_in_the_job.as_str(), one.at_the_top))
            .collect();
        assert_eq!(
            seen,
            vec![("FFMPEG", "", true), ("PNG", "PNG/", false), ("OPEN_EXR_MULTILAYER", "EXR/", false)]
        );
    }

    /// **Note 18: one line above the list says where the job's folder will
    /// sit**, read from the one place that decides it — `Job files and
    /// outputs`' own choice, the render worker's at render start, asked here
    /// with nothing made: **where the file points** (the Creator's shape,
    /// `//../Render/…` from `/home/ubuntu/rendering`, sits in
    /// `/home/ubuntu/Render`), **or which fallback**, in the Design's own words.
    /// No line where the server's layout is not recorded, or where there is no
    /// list. The Creator's second notes, piece 4 (it read Smoozy's job area
    /// from piece 2 until then).
    #[test]
    fn the_line_above_the_list_says_where_the_jobs_folder_will_sit() {
        let layout = TheLayout::of(
            smoozy_job_files::layout::MachineRoot::chosen("/root/smoozy").expect("a root"),
            smoozy_job_files::layout::WhereThePackageAreaSits::UnderTheRoot,
        );
        let sits = where_the_jobs_folder_sits(&layout, &the_creators_shape(), &the_creators_ground(true), None);
        assert_eq!(sits, WhereTheJobsFolderSits::In("/home/ubuntu/Render".to_string()));
        let line = the_folder_line(&sits);
        assert_eq!(line[0].values, vec![Verbatim::new("/home/ubuntu/Render")]);
        assert!(line[0].the_rest_joins, "the comma follows the folder with nothing between");
        assert_eq!(the_second_folder_line(&sits), None);
        let as_read = |sentence: &ASentence| {
            serde_json::to_string(sentence).expect("it serialises").replace('’', "'").replace('\u{a0}', " ")
        };
        let design = include_str!("../../../../Preparation/Design/Render Settings.md").replace('’', "'");
        assert!(design.contains("*In /home/ubuntu/Render, in a folder for this job.*"));
        // **The two fallbacks, to Smoozy's folder**: a file whose folder is
        // the `.blend`'s own in Smoozy's area, and one whose paths are another
        // computer's.
        let in_smoozys_area = TheGroundOfTheCount {
            the_folder_the_blend_sits_in: b"/root/smoozy/assets/set-1".to_vec(),
            ..the_creators_ground(true)
        };
        let mut from_a_mac = the_creators_shape();
        for destination in &mut from_a_mac.destinations {
            if destination.is_one_the_render_asked_for() {
                destination.recorded_location =
                    Some(smoozy_job_files::reading::RecordedLocation::of(b"C:\\Renders\\".to_vec()));
            }
        }
        for (outputs, ground, said) in [
            (the_creators_shape(), &in_smoozys_area, words::SMOOZY_CANT_USE_THE_FOLDER),
            (from_a_mac, &the_creators_ground(true), words::THE_FOLDER_IS_ANOTHER_COMPUTERS),
        ] {
            let plain = said.replace('’', "'");
            assert!(design.contains(&format!("*{plain}*")), "not the Design's words: {plain}");
            let sits = where_the_jobs_folder_sits(&layout, &outputs, ground, None);
            assert!(matches!(sits, WhereTheJobsFolderSits::FallsBack { to: WhereTheFallbackGoes::SmoozysFolder, .. }), "{sits:?}");
            assert!(as_read(&the_folder_line(&sits)).contains(&plain));
            assert_eq!(the_second_folder_line(&sits), None);
            assert_eq!(sits.the_pages_job_folder(), "/root/smoozy/jobs/…");
        }

        // **On the page**: beside the list, where the layout is recorded —
        // the file sits in `/ephemeral/smoozy/assets` on that server, so its
        // `//../Render/…` share `/ephemeral/smoozy/Render`.
        let file = "/ephemeral/smoozy/assets/kitchen_v14.blend";
        let mut held = a_run_with_a_reading();
        held.outputs.insert(keyed(&a_server(), file), vec![(b"Interior".to_vec(), the_creators_shape())]);
        let with = WhatTheRecordHolds { the_layout: Some(layout.clone()), ..WhatTheRecordHolds::default() };
        let panel = the_panel_starting_from(&held, &a_server(), file, &with);
        assert!(panel.what_it_will_write.is_some());
        let line = panel.the_folder_line.expect("the line");
        assert_eq!(line[0].values, vec![Verbatim::new("/ephemeral/smoozy/Render")]);
        assert!(panel.the_second_folder_line.is_none());
        let without = the_panel_starting_from(&held, &a_server(), file, &WhatTheRecordHolds::default());
        assert!(without.the_folder_line.is_none(), "a line with no layout to read");
        let unread = the_panel_starting_from(&WhatTheRunHolds::default(), &a_server(), file, &with);
        assert!(unread.the_folder_line.is_none(), "a line with no list under it");

        // **The page draws it above the list, and the top as words.**
        let group = the_pages_function("function theOutputGroup(said) {");
        let line_at = group.find("said.the_folder_line").expect("the page draws the line");
        assert!(line_at < group.find("said.what_it_will_write.rows").expect("the list"));
        assert!(group.contains("if (row.the_top) {"));
    }

    /// **The chosen fallback folder on the page** (`Job files and
    /// outputs` P23, `Design/Render Settings § What it will write`): where a server has
    /// one set and nothing known before submitting rules it out, the sentence
    /// ends with its path; where something does, the sentence names Smoozy's
    /// folder and a second line says the server's can't be used — the
    /// drawing's `res-fallchosen`. **A file whose own folder can be used takes
    /// neither**: the chosen folder decides nothing for it. The Creator's
    /// second notes, piece 4.
    #[test]
    fn a_chosen_fallback_folder_is_named_where_it_will_take_the_job_and_ruled_out_on_a_second_line() {
        let layout = TheLayout::of(
            smoozy_job_files::layout::MachineRoot::chosen("/root/smoozy").expect("a root"),
            smoozy_job_files::layout::WhereThePackageAreaSits::UnderTheRoot,
        );
        let in_smoozys_area = TheGroundOfTheCount {
            the_folder_the_blend_sits_in: b"/root/smoozy/assets/set-1".to_vec(),
            ..the_creators_ground(true)
        };
        let to_mnt = where_the_jobs_folder_sits(&layout, &the_creators_shape(), &in_smoozys_area, Some("/mnt/renders"));
        assert_eq!(
            to_mnt,
            WhereTheJobsFolderSits::FallsBack {
                why: smoozy_job_files::where_the_folder_sits::WhyTheFilesOwnPlaceWasNotUsed::ItIsAFolderSmoozyCannotUse,
                to: WhereTheFallbackGoes::TheServersFolder("/mnt/renders".to_string()),
                place: "/mnt/renders".to_string(),
            }
        );
        let line = the_folder_line(&to_mnt);
        assert_eq!(line[0].values, vec![Verbatim::new("/mnt/renders")]);
        assert_eq!(line[0].lead, Prose::new(words::SMOOZY_CANT_USE_THE_FOLDER_SO_THE_FILES_GO_TO));
        assert_eq!(the_second_folder_line(&to_mnt), None);

        for ruled_out in ["/tmp/renders", "/root/smoozy/mine", "renders"] {
            let sits = where_the_jobs_folder_sits(&layout, &the_creators_shape(), &in_smoozys_area, Some(ruled_out));
            assert!(
                matches!(sits, WhereTheJobsFolderSits::FallsBack { to: WhereTheFallbackGoes::SmoozysFolderBecauseTheServersCantBeUsed, .. }),
                "{ruled_out}: {sits:?}"
            );
            assert_eq!(the_folder_line(&sits), vec![plainly(words::SMOOZY_CANT_USE_THE_FOLDER)]);
            assert_eq!(the_second_folder_line(&sits), Some(vec![plainly(words::THIS_SERVERS_FALLBACK_FOLDER_CANT_BE_USED)]));
        }
        let design = include_str!("../../../../Preparation/Design/Render Settings.md").replace('’', "'");
        assert!(design.contains(&format!("*{}*", words::THIS_SERVERS_FALLBACK_FOLDER_CANT_BE_USED.replace('’', "'"))));

        let here = where_the_jobs_folder_sits(&layout, &the_creators_shape(), &the_creators_ground(true), Some("/mnt/renders"));
        assert_eq!(here, WhereTheJobsFolderSits::In("/home/ubuntu/Render".to_string()));

        // **On the page**, from the row's own value — the file sits inside
        // Smoozy's own root on this server, so its folder falls back.
        let file = "/ephemeral/smoozy/assets/kitchen_v14.blend";
        let mut held = a_run_with_a_reading();
        held.outputs.insert(keyed(&a_server(), file), vec![(b"Interior".to_vec(), the_creators_shape())]);
        let with = WhatTheRecordHolds {
            the_layout: Some(TheLayout::of(
                smoozy_job_files::layout::MachineRoot::chosen("/ephemeral/smoozy").expect("a root"),
                smoozy_job_files::layout::WhereThePackageAreaSits::UnderTheRoot,
            )),
            the_servers_fallback_folder: Some("renders".to_string()),
            ..WhatTheRecordHolds::default()
        };
        let panel = the_panel_starting_from(&held, &a_server(), file, &with);
        assert!(panel.the_second_folder_line.is_some());

        // **The page draws the second line under the first.**
        let group = the_pages_function("function theOutputGroup(said) {");
        let first = group.find("said.the_folder_line").expect("the first line");
        let second = group.find("said.the_second_folder_line").expect("the page draws the second line");
        assert!(first < second && second < group.find("said.what_it_will_write.rows").expect("the list"));
    }

    /// **Piece 2's account, section 6, ruled for this piece: the collision
    /// warning names the path the files will really land at**, read from the
    /// one place the folder choice lives — the place, then a folder for this
    /// job — never a stand-in in Smoozy's own area. The Creator's second
    /// notes, piece 4.
    #[test]
    fn the_collision_warning_names_where_the_files_will_really_land() {
        let layout = TheLayout::of(
            smoozy_job_files::layout::MachineRoot::chosen("/root/smoozy").expect("a root"),
            smoozy_job_files::layout::WhereThePackageAreaSits::UnderTheRoot,
        );
        let mut outputs = the_creators_shape();
        // Two PNG outputs writing the same name collide.
        let first_png = outputs
            .destinations
            .iter()
            .position(|destination| matches!(destination.kind, smoozy_job_files::reading::DestinationKind::AnOutputOfAFileOutputNode { .. }))
            .expect("a node output");
        let twin = outputs.destinations[first_png].clone();
        outputs.destinations.push(twin);
        let mut ground = the_creators_ground(false);
        let sits = where_the_jobs_folder_sits(&layout, &outputs, &ground, None);
        ground.the_jobs_folder = Some(sits.the_pages_job_folder());
        let (_, warnings, _) = the_output_group(Some(&outputs), None, &[], false, &ground);
        let pairs: Vec<String> = warnings
            .iter()
            .flat_map(|warning| warning.pairs.iter())
            .flat_map(|pair| serde_json::to_value(pair).ok())
            .map(|pair| pair.to_string())
            .collect();
        assert!(!pairs.is_empty(), "the twin collides: {warnings:?}");
        assert!(pairs.iter().all(|pair| pair.contains("/home/ubuntu/Render/…/PNG/")), "{pairs:?}");
        assert!(pairs.iter().all(|pair| !pair.contains("/ephemeral/smoozy/jobs")), "{pairs:?}");
    }

    /// **The Creator's sixth notes, note 2**: a reading of a file of two
    /// scenes lands through the run's own seam, naming *Interior* with its
    /// outputs. The box lists the formats while *Interior* is ticked. **With
    /// *Exterior* ticked, or both, it said the file had not been read** — the
    /// cause, proven at the desk before the fix — and now says it read another
    /// scene, with no *Read it now*. So does a read that could not name its
    /// scene. A file never read still says so; a file of one scene still
    /// lists its formats.
    #[test]
    fn a_read_file_says_so_for_another_scene_and_a_one_scene_file_still_lists_its_formats() {
        use crate::reading_the_file::{TheFilesHalfOfThisRun, WhatTheReadLearnt};
        use smoozy_render_settings::acting::pre_load::HoldsTheFilesHalf;
        use smoozy_render_settings::file_half::StartedUnder;
        use std::sync::{Arc, Mutex};
        let file = "/ephemeral/smoozy/assets/kitchen_v14.blend";
        let one = "/ephemeral/smoozy/assets/one_scene.blend";
        let key_of = |file: &str| FileKey { server: an_identity(&a_server()), path: file.to_string() };
        let picked = BlenderIdentity { path: "/opt/blender-5.1.0/blender".to_string(), version: a_version(5, 1, 0) };
        let land = |held: WhatTheRunHolds, file: &str, scenes: &[&str], read: Option<&str>| {
            let held = Mutex::new(held);
            let learnt = Arc::new(Mutex::new(WhatTheReadLearnt {
                the_outputs_half: read.map(|scene| (scene.as_bytes().to_vec(), the_creators_shape())),
                the_scene_it_read: read.map(|scene| scene.as_bytes().to_vec()),
                ..WhatTheReadLearnt::default()
            }));
            let landed = TheFilesHalfOfThisRun { held: &held, server: &a_server(), file, learnt }.a_reading_landed(
                key_of(file),
                &StartedUnder { version_pick: picked.clone(), file_identity: four_values() },
                SettingsHalf {
                    scenes: scenes.iter().map(|scene| a_scene(scene)).collect(),
                    offered_engine_identifiers: OfferedEngineIdentifiers {
                        cycles: Some(ExactName::as_the_reading_produced_it("CYCLES")),
                        eevee: Some(ExactName::as_the_reading_produced_it("BLENDER_EEVEE")),
                    },
                    announced_add_on_case: None,
                },
                Some(&picked),
                &four_values(),
                std::time::SystemTime::now(),
            );
            assert_eq!(landed, smoozy_render_settings::file_half::Landed::Stored);
            held.into_inner().expect("held")
        };
        let record = WhatTheRecordHolds::default();
        let reading = |held: &WhatTheRunHolds, file: &str| {
            let TheFormatAnswer::TheBox(the_box) = the_download_panel_from(held, &a_server(), file, &record) else {
                panic!("the box");
            };
            the_box.reading
        };
        let another = |several: bool| TheFormatReading::ReadForAnotherScene {
            warning: Prose::new(if several {
                crate::surface::transfers::words::READ_ANOTHER_SCENE_SEVERAL_TICKED
            } else {
                crate::surface::transfers::words::READ_ANOTHER_SCENE
            }),
            what_downloads: crate::surface::transfers::what_downloads_without_a_list(None),
        };
        let tick = |held: &mut WhatTheRunHolds, scene: &str| {
            let provenance = the_provenance(held, &key_of(file)).expect("a reading stands");
            held.store.tick_a_scene(key_of(file), NamedAnswer { name: ExactName::as_the_reading_produced_it(scene), provenance });
        };
        let untick = |held: &mut WhatTheRunHolds, scene: &str| {
            held.store.untick_a_scene(key_of(file), &ExactName::as_the_reading_produced_it(scene));
        };

        let mut held = land(a_run_with_a_reading(), file, &["Interior", "Exterior"], Some("Interior"));
        assert!(matches!(reading(&held, file), TheFormatReading::SeveralFormats { .. }), "the read scene ticked");
        tick(&mut held, "Exterior");
        assert_eq!(reading(&held, file), another(true), "both ticked");
        untick(&mut held, "Interior");
        assert_eq!(reading(&held, file), another(false), "another ticked");
        let drawn = serde_json::to_value(reading(&held, file)).expect("drawn");
        assert_eq!(drawn["reads"], "read-for-another-scene");
        assert!(drawn.get("read_it_now").is_none() && drawn.get("to_choose").is_none(), "{drawn}");
        tick(&mut held, "Interior");
        untick(&mut held, "Exterior");
        assert!(matches!(reading(&held, file), TheFormatReading::SeveralFormats { .. }), "back to the read scene");

        // **A read that could not name its scene**, on a file of two.
        let mut unnamed = land(a_run_with_a_reading(), file, &["Interior", "Exterior"], None);
        for scene in ["Interior", "Exterior"] {
            untick(&mut unnamed, scene);
        }
        tick(&mut unnamed, "Exterior");
        assert_eq!(reading(&unnamed, file), another(false), "a read that named no scene");

        // **Never read**: the sentence that says so, with *Read it now*.
        assert!(matches!(reading(&WhatTheRunHolds::default(), file), TheFormatReading::NotRead { .. }));

        // **A file of one scene still lists its formats.**
        let single = land(a_run_with_a_reading(), one, &["Interior"], Some("Interior"));
        assert!(matches!(reading(&single, one), TheFormatReading::SeveralFormats { .. }), "{:?}", reading(&single, one));
        let single = land(a_run_with_a_reading(), one, &["Interior"], None);
        assert!(!matches!(reading(&single, one), TheFormatReading::ReadForAnotherScene { .. }), "one scene is never another");

        // **The words are the plan's and the drawing's.**
        let design = include_str!("../../../../Preparation/Design/Render Settings.md").replace('’', "'");
        let drawing = include_str!("../../../../Preparation/Design/Mockup/Smoozy.html").replace("&#8217;", "’");
        let the_sentence = crate::surface::transfers::words::READ_ANOTHER_SCENE;
        assert!(design.contains(&format!("*{} Everything downloads.*", the_sentence.replace('’', "'"))), "the plan's words");
        assert!(design.contains("*the selected scenes' formats are unknown*"));
        assert_eq!(
            crate::surface::transfers::words::READ_ANOTHER_SCENE_SEVERAL_TICKED,
            the_sentence.replace("the selected scene’s formats", "the selected scenes’ formats")
        );
        assert!(drawing.contains(the_sentence), "the drawing's words");
    }
}

/// The modules `web/transfers.js` imports, standing in for the box's test —
/// none of them is looked at.
#[cfg(test)]
const THE_BOX_STAND_INS: [(&str, &str); 8] = [
    ("event-route.js", "export function whenTold() {}"),
    (
        "the-rail.js",
        "export const A_SERVER = 'machineview'; export const TRANSFERS = 'transfersview'; \
         export function aScreenIsRegistered() {} export function theCurrentPlaceIs() {} \
         export function theRowsFigureIs() {}",
    ),
    (
        "the-failure-shape.js",
        "export const A_COLLAPSED_ROW = 'collapsed'; export const EXPANDED = 'expanded'; \
         export function theFailureShape() { return document.createElement('div'); }",
    ),
    (
        "the-underway-shape.js",
        "export function theMarkOnARow() { return document.createElement('span'); } \
         export function theCountdown() { return document.createElement('span'); }",
    ),
    ("the-dependency-check.js", "export async function theCheckIsShownOn() {}"),
    ("settings.js", "export const TRANSFERS_SECTION = 's-tr'; export function theSettingsSectionIsOpened() {}"),
    ("the-history.js", "export function theHistoryEntryIsOpened() {}"),
    ("the-value-drawn.js", "export function theValueIsDrawnInto(node, value) { node.textContent = value; }"),
];

/// **The box, opened as `web/render-settings.js` opens it, and pressed.** A
/// stand-in document with enough of the real one for the box: classes,
/// attributes, children, removal, and a click that reaches its listeners.
#[cfg(test)]
const THE_BOX_HARNESS: &str = r#"
class TextPiece {
  constructor(text) { this.data = String(text); this.parentNode = null; }
  get textContent() { return this.data; }
  remove() { if (this.parentNode) this.parentNode.removeChild(this); }
}
class Piece {
  constructor(tag) {
    this.tagName = tag.toUpperCase(); this.children = []; this.own = ""; this.parentNode = null;
    this.dataset = {}; this.attributes = {}; this.listeners = {}; this.disabled = false; this.type = "";
    const names = new Set(); this.names = names;
    this.classList = {
      add: (...more) => more.forEach((one) => names.add(one)),
      remove: (...more) => more.forEach((one) => names.delete(one)),
      toggle: (one, on) => { const want = on === undefined ? !names.has(one) : Boolean(on);
        if (want) names.add(one); else names.delete(one); return want; },
      contains: (one) => names.has(one),
    };
  }
  set className(value) { this.names.clear(); String(value).split(/\s+/).filter(Boolean).forEach((one) => this.names.add(one)); }
  get className() { return [...this.names].join(" "); }
  set textContent(value) { this.children.forEach((one) => { one.parentNode = null; }); this.children = []; this.own = String(value); }
  get textContent() { return this.own + this.children.map((one) => one.textContent).join(""); }
  get firstChild() { return this.children[0] || null; }
  appendChild(child) { if (child.parentNode) child.parentNode.removeChild(child); child.parentNode = this; this.children.push(child); return child; }
  removeChild(child) { const at = this.children.indexOf(child); if (at >= 0) this.children.splice(at, 1); child.parentNode = null; return child; }
  remove() { if (this.parentNode) this.parentNode.removeChild(this); }
  setAttribute(name, value) { this.attributes[name] = String(value); }
  getAttribute(name) { return name in this.attributes ? this.attributes[name] : null; }
  addEventListener(kind, handler) { (this.listeners[kind] ||= []).push(handler); }
  click() { for (const handler of this.listeners.click || []) handler({ preventDefault() {} }); }
  focus() { document.activeElement = this; }
  closest() { return null; }
  everyPiece(found = []) { for (const one of this.children) if (one instanceof Piece) { found.push(one); one.everyPiece(found); } return found; }
  matches(selector) { return selector.startsWith(".") ? this.names.has(selector.slice(1)) : this.tagName === selector.toUpperCase(); }
  querySelector(selector) { return this.everyPiece().find((one) => one.matches(selector)) || null; }
  querySelectorAll(selector) { return this.everyPiece().filter((one) => one.matches(selector)); }
}
globalThis.document = {
  createElement: (tag) => new Piece(tag),
  createTextNode: (text) => new TextPiece(text),
  addEventListener() {},
  activeElement: null,
  body: new Piece("body"),
};
const fs = await import("node:fs");
const answers = JSON.parse(fs.readFileSync("./answers.json", "utf8"));
const calls = [];
let thePanelIsClosed = false;
globalThis.window = {
  addEventListener() {},
  __TAURI__: { core: { invoke: (command) => {
    calls.push(command);
    if (command === "the_folder_is_asked_for_in_the_box" && thePanelIsClosed) return Promise.resolve(null);
    return Promise.resolve(command in answers ? answers[command] : null);
  } } },
};
const settle = () => new Promise((go) => setImmediate(go));
const { theFormatBoxIsOpenedForAFile } = await import("./transfers.js");
const plain = (text) => text.replace(/ /g, " ");
const rows = () => document.body.querySelectorAll(".choice3");
const theChosenRow = () => rows().findIndex((one) => one.classList.contains("on"));
const aButton = (label) => document.body.everyPiece().find((one) => one.tagName === "BUTTON" && plain(one.textContent) === label);
const asked = () => window.__TAURI__.core.invoke("the_folder_is_asked_for_in_the_box", {});
const out = {};
let done = null;

await theFormatBoxIsOpenedForAFile("server", [1], (chosen) => { done = chosen; }, () => {}, asked);
out.readItNowReadsUnavailable = aButton(plain(answers.the_download_panel.reading.read_it_now)).disabled === true;
rows()[1].click();
const before = calls.length;
rows()[rows().length - 1].click();
out.thePanelIsAskedAtOnce = calls.slice(before).includes("the_folder_is_asked_for_in_the_box");
await settle(); await settle();
out.theRowsRead = rows().map((one) => plain(one.textContent));
out.theFolderItGaveIsTheChoice = theChosenRow() === 0;
aButton("Done").click();
out.done = done;

thePanelIsClosed = true;
await theFormatBoxIsOpenedForAFile("server", [1], () => {}, () => {}, asked);
rows()[1].click();
rows()[rows().length - 1].click();
await settle(); await settle();
out.closedLeavesTheBoxAsItWas = theChosenRow() === 1 && rows().length === 3;

answers.the_download_panel = answers.the_download_panel_for_another_scene;
await theFormatBoxIsOpenedForAFile("server", [1], () => {}, () => {}, asked);
const warned = document.body.querySelectorAll(".mwarn").pop();
out.anotherSceneSays = warned ? plain(warned.textContent) : null;
out.anotherSceneOffersNoRead = Boolean(warned) && !warned.everyPiece().some((one) => one.tagName === "BUTTON");

answers.the_download_panel = answers.the_download_panel_narrowed;
await theFormatBoxIsOpenedForAFile("server", [1], () => {}, () => {}, asked);
const narrowed = document.body.querySelectorAll(".mwarn").pop();
out.narrowedSays = narrowed ? plain(narrowed.textContent) : null;
console.log(JSON.stringify(out));
"#;

/// The stand-in document `web/the-typed-text.js` is run against.
#[cfg(test)]
const THE_TYPED_TEXT_HARNESS: &str = r#"// A stand-in for the little of a document the module reads.
const listeners = new Map();
globalThis.document = {
  activeElement: null,
  addEventListener(type, fn) {
    if (!listeners.has(type)) listeners.set(type, []);
    listeners.get(type).push(fn);
  },
};
function fire(target, type, extra) {
  let stopped = false;
  const event = Object.assign({ type, target, stopImmediatePropagation() { stopped = true; } }, extra || {});
  for (const fn of listeners.get(type) || []) {
    fn(event);
    if (stopped) return;
  }
  for (const fn of target.own[type] || []) fn(event);
}
class Field {
  constructor(value) {
    this.tagName = "INPUT";
    this.type = "text";
    this.readOnly = false;
    this.value = value;
    this.selectionStart = value.length;
    this.selectionEnd = value.length;
    this.selectionDirection = "none";
    this.scrollLeft = 0;
    this.isConnected = true;
    this.own = {};
    this.changes = [];
    this.own.change = [() => this.changes.push(this.value)];
  }
  focus() { document.activeElement = this; }
  setSelectionRange(start, end, direction) { this.selectionStart = start; this.selectionEnd = end; this.selectionDirection = direction; }
  dispatchEvent(event) { fire(this, event.type); return true; }
}
const holder = (...fields) => ({ contains: (one) => fields.includes(one) && one.isConnected });
const typing = (field, text) => { field.value += text; field.selectionStart = field.selectionEnd = field.value.length; fire(field, "input"); };

const { aFieldIsBeingTypedIn, theTypingIsTakenFrom, theTypingIsPutBackInto } = await import("./the-typed-text.mjs");
const out = {};

// The ninth pass's case: typed, an answer arrives, the field is drawn again.
const name = new Field("");
name.focus();
typing(name, "p12h9 two");
name.setSelectionRange(5, 5, "none");
out.typingIsMarked = aFieldIsBeingTypedIn(name);
const taken = theTypingIsTakenFrom(holder(name));
name.isConnected = false;
fire(name, "change");
out.aFieldTakenOutCommitsNothing = name.changes.length === 0;
const again = new Field("");
theTypingIsPutBackInto(again, taken);
out.theTypingComesBack = again.value === "p12h9 two" && document.activeElement === again && again.selectionStart === 5 && again.selectionEnd === 5;
out.itIsStillBeingTypedIn = aFieldIsBeingTypedIn(again);
// Enter, with no key since: the change the engine will not send is sent, once.
fire(again, "keyup", { key: "Enter" });
fire(again, "keyup", { key: "Enter" });
out.enterCommitsItOnce = again.changes.length === 1 && again.changes[0] === "p12h9 two";

// Left for another field, with no key since: committed once.
const samples = new Field("");
samples.focus();
typing(samples, "25");
const taken2 = theTypingIsTakenFrom(holder(samples));
samples.isConnected = false;
const samples2 = new Field("");
theTypingIsPutBackInto(samples2, taken2);
document.activeElement = null;
fire(samples2, "focusout");
out.leavingCommitsItOnce = samples2.changes.length === 1 && samples2.changes[0] === "25";

// The window put behind another: the focus stays, nothing is committed.
const range = new Field("");
range.focus();
typing(range, "1-9");
const taken3 = theTypingIsTakenFrom(holder(range));
range.isConnected = false;
const range2 = new Field("");
theTypingIsPutBackInto(range2, taken3);
fire(range2, "focusout");
out.theWindowGoingBehindCommitsNothing = range2.changes.length === 0;

// A key typed after it came back: the engine's own change is the only one.
typing(range2, "0");
fire(range2, "change");
fire(range2, "keyup", { key: "Enter" });
out.aKeyAfterLeavesTheCommitToTheEngine = range2.changes.length === 1 && range2.changes[0] === "1-90";

// A field whose own answer went is drawn from the answer.
const resolution = new Field("");
resolution.focus();
typing(resolution, "50");
fire(resolution, "change");
const taken4 = theTypingIsTakenFrom(holder(resolution));
resolution.isConnected = false;
const resolution2 = new Field("75");
theTypingIsPutBackInto(resolution2, taken4);
out.anAnsweredFieldIsDrawnFromTheAnswer = resolution2.value === "75" && document.activeElement === resolution2 && !aFieldIsBeingTypedIn(resolution2);

// Nothing is taken from a part the focus is not in.
const elsewhere = new Field("x");
out.nothingIsTakenWhereTheFocusIsNot = theTypingIsTakenFrom(holder(elsewhere)) === null;

console.log(JSON.stringify(out));
"#;

//! **§2 performed — P1's nine steps driven in one order, with P1a between
//! the fourth and the seventh and P3's rule where the supervisor is not
//! answering.**
//!
//! # What this module is, and what it is not
//!
//! **It is the driver, and the order is the whole of what it owns.**
//! [`crate::pipeline`] states the nine steps and the judgements inside them;
//! every step's real work belongs to the document that owns it and arrives
//! here as a value or through a seam. **Nothing here re-decides a step**: the
//! reckoning is `Job files and outputs` §10's, taken by calling that crate's
//! own function; the map, the inventory and the two picks are that document's
//! §4.2 item 7, composed by its one
//! `composing::the_reading_and_everything_derived_from`; the hold conditions
//! are its §9.1's, judged by its own `holds::what_holds_this_job` with
//! `Job files and outputs` H5's two triggers; the hold record and its identity
//! are `Rented-server runtime` §8's;
//! the composition is [`crate::composition::compose_the_attempt`]; and the
//! classification is [`crate::endings::classify`].
//!
//! **The steps are walked off [`crate::pipeline::THE_NINE_STEPS`]** rather
//! than written out again, so a step added to the plan's own list is a step
//! this driver meets rather than one somebody has to remember. The `match`
//! over the numbers is exhaustive by its own last arm.
//!
//! # Step 4 is the step everything downstream hangs off, and it used not to be
//!
//! **Until 2026-09-20 this driver took the map in force, its redirect pairs,
//! the expected-output inventory, the whole asked range and the hold answer as
//! inputs composed before step 1** — before the step that produces them. Every
//! one of the eight faults the step 10c reading found followed from that: a job
//! whose file had changed would have been rendered with the manifest's old map,
//! which is `Job files and outputs` R13's forbidden case; the two hold
//! conditions `Job files and outputs` H5 fires only where the map was computed
//! on the machine could
//! never fire; the attempt record carried the manifest's map and the identity
//! at submission rather than the map in force and the identity read at step 3;
//! and a job asking for the file's own frame range reached step 7 with no frame
//! to name. **They are one fault and step 4 is where it is fixed**: what step 3
//! decides and step 4 composes is what steps 5, 6 and 7 read, and nothing is
//! handed in ahead of them.
//!
//! # P2, kept by the module rather than by care
//!
//! **Nothing in the pipeline needs a connection and nothing in it needs
//! Smoozy.** There is no connection, no transfer and no desktop store
//! anywhere in this file: [`TheWorkersOwnSteps`]'s methods take and answer
//! values the machine already holds, the reading is a headless run of a
//! Blender already on the machine ([`crate::acting::reading`]), and every
//! other input of [`TheAttempt`] is durable on the machine before the job
//! exists (L29).
//!
//! # P3, and the one thing a hold may never be
//!
//! **Where the supervisor is not answering at the moment a hold would be
//! placed, the attempt places no hold at all**: it ends as an interrupted
//! attempt and reconciliation takes it. **No hold record is written without a
//! supervisor-issued identity**, kept here by the identity arriving from the
//! supervisor's own answer and by there being no branch that composes one.
//!
//! # Why the order inside step 7 matters
//!
//! **Any uploaded bake the remap material owes is copied into the job folder,
//! the material is placed and read back, then the attempt record is written
//! durably, and only then is the render launched.** The copy is `Dependency
//! Upload & Remap` M22c's and was added on 2026-09-29; a bake sent and not
//! copied is one the render never reads, so a copy that fails ends the attempt
//! here, naming it. A render started
//! before its record is a render whose frame list no later attempt can read,
//! which falls to `Job files and outputs` C20's whole range — re-rendering
//! and re-paying for everything the last attempt wrote. So a refused record
//! ends the attempt before the launch.
//!
//! **And a render whose recorded pair could not be written is ended rather
//! than left running.** L5 aims the stop at that pair; without it there is a
//! Blender on a machine the user pays for by the second that no stop can
//! ever reach.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Job files and outputs` C7, C9, C18a, C20,
//! M8, R1, R2, R4a, R7, R9, R10, R12, R13. `Rented-server runtime` U6.
//! `State, identity and recovery` D5.
//!
//! **The `Job files and outputs` H and I clauses and the `Render Settings`
//! Y clauses this file cites are written out at every site rather than listed
//! here**, each because this plan defines a clause of the same name.

use smoozy_dependencies_machine::placed_copies::{APlacedFile, ThePlacedCopies};
use smoozy_dependencies_machine::remap::RemapMaterial;
use smoozy_job_files_machine::attempts::{AttemptRecord, TheMapInForce};
use smoozy_job_files_machine::composing::{
    the_reading_and_everything_derived_from, TheReadingAndWhatFollows,
    WhatTheReadingIsComposedAgainst,
};
use smoozy_job_files_machine::inventory::WhatTheInventoryIs;
use smoozy_job_files_machine::main_output::WhereTheRenderEnters;
use smoozy_job_files_machine::manifest::{FrameRange, TheReadingAndEverythingDerivedFromIt};
use smoozy_job_files_machine::layout::TheLayout;
use smoozy_job_files_machine::mapping::{HowItMapped, WhyItIsNotMappable};
use smoozy_job_files_machine::where_the_folder_sits::{
    the_folder_is_chosen_and_made, the_run_of, MakesFolders, WhatTheChoiceIsMadeFrom,
    WhereTheJobOutputFolderSits,
};
use smoozy_job_files_machine::reading::{
    the_layer_the_walk_enters_at, what_the_jobs_pin_says, ArtistsText, AViewLayerTheJobPins,
    TheVersionInHand, WhichSideComposedTheMap,
};
use smoozy_job_files_machine::reckoning::{
    any_frame_an_earlier_attempt_wrote_was_kept, WhatTheReckoningSays, WhatTheWalkOfTheFolderIs,
};
use smoozy_render_settings_machine::file_half::{BlenderIdentity, RecordedReading};
use smoozy_render_settings_machine::frames::FrameForm;
use smoozy_render_settings_machine::payload::Payload;
use smoozy_render_settings_machine::reading::{SceneReading, SettingsHalf};
use smoozy_framing::source_reading::SourceReading;
use smoozy_server_setup_machine::blender::Version;
use smoozy_state::identities::Identity;
use smoozy_state::observed::{Clock, Reading};

use crate::acting::launch::{
    AsksABlenderToNameItsVersion, StartsTheRender, TheLaunch, TheRenderAsItWasStarted,
};
use crate::acting::material::{
    place_and_confirm, the_materials_text, PlacesTheMaterial, WhetherThePlacementIsConfirmed,
};
use crate::acting::reading::{TakesTheReading, TheReadingToTake};
use crate::acting::the_optix_library::LooksForTheOptixLibrary;
use crate::technology::Technology;
use smoozy_server_setup_machine::the_optix_library::WhatTheLoaderFound;
use crate::composition::{compose_the_attempt, AttemptInputs};
use crate::endings::BeforeTheRenderInstruction;
use crate::material::RedirectAssignment;
use crate::pipeline::{validate_the_names, Step, ValidatedNames, THE_NINE_STEPS};
use crate::versions::{confirm_the_version, VersionRefused};

/// The steps' own work, behind one trait — **every method is a read or a
/// write the machine answers, and every one of them belongs to another
/// document.**
///
/// **None of them takes a decision.** Each hands back what it found, and what
/// that means is the rules half's.
pub trait TheWorkersOwnSteps {
    /// **Step 2, L30**: the remap material the manifest names, read off the
    /// machine. `None` where it could not be read — **never an empty
    /// material**, because a material composed without it renders the
    /// artist's own recorded locations in force and finishes at success.
    fn read_the_remap_material(&mut self, at: &str) -> Option<RemapMaterial>;

    /// **Step 3, `Job files and outputs` R10 and R12**: the `.blend`'s size,
    /// inode, modification time and change time together, as they stand now.
    /// `None` where the reading could not be taken, **which is not a match**.
    fn the_four_values_now(&mut self, blend_path: &str) -> Option<SourceReading>;

    /// **Step 4, `Job files and outputs` H9 and `State, identity and recovery`
    /// §1.2**: the recorded reading and everything derived from it, written
    /// into the job record directory, and **the supervisor asked to journal it
    /// when it is written**.
    ///
    /// **It answers nothing, and that is deliberate.** The plan names no
    /// ending for a recorded reading that could not be written — the attempt
    /// record is the durable thing M8 puts before the launch — so a write that
    /// fails is the machine's own to complain about, loudly, in its own words,
    /// at the site that has words to complain with. **The render is what the
    /// user paid for and it goes on**; what is lost is the telling
    /// `Job files and outputs` H8 names.
    fn write_the_recorded_reading(&mut self, composed: &TheReadingAndWhatFollows);

    /// **Step 5**: the supervisor issues the hold identity, writes the hold
    /// record and journals it. `None` where the supervisor is not answering
    /// — **which is P3, and no identity is composed here.**
    fn ask_the_supervisor_to_place_the_hold(&mut self, reason: &str) -> Option<Identity>;

    /// **Step 6, `Job files and outputs` C7**: the one walk of the job output
    /// folder.
    fn walk_the_job_output_folder(&mut self, at: &str) -> WhatTheWalkOfTheFolderIs;

    /// **Step 4, `Job files and outputs` P20**: the machine's own stat and
    /// plain make, for the job output folder chosen at render start (`Job
    /// files and outputs` P21's second case). The choice is `Job files and outputs`' own
    /// ([`the_folder_is_chosen_and_made`]); this is only where it acts.
    fn the_machines_folders(&mut self) -> &mut dyn MakesFolders;

    /// **Step 4, `Job files and outputs` P21**: the job output folder's path,
    /// its case and whether the server's chosen fallback folder was passed
    /// over, written into the job record directory once, durably, before
    /// step 5 — so a later attempt never chooses again and the desktop reads
    /// where the files went (`Queue & History` N9). **A write that fails ends
    /// the attempt before the render**: a folder nothing records is a render
    /// nobody can find or bring home.
    fn write_where_the_folder_sits(&mut self, sits: &WhereTheJobOutputFolderSits) -> Result<(), String>;

    /// **Step 7, `Job files and outputs` M8**: the attempt record, written
    /// durably **before the render starts**.
    fn write_the_attempt_record(&mut self, record: &AttemptRecord) -> Result<(), String>;

    /// **Step 7, `Dependency Upload & Remap` M22c**: an uploaded bake copied on
    /// the machine, from its mirror in the asset area to the job-folder place
    /// its write place maps to — **a copy and never a link**, so a render that
    /// recomputes writes over the copy and never over the user's own upload.
    /// A folder is copied whole, each file beneath `to` at the path it had
    /// beneath `from`; a single file lands beneath `to` under its own name.
    ///
    /// **It answers every file it placed with the four values read the moment
    /// that file was made** (`Dependency Upload & Remap` M22f), or why the copy
    /// could not be made.
    fn copy_into_the_job_folder(
        &mut self,
        from: &[u8],
        to: &[u8],
    ) -> Result<Vec<APlacedFile>, String>;

    /// **`Dependency Upload & Remap` M22f**: the attempt's placed-copy list,
    /// written into the job record directory for the desktop to take into the
    /// job's own record.
    ///
    /// **It answers nothing, the way the recorded reading's write does not.**
    /// A list that could not be written costs `Download` N5a its skip, whose
    /// failure direction is the safe one — the placed bake comes home, and the
    /// user downloads a file they already have. **The render goes on**, and the
    /// machine says so in its own words.
    fn write_the_placed_copies(&mut self, placed: &ThePlacedCopies);

    /// **`Dependency Upload & Remap` M8b, on the machine's own disk**: the folder
    /// the file opens from, so its libraries load with it. See
    /// [`crate::acting::opening::make_the_opening_folder`].
    fn make_the_opening_folder(
        &mut self,
        folder: &smoozy_dependencies_machine::opening::TheOpeningFolder,
    ) -> Result<(), String>;

    /// **Step 7, L5**: the render process's identity paired with its
    /// kernel-set start time, written into the operation record at the moment
    /// it was started.
    fn record_the_render_process(
        &mut self,
        pair: smoozy_runtime::execution_group::RecordedPair,
    ) -> Result<(), String>;
}

/// Where an attempt ended, by the step that ended it.
#[derive(Debug)]
pub enum WhereTheAttemptEnded {
    /// The launch was made and the render is handed back held. **What
    /// happens after that is the copy loop's and the ending step's**, which
    /// are steps 8 and 9 and are not this driver's.
    TheRenderWasLaunched(TheRenderAsItWasStarted),
    /// **Step 5 placed a hold**: the attempt ends, the job does not, and no
    /// render was started (`Rented-server runtime` U6).
    AHoldWasPlaced { hold: Identity },
    /// **P3**: the supervisor was not answering, so no hold was placed at all
    /// and the attempt ends as an interrupted one.
    NoHoldWasPlacedAndReconciliationTakesIt,
    /// K14's failed limb, before the render instruction, with the step that
    /// reached it.
    FailedBeforeTheRenderInstruction {
        at: Step,
        reason: BeforeTheRenderInstruction,
    },
    /// **Step 6's walk did not complete**, which `Job files and outputs` C9
    /// makes a failed attempt rather than a whole range.
    TheWalkDidNotComplete { at: Step },
    /// **Step 7's attempt record could not be written durably**, so no render
    /// was started.
    TheAttemptRecordCouldNotBeWritten { at: Step, why: String },
    /// The render was started and its recorded pair could not be written.
    /// **The render was ended rather than left running.**
    TheRecordedPairCouldNotBeWritten { at: Step, why: String },

    // ---------------------------------------------------------------
    // **Three endings K14's own enumeration did not name until 2026-09-05,
    // and the plan is what moved.**
    //
    // These three, and the two above them — the attempt record that could
    // not be written and the recorded pair that could not be written — were
    // reached by this driver while K14's failed limb, a closed set of the
    // plan's, named none of them. **This crate refused to widen a closed set
    // of the plan's on its own authority and wrote the omission down here**,
    // which Round 1 of its code audit found and confirmed as the right way
    // round; K14 and K14a were repaired first, and the five now sit on that
    // limb with *undetermined* beside each.
    //
    // **They stay arms of this driver's own type**, because what this type
    // says is where an attempt ended and not what was recorded of it.
    // **[`crate::endings::the_outcome_of`] is the one route from here into
    // the classification**, so the endings have one home rather than a
    // reading invented at each call site.
    // ---------------------------------------------------------------
    /// **L12's read-back did not confirm the placement.** A failed attempt
    /// before the render instruction, with what the confirmation found.
    /// **Nothing is written again and nothing is repaired**: a second write
    /// is a second thing that could have gone wrong.
    TheMaterialsPlacementWasNotConfirmed {
        at: Step,
        what_happened: WhetherThePlacementIsConfirmed,
    },
    /// **The attempt could not be composed at all** — a spelling that has
    /// not been read (`Render Settings` §13 item 29's five landed on
    /// 2026-09-20 and nothing is owed today), a frame list with no frame to
    /// name, or a line the contract's own order refuses. Loud and total,
    /// with nothing written.
    TheAttemptCouldNotBeComposed { at: Step, what_happened: String },
    /// **The launch itself could not be made** — the build's path could not
    /// be run, or its streams could not be held.
    TheLaunchCouldNotBeMade { at: Step, why: String },
}

/// **How one attempt went**: where it ended, and the two values steps 3 and 4
/// settled that the ending step of P1 step 9 reads back.
///
/// **They are answers of this driver rather than inputs to it**, which is the
/// whole of what step 4 being real changes: the inventory the completeness
/// statement is counted against is *the inventory for the map in force* (C7),
/// and the range it is counted over is the manifest's integers or the file's
/// own as the reading in force gave it (`Render Settings` Y9).
#[derive(Debug)]
pub struct HowTheAttemptWent {
    pub ended: WhereTheAttemptEnded,
    /// **The job output folder in force for this attempt** — the one the
    /// manifest names, the one an earlier attempt chose, or the one step 4
    /// chose (`Job files and outputs` P21). `None` where the attempt ended
    /// before any stood, and then nothing walks one.
    pub the_job_output_folder: Option<String>,
    /// **The inventory for the map in force** (C7). `None` where the attempt
    /// ended before step 4 composed one, in which case the caller says why it
    /// can state no completeness — it never counts against nothing.
    pub inventory: Option<WhatTheInventoryIs>,
    /// **The whole asked range** the reckoning covers (C20), composed from the
    /// reading in force where the manifest asks for the file's own.
    pub the_whole_range: Vec<i64>,
    /// **`Queue & History` B1's plan for the job — its renderlets**, worked
    /// out once from the reading in force; `Queue & History` B1's total and
    /// `Queue & History` B6's finished part are both summed from it and
    /// carried in every report the worker sends
    /// (`Queue & History` R5). `None` where the attempt ended before a reading
    /// stood, or where none could be worked out ([`the_plan_of`]). Added
    /// 2026-10-03, the Creator's notes on 0.1.8, piece 6a, as the total;
    /// carried whole since piece 6b, so the finished part is summed from the
    /// same renderlets.
    pub the_plan: Option<Vec<smoozy_queue_history_machine::plan::Renderlet>>,
}

impl HowTheAttemptWent {
    /// **`Queue & History` B1's plan total**, summed from [`Self::the_plan`].
    pub fn the_plan_total(&self) -> Option<u64> {
        self.the_plan
            .as_deref()
            .map(smoozy_queue_history_machine::plan::the_plan_total)
    }
}

/// What one attempt is handed, all of it durable on the machine (L29).
///
/// **Every field is another document's value, composed before this driver
/// runs.** The driver reads none of them apart: it sequences, and hands them
/// to the functions that own them.
///
/// **What is deliberately not here**: the map in force, its redirect pairs,
/// the regular output path's pair, the expected-output inventory, the whole
/// asked range and the hold answer. **All six are what steps 3, 4 and 5
/// produce**, and every one of them was an input of this type until
/// 2026-09-20, which is the fault the module's own front matter names.
pub struct TheAttempt<'a> {
    pub job: &'a Identity,
    pub manifest: &'a smoozy_job_files_machine::manifest::SubmissionManifest,
    /// The payload read back out of the manifest's carried field.
    pub payload: &'a Payload,
    /// The version the manifest names, parsed as components (`Server Setup`
    /// B2's discipline) by the caller that read the manifest.
    pub the_manifests_version: Version,
    /// Where the remap material sits, from the manifest's own reference.
    pub the_remap_materials_path: &'a str,
    /// The Blender the manifest names, by path — **a version alone cannot
    /// identify a build.**
    pub the_blender_build_path: &'a str,
    /// Where the launch material is placed and read back (L12).
    pub the_launch_materials_path: &'a str,
    /// **Where L8b's script before the file is placed and read back** (L12).
    pub the_load_time_scripts_path: &'a str,
    /// Where the reading's own script is written (`Job files and outputs` R1).
    pub the_readings_script_path: &'a str,
    /// **The job output folder**, walked at step 6 — known before the
    /// attempt, or chosen at step 4 (`Job files and outputs` P21).
    pub the_job_output_folder: TheJobOutputFolderForThisAttempt<'a>,
    /// **The folder the `.blend` sits in on the machine, as bytes** — what
    /// `Job files and outputs` P4's blend-file shorthand resolves against when
    /// the map is composed. **R4a: never decoded.**
    pub the_folder_the_blend_sits_in: &'a [u8],
    /// The attempt records already written, oldest first (C18a).
    pub previous_attempts: &'a [AttemptRecord],
    /// This attempt's number.
    pub number: u32,
    // **The cards are not a field of this shape and never were one the
    // machine could fill.** L23's make comes off the manifest above (`Job
    // files and outputs` M1b), so nothing here carries a second copy of it.
    //
    // **This field existed until 2026-09-20 and every machine-side caller
    // passed *the cards could not be established***, which is what made every
    // cards job end at L23's loud failure before the render.
    /// **The machine's own clock** (`State, identity and recovery` D5), which
    /// the moment on a reading taken here is stamped on. **A machine that
    /// names no clock never reaches this driver**: its caller refuses the
    /// attempt, the same way its supervisor journals nothing.
    pub the_machines_clock: &'a Clock,
    /// **The manifest's recorded reading and everything derived from it**,
    /// where it carries one. **Step 3 decides whether it stands** (R12, R13);
    /// where it does, R7's first bullet makes that map the map in force and
    /// **nothing is recomputed** (R9).
    pub the_manifests_reading: Option<&'a TheReadingAndEverythingDerivedFromIt>,
    /// **The settings half of that same recorded reading**, where the manifest
    /// carries one.
    ///
    /// **It is a second field because the manifest in code carries only the
    /// outputs half**, which `Job files and outputs` M1 and `Render Settings`
    /// Q1 both say should be one reading with both halves. That is the code
    /// being narrower than the plan and it is piece 15's to close; until it
    /// does, this is `None` on every manifest ever written, and step 4 takes
    /// the reading for the settings half even where the manifest's map stands.
    pub the_manifests_settings_half: Option<&'a RecordedReading>,
}

/// **Where this attempt's job output folder comes from** — `Job files and
/// outputs` P21: chosen once per job, on the side that composes the job's
/// first map, and never again.
#[derive(Debug, Clone, Copy)]
pub enum TheJobOutputFolderForThisAttempt<'a> {
    /// **Known before the attempt**: the manifest names it, or an earlier
    /// attempt of this job chose it and recorded it (`Job files and outputs`
        /// P21: *a later attempt
    /// never chooses again*).
    Known(&'a str),
    /// **`Job files and outputs` P21's second case, at this job's first
    /// attempt to reach step 4**:
    /// chosen there, with the map, and made before step 5.
    ToBeChosen {
        /// The machine's layout — the root and the package area's location,
        /// which `Job files and outputs` P19 case 2 keeps every job's folder
        /// out of.
        layout: &'a TheLayout,
        /// `Job files and outputs` N3's name, as the desktop composed it with
        /// the label (`Job files and outputs` M1).
        the_folders_name: &'a str,
        /// `Job files and outputs` P23's chosen fallback folder, as the
        /// manifest carried it.
        the_servers_fallback_folder: Option<&'a str>,
    },
}

/// What steps 3, 4 and 5 settle, in the order they settle it.
///
/// **Nothing in here is handed in.** It is the working state of one run of
/// [`TheRenderWorkersPipeline::run`], named rather than left as eight locals.
#[derive(Default)]
struct WhatTheStepsSettled {
    /// **The job output folder in force** — known before the attempt, or
    /// chosen at step 4 (`Job files and outputs` P21).
    job_output_folder: Option<String>,
    /// The reading in force for this attempt (P1a) — the manifest's where step
    /// 3 found it un-stale, the fresh one where it did not.
    reading: Option<RecordedReading>,
    /// P1a's answer, and the composition is made from it and from nothing
    /// handed in.
    validated: Option<ValidatedNames>,
    /// **R7: which side composed the map in force**, which is the one fact
    /// that document's two hold triggers read.
    side: Option<WhichSideComposedTheMap>,
    map_in_force: Option<smoozy_job_files_machine::mapping::TheMap>,
    inventory: Option<WhatTheInventoryIs>,
    redirect_pairs: Vec<RedirectAssignment>,
    regular_output_path: Vec<u8>,
    /// **`Render Settings` Y9**: the manifest's parsed integers, or the file's
    /// own range composed against the reading in force.
    whole_range: Vec<i64>,
    /// The reckoning's own list (step 6).
    list: Vec<i64>,
    /// **The two facts `Job files and outputs` M8 keeps on the attempt record
    /// for the telling, and neither is read by the render.** The first is R13,
    /// learned at step 3; the second is `Job files and outputs` C12a, learned at
    /// step 6.
    the_file_changed: bool,
    a_frame_an_earlier_attempt_wrote_was_kept: bool,
    /// The four values as they stood at step 3, which M8 records.
    the_identity_read_at_step_three: Option<SourceReading>,
    /// **`Job files and outputs` D21b-i, added 2026-10-02: the baked physics
    /// caches the step-4 reading found on this machine**, which step 7 copies
    /// into their job-folder values. None where the map came from the
    /// manifest, whose reading was not taken on this machine.
    the_baked_caches_on_the_machine:
        Vec<smoozy_job_files_machine::reading::ABakedCacheOnTheMachine>,
    /// **`Job files and outputs` R4 item 8, added 2026-10-02: whether the job's
    /// scene holds a simulation that is not baked**, read off this attempt's
    /// step-4 reading — which is taken on every attempt, the map in force
    /// coming from the manifest or not — and what L10b's stepping is gated on.
    the_scene_holds_a_simulation_not_baked: bool,
    /// **`Job files and outputs` R4c, added 2026-10-27: whether this
    /// attempt's step-4 reading switched the job's scene from a video to a
    /// still picture** — a first-frame check of a video-only job — which is
    /// what L10c's entry in the material is gated on.
    the_check_is_written_as_a_picture: bool,
    /// **`Dependency Upload & Remap` M8b's folder, made at step 4 before the
    /// reading** (`Job files and outputs` R1a), which the reading opens the file
    /// from and the launch line does too. None where no library pair is relative
    /// to the file: the file then opens where it sits.
    the_folder_it_opens_from: Option<smoozy_dependencies_machine::opening::TheOpeningFolder>,
}

/// The pipeline, holding its seams.
pub struct TheRenderWorkersPipeline<'a, S, P, V, R, W, O> {
    pub start: &'a S,
    pub places: &'a P,
    pub asks_the_version: &'a V,
    pub takes_the_reading: &'a R,
    pub steps: &'a mut W,
    /// **L24b's lookup**, asked only where the job names `OPTIX`. Added
    /// 2026-10-03.
    pub looks_for_the_optix_library: &'a O,
}

impl<S, P, V, R, W, O> TheRenderWorkersPipeline<'_, S, P, V, R, W, O>
where
    S: StartsTheRender,
    P: PlacesTheMaterial,
    V: AsksABlenderToNameItsVersion,
    R: TakesTheReading,
    W: TheWorkersOwnSteps,
    O: LooksForTheOptixLibrary,
{
    /// **P1, performed: one pipeline, in one order, and the order is the
    /// contract's.**
    pub fn run(&mut self, attempt: TheAttempt<'_>) -> HowTheAttemptWent {
        let mut remap: Option<RemapMaterial> = None;
        let mut settled = WhatTheStepsSettled::default();
        if let TheJobOutputFolderForThisAttempt::Known(folder) = attempt.the_job_output_folder {
            settled.job_output_folder = Some(folder.to_string());
        }

        for step in THE_NINE_STEPS {
            let ended = match step.number {
                // **Step 1 is the supervisor's**: it admits the render
                // operation at the one admission point, writes the operation
                // record, creates the two sinks and starts this worker. By the
                // time this driver runs it has happened.
                1 => None,
                2 => match self
                    .steps
                    .read_the_remap_material(attempt.the_remap_materials_path)
                {
                    Some(read) => {
                        remap = Some(read);
                        None
                    }
                    None => Some(failed(
                        step,
                        BeforeTheRenderInstruction::TheRemapMaterialCouldNotBeRead {
                            material: attempt.the_remap_materials_path.to_string(),
                        },
                    )),
                },
                3 => {
                    self.step_three(&attempt, &mut settled);
                    None
                }
                4 => self.step_four(&attempt, step, remap.as_ref(), &mut settled),
                5 => self.step_five(&attempt, &settled),
                6 => self.step_six(&attempt, step, &mut settled),
                7 => Some(self.step_seven(&attempt, step, remap.as_ref(), &settled)),
                // **Steps 8 and 9 are the copy loop's and the ending step's**
                // and neither happens inside this driver: the launch is handed
                // back held, the worker runs [`crate::acting::copy_loop`] over
                // it, reaps, walks and reports.
                8 | 9 => None,
                other => unreachable!("the plan's own list gained step {other}"),
            };
            if let Some(ended) = ended {
                let the_plan = the_plan_of(&attempt, &settled);
                return HowTheAttemptWent {
                    ended,
                    the_job_output_folder: settled.job_output_folder,
                    inventory: settled.inventory,
                    the_whole_range: settled.whole_range,
                    the_plan,
                };
            }
        }
        unreachable!("step 7 always ends the attempt")
    }

    /// **Step 3: the fresh four-value check, and the one thing it decides.**
    ///
    /// **R12's first of two moments**, and R7 and R13's rule over it: where the
    /// values match and the manifest carries a map, **that map is the map in
    /// force and nothing is recomputed**; where they differ, or the manifest
    /// carries no map, step 4 takes the reading and composes a fresh one.
    ///
    /// **A reading of the four values that could not be taken is not a
    /// match and is not a change**: nothing says they differ, so the file is
    /// read again and the manifest's map does not stand on a check that was
    /// never made.
    fn step_three(&mut self, attempt: &TheAttempt<'_>, settled: &mut WhatTheStepsSettled) {
        let now = self
            .steps
            .the_four_values_now(&attempt.manifest.blend_path_on_the_machine);
        let unchanged = now
            .as_ref()
            .is_some_and(|now| *now == attempt.manifest.file_identity_at_submission);
        settled.the_file_changed = now
            .as_ref()
            .is_some_and(|now| *now != attempt.manifest.file_identity_at_submission);
        settled.the_identity_read_at_step_three = now;

        // **R13's forbidden case, and it is forbidden here by construction**:
        // the manifest's map is taken only where the fresh check matched. A
        // file that changed under a job whose manifest carries a map takes
        // neither the map nor the reading it was composed from.
        if unchanged {
            if let Some(carried) = attempt.the_manifests_reading {
                settled.side = Some(WhichSideComposedTheMap::TheDesktopAtSubmission);
                settled.map_in_force = Some(carried.map.clone());
                settled.inventory = Some(carried.inventory.clone());
                settled.reading = attempt.the_manifests_settings_half.cloned();
            }
        }
    }

    /// **Step 4: the reading where one is owed, L14's confirmation, which
    /// rides this step, the composition of everything derived from the
    /// reading, and P1a's validation, which rides it too.**
    ///
    /// **P1a sits between step 4 and step 7 and the numbering is not
    /// renumbered for it** ([`crate::pipeline::P1A_SITS_AFTER_STEP_4_AND_BEFORE_STEP_7`]),
    /// so it runs here, at the site of the reading it validates against, in
    /// the same way L14's confirmation rides this step.
    ///
    /// **The version is confirmed before the file is read, not after.** R2
    /// requires the reading to be taken **by the version that will render**, so
    /// a binary that names another version is refused before it is handed the
    /// artist's file at all.
    ///
    /// **What P1a costs when it is missing** is the silent one this whole
    /// module exists to stop: [`crate::line::LaunchLine::compose`] names an
    /// engine only where its input carries one, while the material's own
    /// settings pairs read the payload's engine to decide whose sample counts
    /// to impose — so the two halves of one attempt could disagree, the
    /// material imposing EEVEE's count and the line naming no engine, and
    /// Cycles rendering the file at whatever the artist saved. Exit 0, the
    /// marker passing, the ending *finished*, and a night of the wrong engine
    /// paid for with nothing anywhere saying so.
    fn step_four(
        &mut self,
        attempt: &TheAttempt<'_>,
        step: Step,
        remap: Option<&RemapMaterial>,
        settled: &mut WhatTheStepsSettled,
    ) -> Option<WhereTheAttemptEnded> {
        // **L14, and §9 item 16 is left no space**: Fact 105 is not
        // established, so the ask runs on every attempt, and a silence is
        // never read as a match.
        let answered = self
            .asks_the_version
            .what_version_it_names(attempt.the_blender_build_path);
        if let Err(refused) = confirm_the_version(attempt.the_manifests_version, answered) {
            return Some(failed(step, the_refusal(refused)));
        }

        // **`Job files and outputs` R1a and `Dependency Upload & Remap` M8b: the
        // folder the file opens from is made here, before the reading**, so the
        // reading sees what the render will see: a bake setting, a simulation or a
        // baked cache on a linked object is in neither the map nor the gate
        // otherwise (the film's first job). Made whether or not a reading is owed,
        // because the launch line opens it too. **A folder that cannot be made ends
        // the attempt**: the reading would misread the file, and the render would be
        // the finished wrong picture `Dependency Upload & Remap` M8b exists to stop.
        if let Some(remap) = remap {
            settled.the_folder_it_opens_from = smoozy_dependencies_machine::opening::the_folder_the_file_opens_from(
                &remap.pairs,
                attempt.manifest.blend_path_on_the_machine.as_bytes(),
                the_opening_folders_place(attempt).as_bytes(),
            );
            if let Some(folder) = &settled.the_folder_it_opens_from {
                if let Err(why) = self.steps.make_the_opening_folder(folder) {
                    return Some(WhereTheAttemptEnded::TheAttemptCouldNotBeComposed {
                        at: step,
                        what_happened: format!("the folder the file opens from could not be made: {why}"),
                    });
                }
            }
        }

        // **The reading is owed wherever the driver does not already hold both
        // halves of one.** Where step 3 kept the manifest's map, the map is not
        // recomputed from what this reading says (`Job files and outputs` R9)
        // — but the settings half is what P1a validates against and what
        // `Render Settings` Y9 composes the file's own range from, so a
        // manifest that carries no settings half still costs a read.
        if settled.reading.is_none() || settled.map_in_force.is_none() {
            if let Some(ended) = self.take_the_reading_and_compose(attempt, step, settled) {
                return Some(ended);
            }
        }
        // **`Job files and outputs` R7's first bullet: the render applies the
        // map in force.** Its redirect pairs are that map's own, read out by
        // the one function that turns a map into the material's assignments,
        // and the whole asked range is composed against the reading in force
        // (`Render Settings` Y9).
        if let Some(map) = settled.map_in_force.clone() {
            self.the_pairs_of(attempt, &map, settled);
        }
        if let Some(in_force) = settled.reading.clone() {
            settled.whole_range = the_whole_range(attempt, &in_force);
        }

        // **P1a, performed against the reading in force for this attempt.**
        // Every name-shaped entry of the payload — the scene, each pinned view
        // layer, the engine identifier — checked, and the first one the
        // reading does not carry fails the attempt before the render
        // instruction, naming it (`Render Settings` Y13, `Render Settings`
        // Y14).
        let in_force = settled
            .reading
            .as_ref()
            .expect("the reading is in hand above");
        match validate_the_names(attempt.payload, in_force) {
            Ok(names) => {
                settled.validated = Some(names);
                None
            }
            Err(refused) => Some(failed(
                step,
                BeforeTheRenderInstruction::ANameTheReadingInForceDoesNotCarry {
                    name: refused.name.as_text().to_string(),
                },
            )),
        }
    }

    /// **The reading taken, and everything derived from it composed by the one
    /// function both sides share** (`Job files and outputs` R7, R9).
    ///
    /// **The frames the job covers are handed in, because `Job files and
    /// outputs` R4 item 5 is evaluated at every one of them** — and where the
    /// manifest asks for the file's own range, the frames are not known until
    /// the file has been read.
    ///
    /// **The file is still loaded once, and that is the build runner's ruling of
    /// 2026-09-20 (`Audits/Build/Step 10c/The runner's rulings.md`).** The reading's own script covers the scene's whole span in
    /// its stepped tables anyway, and it now records a per-frame entry over
    /// that span **where no frames were handed in** — so one read answers both
    /// halves. **A job that hands frames in receives exactly what it did
    /// before.**
    ///
    /// **What it replaced**: this read with no frames, then a second read over
    /// the range the first one taught it — **a second load of the artist's
    /// file on every attempt of the commonest job there is**, on a machine the
    /// user pays for by the hour. Leaving the gate and dropping the second
    /// read alone was not available: the inventory would come back empty, the
    /// count zero, and `Job files and outputs` H1's *the job will write nothing
    /// at all* would hold every such job before it started.
    fn take_the_reading_and_compose(
        &mut self,
        attempt: &TheAttempt<'_>,
        step: Step,
        settled: &mut WhatTheStepsSettled,
    ) -> Option<WhereTheAttemptEnded> {
        let named = attempt.manifest.scene.name.as_bytes();
        let the_jobs_scene = (!named.is_empty()).then_some(named);
        let asked: Option<Vec<i64>> = match &attempt.manifest.frame_range {
            FrameRange::TheUsersParsedIntegers(frames) => Some(frames.clone()),
            FrameRange::TheFilesOwn => None,
        };
        // **`Job files and outputs` R1a: through the folder, where there is one.**
        // Outputs still resolve against the file's real folder, which is handed
        // separately below, and everything else in the reading goes by the file's
        // name, which the link keeps.
        let the_file_it_opens = settled
            .the_folder_it_opens_from
            .as_ref()
            .map(|folder| String::from_utf8_lossy(&folder.the_file).into_owned());
        let taking = TheReadingToTake {
            the_blender_build_path: attempt.the_blender_build_path,
            the_blend_path_on_the_machine: the_file_it_opens
                .as_deref()
                .unwrap_or(&attempt.manifest.blend_path_on_the_machine),
            the_version_it_will_render_by: the_version_in_hand(attempt.the_manifests_version),
            the_frames_the_job_covers: asked.as_deref().unwrap_or(&[]),
            the_jobs_scene,
            the_scripts_path: attempt.the_readings_script_path,
            // **`Job files and outputs` R4c**: the payload's own entry,
            // `Render Settings` Y1b, set on a first-frame check alone.
            the_check_is_written_as_a_picture: attempt.payload.the_check_is_written_as_a_picture,
            // **`Job files and outputs` P18a**: a check that writes into its
            // range job's folder has every name it writes marked.
            the_check_is_marked: attempt.payload.the_check_writes_into.is_some(),
        };
        let printed = match self.takes_the_reading.take(&taking) {
            Ok(printed) => printed,
            // **K14's failed limb, *the step-4 reading of the job's file did not
            // end successfully*** — this reading's own occasion (P1 step 4),
            // **and never `Job files and outputs` C9's**, whose words name the
            // output folder. Until 2026-09-29 this borrowed C9's, and a job
            // whose `.blend` could not be read was told as a folder that could
            // not be (step 10c's eighth pass on Hyperstack, fault 5). **What
            // C9 says of its own reading holds of this one too**: a reading
            // that did not end successfully is not a reading.
            Err(_) => {
                return Some(failed(
                    step,
                    BeforeTheRenderInstruction::TheJobsFileReadingDidNotEndSuccessfully,
                ))
            }
        };
        // **L10b's gate, read off this reading whether or not a map is
        // composed from it below** (`Job files and outputs` R4 item 8).
        settled.the_scene_holds_a_simulation_not_baked = printed.the_scene_holds_a_simulation_not_baked;
        // **L10c's gate, read off the same reading**: the material switches the
        // scene where, and only where, the reading did (`Job files and
        // outputs` R4c).
        settled.the_check_is_written_as_a_picture = printed.the_check_is_written_as_a_picture;
        let in_force = match self.a_recorded_reading(attempt, &printed, settled) {
            Some(in_force) => in_force,
            // **The same reading, its settings half unreadable** — K14's
            // step-4 occasion again (P1 step 4).
            None => {
                return Some(failed(
                    step,
                    BeforeTheRenderInstruction::TheJobsFileReadingDidNotEndSuccessfully,
                ))
            }
        };
        // **`Render Settings` Y9's second form, composed from this one
        // reading.** The script's per-frame table covers the scene's whole span
        // where no frames were handed in, so there is nothing left to go back
        // and ask for.
        let whole_range = the_whole_range(attempt, &in_force);

        settled.reading = Some(in_force);
        if settled.map_in_force.is_some() {
            // **`Job files and outputs` R9: nothing anywhere recomputes a map
            // that exists.** Step 3 kept the manifest's, so what this reading
            // was taken for is the settings half alone — P1a's names and
            // `Render Settings` Y9's range — and nothing derived from it is
            // composed, written or journalled.
            return None;
        }
        let in_force = settled.reading.as_ref().expect("set a line above");

        let Some(the_jobs_scene) = the_jobs_scene_in(in_force, attempt) else {
            return Some(failed(
                step,
                BeforeTheRenderInstruction::ANameTheReadingInForceDoesNotCarry {
                    name: attempt.manifest.scene.name.clone(),
                },
            ));
        };
        // **`Job files and outputs` R4 item 5: the job's pins apply before the
        // map, the inventory, the holds and the six things are composed**, and
        // before `Job files and outputs` B1's walk is entered — the
        // manifest's, since this reading runs at render start.
        let pins = the_jobs_pins(attempt.payload);
        let enters_at = where_the_render_enters(the_jobs_scene, &printed.outputs.per_frame, &pins);
        // **`Job files and outputs` P21's second case: the job output
        // folder chosen here, with the map, and made before step 5** — once
        // per job, and only where nothing chose it before. (A manifest that
        // carries a map names its folder, so the branch above never needs one.)
        if settled.job_output_folder.is_none() {
            match self.the_folder_is_chosen(attempt, &printed, &enters_at, &pins) {
                Ok(folder) => settled.job_output_folder = Some(folder),
                Err(what_happened) => {
                    return Some(failed(
                        step,
                        BeforeTheRenderInstruction::TheAttemptCouldNotBeComposed { what_happened },
                    ))
                }
            }
        }
        let composed = match the_reading_and_everything_derived_from(
            printed,
            WhatTheReadingIsComposedAgainst {
                the_folder_the_blend_sits_in: attempt.the_folder_the_blend_sits_in,
                job_output_folder: settled
                    .job_output_folder
                    .as_deref()
                    .expect("chosen above where it was not known"),
                enters_at: &enters_at,
                the_jobs_pins: &pins,
            },
        ) {
            Ok(composed) => composed,
            // **`Render Settings` Q7: a reading is the complete document with
            // an empty error map, and anything else is not a reading** — and it
            // is the step-4 reading of the job's file, so K14's occasion is that
            // reading's own (P1 step 4).
            Err(_) => {
                return Some(failed(
                    step,
                    BeforeTheRenderInstruction::TheJobsFileReadingDidNotEndSuccessfully,
                ))
            }
        };

        // **`State, identity and recovery` §1.2 and `Job files and outputs`
        // H9 together: written into the job record directory, and journalled
        // when it is written.**
        self.steps.write_the_recorded_reading(&composed);

        settled.side = Some(WhichSideComposedTheMap::TheRenderWorkerAtRenderStart);
        settled.the_baked_caches_on_the_machine =
            composed.reading.the_baked_caches_on_the_machine.clone();
        settled.inventory = Some(composed.inventory.clone());
        settled.whole_range = whole_range;
        settled.map_in_force = Some(composed.map.clone());
        None
    }

    /// **`Job files and outputs` P18 to `Job files and outputs` P23, at render
    /// start**: the run
    /// worked out over the destinations this reading will write, the place
    /// chosen and the folder made by that document's one choice
    /// ([`the_folder_is_chosen_and_made`]), and the choice written into the job
    /// record directory before anything else reads it. **`Job files and
    /// outputs` N8's budget is asked
    /// of each place by composing the map there** with the one composition
    /// both sides share, so a place is passed over exactly where the map would
    /// hold the job for a path that will not fit.
    ///
    /// **Why the attempt fails where it does.** The job area refusing a fresh
    /// folder is `Job files and outputs` N10's fault, and a choice nothing
    /// records would be a render nobody can find; both are said in the
    /// machine's own words and nothing is launched.
    fn the_folder_is_chosen(
        &mut self,
        attempt: &TheAttempt<'_>,
        printed: &smoozy_job_files_machine::reading_script::ThePrintedReading,
        enters_at: &WhereTheRenderEnters,
        pins: &[smoozy_job_files_machine::reading::AViewLayerTheJobPins],
    ) -> Result<String, String> {
        let TheJobOutputFolderForThisAttempt::ToBeChosen {
            layout,
            the_folders_name,
            the_servers_fallback_folder,
        } = attempt.the_job_output_folder
        else {
            unreachable!("a known folder is settled before step 3")
        };
        let destinations =
            smoozy_job_files_machine::reading_script::the_destinations_that_will_be_written(&printed.outputs);
        let run = the_run_of(&destinations, attempt.the_folder_the_blend_sits_in, printed.outputs.taken_by);
        let mut fits = |candidate: &str| {
            the_reading_and_everything_derived_from(
                printed.clone(),
                WhatTheReadingIsComposedAgainst {
                    the_folder_the_blend_sits_in: attempt.the_folder_the_blend_sits_in,
                    job_output_folder: candidate,
                    enters_at,
                    the_jobs_pins: pins,
                },
            )
            .map(|composed| {
                !composed.map.pairs.iter().any(|pair| {
                    matches!(
                        pair.outcome,
                        HowItMapped::NotMappable(WhyItIsNotMappable::ItsPredictedDeepestFilePathWillNotFit(_))
                    )
                })
            })
            .unwrap_or(true)
        };
        let sits = the_folder_is_chosen_and_made(
            &WhatTheChoiceIsMadeFrom {
                the_run: &run,
                layout,
                the_job_output_folders_name: the_folders_name,
                the_servers_fallback_folder,
            },
            &mut fits,
            self.steps.the_machines_folders(),
        )
        .map_err(|_| {
            format!(
                "the job output folder could not be made in Smoozy's own job area at {}",
                layout.area(smoozy_job_files_machine::layout::Area::Job)
            )
        })?;
        self.steps
            .write_where_the_folder_sits(&sits)
            .map_err(|why| format!("where the job output folder sits could not be written down: {why}"))?;
        Ok(sits.path)
    }

    /// **`Job files and outputs` D9, composed from the map and from nothing
    /// else**: the redirect pairs
    /// and the regular output path's own value, read out of the map in force by
    /// the one function that does it.
    fn the_pairs_of(
        &self,
        attempt: &TheAttempt<'_>,
        map: &smoozy_job_files_machine::mapping::TheMap,
        settled: &mut WhatTheStepsSettled,
    ) {
        let material = smoozy_job_files_machine::redirect::compose_the_launch_material(
            map,
            the_version_in_hand(attempt.the_manifests_version),
            &attempt.manifest.job,
            attempt.manifest.remap_material.clone(),
        );
        settled.redirect_pairs = material.assignments;
        settled.regular_output_path = material
            .the_regular_output_path
            .map(|argument| argument.value)
            .unwrap_or_default();
    }

    /// **The settings half, as the attempt holds it** — `Render Settings`
    /// Q11's shape with the Blender that took it, the four values read at step
    /// 3 and the moment on the machine's own clock.
    ///
    /// **It is a carrier and not a stored reading.** `Render Settings` Q18's
    /// moment is
    /// documented as resting on nothing, and nothing on the machine reads it;
    /// what this shape is here for is P1a's names and `Render Settings` Y9's
    /// range. **The clock
    /// is the machine's own** (`State, identity and recovery` D5), never this
    /// computer's, so two machines' moments can never be subtracted.
    fn a_recorded_reading(
        &self,
        attempt: &TheAttempt<'_>,
        printed: &smoozy_job_files_machine::reading_script::ThePrintedReading,
        settled: &WhatTheStepsSettled,
    ) -> Option<RecordedReading> {
        let settings: SettingsHalf = serde_json::from_value(printed.settings.0.clone()).ok()?;
        Some(RecordedReading {
            settings,
            taken_by: BlenderIdentity {
                path: attempt.the_blender_build_path.to_string(),
                version: attempt.the_manifests_version,
            },
            // **M8's *the file identity read at that moment***. Where the four
            // values could not be read at all the manifest's own is the only
            // value there is, and nothing here invents one.
            file_identity: settled
                .the_identity_read_at_step_three
                .clone()
                .unwrap_or_else(|| attempt.manifest.file_identity_at_submission.clone()),
            stored_at: Reading::taken(
                (),
                std::time::SystemTime::now(),
                attempt.the_machines_clock.clone(),
            ),
        })
    }

    /// **Step 5: the three hold conditions, evaluated off the recorded
    /// reading, under `Job files and outputs` H5's two triggers.**
    ///
    /// **`Job files and outputs` H2: all three are read off the recorded
    /// reading and none needs a second evaluation**, so the judgement is that
    /// document's own function and what is here is the answer's consequence.
    /// **The difference between its two triggers is the spec's**: the
    /// cannot-bring-under condition holds the job whichever side computed the
    /// map; the writes-nothing and colliding-pair conditions hold it **only
    /// where the map was computed on the machine**, because a user who
    /// pre-loaded and submitted has already answered those two (`Job files and
    /// outputs` H6).
    fn step_five(
        &mut self,
        attempt: &TheAttempt<'_>,
        settled: &WhatTheStepsSettled,
    ) -> Option<WhereTheAttemptEnded> {
        let (Some(map), Some(inventory), Some(side)) = (
            settled.map_in_force.as_ref(),
            settled.inventory.as_ref(),
            settled.side,
        ) else {
            // Step 4 ends the attempt where no reading stood, so there is no
            // branch here that judges a hold against nothing.
            return None;
        };
        let holds = smoozy_job_files_machine::holds::what_holds_this_job(map, inventory, side);
        if holds.is_empty() {
            return None;
        }
        // **`Rented-server runtime` U2: the hold's reason, from the closed set
        // the placing document declares** — `Job files and outputs` H1's own
        // spelling of each condition, which the desktop reads back by the same
        // table. *(Until 2026-10-03, the 38's piece 10, this was Rust's debug
        // print of the list, which no reader could name, so a held entry
        // offered only *Cancel*.)*
        let reason = smoozy_job_files_machine::holds::the_hold_reason_for(&holds);
        let _ = attempt;
        Some(
            match self.steps.ask_the_supervisor_to_place_the_hold(&reason) {
                Some(hold) => WhereTheAttemptEnded::AHoldWasPlaced { hold },
                // **P3**, and no branch here composes an identity.
                None => WhereTheAttemptEnded::NoHoldWasPlacedAndReconciliationTakesIt,
            },
        )
    }

    /// **Step 6: the reckoning, against the inventory for the map in force**
    /// (C7) and over the whole asked range step 4 composed (C20).
    fn step_six(
        &mut self,
        attempt: &TheAttempt<'_>,
        step: Step,
        settled: &mut WhatTheStepsSettled,
    ) -> Option<WhereTheAttemptEnded> {
        let Some(folder) = settled.job_output_folder.clone() else {
            // **Step 4 always stands one before step 6 runs**, so this is a
            // walk of nothing, which C9 makes a failed attempt.
            return Some(WhereTheAttemptEnded::TheWalkDidNotComplete { at: step });
        };
        let walk = self.steps.walk_the_job_output_folder(&folder);
        let inventory = settled.inventory.clone().unwrap_or(
            WhatTheInventoryIs::TheFrameSetIsNotPredictable(
                smoozy_job_files_machine::inventory::WhyTheFrameSetIsNotPredictable::TheRegularOutputPathsNameWasNotAnswered,
            ),
        );
        match smoozy_job_files_machine::reckoning::the_reckoning(
            &settled.whole_range,
            &inventory,
            attempt.previous_attempts,
            &walk,
            &folder,
        ) {
            said @ WhatTheReckoningSays::RenderTheseFrames(_) => {
                settled.a_frame_an_earlier_attempt_wrote_was_kept =
                    any_frame_an_earlier_attempt_wrote_was_kept(
                        &settled.whole_range,
                        &inventory,
                        &said,
                    );
                let WhatTheReckoningSays::RenderTheseFrames(frames) = said else {
                    unreachable!("matched one variant a line above")
                };
                settled.list = frames;
                None
            }
            WhatTheReckoningSays::TheWalkDidNotCompleteSoThisIsAFailedAttempt => {
                Some(WhereTheAttemptEnded::TheWalkDidNotComplete { at: step })
            }
        }
    }

    /// **`Dependency Upload & Remap` M22c, performed inside step 7: each copy
    /// the remap material owes, made into the folder the map in force gives its
    /// write place, and the placed-copy list written (`Dependency Upload &
    /// Remap` M22f).**
    ///
    /// **And, since 2026-10-02, each baked physics cache the reading found on
    /// this machine** (`Job files and outputs` D21b-i): copied the same way to
    /// the point cache's folder for its recorded location, and recorded in the
    /// same list, so `Download` N5a skips it and the user does not download
    /// their own bake back. **A folder a sent copy already goes to is not
    /// copied into again** — the sent bake is the one the user chose.
    ///
    /// **A copy that cannot be made ends the attempt before the render and
    /// names the copy** — and so does one whose write place the map in force
    /// gives no folder, the file having changed since the check or the place
    /// found not mappable. **Rendering without it is the one thing this must
    /// not do**: the render would compute afresh or read nothing, finish, and
    /// nothing anywhere would say the bake the user sent was never read.
    fn the_copies_owed(
        &mut self,
        step: Step,
        remap: &RemapMaterial,
        settled: &WhatTheStepsSettled,
    ) -> Option<WhereTheAttemptEnded> {
        let found = smoozy_dependencies_machine::placed_copies::TheCopyOfABakeOnTheMachine::the_copies_of(
            &settled.the_baked_caches_on_the_machine,
        );
        if remap.copies_owed.is_empty() && found.is_empty() {
            return None;
        }
        let mut placed = ThePlacedCopies::default();
        let mut sent_to: Vec<Vec<u8>> = Vec::new();
        // **No place in the map: the reason names the copy and nothing further**
        // (`Rendering — machine`'s `endings` says why).
        let no_folder = |copy: String| {
            failed(
                step,
                BeforeTheRenderInstruction::AnUploadedBakeCouldNotBeCopied { copy, why: String::new() },
            )
        };
        for copy in &remap.copies_owed {
            let folders = settled
                .map_in_force
                .as_ref()
                .map(|map| copy.where_it_goes(map))
                .unwrap_or_default();
            if folders.is_empty() {
                return Some(no_folder(copy.in_words()));
            }
            for to in folders {
                match self.steps.copy_into_the_job_folder(copy.from.as_bytes(), &to) {
                    Ok(files) => placed.files.extend(files),
                    Err(why) => {
                        return Some(failed(
                            step,
                            BeforeTheRenderInstruction::AnUploadedBakeCouldNotBeCopied {
                                copy: copy.in_words(),
                                why,
                            },
                        ))
                    }
                }
                sent_to.push(to);
            }
        }
        for copy in &found {
            let folders = settled
                .map_in_force
                .as_ref()
                .map(|map| copy.where_it_goes(map))
                .unwrap_or_default();
            if folders.is_empty() {
                return Some(no_folder(copy.in_words()));
            }
            for to in folders {
                if sent_to.contains(&to) {
                    continue;
                }
                match self.steps.copy_into_the_job_folder(copy.from.as_bytes(), &to) {
                    Ok(files) => placed.files.extend(files),
                    Err(why) => {
                        return Some(failed(
                            step,
                            BeforeTheRenderInstruction::AnUploadedBakeCouldNotBeCopied {
                                copy: copy.in_words(),
                                why,
                            },
                        ))
                    }
                }
            }
        }
        self.steps.write_the_placed_copies(&placed);
        None
    }

    /// **Step 7: composition, the material placed and read back, the attempt
    /// record written durably, then the launch — in that order.**
    fn step_seven(
        &mut self,
        attempt: &TheAttempt<'_>,
        step: Step,
        remap: Option<&RemapMaterial>,
        settled: &WhatTheStepsSettled,
    ) -> WhereTheAttemptEnded {
        let (Some(remap), Some(reading), Some(validated)) =
            (remap, settled.reading.as_ref(), settled.validated.as_ref())
        else {
            // None of the three can be absent here — step 2 ends the attempt
            // where the material is, and step 4 ends it where the reading or
            // P1a's validation is — and saying so is cheaper than a branch
            // that pretends one could be empty. **Were one absent, it is step
            // 4's reading that stands missing**, so the occasion is that
            // reading's (P1 step 4) and not the output folder's.
            return failed(
                step,
                BeforeTheRenderInstruction::TheJobsFileReadingDidNotEndSuccessfully,
            );
        };
        let Some(the_jobs_scene) = the_jobs_scene_in(reading, attempt) else {
            // **`Render Settings` Y13 and `Render Settings` Y14, and `Rendering`
            // P1a**: a name-shaped payload entry the reading in force does not
            // carry fails the attempt before the render instruction, naming
            // what was not found.
            return failed(
                step,
                BeforeTheRenderInstruction::ANameTheReadingInForceDoesNotCarry {
                    name: attempt.manifest.scene.name.clone(),
                },
            );
        };

        let composed = match compose_the_attempt(AttemptInputs {
            job: attempt.job,
            manifest: attempt.manifest,
            payload: attempt.payload,
            remap,
            // **The map in force's own pairs** (P1 steps 3 and 4), never the
            // manifest's where step 3 refused it.
            redirect_pairs: &settled.redirect_pairs,
            regular_output_path: &settled.regular_output_path,
            reading_in_force: reading,
            the_jobs_scene,
            // **P1a's own answer, and never a hand-in.** The identifier is
            // the chosen version's own, anchored against the reading in
            // force at step 4, so the line and the material cannot name
            // different engines.
            anchored_engine: validated
                .engine
                .as_ref()
                .map(|engine| engine.as_text().to_string()),
            the_reckonings_list: &settled.list,
            launch_material_path: attempt.the_launch_materials_path.to_string(),
            load_time_script_path: attempt.the_load_time_scripts_path.to_string(),
            the_scene_holds_a_simulation_not_baked: settled.the_scene_holds_a_simulation_not_baked,
            the_check_is_written_as_a_picture: settled.the_check_is_written_as_a_picture,
            // **L17a: the video shape is handed to the line**, read off the
            // map in force and its inventory, so a job with a video-shaped
            // destination is rendered as one run and never as a frame list.
            a_destination_is_video_shaped: a_destination_is_video_shaped(settled),
        }) {
            Ok(composed) => composed,
            Err(why) => {
                // **L23's own occasion is on K14's list and takes it**; the
                // rest are the arm above.
                // **What is recorded is a sentence and never a type's name.**
                // Step 10c's fifth pass on Vast, fault 5: these two lines put
                // `NoReadingStoodAtSubmission` and its enclosing type into the
                // attempt record on a real rented machine.
                return match why {
                    crate::composition::CannotComposeTheAttempt::NoTechnologyToName(nothing) => {
                        failed(
                            step,
                            BeforeTheRenderInstruction::ACardMakeWithNoTechnologyToName {
                                make: nothing.in_words(),
                            },
                        )
                    }
                    other => WhereTheAttemptEnded::TheAttemptCouldNotBeComposed {
                        at: step,
                        what_happened: other.in_words(),
                    },
                };
            }
        };

        // **L24b: where the job names `OPTIX`, the loader is asked for the
        // OptiX library before anything is launched** — here, at the step
        // where L23's lookup ends a make with no row, and before the copies,
        // the scripts and the attempt record, so a machine that cannot render
        // the job costs nothing past this line. **Only a Cycles job names it
        // (L21's bullet of 2026-11-02), so a job on another engine never gets
        // here.**
        //
        // **Only *not found* ends the attempt.** Found, nothing changes. **A
        // lookup that did not answer is not read as either**: it knows nothing
        // about the library, so the launch goes ahead and L24's own refusal
        // stands for whatever is wrong, recorded verbatim as before — the
        // same place a found library leaves everything the lookup cannot see.
        if composed.technology_named == Some(Technology::Optix) {
            match self.looks_for_the_optix_library.what_the_loader_finds() {
                WhatTheLoaderFound::NoLibrary => {
                    return failed(step, BeforeTheRenderInstruction::TheOptixLibraryWasNotFound)
                }
                WhatTheLoaderFound::TheLibrary | WhatTheLoaderFound::NoAnswer { .. } => {}
            }
        }

        // **`Dependency Upload & Remap` M22c: every uploaded bake the material
        // says is owed, copied into the job folder before anything is
        // launched** — after the composition, which can refuse the attempt for
        // nothing a copy costs, and before the launch material.
        if let Some(ended) = self.the_copies_owed(step, remap, settled) {
            return ended;
        }

        // **`Dependency Upload & Remap` M8b: where a library is recorded relative
        // to the file, the file opens from a folder where it resolves**, because
        // the file's own start scripts run as it loads and before anything of ours.
        // **The folder is made at step 4** (`Job files and outputs` R1a), and the line
        // opens the link the reading opened.
        let composed = match &settled.the_folder_it_opens_from {
            None => composed,
            Some(folder) => {
                let line = composed.line.opening_the_file_at(
                    String::from_utf8_lossy(&folder.the_file).into_owned(),
                );
                crate::composition::TheComposedAttempt { line, ..composed }
            }
        };

        // **L8b's script before the file, placed and read back the same way
        // (L12)** — its handler sets the places `Job files and outputs` D1a
        // sets as the file loads, and the material's first check reads that
        // it ran.
        let load_time = crate::acting::load_time::the_load_time_scripts_text(&composed.load_time);
        match place_and_confirm(self.places, attempt.the_load_time_scripts_path, &load_time) {
            WhetherThePlacementIsConfirmed::ItIs => {}
            what_happened => {
                return WhereTheAttemptEnded::TheMaterialsPlacementWasNotConfirmed {
                    at: step,
                    what_happened,
                }
            }
        }

        // **L12: placed and read back under the login the render runs as,
        // before anything is launched.**
        let text = the_materials_text(
            &composed.material,
            attempt.the_manifests_version,
            the_jobs_scene,
            remap,
        );
        match place_and_confirm(self.places, attempt.the_launch_materials_path, &text) {
            WhetherThePlacementIsConfirmed::ItIs => {}
            what_happened => {
                return WhereTheAttemptEnded::TheMaterialsPlacementWasNotConfirmed {
                    at: step,
                    what_happened,
                }
            }
        }

        // **The attempt record, durably, before the render starts** — carrying
        // **the map in force for this attempt** and **the file identity read at
        // step 3** (`Job files and outputs` M8), and never the manifest's map
        // or its identity at submission.
        let record = AttemptRecord {
            number: attempt.number,
            frame_list_in_the_order_it_was_written: settled.list.clone(),
            map_in_force: match settled.map_in_force.clone() {
                Some(map) => TheMapInForce::It(map),
                None => TheMapInForce::NoneWasComposedForThisAttempt,
            },
            file_identity_at_this_attempt: settled
                .the_identity_read_at_step_three
                .clone()
                .unwrap_or_else(|| attempt.manifest.file_identity_at_submission.clone()),
            the_file_changed_before_this_attempt: settled.the_file_changed,
            a_frame_an_earlier_attempt_wrote_was_kept: settled
                .a_frame_an_earlier_attempt_wrote_was_kept,
        };
        if let Err(why) = self.steps.write_the_attempt_record(&record) {
            return WhereTheAttemptEnded::TheAttemptRecordCouldNotBeWritten { at: step, why };
        }

        let launch = match TheLaunch::from(attempt.the_blender_build_path, &composed.line) {
            Ok(launch) => launch,
            Err(missing) => {
                return WhereTheAttemptEnded::TheAttemptCouldNotBeComposed {
                    at: step,
                    what_happened: format!("a spelling is not on record: {}", missing.what),
                }
            }
        };
        let mut started = match self.start.start(&launch) {
            Ok(started) => started,
            Err(why) => return WhereTheAttemptEnded::TheLaunchCouldNotBeMade { at: step, why },
        };
        // **L5**, and a render that cannot be recorded is ended here.
        if let Err(why) = self.steps.record_the_render_process(started.recorded) {
            let _ = started.child.kill();
            let _ = started.child.wait();
            return WhereTheAttemptEnded::TheRecordedPairCouldNotBeWritten { at: step, why };
        }
        WhereTheAttemptEnded::TheRenderWasLaunched(started)
    }
}

/// **`Dependency Upload & Remap` M8b: where an attempt's opening folder goes** —
/// beside this attempt's scripts in the job's record directory, so it goes when the
/// job's records go, under this attempt's own number.
fn the_opening_folders_place(attempt: &TheAttempt<'_>) -> String {
    let scripts = std::path::Path::new(attempt.the_load_time_scripts_path);
    let folder = scripts.parent().unwrap_or_else(|| std::path::Path::new(""));
    folder
        .join(format!("opening-{}", attempt.number))
        .display()
        .to_string()
}

/// **`Rendering` L17a, `Job files and outputs` I19: whether any destination
/// of this attempt is video-shaped** — read off the map in force, whose pairs
/// carry every destination the reading found (`Job files and outputs` P10),
/// and off the inventory
/// composed from it, which is what `Job files and outputs` C6's reckoning
/// reads. Either saying so is enough: the map still says so where the
/// inventory could not be composed, and the reckoning has then handed over
/// the whole range anyway. Added 2026-10-17, the Creator's eleventh notes,
/// the video piece.
fn a_destination_is_video_shaped(settled: &WhatTheStepsSettled) -> bool {
    let in_the_map = settled.map_in_force.as_ref().is_some_and(|map| {
        map.pairs.iter().any(|pair| {
            pair.destination
                .format
                .as_ref()
                .is_some_and(|format| format.is_video_shaped)
        })
    });
    let in_the_inventory = matches!(
        &settled.inventory,
        Some(WhatTheInventoryIs::It(inventory)) if inventory.a_destination_is_video_shaped
    );
    in_the_map || in_the_inventory
}

/// **`Render Settings` Y9: the manifest's frame-range field takes two forms**
/// — the user's parsed integers, or the explicit statement *the file's own*,
/// **the second composed against the reading in force at the attempt**.
///
/// **The step counts forward from the first frame** and a step below one has no
/// count under that rule, so it is floored at one rather than being given an
/// arithmetic of this file's own.
fn the_whole_range(attempt: &TheAttempt<'_>, in_force: &RecordedReading) -> Vec<i64> {
    the_whole_range_of(attempt.manifest, &in_force.settings)
}

/// **The same derivation over a manifest and the settings half of the reading
/// in force, and it is the one function that makes it.** `Rented-server
/// runtime` S11's walk at reconciliation takes it too, over the attempt's own
/// recorded reading, so a job a restart ends names its frames by the rule its
/// worker would have used. Made public 2026-10-01, step 10c's Sesterce-pass
/// fix piece 2; until then that walk had no frames for a job with the file's
/// own range.
pub fn the_whole_range_of(
    manifest: &smoozy_job_files_machine::manifest::SubmissionManifest,
    settings: &SettingsHalf,
) -> Vec<i64> {
    match &manifest.frame_range {
        FrameRange::TheUsersParsedIntegers(frames) => frames.clone(),
        FrameRange::TheFilesOwn => match the_manifests_scene_in(settings, manifest) {
            Some(scene) => FrameForm::Triple {
                first: scene.frame_start,
                last: scene.frame_end,
                step: scene.frame_step.max(1),
            }
            .explicit_list(),
            // **K14's *no frame to name* is what this reaches**, loud and total
            // at step 7, and nothing here composes a range out of a scene the
            // reading does not carry.
            None => Vec::new(),
        },
    }
}

/// **The tile size `Queue & History` B2's grid is taken over, where the reading
/// carries none — and it carries none.** `Queue & History` B2 counts tile-sized
/// steps over the
/// file's own tile size, and the reading the render takes at step 4 does not
/// read it, so the plan assumes the size `Queue & History` B2 records as the
/// default: *"2048×2048 at the default 2048 giving one"*. Label:
/// **assumption**. Added 2026-10-03, the Creator's notes on 0.1.8, piece 6a.
///
/// **What it costs, and what it does not.** A file with another tile size makes
/// the plan total wrong by that frame's tile factor — a plan figure, which
/// `Queue & History` B5 already presents as a plan and never a promise. **Every
/// frame of a job has the one output size, so the factor is the same on every
/// renderlet**, and the second bar's fraction of the plan does not move with
/// it. No reading turns on it (`Queue & History` R12: a frame is tiled where
/// the line says so).
pub const THE_TILE_SIZE_THE_PLAN_ASSUMES: (u32, u32) = (2048, 2048);

/// **`Queue & History` B1, performed by the render worker**: the plan for the
/// job — the renderlets its total and `Queue & History` B6's finished part are
/// summed from — from the reading in force — every frame of the whole asked range,
/// each view layer that renders at it, each at the count in force for that
/// layer, each of `Queue & History` B2's tiles. Added 2026-10-03, the
/// Creator's notes on 0.1.8, piece 6a.
///
/// **Everything it takes is something else's, and it decides nothing of its
/// own.** The renderlets and the sum are `Queue & History`'s
/// ([`smoozy_queue_history_machine::plan`]); the count in force per layer is
/// `Render Settings` C6's ([`the_count_in_force_for`]), the typed count where
/// the payload carries one and the file's own rule where it does not; the
/// engine is the payload's pick or the file's own (`Render Settings` E4); the
/// size is the typed percentage's (`Render Settings` Q2a) or the file's own.
/// **A pinned layer renders at every frame or at none**, as its pin says
/// (`Render Settings` V4), so the pin stands in for the file's own switch —
/// asked of `Job files and outputs` R4 item 5's one rule
/// ([`what_the_jobs_pin_says`]), the same rule the count and the folders are
/// composed by.
///
/// **`None` where no plan can be worked out**: no reading stood, the job's
/// scene is not in it, or the engine in force is neither of the two offered —
/// which counts no samples Smoozy can read.
fn the_plan_of(
    attempt: &TheAttempt<'_>,
    settled: &WhatTheStepsSettled,
) -> Option<Vec<smoozy_queue_history_machine::plan::Renderlet>> {
    use smoozy_render_settings_machine::engine::{what_the_page_shows, WhatThePageShows};
    use smoozy_render_settings_machine::reading::{the_size_at, SwitchAnimation};
    use smoozy_render_settings_machine::samples::the_count_in_force_for;
    let reading = settled.reading.as_ref()?;
    let scene = the_jobs_scene_in(reading, attempt)?;
    let engine = match &attempt.payload.engine {
        Some(entry) => entry.pick,
        None => match what_the_page_shows(scene, reading) {
            WhatThePageShows::Offered(engine) => engine,
            WhatThePageShows::TheFilesOwn(_) => return None,
        },
    };
    let output_size = match attempt.payload.resolution_percentage {
        Some(percentage) => the_size_at(scene.full_size, percentage)?,
        None => scene.size_at_the_files_percentage,
    };
    let mut scene = scene.clone();
    let pins = the_jobs_pins(attempt.payload);
    for layer in scene.view_layers.iter_mut() {
        if let Some(on) = what_the_jobs_pin_says(&pins, layer.name.as_bytes()) {
            layer.renders_at = Vec::new();
            layer.animation = SwitchAnimation::None { value: on };
        }
    }
    let renderlets = smoozy_queue_history_machine::plan::the_renderlets(
        &scene,
        &settled.whole_range,
        output_size,
        THE_TILE_SIZE_THE_PLAN_ASSUMES,
        |layer| the_count_in_force_for(attempt.payload.sample_count, engine, &scene, layer),
    );
    Some(renderlets)
}

/// **The job's pins, in the shape `Job files and outputs` R4 item 5 takes
/// them** — `Render Settings` V4's entries, as bytes.
pub fn the_jobs_pins(payload: &Payload) -> Vec<AViewLayerTheJobPins> {
    payload
        .view_layer_pins
        .iter()
        .map(|pin| AViewLayerTheJobPins {
            layer: ArtistsText::of(pin.layer.as_bytes().to_vec()),
            on: pin.on,
        })
        .collect()
}

/// **`Job files and outputs` B1: where the beauty walk enters** — the job's
/// scene, and the view layer the beauty is followed from.
///
/// **The layer is the first of the scene's, in its own order, that renders at
/// the job's first frame with the job's pins applied**
/// (`Job files and outputs` B1 as corrected 2026-10-04,
/// [`the_layer_the_walk_enters_at`]) — never a layer the job
/// switched off. Until then it was the job's first pinned layer whatever its
/// pin, and the Creator's job, whose first pin was *Box AO* pinned off,
/// entered there. **The scene's own first layer where none renders there**,
/// which is where a render with no pin enters.
fn where_the_render_enters(
    scene: &SceneReading,
    per_frame: &[smoozy_job_files_machine::reading::AtOneFrame],
    pins: &[AViewLayerTheJobPins],
) -> WhereTheRenderEnters {
    let layer = the_layer_the_walk_enters_at(
        scene.view_layers.iter().map(|layer| layer.name.as_bytes()),
        per_frame,
        pins,
    )
    .map(|layer| layer.as_bytes().to_vec())
    .or_else(|| {
        scene
            .view_layers
            .first()
            .map(|layer| layer.name.as_bytes().to_vec())
    })
    .unwrap_or_default();
    WhereTheRenderEnters {
        scene: ArtistsText::of(scene.name.as_bytes().to_vec()),
        view_layer: ArtistsText::of(layer),
    }
}

/// `Server Setup` B2's components, as `Job files and outputs` reads a version.
fn the_version_in_hand(version: Version) -> TheVersionInHand {
    TheVersionInHand::of(version.major as u16, version.minor as u16)
}

/// **`Render Settings` C8's scene: the manifest's named scene, or the file's
/// showing scene where it names none** — found in the reading in force by the
/// name's own bytes and never by a normalised comparison (L15).
fn the_jobs_scene_in<'a>(
    reading: &'a RecordedReading,
    attempt: &TheAttempt<'_>,
) -> Option<&'a SceneReading> {
    the_manifests_scene_in(&reading.settings, attempt.manifest)
}

/// The same, over a settings half and the manifest that names the scene.
///
/// **Public since 2026-10-03, the 38's piece 6**: the machine's program reads
/// a finished job's own scene through it for `Server Setup` W4's second
/// condition, so which scene a job renders has this one home.
pub fn the_manifests_scene_in<'a>(
    settings: &'a SettingsHalf,
    manifest: &smoozy_job_files_machine::manifest::SubmissionManifest,
) -> Option<&'a SceneReading> {
    let named = manifest.scene.name.as_bytes();
    if named.is_empty() {
        // **L15's unnamed branch**: the job names no scene and the render
        // takes the file's showing scene. The reading lists the file's scenes
        // in the file's own order and the first is what a reading with no
        // named scene has to work with; **nothing here invents a name for the
        // launch line**, which composes no scene argument in that branch.
        //
        // **L16 is what bounds this branch, and it is the fallback L16
        // names.** What a file saved from Blender's interface records as its
        // showing scene is untested — every file the research tried was saved
        // by a Blender with no window — so **nothing is built on the untested
        // half**: no name is composed here, none is written to the launch
        // line, and nothing claims which scene the render will take.
        return settings.scenes.first();
    }
    settings
        .scenes
        .iter()
        .find(|scene| scene.name.as_bytes() == named)
}

fn failed(at: Step, reason: BeforeTheRenderInstruction) -> WhereTheAttemptEnded {
    WhereTheAttemptEnded::FailedBeforeTheRenderInstruction { at, reason }
}

fn the_refusal(refused: VersionRefused) -> BeforeTheRenderInstruction {
    // **The reason is recorded verbatim and no number names a cause** (K6):
    // what is written is what the confirmation found, in its own words.
    BeforeTheRenderInstruction::TheVersionConfirmationRefused {
        reason: format!("{refused:?}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::acting::launch::tests::AKernelThat;
    use crate::acting::launch::TheRenderStartedOnTheMachine;
    use crate::composition::tests::a_manifest;
    use crate::material::tests::{a_job, a_remap, four_values};
    use smoozy_job_files_machine::inventory::WhyTheFrameSetIsNotPredictable;
    use smoozy_job_files_machine::mapping::TheMap;
    use smoozy_job_files_machine::reading::{
        AtOneFrame, Destination, DestinationKind, Format, GatingConditions,
        HowManyViewsTheSceneWrites, WhereItComesFrom, WhichScene,
    };
    use smoozy_job_files_machine::reading_script::{TheObservedOutputs, ThePrintedReading};
    use smoozy_job_files_machine::reckoning::AWalkEntry;

    const BLEND_SITS_IN: &[u8] = b"/ephemeral/smoozy/assets";

    fn text(of: &str) -> ArtistsText {
        ArtistsText::of(of.as_bytes().to_vec())
    }

    fn the_machines_clock() -> Clock {
        Clock::AMachine(smoozy_state::observed::MachineClock("box-1".to_string()))
    }

    fn plain_gating() -> GatingConditions {
        GatingConditions {
            the_effects_on_for_render: None,
            compositing_switch: true,
            a_compositing_tree_is_present: true,
            a_live_final_output_node: true,
            the_final_output_node_is_muted: false,
            four_fives_use_nodes_switch: None,
            save_output_switch: None,
            the_render_goes_out_through_a_sequencer_holding_a_scene_strip: false,
        }
    }

    fn a_png() -> Format {
        Format {
            as_the_file_records_it: text("PNG"),
            the_ending_it_writes: Some(text(".png")),
            is_video_shaped: false,
        }
    }

    fn a_node_destination(node: &str, output: &str, at: &str) -> Destination {
        Destination {
            kind: DestinationKind::AnOutputOfAFileOutputNode {
                the_effect: None,
                node: text(node),
                output: text(output),
                the_nodes_base_file_name: None,
                replaced: None,
            },
            recorded_location: Some(
                smoozy_job_files_machine::reading::RecordedLocation::of(at.as_bytes().to_vec()),
            ),
            format: Some(a_png()),
            origin: WhereItComesFrom::AFileOutputNodeNamed(text(node)),
            scene: WhichScene::TheJobsOwnScene,
            the_view_layers_that_feed_it: vec![text("ViewLayer")],
            the_location_with_its_folder_replaced: None,
        }
    }

    fn the_regular_output_path() -> Destination {
        Destination {
            kind: DestinationKind::TheRegularOutputPath,
            recorded_location: Some(
                smoozy_job_files_machine::reading::RecordedLocation::of(b"//out/f_".to_vec()),
            ),
            format: Some(a_png()),
            origin: WhereItComesFrom::TheScenesOutputPath,
            scene: WhichScene::TheJobsOwnScene,
            the_view_layers_that_feed_it: vec![text("ViewLayer")],
            the_location_with_its_folder_replaced: None,
        }
    }

    fn a_frame(frame: i64, renders: bool) -> AtOneFrame {
        AtOneFrame {
            frame,
            view_layers_that_render: if renders {
                vec![text("ViewLayer")]
            } else {
                Vec::new()
            },
            gating: plain_gating(),
            the_regular_output_paths_answered_path: Some(
                smoozy_job_files_machine::reading::RecordedLocation::of(
                    format!("/home/artist/out/f_{frame:04}.png").into_bytes(),
                ),
            ),
        }
    }

    /// **What a reading of the job's file prints**, with the settings half the
    /// rest of this crate's tests already use so the two cannot drift.
    fn a_printed_reading(destinations: Vec<Destination>, frames: &[(i64, bool)]) -> ThePrintedReading {
        ThePrintedReading {
            outputs: TheObservedOutputs {
                taken_by: TheVersionInHand::of(5, 2),
                gating: plain_gating(),
                destinations,
                scenes_reached_through_a_sequencer_strip: Vec::new(),
                the_tree_is_shared_with: None,
                per_frame: frames
                    .iter()
                    .map(|(frame, renders)| a_frame(*frame, *renders))
                    .collect(),
                recorded_locations: Vec::new(),
                views: HowManyViewsTheSceneWrites::One,
                the_compositing_tree: None,
                the_view_transform: None,
            },
            settings: smoozy_job_files_machine::manifest::CarriedAndNotInterpretedHere(
                serde_json::to_value(crate::material::tests::a_reading().settings)
                    .expect("the settings half renders"),
            ),
            dependencies: smoozy_job_files_machine::manifest::CarriedAndNotInterpretedHere(
                serde_json::json!({}),
            ),
            errors: std::collections::BTreeMap::new(),
            the_baked_caches_on_the_machine: Vec::new(),
            the_scene_holds_a_simulation_not_baked: false,
            the_scene_it_read: None,
            the_open_files_path: None,
            the_check_is_written_as_a_picture: false,
        }
    }

    fn an_ordinary_reading() -> ThePrintedReading {
        a_printed_reading(
            vec![the_regular_output_path(), a_node_destination("Beauty", "Image", "//nodes/beauty/")],
            &[(1, true), (2, true), (3, true)],
        )
    }

    /// A stand-in for the reading, recording the frames each run was handed.
    struct AReadingThat {
        printed: Option<ThePrintedReading>,
        handed: std::cell::RefCell<Vec<Vec<i64>>>,
        /// **`Job files and outputs` R4c**: whether each reading was told it
        /// is a first-frame check's.
        told_a_check: std::cell::RefCell<Vec<bool>>,
        /// **`Job files and outputs` R1a**: the path each reading opened the file at.
        opened: std::cell::RefCell<Vec<String>>,
    }

    impl AReadingThat {
        fn prints(printed: ThePrintedReading) -> AReadingThat {
            AReadingThat {
                printed: Some(printed),
                handed: std::cell::RefCell::new(Vec::new()),
                told_a_check: std::cell::RefCell::new(Vec::new()),
                opened: std::cell::RefCell::new(Vec::new()),
            }
        }
        fn answers_nothing() -> AReadingThat {
            AReadingThat {
                printed: None,
                handed: std::cell::RefCell::new(Vec::new()),
                told_a_check: std::cell::RefCell::new(Vec::new()),
                opened: std::cell::RefCell::new(Vec::new()),
            }
        }
    }

    impl TakesTheReading for AReadingThat {
        fn take(&self, taking: &TheReadingToTake<'_>) -> Result<ThePrintedReading, String> {
            self.handed
                .borrow_mut()
                .push(taking.the_frames_the_job_covers.to_vec());
            self.told_a_check
                .borrow_mut()
                .push(taking.the_check_is_written_as_a_picture);
            self.opened
                .borrow_mut()
                .push(taking.the_blend_path_on_the_machine.to_string());
            self.printed
                .clone()
                .ok_or_else(|| "the reading printed no document".to_string())
        }
    }

    /// A stand-in for the machine's own answers. **It answers and never
    /// judges**, which is what the trait is for.
    struct TheMachineAnswers {
        remap: Option<RemapMaterial>,
        four_values: Option<SourceReading>,
        hold: Option<Identity>,
        walk: WhatTheWalkOfTheFolderIs,
        record_refuses: Option<String>,
        pair_refuses: Option<String>,
        wrote: Vec<AttemptRecord>,
        readings_written: Vec<TheReadingAndWhatFollows>,
        hold_reasons: Vec<String>,
        recorded_pairs: usize,
        /// Each copy asked of the machine, as (from, to).
        copies: Vec<(Vec<u8>, Vec<u8>)>,
        copy_refuses: Option<String>,
        opening_folders: Vec<smoozy_dependencies_machine::opening::TheOpeningFolder>,
        opening_refuses: Option<String>,
        placed_written: Vec<ThePlacedCopies>,
        /// The last text placed — the launch material, which is placed after
        /// L8b's script — kept before the run's folder is removed.
        the_last_text_placed: String,
        /// The machine's acts in the order they were asked, for the ones whose
        /// order is the point.
        order: Vec<&'static str>,
        /// **The folders that stand on the machine**, for `Job files and
        /// outputs` P20's stat, and each folder made, in order.
        folders: std::collections::BTreeSet<String>,
        made: Vec<String>,
        /// A folder whose make the machine refuses.
        make_refuses: Option<String>,
        /// Where the job output folder was recorded to sit (`Job files and
        /// outputs` P21), and a refusal of that write.
        where_written: Vec<WhereTheJobOutputFolderSits>,
        where_refuses: Option<String>,
    }

    impl MakesFolders for TheMachineAnswers {
        fn what_stands_at(&mut self, path: &str) -> smoozy_job_files_machine::where_the_folder_sits::WhatStands {
            if self.folders.contains(path) {
                smoozy_job_files_machine::where_the_folder_sits::WhatStands::AFolder
            } else {
                smoozy_job_files_machine::where_the_folder_sits::WhatStands::Nothing
            }
        }
        fn make_one_folder(&mut self, path: &str) -> Result<(), ()> {
            if self.make_refuses.as_deref() == Some(path) || !self.folders.insert(path.to_string()) {
                return Err(());
            }
            self.order.push("folder made");
            self.made.push(path.to_string());
            Ok(())
        }
    }

    impl TheWorkersOwnSteps for TheMachineAnswers {
        fn the_machines_folders(&mut self) -> &mut dyn MakesFolders {
            self
        }
        fn write_where_the_folder_sits(&mut self, sits: &WhereTheJobOutputFolderSits) -> Result<(), String> {
            self.order.push("where the folder sits");
            if let Some(why) = &self.where_refuses {
                return Err(why.clone());
            }
            self.where_written.push(sits.clone());
            Ok(())
        }
        fn read_the_remap_material(&mut self, _at: &str) -> Option<RemapMaterial> {
            self.remap.clone()
        }
        fn the_four_values_now(&mut self, _blend_path: &str) -> Option<SourceReading> {
            self.four_values.clone()
        }
        fn write_the_recorded_reading(&mut self, composed: &TheReadingAndWhatFollows) {
            self.readings_written.push(composed.clone());
        }
        fn ask_the_supervisor_to_place_the_hold(&mut self, reason: &str) -> Option<Identity> {
            self.hold_reasons.push(reason.to_string());
            self.hold.clone()
        }
        fn walk_the_job_output_folder(&mut self, _at: &str) -> WhatTheWalkOfTheFolderIs {
            self.walk.clone()
        }
        fn copy_into_the_job_folder(
            &mut self,
            from: &[u8],
            to: &[u8],
        ) -> Result<Vec<APlacedFile>, String> {
            self.order.push("copy");
            self.copies.push((from.to_vec(), to.to_vec()));
            if let Some(why) = &self.copy_refuses {
                return Err(why.clone());
            }
            let mut at = to.to_vec();
            at.extend_from_slice(b"/lightmap.png");
            Ok(vec![APlacedFile {
                job_folder_path: smoozy_dependencies_machine::bytes::MachineLocation::composed(at),
                size: 16,
                inode: "4242".to_string(),
                modification_time: 1_700_000_000,
                change_time: 1_700_000_001,
            }])
        }
        fn make_the_opening_folder(
            &mut self,
            folder: &smoozy_dependencies_machine::opening::TheOpeningFolder,
        ) -> Result<(), String> {
            self.order.push("opening");
            self.opening_folders.push(folder.clone());
            match &self.opening_refuses {
                Some(why) => Err(why.clone()),
                None => Ok(()),
            }
        }
        fn write_the_placed_copies(&mut self, placed: &ThePlacedCopies) {
            self.order.push("placed-copies");
            self.placed_written.push(placed.clone());
        }
        fn write_the_attempt_record(&mut self, record: &AttemptRecord) -> Result<(), String> {
            self.order.push("attempt record");
            match &self.record_refuses {
                Some(why) => Err(why.clone()),
                None => {
                    self.wrote.push(record.clone());
                    Ok(())
                }
            }
        }
        fn record_the_render_process(
            &mut self,
            _pair: smoozy_runtime::execution_group::RecordedPair,
        ) -> Result<(), String> {
            match &self.pair_refuses {
                Some(why) => Err(why.clone()),
                None => {
                    self.recorded_pairs += 1;
                    Ok(())
                }
            }
        }
    }

    /// A Blender that names the version the manifest names, and one that
    /// names another.
    struct NamesItself(Option<Version>);
    impl AsksABlenderToNameItsVersion for NamesItself {
        fn what_version_it_names(&self, _at: &str) -> Option<Version> {
            self.0
        }
    }

    /// Something in the place a build's path goes that this test can really
    /// run, so the launch is made rather than reasoned about.
    fn a_program_that_ends() -> String {
        "/bin/sh".to_string()
    }

    const FIVE_TWO: Version = Version {
        major: 5,
        minor: 2,
        patch: 0,
    };

    /// A start that records the words the launch carried and then makes it
    /// for real.
    struct Watched {
        start: TheRenderStartedOnTheMachine<AKernelThat>,
        words: std::cell::RefCell<Vec<String>>,
    }
    impl StartsTheRender for Watched {
        fn start(&self, launch: &TheLaunch) -> Result<TheRenderAsItWasStarted, String> {
            *self.words.borrow_mut() = launch.words.clone();
            self.start.start(launch)
        }
    }

    struct Placed(std::path::PathBuf);
    impl PlacesTheMaterial for Placed {
        fn place(&self, _at: &str, text: &str) -> Result<(), String> {
            std::fs::write(&self.0, text).map_err(|why| why.to_string())
        }
        fn read_it_back(&self, _at: &str) -> Option<Vec<u8>> {
            std::fs::read(&self.0).ok()
        }
    }

    fn answers() -> TheMachineAnswers {
        let job = a_job();
        TheMachineAnswers {
            remap: Some(a_remap(&job, false)),
            four_values: Some(four_values()),
            hold: Some(Identity::issue(smoozy_state::identities::IdentityKind::Condition).unwrap()),
            walk: WhatTheWalkOfTheFolderIs::ItCompleted(Vec::<AWalkEntry>::new()),
            record_refuses: None,
            pair_refuses: None,
            wrote: Vec::new(),
            readings_written: Vec::new(),
            hold_reasons: Vec::new(),
            recorded_pairs: 0,
            copies: Vec::new(),
            copy_refuses: None,
            opening_folders: Vec::new(),
            opening_refuses: None,
            placed_written: Vec::new(),
            the_last_text_placed: String::new(),
            order: Vec::new(),
            folders: std::collections::BTreeSet::new(),
            made: Vec::new(),
            make_refuses: None,
            where_written: Vec::new(),
            where_refuses: None,
        }
    }

    /// Everything a run of the driver is composed from, so one test can move
    /// one thing.
    struct ARun {
        names: Option<Version>,
        reading: AReadingThat,
        previous_attempts: Vec<AttemptRecord>,
        payload: Payload,
        manifests_reading: Option<TheReadingAndEverythingDerivedFromIt>,
        build: String,
        frame_range: FrameRange,
        scene: String,
        /// **What the machine's loader answers L24b's lookup with.**
        optix: WhatTheLoaderFound,
        /// **`Job files and outputs` P21**: the folder known before the
        /// attempt, or `None` where it is chosen at step 4 — against the
        /// layout below, with this chosen fallback folder (`Job files and
        /// outputs` P23).
        known_folder: Option<&'static str>,
        the_servers_fallback_folder: Option<&'static str>,
    }

    impl ARun {
        fn ordinary() -> ARun {
            ARun {
                names: Some(FIVE_TWO),
                reading: AReadingThat::prints(an_ordinary_reading()),
                previous_attempts: Vec::new(),
                payload: crate::material::tests::an_untouched_payload(),
                manifests_reading: None,
                build: a_program_that_ends(),
                frame_range: FrameRange::TheUsersParsedIntegers(vec![1, 2, 3]),
                scene: "Kitchen".to_string(),
                optix: WhatTheLoaderFound::TheLibrary,
                known_folder: Some("/ephemeral/smoozy/jobs/j1"),
                the_servers_fallback_folder: None,
            }
        }
    }

    /// **A loader that answers as it is told and counts how often it was
    /// asked**, so a test can say L24b's lookup was made or was not.
    struct ALoaderThatCounts {
        answers: WhatTheLoaderFound,
        asked: std::cell::Cell<u32>,
    }
    impl LooksForTheOptixLibrary for ALoaderThatCounts {
        fn what_the_loader_finds(&self) -> WhatTheLoaderFound {
            self.asked.set(self.asked.get() + 1);
            self.answers.clone()
        }
    }

    /// **One whole run of the driver**, handing back what it ended at and the
    /// words the launch was actually made with.
    fn run_it(machine: &mut TheMachineAnswers, it: &ARun) -> (HowTheAttemptWent, Vec<String>) {
        let (went, words, _) = run_it_counting(machine, it);
        (went, words)
    }

    /// **[`run_it`], and how many times L24b's lookup was made.**
    fn run_it_counting(
        machine: &mut TheMachineAnswers,
        it: &ARun,
    ) -> (HowTheAttemptWent, Vec<String>, u32) {
        let job = a_job();
        let mut manifest = a_manifest(&job, "5.2.0", &it.scene);
        manifest.frame_range = it.frame_range.clone();
        let clock = the_machines_clock();
        let root = std::env::temp_dir().join(format!(
            "smoozy-pipeline-{}",
            Identity::issue(smoozy_state::identities::IdentityKind::Operation)
                .expect("randomness")
                .as_text()
        ));
        std::fs::create_dir_all(&root).expect("the folder");
        let material_at = root.join("launch-material.py");
        let script_at = root.join("reading.py");
        let places = Placed(material_at.clone());
        let start = Watched {
            start: TheRenderStartedOnTheMachine {
                kernel: AKernelThat(Some(19186)),
            },
            words: std::cell::RefCell::new(Vec::new()),
        };
        let asks = NamesItself(it.names);
        let loader = ALoaderThatCounts {
            answers: it.optix.clone(),
            asked: std::cell::Cell::new(0),
        };
        let layout = smoozy_job_files_machine::layout::TheLayout::of(
            smoozy_job_files_machine::layout::MachineRoot::chosen("/srv/smoozy").expect("a root"),
            smoozy_job_files_machine::layout::WhereThePackageAreaSits::UnderTheRoot,
        );
        let the_job_output_folder = match it.known_folder {
            Some(folder) => TheJobOutputFolderForThisAttempt::Known(folder),
            None => TheJobOutputFolderForThisAttempt::ToBeChosen {
                layout: &layout,
                the_folders_name: "j1-Kitchen",
                the_servers_fallback_folder: it.the_servers_fallback_folder,
            },
        };
        let went = {
            let mut pipeline = TheRenderWorkersPipeline {
                start: &start,
                places: &places,
                asks_the_version: &asks,
                takes_the_reading: &it.reading,
                steps: machine,
                looks_for_the_optix_library: &loader,
            };
            pipeline.run(TheAttempt {
                job: &job,
                manifest: &manifest,
                payload: &it.payload,
                the_manifests_version: FIVE_TWO,
                the_remap_materials_path: "/ephemeral/smoozy/assets/remaps/j1",
                the_blender_build_path: &it.build,
                the_launch_materials_path: material_at.to_str().unwrap(),
                the_load_time_scripts_path: "/ephemeral/smoozy/job-records/j1/load-time.py",
                the_readings_script_path: script_at.to_str().unwrap(),
                the_job_output_folder,
                the_folder_the_blend_sits_in: BLEND_SITS_IN,
                previous_attempts: &it.previous_attempts,
                number: 1,
                the_machines_clock: &clock,
                the_manifests_reading: it.manifests_reading.as_ref(),
                the_manifests_settings_half: None,
            })
        };
        machine.the_last_text_placed = std::fs::read_to_string(&material_at).unwrap_or_default();
        std::fs::remove_dir_all(&root).ok();
        let words = start.words.borrow().clone();
        (went, words, loader.asked.get())
    }

    /// **`Job files and outputs` P21's second case, at step 4**: the
    /// folder is chosen where the file points — the run `//out` and
    /// `//nodes/beauty` share, the `.blend`'s own folder here — made with what
    /// is missing, recorded before step 5, walked at step 6, and the render's
    /// own output goes under it. Nothing chose before; nothing is in Smoozy's
    /// job area. The Creator's second notes, piece 4.
    #[test]
    fn a_folder_chosen_at_render_start_is_made_where_the_file_points_recorded_and_rendered_into() {
        let mut machine = answers();
        machine.folders.extend(["/ephemeral".to_string(), "/ephemeral/smoozy".to_string()]);
        let it = ARun {
            known_folder: None,
            ..ARun::ordinary()
        };
        let (went, words) = run_it(&mut machine, &it);
        let folder = "/ephemeral/smoozy/assets/j1-Kitchen";
        assert_eq!(machine.made, vec!["/ephemeral/smoozy/assets".to_string(), folder.to_string()]);
        assert_eq!(machine.where_written.len(), 1, "recorded once");
        assert_eq!(machine.where_written[0].path, folder);
        assert_eq!(
            machine.where_written[0].where_it_is,
            smoozy_job_files_machine::where_the_folder_sits::WhereItIs::WhereTheFilePoints
        );
        assert_eq!(went.the_job_output_folder.as_deref(), Some(folder));
        let at = words.iter().position(|word| word == "--render-output").expect("the line");
        assert!(words[at + 1].starts_with(folder), "{}", words[at + 1]);
        // **Recorded before the hold is judged and before the record of the
        // attempt**: a later attempt finds it and never chooses again.
        let recorded = machine.order.iter().position(|act| *act == "where the folder sits").unwrap();
        let attempt = machine.order.iter().position(|act| *act == "attempt record").unwrap();
        assert!(recorded < attempt, "{:?}", machine.order);
    }

    /// **A folder known before the attempt is never chosen again** (`Job
    /// files and outputs` P21: *a
    /// later attempt never chooses again*), and **a choice that cannot be
    /// recorded, or a job area that refuses a fresh folder (`Job files and
    /// outputs` N10), ends the
    /// attempt before the render**.
    #[test]
    fn a_known_folder_is_never_chosen_again_and_a_choice_nothing_records_launches_nothing() {
        let mut machine = answers();
        let (went, _) = run_it(&mut machine, &ARun::ordinary());
        assert!(machine.made.is_empty() && machine.where_written.is_empty());
        assert_eq!(went.the_job_output_folder.as_deref(), Some("/ephemeral/smoozy/jobs/j1"));

        let mut machine = answers();
        machine.folders.extend(["/ephemeral".to_string(), "/ephemeral/smoozy".to_string()]);
        machine.where_refuses = Some("the disk is full".to_string());
        let (went, _) = run_it(&mut machine, &ARun { known_folder: None, ..ARun::ordinary() });
        assert!(
            matches!(
                went.ended,
                WhereTheAttemptEnded::FailedBeforeTheRenderInstruction {
                    reason: BeforeTheRenderInstruction::TheAttemptCouldNotBeComposed { .. },
                    ..
                }
            ),
            "{:?}",
            went.ended
        );
        assert!(machine.wrote.is_empty(), "no attempt record, no launch");

        // **The run is under nothing that stands, so the make at its first
        // missing folder is refused**; the job area refuses too (`Job files
        // and outputs` N10).
        let mut machine = answers();
        machine.make_refuses = Some("/srv/smoozy/jobs/j1-Kitchen".to_string());
        let (went, _) = run_it(&mut machine, &ARun { known_folder: None, ..ARun::ordinary() });
        assert!(matches!(went.ended, WhereTheAttemptEnded::FailedBeforeTheRenderInstruction { .. }));
        assert!(machine.wrote.is_empty());
    }

    /// **`Job files and outputs` P19 and `Job files and outputs` P23 at step
    /// 4**: a run the machine
    /// lacks at its top (a Mac's
    /// `/Users/…`, here the `.blend`'s own folder with nothing standing) takes
    /// the server's chosen fallback folder, and the record says why.
    #[test]
    fn a_run_that_would_need_a_folder_under_the_root_takes_the_chosen_fallback_folder() {
        let mut machine = answers();
        machine.folders.insert("/mnt".to_string());
        let it = ARun {
            known_folder: None,
            the_servers_fallback_folder: Some("/mnt/renders"),
            ..ARun::ordinary()
        };
        let (went, _) = run_it(&mut machine, &it);
        assert_eq!(went.the_job_output_folder.as_deref(), Some("/mnt/renders/j1-Kitchen"));
        assert_eq!(
            machine.where_written[0].where_it_is,
            smoozy_job_files_machine::where_the_folder_sits::WhereItIs::TheFallback {
                why: Some(smoozy_job_files_machine::where_the_folder_sits::WhyTheFilesOwnPlaceWasNotUsed::ItIsAnotherComputers),
                the_servers_fallback_folder_was_passed_over: false,
            }
        );
    }

    fn launched(went: HowTheAttemptWent) -> TheRenderAsItWasStarted {
        match went.ended {
            WhereTheAttemptEnded::TheRenderWasLaunched(started) => started,
            other => panic!("{other:?}"),
        }
    }

    /// **The whole pipeline run end to end**: the reading taken, everything
    /// derived from it composed, the material placed and read back, the
    /// attempt record written before the launch, the launch made.
    #[test]
    fn the_pipeline_runs_the_steps_in_order_and_ends_at_a_launched_render() {
        let mut machine = answers();
        let it = ARun::ordinary();
        let (went, words) = run_it(&mut machine, &it);
        assert!(went.inventory.is_some(), "the inventory is step 4's answer");
        // **`Job files and outputs` D10: the map decides the regular output
        // path's value and the argument delivers it** — under the job's own
        // folder, never the artist's `//out/f_`. **This is the empty redirect
        // `Job files and outputs` D22 names**, the most visible of the eight
        // faults step 4 being real is what fixes.
        let at = words
            .iter()
            .position(|word| word == "--render-output")
            .expect("the regular output path is on the line");
        assert!(
            words[at + 1].starts_with("/ephemeral/smoozy/jobs/j1"),
            "{}",
            words[at + 1]
        );
        // **L17 and L18: the explicit frame list is the render instruction**,
        // carrying the reckoning's own integers.
        let at = words
            .iter()
            .position(|word| word == "--render-frame")
            .expect("the frame list is on the line");
        assert_eq!(words[at + 1], "1..3");
        assert_eq!(went.the_whole_range, vec![1, 2, 3]);
        let mut started = launched(went);
        // **The record was written before the render started**, and it carries
        // the reckoning's own list in the asked order.
        assert_eq!(machine.wrote.len(), 1);
        assert_eq!(
            machine.wrote[0].frame_list_in_the_order_it_was_written,
            vec![1, 2, 3]
        );
        // **`Job files and outputs` M8: the map in force, and not an empty
        // statement.**
        assert!(matches!(
            machine.wrote[0].map_in_force,
            TheMapInForce::It(_)
        ));
        // **`State, identity and recovery` §1.2 and `Job files and outputs`
        // H9 together: the recorded reading was written, once.**
        assert_eq!(machine.readings_written.len(), 1);
        assert_eq!(machine.recorded_pairs, 1);
        started.child.kill().ok();
        started.child.wait().ok();
    }

    /// **The ordinary job on a rented GPU machine: a cards job composes and
    /// the technology is on the line.**
    ///
    /// **L23's make is the manifest's** (`Job files and outputs` M1b). **Until
    /// 2026-09-20 the plan carried no such field**, so the worker passed *the
    /// cards could not be established* and **every cards job ended at L23's
    /// loud failure before the render** — a failed attempt on a machine the
    /// user is paying for, on the ordinary job rather than on an odd one.
    #[test]
    fn a_cards_job_composes_and_the_technology_is_named_on_the_line() {
        let mut machine = answers();
        let mut it = ARun::ordinary();
        // **`Render Settings` D1: the user's own answer, which L21 reads as
        // the resolved choice.**
        it.payload.device = Some(smoozy_render_settings_machine::device::DeviceChoice::TheCards);
        let (went, words) = run_it(&mut machine, &it);
        // **L22: the bare technology, last on the line** — the device
        // argument sits after the bare double dash and takes its value next,
        // and L23's table gives `OPTIX` for the manifest's NVIDIA make, on the
        // Creator's ruling of 2026-10-03 (`CUDA` until then). The argument's
        // own spelling is read from its one declaration point and is never
        // written out here (L24a; `spellings.rs`'s own guard).
        assert_eq!(words.last().map(String::as_str), Some("OPTIX"), "{words:?}");
        // **It reached a launched render**, which is the whole of what the
        // missing field cost.
        let mut started = launched(went);
        started.child.kill().ok();
        started.child.wait().ok();
    }

    /// **L24b: a cards job naming `OPTIX` on a machine whose loader does not
    /// find the OptiX library ends before the render instruction, in plain
    /// words, and nothing is launched, placed or recorded.**
    ///
    /// **This is the job Fact 47's driver route would have met on every
    /// Ubuntu machine Smoozy installed a driver on**: Blender's own refusal
    /// there is the line a broken driver gives too, and it names nothing.
    #[test]
    fn a_cards_job_on_a_machine_with_no_optix_library_ends_before_anything_is_launched() {
        let mut machine = answers();
        let mut it = ARun::ordinary();
        it.payload.device = Some(smoozy_render_settings_machine::device::DeviceChoice::TheCards);
        it.optix = WhatTheLoaderFound::NoLibrary;
        let (went, words, asked) = run_it_counting(&mut machine, &it);
        assert_eq!(asked, 1, "L24b's lookup is made once, before the launch");
        let WhereTheAttemptEnded::FailedBeforeTheRenderInstruction { at, reason } = &went.ended
        else {
            panic!("{:?}", went.ended);
        };
        assert_eq!(at.number, 7);
        assert_eq!(*reason, BeforeTheRenderInstruction::TheOptixLibraryWasNotFound);
        // **Nothing is launched, so no render is paid for** — and nothing of
        // step 7 that follows the lookup was done either.
        assert!(words.is_empty(), "{words:?}");
        assert!(machine.wrote.is_empty(), "no attempt record before a launch that never comes");
        assert_eq!(machine.recorded_pairs, 0);
        assert!(machine.the_last_text_placed.is_empty(), "no launch material was placed");
        // **K14's failed limb, cause *undetermined* at K14a**, so the phone
        // offers no *try it again* for a refusal the same machine repeats.
        let outcome = crate::endings::the_outcome_of(&went.ended).expect("it classifies");
        assert_eq!(
            crate::endings::the_cause_on(&outcome),
            Some(crate::endings::Cause::Undetermined)
        );
    }

    /// **L24b's other two answers change nothing**: found, the launch goes
    /// ahead naming `OPTIX`; and a lookup that did not answer knows nothing
    /// about the library, so the launch goes ahead too and L24's own refusal
    /// stands for whatever is wrong.
    #[test]
    fn a_found_library_or_a_lookup_with_no_answer_leaves_the_launch_as_it_was() {
        for answer in [
            WhatTheLoaderFound::TheLibrary,
            WhatTheLoaderFound::NoAnswer {
                why: "the lookup for libnvoptix.so.1 ended with status 2".to_string(),
            },
        ] {
            let mut machine = answers();
            let mut it = ARun::ordinary();
            it.payload.device =
                Some(smoozy_render_settings_machine::device::DeviceChoice::TheCards);
            it.optix = answer.clone();
            let (went, words, asked) = run_it_counting(&mut machine, &it);
            assert_eq!(asked, 1, "{answer:?}");
            assert_eq!(words.last().map(String::as_str), Some("OPTIX"), "{answer:?}");
            let mut started = launched(went);
            started.child.kill().ok();
            started.child.wait().ok();
        }
    }

    /// **L24b asks only where the job names `OPTIX`**: a job that renders on
    /// the processor names no technology (L21), and a machine with no OptiX
    /// library renders it exactly as before.
    #[test]
    fn a_processor_job_never_asks_for_the_optix_library() {
        let mut machine = answers();
        let mut it = ARun::ordinary();
        it.payload.device =
            Some(smoozy_render_settings_machine::device::DeviceChoice::TheProcessor);
        it.optix = WhatTheLoaderFound::NoLibrary;
        let (went, words, asked) = run_it_counting(&mut machine, &it);
        assert_eq!(asked, 0);
        assert!(!words.iter().any(|word| word == "OPTIX"), "{words:?}");
        let mut started = launched(went);
        started.child.kill().ok();
        started.child.wait().ok();
    }

    /// **`Rendering` L21's bullet of 2026-11-02, and L24b with it: a job on any
    /// engine but Cycles names no technology, so a machine with no OptiX library
    /// launches it and the lookup is never made**, though its device says the
    /// cards.
    #[test]
    fn an_eevee_job_on_the_cards_never_asks_for_the_optix_library() {
        use smoozy_render_settings_machine::engine::{EngineEntry, OfferedEngine};
        let mut machine = answers();
        let mut it = ARun::ordinary();
        it.payload.device = Some(smoozy_render_settings_machine::device::DeviceChoice::TheCards);
        it.payload.engine = Some(EngineEntry {
            pick: OfferedEngine::Eevee,
            identifier: None,
        });
        it.optix = WhatTheLoaderFound::NoLibrary;
        let (went, words, asked) = run_it_counting(&mut machine, &it);
        assert_eq!(asked, 0, "L24b's lookup is not made");
        assert!(!words.iter().any(|word| word == "OPTIX" || word == "--"), "{words:?}");
        let mut started = launched(went);
        started.child.kill().ok();
        started.child.wait().ok();
    }

    /// **`Job files and outputs` R13's forbidden case, by name.**
    ///
    /// The file changed after submission and the manifest carries a map. **The
    /// manifest's map is not applied**: a fresh reading is taken after the
    /// load, a fresh map composed, and both are written into that attempt's
    /// record. R9's own words are what this is against — *"the folders the user
    /// was shown and the folders the render writes into are the same object"* —
    /// turned against itself by a map composed for a file that no longer
    /// exists.
    #[test]
    fn a_file_that_changed_does_not_take_the_manifests_map_and_a_fresh_one_is_composed() {
        // A map the desktop composed, which this attempt must not apply. Its
        // one pair is for a destination this file no longer holds.
        let stale = TheReadingAndEverythingDerivedFromIt {
            reading: smoozy_job_files_machine::reading::TheOutputsHalf {
                taken_by: TheVersionInHand::of(5, 2),
                gating: plain_gating(),
                destinations: vec![a_node_destination("Old", "Stale", "//old/")],
                scenes_reached_through_a_sequencer_strip: Vec::new(),
                the_tree_is_shared_with: None,
                per_frame: vec![a_frame(1, true)],
                recorded_locations: Vec::new(),
                views: HowManyViewsTheSceneWrites::One,
                the_beauty_walk:
                    smoozy_job_files_machine::main_output::WhatTheWalkFound::NoEntry,
                the_view_transform: None,
            },
            map: TheMap { pairs: Vec::new() },
            inventory: WhatTheInventoryIs::TheFrameSetIsNotPredictable(
                WhyTheFrameSetIsNotPredictable::TheRegularOutputPathsNameWasNotAnswered,
            ),
            picks: smoozy_job_files_machine::main_output::ThePicks::made_once(None, None),
        };

        let mut machine = answers();
        // **The file changed**: the four values now differ from the manifest's.
        machine.four_values = Some(SourceReading {
            size: four_values().size + 1,
            ..four_values()
        });
        let mut it = ARun::ordinary();
        it.manifests_reading = Some(stale);
        let (went, _) = run_it(&mut machine, &it);
        let mut started = launched(went);
        started.child.kill().ok();
        started.child.wait().ok();

        // **A fresh reading was taken after the load**, and it was written.
        assert_eq!(
            it.reading.handed.borrow().len(),
            1,
            "the reading is taken once for a job whose frames the manifest names"
        );
        assert_eq!(machine.readings_written.len(), 1);
        // **A fresh map was composed and it is the one in the attempt record**
        // — never the manifest's, which carried no pair at all.
        let TheMapInForce::It(map) = &machine.wrote[0].map_in_force else {
            panic!("the attempt record carries the map in force");
        };
        assert_eq!(
            map.pairs.len(),
            2,
            "the fresh reading's two destinations, and not the stale map's none"
        );
        // **M8: the identity read at step 3, never the one at submission.**
        assert_eq!(
            machine.wrote[0].file_identity_at_this_attempt.size,
            four_values().size + 1
        );
        assert!(machine.wrote[0].the_file_changed_before_this_attempt);
    }

    /// **`Job files and outputs` R7's first bullet from the other side**: where
    /// the file is unchanged and the manifest carries a map, **that map is the
    /// map in force and nothing is recomputed** (R9).
    #[test]
    fn an_unchanged_file_keeps_the_manifests_map_and_composes_no_second_one() {
        let carried = TheReadingAndEverythingDerivedFromIt {
            reading: smoozy_job_files_machine::reading::TheOutputsHalf {
                taken_by: TheVersionInHand::of(5, 2),
                gating: plain_gating(),
                destinations: vec![the_regular_output_path()],
                scenes_reached_through_a_sequencer_strip: Vec::new(),
                the_tree_is_shared_with: None,
                per_frame: vec![a_frame(1, true)],
                recorded_locations: Vec::new(),
                views: HowManyViewsTheSceneWrites::One,
                the_beauty_walk:
                    smoozy_job_files_machine::main_output::WhatTheWalkFound::NoEntry,
                the_view_transform: None,
            },
            map: TheMap { pairs: Vec::new() },
            inventory: WhatTheInventoryIs::TheFrameSetIsNotPredictable(
                WhyTheFrameSetIsNotPredictable::TheRegularOutputPathsNameWasNotAnswered,
            ),
            picks: smoozy_job_files_machine::main_output::ThePicks::made_once(None, None),
        };
        let mut machine = answers();
        let mut it = ARun::ordinary();
        it.manifests_reading = Some(carried);
        let (went, _) = run_it(&mut machine, &it);
        let mut started = launched(went);
        started.child.kill().ok();
        started.child.wait().ok();
        // **Nothing recomputed the map**: the attempt record carries the
        // manifest's own, which holds no pair.
        let TheMapInForce::It(map) = &machine.wrote[0].map_in_force else {
            panic!("the attempt record carries the map in force");
        };
        assert!(map.pairs.is_empty(), "R9: the manifest's map, unrecomputed");
        // **The reading was still taken**, because the manifest in code carries
        // no settings half and P1a has to validate against one.
        assert_eq!(it.reading.handed.borrow().len(), 1);
        assert!(
            machine.readings_written.is_empty(),
            "nothing of the manifest's job record is written again"
        );
    }

    /// **L10b's gate is read off the step-4 reading on every attempt** —
    /// where the map in force is composed from it, and where the manifest's
    /// map is kept and nothing else of the reading is used — and the placed
    /// material steps from the scene's start to the list's highest frame only
    /// where `Job files and outputs` R4 item 8 says yes.
    #[test]
    fn the_stepping_follows_the_step_four_reading_whether_or_not_the_map_is_the_manifests() {
        let step = r#"{"do":"Step","highest":3,"lowest":1}"#;
        for (says, carried_map, on) in [(false, false, true), (true, false, true), (true, true, true), (true, false, false), (true, true, false)] {
            let mut machine = answers();
            let mut it = ARun::ordinary();
            let mut printed = an_ordinary_reading();
            printed.the_scene_holds_a_simulation_not_baked = says;
            it.payload.prepare_simulations = on;
            it.reading = AReadingThat::prints(printed);
            if carried_map {
                it.manifests_reading = Some(TheReadingAndEverythingDerivedFromIt {
            reading: smoozy_job_files_machine::reading::TheOutputsHalf {
                taken_by: TheVersionInHand::of(5, 2),
                gating: plain_gating(),
                destinations: vec![the_regular_output_path()],
                scenes_reached_through_a_sequencer_strip: Vec::new(),
                the_tree_is_shared_with: None,
                per_frame: vec![a_frame(1, true)],
                recorded_locations: Vec::new(),
                views: HowManyViewsTheSceneWrites::One,
                the_beauty_walk:
                    smoozy_job_files_machine::main_output::WhatTheWalkFound::NoEntry,
                the_view_transform: None,
            },
            map: TheMap { pairs: Vec::new() },
            inventory: WhatTheInventoryIs::TheFrameSetIsNotPredictable(
                WhyTheFrameSetIsNotPredictable::TheRegularOutputPathsNameWasNotAnswered,
            ),
            picks: smoozy_job_files_machine::main_output::ThePicks::made_once(None, None),
                });
            }
            let (went, _) = run_it(&mut machine, &it);
            let mut started = launched(went);
            started.child.kill().ok();
            started.child.wait().ok();
            let text = &machine.the_last_text_placed;
            assert!(text.contains(r#"{"do":"ReadBack""#), "the launch material was the last text placed");
            assert_eq!(text.contains(step), says && on, "item 8 {says}, choice {on}, the manifest's map {carried_map}");
        }
    }

    /// **L10c and `Job files and outputs` R4c, the nineteenth notes, notes 3
    /// and 9: the payload's mark reaches the step-4 reading, and the material
    /// switches the scene to a picture where, and only where, that reading
    /// says it did.** A check whose scene the reading switched is launched as
    /// a frame list, a picture's instruction, never as a video's run.
    #[test]
    fn a_check_the_reading_wrote_as_a_picture_is_switched_in_the_material_and_launched_as_a_list() {
        let switch = r#""do":"WriteAsAPicture""#;
        for (marked, switched) in [(false, false), (true, false), (true, true)] {
            let mut machine = answers();
            let mut it = ARun::ordinary();
            it.payload.the_check_is_written_as_a_picture = marked;
            let mut printed = an_ordinary_reading();
            printed.the_check_is_written_as_a_picture = switched;
            it.reading = AReadingThat::prints(printed);
            let (went, words) = run_it(&mut machine, &it);
            let mut started = launched(went);
            started.child.kill().ok();
            started.child.wait().ok();
            assert_eq!(*it.reading.told_a_check.borrow(), vec![marked], "the reading is told");
            let text = &machine.the_last_text_placed;
            assert!(text.contains(r#"{"do":"ReadBack""#), "the launch material was the last text placed");
            assert_eq!(text.contains(switch), switched, "marked {marked}, switched {switched}");
            if switched {
                assert!(text.find(switch) < text.find(r#"{"do":"ReadBack""#), "before the read-back");
            }
            assert!(words.iter().any(|word| word == "--render-frame"), "{words:?}");
            assert!(!words.iter().any(|word| word == "-a"), "{words:?}");
        }
    }

    /// **`Job files and outputs` H5's two triggers, by name — the first.**
    ///
    /// **The job will write nothing at all.** It fires **only where the map was
    /// computed on the machine**, which is exactly the case step 4 now
    /// produces; before step 4 was real the hold answer was an input composed
    /// ahead of the reading and this condition could never fire.
    #[test]
    fn a_job_that_will_write_nothing_is_held_where_the_machine_composed_the_map() {
        let mut machine = answers();
        let mut it = ARun::ordinary();
        // Every frame has no active view layer, so every frame owes nothing and
        // the count is zero (`Job files and outputs` H1a).
        it.reading = AReadingThat::prints(a_printed_reading(
            vec![the_regular_output_path()],
            &[(1, false), (2, false), (3, false)],
        ));
        let (went, _) = run_it(&mut machine, &it);
        assert!(
            matches!(went.ended, WhereTheAttemptEnded::AHoldWasPlaced { .. }),
            "{:?}",
            went.ended
        );
        // **The reason is `Job files and outputs` H1's own spelling, read back as
        // the condition** —
        // never a debug print (the 38's piece 10, item 26).
        assert_eq!(
            smoozy_job_files_machine::holds::the_conditions_a_hold_reason_names(
                &machine.hold_reasons[0]
            ),
            Some(vec![smoozy_job_files_machine::holds::WhatHoldsTheJob::TheJobWillWriteNothingAtAll]),
            "{}",
            machine.hold_reasons[0]
        );
        assert!(machine.wrote.is_empty(), "no render was started");
        assert_eq!(machine.recorded_pairs, 0);
    }

    /// **`Job files and outputs` H5's two triggers, by name — the second.**
    ///
    /// **Two destinations aimed at one name in one place**, the node-to-node
    /// set of §7.4. It fires on the same trigger and for the same reason.
    #[test]
    fn two_destinations_at_one_name_hold_the_job_where_the_machine_composed_the_map() {
        let mut machine = answers();
        let mut it = ARun::ordinary();
        it.reading = AReadingThat::prints(a_printed_reading(
            vec![
                a_node_destination("Beauty", "Image", "//nodes/beauty/"),
                a_node_destination("Second", "Image", "//nodes/beauty/"),
            ],
            &[(1, true)],
        ));
        let (went, _) = run_it(&mut machine, &it);
        assert!(
            matches!(went.ended, WhereTheAttemptEnded::AHoldWasPlaced { .. }),
            "{:?}",
            went.ended
        );
        // **The reason is `Job files and outputs` H1's own spelling, read back as
        // the condition** —
        // never a debug print (the 38's piece 10, item 26).
        assert_eq!(
            smoozy_job_files_machine::holds::the_conditions_a_hold_reason_names(
                &machine.hold_reasons[0]
            ),
            Some(vec![smoozy_job_files_machine::holds::WhatHoldsTheJob::TwoDestinationsAimedAtOneNameInOnePlace]),
            "{}",
            machine.hold_reasons[0]
        );
    }

    /// **`Job files and outputs` H5's first bullet, from the other side:
    /// neither of those two holds a job whose map came from the manifest**,
    /// because a user who pre-loaded and submitted has already answered them
    /// (`Job files and outputs` H6).
    #[test]
    fn neither_of_the_two_fires_where_the_desktop_composed_the_map() {
        let colliding = smoozy_job_files_machine::reading::TheOutputsHalf {
            taken_by: TheVersionInHand::of(5, 2),
            gating: plain_gating(),
            destinations: vec![
                a_node_destination("Beauty", "Image", "//nodes/beauty/"),
                a_node_destination("Second", "Image", "//nodes/beauty/"),
            ],
            scenes_reached_through_a_sequencer_strip: Vec::new(),
            the_tree_is_shared_with: None,
            per_frame: vec![a_frame(1, true)],
            recorded_locations: Vec::new(),
            views: HowManyViewsTheSceneWrites::One,
            the_beauty_walk: smoozy_job_files_machine::main_output::WhatTheWalkFound::NoEntry,
            the_view_transform: None,
        };
        // The desktop's own composition of that same reading, collisions and
        // all — which the user was shown and answered by submitting.
        let map = smoozy_job_files_machine::mapping::compose_the_map(
            &colliding
                .destinations
                .iter()
                .map(|destination| smoozy_job_files_machine::mapping::ADestinationToMap {
                    destination,
                    the_longest_name_it_will_write: None,
                })
                .collect::<Vec<_>>(),
            BLEND_SITS_IN,
            "/ephemeral/smoozy/jobs/j1",
            TheVersionInHand::of(5, 2),
        );
        let inventory = smoozy_job_files_machine::inventory::compose_the_inventory(&colliding, &map);
        assert!(
            matches!(&inventory, WhatTheInventoryIs::It(held) if !held.collision_record.is_empty()),
            "the fixture really does collide"
        );
        let mut machine = answers();
        let mut it = ARun::ordinary();
        it.manifests_reading = Some(TheReadingAndEverythingDerivedFromIt {
            reading: colliding,
            map,
            inventory,
            picks: smoozy_job_files_machine::main_output::ThePicks::made_once(None, None),
        });
        let (went, _) = run_it(&mut machine, &it);
        let mut started = launched(went);
        started.child.kill().ok();
        started.child.wait().ok();
        assert!(
            machine.hold_reasons.is_empty(),
            "`Job files and outputs` H6: the user is not asked the same question twice"
        );
    }

    /// **`Render Settings` Y9's second form: the file's own frame range,
    /// composed against the reading in force at the attempt.**
    ///
    /// Before step 4 was real the worker left that range empty and the attempt
    /// reached step 7 with no frame to name, which is K14's loud ending as the
    /// ordinary case for every job whose range the user never touched.
    #[test]
    fn the_files_own_range_is_composed_from_the_reading_in_force() {
        let mut machine = answers();
        let mut it = ARun::ordinary();
        it.frame_range = FrameRange::TheFilesOwn;
        let (went, words) = run_it(&mut machine, &it);
        assert_eq!(
            went.the_whole_range,
            (1..=10).collect::<Vec<i64>>(),
            "the scene's own first, last and step"
        );
        let mut started = launched(went);
        started.child.kill().ok();
        started.child.wait().ok();
        assert_eq!(
            machine.wrote[0].frame_list_in_the_order_it_was_written,
            (1..=10).collect::<Vec<i64>>()
        );
        assert!(words.iter().any(|word| word == "1..10"), "{words:?}");
        // **The file is loaded once, and that is the build runner's ruling
        // of 2026-09-20.** This job's frames are not known until the file has been
        // read, so the driver read once with no frames, learnt the range, and
        // read again over it — **a second load of the artist's file on every
        // attempt of the commonest job there is**, on a machine the user pays
        // for by the hour. The reading's own script now records a per-frame
        // entry over the whole span it already computes where no frames were
        // handed in, so one read answers both halves.
        let handed = it.reading.handed.borrow().clone();
        assert_eq!(handed, vec![Vec::<i64>::new()], "read once, with no frames");
    }

    /// **A job that hands its frames in is completely unaffected by that ruling.**
    ///
    /// It was never read twice and it still is not, and the frames it hands in
    /// are the frames it hands in — nothing about the wider table reaches it.
    #[test]
    fn a_job_that_hands_its_frames_in_is_read_once_with_exactly_those_frames() {
        let mut machine = answers();
        let it = ARun::ordinary();
        let (went, _) = run_it(&mut machine, &it);
        let mut started = launched(went);
        started.child.kill().ok();
        started.child.wait().ok();
        let handed = it.reading.handed.borrow().clone();
        assert_eq!(handed, vec![vec![1, 2, 3]]);
    }

    /// **P1a runs, and the line and the material cannot name different
    /// engines. Added 2026-09-05 out of Round 1 of this crate's code
    /// audit**, whose seventh must-do was that
    /// [`crate::pipeline::validate_the_names`] was written, tested and
    /// called by nothing.
    #[test]
    fn p1a_anchors_the_engine_and_the_launch_names_the_one_the_material_imposed_for() {
        use smoozy_render_settings_machine::engine::{EngineEntry, OfferedEngine};

        let mut machine = answers();
        let mut it = ARun::ordinary();
        it.payload.engine = Some(EngineEntry {
            pick: OfferedEngine::Eevee,
            identifier: None,
        });
        let (went, words) = run_it(&mut machine, &it);
        let mut started = launched(went);
        started.child.kill().ok();
        started.child.wait().ok();

        // **The line names the engine, and it is the reading in force's own
        // identifier** — not the pick, and not anything a caller handed in.
        let at = words
            .iter()
            .position(|word| word == "--engine")
            .expect("the launch names an engine");
        assert_eq!(words[at + 1], "BLENDER_EEVEE");
    }

    /// **P1a's refusal fails the attempt before the render instruction,
    /// naming what was not found** — and never at the launch, and never at
    /// success.
    #[test]
    fn an_engine_the_reading_in_force_does_not_register_fails_the_attempt_at_step_four() {
        use smoozy_render_settings_machine::engine::{EngineEntry, OfferedEngine};

        let mut machine = answers();
        let mut it = ARun::ordinary();
        it.payload.scene = Some(smoozy_render_settings_machine::payload::SceneEntry {
            name: smoozy_render_settings_machine::names::ExactName::as_the_reading_produced_it("Attic"),
            anchor: smoozy_render_settings_machine::names::AnchorState::ToAnchor {
                provenance: smoozy_render_settings_machine::names::Provenance::AKeptSetup {
                    the_old_job: "job-0".to_string(),
                },
            },
        });
        it.payload.engine = Some(EngineEntry {
            pick: OfferedEngine::Cycles,
            identifier: None,
        });

        let (went, words) = run_it(&mut machine, &it);
        let WhereTheAttemptEnded::FailedBeforeTheRenderInstruction { at, reason } = went.ended
        else {
            panic!("{:?}", went.ended);
        };
        assert_eq!(at.number, 4, "P1a rides step 4, before the composition");
        assert_eq!(
            reason,
            BeforeTheRenderInstruction::ANameTheReadingInForceDoesNotCarry {
                name: "Attic".to_string()
            }
        );
        // **Nothing was launched and no attempt record was written**: the
        // refusal is before step 7 entirely.
        assert!(words.is_empty());
        assert!(machine.wrote.is_empty());
    }

    /// **The two facts the attempt record carries for the telling, and they
    /// are wired rather than assumed. Added 2026-09-04 out of a code round.**
    #[test]
    fn the_attempt_record_carries_whether_the_file_changed_and_whether_a_frame_was_kept() {
        use smoozy_job_files_machine::reckoning::WhatKindOfEntry;

        let previous = vec![AttemptRecord {
            number: 1,
            frame_list_in_the_order_it_was_written: vec![1, 2, 3],
            map_in_force: TheMapInForce::NoneWasComposedForThisAttempt,
            file_identity_at_this_attempt: four_values(),
            the_file_changed_before_this_attempt: false,
            a_frame_an_earlier_attempt_wrote_was_kept: false,
        }];

        // **The file changed, and the arrangement did not move**: frames 1 and
        // 2 are found at exactly the paths this attempt predicts, so frame 1 is
        // kept and only frame 2 — the last one asked for that is already there
        // — is rendered again with the rest.
        let mut machine = answers();
        machine.four_values = Some(SourceReading {
            size: four_values().size + 1,
            ..four_values()
        });
        let mut it = ARun::ordinary();
        it.previous_attempts = previous;
        it.reading = AReadingThat::prints(a_printed_reading(
            vec![the_regular_output_path()],
            &[(1, true), (2, true), (3, true)],
        ));
        // The regular output path maps to the job's folder, so the names the
        // inventory predicts are the answered names under it.
        machine.walk = WhatTheWalkOfTheFolderIs::ItCompleted(vec![
            AWalkEntry {
                path_under_the_job_output_folder: b"f_0001.png".to_vec(),
                kind: WhatKindOfEntry::AFile,
                size: 4096,
            },
            AWalkEntry {
                path_under_the_job_output_folder: b"f_0002.png".to_vec(),
                kind: WhatKindOfEntry::AFile,
                size: 4096,
            },
        ]);
        let (went, _) = run_it(&mut machine, &it);
        let mut started = launched(went);
        started.child.kill().ok();
        started.child.wait().ok();
        assert_eq!(machine.wrote.len(), 1);
        assert_eq!(
            machine.wrote[0].frame_list_in_the_order_it_was_written,
            vec![2, 3]
        );
        assert!(machine.wrote[0].the_file_changed_before_this_attempt);
        assert!(machine.wrote[0].a_frame_an_earlier_attempt_wrote_was_kept);
    }

    /// **A regular output path that writes a video** (`Job files and outputs`
    /// I19) — the Creator's `_preview` job of the eleventh notes, in small.
    fn a_video_output_path() -> Destination {
        let mut video = the_regular_output_path();
        video.recorded_location = Some(
            smoozy_job_files_machine::reading::RecordedLocation::of(b"//out/_preview".to_vec()),
        );
        video.format = Some(Format {
            as_the_file_records_it: text("FFMPEG"),
            the_ending_it_writes: Some(text(".mp4")),
            is_video_shaped: true,
        });
        video
    }

    fn a_video_job() -> ARun {
        let mut it = ARun::ordinary();
        it.reading = AReadingThat::prints(a_printed_reading(
            vec![a_video_output_path(), a_node_destination("Beauty", "Image", "//nodes/beauty/")],
            &[(1, true), (2, true), (3, true)],
        ));
        it
    }

    /// **`Rendering` L17a, the Creator's eleventh notes, note 1: the video
    /// shape the map in force carries is handed to the line**, and the render
    /// is started as the job's first frame, then its last, then its step (the
    /// video-range piece), then the animation instruction — never as the frame list that
    /// wrote a one-frame video. The attempt record still carries the list,
    /// whole.
    #[test]
    fn a_video_job_is_launched_as_its_first_frame_its_last_and_the_animation() {
        let mut machine = answers();
        let it = a_video_job();
        let (went, words) = run_it(&mut machine, &it);
        let mut started = launched(went);
        started.child.kill().ok();
        started.child.wait().ok();
        assert!(!words.contains(&"--render-frame".to_string()), "{words:?}");
        let at = words.iter().position(|word| word == "-s").expect("{words:?}");
        assert_eq!(&words[at..at + 7], &["-s", "1", "-e", "3", "-j", "1", "-a"]);
        let file = words
            .iter()
            .position(|word| word == "/ephemeral/smoozy/assets/kitchen.blend")
            .unwrap();
        assert!(file < at, "after the file");
        // **Right after the material, and nothing else of the instruction
        // stands before it** — the start and end placed after the animation
        // instruction were ignored at the desk.
        assert_eq!(words[at - 2], "--python", "{words:?}");
        assert_eq!(words.iter().filter(|word| *word == "-a").count(), 1);
        assert_eq!(
            machine.wrote[0].frame_list_in_the_order_it_was_written,
            vec![1, 2, 3]
        );
    }

    /// **`Job files and outputs` C6 on a re-run of a video job**: an earlier
    /// attempt's list and a video already in the folder change nothing — the
    /// reckoning does not run, the whole range is rendered again as one run,
    /// and the video is replaced (`Job files and outputs` I22) rather than carried on from.
    #[test]
    fn a_re_run_of_a_video_job_renders_the_whole_range_again_as_one_run() {
        use smoozy_job_files_machine::reckoning::WhatKindOfEntry;
        let mut machine = answers();
        let mut it = a_video_job();
        it.previous_attempts = vec![AttemptRecord {
            number: 1,
            frame_list_in_the_order_it_was_written: vec![1, 2, 3],
            map_in_force: TheMapInForce::NoneWasComposedForThisAttempt,
            file_identity_at_this_attempt: four_values(),
            the_file_changed_before_this_attempt: false,
            a_frame_an_earlier_attempt_wrote_was_kept: false,
        }];
        machine.walk = WhatTheWalkOfTheFolderIs::ItCompleted(vec![AWalkEntry {
            path_under_the_job_output_folder: b"_preview0001-0003.mp4".to_vec(),
            kind: WhatKindOfEntry::AFile,
            size: 12_000,
        }]);
        let (went, words) = run_it(&mut machine, &it);
        let mut started = launched(went);
        started.child.kill().ok();
        started.child.wait().ok();
        let at = words.iter().position(|word| word == "-s").expect("{words:?}");
        assert_eq!(&words[at..at + 7], &["-s", "1", "-e", "3", "-j", "1", "-a"]);
        assert_eq!(
            machine.wrote[0].frame_list_in_the_order_it_was_written,
            vec![1, 2, 3]
        );
        assert!(!machine.wrote[0].a_frame_an_earlier_attempt_wrote_was_kept);
    }

    /// **L17a: a video job whose range arrives with a gap is refused in
    /// words before anything is launched or recorded** — never rendered as
    /// some other run.
    #[test]
    fn a_video_job_whose_range_has_a_gap_ends_before_the_launch_in_words() {
        let mut machine = answers();
        let mut it = a_video_job();
        it.frame_range = FrameRange::TheUsersParsedIntegers(vec![1, 2, 5]);
        it.reading = AReadingThat::prints(a_printed_reading(
            vec![a_video_output_path()],
            &[(1, true), (2, true), (5, true)],
        ));
        let (went, words) = run_it(&mut machine, &it);
        match went.ended {
            WhereTheAttemptEnded::TheAttemptCouldNotBeComposed { what_happened, .. } => {
                assert_eq!(
                    what_happened,
                    "this job writes a video, and its frames have gaps or are out of order"
                );
            }
            other => panic!("{other:?}"),
        }
        assert!(words.is_empty(), "nothing was launched: {words:?}");
        assert!(machine.wrote.is_empty(), "nothing was recorded");
    }

    /// **What a reading of the job's file prints, its scene saved at the range
    /// and step given** — the settings half the rest of these tests use, with
    /// the job's scene moved.
    fn a_printed_reading_saved_at(
        destinations: Vec<Destination>,
        frames: &[(i64, bool)],
        (start, end, step): (i64, i64, i64),
    ) -> ThePrintedReading {
        let mut printed = a_printed_reading(destinations, frames);
        let mut settings = crate::material::tests::a_reading().settings;
        for scene in &mut settings.scenes {
            scene.frame_start = start;
            scene.frame_end = end;
            scene.frame_step = step;
        }
        printed.settings = smoozy_job_files_machine::manifest::CarriedAndNotInterpretedHere(
            serde_json::to_value(settings).expect("the settings half renders"),
        );
        printed
    }

    /// **`Rendering` L17a's step, the video-range piece: a video job asking for the file's
    /// own range keeps the file's own step** — the reading in force says the
    /// scene was saved at 1 to 10 every second frame, and the render is
    /// started as `-s 1 -e 9 -j 2 -a`. Until the video-range piece it was refused.
    #[test]
    fn a_video_job_of_the_files_own_stepped_range_keeps_the_files_step() {
        let mut machine = answers();
        let mut it = a_video_job();
        it.frame_range = FrameRange::TheFilesOwn;
        it.reading = AReadingThat::prints(a_printed_reading_saved_at(
            vec![a_video_output_path()],
            &[(1, true), (3, true), (5, true), (7, true), (9, true)],
            (1, 10, 2),
        ));
        let (went, words) = run_it(&mut machine, &it);
        let mut started = launched(went);
        started.child.kill().ok();
        started.child.wait().ok();
        let at = words.iter().position(|word| word == "-s").expect("{words:?}");
        assert_eq!(&words[at..at + 7], &["-s", "1", "-e", "9", "-j", "2", "-a"]);
        assert_eq!(
            machine.wrote[0].frame_list_in_the_order_it_was_written,
            vec![1, 3, 5, 7, 9]
        );
    }

    /// **L17b, the video-range piece: frames below zero are written from the
    /// scene's saved start as the reading in force recorded it** — a picture
    /// job of −3 to 0 on a file saved at −5 to 5 is launched with `+2..+4,0`,
    /// never the `-3..0` that rendered nothing and reported success; a video
    /// job of the same frames with `-s +2 -e 0`.
    #[test]
    fn frames_below_zero_are_launched_from_the_saved_start_the_reading_recorded() {
        let below = [(-3, true), (-2, true), (-1, true), (0, true)];
        let mut machine = answers();
        let mut it = ARun::ordinary();
        it.frame_range = FrameRange::TheUsersParsedIntegers(vec![-3, -2, -1, 0]);
        it.reading = AReadingThat::prints(a_printed_reading_saved_at(
            vec![the_regular_output_path()],
            &below,
            (-5, 5, 1),
        ));
        let (went, words) = run_it(&mut machine, &it);
        let mut started = launched(went);
        started.child.kill().ok();
        started.child.wait().ok();
        let at = words.iter().position(|word| word == "--render-frame").expect("{words:?}");
        assert_eq!(words[at + 1], "+2..+4,0");

        let mut machine = answers();
        let mut it = a_video_job();
        it.frame_range = FrameRange::TheUsersParsedIntegers(vec![-3, -2, -1, 0]);
        it.reading = AReadingThat::prints(a_printed_reading_saved_at(
            vec![a_video_output_path()],
            &below,
            (-5, 5, 1),
        ));
        let (went, words) = run_it(&mut machine, &it);
        let mut started = launched(went);
        started.child.kill().ok();
        started.child.wait().ok();
        let at = words.iter().position(|word| word == "-s").expect("{words:?}");
        assert_eq!(&words[at..at + 7], &["-s", "+2", "-e", "0", "-j", "1", "-a"]);
        assert_eq!(
            machine.wrote[0].frame_list_in_the_order_it_was_written,
            vec![-3, -2, -1, 0]
        );
    }

    /// **L30: a remap material that cannot be read must not render.** The
    /// attempt ends at step 2, naming the material, and nothing is composed.
    #[test]
    fn an_unreadable_remap_material_ends_the_attempt_at_step_two_naming_it() {
        let mut machine = answers();
        machine.remap = None;
        let it = ARun::ordinary();
        let (went, _) = run_it(&mut machine, &it);
        let WhereTheAttemptEnded::FailedBeforeTheRenderInstruction { at, reason } = went.ended
        else {
            panic!("{:?}", went.ended);
        };
        assert_eq!(at.number, 2);
        assert!(matches!(
            reason,
            BeforeTheRenderInstruction::TheRemapMaterialCouldNotBeRead { .. }
        ));
        assert!(machine.wrote.is_empty(), "nothing was written");
        assert_eq!(machine.recorded_pairs, 0);
        assert!(
            it.reading.handed.borrow().is_empty(),
            "the file is not even read"
        );
    }

    /// **L14: a Blender that names another version fails the attempt before
    /// the render instruction, and a silence is never read as a match** —
    /// **and it is refused before the file is handed to it** (R2).
    #[test]
    fn a_version_that_does_not_match_and_a_silence_both_end_the_attempt_at_step_four() {
        for names in [
            Some(Version {
                major: 4,
                minor: 5,
                patch: 12,
            }),
            None,
        ] {
            let mut machine = answers();
            let mut it = ARun::ordinary();
            it.names = names;
            let (went, _) = run_it(&mut machine, &it);
            let WhereTheAttemptEnded::FailedBeforeTheRenderInstruction { at, reason } = went.ended
            else {
                panic!("{:?}", went.ended);
            };
            assert_eq!(at.number, 4);
            assert!(matches!(
                reason,
                BeforeTheRenderInstruction::TheVersionConfirmationRefused { .. }
            ));
            assert_eq!(machine.recorded_pairs, 0);
            assert!(
                it.reading.handed.borrow().is_empty(),
                "R2: a binary of another version never reads the artist's file"
            );
        }
    }

    /// **A reading that did not end successfully is not a reading** — C9's
    /// rule, held of P1 step 4's reading too — and the attempt fails rather
    /// than composing against nothing, **naming that reading** (`Rendering`
    /// K14's step-4 occasion).
    #[test]
    fn a_reading_that_did_not_complete_ends_the_attempt_at_step_four() {
        let mut machine = answers();
        let mut it = ARun::ordinary();
        it.reading = AReadingThat::answers_nothing();
        let (went, _) = run_it(&mut machine, &it);
        let WhereTheAttemptEnded::FailedBeforeTheRenderInstruction { at, reason } = went.ended
        else {
            panic!("{:?}", went.ended);
        };
        assert_eq!(at.number, 4);
        // **The ending names the reading that failed**, which is step 4's
        // reading of the job's `.blend` and never the output folder's
        // (`Rendering` K14's step-4 occasion). Until 2026-09-29 this was C9's.
        assert_eq!(
            reason,
            BeforeTheRenderInstruction::TheJobsFileReadingDidNotEndSuccessfully
        );
        assert!(
            !reason.in_words().contains("output folder"),
            "the step-4 reading's failure blames the output folder: {}",
            reason.in_words()
        );
        assert!(
            went.inventory.is_none(),
            "nothing counts against an inventory that was never composed"
        );
    }

    /// **`Render Settings` Q7: a document whose error map is not empty is no
    /// reading**, and that is a failed attempt at step 4 on the step-4
    /// reading's own occasion (`Rendering` K14).
    #[test]
    fn a_reading_carrying_a_complaint_ends_the_attempt_at_step_four() {
        let mut machine = answers();
        let mut it = ARun::ordinary();
        let mut printed = an_ordinary_reading();
        printed.errors.insert(
            "the regular output path at frame 1".to_string(),
            "Unknown variable referenced in template expression".to_string(),
        );
        it.reading = AReadingThat::prints(printed);
        let (went, _) = run_it(&mut machine, &it);
        let WhereTheAttemptEnded::FailedBeforeTheRenderInstruction { at, reason } = went.ended
        else {
            panic!("{:?}", went.ended);
        };
        assert_eq!(at.number, 4);
        assert_eq!(
            reason,
            BeforeTheRenderInstruction::TheJobsFileReadingDidNotEndSuccessfully
        );
        assert!(machine.readings_written.is_empty());
    }

    /// **P3: where the supervisor is not answering, no hold is placed at
    /// all** — the attempt ends as an interrupted one and no identity is
    /// composed here.
    #[test]
    fn no_hold_is_placed_where_the_supervisor_is_not_answering() {
        let mut machine = answers();
        machine.hold = None;
        let mut it = ARun::ordinary();
        it.reading = AReadingThat::prints(a_printed_reading(
            vec![the_regular_output_path()],
            &[(1, false)],
        ));
        let (went, _) = run_it(&mut machine, &it);
        assert!(matches!(
            went.ended,
            WhereTheAttemptEnded::NoHoldWasPlacedAndReconciliationTakesIt
        ));
    }

    /// **`Job files and outputs` C9 again, from the other side: a walk that
    /// did not complete is a failed attempt and never a whole range.**
    #[test]
    fn a_walk_that_did_not_complete_ends_the_attempt_at_step_six() {
        let mut machine = answers();
        machine.walk = WhatTheWalkOfTheFolderIs::ItDidNotComplete;
        let it = ARun::ordinary();
        let (went, _) = run_it(&mut machine, &it);
        let WhereTheAttemptEnded::TheWalkDidNotComplete { at } = went.ended else {
            panic!("{:?}", went.ended);
        };
        assert_eq!(at.number, 6);
        assert_eq!(machine.recorded_pairs, 0);
    }

    /// **The attempt record is written before the render starts**, and a
    /// refused write ends the attempt rather than launching.
    #[test]
    fn a_refused_attempt_record_ends_the_attempt_before_the_launch() {
        let mut machine = answers();
        machine.record_refuses = Some("No space left on device".to_string());
        let it = ARun::ordinary();
        let (went, _) = run_it(&mut machine, &it);
        let WhereTheAttemptEnded::TheAttemptRecordCouldNotBeWritten { at, why } = went.ended else {
            panic!("{:?}", went.ended);
        };
        assert_eq!(at.number, 7);
        assert_eq!(why, "No space left on device");
        assert_eq!(machine.recorded_pairs, 0, "no render was started");
    }

    /// **A render whose recorded pair cannot be written is ended rather than
    /// left running** — the one state no stop could ever reach.
    #[test]
    fn a_render_whose_pair_cannot_be_recorded_is_ended_rather_than_left_running() {
        let mut machine = answers();
        machine.pair_refuses = Some("No space left on device".to_string());
        let it = ARun::ordinary();
        let (went, _) = run_it(&mut machine, &it);
        let WhereTheAttemptEnded::TheRecordedPairCouldNotBeWritten { at, why } = went.ended else {
            panic!("{:?}", went.ended);
        };
        assert_eq!(at.number, 7);
        assert_eq!(why, "No space left on device");
    }

    /// **A build that cannot be run ends the attempt rather than composing
    /// its way past it.**
    #[test]
    fn a_build_that_cannot_be_run_ends_the_attempt_at_step_seven() {
        let mut machine = answers();
        let mut it = ARun::ordinary();
        it.build = "/no/such/blender".to_string();
        let (went, _) = run_it(&mut machine, &it);
        let WhereTheAttemptEnded::TheLaunchCouldNotBeMade { at, why } = went.ended else {
            panic!("{:?}", went.ended);
        };
        assert_eq!(at.number, 7);
        assert!(why.contains("/no/such/blender"));
    }

    // --- `Dependency Upload & Remap` M22c: the uploaded bake copied into the
    // job folder before the render (step 10c's dependency wave piece 8) ---

    /// The scene's bake output path, recorded as the artist left it.
    fn a_bake_output_path(recorded: &str) -> Destination {
        use smoozy_job_files_machine::reading::AnAddedWritePlace;
        Destination {
            kind: DestinationKind::AnAddedWritePlace(AnAddedWritePlace::TheBakeOutputPath),
            recorded_location: Some(smoozy_job_files_machine::reading::RecordedLocation::of(
                recorded.as_bytes().to_vec(),
            )),
            format: None,
            origin: WhereItComesFrom::AnAddedWritePlaceNamed(AnAddedWritePlace::TheBakeOutputPath),
            scene: WhichScene::TheJobsOwnScene,
            the_view_layers_that_feed_it: Vec::new(),
            the_location_with_its_folder_replaced: None,
        }
    }

    /// **A run whose file records `//bake/` as its bake output path**, and a
    /// machine whose remap material says the bake recorded as `owed` was sent.
    fn a_run_with_a_bake_sent(owed: &str) -> (TheMachineAnswers, ARun) {
        use smoozy_dependencies_machine::placed_copies::TheCopyTheMachineOwes;
        use smoozy_job_files_machine::reading::AnAddedWritePlace;
        let mut machine = answers();
        let remap = machine.remap.take().expect("a material").with_the_copies_owed(vec![
            TheCopyTheMachineOwes {
                from: smoozy_dependencies_machine::bytes::MachineLocation::composed(
                    b"/ephemeral/smoozy/assets/files/home/artist/bake".to_vec(),
                ),
                place: AnAddedWritePlace::TheBakeOutputPath,
                recorded: smoozy_dependencies_machine::bytes::RecordedLocation::as_the_reading_carried_it(
                    owed.as_bytes(),
                ),
            },
        ]);
        machine.remap = Some(remap);
        let mut it = ARun::ordinary();
        it.reading = AReadingThat::prints(a_printed_reading(
            vec![
                the_regular_output_path(),
                a_node_destination("Beauty", "Image", "//nodes/beauty/"),
                a_bake_output_path("//bake/"),
            ],
            &[(1, true), (2, true), (3, true)],
        ));
        (machine, it)
    }

    /// **`Dependency Upload & Remap` M22c: the bake the check sent is copied
    /// to the folder the map in force gives the bake output path, before the
    /// launch material, the attempt record and the render** — and the placed
    /// file's four values are written down (`Dependency Upload & Remap` M22f).
    #[test]
    fn an_uploaded_bake_is_copied_to_its_places_folder_before_the_render_is_launched() {
        let (mut machine, it) = a_run_with_a_bake_sent("//bake/");
        let (went, _) = run_it(&mut machine, &it);
        launched(went);
        assert_eq!(
            machine.copies,
            vec![(
                b"/ephemeral/smoozy/assets/files/home/artist/bake".to_vec(),
                b"/ephemeral/smoozy/jobs/j1/bake".to_vec()
            )],
            "the copy goes from the mirror to the bake output path's own folder under the job's"
        );
        assert_eq!(machine.order, vec!["copy", "placed-copies", "attempt record"]);
        assert_eq!(machine.placed_written.len(), 1);
        assert_eq!(
            machine.placed_written[0].files[0].job_folder_path.as_bytes(),
            b"/ephemeral/smoozy/jobs/j1/bake/lightmap.png"
        );
        assert_eq!(machine.placed_written[0].files[0].inode, "4242");
    }

    /// **`Dependency Upload & Remap` M8b: a library recorded relative to the file
    /// has the file opened from a folder where it resolves**, made before the attempt
    /// record and named on the line.
    #[test]
    fn a_library_recorded_relative_to_the_file_has_it_opened_where_it_resolves() {
        let mut machine = answers();
        machine.remap = Some(a_remap(&a_job(), true));
        let it = ARun::ordinary();
        let (went, words) = run_it(&mut machine, &it);
        launched(went);
        assert_eq!(machine.order, vec!["opening", "attempt record"]);
        let folder = &machine.opening_folders[0];
        assert_eq!(folder.root, b"/ephemeral/smoozy/job-records/j1/opening-1");
        let opened = String::from_utf8(folder.the_file.clone()).unwrap();
        assert!(opened.starts_with("/ephemeral/smoozy/job-records/j1/opening-1/"), "{opened}");
        assert!(opened.ends_with(".blend"), "{opened}");
        assert!(words.contains(&opened), "{words:?}");
        assert_eq!(
            words.iter().filter(|word| word.ends_with(".blend")).count(),
            1,
            "the line opens one file: {words:?}"
        );
    }

    /// **No library pair, no folder, and the file opens where it sits.**
    #[test]
    fn a_file_with_no_relative_library_opens_where_it_sits() {
        let mut machine = answers();
        let it = ARun::ordinary();
        let (went, words) = run_it(&mut machine, &it);
        launched(went);
        assert!(machine.opening_folders.is_empty());
        assert!(!words.iter().any(|word| word.contains("opening-")), "{words:?}");
    }

    /// **A folder that cannot be made ends the attempt and says so** — rendering
    /// anyway is the finished wrong picture `Dependency Upload & Remap` M8b exists
    /// to stop.
    #[test]
    fn an_opening_folder_that_cannot_be_made_ends_the_attempt_before_the_render() {
        let mut machine = answers();
        machine.remap = Some(a_remap(&a_job(), true));
        machine.opening_refuses = Some("No space left on device".to_string());
        let it = ARun::ordinary();
        let (went, words) = run_it(&mut machine, &it);
        let WhereTheAttemptEnded::TheAttemptCouldNotBeComposed { at, what_happened } = went.ended else {
            panic!("{:?}", went.ended);
        };
        assert_eq!(at.number, 4, "`Job files and outputs` R1a: it is made before the reading");
        assert!(what_happened.contains("the folder the file opens from"), "{what_happened}");
        assert!(what_happened.contains("No space left on device"), "{what_happened}");
        assert!(words.is_empty(), "no render was started: {words:?}");
        assert!(machine.wrote.is_empty(), "no attempt record was written");
        assert!(it.reading.opened.borrow().is_empty(), "no reading was taken");
    }

    /// **`Job files and outputs` R1a, 2026-11-04: the reading opens the file from the
    /// folder the render opens it from**, so it sees what the render will see. Made
    /// once: the reading and the line name the same link.
    #[test]
    fn the_reading_opens_the_file_from_the_folder_the_render_opens_it_from() {
        let mut machine = answers();
        machine.remap = Some(a_remap(&a_job(), true));
        let it = ARun::ordinary();
        let (went, words) = run_it(&mut machine, &it);
        launched(went);
        assert_eq!(machine.opening_folders.len(), 1, "made once");
        let opened = String::from_utf8(machine.opening_folders[0].the_file.clone()).unwrap();
        assert_eq!(*it.reading.opened.borrow(), vec![opened.clone()]);
        assert!(words.contains(&opened), "{words:?}");
    }

    /// **The file with no relative library costs nothing new**: no folder, and the
    /// reading opens it where it sits, as it always has.
    #[test]
    fn a_file_with_no_relative_library_is_read_where_it_sits() {
        let mut machine = answers();
        let it = ARun::ordinary();
        let (went, _) = run_it(&mut machine, &it);
        launched(went);
        assert!(machine.opening_folders.is_empty());
        let opened = it.reading.opened.borrow();
        assert_eq!(opened.len(), 1);
        assert!(!opened[0].contains("opening-"), "{opened:?}");
    }

    /// **A version refused costs no folder**: L14 stands first, so a binary that
    /// names another version is refused before anything is made for it.
    #[test]
    fn a_version_refused_makes_no_opening_folder() {
        let mut machine = answers();
        machine.remap = Some(a_remap(&a_job(), true));
        let mut it = ARun::ordinary();
        it.names = None;
        let (went, _) = run_it(&mut machine, &it);
        assert!(!matches!(went.ended, WhereTheAttemptEnded::TheRenderWasLaunched(_)));
        assert!(machine.opening_folders.is_empty());
    }

    /// **`Job files and outputs` D21b-i, added 2026-10-02: a baked physics
    /// cache the reading found on the machine is copied to the point cache's
    /// folder for its recorded location before the render, and recorded with
    /// the sent copies** — so `Download` N5a skips it. At the desk a baked cloth
    /// uploaded beside the `.blend` was never read until it was: every frame
    /// but the first was wrong, and after the copy all 24 matched the plain
    /// render, in 4.5.12 and 5.2.2.
    #[test]
    fn a_baked_cache_found_on_the_machine_is_copied_to_its_folder_before_the_render() {
        use smoozy_job_files_machine::reading::{ABakedCacheOnTheMachine, AnAddedWritePlace, RecordedLocation};
        let mut machine = answers();
        let mut it = ARun::ordinary();
        let mut printed = a_printed_reading(
            vec![
                the_regular_output_path(),
                a_node_destination("Beauty", "Image", "//nodes/beauty/"),
                Destination {
                    kind: DestinationKind::AnAddedWritePlace(AnAddedWritePlace::ThePointCache),
                    recorded_location: None,
                    format: None,
                    origin: WhereItComesFrom::AnAddedWritePlaceNamed(AnAddedWritePlace::ThePointCache),
                    scene: WhichScene::TheJobsOwnScene,
                    the_view_layers_that_feed_it: Vec::new(),
                    the_location_with_its_folder_replaced: None,
                },
            ],
            &[(1, true), (2, true), (3, true)],
        );
        printed.the_baked_caches_on_the_machine = vec![ABakedCacheOnTheMachine {
            recorded_location: None,
            its_files_sit_in: RecordedLocation::of(
                b"/ephemeral/smoozy/assets/upload/kitchen/blendcache_kitchen".to_vec(),
            ),
        }];
        it.reading = AReadingThat::prints(printed);
        let (went, _) = run_it(&mut machine, &it);
        launched(went);
        assert_eq!(machine.copies.len(), 1, "{:?}", machine.copies);
        assert_eq!(
            machine.copies[0].0,
            b"/ephemeral/smoozy/assets/upload/kitchen/blendcache_kitchen".to_vec()
        );
        assert!(
            machine.copies[0].1.starts_with(b"/ephemeral/smoozy/jobs/j1/"),
            "the copy goes under the job's folder"
        );
        assert_eq!(machine.order, vec!["copy", "placed-copies", "attempt record"]);
        assert_eq!(machine.placed_written.len(), 1);
    }

    /// **No bake sent, no copy and no list** — `Dependency Upload & Remap`
    /// M22b: *where no bake was sent, nothing changes at all.*
    #[test]
    fn a_job_that_sent_no_bake_copies_nothing() {
        let mut machine = answers();
        let it = ARun::ordinary();
        let (went, _) = run_it(&mut machine, &it);
        launched(went);
        assert!(machine.copies.is_empty());
        assert!(machine.placed_written.is_empty());
    }

    /// **A copy that fails ends the attempt before the render and names the
    /// copy** — never a render that goes on without the bake the user sent.
    #[test]
    fn a_copy_that_fails_ends_the_attempt_before_the_render_naming_the_copy() {
        let (mut machine, it) = a_run_with_a_bake_sent("//bake/");
        machine.copy_refuses = Some("No space left on device".to_string());
        let (went, words) = run_it(&mut machine, &it);
        let WhereTheAttemptEnded::FailedBeforeTheRenderInstruction {
            at,
            reason: BeforeTheRenderInstruction::AnUploadedBakeCouldNotBeCopied { copy, why },
        } = went.ended
        else {
            panic!("{:?}", went.ended);
        };
        assert_eq!(at.number, 7);
        assert!(copy.contains("bake output folder"), "{copy}");
        assert!(copy.contains("//bake/"), "named as the file records it: {copy}");
        assert!(!copy.contains("/ephemeral"), "never Smoozy's own copy of it: {copy}");
        assert_eq!(why, "No space left on device");
        assert!(words.is_empty(), "no render was started: {words:?}");
        assert!(machine.wrote.is_empty(), "no attempt record was written");
        assert!(machine.placed_written.is_empty());
    }

    /// **A bake whose write place the map in force gives no folder ends the
    /// attempt the same way** — the file changed after the check, and a bake
    /// placed nowhere is a bake the render never reads.
    #[test]
    fn a_bake_the_map_in_force_has_no_place_for_ends_the_attempt_naming_it() {
        let (mut machine, it) = a_run_with_a_bake_sent("//an-older-bake/");
        let (went, words) = run_it(&mut machine, &it);
        let WhereTheAttemptEnded::FailedBeforeTheRenderInstruction {
            at,
            reason: BeforeTheRenderInstruction::AnUploadedBakeCouldNotBeCopied { copy, why },
        } = went.ended
        else {
            panic!("{:?}", went.ended);
        };
        assert_eq!(at.number, 7);
        assert!(copy.contains("//an-older-bake/"), "{copy}");
        assert!(why.is_empty(), "nothing further a user can use: {why}");
        assert!(machine.copies.is_empty());
        assert!(words.is_empty(), "no render was started: {words:?}");
    }

    /// **`Queue & History` B1's total, worked out by the driver from the
    /// reading it took at step 4** — the Creator's notes on 0.1.8, piece 6a.
    /// The reading's scene renders its one view layer at 64 samples and
    /// 1920×1080, which `Queue & History` B2's grid over the assumed 2048 counts
    /// as one tile; the job asks for three frames.
    ///
    /// **What it bites on**: no total at all (the state before this piece), a
    /// total that ignores the typed count or the typed size, and one that
    /// ignores a pin.
    #[test]
    fn the_plan_total_is_worked_out_from_the_reading_the_count_the_size_and_the_pins() {
        use smoozy_render_settings_machine::names::{AnchorState, ExactName};
        use smoozy_render_settings_machine::view_layers::PinEntry;
        let mut machine = answers();
        let (went, _) = run_it(&mut machine, &ARun::ordinary());
        assert_eq!(went.the_plan_total(), Some(3 * 64), "three frames, one layer, 64 samples, one tile");

        // **A typed count stands for every layer, and a typed percentage sets
        // the size**: 200 per cent of 1920×1080 is 3840×2160, two tiles each
        // way (`Render Settings` C6, `Render Settings` Q2a).
        let mut typed = ARun::ordinary();
        typed.payload.sample_count = Some(256);
        typed.payload.resolution_percentage = Some(200);
        let (went, _) = run_it(&mut machine, &typed);
        assert_eq!(went.the_plan_total(), Some(3 * 256 * 4));

        // **A layer pinned off renders at no frame** (`Render Settings` V4).
        let mut pinned = ARun::ordinary();
        pinned.payload.view_layer_pins.push(PinEntry {
            layer: ExactName::as_the_reading_produced_it("ViewLayer"),
            anchor: AnchorState::Anchored {
                the_reading_it_came_from: crate::material::tests::a_reading().identity(),
            },
            on: false,
        });
        let (went, _) = run_it(&mut machine, &pinned);
        assert_eq!(went.the_plan_total(), Some(0));

        // **No reading stood, so no total is worked out** — never a nought.
        let mut unread = ARun::ordinary();
        unread.reading = AReadingThat::answers_nothing();
        let (went, _) = run_it(&mut machine, &unread);
        assert_eq!(went.the_plan_total(), None);
    }

    /// **The eleventh notes, the video-range piece, at the desk: a real Blender renders the
    /// step and frames below zero through the real line, with the real
    /// material and the real script before the file** — kept in the tree and
    /// ignored. Run by hand:
    ///
    /// ```text
    /// SMOOZY_DESK_BLENDER=/Applications/Blender\ 5.1.2.app/Contents/MacOS/Blender \
    /// SMOOZY_DESK_VERSION=5.1.2 \
    /// SMOOZY_DESK_FILES=<a folder holding the four files below> \
    /// SMOOZY_DESK_OUT=<an empty folder> \
    /// cargo test -p smoozy-rendering-machine -- --ignored --nocapture a_real_blender_renders_the_step
    /// ```
    ///
    /// **What runs is this driver, whole**: the real reading of the file (by
    /// [`TheReadingOnTheMachine`]), everything composed from it, the script
    /// before the file and the launch material placed and read back on disk,
    /// and the render launched with the line's own words. Only the machine's
    /// own records are this test's stand-ins. **The four Blenders at the desk
    /// are the Creator's**, so the build path is a two-line wrapper adding the
    /// factory-startup switch at the head, as the desk's other real-Blender
    /// tests do; the machine launches without it (`Job files and outputs` R7).
    ///
    /// **What it proves**: that the material leaves the scene's start and end
    /// where the reading found them, so a frame written from the saved range
    /// lands on the frame asked for — a picture job of −3 to 0 on a file saved
    /// at −5 to 5 writes exactly −3, −2, −1 and 0, and a video job of the same
    /// writes four frames — and that a video job keeps its step.
    ///
    /// The files, each Workbench at 64 × 64, saved and widened the way the
    /// second desk test made its own: `png-m5-5.blend` (PNG, −5 to 5),
    /// `video-m5-5.blend` (MPEG-4 H.264, −5 to 5), `video-m5-m1.blend` (the
    /// same, −5 to −1) and `video-1-10-step2.blend` (the same, 1 to 10 at step
    /// 2).
    #[test]
    #[ignore = "run by hand at the desk with a real Blender and real .blend files"]
    fn a_real_blender_renders_the_step_and_frames_below_zero_through_the_real_line() {
        use crate::acting::material::TheWorkersOwnFilesystem;
        use crate::acting::reading::TheReadingOnTheMachine;
        use std::os::unix::fs::PermissionsExt;
        use std::path::{Path, PathBuf};

        const ONE_RUN_AT_MOST: std::time::Duration = std::time::Duration::from_secs(300);
        let blender = std::env::var("SMOOZY_DESK_BLENDER").expect("SMOOZY_DESK_BLENDER");
        let files = PathBuf::from(std::env::var("SMOOZY_DESK_FILES").expect("SMOOZY_DESK_FILES"));
        let out = PathBuf::from(std::env::var("SMOOZY_DESK_OUT").expect("SMOOZY_DESK_OUT"));
        let version_text = std::env::var("SMOOZY_DESK_VERSION").unwrap_or_else(|_| "5.1.2".to_string());
        let version = Version::parse(&version_text).expect("a version");

        // (case, file, the frames asked, what Blender should write)
        let users = |frames: &[i64]| FrameRange::TheUsersParsedIntegers(frames.to_vec());
        let cases: Vec<(&str, &str, FrameRange, Vec<i64>)> = vec![
            ("png-m3-0", "png-m5-5.blend", users(&[-3, -2, -1, 0]), vec![-3, -2, -1, 0]),
            ("png-own", "png-m5-5.blend", FrameRange::TheFilesOwn, (-5..=5).collect()),
            ("video-m3-0", "video-m5-5.blend", users(&[-3, -2, -1, 0]), vec![-3, -2, -1, 0]),
            ("video-m4-m2", "video-m5-5.blend", users(&[-4, -3, -2]), vec![-4, -3, -2]),
            ("video-own-m5-m1", "video-m5-m1.blend", FrameRange::TheFilesOwn, (-5..=-1).collect()),
            ("video-1-10-typed", "video-1-10-step2.blend", users(&(1..=10).collect::<Vec<i64>>()), (1..=10).collect()),
            ("video-own-step2", "video-1-10-step2.blend", FrameRange::TheFilesOwn, vec![1, 3, 5, 7, 9]),
        ];
        let only = std::env::var("SMOOZY_DESK_ONLY").ok();
        let mut faults = Vec::new();
        for (case, file, frame_range, expected) in cases {
            if only.as_deref().is_some_and(|only| only != case) {
                continue;
            }
            let root = out.join(case);
            std::fs::create_dir_all(&root).expect("the case's folder");
            let wrapper = root.join("blender-with-factory-startup");
            std::fs::write(&wrapper, format!("#!/bin/sh\nexec '{blender}' --factory-startup \"$@\"\n"))
                .expect("the wrapper");
            std::fs::set_permissions(&wrapper, std::fs::Permissions::from_mode(0o755)).expect("runnable");
            let blend = root.join(file);
            std::fs::copy(files.join(file), &blend).expect("the file is put in place");
            let job_folder = root.join("job");

            let job = a_job();
            let mut manifest = a_manifest(&job, &version_text, "Scene");
            manifest.frame_range = frame_range;
            manifest.blend_path_on_the_machine = blend.display().to_string();
            let payload = crate::material::tests::an_untouched_payload();
            let mut machine = answers();
            machine.remap = Some(RemapMaterial::composed(&job, &crate::material::tests::a_blender(), Vec::new()));
            let clock = the_machines_clock();
            let start = Watched {
                start: TheRenderStartedOnTheMachine {
                    kernel: AKernelThat(Some(19186)),
                },
                words: std::cell::RefCell::new(Vec::new()),
            };
            let loader = ALoaderThatCounts {
                answers: WhatTheLoaderFound::TheLibrary,
                asked: std::cell::Cell::new(0),
            };
            let at = |name: &str| root.join(name).display().to_string();
            let went = {
                let mut pipeline = TheRenderWorkersPipeline {
                    start: &start,
                    places: &TheWorkersOwnFilesystem,
                    asks_the_version: &NamesItself(Some(version)),
                    takes_the_reading: &TheReadingOnTheMachine,
                    steps: &mut machine,
                    looks_for_the_optix_library: &loader,
                };
                pipeline.run(TheAttempt {
                    job: &job,
                    manifest: &manifest,
                    payload: &payload,
                    the_manifests_version: version,
                    the_remap_materials_path: &at("remap"),
                    the_blender_build_path: &wrapper.display().to_string(),
                    the_launch_materials_path: &at("material.py"),
                    the_load_time_scripts_path: &at("load-time.py"),
                    the_readings_script_path: &at("reading.py"),
                    the_job_output_folder: TheJobOutputFolderForThisAttempt::Known(
                        &job_folder.display().to_string(),
                    ),
                    the_folder_the_blend_sits_in: root.display().to_string().as_bytes(),
                    previous_attempts: &[],
                    number: 1,
                    the_machines_clock: &clock,
                    the_manifests_reading: None,
                    the_manifests_settings_half: None,
                })
            };
            let words = start.words.borrow().clone();
            let the_frame_words: Vec<String> = match words.iter().position(|word| word.ends_with("material.py")) {
                Some(at) => words[at + 1..].to_vec(),
                None => Vec::new(),
            };
            let WhereTheAttemptEnded::TheRenderWasLaunched(mut started) = went.ended else {
                faults.push(format!("{case}: not launched: {:?}", went.ended));
                continue;
            };
            let take = |mut from: Box<dyn std::io::Read + Send>| {
                std::thread::spawn(move || {
                    let mut all = Vec::new();
                    from.read_to_end(&mut all).ok();
                    all
                })
            };
            let ordinary = take(Box::new(started.ordinary));
            let complaints = take(Box::new(started.complaints));
            let began = std::time::Instant::now();
            let status = loop {
                if let Some(status) = started.child.try_wait().expect("it can be asked") {
                    break Some(status);
                }
                if began.elapsed() > ONE_RUN_AT_MOST {
                    started.child.kill().ok();
                    started.child.wait().ok();
                    break None;
                }
                std::thread::sleep(std::time::Duration::from_millis(100));
            };
            let ordinary = ordinary.join().expect("the reader ends");
            let complaints = complaints.join().expect("the reader ends");
            std::fs::write(root.join("render.log"), &ordinary).ok();
            std::fs::write(root.join("render-complaints.log"), &complaints).ok();
            std::fs::write(root.join("words.txt"), words.join("\n")).ok();

            // **What Blender wrote**, read off the job folder: each picture's
            // frame from its name, each video's frames counted by ffprobe, and
            // the frames the log says were appended to it.
            fn every_file_under(folder: &Path) -> Vec<PathBuf> {
                let mut found = Vec::new();
                for entry in std::fs::read_dir(folder).into_iter().flatten().flatten() {
                    let path = entry.path();
                    if path.is_dir() {
                        found.extend(every_file_under(&path));
                    } else {
                        found.push(path);
                    }
                }
                found.sort();
                found
            }
            let written = every_file_under(&job_folder);
            let log = String::from_utf8_lossy(&ordinary).to_string();
            let a_number_after = |line: &str, mark: &str| -> Option<i64> {
                let rest = &line[line.find(mark)? + mark.len()..];
                let number: String = rest
                    .trim_start()
                    .chars()
                    .enumerate()
                    .take_while(|(at, c)| c.is_ascii_digit() || (*at == 0 && *c == '-'))
                    .map(|(_, c)| c)
                    .collect();
                number.parse().ok()
            };
            let appended: Vec<i64> = log
                .lines()
                .filter_map(|line| a_number_after(line, "append frame"))
                .collect();
            let range_line = log
                .lines()
                .find(|line| line.contains("Rendering animation"))
                .unwrap_or("")
                .trim()
                .to_string();
            let mut pictures: Vec<i64> = Vec::new();
            let mut videos = Vec::new();
            for path in &written {
                let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("").to_string();
                // `p_-0003.png` reads back as −3, `p_0004.png` as 4.
                if let Some(stem) = name.strip_suffix(".png") {
                    let number = stem.rsplit_once('_').map(|(_, number)| number).unwrap_or(stem);
                    match number.parse::<i64>() {
                        Ok(frame) => pictures.push(frame),
                        Err(_) => faults.push(format!("{case}: a picture named {name}")),
                    }
                }
                if name.ends_with(".mp4") {
                    let probed = std::process::Command::new("/opt/homebrew/bin/ffprobe")
                        .args([
                            "-v",
                            "error",
                            "-count_frames",
                            "-select_streams",
                            "v:0",
                            "-show_entries",
                            "stream=nb_read_frames",
                            "-of",
                            "csv=p=0",
                        ])
                        .arg(path)
                        .output()
                        .expect("ffprobe runs");
                    let counted = String::from_utf8_lossy(&probed.stdout).trim().to_string();
                    videos.push((name.clone(), counted));
                }
            }
            pictures.sort();
            println!(
                "{case}: exit {:?}; the frame words {:?}; {}; pictures {:?}; videos {:?}; appended {:?}",
                status.and_then(|status| status.code()),
                the_frame_words,
                range_line,
                pictures,
                videos,
                appended,
            );
            let is_a_video = file.starts_with("video");
            let right = status.and_then(|status| status.code()) == Some(0)
                && if is_a_video {
                    videos.len() == 1
                        && videos[0].1 == expected.len().to_string()
                        && appended == expected
                } else {
                    pictures == expected
                };
            if !right {
                faults.push(format!("{case}: expected {expected:?}"));
            }
        }
        assert!(faults.is_empty(), "{faults:#?}");
    }

    /// **The Creator's thirteenth notes, note 8, at the desk: the 5.4.0 Alpha renders a
    /// two-View-Layer file on the processor through this driver, whole** —
    /// the real version ask's answer, the real reading of the file taken by
    /// the alpha, everything composed from it, the script before the file and
    /// the launch material placed and read back on disk, and the render
    /// launched with the line's own words. Kept in the tree and ignored. Run by
    /// hand:
    ///
    /// ```text
    /// SMOOZY_DESK_BLENDER=/Applications/Blender\ 5.4a.app/Contents/MacOS/Blender \
    /// SMOOZY_DESK_VERSION=5.4.0 \
    /// SMOOZY_DESK_FILE=<the two-layer stand-in> \
    /// SMOOZY_DESK_OUT=<an empty folder> \
    /// cargo test -p smoozy-rendering-machine -- --ignored --nocapture a_real_blender_renders_two_view_layers
    /// ```
    ///
    /// The stand-in is the factory scene saved by the alpha with no window:
    /// Cycles on the processor, 32 × 32, four samples, frames 1 to 2, a second
    /// view layer named `Second` beside `ViewLayer`, both rendering, and the
    /// regular output path `//out/frame_` as a multi-layer OpenEXR, so both
    /// layers land in each frame's file. As in the test above, the build path is
    /// a two-line wrapper adding the factory-startup switch at the head, because
    /// the Blenders at the desk are the Creator's.
    ///
    /// **What it proves**: the reading runs in the version it was composed for
    /// and finds both layers at both frames; the line asks for the device line
    /// with the version's own switch and names no device for a job the file
    /// sets on the processor; and the render exits 0 with both frames written,
    /// each holding both layers, and the device line saying the processor did
    /// the work.
    #[test]
    #[ignore = "run by hand at the desk with a real Blender and a real .blend file"]
    fn a_real_blender_renders_two_view_layers_on_the_processor_through_the_real_line() {
        use crate::acting::material::TheWorkersOwnFilesystem;
        use crate::acting::reading::TheReadingOnTheMachine;
        use std::os::unix::fs::PermissionsExt;
        use std::path::PathBuf;

        const ONE_RUN_AT_MOST: std::time::Duration = std::time::Duration::from_secs(300);
        let blender = std::env::var("SMOOZY_DESK_BLENDER").expect("SMOOZY_DESK_BLENDER");
        let file = PathBuf::from(std::env::var("SMOOZY_DESK_FILE").expect("SMOOZY_DESK_FILE"));
        let root = PathBuf::from(std::env::var("SMOOZY_DESK_OUT").expect("SMOOZY_DESK_OUT"));
        let version_text = std::env::var("SMOOZY_DESK_VERSION").expect("SMOOZY_DESK_VERSION");
        let version = Version::parse(&version_text).expect("a version");

        std::fs::create_dir_all(&root).expect("the run's folder");
        let wrapper = root.join("blender-with-factory-startup");
        std::fs::write(&wrapper, format!("#!/bin/sh\nexec '{blender}' --factory-startup \"$@\"\n"))
            .expect("the wrapper");
        std::fs::set_permissions(&wrapper, std::fs::Permissions::from_mode(0o755)).expect("runnable");
        let blend = root.join(file.file_name().expect("the file has a name"));
        std::fs::copy(&file, &blend).expect("the file is put in place");
        let job_folder = root.join("job");

        let job = a_job();
        let mut manifest = a_manifest(&job, &version_text, "Scene");
        manifest.frame_range = FrameRange::TheUsersParsedIntegers(vec![1, 2]);
        manifest.blend_path_on_the_machine = blend.display().to_string();
        let payload = crate::material::tests::an_untouched_payload();
        let mut machine = answers();
        machine.remap = Some(RemapMaterial::composed(&job, &crate::material::tests::a_blender(), Vec::new()));
        let clock = the_machines_clock();
        let start = Watched {
            start: TheRenderStartedOnTheMachine {
                kernel: AKernelThat(Some(19186)),
            },
            words: std::cell::RefCell::new(Vec::new()),
        };
        let loader = ALoaderThatCounts {
            answers: WhatTheLoaderFound::TheLibrary,
            asked: std::cell::Cell::new(0),
        };
        let at = |name: &str| root.join(name).display().to_string();
        let went = {
            let mut pipeline = TheRenderWorkersPipeline {
                start: &start,
                places: &TheWorkersOwnFilesystem,
                asks_the_version: &NamesItself(Some(version)),
                takes_the_reading: &TheReadingOnTheMachine,
                steps: &mut machine,
                looks_for_the_optix_library: &loader,
            };
            pipeline.run(TheAttempt {
                job: &job,
                manifest: &manifest,
                payload: &payload,
                the_manifests_version: version,
                the_remap_materials_path: &at("remap"),
                the_blender_build_path: &wrapper.display().to_string(),
                the_launch_materials_path: &at("material.py"),
                the_load_time_scripts_path: &at("load-time.py"),
                the_readings_script_path: &at("reading.py"),
                the_job_output_folder: TheJobOutputFolderForThisAttempt::Known(
                    &job_folder.display().to_string(),
                ),
                the_folder_the_blend_sits_in: root.display().to_string().as_bytes(),
                previous_attempts: &[],
                number: 1,
                the_machines_clock: &clock,
                the_manifests_reading: None,
                the_manifests_settings_half: None,
            })
        };
        let words = start.words.borrow().clone();
        std::fs::write(root.join("words.txt"), words.join("\n")).ok();

        // **The reading the alpha took**, as the driver composed it.
        let composed = machine.readings_written.last().expect("a reading was taken").clone();
        std::fs::write(
            root.join("reading.json"),
            serde_json::to_string_pretty(&composed.reading).expect("the reading writes out"),
        )
        .ok();
        let outputs = &composed.reading.outputs;
        let layers_at_each_frame: Vec<(i64, Vec<String>)> = outputs
            .per_frame
            .iter()
            .map(|one| {
                (
                    one.frame,
                    one.view_layers_that_render
                        .iter()
                        .map(|layer| String::from_utf8_lossy(layer.as_bytes()).to_string())
                        .collect(),
                )
            })
            .collect();
        println!("the reading: taken by {:?}; the layers at each frame {layers_at_each_frame:?}", outputs.taken_by);
        assert_eq!(
            outputs.taken_by,
            TheVersionInHand::of(version.major as u16, version.minor as u16)
        );
        assert_eq!(
            layers_at_each_frame,
            vec![
                (1, vec!["ViewLayer".to_string(), "Second".to_string()]),
                (2, vec!["ViewLayer".to_string(), "Second".to_string()]),
            ]
        );

        let WhereTheAttemptEnded::TheRenderWasLaunched(mut started) = went.ended else {
            panic!("not launched: {:?}", went.ended);
        };
        let take = |mut from: Box<dyn std::io::Read + Send>| {
            std::thread::spawn(move || {
                let mut all = Vec::new();
                from.read_to_end(&mut all).ok();
                all
            })
        };
        let ordinary = take(Box::new(started.ordinary));
        let complaints = take(Box::new(started.complaints));
        let began = std::time::Instant::now();
        let status = loop {
            if let Some(status) = started.child.try_wait().expect("it can be asked") {
                break Some(status);
            }
            if began.elapsed() > ONE_RUN_AT_MOST {
                started.child.kill().ok();
                started.child.wait().ok();
                break None;
            }
            std::thread::sleep(std::time::Duration::from_millis(100));
        };
        let ordinary = ordinary.join().expect("the reader ends");
        let complaints = complaints.join().expect("the reader ends");
        std::fs::write(root.join("render.log"), &ordinary).ok();
        std::fs::write(root.join("render-complaints.log"), &complaints).ok();

        let log = String::from_utf8_lossy(&ordinary).to_string();
        let the_device_lines: Vec<&str> = log.lines().filter(|line| line.contains("Path tracing on")).collect();
        let saved = log.lines().filter(|line| line.contains("Saved:")).count();
        fn every_picture_under(folder: &std::path::Path) -> Vec<PathBuf> {
            let mut found = Vec::new();
            for entry in std::fs::read_dir(folder).into_iter().flatten().flatten() {
                let path = entry.path();
                if path.is_dir() {
                    found.extend(every_picture_under(&path));
                } else if path.extension().and_then(|e| e.to_str()) == Some("exr") {
                    found.push(path);
                }
            }
            found.sort();
            found
        }
        let written = every_picture_under(&job_folder);
        let layers_in_each_file: Vec<(String, bool, bool)> = written
            .iter()
            .map(|path| {
                let bytes = std::fs::read(path).expect("the picture reads");
                let holds = |name: &[u8]| bytes.windows(name.len()).any(|window| window == name);
                (
                    path.file_name().and_then(|n| n.to_str()).unwrap_or("").to_string(),
                    holds(b"ViewLayer."),
                    holds(b"Second."),
                )
            })
            .collect();
        println!(
            "exit {:?}; the words {words:?}; the device lines {the_device_lines:?}; saved {saved}; the files and their layers {layers_in_each_file:?}",
            status.and_then(|status| status.code()),
        );
        assert_eq!(status.and_then(|status| status.code()), Some(0));
        assert!(words.iter().any(|word| word == "--log-level"), "the version's own ask");
        assert!(!words.iter().any(|word| word == "--"), "nothing is named for the processor");
        assert!(
            !the_device_lines.is_empty() && the_device_lines.iter().all(|line| line.contains("CPU")),
            "the device line names the processor"
        );
        assert_eq!(saved, 2);
        assert_eq!(
            layers_in_each_file,
            vec![
                ("frame_0001.exr".to_string(), true, true),
                ("frame_0002.exr".to_string(), true, true),
            ]
        );
    }
}

//! The launch line — L1–L3 (the route), L8 (the window and the order), L8a
//! (the one argument outside it), L9 (no factory-startup switch, and the
//! file's own scripts run) — **and the
//! order is held by the type, not by care.**
//!
//! **L1: every render is started on the command-line route** — the
//! no-window mode, the file, and the frames named on the command line with
//! the animation render instruction — never by handing Blender a render
//! instruction from a script, because from 5.0 that route prints nothing at
//! all while it works (*Reports 9*). **This is Row 11 and the constraint
//! every other clause sits inside.** L2: the launch material sets what has no
//! argument, inside the window; the render is still started by the
//! instruction on the command line. L3: the no-window mode is not optional
//! — a Blender given a window does not end when its render does (*The
//! machine 7*).
//!
//! **L8: one window, and everything imposed sits in it — after the file,
//! before the render instruction, and before any bare double dash.** Outside
//! it, anything handed in is silently ignored, with no message and no
//! failure (`Job files and outputs` D11). **The left edge is worse than
//! silent — it is plausible**: a pair handed in before the file prints the
//! value asked for straight back and changes nothing, observed in all four
//! versions (*Settings 17*), so nothing inside the material can catch a
//! misplacement, and **L10a says outright that no read-back can stand in
//! for the order.** So the line is not built as a list of strings a later
//! reader has to check. **Every argument has exactly one position
//! ([`Argument::position`]), the line is rendered by walking the positions
//! in the contract's order, and [`LaunchLine::from_placed`] refuses any
//! sequence that is not in that order** — the type refuses a wrongly
//! ordered line, and [`LaunchLine::compose`] can only produce a line the
//! refusal lets through.
//!
//! **L8a: one argument sits outside the window, at the head of the line,
//! and without it every fail-loud conduct in the launch material is a paid
//! wrong render reported as success.** A launch script raising an error
//! after the file *"printed the traceback, rendered anyway (three frames
//! written), and exited 0"*; with `--python-exit-code 63` on the line both
//! versions *"ended at exit 63 with no frame written"*. Four conducts rest on
//! it: the remap's raised pair, D10's read-back raise, and `Render Settings`'
//! pin and anchor refusals. **The number is fixed by this contract and never
//! read back for a cause** (K6). The head is chosen because it is a process
//! option rather than an imposed value, in force before anything the
//! material can raise — the same footing as the no-window mode — and *Reports
//! 12* records the switch working *"placed before the script argument"* and
//! doing nothing after it; §9 item 19 carries the residue.
//!
//! **L8b: one script sits before the file, at the head with L8a's argument,
//! and it only registers a handler Blender runs as the file loads** (added
//! 2026-10-02). It is the one `--python` before the file: a value set before
//! the file is overwritten by the load, which is L8's left edge, but a handler
//! runs after the load, so the window does not move. **A line without it is
//! not a line** ([`NotALine::TheHeadIsIncomplete`]), because the material's
//! first act after its marker is to raise where the handler did not run.
//!
//! **L9: no factory-startup switch is used.** The machine's saved
//! preferences would be discarded silently by one, and there is no
//! [`Argument`] variant for it. **And every line lets the file's own scripts
//! run** (changed 2026-10-08, the Creator's ruling 4: *"yes, Smoozy should
//! definitely make every Blender on it enable this."*): [`Argument::ScriptsRun`]
//! stands at the head right after the no-window mode, where Blender's own
//! security page puts it in its worked example, and **a line without it is
//! not a line** ([`NotALine::TheHeadIsIncomplete`]). A re-run is composed by
//! the same [`LaunchLine::compose`], so it carries it too. The two readings
//! carry the same switch: [`crate::acting::reading::the_words_of_a_reading`]
//! here, and the dependency check's in `Dependency Upload & Remap — machine`.
//!
//! **Where the device-line ask sits, and why it is not where the plan's list
//! puts it — reported to the runner as a fault of the list, and built here
//! on the plan's own words.** L8 lists the ask as item 10, after the device
//! argument at item 9, which sits after the bare double dash. But the ask is
//! Blender's own switch in every version (*What actually ran 1*, *Reports
//! 8*), and L8's own third edge says that after a bare double dash *"Blender
//! reads nothing of its own there"* — a control proved that an argument in
//! that position Blender does not know is swallowed in silence (*Settings
//! 17*). Placed literally at item 10 the ask would be swallowed on every
//! attempt and L26's line would never be printed. L8 item 10's own words are
//! *"placed where that version's evidence shows it taking effect"*, and the
//! switch is a process option like the no-window mode, so it sits at the
//! head with the other process options, before the file. The list's order
//! of items 9 and 10 is the fault; the item's own sentence is what is built.
//!
//! **L17a: a job with any video-shaped destination is the one exception to
//! the explicit list** (added 2026-10-17, the Creator's eleventh notes, note
//! 1, the video piece). Three 90-frame jobs came home as a finished video holding
//! **one frame**, because a frame list renders each frame as its own render
//! and each one rewrites the same file. The video desk test settled the form in
//! five Blenders: **the job's first frame, then its last, then the animation
//! instruction, all after the file** — [`Argument::AnimationOverOneRun`],
//! one argument so the words can never be parted or reordered, at the
//! render instruction's own place.
//!
//! **L17a's step (the video-range piece, the same day): the step is handed too, every
//! time, 1 included, before the animation instruction** — `-s <first> -e
//! <last> -j <step> -a`. The second desk test found the first and last alone
//! keeping the file's saved step (a step-2 file wrote five frames of ten), and
//! `-j` placed after `-a` ignored. So a video job's frames are one evenly
//! stepped ascending run, the step its own — typed, or the file's — and **only
//! a list with gaps is refused, never guessed at**
//! ([`NotALine::AVideosFramesHaveGaps`]).
//!
//! **L17b (the video-range piece): no frame below zero is ever a bare negative number on
//! the line**, for a list and a video's run alike — a sign makes Blender count
//! from the file's own range ([`crate::frames::AFrameWord`]). Until the video-range piece a
//! video starting below zero was refused here; now it is written from the
//! file's saved range, and **a frame below zero outside that range is refused**
//! ([`NotALine::AFrameBelowZeroOutsideTheScenesRange`]), at the gate itself, so
//! no line holding one can exist.
//!
//! **Markers used in this file that belong to another document, each with
//! the document it belongs to.** `Job files and outputs` C6, D6, D10, D11,
//! I19.

use crate::frames::{ABareNegativeFrame, AFrameWord, ExplicitFrameList, OneSteppedRun, TheFilesSavedRange};
use crate::spellings::{
    self, Spelling, ANIMATION_RENDER_INSTRUCTION, DEVICE_ARGUMENT, DEVICE_LINE_ASK,
    ENGINE_ARGUMENT, FAILURE_TO_EXIT_STATUS_SWITCH, FRAME_END_ARGUMENT, FRAME_LIST_ARGUMENT,
    FRAME_START_ARGUMENT, FRAME_STEP_ARGUMENT, LAUNCH_MATERIAL_ARGUMENT, NO_WINDOW_MODE, REGULAR_OUTPUT_PATH_ARGUMENT,
    SCENE_ARGUMENT, SCRIPTS_RUN_SWITCH, THE_BARE_DOUBLE_DASH, THE_FAILURE_STATUS,
};
use crate::technology::Technology;
use crate::versions::TheFourVersions;

/// **The positions on the line, in the contract's order — and the order is
/// the enum's declaration order.** Nothing composes a line by any other
/// ordering: [`LaunchLine::from_placed`] compares positions and refuses a
/// step backwards.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Position {
    /// Process options, in force before the file: the no-window mode (L3),
    /// the failure-to-exit-status switch (L8a), the device-line ask (L26) —
    /// and L8b's script, which registers the load-time handler.
    TheHead,
    /// The file — the manifest's `.blend` path, verbatim (L8 item 2).
    TheFile,
    /// **The window**: after the file, before the render instruction, before
    /// the double dash — the scene, the engine, the regular output path and
    /// the launch material, in that order (L8 items 3, 4, 5, 7).
    TheWindow,
    /// The render instruction, which ends the window (L8 item 8): the
    /// explicit frame list (L17), or for a job that writes a video the first
    /// frame, the last, the step and the animation instruction (L17a).
    TheRenderInstruction,
    /// The bare double dash, after which the line is Cycles' (L8 item 9).
    TheBareDoubleDash,
    /// Cycles' own line: the device argument, and nothing of Blender's own
    /// (L8 item 9, L24a).
    CyclesOwn,
}

/// One argument of the line. **Each variant has exactly one position and
/// one rank inside it**, so a value knows where it goes and nothing chooses
/// for it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Argument {
    /// L3.
    NoWindowMode,
    /// **L9, since 2026-10-08: the file's own scripts run**, on every render,
    /// a re-run's included.
    ScriptsRun,
    /// L8a, carrying [`THE_FAILURE_STATUS`].
    FailureToExitStatus,
    /// L26, the version's own switch.
    DeviceLineAsk { version: TheFourVersions },
    /// **L8b — the script before the file**, as the path it was placed at.
    /// It registers a handler Blender runs as the file loads and sets nothing
    /// itself.
    LoadTimeScript { path: String },
    /// L8 item 2.
    TheFile { path: String },
    /// L8 item 3, L15 — the manifest's scene name, byte for byte.
    Scene { name: String },
    /// L8 item 4, L19 — the chosen version's own engine identifier.
    Engine { identifier: String },
    /// L8 item 5, `Job files and outputs` D10 — the destination map's pair
    /// for the regular output path, delivered by its argument.
    RegularOutputPath { value: String },
    /// L8 item 7 — the launch material, as the path it was placed at.
    LaunchMaterial { path: String },
    /// L8 item 8, L17, L18 — **and L17b**: the list is written against the
    /// file's saved range, every frame below zero as its offset from the saved
    /// start.
    FrameList {
        list: ExplicitFrameList,
        from: TheFilesSavedRange,
    },
    /// **L8 item 8, L17a — a job with any video-shaped destination**: the
    /// first frame of its one evenly stepped run, then the last, then the
    /// step, then the animation instruction, as one argument so they are never
    /// parted or put in another order. Made only by [`LaunchLine::compose`]
    /// out of a list that is one evenly stepped ascending run; **L17b** writes
    /// a first or last below zero from the file's saved range.
    AnimationOverOneRun {
        run: OneSteppedRun,
        from: TheFilesSavedRange,
    },
    /// L8 item 9.
    BareDoubleDash,
    /// L8 item 9, L21, L24a — present exactly where the job is set to use
    /// the cards, carrying the technology looked up from the make.
    Device {
        version: TheFourVersions,
        technology: Technology,
    },
}

impl Argument {
    /// **The one position an argument has.** A total function over the
    /// variants; no variant can be placed anywhere else.
    pub fn position(&self) -> Position {
        match self {
            Argument::NoWindowMode
            | Argument::ScriptsRun
            | Argument::FailureToExitStatus
            | Argument::DeviceLineAsk { .. }
            | Argument::LoadTimeScript { .. } => Position::TheHead,
            Argument::TheFile { .. } => Position::TheFile,
            Argument::Scene { .. }
            | Argument::Engine { .. }
            | Argument::RegularOutputPath { .. }
            | Argument::LaunchMaterial { .. } => Position::TheWindow,
            Argument::FrameList { .. } | Argument::AnimationOverOneRun { .. } => {
                Position::TheRenderInstruction
            }
            Argument::BareDoubleDash => Position::TheBareDoubleDash,
            Argument::Device { .. } => Position::CyclesOwn,
        }
    }

    /// **The rank inside a position — L8's own item numbers**, so the
    /// window's four sit as items 3, 4, 5 and 7 and the head's five sit in
    /// the order L3, L9, L8a, L26 name them, with L8b's script last, nearest
    /// the file and after L8a's switch, which a raise in it reads. L9's switch
    /// sits right after the no-window mode, as the security page's example
    /// has it.
    pub fn rank_inside_its_position(&self) -> u8 {
        match self {
            Argument::NoWindowMode => 1,
            Argument::ScriptsRun => 2,
            Argument::FailureToExitStatus => 3,
            Argument::DeviceLineAsk { .. } => 4,
            Argument::LoadTimeScript { .. } => 5,
            Argument::TheFile { .. } => 2,
            Argument::Scene { .. } => 3,
            Argument::Engine { .. } => 4,
            Argument::RegularOutputPath { .. } => 5,
            Argument::LaunchMaterial { .. } => 7,
            Argument::FrameList { .. } => 8,
            Argument::AnimationOverOneRun { .. } => 8,
            Argument::BareDoubleDash => 9,
            Argument::Device { .. } => 9,
        }
    }

    /// Whether this is a value the window's three edges bind — everything
    /// imposed (L8's *everything imposed sits in it*).
    pub fn is_imposed(&self) -> bool {
        self.position() == Position::TheWindow
    }

    /// The words this argument puts on the line, from the spellings and
    /// from nothing a job carries for the spelling's part.
    fn words(&self) -> Result<Vec<String>, spellings::MissingSpelling> {
        fn text(spelling: Spelling) -> Result<String, spellings::MissingSpelling> {
            spelling.text().map(str::to_string)
        }
        Ok(match self {
            Argument::NoWindowMode => vec![text(NO_WINDOW_MODE)?],
            Argument::ScriptsRun => vec![text(SCRIPTS_RUN_SWITCH)?],
            Argument::FailureToExitStatus => {
                vec![
                    text(FAILURE_TO_EXIT_STATUS_SWITCH)?,
                    THE_FAILURE_STATUS.to_string(),
                ]
            }
            Argument::DeviceLineAsk { version } => {
                let ask = DEVICE_LINE_ASK.for_version(*version);
                let mut words = vec![text(ask.switch)?];
                if let Some(value) = ask.value {
                    words.push(value.to_string());
                }
                words
            }
            Argument::LoadTimeScript { path } => {
                vec![text(LAUNCH_MATERIAL_ARGUMENT)?, path.clone()]
            }
            Argument::TheFile { path } => vec![path.clone()],
            Argument::Scene { name } => vec![text(SCENE_ARGUMENT)?, name.clone()],
            Argument::Engine { identifier } => vec![text(ENGINE_ARGUMENT)?, identifier.clone()],
            Argument::RegularOutputPath { value } => {
                vec![text(REGULAR_OUTPUT_PATH_ARGUMENT)?, value.clone()]
            }
            Argument::LaunchMaterial { path } => {
                vec![text(LAUNCH_MATERIAL_ARGUMENT)?, path.clone()]
            }
            // **L17b: no frame below zero is a bare negative.** The gate
            // ([`Argument::a_frame_with_no_word`]) refuses every line holding
            // a frame with no word, so here every frame has one.
            Argument::FrameList { list, from } => vec![
                text(FRAME_LIST_ARGUMENT)?,
                list.as_argument_text(*from)
                    .expect("the gate refuses a list holding a frame with no word"),
            ],
            // **L17a: first, last, the step, then the animation instruction**
            // — the order the two desk tests found writing the video whole
            // with every step-th frame; a start, end or step placed after the
            // instruction was ignored.
            Argument::AnimationOverOneRun { run, from } => vec![
                text(FRAME_START_ARGUMENT)?,
                AFrameWord::in_a_list(run.first, *from)
                    .expect("the gate refuses a run starting at a frame with no word")
                    .as_text(),
                text(FRAME_END_ARGUMENT)?,
                AFrameWord::at_the_end(run.last, *from)
                    .expect("the gate refuses a run ending at a frame with no word")
                    .as_text(),
                text(FRAME_STEP_ARGUMENT)?,
                run.step.to_string(),
                text(ANIMATION_RENDER_INSTRUCTION)?,
            ],
            Argument::BareDoubleDash => vec![THE_BARE_DOUBLE_DASH.to_string()],
            Argument::Device {
                version,
                technology,
            } => vec![
                text(DEVICE_ARGUMENT.for_version(*version))?,
                technology.value().to_string(),
            ],
        })
    }
}

/// **The line, and the only way to hold one is through
/// [`LaunchLine::from_placed`]** — the field is private, so a `Vec` of
/// arguments in some other order is not a line.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LaunchLine {
    placed: Vec<Argument>,
}

/// Why a sequence of arguments is not a line.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NotALine {
    /// An argument stands before one whose position comes earlier in the
    /// contract's order — a settings value before the file, a Blender switch
    /// after the double dash, the material after the render instruction.
    OutOfOrder { earlier: Argument, later: Argument },
    /// Inside one position, an argument stands before one L8 ranks ahead
    /// of it.
    OutOfRankInsideItsPosition { earlier: Argument, later: Argument },
    /// No file, or more than one.
    NotExactlyOneFile,
    /// No render instruction, or more than one — a line that would render
    /// nothing and report success, or render twice.
    NotExactlyOneRenderInstruction,
    /// A device argument with no bare double dash before it is taken for a
    /// file name (*Settings 9*), and a bare double dash with nothing after
    /// it names nothing.
    TheDoubleDashAndTheDeviceArgumentComeTogether,
    /// The head is missing one of its process options — without L8a's
    /// switch every fail-loud conduct is silent, without L3 the run never
    /// ends, and without L8b's script the material's first check raises on
    /// every attempt.
    TheHeadIsIncomplete,
    /// The same argument twice.
    Repeated(Argument),
    /// **The destination map's regular output path is not text, and it is
    /// refused rather than substituted.** Added 2026-09-05 out of Round 1 of
    /// this crate's code audit.
    ///
    /// A location is bytes on purpose — `Job files and outputs` R4a says so
    /// in terms — and this crate honours that everywhere else, including in
    /// the read-back's own comparison, which compares the map's raw bytes.
    /// **The composition used to turn those bytes into text with any byte
    /// that is not valid text quietly replaced.** The harm was bounded,
    /// because the read-back then failed the attempt before the render
    /// instruction — but the job never rendered and **the reason recorded
    /// pointed at a mismatch rather than at the substitution that caused
    /// it**, which was the one place in this crate where the failure's cause
    /// was not the one the record named.
    TheRegularOutputPathIsNotText { value: Vec<u8> },
    /// **L17a: a job with a video-shaped destination whose frames are not one
    /// evenly stepped ascending run** — a gap, or frames out of order. One
    /// video cannot skip frames and the instruction carries only a first, a
    /// last and one step, so the attempt is refused rather than rendered as
    /// some other run. Added 2026-10-17, the video piece; **a step is no longer
    /// refused since the video-range piece**, the same day. Render Settings refuses a typed
    /// list with gaps before it is sent (the video-range piece); this is the machine's own
    /// refusal for whatever still arrives.
    AVideosFramesHaveGaps { asked: Vec<i64> },
    /// **L17b: a frame below zero outside the job's scene's saved range** — it
    /// has no word Blender reads as that frame (every signed number counts from
    /// the file's own range), so the line is refused rather than handed a
    /// guess. Added 2026-10-17, the video-range piece, in place of the video piece's refusal of every
    /// video starting below zero.
    AFrameBelowZeroOutsideTheScenesRange { frame: i64, start: i64, end: i64 },
}

impl Argument {
    /// **What one argument is, in standard software words** (Rule 9) — so
    /// nothing that says why a line was refused has to name this type.
    /// Added 2026-09-23, step 10c's fifth pass on Vast, fault 5.
    pub fn in_words(&self) -> &'static str {
        match self {
            Argument::NoWindowMode => "the no-window switch",
            Argument::ScriptsRun => "the Auto Run Python Scripts switch",
            Argument::FailureToExitStatus => "the fail-loud switch",
            Argument::DeviceLineAsk { .. } => "the ask for the device line",
            Argument::LoadTimeScript { .. } => "the script Blender runs as the file opens",
            Argument::TheFile { .. } => "the file to render",
            Argument::Scene { .. } => "the scene",
            Argument::Engine { .. } => "the engine",
            Argument::RegularOutputPath { .. } => "where the frames are written",
            Argument::LaunchMaterial { .. } => "the settings file Blender reads at the start",
            Argument::FrameList { .. } | Argument::AnimationOverOneRun { .. } => {
                "the frames to render"
            }
            Argument::BareDoubleDash => "the separator before the device",
            Argument::Device { .. } => "the device to render on",
        }
    }
}

impl Argument {
    /// **L17b: the first frame below zero this argument cannot write as an
    /// offset from the file's saved range**, where it holds one.
    fn a_frame_with_no_word(&self) -> Option<ABareNegativeFrame> {
        match self {
            Argument::FrameList { list, from } => list.as_argument_text(*from).err(),
            Argument::AnimationOverOneRun { run, from } => AFrameWord::in_a_list(run.first, *from)
                .and(AFrameWord::at_the_end(run.last, *from))
                .err(),
            _ => None,
        }
    }
}

impl NotALine {
    /// **Why the command line was refused, in standard software words**
    /// (Rule 9). **Every one of these is Smoozy's own composition and never
    /// anything the person did**, and the words say so rather than reading
    /// like a complaint about their file.
    pub fn in_words(&self) -> String {
        match self {
            NotALine::OutOfOrder { earlier, later } => format!(
                "Smoozy's command line has {} after {}",
                earlier.in_words(),
                later.in_words()
            ),
            NotALine::OutOfRankInsideItsPosition { earlier, later } => format!(
                "Smoozy's command line has {} before {}",
                later.in_words(),
                earlier.in_words()
            ),
            NotALine::NotExactlyOneFile => {
                "Smoozy's command line does not name exactly one file".to_string()
            }
            NotALine::NotExactlyOneRenderInstruction => {
                "Smoozy's command line does not ask for exactly one render".to_string()
            }
            NotALine::TheDoubleDashAndTheDeviceArgumentComeTogether => {
                "Smoozy's command line names a device without the separator before it".to_string()
            }
            NotALine::TheHeadIsIncomplete => {
                "Smoozy's command line is missing a required switch".to_string()
            }
            NotALine::Repeated(argument) => {
                format!("Smoozy's command line has {} twice", argument.in_words())
            }
            NotALine::TheRegularOutputPathIsNotText { .. } => {
                "the output path cannot be put on a command line"
                    .to_string()
            }
            NotALine::AVideosFramesHaveGaps { .. } => {
                "this job writes a video, and its frames have gaps or are out of order".to_string()
            }
            NotALine::AFrameBelowZeroOutsideTheScenesRange { frame, .. } => format!(
                "frame {frame} is below 0 and outside the scene's frame range, so it cannot be rendered"
            ),
        }
    }
}

impl LaunchLine {
    /// **The one gate.** A sequence is a line only where every argument's
    /// position is at or after the previous one's, L8's rank holds inside
    /// each position, exactly one file and one render instruction stand, the
    /// head carries L3's, L9's and L8a's options, and a device argument and the
    /// double dash come together.
    pub fn from_placed(placed: Vec<Argument>) -> Result<LaunchLine, NotALine> {
        for window in placed.windows(2) {
            let (earlier, later) = (&window[0], &window[1]);
            if later.position() < earlier.position() {
                return Err(NotALine::OutOfOrder {
                    earlier: earlier.clone(),
                    later: later.clone(),
                });
            }
            if later.position() == earlier.position()
                && later.rank_inside_its_position() < earlier.rank_inside_its_position()
            {
                return Err(NotALine::OutOfRankInsideItsPosition {
                    earlier: earlier.clone(),
                    later: later.clone(),
                });
            }
        }
        for (at, argument) in placed.iter().enumerate() {
            if placed[..at].contains(argument) {
                return Err(NotALine::Repeated(argument.clone()));
            }
        }
        let count = |position: Position| placed.iter().filter(|a| a.position() == position).count();
        if count(Position::TheFile) != 1 {
            return Err(NotALine::NotExactlyOneFile);
        }
        if count(Position::TheRenderInstruction) != 1 {
            return Err(NotALine::NotExactlyOneRenderInstruction);
        }
        if count(Position::TheBareDoubleDash) != count(Position::CyclesOwn)
            || count(Position::CyclesOwn) > 1
        {
            return Err(NotALine::TheDoubleDashAndTheDeviceArgumentComeTogether);
        }
        // **L17b, at the gate: no line holds a frame below zero it cannot
        // write as an offset**, so the words below always have one.
        if let Some(bare) = placed.iter().find_map(Argument::a_frame_with_no_word) {
            return Err(NotALine::AFrameBelowZeroOutsideTheScenesRange {
                frame: bare.frame,
                start: bare.from.start,
                end: bare.from.end,
            });
        }
        let has = |wanted: &Argument| placed.contains(wanted);
        if !has(&Argument::NoWindowMode)
            || !has(&Argument::ScriptsRun)
            || !has(&Argument::FailureToExitStatus)
            || !placed
                .iter()
                .any(|argument| matches!(argument, Argument::LoadTimeScript { .. }))
        {
            return Err(NotALine::TheHeadIsIncomplete);
        }
        Ok(LaunchLine { placed })
    }

    /// **L8, performed: the line composed in the contract's order from its
    /// inputs.** Every argument goes to its own position; nothing here
    /// chooses a position, and the result passes through
    /// [`LaunchLine::from_placed`] like anything else.
    pub fn compose(inputs: LineInputs) -> Result<LaunchLine, NotALine> {
        let mut placed = vec![
            Argument::NoWindowMode,
            Argument::ScriptsRun,
            Argument::FailureToExitStatus,
            Argument::DeviceLineAsk {
                version: inputs.version,
            },
            Argument::LoadTimeScript {
                path: inputs.load_time_script_path,
            },
            Argument::TheFile {
                path: inputs.blend_path,
            },
        ];
        if let Some(name) = inputs.scene {
            placed.push(Argument::Scene { name });
        }
        if let Some(identifier) = inputs.engine {
            placed.push(Argument::Engine { identifier });
        }
        // **The map's own bytes reach here and the conversion happens once,
        // at this gate, as a refusal rather than as a substitution.**
        placed.push(Argument::RegularOutputPath {
            value: String::from_utf8(inputs.regular_output_path).map_err(|not_text| {
                NotALine::TheRegularOutputPathIsNotText {
                    value: not_text.into_bytes(),
                }
            })?,
        });
        placed.push(Argument::LaunchMaterial {
            path: inputs.launch_material_path,
        });
        placed.push(the_render_instruction(
            inputs.frames,
            inputs.a_destination_is_video_shaped,
            inputs.the_files_saved_range,
        )?);
        if let Some(technology) = inputs.technology {
            placed.push(Argument::BareDoubleDash);
            placed.push(Argument::Device {
                version: inputs.version,
                technology,
            });
        }
        LaunchLine::from_placed(placed)
    }

    /// The arguments, in the order the line carries them.
    pub fn placed(&self) -> &[Argument] {
        &self.placed
    }

    /// **The words after the program's own path**, rendered from the
    /// spellings. Refuses where a spelling is not on record — the absence
    /// is a typed one and no word is invented for it.
    pub fn as_words(&self) -> Result<Vec<String>, spellings::MissingSpelling> {
        let mut words = Vec::new();
        for argument in &self.placed {
            words.extend(argument.words()?);
        }
        Ok(words)
    }

    /// **The window's three edges, as indexes into the placed arguments**:
    /// the file, the render instruction, and the double dash where there is
    /// one. Every imposed argument sits strictly between the first two, and
    /// the third stands after both.
    pub fn the_window(&self) -> (usize, usize, Option<usize>) {
        let at = |position: Position| self.placed.iter().position(|a| a.position() == position);
        (
            at(Position::TheFile).expect("a line has one file"),
            at(Position::TheRenderInstruction).expect("a line has one render instruction"),
            at(Position::TheBareDoubleDash),
        )
    }
}

/// What the line is composed from — **the manifest's, the map's, the
/// reckoning's and the lookup's values, and nothing that could carry a
/// spelling.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LineInputs {
    pub version: TheFourVersions,
    /// The manifest's `.blend` path, verbatim (L8 item 2).
    pub blend_path: String,
    /// Exactly where the manifest carries a scene name (L15).
    pub scene: Option<String>,
    /// Exactly where the settings payload names an engine, as the chosen
    /// version's own identifier anchored at the attempt (L19, P1a).
    pub engine: Option<String>,
    /// The destination map's pair for the regular output path — **the map's
    /// own bytes**, template marks and all (`Job files and outputs` D10,
    /// `Job files and outputs` R4a). A value that is not text is refused by
    /// [`NotALine::TheRegularOutputPathIsNotText`] and never substituted.
    pub regular_output_path: Vec<u8>,
    /// Where the material was placed and read back (L12).
    pub launch_material_path: String,
    /// **Where L8b's script was placed and read back** (L12).
    pub load_time_script_path: String,
    /// The reckoning's list (L18).
    pub frames: ExplicitFrameList,
    /// **L17b: the job's scene's saved first and last frame, as the reading in
    /// force recorded them** — what a frame below zero is written from.
    pub the_files_saved_range: TheFilesSavedRange,
    /// **L17a, `Job files and outputs` I19: whether any of the job's
    /// destinations is video-shaped**, read off the map in force. Where it
    /// is, the render instruction is the first frame, the last and the
    /// animation instruction rather than the list.
    pub a_destination_is_video_shaped: bool,
    /// The technology looked up from the make, exactly where the job is set
    /// to use the cards (L21, L23); `None` where the file asks for the
    /// processor.
    pub technology: Option<Technology>,
}

/// **L17 and L17a: which render instruction the line carries.** The explicit
/// list, exactly as before, unless a destination is video-shaped; then the
/// first frame, the last, the step and the animation instruction — and only
/// where the list is one evenly stepped ascending run, refused in words
/// otherwise. `Job files and outputs` C6 makes a video job's list the whole
/// range, so on a job sent from Render Settings the refusal stands for what
/// should never arrive. **L17b's refusal is the gate's**, for both kinds.
fn the_render_instruction(
    frames: ExplicitFrameList,
    a_destination_is_video_shaped: bool,
    from: TheFilesSavedRange,
) -> Result<Argument, NotALine> {
    if !a_destination_is_video_shaped {
        return Ok(Argument::FrameList { list: frames, from });
    }
    match frames.as_one_stepped_run() {
        Some(run) => Ok(Argument::AnimationOverOneRun { run, from }),
        None => Err(NotALine::AVideosFramesHaveGaps {
            asked: frames.in_the_asked_order().to_vec(),
        }),
    }
}

/// **L8's thread-count exception is noted and not used.** The thread count
/// is the one argument taken on either side of the file (*What a render
/// demands 3*); nothing here relies on that exemption, and there is no
/// [`Argument`] for it.
pub const THE_THREAD_COUNT_EXEMPTION_IS_NOT_USED: bool = true;

/// **L9: no factory-startup switch** — kept by there being no variant —
/// **and the file's own scripts run on every line**, kept by
/// [`Argument::ScriptsRun`] being one of the head's required switches.
/// Changed 2026-10-08: until then the line carried no stored-scripts switch
/// in either direction.
pub const NO_FACTORY_STARTUP_AND_THE_FILES_SCRIPTS_RUN: bool = true;

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    /// **A file saved at 1 to 250** — what a frame below zero is written from.
    pub(crate) const SAVED: TheFilesSavedRange = TheFilesSavedRange { start: 1, end: 250 };

    pub(crate) fn frames(asked: &[i64]) -> ExplicitFrameList {
        ExplicitFrameList::from_the_reckonings_integers(asked).unwrap()
    }

    pub(crate) fn inputs(technology: Option<Technology>) -> LineInputs {
        LineInputs {
            version: TheFourVersions::V5_2,
            blend_path: "/ephemeral/smoozy/assets/kitchen.blend".to_string(),
            scene: Some("Kitchen".to_string()),
            engine: Some("CYCLES".to_string()),
            regular_output_path: b"/ephemeral/smoozy/jobs/j1/frame_####".to_vec(),
            launch_material_path: "/ephemeral/smoozy/job-records/j1/material.py".to_string(),
            load_time_script_path: "/ephemeral/smoozy/job-records/j1/load-time.py".to_string(),
            frames: frames(&[1, 2, 3]),
            a_destination_is_video_shaped: false,
            the_files_saved_range: SAVED,
            technology,
        }
    }

    /// **L8's order, item by item, read off a composed line.**
    #[test]
    fn a_composed_line_carries_the_contracts_order_and_the_window_holds() {
        let line = LaunchLine::compose(inputs(Some(Technology::Optix))).unwrap();
        let words = line.as_words().unwrap();
        assert_eq!(
            words,
            vec![
                "--background",
                "--enable-autoexec",
                "--python-exit-code",
                "63",
                "--log-level",
                "info",
                "--python",
                "/ephemeral/smoozy/job-records/j1/load-time.py",
                "/ephemeral/smoozy/assets/kitchen.blend",
                "-S",
                "Kitchen",
                "--engine",
                "CYCLES",
                "--render-output",
                "/ephemeral/smoozy/jobs/j1/frame_####",
                "--python",
                "/ephemeral/smoozy/job-records/j1/material.py",
                "--render-frame",
                "1..3",
                "--",
                &["--cycles", "-device"].concat(),
                "OPTIX",
            ]
        );
        let (file, render, dash) = line.the_window();
        for (at, argument) in line.placed().iter().enumerate() {
            if argument.is_imposed() {
                assert!(
                    at > file && at < render,
                    "{argument:?} is outside the window"
                );
                assert!(at < dash.unwrap());
            }
        }
        assert!(dash.unwrap() > render);
    }

    /// **The second of the four named faults: a settings value handed in
    /// before the file.** The type refuses it and names both arguments.
    #[test]
    fn a_settings_value_before_the_file_is_not_a_line() {
        let refused = LaunchLine::from_placed(vec![
            Argument::NoWindowMode,
            Argument::FailureToExitStatus,
            Argument::LaunchMaterial {
                path: "material.py".to_string(),
            },
            Argument::TheFile {
                path: "kitchen.blend".to_string(),
            },
            Argument::FrameList { list: frames(&[1]), from: SAVED },
        ]);
        assert!(
            matches!(
                refused,
                Err(NotALine::OutOfOrder {
                    earlier: Argument::LaunchMaterial { .. },
                    later: Argument::TheFile { .. }
                })
            ),
            "{refused:?}"
        );
    }

    /// **The third of the four named faults: the device argument swallowed
    /// after the bare double dash** — here from the other side, anything of
    /// Blender's own placed after the double dash, and the device argument
    /// placed without one.
    #[test]
    fn nothing_of_blenders_own_stands_after_the_double_dash_and_the_device_never_stands_without_it()
    {
        let ask_after_the_dash = LaunchLine::from_placed(vec![
            Argument::NoWindowMode,
            Argument::FailureToExitStatus,
            Argument::TheFile {
                path: "kitchen.blend".to_string(),
            },
            Argument::FrameList { list: frames(&[1]), from: SAVED },
            Argument::BareDoubleDash,
            Argument::Device {
                version: TheFourVersions::V5_2,
                technology: Technology::Optix,
            },
            Argument::DeviceLineAsk {
                version: TheFourVersions::V5_2,
            },
        ]);
        assert!(matches!(
            ask_after_the_dash,
            Err(NotALine::OutOfOrder { .. })
        ));

        let device_without_the_dash = LaunchLine::from_placed(vec![
            Argument::NoWindowMode,
            Argument::FailureToExitStatus,
            Argument::TheFile {
                path: "kitchen.blend".to_string(),
            },
            Argument::FrameList { list: frames(&[1]), from: SAVED },
            Argument::Device {
                version: TheFourVersions::V5_2,
                technology: Technology::Optix,
            },
        ]);
        assert_eq!(
            device_without_the_dash,
            Err(NotALine::TheDoubleDashAndTheDeviceArgumentComeTogether)
        );
    }

    #[test]
    fn a_value_after_the_render_instruction_is_not_a_line_either() {
        let refused = LaunchLine::from_placed(vec![
            Argument::NoWindowMode,
            Argument::FailureToExitStatus,
            Argument::TheFile {
                path: "kitchen.blend".to_string(),
            },
            Argument::FrameList { list: frames(&[1]), from: SAVED },
            Argument::RegularOutputPath {
                value: "out".to_string(),
            },
        ]);
        assert!(matches!(refused, Err(NotALine::OutOfOrder { .. })));
    }

    #[test]
    fn inside_the_window_l8s_own_ranks_hold() {
        let refused = LaunchLine::from_placed(vec![
            Argument::NoWindowMode,
            Argument::FailureToExitStatus,
            Argument::TheFile {
                path: "kitchen.blend".to_string(),
            },
            Argument::LaunchMaterial {
                path: "material.py".to_string(),
            },
            Argument::RegularOutputPath {
                value: "out".to_string(),
            },
            Argument::FrameList { list: frames(&[1]), from: SAVED },
        ]);
        assert!(matches!(
            refused,
            Err(NotALine::OutOfRankInsideItsPosition { .. })
        ));
    }

    #[test]
    fn a_line_without_l8as_switch_or_the_no_window_mode_is_not_a_line() {
        let no_switch = LaunchLine::from_placed(vec![
            Argument::NoWindowMode,
            Argument::TheFile {
                path: "kitchen.blend".to_string(),
            },
            Argument::FrameList { list: frames(&[1]), from: SAVED },
        ]);
        assert_eq!(no_switch, Err(NotALine::TheHeadIsIncomplete));
        let no_mode = LaunchLine::from_placed(vec![
            Argument::FailureToExitStatus,
            Argument::TheFile {
                path: "kitchen.blend".to_string(),
            },
            Argument::FrameList { list: frames(&[1]), from: SAVED },
        ]);
        assert_eq!(no_mode, Err(NotALine::TheHeadIsIncomplete));
    }

    #[test]
    fn exactly_one_file_and_exactly_one_render_instruction() {
        let two_files = LaunchLine::from_placed(vec![
            Argument::NoWindowMode,
            Argument::FailureToExitStatus,
            Argument::TheFile {
                path: "a.blend".to_string(),
            },
            Argument::TheFile {
                path: "b.blend".to_string(),
            },
            Argument::FrameList { list: frames(&[1]), from: SAVED },
        ]);
        assert_eq!(two_files, Err(NotALine::NotExactlyOneFile));
        let no_instruction = LaunchLine::from_placed(vec![
            Argument::NoWindowMode,
            Argument::FailureToExitStatus,
            Argument::TheFile {
                path: "a.blend".to_string(),
            },
        ]);
        assert_eq!(
            no_instruction,
            Err(NotALine::NotExactlyOneRenderInstruction)
        );
        let repeated = LaunchLine::from_placed(vec![
            Argument::NoWindowMode,
            Argument::NoWindowMode,
            Argument::FailureToExitStatus,
            Argument::TheFile {
                path: "a.blend".to_string(),
            },
            Argument::FrameList { list: frames(&[1]), from: SAVED },
        ]);
        assert_eq!(repeated, Err(NotALine::Repeated(Argument::NoWindowMode)));
    }

    /// **L21: the device argument is present exactly where the job is set
    /// to use the cards, and absent where the file asks for the processor.**
    #[test]
    fn a_processor_job_carries_no_double_dash_and_no_device_argument() {
        let line = LaunchLine::compose(inputs(None)).unwrap();
        let words = line.as_words().unwrap();
        assert!(!words.contains(&THE_BARE_DOUBLE_DASH.to_string()));
        assert!(!words.iter().any(|word| word.contains("cycles")));
        assert_eq!(line.the_window().2, None);
    }

    /// **L15, L19: the scene and the engine arguments are present exactly
    /// where the manifest and the payload carry them.**
    #[test]
    fn the_scene_and_engine_arguments_are_present_exactly_where_carried() {
        let mut without = inputs(None);
        without.scene = None;
        without.engine = None;
        let words = LaunchLine::compose(without).unwrap().as_words().unwrap();
        assert!(!words.contains(&"-S".to_string()));
        assert!(!words.contains(&"--engine".to_string()));
    }

    /// **The ask is the version's own, at the head** — 4.5's debugging
    /// switch, and the logging switch with its value from 5.0.
    #[test]
    fn the_ask_at_the_head_is_the_versions_own() {
        let mut older = inputs(None);
        older.version = TheFourVersions::V4_5;
        let words = LaunchLine::compose(older).unwrap().as_words().unwrap();
        assert_eq!(
            &words[..5],
            &["--background", "--enable-autoexec", "--python-exit-code", "63", "--debug-cycles"]
        );
        assert_eq!(words[7], "/ephemeral/smoozy/assets/kitchen.blend");
    }

    /// **L8b: the script before the file sits at the head, after L8a's
    /// switch and right before the file — and the type refuses it anywhere
    /// else, and refuses a line without it.** Added 2026-10-02, simulation
    /// fix piece 4. After the file it would run after the load, which is
    /// exactly what a geometry-nodes bake folder ignores and what a Replay
    /// fluid's first cache files are written before.
    #[test]
    fn the_load_time_script_sits_before_the_file_and_nowhere_else() {
        let words = LaunchLine::compose(inputs(None)).unwrap().as_words().unwrap();
        let script = words
            .iter()
            .position(|word| word == "/ephemeral/smoozy/job-records/j1/load-time.py")
            .expect("the script is on the line");
        let file = words
            .iter()
            .position(|word| word == "/ephemeral/smoozy/assets/kitchen.blend")
            .unwrap();
        let switch = words.iter().position(|word| word == "--python-exit-code").unwrap();
        assert_eq!(words[script - 1], "--python");
        assert_eq!(script + 1, file, "nothing between the script and the file");
        assert!(switch < script);

        let script = || Argument::LoadTimeScript {
            path: "load-time.py".to_string(),
        };
        let after_the_file = LaunchLine::from_placed(vec![
            Argument::NoWindowMode,
            Argument::FailureToExitStatus,
            Argument::TheFile {
                path: "kitchen.blend".to_string(),
            },
            script(),
            Argument::FrameList { list: frames(&[1]), from: SAVED },
        ]);
        assert!(
            matches!(
                after_the_file,
                Err(NotALine::OutOfOrder {
                    earlier: Argument::TheFile { .. },
                    later: Argument::LoadTimeScript { .. }
                })
            ),
            "{after_the_file:?}"
        );
        let before_the_switch = LaunchLine::from_placed(vec![
            Argument::NoWindowMode,
            script(),
            Argument::FailureToExitStatus,
            Argument::TheFile {
                path: "kitchen.blend".to_string(),
            },
            Argument::FrameList { list: frames(&[1]), from: SAVED },
        ]);
        assert!(matches!(
            before_the_switch,
            Err(NotALine::OutOfRankInsideItsPosition { .. })
        ));
        let without_it = LaunchLine::from_placed(vec![
            Argument::NoWindowMode,
            Argument::FailureToExitStatus,
            Argument::TheFile {
                path: "kitchen.blend".to_string(),
            },
            Argument::FrameList { list: frames(&[1]), from: SAVED },
        ]);
        assert_eq!(without_it, Err(NotALine::TheHeadIsIncomplete));
    }

    /// **A regular output path that is not text is refused and named, and
    /// never substituted** — a location is bytes on purpose (`Job files and
    /// outputs` R4a) and the bytes handed in are the ones the refusal
    /// carries, so the record names the cause rather than the mismatch it
    /// would have produced two steps later.
    #[test]
    fn a_regular_output_path_that_is_not_text_is_refused_and_never_substituted() {
        let mut not_text = inputs(None);
        not_text.regular_output_path = b"/out/fra\xffme_####".to_vec();
        let refused = LaunchLine::compose(not_text).unwrap_err();
        assert_eq!(
            refused,
            NotALine::TheRegularOutputPathIsNotText {
                value: b"/out/fra\xffme_####".to_vec()
            }
        );
        // A path that is text composes as it always did, byte for byte.
        let line = LaunchLine::compose(inputs(None)).unwrap();
        let words = line.as_words().unwrap();
        let at = words.iter().position(|word| word == "--render-output").unwrap();
        assert_eq!(words[at + 1], "/ephemeral/smoozy/jobs/j1/frame_####");
    }

    /// **L9, changed 2026-10-08: every composed line lets the file's own
    /// scripts run** — the switch once, right after the no-window mode and so
    /// before the file, in every version, on the cards and on the processor.
    /// A re-run is composed by the same [`LaunchLine::compose`] (L29: the
    /// line is composed per attempt), so it carries it too. **And a line
    /// without it is not a line.**
    #[test]
    fn every_composed_line_lets_the_files_own_scripts_run_once_before_the_file() {
        let switch = crate::spellings::SCRIPTS_RUN_SWITCH.text().unwrap();
        for version in TheFourVersions::ALL {
            for technology in [None, Some(Technology::Optix)] {
                let mut asked = inputs(technology);
                asked.version = version;
                let words = LaunchLine::compose(asked).unwrap().as_words().unwrap();
                assert_eq!(
                    words.iter().filter(|word| *word == switch).count(),
                    1,
                    "{version:?} {technology:?}: {words:?}"
                );
                assert_eq!(&words[..2], &["--background", switch], "{words:?}");
                let file = words
                    .iter()
                    .position(|word| word == "/ephemeral/smoozy/assets/kitchen.blend")
                    .unwrap();
                assert!(1 < file);
            }
        }
        let without_it = LaunchLine::from_placed(vec![
            Argument::NoWindowMode,
            Argument::FailureToExitStatus,
            Argument::LoadTimeScript {
                path: "load-time.py".to_string(),
            },
            Argument::TheFile {
                path: "kitchen.blend".to_string(),
            },
            Argument::FrameList { list: frames(&[1]), from: SAVED },
        ]);
        assert_eq!(without_it, Err(NotALine::TheHeadIsIncomplete));
    }

    /// **L17a, the fault the Creator's eleventh notes found: a video job of
    /// 1 to 90 is started as its first frame, then its last, then its step,
    /// then the animation instruction, all after the file** — after the
    /// material, as the render instruction, before the double dash — and never
    /// as a frame list, which wrote a one-frame video. **The step is handed
    /// every time, 1 included** (the video-range piece): without it a file saved at step 2
    /// wrote five frames of ten.
    #[test]
    fn a_video_job_of_one_to_ninety_is_its_first_its_last_its_step_then_the_animation_after_the_file() {
        let mut video = inputs(Some(Technology::Optix));
        video.frames = frames(&(1..=90).collect::<Vec<i64>>());
        video.a_destination_is_video_shaped = true;
        let line = LaunchLine::compose(video).unwrap();
        let words = line.as_words().unwrap();
        assert_eq!(
            words,
            vec![
                "--background",
                "--enable-autoexec",
                "--python-exit-code",
                "63",
                "--log-level",
                "info",
                "--python",
                "/ephemeral/smoozy/job-records/j1/load-time.py",
                "/ephemeral/smoozy/assets/kitchen.blend",
                "-S",
                "Kitchen",
                "--engine",
                "CYCLES",
                "--render-output",
                "/ephemeral/smoozy/jobs/j1/frame_####",
                "--python",
                "/ephemeral/smoozy/job-records/j1/material.py",
                "-s",
                "1",
                "-e",
                "90",
                "-j",
                "1",
                "-a",
                "--",
                &["--cycles", "-device"].concat(),
                "OPTIX",
            ]
        );
        assert!(!words.contains(&"--render-frame".to_string()));
        assert_eq!(
            line.placed()[line.the_window().1],
            Argument::AnimationOverOneRun {
                run: OneSteppedRun { first: 1, last: 90, step: 1 },
                from: SAVED
            }
        );
        // On the processor the instruction ends the line.
        let mut on_the_processor = inputs(None);
        on_the_processor.frames = frames(&(1..=90).collect::<Vec<i64>>());
        on_the_processor.a_destination_is_video_shaped = true;
        let words = LaunchLine::compose(on_the_processor).unwrap().as_words().unwrap();
        assert_eq!(&words[words.len() - 7..], &["-s", "1", "-e", "90", "-j", "1", "-a"]);
    }

    /// **L17a's step, the video-range piece: a video job keeps its own step** — typed, or
    /// the file's own range at its saved step, which reaches the line as an
    /// evenly stepped list. A step of 2 from 1 is `-j 2`, and a run that does
    /// not land on its typed last frame ends at the last it reaches.
    #[test]
    fn a_video_job_keeps_its_step_handed_before_the_animation() {
        let video_of = |asked: &[i64]| {
            let mut video = inputs(None);
            video.frames = frames(asked);
            video.a_destination_is_video_shaped = true;
            LaunchLine::compose(video).unwrap().as_words().unwrap()
        };
        let stepped = video_of(&[1, 3, 5, 7, 9]);
        assert_eq!(&stepped[stepped.len() - 7..], &["-s", "1", "-e", "9", "-j", "2", "-a"]);
        let thirds = video_of(&[10, 13, 16, 19]);
        assert_eq!(&thirds[thirds.len() - 7..], &["-s", "10", "-e", "19", "-j", "3", "-a"]);
        // **The step always stands before the animation instruction**: after
        // it, the second desk test found it ignored.
        for words in [stepped, thirds, video_of(&[7])] {
            let step = words.iter().position(|word| word == "-j").unwrap();
            let animation = words.iter().position(|word| word == "-a").unwrap();
            assert_eq!(step + 2, animation, "{words:?}");
            assert_eq!(words.iter().filter(|word| *word == "-j").count(), 1);
        }
    }

    /// **L17b, the video-range piece: a video's run below zero is written from the file's
    /// saved range and never as a bare negative** — the start after the saved
    /// start, the end before the saved end, a frame at or above zero as
    /// itself. On a file saved at −5 to 5: −3 to 0 is `-s +2 -e 0`, −4 to −2 is
    /// `-s +1 -e -7`, and the whole saved range is `-s +0 -e 5`. Until that
    /// piece a video starting below zero was refused.
    #[test]
    fn a_videos_run_below_zero_is_written_from_the_files_saved_range() {
        let video_of = |asked: &[i64], from: TheFilesSavedRange| {
            let mut video = inputs(None);
            video.frames = frames(asked);
            video.a_destination_is_video_shaped = true;
            video.the_files_saved_range = from;
            let words = LaunchLine::compose(video).unwrap().as_words().unwrap();
            words[words.len() - 7..].to_vec()
        };
        let saved = TheFilesSavedRange { start: -5, end: 5 };
        assert_eq!(video_of(&[-3, -2, -1, 0], saved), ["-s", "+2", "-e", "0", "-j", "1", "-a"]);
        assert_eq!(video_of(&[-4, -3, -2], saved), ["-s", "+1", "-e", "-7", "-j", "1", "-a"]);
        assert_eq!(
            video_of(&(-5..=5).collect::<Vec<i64>>(), saved),
            ["-s", "+0", "-e", "5", "-j", "1", "-a"]
        );
        let ends_below = TheFilesSavedRange { start: -5, end: -1 };
        assert_eq!(
            video_of(&[-5, -4, -3, -2, -1], ends_below),
            ["-s", "+0", "-e", "+0", "-j", "1", "-a"]
        );
        // At or above zero nothing changes, whatever the file's range.
        assert_eq!(video_of(&[0, 1, 2], saved), ["-s", "0", "-e", "2", "-j", "1", "-a"]);
    }

    /// **L17b, the video-range piece: a picture job's frames below zero are written after
    /// the file's saved start** — the `-3..0` it was handed until the video-range piece
    /// rendered nothing at all and reported success.
    #[test]
    fn a_picture_jobs_frames_below_zero_are_written_after_the_files_saved_start() {
        let mut pictures = inputs(None);
        pictures.frames = frames(&[-3, -2, -1, 0]);
        pictures.the_files_saved_range = TheFilesSavedRange { start: -5, end: 5 };
        let words = LaunchLine::compose(pictures).unwrap().as_words().unwrap();
        assert_eq!(&words[words.len() - 2..], &["--render-frame", "+2..+4,0"]);
    }

    /// **L17b at the gate: a frame below zero outside the scene's saved range
    /// is refused in words**, for a list and a video's run alike, and no line
    /// holding one can be placed by hand either.
    #[test]
    fn a_frame_below_zero_outside_the_scenes_range_is_refused_in_words() {
        for video in [false, true] {
            let mut asked = inputs(None);
            asked.frames = frames(&[-2, -1, 0, 1]);
            asked.a_destination_is_video_shaped = video;
            let refused = LaunchLine::compose(asked).unwrap_err();
            assert_eq!(
                refused,
                NotALine::AFrameBelowZeroOutsideTheScenesRange { frame: -2, start: 1, end: 250 }
            );
            assert_eq!(
                refused.in_words(),
                "frame -2 is below 0 and outside the scene's frame range, so it cannot be rendered"
            );
        }
        let by_hand = LaunchLine::from_placed(vec![
            Argument::NoWindowMode,
            Argument::ScriptsRun,
            Argument::FailureToExitStatus,
            Argument::LoadTimeScript {
                path: "load-time.py".to_string(),
            },
            Argument::TheFile {
                path: "kitchen.blend".to_string(),
            },
            Argument::AnimationOverOneRun {
                run: OneSteppedRun { first: -3, last: -1, step: 1 },
                from: TheFilesSavedRange { start: -2, end: 5 },
            },
        ]);
        assert_eq!(
            by_hand,
            Err(NotALine::AFrameBelowZeroOutsideTheScenesRange { frame: -3, start: -2, end: 5 })
        );
    }

    /// **L17 untouched: a job that writes pictures gets the explicit list
    /// exactly as before** — an unbroken run, a typed list out of order, a
    /// stepped list and one frame alike.
    #[test]
    fn a_picture_job_gets_the_frame_list_exactly_as_before() {
        for (asked, text) in [
            ((1..=90).collect::<Vec<i64>>(), "1..90"),
            (vec![40, 22, 23, 24, 25], "40,22..25"),
            (vec![1, 3, 5, 7], "1,3,5,7"),
            (vec![7], "7"),
        ] {
            let mut pictures = inputs(None);
            pictures.frames = frames(&asked);
            let line = LaunchLine::compose(pictures).unwrap();
            let words = line.as_words().unwrap();
            let at = words.iter().position(|word| word == "--render-frame").unwrap();
            assert_eq!(words[at + 1], text);
            assert_eq!(at + 2, words.len(), "the list ends a processor job's line");
            assert!(!words.contains(&"-a".to_string()), "{words:?}");
            assert!(!words.contains(&"-j".to_string()), "{words:?}");
            assert_eq!(
                line.placed()[line.the_window().1],
                Argument::FrameList {
                    list: frames(&asked),
                    from: SAVED
                }
            );
        }
    }

    /// **L17a: a video job whose frames have gaps is refused in words, never
    /// rendered as some other run** — a gap, a list out of order. A step is no
    /// gap (the video-range piece), and a run of one frame and a run from frame 0 are runs.
    #[test]
    fn a_video_job_with_a_gapped_list_is_refused_in_words() {
        let video_of = |asked: &[i64]| {
            let mut video = inputs(None);
            video.frames = frames(asked);
            video.a_destination_is_video_shaped = true;
            LaunchLine::compose(video)
        };
        for broken in [
            vec![1, 2, 3, 20, 21, 22],
            vec![1, 3, 4, 5],
            vec![3, 2, 1],
            vec![40, 22, 23, 24, 25],
        ] {
            let refused = video_of(&broken).unwrap_err();
            assert_eq!(
                refused,
                NotALine::AVideosFramesHaveGaps {
                    asked: broken.clone()
                }
            );
            let words = refused.in_words();
            assert_eq!(
                words,
                "this job writes a video, and its frames have gaps or are out of order"
            );
            assert!(!words.contains('{') && !words.contains("AVideo"), "{words}");
        }

        let one = video_of(&[7]).unwrap().as_words().unwrap();
        assert_eq!(&one[one.len() - 7..], &["-s", "7", "-e", "7", "-j", "1", "-a"]);
        let from_zero = video_of(&[0, 1, 2]).unwrap().as_words().unwrap();
        assert_eq!(&from_zero[from_zero.len() - 7..], &["-s", "0", "-e", "2", "-j", "1", "-a"]);
        let stepped = video_of(&[1, 3, 5, 7]).unwrap().as_words().unwrap();
        assert_eq!(&stepped[stepped.len() - 7..], &["-s", "1", "-e", "7", "-j", "2", "-a"]);
    }

    /// **The type holds one render instruction, whichever it is**: a list
    /// and an animation run together are not a line, and the run cannot sit
    /// before the material.
    #[test]
    fn a_line_carries_one_render_instruction_of_either_kind_and_only_at_its_place() {
        let head = || {
            vec![
                Argument::NoWindowMode,
                Argument::ScriptsRun,
                Argument::FailureToExitStatus,
                Argument::LoadTimeScript {
                    path: "load-time.py".to_string(),
                },
                Argument::TheFile {
                    path: "kitchen.blend".to_string(),
                },
            ]
        };
        let a_run = |first, last| Argument::AnimationOverOneRun {
            run: OneSteppedRun { first, last, step: 1 },
            from: SAVED,
        };
        let mut both = head();
        both.push(Argument::FrameList { list: frames(&[1]), from: SAVED });
        both.push(a_run(1, 1));
        assert_eq!(
            LaunchLine::from_placed(both),
            Err(NotALine::NotExactlyOneRenderInstruction)
        );
        let mut early = head();
        early.push(a_run(1, 9));
        early.push(Argument::LaunchMaterial {
            path: "material.py".to_string(),
        });
        assert!(matches!(
            LaunchLine::from_placed(early),
            Err(NotALine::OutOfOrder {
                earlier: Argument::AnimationOverOneRun { .. },
                later: Argument::LaunchMaterial { .. }
            })
        ));
    }

    /// **The line the video piece's real-Blender test at the desk runs**: a 10-frame
    /// video job, on 5.1, on the processor, in the factory scene, its engine
    /// the file's own. The desk test ran these words with the three paths
    /// put at its scratch folder and nothing else changed. **Since the video-range piece
    /// the step stands before the animation instruction.**
    #[test]
    fn the_ten_frame_video_line_the_desk_test_runs() {
        let line = LaunchLine::compose(LineInputs {
            version: TheFourVersions::V5_1,
            blend_path: "<scratch>/video.blend".to_string(),
            scene: Some("Scene".to_string()),
            engine: None,
            regular_output_path: b"<scratch>/out/v_".to_vec(),
            launch_material_path: "<scratch>/material.py".to_string(),
            load_time_script_path: "<scratch>/load-time.py".to_string(),
            frames: frames(&(1..=10).collect::<Vec<i64>>()),
            a_destination_is_video_shaped: true,
            the_files_saved_range: TheFilesSavedRange { start: 1, end: 250 },
            technology: None,
        })
        .unwrap();
        assert_eq!(
            line.as_words().unwrap(),
            vec![
                "--background",
                "--enable-autoexec",
                "--python-exit-code",
                "63",
                "--log-level",
                "info",
                "--python",
                "<scratch>/load-time.py",
                "<scratch>/video.blend",
                "-S",
                "Scene",
                "--render-output",
                "<scratch>/out/v_",
                "--python",
                "<scratch>/material.py",
                "-s",
                "1",
                "-e",
                "10",
                "-j",
                "1",
                "-a",
            ]
        );
    }

    #[test]
    fn every_variant_has_one_position_and_the_positions_are_ordered() {
        assert!(Position::TheHead < Position::TheFile);
        assert!(Position::TheFile < Position::TheWindow);
        assert!(Position::TheWindow < Position::TheRenderInstruction);
        assert!(Position::TheRenderInstruction < Position::TheBareDoubleDash);
        assert!(Position::TheBareDoubleDash < Position::CyclesOwn);
        const _: () = assert!(THE_THREAD_COUNT_EXEMPTION_IS_NOT_USED);
        const _: () = assert!(NO_FACTORY_STARTUP_AND_THE_FILES_SCRIPTS_RUN);
    }
}

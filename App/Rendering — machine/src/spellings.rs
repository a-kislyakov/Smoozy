//! Every launch argument, switch and property spelling this contract
//! composes from, in one place, each with the comparison question it is
//! read from and the label that question gives it — **and the device
//! argument's one declaration point, which L24a makes the only defence there
//! is.**
//!
//! **L24a, and it is the rule that binds this file.** After a bare double
//! dash the line is Cycles', and an argument name Cycles does not know is
//! swallowed without a word while the render goes ahead and ends at success
//! (*Settings 17*, *Reports 12*, with a control). L8 item 9 places the device
//! argument exactly there. **So a device argument whose name is wrong by one
//! character is dropped in silence, the file's own saved settings decide the
//! device, and a Cycles render on a machine whose backend selector arrives
//! as `NONE` runs on the processor, writes the picture and exits 0** — six
//! paid hours on the wrong silicon, and nothing afterwards can catch it,
//! because no read-back exists for a line Blender never parsed. **So the
//! check is at composition and nowhere else: the spelling is a constant of
//! this contract, per version, declared here, and is never composed from a
//! manifest value, a settings payload value, or anything a user can reach.**
//! A test reads this crate's own sources and fails on any line of code
//! outside this file that spells it.
//!
//! **Where each spelling is taken from, and it is the plan's own rule**: the
//! exact spelling of every launch argument and switch lives in the
//! comparison questions cited beside each clause (the plan's *Where a
//! Blender fact is looked up is fixed*), and where a frozen machine-evidence
//! document shows a form observed on a real box, that form is quoted with its
//! source. **Nothing here is a memory and nothing is a plausible
//! placeholder.** Where a spelling is not on record it is
//! [`Spelling::NotOnRecord`] — an absence a type can tell from a value, the
//! shape `App/Render Settings/src/spellings.rs` uses for its five — and no
//! composition can turn one into an argument.
//!
//! **Two property spellings carry a narrower label than the rest and say
//! so.** The skip-existing-frames switch and the regular output path's
//! read-back are read out of the comparisons by their property *names*
//! (`use_overwrite`, `filepath`), and their written-out form from a scene is
//! **inferred** in exactly the way *Settings 17* labels every written-out
//! form: *"the page gives the property, another page gives the group it
//! hangs on, and joining them is a reader's step."* Both hang off the
//! render settings and not off Cycles' own group, which is the one group in
//! the one version where a wrong name is swallowed in silence (L10a) — so a
//! wrong written-out form here is refused loudly on the terminal and L11
//! with L8a ends the attempt with nothing written. That is the fail-loud
//! direction, and it is why they are cited rather than held absent.
//!
//! **Markers used in this file that belong to another document, each with
//! the document it belongs to.** `Job files and outputs` D7, D10, D11,
//! D16. `Render Settings` C7, C9. `Dependency Upload & Remap` M8.

use crate::versions::TheFourVersions;

/// **The 5.3 lookup of 2026-10-02 — the source of every 5.3 arm below.** The
/// Creator's own install, `/Applications/Blender 5.3.0a.app`, which answered
/// *5.3.0 Alpha*, build hash `68609be8e23b`, built 2026-10-01, run with no
/// window beside 5.2.2 (`/Applications/Blender.app`, hash `d13f752e3b9c`):
/// its help text, its whole property lists for every structure the
/// composition and the launch material reach, and Smoozy's composed line's
/// shape rendering two frames on the processor. **Every name here, and every
/// name the launch material and `Render Settings`' pairs use, read the same in
/// both**; the help text differs by one added argument this contract does not
/// use; the device line, the frame and save announcements and the failure
/// status came back identical. Label: **observed in a test**, at the desk, on
/// one Mac, macOS on Apple silicon — never on Linux, where Smoozy renders.
/// Recorded at `Audits/Build/Step 10c/The fixes after the Vast pass before
/// 0.1.9 — what landed.md`, item 3, and in the plan's companion sources file
/// under its own entry for this lookup.
pub const THE_5_3_LOOKUP: &str =
    "the 5.3.0 Alpha lookup, 2026-10-02, on the Creator's own install, hash 68609be8e23b (the plan's Rendering sources file, under its own entry)";

/// **The 5.4 lookup of 2026-10-18 — the source of every 5.4 arm below.** The
/// 5.4.0 Alpha from Blender's build service
/// (`blender-5.4.0-alpha+main.3f4216e1bad1-darwin.arm64-release.dmg`), copied
/// to `/Applications/Blender 5.4a.app`, which answered *5.4.0 Alpha*, build
/// hash `3f4216e1bad1`, built 2026-10-18, run with no window beside the 5.3.0
/// Beta (`/Applications/Blender 5.3b.app`, hash `5d8151335b6b`): its help text,
/// the whole property lists of every structure either registers, every name
/// the composition, the launch material and the reading script use, and a real
/// render through this driver on the processor. **Nothing used was removed or
/// changed**; the help text differs only in its version line; the device
/// argument changed the device a render used, and the device line, the frame
/// and save announcements and the failure status came back in 5.3's shape.
/// Label: **observed in a test**, at the desk, on one Mac, macOS on Apple
/// silicon — never on Linux, where Smoozy renders. Recorded in
/// `Audits/Build/Step 10c/`, in the account of the thirteenth notes' piece for
/// 5.4, and in the plan's companion sources file under its own entry for this
/// lookup.
pub const THE_5_4_LOOKUP: &str =
    "the 5.4.0 Alpha lookup, 2026-10-18, off Blender's build service, hash 3f4216e1bad1 (the plan's Rendering sources file, under its own entry)";

/// One spelling, as an argument or a pair is composed from it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Spelling {
    /// On record: read out of a comparison question, or observed on a real
    /// box in a frozen evidence document, and cited to it.
    Cited {
        /// The exact text the line or the material carries.
        text: &'static str,
        /// The comparison question or evidence document it is read from.
        source: &'static str,
        /// The label that source gives it, in the research's own words.
        label: &'static str,
    },
    /// **An explicit absence.** No composition can make an argument or a
    /// pair from it; what it names is what is missing.
    NotOnRecord(MissingSpelling),
}

/// What is missing, named so a refusal can say it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MissingSpelling {
    pub what: &'static str,
    pub the_clause: &'static str,
}

impl Spelling {
    /// The text, or the absence that stops the composition.
    pub fn text(&self) -> Result<&'static str, MissingSpelling> {
        match self {
            Spelling::Cited { text, .. } => Ok(text),
            Spelling::NotOnRecord(missing) => Err(*missing),
        }
    }
}

/// One value per supported version. **Every lookup goes through
/// [`PerVersion::for_version`]**, so a version has exactly one spelling and
/// nothing composes across the four.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PerVersion<T> {
    pub v4_5: T,
    pub v5_0: T,
    pub v5_1: T,
    pub v5_2: T,
    pub v5_3: T,
    pub v5_4: T,
}

impl<T: Copy> PerVersion<T> {
    pub fn for_version(&self, version: TheFourVersions) -> T {
        match version {
            TheFourVersions::V4_5 => self.v4_5,
            TheFourVersions::V5_0 => self.v5_0,
            TheFourVersions::V5_1 => self.v5_1,
            TheFourVersions::V5_2 => self.v5_2,
            TheFourVersions::V5_3 => self.v5_3,
            TheFourVersions::V5_4 => self.v5_4,
        }
    }

    /// The same value in all four that were read together, **and nothing
    /// for 5.3 or 5.4**: each of their arms is written out where it is used,
    /// with its own source, so a value cited to the four is never inherited by
    /// a fifth or a sixth.
    pub const fn the_same_in_all_four(value: T, v5_3: T, v5_4: T) -> PerVersion<T> {
        PerVersion {
            v4_5: value,
            v5_0: value,
            v5_1: value,
            v5_2: value,
            v5_3,
            v5_4,
        }
    }
}

// ---------------------------------------------------------------------------
// The head of the line — process options, in force before the file (L3, L8a)
// ---------------------------------------------------------------------------

/// **L3: the no-window mode, and it is not optional.** *The machine 7*, in
/// all four: *"There is a mode with no interface at all and it is meant for
/// this: `-b` / `--background`, 'often used for UI-less rendering', needing
/// no graphical display and workable over a remote shell."* Observed on a
/// real rented box as `blender -b file.blend …` (`The rented machine`) and
/// as `blender --background scene.blend --render-output <folder>/f_
/// --render-anim` (*Add-ons 12*). The long form is composed; *Writing 13*
/// records the long and short forms behaving identically.
pub const NO_WINDOW_MODE: Spelling = Spelling::Cited {
    text: "--background",
    source: "The machine 7; observed in Add-ons 12 and on The rented machine",
    label: "documented in all four; observed on a rented Linux box",
};

/// **L9: the switch that lets the file's own scripts run, at the head of the
/// line, right after the no-window mode** (added 2026-10-08, the Creator's
/// ruling 4 and *Later again* in `Audits/Build/Step 10c/The Creator's rulings
/// on add-ons and a file's scripts, 2026-10-08.md`). Blender's own command-line
/// page, quoted in *Settings* (5.1 findings): *"`-y, --enable-autoexec` —
/// Enable automatic Python script execution."*, the opposite being the
/// default. Where it sits is taken from the security page's worked example,
/// quoted there: `blender --background --enable-autoexec my_movie.blend
/// --render-anim` — after the no-window mode, before the file. The long form
/// is composed, as the example writes it. **Every render carries it, a
/// re-run's included, and so do both readings** (`Rendering` spec, *The
/// file's own scripts run*; `Render Settings` V3).
pub const SCRIPTS_RUN_SWITCH: Spelling = Spelling::Cited {
    text: "--enable-autoexec",
    source: "Settings (5.1 findings): the command-line page and the security page's example; Rendering L9",
    label: "documented; observed at the desk 2026-10-08 in 4.5.12 and 5.1.2, with and without a factory startup",
};

/// **L8a: the failure-to-exit-status switch, at the head of the line.**
/// *Reports 12*, all four versions, both machines: *"With `--python-exit-code
/// 42` placed before the script argument the run gives 42, the render does
/// not happen … Placed after the script argument the option does nothing."*
/// And the controlled test of 2026-08-19 (`Render Settings` sources item 5):
/// *"With `--python-exit-code 63` on the line, both versions ended at exit
/// 63 with no frame written."*
pub const FAILURE_TO_EXIT_STATUS_SWITCH: Spelling = Spelling::Cited {
    text: "--python-exit-code",
    source: "Reports 12; Render Settings sources item 5",
    label: "observed in a test, all four versions on both machines",
};

/// **L8a: the fixed non-zero number this contract chooses, and it is never
/// read back for a cause** (K6). It is the number the observation used, so
/// the observed positive and the composed line carry the same value; any
/// other non-zero number would do, and nothing anywhere reads it.
pub const THE_FAILURE_STATUS: u8 = 63;

// ---------------------------------------------------------------------------
// The device-line ask (L26) — the version's own switch on the version's own
// stream
// ---------------------------------------------------------------------------

/// **L26: every render is asked for the line that names what did the work,
/// by the version's own switch.** *What actually ran 1*: **4.5** — *"no log
/// switch reaches the line at all. `--debug-cycles` is the only route, and it
/// lands on the error stream"*; **5.0, 5.1 and 5.2** — *"`--log-level info`
/// on its own is enough, and it lands on standard output."* Run in all four
/// versions on both machines.
///
/// **A value travels with the switch where the switch takes one**, and the
/// value is the comparison's own word.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TheAsk {
    pub switch: Spelling,
    pub value: Option<&'static str>,
    pub on_stream: WhichStreamTheLineLandsOn,
}

/// Which stream the version's route prints the device line on — L7's
/// reason both streams are captured, always.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhichStreamTheLineLandsOn {
    TheComplaints,
    TheOrdinaryOutput,
}

pub const DEVICE_LINE_ASK: PerVersion<TheAsk> = PerVersion {
    v4_5: TheAsk {
        switch: Spelling::Cited {
            text: "--debug-cycles",
            source: "What actually ran 1",
            label: "observed in a test",
        },
        value: None,
        on_stream: WhichStreamTheLineLandsOn::TheComplaints,
    },
    v5_0: THE_LOG_LEVEL_ASK,
    v5_1: THE_LOG_LEVEL_ASK,
    v5_2: THE_LOG_LEVEL_ASK,
    // **5.3: the same switch on the same stream**, read off the 5.3.0 Alpha —
    // `--log-level info` printed the device line on standard output, word for
    // word 5.2.2's shape.
    v5_3: TheAsk {
        switch: Spelling::Cited {
            text: "--log-level",
            source: THE_5_3_LOOKUP,
            label: "observed in a test",
        },
        value: Some("info"),
        on_stream: WhichStreamTheLineLandsOn::TheOrdinaryOutput,
    },
    // **5.4: the same switch on the same stream**, read off the 5.4.0 Alpha —
    // `--log-level info` printed the device line on standard output in 5.3's
    // shape, through this driver's own line.
    v5_4: TheAsk {
        switch: Spelling::Cited {
            text: "--log-level",
            source: THE_5_4_LOOKUP,
            label: "observed in a test",
        },
        value: Some("info"),
        on_stream: WhichStreamTheLineLandsOn::TheOrdinaryOutput,
    },
};

const THE_LOG_LEVEL_ASK: TheAsk = TheAsk {
    switch: Spelling::Cited {
        text: "--log-level",
        source: "What actually ran 1",
        label: "observed in a test",
    },
    value: Some("info"),
    on_stream: WhichStreamTheLineLandsOn::TheOrdinaryOutput,
};

// ---------------------------------------------------------------------------
// Inside the window — after the file, before the render instruction, before
// the bare double dash (L8)
// ---------------------------------------------------------------------------

/// **L8 item 3, L15: the scene argument.** *Settings 1*: *"which scene
/// (`-S`)"*, among the fifteen that *"appear with the same names and short
/// forms in every version's help text on both machines"*. The short form is
/// composed because it is the form the comparison carries.
pub const SCENE_ARGUMENT: Spelling = Spelling::Cited {
    text: "-S",
    source: "Settings 1; What a render demands 3",
    label: "documented in all four, observed on both machines",
};

/// **L8 item 4, L19: the engine argument.** *Settings 1*: *"a render engine
/// (`-E` / `--engine`)"*.
pub const ENGINE_ARGUMENT: Spelling = Spelling::Cited {
    text: "--engine",
    source: "Settings 1; What a render demands 3",
    label: "documented in all four, observed on both machines",
};

/// **L8 item 5, `Job files and outputs` D10: the regular output path's own
/// argument.** *Settings 1*: *"the output path (`-o`)"*; *Reports 12* quotes
/// Blender's own complaint naming both forms, *"`-o  / --render-output`"*;
/// observed as `--render-output` on a real box (*Add-ons 12*) and as `-o`
/// on `The rented machine`.
pub const REGULAR_OUTPUT_PATH_ARGUMENT: Spelling = Spelling::Cited {
    text: "--render-output",
    source: "Settings 1; Reports 12; observed in Add-ons 12",
    label: "documented in all four, observed on both machines",
};

/// **L8 item 7, L10: the launch material, handed in as a script file.**
/// *Reports 12*: *"with `--python` the render goes ahead and the picture is
/// written although the script failed"* — which is the observation L8a's
/// switch exists against, and the argument's spelling with it. A file
/// rather than an expression, because L12 confirms the material's placement
/// by reading it back under the render's login.
pub const LAUNCH_MATERIAL_ARGUMENT: Spelling = Spelling::Cited {
    text: "--python",
    source: "Reports 12",
    label: "observed in a test, all four versions on both machines",
};

/// **L8 item 8, L17, L18: the explicit frame list, which is the render
/// instruction and ends the window.** *Settings 1*: *"a list of separate
/// frames (`-f` / `--render-frame`, which takes a comma list, a `..` range,
/// and both mixed in one argument)"*; *Frames 7*: *"`-f 5,1,9` gave 5, then
/// 1, then 9, in all four on both"*, *"`-f 1..3,10..12` was accepted with no
/// error and wrote all six files"*. **This is the animation form**: *Writing
/// 8*'s naming rules — the frame number in the name — *"were taken from a
/// command-line frame render, a command-line animation render and a Python
/// animation render"*, and the one form that leaves the number out is the
/// Python still call, which this contract never uses (L1).
pub const FRAME_LIST_ARGUMENT: Spelling = Spelling::Cited {
    text: "--render-frame",
    source: "Settings 1; Frames 7; Writing 8",
    label: "documented in all four, observed on both machines",
};

/// **L17a: the render instruction of a job with a video-shaped destination —
/// the job's first frame, then its last, then the animation instruction, all
/// after the file.** Added 2026-10-17, the Creator's eleventh notes, note 1,
/// the video piece. Read off the build runner's desk test of 2026-10-17 in
/// `Audits/Build/Step 10c/`, *how a video is written whole*, a controlled
/// test in Blender 4.5.12, 5.0.1, 5.1.2, 5.2.2 and 5.3.0 Beta: *`-s 1 -e 10 -a`* wrote a video holding 10 frames
/// named `0001-0010`, from a file saved at 1 to 250, in every one; a frame
/// list (`-f`) left a one-frame video, and a first and last placed after the
/// animation instruction were ignored. **The short forms are composed because
/// they are the forms that test ran**; Blender's own help names `-s` /
/// `--frame-start`, `-e` / `--frame-end` and `-a` / `--render-anim` in 4.5.12
/// and 5.3.0 (the same test's table of names). **The three are one
/// instruction and their order is part of it**, so they are composed together
/// by one argument of the line and never apart.
pub const FRAME_START_ARGUMENT: Spelling = Spelling::Cited {
    text: "-s",
    source: "the video desk test of 2026-10-17 (Rendering L17a); the help text of 4.5.12 and 5.3.0",
    label: "observed in a test in 4.5.12, 5.0.1, 5.1.2, 5.2.2 and 5.3.0 Beta, on a Mac",
};

pub const FRAME_END_ARGUMENT: Spelling = Spelling::Cited {
    text: "-e",
    source: "the video desk test of 2026-10-17 (Rendering L17a); the help text of 4.5.12 and 5.3.0",
    label: "observed in a test in 4.5.12, 5.0.1, 5.1.2, 5.2.2 and 5.3.0 Beta, on a Mac",
};

pub const ANIMATION_RENDER_INSTRUCTION: Spelling = Spelling::Cited {
    text: "-a",
    source: "the video desk test of 2026-10-17 (Rendering L17a); the help text of 4.5.12 and 5.3.0",
    label: "observed in a test in 4.5.12, 5.0.1, 5.1.2, 5.2.2 and 5.3.0 Beta, on a Mac",
};

/// **L17a's step: how many frames a video job's run moves on by, handed every
/// time, 1 included, before the animation instruction.** Added 2026-10-17, the
/// Creator's eleventh notes, the video-range piece. Read off the build runner's second desk
/// test of 2026-10-17 in `Audits/Build/Step 10c/`, *the step on a video and
/// frames below zero*, a controlled test in Blender 4.5.12, 5.0.1, 5.1.2, 5.2.2
/// and 5.3.0 Beta: the first and last frame alone keep the file's saved step
/// (*`-s 1 -e 10 -a`* on a file saved at step 2 wrote 5 frames), and
/// *`-s 1 -e 10 -j 1 -a`* on that file wrote 10, *`-j 2`* on a step-1 file 5;
/// **placed after the animation instruction it was ignored.** Every one of the
/// five helps reads, word for word: *"-j or --frame-jump <frames> Set number of
/// frames to step forward after each rendered frame."* The short form is the
/// one the test ran.
pub const FRAME_STEP_ARGUMENT: Spelling = Spelling::Cited {
    text: "-j",
    source: "the second desk test of 2026-10-17 (Rendering L17a's step); the help text of all five Blenders it ran",
    label: "observed in a test in 4.5.12, 5.0.1, 5.1.2, 5.2.2 and 5.3.0 Beta, on a Mac",
};

// ---------------------------------------------------------------------------
// After the window — the bare double dash and Cycles' own line (L8 item 9,
// L21, L24a)
// ---------------------------------------------------------------------------

/// **L8 item 9: the bare double dash, after which the line is Cycles'.**
/// *The machine 5*: the device option is *"only read when a bare `--` comes
/// first"*; *Settings 9*: *"The argument only works after a bare double
/// dash."*
pub const THE_BARE_DOUBLE_DASH: &str = "--";

/// **L24a: the device argument's spelling, per version, at its one
/// declaration point.** *The machine 5*: *"One command-line option,
/// `--cycles-device`, taking exactly the same six values in every version —
/// CPU, CUDA, OPTIX, HIP, ONEAPI, METAL — with `+CPU` available to add the
/// processor, and only read when a bare `--` comes first. Its help text is
/// worded identically in all four."* Observed on a real rented box in
/// 4.5.12 and 5.2.0 (`Six machines`, Fact 3): *"The observed spelling of the
/// argument is `--cycles-device`."* And Blender's own command-line page,
/// read 2026-07-30: *"--cycles-device <device> … Valid options are: CPU
/// CUDA OPTIX HIP ONEAPI METAL."*
///
/// **Declared per version even though the four are the same**, because the
/// rule is per version and a fifth version has no arm here rather than an
/// inherited one.
/// The argument's text, written once so the scan below finds one declaration
/// whatever the number of arms citing it.
const THE_DEVICE_ARGUMENTS_TEXT: &str = "--cycles-device";

pub const DEVICE_ARGUMENT: PerVersion<Spelling> = PerVersion::the_same_in_all_four(
    Spelling::Cited {
        text: THE_DEVICE_ARGUMENTS_TEXT,
        source: "The machine 5; Six machines, Fact 3; Blender's command-line page read 2026-07-30",
        label: "documented in all four with the help text worded identically; observed on real boxes in 4.5.12 and 5.2.0",
    },
    // **5.3**: the 5.3.0 Alpha's own help text names it with the same six
    // values and the same bare double dash before it, and the composed line
    // rendered on the processor with it.
    Spelling::Cited {
        text: THE_DEVICE_ARGUMENTS_TEXT,
        source: THE_5_3_LOOKUP,
        label: "the help text worded as 5.2.2's; observed in a test on the processor",
    },
    // **5.4**: the 5.4.0 Alpha's own help text names it with the same six
    // values and the same bare double dash before it, word for word 5.3's, and
    // after a bare double dash it moved a render saved for the processor onto
    // the Mac's own graphics.
    Spelling::Cited {
        text: THE_DEVICE_ARGUMENTS_TEXT,
        source: THE_5_4_LOOKUP,
        label: "the help text worded as 5.3.0's; observed in a test to change the device",
    },
);

// ---------------------------------------------------------------------------
// The material's own property spellings (L10)
// ---------------------------------------------------------------------------

/// **L10: the skip-existing-frames switch, imposed off on every attempt.**
/// The switch is Blender's *Overwrite*: *Frames 4* — *"switch Overwrite
/// off, and a frame is skipped when a file already exists at the path that
/// frame would be written to"*; *Dependencies 17* — *"Present and reading the
/// same in all four: `use_render_cache`, `use_overwrite` (true),
/// `use_placeholder` (false)"*. **Skipping off is Overwrite on**, so the pair
/// assigns the switch its on value, whatever the artist saved. The written-
/// out form from a scene is inferred as this file's header states.
pub const SKIP_EXISTING_FRAMES_SWITCH: Spelling = Spelling::Cited {
    text: "scene.render.use_overwrite",
    source: "Dependencies 17 (the name); Frames 4 and Frames 14 (what it does)",
    label: "observed in all four on both machines for the name and the conduct; the written-out form inferred",
};

/// **The value the switch is set to: on — skipping off.** A switch, and the
/// same value on every attempt in every version.
pub const SKIP_EXISTING_FRAMES_OFF_IS_OVERWRITE_ON: bool = true;

/// **L10's read-back, `Job files and outputs` D10: the regular output path
/// in force, read inside the material.** *Reading a file 4*: *"the scene's
/// render output path is a readable property"* in all four; the property's
/// name is `filepath` on the render settings, as *Dependencies 9*'s own
/// transcript sets it (*"set render.filepath = …"*); and what it reads back
/// is *"the raw text handed in, template marks and all"* (*Writing 13*).
/// The written-out form from a scene is inferred as this file's header
/// states.
pub const REGULAR_OUTPUT_PATH_READ_BACK: Spelling = Spelling::Cited {
    text: "scene.render.filepath",
    source: "Reading a file 4; Dependencies 9; Writing 13",
    label: "documented in all four for the property; observed for the read-back; the written-out form inferred",
};

/// **A1d, L10: Simplify's switch and three render-half settings, as the
/// plan names them out of *What a render demands 6*** — the switch
/// `scene.render.use_simplify`; maximum subdivision
/// `scene.render.simplify_subdivision_render`; child particles
/// `scene.render.simplify_child_particles_render`; and the texture size
/// limit `scene.cycles.texture_limit_render`, *"the texture pair belonging
/// to Cycles, sitting on the scene's Cycles block by its own path, and
/// present whatever the active engine."* **The viewport half of each pair is
/// never read, never composed and never assigned** — there is no constant
/// for it here.
pub const SIMPLIFY_SWITCH: Spelling = Spelling::Cited {
    text: "scene.render.use_simplify",
    source: "What a render demands 6, through A1d",
    label: "documented, the same in all four versions",
};

pub const SIMPLIFY_SUBDIVISION_RENDER: Spelling = Spelling::Cited {
    text: "scene.render.simplify_subdivision_render",
    source: "What a render demands 6, through A1d",
    label: "documented, the same in all four versions",
};

pub const SIMPLIFY_CHILD_PARTICLES_RENDER: Spelling = Spelling::Cited {
    text: "scene.render.simplify_child_particles_render",
    source: "What a render demands 6, through A1d",
    label: "documented, the same in all four versions",
};

/// **The one Simplify pair on Cycles' own group** — the group in which 4.5
/// swallows a wrong name in silence (L10a), so this spelling is the one the
/// name check exists for.
pub const SIMPLIFY_TEXTURE_LIMIT_RENDER: Spelling = Spelling::Cited {
    text: "scene.cycles.texture_limit_render",
    source: "What a render demands 6, through A1d",
    label: "observed from Cycles' own files, the same in all four versions",
};

/// **Every property spelling this crate composes a pair from, as one
/// list**, so the test below can hold that each begins at the job's scene
/// (`Render Settings` C8's assignment target) and none names a viewport
/// half.
pub const EVERY_PROPERTY_SPELLING_HERE: [Spelling; 6] = [
    SKIP_EXISTING_FRAMES_SWITCH,
    REGULAR_OUTPUT_PATH_READ_BACK,
    SIMPLIFY_SWITCH,
    SIMPLIFY_SUBDIVISION_RENDER,
    SIMPLIFY_CHILD_PARTICLES_RENDER,
    SIMPLIFY_TEXTURE_LIMIT_RENDER,
];

/// **The other pair spellings this contract composes are `Render
/// Settings`' own and are taken from that crate** — C7 for the percentage,
/// C9 for the engine counts, the layer override and the pin, and its five
/// not-yet-read ones for the imposition set and the processor override.
/// Nothing of theirs is copied here.
pub const THE_SETTINGS_PAIRS_SPELLINGS_ARE_RENDER_SETTINGS_OWN: bool = true;

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn every_launch_spelling_is_cited_and_none_is_an_absence() {
        for spelling in [
            NO_WINDOW_MODE,
            SCRIPTS_RUN_SWITCH,
            FAILURE_TO_EXIT_STATUS_SWITCH,
            SCENE_ARGUMENT,
            ENGINE_ARGUMENT,
            REGULAR_OUTPUT_PATH_ARGUMENT,
            LAUNCH_MATERIAL_ARGUMENT,
            FRAME_LIST_ARGUMENT,
            FRAME_START_ARGUMENT,
            FRAME_END_ARGUMENT,
            FRAME_STEP_ARGUMENT,
            ANIMATION_RENDER_INSTRUCTION,
        ] {
            let Spelling::Cited { text, source, .. } = spelling else {
                panic!("{spelling:?} is not on record");
            };
            assert!(text.starts_with('-'), "{text}");
            assert!(!source.is_empty());
        }
        assert_ne!(THE_FAILURE_STATUS, 0, "L8a: a fixed non-zero number");
    }

    /// **L24a's one declaration point, held.** The device argument is
    /// declared for every version, every arm is cited, and the four arms
    /// carry the spelling *The machine 5* records as worded identically.
    #[test]
    fn the_device_argument_is_declared_for_all_four_versions_and_cited_in_each() {
        // Built by joining two halves, so this test is not itself a second
        // spelling of the argument for the scan below to find.
        let the_recorded_spelling = ["--cycles", "-device"].concat();
        for version in TheFourVersions::ALL {
            let Spelling::Cited { text, source, .. } = DEVICE_ARGUMENT.for_version(version) else {
                panic!("{version:?}: the device argument is not on record");
            };
            assert_eq!(text, the_recorded_spelling, "{version:?}");
            if version == TheFourVersions::V5_3 {
                assert_eq!(source, THE_5_3_LOOKUP);
            } else if version == TheFourVersions::V5_4 {
                assert_eq!(source, THE_5_4_LOOKUP);
            } else {
                assert!(source.contains("The machine 5"), "{version:?}");
            }
        }
    }

    /// **The spelling is composed from nowhere but here.** Every `.rs` file
    /// of this crate is read, comment lines dropped, and any line of code
    /// outside this file that spells the argument's text fails the test —
    /// so a second spelling, a per-job composition, or a literal in a test
    /// standing in for the constant cannot be written without this failing.
    #[test]
    fn no_line_of_code_outside_this_file_spells_the_device_argument() {
        let this_file = Path::new(file!())
            .file_name()
            .and_then(|name| name.to_str())
            .expect("this file has a name")
            .to_string();
        let the_text = ["--cycles", "-device"].concat();
        let mut declarations = 0;
        for (path, text) in crate::markers::tests::every_source_of_this_crate() {
            let name = Path::new(&path)
                .file_name()
                .and_then(|n| n.to_str())
                .expect("the file has a name")
                .to_string();
            for line in text.lines() {
                let trimmed = line.trim_start();
                if trimmed.starts_with("//") {
                    continue;
                }
                if trimmed.contains(&the_text) {
                    assert_eq!(
                        name, this_file,
                        "{name} spells the device argument outside its one declaration point: {line}"
                    );
                    declarations += 1;
                }
            }
        }
        assert_eq!(declarations, 1, "the spelling is declared exactly once");
    }

    #[test]
    fn the_ask_is_the_versions_own_switch_on_the_versions_own_stream() {
        let older = DEVICE_LINE_ASK.for_version(TheFourVersions::V4_5);
        assert_eq!(older.switch.text(), Ok("--debug-cycles"));
        assert_eq!(older.value, None);
        assert_eq!(older.on_stream, WhichStreamTheLineLandsOn::TheComplaints);
        for version in [
            TheFourVersions::V5_0,
            TheFourVersions::V5_1,
            TheFourVersions::V5_2,
            TheFourVersions::V5_3,
            TheFourVersions::V5_4,
        ] {
            let ask = DEVICE_LINE_ASK.for_version(version);
            assert_eq!(ask.switch.text(), Ok("--log-level"), "{version:?}");
            assert_eq!(ask.value, Some("info"), "{version:?}");
            assert_eq!(ask.on_stream, WhichStreamTheLineLandsOn::TheOrdinaryOutput);
        }
    }

    #[test]
    fn every_property_spelling_begins_at_the_jobs_scene_and_none_names_a_viewport_half() {
        for spelling in EVERY_PROPERTY_SPELLING_HERE {
            let Spelling::Cited { text, .. } = spelling else {
                panic!("{spelling:?}");
            };
            assert!(text.starts_with("scene."), "{text}");
            // A1d: the render half is set and the viewport half is left as
            // saved. A viewport half has no `_render` suffix; the switch
            // and the two read-backs are not halves of a pair at all.
            if text.contains("simplify_") || text.contains("texture_limit") {
                assert!(text.ends_with("_render"), "{text} names a viewport half");
            }
        }
        const _: () = assert!(SKIP_EXISTING_FRAMES_OFF_IS_OVERWRITE_ON);
    }

    #[test]
    fn an_absence_stops_a_composition_and_names_what_is_missing() {
        let absent = Spelling::NotOnRecord(MissingSpelling {
            what: "a spelling nobody read",
            the_clause: "L8",
        });
        assert_eq!(
            absent.text(),
            Err(MissingSpelling {
                what: "a spelling nobody read",
                the_clause: "L8"
            })
        );
    }
}

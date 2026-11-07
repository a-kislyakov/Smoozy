//! **R1's script: the one thing that runs inside Blender to read a `.blend`,
//! and the shape of what it prints.**
//!
//! **Nothing Smoozy builds could render until this existed.** Every render
//! attempt ended at the render's step 4 with the reading not taken, because no
//! step of the build had owned the script and none had written it.
//!
//! **One run, no window, everything out of one read** (R1), printing one
//! document between the two marks [`crate::reading::THE_OPENING_SENTINEL`] and
//! [`crate::reading::THE_CLOSING_SENTINEL`] because Blender's own chatter
//! shares that stream. **It is written once and used on both sides** (R3) — the
//! machine's render start and the desktop's pre-load — so nothing in it may
//! assume which side called it, and nothing in it does.
//!
//! **Everything it needs is handed in as a data document rather than written
//! into its code**, which is the shape `Rendering`'s launch material already
//! uses: one JSON object at the head, read by the script's first line.
//! **Every Blender name the script uses that moves with the version is in that
//! document**, composed here from the cited table below, so a name lives at one
//! declaration point and a version that has no name for a thing has an absence
//! rather than a neighbour's name.
//!
//! **Locations go out as bytes and are never decoded** (R4a). The script puts
//! every location and every piece of the artist's own text back through
//! `os.fsencode` — the reverse of the decoding that produced the string — and
//! writes the bytes out one number at a time, which is the same representation
//! `Dependency Upload & Remap`'s script already uses and the same one
//! [`crate::reading::RecordedLocation`] reads back.
//!
//! # The started mark, and then the composed-for guard
//!
//! **The script's very first act is to write [`crate::reading::THE_STARTED_MARK`]**
//! (`Render Settings` Q8, added 2026-10-02 for the Local View crash), so a
//! reading that goes wrong anywhere inside the script is told apart from a
//! file the Blender could not open — that second kind never runs the script.
//! **The guard is the first thing after it.**
//!
//! **The script is composed for one version and refuses to run in another.**
//! The property carrying a file output node's per-output format switch changed
//! **name and sense together at 5.0 while keeping the same starting value**, so
//! a script carrying 4.5's name and sense into 5.x reads every output backwards
//! and never raises. The guard is what makes that impossible: the run ends at
//! the failure status with no opening mark, which is no reading at all — and,
//! the started mark standing, `Render Settings` Q8's third ending.
//!
//! # Every Blender name the script uses, with where it was read
//!
//! **Rule 2 binds here hardest, because a name written from memory is a reading
//! that fails on every file — or, worse, one that succeeds and is wrong.**
//! Every name below is read from a running Blender or cited to a comparison
//! through a spellings file that cites its own. **Nothing here is written from
//! memory and nothing is taken from a near neighbour.**
//!
//! Three sources, and no fourth:
//!
//! - **[`THE_LOOKUP`]** — the property-name lookup of 2026-09-20, four running
//!   Blenders at the desk, recorded at `Audits/Build/Step 10c/The property-name
//!   lookup — what was read.md`.
//! - **[`THIS_PIECES_LOOKUP`]** — the same mechanism, run again the same day
//!   for the names this script needs that that record does not carry, recorded
//!   at `Audits/Build/Step 10c/Piece 13 — what landed.md`. Whole property lists
//!   were printed off each of the four and then read against what was wanted;
//!   nothing was matched inside Blender.
//! - **A spellings file that cites its own** — `Render
//!   Settings — machine/src/spellings.rs` and `Rendering — machine/src/spellings.rs`,
//!   which carry the names those two documents' own clauses assign against,
//!   each cited to a comparison question or to the lookup. **This crate cannot
//!   reach either**, the dependency running the other way, so the name is
//!   written here with the constant it matches named beside it and a test in
//!   this file holds the pair.
//! - **[`THE_NAME_START_DESK_TEST`]** — added 2026-10-31 for P18a's mark: a
//!   controlled test in six Blenders, 4.5 to 5.4, of which setting starts each
//!   written file's name, every property name read off that Blender's own
//!   list before it was set. It names no new property; it says which of the
//!   names above holds a name's start.
//!
//! **The names, by what the script does with them.**
//!
//! - **The scene's compositing tree.** `scene.node_tree` in 4.5, *"Compositing
//!   node tree"*; **from 5.0 a scene carries no `node_tree` at all** and the
//!   name is `scene.compositing_node_group`, *"Compositor Nodes"*
//!   ([`THE_LOOKUP`], confirmed by [`THIS_PIECES_LOOKUP`]).
//! - **From 5.3, the stack of effects** (R4 item 1): `scene.compositor_effects`,
//!   each effect's `node_group` and `enable_for_render`, 5.4 the same
//!   ([`THE_EFFECTS_LOOKUP`]). **That route reads no older name**, which is the
//!   third side of [`AcrossTheBoundary`].
//! - **The tree's final output node.** `CompositorNodeComposite`, labelled
//!   *Composite*, described *"Final render output"* — **present in 4.5 and in
//!   no later version**; `NodeGroupOutput`, labelled *Group Output*, present in
//!   all four ([`THIS_PIECES_LOOKUP`], which listed every node type each build
//!   registers). That is R4 item 1's own statement met by enumeration.
//! - **The group boundary.** `NodeGroupInput` and `NodeGroupOutput`, all four;
//!   a group node carries `node_tree` ([`THIS_PIECES_LOOKUP`]).
//! - **The render's entry point.** `CompositorNodeRLayers`, with `scene` and
//!   `layer`, all four ([`THIS_PIECES_LOOKUP`]).
//! - **The file output node.** `CompositorNodeOutputFile`, labelled *File
//!   Output*, all four ([`THE_LOOKUP`]).
//! - **A node's own parts**: `bl_idname`, `name`, `mute`, `inputs`, `outputs`,
//!   `format`; a tree's `nodes` and `links`; a link's `from_node`, `to_node`,
//!   `from_socket`, `to_socket`; a socket's `name`, `identifier`, `type`,
//!   `is_linked` — all four, unchanged ([`THIS_PIECES_LOOKUP`]).
//! - **The node's folder and its base file name.** `base_path` in 4.5;
//!   `directory` and `file_name` from 5.0 ([`THE_LOOKUP`]; D8 observed the
//!   first pair on disk).
//! - **The node's outputs.** `file_slots`, each a `NodeOutputFileSlotFile`
//!   carrying `path`, in 4.5; `file_output_items`, each a
//!   `NodeCompositorFileOutputItem` carrying `name`, from 5.0
//!   ([`THIS_PIECES_LOOKUP`] for the two lists' members; [`THE_LOOKUP`] for the
//!   list names).
//! - **The per-output format switch, and this is the dangerous one.**
//!   `use_node_format` in 4.5, starting **off**, where **off means this output
//!   has its own format**; `override_node_format` from 5.0, starting **off**,
//!   where **off means this output uses the node's format** ([`THE_LOOKUP`]).
//!   **Same starting value, opposite meaning**, which is why the sense travels
//!   in the handed-in document beside the name and why the guard above exists.
//! - **The gathering mode** (I7). 4.5 reads it off the format itself —
//!   `file_format` at `OPEN_EXR_MULTILAYER`; from 5.0 an image format carries
//!   `media_type`, *"The type of media to save"*, and the value is
//!   `MULTI_LAYER_IMAGE` ([`THIS_PIECES_LOOKUP`]).
//! - **The trailing socket a new output is made on, from 5.0.** A file output
//!   node's last input is of kind `CUSTOM`, which is **not among the kinds an
//!   output entry can be**, so the outputs are the sockets of every other kind
//!   ([`THIS_PIECES_LOOKUP`]). 4.5 carries no such socket.
//! - **The gating conditions** (R4 item 1). `scene.render.use_compositing`,
//!   labelled *Compositing*, all four ([`THE_LOOKUP`]); `scene.use_nodes`, all
//!   four, read only in 4.5 because it decides nothing from 5.0;
//!   `scene.render.save_output`, *"Write frames to disk for animation
//!   renders"*, **5.2 alone**, which is D12's switch
//!   ([`THIS_PIECES_LOOKUP`]).
//! - **The sequencer.** `scene.render.use_sequencer`,
//!   `scene.sequence_editor`, its `strips_all` — *"All strips, recursively
//!   including those inside metastrips"* — a strip's `type` with the value
//!   `SCENE`, and a scene strip's `scene` ([`THE_LOOKUP`] for the editor,
//!   [`THIS_PIECES_LOOKUP`] for the rest).
//! - **The regular output path and its name for a frame.**
//!   `scene.render.filepath`, which `Rendering — machine/src/spellings.rs`
//!   carries as `REGULAR_OUTPUT_PATH_READ_BACK` cited to its own comparisons;
//!   and `scene.render.frame_path(frame=…)`, *"Return the absolute path to the
//!   filename to be written for a given frame"*, all four
//!   ([`THIS_PIECES_LOOKUP`]). That second one is I1's asking. **For a video
//!   it answers one name built from the scene's start and end at the moment it
//!   is asked, whatever the frame**, and `-s 1 -e 2 -a` on a file saved at 1
//!   to 12 writes the 1–2 name — so a job that hands its frames in has its
//!   video's name asked with the scene's start and end set to its first and
//!   last frame, and put back at once (the live Telegram pass's fault 4,
//!   2026-10-31; measured in six Blenders, 4.5 to 5.4, `Audits/Build/Step
//!   10c/A desk test — the name of a video over part of its range.md`).
//! - **The format a destination writes.** `image_settings.file_format`;
//!   `scene.render.use_file_extension` and `scene.render.file_extension`,
//!   *"The file extension used for saving renders"*; `scene.render.
//!   is_movie_format`, *"When true the format is a movie"*; and, **5.2 alone**,
//!   a file output node's own `use_file_extension` ([`THIS_PIECES_LOOKUP`]).
//! - **Stepping to a frame.** `scene.frame_set(frame)`, *"Set scene frame
//!   updating all objects and view layers immediately"*, an act and not a
//!   property, all four ([`THE_LOOKUP`]).
//! - **Whether a scene is stepped at all** (added 2026-10-02, the Creator's
//!   notes on 0.1.8, piece 2). `animation_data` on the scene and on a node
//!   tree, both of kind `AnimData`; `scene.timeline_markers`, of kind
//!   `TimelineMarker`, and a marker's `camera`, an object, *"Camera that
//!   becomes active on this frame"* — all four, unchanged
//!   ([`THE_STEPPING_LOOKUP`]).
//! - **The scene the reading read** (the 38's piece 10, item 25).
//!   `bpy.context.scene`, which the script has always read the job's scene
//!   from where the job names none, and a scene's `name`, *"Unique data-block
//!   ID name (within a same type and library)"* — all four
//!   ([`THE_SCENE_NAME_LOOKUP`]).
//! - **The frames and the size.** `scene.frame_start`, `frame_end`,
//!   `frame_step`; `scene.render.resolution_x` and `resolution_y`
//!   ([`THIS_PIECES_LOOKUP`]); `resolution_percentage`, which `Render
//!   Settings — machine/src/spellings.rs` carries as `RESOLUTION_PERCENTAGE`.
//! - **The view layers.** `scene.view_layers`, a layer's `name`, its `use` and
//!   its `samples` — that last *"Override number of render samples for this
//!   view layer, **0 will use the scene setting**"*, which is where *whether it
//!   overrides the scene's* is read from, in Blender's own words
//!   ([`THIS_PIECES_LOOKUP`]; the two spellings are `Render
//!   Settings — machine/src/spellings.rs`'s `VIEW_LAYER_USE` and
//!   `VIEW_LAYER_SAMPLES`).
//! - **The animation route, per version** (`Render Settings` V9). An
//!   animation's `animation_data`, its `action` and its `drivers`; **`fcurves`
//!   hangs off the action in 4.5 and is absent from the action in 5.0, 5.1 and
//!   5.2**, where the curves are reached by `layers`, then `strips`, then
//!   `channelbags`, then `fcurves`; a curve's `data_path`, `keyframe_points`
//!   and `driver`; a key's `co` and `interpolation` with the value `CONSTANT`;
//!   a driver's `expression` and `is_valid` ([`THIS_PIECES_LOOKUP`], which
//!   found the action's own `fcurves` present in 4.5 and absent in the other
//!   three — V9's statement met by enumeration).
//! - **The views** (I9a). `scene.render.use_multiview`, `views_format` and
//!   `views` with a view's `use`; and an image format's own `views_format`,
//!   whose value `INDIVIDUAL` is the one that writes a file per view
//!   ([`THE_LOOKUP`] for the first pair, [`THIS_PIECES_LOOKUP`] for the rest).
//!   **The two `views_format` name different settings and are told apart only
//!   by what they hang off**, which the lookup record states outright.
//! - **The engines** (`Render Settings` Q2 item 4). The engine property's own
//!   enumeration, and `bpy.types.RenderEngine.__subclasses__()` with each
//!   class's `bl_idname`. **Read in all four and the negative reproduced**: the
//!   enumeration offers only the built-in identifier — `BLENDER_EEVEE_NEXT` in
//!   4.5, `BLENDER_EEVEE` in the other three — and the class walk offers
//!   `CYCLES` and one class with no identifier at all
//!   ([`THIS_PIECES_LOOKUP`]).
//! - **The file's own saved-with version.** `bpy.data.version`, *"File format
//!   version the .blend file was saved with"* ([`THE_LOOKUP`]). **Not
//!   `bpy.app.version_file`**, which is the running program's own number and
//!   answers the same for every file it opens.
//! - **The running program's version.** `bpy.app.version`, which the *Add-ons*
//!   comparison carries.
//! - **Blender's own three path marks**, which item 6's walk is made of:
//!   `is_path_output`, `is_path_supports_blend_relative` and
//!   `is_path_supports_templates`, on the description of a text property, all
//!   four ([`THE_LOOKUP`]). The walk reaches them through `bl_rna`,
//!   `properties`, `identifier`, `type` and `fixed_type`
//!   ([`THIS_PIECES_LOOKUP`]).
//! - **D20's four added write places.** `scene.render.bake.filepath`, *"Image
//!   filepath to use when saving externally"*; a point cache's `filepath`,
//!   its `point_caches`, and its `use_disk_cache`, `use_external` and
//!   `is_baked`, read and never written (`Job files and outputs` D21b-i, read
//!   off 4.5.12 and 5.2.2 at the desk 2026-10-02, and the material's own
//!   walk); `bake_directory` on a nodes modifier and `directory`
//!   on each of its `bakes`; and `proxy_dir` on the sequence editor with
//!   `directory` on a strip's `proxy` ([`THE_LOOKUP`] for the two folders on
//!   R9's list, [`THIS_PIECES_LOOKUP`] for the rest).
//! - **D20's fifth added write place, added 2026-10-02.** A modifier's `type`
//!   `FLUID`, its `fluid_type` `DOMAIN`, its `domain_settings`, and on those
//!   `cache_type` `REPLAY` and `cache_directory` — read off all four
//!   (`crate::redirect::THE_FLUID_CACHE_LOOKUP`).
//! - **R4 item 8, added 2026-10-02.** A modifier's `type` `NODES`, its
//!   `bakes`, each bake's `node` and that node's `bl_idname`
//!   `GeometryNodeSimulationOutput`; `Scene.rigidbody_world`, `Scene.objects`;
//!   the five carriers of a `PointCache` and its `is_baked`; a particle
//!   system's `settings`, `use_hair_dynamics` and `ParticleSettings.type`
//!   `HAIR` — read off all four (`crate::reading::THE_SIMULATION_LOOKUP`).
//! - **The Cycles and Simplify settings the settings half carries**, each
//!   matching a constant of `Render Settings — machine/src/spellings.rs` or
//!   `Rendering — machine/src/spellings.rs`: `scene.cycles.samples`,
//!   `scene.eevee.taa_render_samples`, `scene.cycles.device`,
//!   `scene.cycles.use_adaptive_sampling`, `adaptive_threshold`,
//!   `adaptive_min_samples`, `time_limit`, `use_layer_samples`,
//!   `scene.render.use_simplify`, `simplify_subdivision_render`,
//!   `simplify_child_particles_render` and `scene.cycles.texture_limit_render`.
//!
//! # The three absences, named rather than guessed at
//!
//! **A name with no source is an absence written down, never a near
//! neighbour.** Three things the shapes ask for have no reader here, and each
//! is a stated absence rather than a value composed out of nothing.
//!
//! - **Which of a two-picture node's inputs came out.** B3's fourth limit says
//!   it can be told *"for five named kinds of two-picture node and for no
//!   others"*, and [`crate::main_output`] states outright that **the five are
//!   not named in this document and are not named here**. So the script records
//!   no answer, and [`crate::main_output::walk_the_beauty`] stops the walk at
//!   every two-picture node, which is what B3 asks of a limit.
//! - **Whether a two-picture node's factor sits between its ends** (B3's third
//!   limit). The factor socket has no name this project has read. The script
//!   records *no*, and **nothing turns on it**, because the fourth limit above
//!   already stops the walk at the same node. It is recorded here so a later
//!   reader does not take the *no* for an observation.
//! - **The one add-on case a file announces** (`Render Settings` E5). What form
//!   it takes to a run with no window is not established, and that document
//!   writes in no second mechanism, so the script records nothing and reads
//!   nothing Blender printed.
//!
//! # What this file does not do, and where it goes instead
//!
//! **[`crate::reading::the_document_in`] does not parse what this script
//! prints, and that is not an oversight.** [`crate::reading::TheOutputsHalf`]
//! carries `the_beauty_walk`, which is R4 item **7** — a derived thing, not
//! something a file can be asked — and the walk that derives it is
//! [`crate::main_output::walk_the_beauty`], in Rust, with its own tests. **A
//! second copy of that walk written in Python would be the one-fact-in-two-
//! places fault this stage keeps paying for.** So the script prints what a file
//! can actually say — items 1 to 6, with the compositing tree among them — and
//! [`the_one_document_from`] composes [`crate::reading::TheOneDocument`] from it
//! by running the walk that already exists. `the_document_in` stays what it is:
//! the reader of a **recorded** reading, which is what a manifest carries.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Render Settings` Q1, Q2, Q3, Q7, Q8, V9, E5.
//! `Dependency Upload & Remap` G3.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::main_output::{TheCompositingTree, WhatTheWalkFound, WhereTheRenderEnters};
use crate::manifest::CarriedAndNotInterpretedHere;
use crate::reading::{
    ArtistsText, Destination, GatingConditions, HowManyViewsTheSceneWrites, RecordedLocation,
    SceneReachedThroughAStrip, TheInvocation, TheOneDocument, TheOutputsHalf, TheVersionInHand,
    WhichBranch, WhyTheDocumentWasNotRead, AtOneFrame, THE_CLOSING_SENTINEL, THE_OPENING_SENTINEL,
    THE_STARTED_MARK,
};

/// **The property-name lookup of 2026-09-20** — four running Blenders at the
/// desk, started with no window and from factory settings, printing whole
/// property lists. Its record is the audit file named here.
pub const THE_LOOKUP: &str =
    "the property-name lookup, 2026-09-20 (Audits/Build/Step 10c/The property-name lookup — what was read.md)";

/// **This piece's own run of the same lookup, 2026-09-20**, for the names the
/// record above does not carry. Same four Blenders, same shape, whole lists
/// printed and read afterwards; its record is the audit file named here.
pub const THIS_PIECES_LOOKUP: &str =
    "piece 13's own property-name lookup, 2026-09-20 (Audits/Build/Step 10c/Piece 13 — what landed.md)";

/// **The lookup of 2026-10-02 for the names that decide whether a scene is
/// stepped** — the same four Blenders, started with no window and from
/// factory settings, printing the property lists of `Scene`, `TimelineMarker`,
/// `NodeTree` and `AnimData`. Like the other lookups added that day, it is
/// cited in place rather than joining the composition's own table, because
/// the names are written in the script's body and are the same in all four.
pub const THE_STEPPING_LOOKUP: &str = "the stepping lookup, 2026-10-02 \
     (Audits/Build/Step 10c/The Creator's notes on 0.1.8, piece 2 — what landed.md)";

/// One Blender name the script is composed from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Spelling {
    /// Read off a running Blender, or carried by a spellings file that cites
    /// its own, and cited to it here.
    Cited {
        /// The name exactly as the script writes it.
        name: &'static str,
        /// Where it was read.
        source: &'static str,
    },
    /// **An explicit absence.** No script can be composed from it; what it
    /// names is what is missing.
    NotOnRecord(MissingSpelling),
}

/// What is missing, named so a refusal can say it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MissingSpelling {
    pub what: &'static str,
    pub the_clause: &'static str,
}

impl Spelling {
    /// The name, or the absence that stops the composition.
    pub fn name(&self) -> Result<&'static str, MissingSpelling> {
        match self {
            Spelling::Cited { name, .. } => Ok(name),
            Spelling::NotOnRecord(missing) => Err(*missing),
        }
    }
}

/// **A name that is one thing in 4.5 and another from 5.0**, or that one side
/// of the boundary has and the other does not.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AcrossTheBoundary {
    pub four_five: Option<Spelling>,
    pub from_five_zero: Option<Spelling>,
    /// **The third side, from 5.3.** Where a scene holds a stack of effects, a
    /// few names part from 5.0 to 5.2's; every other name is the same.
    pub from_five_three: TheThirdSide,
}

/// **What a name does from 5.3** ([`TheVersionInHand::holds_a_stack_of_effects`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TheThirdSide {
    TheSameAsFromFiveZero,
    /// Its own spelling, or an absence meaning **this version's route reads no
    /// such thing**.
    ItsOwn(Option<Spelling>),
}

impl AcrossTheBoundary {
    /// The name this version uses, or an absence meaning **this version has no
    /// such thing**, which is a state the script reads rather than a fault.
    pub fn for_version(
        self,
        version: TheVersionInHand,
    ) -> Result<Option<&'static str>, MissingSpelling> {
        let side = match version.branch() {
            WhichBranch::FourFive => self.four_five,
            WhichBranch::FromFiveZero => match self.from_five_three {
                TheThirdSide::ItsOwn(own) if version.holds_a_stack_of_effects() => own,
                _ => self.from_five_zero,
            },
        };
        match side {
            None => Ok(None),
            Some(spelling) => spelling.name().map(Some),
        }
    }

    /// The same name on both sides.
    pub const fn the_same_in_all_four(name: &'static str, source: &'static str) -> AcrossTheBoundary {
        let cited = Spelling::Cited { name, source };
        AcrossTheBoundary {
            four_five: Some(cited),
            from_five_zero: Some(cited),
            from_five_three: TheThirdSide::TheSameAsFromFiveZero,
        }
    }
}

// ---------------------------------------------------------------------------
// The names, at their one declaration point
// ---------------------------------------------------------------------------

/// **The desk reading of the stack's names**, on 5.3.0 Beta and 5.4.0 Alpha with
/// 5.2.3 as the control, each read off that Blender's own property lists:
/// `Audits/Build/Blender 5.3/Piece 1, the desk readings — what landed.md`,
/// reading 1. 5.3's and 5.4's words agree; 5.2.3 has none of the three.
pub const THE_EFFECTS_LOOKUP: &str = "the 5.3 integration's desk reading, 2026-11-02 \
     (Audits/Build/Blender 5.3/Piece 1, the desk readings — what landed.md, reading 1)";

/// **`Render Settings` Q2 item 1, from 5.3: EEVEE's time limit**, on the scene's
/// EEVEE settings, *"Limit the render time. Zero disables the limit."*
pub const THE_EEVEE_TIME_LIMIT: AcrossTheBoundary = AcrossTheBoundary {
    four_five: None,
    from_five_zero: None,
    from_five_three: TheThirdSide::ItsOwn(Some(Spelling::Cited {
        name: "time_limit",
        source: THE_EFFECTS_LOOKUP,
    })),
};

/// **The scene's compositing tree.** 4.5 alone has `node_tree`; from 5.0 the
/// name is `compositing_node_group` and a scene has no `node_tree` at all.
///
/// **From 5.3 this route reads no such thing** (R4 item 1): that name answers
/// only the active effect's group, Blender marks it for removal in 6.0, and
/// reading it puts a line on the Errors stream that the user reads as Blender's.
pub const THE_SCENES_COMPOSITING_TREE: AcrossTheBoundary = AcrossTheBoundary {
    four_five: Some(Spelling::Cited {
        name: "node_tree",
        source: THE_LOOKUP,
    }),
    from_five_zero: Some(Spelling::Cited {
        name: "compositing_node_group",
        source: THE_LOOKUP,
    }),
    from_five_three: TheThirdSide::ItsOwn(None),
};

/// **R4 item 1, from 5.3: a scene's stack of effects**, *"Compositor effects for
/// this scene"*.
pub const THE_SCENES_EFFECTS: AcrossTheBoundary = AcrossTheBoundary {
    four_five: None,
    from_five_zero: None,
    from_five_three: TheThirdSide::ItsOwn(Some(Spelling::Cited {
        name: "compositor_effects",
        source: THE_EFFECTS_LOOKUP,
    })),
};

/// **An effect's group**, *"Node group that controls what this effect does"*.
pub const AN_EFFECTS_GROUP: AcrossTheBoundary = AcrossTheBoundary {
    four_five: None,
    from_five_zero: None,
    from_five_three: TheThirdSide::ItsOwn(Some(Spelling::Cited {
        name: "node_group",
        source: THE_EFFECTS_LOOKUP,
    })),
};

/// **An effect's render switch**, *"Use effect during render"*: a node writes
/// only where its effect has it on (*File output nodes 13*).
pub const AN_EFFECTS_RENDER_SWITCH: AcrossTheBoundary = AcrossTheBoundary {
    four_five: None,
    from_five_zero: None,
    from_five_three: TheThirdSide::ItsOwn(Some(Spelling::Cited {
        name: "enable_for_render",
        source: THE_EFFECTS_LOOKUP,
    })),
};

/// **The name the walk over the marks must not step on, from 5.3.** That walk
/// goes through every pointer on a scene whose type it reaches, so it would read
/// the older one-group name and warn on every file (`Piece 1, the desk readings
/// — what landed.md`, reading 2). Every effect's group is reached through the
/// stack and the file's own list of groups, so nothing is lost by stepping over
/// it. Before 5.3 the list is [`THE_WALK_STEPS_OVER`] as it was.
pub fn the_walk_steps_over(version: TheVersionInHand) -> Result<Vec<&'static str>, MissingSpelling> {
    let mut over = THE_WALK_STEPS_OVER.to_vec();
    if version.holds_a_stack_of_effects() {
        if let Some(cited) = THE_SCENES_COMPOSITING_TREE.from_five_zero {
            over.push(cited.name()?);
        }
    }
    Ok(over)
}

/// **The tree's final output node** — R4 item 1's *live final output node*. The
/// Composite node in 4.5, which no later version registers at all; the Group
/// Output node from 5.0, the scene's tree being a node group there.
pub const THE_FINAL_OUTPUT_NODES_TYPE: AcrossTheBoundary = AcrossTheBoundary {
    four_five: Some(Spelling::Cited {
        name: "CompositorNodeComposite",
        source: THIS_PIECES_LOOKUP,
    }),
    from_five_zero: Some(Spelling::Cited {
        name: "NodeGroupOutput",
        source: THIS_PIECES_LOOKUP,
    }),
    from_five_three: TheThirdSide::TheSameAsFromFiveZero,
};

/// **B3's first limit's two ends**: the picture arriving inside a group and the
/// picture leaving it.
pub const THE_GROUPS_OWN_INPUT_TYPE: AcrossTheBoundary =
    AcrossTheBoundary::the_same_in_all_four("NodeGroupInput", THIS_PIECES_LOOKUP);
pub const THE_GROUPS_OWN_OUTPUT_TYPE: AcrossTheBoundary =
    AcrossTheBoundary::the_same_in_all_four("NodeGroupOutput", THIS_PIECES_LOOKUP);

/// **B1's entry point**, which names the scene and the view layer it reads.
pub const THE_RENDER_LAYERS_NODES_TYPE: AcrossTheBoundary =
    AcrossTheBoundary::the_same_in_all_four("CompositorNodeRLayers", THIS_PIECES_LOOKUP);

/// **The file output node**, whose outputs are R4 item 2's second kind.
pub const THE_FILE_OUTPUT_NODES_TYPE: AcrossTheBoundary =
    AcrossTheBoundary::the_same_in_all_four("CompositorNodeOutputFile", THE_LOOKUP);

/// **4.5's old Use Nodes switch, which decides nothing from 5.0** (R4 item 1),
/// so the reading records it only where the version has one.
pub const THE_FOUR_FIVE_USE_NODES_SWITCH: AcrossTheBoundary = AcrossTheBoundary {
    four_five: Some(Spelling::Cited {
        name: "use_nodes",
        source: THE_LOOKUP,
    }),
    from_five_zero: None,
    from_five_three: TheThirdSide::TheSameAsFromFiveZero,
};

/// **D12's switch, from 5.2 alone**: it stops the scene's own frames being
/// written even when a path is handed in on the command line, while a node in
/// the same tree keeps writing. Held apart from the boundary because it arrives
/// later than the boundary does.
pub const THE_SAVE_OUTPUT_SWITCH: Spelling = Spelling::Cited {
    name: "save_output",
    source: THIS_PIECES_LOOKUP,
};

/// **D8: the property carrying a node's folder is not named the same across the
/// boundary**, and the redirect must know which.
pub const THE_NODES_FOLDER: AcrossTheBoundary = AcrossTheBoundary {
    four_five: Some(Spelling::Cited {
        name: "base_path",
        source: THE_LOOKUP,
    }),
    from_five_zero: Some(Spelling::Cited {
        name: "directory",
        source: THE_LOOKUP,
    }),
    from_five_three: TheThirdSide::TheSameAsFromFiveZero,
};

/// **I2: from 5.0 the node's own base file name sits between the folder and the
/// output's name.** 4.5 has no such value — an absence, and not an empty name.
pub const THE_NODES_BASE_FILE_NAME: AcrossTheBoundary = AcrossTheBoundary {
    four_five: None,
    from_five_zero: Some(Spelling::Cited {
        name: "file_name",
        source: THE_LOOKUP,
    }),
    from_five_three: TheThirdSide::TheSameAsFromFiveZero,
};

/// **The list carrying one entry per output**, and the entry's own piece of
/// path. The two lists hold different types and the piece is named differently
/// on each.
pub const THE_NODES_OUTPUT_LIST: AcrossTheBoundary = AcrossTheBoundary {
    four_five: Some(Spelling::Cited {
        name: "file_slots",
        source: THE_LOOKUP,
    }),
    from_five_zero: Some(Spelling::Cited {
        name: "file_output_items",
        source: THE_LOOKUP,
    }),
    from_five_three: TheThirdSide::TheSameAsFromFiveZero,
};

pub const AN_OUTPUTS_OWN_PIECE_OF_PATH: AcrossTheBoundary = AcrossTheBoundary {
    four_five: Some(Spelling::Cited {
        name: "path",
        source: THIS_PIECES_LOOKUP,
    }),
    from_five_zero: Some(Spelling::Cited {
        name: "name",
        source: THIS_PIECES_LOOKUP,
    }),
    from_five_three: TheThirdSide::TheSameAsFromFiveZero,
};

/// **The per-output format switch, whose name and sense both moved at 5.0 while
/// the starting value stayed the same.**
///
/// 4.5's `use_node_format` starts **off**, and off means *this output has its
/// own format*. From 5.0 `override_node_format` starts **off**, and off means
/// *this output uses the node's format*. **A reader carrying one version's pair
/// into the other reads every output backwards and never raises**, which is why
/// the sense travels beside the name and why the script refuses to run in a
/// version it was not composed for.
pub const THE_PER_OUTPUT_FORMAT_SWITCH: AcrossTheBoundary = AcrossTheBoundary {
    four_five: Some(Spelling::Cited {
        name: "use_node_format",
        source: THE_LOOKUP,
    }),
    from_five_zero: Some(Spelling::Cited {
        name: "override_node_format",
        source: THE_LOOKUP,
    }),
    from_five_three: TheThirdSide::TheSameAsFromFiveZero,
};

/// **Whether the switch above being on means the output carries its own
/// format.** False in 4.5 and true from 5.0 — the reversal, as a value.
pub fn the_switch_is_on_when_the_output_has_its_own_format(version: TheVersionInHand) -> bool {
    version.branch() == WhichBranch::FromFiveZero
}

/// **I7's mode**, read off the format. 4.5 reads the format itself; from 5.0 an
/// image format carries a separate media setting and the mode is its own value.
pub const THE_GATHERING_MARKS_PROPERTY: AcrossTheBoundary = AcrossTheBoundary {
    four_five: Some(Spelling::Cited {
        name: "file_format",
        source: THIS_PIECES_LOOKUP,
    }),
    from_five_zero: Some(Spelling::Cited {
        name: "media_type",
        source: THIS_PIECES_LOOKUP,
    }),
    from_five_three: TheThirdSide::TheSameAsFromFiveZero,
};

pub const THE_GATHERING_MARKS_VALUE: AcrossTheBoundary = AcrossTheBoundary {
    four_five: Some(Spelling::Cited {
        name: "OPEN_EXR_MULTILAYER",
        source: THIS_PIECES_LOOKUP,
    }),
    from_five_zero: Some(Spelling::Cited {
        name: "MULTI_LAYER_IMAGE",
        source: THIS_PIECES_LOOKUP,
    }),
    from_five_three: TheThirdSide::TheSameAsFromFiveZero,
};

/// **The media setting, where the version has one**, which is also what has to
/// be set before a format can be asked for its ending from 5.0: the list of
/// formats a scene offers depends on it.
pub const THE_MEDIA_KIND: AcrossTheBoundary = AcrossTheBoundary {
    four_five: None,
    from_five_zero: Some(Spelling::Cited {
        name: "media_type",
        source: THIS_PIECES_LOOKUP,
    }),
    from_five_three: TheThirdSide::TheSameAsFromFiveZero,
};

/// **A file output node's own ending switch, from 5.2 alone.** Before 5.2 the
/// scene's is the only one there is.
pub const A_NODES_OWN_ENDING_SWITCH: Spelling = Spelling::Cited {
    name: "use_file_extension",
    source: THIS_PIECES_LOOKUP,
};

/// **`Render Settings` V9's route, as a value.** In 4.5 an action's curves hang
/// directly off the action; from 5.0 the action carries no `fcurves` at all and
/// the curves are reached through its layers, strips and channel bags.
pub fn the_curves_hang_off_the_action(version: TheVersionInHand) -> bool {
    version.branch() == WhichBranch::FourFive
}

/// **The kind a socket carrying a picture is**, which is how a node's shape is
/// told for B3's limits.
pub const A_PICTURE_SOCKETS_KIND: AcrossTheBoundary =
    AcrossTheBoundary::the_same_in_all_four("RGBA", THIS_PIECES_LOOKUP);

/// **The kind of the trailing socket a new output is made on, from 5.0**, which
/// is not among the kinds an output entry can be. 4.5 carries no such socket
/// and nothing there is of this kind either, so the test is harmless in both.
pub const THE_KIND_NO_OUTPUT_CAN_BE: AcrossTheBoundary =
    AcrossTheBoundary::the_same_in_all_four("CUSTOM", THIS_PIECES_LOOKUP);

/// **The identifier the engine-class walk gives Cycles** (`Render Settings`
/// Q2 item 4), which `Render Settings — machine/src/engine.rs` also carries.
pub const THE_CYCLES_IDENTIFIER: AcrossTheBoundary =
    AcrossTheBoundary::the_same_in_all_four("CYCLES", THIS_PIECES_LOOKUP);

/// **The properties item 6's walk steps over rather than into.** **Everything
/// else is walked**, and which properties carry a location is Blender's own
/// three marks and never a list kept here.
///
/// **The first five lead back to the whole program rather than to a place a
/// file writes**: a thing's own description of itself, its unevaluated twin,
/// the evaluated graph, a preview picture, and the weak reference to the
/// application's own files.
///
/// **The rest crash Blender when the walk reads them, and this is the one list
/// of them.** Added 2026-10-02, the Local View crash
/// (`Audits/Build/Step 10c/The Local View crash — what landed.md`). Each is
/// written with the file and the Blenders it crashed in, *observed at the
/// desk*; none leads to a place a render writes.
///
/// - **`local_view`** — a 3D viewport's own stored view while it shows Local
///   View. Reading it ended Blender by a crash (status 139) on the Creator's
///   own file, saved with 5.1 and left in Local View, in 4.5.12, 5.0.1, 5.1.2
///   and 5.2.2; and on a factory scene put into Local View and saved by each
///   of the four, in all four.
pub const THE_WALK_STEPS_OVER: &[&str] = &[
    "rna_type",
    "original",
    "depsgraph",
    "preview",
    "library_weak_reference",
    "local_view",
];

/// **The name of the scene the reading makes to ask a format for its ending,
/// and removes again.** Blender offers no route from an image format to the
/// ending it writes; the only route is a scene's own, so the reading makes one
/// scene in the running program's memory, asks it, and removes it before
/// anything else reads `bpy.data`. **The job's own scenes are never written
/// to**, and nothing is ever saved.
pub const THE_ASKING_SCENES_NAME: &str = "smoozy-reading-asking";

/// **The Blender test of 2026-10-27 for R4c**: Blender 4.5.15 and 5.2.3 on
/// Linux, the Blender Foundation's own builds of Blender as a Python module, a
/// factory scene set to write an MPEG-4 video switched to a still picture for
/// one render. Its record is the audit file named here; *observed in a test*.
pub const THE_VIDEO_WANTS_TEST: &str = "the video wants' Blender test, 2026-10-27 \
     (Audits/Build/Step 10c/The video wants — what landed.md, The Blender test)";

/// **R4c: the media setting's value for a still picture, from 5.0.** Set
/// before the format, because from 5.0 the formats a scene offers follow it:
/// with the media setting at `VIDEO` the picture format was refused outright
/// (`enum "PNG" not found in ('FFMPEG')`), and set to `IMAGE` it made the
/// format PNG at once ([`THE_VIDEO_WANTS_TEST`]). 4.5 has no media setting
/// ([`THE_MEDIA_KIND`]).
pub const A_STILL_PICTURES_MEDIA_KIND: AcrossTheBoundary = AcrossTheBoundary {
    four_five: None,
    from_five_zero: Some(Spelling::Cited {
        name: "IMAGE",
        source: THE_VIDEO_WANTS_TEST,
    }),
    from_five_three: TheThirdSide::TheSameAsFromFiveZero,
};

/// **R4c: the format a first-frame check of a video job is written in** —
/// PNG, the scene's own starting format in every version (*"THE SCENE STARTS
/// ON: file_format=PNG media_type=IMAGE"*, the Blender research's *File output
/// nodes* hand test), and the one the switch was seen to write a picture in
/// ([`THE_VIDEO_WANTS_TEST`]).
pub const THE_CHECKS_PICTURE_FORMAT: AcrossTheBoundary =
    AcrossTheBoundary::the_same_in_all_four("PNG", THE_VIDEO_WANTS_TEST);

/// **P18a's desk test, 2026-10-31**: Blender 4.5.12, 5.0.1, 5.1.2, 5.2.3,
/// 5.3.0 beta and 5.4.0 alpha at the desk, each started with no window and
/// from factory settings on a small scene made by script, every name read off
/// that Blender's own property list before it was set, and every written file
/// listed afterwards. Its record is the audit file named here; *observed in a
/// test*.
pub const THE_NAME_START_DESK_TEST: &str = "the desk test of where a name's start lands, \
     2026-10-31 (Audits/Build/Step 10c/A desk test — where a name's start lands.md)";

/// **P18a: the mark every name a first-frame check writes starts with.** The
/// Creator approved it in the twenty-first notes' pictures (note 6).
pub const THE_CHECKS_MARK: &str = "check_";

/// **P18a: the setting whose last piece starts the regular output's file
/// name** — the scene output's path, in every version ([`THE_NAME_START_DESK_TEST`]:
/// `sub/check_shot_` wrote `sub/check_shot_0001.png` in all six; a path ending
/// in a slash wrote `0001.png`, and `sub/check_` wrote `check_0001.png`).
pub const WHERE_THE_SCENES_NAME_STARTS: AcrossTheBoundary =
    AcrossTheBoundary::the_same_in_all_four("filepath", THE_NAME_START_DESK_TEST);

/// **P18a: the setting whose last piece starts a plain File Output node's
/// file names.** In 4.5 **each output's own path**, one per output — the
/// node's folder is always a folder there, even with no trailing slash. From
/// 5.0 **the node's base file name**, one for all its outputs, the output's
/// name following it — **except, from 5.1, an output whose name holds a
/// subfolder**: the base file name and the output's name are joined and then
/// split at the slashes, so that output's file starts with its name's last
/// piece (`nm_` and `a/beauty_` wrote `nm_a/beauty_0001.png`). 5.0 turns a
/// slash in an output's name into `_`, so no output there holds one
/// ([`THE_NAME_START_DESK_TEST`]).
pub const WHERE_A_PLAIN_NODES_NAMES_START: AcrossTheBoundary = AcrossTheBoundary {
    four_five: Some(Spelling::Cited {
        name: "path",
        source: THE_NAME_START_DESK_TEST,
    }),
    from_five_zero: Some(Spelling::Cited {
        name: "file_name",
        source: THE_NAME_START_DESK_TEST,
    }),
    from_five_three: TheThirdSide::TheSameAsFromFiveZero,
};

/// **P18a: the setting whose last piece starts a multilayer File Output
/// node's one file name.** In 4.5 **the node's folder property**, which holds
/// the start of the name too (D8; `ml/sub/mlname_` wrote `check_mlname_0001.exr`
/// once marked); from 5.0 **the node's base file name**, the folder being a
/// folder only. Input names change no file name in either
/// ([`THE_NAME_START_DESK_TEST`]).
pub const WHERE_A_GATHERING_NODES_NAME_STARTS: AcrossTheBoundary = AcrossTheBoundary {
    four_five: Some(Spelling::Cited {
        name: "base_path",
        source: THE_NAME_START_DESK_TEST,
    }),
    from_five_zero: Some(Spelling::Cited {
        name: "file_name",
        source: THE_NAME_START_DESK_TEST,
    }),
    from_five_three: TheThirdSide::TheSameAsFromFiveZero,
};

/// **P18a, performed inside Blender: every name a first-frame check will write
/// is marked [`THE_CHECKS_MARK`] at the start of its last piece.** One text in
/// two scripts — this reading's, before it reads anything, and `Rendering`'s
/// launch material (`Rendering` L10c's neighbour) — so the names the reading predicts and
/// the names the render writes cannot come apart.
///
/// `names` carries the three settings above, the tree's name and the node
/// kind, the output list and an output's own piece, and the gathering mark —
/// each from this file's table. **`every_setting`** is true in the reading;
/// in the material it is false, because the regular output path and 4.5's
/// multilayer folder property reach the render already marked, through the
/// map the reading composed (D10, D8), and marking them again would mark them
/// twice. **Folders are never marked**: only the last piece of a setting is,
/// and a setting ending in a slash takes the mark as a new last piece.
///
/// **From 5.1 a node with an output whose name holds a subfolder** has its
/// base file name moved into each output's name before the mark, so every
/// file still starts with the mark and no folder is renamed; the joined text
/// Blender splits is unchanged but for the mark. Build's own arrangement of
/// the desk test's rule, for a node mixing outputs with and without a
/// subfolder, which the test did not write.
pub const THE_CHECKS_MARK_IN_BLENDER: &str = r#"
def smoozy_marked(text, mark):
    at = text.rfind("/") + 1
    return text[:at] + mark + text[at:]


def smoozy_the_scenes_the_check_marks(scene):
    out = [scene]
    editor = scene.sequence_editor
    if editor is not None:
        for strip in editor.strips_all:
            if strip.type == "SCENE" and strip.scene is not None and strip.scene not in out:
                out.append(strip.scene)
    return out


def smoozy_the_check_is_marked(scene, names, mark, every_setting):
    seen = set()
    for one in smoozy_the_scenes_the_check_marks(scene):
        if every_setting:
            setattr(one.render, names["the_scenes_name_start"],
                    smoozy_marked(getattr(one.render, names["the_scenes_name_start"]), mark))
        # **R4 item 1**: from 5.3 every effect's group; before it the one tree.
        if names["the_scenes_effects"]:
            frontier = [getattr(effect, names["an_effects_group"]) for effect in getattr(one, names["the_scenes_effects"])]
            frontier = [top for top in frontier if top is not None]
        else:
            top = getattr(one, names["the_scenes_compositing_tree"])
            frontier = [top] if top is not None else []
        while frontier:
            tree = frontier.pop()
            if tree.as_pointer() in seen:
                continue
            seen.add(tree.as_pointer())
            for node in tree.nodes:
                inner = getattr(node, "node_tree", None)
                if inner is not None:
                    frontier.append(inner)
                if node.bl_idname != names["the_file_output_nodes_type"]:
                    continue
                gathering = names["the_gathering_mark"]
                if getattr(node.format, gathering["property"]) == gathering["value"]:
                    start = names["the_gathering_nodes_name_start"]
                    if every_setting or start != names["the_nodes_folder"]:
                        setattr(node, start, smoozy_marked(getattr(node, start), mark))
                    continue
                own = names["an_outputs_own_piece_of_path"]
                entries = list(getattr(node, names["the_nodes_output_list"]))
                start = names["the_plain_nodes_names_start"]
                if start == own:
                    for entry in entries:
                        setattr(entry, own, smoozy_marked(getattr(entry, own), mark))
                elif any("/" in getattr(entry, own) for entry in entries):
                    base = getattr(node, start)
                    for entry in entries:
                        setattr(entry, own, smoozy_marked(base + getattr(entry, own), mark))
                    setattr(node, start, "")
                else:
                    setattr(node, start, smoozy_marked(getattr(node, start), mark))
"#;

/// **The names [`THE_CHECKS_MARK_IN_BLENDER`] is handed**, for one version,
/// from this file's table — the same object in the reading and in the
/// material. **A version with no name for one of them stops the composition.**
pub fn the_names_the_mark_is_set_by(
    version: TheVersionInHand,
) -> Result<serde_json::Value, MissingSpelling> {
    Ok(serde_json::json!({
        "the_scenes_name_start": WHERE_THE_SCENES_NAME_STARTS.for_version(version)?,
        "the_plain_nodes_names_start": WHERE_A_PLAIN_NODES_NAMES_START.for_version(version)?,
        "the_gathering_nodes_name_start": WHERE_A_GATHERING_NODES_NAME_STARTS.for_version(version)?,
        "the_scenes_compositing_tree": THE_SCENES_COMPOSITING_TREE.for_version(version)?,
        "the_scenes_effects": THE_SCENES_EFFECTS.for_version(version)?,
        "an_effects_group": AN_EFFECTS_GROUP.for_version(version)?,
        "the_file_output_nodes_type": THE_FILE_OUTPUT_NODES_TYPE.for_version(version)?,
        "the_nodes_folder": THE_NODES_FOLDER.for_version(version)?,
        "the_nodes_output_list": THE_NODES_OUTPUT_LIST.for_version(version)?,
        "an_outputs_own_piece_of_path": AN_OUTPUTS_OWN_PIECE_OF_PATH.for_version(version)?,
        "the_gathering_mark": {
            "property": THE_GATHERING_MARKS_PROPERTY.for_version(version)?,
            "value": THE_GATHERING_MARKS_VALUE.for_version(version)?,
        },
    }))
}

// ---------------------------------------------------------------------------
// Composing the script
// ---------------------------------------------------------------------------

/// **R1's script, composed for the version the reading will be taken by.**
///
/// The values it needs ride in one JSON object at the head, which is the shape
/// `Rendering`'s launch material already uses: nothing about a file is written
/// into the script's code, so the same text serves every file of that version.
///
/// **A name with no source stops the composition** and names what is missing,
/// which is the shape both spellings files already use. Nothing composes a
/// script from an absence.
pub fn the_reading_script(
    invocation: &TheInvocation,
    the_jobs_scene: Option<&ArtistsText>,
) -> Result<String, MissingSpelling> {
    the_script_composed(invocation, the_jobs_scene, false)
}

/// **R4b: the pre-load reading, which also holds every other scene's outputs
/// half.** No job scene is named, so the file's own scene is read as
/// [`the_reading_script`] reads it with none, and beside it every other scene
/// in the file, each keyed by its name and composed exactly as for a named
/// scene ([`ThePrintedReading::the_other_scenes`]). **The same single opening
/// of the file; no second Blender starts.** The render-start reading is
/// [`the_reading_script`]'s, and is unchanged. Added 2026-10-10 on the
/// Creator's seventh notes, note 1.
pub fn the_pre_load_reading_script(invocation: &TheInvocation) -> Result<String, MissingSpelling> {
    the_script_composed(invocation, None, true)
}

fn the_script_composed(
    invocation: &TheInvocation,
    the_jobs_scene: Option<&ArtistsText>,
    every_scene: bool,
) -> Result<String, MissingSpelling> {
    let version = invocation.the_version_it_will_render_by;
    let mut names = serde_json::Map::new();
    let mut put = |key: &str, value: Option<&'static str>| {
        names.insert(
            key.to_string(),
            match value {
                Some(name) => serde_json::Value::from(name),
                None => serde_json::Value::Null,
            },
        );
    };
    put(
        "the_scenes_compositing_tree",
        THE_SCENES_COMPOSITING_TREE.for_version(version)?,
    );
    put("the_scenes_effects", THE_SCENES_EFFECTS.for_version(version)?);
    put("the_eevee_time_limit", THE_EEVEE_TIME_LIMIT.for_version(version)?);
    put("an_effects_group", AN_EFFECTS_GROUP.for_version(version)?);
    put(
        "an_effects_render_switch",
        AN_EFFECTS_RENDER_SWITCH.for_version(version)?,
    );
    put(
        "the_final_output_nodes_type",
        THE_FINAL_OUTPUT_NODES_TYPE.for_version(version)?,
    );
    put(
        "the_groups_own_input_type",
        THE_GROUPS_OWN_INPUT_TYPE.for_version(version)?,
    );
    put(
        "the_groups_own_output_type",
        THE_GROUPS_OWN_OUTPUT_TYPE.for_version(version)?,
    );
    put(
        "the_render_layers_nodes_type",
        THE_RENDER_LAYERS_NODES_TYPE.for_version(version)?,
    );
    put(
        "the_file_output_nodes_type",
        THE_FILE_OUTPUT_NODES_TYPE.for_version(version)?,
    );
    put(
        "the_four_five_use_nodes_switch",
        THE_FOUR_FIVE_USE_NODES_SWITCH.for_version(version)?,
    );
    put(
        "the_save_output_switch",
        if version.has_the_save_output_switch() {
            Some(THE_SAVE_OUTPUT_SWITCH.name()?)
        } else {
            None
        },
    );
    put("the_nodes_folder", THE_NODES_FOLDER.for_version(version)?);
    put(
        "the_nodes_base_file_name",
        THE_NODES_BASE_FILE_NAME.for_version(version)?,
    );
    put(
        "the_nodes_output_list",
        THE_NODES_OUTPUT_LIST.for_version(version)?,
    );
    put(
        "an_outputs_own_piece_of_path",
        AN_OUTPUTS_OWN_PIECE_OF_PATH.for_version(version)?,
    );
    put(
        "the_per_output_format_switch",
        THE_PER_OUTPUT_FORMAT_SWITCH.for_version(version)?,
    );
    put("the_media_kind", THE_MEDIA_KIND.for_version(version)?);
    put(
        "a_nodes_own_ending_switch",
        if version.has_the_save_output_switch() {
            Some(A_NODES_OWN_ENDING_SWITCH.name()?)
        } else {
            None
        },
    );
    put(
        "a_picture_sockets_kind",
        A_PICTURE_SOCKETS_KIND.for_version(version)?,
    );
    put(
        "the_kind_no_output_can_be",
        THE_KIND_NO_OUTPUT_CAN_BE.for_version(version)?,
    );
    put(
        "the_cycles_identifier",
        THE_CYCLES_IDENTIFIER.for_version(version)?,
    );
    names.insert(
        "the_switch_is_on_when_the_output_has_its_own_format".to_string(),
        serde_json::Value::Bool(the_switch_is_on_when_the_output_has_its_own_format(version)),
    );
    names.insert(
        "the_curves_hang_off_the_action".to_string(),
        serde_json::Value::Bool(the_curves_hang_off_the_action(version)),
    );
    names.insert(
        "the_gathering_mark".to_string(),
        serde_json::json!({
            "property": THE_GATHERING_MARKS_PROPERTY.for_version(version)?,
            "value": THE_GATHERING_MARKS_VALUE.for_version(version)?,
        }),
    );

    let document = serde_json::json!({
        "composed_for": [version.major, version.minor],
        "the_frames_the_job_covers": invocation.the_frames_the_job_covers,
        // **The scene, where the caller has one.** At render start the worker
        // names the job's scene; at pre-load there is no job yet and the file's
        // own scene is read, which is the scene a render with no scene argument
        // would use. The name crosses as the file's own text, which is what
        // `Rendering` L15 requires of every name a reading produced.
        "the_jobs_scene": the_jobs_scene.map(|name| {
            String::from_utf8_lossy(name.as_bytes()).into_owned()
        }),
        // **The two marks, from this crate's own constants**, so the script
        // and the reader cannot come to spell them differently.
        "the_marks": [
            String::from_utf8_lossy(THE_OPENING_SENTINEL),
            String::from_utf8_lossy(THE_CLOSING_SENTINEL),
        ],
        "names": names,
        "the_walk_steps_over": the_walk_steps_over(version)?,
        "the_asking_scenes_name": THE_ASKING_SCENES_NAME,
        // **R4b**: only the pre-load reading asks for every scene's half.
        "every_scene": every_scene && the_jobs_scene.is_none(),
        // **R4c: a first-frame check of a job whose only output is a video is
        // read as a picture.** Only the render-start reading of a check is
        // told; the pre-load reading never is (added 2026-10-27).
        "the_check_is_written_as_a_picture": invocation.the_check_is_written_as_a_picture && !every_scene,
        "the_picture_the_check_is_written_as": {
            "media": A_STILL_PICTURES_MEDIA_KIND.for_version(version)?,
            "format": THE_CHECKS_PICTURE_FORMAT.for_version(version)?,
        },
        // **P18a: a first-frame check that writes into its range job's
        // folder has every name marked before anything is read.** Only the
        // render-start reading of such a check is told; the pre-load reading
        // never is (added 2026-10-31).
        "the_check_is_marked": invocation.the_check_is_marked && !every_scene,
        "the_checks_mark": THE_CHECKS_MARK,
        "the_names_the_mark_is_set_by": the_names_the_mark_is_set_by(version)?,
    });
    let rendered = serde_json::to_string(&document)
        .expect("the document is composed of values that always render");
    // The one thing that could end the raw string is a run of three single
    // quotes, which cannot appear in JSON output — its only string delimiter is
    // the double quote. The same guard the launch material keeps.
    debug_assert!(!rendered.contains("'''"));
    // **`Render Settings` Q8: the started mark is the script's very first
    // act**, before the handed-in document is even parsed, so anything that
    // goes wrong after it — Blender crashing while the walk reads the file, a
    // Python error, the composed-for guard — reads as *reading the file did not
    // finish* and never as *this Blender could not open this file*. Written to
    // the ordinary stream and pushed out at once, so a crash a moment later
    // cannot leave it in a buffer. Added 2026-10-02, the Local View crash.
    let started = std::str::from_utf8(THE_STARTED_MARK).expect("the mark is text");
    debug_assert!(!started.contains(['"', '\\']));
    Ok(format!(
        "import sys\nsys.stdout.write(\"{started}\")\nsys.stdout.flush()\n\
         import json\nSMOOZY = json.loads(r'''{rendered}''')\n{THE_READ_AS_THE_FILE_HAS_IT}{THE_CHECKS_MARK_IN_BLENDER}{THE_SIMULATION_SHUTTER}{THE_BODY}"
    ))
}

/// **R18: a read reports the file's own values, never the throwaway ones.**
/// Added 2026-10-02, simulation fix piece 5.
///
/// Every Blender Smoozy starts on the user's file to read it carries the
/// load-time handler `Rendering` L8b registers, pointing every Replay fluid
/// cache and every disk cache at a throwaway folder. **That changes the very
/// settings a read reports** — a cache's folder, whether *external* is on,
/// whether it is baked. So the handler keeps each value as the file had it, in
/// its record under [`crate::redirect::THE_LOAD_TIME_RECORD`], and **every read
/// of those settings goes through these two**: one gives a setting as the file
/// had it, the other turns a throwaway folder in a list of paths back into the
/// value it replaced. Where no handler ran, both give what is there.
///
/// Shared by this reading's script and `Dependency Upload & Remap`'s check, so
/// the two cannot read the handler's record differently.
pub const THE_READ_AS_THE_FILE_HAS_IT: &str = r#"
import os, sys


def smoozy_as_the_file_has_it(holder, name):
    record = sys.modules.get("smoozy_load_time")
    kept = getattr(record, "as_the_file_has_it", None) or {}
    key = (holder.as_pointer(), name)
    if key in kept:
        return kept[key]
    return getattr(holder, name)


def smoozy_a_path_as_the_file_has_it(value):
    record = sys.modules.get("smoozy_load_time")
    back = getattr(record, "paths_put_back", None) or {}
    if not value:
        return value
    return back.get(os.path.normpath(value), value)
"#;

// ---------------------------------------------------------------------------
// What the script prints, and what is composed from it
// ---------------------------------------------------------------------------

/// **Items 1 to 6 of R4, which are what a file can actually be asked**, plus the
/// compositing tree the walk of §8 is made against.
///
/// **This is [`TheOutputsHalf`] without `the_beauty_walk`**, which is item 7 and
/// is derived rather than read. [`the_one_document_from`] derives it.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TheObservedOutputs {
    pub taken_by: TheVersionInHand,
    pub gating: GatingConditions,
    pub destinations: Vec<Destination>,
    pub scenes_reached_through_a_sequencer_strip: Vec<SceneReachedThroughAStrip>,
    pub the_tree_is_shared_with: Option<ArtistsText>,
    pub per_frame: Vec<AtOneFrame>,
    pub recorded_locations: Vec<RecordedLocation>,
    pub views: HowManyViewsTheSceneWrites,
    /// **The job's scene's compositing tree, as the file holds it.** Absent
    /// where the scene has none, which is B5's case: the raw render lands at
    /// the regular output path and no walk is needed.
    pub the_compositing_tree: Option<TheCompositingTree>,
    /// **B25: the job's scene's own view transform**, carried whole into
    /// [`TheOutputsHalf`]. Absent from a script older than 2026-10-03, which
    /// reads as not read.
    #[serde(default)]
    pub the_view_transform: Option<ArtistsText>,
}

/// **B25's name: the scene's view transform.** `view_settings.view_transform`
/// on a scene, set and read back in all four versions by the Blender
/// research's own hand tests for *Images* question 6 — the same three
/// treatments written with `view_transform = "AgX"`, `"Standard"` and a look,
/// and the factory default printed as `view_transform='AgX'` — in
/// `Preparation/Blender Research/Findings/4.5/Images.md`, `5.0/Images.md`,
/// `5.1/Images.md` and `5.2/Images.md`, section 6, on the Creator's Mac and on
/// the rented Linux machine. Label: **observed in a test**. Cited in place,
/// like [`THE_STEPPING_LOOKUP`], because the name is written in the script's
/// body and is the same in all four. Added 2026-10-03, the 38's piece 3.
pub const THE_VIEW_TRANSFORMS_NAME: &str = "the Blender research's Images question 6, its hand \
     tests in all four versions (Preparation/Blender Research/Findings/<version>/Images.md, \
     section 6)";

/// **What the script prints between the two marks.**
///
/// **`Render Settings` Q7: a reading is the complete sentinel-framed document
/// with an empty error map, and anything else is not a reading.** The error map
/// is a field here because that clause makes it one; [`the_one_document_from`]
/// is where it is read.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ThePrintedReading {
    pub outputs: TheObservedOutputs,
    /// The settings half, carried and not interpreted here — it is `Render
    /// Settings`' shape and a second one here would be a second reading.
    pub settings: CarriedAndNotInterpretedHere,
    /// **The third payload, and it stays empty.** R1 names three payloads out
    /// of one read and the plans give this one no content: the recorded
    /// locations and the destinations are both in the outputs half already
    /// (R4 items 2 and 6), and the dependency check is a separate act with its
    /// own reading and its own marks (`Dependency Upload & Remap` G3's
    /// neighbour). **It stays rather than going** because
    /// [`crate::reading::TheOneDocument`] carries the field and this crate's
    /// shapes are not this piece's to change; what this piece settles is that
    /// nothing fills it and nothing reads it.
    pub dependencies: CarriedAndNotInterpretedHere,
    /// **The error map.** One thing fills it: a frame whose regular output path
    /// Blender would not answer, which is I1's broken template — *"the asking
    /// doubles as a check"*. Everything else raises uncaught, so the run ends
    /// at the failure status with a short document, which is `Render Settings`
    /// Q8's third ending and `Dependency Upload & Remap` G3's rule.
    pub errors: BTreeMap<String, String>,
    /// **D21b-i, added 2026-10-02: every physics disk cache the file marks
    /// baked whose files are on this machine** — carried into
    /// [`TheOneDocument`] whole. Absent from a script older than that date,
    /// which reads as none.
    #[serde(default)]
    pub the_baked_caches_on_the_machine: Vec<crate::reading::ABakedCacheOnTheMachine>,
    /// **R4 item 8, added 2026-10-02**: whether the job's scene holds a
    /// simulation that is not baked — carried into [`TheOneDocument`] whole.
    /// Absent from a script older than that date, which reads as no.
    #[serde(default)]
    pub the_scene_holds_a_simulation_not_baked: bool,
    /// **The scene this reading read**, by its own name — the job's named
    /// scene, or where the job names none the scene a run with no window
    /// takes as `bpy.context.scene` (the 38's piece 10, item 25; `Design/Render
    /// Settings § What it will write`). **It names the scene the outputs half
    /// was read from and claims nothing else**: which scene a file saved from
    /// Blender's interface records as its showing scene is `Rendering` L16's
    /// untested half, and this does not settle it. Cited at
    /// [`THE_SCENE_NAME_LOOKUP`]. Absent from a script older than 2026-10-03,
    /// which reads as not named.
    #[serde(default)]
    pub the_scene_it_read: Option<ArtistsText>,
    /// **The open file's own path, as the Blender reading it holds it**
    /// (`bpy.data.filepath`, as bytes under R4a) — where `{blend_name}`'s
    /// value comes from (`Job files and outputs` I2's *How Smoozy gets the
    /// replaced text*, added 2026-10-18). Absent from a script older than
    /// that date, and **the reading is then taken as its raw text**: nothing
    /// is replaced ([`ThePrintedReading::with_the_braces_replaced`]).
    #[serde(default)]
    pub the_open_files_path: Option<RecordedLocation>,
    /// **R4c: whether this reading switched the job's scene from a video to a
    /// still picture**, which only the render-start reading of a first-frame
    /// check does, and only where the scene's only output is a video.
    /// `Rendering` L10c makes the same switch in the render's own material
    /// where this says yes. Absent from a script older than 2026-10-27, which
    /// reads as no.
    #[serde(default)]
    pub the_check_is_written_as_a_picture: bool,
}

/// **R4b: one other scene's outputs half, as the pre-load reading prints it**
/// beside the scene it read. Added 2026-10-10 on the Creator's seventh notes,
/// note 1.
///
/// **Its recorded locations are the file's** (R4 item 6) and ride once, in the
/// read scene's half; this one prints none, and
/// [`ThePrintedReading::as_a_reading_naming`] puts them back. **Its errors are
/// its own**: a scene the reading does not name never makes the reading fail,
/// and a half with errors is no reading of that scene, as a reading naming it
/// would be none (`Render Settings` Q7).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AnotherScenesHalf {
    pub scene: ArtistsText,
    /// Absent where reading the scene raised, which its errors then say.
    pub outputs: Option<TheObservedOutputs>,
    pub the_scene_holds_a_simulation_not_baked: bool,
    pub errors: BTreeMap<String, String>,
}

/// **R4b's halves, taken out of the same printed document.** `None` is a
/// reading that holds one scene: a reading that named its scene, or one from a
/// machine program older than R4b.
#[derive(Debug, Clone, PartialEq, Default, Deserialize)]
pub struct TheOtherScenes {
    #[serde(default)]
    pub the_other_scenes: Option<Vec<AnotherScenesHalf>>,
}

impl TheOtherScenes {
    /// From the same stream, between the same two marks as
    /// [`the_printed_reading_in`]; nothing where they are not both there.
    pub fn in_the_stream(stream: &[u8]) -> TheOtherScenes {
        let Some(opens) = find(stream, THE_OPENING_SENTINEL) else {
            return TheOtherScenes::default();
        };
        let from = opens + THE_OPENING_SENTINEL.len();
        let Some(closes) = find(&stream[from..], THE_CLOSING_SENTINEL) else {
            return TheOtherScenes::default();
        };
        serde_json::from_slice(&stream[from..from + closes]).unwrap_or_default()
    }
}

impl ThePrintedReading {
    /// **R4b: what a reading naming that scene alone prints**, composed from
    /// this reading and that scene's half — its outputs with the file's
    /// recorded locations put back, its simulation answer, its own errors, and
    /// the scene named as the one read; everything about the file as a whole
    /// is this reading's. **The desk proved the two the same** in all four
    /// Blenders (§14 item 30; the account of the seventh notes' note 1 in
    /// `Audits/Build/Step 10c`, and the desk test
    /// `every_scene_read_in_one_opening_is_the_same_as_a_reading_naming_it_in_all_four`).
    /// `None` where reading that scene raised.
    pub fn as_a_reading_naming(&self, half: &AnotherScenesHalf) -> Option<ThePrintedReading> {
        let mut outputs = half.outputs.clone()?;
        outputs.recorded_locations = self.outputs.recorded_locations.clone();
        Some(
            ThePrintedReading {
                outputs,
                settings: self.settings.clone(),
                dependencies: self.dependencies.clone(),
                errors: half.errors.clone(),
                the_baked_caches_on_the_machine: self.the_baked_caches_on_the_machine.clone(),
                the_scene_holds_a_simulation_not_baked: half.the_scene_holds_a_simulation_not_baked,
                the_scene_it_read: Some(half.scene.clone()),
                the_open_files_path: self.the_open_files_path.clone(),
                // **R4c** is the render-start reading's alone, and R4b's
                // halves are the pre-load reading's.
                the_check_is_written_as_a_picture: false,
            }
            .with_the_braces_replaced(),
        )
    }

    /// **`Job files and outputs` I2's *A curly-brace expression in a node's
    /// names*, applied as the reading is taken in** (added 2026-10-18, the
    /// Creator's thirteenth notes, notes 5, 9 and 10): beside each node
    /// destination's raw folder, base file name and output's own name, the
    /// text with `{blend_name}` and a doubled brace put in, or *unresolved*
    /// with the expression named ([`crate::reading::TheNodesTextReplaced`]).
    ///
    /// **`{blend_name}`'s value is the open file's own name**, which the
    /// reading read inside the Blender that opened it; nothing here calls
    /// anything that claims to work out templates. **A reading that carries
    /// no path is left as it is**, and is taken as its raw text.
    pub fn with_the_braces_replaced(mut self) -> ThePrintedReading {
        let Some(path) = self.the_open_files_path.as_ref() else {
            return self;
        };
        let the_blend_name = crate::reading::the_blend_name_of(path.as_bytes());
        let version = self.outputs.taken_by;
        for destination in &mut self.outputs.destinations {
            destination.put_in_the_replaced_text(the_blend_name.as_deref(), version);
        }
        self
    }
}

/// **The lookup of 2026-10-03 for the scene a reading names** — 4.5.12,
/// 5.0.1, 5.1.2 and 5.2.2 at the desk, started with no window and from factory
/// settings, printing the property descriptions themselves: `Context.scene`, a
/// pointer of kind `Scene`, present in all four; and `Scene.name`, *"Unique
/// data-block ID name (within a same type and library)"*, present in all four.
/// Cited in place, like [`THE_STEPPING_LOOKUP`], because the two names are
/// written in the script's body and are the same in all four.
pub const THE_SCENE_NAME_LOOKUP: &str = "the scene-name lookup, 2026-10-03 \
     (Audits/Build/Step 10c/The 38, piece 10 — what landed.md)";

/// Why a printed reading is not a reading.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhyItIsNoReading {
    /// `Render Settings` Q7's second half.
    TheErrorMapIsNotEmpty(BTreeMap<String, String>),
}

/// **What the script printed, taken out from between the same two marks the
/// recorded reading uses.**
///
/// **Nothing is guessed from a stream that does not carry both marks**, which
/// is [`crate::reading::the_document_in`]'s own discipline and `Dependency
/// Upload & Remap` G3's from the other end.
pub fn the_printed_reading_in(
    stream: &[u8],
) -> Result<ThePrintedReading, WhyTheDocumentWasNotRead> {
    let opens = find(stream, THE_OPENING_SENTINEL)
        .ok_or(WhyTheDocumentWasNotRead::TheSentinelsWereNotBothThere)?;
    let from = opens + THE_OPENING_SENTINEL.len();
    let closes = find(&stream[from..], THE_CLOSING_SENTINEL)
        .ok_or(WhyTheDocumentWasNotRead::TheSentinelsWereNotBothThere)?;
    serde_json::from_slice::<ThePrintedReading>(&stream[from..from + closes])
        .map(ThePrintedReading::with_the_braces_replaced)
        .map_err(|_| WhyTheDocumentWasNotRead::WhatSatBetweenTheMarksIsNotADocument)
}

fn find(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack
        .windows(needle.len())
        .position(|window| window == needle)
}

/// **The recorded reading, composed from what the script printed.**
///
/// Two things happen here and nothing else does.
///
/// **`Render Settings` Q7 is applied**: a document whose error map is not empty
/// is no reading, and nothing of it is carried forward.
///
/// **R4 item 7's beauty walk is derived** by
/// [`crate::main_output::walk_the_beauty`], over the tree the script printed —
/// the one walk this project has, run at the place the reading is composed,
/// rather than a second copy written in Python.
pub fn the_one_document_from(
    printed: ThePrintedReading,
    enters_at: &WhereTheRenderEnters,
) -> Result<TheOneDocument, WhyItIsNoReading> {
    if !printed.errors.is_empty() {
        return Err(WhyItIsNoReading::TheErrorMapIsNotEmpty(printed.errors));
    }
    let observed = printed.outputs;
    let the_beauty_walk = match &observed.the_compositing_tree {
        // **B1a: from 5.3 the tree is the stack, walked as one chain**, by the
        // effects on at the job's first frame.
        Some(tree) if crate::main_output::it_is_a_stack_of_effects(tree) => {
            let at_the_first_frame = observed.per_frame.first().map_or(&observed.gating, |frame| &frame.gating);
            crate::main_output::walk_the_beauty_through_the_effects(
                tree,
                at_the_first_frame.the_effects_on_for_render.as_deref().unwrap_or(&[]),
                enters_at,
                observed.taken_by,
            )
        }
        Some(tree) => {
            crate::main_output::walk_the_beauty(tree, enters_at, observed.taken_by)
        }
        // **B4's *no entry*, and it is a declared outcome rather than an empty
        // answer.** A scene with no compositing tree is B5's case: the raw
        // render lands at the regular output path and no walk is needed.
        None => WhatTheWalkFound::NoEntry,
    };
    Ok(TheOneDocument {
        outputs: TheOutputsHalf {
            taken_by: observed.taken_by,
            gating: observed.gating,
            destinations: observed.destinations,
            scenes_reached_through_a_sequencer_strip: observed
                .scenes_reached_through_a_sequencer_strip,
            the_tree_is_shared_with: observed.the_tree_is_shared_with,
            per_frame: observed.per_frame,
            recorded_locations: observed.recorded_locations,
            views: observed.views,
            the_beauty_walk,
            the_view_transform: observed.the_view_transform,
        },
        settings: printed.settings,
        dependencies: printed.dependencies,
        the_baked_caches_on_the_machine: printed.the_baked_caches_on_the_machine,
        the_scene_holds_a_simulation_not_baked: printed.the_scene_holds_a_simulation_not_baked,
        the_scene_it_read: printed.the_scene_it_read,
    })
}

/// **R4 item 2: a file output node's outputs are destinations only where the
/// node will run**, which is items 1 and 2 together.
///
/// **The script records every node's outputs and the gating beside them**; the
/// filtering is here, on [`GatingConditions::a_file_output_node_will_write`],
/// which is the one place that rule is kept. **A second copy of it in the
/// script would be the one-fact-in-two-places fault this stage keeps paying
/// for**, and this is the function piece 14's composing step calls before it
/// writes the map.
pub fn the_destinations_that_will_be_written(
    observed: &TheObservedOutputs,
) -> Vec<Destination> {
    use crate::reading::{DestinationKind, WhichScene};
    observed
        .destinations
        .iter()
        .filter(|destination| {
            let from_a_node = matches!(
                destination.kind,
                DestinationKind::AnOutputOfAFileOutputNode { .. }
                    | DestinationKind::AWholeFileOutputNodeGatheringIntoOneFile { .. }
            );
            if !from_a_node {
                return true;
            }
            // **R4 item 3: a scene reached through a strip is read against its
            // own gating conditions rather than the job's.**
            let gating = match &destination.scene {
                WhichScene::TheJobsOwnScene => &observed.gating,
                WhichScene::ReachedThroughASequencerStrip { scene, .. } => observed
                    .scenes_reached_through_a_sequencer_strip
                    .iter()
                    .find(|reached| reached.scene.as_bytes() == scene.as_bytes())
                    .map(|reached| &reached.gating)
                    .unwrap_or(&observed.gating),
            };
            gating.a_file_output_node_will_write()
        })
        .cloned()
        .collect()
}

/// `Rendering` L10b and `Render Settings` Q2b share the saved shutter reading.
/// Documented: Blender RenderSettings, 5.2; observed in the 2026-11-06 desk check.
pub const THE_SIMULATION_SHUTTER: &str = r#"
def smoozy_motion_blur(scene):
    render = scene.render
    return {"enabled": bool(render.use_motion_blur),
            "position": render.motion_blur_position,
            "shutter": float(render.motion_blur_shutter)}

def smoozy_simulation_tail(scene):
    import math
    blur = smoozy_motion_blur(scene)
    if not blur["enabled"] or blur["position"] == "END":
        return 0
    reach = blur["shutter"] * (0.5 if blur["position"] == "CENTER" else 1.0)
    return math.ceil(reach)

"#;

const THE_BODY: &str = r#"
import bpy, json, os, sys

# The two marks are the crate's own and travel in the handed-in document, so
# there is one spelling of each rather than two that could drift apart.
OPENS = SMOOZY["the_marks"][0]
CLOSES = SMOOZY["the_marks"][1]

NAMES = SMOOZY["names"]
FRAMES = sorted(set(int(one) for one in SMOOZY["the_frames_the_job_covers"]))

if list(bpy.app.version[:2]) != SMOOZY["composed_for"]:
    raise Exception(
        "smoozy: this reading was composed for "
        + str(SMOOZY["composed_for"])
        + " and is running in "
        + str(list(bpy.app.version[:2]))
    )

ERRORS = {}


def as_bytes(value):
    if isinstance(value, bytes):
        raw = value
    else:
        raw = os.fsencode(value)
    return list(raw)


# **`Job files and outputs` R4 item 1**: a scene's compositor as (the effect's
# name, its group, whether it is on for render). From 5.3 every effect holding a
# group, in list order, and the older one-group name is never read. Before 5.3
# one entry with no effect named, so everything below reads as it always did.
def the_effects_of(scene):
    if not NAMES["the_scenes_effects"]:
        top = getattr(scene, NAMES["the_scenes_compositing_tree"])
        return [(None, top, True)] if top is not None else []
    out = []
    for effect in getattr(scene, NAMES["the_scenes_effects"]):
        group = getattr(effect, NAMES["an_effects_group"])
        if group is not None:
            out.append((effect.name, group, bool(getattr(effect, NAMES["an_effects_render_switch"]))))
    return out


def is_kind(node, wanted):
    return node.bl_idname == wanted


def renders_through_a_scene_strip(scene):
    if not scene.render.use_sequencer:
        return False
    editor = scene.sequence_editor
    if editor is None:
        return False
    return any(strip.type == "SCENE" for strip in editor.strips_all)


def gating_of(scene):
    on = [one for one in the_effects_of(scene) if one[2]]
    live = False
    muted = False
    for _effect, tree, _on in on:
        for node in tree.nodes:
            if is_kind(node, NAMES["the_final_output_nodes_type"]):
                live = True
                muted = bool(node.mute)
                break
        if live:
            break
    four_five = None
    if NAMES["the_four_five_use_nodes_switch"]:
        four_five = bool(getattr(scene, NAMES["the_four_five_use_nodes_switch"]))
    save_output = None
    if NAMES["the_save_output_switch"]:
        save_output = bool(getattr(scene.render, NAMES["the_save_output_switch"]))
    gating = {
        "compositing_switch": bool(scene.render.use_compositing),
        "a_compositing_tree_is_present": len(on) > 0,
        "a_live_final_output_node": live,
        "the_final_output_node_is_muted": muted,
        "four_fives_use_nodes_switch": four_five,
        "save_output_switch": save_output,
        "the_render_goes_out_through_a_sequencer_holding_a_scene_strip":
            renders_through_a_scene_strip(scene),
    }
    # **R4 item 1**: which effects are on, as this frame has them, so a keyed
    # or driven switch comes with it.
    if NAMES["the_scenes_effects"]:
        gating["the_effects_on_for_render"] = [as_bytes(one[0]) for one in on]
    return gating


def every_tree_of(scene):
    """Every tree of the scene's compositor, each at the first chain that
    reached it, whichever effect holds it."""
    seen = set()
    for _effect, top, _on in the_effects_of(scene):
        for one in the_trees_under(top, seen):
            yield one


def the_trees_under(top, seen):
    """A compositing tree and every group tree inside it, each with the chain of
    (tree, group node) it was reached through, so a walk can step back out of a
    group. A tree used in two places is walked once, at the first chain that
    reached it."""
    frontier = [(top, None)]
    while frontier:
        tree, stack = frontier.pop()
        if tree.as_pointer() in seen:
            continue
        seen.add(tree.as_pointer())
        yield tree, stack
        for node in tree.nodes:
            inner = getattr(node, "node_tree", None)
            if inner is not None:
                frontier.append((inner, (tree, node, stack)))


def gathers_into_one_file(node):
    mark = NAMES["the_gathering_mark"]
    return getattr(node.format, mark["property"]) == mark["value"]


def the_outputs_of(node):
    """Each of a file output node's outputs: its input socket, the entry that
    carries its own piece of path, and that piece.

    **From 5.0 the node carries a trailing socket that is not an output** — the
    one a new output is made by linking to, of the kind no output entry can be
    — so the outputs are the sockets of every other kind, paired in order with
    the entries. 4.5 carries no such socket and pairs one for one."""
    entries = list(getattr(node, NAMES["the_nodes_output_list"]))
    out = []
    at = 0
    for socket in node.inputs:
        if socket.type == NAMES["the_kind_no_output_can_be"]:
            continue
        entry = entries[at] if at < len(entries) else None
        if entry is None:
            own = socket.name
        else:
            own = getattr(entry, NAMES["an_outputs_own_piece_of_path"])
        out.append((at, socket, entry, own))
        at += 1
    return out


ENDINGS = {}


def a_formats_key(settings):
    kind = ""
    if NAMES["the_media_kind"]:
        kind = getattr(settings, NAMES["the_media_kind"])
    return (kind, settings.file_format)


def the_endings_of(formats):
    """The ending each format writes, asked of Blender on a scene made for the
    asking and removed again. The job's own scenes are never written to."""
    wanted = {}
    for settings in formats:
        wanted.setdefault(a_formats_key(settings), settings)
    if not wanted:
        return {}
    out = {}
    throwaway = bpy.data.scenes.new(SMOOZY["the_asking_scenes_name"])
    try:
        for key, settings in wanted.items():
            if NAMES["the_media_kind"]:
                setattr(
                    throwaway.render.image_settings,
                    NAMES["the_media_kind"],
                    getattr(settings, NAMES["the_media_kind"]),
                )
            throwaway.render.image_settings.file_format = settings.file_format
            out[key] = throwaway.render.file_extension
    finally:
        bpy.data.scenes.remove(throwaway)
    return out


def an_endings_bytes(scene, node, settings):
    switch = scene.render.use_file_extension
    if NAMES["a_nodes_own_ending_switch"]:
        switch = getattr(node, NAMES["a_nodes_own_ending_switch"])
    if not switch:
        return None
    return as_bytes(ENDINGS[a_formats_key(settings)])


def the_view_layers_feeding(tree, socket, stack, the_top_entrys_layer):
    """Which view layers feed one input socket, by following the links
    backwards to the Render Layers nodes the picture entered on. A group
    boundary is stepped through in both directions, by socket place.

    **`Job files and outputs` I8, from 5.3: a top-level Group Input is the
    render's entry, for the first view layer** (`Frames 9`), handed in here. A
    Group Input inside a nested group carries what the outer tree hands it, and
    is stepped through as ever. **Caution, the runner's ruling:** which layer it
    carries where the first is off at a frame is not established, a render with
    only the second layer on having crashed in 5.3 and in 5.2.2. So this stands
    as written. A later effect's Group Input is named the same, not traced back
    through the chain."""
    found = []
    seen = set()
    frontier = [(tree, socket, stack)]
    while frontier:
        inside, one, chain = frontier.pop()
        key = (one.as_pointer(), id(chain))
        if key in seen:
            continue
        seen.add(key)
        for link in inside.links:
            if link.to_socket.as_pointer() != one.as_pointer():
                continue
            giver = link.from_node
            if is_kind(giver, NAMES["the_render_layers_nodes_type"]):
                if giver.layer and giver.layer not in found:
                    found.append(giver.layer)
                continue
            if is_kind(giver, NAMES["the_groups_own_input_type"]):
                if chain is None:
                    if the_top_entrys_layer is not None and the_top_entrys_layer not in found:
                        found.append(the_top_entrys_layer)
                    continue
                outer, group, beyond = chain
                index = socket_index(giver.outputs, link.from_socket)
                if index < len(group.inputs):
                    frontier.append((outer, group.inputs[index], beyond))
                continue
            inner = getattr(giver, "node_tree", None)
            if inner is not None:
                index = socket_index(giver.outputs, link.from_socket)
                for node in inner.nodes:
                    if is_kind(node, NAMES["the_groups_own_output_type"]):
                        if index < len(node.inputs):
                            frontier.append(
                                (inner, node.inputs[index], (inside, giver, chain))
                            )
                continue
            for deeper in giver.inputs:
                frontier.append((inside, deeper, chain))
    return found


def destinations_of(scene, which_scene):
    out = []
    # **`Job files and outputs` R4 item 3 as corrected 2026-11-02**: a scene
    # reached through a strip writes nothing through its own regular output
    # path (observed at the desk in six versions), so only the job's own scene
    # gives one.
    if which_scene == "TheJobsOwnScene":
        out.append(
            {
                "kind": "TheRegularOutputPath",
                "recorded_location": as_bytes(scene.render.filepath),
                "format": {
                    "as_the_file_records_it": as_bytes(
                        scene.render.image_settings.file_format
                    ),
                    "the_ending_it_writes": as_bytes(scene.render.file_extension)
                    if scene.render.use_file_extension
                    else None,
                    "is_video_shaped": bool(scene.render.is_movie_format),
                },
                "origin": "TheScenesOutputPath",
                "scene": which_scene,
                "the_view_layers_that_feed_it": [
                    as_bytes(layer.name) for layer in scene.view_layers if layer.use
                ],
            }
        )
    # **`Job files and outputs` R4 item 2, from 5.3**: every effect holding a
    # group, the active one no differently, each node named by its effect. A
    # node whose effect is off at every frame stepped contributes none. A group
    # two effects hold is walked once per effect, each owing by its own switch.
    the_top_entrys_layer = None
    if NAMES["the_scenes_effects"] and len(scene.view_layers) > 0:
        the_top_entrys_layer = scene.view_layers[0].name
    on_somewhere = EFFECTS_ON_AT_SOME_FRAME.get(scene.name)
    for the_effect, top, _on in the_effects_of(scene):
        if the_effect is not None and on_somewhere is not None and the_effect not in on_somewhere:
            continue
        named_by_its_effect = {} if the_effect is None else {"the_effect": as_bytes(the_effect)}
        for tree, stack in the_trees_under(top, set()):
            for node in tree.nodes:
                if not is_kind(node, NAMES["the_file_output_nodes_type"]):
                    continue
                if node.mute:
                    continue
                folder = as_bytes(getattr(node, NAMES["the_nodes_folder"]))
                base = None
                if NAMES["the_nodes_base_file_name"]:
                    base = as_bytes(getattr(node, NAMES["the_nodes_base_file_name"]))
                if gathers_into_one_file(node):
                    layers = []
                    for socket in node.inputs:
                        if not socket.is_linked:
                            continue
                        for one in the_view_layers_feeding(tree, socket, stack, the_top_entrys_layer):
                            if one not in layers:
                                layers.append(one)
                    out.append(
                        {
                            "kind": {
                                "AWholeFileOutputNodeGatheringIntoOneFile": {
                                    "node": as_bytes(node.name),
                                    "the_nodes_base_file_name": base,
                                    **named_by_its_effect,
                                }
                            },
                            "recorded_location": folder,
                            "format": {
                                "as_the_file_records_it": as_bytes(node.format.file_format),
                                "the_ending_it_writes": an_endings_bytes(scene, node, node.format),
                                "is_video_shaped": False,
                            },
                            "origin": {"AFileOutputNodeNamed": as_bytes(node.name)},
                            "scene": which_scene,
                            "the_view_layers_that_feed_it": [as_bytes(one) for one in layers],
                        }
                    )
                    continue
                for at, socket, entry, own in the_outputs_of(node):
                    if not socket.is_linked:
                        continue
                    settings = node.format
                    if entry is not None:
                        switch = bool(getattr(entry, NAMES["the_per_output_format_switch"]))
                        if switch == NAMES["the_switch_is_on_when_the_output_has_its_own_format"]:
                            settings = entry.format
                    out.append(
                        {
                            "kind": {
                                "AnOutputOfAFileOutputNode": {
                                    "node": as_bytes(node.name),
                                    "output": as_bytes(own),
                                    "the_nodes_base_file_name": base,
                                    **named_by_its_effect,
                                }
                            },
                            "recorded_location": folder,
                            "format": {
                                "as_the_file_records_it": as_bytes(settings.file_format),
                                "the_ending_it_writes": an_endings_bytes(scene, node, settings),
                                "is_video_shaped": False,
                            },
                            "origin": {"AFileOutputNodeNamed": as_bytes(node.name)},
                            "scene": which_scene,
                            "the_view_layers_that_feed_it": [
                                as_bytes(one) for one in the_view_layers_feeding(tree, socket, stack, the_top_entrys_layer)
                            ],
                        }
                    )
    return out


THE_ADDED_WRITE_PLACES = (
    "TheBakeOutputPath",
    "ThePointCache",
    "TheGeometryNodesBakeFolder",
    "TheSequencerProxyFolder",
    "TheReplayFluidCacheFolder",
)


def the_scene_holds_a_simulation_not_baked(scene):
    # **`Job files and outputs` R4 item 8, added 2026-10-02, for
    # `Rendering` L10b — the Creator's decision A**: yes where the job's scene holds a
    # geometry-nodes simulation zone, a point cache that is not baked on any
    # carrier the walk reaches — the rigid-body world's included — or a fluid
    # domain whose cache type is Replay. Every name below was read off the four
    # Blenders (`crate::reading::THE_SIMULATION_LOOKUP`).
    #
    # **R18**: whether a cache is baked is read as the file had it, never as
    # the read's handler left it.
    world = scene.rigidbody_world
    if world is not None:
        cache = world.point_cache
        if not smoozy_as_the_file_has_it(cache, "is_baked"):
            return True
    for obj in scene.objects:
        for modifier in getattr(obj, "modifiers", ()):
            if modifier.type == "NODES":
                # **A simulation zone counts whether or not it is baked**: no
                # name in any of the four says whether a geometry-nodes bake
                # holds data, and stepping a baked zone renders the same frame.
                # A bake node (`GeometryNodeBake`) is not a simulation.
                for bake in modifier.bakes:
                    node = bake.node
                    if node is not None and node.bl_idname == "GeometryNodeSimulationOutput":
                        return True
                continue
            if modifier.type == "FLUID":
                if modifier.fluid_type == "DOMAIN" and modifier.domain_settings.cache_type == "REPLAY":
                    return True
                continue
            caches = []
            if hasattr(modifier, "point_cache"):
                caches.append(modifier.point_cache)
            canvas = getattr(modifier, "canvas_settings", None)
            if canvas is not None:
                caches.extend(surface.point_cache for surface in canvas.canvas_surfaces)
            for cache in caches:
                if not smoozy_as_the_file_has_it(cache, "is_baked"):
                    return True
        for system in getattr(obj, "particle_systems", ()):
            # Hair with no hair dynamics simulates nothing; its cache is never
            # baked and never used.
            if system.settings.type == "HAIR" and not system.use_hair_dynamics:
                continue
            cache = system.point_cache
            if not smoozy_as_the_file_has_it(cache, "is_baked"):
                return True
    return False


def the_replay_fluid_domains():
    # **`Job files and outputs` D20's fifth place, added 2026-10-02**: every
    # fluid domain whose cache type is Replay, and no other — the same walk as
    # `Rendering — machine`'s `smoozy_carriers` for `EveryReplayFluidDomain`.
    # A domain of another cache type is left as the file has it.
    for obj in bpy.data.objects:
        for modifier in getattr(obj, "modifiers", ()):
            if modifier.type != "FLUID" or modifier.fluid_type != "DOMAIN":
                continue
            if modifier.domain_settings.cache_type == "REPLAY":
                yield modifier.domain_settings


def the_point_caches_the_redirect_sets():
    # **`Job files and outputs` D21b-i, corrected 2026-10-02**: the caches the
    # material sets, and no other — a cloth or soft body modifier's, a dynamic
    # paint canvas surface's and a particle system's, **where the file writes
    # the cache to disk** (`use_disk_cache` on). A cache kept in memory writes
    # nothing to disk and is left as the file has it, and a rigid-body world's
    # is never set, so neither records a place the job writes. The same walk
    # as `Rendering — machine`'s `smoozy_carriers`.
    for obj in bpy.data.objects:
        for modifier in getattr(obj, "modifiers", ()):
            if getattr(modifier, "point_cache", None) is not None:
                if modifier.point_cache.use_disk_cache:
                    yield modifier.point_cache
            canvas = getattr(modifier, "canvas_settings", None)
            if canvas is not None:
                for surface in getattr(canvas, "canvas_surfaces", ()):
                    if getattr(surface, "point_cache", None) is not None:
                        if surface.point_cache.use_disk_cache:
                            yield surface.point_cache
        for system in getattr(obj, "particle_systems", ()):
            if getattr(system, "point_cache", None) is not None:
                if system.point_cache.use_disk_cache:
                    yield system.point_cache


def the_baked_caches_on_the_machine():
    # **`Job files and outputs` D21b-i, added 2026-10-02: a baked physics
    # cache whose files are on the machine where the file records them is
    # copied to its job-folder value before the render.** This says which.
    # Where its files sit is its recorded folder where *external* is on, else
    # the default folder beside the `.blend` (D21c): `blendcache_` and the
    # file's own name without its ending. A cache marked baked whose folder
    # is not on this machine is left out, and renders as it would at home.
    #
    # **A cache on a linked object goes by its library, 2026-11-04** (D21b-iii):
    # observed at the desk in 5.2.3, a linked cloth reads `blendcache_` and the
    # library's name beside the library, and an external one measures `//` from
    # the library. **The library's real place is looked at as well as where it was
    # opened**, since `Dependency Upload & Remap` M8b's link stands in one folder
    # and its files in another. **And what is handed on is the real folder, never
    # a link**: the copy refuses one (`Dependency Upload & Remap` M22c), and
    # through that same folder everything beside the file is reached through a
    # link.
    found = []
    for cache in the_point_caches_the_redirect_sets():
        # **R18**: as the file had them, never the read's throwaway values.
        if not smoozy_as_the_file_has_it(cache, "is_baked"):
            continue
        recorded = smoozy_as_the_file_has_it(cache, "filepath")
        library = cache.id_data.library
        opened = bpy.path.abspath(library.filepath) if library is not None else bpy.data.filepath
        for blend in (opened, os.path.realpath(opened)):
            if smoozy_as_the_file_has_it(cache, "use_external"):
                # The blend-file shorthand `//` is the folder the `.blend` sits in.
                where = recorded
                if where.startswith("//"):
                    where = os.path.join(os.path.dirname(blend), where[2:])
            else:
                where = os.path.join(
                    os.path.dirname(blend),
                    "blendcache_" + os.path.splitext(os.path.basename(blend))[0],
                )
            if os.path.isdir(where):
                break
        if not os.path.isdir(where):
            continue
        one = {
            "recorded_location": as_bytes(recorded) if recorded else None,
            "its_files_sit_in": as_bytes(os.path.realpath(where)),
        }
        if one not in found:
            found.append(one)
    return found


def what_each_carrier_of_an_added_write_place_records(scene, place):
    # **`Job files and outputs` R4 item 2, corrected 2026-10-02**: one value
    # per carrier of the place's setting, `""` where that carrier records
    # none — the same carriers, in the same walk, as the material's
    # `smoozy_carriers` sets. Each distinct value becomes its own destination.
    if place == "TheBakeOutputPath":
        return [scene.render.bake.filepath]
    if place == "ThePointCache":
        values = [
            smoozy_as_the_file_has_it(cache, "filepath")
            for cache in the_point_caches_the_redirect_sets()
        ]
        return values
    if place == "TheGeometryNodesBakeFolder":
        values = []
        for obj in bpy.data.objects:
            for modifier in getattr(obj, "modifiers", ()):
                if not hasattr(modifier, "bake_directory"):
                    continue
                values.append(modifier.bake_directory)
                for bake in modifier.bakes:
                    values.append(bake.directory)
        return values
    if place == "TheSequencerProxyFolder":
        editor = scene.sequence_editor
        if editor is None:
            return []
        values = [editor.proxy_dir]
        for strip in editor.strips_all:
            proxy = getattr(strip, "proxy", None)
            if proxy is not None:
                values.append(proxy.directory)
        return values
    if place == "TheReplayFluidCacheFolder":
        return [
            smoozy_as_the_file_has_it(domain, "cache_directory")
            for domain in the_replay_fluid_domains()
        ]
    raise Exception("smoozy: an added write place with no reader: " + place)


def the_destinations_of_an_added_write_place(scene, place):
    # **One destination for each distinct location the file records for the
    # place, and one for the carriers that record none** (`Job files and
    # outputs` R4 item 2, P7a). **A place no carrier records anything for
    # still enters the set** (D21b), as the one destination with no location,
    # which is every place on a file that carries none of it.
    recorded = what_each_carrier_of_an_added_write_place_records(scene, place)
    locations = []
    for value in recorded:
        if value and value not in locations:
            locations.append(value)
    if not locations or any(not value for value in recorded):
        locations.append(None)
    return [
        {
            "kind": {"AnAddedWritePlace": place},
            "recorded_location": as_bytes(value) if value else None,
            "format": None,
            "origin": {"AnAddedWritePlaceNamed": place},
            "scene": "TheJobsOwnScene",
            "the_view_layers_that_feed_it": [],
        }
        for value in locations
    ]


def types_that_reach_a_mark():
    # **No catch anywhere in this walk.** A name in `bpy.types` that is not a
    # kind has no description of itself and is passed over by the test below,
    # which is a state rather than a silence; anything that raises is a reading
    # that did not complete, and that is not a reading.
    holds = {}
    points = {}
    for name in dir(bpy.types):
        kind = getattr(bpy.types, name, None)
        rna = getattr(kind, "bl_rna", None)
        if rna is None:
            continue
        if rna.identifier in holds:
            continue
        own = False
        onwards = set()
        for prop in rna.properties:
            if prop.identifier in SMOOZY["the_walk_steps_over"]:
                continue
            if prop.type == "STRING":
                if (
                    prop.is_path_output
                    or prop.is_path_supports_blend_relative
                    or prop.is_path_supports_templates
                ):
                    own = True
            elif prop.type in ("POINTER", "COLLECTION"):
                onwards.add(prop.fixed_type.identifier)
        # **A list says what kind its members are of, and its members are
        # often of a narrower kind than that.** A tree's nodes are declared as
        # nodes; the one that carries a folder is a file output node. So what a
        # kind holds and what it leads to are both carried up to the kinds it
        # is a kind of, and the walk reads each thing's own description when it
        # gets there. **Without this the walk never steps into a list at all
        # where the mark is on a narrower kind**, and every file output node's
        # folder goes unfound.
        wider = rna
        while wider is not None:
            holds[wider.identifier] = holds.get(wider.identifier, False) or own
            points.setdefault(wider.identifier, set()).update(onwards)
            wider = wider.base
    reaches = set(name for name, own in holds.items() if own)
    moved = True
    while moved:
        moved = False
        for name, onwards in points.items():
            if name not in reaches and (onwards & reaches):
                reaches.add(name)
                moved = True
    return reaches


def the_recorded_locations():
    reaches = types_that_reach_a_mark()
    found = []
    already = set()
    seen = set()
    frontier = []
    # **What the file holds, by Blender's own description of `bpy.data` rather
    # than by a list of collection names kept here.** Everything else on it — a
    # path, a version, a switch — is not a collection and is not walked.
    for prop in bpy.data.bl_rna.properties:
        if prop.type != "COLLECTION":
            continue
        frontier.extend(getattr(bpy.data, prop.identifier))
    while frontier:
        one = frontier.pop()
        rna = getattr(one, "bl_rna", None)
        if rna is None:
            continue
        key = (rna.identifier, one.as_pointer())
        if key in seen:
            continue
        seen.add(key)
        for prop in rna.properties:
            if prop.identifier in SMOOZY["the_walk_steps_over"]:
                continue
            if prop.type == "STRING":
                if not (
                    prop.is_path_output
                    or prop.is_path_supports_blend_relative
                    or prop.is_path_supports_templates
                ):
                    continue
                # **R18**: as the file had it, never the read's throwaway folder.
                value = smoozy_as_the_file_has_it(one, prop.identifier)
                if not value:
                    continue
                raw = tuple(as_bytes(value))
                if raw in already:
                    continue
                already.add(raw)
                found.append(list(raw))
            elif prop.type == "POINTER":
                if prop.fixed_type.identifier in reaches:
                    onwards = getattr(one, prop.identifier, None)
                    if onwards is not None:
                        frontier.append(onwards)
            elif prop.type == "COLLECTION":
                if prop.fixed_type.identifier in reaches:
                    # **Not caught.** A collection that will not enumerate loses
                    # everything under it, which could be a place the render
                    # writes, so it ends the reading loudly instead.
                    frontier.extend(getattr(one, prop.identifier))
    return found


def socket_index(sockets, wanted):
    for at, one in enumerate(sockets):
        if one.as_pointer() == wanted.as_pointer():
            return at
    raise Exception("smoozy: a link ends at a socket its node does not carry")


def picture_sockets(sockets):
    return [one for one in sockets if one.type == NAMES["a_picture_sockets_kind"]]


def the_kind_of(node, top):
    if is_kind(node, NAMES["the_render_layers_nodes_type"]):
        return {
            "RenderLayers": {
                "scene": as_bytes(node.scene.name if node.scene is not None else ""),
                "view_layer": as_bytes(node.layer),
                "outputs": [as_bytes(one.name) for one in node.outputs],
            }
        }
    if is_kind(node, NAMES["the_file_output_nodes_type"]):
        # **R4 item 2: a muted node contributes none**, so the walk is told.
        return {
            "AFileOutputNode": {
                "inputs": [as_bytes(own) for _, _, _, own in the_outputs_of(node)],
                "gathering_into_one_file": gathers_into_one_file(node),
                "muted": bool(node.mute),
            }
        }
    if is_kind(node, NAMES["the_groups_own_input_type"]):
        # **`Job files and outputs` B1a, from 5.3: at the top of an effect's
        # group it is where the render enters.** Inside a nested group it stays
        # B3's first limit.
        if top and NAMES["the_scenes_effects"]:
            return {"TheRendersEntry": {"outputs": [as_bytes(one.name) for one in node.outputs]}}
        return "TheGroupsOwnInput"
    if is_kind(node, NAMES["the_final_output_nodes_type"]) and top:
        return "TheFinalOutputNode"
    if is_kind(node, NAMES["the_groups_own_output_type"]):
        return "TheGroupsOwnOutput"
    inner = getattr(node, "node_tree", None)
    if inner is not None:
        return {"ANodeGroup": {"inner": the_tree_as_read(inner, False)}}
    pictures_in = picture_sockets(node.inputs)
    pictures_out = picture_sockets(node.outputs)
    if len(pictures_out) == 1 and len(pictures_in) == 1:
        return "OnePictureInOnePictureOut"
    if len(pictures_out) == 1 and len(pictures_in) == 2:
        return {
            "ATwoPictureNode": {
                "the_factor_sits_between_its_ends": False,
                "which_input_came_out": None,
            }
        }
    return "ANodeTheWalkCannotPass"


def the_tree_as_read(tree, top):
    at_of = {}
    nodes = []
    for at, node in enumerate(tree.nodes):
        at_of[node.name] = at
        nodes.append({"name": as_bytes(node.name), "kind": the_kind_of(node, top)})
    links = []
    for link in tree.links:
        giver = at_of.get(link.from_node.name)
        taker = at_of.get(link.to_node.name)
        if giver is None or taker is None:
            continue
        links.append(
            {
                "from": {
                    "node": giver,
                    "index": socket_index(link.from_node.outputs, link.from_socket),
                },
                "to": {
                    "node": taker,
                    "index": socket_index(link.to_node.inputs, link.to_socket),
                },
            }
        )
    return {"nodes": nodes, "links": links}


def the_curves_of(holder):
    data = getattr(holder, "animation_data", None)
    if data is None:
        return [], []
    drivers = list(getattr(data, "drivers", ()))
    action = getattr(data, "action", None)
    if action is None:
        return [], drivers
    if NAMES["the_curves_hang_off_the_action"]:
        return list(action.fcurves), drivers
    curves = []
    for layer in action.layers:
        for strip in layer.strips:
            for bag in getattr(strip, "channelbags", ()):
                curves.extend(bag.fcurves)
    return curves, drivers


def names_the_layer(data_path, layer_name):
    if not data_path.endswith("].use"):
        return False
    opens = data_path.find("[")
    closes = data_path.rfind("]")
    if opens < 0 or closes < opens:
        return False
    return data_path[opens + 1 : closes].strip("\"'") == layer_name


def the_key_frames_of(scene):
    frames = set()
    curves, _ = the_curves_of(scene)
    for curve in curves:
        if not curve.data_path.endswith("].use"):
            continue
        for key in curve.keyframe_points:
            frames.add(int(round(key.co[0])))
    return frames


def the_span_of(scene, extra):
    span = set(extra)
    step = max(1, int(scene.frame_step))
    span |= set(range(int(scene.frame_start), int(scene.frame_end) + 1, step))
    keys = the_key_frames_of(scene)
    span |= keys
    span |= set(one - 1 for one in keys)
    span |= set(one + 1 for one in keys)
    return sorted(span)


def the_switch_animation(scene, layer, table):
    curves, drivers = the_curves_of(scene)
    for curve in drivers:
        if names_the_layer(curve.data_path, layer.name):
            return {
                "Driven": {
                    "expression": curve.driver.expression,
                    "valid": bool(curve.driver.is_valid),
                }
            }
    for curve in curves:
        if not names_the_layer(curve.data_path, layer.name):
            continue
        keys = []
        for key in curve.keyframe_points:
            keys.append(
                {
                    "frame": int(round(key.co[0])),
                    "value": bool(key.co[1]),
                    "interpolation": "Constant"
                    if key.interpolation == "CONSTANT"
                    else "Other",
                }
            )
        if not keys:
            return {
                "Keyframed": {
                    "keys": [],
                    "before_the_first_key": bool(layer.use),
                    "after_the_last_key": bool(layer.use),
                }
            }
        first = min(one["frame"] for one in keys)
        last = max(one["frame"] for one in keys)
        return {
            "Keyframed": {
                "keys": keys,
                "before_the_first_key": table.get(first - 1, keys[0]["value"]),
                "after_the_last_key": table.get(last + 1, keys[-1]["value"]),
            }
        }
    return {"None": {"value": bool(layer.use)}}


def the_size_at(full, percentage):
    return [(full[0] * percentage) // 100, (full[1] * percentage) // 100]


the_jobs_scene = bpy.context.scene
if SMOOZY["the_jobs_scene"] is not None:
    the_jobs_scene = bpy.data.scenes[SMOOZY["the_jobs_scene"]]

# **`Job files and outputs` R4b: at pre-load, where no job scene is named, the
# one reading also holds every other scene's outputs half**, composed exactly
# as for a named scene. The render-start reading never sets it.
EVERY_SCENE = bool(SMOOZY.get("every_scene")) and SMOOZY["the_jobs_scene"] is None


def the_scenes_reached_from(scene):
    out = []
    editor = scene.sequence_editor
    if editor is not None:
        for strip in editor.strips_all:
            if strip.type == "SCENE" and strip.scene is not None and strip.scene != scene:
                out.append((strip, strip.scene))
    return out


reached = the_scenes_reached_from(the_jobs_scene)

# **`Job files and outputs` R4c, added 2026-10-27 (the Creator's nineteenth
# notes, notes 3 and 9): a first-frame check of a job whose only output is a
# video is read, and rendered, as a still picture.** Asked before anything
# else is read, so every destination, every frame's name and the map below are
# a picture's. Only the render-start reading of a check is told. **The scene
# writes only a video** where its regular output path writes one, where 5.2's
# *write frames* switch is on, and where no file output node stands unmuted in
# its trees or in a scene a sequencer strip reaches — a node never writes
# video, so its pictures already give the check a preview. Nothing is saved.
def smoozy_writes_only_a_video(scene, its_reached):
    if not scene.render.is_movie_format:
        return False
    if NAMES["the_save_output_switch"] and not getattr(scene.render, NAMES["the_save_output_switch"]):
        return False
    for one in [scene] + [inner for _, inner in its_reached]:
        for tree, _stack in every_tree_of(one):
            for node in tree.nodes:
                if is_kind(node, NAMES["the_file_output_nodes_type"]) and not node.mute:
                    return False
    return True


THE_CHECK_IS_WRITTEN_AS_A_PICTURE = False
if SMOOZY.get("the_check_is_written_as_a_picture") and smoozy_writes_only_a_video(the_jobs_scene, reached):
    still = SMOOZY["the_picture_the_check_is_written_as"]
    image_settings = the_jobs_scene.render.image_settings
    if still["media"] is not None:
        setattr(image_settings, NAMES["the_media_kind"], still["media"])
    image_settings.file_format = still["format"]
    THE_CHECK_IS_WRITTEN_AS_A_PICTURE = not the_jobs_scene.render.is_movie_format

# **`Job files and outputs` P18a, added 2026-10-31 (the Creator's twenty-first
# notes, note 6): a first-frame check that writes into its range job's folder
# has every name it will write marked at the start of its last piece**, before
# anything below reads a name — so every destination, every frame's name, the
# map and the inventory are the marked ones, and nothing downstream carries a
# case of its own. Only the render-start reading of such a check is told.
# Nothing is saved.
if SMOOZY.get("the_check_is_marked"):
    smoozy_the_check_is_marked(
        the_jobs_scene, SMOOZY["the_names_the_mark_is_set_by"], SMOOZY["the_checks_mark"], True
    )

# ---- the stepped tables, one walk of the frames per scene.
#
# **Where no frames were handed in, the per-frame table covers the whole span
# the script already computes** — the scene's own frame range by its own step,
# unioned with the switch's key frames and their neighbours. **That is the build
# runner's ruling of 2026-09-20, and it is what stops the artist's file being read
# twice on the ordinary job.** A job whose range is *the file's own* does not
# know its frames until the file has been read, so the driver used to read with
# no frames, learn the range, and read again over it — a second load of a heavy
# production file on every attempt of the commonest job there is.
#
# **A job that hands frames in is completely unaffected**: `the_jobs_frames` is
# non-empty, the gate is exactly what it was, and the table is exactly what it
# was.
#
# **A scene is moved to its frames only where something on it can move**
# (`Render Settings` Q3 and §13 item 22, `Job files and outputs` R4 item 5;
# the Creator's notes on 0.1.8, note 4). Moving a scene to a frame makes
# Blender rebuild the whole scene, which cost one to four seconds a scene at
# the desk, so a file of ten still scenes spent most of a minute here. Where
# nothing on the scene moves, the values read once stand for every frame of
# its span, and the tables, the per-frame entries and every value read after
# this loop come out byte for byte as they did when every scene was moved —
# proved over the 170-file sweep in all four Blenders (`Audits/Build/Step
# 10c/The Creator's notes on 0.1.8, piece 2 — what landed.md`). **The job's
# own scene follows the same rule**: `frame_path(frame=…)` takes the frame
# as its argument.
def something_on_it_moves(scene):
    # Keys, drivers or strips of its own — which is where its view layers'
    # switches, its render settings and 4.5's Use Nodes are animated.
    if scene.animation_data is not None:
        return True
    # Its compositing tree, or any group inside it: a node's mute is read
    # per frame by `gating_of`, and every file output node's after this loop.
    for tree, _stack in every_tree_of(scene):
        if tree.animation_data is not None:
            return True
    # A marker that names a camera changes the scene's camera at its frame
    # with no animation at all — *"Camera that becomes active on this
    # frame"* — and a path template can name the camera.
    for marker in scene.timeline_markers:
        if marker.camera is not None:
            return True
    return False


# **A video's name is asked over the job's own range** (the live Telegram
# pass's fault 4, added 2026-10-31). For a video, `frame_path` answers one name
# built from the scene's start and end at the moment it is asked, whatever the
# frame, and `-s <first> -e <last> -a` writes the file named for the range it
# renders — measured in six Blenders, 4.5 to 5.4 (`Audits/Build/Step 10c/A desk
# test — the name of a video over part of its range.md`). So where the job hands
# its frames in and the job's scene writes a video, its start and end are set
# to the job's first and last frame for that one question and put back at once:
# nothing else the reading reports is read over them. Set in an order that never
# puts the start after the end.
def the_range_is_set_on(scene, start, end):
    if start > int(scene.frame_end):
        scene.frame_end = end
        scene.frame_start = start
    else:
        scene.frame_start = start
        scene.frame_end = end


def the_videos_name_over_the_jobs_range(scene, frame):
    saved = (int(scene.frame_start), int(scene.frame_end))
    the_range_is_set_on(scene, FRAMES[0], FRAMES[-1])
    try:
        return scene.render.frame_path(frame=frame)
    finally:
        the_range_is_set_on(scene, saved[0], saved[1])


def the_entry_at(scene, frame, errors, the_jobs_scene_is_asked=False):
    answered = None
    try:
        if the_jobs_scene_is_asked and FRAMES and scene.render.is_movie_format:
            answered = the_videos_name_over_the_jobs_range(scene, frame)
        else:
            answered = scene.render.frame_path(frame=frame)
    except Exception as why:
        errors["the regular output path at frame " + str(frame)] = str(why)
    return {
        "frame": frame,
        "view_layers_that_render": [
            as_bytes(layer.name) for layer in scene.view_layers if layer.use
        ],
        "gating": gating_of(scene),
        "the_regular_output_paths_answered_path": as_bytes(answered)
        if answered is not None
        else None,
    }


# **`Job files and outputs` R4 item 2**: which effects each scene has on at some
# frame it was stepped through, by name. Filled by the walk just below.
EFFECTS_ON_AT_SOME_FRAME = {}
tables = {}
per_frame = []
the_jobs_frames = set(FRAMES)
the_other_scenes = []
the_other_scenes_frames = {}
the_other_scenes_errors = {}
for scene in bpy.data.scenes:
    if EVERY_SCENE and scene != the_jobs_scene:
        the_other_scenes.append(scene)
    extra = FRAMES if scene == the_jobs_scene else []
    moves = something_on_it_moves(scene)
    for frame in the_span_of(scene, extra):
        if moves:
            scene.frame_set(frame)
        if NAMES["the_scenes_effects"]:
            EFFECTS_ON_AT_SOME_FRAME.setdefault(scene.name, set()).update(
                one[0] for one in the_effects_of(scene) if one[2]
            )
        for layer in scene.view_layers:
            tables.setdefault((scene.name, layer.name), []).append([frame, bool(layer.use)])
        if scene == the_jobs_scene and (not the_jobs_frames or frame in the_jobs_frames):
            per_frame.append(the_entry_at(scene, frame, ERRORS, True))
        elif EVERY_SCENE and scene != the_jobs_scene:
            # **R4b: each other scene's per-frame set, recorded inside the
            # same walk**, so nothing is stepped twice. Its errors are its own:
            # a scene the job does not name never makes the reading fail.
            its_errors = the_other_scenes_errors.setdefault(scene.name, {})
            its_frames = the_other_scenes_frames.setdefault(scene.name, [])
            try:
                its_frames.append(the_entry_at(scene, frame, its_errors))
            except Exception as why:
                its_errors["the reading of this scene at frame " + str(frame)] = str(why)

# The endings every format in play writes, asked of Blender before anything
# else reads `bpy.data`, so the scene made for the asking is gone again before
# the walk over the marks and before the settings half enumerate the scenes.
def the_formats_of(scenes, into):
    for one in scenes:
        for tree, _stack in every_tree_of(one):
            for node in tree.nodes:
                if not is_kind(node, NAMES["the_file_output_nodes_type"]):
                    continue
                if node.mute:
                    continue
                into.append(node.format)
                for _, _, entry, _ in the_outputs_of(node):
                    if entry is None:
                        continue
                    switch = bool(getattr(entry, NAMES["the_per_output_format_switch"]))
                    if switch == NAMES["the_switch_is_on_when_the_output_has_its_own_format"]:
                        into.append(entry.format)


the_formats = []
the_formats_of([the_jobs_scene] + [scene for _, scene in reached], the_formats)
# **R4b: every other scene's formats are asked in the same one asking**, so the
# scene made for it is gone before anything enumerates the scenes.
for other in the_other_scenes:
    try:
        the_formats_of([other] + [scene for _, scene in the_scenes_reached_from(other)], the_formats)
    except Exception as why:
        the_other_scenes_errors.setdefault(other.name, {})["the reading of this scene's formats"] = str(why)
ENDINGS.update(the_endings_of(the_formats))


def the_destinations_of_the_scene(the_scene, its_reached):
    out = destinations_of(the_scene, "TheJobsOwnScene")
    for strip, scene in its_reached:
        out.extend(
            destinations_of(
                scene,
                {
                    "ReachedThroughASequencerStrip": {
                        "scene": as_bytes(scene.name),
                        "strip": as_bytes(strip.name),
                    }
                },
            )
        )
    for place in THE_ADDED_WRITE_PLACES:
        out.extend(the_destinations_of_an_added_write_place(the_scene, place))
    return out


def the_views_of(the_scene):
    if not the_scene.render.use_multiview:
        return "One"
    if the_scene.render.image_settings.views_format != "INDIVIDUAL":
        return "MoreThanOneGatheredIntoOneFile"
    return {
        "MoreThanOneWrittenSeparately": {
            "how_many": len([one for one in the_scene.render.views if one.use])
        }
    }


def the_tree_shared_with(the_scene):
    if not NAMES["the_four_five_use_nodes_switch"]:
        mine = [one[1] for one in the_effects_of(the_scene)]
        if mine:
            for scene in bpy.data.scenes:
                if scene != the_scene and any(one[1] is tree for one in the_effects_of(scene) for tree in mine):
                    return as_bytes(scene.name)
    return None


# **`Job files and outputs` B1a**: from 5.3 the stack, as a tree of effects in list
# order with no links; before it the one tree.
def the_compositing_tree_of(the_scene):
    effects = the_effects_of(the_scene)
    if not effects:
        return None
    if not NAMES["the_scenes_effects"]:
        return the_tree_as_read(effects[0][1], True)
    return {
        "nodes": [
            {"name": as_bytes(name), "kind": {"AnEffect": {"inner": the_tree_as_read(tree, True)}}}
            for name, tree, _on in effects
        ],
        "links": [],
    }


def the_outputs_half_of(the_scene, its_reached, its_per_frame, its_destinations, its_views, its_shared_with, recorded):
    # **`Job files and outputs` R4 item 1**: from 5.3 the scene's own gating is
    # its first frame's, the frame B1 enters at, since a keyed effect switch
    # would otherwise be read at wherever the stepping stopped.
    gating = gating_of(the_scene)
    if NAMES["the_scenes_effects"] and its_per_frame:
        gating = its_per_frame[0]["gating"]
    return {
        "taken_by": {"major": bpy.app.version[0], "minor": bpy.app.version[1]},
        "gating": gating,
        "destinations": its_destinations,
        "scenes_reached_through_a_sequencer_strip": [
            {
                "scene": as_bytes(scene.name),
                "strip": as_bytes(strip.name),
                "gating": gating_of(scene),
            }
            for strip, scene in its_reached
        ],
        "the_tree_is_shared_with": its_shared_with,
        "per_frame": its_per_frame,
        "recorded_locations": recorded,
        "views": its_views,
        "the_compositing_tree": the_compositing_tree_of(the_scene),
        # `Job files and outputs` B25: the job's own view transform, which a
        # derivative of a working-format picture is made in.
        "the_view_transform": as_bytes(the_scene.view_settings.view_transform),
    }


destinations = the_destinations_of_the_scene(the_jobs_scene, reached)
views = the_views_of(the_jobs_scene)
shared_with = the_tree_shared_with(the_jobs_scene)

# **R4b: every other scene's half, composed exactly as for a named scene.** Its
# recorded locations are the file's, not the scene's, and ride once, in the
# job's half; a reader puts them back. Anything that goes wrong reading one of
# these scenes is that scene's error, and never the reading's.
the_other_halves = None
if EVERY_SCENE:
    the_other_halves = []
    for other in the_other_scenes:
        its_errors = the_other_scenes_errors.setdefault(other.name, {})
        half = None
        simulation = False
        try:
            its_reached = the_scenes_reached_from(other)
            half = the_outputs_half_of(
                other,
                its_reached,
                the_other_scenes_frames.get(other.name, []),
                the_destinations_of_the_scene(other, its_reached),
                the_views_of(other),
                the_tree_shared_with(other),
                [],
            )
            simulation = the_scene_holds_a_simulation_not_baked(other)
        except Exception as why:
            its_errors["the reading of this scene"] = str(why)
        the_other_halves.append(
            {
                "scene": as_bytes(other.name),
                "outputs": half,
                "the_scene_holds_a_simulation_not_baked": simulation,
                "errors": its_errors,
            }
        )

scenes_out = []
for scene in bpy.data.scenes:
    layers = []
    for layer in scene.view_layers:
        table = tables.get((scene.name, layer.name), [])
        layers.append(
            {
                "name": layer.name,
                "renders_at": table,
                "animation": the_switch_animation(
                    scene, layer, dict((at, value) for at, value in table)
                ),
                "own_sample_count": int(layer.samples),
                "overrides_the_scenes_count": int(layer.samples) != 0,
            }
        )
    pulls_in = []
    for tree, _stack in every_tree_of(scene):
        for node in tree.nodes:
            if is_kind(node, NAMES["the_render_layers_nodes_type"]):
                if node.scene is not None and node.scene != scene:
                    pulls_in.append({"scene": node.scene.name, "route": "TheCompositor"})
    its_editor = scene.sequence_editor
    if its_editor is not None:
        for strip in its_editor.strips_all:
            if strip.type == "SCENE" and strip.scene is not None and strip.scene != scene:
                pulls_in.append({"scene": strip.scene.name, "route": "TheVideoSequencer"})
    full = [int(scene.render.resolution_x), int(scene.render.resolution_y)]
    percentage = int(scene.render.resolution_percentage)
    step = max(1, int(scene.frame_step))
    count = 0
    if int(scene.frame_end) >= int(scene.frame_start):
        count = (int(scene.frame_end) - int(scene.frame_start)) // step + 1
    scenes_out.append(
        {
            "name": scene.name,
            "frame_start": int(scene.frame_start),
            "frame_end": int(scene.frame_end),
            "frame_step": step,
            "frame_count": count,
            # `Render Settings` Q2b: the same search, without extra playing.
            "prepare_simulations": the_scene_holds_a_simulation_not_baked(scene),
            "motion_blur": smoozy_motion_blur(scene),
            "full_size": full,
            "resolution_percentage": percentage,
            "size_at_the_files_percentage": the_size_at(full, percentage),
            "engine_as_read": scene.render.engine,
            "sample_counts": {
                "cycles": int(scene.cycles.samples),
                "eevee": int(scene.eevee.taa_render_samples),
            },
            "device": "TheCards" if scene.cycles.device == "GPU" else "TheProcessor",
            "early_stop": {
                "adaptive_sampling_is_on": bool(scene.cycles.use_adaptive_sampling),
                "threshold": float(scene.cycles.adaptive_threshold),
                "minimum": int(scene.cycles.adaptive_min_samples),
            },
            "time_limit": float(scene.cycles.time_limit),
            **(
                {"eevee_time_limit": float(getattr(scene.eevee, NAMES["the_eevee_time_limit"]))}
                if NAMES["the_eevee_time_limit"]
                else {}
            ),
            "layer_samples_rule": {
                "USE": "Use",
                "BOUNDED": "Bounded",
                "IGNORE": "Ignore",
            }[scene.cycles.use_layer_samples],
            "saved_with": ".".join(str(one) for one in bpy.data.version),
            "view_layers": layers,
            "pulls_in": pulls_in,
            "simplify": {
                "use_simplify": bool(scene.render.use_simplify),
                "subdivision_render": int(scene.render.simplify_subdivision_render),
                "child_particles_render": float(
                    scene.render.simplify_child_particles_render
                ),
                "texture_limit_render": scene.cycles.texture_limit_render,
            },
        }
    )

recorded_locations = the_recorded_locations()

engine_classes = []
for kind in bpy.types.RenderEngine.__subclasses__():
    identifier = getattr(kind, "bl_idname", None)
    if identifier:
        engine_classes.append(identifier)
eevee = None
for item in the_jobs_scene.render.bl_rna.properties["engine"].enum_items:
    eevee = item.identifier
    break

# **`Render Settings` Q2 item 7**: what P20's first stat would find, for the
# page's folder line. A name that is not text is left out, since no run can name
# it, and a list that will not read is no list.
def the_folders_at_the_machines_top():
    try:
        names = [one.name.decode("utf-8", "replace") for one in os.scandir(b"/") if one.is_dir()]
    except OSError:
        return None
    return sorted(name for name in names if "\ufffd" not in name)


document = {
    "outputs": the_outputs_half_of(
        the_jobs_scene, reached, per_frame, destinations, views, shared_with, recorded_locations
    ),
    "settings": {
        "the_folders_at_the_machines_top": the_folders_at_the_machines_top(),
        "scenes": scenes_out,
        "offered_engine_identifiers": {
            "cycles": NAMES["the_cycles_identifier"]
            if NAMES["the_cycles_identifier"] in engine_classes
            else None,
            "eevee": eevee,
        },
        "announced_add_on_case": None,
    },
    "dependencies": {},
    "errors": ERRORS,
    "the_baked_caches_on_the_machine": the_baked_caches_on_the_machine(),
    "the_scene_holds_a_simulation_not_baked": the_scene_holds_a_simulation_not_baked(the_jobs_scene),
    "the_scene_it_read": as_bytes(the_jobs_scene.name),
    # **`Job files and outputs` I2, added 2026-10-18**: the open file's own
    # path, which `{blend_name}` in a node's names is replaced from.
    "the_open_files_path": as_bytes(bpy.data.filepath),
    # **R4c**: whether this reading switched the job's scene to a still picture.
    "the_check_is_written_as_a_picture": THE_CHECK_IS_WRITTEN_AS_A_PICTURE,
}
if the_other_halves is not None:
    document["the_other_scenes"] = the_other_halves

sys.stdout.write(OPENS)
sys.stdout.write(json.dumps(document))
sys.stdout.write(CLOSES)
sys.stdout.flush()
"#;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::reading::WhichBranch;

    // `Job files and outputs` D21b-i: cloth preservation was withdrawn.
    #[test]
    fn memory_cloth_reserves_no_fallback_folder_beside_a_disk_cache() {
        let function = &THE_BODY[THE_BODY.find("def what_each_carrier_of_an_added_write_place_records").unwrap()
            ..THE_BODY.find("def the_destinations_of_an_added_write_place").unwrap()];
        let script = r#"
from types import SimpleNamespace as NS
memory=NS(use_disk_cache=False,is_baked=False)
bpy=NS(data=NS(objects=[NS(modifiers=[NS(type='CLOTH',point_cache=memory)])]))
smoozy_as_the_file_has_it=lambda holder,name:getattr(holder,name)
the_point_caches_the_redirect_sets=lambda:[NS(filepath='//other-cache')]
assert what_each_carrier_of_an_added_write_place_records(None,'ThePointCache') == ['//other-cache']
memory.is_baked=True
assert what_each_carrier_of_an_added_write_place_records(None,'ThePointCache') == ['//other-cache']
"#;
        let out=std::process::Command::new("python3").arg("-c").arg(format!("{function}\n{script}")).output().unwrap();
        assert!(out.status.success(),"{}",String::from_utf8_lossy(&out.stderr));
    }

    fn an_invocation(major: u16, minor: u16) -> TheInvocation {
        TheInvocation {
            the_blender_build_path: "/ephemeral/smoozy/blender/blender-4.5.12/blender".to_string(),
            the_blend_path_on_the_machine: RecordedLocation::of(
                b"/ephemeral/smoozy/assets/kitchen.blend".to_vec(),
            ),
            the_version_it_will_render_by: TheVersionInHand::of(major, minor),
            the_frames_the_job_covers: vec![1, 2, 3],
            the_check_is_written_as_a_picture: false,
            the_check_is_marked: false,
        }
    }

    fn the_script(major: u16, minor: u16) -> String {
        the_reading_script(&an_invocation(major, minor), None).expect("every name is on record")
    }

    /// **A stand-in Blender for P18a's mark**: plain objects shaped like the
    /// settings the mark touches, and the file names each would write, by the
    /// rule the desk test measured per family (`THE_NAME_START_DESK_TEST`) —
    /// 4.5: the scene's path, a plain node's folder then each output's own
    /// path, a multilayer node's folder property; 5.0: the node's folder, then
    /// its file name, then the output's name with a slash written `_`; from
    /// 5.1 the file name and the output's name joined and split at the
    /// slashes. **It runs [`THE_CHECKS_MARK_IN_BLENDER`] itself**, so the text
    /// Blender runs is the text tested. Prints, per case, each file name
    /// before and after the mark.
    const THE_STAND_IN: &str = r#"
import json, sys
given = json.loads(sys.argv[1])
NAMES = given["names"]
FAMILY = given["family"]

class A:
    def __init__(self, **kw):
        self.__dict__.update(kw)
    def as_pointer(self):
        return id(self)

def a_node(gathering, folder, file_name, outputs):
    node = A(bl_idname=NAMES["the_file_output_nodes_type"], node_tree=None)
    node.format = A(**{NAMES["the_gathering_mark"]["property"]:
                       NAMES["the_gathering_mark"]["value"] if gathering else "PNG"})
    setattr(node, NAMES["the_nodes_folder"], folder)
    if FAMILY != "4.5":
        node.file_name = file_name
    own = NAMES["an_outputs_own_piece_of_path"]
    setattr(node, NAMES["the_nodes_output_list"], [A(**{own: one}) for one in outputs])
    return node

def a_scene(path, nodes):
    scene = A(render=A(filepath=path), sequence_editor=None)
    tree = A(nodes=nodes)
    if NAMES["the_scenes_effects"]:
        setattr(scene, NAMES["the_scenes_effects"], [A(**{NAMES["an_effects_group"]: tree})])
    else:
        setattr(scene, NAMES["the_scenes_compositing_tree"], tree)
    return scene

def written(scene):
    out = [scene.render.filepath + "0001.png"]
    if NAMES["the_scenes_effects"]:
        tree = getattr(getattr(scene, NAMES["the_scenes_effects"])[0], NAMES["an_effects_group"])
    else:
        tree = getattr(scene, NAMES["the_scenes_compositing_tree"])
    for node in tree.nodes:
        folder = getattr(node, NAMES["the_nodes_folder"])
        mark = NAMES["the_gathering_mark"]
        gathering = getattr(node.format, mark["property"]) == mark["value"]
        own = NAMES["an_outputs_own_piece_of_path"]
        entries = getattr(node, NAMES["the_nodes_output_list"])
        if FAMILY == "4.5":
            if gathering:
                out.append(folder + "0001.exr")
            else:
                out += [folder + "/" + getattr(one, own) + "0001.png" for one in entries]
        elif gathering:
            out.append(folder + "/" + node.file_name + "0001.exr")
        elif FAMILY == "5.0":
            out += [folder + "/" + node.file_name + getattr(one, own).replace("/", "_") + "0001.png"
                    for one in entries]
        else:
            out += [folder + "/" + node.file_name + getattr(one, own) + "0001.png" for one in entries]
    return out

results = []
for case in given["cases"]:
    for every_setting in (True, False):
        scene = a_scene(case["scene"], [a_node(*one) for one in case["nodes"]])
        before = written(scene)
        smoozy_the_check_is_marked(scene, NAMES, "check_", every_setting)
        results.append({"every_setting": every_setting, "before": before, "after": written(scene)})
print(json.dumps(results))
"#;

    /// Runs the stand-in for one family, or nothing where this computer has
    /// no `python3` to run it with.
    fn the_stand_in_marks(
        family: &str,
        version: TheVersionInHand,
        cases: serde_json::Value,
    ) -> Option<Vec<serde_json::Value>> {
        let given = serde_json::json!({
            "names": the_names_the_mark_is_set_by(version).expect("every name is on record"),
            "family": family,
            "cases": cases,
        });
        let script = format!("{THE_CHECKS_MARK_IN_BLENDER}{THE_STAND_IN}");
        let ran = match std::process::Command::new("python3")
            .args(["-c", &script, &given.to_string()])
            .output()
        {
            Ok(ran) => ran,
            Err(why) => {
                eprintln!("python3 could not be started ({why}); the stand-in did not run");
                return None;
            }
        };
        assert!(ran.status.success(), "{}", String::from_utf8_lossy(&ran.stderr));
        Some(serde_json::from_slice(&ran.stdout).expect("the stand-in prints one document"))
    }

    /// The written name, split into its folder and its file's own name.
    fn split(path: &str) -> (&str, &str) {
        path.rsplit_once('/').unwrap_or(("", path))
    }

    /// **P18a, per Blender family: the mark lands at the start of every
    /// written file's name and no folder changes** — the scene output, a plain
    /// File Output input with and without a subfolder, multilayer, and a path
    /// ending in a slash. In the reading every setting is marked; in the
    /// render's material the regular output path and 4.5's multilayer folder
    /// are left alone, because they arrive marked through the map. The
    /// twenty-first notes, note 6.
    #[test]
    fn twenty_first_6_the_mark_lands_at_each_names_start_in_every_family() {
        let families = [
            (
                "4.5",
                TheVersionInHand::of(4, 5),
                serde_json::json!([
                    {"scene": "out/sub/shot_", "nodes": [
                        [false, "out/node", "", ["mist", "a/beauty_"]],
                        [true, "out/ml/mlname_", "", ["a/beauty_", "mist"]],
                    ]},
                    {"scene": "out/sub/", "nodes": [[true, "out/ml/", "", ["mist"]]]},
                ]),
            ),
            (
                "5.0",
                TheVersionInHand::of(5, 0),
                serde_json::json!([
                    {"scene": "//sub/shot_", "nodes": [
                        [false, "out/node", "nm_", ["mist", "a_beauty_"]],
                        [false, "out/empty", "", ["mist"]],
                        [true, "out/ml", "fsub/mlname_", ["mist"]],
                    ]},
                    {"scene": "out/sub/", "nodes": [[true, "out/ml", "", ["mist"]]]},
                ]),
            ),
            (
                "5.1",
                TheVersionInHand::of(5, 1),
                serde_json::json!([
                    {"scene": "out/sub/shot_", "nodes": [
                        [false, "out/flat", "nm_", ["mist"]],
                        [false, "out/sub", "nm_", ["a/beauty_"]],
                        [false, "out/mixed", "nm_", ["mist", "a/beauty_"]],
                        [false, "out/empty", "", ["mist", "a/beauty_"]],
                        [true, "out/ml", "mlname_", ["a/beauty_"]],
                    ]},
                    {"scene": "out/sub/", "nodes": [
                        [false, "out/slash", "fsub/", ["mist"]],
                        [true, "out/ml", "", ["mist"]],
                    ]},
                ]),
            ),
            // **From 5.3 the same names, reached through the stack of effects**
            // (`Job files and outputs` R4 item 1): the mark asks for no older name.
            (
                "5.1",
                TheVersionInHand::of(5, 3),
                serde_json::json!([
                    {"scene": "out/sub/shot_", "nodes": [
                        [false, "out/flat", "nm_", ["mist"]],
                        [true, "out/ml", "mlname_", ["a/beauty_"]],
                    ]},
                ]),
            ),
        ];
        for (family, version, cases) in families {
            let Some(results) = the_stand_in_marks(family, version, cases) else {
                return;
            };
            for result in results {
                let every_setting = result["every_setting"].as_bool().unwrap();
                let before = result["before"].as_array().unwrap();
                let after = result["after"].as_array().unwrap();
                assert_eq!(before.len(), after.len());
                for (at, (before, after)) in before.iter().zip(after).enumerate() {
                    let (before, after) = (before.as_str().unwrap(), after.as_str().unwrap());
                    let (folder, name) = split(before);
                    let carried_by_the_map = !every_setting
                        && (at == 0 || (family == "4.5" && name.ends_with(".exr")));
                    if carried_by_the_map {
                        assert_eq!(after, before, "{family}: the material leaves {before} to the map");
                        continue;
                    }
                    let marked = format!("check_{name}");
                    assert_eq!(split(after), (folder, marked.as_str()), "{family}: {before}");
                }
            }
        }
    }

    /// **P18a's names come from the table, and each is cited to the desk
    /// test**: the scene's path everywhere; 4.5's output path and folder
    /// property; the node's file name from 5.0.
    #[test]
    fn twenty_first_6_the_names_the_mark_is_set_by_are_the_desk_tests() {
        for spelling in [
            WHERE_THE_SCENES_NAME_STARTS.four_five,
            WHERE_A_PLAIN_NODES_NAMES_START.four_five,
            WHERE_A_PLAIN_NODES_NAMES_START.from_five_zero,
            WHERE_A_GATHERING_NODES_NAME_STARTS.four_five,
            WHERE_A_GATHERING_NODES_NAME_STARTS.from_five_zero,
        ] {
            assert!(matches!(spelling, Some(Spelling::Cited { source, .. }) if source == THE_NAME_START_DESK_TEST));
        }
        let four = the_names_the_mark_is_set_by(TheVersionInHand::of(4, 5)).unwrap();
        assert_eq!(four["the_plain_nodes_names_start"], "path");
        assert_eq!(four["the_gathering_nodes_name_start"], "base_path");
        let five = the_names_the_mark_is_set_by(TheVersionInHand::of(5, 2)).unwrap();
        assert_eq!(five["the_plain_nodes_names_start"], "file_name");
        assert_eq!(five["the_gathering_nodes_name_start"], "file_name");
        assert_eq!(five["the_scenes_name_start"], "filepath");
    }

    /// **P18a: only the render-start reading of a check is told to mark**, and
    /// the script carries the text either way.
    #[test]
    fn twenty_first_6_only_a_checks_render_start_reading_marks() {
        let handed_in = |text: &str| -> serde_json::Value {
            let opening = "json.loads(r'''";
            let from = text.find(opening).unwrap() + opening.len();
            let to = from + text[from..].find("'''").unwrap();
            serde_json::from_str(&text[from..to]).unwrap()
        };
        let mut invocation = an_invocation(5, 1);
        assert_eq!(handed_in(&the_reading_script(&invocation, None).unwrap())["the_check_is_marked"], false);
        invocation.the_check_is_marked = true;
        let text = the_reading_script(&invocation, None).unwrap();
        assert_eq!(handed_in(&text)["the_check_is_marked"], true);
        assert_eq!(handed_in(&text)["the_checks_mark"], THE_CHECKS_MARK);
        assert!(text.contains("def smoozy_the_check_is_marked("));
        assert_eq!(handed_in(&the_pre_load_reading_script(&invocation).unwrap())["the_check_is_marked"], false);
    }

    /// **The script composes for every supported version and no name is an
    /// absence.** An absence would stop the composition and name itself; this
    /// is the check that none does.
    #[test]
    fn the_script_composes_for_all_four_versions_and_no_name_is_missing() {
        for (major, minor) in [(4u16, 5u16), (5, 0), (5, 1), (5, 2)] {
            let text = the_script(major, minor);
            assert!(text.contains("\nimport json\nSMOOZY = json.loads(r'''"));
            assert!(text.contains(&format!("\"composed_for\":[{major},{minor}]")));
        }
    }

    /// **The trap the lookup found, held: the per-output format switch is a
    /// different name on the two sides of 5.0 and means the opposite thing.**
    ///
    /// A reader carrying 4.5's pair into 5.x reads every output backwards and
    /// never raises, because both properties exist under their own version and
    /// both start off. This is the one name on the list where being wrong is
    /// silent, so it is held by name **and** by sense.
    #[test]
    fn the_per_output_format_switch_changes_name_and_sense_together_at_five_zero() {
        let four_five = TheVersionInHand::of(4, 5);
        assert_eq!(
            THE_PER_OUTPUT_FORMAT_SWITCH.for_version(four_five).unwrap(),
            Some("use_node_format")
        );
        assert!(!the_switch_is_on_when_the_output_has_its_own_format(
            four_five
        ));
        for minor in [0u16, 1, 2] {
            let version = TheVersionInHand::of(5, minor);
            assert_eq!(
                THE_PER_OUTPUT_FORMAT_SWITCH.for_version(version).unwrap(),
                Some("override_node_format")
            );
            assert!(the_switch_is_on_when_the_output_has_its_own_format(version));
        }
        // And the composed document carries both, so the script never has to
        // work either out for itself.
        let in_four_five = the_script(4, 5);
        assert!(in_four_five.contains(r#""the_per_output_format_switch":"use_node_format""#));
        assert!(in_four_five
            .contains(r#""the_switch_is_on_when_the_output_has_its_own_format":false"#));
        let from_five = the_script(5, 2);
        assert!(from_five.contains(r#""the_per_output_format_switch":"override_node_format""#));
        assert!(
            from_five.contains(r#""the_switch_is_on_when_the_output_has_its_own_format":true"#)
        );
    }

    /// **The guard that makes the trap above impossible**, and it is the first
    /// thing the script does.
    /// **The two marks are this crate's own and are spelled once.** The
    /// script takes them from the handed-in document rather than carrying a
    /// second copy that could drift from the one the reader looks for.
    #[test]
    fn the_marks_the_script_writes_are_this_crates_own_and_are_spelled_once() {
        let text = the_script(5, 2);
        let opens = std::str::from_utf8(THE_OPENING_SENTINEL).expect("the mark is text");
        let closes = std::str::from_utf8(THE_CLOSING_SENTINEL).expect("the mark is text");
        assert_eq!(text.matches(opens).count(), 1, "the opening mark is spelled once");
        assert_eq!(text.matches(closes).count(), 1, "the closing mark is spelled once");
        assert!(text.contains(&format!("[\"{opens}\",\"{closes}\"]")));
        assert!(text.contains("OPENS = SMOOZY[\"the_marks\"][0]"));
    }

    /// **`Render Settings` Q8, the Local View crash: the started mark is the
    /// script's very first act**, before the handed-in document is parsed and
    /// before the guard, and is this crate's own mark spelled once. The
    /// document's opening mark is still written only with the document.
    #[test]
    fn the_started_mark_is_the_scripts_first_act_in_every_version() {
        let started = std::str::from_utf8(THE_STARTED_MARK).expect("the mark is text");
        for (major, minor) in [(4u16, 5u16), (5, 0), (5, 1), (5, 2)] {
            let text = the_script(major, minor);
            assert!(
                text.starts_with(&format!(
                    "import sys\nsys.stdout.write(\"{started}\")\nsys.stdout.flush()\nimport json\n"
                )),
                "{major}.{minor}"
            );
            assert_eq!(text.matches(started).count(), 1, "spelled once");
            let guard = text
                .find("if list(bpy.app.version[:2]) != SMOOZY[\"composed_for\"]:")
                .expect("the guard");
            assert!(text.find(started).unwrap() < guard, "before the guard");
        }
    }

    /// **The Local View crash: the walk steps over a 3D viewport's stored
    /// Local View**, and every property on the list reaches the script the
    /// walk reads it from.
    #[test]
    fn the_walk_steps_over_local_view_and_the_whole_list_reaches_the_script() {
        assert!(THE_WALK_STEPS_OVER.contains(&"local_view"));
        let text = the_script(5, 1);
        let listed = serde_json::to_string(THE_WALK_STEPS_OVER).expect("a list");
        assert!(
            text.contains(&format!("\"the_walk_steps_over\":{listed}")),
            "the script carries the whole list"
        );
        assert_eq!(
            text.matches("if prop.identifier in SMOOZY[\"the_walk_steps_over\"]:").count(),
            2,
            "both halves of the walk read it"
        );
    }

    #[test]
    fn the_script_refuses_to_run_in_a_version_it_was_not_composed_for() {
        let text = the_script(4, 5);
        let head = text
            .find("if list(bpy.app.version[:2]) != SMOOZY[\"composed_for\"]:")
            .expect("the guard is in the script");
        // Nothing that reads the file may come before it.
        let first_read = text.find("bpy.data.scenes").unwrap_or(usize::MAX);
        assert!(head < first_read, "the guard is not the first thing");
    }

    /// **The saved-with version is the file's own and not the program's.**
    ///
    /// The reading of 2026-09-19 named `bpy.app.version_file`, which is the
    /// running program's own number and answers the same for every file it
    /// opens — a wrong answer that never raises. **The file's own is
    /// `bpy.data.version`**, and the script reads that and nothing else.
    #[test]
    fn the_saved_with_version_is_the_files_own_and_the_program_number_is_nowhere() {
        let text = the_script(5, 2);
        assert!(text.contains("bpy.data.version"));
        assert!(
            !text.contains("version_file"),
            "the running program's own file-format number is not the file's"
        );
    }

    /// **R4 item 2, corrected 2026-10-02: an added write place gives one
    /// destination for each distinct location its carriers record, and one
    /// for the carriers that record none** — read off every carrier the
    /// material sets, and never off the first one found. Until then the script
    /// returned the first location and nothing else, so a file whose
    /// geometry-nodes bakes sat in four folders had one destination, and three
    /// sent folders had nowhere to go.
    #[test]
    fn an_added_write_place_gives_one_destination_for_each_location_it_records() {
        let text = the_script(5, 2);
        // One composition for the job's scene and, since R4b, every other.
        assert!(text.contains("out.extend(the_destinations_of_an_added_write_place(the_scene, place))"));
        assert!(text.contains("destinations = the_destinations_of_the_scene(the_jobs_scene, reached)"));
        assert!(text.contains("if value and value not in locations:"));
        assert!(text.contains("if not locations or any(not value for value in recorded):"));
        assert!(!text.contains("def an_added_write_places_location("));
        // **The point caches read are the ones the material sets** (D21b-i):
        // disk caches only, and no rigid-body world.
        let walk = &text[text.find("def the_point_caches_the_redirect_sets").unwrap()
            ..text.find("def what_each_carrier_of_an_added_write_place_records").unwrap()];
        assert!(walk.contains(".use_disk_cache"));
        assert!(!walk.contains("rigidbody_world"));
    }

    /// **D21b-i, added 2026-10-02: the reading lists every physics disk cache
    /// the file marks baked whose folder is on this machine** — its recorded
    /// folder where *external* is on, else the default folder beside the
    /// `.blend` (D21c) — so the render worker can copy it into its job-folder
    /// value. A baked cloth uploaded beside the `.blend` was never read until
    /// then.
    #[test]
    fn the_reading_lists_the_baked_caches_whose_files_are_on_the_machine() {
        let text = the_script(5, 2);
        let found = &text[text.find("def the_baked_caches_on_the_machine():").unwrap()
            ..text.find("def what_each_carrier_of_an_added_write_place_records").unwrap()];
        assert!(found.contains("for cache in the_point_caches_the_redirect_sets():"));
        assert!(found.contains(r#"if not smoozy_as_the_file_has_it(cache, "is_baked"):"#));
        assert!(found.contains(r#"if smoozy_as_the_file_has_it(cache, "use_external"):"#));
        assert!(found.contains(r#""blendcache_" + os.path.splitext(os.path.basename(blend))[0]"#));
        assert!(found.contains("if not os.path.isdir(where):"));
        // **2026-11-04: by the library where the object is linked, and never a link
        // handed on**, since the copy refuses one.
        assert!(found.contains("library = cache.id_data.library"));
        assert!(found.contains(r#""its_files_sit_in": as_bytes(os.path.realpath(where))"#));
        assert!(text.contains(
            r#""the_baked_caches_on_the_machine": the_baked_caches_on_the_machine(),"#
        ));
    }

    /// **R4 item 8, added 2026-10-02 for `Rendering` L10b: the reading says
    /// whether the job's scene holds a simulation that is not baked**, by the
    /// names read off the four Blenders (`crate::reading::THE_SIMULATION_LOOKUP`)
    /// — a simulation zone whatever its bake, a point cache not baked on any
    /// of the five carriers (the rigid-body world's included, hair with no
    /// dynamics left out), a Replay fluid domain — on the job's own scene,
    /// with whether a cache is baked read as the file had it (R18). **A
    /// printed reading and a recorded one without the item read as no.**
    #[test]
    fn the_reading_names_the_scene_it_read_and_an_older_reading_names_none() {
        // **The 38's piece 10, item 25**: the script prints the name of the
        // scene whose outputs half it read, the same object the half is read
        // from, and the document carries it whole.
        let text = the_script(5, 2);
        assert!(text.contains(r#""the_scene_it_read": as_bytes(the_jobs_scene.name),"#));
        assert!(text.contains("the_jobs_scene = bpy.context.scene"));
        let mut value = serde_json::to_value(a_printed_reading()).unwrap();
        let object = value.as_object_mut().unwrap();
        object.remove("the_scene_it_read");
        let older = serde_json::from_value::<ThePrintedReading>(value.clone()).unwrap();
        assert_eq!(older.the_scene_it_read, None);
        value
            .as_object_mut()
            .unwrap()
            .insert("the_scene_it_read".to_string(), serde_json::json!(b"Interior".to_vec()));
        let named = serde_json::from_value::<ThePrintedReading>(value).unwrap();
        assert_eq!(
            named.the_scene_it_read.as_ref().map(|name| name.as_bytes().to_vec()),
            Some(b"Interior".to_vec())
        );
        let enters = WhereTheRenderEnters {
            scene: ArtistsText::of(b"Interior".to_vec()),
            view_layer: ArtistsText::of(b"ViewLayer".to_vec()),
        };
        let document = the_one_document_from(named, &enters).expect("a document");
        assert_eq!(
            document.the_scene_it_read.map(|name| name.as_bytes().to_vec()),
            Some(b"Interior".to_vec())
        );
    }

    // --- a File Output node's names with curly braces (the thirteenth notes,
    // piece P; `Architecture/Job files and outputs` I2) ---

    /// **What Smoozy's reading printed in Blender 5.3 beta for the stand-in**
    /// of `tests/names_with_curly_braces_at_the_desk.rs`: nodes *File Output a*
    /// and *File Output b*, base file names `{blend_name} - a` and
    /// `{blend_name} - b`, output `Image`, folder `//../Render/_preview`.
    const THE_CURLY_BRACE_STAND_IN: &str =
        include_str!("../tests/the_curly_brace_stand_in_as_blender_5_3_printed_it.json");

    /// The stand-in's printed document, to be changed for one case.
    fn the_stand_ins_document() -> serde_json::Value {
        serde_json::from_str(THE_CURLY_BRACE_STAND_IN).expect("the real document parses")
    }

    /// One field of node *a* (destination 1) or *b* (destination 2), set.
    fn set_a_nodes(document: &mut serde_json::Value, at: usize, field: &str, to: &str) {
        let destination = &mut document["outputs"]["destinations"][at];
        let value = serde_json::json!(to.as_bytes());
        if field == "recorded_location" {
            destination["recorded_location"] = value;
        } else {
            destination["kind"]["AnOutputOfAFileOutputNode"][field] = value;
        }
    }

    /// The document between its marks, taken in as the machine takes it in.
    fn taken_in(document: &serde_json::Value) -> ThePrintedReading {
        let mut stream = THE_OPENING_SENTINEL.to_vec();
        stream.extend_from_slice(&serde_json::to_vec(document).unwrap());
        stream.extend_from_slice(THE_CLOSING_SENTINEL);
        the_printed_reading_in(&stream).expect("it reads")
    }

    const THE_STAND_INS_JOB: &str = "/home/ubuntu/Render/standin job";

    /// The reading composed, and the names it predicts under the job's folder.
    fn composed_and_named(printed: ThePrintedReading) -> (crate::composing::TheReadingAndWhatFollows, Vec<String>) {
        let composed = crate::composing::the_reading_and_everything_derived_from(
            printed,
            crate::composing::WhatTheReadingIsComposedAgainst {
                the_folder_the_blend_sits_in: b"/home/ubuntu/rendering",
                job_output_folder: THE_STAND_INS_JOB,
                enters_at: &WhereTheRenderEnters {
                    scene: ArtistsText::of(b"Scene".to_vec()),
                    view_layer: ArtistsText::of(b"a".to_vec()),
                },
                the_jobs_pins: &[],
            },
        )
        .expect("a reading");
        let crate::inventory::WhatTheInventoryIs::It(inventory) = &composed.inventory else {
            panic!("the inventory is composed: {:?}", composed.inventory);
        };
        let mut names: Vec<String> = inventory
            .entries
            .iter()
            .map(|entry| {
                let path = String::from_utf8_lossy(&entry.predicted_path).into_owned();
                path.strip_prefix(&format!("{THE_STAND_INS_JOB}/")).unwrap_or(&path).to_string()
            })
            .collect();
        names.sort();
        (composed, names)
    }

    fn the_replaced_text_of(printed: &ThePrintedReading, at: usize) -> crate::reading::TheNodesTextReplaced {
        printed.outputs.destinations[at].the_replaced_text().cloned().expect("the reading replaced it")
    }

    fn replaced(text: &str) -> crate::reading::WithItsBracesReplaced {
        crate::reading::WithItsBracesReplaced::Replaced(ArtistsText::of(text.as_bytes().to_vec()))
    }

    fn unresolved(expression: &str) -> crate::reading::WithItsBracesReplaced {
        crate::reading::WithItsBracesReplaced::Unresolved {
            the_expression: ArtistsText::of(expression.as_bytes().to_vec()),
        }
    }

    /// **The script prints the open file's own path, and the reading taken in
    /// puts `{blend_name}` in from it**: *File Output a*'s `{blend_name} - a`
    /// is read as `standin - a`, its folder and its output as they are; the
    /// names predicted are the ones Blender wrote.
    #[test]
    fn the_reading_puts_blend_name_in_from_the_open_files_own_path() {
        for (major, minor) in [(4, 5), (5, 0), (5, 1), (5, 2), (5, 3)] {
            assert!(
                the_script(major, minor).contains(r#""the_open_files_path": as_bytes(bpy.data.filepath),"#),
                "{major}.{minor}"
            );
        }
        let printed = taken_in(&the_stand_ins_document());
        assert_eq!(
            the_replaced_text_of(&printed, 1),
            crate::reading::TheNodesTextReplaced {
                the_folder: replaced("//../Render/_preview"),
                the_base_file_name: Some(replaced("standin - a")),
                the_outputs_own_name: Some(replaced("Image")),
            }
        );
        let (_, names) = composed_and_named(printed);
        assert_eq!(
            names,
            ["_preview/standin - aImage0001.png", "_preview/standin - bImage0001.png", "_preview0001.png"]
        );
    }

    /// **The plan's three cases** (I2's *Proof*): `{fps}` in a base file name
    /// and `{blend_name}` in an output's own name are each unresolved, with
    /// the expression named, and that output owes no entry while the frame
    /// still owes the others; `{{x}}` is read as `{x}`.
    #[test]
    fn fps_in_a_base_file_name_and_any_brace_in_an_outputs_name_are_unresolved_and_a_doubled_brace_is_one() {
        let mut document = the_stand_ins_document();
        set_a_nodes(&mut document, 1, "the_nodes_base_file_name", "{fps} - a");
        let printed = taken_in(&document);
        assert_eq!(the_replaced_text_of(&printed, 1).the_base_file_name, Some(unresolved("{fps}")));
        assert_eq!(
            the_replaced_text_of(&printed, 1).the_unresolved_expression().map(|one| one.as_bytes().to_vec()),
            Some(b"{fps}".to_vec())
        );
        let (_, names) = composed_and_named(printed);
        assert_eq!(names, ["_preview/standin - bImage0001.png", "_preview0001.png"], "a owes nothing");

        let mut document = the_stand_ins_document();
        set_a_nodes(&mut document, 2, "output", "{blend_name}Image");
        let printed = taken_in(&document);
        assert_eq!(
            the_replaced_text_of(&printed, 2).the_outputs_own_name,
            Some(unresolved("{blend_name}")),
            "an output's own name is not replaced, even where its expression is"
        );
        let (_, names) = composed_and_named(printed);
        assert_eq!(names, ["_preview/standin - aImage0001.png", "_preview0001.png"], "b owes nothing");

        let mut document = the_stand_ins_document();
        set_a_nodes(&mut document, 1, "the_nodes_base_file_name", "{{x}} - a");
        let printed = taken_in(&document);
        assert_eq!(the_replaced_text_of(&printed, 1).the_base_file_name, Some(replaced("{x} - a")));
        let (_, names) = composed_and_named(printed);
        assert_eq!(
            names,
            ["_preview/standin - bImage0001.png", "_preview/{x} - aImage0001.png", "_preview0001.png"]
        );
    }

    /// **A node's folder is read the same way**: `{blend_name}` in it is put
    /// in before the redirect maps it, so the folder Blender is handed holds
    /// no expression; a doubled brace is mapped as one and handed to Blender
    /// doubled, so it lands as one; a folder holding another expression is
    /// handed on as it is and its node owes nothing.
    #[test]
    fn a_folder_holding_blend_name_is_mapped_and_handed_on_without_braces() {
        let the_values_handed_to_blender = |composed: &crate::composing::TheReadingAndWhatFollows| -> Vec<String> {
            crate::redirect::compose_the_launch_material(
                &composed.map,
                TheVersionInHand::of(5, 3),
                "a job",
                CarriedAndNotInterpretedHere(serde_json::json!({})),
            )
            .assignments
            .iter()
            .filter(|assignment| assignment.property.is_some())
            .map(|assignment| String::from_utf8_lossy(&assignment.folder).into_owned())
            .collect()
        };
        let mut document = the_stand_ins_document();
        for at in [1, 2] {
            set_a_nodes(&mut document, at, "recorded_location", "//../Render/{blend_name}/_preview");
        }
        let printed = taken_in(&document);
        assert_eq!(the_replaced_text_of(&printed, 2).the_folder, replaced("//../Render/standin/_preview"));
        let (composed, names) = composed_and_named(printed);
        assert_eq!(
            names,
            [
                "_preview0001.png",
                "standin/_preview/standin - aImage0001.png",
                "standin/_preview/standin - bImage0001.png",
            ]
        );
        assert_eq!(
            the_values_handed_to_blender(&composed),
            [format!("{THE_STAND_INS_JOB}/standin/_preview/"), format!("{THE_STAND_INS_JOB}/standin/_preview/")]
        );

        let mut document = the_stand_ins_document();
        for at in [1, 2] {
            set_a_nodes(&mut document, at, "recorded_location", "//../Render/{{x}}");
        }
        let (composed, names) = composed_and_named(taken_in(&document));
        assert_eq!(names, ["_preview0001.png", "{x}/standin - aImage0001.png", "{x}/standin - bImage0001.png"]);
        assert_eq!(
            the_values_handed_to_blender(&composed),
            [format!("{THE_STAND_INS_JOB}/{{{{x}}}}/"), format!("{THE_STAND_INS_JOB}/{{{{x}}}}/")],
            "the one brace is handed to Blender doubled"
        );

        let mut document = the_stand_ins_document();
        set_a_nodes(&mut document, 1, "recorded_location", "//../Render/{scene_name}");
        let printed = taken_in(&document);
        assert_eq!(the_replaced_text_of(&printed, 1).the_folder, unresolved("{scene_name}"));
        let (composed, names) = composed_and_named(printed);
        assert_eq!(names, ["_preview/standin - bImage0001.png", "_preview0001.png"]);
        assert!(
            the_values_handed_to_blender(&composed).contains(&format!("{THE_STAND_INS_JOB}/{{scene_name}}/")),
            "an unresolved folder is handed on as it is"
        );
    }

    /// **B5 narrowed (2026-10-18), on the real stand-in**: its tree holds no
    /// Group Output, and the live File Output nodes in it make the compositor
    /// run, so the walk's node destination is the main output. **Only a node
    /// in the job's own scene counts**: the same nodes read as a scene's
    /// reached through a strip leave the job's compositor not running, and the
    /// regular output path the main output, as before.
    #[test]
    fn a_live_file_output_node_in_the_jobs_own_scene_makes_its_compositor_run() {
        let (composed, _) = composed_and_named(taken_in(&the_stand_ins_document()));
        let outputs = &composed.reading.outputs;
        assert!(!outputs.gating.a_live_final_output_node, "the stand-in has no Group Output");
        assert!(outputs.a_live_file_output_node_is_in_the_jobs_scene());
        assert!(outputs.the_compositor_runs());
        assert_eq!(
            crate::composing::the_main_output_of(outputs),
            crate::main_output::WhatTheMainOutputIs::ADestinationInsideAFileOutputNode
        );

        let mut document = the_stand_ins_document();
        for at in [1, 2] {
            document["outputs"]["destinations"][at]["scene"] = serde_json::json!({
                "ReachedThroughASequencerStrip": {"scene": b"Other".to_vec(), "strip": b"Other".to_vec()}
            });
        }
        let printed = taken_in(&document);
        assert!(!printed.outputs.destinations.is_empty());
        let observed_scene_nodes = printed
            .outputs
            .destinations
            .iter()
            .filter(|one| one.scene != crate::reading::WhichScene::TheJobsOwnScene)
            .count();
        assert_eq!(observed_scene_nodes, 2);
        let composed = crate::composing::the_reading_and_everything_derived_from(
            printed,
            crate::composing::WhatTheReadingIsComposedAgainst {
                the_folder_the_blend_sits_in: b"/home/ubuntu/rendering",
                job_output_folder: THE_STAND_INS_JOB,
                enters_at: &WhereTheRenderEnters {
                    scene: ArtistsText::of(b"Scene".to_vec()),
                    view_layer: ArtistsText::of(b"a".to_vec()),
                },
                the_jobs_pins: &[],
            },
        )
        .expect("a reading");
        let outputs = &composed.reading.outputs;
        assert!(!outputs.a_live_file_output_node_is_in_the_jobs_scene());
        assert!(!outputs.the_compositor_runs());
        assert_eq!(
            crate::composing::the_main_output_of(outputs),
            crate::main_output::WhatTheMainOutputIs::TheRegularOutputPath
        );
    }

    /// **`Job files and outputs` R4 item 3 as corrected 2026-11-02: a strip's
    /// scene gives its File Output nodes and no regular output path.** The
    /// script says so where the destination is written, with the job's own
    /// scene named, and it still walks a strip's scene for its nodes. Run for
    /// real in `a_scene_with_strips_writes_where_the_job_says`.
    #[test]
    fn a_strip_scene_gives_no_regular_output_path() {
        let script = the_script(5, 3);
        let at = script.find("def destinations_of(scene, which_scene):").expect("the walk");
        let its_body = &script[at..at + 900];
        let the_test = its_body.find("if which_scene == \"TheJobsOwnScene\":").expect("only the job's own scene");
        let the_regular = its_body.find("\"kind\": \"TheRegularOutputPath\"").expect("the regular output path");
        assert!(the_test < the_regular, "{its_body}");
        assert!(script.contains("\"ReachedThroughASequencerStrip\": {"), "the nodes are still walked");
    }

    /// **The regular output path's folder is read the same way** (I2, added
    /// 2026-10-18, piece P2): `{blend_name}` in it is put in before the
    /// redirect maps it and the front of the name is left as recorded; a
    /// doubled brace is mapped as one and handed to Blender doubled; a folder
    /// holding another expression leaves the regular output path's entries
    /// unresolved and is handed on as it is. A brace in the front of the name
    /// is not the folder's, and is handed on as recorded for Blender to
    /// replace in the name it was asked for.
    #[test]
    fn the_regular_output_paths_folder_is_read_the_same_way() {
        let set_the_regular_output_path = |document: &mut serde_json::Value, to: &str, answered_name: &str| {
            let outputs = &mut document["outputs"];
            outputs["destinations"][0]["recorded_location"] = serde_json::json!(to.as_bytes());
            outputs["per_frame"][0]["the_regular_output_paths_answered_path"] =
                serde_json::json!(format!("/somewhere/{answered_name}").as_bytes());
        };
        let the_value_handed_to_blender = |composed: &crate::composing::TheReadingAndWhatFollows| -> String {
            let material = crate::redirect::compose_the_launch_material(
                &composed.map,
                TheVersionInHand::of(5, 3),
                "a job",
                CarriedAndNotInterpretedHere(serde_json::json!({})),
            );
            String::from_utf8_lossy(&material.the_regular_output_path.expect("a regular output path").value).into_owned()
        };
        let the_regular_output_paths = |printed: &ThePrintedReading| {
            printed.outputs.destinations[0].the_location_with_its_folder_replaced.clone()
        };

        let mut document = the_stand_ins_document();
        set_the_regular_output_path(&mut document, "//../Render/{blend_name}/{blend_name}_", "standin_0001.png");
        let printed = taken_in(&document);
        assert_eq!(
            the_regular_output_paths(&printed),
            Some(replaced("//../Render/standin/{blend_name}_")),
            "the folder replaced, the front of the name as recorded"
        );
        let (composed, names) = composed_and_named(printed);
        assert_eq!(
            names,
            [
                "_preview/standin - aImage0001.png",
                "_preview/standin - bImage0001.png",
                "standin/standin_0001.png",
            ]
        );
        assert_eq!(the_value_handed_to_blender(&composed), format!("{THE_STAND_INS_JOB}/standin/{{blend_name}}_"));

        let mut document = the_stand_ins_document();
        set_the_regular_output_path(&mut document, "//../Render/{{x}}/x_", "x_0001.png");
        let (composed, names) = composed_and_named(taken_in(&document));
        assert_eq!(names, ["_preview/standin - aImage0001.png", "_preview/standin - bImage0001.png", "{x}/x_0001.png"]);
        assert_eq!(
            the_value_handed_to_blender(&composed),
            format!("{THE_STAND_INS_JOB}/{{{{x}}}}/x_"),
            "the one brace is handed to Blender doubled"
        );

        let mut document = the_stand_ins_document();
        set_the_regular_output_path(&mut document, "//../Render/{scene_name}/x_", "x_0001.png");
        let printed = taken_in(&document);
        assert_eq!(the_regular_output_paths(&printed), Some(unresolved("{scene_name}")));
        let (composed, names) = composed_and_named(printed);
        assert_eq!(
            names,
            ["_preview/standin - aImage0001.png", "_preview/standin - bImage0001.png"],
            "the regular output path owes no entry, and the frame still owes the nodes'"
        );
        assert_eq!(
            the_value_handed_to_blender(&composed),
            format!("{THE_STAND_INS_JOB}/{{scene_name}}/x_"),
            "an unresolved folder is handed on as it is"
        );

        // **A video at an unresolved folder owes no entry either.**
        let mut document = the_stand_ins_document();
        set_the_regular_output_path(&mut document, "//../Render/{scene_name}/x_", "x_0001.png");
        document["outputs"]["destinations"][0]["format"]["is_video_shaped"] = serde_json::json!(true);
        let (_, names) = composed_and_named(taken_in(&document));
        assert_eq!(names, ["_preview/standin - aImage0001.png", "_preview/standin - bImage0001.png"]);

        // **Left as the only file the frame owes, the unresolved regular
        // output path withholds the inventory** with the reason named.
        let mut document = the_stand_ins_document();
        set_the_regular_output_path(&mut document, "//../Render/{scene_name}/x_", "x_0001.png");
        document["outputs"]["destinations"].as_array_mut().unwrap().drain(1..3);
        let composed = crate::composing::the_reading_and_everything_derived_from(
            taken_in(&document),
            crate::composing::WhatTheReadingIsComposedAgainst {
                the_folder_the_blend_sits_in: b"/home/ubuntu/rendering",
                job_output_folder: THE_STAND_INS_JOB,
                enters_at: &WhereTheRenderEnters {
                    scene: ArtistsText::of(b"Scene".to_vec()),
                    view_layer: ArtistsText::of(b"a".to_vec()),
                },
                the_jobs_pins: &[],
            },
        )
        .expect("a reading");
        assert_eq!(
            composed.inventory,
            crate::inventory::WhatTheInventoryIs::TheFrameSetIsNotPredictable(
                crate::inventory::WhyTheFrameSetIsNotPredictable::AFramesFilesAreNamedByAnExpressionThatIsNotReplaced
            )
        );

        // **A reading with no open file's path is taken raw**, as before.
        let mut document = the_stand_ins_document();
        set_the_regular_output_path(&mut document, "//../Render/{blend_name}/x_", "x_0001.png");
        document.as_object_mut().unwrap().remove("the_open_files_path");
        let printed = taken_in(&document);
        assert_eq!(the_regular_output_paths(&printed), None);
        let (composed, names) = composed_and_named(printed);
        assert!(names.contains(&"{blend_name}/x_0001.png".to_string()), "{names:?}");
        assert_eq!(the_value_handed_to_blender(&composed), format!("{THE_STAND_INS_JOB}/{{blend_name}}/x_"));
    }

    /// **A reading printed before 2026-10-18 carries no open file's path, and
    /// is taken as its raw text**: nothing is replaced and the names are built
    /// as before. **A recorded reading with no replaced text parses** and is
    /// taken as raw too.
    #[test]
    fn an_older_reading_is_taken_as_its_raw_text() {
        let mut document = the_stand_ins_document();
        document.as_object_mut().unwrap().remove("the_open_files_path");
        let printed = taken_in(&document);
        assert_eq!(printed.the_open_files_path, None);
        assert!(printed.outputs.destinations.iter().all(|one| one.the_replaced_text().is_none()));
        let (composed, names) = composed_and_named(printed);
        assert_eq!(
            names,
            ["_preview/{blend_name} - aImage0001.png", "_preview/{blend_name} - bImage0001.png", "_preview0001.png"]
        );

        let (composed_now, _) = composed_and_named(taken_in(&the_stand_ins_document()));
        let mut recorded = serde_json::to_value(&composed_now).unwrap();
        for destination in recorded["reading"]["outputs"]["destinations"].as_array_mut().unwrap() {
            if let Some(kind) = destination["kind"].get_mut("AnOutputOfAFileOutputNode") {
                kind.as_object_mut().unwrap().remove("replaced");
            }
            // **The regular output path's replaced folder, added the same day
            // (piece P2)**, is absent from a reading recorded before it too.
            destination.as_object_mut().unwrap().remove("the_location_with_its_folder_replaced");
        }
        let read_back: crate::composing::TheReadingAndWhatFollows =
            serde_json::from_value(recorded).expect("a recorded reading with no replaced text parses");
        assert!(read_back.reading.outputs.destinations.iter().all(|one| one.the_replaced_text().is_none()));
        assert_eq!(composed.reading.outputs.destinations, read_back.reading.outputs.destinations);
    }

    /// **B25: the reading carries the job's scene's own view transform into
    /// the outputs half**, which is what the manifest carries; a reading
    /// printed before 2026-10-03 carries none and reads as not read. The 38's
    /// piece 3.
    #[test]
    fn the_reading_carries_the_jobs_own_view_transform_and_an_older_one_none() {
        let text = the_script(5, 2);
        // One composition for the job's scene and, since R4b, every other.
        assert!(text.contains(r#""the_view_transform": as_bytes(the_scene.view_settings.view_transform),"#));
        assert!(text.contains("the_jobs_scene, reached, per_frame, destinations, views, shared_with, recorded_locations"));
        let mut value = serde_json::to_value(a_printed_reading()).unwrap();
        let outputs = value["outputs"].as_object_mut().unwrap();
        outputs.remove("the_view_transform");
        let older = serde_json::from_value::<ThePrintedReading>(value.clone()).unwrap();
        assert_eq!(older.outputs.the_view_transform, None);
        value["outputs"]
            .as_object_mut()
            .unwrap()
            .insert("the_view_transform".to_string(), serde_json::json!(b"AgX".to_vec()));
        let read = serde_json::from_value::<ThePrintedReading>(value).unwrap();
        let enters = WhereTheRenderEnters {
            scene: ArtistsText::of(b"Kitchen".to_vec()),
            view_layer: ArtistsText::of(b"ViewLayer".to_vec()),
        };
        let document = the_one_document_from(read, &enters).expect("a document");
        assert_eq!(
            document.outputs.the_view_transform.map(|name| name.as_bytes().to_vec()),
            Some(b"AgX".to_vec()),
            "carried whole into the half the manifest carries"
        );
    }

    #[test]
    fn the_reading_says_whether_the_jobs_scene_holds_a_simulation_not_baked() {
        let text = the_script(5, 2);
        let item = &text[text.find("def the_scene_holds_a_simulation_not_baked(scene):").unwrap()
            ..text.find("def the_replay_fluid_domains():").unwrap()];
        for read in [
            "world = scene.rigidbody_world",
            "for obj in scene.objects:",
            r#"if modifier.type == "NODES":"#,
            "for bake in modifier.bakes:",
            r#"if node is not None and node.bl_idname == "GeometryNodeSimulationOutput":"#,
            r#"if modifier.fluid_type == "DOMAIN" and modifier.domain_settings.cache_type == "REPLAY":"#,
            r#"if hasattr(modifier, "point_cache"):"#,
            "caches.extend(surface.point_cache for surface in canvas.canvas_surfaces)",
            r#"if system.settings.type == "HAIR" and not system.use_hair_dynamics:"#,
            r#"if not smoozy_as_the_file_has_it(cache, "is_baked"):"#,
        ] {
            assert!(item.contains(read), "{read}");
        }
        assert_eq!(item.matches(r#"smoozy_as_the_file_has_it(cache, "is_baked")"#).count(), 3);
        assert!(!item.contains("bpy.data.objects"), "the job's own scene, not the whole file");
        assert!(text.contains(
            r#""the_scene_holds_a_simulation_not_baked": the_scene_holds_a_simulation_not_baked(the_jobs_scene),"#
        ));
        // A printed reading without the item reads as no, and so does a
        // recorded one.
        let printed = |with: Option<bool>| {
            let mut value = serde_json::to_value(a_printed_reading()).unwrap();
            let object = value.as_object_mut().unwrap();
            object.remove("the_scene_holds_a_simulation_not_baked");
            if let Some(with) = with {
                object.insert("the_scene_holds_a_simulation_not_baked".to_string(), with.into());
            }
            serde_json::from_value::<ThePrintedReading>(value).unwrap()
        };
        assert!(!printed(None).the_scene_holds_a_simulation_not_baked);
        let yes = printed(Some(true));
        assert!(yes.the_scene_holds_a_simulation_not_baked);
        let document = the_one_document_from(yes, &WhereTheRenderEnters {
                scene: ArtistsText::of(b"Kitchen".to_vec()),
                view_layer: ArtistsText::of(b"ViewLayer".to_vec()),
            }).unwrap();
        assert!(document.the_scene_holds_a_simulation_not_baked);
        let mut recorded = serde_json::to_value(&document).unwrap();
        recorded.as_object_mut().unwrap().remove("the_scene_holds_a_simulation_not_baked");
        assert!(
            !serde_json::from_value::<crate::reading::TheOneDocument>(recorded)
                .unwrap()
                .the_scene_holds_a_simulation_not_baked
        );
    }

    /// **D20's fifth place, added 2026-10-02: the reading names it, and reads
    /// every Replay fluid domain's cache folder and no other domain's**, by
    /// the names read off the four Blenders (`crate::redirect::THE_FLUID_CACHE_LOOKUP`)
    /// — so the map gives each recorded Replay folder its own job folder, which
    /// `Rendering`'s load-time handler then sets as the file loads (D1a).
    #[test]
    fn the_reading_reads_every_replay_fluid_cache_folder_as_the_fifth_place() {
        let text = the_script(5, 2);
        assert!(text.contains(r#"    "TheReplayFluidCacheFolder",
)"#));
        let walk = &text[text.find("def the_replay_fluid_domains():").unwrap()
            ..text.find("def the_point_caches_the_redirect_sets():").unwrap()];
        assert!(walk.contains(r#"if modifier.type != "FLUID" or modifier.fluid_type != "DOMAIN":"#));
        assert!(walk.contains(r#"if modifier.domain_settings.cache_type == "REPLAY":"#));
        assert!(text.contains(r#"smoozy_as_the_file_has_it(domain, "cache_directory")"#));
        for place in crate::reading::EVERY_ADDED_WRITE_PLACE {
            let named = serde_json::to_string(&place).unwrap();
            assert!(text.contains(&format!("    {named},\n")), "{named} is not read");
        }
    }

    /// **R18: every read of a setting the load-time handler changes goes
    /// through the record it keeps, so the reading reports the file's own
    /// values and never the throwaway folder.** The helper reads the record
    /// under [`crate::redirect::THE_LOAD_TIME_RECORD`], and every cache
    /// setting the reading reports is read through it — the generic walk over
    /// Blender's own path marks included.
    #[test]
    fn every_setting_the_handler_changes_is_read_as_the_file_had_it() {
        let text = the_script(5, 2);
        assert!(text.contains(THE_READ_AS_THE_FILE_HAS_IT));
        assert!(THE_READ_AS_THE_FILE_HAS_IT.contains(&format!(
            "sys.modules.get(\"{}\")",
            crate::redirect::THE_LOAD_TIME_RECORD
        )));
        for read in [
            r#"smoozy_as_the_file_has_it(cache, "is_baked")"#,
            r#"smoozy_as_the_file_has_it(cache, "use_external")"#,
            r#"smoozy_as_the_file_has_it(cache, "filepath")"#,
            r#"smoozy_as_the_file_has_it(domain, "cache_directory")"#,
            "value = smoozy_as_the_file_has_it(one, prop.identifier)",
        ] {
            assert!(text.contains(read), "{read}");
        }
        for raw in ["cache.filepath", "cache.is_baked", "cache.use_external", "domain.cache_directory"] {
            assert!(!text.contains(raw), "{raw} is read as it now stands");
        }
    }

    /// **R4a: a location crosses as the bytes the file holds**, and the script
    /// has one route for that and no other.
    #[test]
    fn every_location_leaves_the_script_as_bytes_and_none_is_decoded() {
        let text = the_script(4, 5);
        assert!(text.contains("os.fsencode"));
        // The one shape a location is written in.
        assert!(text.contains("def as_bytes(value):"));
    }

    /// **`Dependency Upload & Remap` G3's rule, held over this script too: no
    /// branch turns a missing answer into an empty one.**
    ///
    /// The check reading's script has had this test since a round found three
    /// catches that each ended a reading quietly — two substituting an empty
    /// answer and one dropping a reference with `continue`. **This script is
    /// allowed catches only where what it cannot read is a state rather than a
    /// silence**, and the count is asserted exactly rather than as a ceiling,
    /// because a catch added quietly is the whole fault this test is about.
    #[test]
    fn the_script_carries_exactly_one_catch_and_it_names_a_state_rather_than_a_silence() {
        let text = the_script(5, 2);
        let catches = text
            .lines()
            .filter(|line| line.trim_start().starts_with("except"))
            .count();
        assert_eq!(
            catches, 5,
            "a catch was added or removed without this count moving with it"
        );
        // The first catch writes into the error map, which `Render Settings`
        // Q7 makes *no reading* — loud, and never an empty answer. **The other
        // three are R4b's** (2026-10-10): each writes into the error map of a
        // scene the reading does not name, which makes that scene's half no
        // reading of it and never fails the reading of the scene it names.
        // **The fifth is `Render Settings` Q2 item 7's**: no list is a state the
        // page reads as *cannot say*, and it is no part of the file's reading.
        assert!(text.contains("except OSError:\n        return None\n    return sorted(name for name in names"));
        assert!(text.contains("errors[\"the regular output path at frame \""));
        assert!(text.contains("per_frame.append(the_entry_at(scene, frame, ERRORS, True))"));
        assert_eq!(text.matches("its_errors[\"the reading of this scene").count(), 2);
        assert_eq!(text.matches("[\"the reading of this scene's formats\"] = str(why)").count(), 1);
        // And nothing anywhere drops what it was reading and carries on.
        for (at, code) in the_except_bodies(&text) {
            assert!(
                code != "continue" && code != "pass",
                "line {at}: the catch drops what it was reading and says nothing"
            );
            for empty in ["{}", "[]", "()", "set()", "dict()", "list()", "tuple()"] {
                assert!(
                    !code.ends_with(&format!("= {empty}")),
                    "line {at}: the catch substitutes an empty answer — `{code}`"
                );
            }
        }
        assert!(!text.contains("contextlib.suppress"));
        // The other two `try`s in the script have no catch at all. One puts
        // back the scene the reading made to ask a format for its ending, so
        // that scene is gone whatever happens; the other (the live Telegram
        // pass's fault 4, 2026-10-31) puts back the job's scene's start and
        // end after a video's name is asked over the job's range, and what it
        // raises reaches the first catch above.
        let tries = text
            .lines()
            .filter(|line| line.trim_start().starts_with("try:"))
            .count();
        assert_eq!(tries, 7);
        assert!(text.contains("bpy.data.scenes.remove(throwaway)"));
        assert!(text.contains("    finally:\n        the_range_is_set_on(scene, saved[0], saved[1])"));
    }

    /// Every line of a catch's body, with the line the catch sits on.
    /// Indentation is what closes a block, which is Python's own rule and is
    /// enough here because the script is this file's own constant.
    fn the_except_bodies(script: &str) -> Vec<(usize, String)> {
        let lines: Vec<&str> = script.lines().collect();
        let mut out = Vec::new();
        for (at, line) in lines.iter().enumerate() {
            if !line.trim_start().starts_with("except") {
                continue;
            }
            let indent = line.len() - line.trim_start().len();
            for below in lines.iter().skip(at + 1) {
                if below.trim().is_empty() {
                    continue;
                }
                if below.len() - below.trim_start().len() <= indent {
                    break;
                }
                out.push((at + 1, below.trim().to_string()));
            }
        }
        out
    }

    /// **R4 item 2's filtering lives in Rust and in one place.**
    ///
    /// The script records every node's outputs and the gating beside them; a
    /// node whose scene will write none contributes no destination, and the
    /// rule that decides it is `GatingConditions`' own.
    #[test]
    fn a_node_in_a_scene_that_writes_nothing_contributes_no_destination() {
        use crate::reading::tests::{a_destination, plain_gating};
        use crate::reading::{DestinationKind, WhereItComesFrom};

        let mut gating = plain_gating();
        gating.the_render_goes_out_through_a_sequencer_holding_a_scene_strip = true;
        assert!(!gating.a_file_output_node_will_write());

        let observed = TheObservedOutputs {
            taken_by: TheVersionInHand::of(5, 2),
            gating,
            destinations: vec![
                a_destination(
                    DestinationKind::TheRegularOutputPath,
                    "/home/a/out/f_",
                    WhereItComesFrom::TheScenesOutputPath,
                ),
                a_destination(
                    DestinationKind::AnOutputOfAFileOutputNode {
                        the_effect: None,
                        node: ArtistsText::of(b"Beauty".to_vec()),
                        output: ArtistsText::of(b"Image".to_vec()),
                        the_nodes_base_file_name: None,
                        replaced: None,
                    },
                    "/home/a/nodes/",
                    WhereItComesFrom::AFileOutputNodeNamed(ArtistsText::of(b"Beauty".to_vec())),
                ),
            ],
            scenes_reached_through_a_sequencer_strip: Vec::new(),
            the_tree_is_shared_with: None,
            per_frame: Vec::new(),
            recorded_locations: Vec::new(),
            views: HowManyViewsTheSceneWrites::One,
            the_compositing_tree: None,
            the_view_transform: None,
        };
        let kept = the_destinations_that_will_be_written(&observed);
        assert_eq!(kept.len(), 1);
        assert_eq!(kept[0].kind, DestinationKind::TheRegularOutputPath);
    }

    /// **`Render Settings` Q7: a document whose error map is not empty is no
    /// reading**, and nothing of it is carried forward.
    #[test]
    fn a_printed_reading_with_a_non_empty_error_map_is_no_reading() {
        let mut printed = a_printed_reading();
        printed.errors.insert(
            "the regular output path at frame 1".to_string(),
            "Unknown variable referenced in template expression".to_string(),
        );
        let enters_at = WhereTheRenderEnters {
            scene: ArtistsText::of(b"Kitchen".to_vec()),
            view_layer: ArtistsText::of(b"ViewLayer".to_vec()),
        };
        assert!(matches!(
            the_one_document_from(printed, &enters_at),
            Err(WhyItIsNoReading::TheErrorMapIsNotEmpty(_))
        ));
    }

    /// **The beauty walk is derived here, by the one walk this project has.**
    ///
    /// A tree with no entry for the job's scene and view layer comes back as
    /// B4's declared *no entry* rather than as an empty answer, and a scene
    /// with no tree at all comes back the same way, which is B5's case.
    #[test]
    fn the_beauty_walk_is_derived_from_the_printed_tree_and_never_read_from_it() {
        let enters_at = WhereTheRenderEnters {
            scene: ArtistsText::of(b"Kitchen".to_vec()),
            view_layer: ArtistsText::of(b"ViewLayer".to_vec()),
        };
        let document = the_one_document_from(a_printed_reading(), &enters_at)
            .expect("an empty error map is a reading");
        assert_eq!(document.outputs.the_beauty_walk, WhatTheWalkFound::NoEntry);
    }

    /// **The round trip: what the script prints is read back by this crate.**
    #[test]
    fn what_the_script_prints_is_taken_out_from_between_the_two_marks() {
        let printed = a_printed_reading();
        let mut stream = b"Blender 5.2.2 LTS\nRead prefs\n".to_vec();
        stream.extend_from_slice(THE_OPENING_SENTINEL);
        stream.extend_from_slice(&serde_json::to_vec(&printed).unwrap());
        stream.extend_from_slice(THE_CLOSING_SENTINEL);
        stream.extend_from_slice(b"\nBlender quit\n");
        assert_eq!(the_printed_reading_in(&stream).unwrap(), printed);

        // And a stream cut short before the closing mark is no document rather
        // than an empty one.
        let mut short = b"chatter".to_vec();
        short.extend_from_slice(THE_OPENING_SENTINEL);
        short.extend_from_slice(br#"{"outputs":"#);
        assert_eq!(
            the_printed_reading_in(&short),
            Err(WhyTheDocumentWasNotRead::TheSentinelsWereNotBothThere)
        );
    }

    /// **The third payload stays and stays empty.** Nothing fills it and
    /// nothing reads it, and the script writes an empty object into it.
    #[test]
    fn the_third_payload_is_empty_and_the_script_writes_it_so() {
        assert!(the_script(4, 5).contains("\"dependencies\": {}"));
        let printed = a_printed_reading();
        assert_eq!(printed.dependencies.0, serde_json::json!({}));
    }

    /// **The per-frame table widens where no frames were handed in, and
    /// narrows to exactly the handed frames where any were** — the build
    /// runner's ruling of 2026-09-20, which is what stops the artist's file
    /// being read twice on a job whose range is *the file's own*.
    ///
    /// **The gate is one line and it is read here rather than described**, in
    /// every version the script is composed for.
    #[test]
    fn the_per_frame_table_covers_the_span_only_where_no_frames_were_handed_in() {
        for (major, minor) in [(4u16, 5u16), (5, 0), (5, 1), (5, 2)] {
            let script = the_script(major, minor);
            assert!(
                script.contains(
                    "if scene == the_jobs_scene and (not the_jobs_frames or frame in the_jobs_frames):"
                ),
                "{major}.{minor}"
            );
            // **The span is the script's own and is not recomputed for this**
            // — the scene's own range by its own step, unioned with the
            // switch's key frames and their neighbours.
            assert!(
                script.contains("for frame in the_span_of(scene, extra):"),
                "{major}.{minor}"
            );
            // **The narrow gate is gone and nothing else narrows the table**,
            // so a job that hands frames in receives what it always did and a
            // job that hands none receives the span.
            assert!(
                !script.contains("and frame in the_jobs_frames:"),
                "{major}.{minor}: the narrow gate is still in the script"
            );
        }
    }

    /// **A scene is moved to its frames only where something on it can move**
    /// — the Creator's notes on 0.1.8, piece 2. The condition is read here in
    /// every version rather than described: animation data on the scene, on
    /// any tree its compositing reaches, or a marker that names a camera.
    /// **There is one step in the script and it is behind the condition**, so
    /// nothing moves a still scene by another road. The four Blenders' own
    /// proof is `tests/stepping_at_the_desk.rs`, and the sweep is the account
    /// [`THE_STEPPING_LOOKUP`] names.
    #[test]
    fn a_scene_is_moved_to_its_frames_only_where_something_on_it_moves() {
        assert!(THE_STEPPING_LOOKUP.contains("piece 2 — what landed.md"));
        for (major, minor) in [(4u16, 5u16), (5, 0), (5, 1), (5, 2)] {
            let script = the_script(major, minor);
            let condition = script
                .find("def something_on_it_moves(scene):\n")
                .unwrap_or_else(|| panic!("{major}.{minor}: the condition"));
            let body = &script[condition..script[condition..].find("\n\n\n").unwrap() + condition];
            for part in [
                "    if scene.animation_data is not None:\n        return True\n",
                "    for tree, _stack in every_tree_of(scene):\n        if tree.animation_data is not None:\n            return True\n",
                "    for marker in scene.timeline_markers:\n        if marker.camera is not None:\n            return True\n",
                "    return False",
            ] {
                assert!(body.contains(part), "{major}.{minor}: {part}");
            }
            assert!(
                script.contains(
                    "    moves = something_on_it_moves(scene)\n    for frame in the_span_of(scene, extra):\n        if moves:\n            scene.frame_set(frame)\n"
                ),
                "{major}.{minor}: the step is behind the condition"
            );
            assert_eq!(
                script.matches("frame_set(").count(),
                1,
                "{major}.{minor}: one step in the whole script"
            );
        }
    }

    /// **Every name the script is composed from is cited, and to one of the
    /// three sources this file names.**
    #[test]
    fn every_name_is_cited_and_to_one_of_the_three_sources() {
        let every = [
            THE_SCENES_COMPOSITING_TREE,
            THE_FINAL_OUTPUT_NODES_TYPE,
            THE_GROUPS_OWN_INPUT_TYPE,
            THE_GROUPS_OWN_OUTPUT_TYPE,
            THE_RENDER_LAYERS_NODES_TYPE,
            THE_FILE_OUTPUT_NODES_TYPE,
            THE_FOUR_FIVE_USE_NODES_SWITCH,
            THE_NODES_FOLDER,
            THE_NODES_BASE_FILE_NAME,
            THE_NODES_OUTPUT_LIST,
            AN_OUTPUTS_OWN_PIECE_OF_PATH,
            THE_PER_OUTPUT_FORMAT_SWITCH,
            THE_GATHERING_MARKS_PROPERTY,
            THE_GATHERING_MARKS_VALUE,
            THE_MEDIA_KIND,
            A_PICTURE_SOCKETS_KIND,
            THE_KIND_NO_OUTPUT_CAN_BE,
            THE_CYCLES_IDENTIFIER,
        ];
        let mut how_many = 0;
        for across in every {
            for side in [across.four_five, across.from_five_zero] {
                let Some(spelling) = side else { continue };
                let Spelling::Cited { name, source } = spelling else {
                    panic!("{spelling:?} is not on record");
                };
                assert!(!name.is_empty());
                assert!(
                    source == THE_LOOKUP || source == THIS_PIECES_LOOKUP,
                    "{name} is cited to {source}"
                );
                how_many += 1;
            }
        }
        for one in [THE_SAVE_OUTPUT_SWITCH, A_NODES_OWN_ENDING_SWITCH] {
            let Spelling::Cited { source, .. } = one else {
                panic!("{one:?} is not on record");
            };
            assert!(source == THE_LOOKUP || source == THIS_PIECES_LOOKUP);
            how_many += 1;
        }
        assert_eq!(how_many, 35, "a name was added or removed without this count");
    }

    /// **The version boundary is the one this crate already draws**, so 5.1
    /// sits on the from-5.0 side of every name here, as it does everywhere
    /// else in this document.
    #[test]
    fn five_one_takes_the_from_five_zero_names() {
        assert_eq!(
            TheVersionInHand::of(5, 1).branch(),
            WhichBranch::FromFiveZero
        );
        assert_eq!(
            THE_SCENES_COMPOSITING_TREE
                .for_version(TheVersionInHand::of(5, 1))
                .unwrap(),
            Some("compositing_node_group")
        );
        // And the two things that arrive at 5.2 are absent at 5.1.
        assert!(!TheVersionInHand::of(5, 1).has_the_save_output_switch());
        assert!(the_script(5, 1).contains("\"the_save_output_switch\":null"));
        assert!(the_script(5, 2).contains("\"the_save_output_switch\":\"save_output\""));
    }

    /// **An absence stops the composition and names what is missing**, which is
    /// the shape both spellings files use and the reason no script can be
    /// composed from a guess.
    #[test]
    fn an_absence_stops_the_composition_and_names_itself() {
        let absent = Spelling::NotOnRecord(MissingSpelling {
            what: "a name nobody read",
            the_clause: "R4",
        });
        assert_eq!(
            absent.name(),
            Err(MissingSpelling {
                what: "a name nobody read",
                the_clause: "R4"
            })
        );
        let across = AcrossTheBoundary {
            four_five: Some(absent),
            from_five_zero: None,
            from_five_three: TheThirdSide::TheSameAsFromFiveZero,
        };
        assert!(across.for_version(TheVersionInHand::of(4, 5)).is_err());
        // The other side is an absence of a different kind: this version has no
        // such thing, which the script reads as a state rather than a fault.
        assert_eq!(
            across.for_version(TheVersionInHand::of(5, 2)).unwrap(),
            None
        );
    }


    /// **What two running Blenders at the desk actually printed, held against
    /// the shapes this crate carries.**
    ///
    /// **This said four and held two until 2026-09-20**, which is the same
    /// overstatement the account of that sitting carried.
    ///
    /// These two are the whole document a real 4.5.12 and a real 5.2.2 wrote
    /// between the marks, for one file with a compositing tree, a Render
    /// Layers node, the tree's final output node and one file output node,
    /// over one frame. **Nothing here is composed by hand**: they were taken
    /// out of the two runs and pasted in, which is the only check this project
    /// has that the script's document and this crate's shapes are the same
    /// object. Every other test in this file reads what the composition
    /// produces rather than what Blender does.
    const WHAT_FOUR_FIVE_PRINTED: &str = r#"{"outputs": {"taken_by": {"major": 4, "minor": 5}, "gating": {"compositing_switch": true, "a_compositing_tree_is_present": true, "a_live_final_output_node": true, "the_final_output_node_is_muted": false, "four_fives_use_nodes_switch": true, "save_output_switch": null, "the_render_goes_out_through_a_sequencer_holding_a_scene_strip": false}, "destinations": [{"kind": "TheRegularOutputPath", "recorded_location": [47, 47, 111, 117, 116, 47, 102, 95], "format": {"as_the_file_records_it": [80, 78, 71], "the_ending_it_writes": [46, 112, 110, 103], "is_video_shaped": false}, "origin": "TheScenesOutputPath", "scene": "TheJobsOwnScene", "the_view_layers_that_feed_it": [[86, 105, 101, 119, 76, 97, 121, 101, 114]]}, {"kind": {"AnOutputOfAFileOutputNode": {"node": [66, 101, 97, 117, 116, 121], "output": [73, 109, 97, 103, 101], "the_nodes_base_file_name": null}}, "recorded_location": [47, 47, 110, 111, 100, 101, 115, 47, 98, 101, 97, 117, 116, 121, 47], "format": {"as_the_file_records_it": [80, 78, 71], "the_ending_it_writes": [46, 112, 110, 103], "is_video_shaped": false}, "origin": {"AFileOutputNodeNamed": [66, 101, 97, 117, 116, 121]}, "scene": "TheJobsOwnScene", "the_view_layers_that_feed_it": [[86, 105, 101, 119, 76, 97, 121, 101, 114]]}, {"kind": {"AnAddedWritePlace": "TheBakeOutputPath"}, "recorded_location": [47, 47], "format": null, "origin": {"AnAddedWritePlaceNamed": "TheBakeOutputPath"}, "scene": "TheJobsOwnScene", "the_view_layers_that_feed_it": []}, {"kind": {"AnAddedWritePlace": "ThePointCache"}, "recorded_location": null, "format": null, "origin": {"AnAddedWritePlaceNamed": "ThePointCache"}, "scene": "TheJobsOwnScene", "the_view_layers_that_feed_it": []}, {"kind": {"AnAddedWritePlace": "TheGeometryNodesBakeFolder"}, "recorded_location": null, "format": null, "origin": {"AnAddedWritePlaceNamed": "TheGeometryNodesBakeFolder"}, "scene": "TheJobsOwnScene", "the_view_layers_that_feed_it": []}, {"kind": {"AnAddedWritePlace": "TheSequencerProxyFolder"}, "recorded_location": null, "format": null, "origin": {"AnAddedWritePlaceNamed": "TheSequencerProxyFolder"}, "scene": "TheJobsOwnScene", "the_view_layers_that_feed_it": []}], "scenes_reached_through_a_sequencer_strip": [], "the_tree_is_shared_with": null, "per_frame": [{"frame": 1, "view_layers_that_render": [[86, 105, 101, 119, 76, 97, 121, 101, 114]], "gating": {"compositing_switch": true, "a_compositing_tree_is_present": true, "a_live_final_output_node": true, "the_final_output_node_is_muted": false, "four_fives_use_nodes_switch": true, "save_output_switch": null, "the_render_goes_out_through_a_sequencer_holding_a_scene_strip": false}, "the_regular_output_paths_answered_path": [47, 112, 114, 105, 118, 97, 116, 101, 47, 116, 109, 112, 47, 99, 108, 97, 117, 100, 101, 45, 53, 48, 49, 47, 45, 85, 115, 101, 114, 115, 45, 107, 105, 115, 108, 121, 97, 107, 111, 118, 45, 68, 101, 115, 107, 116, 111, 112, 45, 87, 111, 114, 107, 45, 83, 109, 111, 111, 122, 121, 45, 118, 50, 47, 48, 100, 54, 52, 102, 54, 55, 99, 45, 50, 100, 102, 57, 45, 52, 57, 53, 101, 45, 56, 56, 50, 52, 45, 57, 48, 53, 102, 51, 54, 97, 97, 50, 98, 53, 49, 47, 115, 99, 114, 97, 116, 99, 104, 112, 97, 100, 47, 112, 49, 51, 45, 50, 101, 48, 100, 100, 97, 54, 53, 99, 51, 54, 55, 98, 49, 51, 56, 47, 102, 105, 108, 101, 115, 47, 111, 117, 116, 47, 102, 95, 48, 48, 48, 49, 46, 112, 110, 103]}], "recorded_locations": [[47, 47, 111, 117, 116, 47, 102, 95], [47, 47], [47, 47, 110, 111, 100, 101, 115, 47, 98, 101, 97, 117, 116, 121, 47], [73, 109, 97, 103, 101]], "views": "One", "the_compositing_tree": {"nodes": [{"name": [67, 111, 109, 112, 111, 115, 105, 116, 101], "kind": "TheFinalOutputNode"}, {"name": [82, 101, 110, 100, 101, 114, 32, 76, 97, 121, 101, 114, 115], "kind": {"RenderLayers": {"scene": [75, 105, 116, 99, 104, 101, 110], "view_layer": [86, 105, 101, 119, 76, 97, 121, 101, 114], "outputs": [[73, 109, 97, 103, 101], [65, 108, 112, 104, 97], [68, 101, 112, 116, 104], [78, 111, 114, 109, 97, 108], [85, 86], [86, 101, 99, 116, 111, 114], [80, 111, 115, 105, 116, 105, 111, 110], [68, 101, 112, 114, 101, 99, 97, 116, 101, 100], [68, 101, 112, 114, 101, 99, 97, 116, 101, 100], [83, 104, 97, 100, 111, 119], [65, 79], [68, 101, 112, 114, 101, 99, 97, 116, 101, 100], [68, 101, 112, 114, 101, 99, 97, 116, 101, 100], [68, 101, 112, 114, 101, 99, 97, 116, 101, 100], [73, 110, 100, 101, 120, 79, 66], [73, 110, 100, 101, 120, 77, 65], [77, 105, 115, 116], [69, 109, 105, 116], [69, 110, 118], [68, 105, 102, 102, 68, 105, 114], [68, 105, 102, 102, 73, 110, 100], [68, 105, 102, 102, 67, 111, 108], [71, 108, 111, 115, 115, 68, 105, 114], [71, 108, 111, 115, 115, 73, 110, 100], [71, 108, 111, 115, 115, 67, 111, 108], [84, 114, 97, 110, 115, 68, 105, 114], [84, 114, 97, 110, 115, 73, 110, 100], [84, 114, 97, 110, 115, 67, 111, 108], [83, 117, 98, 115, 117, 114, 102, 97, 99, 101, 68, 105, 114], [83, 117, 98, 115, 117, 114, 102, 97, 99, 101, 73, 110, 100], [83, 117, 98, 115, 117, 114, 102, 97, 99, 101, 67, 111, 108]]}}}, {"name": [66, 101, 97, 117, 116, 121], "kind": {"AFileOutputNode": {"inputs": [[73, 109, 97, 103, 101]], "gathering_into_one_file": false}}}], "links": [{"from": {"node": 1, "index": 0}, "to": {"node": 2, "index": 0}}, {"from": {"node": 1, "index": 0}, "to": {"node": 0, "index": 0}}]}}, "settings": {"scenes": [{"name": "Kitchen", "frame_start": 1, "frame_end": 1, "frame_step": 1, "frame_count": 1, "full_size": [1920, 1080], "resolution_percentage": 100, "size_at_the_files_percentage": [1920, 1080], "engine_as_read": "BLENDER_EEVEE_NEXT", "sample_counts": {"cycles": 4096, "eevee": 64}, "device": "TheProcessor", "early_stop": {"adaptive_sampling_is_on": true, "threshold": 0.009999999776482582, "minimum": 0}, "time_limit": 0.0, "layer_samples_rule": "Use", "saved_with": "4.5.92", "view_layers": [{"name": "ViewLayer", "renders_at": [[1, true]], "animation": {"None": {"value": true}}, "own_sample_count": 0, "overrides_the_scenes_count": false}], "pulls_in": [], "simplify": {"use_simplify": false, "subdivision_render": 6, "child_particles_render": 1.0, "texture_limit_render": "OFF"}}], "offered_engine_identifiers": {"cycles": "CYCLES", "eevee": "BLENDER_EEVEE_NEXT"}, "announced_add_on_case": null}, "dependencies": {}, "errors": {}}"#;

    const WHAT_FIVE_TWO_PRINTED: &str = r#"{"outputs": {"taken_by": {"major": 5, "minor": 2}, "gating": {"compositing_switch": true, "a_compositing_tree_is_present": true, "a_live_final_output_node": true, "the_final_output_node_is_muted": false, "four_fives_use_nodes_switch": null, "save_output_switch": true, "the_render_goes_out_through_a_sequencer_holding_a_scene_strip": false}, "destinations": [{"kind": "TheRegularOutputPath", "recorded_location": [47, 47, 111, 117, 116, 47, 102, 95], "format": {"as_the_file_records_it": [80, 78, 71], "the_ending_it_writes": [46, 112, 110, 103], "is_video_shaped": false}, "origin": "TheScenesOutputPath", "scene": "TheJobsOwnScene", "the_view_layers_that_feed_it": [[86, 105, 101, 119, 76, 97, 121, 101, 114]]}, {"kind": {"AnOutputOfAFileOutputNode": {"node": [66, 101, 97, 117, 116, 121], "output": [73, 109, 97, 103, 101], "the_nodes_base_file_name": [102, 105, 120, 116, 117, 114, 101]}}, "recorded_location": [47, 47, 110, 111, 100, 101, 115, 47, 98, 101, 97, 117, 116, 121, 47], "format": {"as_the_file_records_it": [80, 78, 71], "the_ending_it_writes": [46, 112, 110, 103], "is_video_shaped": false}, "origin": {"AFileOutputNodeNamed": [66, 101, 97, 117, 116, 121]}, "scene": "TheJobsOwnScene", "the_view_layers_that_feed_it": [[86, 105, 101, 119, 76, 97, 121, 101, 114]]}, {"kind": {"AnAddedWritePlace": "TheBakeOutputPath"}, "recorded_location": [47, 47], "format": null, "origin": {"AnAddedWritePlaceNamed": "TheBakeOutputPath"}, "scene": "TheJobsOwnScene", "the_view_layers_that_feed_it": []}, {"kind": {"AnAddedWritePlace": "ThePointCache"}, "recorded_location": null, "format": null, "origin": {"AnAddedWritePlaceNamed": "ThePointCache"}, "scene": "TheJobsOwnScene", "the_view_layers_that_feed_it": []}, {"kind": {"AnAddedWritePlace": "TheGeometryNodesBakeFolder"}, "recorded_location": null, "format": null, "origin": {"AnAddedWritePlaceNamed": "TheGeometryNodesBakeFolder"}, "scene": "TheJobsOwnScene", "the_view_layers_that_feed_it": []}, {"kind": {"AnAddedWritePlace": "TheSequencerProxyFolder"}, "recorded_location": null, "format": null, "origin": {"AnAddedWritePlaceNamed": "TheSequencerProxyFolder"}, "scene": "TheJobsOwnScene", "the_view_layers_that_feed_it": []}], "scenes_reached_through_a_sequencer_strip": [], "the_tree_is_shared_with": null, "per_frame": [{"frame": 1, "view_layers_that_render": [[86, 105, 101, 119, 76, 97, 121, 101, 114]], "gating": {"compositing_switch": true, "a_compositing_tree_is_present": true, "a_live_final_output_node": true, "the_final_output_node_is_muted": false, "four_fives_use_nodes_switch": null, "save_output_switch": true, "the_render_goes_out_through_a_sequencer_holding_a_scene_strip": false}, "the_regular_output_paths_answered_path": [47, 112, 114, 105, 118, 97, 116, 101, 47, 116, 109, 112, 47, 99, 108, 97, 117, 100, 101, 45, 53, 48, 49, 47, 45, 85, 115, 101, 114, 115, 45, 107, 105, 115, 108, 121, 97, 107, 111, 118, 45, 68, 101, 115, 107, 116, 111, 112, 45, 87, 111, 114, 107, 45, 83, 109, 111, 111, 122, 121, 45, 118, 50, 47, 48, 100, 54, 52, 102, 54, 55, 99, 45, 50, 100, 102, 57, 45, 52, 57, 53, 101, 45, 56, 56, 50, 52, 45, 57, 48, 53, 102, 51, 54, 97, 97, 50, 98, 53, 49, 47, 115, 99, 114, 97, 116, 99, 104, 112, 97, 100, 47, 112, 49, 51, 45, 50, 101, 48, 100, 100, 97, 54, 53, 99, 51, 54, 55, 98, 49, 51, 56, 47, 102, 105, 108, 101, 115, 47, 111, 117, 116, 47, 102, 95, 48, 48, 48, 49, 46, 112, 110, 103]}], "recorded_locations": [[47, 47, 111, 117, 116, 47, 102, 95], [47, 47], [47, 47, 110, 111, 100, 101, 115, 47, 98, 101, 97, 117, 116, 121, 47], [102, 105, 120, 116, 117, 114, 101]], "views": "One", "the_compositing_tree": {"nodes": [{"name": [71, 114, 111, 117, 112, 32, 79, 117, 116, 112, 117, 116], "kind": "TheFinalOutputNode"}, {"name": [82, 101, 110, 100, 101, 114, 32, 76, 97, 121, 101, 114, 115], "kind": {"RenderLayers": {"scene": [75, 105, 116, 99, 104, 101, 110], "view_layer": [86, 105, 101, 119, 76, 97, 121, 101, 114], "outputs": [[73, 109, 97, 103, 101], [65, 108, 112, 104, 97]]}}}, {"name": [66, 101, 97, 117, 116, 121], "kind": {"AFileOutputNode": {"inputs": [[73, 109, 97, 103, 101]], "gathering_into_one_file": false}}}], "links": [{"from": {"node": 1, "index": 0}, "to": {"node": 2, "index": 0}}, {"from": {"node": 1, "index": 0}, "to": {"node": 0, "index": 0}}]}}, "settings": {"scenes": [{"name": "Kitchen", "frame_start": 1, "frame_end": 1, "frame_step": 1, "frame_count": 1, "full_size": [1920, 1080], "resolution_percentage": 100, "size_at_the_files_percentage": [1920, 1080], "engine_as_read": "BLENDER_EEVEE", "sample_counts": {"cycles": 4096, "eevee": 64}, "device": "TheProcessor", "early_stop": {"adaptive_sampling_is_on": true, "threshold": 0.009999999776482582, "minimum": 0}, "time_limit": 0.0, "layer_samples_rule": "Use", "saved_with": "5.2.45", "view_layers": [{"name": "ViewLayer", "renders_at": [[1, true]], "animation": {"None": {"value": true}}, "own_sample_count": 0, "overrides_the_scenes_count": false}], "pulls_in": [], "simplify": {"use_simplify": false, "subdivision_render": 6, "child_particles_render": 1.0, "texture_limit_render": "OFF"}}], "offered_engine_identifiers": {"cycles": "CYCLES", "eevee": "BLENDER_EEVEE"}, "announced_add_on_case": null}, "dependencies": {}, "errors": {}}"#;

    #[test]
    fn what_a_real_blender_printed_reads_back_into_this_crates_shapes() {
        for (named, printed, major, minor, final_node) in [
            ("4.5.12", WHAT_FOUR_FIVE_PRINTED, 4u16, 5u16, "Composite"),
            ("5.2.2", WHAT_FIVE_TWO_PRINTED, 5, 2, "Group Output"),
        ] {
            let mut stream = b"Blender\n".to_vec();
            stream.extend_from_slice(THE_OPENING_SENTINEL);
            stream.extend_from_slice(printed.as_bytes());
            stream.extend_from_slice(THE_CLOSING_SENTINEL);
            let read = the_printed_reading_in(&stream)
                .unwrap_or_else(|why| panic!("{named}: {why:?}"));
            assert_eq!(
                read.outputs.taken_by,
                TheVersionInHand::of(major, minor),
                "{named}"
            );
            assert!(read.errors.is_empty(), "{named}");

            // **The tree the walk is made against**, with the version's own
            // final output node found as that and not as something else.
            let tree = read
                .outputs
                .the_compositing_tree
                .clone()
                .unwrap_or_else(|| panic!("{named}: the scene has a tree"));
            let at = tree
                .nodes
                .iter()
                .position(|node| node.name.as_bytes() == final_node.as_bytes())
                .unwrap_or_else(|| panic!("{named}: the final output node is named"));
            assert_eq!(
                tree.nodes[at].kind,
                crate::main_output::NodeKind::TheFinalOutputNode,
                "{named}"
            );

            // **The entry point's own sockets, and what this run found there.**
            //
            // **B1 read the beauty off the socket by the name `Combined` until
            // 2026-09-20**, on the research's words *"called Combined in every
            // version — in a written file, in a multi-layer file's list of
            // layers, and on the socket it comes out of"*. **The third of those
            // three is not what these Blenders say.** Every Render Layers node
            // in all four, under Cycles and under the built-in engine alike,
            // calls the socket the beauty leaves on **`Image`**, and no socket
            // on the node is called `Combined` at all.
            //
            // **Where that four comes from, since the two documents below are
            // 4.5.12's and 5.2.2's.** 5.0.1 and 5.1.2 were read for the socket
            // on 2026-09-20 by a later sitting, which printed the whole output
            // socket list of a Render Layers node in all four, under Cycles and
            // under the built-in engine, at the factory passes and with every
            // pass switch on. Those lists are held below.
            //
            // **So the walk came back `NoEntry` for every real file**, which is
            // B8's stated outcome applied to every job rather than to a failed
            // one — *show outputs* opening the job's own folder, the preview
            // falling to the last image, and B9's duplicate-picture warning
            // never firing. **It stopped no render**: the map, the inventory
            // and the frames were all composed without it.
            //
            // **B1 was corrected to the socket name the four Blenders print,
            // and the other two of its three places were left alone** — the
            // name inside a written file and the name in a multi-layer file's
            // list of layers, which is what B19 rests on. **Nothing here
            // substitutes a name to make a test pass**: `Image` is what these
            // runs printed, and the assertions below hold both halves of it.
            let entry = tree
                .nodes
                .iter()
                .find_map(|node| match &node.kind {
                    crate::main_output::NodeKind::RenderLayers { outputs, .. } => Some(outputs),
                    _ => None,
                })
                .unwrap_or_else(|| panic!("{named}: the tree has an entry point"));
            assert_eq!(
                entry[0].as_bytes(),
                crate::main_output::THE_BEAUTYS_SOCKET_NAME,
                "{named}"
            );
            assert!(
                !entry
                    .iter()
                    .any(|one| one.as_bytes() == crate::main_output::THE_BEAUTY_PASSES_NAME),
                "{named}: no socket on the node carries the pass's own name"
            );

            // **R4 item 7's walk, derived from what was printed** by the one
            // walk this project has.
            let enters_at = WhereTheRenderEnters {
                scene: ArtistsText::of(b"Kitchen".to_vec()),
                view_layer: ArtistsText::of(b"ViewLayer".to_vec()),
            };
            let document = the_one_document_from(read, &enters_at)
                .unwrap_or_else(|_| panic!("{named}: an empty error map is a reading"));
            // **The walk reaches the beauty on an ordinary file**, which is
            // what the socket name being right buys: this file wires the entry
            // node into a file output node called `Beauty` and on to the
            // tree's final output node, so B4's *reached* outcome carries that
            // destination and the regular output path is a candidate too.
            let crate::main_output::WhatTheWalkFound::Reached {
                destinations,
                it_reached_the_final_output_node,
            } = &document.outputs.the_beauty_walk
            else {
                panic!("{named}: {:?}", document.outputs.the_beauty_walk);
            };
            assert!(it_reached_the_final_output_node, "{named}");
            assert_eq!(destinations.len(), 1, "{named}");
            assert_eq!(destinations[0].node.as_bytes(), b"Beauty", "{named}");

            // **The regular output path crossed as the bytes the file holds**
            // (R4a), template marks and all.
            let regular = document
                .outputs
                .destinations
                .iter()
                .find(|one| {
                    one.kind == crate::reading::DestinationKind::TheRegularOutputPath
                })
                .unwrap_or_else(|| panic!("{named}: the regular output path is a destination"));
            assert_eq!(
                regular.recorded_location.as_ref().map(|one| one.as_bytes()),
                Some(&b"//out/f_"[..]),
                "{named}"
            );

            // **I1's asking: the name for the frame came from Blender**, whole
            // and absolute, rather than being built here.
            let frame = &document.outputs.per_frame[0];
            assert_eq!(frame.frame, 1, "{named}");
            let answered = frame
                .the_regular_output_paths_answered_path
                .as_ref()
                .unwrap_or_else(|| panic!("{named}: the frame's name was answered"));
            assert!(answered.as_bytes().ends_with(b"f_0001.png"), "{named}");
        }
    }

    /// **The whole output socket list of a Render Layers node, off all four
    /// Blenders at the desk, under both engines.**
    ///
    /// The two documents above are 4.5.12's and 5.2.2's, so on their own they
    /// hold B1's third place for two of the four versions the clause names.
    /// **5.0.1 and 5.1.2 were read for it on 2026-09-20** and these are all
    /// four: each list is a node's outputs printed whole, with the passes as
    /// the factory leaves them and again with every pass switch on, and read
    /// afterwards. **Blender was never asked whether a socket called
    /// `Combined` exists.** Record: `Audits/Build/Step 10c/How many Blenders
    /// were read for the socket — what was found.md`.
    /// **The tree as read carries a File Output node's mute**, and the walk
    /// derived from it counts a muted node as nothing (R4 item 2). The
    /// Creator's twelfth notes, note 6. **A reading printed before the field
    /// existed** — the two real Blenders' documents above — **still parses, and
    /// reads as not muted.**
    #[test]
    fn a_file_output_nodes_mute_is_read_and_an_older_reading_reads_as_not_muted() {
        // The script asks every version's node for its mute, inside the
        // tree's own reading of a File Output node.
        for (major, minor) in [(4u16, 5u16), (5, 0), (5, 1), (5, 2)] {
            let text = the_script(major, minor);
            let kind_of = text.find("def the_kind_of(node, top):").expect("the tree's reading");
            let its_end = kind_of + text[kind_of..].find("\ndef ").expect("its end");
            let asked = &text[kind_of..its_end];
            let node = asked.find("\"AFileOutputNode\": {").expect("the node's shape");
            let mute = asked.find("\"muted\": bool(node.mute),").expect("the node's mute");
            assert!(node < mute, "{major}.{minor}: the mute is the node's own");
        }

        let read = |printed: &str| {
            let mut stream = b"Blender\n".to_vec();
            stream.extend_from_slice(THE_OPENING_SENTINEL);
            stream.extend_from_slice(printed.as_bytes());
            stream.extend_from_slice(THE_CLOSING_SENTINEL);
            the_printed_reading_in(&stream).expect("it reads")
        };
        let the_nodes_mute = |printed: &ThePrintedReading| {
            printed
                .outputs
                .the_compositing_tree
                .as_ref()
                .expect("a tree")
                .nodes
                .iter()
                .find_map(|node| match &node.kind {
                    crate::main_output::NodeKind::AFileOutputNode { muted, .. } => Some(*muted),
                    _ => None,
                })
                .expect("a File Output node")
        };
        let enters_at = WhereTheRenderEnters {
            scene: ArtistsText::of(b"Kitchen".to_vec()),
            view_layer: ArtistsText::of(b"ViewLayer".to_vec()),
        };
        let the_walk = |printed: ThePrintedReading| {
            the_one_document_from(printed, &enters_at)
                .expect("a reading")
                .outputs
                .the_beauty_walk
        };

        for older in [WHAT_FOUR_FIVE_PRINTED, WHAT_FIVE_TWO_PRINTED] {
            assert!(!older.contains("\"muted\""), "printed before the field");
            let printed = read(older);
            assert!(!the_nodes_mute(&printed));
            let crate::main_output::WhatTheWalkFound::Reached { destinations, .. } =
                the_walk(printed)
            else {
                panic!("the walk reached the node");
            };
            assert_eq!(destinations.len(), 1);
        }

        // **The same document with the node muted**: read as muted, and the
        // walk reaches the final output node and no File Output node.
        let the_node_as_printed = "\"gathering_into_one_file\": false}}}";
        assert_eq!(WHAT_FIVE_TWO_PRINTED.matches(the_node_as_printed).count(), 1);
        let muted = WHAT_FIVE_TWO_PRINTED.replace(
            the_node_as_printed,
            "\"gathering_into_one_file\": false, \"muted\": true}}}",
        );
        let printed = read(&muted);
        assert!(the_nodes_mute(&printed));
        assert_eq!(
            the_walk(printed),
            crate::main_output::WhatTheWalkFound::Reached {
                destinations: Vec::new(),
                it_reached_the_final_output_node: true,
            }
        );
    }

    const WHAT_THE_FOUR_PRINTED_AT_THE_ENTRY_POINT: &[(&str, &str, &str, &str)] = &[
        (
            "4.5.12",
            "BLENDER_EEVEE_NEXT",
            "Image, Alpha, Depth, Normal, UV, Vector, Position, Deprecated, Deprecated, Shadow, AO, Deprecated, Deprecated, Deprecated, IndexOB, IndexMA, Mist, Emit, Env, DiffDir, DiffInd, DiffCol, GlossDir, GlossInd, GlossCol, TransDir, TransInd, TransCol, SubsurfaceDir, SubsurfaceInd, SubsurfaceCol",
            "Image, Alpha, Depth, Mist, Normal, Position, Vector, DiffDir, DiffCol, GlossDir, GlossCol, Emit, Env, Shadow, AO, Transp, CryptoObject00, CryptoObject01, CryptoObject02, CryptoAsset00, CryptoAsset01, CryptoAsset02, CryptoMaterial00, CryptoMaterial01, CryptoMaterial02, GreasePencil, UV, Deprecated, Deprecated, Deprecated, Deprecated",
        ),
        (
            "4.5.12",
            "CYCLES",
            "Image, Alpha, Noisy Image, Depth, Normal, UV, Vector, Position, Deprecated, Deprecated, Shadow, AO, Deprecated, Deprecated, Deprecated, IndexOB, IndexMA, Mist, Emit, Env, DiffDir, DiffInd, DiffCol, GlossDir, GlossInd, GlossCol, TransDir, TransInd, TransCol, SubsurfaceDir, SubsurfaceInd",
            "Image, Alpha, Depth, Mist, Position, Normal, Vector, UV, IndexOB, IndexMA, DiffDir, DiffInd, DiffCol, GlossDir, GlossInd, GlossCol, TransDir, TransInd, TransCol, VolumeDir, VolumeInd, Emit, Env, AO, Shadow Catcher, CryptoObject00, CryptoObject01, CryptoObject02, CryptoMaterial00, CryptoMaterial01, CryptoMaterial02, CryptoAsset00, CryptoAsset01, CryptoAsset02, Noisy Image, Noisy Shadow Catcher, Denoising Normal, Denoising Albedo, Denoising Depth, GreasePencil",
        ),
        (
            "5.0.1",
            "BLENDER_EEVEE",
            "Image, Alpha, Depth, Normal, UV, Vector, Position, Deprecated, Deprecated, Shadow, Ambient Occlusion, Deprecated, Deprecated, Deprecated, Object Index, Material Index, Mist, Emission, Environment, Diffuse Direct, Diffuse Indirect, Diffuse Color, Glossy Direct, Glossy Indirect, Glossy Color, Transmission Direct, Transmission Indirect, Transmission Color, Subsurface Direct, Subsurface Indirect, Subsurface Color",
            "Image, Alpha, Depth, Mist, Normal, Position, Vector, Diffuse Direct, Diffuse Color, Glossy Direct, Glossy Color, Emission, Environment, Shadow, Ambient Occlusion, Transparent, CryptoObject00, CryptoObject01, CryptoObject02, CryptoAsset00, CryptoAsset01, CryptoAsset02, CryptoMaterial00, CryptoMaterial01, CryptoMaterial02, Grease Pencil, UV, Deprecated, Deprecated, Deprecated, Deprecated",
        ),
        (
            "5.0.1",
            "CYCLES",
            "Image, Alpha, Noisy Image, Depth, Normal, UV, Vector, Position, Deprecated, Deprecated, Shadow, Ambient Occlusion, Deprecated, Deprecated, Deprecated, Object Index, Material Index, Mist, Emission, Environment, Diffuse Direct, Diffuse Indirect, Diffuse Color, Glossy Direct, Glossy Indirect, Glossy Color, Transmission Direct, Transmission Indirect, Transmission Color, Subsurface Direct, Subsurface Indirect",
            "Image, Alpha, Depth, Mist, Position, Normal, Vector, UV, Object Index, Material Index, Diffuse Direct, Diffuse Indirect, Diffuse Color, Glossy Direct, Glossy Indirect, Glossy Color, Transmission Direct, Transmission Indirect, Transmission Color, Volume Direct, Volume Indirect, Volume Scatter, Volume Transmit, Volume Majorant, Emission, Environment, Ambient Occlusion, Shadow Catcher, CryptoObject00, CryptoObject01, CryptoObject02, CryptoMaterial00, CryptoMaterial01, CryptoMaterial02, CryptoAsset00, CryptoAsset01, CryptoAsset02, Noisy Image, Noisy Shadow Catcher, Denoising Normal, Denoising Albedo, Denoising Depth, Grease Pencil",
        ),
        (
            "5.1.2",
            "BLENDER_EEVEE",
            "Image, Alpha",
            "Image, Alpha, Depth, Mist, Normal, Position, Vector, Diffuse Direct, Diffuse Color, Glossy Direct, Glossy Color, Emission, Environment, Shadow, Ambient Occlusion, Transparent, CryptoObject00, CryptoObject01, CryptoObject02, CryptoAsset00, CryptoAsset01, CryptoAsset02, CryptoMaterial00, CryptoMaterial01, CryptoMaterial02, Grease Pencil",
        ),
        (
            "5.1.2",
            "CYCLES",
            "Image, Alpha, Noisy Image",
            "Image, Alpha, Depth, Mist, Position, Normal, Vector, UV, Object Index, Material Index, Diffuse Direct, Diffuse Indirect, Diffuse Color, Glossy Direct, Glossy Indirect, Glossy Color, Transmission Direct, Transmission Indirect, Transmission Color, Volume Direct, Volume Indirect, Volume Scatter, Volume Transmit, Volume Majorant, Emission, Environment, Ambient Occlusion, Shadow Catcher, CryptoObject00, CryptoObject01, CryptoObject02, CryptoMaterial00, CryptoMaterial01, CryptoMaterial02, CryptoAsset00, CryptoAsset01, CryptoAsset02, Noisy Image, Noisy Shadow Catcher, Denoising Albedo, Denoising Normal, Denoising Depth, Grease Pencil",
        ),
        (
            "5.2.2 LTS",
            "BLENDER_EEVEE",
            "Image, Alpha",
            "Image, Alpha, Depth, Mist, Normal, Position, Vector, Diffuse Direct, Diffuse Color, Glossy Direct, Glossy Color, Emission, Environment, Shadow, Ambient Occlusion, Transparent, CryptoObject00, CryptoObject01, CryptoObject02, CryptoAsset00, CryptoAsset01, CryptoAsset02, CryptoMaterial00, CryptoMaterial01, CryptoMaterial02, Grease Pencil",
        ),
        (
            "5.2.2 LTS",
            "CYCLES",
            "Image, Alpha, Noisy Image",
            "Image, Alpha, Depth, Mist, Position, Normal, Vector, UV, Object Index, Material Index, Diffuse Direct, Diffuse Indirect, Diffuse Color, Glossy Direct, Glossy Indirect, Glossy Color, Transmission Direct, Transmission Indirect, Transmission Color, Volume Direct, Volume Indirect, Volume Scatter, Volume Transmit, Volume Majorant, Emission, Environment, Ambient Occlusion, Shadow Catcher, CryptoObject00, CryptoObject01, CryptoObject02, CryptoMaterial00, CryptoMaterial01, CryptoMaterial02, CryptoAsset00, CryptoAsset01, CryptoAsset02, Noisy Image, Noisy Shadow Catcher, Denoising Albedo, Denoising Specular Albedo, Denoising Normal, Denoising Roughness, Denoising Depth, Grease Pencil",
        ),
    ];

    /// **The beauty leaves the entry point on `Image` in all four, under both
    /// engines, and no socket on that node carries the pass's own name.**
    #[test]
    fn the_entry_points_socket_list_is_held_for_all_four() {
        let mut how_many = 0;
        for (version, engine, as_the_factory_leaves_it, with_every_pass_on) in
            WHAT_THE_FOUR_PRINTED_AT_THE_ENTRY_POINT
        {
            for (state, printed) in [
                ("as the factory leaves it", as_the_factory_leaves_it),
                ("with every pass switch on", with_every_pass_on),
            ] {
                let named = format!("{version} under {engine}, {state}");
                let sockets: Vec<&str> = printed.split(", ").collect();
                assert_eq!(
                    sockets[0].as_bytes(),
                    crate::main_output::THE_BEAUTYS_SOCKET_NAME,
                    "{named}"
                );
                assert!(
                    !sockets
                        .iter()
                        .any(|one| one.as_bytes() == crate::main_output::THE_BEAUTY_PASSES_NAME),
                    "{named}: no socket on the node carries the pass's own name"
                );
            }
            how_many += 1;
        }
        assert_eq!(
            how_many, 8,
            "four versions under two engines, and a list was added or lost"
        );
    }

    /// **R4c, the nineteenth notes, notes 3 and 9: only the render-start
    /// reading of a first-frame check is told to read a video-only scene as a
    /// picture**, with the version's own names; the switch comes before
    /// anything is read, asks whether the scene writes only a video, and the
    /// reading says whether it switched. Nothing is saved.
    #[test]
    fn only_a_checks_render_start_reading_is_told_to_read_a_video_as_a_picture() {
        let opens = "SMOOZY = json.loads(r\'\'\'";
        let handed_in = |script: &str| -> serde_json::Value {
            let from = script.find(opens).expect("the document") + opens.len();
            let to = from + script[from..].find("\'\'\'").expect("its end");
            serde_json::from_str(&script[from..to]).expect("a document")
        };
        for (major, minor, media) in [(4, 5, None), (5, 0, Some("IMAGE")), (5, 2, Some("IMAGE")), (5, 4, Some("IMAGE"))] {
            let mut invocation = an_invocation(major, minor);
            let ordinary = the_reading_script(&invocation, None).expect("composed");
            assert_eq!(handed_in(&ordinary)["the_check_is_written_as_a_picture"], false);
            invocation.the_check_is_written_as_a_picture = true;
            let check = the_reading_script(&invocation, None).expect("composed");
            let document = handed_in(&check);
            assert_eq!(document["the_check_is_written_as_a_picture"], true);
            assert_eq!(document["the_picture_the_check_is_written_as"]["format"], "PNG");
            assert_eq!(
                document["the_picture_the_check_is_written_as"]["media"],
                media.map_or(serde_json::Value::Null, serde_json::Value::from),
                "{major}.{minor}"
            );
            // **The pre-load reading is never told**, whatever is handed in.
            let pre_load = the_pre_load_reading_script(&invocation).expect("composed");
            assert_eq!(handed_in(&pre_load)["the_check_is_written_as_a_picture"], false);
        }
        let text = the_script(5, 2);
        let switch = text.find("THE_CHECK_IS_WRITTEN_AS_A_PICTURE = False").expect("the switch");
        for later in [
            "for scene in bpy.data.scenes:",
            "destinations = the_destinations_of_the_scene(the_jobs_scene, reached)",
        ] {
            assert!(switch < text.find(later).expect(later), "the switch comes before {later}");
        }
        let asked = &text[text.find("def smoozy_writes_only_a_video").unwrap()..switch];
        assert!(asked.contains("if not scene.render.is_movie_format:"));
        assert!(asked.contains(r#"NAMES["the_save_output_switch"]"#));
        assert!(asked.contains(r#"is_kind(node, NAMES["the_file_output_nodes_type"]) and not node.mute"#));
        let set = &text[switch..switch + text[switch..].find("\n# ").expect("the next comment")];
        let media = set.find(r#"setattr(image_settings, NAMES["the_media_kind"], still["media"])"#).expect("media");
        let format = set.find(r#"image_settings.file_format = still["format"]"#).expect("format");
        assert!(media < format, "the media setting first");
        assert!(text.contains(r#""the_check_is_written_as_a_picture": THE_CHECK_IS_WRITTEN_AS_A_PICTURE,"#));
        for never in ["save_as_mainfile", "save_mainfile"] {
            assert!(!text.contains(never), "{never}: the reading saves nothing");
        }
        // **The reading's answer reads back, and an older script's reads as no.**
        let mut printed = serde_json::to_value(&a_printed_reading()).unwrap();
        printed.as_object_mut().unwrap().remove("the_check_is_written_as_a_picture");
        let old: ThePrintedReading = serde_json::from_value(printed).unwrap();
        assert!(!old.the_check_is_written_as_a_picture);
    }

    /// **R4b: only the pre-load reading asks for every scene's half**; the
    /// render-start reading, named scene or none, never does. The Creator's
    /// seventh notes, note 1.
    #[test]
    fn only_the_pre_load_reading_asks_for_every_scenes_half() {
        let opens = "SMOOZY = json.loads(r'''";
        let handed_in = |script: &str| -> serde_json::Value {
            let from = script.find(opens).expect("the document") + opens.len();
            let to = from + script[from..].find("'''").expect("its end");
            serde_json::from_str(&script[from..to]).expect("a document")
        };
        for (major, minor) in [(4, 5), (5, 0), (5, 1), (5, 2)] {
            let invocation = an_invocation(major, minor);
            let pre_load = the_pre_load_reading_script(&invocation).expect("composed");
            assert_eq!(handed_in(&pre_load)["every_scene"], true);
            assert_eq!(handed_in(&pre_load)["the_jobs_scene"], serde_json::Value::Null);
            let render_start = the_reading_script(&invocation, None).expect("composed");
            assert_eq!(handed_in(&render_start)["every_scene"], false);
            let named = the_reading_script(&invocation, Some(&ArtistsText::of(b"Interior".to_vec())))
                .expect("composed");
            assert_eq!(handed_in(&named)["every_scene"], false);
            // **The same script otherwise**: only the one value differs.
            assert_eq!(
                pre_load.replace(r#""every_scene":true"#, r#""every_scene":false"#),
                render_start
            );
        }
        // **The script's own guard**: a named scene never reads every scene,
        // whatever is handed in, and the halves ride only where it asked.
        let text = the_script(5, 2);
        assert!(text.contains(
            r#"EVERY_SCENE = bool(SMOOZY.get("every_scene")) and SMOOZY["the_jobs_scene"] is None"#
        ));
        assert!(text.contains("if the_other_halves is not None:\n    document[\"the_other_scenes\"] = the_other_halves"));
        // **Each other scene's per-frame set is recorded inside the one walk.**
        assert_eq!(text.matches("scene.frame_set(frame)").count(), 1);
    }

    /// **R4b: another scene's half, made into the reading naming it** — its own
    /// outputs, simulation answer and errors, the file's recorded locations
    /// put back, and the scene named; everything about the file the reading's.
    /// **A reading that holds one scene carries no halves**, which is a
    /// machine program older than R4b.
    #[test]
    fn another_scenes_half_becomes_the_reading_naming_it_and_an_older_reading_holds_none() {
        let mut printed = a_printed_reading();
        printed.outputs.recorded_locations = vec![RecordedLocation::of(b"//textures/wood.png".to_vec())];
        let mut theirs = printed.outputs.clone();
        theirs.recorded_locations = Vec::new();
        theirs.the_view_transform = Some(ArtistsText::of(b"Standard".to_vec()));
        theirs.per_frame.truncate(1);
        let half = AnotherScenesHalf {
            scene: ArtistsText::of(b"Exterior".to_vec()),
            outputs: Some(theirs.clone()),
            the_scene_holds_a_simulation_not_baked: !printed.the_scene_holds_a_simulation_not_baked,
            errors: BTreeMap::from([("the regular output path at frame 1".to_string(), "x".to_string())]),
        };
        let named = printed.as_a_reading_naming(&half).expect("its half was read");
        let mut expected = theirs;
        expected.recorded_locations = printed.outputs.recorded_locations.clone();
        assert_eq!(named.outputs, expected);
        assert_eq!(named.the_scene_it_read, Some(ArtistsText::of(b"Exterior".to_vec())));
        assert_eq!(named.the_scene_holds_a_simulation_not_baked, half.the_scene_holds_a_simulation_not_baked);
        assert_eq!(named.errors, half.errors);
        assert_eq!(named.settings, printed.settings);
        assert_eq!(named.the_baked_caches_on_the_machine, printed.the_baked_caches_on_the_machine);
        assert_eq!(printed.as_a_reading_naming(&AnotherScenesHalf { outputs: None, ..half.clone() }), None);

        let framed = |document: &serde_json::Value| {
            let mut stream = b"chatter\n".to_vec();
            stream.extend_from_slice(THE_OPENING_SENTINEL);
            stream.extend_from_slice(&serde_json::to_vec(document).unwrap());
            stream.extend_from_slice(THE_CLOSING_SENTINEL);
            stream
        };
        let mut document = serde_json::to_value(&printed).unwrap();
        assert_eq!(TheOtherScenes::in_the_stream(&framed(&document)).the_other_scenes, None, "an older reading");
        document
            .as_object_mut()
            .unwrap()
            .insert("the_other_scenes".to_string(), serde_json::to_value(vec![half.clone()]).unwrap());
        let stream = framed(&document);
        assert_eq!(TheOtherScenes::in_the_stream(&stream).the_other_scenes, Some(vec![half]));
        // **The reading itself reads exactly as before** beside them.
        assert_eq!(the_printed_reading_in(&stream).unwrap(), printed);
        assert_eq!(TheOtherScenes::in_the_stream(b"no marks").the_other_scenes, None);
    }

    /// **The Creator's twelfth notes, note 6, read by a real Blender**: one
    /// run of this script, alone, on a copy of the Creator's file. Run by hand
    /// at the desk, with the copy's path:
    ///
    /// ```text
    /// SMOOZY_DESK_BLEND=<copy>.blend cargo test -p smoozy-job-files-machine --lib a_real_blender_reads_a_muted -- --ignored --nocapture
    /// ```
    ///
    /// Blender 5.1.2 is found at its usual place on this Mac or at
    /// `SMOOZY_DESK_BLENDER_51`; the scene is `Scene` or `SMOOZY_DESK_SCENE`.
    /// **The run is never waited on past five minutes.**
    #[test]
    #[ignore = "run by hand at the desk with a real Blender and a copy of the file"]
    fn a_real_blender_reads_a_muted_file_output_node_as_muted() {
        use std::time::{Duration, Instant};
        let file = std::env::var("SMOOZY_DESK_BLEND").expect("SMOOZY_DESK_BLEND names the copy");
        let blender = std::env::var("SMOOZY_DESK_BLENDER_51")
            .unwrap_or_else(|_| "/Applications/Blender 5.1.2.app/Contents/MacOS/Blender".to_string());
        let scene = std::env::var("SMOOZY_DESK_SCENE").unwrap_or_else(|_| "Scene".to_string());
        let scene = ArtistsText::of(scene.into_bytes());
        let script = the_reading_script(
            &TheInvocation {
                the_blender_build_path: blender.clone(),
                the_blend_path_on_the_machine: RecordedLocation::of(file.as_bytes().to_vec()),
                the_version_it_will_render_by: TheVersionInHand::of(5, 1),
                the_frames_the_job_covers: vec![1],
                the_check_is_written_as_a_picture: false,
                the_check_is_marked: false,
            },
            Some(&scene),
        )
        .expect("every name is on record");
        let folder = std::path::Path::new(&file).parent().expect("the copy's folder");
        let written = folder.join("smoozy-reading-5-1.py");
        std::fs::write(&written, script).expect("written");

        let mut child = std::process::Command::new(&blender)
            .args(["--factory-startup", "--background", "--python-exit-code", "63", &file])
            .arg("--python")
            .arg(&written)
            .stdin(std::process::Stdio::null())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::null())
            .spawn()
            .expect("Blender starts");
        let mut stdout = child.stdout.take().expect("its stream");
        let reader = std::thread::spawn(move || {
            let mut all = Vec::new();
            std::io::Read::read_to_end(&mut stdout, &mut all).expect("the stream reads");
            all
        });
        let started = Instant::now();
        let status = loop {
            if let Some(status) = child.try_wait().expect("it can be asked") {
                break status;
            }
            if started.elapsed() > Duration::from_secs(300) {
                let _ = child.kill();
                let _ = child.wait();
                panic!("Blender ran past five minutes");
            }
            std::thread::sleep(Duration::from_millis(100));
        };
        let stream = reader.join().expect("the reader ends");
        println!("Blender {blender} ended {status:?} in {:?}", started.elapsed());
        assert_eq!(status.code(), Some(0), "the reading ends at success");

        let printed = the_printed_reading_in(&stream).expect("the document is between its marks");
        assert!(printed.errors.is_empty(), "{:?}", printed.errors);
        let tree = printed.outputs.the_compositing_tree.clone().expect("the scene has a tree");
        let mut muted_nodes = 0;
        for node in &tree.nodes {
            if let crate::main_output::NodeKind::AFileOutputNode { muted, .. } = &node.kind {
                println!(
                    "File Output node {:?}: muted {muted}",
                    String::from_utf8_lossy(node.name.as_bytes())
                );
                if *muted {
                    muted_nodes += 1;
                }
            }
        }
        assert!(muted_nodes > 0, "the file's File Output node is read as muted");

        let video = printed.outputs.destinations.iter().any(|one| {
            one.kind == crate::reading::DestinationKind::TheRegularOutputPath
                && one.format.as_ref().is_some_and(|format| format.is_video_shaped)
        });
        println!("the regular output path writes a video: {video}");
        let enters_at = WhereTheRenderEnters {
            scene,
            view_layer: tree
                .nodes
                .iter()
                .find_map(|node| match &node.kind {
                    crate::main_output::NodeKind::RenderLayers { view_layer, .. } => {
                        Some(view_layer.clone())
                    }
                    _ => None,
                })
                .expect("a Render Layers node"),
        };
        let walk = the_one_document_from(printed, &enters_at)
            .expect("a reading")
            .outputs
            .the_beauty_walk;
        println!("the walk: {walk:?}");
        let frames_and_a_video =
            crate::main_output::the_render_reaches_frames_and_a_video(&walk, video);
        let the_warning =
            crate::main_output::the_beauty_reaches_more_than_one_destination(&walk, video);
        println!("frames and a video: {frames_and_a_video}; the duplicate-picture warning: {the_warning}");
        assert!(!frames_and_a_video);
        assert!(!the_warning);
    }

    /// **The live Telegram pass's fault 4: a video's name is asked over the
    /// job's own range**, in all four, and only there. A job that hands its
    /// frames in has its scene's start and end set to its first and last frame
    /// around the one `frame_path` question, and put back in the same breath;
    /// the job's own scene alone is asked so, and only where it writes a video.
    /// The real Blenders' proof is `tests/a_videos_name_at_the_desk.rs`.
    #[test]
    fn a_videos_name_is_asked_over_the_jobs_own_range_in_all_four() {
        for (major, minor) in [(4, 5), (5, 0), (5, 1), (5, 2)] {
            let text = the_script(major, minor);
            let asked = &text[text.find("def the_videos_name_over_the_jobs_range").expect("the asking")..];
            let asked = &asked[..asked.find("\n\n\ndef ").expect("its end")];
            let set = asked.find("the_range_is_set_on(scene, FRAMES[0], FRAMES[-1])").expect("set");
            let question = asked.find("return scene.render.frame_path(frame=frame)").expect("asked");
            let put_back = asked.find("finally:\n        the_range_is_set_on(scene, saved[0], saved[1])").expect("put back");
            assert!(set < question && question < put_back, "{major}.{minor}: set, asked, put back");
            assert!(
                text.contains("if the_jobs_scene_is_asked and FRAMES and scene.render.is_movie_format:"),
                "{major}.{minor}: only the job's own video, and only with frames handed in"
            );
            assert!(text.contains("per_frame.append(the_entry_at(scene, frame, ERRORS, True))"));
            assert!(text.contains("its_frames.append(the_entry_at(scene, frame, its_errors))"));
            // **The start never passes the end** while it is set.
            let order = &text[text.find("def the_range_is_set_on").unwrap()..];
            assert!(order.starts_with(
                "def the_range_is_set_on(scene, start, end):\n    if start > int(scene.frame_end):\n        scene.frame_end = end\n        scene.frame_start = start\n    else:\n        scene.frame_start = start\n        scene.frame_end = end\n"
            ));
        }
    }

    fn a_printed_reading() -> ThePrintedReading {
        use crate::reading::tests::plain_gating;
        ThePrintedReading {
            outputs: TheObservedOutputs {
                taken_by: TheVersionInHand::of(5, 2),
                gating: plain_gating(),
                destinations: Vec::new(),
                scenes_reached_through_a_sequencer_strip: Vec::new(),
                the_tree_is_shared_with: None,
                per_frame: Vec::new(),
                recorded_locations: Vec::new(),
                views: HowManyViewsTheSceneWrites::One,
                the_compositing_tree: None,
                the_view_transform: None,
            },
            settings: CarriedAndNotInterpretedHere(serde_json::json!({})),
            dependencies: CarriedAndNotInterpretedHere(serde_json::json!({})),
            errors: BTreeMap::new(),
            the_baked_caches_on_the_machine: Vec::new(),
            the_scene_holds_a_simulation_not_baked: false,
            the_scene_it_read: None,
            the_open_files_path: None,
            the_check_is_written_as_a_picture: false,
        }
    }

    // -----------------------------------------------------------------------
    // The 5.3 integration's piece 3
    // -----------------------------------------------------------------------

    /// **The third side of the table.** 4.5 and 5.0 to 5.2 read as they did; from
    /// 5.3 the older name is no name at all and the stack's three are cited; and
    /// every other name is the same on the third side as on the second.
    #[test]
    fn from_five_three_the_older_name_is_gone_and_the_stack_is_cited() {
        let five_two = TheVersionInHand::of(5, 2);
        for version in [TheVersionInHand::of(5, 3), TheVersionInHand::of(5, 4)] {
            assert!(version.holds_a_stack_of_effects());
            assert_eq!(THE_SCENES_COMPOSITING_TREE.for_version(version), Ok(None));
            assert_eq!(THE_SCENES_EFFECTS.for_version(version), Ok(Some("compositor_effects")));
            assert_eq!(AN_EFFECTS_GROUP.for_version(version), Ok(Some("node_group")));
            assert_eq!(AN_EFFECTS_RENDER_SWITCH.for_version(version), Ok(Some("enable_for_render")));
            assert_eq!(THE_NODES_FOLDER.for_version(version), THE_NODES_FOLDER.for_version(five_two));
            assert_eq!(the_walk_steps_over(version).unwrap().last(), Some(&"compositing_node_group"));
        }
        assert!(!five_two.holds_a_stack_of_effects());
        assert_eq!(THE_SCENES_COMPOSITING_TREE.for_version(five_two), Ok(Some("compositing_node_group")));
        assert_eq!(THE_SCENES_COMPOSITING_TREE.for_version(TheVersionInHand::of(4, 5)), Ok(Some("node_tree")));
        assert_eq!(THE_SCENES_EFFECTS.for_version(five_two), Ok(None));
        assert_eq!(the_walk_steps_over(five_two).unwrap(), THE_WALK_STEPS_OVER.to_vec());
    }

    /// **The name reaches no 5.3 script as anything but a step-over, and every
    /// script before 5.3 is handed no stack.** The script's words are the same
    /// text in each; the handed-in document decides.
    #[test]
    fn a_five_three_script_hands_in_the_stack_and_no_older_name() {
        let five_three = the_script(5, 3);
        assert!(five_three.contains(r#""the_scenes_compositing_tree":null"#));
        assert!(five_three.contains(r#""the_scenes_effects":"compositor_effects""#));
        assert!(five_three.contains(r#""an_effects_render_switch":"enable_for_render""#));
        let five_two = the_script(5, 2);
        assert!(five_two.contains(r#""the_scenes_compositing_tree":"compositing_node_group""#));
        assert!(five_two.contains(r#""the_scenes_effects":null"#));
        let marks = the_names_the_mark_is_set_by(TheVersionInHand::of(5, 3)).unwrap();
        assert_eq!(marks["the_scenes_effects"], "compositor_effects");
        assert_eq!(marks["an_effects_group"], "node_group");
        assert!(marks["the_scenes_compositing_tree"].is_null());
    }

    /// **R4 item 1 and B1a through the Rust half**: the printed stack parses, the
    /// walk runs through it by the first frame's effects, and a printed reading
    /// holding no stack parses as it did.
    #[test]
    fn a_printed_stack_is_walked_by_the_effects_on_at_the_first_frame() {
        let bytes = |text: &str| serde_json::json!(text.as_bytes());
        let tree = serde_json::json!({
            "nodes": [{
                "name": bytes("Scene Effect"),
                "kind": {"AnEffect": {"inner": {
                    "nodes": [
                        {"name": bytes("Group Input"), "kind": {"TheRendersEntry": {"outputs": [bytes("Image"), bytes("")]}}},
                        {"name": bytes("Group Output"), "kind": "TheFinalOutputNode"}
                    ],
                    "links": [{"from": {"node": 0, "index": 0}, "to": {"node": 1, "index": 0}}]
                }}}
            }],
            "links": []
        });
        let tree: TheCompositingTree = serde_json::from_value(tree).expect("the stack parses");
        let enters_at = WhereTheRenderEnters { scene: ArtistsText::of(b"Scene".to_vec()), view_layer: ArtistsText::of(b"ViewLayer".to_vec()) };
        assert_eq!(
            crate::main_output::walk_the_beauty_through_the_effects(
                &tree,
                &[ArtistsText::of(b"Scene Effect".to_vec())],
                &enters_at,
                TheVersionInHand::of(5, 3),
            ),
            WhatTheWalkFound::Reached { destinations: Vec::new(), it_reached_the_final_output_node: true }
        );
        let gating: GatingConditions = serde_json::from_value(serde_json::json!({
            "compositing_switch": true,
            "a_compositing_tree_is_present": true,
            "a_live_final_output_node": false,
            "the_final_output_node_is_muted": false,
            "four_fives_use_nodes_switch": null,
            "save_output_switch": true,
            "the_render_goes_out_through_a_sequencer_holding_a_scene_strip": false
        }))
        .expect("a gating with no stack parses");
        assert_eq!(gating.the_effects_on_for_render, None);
        assert!(gating.a_nodes_effect_is_on(None));
        assert!(!serde_json::to_string(&gating).unwrap().contains("the_effects_on_for_render"));
    }
}

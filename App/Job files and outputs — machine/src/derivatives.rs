//! §8.4, performed — **the one derivative of one frame: which file it is made
//! of, the script Blender makes it by, and the record it carries.** Added
//! 2026-10-03, the 38's piece 3.
//!
//! **What was here before this file and is read rather than written again**:
//! the candidate list and the picks ([`crate::main_output::the_candidates`],
//! [`crate::main_output::which_file_the_preview_reads`]), the route by the
//! source's kind ([`crate::main_output::which_route`]), the target size
//! ([`crate::main_output::the_target_size`]), where a derivative is written
//! ([`crate::main_output::where_a_derivative_is_written`]) and the view
//! layer its record carries
//! ([`crate::main_output::the_view_layer_a_derivatives_record_carries`]). This
//! file joins them to what the machine holds for one job: the recorded
//! reading in force, its inventory, and which of its files are on the disk.
//!
//! # B23: made by the Blender the manifest names, headless, and by nothing else
//!
//! **The script is composed from names the Blender research or a desk lookup
//! read, each cited below, and a name with no source stops the composition**
//! rather than being guessed — the shape [`crate::reading_script`] keeps. The
//! names, by what the script does with them:
//!
//! - **Loading a picture, making it smaller and writing it out.**
//!   `bpy.data.images.load`, `Image.scale(width, height)` and
//!   `Image.save_render(filepath, scene=…)`, which the research ran with no
//!   window in all four versions, on the Creator's Mac and on a rented Linux
//!   machine, **with the catch that `save_render` writes in the scene's own
//!   output format whatever the path's ending says** — so the scene's format
//!   is set to PNG first, as those tests did ([`THE_SMALL_COPY_QUESTION`]).
//! - **The picture's size in pixels**, read before any scaling so the target
//!   is this crate's arithmetic and never a second copy of it in the script:
//!   `Image.size`, which the same hand tests printed after the scale
//!   (*"scale() returned, size now (320, 180)"*) ([`THE_SMALL_COPY_QUESTION`]).
//! - **The scene**: `bpy.context.scene`
//!   ([`crate::reading_script::THE_SCENE_NAME_LOOKUP`]), its
//!   `render.image_settings.file_format` (the reading's own name for a
//!   destination's format) set to [`crate::main_output::PNG`], and its
//!   `view_settings.view_transform`
//!   ([`crate::reading_script::THE_VIEW_TRANSFORMS_NAME`]).
//! - **The launch**: the build's own path, then `-b`, `--factory-startup`,
//!   `--python-exit-code 1` and `--python` with the script — the words the
//!   check's reading is launched with, in `Dependency Upload & Remap —
//!   machine`'s `check_script`, less the `.blend` and the load-time script,
//!   because a derivative opens no `.blend`.
//!
//! # B25 and B27: the colour treatment
//!
//! **A working-format picture is written out in the job's own view
//! transform**, which the reading carries
//! ([`crate::reading::TheOutputsHalf::the_view_transform`]). Blender's own
//! manual, as the research quotes it, says the save *"with render color
//! management"* applies *"view and display transform"* for a display format
//! like PNG (`Findings/5.2/Images.md`, section 6). **Where the reading carries
//! none, no derivative of a working-format picture is made**: the choice is
//! B25's *whole difficulty* and nothing here makes it by guessing.
//!
//! **A viewing-format picture already carries its treatment** — the render
//! applied the view transform when it wrote the PNG or JPEG — so the copy is
//! written with the view transform `Standard`, the plain one the same hand
//! tests set, and **no treatment is applied a second time**. That the result
//! matches the source is observed at the desk (the 38's piece 3 account),
//! not documented.
//!
//! # B26's second route: a multi-layer picture
//!
//! **A multi-layer file goes to the image library bundled inside Blender's own
//! Python, pointed at Blender's own colour configuration (B27).** Its names
//! were read off the four Blenders at the desk on 2026-10-03
//! ([`THE_MULTI_LAYER_LOOKUP`]), and the script uses no other:
//!
//! - **The library**: the module `OpenImageIO`, the one module in each
//!   Blender's bundled Python that describes itself as reading and writing
//!   image files.
//! - **Blender's own colour configuration**: `config.ocio` in the folder
//!   `bpy.utils.system_resource("DATAFILES", path="colormanagement")`
//!   returns; its `scene_linear` role, read with
//!   `ColorConfig(path).getColorSpaceNameByRole`, is the space the file's
//!   pixels are in.
//! - **Finding the picture, by name and never by position (B19a)**: every
//!   part, `ImageBuf(path, part, 0)` over `nsubimages`, is asked its
//!   `spec().channelnames`, and the picture is the channels
//!   `<view layer>.Combined.R`, `.G`, `.B` and, where present, `.A`, taken
//!   out with `ImageBufAlgo.channels`. Those channel names hold in every shape
//!   the four write: 4.5's one part, ordered by name; 5.x's part per pass; and
//!   5.x's interleaved one part.
//! - **Scaling** with `ImageBufAlgo.resize` to `ROI(0, width, 0, height)`;
//!   **the view transform** with `ImageBufAlgo.ociodisplay` — the factory
//!   scene's own display, the job's view, `fromspace=` the role's space and
//!   `colorconfig=` the file; **writing** with
//!   `write(path, TypeUInt8, "png")`.
//!
//! **The view layer is the first that feeds the destination, else the first
//! that renders at the frame** — the name B31a's record carries. **A file a
//! file output node gathers is not made**: its pictures are named after the
//! node's own entries, and the reading does not record which entry the beauty
//! reached ([`WhyNothingIsMade::TheBeautysNameInsideTheFileIsNotKnown`]).
//!
//! # B28: a copy with transparency, over a checkerboard
//!
//! **Where the copy carries transparency, it is laid over a checkerboard of
//! light grey and white squares, each a 24th of the copy's shorter side, and
//! written with none** (B28, added 2026-10-17 for the Creator's tenth notes,
//! note 7; built the same day). **Both routes end in the same step**
//! ([`THE_CHECKERBOARD_STEP`]): the multi-layer route lays its picture over the
//! board after the view transform and before the write; the single-picture
//! route, whose copy keeps the source's transparency, reads its own copy back
//! with the bundled library and writes it again laid over the board. Every name
//! is the desk lookup's ([`THE_CHECKERBOARD_LOOKUP`]). **A copy with no alpha
//! channel is left as it is; an opaque copy with one comes out with the same
//! colours, level for level, and no alpha channel** — laying a fully opaque
//! picture over anything gives the picture.
//!
//! **Every marker of another document's is written with that document at its
//! site**, `Queue & History`'s P-clauses among them, because this plan defines
//! a P3 and a P10 of its own.

use serde::{Deserialize, Serialize};

use crate::inventory::{AnEntry, TheInventory};
use crate::main_output::{
    the_view_layer_a_derivatives_record_carries, which_route, ADerivativesRecord, APresentFile,
    ATargetSize, WhatTheMainOutputIs, WhatTheRungsAreToldAboutTheJob, WhatTheWalkFound,
    WhereThePictureCameFrom, WhichRouteMakesTheCopy, PNG, THE_BEAUTY_PASSES_NAME,
};
use crate::reading::{ArtistsText, DestinationKind, TheOutputsHalf, TheVersionInHand};

/// **Where the names of loading, scaling, sizing and saving a picture come
/// from**: the Blender research's *Images* question 8 — *can Blender be made
/// to produce a smaller, plainly viewable copy of a picture a render
/// produced?* — whose hand tests ran in all four versions on the Creator's Mac
/// and again on a rented Linux machine, with the same results
/// (`Preparation/Blender Research/Comparison/Images.md`, question 8; each
/// version's `Findings/<version>/Images.md`, *Resizing a rendered picture and
/// saving a copy*). Label: **observed in a test**; `scale` and `save_render`
/// are also **documented**, their signatures quoted from the API pages the
/// research read.
pub const THE_SMALL_COPY_QUESTION: &str = "the Blender research's Images question 8 \
     (Preparation/Blender Research/Comparison/Images.md; Findings/<version>/Images.md, \
     Resizing a rendered picture and saving a copy)";

/// **Where every name of B26's multi-layer route comes from**: a desk lookup
/// on 4.5.12, 5.0.1, 5.1.2 and 5.2.2, each Blender's bundled Python listed
/// module by module, the route's names enumerated with their own signature
/// text, and the route run against that Blender's own PNG of the same frame —
/// a mean difference of 0.0012, at most one level in 255, in all four.
/// Label: **observed in a test**. Not the Blender research.
pub const THE_MULTI_LAYER_LOOKUP: &str = "Audits/Build/Step 10c/A picture for a multi-layer \
     OpenEXR job — what landed.md, section 1; Preparation/Architecture/Sources/Job files and \
     outputs.sources.md, entry 52a";

/// **Where every name of B28's checkerboard comes from**: a desk lookup on
/// 4.5.12, 5.0.1, 5.1.2, 5.2.2 and 5.3.0 Beta (bundled library 3.0.6.1 to
/// 3.1.16.0), each name read off its own docstring and tried on one picture:
/// `ImageBufAlgo.checker` with an eight-argument `ROI` giving the board four
/// channels, `ImageBufAlgo.over(picture, board)` putting its first argument on
/// top, and `ImageBufAlgo.channels(…, (0, 1, 2))` dropping the alpha. **The
/// board must carry an alpha channel of its own**, or `over` refuses. The same
/// lookup observed that the single-picture route's copy keeps the source's
/// transparency. Label: **observed in a test** (2026-10-17).
pub const THE_CHECKERBOARD_LOOKUP: &str =
    "Audits/Build/Step 10c/Note 7's lookup — transparency in a preview copy.md";

/// **B28's two squares**, as the copy's own eight-bit levels over 255: light
/// grey `#cccccc` (204) in the top-left square, and white `#ffffff` — the
/// drawing's *Product spins* frame and `q-pvalpha` state. Each is opaque: the
/// board needs an alpha channel of its own ([`THE_CHECKERBOARD_LOOKUP`]).
pub const THE_BOARDS_GREY: [f64; 4] = [0.8, 0.8, 0.8, 1.0];
pub const THE_BOARDS_WHITE: [f64; 4] = [1.0, 1.0, 1.0, 1.0];

/// **B28: a square is this fraction of the copy's shorter side** — a 24th,
/// whole pixels, and never less than one.
pub const SQUARES_ALONG_THE_SHORTER_SIDE: u32 = 24;

/// **The format a multi-layer destination records, in every version.** 4.5
/// names it as the format itself ([`crate::reading_script::THE_GATHERING_MARKS_VALUE`]);
/// from 5.0 the format reads the same once the media setting is
/// `MULTI_LAYER_IMAGE`, observed in 5.0.1, 5.1.2 and 5.2.2
/// ([`THE_MULTI_LAYER_LOOKUP`]).
pub const THE_MULTI_LAYER_FORMAT: &[u8] = b"OPEN_EXR_MULTILAYER";

/// **The plain view transform a viewing-format picture is written out in**, so
/// the treatment it already carries is not applied twice. Set and read back by
/// name in all four versions by [`crate::reading_script::THE_VIEW_TRANSFORMS_NAME`]'s
/// hand tests.
pub const THE_PLAIN_VIEW_TRANSFORM: &[u8] = b"Standard";

/// **The marks the script says the source's size between**, and the mark it
/// writes once the copy is written. Smoozy's own, carrying no value of the
/// user's; Blender's own chatter shares the stream, which is why they are
/// marks.
pub const THE_SIZE_OPENS: &[u8] = b"<<<SMOOZY-SIZE>>>";
pub const THE_SIZE_CLOSES: &[u8] = b"<<<END-SMOOZY-SIZE>>>";
pub const THE_MADE_MARK: &[u8] = b"<<<SMOOZY-DERIVATIVE-MADE>>>";

/// **The name of the script file a production writes** — Smoozy's own,
/// beside the operation's two sinks.
pub const THE_SCRIPTS_FILE_NAME: &str = "derivative.py";

/// **B25: the colour treatment one derivative is written out in.**
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TheTreatment {
    /// A working-format source, written out in **the job's own view
    /// transform**, as the reading carries it.
    TheJobsOwnViewTransform(Vec<u8>),
    /// A viewing-format source, which already carries its treatment.
    ItAlreadyCarriesItsTreatment,
}

impl TheTreatment {
    /// The view transform the script sets on its scene before the save.
    pub fn the_view_transform(&self) -> &[u8] {
        match self {
            TheTreatment::TheJobsOwnViewTransform(named) => named,
            TheTreatment::ItAlreadyCarriesItsTreatment => THE_PLAIN_VIEW_TRANSFORM,
        }
    }
}

/// **One derivative, as the script is told it.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ADerivativeToMake {
    /// **B31: the one named source file**, as the bytes the inventory
    /// predicted (R4a).
    pub source: Vec<u8>,
    /// **B29: the staging path**, inside the job record directory — Smoozy's
    /// own path.
    pub staged_at: String,
    pub treatment: TheTreatment,
    /// **B19: the picture's name inside a multi-layer file** —
    /// `<view layer>.Combined` — which sends the script down B26's second
    /// route. Absent for a single-picture file, which Blender's own image
    /// handling loads whole.
    pub the_picture_inside: Option<Vec<u8>>,
}

/// **Why a frame has nothing made for it**, stated rather than left as an
/// absence. Each is a fact about the job's own files or the record, and none
/// is a production that failed: **`Queue & History` P1a's *queued again at the
/// next report* is for a production that failed, and these would fail the
/// same way every time.**
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum WhyNothingIsMade {
    /// **B21**: no candidate at that frame — nothing present, nothing but an
    /// empty or video file, or B20's interrupted last frame.
    ThereIsNoCandidate,
    /// **B19: a multi-layer source whose picture's name inside is not
    /// known** — a file a file output node gathered, whose pictures carry the
    /// node's own entry names while the reading records the node and not the
    /// entry the beauty reached; or no view layer to name it by.
    TheBeautysNameInsideTheFileIsNotKnown,
    /// **B25: a working-format source, and the reading carries no view
    /// transform to write it out in.**
    TheJobsViewTransformWasNotRead,
    /// **The loaded picture has no pixels** — Blender's answer for a
    /// multi-layer file its name did not reveal (*Images* question 8's
    /// neighbour: a multi-layer file loads at `0x0`) — **or a multi-layer
    /// file holds no picture by the name asked for**, which the script says
    /// the same way, as a size of nothing.
    TheSourceHasNoPixels,
    /// No recorded reading stands for the attempt, and the manifest carries
    /// none: there is no inventory to choose a source from.
    NoReadingStands(String),
}

impl WhyNothingIsMade {
    /// **In the machine's own words**, for the record and the complaints
    /// stream — never a screen's.
    pub fn in_words(&self) -> String {
        match self {
            WhyNothingIsMade::ThereIsNoCandidate => {
                "no file of this frame is a candidate (B20, B21)".to_string()
            }
            WhyNothingIsMade::TheBeautysNameInsideTheFileIsNotKnown => {
                "the source is a multi-layer file a file output node gathered, or no view layer \
                 feeds it, so the name of its picture inside is not known (B19)"
                    .to_string()
            }
            WhyNothingIsMade::TheJobsViewTransformWasNotRead => {
                "the source is a working format, and the job's reading carries no view transform \
                 to write it out in (B25)"
                    .to_string()
            }
            WhyNothingIsMade::TheSourceHasNoPixels => {
                "the source loaded with no pixels, or holds no picture by the name asked for"
                    .to_string()
            }
            WhyNothingIsMade::NoReadingStands(why) => why.clone(),
        }
    }
}

/// **Whether a destination writes multi-layer files**, read off the reading
/// and never off the file: a whole node gathering its outputs into one file
/// (I7), or the format recorded as [`THE_MULTI_LAYER_FORMAT`], which is the
/// regular output path's multi-layer files in every version — from 5.0 the
/// format reads it once the separate media setting is the multi-layer one
/// ([`THE_MULTI_LAYER_LOOKUP`]).
pub fn it_writes_multi_layer_files(kind: &DestinationKind, recorded_format: &[u8]) -> bool {
    matches!(kind, DestinationKind::AWholeFileOutputNodeGatheringIntoOneFile { .. })
        || recorded_format == THE_MULTI_LAYER_FORMAT
}

/// **B19 and B31a: the view layer a multi-layer file's beauty is found under**
/// — the first that feeds the destination **and renders at the frame**, else
/// the first that renders at the frame. **None for a file a file output node
/// fed**: its pictures carry the node's own entry names, not a view layer's.
/// That holds after the Creator's ninth notes, note 2: a node gathering one
/// layer's passes into one file still has picture names B19 cannot know, so
/// nothing is made from it; **the name a node-fed copy's record carries** is
/// [`the_derivatives_record`]'s, by [`the_one_view_layer_that_feeds_it`].
///
/// **The frame's layers are the job's**: `the_view_layers_at_this_frame` is
/// the per-frame set with the job's pins applied
/// ([`what_one_frame_is_made_of`]). Corrected 2026-10-04 by piece 1b, on the
/// runner's ruling: the first feeding layer was taken whatever the frame
/// rendered, and a regular output path is fed by every layer the file
/// switches on — so a layer the job pinned off, which the file does not hold,
/// could be picked.
pub fn the_view_layer_the_beauty_is_under<'a>(
    chosen: &'a APresentFileAndItsDestination,
    the_view_layers_at_this_frame: &'a [ArtistsText],
) -> Option<&'a ArtistsText> {
    if chosen.it_came_from_a_file_output_node() {
        return None;
    }
    the_first_layer_that_feeds_it_and_renders(chosen, the_view_layers_at_this_frame)
}

/// The first layer that feeds the destination and renders at this frame,
/// else the frame's first — B19's fallback and B31a's name, one rule.
fn the_first_layer_that_feeds_it_and_renders<'a>(
    chosen: &'a APresentFileAndItsDestination,
    the_view_layers_at_this_frame: &'a [ArtistsText],
) -> Option<&'a ArtistsText> {
    chosen
        .the_view_layers_that_feed_it
        .iter()
        .find(|feeding| the_view_layers_at_this_frame.contains(feeding))
        .or(the_view_layers_at_this_frame.first())
}

/// **B19: the beauty's name inside a multi-layer file**, `<view layer>.Combined`
/// — the stem of the channel names the script looks for
/// ([`THE_MULTI_LAYER_LOOKUP`]).
pub fn the_beautys_name_inside(the_view_layer: &ArtistsText) -> ArtistsText {
    let mut name = the_view_layer.as_bytes().to_vec();
    name.push(b'.');
    name.extend_from_slice(THE_BEAUTY_PASSES_NAME);
    ArtistsText::of(name)
}

// =============================================================================
// The candidate list, from what the machine holds
// =============================================================================

/// **One present file and what this file needs to know about it beyond the
/// candidate list's own fields** — whether it is multi-layer, and the
/// destination's view layers and kind for B31a.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct APresentFileAndItsDestination {
    pub file: APresentFile,
    pub multi_layer: bool,
    /// **Which write place wrote it** — the regular output path, or a File
    /// Output node by name, the node's own whether one of its inputs or the
    /// whole node gathering into one file. What a copy that names no View
    /// Layer records (B31a's last bullet, the Creator's tenth notes, note 2).
    /// *Until 2026-10-17 a yes-or-no that it came from a node, which
    /// [`APresentFileAndItsDestination::it_came_from_a_file_output_node`]
    /// still answers.*
    pub the_destination_it_came_from: WhereThePictureCameFrom,
    pub the_view_layers_that_feed_it: Vec<ArtistsText>,
}

impl APresentFileAndItsDestination {
    /// **Whether a File Output node wrote it** — read off
    /// [`APresentFileAndItsDestination::the_destination_it_came_from`], so it
    /// is one fact kept once.
    pub fn it_came_from_a_file_output_node(&self) -> bool {
        matches!(self.the_destination_it_came_from, WhereThePictureCameFrom::AFileOutputNode(_))
    }
}

/// **B16: the candidate list is the expected-output inventory filtered to the
/// files that are present**, for one frame.
///
/// `size_of` answers a predicted path's size where a file is there, and
/// nothing where none is — **the one look at the disk a derivative's choosing
/// takes**, made by the preview worker and never by the supervisor's
/// queueing (`Queue & History` P1a).
///
/// **What each file's picture is** is the destination's: the beauty
/// (`Combined`, B19) where the walk reached that destination — or, for the
/// regular output path, where the main output is the regular output path —
/// and otherwise the output's own name, which is how B19 chooses a picture
/// written by a file output node.
pub fn the_present_files_at(
    frame: i64,
    reading: &TheOutputsHalf,
    inventory: &TheInventory,
    the_main_output: WhatTheMainOutputIs,
    size_of: &dyn Fn(&[u8]) -> Option<u64>,
) -> Vec<APresentFileAndItsDestination> {
    let reached = |node: &ArtistsText, output: Option<&ArtistsText>| match &reading.the_beauty_walk {
        WhatTheWalkFound::Reached { destinations, .. } => destinations
            .iter()
            .any(|one| &one.node == node && one.output.as_ref() == output),
        _ => false,
    };
    let at_this_frame: Vec<&AnEntry> = inventory
        .entries
        .iter()
        .filter(|entry| entry.frame == Some(frame))
        .collect();
    let the_main_outputs_folders: Vec<&[u8]> = at_this_frame
        .iter()
        .filter(|entry| {
            entry.the_destinations_aiming_at_it.iter().any(|at| {
                reading.destinations.get(*at).is_some_and(|destination| {
                    match &destination.kind {
                        DestinationKind::TheRegularOutputPath => {
                            the_main_output == WhatTheMainOutputIs::TheRegularOutputPath
                        }
                        DestinationKind::AnOutputOfAFileOutputNode { node, output, .. } => {
                            the_main_output == WhatTheMainOutputIs::ADestinationInsideAFileOutputNode
                                && reached(node, Some(output))
                        }
                        DestinationKind::AWholeFileOutputNodeGatheringIntoOneFile { node, .. } => {
                            the_main_output == WhatTheMainOutputIs::ADestinationInsideAFileOutputNode
                                && reached(node, None)
                        }
                        DestinationKind::AnAddedWritePlace(_) => false,
                    }
                })
            })
        })
        .map(|entry| the_folder_of(&entry.predicted_path))
        .collect();
    let mut out = Vec::new();
    for entry in at_this_frame {
        let Some(destination) = entry
            .the_destinations_aiming_at_it
            .first()
            .and_then(|at| reading.destinations.get(*at))
        else {
            continue;
        };
        let Some(bytes) = size_of(&entry.predicted_path) else {
            continue;
        };
        let format = destination
            .format
            .as_ref()
            .map(|format| format.as_the_file_records_it.as_bytes().to_vec())
            .unwrap_or_default();
        let is_video_shaped = entry.may_grow
            || destination.format.as_ref().is_some_and(|format| format.is_video_shaped);
        let beauty = || Some(ArtistsText::of(THE_BEAUTY_PASSES_NAME.to_vec()));
        let (the_picture_it_holds, written_by) = match &destination.kind {
            DestinationKind::TheRegularOutputPath => (
                if the_main_output == WhatTheMainOutputIs::TheRegularOutputPath {
                    beauty()
                } else {
                    None
                },
                WhereThePictureCameFrom::TheOutputPath,
            ),
            DestinationKind::AnOutputOfAFileOutputNode { node, output, .. } => (
                if reached(node, Some(output)) {
                    beauty()
                } else {
                    Some(output.clone())
                },
                WhereThePictureCameFrom::AFileOutputNode(node.clone()),
            ),
            DestinationKind::AWholeFileOutputNodeGatheringIntoOneFile { node, .. } => (
                if reached(node, None) { beauty() } else { None },
                WhereThePictureCameFrom::AFileOutputNode(node.clone()),
            ),
            DestinationKind::AnAddedWritePlace(_) => continue,
        };
        out.push(APresentFileAndItsDestination {
            multi_layer: it_writes_multi_layer_files(&destination.kind, &format),
            file: APresentFile {
                path: entry.predicted_path.clone(),
                frame: entry.frame,
                the_picture_it_holds,
                format,
                bytes,
                is_video_shaped,
                it_sits_in_the_main_outputs_folder: the_main_outputs_folders
                    .contains(&the_folder_of(&entry.predicted_path)),
            },
            the_destination_it_came_from: written_by,
            the_view_layers_that_feed_it: destination.the_view_layers_that_feed_it.clone(),
        });
    }
    out
}

/// **B20's second exclusion, worked out for an interrupted job**: the last
/// frame of the asked order that has a file present. Absent where the job was
/// not interrupted, or where nothing is present.
pub fn the_last_frame_asked_for_that_is_present(
    the_asked_order: &[i64],
    it_was_interrupted: bool,
    present: &dyn Fn(i64) -> bool,
) -> Option<i64> {
    if !it_was_interrupted {
        return None;
    }
    the_asked_order.iter().rev().copied().find(|frame| present(*frame))
}

/// What [`the_rungs_are_told`] needs, so a caller cannot forget B20's half.
pub fn the_rungs_are_told(
    version: TheVersionInHand,
    the_last_frame_asked_for_that_is_present: Option<i64>,
) -> WhatTheRungsAreToldAboutTheJob {
    WhatTheRungsAreToldAboutTheJob {
        version,
        the_last_frame_asked_for_that_is_present,
    }
}

/// **The one view layer that feeds a destination**, where exactly one does —
/// `the_view_layers_that_feed_it` holding that layer and no other, which is
/// what B31a's last bullet calls a destination *whose only feeding view layer
/// it is*. None where several feed it, or none does. Added 2026-10-16, the
/// Creator's ninth notes, note 2.
///
/// **What the reading's list is**, checked against `reading_script.rs`'s
/// `the_view_layers_feeding` and `destinations_of`: each file output node
/// input is its own destination, listing the layers whose Render Layers nodes
/// reach it through any node, a group in either direction, or a gathering
/// node; the regular output path lists every layer the file switches on. Its
/// limits stand as read: a Render Layers node's layer is taken by name,
/// whichever scene the node shows, and muted links and nodes on the way are
/// not skipped.
pub fn the_one_view_layer_that_feeds_it(chosen: &APresentFileAndItsDestination) -> Option<&ArtistsText> {
    match chosen.the_view_layers_that_feed_it.as_slice() {
        [only] => Some(only),
        _ => None,
    }
}

/// **B31a's record for the source chosen**, from the destination it came from
/// and the frame's own view layers (R4 item 5): only where more than one view
/// layer renders at that frame — **inside a multi-layer file, the view layer
/// the beauty is found under** ([`the_view_layer_the_beauty_is_under`]), the
/// name the script is told.
///
/// **A picture a file output node fed with exactly one view layer names that
/// layer** (B31a's first bullet, changed 2026-10-12 for the Creator's ninth
/// notes, note 2; built 2026-10-16) — where that layer renders at the frame,
/// so the name always has a place in the frame's set.
///
/// **A copy that names no View Layer, in a frame of more than one, records
/// its source instead** — the regular output path, or the File Output node by
/// name (B31a's last bullet and the dated note on its first, the Creator's
/// tenth notes, note 2, built 2026-10-17). **That now includes the regular
/// output path's ordinary image**, which was named by the frame's first layer
/// until that day though it holds whichever picture the compositor was set to
/// read. Its place is nought: it is no layer's, and the desktop lists it first.
pub fn the_derivatives_record(
    chosen: &APresentFileAndItsDestination,
    frame: i64,
    the_view_layers_at_this_frame: &[ArtistsText],
) -> ADerivativesRecord {
    let more_than_one = the_view_layers_at_this_frame.len() > 1;
    let the_view_layer = if chosen.it_came_from_a_file_output_node() && more_than_one {
        the_one_view_layer_that_feeds_it(chosen)
            .filter(|only| the_view_layers_at_this_frame.contains(only))
            .cloned()
    } else {
        the_view_layer_a_derivatives_record_carries(
            more_than_one,
            chosen.multi_layer,
            the_view_layer_the_beauty_is_under(chosen, the_view_layers_at_this_frame),
            chosen.it_came_from_a_file_output_node(),
        )
    };
    // **A copy that names no View Layer, in a frame of more than one, records
    // its source** (B31a's last bullet, the Creator's tenth notes, note 2):
    // the regular output path, or the File Output node by name.
    let where_it_came_from = (more_than_one && the_view_layer.is_none())
        .then(|| chosen.the_destination_it_came_from.clone());
    // **Its place in the frame's set** (`Queue & History` P4), where it is
    // named; the frame's one copy is at nought.
    let place = the_view_layer
        .as_ref()
        .and_then(|named| the_view_layers_at_this_frame.iter().position(|at| at == named))
        .unwrap_or(0);
    ADerivativesRecord {
        the_source_file: chosen.file.path.clone(),
        frame,
        the_view_layer_its_picture_came_from: the_view_layer,
        it_is_the_main_copy: true,
        its_place_in_the_frames_set: u32::try_from(place).unwrap_or(u32::MAX),
        where_it_came_from,
    }
}

/// **One other View Layer's copy of a frame** — `Queue & History` P1's new
/// bullet and this document's B31a: made by B26's same four steps, either with
/// this layer's name in step 1 inside the main copy's multi-layer file
/// (added 2026-10-10, the Creator's seventh notes, note 4), or from the file
/// of a destination this layer alone feeds (B31a's last bullet, added
/// 2026-10-16, the Creator's ninth notes, note 2). **The file it is made of is
/// its record's `the_source_file`.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnotherLayersCopy {
    /// B25: the treatment its own source takes — the main copy's, inside the
    /// same multi-layer file.
    pub treatment: TheTreatment,
    /// B19: `<view layer>.Combined` inside a multi-layer source; absent for a
    /// single-picture one, which Blender's own image handling loads whole.
    pub the_picture_inside: Option<ArtistsText>,
    /// B31a: this layer's name, not the main copy, its place, and its source.
    pub record: ADerivativesRecord,
}

/// **`Queue & History` P1: every other View Layer in the frame's set, in the
/// set's order, owed a copy beside the main one, inside the main copy's
/// multi-layer file** — only where the main copy is made from one (its picture
/// is named inside). **Whether the file holds a layer's picture is the
/// script's to find**, as it is for the main copy: a layer the file does not
/// hold says a size of nothing and has nothing made.
pub fn the_other_layers_copies(
    main: &ADerivativesRecord,
    the_main_copys_treatment: &TheTreatment,
    the_main_copys_picture_inside: Option<&ArtistsText>,
    the_view_layers_at_this_frame: &[ArtistsText],
) -> Vec<AnotherLayersCopy> {
    if the_main_copys_picture_inside.is_none() {
        return Vec::new();
    }
    let Some(the_main_layer) = main.the_view_layer_its_picture_came_from.as_ref() else {
        return Vec::new();
    };
    the_view_layers_at_this_frame
        .iter()
        .enumerate()
        .filter(|(_, layer)| *layer != the_main_layer)
        .map(|(place, layer)| AnotherLayersCopy {
            treatment: the_main_copys_treatment.clone(),
            the_picture_inside: Some(the_beautys_name_inside(layer)),
            record: ADerivativesRecord {
                the_source_file: main.the_source_file.clone(),
                frame: main.frame,
                the_view_layer_its_picture_came_from: Some(layer.clone()),
                it_is_the_main_copy: false,
                its_place_in_the_frames_set: u32::try_from(place).unwrap_or(u32::MAX),
                where_it_came_from: None,
            },
        })
        .collect()
}

/// **B31a's last bullet: where the frame's copy is not made by the multi-layer
/// route, each other view layer gets its copy from its own destination.**
/// Added 2026-10-16, the Creator's ninth notes, note 2.
///
/// For each view layer in the frame's set but the main copy's own, in the
/// set's order: the present files of the destinations it alone feeds
/// ([`the_one_view_layer_that_feeds_it`]), put through B19's rungs and B18's
/// tie-break exactly as the main copy's are, with B20's exclusions; then
/// B26's route and B25's treatment for the file chosen ([`how_it_is_made`]).
///
/// - **A destination fed by more than one view layer is no other layer's
///   source** — the regular output path, and a node gathering several layers
///   into one file — nor is one fed by none.
/// - **A layer with no destination of its own has no copy**, and nothing is
///   made or said for it.
/// - **A file the route cannot make a copy of has none either** — a node
///   gathering one layer's passes into one file, whose picture names B19
///   cannot know, or a working format with no view transform read — the same
///   reasons the main copy would have nothing made.
/// - **Only where more than one view layer renders at the frame** is any of
///   this done (B31a's second bullet).
pub fn the_other_layers_own_copies(
    main: &ADerivativesRecord,
    present: &[APresentFileAndItsDestination],
    told: &WhatTheRungsAreToldAboutTheJob,
    picks: &crate::main_output::ThePicks,
    the_view_transform: Option<&ArtistsText>,
    the_view_layers_at_this_frame: &[ArtistsText],
) -> Vec<AnotherLayersCopy> {
    if the_view_layers_at_this_frame.len() < 2 {
        return Vec::new();
    }
    let the_main_layer = main.the_view_layer_its_picture_came_from.as_ref();
    the_view_layers_at_this_frame
        .iter()
        .enumerate()
        .filter(|(_, layer)| Some(*layer) != the_main_layer)
        .filter_map(|(place, layer)| {
            let its_own: Vec<&APresentFileAndItsDestination> = present
                .iter()
                .filter(|one| the_one_view_layer_that_feeds_it(one) == Some(layer))
                .collect();
            let files: Vec<APresentFile> = its_own.iter().map(|one| one.file.clone()).collect();
            let candidates = crate::main_output::the_candidates(&files, told);
            let chosen = crate::main_output::which_file_the_preview_reads(&candidates, picks)?;
            let the_source = its_own.into_iter().find(|one| one.file.path == chosen.path)?;
            let (treatment, the_picture_inside) =
                how_it_is_made(the_source, the_view_transform, the_view_layers_at_this_frame).ok()?;
            Some(AnotherLayersCopy {
                treatment,
                the_picture_inside,
                record: ADerivativesRecord {
                    the_source_file: the_source.file.path.clone(),
                    frame: main.frame,
                    the_view_layer_its_picture_came_from: Some(layer.clone()),
                    it_is_the_main_copy: false,
                    its_place_in_the_frames_set: u32::try_from(place).unwrap_or(u32::MAX),
                    where_it_came_from: None,
                },
            })
        })
        .collect()
}

/// **B26 for the chosen file, and B25's treatment**: the route; for a
/// multi-layer file, the name of the picture inside it (B19); then the view
/// transform the copy is written in. **A multi-layer file is always a working
/// format**, so it always takes the job's own view transform.
pub fn how_it_is_made(
    chosen: &APresentFileAndItsDestination,
    the_view_transform: Option<&ArtistsText>,
    the_view_layers_at_this_frame: &[ArtistsText],
) -> Result<(TheTreatment, Option<ArtistsText>), WhyNothingIsMade> {
    let the_jobs_own = || match the_view_transform {
        Some(named) => Ok(TheTreatment::TheJobsOwnViewTransform(named.as_bytes().to_vec())),
        None => Err(WhyNothingIsMade::TheJobsViewTransformWasNotRead),
    };
    match which_route(chosen.multi_layer) {
        WhichRouteMakesTheCopy::TheBundledImageLibrary => {
            let layer = the_view_layer_the_beauty_is_under(chosen, the_view_layers_at_this_frame)
                .ok_or(WhyNothingIsMade::TheBeautysNameInsideTheFileIsNotKnown)?;
            Ok((the_jobs_own()?, Some(the_beautys_name_inside(layer))))
        }
        WhichRouteMakesTheCopy::BlendersOwnImageHandling => {
            let viewing = chosen.file.format.as_slice() == PNG
                || chosen.file.format.as_slice() == crate::main_output::JPEG;
            if viewing {
                return Ok((TheTreatment::ItAlreadyCarriesItsTreatment, None));
            }
            Ok((the_jobs_own()?, None))
        }
    }
}

/// **One frame's derivative, chosen: the file it is made of, the treatment it
/// is written in, and the record it carries.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhatOneFrameIsMadeOf {
    pub the_source: APresentFileAndItsDestination,
    pub treatment: TheTreatment,
    /// **B19: the picture's name inside a multi-layer source**; absent for a
    /// single-picture one.
    pub the_picture_inside: Option<ArtistsText>,
    pub record: ADerivativesRecord,
    /// **The frame's other View Layers' copies**, in the frame's set's order,
    /// made after the main one: inside the main copy's multi-layer file
    /// ([`the_other_layers_copies`], added 2026-10-10), or otherwise each from
    /// a destination that layer alone feeds
    /// ([`the_other_layers_own_copies`], added 2026-10-16).
    pub the_other_layers: Vec<AnotherLayersCopy>,
}

/// **§8.3 and §8.4 for one frame, over the reading in force**: the present
/// files (B16), the rungs and B20's exclusions (B17 to B20), B18's tie-break
/// with B13's pick read rather than made again (B15), B26's route, B25's
/// treatment and B31a's record — **or why nothing is made**.
///
/// `the_asked_order` and `it_was_interrupted` are what B20's second exclusion
/// is worked out from; `size_of` is the one look at the disk
/// ([`the_present_files_at`]). **`the_jobs_pins` are the job's pins as the
/// render applied them** — the manifest payload's (`Render Settings` V4), the
/// caller's to hand over, since this crate does not read the payload — so the
/// frame's view layers are read with the job's choices (R4 item 5); none reads
/// as no choices.
///
/// **`the_scenes_layers_in_order` are the job's scene's View Layers in
/// Blender's own order**, from the settings half of the reading in force —
/// the caller's to hand over, as the pins are. A layer the job pins on that
/// the file switched off takes its place in that order
/// ([`crate::reading::the_per_frame_set_with_the_jobs_pins_in_the_scenes_order`]),
/// so its copy's place, which the preview's dropdown lists by (`Queue &
/// History` P1, P4), is its place in the scene. None goes at the end, the
/// answer before 2026-10-18 (piece P8b of step 10c).
#[allow(clippy::too_many_arguments)]
pub fn what_one_frame_is_made_of(
    frame: i64,
    reading: &TheOutputsHalf,
    inventory: &crate::inventory::WhatTheInventoryIs,
    picks: &crate::main_output::ThePicks,
    the_jobs_pins: &[crate::reading::AViewLayerTheJobPins],
    the_scenes_layers_in_order: &[ArtistsText],
    the_asked_order: &[i64],
    it_was_interrupted: bool,
    size_of: &dyn Fn(&[u8]) -> Option<u64>,
) -> Result<WhatOneFrameIsMadeOf, WhyNothingIsMade> {
    let crate::inventory::WhatTheInventoryIs::It(inventory) = inventory else {
        return Err(WhyNothingIsMade::NoReadingStands(
            "the job's frame set is not predictable, so no inventory names a file (I9a)"
                .to_string(),
        ));
    };
    let the_main_output = crate::composing::the_main_output_of(reading);
    let present = the_present_files_at(frame, reading, inventory, the_main_output, size_of);
    let last = the_last_frame_asked_for_that_is_present(the_asked_order, it_was_interrupted, &|at| {
        !the_present_files_at(at, reading, inventory, the_main_output, size_of).is_empty()
    });
    let files: Vec<APresentFile> = present.iter().map(|one| one.file.clone()).collect();
    let told = the_rungs_are_told(reading.taken_by, last);
    let candidates = crate::main_output::the_candidates(&files, &told);
    let chosen = crate::main_output::which_file_the_preview_reads(&candidates, picks)
        .ok_or(WhyNothingIsMade::ThereIsNoCandidate)?;
    let the_source = present
        .iter()
        .find(|one| one.file.path == chosen.path)
        .cloned()
        .ok_or(WhyNothingIsMade::ThereIsNoCandidate)?;
    // **R4 item 5: the frame's layers with the job's pins applied**, through
    // the one rule, a layer pinned on at its place in the scene's order — the
    // recorded reading itself stays the file's.
    let the_view_layers_at_this_frame: Vec<ArtistsText> =
        crate::reading::the_per_frame_set_with_the_jobs_pins_in_the_scenes_order(
            &reading.per_frame,
            the_jobs_pins,
            the_scenes_layers_in_order.iter().map(ArtistsText::as_bytes),
        )
        .into_iter()
        .find(|at| at.frame == frame)
        .map(|at| at.view_layers_that_render)
        .unwrap_or_default();
    let (treatment, the_picture_inside) = how_it_is_made(
        &the_source,
        reading.the_view_transform.as_ref(),
        &the_view_layers_at_this_frame,
    )?;
    let record = the_derivatives_record(&the_source, frame, &the_view_layers_at_this_frame);
    // **The multi-layer route's copies come from inside the main copy's file;
    // every other main copy leaves each other layer to its own destination**
    // (B31a's last bullet, 2026-10-16).
    let the_other_layers = if the_picture_inside.is_some() {
        the_other_layers_copies(
            &record,
            &treatment,
            the_picture_inside.as_ref(),
            &the_view_layers_at_this_frame,
        )
    } else {
        the_other_layers_own_copies(
            &record,
            &present,
            &told,
            picks,
            reading.the_view_transform.as_ref(),
            &the_view_layers_at_this_frame,
        )
    };
    Ok(WhatOneFrameIsMadeOf {
        the_source,
        treatment,
        the_picture_inside,
        record,
        the_other_layers,
    })
}

// =============================================================================
// The script and its launch
// =============================================================================

/// **The script one derivative is made by.**
///
/// **Everything it needs rides in one data document at its head**, the shape
/// [`crate::reading_script`] uses, so the same text serves every file and
/// nothing of the user's is written into code. **It says the source's size
/// and waits for the size it is to write**, so B26a's target is
/// [`crate::main_output::the_target_size`]'s and never a second copy of that
/// arithmetic in the script; it is told `w h` on its standard input, and a
/// line that is not two numbers ends it at the failure status.
///
/// **B26 picks the body**: a source with a picture named inside it is a
/// multi-layer file and takes the bundled library's
/// ([`THE_MULTI_LAYER_LOOKUP`]); any other takes Blender's own image handling.
/// Both speak the same size handshake, and a multi-layer file holding no
/// picture by the name asked for says a size of nothing. **Both end in B28's
/// checkerboard** ([`THE_CHECKERBOARD_STEP`]), told its two squares and their
/// size in the same document.
pub fn the_derivatives_script(to_make: &ADerivativeToMake) -> String {
    let data = serde_json::json!({
        "source": to_make.source,
        "picture": to_make.the_picture_inside,
        "staged_at": to_make.staged_at,
        "view_transform": to_make.treatment.the_view_transform(),
        "format": String::from_utf8_lossy(PNG),
        "size_opens": String::from_utf8_lossy(THE_SIZE_OPENS),
        "size_closes": String::from_utf8_lossy(THE_SIZE_CLOSES),
        "made": String::from_utf8_lossy(THE_MADE_MARK),
        "grey": THE_BOARDS_GREY,
        "white": THE_BOARDS_WHITE,
        "squares": SQUARES_ALONG_THE_SHORTER_SIDE,
    });
    // **The document rides as a JSON string literal**, whose escapes are all
    // Python's own, and is read by `json.loads` — so no byte of a path is ever
    // spliced into the script's code.
    format!(
        "import json, os, sys\nimport bpy\n\nDATA = json.loads({})\n{}{}",
        serde_json::to_string(&data.to_string()).expect("a string always renders"),
        THE_CHECKERBOARD_STEP,
        match to_make.the_picture_inside {
            Some(_) => THE_MULTI_LAYER_BODY,
            None => THE_SCRIPTS_BODY,
        }
    )
}

/// **B26's second route, B27 and B25**, every name as [`THE_MULTI_LAYER_LOOKUP`]
/// read it. The picture is found by its channel names in every part (B19a);
/// scaled in the file's own linear space, as Blender's own route scales before
/// its save; then given the view transform; then written as an eight-bit PNG,
/// the format named because the staging path's ending is Smoozy's own.
const THE_MULTI_LAYER_BODY: &str = r#"
import OpenImageIO
scene = bpy.context.scene
source = os.fsdecode(bytes(DATA["source"]))
picture = bytes(DATA["picture"]).decode("utf-8", "replace")
config = os.path.join(bpy.utils.system_resource("DATAFILES", path="colormanagement"), "config.ocio")
the_linear_space = OpenImageIO.ColorConfig(config).getColorSpaceNameByRole("scene_linear")
wanted = [picture + "." + letter for letter in ("R", "G", "B")]
found = None
for part in range(OpenImageIO.ImageBuf(source).nsubimages):
    buf = OpenImageIO.ImageBuf(source, part, 0)
    names = list(buf.spec().channelnames)
    if all(name in names for name in wanted):
        order = [names.index(name) for name in wanted]
        if picture + ".A" in names:
            order.append(names.index(picture + ".A"))
        found = OpenImageIO.ImageBufAlgo.channels(buf, tuple(order))
        break
width, height = (found.spec().width, found.spec().height) if found is not None else (0, 0)
sys.stdout.write(DATA["size_opens"] + "%d %d" % (width, height) + DATA["size_closes"] + "\n")
sys.stdout.flush()
told = sys.stdin.readline().split()
if found is None or len(told) != 2:
    sys.exit(1)
wanted_size = (int(told[0]), int(told[1]))
if wanted_size != (width, height):
    found = OpenImageIO.ImageBufAlgo.resize(found, roi=OpenImageIO.ROI(0, wanted_size[0], 0, wanted_size[1]))
shown = OpenImageIO.ImageBufAlgo.ociodisplay(
    found,
    scene.display_settings.display_device,
    os.fsdecode(bytes(DATA["view_transform"])),
    fromspace=the_linear_space,
    colorconfig=config,
)
if shown.has_error:
    sys.stderr.write(shown.geterror() + "\n")
    sys.exit(1)
flat = laid_over_the_checkerboard(shown)
if not flat.write(DATA["staged_at"], OpenImageIO.TypeUInt8, "png"):
    sys.stderr.write(flat.geterror() + "\n")
    sys.exit(1)
sys.stdout.write(DATA["made"] + "\n")
sys.stdout.flush()
"#;

/// **B28: the step both routes end in.** A copy with four channels — the
/// fourth its alpha, as both routes take them out — is laid over a checkerboard
/// the copy's own size, of [`THE_BOARDS_GREY`] and [`THE_BOARDS_WHITE`]
/// squares a [`SQUARES_ALONG_THE_SHORTER_SIDE`]th of its shorter side, and its
/// alpha dropped; any other copy is handed back as it is. Every name is
/// [`THE_CHECKERBOARD_LOOKUP`]'s. **The copy's channels are first named `R`,
/// `G`, `B` and `A`** with the same `channels` call's third argument, so the
/// library finds its alpha whatever the multi-layer file called it. A step
/// the library reports an error on ends the script at the failure status.
pub const THE_CHECKERBOARD_STEP: &str = r#"
import OpenImageIO


def laid_over_the_checkerboard(picture):
    spec = picture.spec()
    if picture.has_error:
        sys.stderr.write(picture.geterror() + "\n")
        sys.exit(1)
    if len(spec.channelnames) != 4:
        return picture
    width, height = spec.width, spec.height
    side = max(1, min(width, height) // DATA["squares"])
    named = OpenImageIO.ImageBufAlgo.channels(picture, (0, 1, 2, 3), ("R", "G", "B", "A"))
    board = OpenImageIO.ImageBufAlgo.checker(
        side, side, 1, tuple(DATA["grey"]), tuple(DATA["white"]), 0, 0, 0,
        OpenImageIO.ROI(0, width, 0, height, 0, 1, 0, 4),
    )
    laid = OpenImageIO.ImageBufAlgo.over(named, board)
    flat = OpenImageIO.ImageBufAlgo.channels(laid, (0, 1, 2))
    for step in (named, board, laid, flat):
        if step.has_error:
            sys.stderr.write(step.geterror() + "\n")
            sys.exit(1)
    return flat
"#;

const THE_SCRIPTS_BODY: &str = r#"
scene = bpy.context.scene
# `save_render` writes in the scene's own output format, whatever the path's
# ending says (the research's Images question 8).
scene.render.image_settings.file_format = DATA["format"]
scene.view_settings.view_transform = os.fsdecode(bytes(DATA["view_transform"]))
image = bpy.data.images.load(os.fsdecode(bytes(DATA["source"])))
width, height = image.size[0], image.size[1]
sys.stdout.write(DATA["size_opens"] + "%d %d" % (width, height) + DATA["size_closes"] + "\n")
sys.stdout.flush()
told = sys.stdin.readline().split()
if len(told) != 2:
    sys.exit(1)
wanted = (int(told[0]), int(told[1]))
if wanted != (width, height):
    image.scale(wanted[0], wanted[1])
image.save_render(DATA["staged_at"], scene=scene)
# The copy keeps the source's transparency (B28's lookup), so it is read back
# with the bundled library and, where it has an alpha channel, written again
# over the checkerboard.
copy = OpenImageIO.ImageBuf(DATA["staged_at"])
flat = laid_over_the_checkerboard(copy)
if flat is not copy and not flat.write(DATA["staged_at"], OpenImageIO.TypeUInt8, "png"):
    sys.stderr.write(flat.geterror() + "\n")
    sys.exit(1)
sys.stdout.write(DATA["made"] + "\n")
sys.stdout.flush()
"#;

/// **The words after the build's own path**, each handed to the kernel as an
/// argument and never spliced into a line a shell reads.
pub fn the_words_of_a_derivative(the_scripts_path: &str) -> Vec<Vec<u8>> {
    vec![
        b"-b".to_vec(),
        b"--factory-startup".to_vec(),
        b"--python-exit-code".to_vec(),
        b"1".to_vec(),
        b"--python".to_vec(),
        the_scripts_path.as_bytes().to_vec(),
    ]
}

/// **The source's size, where the script has said it** — the first pair
/// between the two size marks.
pub fn the_size_in(stream: &[u8]) -> Option<(u32, u32)> {
    let opens = find(stream, THE_SIZE_OPENS)? + THE_SIZE_OPENS.len();
    let closes = opens + find(&stream[opens..], THE_SIZE_CLOSES)?;
    let text = std::str::from_utf8(&stream[opens..closes]).ok()?;
    let mut parts = text.split_whitespace();
    let width = parts.next()?.parse().ok()?;
    let height = parts.next()?.parse().ok()?;
    parts.next().is_none().then_some((width, height))
}

/// **What the script is told on its standard input**: the target, as two
/// numbers and a line ending.
pub fn the_size_told(target: ATargetSize) -> String {
    format!("{} {}\n", target.width, target.height)
}

/// Whether the script wrote its made mark.
pub fn it_was_made(stream: &[u8]) -> bool {
    find(stream, THE_MADE_MARK).is_some()
}

fn find(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    if needle.len() > haystack.len() {
        return None;
    }
    haystack
        .windows(needle.len())
        .position(|window| window == needle)
}

fn the_folder_of(path: &[u8]) -> &[u8] {
    match path.iter().rposition(|byte| *byte == b'/') {
        Some(at) => &path[..at],
        None => &[],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::inventory::compose_the_inventory;
    use crate::inventory::tests::{a_frame, a_node_destination, a_reading, mapped};
    use crate::main_output::{the_candidates, which_file_the_preview_reads, ThePicks};

    const FIVE_TWO: TheVersionInHand = TheVersionInHand { major: 5, minor: 2 };

    fn the_inventory(reading: &TheOutputsHalf) -> TheInventory {
        match compose_the_inventory(reading, &mapped(reading, FIVE_TWO)) {
            crate::inventory::WhatTheInventoryIs::It(it) => it,
            other => panic!("withheld: {other:?}"),
        }
    }

    /// **B16 to B19 over what the machine holds**: of a frame's two node
    /// files, the one the beauty reached is the beauty rung and is read; a
    /// file not on the disk is no candidate at all; the other frame is not
    /// looked at.
    #[test]
    fn the_beauty_the_walk_reached_is_the_candidate_and_an_absent_file_is_none() {
        let mut reading = a_reading(
            vec![
                a_node_destination("Beauty", "b_", "/home/a/out", None),
                a_node_destination("Passes", "p_", "/home/a/passes", None),
            ],
            vec![a_frame(1, &["VL"], None), a_frame(2, &["VL"], None)],
            FIVE_TWO,
        );
        reading.the_beauty_walk = WhatTheWalkFound::Reached {
            destinations: vec![crate::main_output::ADestinationTheBeautyReached {
                node: ArtistsText::of(b"Beauty".to_vec()),
                output: Some(ArtistsText::of(b"b_".to_vec())),
            }],
            it_reached_the_final_output_node: false,
        };
        let inventory = the_inventory(&reading);
        let every = |_: &[u8]| Some(10);
        let present = the_present_files_at(
            1,
            &reading,
            &inventory,
            WhatTheMainOutputIs::ADestinationInsideAFileOutputNode,
            &every,
        );
        assert_eq!(present.len(), 2, "two files owed at frame 1, both present");
        assert!(present.iter().all(|one| one.file.frame == Some(1)));
        let files: Vec<APresentFile> = present.iter().map(|one| one.file.clone()).collect();
        let candidates = the_candidates(&files, &the_rungs_are_told(FIVE_TWO, None));
        let read = which_file_the_preview_reads(&candidates, &ThePicks::made_once(None, None))
            .expect("a candidate");
        assert!(read.it_sits_in_the_main_outputs_folder);
        assert_eq!(
            read.the_picture_it_holds.as_ref().map(|picture| picture.as_bytes()),
            Some(THE_BEAUTY_PASSES_NAME)
        );
        let none = |_: &[u8]| None;
        assert!(the_present_files_at(
            1,
            &reading,
            &inventory,
            WhatTheMainOutputIs::ADestinationInsideAFileOutputNode,
            &none
        )
        .is_empty());
    }

    /// **B26 and B25**: a multi-layer source takes the bundled library, in the
    /// job's own view transform, its picture named `<view layer>.Combined` by
    /// the first view layer that feeds it, else the frame's first; one a file
    /// output node gathered, or with no view layer, has nothing made; a
    /// working format with no view transform read has nothing made; a viewing
    /// format is written plain; a working format in the job's own.
    #[test]
    fn the_route_and_the_treatment_follow_the_sources_kind() {
        let a = |format: &[u8], multi_layer: bool| APresentFileAndItsDestination {
            file: APresentFile {
                path: b"/j/out/f_0001.x".to_vec(),
                frame: Some(1),
                the_picture_it_holds: None,
                format: format.to_vec(),
                bytes: 10,
                is_video_shaped: false,
                it_sits_in_the_main_outputs_folder: true,
            },
            multi_layer,
            the_destination_it_came_from: WhereThePictureCameFrom::TheOutputPath,
            the_view_layers_that_feed_it: Vec::new(),
        };
        let agx = ArtistsText::of(b"AgX".to_vec());
        let text = |name: &[u8]| ArtistsText::of(name.to_vec());
        let the_frames = [text(b"Main"), text(b"Reflections")];
        let mut fed = a(THE_MULTI_LAYER_FORMAT, true);
        fed.the_view_layers_that_feed_it = vec![text(b"Reflections"), text(b"Main")];
        assert_eq!(
            how_it_is_made(&fed, Some(&agx), &the_frames),
            Ok((
                TheTreatment::TheJobsOwnViewTransform(b"AgX".to_vec()),
                Some(text(b"Reflections.Combined"))
            ))
        );
        assert_eq!(
            how_it_is_made(&a(THE_MULTI_LAYER_FORMAT, true), Some(&agx), &the_frames),
            Ok((
                TheTreatment::TheJobsOwnViewTransform(b"AgX".to_vec()),
                Some(text(b"Main.Combined"))
            )),
            "no layer recorded on the destination: the frame's first"
        );
        assert_eq!(
            how_it_is_made(&a(THE_MULTI_LAYER_FORMAT, true), None, &the_frames),
            Err(WhyNothingIsMade::TheJobsViewTransformWasNotRead)
        );
        assert_eq!(
            how_it_is_made(&a(THE_MULTI_LAYER_FORMAT, true), Some(&agx), &[]),
            Err(WhyNothingIsMade::TheBeautysNameInsideTheFileIsNotKnown)
        );
        let mut gathered = fed.clone();
        gathered.the_destination_it_came_from = WhereThePictureCameFrom::AFileOutputNode(text(b"Gather"));
        assert_eq!(
            how_it_is_made(&gathered, Some(&agx), &the_frames),
            Err(WhyNothingIsMade::TheBeautysNameInsideTheFileIsNotKnown)
        );
        assert_eq!(
            how_it_is_made(&a(b"OPEN_EXR", false), None, &the_frames),
            Err(WhyNothingIsMade::TheJobsViewTransformWasNotRead)
        );
        assert_eq!(
            how_it_is_made(&a(b"OPEN_EXR", false), Some(&agx), &the_frames),
            Ok((TheTreatment::TheJobsOwnViewTransform(b"AgX".to_vec()), None))
        );
        assert_eq!(
            how_it_is_made(&a(PNG, false), None, &the_frames),
            Ok((TheTreatment::ItAlreadyCarriesItsTreatment, None))
        );
        assert_eq!(
            TheTreatment::ItAlreadyCarriesItsTreatment.the_view_transform(),
            THE_PLAIN_VIEW_TRANSFORM
        );
        assert!(it_writes_multi_layer_files(
            &DestinationKind::TheRegularOutputPath,
            b"OPEN_EXR_MULTILAYER"
        ));
        assert!(!it_writes_multi_layer_files(&DestinationKind::TheRegularOutputPath, b"OPEN_EXR"));
        assert!(it_writes_multi_layer_files(
            &DestinationKind::AWholeFileOutputNodeGatheringIntoOneFile {
                node: text(b"File Output"),
                the_nodes_base_file_name: None,
                replaced: None,
            },
            b"OPEN_EXR_MULTILAYER"
        ));
    }

    /// **The script's data and the size handshake**: the data document
    /// carries the source as bytes, the staging path and the view transform;
    /// the size is read back from between the marks, and anything else there
    /// is no size.
    #[test]
    fn the_script_carries_its_data_and_the_size_is_read_from_between_the_marks() {
        let script = the_derivatives_script(&ADerivativeToMake {
            source: vec![b'/', 0xff, b'.', b'e'],
            staged_at: "/r/job/staging-preview-1.png".to_string(),
            treatment: TheTreatment::TheJobsOwnViewTransform(b"AgX".to_vec()),
            the_picture_inside: None,
        });
        let head = script.lines().find(|line| line.starts_with("DATA = ")).expect("the data");
        let quoted: String =
            serde_json::from_str(head.trim_start_matches("DATA = json.loads(").trim_end_matches(')'))
                .expect("a string");
        let data: serde_json::Value = serde_json::from_str(&quoted).expect("a document");
        assert_eq!(data["source"], serde_json::json!([47, 255, 46, 101]));
        assert_eq!(data["staged_at"], "/r/job/staging-preview-1.png");
        assert_eq!(data["view_transform"], serde_json::json!(b"AgX".to_vec()));
        assert_eq!(data["format"], "PNG");
        assert!(script.contains("image.save_render(DATA[\"staged_at\"], scene=scene)"));
        // **Blender's own loading, never the library's** — the library only
        // reads the copy back for B28's checkerboard.
        assert!(!script.contains("nsubimages"));
        assert!(!script.contains("ociodisplay"));
        assert_eq!(data["picture"], serde_json::Value::Null);

        // **A multi-layer source takes the bundled library**, told the
        // picture's name as bytes, and never Blender's own loading.
        let multi = the_derivatives_script(&ADerivativeToMake {
            source: b"/j/out/0001.exr".to_vec(),
            staged_at: "/r/job/staging-preview-1.png".to_string(),
            treatment: TheTreatment::TheJobsOwnViewTransform(b"AgX".to_vec()),
            the_picture_inside: Some(b"Reflections.Combined".to_vec()),
        });
        let head = multi.lines().find(|line| line.starts_with("DATA = ")).expect("the data");
        let quoted: String =
            serde_json::from_str(head.trim_start_matches("DATA = json.loads(").trim_end_matches(')'))
                .expect("a string");
        let data: serde_json::Value = serde_json::from_str(&quoted).expect("a document");
        assert_eq!(data["picture"], serde_json::json!(b"Reflections.Combined".to_vec()));
        for name in [
            "import OpenImageIO",
            "bpy.utils.system_resource(\"DATAFILES\", path=\"colormanagement\")",
            "getColorSpaceNameByRole(\"scene_linear\")",
            "OpenImageIO.ImageBufAlgo.channels(",
            "OpenImageIO.ImageBufAlgo.resize(",
            "OpenImageIO.ImageBufAlgo.ociodisplay(",
            "colorconfig=config",
            "OpenImageIO.TypeUInt8, \"png\"",
        ] {
            assert!(multi.contains(name), "{name}");
        }
        assert!(!multi.contains("bpy.data.images.load"));
        assert_eq!(the_size_in(b"chatter <<<SMOOZY-SIZE>>>1920 1080<<<END-SMOOZY-SIZE>>>\n"), Some((1920, 1080)));
        assert_eq!(the_size_in(b"<<<SMOOZY-SIZE>>>1920<<<END-SMOOZY-SIZE>>>"), None);
        assert_eq!(the_size_in(b"<<<SMOOZY-SIZE>>>1920 1080"), None);
        assert_eq!(
            the_size_told(crate::main_output::the_target_size(9000, 3000)),
            "7500 2500\n"
        );
        assert!(it_was_made(b"x <<<SMOOZY-DERIVATIVE-MADE>>>\n"));
        assert_eq!(
            the_words_of_a_derivative("/p/derivative.py").last().map(Vec::as_slice),
            Some(b"/p/derivative.py".as_slice())
        );
    }

    /// The data document at the head of a composed script.
    fn the_data_of(script: &str) -> serde_json::Value {
        let head = script.lines().find(|line| line.starts_with("DATA = ")).expect("the data");
        let quoted: String =
            serde_json::from_str(head.trim_start_matches("DATA = json.loads(").trim_end_matches(')'))
                .expect("a string");
        serde_json::from_str(&quoted).expect("a document")
    }

    /// Where `needle` first sits in `script`, or a failure naming it.
    fn at(script: &str, needle: &str) -> usize {
        script.find(needle).unwrap_or_else(|| panic!("not in the script: {needle}"))
    }

    /// **B28, the Creator's tenth notes, note 7: both routes end in the
    /// checkerboard.** The step is in each script once, defined before
    /// either body calls it, and told its two squares and their size in the
    /// data: light grey `#cccccc` in the top-left square and white, each
    /// opaque, a 24th of the shorter side. **It lays the picture on top of the
    /// board** (`over`'s first argument), **the board has four channels**
    /// (without an alpha, `over` refuses), **the alpha is dropped** before the
    /// write, and **a copy with no alpha channel is handed back as it is**.
    /// The multi-layer route lays its picture over after the view transform
    /// and writes the flattened one; the single-picture route reads back the
    /// copy Blender saved and writes it again, flattened.
    ///
    /// **What it bites on**: the step left out of either route, the picture
    /// and the board swapped, a board with three channels, the alpha kept, the
    /// view-transformed picture written in place of the flattened one, and
    /// the wrong grey.
    #[test]
    fn tenth_notes_7_both_routes_lay_a_transparent_copy_over_the_checkerboard() {
        let single = the_derivatives_script(&ADerivativeToMake {
            source: b"/j/out/f_0001.png".to_vec(),
            staged_at: "/r/job/staging-preview-1.png".to_string(),
            treatment: TheTreatment::ItAlreadyCarriesItsTreatment,
            the_picture_inside: None,
        });
        let multi = the_derivatives_script(&ADerivativeToMake {
            source: b"/j/out/f_0001.exr".to_vec(),
            staged_at: "/r/job/staging-preview-1.png".to_string(),
            treatment: TheTreatment::TheJobsOwnViewTransform(b"AgX".to_vec()),
            the_picture_inside: Some(b"ViewLayer.Combined".to_vec()),
        });

        // **The squares**, as the data tells them.
        assert_eq!((THE_BOARDS_GREY[0] * 255.0).round() as u32, 0xcc, "#cccccc");
        assert_eq!(THE_BOARDS_GREY[0], THE_BOARDS_GREY[1]);
        assert_eq!(THE_BOARDS_GREY[1], THE_BOARDS_GREY[2]);
        assert_eq!((THE_BOARDS_WHITE[0] * 255.0).round() as u32, 0xff, "#ffffff");
        assert_eq!(THE_BOARDS_GREY[3], 1.0, "the board carries an opaque alpha");
        assert_eq!(THE_BOARDS_WHITE[3], 1.0, "the board carries an opaque alpha");
        assert_eq!(SQUARES_ALONG_THE_SHORTER_SIDE, 24);
        for script in [&single, &multi] {
            let data = the_data_of(script);
            assert_eq!(data["grey"], serde_json::json!(THE_BOARDS_GREY));
            assert_eq!(data["white"], serde_json::json!(THE_BOARDS_WHITE));
            assert_eq!(data["squares"], serde_json::json!(24));
            assert_eq!(script.matches(THE_CHECKERBOARD_STEP).count(), 1);
            assert_eq!(script.matches("def laid_over_the_checkerboard(").count(), 1);
        }

        // **The step itself**, every name the lookup's.
        let step = THE_CHECKERBOARD_STEP;
        for name in [
            "if len(spec.channelnames) != 4:\n        return picture",
            "side = max(1, min(width, height) // DATA[\"squares\"])",
            "OpenImageIO.ImageBufAlgo.channels(picture, (0, 1, 2, 3), (\"R\", \"G\", \"B\", \"A\"))",
            "OpenImageIO.ImageBufAlgo.checker(\n        side, side, 1, tuple(DATA[\"grey\"]), tuple(DATA[\"white\"]), 0, 0, 0,",
            "OpenImageIO.ROI(0, width, 0, height, 0, 1, 0, 4)",
            "laid = OpenImageIO.ImageBufAlgo.over(named, board)",
            "flat = OpenImageIO.ImageBufAlgo.channels(laid, (0, 1, 2))",
            "for step in (named, board, laid, flat):",
            "return flat",
        ] {
            assert!(step.contains(name), "{name}");
        }
        assert!(at(step, "def laid_over_the_checkerboard(") < at(step, "return flat"));

        // **The multi-layer route**: after the view transform, before the
        // write, and the flattened picture is what is written.
        let defined = at(&multi, "def laid_over_the_checkerboard(");
        let shown = at(&multi, "OpenImageIO.ImageBufAlgo.ociodisplay(");
        let laid = at(&multi, "flat = laid_over_the_checkerboard(shown)");
        let written = at(&multi, "flat.write(DATA[\"staged_at\"], OpenImageIO.TypeUInt8, \"png\")");
        assert!(defined < shown && shown < laid && laid < written);
        assert!(!multi.contains("shown.write("), "the view-transformed picture is never written");

        // **The single-picture route**: Blender's copy read back from the
        // staging path, laid over, and written again where it was.
        let defined = at(&single, "def laid_over_the_checkerboard(");
        let saved = at(&single, "image.save_render(DATA[\"staged_at\"], scene=scene)");
        let read_back = at(&single, "copy = OpenImageIO.ImageBuf(DATA[\"staged_at\"])");
        let laid = at(&single, "flat = laid_over_the_checkerboard(copy)");
        let written = at(
            &single,
            "flat is not copy and not flat.write(DATA[\"staged_at\"], OpenImageIO.TypeUInt8, \"png\")",
        );
        assert!(defined < saved && saved < read_back && read_back < laid && laid < written);
        assert!(written < at(&single, "sys.stdout.write(DATA[\"made\"]"), "made only once flattened");
    }

    /// **B20's second exclusion** is worked out only for an interrupted job.
    #[test]
    fn the_last_frame_asked_for_that_is_present_is_only_an_interrupted_jobs() {
        let present = |frame: i64| frame <= 3;
        assert_eq!(the_last_frame_asked_for_that_is_present(&[1, 2, 3, 4], true, &present), Some(3));
        assert_eq!(the_last_frame_asked_for_that_is_present(&[1, 2, 3, 4], false, &present), None);
    }

    /// **B19's view layer is read with the job's choices** (R4 item 5, piece
    /// 1b, 2026-10-04): a regular output path's multi-layer file is fed by
    /// every layer the file switches on, and a layer the job pinned off is
    /// not in the file — so the beauty is looked for under the next. **No
    /// pins — a record written before they were carried — gives today's
    /// answer**, the first.
    #[test]
    fn the_beautys_view_layer_is_one_the_job_renders_and_no_pins_give_todays_answer() {
        let mut regular = crate::reading::tests::a_destination(
            DestinationKind::TheRegularOutputPath,
            "/home/a/out/f_",
            crate::reading::WhereItComesFrom::TheScenesOutputPath,
        );
        regular.format = Some(crate::reading::Format {
            as_the_file_records_it: ArtistsText::of(THE_MULTI_LAYER_FORMAT.to_vec()),
            the_ending_it_writes: Some(ArtistsText::of(b".exr".to_vec())),
            is_video_shaped: false,
        });
        regular.the_view_layers_that_feed_it =
            vec![ArtistsText::of(b"Box".to_vec()), ArtistsText::of(b"Mattress".to_vec())];
        let mut reading = a_reading(
            vec![regular],
            vec![a_frame(1, &["Box", "Mattress"], Some("/home/a/out/f_0001.exr"))],
            FIVE_TWO,
        );
        reading.the_view_transform = Some(ArtistsText::of(b"AgX".to_vec()));
        let inventory = crate::inventory::WhatTheInventoryIs::It(the_inventory(&reading));
        let every = |_: &[u8]| Some(10);
        let picks = ThePicks::made_once(None, None);
        let inside = |pins: &[crate::reading::AViewLayerTheJobPins]| {
            what_one_frame_is_made_of(1, &reading, &inventory, &picks, pins, &[], &[1], false, &every)
                .expect("a frame is made")
                .the_picture_inside
                .map(|name| name.as_bytes().to_vec())
        };
        assert_eq!(inside(&[]), Some(b"Box.Combined".to_vec()));
        let box_off = [crate::reading::AViewLayerTheJobPins {
            layer: ArtistsText::of(b"Box".to_vec()),
            on: false,
        }];
        assert_eq!(inside(&box_off), Some(b"Mattress.Combined".to_vec()));
    }

    /// **`Queue & History` P1's and this document's B31a's new bullets (the Creator's seventh
    /// notes, note 4): a two-layer multi-layer frame owes two copies, the main
    /// one first and marked; a frame where one layer did not render owes one;
    /// an ordinary picture owes one.** Each other copy is `<layer>.Combined` in
    /// the same file, carries its layer's name, no main mark and its place,
    /// and is written under its place, never its name.
    #[test]
    fn a_multi_layer_frame_owes_one_copy_per_view_layer_the_main_one_first() {
        let mut regular = crate::reading::tests::a_destination(
            DestinationKind::TheRegularOutputPath,
            "/home/a/out/f_",
            crate::reading::WhereItComesFrom::TheScenesOutputPath,
        );
        regular.format = Some(crate::reading::Format {
            as_the_file_records_it: ArtistsText::of(THE_MULTI_LAYER_FORMAT.to_vec()),
            the_ending_it_writes: Some(ArtistsText::of(b".exr".to_vec())),
            is_video_shaped: false,
        });
        regular.the_view_layers_that_feed_it =
            vec![ArtistsText::of(b"Box".to_vec()), ArtistsText::of(b"Mattress".to_vec())];
        let mut reading = a_reading(
            vec![regular],
            vec![
                a_frame(1, &["Box", "Mattress"], Some("/home/a/out/f_0001.exr")),
                a_frame(2, &["Box"], Some("/home/a/out/f_0002.exr")),
            ],
            FIVE_TWO,
        );
        reading.the_view_transform = Some(ArtistsText::of(b"AgX".to_vec()));
        let inventory = crate::inventory::WhatTheInventoryIs::It(the_inventory(&reading));
        let every = |_: &[u8]| Some(10);
        let picks = ThePicks::made_once(None, None);
        let made = |frame: i64| {
            what_one_frame_is_made_of(frame, &reading, &inventory, &picks, &[], &[], &[1, 2], false, &every)
                .expect("a frame is made")
        };

        let first = made(1);
        assert!(first.record.it_is_the_main_copy);
        assert_eq!(first.record.its_place_in_the_frames_set, 0);
        assert_eq!(
            first.record.the_view_layer_its_picture_came_from.as_ref().map(|name| name.as_bytes().to_vec()),
            Some(b"Box".to_vec())
        );
        assert_eq!(first.the_other_layers.len(), 1, "two layers, two copies");
        let other = &first.the_other_layers[0];
        assert_eq!(
            other.the_picture_inside.as_ref().map(|name| name.as_bytes().to_vec()),
            Some(b"Mattress.Combined".to_vec())
        );
        assert_eq!(other.treatment, first.treatment, "the same file, the same treatment");
        assert!(!other.record.it_is_the_main_copy);
        assert_eq!(other.record.its_place_in_the_frames_set, 1);
        assert_eq!(other.record.the_source_file, first.record.the_source_file);
        assert_eq!(
            other.record.the_view_layer_its_picture_came_from.as_ref().map(|name| name.as_bytes().to_vec()),
            Some(b"Mattress".to_vec())
        );

        // Frame 2 rendered one layer: one copy, unnamed (B31a), nothing beside it.
        let second = made(2);
        assert!(second.the_other_layers.is_empty());
        assert!(second.record.it_is_the_main_copy);

        // The main copy keeps the frame's own name; the other is at its place.
        let main_at = crate::main_output::where_this_copy_is_written_in("/r", 1, true, 0);
        let other_at = crate::main_output::where_this_copy_is_written_in("/r", 1, false, 1);
        assert_eq!(main_at.published_at, "/r/preview-1.png");
        assert_eq!(other_at.published_at, "/r/preview-1-layer-1.png");
        assert_ne!(main_at.staged_at, other_at.staged_at);

        // An ordinary picture is one copy, whatever the frame rendered.
        assert!(the_other_layers_copies(&first.record, &first.treatment, None, &[
            ArtistsText::of(b"Box".to_vec()),
            ArtistsText::of(b"Mattress".to_vec()),
        ])
        .is_empty());

        // A record written before the layers reads as the frame's main copy.
        let old: ADerivativesRecord =
            serde_json::from_str(r#"{"the_source_file":[47],"frame":3,"the_view_layer_its_picture_came_from":null}"#)
                .expect("an older record reads");
        assert!(old.it_is_the_main_copy);
        assert_eq!(old.its_place_in_the_frames_set, 0);
    }

    /// **Piece P8b of step 10c: a layer the job pins on that the file switched
    /// off takes its place in the scene's order in the preview's copies.**
    /// Five layers, made out of alphabetical order, to one multi-layer file at
    /// the regular output path; the third, *Kitchen*, is switched off in the
    /// file and pinned on. The main copy is the first layer's at place nought,
    /// and the other four follow at places one to four in the scene's own
    /// order, *Kitchen* third. **With no order handed in, the answer before
    /// P8b stands**: *Kitchen* last, and the others in the file's order.
    #[test]
    fn p8b_a_layer_pinned_on_takes_its_place_among_the_frames_copies() {
        let scene = ["Mid", "Aardvark", "Kitchen", "Alpha", "Beta"];
        let the_files_own = ["Mid", "Aardvark", "Alpha", "Beta"];
        let mut regular = crate::reading::tests::a_destination(
            DestinationKind::TheRegularOutputPath,
            "/home/a/out/f_",
            crate::reading::WhereItComesFrom::TheScenesOutputPath,
        );
        regular.format = Some(crate::reading::Format {
            as_the_file_records_it: ArtistsText::of(THE_MULTI_LAYER_FORMAT.to_vec()),
            the_ending_it_writes: Some(ArtistsText::of(b".exr".to_vec())),
            is_video_shaped: false,
        });
        regular.the_view_layers_that_feed_it =
            the_files_own.iter().map(|name| ArtistsText::of(name.as_bytes().to_vec())).collect();
        let mut reading = a_reading(
            vec![regular],
            vec![a_frame(1, &the_files_own, Some("/home/a/out/f_0001.exr"))],
            FIVE_TWO,
        );
        reading.the_view_transform = Some(ArtistsText::of(b"AgX".to_vec()));
        let inventory = crate::inventory::WhatTheInventoryIs::It(the_inventory(&reading));
        let every = |_: &[u8]| Some(10);
        let pinned_on = [crate::reading::AViewLayerTheJobPins {
            layer: ArtistsText::of(b"Kitchen".to_vec()),
            on: true,
        }];
        let in_order: Vec<ArtistsText> =
            scene.iter().map(|name| ArtistsText::of(name.as_bytes().to_vec())).collect();
        let the_copies = |order: &[ArtistsText]| {
            let made = what_one_frame_is_made_of(
                1,
                &reading,
                &inventory,
                &ThePicks::made_once(None, None),
                &pinned_on,
                order,
                &[1],
                false,
                &every,
            )
            .expect("a frame is made");
            assert!(made.record.it_is_the_main_copy);
            std::iter::once(&made.record)
                .chain(made.the_other_layers.iter().map(|other| &other.record))
                .map(|record| {
                    (
                        String::from_utf8_lossy(
                            record.the_view_layer_its_picture_came_from.as_ref().expect("named").as_bytes(),
                        )
                        .to_string(),
                        record.its_place_in_the_frames_set,
                    )
                })
                .collect::<Vec<_>>()
        };
        let places = |names: &[&str]| {
            names.iter().enumerate().map(|(at, name)| (name.to_string(), at as u32)).collect::<Vec<_>>()
        };
        assert_eq!(the_copies(&in_order), places(&scene), "the scene's order, Kitchen third");
        assert_eq!(
            the_copies(&[]),
            places(&["Mid", "Aardvark", "Alpha", "Beta", "Kitchen"]),
            "no order handed in: the answer before P8b"
        );
    }

    /// **B31a**: a name only where more than one view layer renders at the
    /// frame, the first one's for the regular output path, and none for a
    /// picture a file output node fed.
    #[test]
    fn the_record_names_a_view_layer_only_where_there_is_one_to_tell_apart() {
        let regular = APresentFileAndItsDestination {
            file: APresentFile {
                path: b"/j/out/f_0001.png".to_vec(),
                frame: Some(1),
                the_picture_it_holds: Some(ArtistsText::of(THE_BEAUTY_PASSES_NAME.to_vec())),
                format: PNG.to_vec(),
                bytes: 10,
                is_video_shaped: false,
                it_sits_in_the_main_outputs_folder: true,
            },
            multi_layer: false,
            the_destination_it_came_from: WhereThePictureCameFrom::TheOutputPath,
            the_view_layers_that_feed_it: vec![ArtistsText::of(b"Reflections".to_vec())],
        };
        let two = [ArtistsText::of(b"Reflections".to_vec()), ArtistsText::of(b"Main".to_vec())];
        // **The regular output path's ordinary image names no view layer in a
        // frame of two, and records its source** (the Creator's tenth notes,
        // note 2): until 2026-10-17 it was named by the first layer feeding it.
        let unnamed = the_derivatives_record(&regular, 1, &two);
        assert_eq!(unnamed.the_view_layer_its_picture_came_from, None);
        assert_eq!(unnamed.where_it_came_from, Some(WhereThePictureCameFrom::TheOutputPath));
        assert_eq!(unnamed.its_place_in_the_frames_set, 0);
        assert_eq!(unnamed.the_source_file, regular.file.path);
        // **A frame of one layer: no name, and no source either.**
        let alone = the_derivatives_record(&regular, 1, &two[..1]);
        assert_eq!(alone.the_view_layer_its_picture_came_from, None);
        assert_eq!(alone.where_it_came_from, None);
        // **A file output node fed by one view layer names it** (B31a's first
        // bullet, changed 2026-10-12, the Creator's ninth notes, note 2); fed
        // by two, or by one the frame does not render, it names none.
        let mut from_a_node = regular.clone();
        from_a_node.the_destination_it_came_from =
            WhereThePictureCameFrom::AFileOutputNode(ArtistsText::of(b"Beauty".to_vec()));
        from_a_node.file.the_picture_it_holds = Some(ArtistsText::of(b"Image".to_vec()));
        let from_it = the_derivatives_record(&from_a_node, 1, &two);
        assert_eq!(
            from_it.the_view_layer_its_picture_came_from.map(|name| name.as_bytes().to_vec()),
            Some(b"Reflections".to_vec())
        );
        assert_eq!(from_it.where_it_came_from, None, "a named copy records no source");
        assert_eq!(from_it.its_place_in_the_frames_set, 0);
        assert_eq!(the_derivatives_record(&from_a_node, 1, &two[..1]).the_view_layer_its_picture_came_from, None);
        let mut fed_by_main = from_a_node.clone();
        fed_by_main.the_view_layers_that_feed_it = vec![ArtistsText::of(b"Main".to_vec())];
        assert_eq!(the_derivatives_record(&fed_by_main, 1, &two).its_place_in_the_frames_set, 1);
        let mut fed_by_both = from_a_node.clone();
        fed_by_both.the_view_layers_that_feed_it = two.to_vec();
        let by_both = the_derivatives_record(&fed_by_both, 1, &two);
        assert_eq!(by_both.the_view_layer_its_picture_came_from, None);
        // **Fed by two, it is named by its node** (the Creator's tenth notes,
        // note 2).
        assert_eq!(
            by_both.where_it_came_from,
            Some(WhereThePictureCameFrom::AFileOutputNode(ArtistsText::of(b"Beauty".to_vec())))
        );
        let mut fed_by_another = from_a_node.clone();
        fed_by_another.the_view_layers_that_feed_it = vec![ArtistsText::of(b"Sky".to_vec())];
        assert_eq!(the_derivatives_record(&fed_by_another, 1, &two).the_view_layer_its_picture_came_from, None);

        // **Inside a multi-layer file the record carries the view layer the
        // beauty was found under** — the same name the script was told.
        let mut multi = regular.clone();
        multi.multi_layer = true;
        multi.file.format = THE_MULTI_LAYER_FORMAT.to_vec();
        multi.the_view_layers_that_feed_it =
            vec![ArtistsText::of(b"Main".to_vec()), ArtistsText::of(b"Reflections".to_vec())];
        assert_eq!(
            the_derivatives_record(&multi, 1, &two)
                .the_view_layer_its_picture_came_from
                .map(|name| name.as_bytes().to_vec()),
            Some(b"Main".to_vec())
        );
    }

    // =========================================================================
    // B31a's last bullet: each other view layer's copy from its own destination
    // (the Creator's ninth notes, note 2, 2026-10-11)
    // =========================================================================

    /// The Creator's shape: the regular output path is a video, fed by every
    /// layer, and each layer's beauty goes to its own file output node as a
    /// PNG. `extra` adds destinations, each with the layers feeding it.
    fn a_node_fed_job(
        the_frames_layers: &[&str],
        nodes: &[(&str, &str, &[&str])],
    ) -> (TheOutputsHalf, crate::inventory::WhatTheInventoryIs) {
        let mut video = crate::reading::tests::a_destination(
            DestinationKind::TheRegularOutputPath,
            "/home/a/video/film_",
            crate::reading::WhereItComesFrom::TheScenesOutputPath,
        );
        video.format = Some(crate::reading::Format {
            as_the_file_records_it: ArtistsText::of(b"FFMPEG".to_vec()),
            the_ending_it_writes: Some(ArtistsText::of(b".mp4".to_vec())),
            is_video_shaped: true,
        });
        video.the_view_layers_that_feed_it =
            the_frames_layers.iter().map(|name| ArtistsText::of(name.as_bytes().to_vec())).collect();
        let mut destinations = vec![video];
        for (node, at, fed_by) in nodes {
            let mut one = a_node_destination(node, "Image", at, None);
            one.the_view_layers_that_feed_it =
                fed_by.iter().map(|name| ArtistsText::of(name.as_bytes().to_vec())).collect();
            destinations.push(one);
        }
        let mut reading = a_reading(
            destinations,
            vec![a_frame(1, the_frames_layers, None), a_frame(2, the_frames_layers, None)],
            FIVE_TWO,
        );
        // **The walk follows one view layer's beauty** (B1), so it reaches the
        // first node alone; the others' pictures are named after their output.
        reading.the_beauty_walk = WhatTheWalkFound::Reached {
            destinations: vec![crate::main_output::ADestinationTheBeautyReached {
                node: ArtistsText::of(nodes[0].0.as_bytes().to_vec()),
                output: Some(ArtistsText::of(b"Image".to_vec())),
            }],
            it_reached_the_final_output_node: false,
        };
        reading.the_view_transform = Some(ArtistsText::of(b"AgX".to_vec()));
        let inventory = crate::inventory::WhatTheInventoryIs::It(the_inventory(&reading));
        (reading, inventory)
    }

    fn named(record: &ADerivativesRecord) -> Option<Vec<u8>> {
        record.the_view_layer_its_picture_came_from.as_ref().map(|name| name.as_bytes().to_vec())
    }

    fn the_frame(
        frame: i64,
        reading: &TheOutputsHalf,
        inventory: &crate::inventory::WhatTheInventoryIs,
    ) -> WhatOneFrameIsMadeOf {
        let every = |_: &[u8]| Some(10);
        what_one_frame_is_made_of(
            frame,
            reading,
            inventory,
            &ThePicks::made_once(None, None),
            &[],
            &[],
            &[1, 2],
            false,
            &every,
        )
        .expect("a frame is made")
    }

    /// **A two-layer file, each layer's beauty to its own file output node as
    /// a PNG, gives one copy per layer per frame, each named**: the main copy
    /// from the first node names its one layer, and the other layer's copy is
    /// made from the file its own node wrote — not the main copy's.
    #[test]
    fn each_layers_own_file_output_node_gives_it_a_copy_each_named() {
        let (reading, inventory) = a_node_fed_job(
            &["Box", "Mattress"],
            &[("Box", "/home/a/box", &["Box"]), ("Mattress", "/home/a/mattress", &["Mattress"])],
        );
        for frame in [1, 2] {
            let made = the_frame(frame, &reading, &inventory);
            assert!(made.record.it_is_the_main_copy);
            assert_eq!(named(&made.record), Some(b"Box".to_vec()), "frame {frame}");
            assert_eq!(made.record.its_place_in_the_frames_set, 0);
            assert!(made.record.the_source_file.windows(5).any(|at| at == b"/box/"));
            assert_eq!(made.the_other_layers.len(), 1, "frame {frame}: two layers, two copies");
            let other = &made.the_other_layers[0];
            assert!(!other.record.it_is_the_main_copy);
            assert_eq!(other.record.frame, frame);
            assert_eq!(named(&other.record), Some(b"Mattress".to_vec()));
            assert_eq!(other.record.its_place_in_the_frames_set, 1);
            assert!(
                other.record.the_source_file.windows(10).any(|at| at == b"/mattress/"),
                "{:?}",
                String::from_utf8_lossy(&other.record.the_source_file)
            );
            // A PNG already carries its treatment, and is loaded whole.
            assert_eq!(other.treatment, TheTreatment::ItAlreadyCarriesItsTreatment);
            assert_eq!(other.the_picture_inside, None);
        }
        // **Nothing on the disk for the second node: no copy for its layer.**
        let only_the_box = |path: &[u8]| path.windows(5).any(|at| at == b"/box/").then_some(10);
        let made = what_one_frame_is_made_of(
            1,
            &reading,
            &inventory,
            &ThePicks::made_once(None, None),
            &[],
            &[],
            &[1, 2],
            false,
            &only_the_box,
        )
        .expect("a frame is made");
        assert!(made.the_other_layers.is_empty());
        // **The job pinned the second layer off: the frame has one layer, and
        // its copy carries no name** (B31a's second bullet).
        let off = [crate::reading::AViewLayerTheJobPins {
            layer: ArtistsText::of(b"Mattress".to_vec()),
            on: false,
        }];
        let every = |_: &[u8]| Some(10);
        let made = what_one_frame_is_made_of(
            1,
            &reading,
            &inventory,
            &ThePicks::made_once(None, None),
            &off,
            &[],
            &[1, 2],
            false,
            &every,
        )
        .expect("a frame is made");
        assert!(made.the_other_layers.is_empty());
        assert_eq!(named(&made.record), None);
    }

    /// **The Creator's shape** (the tenth notes, note 2): the regular output
    /// path writes PNG frames and is fed by every layer, and each layer's
    /// beauty also goes to a File Output node of its own. `nodes` names the
    /// layers that have one; the walk reaches the final output node, so the
    /// output path's file is the beauty and the main copy's source.
    fn an_output_path_and_a_node_per_layer(the_frames_layers: &[&str], nodes: &[&str]) -> (TheOutputsHalf, crate::inventory::WhatTheInventoryIs) {
        let mut regular = crate::reading::tests::a_destination(
            DestinationKind::TheRegularOutputPath,
            "/home/a/out/f_",
            crate::reading::WhereItComesFrom::TheScenesOutputPath,
        );
        regular.the_view_layers_that_feed_it =
            the_frames_layers.iter().map(|name| ArtistsText::of(name.as_bytes().to_vec())).collect();
        let mut destinations = vec![regular];
        for node in nodes {
            let mut one = a_node_destination(node, "Image", &format!("/home/a/{}", node.to_lowercase()), None);
            one.the_view_layers_that_feed_it = vec![ArtistsText::of(node.as_bytes().to_vec())];
            destinations.push(one);
        }
        let mut reading = a_reading(
            destinations,
            vec![
                a_frame(1, the_frames_layers, Some("/home/a/out/f_0001.png")),
                a_frame(2, the_frames_layers, Some("/home/a/out/f_0002.png")),
            ],
            FIVE_TWO,
        );
        reading.the_beauty_walk =
            WhatTheWalkFound::Reached { destinations: Vec::new(), it_reached_the_final_output_node: true };
        let inventory = crate::inventory::WhatTheInventoryIs::It(the_inventory(&reading));
        (reading, inventory)
    }

    /// **The Creator's shape gives a main copy marked *Output path* and a
    /// named copy per layer** (B31a's last bullet and its dated note on the
    /// first, the Creator's tenth notes, note 2): the output path's file names
    /// no layer and records its source; every layer's own node gives it a
    /// copy, the first layer's included, each at its own place. **One node
    /// beside the output path gives the main copy and that one layer's.** A
    /// frame of one layer is unchanged: one copy, unnamed, no source.
    ///
    /// **What it bites on**: the output path's file named by the first layer,
    /// which then got no copy of its own; and a main copy with no source.
    #[test]
    fn tenth_notes_2_the_output_path_is_the_main_copys_source_and_each_layer_has_its_own() {
        let (reading, inventory) =
            an_output_path_and_a_node_per_layer(&["Box", "Mattress"], &["Box", "Mattress"]);
        let made = the_frame(1, &reading, &inventory);
        assert!(made.record.it_is_the_main_copy);
        assert!(made.record.the_source_file.windows(5).any(|at| at == b"/out/"), "the output path's file");
        assert_eq!(named(&made.record), None, "the output path's picture is no one layer's");
        assert_eq!(made.record.where_it_came_from, Some(WhereThePictureCameFrom::TheOutputPath));
        assert_eq!(made.record.its_place_in_the_frames_set, 0);
        let others: Vec<(Option<Vec<u8>>, u32, bool, Option<WhereThePictureCameFrom>)> = made
            .the_other_layers
            .iter()
            .map(|other| {
                (
                    named(&other.record),
                    other.record.its_place_in_the_frames_set,
                    other.record.it_is_the_main_copy,
                    other.record.where_it_came_from.clone(),
                )
            })
            .collect();
        assert_eq!(
            others,
            vec![(Some(b"Box".to_vec()), 0, false, None), (Some(b"Mattress".to_vec()), 1, false, None)],
            "every layer's own node, the first layer's too"
        );
        assert!(made.the_other_layers[0].record.the_source_file.windows(5).any(|at| at == b"/box/"));

        // **One named copy beside the main: both.**
        let (reading, inventory) = an_output_path_and_a_node_per_layer(&["Box", "Mattress"], &["Mattress"]);
        let made = the_frame(1, &reading, &inventory);
        assert_eq!(made.record.where_it_came_from, Some(WhereThePictureCameFrom::TheOutputPath));
        assert_eq!(made.the_other_layers.len(), 1);
        assert_eq!(named(&made.the_other_layers[0].record), Some(b"Mattress".to_vec()));
        assert_eq!(made.the_other_layers[0].record.its_place_in_the_frames_set, 1);

        // **A frame of one layer is unchanged.**
        let (reading, inventory) = an_output_path_and_a_node_per_layer(&["Box"], &["Box"]);
        let made = the_frame(1, &reading, &inventory);
        assert_eq!(named(&made.record), None);
        assert_eq!(made.record.where_it_came_from, None, "nothing to tell apart: no source");
        assert!(made.the_other_layers.is_empty());
    }

    /// **A node fed by several layers names its source by the node's own
    /// name** (the Creator's tenth notes, note 2), and the main copy of a
    /// node-fed frame named by its one layer records none.
    #[test]
    fn tenth_notes_2_a_node_fed_by_several_is_named_by_the_node() {
        let (reading, inventory) =
            a_node_fed_job(&["Box", "Mattress"], &[("Both", "/home/a/both", &["Box", "Mattress"])]);
        let made = the_frame(1, &reading, &inventory);
        assert_eq!(named(&made.record), None);
        assert_eq!(
            made.record.where_it_came_from,
            Some(WhereThePictureCameFrom::AFileOutputNode(ArtistsText::of(b"Both".to_vec())))
        );
        let (reading, inventory) = a_node_fed_job(
            &["Box", "Mattress"],
            &[("Box", "/home/a/box", &["Box"]), ("Mattress", "/home/a/mattress", &["Mattress"])],
        );
        assert_eq!(the_frame(1, &reading, &inventory).record.where_it_came_from, None);
    }

    /// **A destination fed by more than one view layer is no other layer's
    /// source**: a node fed by two layers gives the main copy alone, unnamed.
    #[test]
    fn a_node_fed_by_two_layers_gives_the_main_copy_alone() {
        let (reading, inventory) =
            a_node_fed_job(&["Box", "Mattress"], &[("Both", "/home/a/both", &["Box", "Mattress"])]);
        let made = the_frame(1, &reading, &inventory);
        assert!(made.record.it_is_the_main_copy);
        assert_eq!(named(&made.record), None, "two layers feed it: no name to give");
        assert!(made.the_other_layers.is_empty());

        // **Beside a node of its own, the second layer gets its copy; the
        // first, held only in the shared node, gets none.**
        let (reading, inventory) = a_node_fed_job(
            &["Box", "Mattress"],
            &[("Both", "/home/a/both", &["Box", "Mattress"]), ("Mattress", "/home/a/mattress", &["Mattress"])],
        );
        let made = the_frame(1, &reading, &inventory);
        assert!(made.record.the_source_file.windows(6).any(|at| at == b"/both/"));
        assert_eq!(named(&made.record), None);
        assert_eq!(made.the_other_layers.len(), 1);
        assert_eq!(named(&made.the_other_layers[0].record), Some(b"Mattress".to_vec()));
        assert_eq!(made.the_other_layers[0].record.its_place_in_the_frames_set, 1);
    }

    /// **A layer no file holds has no copy**, nor does one whose only file is
    /// a node gathering into one multi-layer file, whose picture names are not
    /// known (B19) — the plan pass's third edge.
    #[test]
    fn a_layer_no_file_holds_has_no_copy() {
        let (reading, inventory) = a_node_fed_job(
            &["Box", "Mattress", "Sky"],
            &[("Box", "/home/a/box", &["Box"]), ("Sky", "/home/a/sky", &["Sky"])],
        );
        let made = the_frame(1, &reading, &inventory);
        assert_eq!(named(&made.record), Some(b"Box".to_vec()));
        let others: Vec<(Option<Vec<u8>>, u32)> = made
            .the_other_layers
            .iter()
            .map(|other| (named(&other.record), other.record.its_place_in_the_frames_set))
            .collect();
        assert_eq!(others, vec![(Some(b"Sky".to_vec()), 2)], "Mattress is held by the video alone");

        // **A gathering node fed by one layer alone**: chosen by the rungs,
        // and nothing is made from it.
        let (mut reading, _) = a_node_fed_job(
            &["Box", "Mattress"],
            &[("Box", "/home/a/box", &["Box"]), ("Mattress", "/home/a/mattress", &["Mattress"])],
        );
        reading.destinations[2].kind = DestinationKind::AWholeFileOutputNodeGatheringIntoOneFile {
            node: ArtistsText::of(b"Mattress".to_vec()),
            the_nodes_base_file_name: None,
            replaced: None,
        };
        reading.destinations[2].format = Some(crate::reading::Format {
            as_the_file_records_it: ArtistsText::of(THE_MULTI_LAYER_FORMAT.to_vec()),
            the_ending_it_writes: Some(ArtistsText::of(b".exr".to_vec())),
            is_video_shaped: false,
        });
        let inventory = crate::inventory::WhatTheInventoryIs::It(the_inventory(&reading));
        let made = the_frame(1, &reading, &inventory);
        assert_eq!(named(&made.record), Some(b"Box".to_vec()));
        assert!(made.the_other_layers.is_empty());
    }
}

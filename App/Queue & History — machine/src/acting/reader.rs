//! **§5 performed — R2's two grammars, R3's rule for a line that matches
//! neither, and the parse of each kind of line R7, R9, R10, R14 and R15
//! name.**
//!
//! # The two grammars, and why nothing is shared across the boundary
//!
//! **R2: the reader is composed per version, selected by the manifest's
//! Blender version, in two grammars.** The 4.5 grammar: plain lines, no
//! frame announced before it is rendered, the progress line naming frame and
//! view layer. The 5.0-and-later grammar: timestamped, sorted lines, every
//! frame announced, the frame range announced once at the start. **The
//! boundary is one and the whole stream moved together across it — seven
//! kinds of line, recorded separately — so no line is parsed by a rule
//! shared across the boundary.**
//!
//! **That is kept by the type here**: [`TheGrammar`] is chosen once from the
//! version, and every field it reads is that grammar's own. The two differ
//! by more than a prefix — in 4.5 the frame field is `Fra:1` and the estimate
//! `Remaining:00:00.05`, and from 5.0 they are `Fra: 1` and
//! `Remaining: 00:00.05`, one space apart — and **a rule that read both
//! would be exactly the rule R2 forbids.**
//!
//! # Where every shape below comes from
//!
//! Every line form is quoted from this project's own Blender research, at
//! the comparison questions the joined answers name, and **nothing is
//! written from memory.** Where there is no shape to build on, there is no
//! match and the absence is a value — see the colour lines below, which are
//! the one case of it.
//!
//! - **The progress line, both grammars** — *Reports 3*: in 4.5
//!   `Fra:1 Mem:9.77M (Peak 9.77M) | Time:00:00.02 | Remaining:00:00.05 |
//!   Mem:1.48M, Peak:1.48M | Scene, ViewLayer | Sample 1/16`, *"a frame
//!   number, two memory pairs, an elapsed time, sometimes a remaining time,
//!   the scene and view layer, and the stage or sample count"*; in 5.0, 5.1
//!   and 5.2 `00:00.430  render           | Fra: 1 | Remaining: 00:00.05 |
//!   Mem: 2M | Sample 1/16`, *"an elapsed timestamp, the category `render`, a
//!   vertical bar, then a frame number, sometimes a remaining time, one
//!   memory figure and the stage or sample count"*. **The scene and view
//!   layer and the second memory pair are gone from the newer three**, which
//!   is R10's fallback stated as a shape.
//! - **The tiled line** — *Reports 7*: *"the end of the line becomes
//!   `Rendered <n>/<total> Tiles, Sample <s>/<max>`"*, and they are *"these
//!   same two shapes with a tile count added inside the line, before the
//!   sample count"* (*Reports 3*). **R12's deciding condition is the printed
//!   line and never a computed size**: a line carrying a tile figure is a
//!   tiled frame and a line carrying none is not.
//! - **The frame announcements, 5.0 and later only** — *Reports 3*: a
//!   single-frame run opens with `Rendering single frame`, `Rendering frame
//!   1`, `Start rendering: Scene, ViewLayer` and `Engine: Cycles`, and an
//!   animation opens with `Rendering animation (frames 1..3)` and then names
//!   each frame as it begins. **In 4.5 none of those lines exists**, which is
//!   R9's *read off the line in 4.5, counted off the announcements in 5.0 and
//!   later*.
//! - **The view layer's announcement, 5.0 and later only** — R10 as
//!   corrected 2026-10-04: `Start rendering: <scene>, <view layer>`, printed
//!   on the ordinary output at the start of each view layer of each frame.
//!   Label: **observed in a test** at the desk, 2026-10-04, in 5.0.1, 5.1.2
//!   and 5.2.2 under Cycles and EEVEE, two layers over two frames; 4.5.12
//!   never prints it. **The known scene's name is taken off the front**
//!   (piece 3c, 2026-10-04), in this line and in 4.5's own scene-and-layer
//!   field, so a scene whose name holds `, ` (seen at the desk: `Start
//!   rendering: Shot, 01, Back, plate`) gives the right layer; only where the
//!   scene is not known is the text split at its first `, `.
//! - **The estimate** — *Reports 5*: 4.5 writes `Remaining:00:00.05` with no
//!   space after the colon, and 5.0, 5.1 and 5.2 write `Remaining: 00:00.05`
//!   with one; it *"sits between the elapsed time and the memory figure"*,
//!   covers the frame in hand, and *"does not appear on every line"*.
//! - **The save announcement** — *Frames 3*: 4.5 prints `Saved: '…'` and
//!   `Time: 00:01.20 (Saving: 00:00.34)` bare, and 5.0, 5.1 and 5.2 print the
//!   same two lines behind an elapsed time and the word `render`; *"What
//!   lands on disk is identical."* *Reports 8*: *"A successful run ends
//!   `Saved:`, then `Time:`, then `Blender quit`, and exits 0."*
//! - **EEVEE** — *Reports 3*: in 4.5 `Fra:1 Mem:40.65M (Peak 40.65M) |
//!   Time:00:00.28 | Rendering 1 / 64 samples`, and from 5.0 `Fra: 1 |
//!   Rendering 1 / 64 samples`. **Workbench prints no progress line at all**,
//!   in any version on either machine (*Reports 8*), which is R14's ordinary
//!   case rather than a fault.
//! - **The device line** — *What actually ran 1*: *"The line reads `Path
//!   tracing on:` and carries the model, and for a processor the thread
//!   count, and for a card the route taken to it and its place on the bus.
//!   Every version prints it."* In 4.5 it is Cycles' own printing on the
//!   error stream; from 5.0 it comes through the log on standard output.
//!
//! # The one shape left absent on purpose
//!
//! **No colour-substitution line is read here, in any version** —
//! [`crate::ending::ThePrintedText`] is that absence as a value, and it has
//! one variant on purpose. For 4.5 to 5.2, *Dependencies 15* records what the
//! three lines **say** — the scene view not found and a default set, the image
//! colour space not found and a default to be used instead, and a warning that
//! the current value matches no entry in the list — and the exact printed text
//! of each sits outside this stage's citation surface. For 5.3 it quotes the
//! text, from a beta on a Mac, and the view's replacement prints no line. **The
//! build runner's ruling of 2026-11-02 parks a reader for those in Open Threads
//! until the release re-read.** [`WhatTheLineWas`] carries no colour arm, and
//! E13a's observation reaches [`crate::ending::what_the_entry_says_about_colour`]
//! only from a reader somebody writes then. **Absence is never evidence that no
//! substitution happened**, which is E13a's own rule, and the entry saying
//! nothing is what it already does.
//!
//! # R3 and R8, kept by the shape of the answer
//!
//! **R3: a line that matches no grammar yields nothing and is kept anyway.**
//! Every function below answers [`WhatTheLineWas::NothingReadable`] rather
//! than a failure, so an unmatched line is a line with no reading in it —
//! which is what lets the reader survive the complaints stream not being
//! reliably empty and the shell adding lines of its own on Linux. **The sink
//! keeps every byte either way**, which is the copy loop's and not this
//! module's.
//!
//! **R8: the reader never fails the render and never slows it by more than
//! its own copy loop.** There is no allocation per field, no second read of
//! anything, and no route from here to the job's state — this file returns
//! neither a [`crate::line::QueueState`] nor an [`crate::ending::Ending`].
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Rendering` L26.
//!
//! **`Rendering` L1 and `Rendering` L7 are deliberately not on that list and
//! are written out at every site instead**, because this plan defines a
//! clause of each name and a bare one would resolve to a real and wrong
//! clause.

use crate::readings::{AProgressLine, SamplePair, TilePair};

/// **R2's two grammars.** Chosen once from the manifest's Blender version,
/// and **nothing below is shared across the boundary**.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TheGrammar {
    /// 4.5: plain lines, no frame announced before it is rendered, the
    /// progress line naming frame and view layer.
    PlainLinesNamingTheViewLayer,
    /// 5.0, 5.1 and 5.2: timestamped, sorted lines, every frame announced,
    /// the frame range announced once at the start.
    TimestampedAndSortedWithEveryFrameAnnounced,
}

impl TheGrammar {
    /// **The one boundary the research draws**, and the only thing the
    /// grammar turns on.
    pub fn for_a_version(major: u32) -> TheGrammar {
        if major >= 5 {
            TheGrammar::TimestampedAndSortedWithEveryFrameAnnounced
        } else {
            TheGrammar::PlainLinesNamingTheViewLayer
        }
    }

    /// The frame field's own spelling in this grammar — `Fra:` against
    /// `Fra: `, which is the whole of the difference (*Reports 3*).
    fn the_frame_field(self) -> &'static str {
        match self {
            TheGrammar::PlainLinesNamingTheViewLayer => "Fra:",
            TheGrammar::TimestampedAndSortedWithEveryFrameAnnounced => "Fra: ",
        }
    }

    /// The estimate's own spelling in this grammar — `Remaining:` against
    /// `Remaining: `, one space apart (*Reports 5*).
    fn the_estimate_field(self) -> &'static str {
        match self {
            TheGrammar::PlainLinesNamingTheViewLayer => "Remaining:",
            TheGrammar::TimestampedAndSortedWithEveryFrameAnnounced => "Remaining: ",
        }
    }

    /// **One field's value, read exactly.**
    ///
    /// **The space after the colon is the difference between the two
    /// grammars and it is honoured in both directions**: the older
    /// grammar's head is a prefix of the newer one's, so a read that
    /// merely stripped it would let 4.5's rules read a 5.2 line — which is
    /// the rule shared across the boundary R2 forbids, arriving by
    /// accident. **A value beginning with a space is not this grammar's
    /// field.**
    fn the_value_after<'a>(self, field: &'a str, head: &str) -> Option<&'a str> {
        let rest = field.strip_prefix(head)?;
        (!rest.starts_with(' ')).then_some(rest)
    }
}

/// What one line was.
///
/// **The set is closed and every arm is a shape the research carries.** A
/// line that is none of them is [`WhatTheLineWas::NothingReadable`], which is
/// R3's rule rather than an error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatTheLineWas {
    /// `Queue & History` R6a: Smoozy's own simulation progress and closing line.
    Simulation { frames: Option<(i64, i64)> },
    /// A progress line, as the grammar read it. **R6's closed list of
    /// readings is what [`crate::readings::fold_a_line`] folds it into**, and
    /// nothing here folds.
    AProgressLine(AProgressLine),
    /// **R9's announcement, 5.0 and later only**: the frame that has begun.
    /// **It is not a count of frames** — the newer three print it once per
    /// step of a stepped run, over and over where the step is zero, once past
    /// the end of an overshooting run, and once before each frame that is
    /// then skipped.
    AFrameHasBegun { frame: i64 },
    /// **R10's announcement, 5.0 and later only**: the view layer whose
    /// renderlet has begun, on the frame in hand. Added 2026-10-04, the
    /// Creator's second notes, piece 3b.
    AViewLayerHasBegun { view_layer: String },
    /// **The frame range, announced once at the start, 5.0 and later only.**
    /// The one figure covering the animation any version gives.
    TheFrameRange { from: i64, to: i64 },
    /// **P1a's save announcement**, in both grammars. A frame announced is a
    /// frame begun; **only its save ends the write, and the line proves the
    /// write ended and never that the file is whole** — a write that ran out
    /// of room is announced with exactly the same line.
    AFrameWasSaved { at: String },
    /// **`Rendering` L26's device line**, observed in passing on the
    /// version's own stream. **What it is evidence of is E12's**, and turns
    /// on the recorded outcome rather than on the line's presence.
    ///
    /// **Only the head is matched and the rest is carried as printed.** The
    /// research states what the line carries — the model, and for a
    /// processor the thread count, and for a card the route taken to it and
    /// its place on the bus — and a rule reading any of that would be a rule
    /// this file may not write, since the model is the machine's and not
    /// Blender's.
    TheDeviceLine { as_printed: String },
    /// R3: no reading in it, and the sink keeps it either way.
    NothingReadable,
}

/// **The parse, per version, of one line.**
///
/// `line` is one line of the render's own output as the copy loop saw it,
/// with no trailing newline. **Nothing here reads the folder, the sink or
/// anything but this line** — and the scene's name, where the job's reading
/// knows it (`the_scene`), which is how the view layer is told apart from a
/// scene whose name holds `, ` ([`the_layer_after_the_scene`]).
pub fn read_one_line(grammar: TheGrammar, the_scene: Option<&str>, line: &str) -> WhatTheLineWas {
    // `Queue & History` R6a and `Rendering` L10b: this is Smoozy's own
    // marker, identical in every version, not part of a Blender grammar.
    if let Some(json) = line.trim_end().strip_prefix("SMOOZY-SIMULATION ") {
        if let Ok(value) = serde_json::from_str::<serde_json::Value>(json) {
            if value.get("finished").and_then(|one| one.as_bool()) == Some(true) {
                return WhatTheLineWas::Simulation { frames: None };
            }
            if let Some((frame, to)) = value.get("frame").and_then(|one| one.as_i64())
                .zip(value.get("to").and_then(|one| one.as_i64()))
                .filter(|(frame, to)| frame <= to)
            {
                return WhatTheLineWas::Simulation { frames: Some((frame, to)) };
            }
        }
        return WhatTheLineWas::NothingReadable;
    }
    let body = the_message_part(grammar, line);

    // The device line is Cycles' own closing report and sits inside its own
    // block, unprefixed in 4.5 and prefixed from 5.0 — so it is read off the
    // message part like everything else.
    if let Some(at) = body.find(THE_DEVICE_LINES_HEAD) {
        return WhatTheLineWas::TheDeviceLine {
            as_printed: body[at..].trim().to_string(),
        };
    }
    if let Some(at) = body.find(THE_SAVE_ANNOUNCEMENTS_HEAD) {
        let rest = &body[at + THE_SAVE_ANNOUNCEMENTS_HEAD.len()..];
        if let Some(path) = between_single_quotes(rest) {
            return WhatTheLineWas::AFrameWasSaved { at: path };
        }
    }
    if grammar == TheGrammar::TimestampedAndSortedWithEveryFrameAnnounced {
        if let Some(rest) = body.strip_prefix(THE_ANIMATIONS_ANNOUNCEMENT) {
            if let Some((from, to)) = the_range_in(rest) {
                return WhatTheLineWas::TheFrameRange { from, to };
            }
        }
        if let Some(rest) = body.strip_prefix(THE_FRAMES_ANNOUNCEMENT) {
            if let Ok(frame) = rest.trim().parse::<i64>() {
                return WhatTheLineWas::AFrameHasBegun { frame };
            }
        }
        if let Some(rest) = body.strip_prefix(THE_VIEW_LAYERS_ANNOUNCEMENT) {
            if let Some(layer) = the_layer_after_the_scene(rest, the_scene) {
                return WhatTheLineWas::AViewLayerHasBegun {
                    view_layer: layer.to_string(),
                };
            }
        }
    }

    let fields: Vec<&str> = body.split(" | ").map(str::trim).collect();
    let frame = fields.iter().find_map(|field| {
        let rest = grammar.the_value_after(field, grammar.the_frame_field())?;
        let digits: String = rest
            .chars()
            .take_while(|c| c.is_ascii_digit() || *c == '-')
            .collect();
        digits.parse::<i64>().ok()
    });
    if frame.is_none() {
        return WhatTheLineWas::NothingReadable;
    }

    // **R15: shown as printed, marked as Blender's and as rough, and nothing
    // is computed from it.** It is kept as the text the line carried, so a
    // figure with an hours field is carried whole rather than mis-read by an
    // arithmetic nobody asked for.
    let estimate = fields
        .iter()
        .find_map(|field| grammar.the_value_after(field, grammar.the_estimate_field()))
        .map(|rest| rest.trim().to_string());

    // **R10: the renderlet is named where it can be and the frame alone
    // where it cannot.** In 4.5 the line names scene and view layer in a
    // field of its own; from 5.0 the progress line names none, and the view
    // layer comes from its own announcement above (R10 as corrected
    // 2026-10-04).
    let view_layer = match grammar {
        TheGrammar::PlainLinesNamingTheViewLayer => fields
            .iter()
            .find_map(|field| {
                // The known scene's name off the front, where the reading
                // knows it (piece 3c).
                let scene = the_scene.filter(|scene| !scene.is_empty())?;
                field
                    .strip_prefix(scene)?
                    .strip_prefix(", ")
                    .filter(|layer| !layer.is_empty())
            })
            .or_else(|| {
                fields
                    .iter()
                    .filter_map(|field| field.split_once(", "))
                    .find(|(scene, layer)| {
                        // The line carries two fields with a comma in them —
                        // the second memory pair and the scene-and-layer one —
                        // and the memory pair's halves each carry a colon.
                        !scene.is_empty()
                            && !layer.is_empty()
                            && !scene.contains(':')
                            && !layer.contains(':')
                    })
                    .map(|(_, layer)| layer)
            })
            .map(str::to_string),
        TheGrammar::TimestampedAndSortedWithEveryFrameAnnounced => None,
    };

    let last = fields.last().copied().unwrap_or_default();
    let (tiles, samples) = the_tiles_and_samples_in(last);

    WhatTheLineWas::AProgressLine(AProgressLine {
        frame,
        view_layer,
        samples,
        tiles,
        estimate,
        // **R12's line of 2026-11-02.** Read off the line's last field in both
        // grammars, like the samples beside it, and never for what it says about
        // a file (`Queue & History` E3).
        the_line_says_finished: last == THE_FINISHED_STAGE_WORD,
    })
}

/// **The view layer, from `<scene>, <view layer>`** — the 5.0-and-later
/// announcement's text after its head. **The known scene's name is taken off
/// the front, and the text is split at its first `, ` only where the scene is
/// not known.** Added 2026-10-04, the Creator's second notes, piece 3c, on the
/// build runner's ruling: a scene named `Shot, 01` with a layer named
/// `Back, plate` printed `Start rendering: Shot, 01, Back, plate` (5.2.2, at
/// the desk), and the split at the first `, ` named the layer `01, Back,
/// plate`. The scene's name is known from the job's reading; where it is not
/// — or the text does not begin with it — the first `, ` is the rule there
/// is. Nothing where no layer follows.
fn the_layer_after_the_scene<'a>(text: &'a str, the_scene: Option<&str>) -> Option<&'a str> {
    let after_the_scene = the_scene
        .filter(|scene| !scene.is_empty())
        .and_then(|scene| text.strip_prefix(scene))
        .filter(|rest| rest.is_empty() || rest.starts_with(", "));
    match after_the_scene {
        Some(rest) => rest.strip_prefix(", "),
        None => text.split_once(", ").map(|(_, layer)| layer),
    }
    .filter(|layer| !layer.is_empty())
}

/// **The message part**, which is the whole line in 4.5 and everything after
/// the first bar from 5.0 — *"an elapsed timestamp, the category `render`, a
/// vertical bar, then …"* (*Reports 3*).
///
/// **The category is dropped rather than matched on.** More than one category
/// prints lines this reader meets — `render` for the progress lines, `cycles`
/// for the device line's block — and a rule keyed to one of them would drop
/// the other in silence.
fn the_message_part(grammar: TheGrammar, line: &str) -> &str {
    match grammar {
        TheGrammar::PlainLinesNamingTheViewLayer => line.trim_end(),
        TheGrammar::TimestampedAndSortedWithEveryFrameAnnounced => match line.split_once("| ") {
            Some((_, rest)) => rest.trim_end(),
            None => line.trim_end(),
        },
    }
}

/// **R7: the tiled line's own shape, and R11's sample pair beside it.**
///
/// *Reports 7*: `Rendered <n>/<total> Tiles, Sample <s>/<max>`. **The
/// deciding condition is the printed line** — a line carrying a tile figure
/// is a tiled frame and a line carrying none is not — so the tile pair is
/// read first and the sample pair from what follows it.
fn the_tiles_and_samples_in(field: &str) -> (Option<TilePair>, Option<SamplePair>) {
    if let Some(rest) = field.strip_prefix(THE_TILED_HEAD) {
        if let Some((tiles, after)) = rest.split_once(THE_TILED_JOIN) {
            return (
                a_pair(tiles).map(|(done, of)| TilePair { done, of }),
                a_sample_pair(after),
            );
        }
        return (None, None);
    }
    (None, a_sample_pair(field))
}

/// Cycles' own sample field, and EEVEE's, which are two spellings of one
/// reading (*Reports 3*, *Reports 8*).
///
/// **A note in brackets after Cycles' count is set aside, whatever it says.**
/// Added 2026-10-04, the Creator's second notes, piece 3b: on this Mac's
/// graphics card 5.2.2 ended every count line with one — `Rendered 0/4 Tiles,
/// Sample 129/1000 (1 kernels to optimize)`, `Sample 1000/1000 (Using
/// optimized kernels)` — and the count was read as absent on every line.
/// Label: **observed in a test** at the desk (the Creator's 4K, 1,000-sample
/// shape).
fn a_sample_pair(field: &str) -> Option<SamplePair> {
    let field = field.trim();
    if let Some(rest) = field.strip_prefix(CYCLES_SAMPLE_HEAD) {
        let count = rest.split_once(" (").map_or(rest, |(count, _)| count);
        return a_pair(count).map(|(seen, of)| SamplePair { seen, of });
    }
    if let Some(rest) = field.strip_prefix(EEVEE_SAMPLE_HEAD) {
        let rest = rest.strip_suffix(EEVEE_SAMPLE_TAIL)?;
        let (seen, of) = rest.split_once(" / ")?;
        return Some(SamplePair {
            seen: seen.trim().parse().ok()?,
            of: of.trim().parse().ok()?,
        });
    }
    None
}

fn a_pair(text: &str) -> Option<(u64, u64)> {
    let (left, right) = text.trim().split_once('/')?;
    Some((left.trim().parse().ok()?, right.trim().parse().ok()?))
}

fn the_range_in(text: &str) -> Option<(i64, i64)> {
    let inside = text
        .trim()
        .strip_prefix('(')?
        .strip_suffix(')')?
        .strip_prefix(THE_RANGES_WORD)?;
    let (from, to) = inside.split_once("..")?;
    Some((from.trim().parse().ok()?, to.trim().parse().ok()?))
}

fn between_single_quotes(text: &str) -> Option<String> {
    let opens = text.find('\'')? + 1;
    let closes = text[opens..].rfind('\'')? + opens;
    (closes > opens).then(|| text[opens..closes].to_string())
}

/// *Reports 7*: the tiled line's own head.
const THE_TILED_HEAD: &str = "Rendered ";
/// *Reports 7*: what sits between the tile figure and the sample pair.
const THE_TILED_JOIN: &str = " Tiles, ";
/// *Reports 11*: the stage word that closes a frame's sampling, on the line after
/// its last count. **On a frame with tiles too, once per layer, in 4.5.12,
/// 5.0.1, 5.1.2, 5.2.3 and the 5.3.0 Beta** (`Queue & History` R12's second line
/// of 2026-11-02, observed at the desk).
const THE_FINISHED_STAGE_WORD: &str = "Finished";
/// *Reports 3* and *Reports 8*: Cycles' own sample field.
const CYCLES_SAMPLE_HEAD: &str = "Sample ";
/// *Reports 3*: EEVEE's, which carries the count with spaces around the
/// slash and a word after it — `Rendering 1 / 64 samples`.
const EEVEE_SAMPLE_HEAD: &str = "Rendering ";
const EEVEE_SAMPLE_TAIL: &str = " samples";
/// *Reports 3*: the per-frame announcement, 5.0 and later.
const THE_FRAMES_ANNOUNCEMENT: &str = "Rendering frame ";
/// R10 as corrected 2026-10-04: the view layer's announcement, 5.0 and later
/// — `Start rendering: Scene, ViewLayer` (*Reports 3* quotes the line; the
/// desk showed it at every layer of every frame).
const THE_VIEW_LAYERS_ANNOUNCEMENT: &str = "Start rendering: ";
/// *Reports 3* and *Frames 3*: the range, announced once at the start —
/// `Rendering animation (frames 1..3)`.
const THE_ANIMATIONS_ANNOUNCEMENT: &str = "Rendering animation ";
/// The word inside that line's brackets.
const THE_RANGES_WORD: &str = "frames ";
/// *Frames 3* and *Reports 8*: the save announcement, in both grammars.
const THE_SAVE_ANNOUNCEMENTS_HEAD: &str = "Saved: ";
/// *What actually ran 1*: the device line's own head.
const THE_DEVICE_LINES_HEAD: &str = "Path tracing on:";

/// **The colour-substitution lines are matched by nothing here, and that is
/// the point.**
///
/// [`crate::ending::ThePrintedText`] states the absence. For 4.5 to 5.2 the
/// exact printed text of the three is on record nowhere, and **a grammar
/// written for one would be written from memory**, which is the one thing this
/// file may not do, and it would be invisible until a real render — matching
/// nothing, or worse, matching the wrong line. 5.3's is parked (the head of
/// this file). **E13a's rule already covers what stands
/// meanwhile**: where no such line was observed the entry says nothing at
/// all, and absence is never evidence that no substitution happened.
pub const NO_COLOUR_SUBSTITUTION_LINE_IS_MATCHED_HERE: bool = true;

#[cfg(test)]
mod tests {
    use super::*;

    const FOUR_FIVE: TheGrammar = TheGrammar::PlainLinesNamingTheViewLayer;
    const FROM_FIVE: TheGrammar = TheGrammar::TimestampedAndSortedWithEveryFrameAnnounced;

    fn a_progress_line(what: WhatTheLineWas) -> AProgressLine {
        match what {
            WhatTheLineWas::AProgressLine(line) => line,
            other => panic!("{other:?}"),
        }
    }

    /// **The 4.5 grammar, on *Reports 3*'s own line.**
    #[test]
    fn the_four_five_progress_line_gives_frame_view_layer_estimate_and_samples() {
        let line = read_one_line(
            FOUR_FIVE,
            None,
            "Fra:1 Mem:9.77M (Peak 9.77M) | Time:00:00.02 | Remaining:00:00.05 | \
             Mem:1.48M, Peak:1.48M | Scene, ViewLayer | Sample 1/16",
        );
        let read = a_progress_line(line);
        assert_eq!(read.frame, Some(1));
        assert_eq!(read.view_layer.as_deref(), Some("ViewLayer"));
        assert_eq!(read.estimate.as_deref(), Some("00:00.05"));
        assert_eq!(read.samples, Some(SamplePair { seen: 1, of: 16 }));
        assert_eq!(read.tiles, None);
        assert!(!read.is_a_tiled_frame());
    }

    /// **The 5.0-and-later grammar, on *Reports 3*'s own line.** The view
    /// layer comes from nowhere at all, which is R10's fallback.
    #[test]
    fn the_from_five_progress_line_gives_frame_estimate_and_samples_and_no_view_layer() {
        let read = a_progress_line(read_one_line(
            FROM_FIVE,
            None,
            "00:00.430  render           | Fra: 1 | Remaining: 00:00.05 | Mem: 2M | Sample 1/16",
        ));
        assert_eq!(read.frame, Some(1));
        assert_eq!(read.view_layer, None, "R10's fallback, not a gap");
        assert_eq!(read.estimate.as_deref(), Some("00:00.05"));
        assert_eq!(read.samples, Some(SamplePair { seen: 1, of: 16 }));
    }

    /// **R2, checked by the failure rather than argued**: the other
    /// grammar's line is not read by this one. One space apart in two
    /// fields, and a rule shared across the boundary would have read both.
    #[test]
    fn neither_grammar_reads_the_others_line() {
        let the_four_five_line = "Fra:1 Mem:9.77M (Peak 9.77M) | Time:00:00.02 | \
                                  Remaining:00:00.05 | Mem:1.48M, Peak:1.48M | \
                                  Scene, ViewLayer | Sample 1/16";
        assert_eq!(
            read_one_line(FROM_FIVE, None, the_four_five_line),
            WhatTheLineWas::NothingReadable
        );
        let the_newer_line =
            "00:00.430  render           | Fra: 1 | Remaining: 00:00.05 | Mem: 2M | Sample 1/16";
        // 4.5's grammar reads the newer line's frame field as absent: its own
        // field has no space and the newer line's number never parses.
        assert_eq!(
            read_one_line(FOUR_FIVE, None, the_newer_line),
            WhatTheLineWas::NothingReadable
        );
    }

    /// **R7 and R12, on the thirteen-line run's own first and last lines.**
    /// Up to 5.2 the last line of a 2049 by 2049 run that exited 0 and wrote its
    /// picture ends on a trailing zero, and 5.3's on the last tile's full count
    /// (*Reports 7*) — which is why the tiling condition is read off the line
    /// and never from a size.
    #[test]
    fn a_tiled_line_gives_the_tile_pair_and_the_tiles_own_sample_pair_in_both_grammars() {
        let read = a_progress_line(read_one_line(
            FOUR_FIVE,
            None,
            "Fra:1 Mem:8.42M (Peak 248.42M) | Time:00:03.25 | Mem:0.19M, Peak:240.19M | \
             Scene, ViewLayer | Rendered 3/4 Tiles, Sample 0/16",
        ));
        assert_eq!(read.tiles, Some(TilePair { done: 3, of: 4 }));
        assert_eq!(read.samples, Some(SamplePair { seen: 0, of: 16 }));
        assert!(read.is_a_tiled_frame());

        let read = a_progress_line(read_one_line(
            FROM_FIVE,
            None,
            "00:04.055  render           | Fra: 1 | Mem: 1M | Rendered 3/4 Tiles, Sample 0/16",
        ));
        assert_eq!(read.tiles, Some(TilePair { done: 3, of: 4 }));
        assert!(read.is_a_tiled_frame());
    }

    /// The huge tile count, which is the figure a working picture reaches —
    /// `Rendered 780/53284 Tiles, Sample 1/1`, with an estimate carrying an
    /// hours field.
    #[test]
    fn a_large_tile_count_and_an_estimate_with_an_hours_field_are_carried_as_printed() {
        let read = a_progress_line(read_one_line(
            FOUR_FIVE,
            None,
            "Fra:1 Mem:315.03M (Peak 619.03M) | Time:03:59.02 | Remaining:04:27:39.94 | \
             Mem:306.47M, Peak:306.47M | Scene, ViewLayer | Rendered 780/53284 Tiles, Sample 1/1",
        ));
        assert_eq!(
            read.tiles,
            Some(TilePair {
                done: 780,
                of: 53284
            })
        );
        // **R15: shown as printed and nothing computed from it**, so a third
        // field costs nothing and is never mis-read as minutes.
        assert_eq!(read.estimate.as_deref(), Some("04:27:39.94"));
    }

    /// **R14: EEVEE prints far fewer lines with its own sample count and no
    /// estimate**, in both grammars.
    #[test]
    fn eevees_own_sample_field_is_read_in_both_grammars_and_carries_no_estimate() {
        let read = a_progress_line(read_one_line(
            FOUR_FIVE,
            None,
            "Fra:1 Mem:40.65M (Peak 40.65M) | Time:00:00.28 | Rendering 1 / 64 samples",
        ));
        assert_eq!(read.frame, Some(1));
        assert_eq!(read.samples, Some(SamplePair { seen: 1, of: 64 }));
        assert_eq!(read.estimate, None);

        let read = a_progress_line(read_one_line(
            FROM_FIVE,
            None,
            "00:00.790  render           | Fra: 1 | Rendering 1 / 64 samples",
        ));
        assert_eq!(read.samples, Some(SamplePair { seen: 1, of: 64 }));
    }

    /// **R9: the frame is announced in 5.0 and later and in 4.5 no such line
    /// exists**, so the 4.5 grammar reads one as nothing.
    #[test]
    fn the_announcements_are_read_from_five_and_are_nothing_in_four_five() {
        assert_eq!(
            read_one_line(FROM_FIVE, None, "00:00.573  render           | Rendering frame 2"),
            WhatTheLineWas::AFrameHasBegun { frame: 2 }
        );
        assert_eq!(
            read_one_line(
                FROM_FIVE,
                None,
                "00:00.405  render           | Rendering animation (frames 1..3)"
            ),
            WhatTheLineWas::TheFrameRange { from: 1, to: 3 }
        );
        // A reversed range is on record and reads as it printed.
        assert_eq!(
            read_one_line(
                FROM_FIVE,
                None,
                "00:00.402  render           | Rendering animation (frames 10..5)"
            ),
            WhatTheLineWas::TheFrameRange { from: 10, to: 5 }
        );
        assert_eq!(
            read_one_line(FOUR_FIVE, None, "Rendering frame 2"),
            WhatTheLineWas::NothingReadable
        );
    }

    /// **Piece 3b: a note in brackets after the count leaves the count
    /// read**, tiled or not — the lines 5.2.2 printed on this Mac's graphics
    /// card. **What it bites on**: the read of before, which gave no count.
    #[test]
    fn a_note_in_brackets_after_the_count_leaves_the_count_read() {
        let read = a_progress_line(read_one_line(
            FROM_FIVE,
            None,
            "00:10.737  render           | Fra: 1 | Remaining: 02:22.39 | Mem: 1489M | \
             Rendered 0/4 Tiles, Sample 129/1000 (1 kernels to optimize)",
        ));
        assert_eq!(read.tiles, Some(TilePair { done: 0, of: 4 }));
        assert_eq!(read.samples, Some(SamplePair { seen: 129, of: 1000 }));
        assert_eq!(read.estimate.as_deref(), Some("02:22.39"));
        let read = a_progress_line(read_one_line(
            FROM_FIVE,
            None,
            "01:20.439  render           | Fra: 1 | Mem: 1489M | Sample 1000/1000 (Using optimized kernels)",
        ));
        assert_eq!(read.samples, Some(SamplePair { seen: 1000, of: 1000 }));
        // Whatever the note says, and in the older grammar too.
        let read = a_progress_line(read_one_line(
            FOUR_FIVE,
            None,
            "Fra:1 Mem:8.30M (Peak 9.10M) | Time:00:00.02 | Mem:0.09M, Peak:0.09M | Scene, First | \
             Sample 7/1000 (any words at all)",
        ));
        assert_eq!(read.samples, Some(SamplePair { seen: 7, of: 1000 }));
    }

    /// **R10 as corrected 2026-10-04: from 5.0 the view layer is announced at
    /// each layer's start, and in 4.5 no such line exists.** The lines are the
    /// ones 5.2.2 printed at the desk; the scene and the layer split at the
    /// first `, `.
    #[test]
    fn the_view_layers_announcement_is_read_from_five_and_is_nothing_in_four_five() {
        assert_eq!(
            read_one_line(FROM_FIVE, None, "00:26.288  render           | Start rendering: Scene, Second"),
            WhatTheLineWas::AViewLayerHasBegun {
                view_layer: "Second".to_string()
            }
        );
        assert_eq!(
            read_one_line(FROM_FIVE, None, "00:00.409  render           | Start rendering: Shot, 01, Back, plate"),
            WhatTheLineWas::AViewLayerHasBegun {
                view_layer: "01, Back, plate".to_string()
            },
            "a scene name holding a comma gives a wrong name, as recorded"
        );
        assert_eq!(
            read_one_line(FOUR_FIVE, None, "Start rendering: Scene, Second"),
            WhatTheLineWas::NothingReadable
        );
        assert_eq!(
            read_one_line(FROM_FIVE, None, "00:00.407  render           | Start rendering: Scene"),
            WhatTheLineWas::NothingReadable,
            "no layer, no reading"
        );
    }

    /// **Piece 3c: the known scene's name is taken off the front**, in the
    /// 5.0-and-later announcement and in 4.5's own field, so a scene whose
    /// name holds `, ` gives the right layer — the line 5.2.2 printed at the
    /// desk for scene `Shot, 01`, layer `Back, plate`. Where the scene is not
    /// known, or the text does not begin with it, the first `, ` stands.
    /// **What it bites on**: the split at the first `, ` alone, which named
    /// the layer `01, Back, plate`.
    #[test]
    fn the_known_scene_is_taken_off_the_front_and_the_first_comma_splits_only_where_it_is_not_known() {
        let announced = "00:00.409  render           | Start rendering: Shot, 01, Back, plate";
        let begun = |layer: &str| WhatTheLineWas::AViewLayerHasBegun {
            view_layer: layer.to_string(),
        };
        assert_eq!(read_one_line(FROM_FIVE, Some("Shot, 01"), announced), begun("Back, plate"));
        assert_eq!(read_one_line(FROM_FIVE, None, announced), begun("01, Back, plate"));
        assert_eq!(
            read_one_line(FROM_FIVE, Some("Another"), announced),
            begun("01, Back, plate"),
            "a scene the line does not begin with: the first comma"
        );
        assert_eq!(
            read_one_line(FROM_FIVE, Some("Shot, 01"), "00:00.409  render           | Start rendering: Shot, 01"),
            WhatTheLineWas::NothingReadable,
            "the scene alone names no layer"
        );
        let four_five = "Fra:1 Mem:8.30M (Peak 9.10M) | Time:00:00.02 | Mem:0.09M, Peak:0.09M | \
                         Shot, 01, Back, plate | Sample 7/16";
        assert_eq!(
            a_progress_line(read_one_line(FOUR_FIVE, Some("Shot, 01"), four_five)).view_layer.as_deref(),
            Some("Back, plate")
        );
        assert_eq!(
            a_progress_line(read_one_line(FOUR_FIVE, None, four_five)).view_layer.as_deref(),
            Some("01, Back, plate")
        );
    }

    /// **P1a's save announcement, in both grammars** — the same line behind
    /// an elapsed time and the word `render` from 5.0.
    #[test]
    fn the_save_announcement_is_read_in_both_grammars_and_gives_the_path_it_named() {
        assert_eq!(
            read_one_line(FOUR_FIVE, None, "Saved: '/root/smoozy/out/frame_0001.png'"),
            WhatTheLineWas::AFrameWasSaved {
                at: "/root/smoozy/out/frame_0001.png".to_string()
            }
        );
        assert_eq!(
            read_one_line(
                FROM_FIVE,
                None,
                "00:00.935  render           | Saved: '/root/smoozy/out/frame_0001.png'"
            ),
            WhatTheLineWas::AFrameWasSaved {
                at: "/root/smoozy/out/frame_0001.png".to_string()
            }
        );
    }

    /// **`Rendering` L26's line, in both grammars** — unprefixed in 4.5,
    /// where it is Cycles' own printing on the error stream, and prefixed
    /// from 5.0, where it comes through the log on standard output.
    ///
    /// **The remainder of each line here is illustrative and nothing turns
    /// on it**: only the head is matched, and what follows is the machine's
    /// own words carried as printed.
    #[test]
    fn the_device_line_is_read_in_both_grammars_and_is_carried_as_printed() {
        let carries = "Path tracing on: a processor, its thread count [CPU]";
        assert_eq!(
            read_one_line(FOUR_FIVE, None, carries),
            WhatTheLineWas::TheDeviceLine {
                as_printed: carries.to_string()
            }
        );
        assert_eq!(
            read_one_line(
                FROM_FIVE,
                None,
                &format!("00:00.600  cycles           | {carries}")
            ),
            WhatTheLineWas::TheDeviceLine {
                as_printed: carries.to_string()
            }
        );
    }

    /// **R3: a line that matches no grammar yields nothing and is not an
    /// error.** The complaints stream is not reliably empty on a run that
    /// succeeds and on Linux the shell can add lines of its own.
    #[test]
    fn a_line_matching_no_grammar_yields_nothing_and_never_a_failure() {
        for line in [
            "",
            "bash: warning: setlocale: LC_ALL: cannot change locale",
            "Blender quit",
            "Time: 00:00.22 (Saving: 00:00.05)",
            "00:00.667  render           | Engine: Cycles",
        ] {
            assert_eq!(
                read_one_line(FOUR_FIVE, None, line),
                WhatTheLineWas::NothingReadable
            );
            assert_eq!(
                read_one_line(FROM_FIVE, None, line),
                WhatTheLineWas::NothingReadable
            );
        }
    }

    /// **The stage words are stage words and not sample pairs.** `Finished`
    /// and `Cancelled` end an ordinary progress line, and reading either as a
    /// count would put a figure on the entry that nothing printed.
    #[test]
    fn a_stage_word_leaves_the_sample_pair_absent_and_the_frame_still_read() {
        let read = a_progress_line(read_one_line(
            FOUR_FIVE,
            None,
            "Fra:1 Mem:10.36M (Peak 10.95M) | Time:00:00.16 | Mem:1.48M, Peak:1.48M | \
             Scene, ViewLayer | Finished",
        ));
        assert_eq!(read.frame, Some(1));
        assert_eq!(read.samples, None);
        let read = a_progress_line(read_one_line(
            FROM_FIVE,
            None,
            "00:28.808  render           | Fra: 1 | Mem: 5M | Cancelled",
        ));
        assert_eq!(read.frame, Some(1));
        assert_eq!(read.samples, None);
    }

    /// **`Queue & History` R12's second source, added 2026-11-02 (*Reports 3*,
    /// *Reports 7*, *Reports 11*): a frame cut short in 5.3 goes `Sample
    /// 160/4096`, then `Finished`, with no closing zero.** `Finished` on an
    /// untiled frame short of its count is the mark, by the one rule that reads
    /// every version — **and it harms nothing older or tiled**: the same frame
    /// in 5.2 and 4.5 reads as it did, a frame at its full count sets nothing, a
    /// tiled frame sets nothing, and `Cancelled` is no such word.
    #[test]
    fn finished_marks_an_untiled_frame_cut_short_and_harms_no_older_or_tiled_reading() {
        use crate::readings::{fold_a_line, LiveReadings, SamplePair};
        let fold = |grammar: TheGrammar, lines: &[&str]| {
            let mut held = LiveReadings::default();
            for line in lines {
                if let WhatTheLineWas::AProgressLine(read) = read_one_line(grammar, None, line) {
                    held = fold_a_line(&held, &read);
                }
            }
            held
        };
        let reached = Some(SamplePair { seen: 160, of: 4096 });

        // **5.3 under adaptive sampling**, as *Reports 11* has it.
        let up_to_the_last_sample = [
            "00:01.000  render           | Fra: 1 | Remaining: 00:20.00 | Mem: 5M | Sample 1/4096",
            "00:02.000  render           | Fra: 1 | Remaining: 00:19.00 | Mem: 5M | Sample 160/4096",
        ];
        let held = fold(FROM_FIVE, &up_to_the_last_sample);
        assert_eq!(held.samples, reached);
        assert!(!held.stopped_short, "still sampling");
        let finished = "00:02.500  render           | Fra: 1 | Mem: 5M | Finished";
        let held = fold(FROM_FIVE, &[up_to_the_last_sample[0], up_to_the_last_sample[1], finished]);
        assert_eq!(held.samples, reached, "the figures stay as printed (`Queue & History` R11)");
        assert!(held.stopped_short, "the frame ended short of its count");

        // **4.5 to 5.2's own ending reads the same, and its zero is what marks it.**
        let held = fold(
            FROM_FIVE,
            &[
                up_to_the_last_sample[1],
                "00:02.400  render           | Fra: 1 | Mem: 5M | Sample 0/4096",
                finished,
            ],
        );
        assert_eq!(held.samples, reached);
        assert!(held.stopped_short);
        let held = fold(
            FOUR_FIVE,
            &[
                "Fra:1 Mem:10.36M (Peak 10.95M) | Time:00:00.16 | Mem:1.48M, Peak:1.48M | Scene, ViewLayer | Sample 160/4096",
                "Fra:1 Mem:10.36M (Peak 10.95M) | Time:00:00.16 | Mem:1.48M, Peak:1.48M | Scene, ViewLayer | Sample 0/4096",
                "Fra:1 Mem:10.36M (Peak 10.95M) | Time:00:00.16 | Mem:1.48M, Peak:1.48M | Scene, ViewLayer | Finished",
            ],
        );
        assert_eq!(held.samples, reached);
        assert!(held.stopped_short);

        // **A frame at its full count, in any version, is not short.**
        let held = fold(
            FROM_FIVE,
            &["00:02.000  render           | Fra: 1 | Mem: 5M | Sample 4096/4096", finished],
        );
        assert!(!held.stopped_short);
        let held = fold(
            FOUR_FIVE,
            &[
                "Fra:1 Mem:10.36M (Peak 10.95M) | Time:00:00.16 | Mem:1.48M, Peak:1.48M | Scene, ViewLayer | Sample 16/16",
                "Fra:1 Mem:10.36M (Peak 10.95M) | Time:00:00.16 | Mem:1.48M, Peak:1.48M | Scene, ViewLayer | Finished",
            ],
        );
        assert!(!held.stopped_short);

        // **Tiled: `Finished` is the mark at any count (`Queue & History` R12's
        // line of 2026-11-02), and the figures stay as printed.** The tiled test
        // below reads the whole of it.
        for (last, seen) in [
            ("Rendered 3/4 Tiles, Sample 16/16", 16),
            ("Rendered 3/4 Tiles, Sample 0/16", 0),
            ("Rendered 3/4 Tiles, Sample 4/16", 4),
        ] {
            let held = fold(
                FROM_FIVE,
                &[
                    &format!("00:04.055  render           | Fra: 1 | Mem: 1M | {last}"),
                    finished,
                ],
            );
            assert!(held.stopped_short, "{last}");
            assert_eq!(held.tiles.map(|pair| (pair.done, pair.of)), Some((3, 4)), "{last}");
            assert_eq!(held.samples, Some(SamplePair { seen, of: 16 }), "{last}");
        }

        // **`Cancelled` ends no unit here**: *Reports 11*'s stopped 5.3 frame.
        let held = fold(
            FROM_FIVE,
            &[
                "00:12.000  render           | Fra: 1 | Mem: 5M | Sample 1025/1000000",
                "00:12.100  render           | Fra: 1 | Mem: 5M | Cancelled",
            ],
        );
        assert!(!held.stopped_short);

        // **No count at all, nothing to be short of**: a stream that printed none.
        let held = fold(FROM_FIVE, &[finished]);
        assert!(!held.stopped_short);
        assert_eq!(held.samples, None);
    }

    /// **`Queue & History` R12's line of 2026-11-02 and B3b: `Finished` ends a
    /// frame with tiles too, in every version.** The lines are the ones a real
    /// frame prints: 4.5.12, 5.2.3 and the 5.3.0 Beta at 2049 by 2049 on the
    /// Creator's Mac (observed that day), and 5.3 on Linux and a card under
    /// OptiX (the first live pass, *A5e*). **What it bites on**: a tiled
    /// frame took no mark, so both bars sat where the tile figures put them.
    #[test]
    fn finished_ends_a_frame_with_tiles_in_every_version_and_harms_nothing_else() {
        use crate::readings::{fold_a_line, LiveReadings, SamplePair, TilePair};
        let fold = |grammar: TheGrammar, lines: &[&str]| {
            let mut held = LiveReadings::default();
            for line in lines {
                if let WhatTheLineWas::AProgressLine(read) = read_one_line(grammar, None, line) {
                    held = fold_a_line(&held, &read);
                }
            }
            held
        };

        // **The live pass's own lines, then what comes after `Finished`.**
        let the_pass = [
            "00:39.309  render           | Fra: 2 | Remaining: 00:01.91 | Mem: 574M | Rendered 3/4 Tiles, Sample 48/4096",
            "00:39.430  render           | Fra: 2 | Remaining: 00:01.92 | Mem: 574M | Finished",
        ];
        let held = fold(FROM_FIVE, &the_pass[..1]);
        assert!(!held.stopped_short, "still sampling");
        let held = fold(FROM_FIVE, &the_pass);
        assert!(held.stopped_short, "sampling has ended");
        assert_eq!(held.tiles, Some(TilePair { done: 3, of: 4 }), "as printed");
        assert_eq!(held.samples, Some(SamplePair { seen: 48, of: 4096 }), "as printed");
        let held = fold(
            FROM_FIVE,
            &[
                the_pass[0],
                the_pass[1],
                "00:39.454  render           | Fra: 2 | Remaining: 00:01.92 | Mem: 496M | Reading full buffer from disk",
                "00:40.178  render           | Fra: 2 | Remaining: 00:01.96 | Mem: 496M | ViewLayer | Denoising",
                "01:11.166  render           | Fra: 2 | Remaining: 00:03.63 | Mem: 1413M | ViewLayer | Finishing",
            ],
        );
        assert!(held.stopped_short, "denoising and saving do not take it back");
        assert_eq!(held.tiles, Some(TilePair { done: 3, of: 4 }));

        // **4.5 to 5.2 end the tile lines on a zero and 5.3 on a full count**;
        // the same mark, and each figure as printed.
        let older = fold(
            FROM_FIVE,
            &[
                "00:03.151  render           | Fra: 1 | Mem: 1M | Rendered 3/4 Tiles, Sample 16/16",
                "00:03.151  render           | Fra: 1 | Mem: 1M | Rendered 3/4 Tiles, Sample 0/16",
                "00:03.151  render           | Fra: 1 | Mem: 1M | Finished",
            ],
        );
        assert!(older.stopped_short);
        assert_eq!(older.samples, Some(SamplePair { seen: 0, of: 16 }), "the tile's own, as printed");
        let four_five = fold(
            FOUR_FIVE,
            &[
                "Fra:1 Mem:8.38M (Peak 88.38M) | Time:00:02.71 | Mem:0.09M, Peak:80.09M | Scene, ViewLayer | Rendered 3/4 Tiles, Sample 16/16",
                "Fra:1 Mem:8.38M (Peak 88.38M) | Time:00:02.71 | Mem:0.09M, Peak:80.09M | Scene, ViewLayer | Rendered 3/4 Tiles, Sample 0/16",
                "Fra:1 Mem:8.38M (Peak 88.38M) | Time:00:02.71 | Mem:0.09M, Peak:80.09M | Scene, ViewLayer | Finished",
            ],
        );
        assert!(four_five.stopped_short);
        assert_eq!(four_five.tiles, Some(TilePair { done: 3, of: 4 }));

        // **The zero alone is still no mark**: the frame has not said it ended.
        let held = fold(FROM_FIVE, &["00:03.151  render           | Fra: 1 | Mem: 1M | Rendered 3/4 Tiles, Sample 0/16"]);
        assert!(!held.stopped_short);

        // **A stopped frame prints no `Finished`, and `Finishing` is not it.**
        let held = fold(
            FROM_FIVE,
            &[
                "00:12.000  render           | Fra: 1 | Mem: 5M | Rendered 2/4 Tiles, Sample 1025/1000000",
                "00:12.100  render           | Fra: 1 | Mem: 5M | Cancelled",
                "00:12.200  render           | Fra: 1 | Mem: 5M | ViewLayer | Finishing",
            ],
        );
        assert!(!held.stopped_short);

        // **The next frame and the next layer start over**, so nothing carries.
        let held = fold(
            FROM_FIVE,
            &[
                the_pass[0],
                the_pass[1],
                "00:41.000  render           | Fra: 3 | Mem: 574M | Rendered 0/4 Tiles, Sample 1/4096",
            ],
        );
        assert!(!held.stopped_short);
        assert_eq!(held.tiles, Some(TilePair { done: 0, of: 4 }));
    }

    /// **The grammar branches at 5 and on nothing else** — the same boundary
    /// every other version branch in this step turns on.
    #[test]
    fn the_grammar_branches_at_five_and_on_nothing_else() {
        assert_eq!(
            TheGrammar::for_a_version(4),
            TheGrammar::PlainLinesNamingTheViewLayer
        );
        for major in [5, 6] {
            assert_eq!(
                TheGrammar::for_a_version(major),
                TheGrammar::TimestampedAndSortedWithEveryFrameAnnounced
            );
        }
    }

    /// **No colour-substitution line is matched here**, and the absence is
    /// the value the rules half already states.
    #[test]
    fn nothing_here_matches_a_colour_substitution_line() {
        const _: () = assert!(NO_COLOUR_SUBSTITUTION_LINE_IS_MATCHED_HERE);
        assert_eq!(
            crate::ending::ColourSubstitutionKind::TheSceneViewNotFoundAndADefaultSet
                .the_printed_text(),
            crate::ending::ThePrintedText::NotHeldHere
        );
        // The one thing *Dependencies 15* does quote is the pair of values a
        // script read back after the substitution, which is not a printed
        // line and is matched by nothing.
        for line in [
            "  image colorspace name now = ''",
            "  scene view transform now  = 'TinyView'",
        ] {
            assert_eq!(
                read_one_line(FOUR_FIVE, None, line),
                WhatTheLineWas::NothingReadable
            );
        }
    }
}

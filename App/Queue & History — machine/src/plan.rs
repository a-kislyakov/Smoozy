//! The plan total — B1 and B2, the arithmetic the second bar is measured
//! against, here so the render worker can work B1's total out on the machine.
//!
//! **Moved here from `smoozy-queue-history`'s `bars` on 2026-10-03** (the
//! Creator's notes on 0.1.8, piece 6a), unchanged. B1 says the total *"is
//! worked out on the machine, by the render worker, from the recorded reading
//! the job record holds once `Rendering` P1 step 4 has run"*, and the program
//! the machine runs cannot take that crate. `bars` re-exports all four at the
//! path they have always had, so there is one copy of each.
//!
//! **Constraint:** Row 64 — a renderlet's size is its tiles multiplied by its
//! samples (**Creator-stated**, 2026-08-13), the sum honours whichever sample
//! count Render Settings put in force, and a total is a plan, not a promise
//! (B5).

use smoozy_render_settings_machine::reading::{
    renders_at, FrameAnswer, SceneReading, ViewLayerReading,
};

use crate::readings::{LiveReadings, SamplePair, TilePair};

/// **The atom the plan counts**: one view layer, on one frame, with its
/// planned samples and its tiles.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Renderlet {
    pub frame: i64,
    pub view_layer: String,
    /// The count actually in force for that layer under Render Settings'
    /// rule — carried in the settings payload and the reading, never one
    /// number multiplied by everything (B1).
    pub planned_samples: u64,
    /// B2's arithmetic.
    pub tiles: u64,
}

impl Renderlet {
    /// **Row 64's own rule: a renderlet's size is its tiles multiplied by its
    /// samples.** Label: Creator-stated, 2026-08-13.
    pub fn plan_units(&self) -> u64 {
        self.planned_samples * self.tiles
    }
}

/// **B2: the plan's tile count is Smoozy's own grid arithmetic over the
/// frame's output size and the file's tile size** — the count of tile-sized
/// steps across the width times the count across the height, each rounded
/// up. Reproduces 2048×2048 at the default 2048 giving one and printing no
/// tile figure, and 2049×2049 giving four; observed at five counts, and an
/// area rule is refused because it disagrees at three of them.
///
/// **The threshold at which a picture is split into tiles is permanently
/// unresolved (§18 item 8), and no threshold is written here or anywhere
/// below.** This is the arithmetic and nothing else: no function in this
/// crate answers *is this picture tiled* from a size — the readings answer
/// it off the printed line (R12, [`crate::readings::AProgressLine`]).
pub fn the_grid_tile_count(output_size: (u32, u32), tile_size: (u32, u32)) -> u64 {
    let steps = |length: u32, tile: u32| -> u64 {
        if tile == 0 || length == 0 {
            return 0;
        }
        (u64::from(length)).div_ceil(u64::from(tile))
    };
    steps(output_size.0, tile_size.0) * steps(output_size.1, tile_size.1)
}

/// **B1: for every frame the job covers, the reading already answers which
/// view layers render at that frame (`Job files and outputs` I6); each such
/// pairing is one renderlet.** Composed once, from the recorded reading, and
/// carried on the entry. **B8: the total is computed from the reading's
/// evaluated per-frame set, for every file, and no file is routed to the
/// fallback.**
///
/// A layer whose per-frame answer is *not known without a read* is not in
/// the evaluated set and counts no renderlet: nothing evaluated says it
/// renders, and the plan is a plan (B5). That is Build's own reading of B8
/// and is recorded as such.
pub fn the_renderlets(
    scene: &SceneReading,
    frames: &[i64],
    output_size: (u32, u32),
    tile_size: (u32, u32),
    the_count_in_force_for: impl Fn(&ViewLayerReading) -> u64,
) -> Vec<Renderlet> {
    let tiles = the_grid_tile_count(output_size, tile_size);
    let mut out = Vec::new();
    for frame in frames {
        for layer in &scene.view_layers {
            let renders = match renders_at(layer, *frame) {
                FrameAnswer::Evaluated(value)
                | FrameAnswer::Extrapolated(value)
                | FrameAnswer::FromTheCurve { value, .. } => value,
                FrameAnswer::NotKnownWithoutARead => false,
            };
            if renders {
                out.push(Renderlet {
                    frame: *frame,
                    view_layer: layer.name.as_text().to_string(),
                    planned_samples: the_count_in_force_for(layer),
                    tiles,
                });
            }
        }
    }
    out
}

/// **B1: the total is the sum, per group member separately** — a group is
/// several counts, never one.
pub fn the_plan_total(renderlets: &[Renderlet]) -> u64 {
    renderlets.iter().map(Renderlet::plan_units).sum()
}

/// **B6's finished part, worked out on the machine** — the plan units of every
/// frame finished so far, by the same renderlets [`the_plan_total`] sums, **and
/// the frame in hand's share**, so the second bar is this over that total and
/// both come from the machine.
///
/// **A frame is finished** where the attempt does not ask for it — B7's
/// complement of the attempt's list against the range — or where it stands in
/// the attempt's own list at or before `newest_frame_saved`, P1a's figure:
/// frames are worked through in the order given, so the figure marks the
/// list's finished prefix.
///
/// **A frame of the attempt's list also counts as finished once a later frame
/// of that list has begun** — the frames before the frame in hand's place in
/// the list — because frames are worked in the order given (P1a). The save
/// still counts too: the finished prefix is the longer of the two. This is
/// what moves the second bar for a job that announces no save, such as one
/// whose only output is a video. Added 2026-10-17, the Creator's eleventh
/// notes, note 1, on `Queue & History` B6's text of that day: a
/// 90-frame video job sat at one frame in 90 for its whole render.
///
/// **The frame in hand adds its share** — B6's paragraph corrected 2026-10-04
/// (the Creator's second notes on the test copy, note 13): the planned
/// units of its renderlets that have ended (B3b: the next one began), plus the
/// first bar's fraction of the unit in hand times that unit's planned units
/// ([`the_frame_in_hands_share`]). The spec counts both bars in samples, the
/// second over every frame-and-view-layer pairing, so the work inside the
/// frame in hand counts. Until that day the frame in hand counted nothing
/// until its save, and a one-frame job read nothing on its second bar.
///
/// **The figure never moves backwards**: it is never less than
/// `the_last_figure_sent`, so a count restarting at the next view layer, or a
/// frame that ends without a save, cannot pull it down.
///
/// **Never the total divided by the frames**: view layers switch on and off
/// over the range (`Job files and outputs` I6), so one frame's units are not
/// another's. Added 2026-10-03, the Creator's notes on 0.1.8, piece 6b, on
/// the build runner's ruling of that day.
pub fn the_units_finished(
    renderlets: &[Renderlet],
    the_attempts_frame_list: &[i64],
    newest_frame_saved: Option<i64>,
    the_frame_in_hand: Option<TheFrameInHand<'_>>,
    the_last_figure_sent: u64,
) -> u64 {
    let place_in_the_list =
        |wanted: i64| the_attempts_frame_list.iter().position(|frame| *frame == wanted);
    let saved_through = newest_frame_saved
        .and_then(place_in_the_list)
        .map_or(0, |at| at + 1);
    let begun_before = the_frame_in_hand
        .and_then(|hand| place_in_the_list(hand.frame))
        .unwrap_or(0);
    let the_finished_prefix = &the_attempts_frame_list[..saved_through.max(begun_before)];
    let finished =
        |frame: i64| !the_attempts_frame_list.contains(&frame) || the_finished_prefix.contains(&frame);
    let whole_frames: u64 = renderlets
        .iter()
        .filter(|renderlet| finished(renderlet.frame))
        .map(Renderlet::plan_units)
        .sum();
    let in_hand = match the_frame_in_hand {
        Some(hand) if !finished(hand.frame) => the_frame_in_hands_share(renderlets, hand),
        _ => 0,
    };
    (whole_frames + in_hand).max(the_last_figure_sent)
}

/// **The frame in hand, as the reader holds it** — what B6's share of it is
/// worked out from. Added 2026-10-04, the Creator's second notes, piece 3.
#[derive(Debug, Clone, Copy)]
pub struct TheFrameInHand<'a> {
    pub frame: i64,
    /// **B3b: the view layers of this frame whose renderlet has ended**,
    /// because the next one began. Only a stream that names its view layer can
    /// say so (R10 as corrected 2026-10-04: 4.5's progress line, and from 5.0
    /// the layer's own announcement); empty where no layer is named.
    pub the_layers_ended: &'a [String],
    /// **R10: the view layer in hand, where the stream names one.** `None`
    /// where nothing does — 4.5 under EEVEE, observed at the desk — and B3a
    /// then makes the frame the unit.
    pub the_layer_in_hand: Option<&'a str>,
    /// **R11–R13's readings for the unit in hand alone** — the readings the
    /// entry shows, which [`crate::readings::fold_a_line`] starts over at each
    /// named layer (piece 3b), so they are the renderlet's where a layer is
    /// named and the frame's where none is.
    pub the_unit_in_hand: &'a LiveReadings,
}

/// **B6's share of the frame in hand, as corrected 2026-10-04**: its ended renderlets'
/// planned units, plus the first bar's fraction of the unit in hand times that
/// unit's planned units, capped at the frame's own planned units.
///
/// - **Where the stream names the layer and the plan holds it, the unit is
///   that renderlet** (B3a): its samples against its planned samples (B3),
///   its tiles finished and the tile in hand's samples where the line is
///   tiled ([`the_part_of_a_tiled_renderlet`], B4 as corrected 2026-10-04,
///   piece 3c), and the whole of it where
///   its stopped-short mark is set (B3b). Every version names it under Cycles
///   since R10's correction of 2026-10-04 — 4.5 on its progress line, 5.0 and
///   later in the layer's own announcement.
/// - **Where no layer is named, the unit is the frame** (B3a) — 4.5 under
///   EEVEE, or a name the plan does not hold. A frame of one renderlet is that
///   renderlet, so the rule above holds. **A frame of several counts its
///   readings as a part of one renderlet, the smallest**, because which
///   renderlet they belong to is not on the stream, and **the stopped-short
///   mark does not end it**: with no named boundary the fold runs across the
///   frame's layers, and a later layer's opening `Sample 0/N` after an earlier
///   layer's `N/N` would read as R12's mark (observed at the desk, 2026-10-04,
///   in 5.2.2 and 4.5.12 before the layer was read) — so the frame would read
///   finished while a later layer had all its work still to do. For samples
///   this is B3a's own arithmetic (the largest seen against the frame's
///   planned samples, in units) bar its cap; for tiles it claims no more than
///   one renderlet's share, where B4's figures over the whole frame would
///   claim the frame. The frame is whole on its save, as before.
pub fn the_frame_in_hands_share(renderlets: &[Renderlet], hand: TheFrameInHand<'_>) -> u64 {
    let of_the_frame: Vec<&Renderlet> = renderlets
        .iter()
        .filter(|renderlet| renderlet.frame == hand.frame)
        .collect();
    let the_frames_units: u64 = of_the_frame.iter().map(|renderlet| renderlet.plan_units()).sum();
    let named = hand.the_layer_in_hand.and_then(|layer| {
        of_the_frame
            .iter()
            .find(|renderlet| renderlet.view_layer == layer)
    });
    let share = match (named, of_the_frame.as_slice()) {
        (Some(in_hand), _) => {
            let ended: u64 = of_the_frame
                .iter()
                .filter(|renderlet| {
                    renderlet.view_layer != in_hand.view_layer
                        && hand.the_layers_ended.contains(&renderlet.view_layer)
                })
                .map(|renderlet| renderlet.plan_units())
                .sum();
            ended + the_part_done(hand.the_unit_in_hand, in_hand, true)
        }
        (None, [only]) => the_part_done(hand.the_unit_in_hand, only, true),
        (None, several) => match several.iter().min_by_key(|renderlet| renderlet.plan_units()) {
            Some(smallest) => the_part_done(hand.the_unit_in_hand, smallest, false),
            None => 0,
        },
    };
    share.min(the_frames_units)
}

/// **The first bar's fraction of one renderlet, in its planned units** — B3,
/// B3b and B4 over the readings held for it.
fn the_part_done(held: &LiveReadings, renderlet: &Renderlet, the_mark_ends_it: bool) -> u64 {
    let units = renderlet.plan_units();
    if held.stopped_short && the_mark_ends_it {
        return units;
    }
    let part = |done: u64, of: u64| -> u64 {
        if of == 0 {
            return 0;
        }
        ((u128::from(units) * u128::from(done.min(of))) / u128::from(of)) as u64
    };
    match (held.tiles, held.samples) {
        (Some(tiles), samples) => {
            let (done, of) = the_part_of_a_tiled_renderlet(tiles, samples, renderlet.planned_samples);
            part(done, of)
        }
        (None, Some(samples)) => part(samples.seen, renderlet.planned_samples),
        (None, None) => 0,
    }
}

/// **B4 as corrected 2026-10-04 (the Creator's second notes, piece 3c): the
/// part done of a tiled renderlet is the tiles finished times the samples,
/// plus the samples of the tile in hand, over its tiles times its samples** —
/// the Specs' own arithmetic (*"a renderlet's size is plain arithmetic: its
/// tiles multiplied by its samples"*, Creator-stated 2026-08-13), and the
/// same units B1's total counts. Answered as done against of, in those units.
/// Until that day the bars were filled from the tile figure alone, and both
/// sat at nought through the whole first tile of the Creator's 4K frame.
///
/// - **The tile count is the one Blender printed** (`tiles`), never worked
///   out from the picture — the Specs forbid that. `samples_per_tile` is the
///   renderlet's samples: its planned count on the machine, and the total
///   Blender printed beside the count on the desktop, which holds no plan.
/// - **The tile in hand adds its count only while it is short of the total
///   printed beside it.** The line that counts a tile finished repeats that
///   tile's full count, and the next tile's own count follows it
///   (`Rendered 0/4 Tiles, Sample 1000/1000`, then `Rendered 1/4 Tiles,
///   Sample 1000/1000`, then `Rendered 1/4 Tiles, Sample 1/1000`; observed in
///   all four versions at the desk, 2026-10-04), so a full count beside a
///   tile figure is never the tile in hand's — counting it would put a whole
///   tile on the bar that nothing has rendered.
/// - Where the line carried no count, the tiles finished alone; where the
///   renderlet has no samples, the tile figure alone.
pub fn the_part_of_a_tiled_renderlet(
    tiles: TilePair,
    samples: Option<SamplePair>,
    samples_per_tile: u64,
) -> (u64, u64) {
    if samples_per_tile == 0 {
        return (tiles.done.min(tiles.of), tiles.of);
    }
    let of = tiles.of.saturating_mul(samples_per_tile);
    let finished = tiles.done.min(tiles.of).saturating_mul(samples_per_tile);
    let in_hand = match samples {
        Some(pair) if pair.seen < pair.of && tiles.done < tiles.of => pair.seen.min(samples_per_tile),
        _ => 0,
    };
    (finished.saturating_add(in_hand).min(of), of)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn a_renderlet(frame: i64, layer: &str, samples: u64, tiles: u64) -> Renderlet {
        Renderlet {
            frame,
            view_layer: layer.to_string(),
            planned_samples: samples,
            tiles,
        }
    }

    /// **B6's finished part counts each finished frame's own renderlets** —
    /// so a frame with two layers on counts twice what a frame with one does,
    /// which the total divided by the frames would not.
    #[test]
    fn the_finished_part_counts_each_finished_frames_own_renderlets() {
        // Frames 1–4; frame 2 and frame 4 have a second layer on.
        let plan = vec![
            a_renderlet(1, "Main", 64, 1),
            a_renderlet(2, "Main", 64, 1),
            a_renderlet(2, "Reflections", 128, 1),
            a_renderlet(3, "Main", 64, 1),
            a_renderlet(4, "Main", 64, 4),
            a_renderlet(4, "Reflections", 128, 4),
        ];
        let list = [1, 2, 3, 4];
        assert_eq!(the_units_finished(&plan, &list, None, None, 0), 0, "nothing saved yet");
        assert_eq!(the_units_finished(&plan, &list, Some(1), None, 0), 64);
        assert_eq!(the_units_finished(&plan, &list, Some(2), None, 0), 64 + 64 + 128);
        assert_eq!(the_units_finished(&plan, &list, Some(4), None, 0), the_plan_total(&plan));
        // Two of four frames saved is not half the total here.
        assert_ne!(the_units_finished(&plan, &list, Some(2), None, 0) * 2, the_plan_total(&plan));
    }

    /// **B7's complement counts as finished**: a re-run that does not ask for
    /// frames 1 and 2 starts with their units already in hand.
    #[test]
    fn the_frames_the_attempt_does_not_ask_for_count_as_finished() {
        let plan = vec![
            a_renderlet(1, "Main", 10, 1),
            a_renderlet(2, "Main", 10, 1),
            a_renderlet(3, "Main", 10, 1),
            a_renderlet(4, "Main", 10, 1),
        ];
        assert_eq!(the_units_finished(&plan, &[3, 4], None, None, 0), 20);
        assert_eq!(the_units_finished(&plan, &[3, 4], Some(3), None, 0), 30);
        // A newest saved frame that is not in the list counts nothing more.
        assert_eq!(the_units_finished(&plan, &[3, 4], Some(1), None, 0), 20);
    }

    use crate::readings::{SamplePair, TilePair};

    fn samples(seen: u64, of: u64) -> LiveReadings {
        LiveReadings {
            frame_in_hand: Some(1),
            samples: Some(SamplePair { seen, of }),
            ..LiveReadings::default()
        }
    }

    fn the_frame_in_hand<'a>(
        frame: i64,
        ended: &'a [String],
        layer: Option<&'a str>,
        held: &'a LiveReadings,
    ) -> Option<TheFrameInHand<'a>> {
        Some(TheFrameInHand {
            frame,
            the_layers_ended: ended,
            the_layer_in_hand: layer,
            the_unit_in_hand: held,
        })
    }

    fn one_frame_two_layers_of_a_thousand() -> Vec<Renderlet> {
        vec![a_renderlet(1, "First", 1_000, 1), a_renderlet(1, "Second", 1_000, 1)]
    }

    fn percent(done: u64, of: u64) -> u64 {
        done * 100 / of
    }

    /// **Note 13: the second bar moves inside a frame.** One frame, two layers of
    /// 1,000 samples: the first at 300 is 15 per cent of the job; the first
    /// ended and the second at 500 is 75; the frame saved is 100. **What it
    /// bites on**: the figure of 2026-10-03, which read 0, 0 and 100.
    #[test]
    fn one_frame_of_two_layers_reads_fifteen_then_seventy_five_then_a_hundred() {
        let plan = one_frame_two_layers_of_a_thousand();
        let total = the_plan_total(&plan);
        let held = samples(300, 1_000);
        let at_300 = the_units_finished(&plan, &[1], None, the_frame_in_hand(1, &[], Some("First"), &held), 0);
        assert_eq!(percent(at_300, total), 15);
        let ended = ["First".to_string()];
        let held = samples(500, 1_000);
        let at_500 =
            the_units_finished(&plan, &[1], None, the_frame_in_hand(1, &ended, Some("Second"), &held), at_300);
        assert_eq!(percent(at_500, total), 75);
        let saved =
            the_units_finished(&plan, &[1], Some(1), the_frame_in_hand(1, &ended, Some("Second"), &held), at_500);
        assert_eq!(percent(saved, total), 100);
        // The same with no layer named (4.5 under EEVEE): the readings count as
        // a part of one renderlet, so 300 is still 15 per cent.
        let held = samples(300, 1_000);
        let unnamed = the_units_finished(&plan, &[1], None, the_frame_in_hand(1, &[], None, &held), 0);
        assert_eq!(percent(unnamed, total), 15);
    }

    /// **Never backwards when the second layer's count starts again.** The
    /// first layer at 1,000 and then the second at its own `0` and `1`: the
    /// figure holds at half; and a frame that ends with no save holds where it
    /// stood when the next frame begins.
    #[test]
    fn the_figure_never_moves_backwards_when_a_count_starts_again() {
        let plan = one_frame_two_layers_of_a_thousand();
        // **No layer named**: a frame-wide fold would read the second layer's
        // opening zero as the stopped-short mark (observed at the desk before
        // the layer was read).
        let held = samples(1_000, 1_000);
        let half = the_units_finished(&plan, &[1], None, the_frame_in_hand(1, &[], None, &held), 0);
        assert_eq!(half, 1_000);
        let marked = LiveReadings {
            stopped_short: true,
            ..samples(1_000, 1_000)
        };
        let still_half =
            the_units_finished(&plan, &[1], None, the_frame_in_hand(1, &[], None, &marked), half);
        assert_eq!(still_half, 1_000, "the mark does not finish a frame of two layers");
        // **A layer named**: the renderlet's own fold starts again at the next.
        let ended = ["First".to_string()];
        let held = samples(0, 1_000);
        let restarted =
            the_units_finished(&plan, &[1], None, the_frame_in_hand(1, &ended, Some("Second"), &held), half);
        assert_eq!(restarted, 1_000);
        // A figure already sent is never undercut, even by a reading of nothing.
        assert_eq!(the_units_finished(&plan, &[1], None, None, 1_500), 1_500);
        // A frame of two that moved on with no save: frame 2 begins at nothing.
        // Until 2026-10-17 the figure stayed at what frame 1 had reached (60);
        // since the Creator's eleventh notes, note 1, frame 1 has ended
        // because frame 2 has begun, so it counts whole.
        let two_frames = vec![a_renderlet(1, "Main", 100, 1), a_renderlet(2, "Main", 100, 1)];
        let held = samples(60, 100);
        let sixty = the_units_finished(&two_frames, &[1, 2], None, the_frame_in_hand(1, &[], None, &held), 0);
        assert_eq!(sixty, 60);
        let held = LiveReadings {
            frame_in_hand: Some(2),
            ..LiveReadings::default()
        };
        assert_eq!(
            the_units_finished(&two_frames, &[1, 2], None, the_frame_in_hand(2, &[], None, &held), sixty),
            100
        );
    }

    /// **A ten-frame job at frame 3, halfway through, reads between 20 and 30
    /// per cent** — two frames whole and half of the third.
    #[test]
    fn a_ten_frame_job_halfway_through_frame_three_reads_between_twenty_and_thirty() {
        let plan: Vec<Renderlet> = (1..=10).map(|frame| a_renderlet(frame, "Main", 256, 1)).collect();
        let list: Vec<i64> = (1..=10).collect();
        let held = LiveReadings {
            frame_in_hand: Some(3),
            ..samples(128, 256)
        };
        let done = the_units_finished(&plan, &list, Some(2), the_frame_in_hand(3, &[], None, &held), 0);
        let read = percent(done, the_plan_total(&plan));
        assert!((20..=30).contains(&read) && read == 25, "{read}");
    }

    /// **Piece 3c: B4 as corrected — a tiled renderlet's part done is its
    /// tiles finished times its samples, plus the tile in hand's samples**, off
    /// the lines 5.2.2 printed at the desk in the Creator's 4K shape (four
    /// tiles, 1,000 samples, two layers, one frame). The whole-job figure moves
    /// inside the first tile, a tile's repeated full count is never taken for
    /// the next tile's, and the figure never moves backwards. **What it bites
    /// on**: the tile figure alone, which read nought through the whole first
    /// tile; and a full count counted as the tile in hand, which jumps a whole
    /// tile ahead at each tile's end.
    #[test]
    fn a_tiled_renderlet_counts_its_tiles_finished_and_the_tile_in_hand() {
        let plan = vec![a_renderlet(1, "First", 1_000, 4), a_renderlet(1, "Second", 1_000, 4)];
        assert_eq!(the_plan_total(&plan), 8_000);
        let tiled = |layer: &str, done: u64, seen: u64| LiveReadings {
            frame_in_hand: Some(1),
            view_layer: Some(layer.to_string()),
            tiles: Some(TilePair { done, of: 4 }),
            samples: Some(SamplePair { seen, of: 1_000 }),
            ..LiveReadings::default()
        };
        let mut last = 0;
        let mut read = Vec::new();
        for (done, seen) in [
            (0, 0), (0, 1), (0, 257), (0, 648), (0, 1_000), (1, 1_000), (1, 1), (1, 513),
            (1, 1_000), (2, 1_000), (2, 1), (2, 1_000), (3, 1_000), (3, 1), (3, 1_000), (3, 0),
        ] {
            let held = tiled("First", done, seen);
            last = the_units_finished(&plan, &[1], None, the_frame_in_hand(1, &[], Some("First"), &held), last);
            read.push(last);
        }
        assert_eq!(
            read,
            vec![0, 1, 257, 648, 648, 1_000, 1_001, 1_513, 1_513, 2_000, 2_001, 2_001, 3_000, 3_001, 3_001, 3_001]
        );
        assert_eq!(percent(648, 8_000), 8, "the whole-job bar moves inside the first tile");
        // The second layer begins: the first is whole, and the second's own
        // first tile counts from its first sample.
        let ended = ["First".to_string()];
        let held = tiled("Second", 0, 257);
        last = the_units_finished(&plan, &[1], None, the_frame_in_hand(1, &ended, Some("Second"), &held), last);
        assert_eq!(last, 4_257);
        // The arithmetic itself.
        let tiles = |done| TilePair { done, of: 4 };
        let pair = |seen| Some(SamplePair { seen, of: 1_000 });
        assert_eq!(the_part_of_a_tiled_renderlet(tiles(0), pair(648), 1_000), (648, 4_000));
        assert_eq!(the_part_of_a_tiled_renderlet(tiles(0), pair(1_000), 1_000), (0, 4_000));
        assert_eq!(the_part_of_a_tiled_renderlet(tiles(1), pair(1_000), 1_000), (1_000, 4_000));
        assert_eq!(the_part_of_a_tiled_renderlet(tiles(4), pair(7), 1_000), (4_000, 4_000));
        assert_eq!(the_part_of_a_tiled_renderlet(tiles(2), None, 1_000), (2_000, 4_000));
        assert_eq!(the_part_of_a_tiled_renderlet(tiles(2), pair(900), 500), (1_500, 2_000), "capped at the samples");
        assert_eq!(the_part_of_a_tiled_renderlet(tiles(3), pair(1), 0), (3, 4), "no samples: the tile figure");
        // `Rendered 780/53284 Tiles, Sample 1/1`: each tile one sample, so the
        // tiles finished are the whole of it.
        let many = TilePair { done: 780, of: 53_284 };
        assert_eq!(
            the_part_of_a_tiled_renderlet(many, Some(SamplePair { seen: 1, of: 1 }), 1),
            (780, 53_284)
        );
    }

    /// **B3b, B4 and the cap.** A one-renderlet frame stopped short counts
    /// whole; a tiled line counts by its tile figures; a count past the plan
    /// counts no more than the frame; and a frame the plan holds nothing for
    /// adds nothing.
    #[test]
    fn the_mark_the_tiles_and_the_cap_each_hold() {
        let one = vec![a_renderlet(1, "Main", 500, 4)];
        let marked = LiveReadings {
            stopped_short: true,
            ..samples(200, 500)
        };
        assert_eq!(the_units_finished(&one, &[1], None, the_frame_in_hand(1, &[], None, &marked), 0), 2_000);
        let tiled = LiveReadings {
            tiles: Some(TilePair { done: 3, of: 4 }),
            ..samples(0, 500)
        };
        assert_eq!(the_units_finished(&one, &[1], None, the_frame_in_hand(1, &[], None, &tiled), 0), 1_500);
        let over = samples(900, 500);
        assert_eq!(the_units_finished(&one, &[1], None, the_frame_in_hand(1, &[], None, &over), 0), 2_000);
        // A renderlet past its plan counts no more than itself, so the frame's
        // other layer is not counted done on its behalf.
        let plan = one_frame_two_layers_of_a_thousand();
        let past = samples(1_500, 1_000);
        assert_eq!(
            the_units_finished(&plan, &[1], None, the_frame_in_hand(1, &[], Some("First"), &past), 0),
            1_000
        );
        // A tiled frame of two unnamed layers claims no more than one layer's share.
        let two = vec![a_renderlet(1, "A", 500, 4), a_renderlet(1, "B", 100, 4)];
        let all_tiles = LiveReadings {
            tiles: Some(TilePair { done: 4, of: 4 }),
            ..samples(0, 500)
        };
        assert_eq!(the_units_finished(&two, &[1], None, the_frame_in_hand(1, &[], None, &all_tiles), 0), 400);
        // A layer the plan does not hold: the frame is the unit.
        let held = samples(250, 500);
        assert_eq!(
            the_units_finished(&one, &[1], None, the_frame_in_hand(1, &[], Some("Elsewhere"), &held), 0),
            1_000
        );
        // A frame in hand the plan holds nothing for adds nothing, and one
        // already saved. Frame 1 counts whole here because frame 2 has begun
        // (the Creator's eleventh notes, note 1, 2026-10-17); until then this
        // read nought.
        assert_eq!(the_units_finished(&one, &[1, 2], None, the_frame_in_hand(2, &[], None, &held), 0), 2_000);
        assert_eq!(
            the_units_finished(&one, &[1], Some(1), the_frame_in_hand(1, &[], None, &held), 0),
            2_000,
            "a saved frame is counted once, whole"
        );
    }

    /// Ninety frames of one renderlet of 64 samples, frames 1 to 90, in order.
    fn ninety_frames() -> (Vec<Renderlet>, Vec<i64>) {
        let plan = (1..=90).map(|frame| a_renderlet(frame, "ViewLayer", 64, 1)).collect();
        (plan, (1..=90).collect())
    }

    fn in_hand_at(frame: i64, seen: u64) -> LiveReadings {
        LiveReadings {
            frame_in_hand: Some(frame),
            ..samples(seen, 64)
        }
    }

    /// **The Creator's eleventh notes, note 1: the bar moves for a
    /// job that announces no save.** A 90-frame video job announced no save
    /// at all, and its second bar sat at one frame in 90 for the whole render.
    /// A frame of the attempt's list counts as finished once a later frame of
    /// that list has begun, since frames are worked in the order given: frame
    /// 45 in hand, half done, reads 44 frames' units plus that half. **What it
    /// bites on**: the figure of 2026-10-04, which read the share of frame 45
    /// alone; and a rule that counts the frame in hand itself as finished.
    #[test]
    fn ninety_frames_with_no_save_and_frame_forty_five_in_hand_read_forty_four_frames_and_the_share() {
        let (plan, list) = ninety_frames();
        let held = in_hand_at(45, 32);
        let figure = the_units_finished(&plan, &list, None, the_frame_in_hand(45, &[], None, &held), 0);
        assert_eq!(figure, 44 * 64 + 32);
        // Frame 45 just begun, nothing read for it yet: the 44 before it.
        let begun = in_hand_at(45, 0);
        assert_eq!(
            the_units_finished(&plan, &list, None, the_frame_in_hand(45, &[], None, &begun), 0),
            44 * 64
        );
        // The last frame, all its samples read: 89 whole and the 90th's share,
        // which is the whole of it, so the bar reaches a hundred.
        let last = in_hand_at(90, 64);
        assert_eq!(
            the_units_finished(&plan, &list, None, the_frame_in_hand(90, &[], None, &last), 0),
            the_plan_total(&plan)
        );
        // A frame in hand that is not in the attempt's list begins nothing.
        let stray = in_hand_at(400, 32);
        assert_eq!(the_units_finished(&plan, &list, None, the_frame_in_hand(400, &[], None, &stray), 0), 0);
    }

    /// **A job that announces its saves reads exactly as before.** Each frame's
    /// save is announced before the next frame begins, so every frame the new
    /// rule finishes is one the save already finished. The figures here are
    /// the ones the rule of 2026-10-04 gave, worked out by hand: four frames,
    /// frames 2 and 4 with a second layer, frame 4 tiled.
    #[test]
    fn a_job_that_announces_its_saves_reads_exactly_as_before() {
        let plan = vec![
            a_renderlet(1, "Main", 64, 1),
            a_renderlet(2, "Main", 64, 1),
            a_renderlet(2, "Reflections", 128, 1),
            a_renderlet(3, "Main", 64, 1),
            a_renderlet(4, "Main", 64, 4),
            a_renderlet(4, "Reflections", 128, 4),
        ];
        let list = [1, 2, 3, 4];
        let mut last = 0;
        let mut read = Vec::new();
        let no_layers: [String; 0] = [];
        let main_ended = ["Main".to_string()];
        // (newest saved, frame in hand, layers ended, layer in hand, samples seen of planned)
        let steps: [(Option<i64>, i64, &[String], Option<&str>, u64, u64); 9] = [
            (None, 1, &no_layers, Some("Main"), 32, 64),
            (Some(1), 1, &no_layers, Some("Main"), 64, 64),
            (Some(1), 2, &no_layers, Some("Main"), 16, 64),
            (Some(1), 2, &main_ended, Some("Reflections"), 64, 128),
            (Some(2), 2, &main_ended, Some("Reflections"), 128, 128),
            (Some(2), 3, &no_layers, Some("Main"), 0, 64),
            (Some(3), 3, &no_layers, Some("Main"), 64, 64),
            (Some(3), 4, &no_layers, Some("Main"), 32, 64),
            (Some(4), 4, &main_ended, Some("Reflections"), 128, 128),
        ];
        for (saved, frame, ended, layer, seen, of) in steps {
            let held = LiveReadings {
                frame_in_hand: Some(frame),
                ..samples(seen, of)
            };
            last = the_units_finished(&plan, &list, saved, the_frame_in_hand(frame, ended, layer, &held), last);
            read.push(last);
        }
        assert_eq!(read, vec![32, 64, 80, 192, 256, 256, 320, 448, 1_088]);
        assert_eq!(read[8], the_plan_total(&plan));
    }

    /// **The figure never goes backwards across a frame announced out of step
    /// or a retry.** Frames 1 to 50 begin with no save; frame 45 is then
    /// announced again, as the announcement can be (R9: it prints again on a
    /// step of zero), and the figure holds at what frame 50 had reached. On a
    /// retry whose attempt asks for frames 46 to 90, the frames it does not
    /// ask for count whole from the start (B7), and the frames it begins add
    /// to them. **What it bites on**: the figure of 2026-10-04, which never
    /// passed one frame's units here; and a figure that forgets the last one
    /// sent.
    #[test]
    fn the_figure_never_goes_backwards_across_a_frame_out_of_step_or_a_retry() {
        let (plan, list) = ninety_frames();
        let mut last = 0;
        for frame in 1..=50 {
            let held = in_hand_at(frame, 16);
            let figure = the_units_finished(&plan, &list, None, the_frame_in_hand(frame, &[], None, &held), last);
            assert!(figure >= last, "frame {frame}: {figure} after {last}");
            last = figure;
        }
        assert_eq!(last, 49 * 64 + 16);
        let again = in_hand_at(45, 0);
        let held_at = the_units_finished(&plan, &list, None, the_frame_in_hand(45, &[], None, &again), last);
        assert_eq!(held_at, 49 * 64 + 16, "a frame announced out of step does not pull it down");
        let on = in_hand_at(51, 8);
        assert_eq!(
            the_units_finished(&plan, &list, None, the_frame_in_hand(51, &[], None, &on), held_at),
            50 * 64 + 8
        );
        // A retry asking for frames 46 to 90: 45 frames carried, then frame 60
        // in hand means 46 to 59 have ended too.
        let retry: Vec<i64> = (46..=90).collect();
        assert_eq!(the_units_finished(&plan, &retry, None, None, 0), 45 * 64);
        let held = in_hand_at(46, 32);
        let first = the_units_finished(&plan, &retry, None, the_frame_in_hand(46, &[], None, &held), 0);
        assert_eq!(first, 45 * 64 + 32);
        let held = in_hand_at(60, 32);
        let later = the_units_finished(&plan, &retry, None, the_frame_in_hand(60, &[], None, &held), first);
        assert_eq!(later, 59 * 64 + 32);
        let back = in_hand_at(46, 0);
        assert_eq!(
            the_units_finished(&plan, &retry, None, the_frame_in_hand(46, &[], None, &back), later),
            later
        );
    }
}

//! The two bars, and the countdown that is not offered — `Queue & History`
//! §6 B1–B9.
//!
//! **Constraint:** Row 64 — both bars count samples, a renderlet's size is
//! its tiles multiplied by its samples (**Creator-stated**, 2026-08-13), the
//! sum honours whichever sample count Render Settings put in force, a total
//! is a plan not a promise, and the bars are written to survive it being
//! wrong. Fact J is permanently unresolved and the spec's own fallback is
//! written into the promise.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Job files and outputs` I6, C12, C13, C17.

// **B1 and B2's arithmetic lives in `smoozy-queue-history-machine`** since
// 2026-10-03 (the Creator's notes on 0.1.8, piece 6a), because B1's total is
// worked out by the render worker on the machine. It is re-exported here at the
// path it has always had, so there is one copy.
pub use smoozy_queue_history_machine::plan::{
    the_grid_tile_count, the_plan_total, the_renderlets, Renderlet,
};

use crate::readings::{LiveReadings, SamplePair, TilePair};

/// **R2's one boundary, as the bars read it.** The 4.5 grammar names frame
/// and view layer on the progress line; the 5.0-and-later grammar announces
/// frames, and names the view layer in its own announcement at each layer's
/// start (R10 as corrected 2026-10-04) rather than on the progress line.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhichGrammar {
    FourPointFive,
    FivePointZeroAndLater,
}

/// **B3a: the first bar's unit is per version, because the renderlet
/// boundary is observable in only one of them.** And **B4: where the frame
/// is tiled, the first bar says what it is showing, and what it shows is
/// tiles.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TheFirstBarsUnit {
    /// On 4.5, and on 5.0 and later wherever the layer is named (R10 as
    /// corrected 2026-10-04): the renderlet, and the line above the bar names
    /// the layer.
    TheRenderlet { view_layer: Option<String> },
    /// On 5.0 and later where no layer is named: the frame, against the
    /// frame's whole planned samples, and it does not reset inside a frame.
    TheFrame,
    /// A tiled frame: labelled as tiles, and filled by B4's arithmetic as
    /// corrected 2026-10-04 (piece 3c) — the tiles finished times the samples,
    /// plus the tile in hand's samples, over the tiles times the samples.
    Tiles,
}

/// How full the first bar is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Fill {
    /// Hundredths, never above a hundred.
    Percent(u8),
    /// **B4: where a tiled line matches no grammar, the bar shows no
    /// percentage** and the entry shows the figures it has — the spec's
    /// fallback, taken exactly.
    NoPercentage,
}

/// **The first bar: the unit in hand against its planned work.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FirstBar {
    pub unit: TheFirstBarsUnit,
    pub fill: Fill,
    /// **B3b: the figures are unchanged and are still shown as printed** — a
    /// unit taken to its end still reads 200 of 500 beside it, because R11's
    /// *both figures are shown as printed* is what stops the fill becoming a
    /// claim that 500 samples were taken.
    pub samples_as_printed: Option<SamplePair>,
    pub tiles_as_printed: Option<TilePair>,
}

/// **B3, B3a, B3b and B4, performed.**
///
/// `planned_for_the_unit` is the renderlet's planned count on 4.5 and the
/// frame's whole planned samples — every renderlet B1 counted for that
/// frame, summed — on 5.0 and later. `the_unit_has_ended` is B3b's signal:
/// R12's trailing zero on an untiled frame, or the next unit beginning, or
/// the job's own ending — never a figure.
pub fn the_first_bar(
    grammar: WhichGrammar,
    held: &LiveReadings,
    planned_for_the_unit: u64,
    the_unit_has_ended: bool,
) -> FirstBar {
    if let Some(tiles) = held.tiles {
        // **B4 as corrected 2026-10-04 (piece 3c): the tiles finished times
        // the samples, plus the samples of the tile in hand, over the tiles
        // times the samples** — the Specs' own arithmetic, the one the second
        // bar counts on the machine, so both bars count the same thing. Until
        // that day this was the tile figure alone, and the bar sat at nought
        // through the whole first tile. The tile figure is still not what takes
        // the bar to full: a finished 2049×2049 run ends on 3/4 tiles, so B3b
        // fills it on the unit's ending.
        let fill = if the_unit_has_ended {
            Fill::Percent(100)
        } else if tiles.of == 0 {
            Fill::NoPercentage
        } else {
            let (done, of) = smoozy_queue_history_machine::plan::the_part_of_a_tiled_renderlet(
                tiles,
                held.samples,
                planned_for_the_unit,
            );
            Fill::Percent(percent_of(done, of))
        };
        return FirstBar {
            unit: TheFirstBarsUnit::Tiles,
            fill,
            samples_as_printed: held.samples,
            tiles_as_printed: Some(tiles),
        };
    }
    let unit = match grammar {
        WhichGrammar::FourPointFive => TheFirstBarsUnit::TheRenderlet {
            view_layer: held.view_layer.clone(),
        },
        WhichGrammar::FivePointZeroAndLater => match &held.view_layer {
            Some(layer) => TheFirstBarsUnit::TheRenderlet {
                view_layer: Some(layer.clone()),
            },
            None => TheFirstBarsUnit::TheFrame,
        },
    };
    // **B3b: the bar never moves backwards, and never sits short of full for
    // a unit that has ended.** The first is R11's largest-seen; the second is
    // filling on an ending, which is not a sample reading at all.
    //
    // **A frame in hand with no sample printed yet reads nought** — `Design/
    // Queue & History`, after line 66: *"between a frame starting and
    // Blender's first sample on it, it stands empty, and it is never taken
    // away and put back."* At a frame that has begun, R11's largest-seen
    // count is nought, not unknown. Added 2026-10-18, the Creator's twelfth
    // notes, note 7 (W4); until then this stretch read *no percentage*, and
    // with fast frames most readings landed in it, so the bar kept vanishing.
    // **B4's *no percentage* is untouched**: a tiled line no grammar reads is
    // the arm above, and a reading with no frame in hand still shows none.
    let fill = if the_unit_has_ended || held.stopped_short {
        Fill::Percent(100)
    } else {
        match held.samples {
            Some(pair) if planned_for_the_unit > 0 => {
                Fill::Percent(percent_of(pair.seen, planned_for_the_unit))
            }
            None if held.frame_in_hand.is_some() => Fill::Percent(0),
            _ => Fill::NoPercentage,
        }
    };
    FirstBar {
        unit,
        fill,
        samples_as_printed: held.samples,
        tiles_as_printed: None,
    }
}

fn percent_of(part: u64, whole: u64) -> u8 {
    if whole == 0 {
        return 0;
    }
    ((part.min(whole) * 100) / whole) as u8
}

/// **B3a's fourth bullet: nothing infers a renderlet boundary from the
/// sample reading falling.** A falling or restarting count is a tile
/// boundary where the line carries a tile figure and an ending where it
/// does not, and neither is the boundary this is about. Kept by
/// [`the_first_bar`] taking the unit's ending as an argument rather than
/// deriving it from the figures.
pub const NO_RENDERLET_BOUNDARY_IS_INFERRED_FROM_A_FALLING_COUNT: bool = true;

/// **B6's provenance, stated so nobody later mistakes it for a measured
/// fact.** The unit reconciliation rests on a Blender write rule this
/// document carries as **Creator-stated (2026-07-24)** — the Glossary's
/// *Output* entry: a frame's outputs land only once every view layer active
/// on that frame has finished. **No research question covers this write
/// rule, in any version or on either machine**, so nothing later confirms or
/// overturns it. Breaks if it is wrong: the reconciliation is not exact and
/// a frame counted whole can contribute plan units for work that never
/// landed; what holds: B5's plan-not-promise presentation, and the first
/// bar, which reads the stream rather than the reconciliation.
pub const THE_WRITE_RULE_BEHIND_THE_SECOND_BAR_IS_A_RULING_NOT_AN_OBSERVATION: bool = true;

/// **B5: the second bar is the whole job against the plan total, presented
/// as progress against the plan, and it survives the plan being wrong by
/// saying what it is.** A job can correctly end short of it or pass it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SecondBar {
    ProgressAgainstThePlan {
        done_units: u64,
        plan_total: u64,
        /// Adaptive sampling can finish a correct job short of the total;
        /// B2's tile count can misprice a frame. The bar's claim is bounded
        /// to the plan.
        passed_the_plan: bool,
    },
}

/// **The two bars as the entry draws them, from what the machine carries** —
/// `the_first_bar` over R5's figures, and B6's second bar as B6's
/// paragraph of 2026-10-03, corrected 2026-10-04, has it: the finished part,
/// the frame in hand's share included, over B1's total, both worked out on
/// the machine. **It is the only second bar there is**: an older rule that
/// composed one on the desktop from the drawn first bar was removed on
/// 2026-10-04 (the Creator's second notes, piece 3b). Added 2026-10-03, the
/// Creator's notes on 0.1.8, piece 6b.
///
/// - **The first bar's planned count is the total Blender printed beside the
///   samples**, because the desktop holds no per-renderlet plan: B1's
///   renderlets stay on the machine and only their total and finished part
///   travel. It is Blender's own figure for the piece in hand, shown as
///   printed (R11), and nothing here substitutes Render Settings' number for
///   it. Where no sample pair was printed the bar shows no percentage —
///   **except on a frame in hand of a job the machine carried a plan total
///   for, where it reads nought until the first sample** (W4, 2026-10-18,
///   the Creator's twelfth notes, note 7). Each
///   named view layer starts the readings over (`fold_a_line`, piece 3b), so
///   on a frame of several layers the bar fills once per layer against that
///   layer's printed total, in every version that names the layer.
/// - **The unit's ending is not known from one reading** (B3b's next unit
///   beginning is a later reading's), so only R12's stopped-short mark fills
///   the bar early.
/// - **The second bar is absent where either figure is**, or where the total
///   is nought (no layer renders at any frame): absent stays absent (R4),
///   never a zero.
pub fn the_two_bars_from_what_the_machine_carries(
    grammar: WhichGrammar,
    report: &crate::readings::AWorkersReport,
) -> (FirstBar, Option<SecondBar>) {
    let held = report.readings.value();
    let planned = held.samples.map(|pair| pair.of).unwrap_or(0);
    let mut first = the_first_bar(grammar, held, planned, false);
    // **The empty bar of a frame not yet sampled is for a job whose samples
    // are counted** — one the machine carried a plan total for. Where it
    // carried none, the engine in force may print no sample at all
    // (Workbench prints no progress line, *Reports 8*), and a bar standing
    // empty through a whole job would say no work was done. So there the bar
    // shows no percentage, as before W4.
    if held.samples.is_none() && held.tiles.is_none() && report.the_plan_total.is_none() {
        first.fill = match first.fill {
            Fill::Percent(0) => Fill::NoPercentage,
            other => other,
        };
    }
    let second = match (report.the_units_finished, report.the_plan_total) {
        (Some(done_units), Some(plan_total)) if plan_total > 0 => {
            Some(SecondBar::ProgressAgainstThePlan {
                done_units,
                plan_total,
                passed_the_plan: done_units > plan_total,
            })
        }
        _ => None,
    };
    (first, second)
}

/// **A bar as a whole percentage, for drawing** — `None` where it shows none.
pub fn as_a_percentage(first: &FirstBar) -> Option<u32> {
    match first.fill {
        Fill::Percent(percent) => Some(u32::from(percent)),
        Fill::NoPercentage => None,
    }
}

/// **The second bar as a whole percentage**, never above a hundred (B5), and
/// worked in whole numbers so a fraction like 768 of 1,280 reads 60 and not 59.
pub fn the_second_as_a_percentage(second: &SecondBar) -> Option<u32> {
    match second {
        SecondBar::ProgressAgainstThePlan {
            done_units,
            plan_total,
            ..
        } if *plan_total > 0 => Some(u32::from(percent_of(*done_units, *plan_total))),
        _ => None,
    }
}

/// **B9: no countdown to the end of a job is offered, and the unused
/// permission is recorded here.** No running figure covers a whole animation
/// in any version, the whole-job total is a plan adaptive sampling
/// legitimately undershoots, and Blender's own estimate covers the frame in
/// hand and comes and goes. **A type with no values, so nothing can compose
/// one**: the entry shows Blender's own per-renderlet estimate as Blender's
/// and as rough (R6, R15), and nothing wider is offered.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ACountdownToTheEndOfAJob {}

#[cfg(test)]
mod tests {
    use super::*;
    use smoozy_render_settings::names::ExactName;
    use smoozy_render_settings::reading::{
        DeviceSetting, EarlyStop, LayerSamplesRule, PerEngineCounts, SceneReading,
        SimplifyReadBack, SwitchAnimation, ViewLayerReading,
    };

    fn a_layer(name: &str, on: bool) -> ViewLayerReading {
        ViewLayerReading {
            name: ExactName::as_the_reading_produced_it(name),
            renders_at: (1..=4).map(|frame| (frame, on)).collect(),
            animation: SwitchAnimation::None { value: on },
            own_sample_count: 0,
            overrides_the_scenes_count: false,
        }
    }

    fn a_scene(layers: Vec<ViewLayerReading>) -> SceneReading {
        SceneReading {
            name: ExactName::as_the_reading_produced_it("Kitchen"),
            frame_start: 1,
            frame_end: 4,
            frame_step: 1,
            frame_count: 4,
            full_size: (1920, 1080),
            resolution_percentage: 100,
            size_at_the_files_percentage: (1920, 1080),
            engine_as_read: ExactName::as_the_reading_produced_it("CYCLES"),
            sample_counts: PerEngineCounts {
                cycles: 64,
                eevee: 64,
            },
            device: DeviceSetting::TheCards,
            early_stop: EarlyStop {
                adaptive_sampling_is_on: true,
                threshold: 0.01,
                minimum: 0,
            },
            time_limit: 0.0,
            layer_samples_rule: LayerSamplesRule::Use,
            saved_with: "4.5.12".to_string(),
            view_layers: layers,
            pulls_in: Vec::new(),
            simplify: SimplifyReadBack {
                use_simplify: false,
                subdivision_render: 6,
                child_particles_render: 1.0,
                texture_limit_render: "OFF".to_string(),
            },
        }
    }

    fn held(seen: u64, of: u64, tiles: Option<(u64, u64)>) -> LiveReadings {
        LiveReadings {
            frame_in_hand: Some(1),
            samples: Some(SamplePair { seen, of }),
            tiles: tiles.map(|(done, of)| TilePair { done, of }),
            ..LiveReadings::default()
        }
    }

    #[test]
    fn the_grid_reproduces_the_boundary_counts_and_the_unequal_sided_case() {
        assert_eq!(the_grid_tile_count((2048, 2048), (2048, 2048)), 1);
        assert_eq!(the_grid_tile_count((2049, 2049), (2048, 2048)), 4);
        assert_eq!(the_grid_tile_count((2048, 2049), (2048, 2048)), 2);
        assert_eq!(the_grid_tile_count((1920, 1080), (2048, 2048)), 1);
        assert_eq!(the_grid_tile_count((1920, 1080), (0, 0)), 0);
    }

    #[test]
    fn an_area_rule_is_not_what_is_built() {
        // 2049×2049 by area over 2048² is 1.0009…, which an area rule rounds
        // to two and the grid counts as four.
        let by_area = (2049u64 * 2049).div_ceil(2048 * 2048);
        assert_ne!(the_grid_tile_count((2049, 2049), (2048, 2048)), by_area);
    }

    #[test]
    fn the_plan_counts_one_renderlet_per_active_layer_per_frame_and_sums_their_units() {
        let scene = a_scene(vec![a_layer("ViewLayer", true), a_layer("Detail", false)]);
        let renderlets = the_renderlets(&scene, &[1, 2, 3, 4], (1920, 1080), (2048, 2048), |_| 64);
        assert_eq!(renderlets.len(), 4, "the switched-off layer counts nothing");
        assert!(renderlets
            .iter()
            .all(|renderlet| renderlet.view_layer == "ViewLayer"));
        assert_eq!(the_plan_total(&renderlets), 4 * 64);
        // Tiles multiply: the same job at 2049×2049 is four tiles a renderlet.
        let tiled = the_renderlets(&scene, &[1, 2, 3, 4], (2049, 2049), (2048, 2048), |_| 64);
        assert_eq!(the_plan_total(&tiled), 4 * 64 * 4);
    }

    #[test]
    fn a_count_not_known_without_a_read_counts_no_renderlet() {
        let mut driven = a_layer("Driven", true);
        driven.renders_at = vec![(1, true)];
        driven.animation = SwitchAnimation::Driven {
            expression: "frame > 1".to_string(),
            valid: true,
        };
        let scene = a_scene(vec![driven]);
        let renderlets = the_renderlets(&scene, &[1, 2], (1920, 1080), (2048, 2048), |_| 8);
        assert_eq!(renderlets.len(), 1);
        assert_eq!(renderlets[0].frame, 1);
    }

    #[test]
    fn the_first_bar_is_the_renderlet_where_a_layer_is_named_and_the_frame_where_none_is() {
        let readings = LiveReadings {
            view_layer: Some("Reflections".to_string()),
            ..held(100, 500, None)
        };
        let old = the_first_bar(WhichGrammar::FourPointFive, &readings, 500, false);
        assert_eq!(
            old.unit,
            TheFirstBarsUnit::TheRenderlet {
                view_layer: Some("Reflections".to_string())
            }
        );
        assert_eq!(old.fill, Fill::Percent(20));
        let new = the_first_bar(WhichGrammar::FivePointZeroAndLater, &held(100, 500, None), 1500, false);
        assert_eq!(new.unit, TheFirstBarsUnit::TheFrame);
        assert_eq!(new.fill, Fill::Percent(6));
        // **R10 as corrected 2026-10-04**: from 5.0 a named layer is the unit.
        let named = the_first_bar(WhichGrammar::FivePointZeroAndLater, &readings, 500, false);
        assert_eq!(
            named.unit,
            TheFirstBarsUnit::TheRenderlet {
                view_layer: Some("Reflections".to_string())
            }
        );
        assert_eq!(named.fill, Fill::Percent(20));
    }

    #[test]
    fn a_unit_that_has_ended_takes_its_bar_to_the_end_and_the_figures_still_read_as_printed() {
        let readings = LiveReadings {
            stopped_short: true,
            ..held(200, 500, None)
        };
        let bar = the_first_bar(WhichGrammar::FivePointZeroAndLater, &readings, 500, false);
        assert_eq!(bar.fill, Fill::Percent(100));
        assert_eq!(
            bar.samples_as_printed,
            Some(SamplePair { seen: 200, of: 500 })
        );
        // And on the next unit beginning or the job's ending, with no mark.
        let by_ending = the_first_bar(
            WhichGrammar::FivePointZeroAndLater,
            &held(200, 500, None),
            500,
            true,
        );
        assert_eq!(by_ending.fill, Fill::Percent(100));
    }

    #[test]
    fn a_tiled_frame_is_shown_as_tiles_and_is_taken_to_full_by_its_ending_not_its_figures() {
        let readings = held(0, 16, Some((3, 4)));
        let live = the_first_bar(WhichGrammar::FivePointZeroAndLater, &readings, 16, false);
        assert_eq!(live.unit, TheFirstBarsUnit::Tiles);
        assert_eq!(live.fill, Fill::Percent(75));
        assert_eq!(
            live.samples_as_printed,
            Some(SamplePair { seen: 0, of: 16 })
        );
        let ended = the_first_bar(WhichGrammar::FivePointZeroAndLater, &readings, 16, true);
        assert_eq!(ended.fill, Fill::Percent(100));
        // A tiled line whose total could not be read: no percentage.
        let unread = the_first_bar(
            WhichGrammar::FivePointZeroAndLater,
            &held(0, 16, Some((0, 0))),
            16,
            false,
        );
        assert_eq!(unread.fill, Fill::NoPercentage);
    }

    /// **Piece 3c: B4 as corrected — on a tiled frame the first bar moves
    /// inside a tile**, off the figures 5.2.2 printed at the desk in the
    /// Creator's 4K shape (four tiles, 1,000 samples), drawn the way the entry
    /// draws them, against the total printed beside the count. **What it bites
    /// on**: the tile figure alone, which read 0 per cent through the whole
    /// first tile; and a tile's repeated full count read as the next tile's,
    /// which reads 50 per cent the moment the first tile ends.
    #[test]
    fn a_tiled_frames_first_bar_counts_the_tiles_finished_and_the_tile_in_hand() {
        use crate::readings::AWorkersReport;
        use smoozy_state::observed::{Clock, Reading};
        let drawn = |done: u64, seen: u64| {
            let report = AWorkersReport {
                attempt: 1,
                readings: Reading::taken(
                    LiveReadings {
                        view_layer: Some("First".to_string()),
                        ..held(seen, 1_000, Some((done, 4)))
                    },
                    std::time::SystemTime::UNIX_EPOCH,
                    Clock::ThisComputer,
                ),
                newest_frame_saved: None,
                the_plan_total: None,
                the_units_finished: None,
            };
            let (first, _) =
                the_two_bars_from_what_the_machine_carries(WhichGrammar::FivePointZeroAndLater, &report);
            assert_eq!(first.unit, TheFirstBarsUnit::Tiles, "still labelled as tiles");
            as_a_percentage(&first)
        };
        let read: Vec<Option<u32>> = [(0, 1), (0, 257), (0, 648), (1, 1_000), (1, 513), (3, 1)]
            .into_iter()
            .map(|(done, seen)| drawn(done, seen))
            .collect();
        assert_eq!(read, vec![Some(0), Some(6), Some(16), Some(25), Some(37), Some(75)]);
    }

    #[test]
    fn the_first_bar_never_reads_above_full_and_shows_no_percentage_with_no_reading() {
        let over = the_first_bar(
            WhichGrammar::FourPointFive,
            &held(600, 500, None),
            500,
            false,
        );
        assert_eq!(over.fill, Fill::Percent(100));
        let none = the_first_bar(
            WhichGrammar::FourPointFive,
            &LiveReadings::default(),
            500,
            false,
        );
        assert_eq!(none.fill, Fill::NoPercentage);
    }

    /// **W4, the Creator's twelfth notes, note 7: a frame in hand with no
    /// sample printed yet reads nought, and the bar is there** — `Design/Queue
    /// & History`, after line 66. Between one frame ending and the next one's
    /// first sample line the readings hold the frame and no sample pair; that
    /// stretch read *no percentage*, so the row and the entry lost the bar.
    /// **A line no grammar reads still shows no percentage** (B4's tiled line
    /// whose total could not be read), and so does a reading with no frame in
    /// hand. And where the machine carried no plan total the bar stays absent,
    /// so an engine that prints no sample never stands empty for a whole job.
    ///
    /// **What it bites on**: the frame in hand with no sample read as *no
    /// percentage* (the state before W4), and the nought spreading to the
    /// unreadable tiled line or to a reading with no frame.
    #[test]
    fn w4_a_frame_in_hand_with_no_sample_yet_reads_nought_and_an_unread_line_still_has_none() {
        use crate::readings::AWorkersReport;
        use smoozy_state::observed::{Clock, Reading};
        let begun = LiveReadings {
            frame_in_hand: Some(12),
            frames_completed: 11,
            ..LiveReadings::default()
        };
        for grammar in [WhichGrammar::FourPointFive, WhichGrammar::FivePointZeroAndLater] {
            let bar = the_first_bar(grammar, &begun, 0, false);
            assert_eq!(bar.fill, Fill::Percent(0), "a frame begun reads nought");
            assert_eq!(as_a_percentage(&bar), Some(0));
            assert_eq!(bar.samples_as_printed, None, "and no figure is made up beside it");
        }
        // **A line no grammar reads**: a tiled line whose total could not be
        // read, on a frame in hand, with no sample pair — still no percentage.
        let unread = LiveReadings {
            tiles: Some(TilePair { done: 0, of: 0 }),
            ..begun.clone()
        };
        assert_eq!(
            the_first_bar(WhichGrammar::FivePointZeroAndLater, &unread, 0, false).fill,
            Fill::NoPercentage
        );
        // No frame in hand: nothing has begun, so nothing is drawn.
        assert_eq!(
            the_first_bar(WhichGrammar::FivePointZeroAndLater, &LiveReadings::default(), 0, false).fill,
            Fill::NoPercentage
        );
        // **As the entry draws it**: with the machine's plan total, nought; with
        // none, absent.
        let report = |held: LiveReadings, total: Option<u64>| AWorkersReport {
            attempt: 1,
            readings: Reading::taken(held, std::time::SystemTime::UNIX_EPOCH, Clock::ThisComputer),
            newest_frame_saved: Some(11),
            the_plan_total: total,
            the_units_finished: total.map(|_| 1_408),
        };
        let (first, second) = the_two_bars_from_what_the_machine_carries(
            WhichGrammar::FivePointZeroAndLater,
            &report(begun.clone(), Some(4_096)),
        );
        assert_eq!(as_a_percentage(&first), Some(0), "the top bar stays, empty");
        assert_eq!(second.as_ref().and_then(the_second_as_a_percentage), Some(34));
        let (first, _) = the_two_bars_from_what_the_machine_carries(
            WhichGrammar::FivePointZeroAndLater,
            &report(unread, Some(4_096)),
        );
        assert_eq!(as_a_percentage(&first), None, "an unread line still has no percentage");
        let (first, _) = the_two_bars_from_what_the_machine_carries(
            WhichGrammar::FivePointZeroAndLater,
            &report(begun, None),
        );
        assert_eq!(as_a_percentage(&first), None, "no plan total, no empty bar");
    }

    /// **Piece 6b: an entry with figures composes two bars, and one without
    /// composes none.** The second bar is the machine's finished part over its
    /// total — never the total divided by the frames — and absent where either
    /// figure is.
    #[test]
    fn the_two_bars_come_from_what_the_machine_carries_and_absent_stays_absent() {
        use crate::readings::AWorkersReport;
        use smoozy_state::observed::{Clock, Reading};
        let report = |held: LiveReadings, finished: Option<u64>, total: Option<u64>| AWorkersReport {
            attempt: 1,
            readings: Reading::taken(held, std::time::SystemTime::UNIX_EPOCH, Clock::ThisComputer),
            newest_frame_saved: None,
            the_plan_total: total,
            the_units_finished: finished,
        };
        // 812 of 2048 printed, a quarter of the plan finished.
        let (first, second) = the_two_bars_from_what_the_machine_carries(
            WhichGrammar::FivePointZeroAndLater,
            &report(held(812, 2048, None), Some(1_024), Some(4_096)),
        );
        assert_eq!(first.unit, TheFirstBarsUnit::TheFrame);
        assert_eq!(as_a_percentage(&first), Some(39));
        assert_eq!(first.samples_as_printed, Some(SamplePair { seen: 812, of: 2048 }));
        let second = second.expect("both figures carried");
        assert_eq!(the_second_as_a_percentage(&second), Some(25));
        // A tiled line: the tile figures fill it, and the samples stay the tile's.
        let (first, _) = the_two_bars_from_what_the_machine_carries(
            WhichGrammar::FivePointZeroAndLater,
            &report(held(12, 16, Some((780, 53_284))), None, None),
        );
        assert_eq!(first.unit, TheFirstBarsUnit::Tiles);
        assert_eq!(as_a_percentage(&first), Some(1));
        // **Absent stays absent**: no sample pair, no percentage; no total or
        // no finished part, no second bar — never a zero.
        let (first, second) = the_two_bars_from_what_the_machine_carries(
            WhichGrammar::FourPointFive,
            &report(LiveReadings { frame_in_hand: Some(3), ..LiveReadings::default() }, Some(10), None),
        );
        assert_eq!(as_a_percentage(&first), None);
        assert_eq!(second, None);
        let (_, second) = the_two_bars_from_what_the_machine_carries(
            WhichGrammar::FourPointFive,
            &report(held(1, 2, None), None, Some(100)),
        );
        assert_eq!(second, None);
        let (_, second) = the_two_bars_from_what_the_machine_carries(
            WhichGrammar::FourPointFive,
            &report(held(1, 2, None), Some(0), Some(0)),
        );
        assert_eq!(second, None, "a total of nought shows no percentage");
        // A plan that ran short or long reads within the bar (B5).
        let (_, second) = the_two_bars_from_what_the_machine_carries(
            WhichGrammar::FourPointFive,
            &report(held(1, 2, None), Some(120), Some(100)),
        );
        assert_eq!(second.as_ref().and_then(the_second_as_a_percentage), Some(100));
    }

    #[test]
    fn no_countdown_can_be_composed() {
        // An empty enum has no values; the type exists so the refusal has a
        // name, and this is the whole of what can be said about it.
        fn never(_: ACountdownToTheEndOfAJob) {}
        let _ = never;
        assert_eq!(std::mem::size_of::<Option<ACountdownToTheEndOfAJob>>(), 0);
    }
}

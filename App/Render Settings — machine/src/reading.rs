//! The settings half of the one recorded reading — `Render Settings` §1.1
//! Q1–Q3, and the per-frame answer it supplies.
//!
//! **Q1: the recorded reading of `Job files and outputs` R4 also holds the
//! settings payload — payload two of Fact 5 — and it is the same single read,
//! never a second invocation.** R1's own statement is that all three payloads
//! come out of one read. A second reading shape here would be the fault R9
//! exists to prevent, so what this file holds is the *shape* of the settings
//! half, filled by the acting half out of the one document.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Job files and outputs` R1, R4, R9. `Rendering`
//! A1d. `Server Setup` A5.

use serde::{Deserialize, Serialize};

use crate::names::ExactName;

/// **Q2: what the settings half holds, itemised, per version-correct
/// evaluation.**
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SettingsHalf {
    /// **Item 1 and item 2, per scene — for every scene the file holds, never
    /// only the one in hand**, which is the spec's *one read covers the whole
    /// file*.
    pub scenes: Vec<SceneReading>,
    /// **Item 4: the two offered engines' identifiers in the reading
    /// Blender.** Observed, 2026-08-19 test, and the negative is the finding:
    /// the property's enumeration lists only the built-in identifier and not
    /// Cycles; the class walk lists `CYCLES` and not the built-in. Neither
    /// route alone answers, so the reading takes both.
    pub offered_engine_identifiers: OfferedEngineIdentifiers,
    /// **Item 5: the one add-on case a file announces** (E5).
    pub announced_add_on_case: Option<AnnouncedAddOnCase>,
    /// **Item 7: the folders standing directly under the machine's root**, so
    /// `Job files and outputs` P21's note can ask `Job files and outputs` P20's first question of the
    /// machine's own answer. **Nothing where the list would not read, or a
    /// server program older than this took the reading**, and nothing is then
    /// assumed: serde's own rule for an absent optional field.
    pub the_folders_at_the_machines_top: Option<Vec<String>>,
}

/// **Q2 item 1**, per scene.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SceneReading {
    /// `Render Settings` Q2b: R4 item 8's existing answer. Old readings are unknown.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prepare_simulations: Option<bool>,
    /// `Render Settings` Q2b: saved shutter; absent in older readings.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub motion_blur: Option<MotionBlur>,
    pub name: ExactName,
    pub frame_start: i64,
    pub frame_end: i64,
    pub frame_step: i64,
    pub frame_count: u64,
    /// The full size in pixels, both axes.
    pub full_size: (u32, u32),
    pub resolution_percentage: u32,
    /// **The size the percentage works out to — Blender's own answer at the
    /// file's own percentage**, carried and not computed here (Q2a's fifth
    /// bullet).
    pub size_at_the_files_percentage: (u32, u32),
    /// The engine as read, as the reading version's own identifier (E4's
    /// `engine_as_read`).
    pub engine_as_read: ExactName,
    /// The per-engine sample counts, all present at once.
    pub sample_counts: PerEngineCounts,
    pub device: DeviceSetting,
    /// The early-stop setup: whether adaptive sampling is on, its threshold
    /// and its minimum.
    pub early_stop: EarlyStop,
    pub time_limit: f64,
    /// **`Render Settings` Q2 item 1, from 5.3: EEVEE's own time limit.** Absent
    /// where the version has none, and from a reading taken before 2026-11-02.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub eevee_time_limit: Option<f64>,
    /// The layer-samples rule's value — identifiers **observed (2026-08-19
    /// test)**.
    pub layer_samples_rule: LayerSamplesRule,
    /// The saved-with version, as the file records it.
    pub saved_with: String,
    /// **Item 2, per view layer.**
    pub view_layers: Vec<ViewLayerReading>,
    /// **Item 3: whether this scene pulls in another, by both routes** —
    /// consumed for the warning at the spec's *A scene that pulls in another
    /// scene*.
    pub pulls_in: Vec<PulledIn>,
    /// **Item 6: the Simplify group — the switch, and the render half of each
    /// of the three offered settings.** Added 2026-08-29 per Row 472. **The
    /// value carried is the read-back value and never a documented default**
    /// (`Rendering` A1d).
    pub simplify: SimplifyReadBack,
}

/// `Rendering` L10b: how far the saved shutter reaches past the frame.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MotionBlur {
    pub enabled: bool,
    pub position: ShutterPosition,
    pub shutter: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ShutterPosition {
    #[serde(rename = "START")]
    Start,
    #[serde(rename = "CENTER")]
    Center,
    #[serde(rename = "END")]
    End,
}

impl MotionBlur {
    pub fn last_played_frame(&self, last: i64) -> i64 {
        if !self.enabled { return last; }
        let reach = match self.position {
            ShutterPosition::Start => self.shutter,
            ShutterPosition::Center => self.shutter * 0.5,
            ShutterPosition::End => 0.0,
        };
        last.saturating_add(reach.ceil() as i64)
    }
}

/// The per-engine sample counts a scene carries, all present at once. The two
/// offered engines' counts are numbers; Workbench's is a fixed list of
/// choices rather than a count (C9) and this page never composes it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct PerEngineCounts {
    pub cycles: u64,
    pub eevee: u64,
}

/// The scene's own device setting — cards or processor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeviceSetting {
    TheCards,
    TheProcessor,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct EarlyStop {
    pub adaptive_sampling_is_on: bool,
    pub threshold: f64,
    pub minimum: u64,
}

/// **The three answers of the layer-samples rule** — `USE`, `BOUNDED`,
/// `IGNORE` — identifiers observed in this document's own 2026-08-19 test.
/// These are the **values** the rule takes; the name of the property they are
/// assigned to was §13 item 29's and was read on 2026-09-20 — C10, and
/// [`crate::spellings::LAYER_SAMPLES_RULE`]. **The name was found by these
/// three values rather than the other way round**: it is the one property
/// anywhere in any of the four versions that offers exactly them.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LayerSamplesRule {
    Use,
    Bounded,
    Ignore,
}

impl LayerSamplesRule {
    /// The identifier as observed, which is what a pair assigns.
    pub fn identifier(self) -> &'static str {
        match self {
            LayerSamplesRule::Use => "USE",
            LayerSamplesRule::Bounded => "BOUNDED",
            LayerSamplesRule::Ignore => "IGNORE",
        }
    }
}

/// **Q2 item 2**, per view layer.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ViewLayerReading {
    /// The name, exactly as the file's own bytes.
    pub name: ExactName,
    /// **Q3's stepped table**: whether it renders at each frame of the
    /// evaluated span, as (frame, renders).
    pub renders_at: Vec<(i64, bool)>,
    /// Whether the switch is animated, and by what.
    pub animation: SwitchAnimation,
    /// The layer's own sample count, with whether it overrides the scene's.
    pub own_sample_count: u64,
    pub overrides_the_scenes_count: bool,
}

/// How the view-layer switch is animated, with the curve's keys where
/// keyframed and the driver's expression and validity where driven
/// (**observed, 2026-08-19 test**, both versions).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SwitchAnimation {
    /// No animation: the switch holds one value everywhere.
    None {
        value: bool,
    },
    Keyframed {
        keys: Vec<Key>,
        /// **Q3's two extrapolation constants.** Frames past the last key held
        /// the last key's value in both versions; **the before-first side was
        /// not posed** and is §13 item 8 — the read records it, and it is
        /// labelled as recorded rather than as established.
        before_the_first_key: bool,
        after_the_last_key: bool,
    },
    Driven {
        expression: String,
        valid: bool,
    },
    /// A switch a stored script drives under an enabled preference (V3).
    DrivenByAStoredScript,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Key {
    pub frame: i64,
    pub value: bool,
    pub interpolation: Interpolation,
}

/// The interpolation kind a key carries. **Constant is the one Q3's curve
/// answer is exact for**; every other kind is labelled computed rather than
/// evaluated (§13 item 8).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Interpolation {
    Constant,
    Other,
}

/// **Q2 item 3**: one scene pulling in another, by the route it does it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PulledIn {
    pub scene: ExactName,
    pub route: PullInRoute,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PullInRoute {
    TheCompositor,
    TheVideoSequencer,
}

/// **Q2 item 4**: the identifiers the two offered engines carry in the
/// reading Blender, or an absence where that build does not register one —
/// which is E2's refusal at the anchor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OfferedEngineIdentifiers {
    /// From the engine-class walk; `CYCLES` where the add-on is on.
    pub cycles: Option<ExactName>,
    /// The built-in real-time engine's identifier from the engine property's
    /// own enumeration — `BLENDER_EEVEE_NEXT` in 4.5, `BLENDER_EEVEE` from
    /// 5.0.
    pub eevee: Option<ExactName>,
}

impl OfferedEngineIdentifiers {
    pub fn both(&self) -> Vec<&ExactName> {
        self.cycles.iter().chain(self.eevee.iter()).collect()
    }
}

/// **E5: the one announcement — a Blender lacking the file's engine's add-on
/// saying so the moment the file is opened — recorded by the reader in
/// whatever form it arrives, as a field of the reading, and handed to `Server
/// Setup` as that one case and never a list** (that document's A5).
///
/// **What form it takes to a run with no window is not established** (§13
/// item 5), and no second mechanism is written in: the reader records what
/// arrived, and nothing here reads Blender's printed output to compose it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AnnouncedAddOnCase {
    /// The announcement as it arrived.
    pub as_it_arrived: String,
    /// **The 4.5-only trace**: a file saved with Cycles leaving the add-on's
    /// own name listed on the scene when read by a Blender with Cycles off,
    /// and nothing listed from 5.0 (*Add-ons 1*). A version note on the same
    /// field, not a second mechanism.
    pub the_add_on_name_left_on_the_scene: Option<String>,
}

/// **Q2 item 6: the Simplify group's read-back values** — the switch and the
/// render half of each of the three offered settings, reached by the scene's
/// Cycles-block path and present whatever the active engine.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SimplifyReadBack {
    pub use_simplify: bool,
    pub subdivision_render: u32,
    pub child_particles_render: f64,
    /// The texture limit is a pick over the file's own enumeration (`Rendering`
    /// A1d), carried as the identifier read back.
    pub texture_limit_render: String,
}

/// **Q2a: the pixel figure beside the percentage is computed on the desktop,
/// per axis, by truncation, and it costs no read** (Row 366).
///
/// **The rule: each axis on its own — the axis's full pixel size multiplied by
/// the percentage, divided by a hundred, the fraction discarded. Never
/// rounded.** Observed out of the written files in all four versions on both
/// of the research's machines (*Settings 15*): 1920×1080 at 200 % wrote
/// 3840×2160; at 33 %, 633×356; at 1 %, 19×10; and 4×4 at 32767 %, 1310×1310.
/// The two axes whose products end .6 and .8 still came back down, which is
/// what rules rounding out. The bound is §13 item 25.
///
/// **A figure of zero on an axis is producible and is left as the figure it
/// is** — a request working out to less than a pixel is refused by Blender
/// with `Image too small` and nothing is written. This document supplies the
/// figure and adds no warning of its own.
///
/// **Every whole percentage from 1 to 32767 is producible against a standing
/// reading, and no other value ever reaches the arithmetic**: the caller
/// hands in [`crate::refusals::percentage_box`]'s accepted value or the
/// reading's own, and the function refuses to compute from anything else.
pub fn the_size_at(full_size: (u32, u32), percentage: u32) -> Option<(u32, u32)> {
    if !crate::refusals::PERCENTAGE_RANGE.contains(&percentage) {
        return None;
    }
    let axis = |pixels: u32| -> u32 {
        // u64 arithmetic: 65535 × 32767 fits with room, and the division
        // truncates, which is the whole rule.
        ((pixels as u64 * percentage as u64) / 100) as u32
    };
    Some((axis(full_size.0), axis(full_size.1)))
}

/// **Q2a: the percentage in force** — the store's entry where the user
/// answered, else the standing reading's own (J1). Typing a percentage starts
/// no read.
pub fn the_percentage_in_force(users_entry: Option<u32>, reading: &SceneReading) -> u32 {
    users_entry.unwrap_or(reading.resolution_percentage)
}

/// **Q3's answer for one frame**, labelled by how it was got.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrameAnswer {
    /// Inside the evaluated span: the stepped table's own value.
    Evaluated(bool),
    /// Outside the span, on a side that has an extrapolation constant.
    Extrapolated(bool),
    /// Inside the key span but outside the table: answered from the curve
    /// data. **Exact where every key is constant-interpolation**, and labelled
    /// computed rather than evaluated where any key carries another kind.
    FromTheCurve { value: bool, exact: bool },
    /// **A driven switch has no key frames, so the span is the scene's own
    /// frame range alone and there is no extrapolation constant either side of
    /// it**; a frame outside it is answered *not known without a read* and
    /// labelled as such. The same for a switch a stored script drives (V3).
    /// The residue is §13 item 7.
    NotKnownWithoutARead,
}

/// **Q3: the per-frame view-layer answer is evaluated by stepping frames
/// inside the read, over a stated span, with the two extrapolation constants
/// beside it.** A frame inside the span is answered by the table; a frame
/// outside it by the extrapolation constant; a frame inside the key span but
/// outside the evaluated table from the curve data.
///
/// **Stepping frames inside the read evaluates a driver identically to the
/// render** (observed, 2026-08-19 test, V2), which is why the answer is the
/// stepped table first and the curve second.
pub fn renders_at(layer: &ViewLayerReading, frame: i64) -> FrameAnswer {
    if let Some((_, renders)) = layer.renders_at.iter().find(|(at, _)| *at == frame) {
        return FrameAnswer::Evaluated(*renders);
    }
    match &layer.animation {
        // An un-animated switch holds one value, and that value is the
        // constant the read recorded on both sides of its span.
        SwitchAnimation::None { value } => FrameAnswer::Extrapolated(*value),
        SwitchAnimation::Keyframed {
            keys,
            before_the_first_key,
            after_the_last_key,
        } => {
            let first = keys.iter().map(|key| key.frame).min();
            let last = keys.iter().map(|key| key.frame).max();
            match (first, last) {
                (Some(first), _) if frame < first => {
                    FrameAnswer::Extrapolated(*before_the_first_key)
                }
                (_, Some(last)) if frame > last => FrameAnswer::Extrapolated(*after_the_last_key),
                (Some(_), Some(_)) => {
                    // Inside the key span: the key at or before the frame
                    // decides, which is exact under constant interpolation.
                    let holding = keys
                        .iter()
                        .filter(|key| key.frame <= frame)
                        .max_by_key(|key| key.frame)
                        .expect("a key at or before the frame exists inside the span");
                    let exact = keys
                        .iter()
                        .all(|key| key.interpolation == Interpolation::Constant);
                    FrameAnswer::FromTheCurve {
                        value: holding.value,
                        exact,
                    }
                }
                // A keyframed switch with no keys at all has no span to
                // reason from; nothing is claimed.
                _ => FrameAnswer::NotKnownWithoutARead,
            }
        }
        SwitchAnimation::Driven { .. } | SwitchAnimation::DrivenByAStoredScript => {
            FrameAnswer::NotKnownWithoutARead
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // `Render Settings` Q2b; `Rendering` L10b. No record migration is needed.
    #[test]
    fn a_saved_shutter_round_trips_and_old_readings_have_none() {
        let mut scene = crate::file_half::tests::a_scene("Scene");
        let mut old = serde_json::to_value(&scene).unwrap();
        old.as_object_mut().unwrap().remove("motion_blur");
        assert_eq!(serde_json::from_value::<SceneReading>(old).unwrap().motion_blur, None);
        scene.motion_blur = Some(MotionBlur { enabled: true, position: ShutterPosition::Center, shutter: 2.25 });
        let read: SceneReading = serde_json::from_value(serde_json::to_value(&scene).unwrap()).unwrap();
        assert_eq!(read, scene);
        assert_eq!(read.motion_blur.unwrap().last_played_frame(125),127);
    }

    fn a_layer(animation: SwitchAnimation, table: &[(i64, bool)]) -> ViewLayerReading {
        ViewLayerReading {
            name: ExactName::as_the_reading_produced_it("A"),
            renders_at: table.to_vec(),
            animation,
            own_sample_count: 0,
            overrides_the_scenes_count: false,
        }
    }

    #[test]
    fn the_pixel_figure_truncates_on_each_axis_and_never_rounds() {
        // The four measured pairs of *Settings 15*, which are what the rule is
        // read off.
        assert_eq!(the_size_at((1920, 1080), 200), Some((3840, 2160)));
        assert_eq!(the_size_at((1920, 1080), 33), Some((633, 356)));
        assert_eq!(the_size_at((1920, 1080), 1), Some((19, 10)));
        assert_eq!(the_size_at((4, 4), 32767), Some((1310, 1310)));
        // A figure of zero is producible and is left as the figure it is.
        assert_eq!(the_size_at((4, 4), 1), Some((0, 0)));
    }

    #[test]
    fn no_value_outside_the_percentage_range_ever_reaches_the_arithmetic() {
        assert_eq!(the_size_at((1920, 1080), 0), None);
        assert_eq!(the_size_at((1920, 1080), 32768), None);
    }

    #[test]
    fn the_percentage_in_force_is_the_users_where_answered_else_the_files() {
        let scene = crate::file_half::tests::a_scene("Kitchen");
        assert_eq!(the_percentage_in_force(Some(50), &scene), 50);
        assert_eq!(
            the_percentage_in_force(None, &scene),
            scene.resolution_percentage
        );
    }

    #[test]
    fn a_frame_inside_the_table_is_evaluated_and_never_anything_else() {
        let layer = a_layer(
            SwitchAnimation::Driven {
                expression: "frame % 2".to_string(),
                valid: true,
            },
            &[(1, true), (2, false)],
        );
        assert_eq!(renders_at(&layer, 1), FrameAnswer::Evaluated(true));
        assert_eq!(renders_at(&layer, 2), FrameAnswer::Evaluated(false));
    }

    #[test]
    fn a_driven_switch_outside_the_table_is_not_known_without_a_read() {
        let driven = a_layer(
            SwitchAnimation::Driven {
                expression: "frame % 2".to_string(),
                valid: true,
            },
            &[(1, true)],
        );
        assert_eq!(renders_at(&driven, 50), FrameAnswer::NotKnownWithoutARead);
        let scripted = a_layer(SwitchAnimation::DrivenByAStoredScript, &[(1, true)]);
        assert_eq!(renders_at(&scripted, 50), FrameAnswer::NotKnownWithoutARead);
    }

    #[test]
    fn a_keyframed_switch_extrapolates_by_its_two_constants_and_computes_inside_the_key_span() {
        let keys = vec![
            Key {
                frame: 10,
                value: true,
                interpolation: Interpolation::Constant,
            },
            Key {
                frame: 20,
                value: false,
                interpolation: Interpolation::Constant,
            },
        ];
        let layer = a_layer(
            SwitchAnimation::Keyframed {
                keys: keys.clone(),
                before_the_first_key: false,
                after_the_last_key: false,
            },
            &[(10, true), (11, true)],
        );
        assert_eq!(renders_at(&layer, 5), FrameAnswer::Extrapolated(false));
        assert_eq!(renders_at(&layer, 25), FrameAnswer::Extrapolated(false));
        assert_eq!(
            renders_at(&layer, 15),
            FrameAnswer::FromTheCurve {
                value: true,
                exact: true
            }
        );
        assert_eq!(
            renders_at(&layer, 20),
            FrameAnswer::FromTheCurve {
                value: false,
                exact: true
            }
        );

        // One key of another interpolation kind and the curve answer is
        // labelled computed rather than exact.
        let mut other = keys;
        other[0].interpolation = Interpolation::Other;
        let layer = a_layer(
            SwitchAnimation::Keyframed {
                keys: other,
                before_the_first_key: false,
                after_the_last_key: false,
            },
            &[],
        );
        assert_eq!(
            renders_at(&layer, 15),
            FrameAnswer::FromTheCurve {
                value: true,
                exact: false
            }
        );
    }

    #[test]
    fn an_unanimated_switch_holds_its_one_value_everywhere() {
        let layer = a_layer(SwitchAnimation::None { value: true }, &[(1, true)]);
        assert_eq!(renders_at(&layer, 999), FrameAnswer::Extrapolated(true));
    }
}

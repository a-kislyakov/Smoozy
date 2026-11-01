//! View layers — `Render Settings` §4: what one read answers per frame
//! (V1–V3, V11–V12), the pin and the mechanism that holds it (V4–V8), and the
//! route to the curve per version (V9–V10).
//!
//! **Constraint:** Rows 147, 148, 149. The switch has no argument in any
//! version and is set by a script or expression at launch inside the one
//! window; **an animation render honours the keyframed value frame by frame**
//! (*Reading a file 3*, all four versions, both machines). The switch that
//! would render one view layer only never reaches a render started from
//! outside, so unchecking is delivered by the layer's own switch and never by
//! that one.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Rendering` L9, L11, L8a. `Job files and outputs`
//! D18, R4, I6.
//!
//! **`Job files and outputs` D5 and `Queue & History` B1 are not listed above
//! and are written out at each site**, because this plan defines a D5 and a
//! B1 of its own and a bare one could mean either.

use serde::{Deserialize, Serialize};
use smoozy_server_setup_machine::blender::Version;

use crate::frames::FrameForm;
use crate::names::{AnchorState, ExactName};
use crate::reading::{renders_at, FrameAnswer, SceneReading, SwitchAnimation, ViewLayerReading};

/// **V1: one read answers which view layers render at any frame later asked
/// about**, and the answer is Q3's stepped table with the curve data and
/// extrapolation constants beside it ([`renders_at`]). **V2: the driver case
/// is answered on the shape posed** — stepping frames inside the read
/// evaluates the driver exactly as the render evaluates it (observed,
/// 2026-08-19 test, both versions). **V3: the scripted case is answered on
/// both sides of the one preference that gates it** — under the starting
/// state neither the read nor the render runs the file's script, with the
/// preference enabled both do, and the two agree either way; the launch
/// contract overrides the preference in neither direction (`Rendering` L9).
/// The residue for both is §13 item 7.
pub const ONE_READ_ANSWERS_FOR_ANY_FRAME_LATER_ASKED_ABOUT: bool = true;

/// **V11: the frame an untouched box states is the numerical minimum of the
/// frame set the page has in hand** — the store entry F5's composition would
/// read at this moment, and where no entry stands, the frame start of the
/// reading in force for the scene in hand. Never the first entry written,
/// never the frame in view. **A first frame may be zero or negative**, so the
/// minimum is arithmetic over signed integers.
pub fn the_frame_the_untouched_box_states(
    entry: Option<&FrameForm>,
    reading: &SceneReading,
) -> i64 {
    entry
        .and_then(FrameForm::lowest)
        .unwrap_or(reading.frame_start)
}

/// **V12: the box's state is Q3's answer for that frame**, composed on the
/// desktop from the reading in force, asking the machine nothing. A range edit
/// is not one of J10's three drops, so redrawing at a new frame runs against
/// the standing reading and starts no read. **A box never states another
/// frame's answer as this frame's.**
pub fn the_box(layer: &ViewLayerReading, frame: i64) -> FrameAnswer {
    renders_at(layer, frame)
}

/// **Whether a layer renders at any frame of the range in force** — the rule
/// behind the page's *renders at its own sample count* line, which names only
/// the layers still in the job.
///
/// **The range in force is the user's where one stands, otherwise the scene's
/// own**, stepped the way the scene steps it, and each frame is asked of
/// [`the_box`] — V12's own answer, so the line and the boxes cannot disagree.
/// **A frame whose answer is *not known without a read* counts as rendering**:
/// V12 composes that answer rather than guessing, and a layer that may render
/// is still in the job. **A pin decides outright** — V4's pin severs the file's
/// animation, so a pinned-on layer renders at every frame and a pinned-off one
/// at none.
///
/// Written 2026-09-17 out of the step 10a round 1 audit of the screens (must-do
/// 8): the page had counted a layer unless a pin turned it off, ignoring the
/// file's own switch and the range, so a layer that never renders in the range
/// was still named — one of the spec's three endings. The judgement is this
/// module's, and the page calls it rather than keeping a second copy.
pub fn renders_somewhere_in_the_range(
    layer: &ViewLayerReading,
    the_users_range: Option<&FrameForm>,
    reading: &SceneReading,
    pinned: Option<bool>,
) -> bool {
    if let Some(on) = pinned {
        return on;
    }
    let renders = |frame: i64| match the_box(layer, frame) {
        FrameAnswer::Evaluated(on) | FrameAnswer::Extrapolated(on) => on,
        FrameAnswer::FromTheCurve { value, .. } => value,
        FrameAnswer::NotKnownWithoutARead => true,
    };
    match the_users_range {
        Some(range) => range.explicit_list().into_iter().any(renders),
        None => {
            let step = reading.frame_step.max(1) as usize;
            (reading.frame_start..=reading.frame_end)
                .step_by(step)
                .any(renders)
        }
    }
}

/// **V12's diamond**, computed from Q2 item 2's own material and from nothing
/// else.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Diamond {
    /// A switch with no animation carries no diamond.
    None,
    /// Keyframed, and the stated frame is exactly one of the curve's key
    /// frames — an equality, never a nearest-key and never a tolerance.
    Solid,
    /// Keyframed at every other frame; and **driven, or driven by a stored
    /// script — animated with no key frames at all, so hollow at every frame
    /// it is drawn at.** Which mark is drawn for a driven switch is Design's.
    Hollow,
}

pub fn the_diamond(layer: &ViewLayerReading, frame: i64) -> Diamond {
    match &layer.animation {
        SwitchAnimation::None { .. } => Diamond::None,
        SwitchAnimation::Keyframed { keys, .. } => {
            if keys.iter().any(|key| key.frame == frame) {
                Diamond::Solid
            } else {
                Diamond::Hollow
            }
        }
        SwitchAnimation::Driven { .. } | SwitchAnimation::DrivenByAStoredScript => Diamond::Hollow,
    }
}

/// **V4: a pin rides the payload as a per-layer entry** (Y1's view-layer pins
/// row), **and is applied by the launch material as a pin pair: resolve the
/// layer by J7's bytes, sever the switch's animation, then assign the pinned
/// value.** All three parts are load-bearing and the middle one is the
/// finding.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PinEntry {
    pub layer: ExactName,
    pub anchor: AnchorState,
    pub on: bool,
}

/// The three parts of a pin pair, in order. **The switch the pair assigns is
/// C9's `use`** ([`crate::spellings::VIEW_LAYER_USE`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PinPairStep {
    /// Resolve the layer by J7's bytes against the running file's own
    /// view-layer list; a name matching nothing raises (V7).
    ResolveTheLayerByItsBytes,
    /// **V5: a plain assignment does not keep the promise for an animated
    /// switch** — observed: the file's animation wins over a value set once at
    /// launch, totally and silently. **V6: severing then assigning holds, on
    /// and off, in both versions.** The severing happens in the running
    /// Blender only; nothing saves the file (`Job files and outputs` D5's
    /// ground).
    SeverTheSwitchsAnimation,
    AssignThePinnedValue,
}

pub const THE_PIN_PAIR: [PinPairStep; 3] = [
    PinPairStep::ResolveTheLayerByItsBytes,
    PinPairStep::SeverTheSwitchsAnimation,
    PinPairStep::AssignThePinnedValue,
];

/// **V7: a pin pair that cannot be applied fails the attempt before the
/// render instruction, and the failure must be made loud deliberately**: a
/// launch script that raises prints its traceback and the render runs anyway
/// at exit 0 — unless the line carries Blender's failure-to-exit-status
/// switch, with which the run ends at the named status with nothing written.
/// The material contains no failure handling of its own (`Rendering` L11,
/// citing `Job files and outputs` D18); the switch is `Rendering` L8a's, closed
/// 2026-08-22. **An unapplied pin must end the attempt, never survive as a
/// render of layers the user did not ask for.**
pub const AN_UNAPPLIED_PIN_ENDS_THE_ATTEMPT: bool = true;

/// **V8: one pin shape stays unresolved** — a pinned layer whose switch a
/// file's own script redrives, on a machine whose preferences allow stored
/// scripts to run. The severing removes curves and drivers; it does not
/// remove a handler. Under the preference's starting state the case does not
/// arise. **§13 item 9 carries what breaks, and the one repair available —
/// Blender's own switch for disabling automatic script execution — was
/// weighed and deliberately declined at `Rendering` L9**; the lever exists and
/// pulling it is a decision this document does not own.
pub const A_SCRIPT_REDRIVEN_PIN_IS_NOT_HELD_AND_NO_REPAIR_IS_TAKEN: bool = true;

/// **V9: the route to the curve, per version.** In 4.5 the action's curves
/// hang directly off the action; in 5.0, 5.1 and 5.2 that route does not
/// exist — asking for it raises — and the curve is reached by walking the
/// action's layers, then its strips, then its channel bags, then its curves
/// (*Reading a file 3*, all four versions on both machines; re-confirmed in
/// 4.5.12 and 5.2.0 by the 2026-08-19 test). **Both the reading and the pin's
/// severing branch on this and on nothing else.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RouteToTheCurve {
    DirectlyOffTheAction,
    ThroughLayersStripsChannelBagsAndCurves,
}

pub fn the_route_to_the_curve(version: Version) -> RouteToTheCurve {
    if version.major >= 5 {
        RouteToTheCurve::ThroughLayersStripsChannelBagsAndCurves
    } else {
        RouteToTheCurve::DirectlyOffTheAction
    }
}

/// **V10: the switch's declared starting value — off in 4.5, on from 5.0 —
/// never reaches this page.** Every box is drawn from a reading of a real
/// file; recorded so nothing cold-reads the default. Kept by there being no
/// function in this crate that takes a version and answers a switch's value.
pub const THE_DECLARED_STARTING_VALUE_NEVER_REACHES_THE_PAGE: () = ();

/// **`Job files and outputs` R4 item 5 and I6, and `Queue & History` B1's plan
/// total**, are computed from the same read as the boxes — the honest *plan
/// rather than promise* shape standing for shapes not posed.
pub const THE_PLAN_TOTAL_IS_A_PLAN_AND_NEVER_A_PROMISE: bool = true;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::file_half::tests::{a_layer, a_scene, a_version};
    use crate::frames::parse_the_frame_field;
    use crate::reading::{Interpolation, Key};

    #[test]
    fn the_untouched_box_states_the_lowest_frame_of_the_set_in_hand_or_the_readings_start() {
        let scene = a_scene("Kitchen");
        // The spec's own example: `40, 22-25` reads from 22.
        let list = parse_the_frame_field("40, 22-25").unwrap();
        assert_eq!(the_frame_the_untouched_box_states(Some(&list), &scene), 22);
        let negative = parse_the_frame_field("-3-3").unwrap();
        assert_eq!(
            the_frame_the_untouched_box_states(Some(&negative), &scene),
            -3
        );
        assert_eq!(
            the_frame_the_untouched_box_states(None, &scene),
            scene.frame_start
        );
    }

    /// **The samples line's rule: a layer is in the job where it renders at
    /// some frame of the range in force.** A layer the file keeps off in the
    /// range is not; a layer keyed on only after the user's range ends is not;
    /// a pin decides outright; and *not known* counts as rendering.
    #[test]
    fn a_layer_is_in_the_job_only_where_it_renders_somewhere_in_the_range() {
        let scene = a_scene("Kitchen");
        let off = a_layer("Off", false);
        assert!(!renders_somewhere_in_the_range(&off, None, &scene, None));
        assert!(renders_somewhere_in_the_range(&off, None, &scene, Some(true)));
        let on = a_layer("On", true);
        assert!(renders_somewhere_in_the_range(&on, None, &scene, None));
        assert!(!renders_somewhere_in_the_range(&on, None, &scene, Some(false)));

        let mut later = a_layer("Later", false);
        later.animation = SwitchAnimation::Keyframed {
            keys: vec![
                Key { frame: 1, value: false, interpolation: Interpolation::Constant },
                Key { frame: 8, value: true, interpolation: Interpolation::Constant },
            ],
            before_the_first_key: false,
            after_the_last_key: true,
        };
        later.renders_at = (1..=10).map(|frame| (frame, frame >= 8)).collect();
        let early = parse_the_frame_field("1-5").unwrap();
        assert!(!renders_somewhere_in_the_range(&later, Some(&early), &scene, None));
        assert!(renders_somewhere_in_the_range(&later, None, &scene, None));

        let mut driven = a_layer("Driven", false);
        driven.renders_at = Vec::new();
        driven.animation = SwitchAnimation::Driven {
            expression: "frame % 2".to_string(),
            valid: true,
        };
        let outside = parse_the_frame_field("50-60").unwrap();
        assert!(
            renders_somewhere_in_the_range(&driven, Some(&outside), &scene, None),
            "*not known without a read* counts as rendering (V12)"
        );
    }

    #[test]
    fn the_diamond_is_solid_exactly_on_a_key_frame_and_hollow_everywhere_else() {
        let mut layer = a_layer("A", true);
        assert_eq!(the_diamond(&layer, 1), Diamond::None);
        layer.animation = SwitchAnimation::Keyframed {
            keys: vec![Key {
                frame: 10,
                value: true,
                interpolation: Interpolation::Constant,
            }],
            before_the_first_key: false,
            after_the_last_key: true,
        };
        assert_eq!(the_diamond(&layer, 10), Diamond::Solid);
        assert_eq!(the_diamond(&layer, 11), Diamond::Hollow);
        assert_eq!(the_diamond(&layer, 9), Diamond::Hollow);
        layer.animation = SwitchAnimation::Driven {
            expression: "frame % 2".to_string(),
            valid: true,
        };
        assert_eq!(the_diamond(&layer, 10), Diamond::Hollow);
    }

    #[test]
    fn the_box_is_q3s_answer_and_a_driven_switch_outside_the_span_is_not_known() {
        let mut layer = a_layer("A", true);
        assert_eq!(the_box(&layer, 5), FrameAnswer::Evaluated(true));
        layer.animation = SwitchAnimation::Driven {
            expression: "frame % 2".to_string(),
            valid: true,
        };
        assert_eq!(the_box(&layer, 50), FrameAnswer::NotKnownWithoutARead);
    }

    #[test]
    fn the_route_to_the_curve_branches_at_five_and_on_nothing_else() {
        assert_eq!(
            the_route_to_the_curve(a_version(4, 5, 12)),
            RouteToTheCurve::DirectlyOffTheAction
        );
        for version in [a_version(5, 0, 1), a_version(5, 1, 2), a_version(5, 2, 1)] {
            assert_eq!(
                the_route_to_the_curve(version),
                RouteToTheCurve::ThroughLayersStripsChannelBagsAndCurves
            );
        }
    }

    #[test]
    fn the_pin_pair_resolves_then_severs_then_assigns() {
        assert_eq!(
            THE_PIN_PAIR,
            [
                PinPairStep::ResolveTheLayerByItsBytes,
                PinPairStep::SeverTheSwitchsAnimation,
                PinPairStep::AssignThePinnedValue,
            ]
        );
    }
}

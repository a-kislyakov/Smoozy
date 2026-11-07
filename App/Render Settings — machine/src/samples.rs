//! Samples: the imposition set — `Render Settings` §5 C1–C9, with C7's
//! percentage pair, which has no section of its own.
//!
//! **Constraint:** Row 150 — *"Typed into, none of it does: the job renders
//! the number the user typed, and anything that would have cut it short is
//! set aside"* — with the spec's own statement that the three it names **are
//! not everything**, so the answer must survive another displacer being
//! found. A sample count handed in is a ceiling, not a quantity: five things
//! in Cycles stand between it and the samples a pixel gets (*Settings 16*).
//! Nothing announces any of it while it happens (*Reports 11*).
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Rendering` L10, L10a, L15, L21.

use crate::engine::OfferedEngine;
use crate::reading::{LayerSamplesRule, SceneReading, ViewLayerReading};
use crate::spellings::{
    Spelling, ADAPTIVE_SAMPLING_OFF, CYCLES_SAMPLES, EEVEE_HAS_A_TIME_LIMIT_FROM, EEVEE_SAMPLES,
    EEVEE_TIME_LIMIT_OFF, LAYER_SAMPLES_RULE, RESOLUTION_PERCENTAGE, SAMPLE_SUBSET_OFF,
    THE_TIME_LIMITS_OFF_VALUE, TIME_LIMIT_OFF,
};

pub use crate::spellings::MissingSpelling;

/// **A settings pair as `Rendering` L10 delivers it**: a property written out
/// from the job's scene, and the value assigned to it.
#[derive(Debug, Clone, PartialEq)]
pub struct SettingsPair {
    /// Written out from the job's scene — **C8: every imposition is assigned
    /// against the job's scene, and none ever names another scene of the
    /// file.**
    pub property: &'static str,
    pub value: PairValue,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PairValue {
    Number(u64),
    /// An identifier, matched exactly, letter case included.
    Identifier(&'static str),
    Switch(bool),
}

/// **C1: a typed count rides the payload as one entry, and its application is
/// the count pair plus the samples-imposition set — a named list of
/// displacer-disabling pairs that lives in this clause and nowhere else.**
///
/// 1. the chosen engine's own sample count, set to the typed number;
/// 2. adaptive sampling, switched off — the early stop is on in every
///    version's starting state;
/// 3. the time limit, set to its off value — and from 5.3, under EEVEE,
///    EEVEE's own time limit too;
/// 4. the layer-samples rule, set to `IGNORE` — the one value that throws
///    every per-view-layer count away in a single act;
/// 5. the sample subset, switched off — the fourth known displacer.
///
/// **C3: under EEVEE the count pair targets EEVEE's own count setting**, and
/// the reasoning for setting nothing aside reaches exactly the two displacers
/// it names — the early stop is Cycles' alone, and the time limit is Cycles'
/// alone up to 5.2 — while **it does not reach item 4**, a view layer's own
/// sample count not being Cycles alone. Items 2, 3 and 5 ride with every typed
/// count (§13 item 29's own words), whatever engine; under EEVEE two of them
/// have nothing to bite on. **From 5.3 a limit EEVEE's file was saved with would
/// cut a typed count short and say nothing (*Settings 2*), so that pair is
/// composed per version**: `the_version` is the manifest's, as major and minor,
/// and an older version never meets the name. **C2: the set is the extension point** — a newly found displacer is one
/// more pair added here, with nothing else moving; what stands against a
/// displacer not on this list is only this list's completeness (§13 item 10).
///
/// **§13 item 29's lookup landed on 2026-09-20 and this composes.** Items 2
/// to 5 assign against the names C10 carries, read off the four running
/// Blenders that day and identical in all four. Until then this failed here,
/// naming the first missing spelling, and every attempt of every
/// typed-sample-count job failed loudly and totally — never a silent render
/// at another count, which is the direction the entry ruled safe.
///
/// **The failure route is not removed and must not be**: this still returns
/// the absence where a spelling is one, and `Rendering` L10a's name check
/// still stands behind every pair composed here, because no name in C10 has
/// been set on Linux, where Smoozy renders (§13 item 28).
pub fn the_imposition_set(
    engine: OfferedEngine,
    typed_count: u64,
    the_version: (u32, u32),
) -> Result<Vec<SettingsPair>, MissingSpelling> {
    let count_setting: Spelling = match engine {
        OfferedEngine::Cycles => CYCLES_SAMPLES,
        OfferedEngine::Eevee => EEVEE_SAMPLES,
    };
    let eevees_own_limit = (engine == OfferedEngine::Eevee
        && the_version >= EEVEE_HAS_A_TIME_LIMIT_FROM)
        .then(|| {
            Ok(SettingsPair {
                property: EEVEE_TIME_LIMIT_OFF.written_out()?,
                value: PairValue::Number(THE_TIME_LIMITS_OFF_VALUE),
            })
        })
        .transpose()?;
    let mut set = vec![
        SettingsPair {
            property: count_setting.written_out()?,
            value: PairValue::Number(typed_count),
        },
        SettingsPair {
            property: ADAPTIVE_SAMPLING_OFF.written_out()?,
            value: PairValue::Switch(false),
        },
        SettingsPair {
            // **The off value is zero, and that is Blender's own statement of
            // it** — the property's description reads *"Zero disables the
            // limit"* in all four versions (C10, the lookup of 2026-09-20).
            // It was the absence's own placeholder until that date.
            property: TIME_LIMIT_OFF.written_out()?,
            value: PairValue::Number(THE_TIME_LIMITS_OFF_VALUE),
        },
    ];
    set.extend(eevees_own_limit);
    set.extend([
        SettingsPair {
            property: LAYER_SAMPLES_RULE.written_out()?,
            value: PairValue::Identifier(LayerSamplesRule::Ignore.identifier()),
        },
        SettingsPair {
            property: SAMPLE_SUBSET_OFF.written_out()?,
            value: PairValue::Switch(false),
        },
    ]);
    Ok(set)
}

/// **C4: an untouched count is no entry** (J4): the file renders its own
/// setup entire — early stop, time limit, subsets and per-layer counts all
/// standing. Kept by [`the_imposition_set`] being reachable only from a typed
/// count.
pub const AN_UNTOUCHED_COUNT_IS_NO_ENTRY: () = ();

/// **C5: nothing establishes the imposition set on a graphics card**, and
/// Smoozy renders on machines rented for their cards. *Settings 16* names the
/// device of every displacer run outright — *"Every Cycles render behind
/// question 16 was on the processor, not on the graphics card"* — so nothing
/// under it says anything about a render on a graphics card. Carried at §13
/// item 10.
pub const THE_SET_IS_ESTABLISHED_ON_THE_PROCESSOR_ALONE: bool = true;

/// **C6: `Queue & History` B1's per-renderlet count is recoverable, both
/// ways.** With a typed count, the payload's one entry is the count in force
/// for every layer — the imposition set has removed every per-layer
/// displacement. With none, the count in force per layer is the reading's:
/// the scene's own count and each layer's override under the file's own
/// layer-samples rule, all of which Q2 items 1 and 2 carry. *Carried in the
/// settings payload and the reading* — that document's B1's own words — is
/// exactly this split.
pub fn the_count_in_force_for(
    typed_count: Option<u64>,
    engine_in_force: OfferedEngine,
    scene: &SceneReading,
    layer: &ViewLayerReading,
) -> u64 {
    if let Some(typed) = typed_count {
        return typed;
    }
    let scenes_own = match engine_in_force {
        OfferedEngine::Cycles => scene.sample_counts.cycles,
        OfferedEngine::Eevee => scene.sample_counts.eevee,
    };
    if !layer.overrides_the_scenes_count {
        return scenes_own;
    }
    // The file's own layer-samples rule, as *Settings 16* establishes it under
    // Cycles: the layer's count stands in place of the scene's under `USE`, is
    // capped by it under `BOUNDED`, and is thrown away under `IGNORE`. What
    // the scene-level rule does to an EEVEE render is §13 item 10's named
    // question and is not answered here.
    match scene.layer_samples_rule {
        LayerSamplesRule::Use => layer.own_sample_count,
        LayerSamplesRule::Bounded => layer.own_sample_count.min(scenes_own),
        LayerSamplesRule::Ignore => scenes_own,
    }
}

/// **C7: the resolution percentage's imposition** — a typed percentage rides
/// the payload as one entry (Y1) and is imposed as a settings pair against
/// the scene's own percentage setting, per version, at composition. The
/// spelling is C7's own cited one, so this pair composes today.
pub fn the_percentage_pair(typed_percentage: u32) -> SettingsPair {
    SettingsPair {
        property: RESOLUTION_PERCENTAGE
            .written_out()
            .expect("C7's spelling is cited, at *Settings 17*"),
        value: PairValue::Number(typed_percentage as u64),
    }
}

/// **C8: which scene every imposition is assigned against — the job's scene,
/// and never another scene of the file.** The job's scene is the one the
/// manifest carries — the launch's own scene, its name anchored by Y13 — or
/// the file's showing scene where the job carries none (Y10, `Rendering`
/// L15's unnamed branch). This covers C1's five pairs, C7's percentage pair,
/// D3's device pair, the pin pairs and the Simplify pairs: each targets that
/// scene's own property, and no pair walks the file's scene list.
///
/// **The rule is load-bearing because the lever reaches further than the
/// pairs use it**: a script handed in at launch does reach a pulled-in scene
/// where it names one (*The compositor 9*, observed), so the drawn line —
/// *rendering this scene renders Exterior too, at its own engine and sample
/// count* — rests on this composition rule as much as on Blender. What the
/// pulled-in scene renders at is established for the engine, the count and
/// the override's governing policy, in all four versions, on the Creator's
/// Mac alone (§13 item 27); and the engine argument never reaches the scene
/// behind the node.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssignmentTarget {
    TheJobsScene,
}

pub const THE_ASSIGNMENT_TARGET: AssignmentTarget = AssignmentTarget::TheJobsScene;

/// **The five pairs, C7's pair, D3's pair, the pin pairs and the Simplify
/// pairs are all written out from the job's scene** — every spelling in
/// [`crate::spellings`] begins `scene.`, and the test below holds it.
pub const NO_PAIR_WALKS_THE_FILES_SCENE_LIST: bool = true;

/// **C9: the names the count pairs and the pin pair are composed from** are
/// [`crate::spellings`]'; and the one Simplify pair that is Cycles' sits on
/// the scene's Cycles block by its own path (`Rendering` A1d), present
/// whatever the active engine. **`render_aa` takes text and not a number, and
/// this page never composes it.**
pub const WORKBENCH_IS_NEVER_COMPOSED: bool = true;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::file_half::tests::{a_layer, a_scene};

    /// **The lookup landed on 2026-09-20, so a typed count composes** — under
    /// both engines, the count pair first and then C1's four displacer pairs
    /// in the clause's own order. This test held the refusal until that date.
    #[test]
    fn a_typed_count_composes_the_whole_set_now_that_the_spellings_are_read() {
        for (engine, count_pair) in [
            (OfferedEngine::Cycles, "scene.cycles.samples"),
            (OfferedEngine::Eevee, "scene.eevee.taa_render_samples"),
        ] {
            let set = the_imposition_set(engine, 128, (5, 2)).expect("C10 carries every name");
            let names: Vec<&str> = set.iter().map(|pair| pair.property).collect();
            assert_eq!(
                names,
                vec![
                    count_pair,
                    "scene.cycles.use_adaptive_sampling",
                    "scene.cycles.time_limit",
                    "scene.cycles.use_layer_samples",
                    "scene.cycles.use_sample_subset",
                ]
            );
            assert_eq!(set[0].value, PairValue::Number(128));
            assert_eq!(set[1].value, PairValue::Switch(false));
            // **The off value is zero, in Blender's own words** (C10).
            assert_eq!(set[2].value, PairValue::Number(0));
            assert_eq!(set[3].value, PairValue::Identifier("IGNORE"));
            assert_eq!(set[4].value, PairValue::Switch(false));
        }
    }

    /// **C1 item 3 and C3, corrected 2026-11-02: from 5.3 EEVEE's own time limit
    /// goes off too, under EEVEE alone**, right after Cycles', and no older
    /// version ever meets the name, because `Rendering` L10a would fail the job
    /// on it (*Settings 2*).
    #[test]
    fn a_typed_count_under_eevee_turns_eevees_own_limit_off_from_5_3_and_never_before() {
        let names = |engine, version| -> Vec<&'static str> {
            the_imposition_set(engine, 128, version)
                .expect("C10 carries every name")
                .iter()
                .map(|pair| pair.property)
                .collect()
        };
        let with_it = vec![
            "scene.eevee.taa_render_samples",
            "scene.cycles.use_adaptive_sampling",
            "scene.cycles.time_limit",
            "scene.eevee.time_limit",
            "scene.cycles.use_layer_samples",
            "scene.cycles.use_sample_subset",
        ];
        assert_eq!(names(OfferedEngine::Eevee, (5, 3)), with_it);
        assert_eq!(names(OfferedEngine::Eevee, (5, 4)), with_it);
        let set = the_imposition_set(OfferedEngine::Eevee, 128, (5, 3)).unwrap();
        assert_eq!(set[3].value, PairValue::Number(0), "Zero disables the limit.");

        for version in [(4, 5), (5, 0), (5, 1), (5, 2)] {
            assert!(!names(OfferedEngine::Eevee, version).contains(&"scene.eevee.time_limit"));
        }
        for version in [(4, 5), (5, 2), (5, 3), (5, 4)] {
            assert!(!names(OfferedEngine::Cycles, version).contains(&"scene.eevee.time_limit"));
        }
    }

    /// **The loud-failure path is not deleted.** `Rendering` L10a's name check
    /// stands whatever the names turned out to be, and a spelling later found
    /// to differ where Smoozy renders comes back as an absence rather than as
    /// a guess — so the route that refuses to compose one is still here and
    /// still names what is missing.
    #[test]
    fn a_spelling_that_is_an_absence_still_stops_the_set_and_says_which() {
        let absent = Spelling::NotYetRead(MissingSpelling {
            the_setting: "a setting nobody has read",
            the_clause: "C1 item 2",
        });
        let refused = absent.written_out().expect_err("an absence composes nothing");
        assert_eq!(refused.the_clause, "C1 item 2");
        assert!(!refused.the_setting.is_empty());
    }

    #[test]
    fn the_percentage_pair_composes_today_against_the_scenes_own_setting() {
        assert_eq!(
            the_percentage_pair(25),
            SettingsPair {
                property: "scene.render.resolution_percentage",
                value: PairValue::Number(25),
            }
        );
    }

    #[test]
    fn every_pair_is_written_out_from_the_jobs_scene_and_none_walks_the_scene_list() {
        let every: [Spelling; 6] = [
            RESOLUTION_PERCENTAGE,
            CYCLES_SAMPLES,
            EEVEE_SAMPLES,
            crate::spellings::VIEW_LAYER_SAMPLES,
            crate::spellings::VIEW_LAYER_USE,
            crate::spellings::WORKBENCH_RENDER_AA_NEVER_COMPOSED,
        ];
        for spelling in every {
            assert!(spelling.written_out().unwrap().starts_with("scene."));
            assert!(!spelling.written_out().unwrap().contains("scenes["));
        }
        assert_eq!(THE_ASSIGNMENT_TARGET, AssignmentTarget::TheJobsScene);
    }

    #[test]
    fn the_per_renderlet_count_is_the_typed_one_everywhere_or_the_files_own_under_its_own_rule() {
        let mut scene = a_scene("Kitchen");
        let mut layer = a_layer("Detail", true);
        layer.own_sample_count = 8;
        layer.overrides_the_scenes_count = true;

        // Typed: the one number, for every layer, whatever the file's rule.
        assert_eq!(
            the_count_in_force_for(Some(500), OfferedEngine::Cycles, &scene, &layer),
            500
        );

        // Untyped: the file's own, under its own rule.
        scene.layer_samples_rule = LayerSamplesRule::Use;
        assert_eq!(
            the_count_in_force_for(None, OfferedEngine::Cycles, &scene, &layer),
            8
        );
        scene.layer_samples_rule = LayerSamplesRule::Bounded;
        layer.own_sample_count = 1000;
        assert_eq!(
            the_count_in_force_for(None, OfferedEngine::Cycles, &scene, &layer),
            64
        );
        scene.layer_samples_rule = LayerSamplesRule::Ignore;
        assert_eq!(
            the_count_in_force_for(None, OfferedEngine::Cycles, &scene, &layer),
            64
        );

        // A layer with no override is the scene's own count for the engine in
        // force.
        let plain = a_layer("ViewLayer", true);
        scene.sample_counts.eevee = 32;
        assert_eq!(
            the_count_in_force_for(None, OfferedEngine::Eevee, &scene, &plain),
            32
        );
    }
}

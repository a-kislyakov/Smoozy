//! The submission payload — `Render Settings` §9.1 Y1–Y3, §9.2 Y4–Y6, §9.3
//! Y7–Y11, and Y1a's companion.
//!
//! **Constraint:** Rows 152 and 153. Every setting this page offers reaches
//! the render from outside the file, in one silent window, by two delivery
//! kinds; the user only ever states the differences.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Rendering` L10, L15, L18, L19, L21, A1d, S5,
//! S6. `Job files and outputs` M1, M1a, R3, R12, R13. `Queue & History` N11,
//! N14. `Server Setup` B13.

use serde::{Deserialize, Serialize};
use smoozy_job_files_machine::manifest::{self, CarriedAndNotInterpretedHere, TheBlenderToRenderBy};

use crate::device::DeviceChoice;
use crate::engine::{the_engine_entry, EngineEntry, OfferedEngine};
use crate::file_half::{version_text, BlenderIdentity, RecordedReading};
use crate::frames::FrameForm;
use crate::names::{
    anchor_at_submission, AnchorState, ExactName, NamedAnswer, Provenance, WhatIsAnchored,
};
use crate::reading::{DeviceSetting, SceneReading, SwitchAnimation};
use crate::store::{SceneEntries, SimplifyEntries};
use crate::view_layers::PinEntry;

/// **Y1's always-present row: the version pick — the picked build's path and
/// answered version** (B4, `Server Setup` B13).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VersionPick {
    pub build_path: String,
    pub version: String,
}

impl VersionPick {
    pub fn of(identity: &BlenderIdentity) -> VersionPick {
        VersionPick {
            build_path: identity.path.clone(),
            version: version_text(identity.version),
        }
    }
}

/// **Y1's scene row: J7's bytes and provenance, with Y12's anchor state.**
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SceneEntry {
    pub name: ExactName,
    pub anchor: AnchorState,
}

/// **Y1: the payload is a statement of overrides — one entry per setting the
/// user answered, no entry at all for a setting they left alone — plus the
/// version pick, which is always present, and the scene entry where the job
/// carries one.**
///
/// **Y2: immutable from submission and carried, not interpreted, everywhere
/// below the page.** There is no method that changes a field; it goes into
/// the manifest as M1's settings payload field, `Rendering` S5 composes the
/// manifest from it, and every attempt reads it as it was written.
///
/// **What the payload never carries**: the skip-existing-frames switch, which
/// has one home in `Rendering` L10; any output path, which is the redirect's;
/// any file value re-sent as an override — J4 makes that unrepresentable,
/// because a file value never has a store entry to be composed from.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Payload {
    /// `Render Settings` Y1c: old jobs play; off survives every attempt.
    #[serde(default = "simulations_on")]
    pub prepare_simulations: bool,
    pub version_pick: VersionPick,
    pub scene: Option<SceneEntry>,
    /// F1's parsed triple, or the tidied ascending integer list.
    pub frame_range: Option<FrameForm>,
    /// Inside the range the picked engine declares; the page holds the box to
    /// it (J11 item 3). Implies C1's imposition set at composition.
    pub sample_count: Option<u64>,
    /// One integer, 1 to 32767; the page holds the box to it.
    pub resolution_percentage: Option<u32>,
    /// The pick, plus the anchored identifier where one stands (E1–E2).
    pub engine: Option<EngineEntry>,
    pub device: Option<DeviceChoice>,
    pub view_layer_pins: Vec<PinEntry>,
    /// Per setting, its own entry — each an ordinary pair against the
    /// render-half property `Rendering` A1d names, carried not decided here.
    pub simplify: SimplifyEntries,
    /// **Y1b: *the check is written as a picture*** — set by the act that
    /// composes a first-frame check's two jobs on the first job alone
    /// (`Rendering` S13), and on no other. It asks the step-4 reading to write
    /// the check's frame as a still picture where the job's only output is a
    /// video; the reading decides whether that holds (`Job files and outputs`
    /// R4c, `Rendering` L10c). **Absent from every payload written before
    /// 2026-10-27, which reads as a job that is not a check**, and left out of
    /// the carried field when false, so every other payload is written exactly
    /// as before. Added 2026-10-27, the nineteenth notes, notes 3 and 9.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub the_check_is_written_as_a_picture: bool,
    /// **`Rendering` S13's *writes into*: the range job's identity, as text,
    /// on a first-frame check alone** — set by the act that composes the two
    /// jobs, once both identities are issued. The check then writes into the
    /// range job's output folder and every file it writes starts `check_`
    /// (`Job files and outputs` P18a). **Absent on every other payload, and on
    /// every payload written before 2026-10-31**, which reads as a job that is
    /// not a check; left out of the carried field when absent, so every other
    /// payload is written exactly as before. Added 2026-10-31, the twenty-first
    /// notes, note 6.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub the_check_writes_into: Option<String>,
}

fn simulations_on() -> bool { true }

/// What the composition is given, at submission.
pub struct Composition<'a> {
    /// B4: always present, always a concrete pair.
    pub version_pick: &'a BlenderIdentity,
    /// The scene the job carries, where it carries one (Y10).
    pub scene: Option<&'a NamedAnswer>,
    /// F5's answers for that scene — or the pre-read set where no scene is
    /// in hand.
    pub answers: &'a SceneEntries,
    /// The standing reading for the (server, file), where one stands.
    pub standing: Option<&'a RecordedReading>,
    /// Q13's staleness test against the four values now standing and the
    /// version in force (`Job files and outputs` R13), taken by the caller.
    pub stale: bool,
}

/// **Y1, performed: composed whole in one act at submission from the store
/// plus the resolved version pick** (§11's row). **Y3: every entry is
/// deliverable inside the one window** — after the file, before the render
/// instruction — because every entry is either a value with its own argument
/// or a value the launch material assigns, and nothing else exists in it.
/// **Y4: which entries ride as arguments and which as pairs is resolved at
/// composition on the machine, per the manifest's version, by the launch
/// contract** (`Rendering` L10) — never in the payload and never at run time.
pub fn compose(given: Composition<'_>) -> Payload {
    Payload {
        prepare_simulations: given.answers.prepare_simulations.unwrap_or(true),
        version_pick: VersionPick::of(given.version_pick),
        scene: given.scene.map(|scene| SceneEntry {
            name: scene.name.clone(),
            anchor: anchor_at_submission(
                scene,
                &WhatIsAnchored::AScene,
                given.standing,
                given.stale,
            ),
        }),
        frame_range: given.answers.frame_range.clone(),
        sample_count: given.answers.sample_count,
        resolution_percentage: given.answers.resolution_percentage,
        engine: the_engine_entry(
            given.answers.engine_pick,
            given.standing.filter(|_| !given.stale),
        ),
        device: given.answers.device,
        // **C8: a pin's layer is the job's scene's**, so the anchor is asked
        // about that scene's own list and never the file's. **A job carrying
        // no scene name carries no pins**, because J3's pre-read answer set
        // holds none — a pin needs a name only a reading or a kept setup
        // supplies, and neither can exist while no scene is in hand.
        view_layer_pins: given
            .scene
            .map(|scene| {
                given
                    .answers
                    .pins
                    .iter()
                    .map(|pin| PinEntry {
                        layer: pin.layer.name.clone(),
                        anchor: anchor_at_submission(
                            &pin.layer,
                            &WhatIsAnchored::AViewLayerOf {
                                the_jobs_scene: scene.name.clone(),
                            },
                            given.standing,
                            given.stale,
                        ),
                        on: pin.on,
                    })
                    .collect()
            })
            .unwrap_or_default(),
        simplify: given.answers.simplify.clone(),
        // **Y1b**: never from the page; the act that composes a first-frame
        // check's two jobs sets it on the first.
        the_check_is_written_as_a_picture: false,
        // **`Rendering` S13's *writes into***: never from the page either; the
        // same act sets it on the check once the range job's identity is issued.
        the_check_writes_into: None,
    }
}

impl Payload {
    /// **Y2: M1's settings payload field, carried and not interpreted.**
    pub fn as_carried(&self) -> CarriedAndNotInterpretedHere {
        CarriedAndNotInterpretedHere(
            serde_json::to_value(self).expect("a payload is composed of values that always render"),
        )
    }

    /// What the manifest's own settings-payload field reads back as. **The
    /// manifest is the vehicle; nothing below the page reinterprets it.**
    pub fn read_back(carried: &CarriedAndNotInterpretedHere) -> Option<Payload> {
        serde_json::from_value(carried.0.clone()).ok()
    }

    /// **Y9: the manifest's frame-range field takes two forms — the user's
    /// parsed integers, or the explicit statement *the file's own*** — and the
    /// reckoning composes the attempt's list from whichever stands, using the
    /// reading in force for the second (`Job files and outputs` M1a).
    pub fn the_manifests_frame_range(&self) -> manifest::FrameRange {
        match &self.frame_range {
            Some(form) => form.as_the_manifests_field(),
            None => manifest::FrameRange::TheFilesOwn,
        }
    }

    /// **M1a's scene field**, where the job carries a scene; `None` is
    /// `Rendering` L15's unnamed branch — the job renders whichever scene the
    /// file was saved showing (Y10, `Queue & History` N11).
    pub fn the_manifests_scene_field(&self) -> Option<manifest::SceneField> {
        self.scene.as_ref().map(|scene| manifest::SceneField {
            name: scene.name.as_text().to_string(),
            anchor: match &scene.anchor {
                AnchorState::Anchored {
                    the_reading_it_came_from,
                } => manifest::AnchorState::Anchored {
                    the_reading_it_came_from: the_reading_it_came_from.as_text(),
                },
                AnchorState::ToAnchor { provenance } => manifest::AnchorState::ToAnchor {
                    provenance: serde_json::to_string(provenance)
                        .expect("a provenance always renders"),
                },
            },
        })
    }

    /// **B4, M1a's build field: path beside version.**
    pub fn the_manifests_blender(&self) -> TheBlenderToRenderBy {
        TheBlenderToRenderBy {
            version: self.version_pick.version.clone(),
            build_path: self.version_pick.build_path.clone(),
        }
    }
}

/// **Y7: a job submitted with no read exists, and the payload makes it
/// composable: every absent entry means *the file's own*, resolved at the
/// attempt from the reading in force.** The render-start reading runs anyway
/// (`Job files and outputs` R3), and the file's range, count, engine, device
/// and everything untouched come from it, at the file as it is when the job
/// starts. **Y8: a stale earlier reading never leaks into the payload**,
/// because no reading value is ever in it (J4); what a change between
/// submission and render start can move is the reading in force, and
/// R12–R13 own that.
pub const AN_ABSENT_ENTRY_MEANS_THE_FILES_OWN: () = ();

/// **Y11: the all-scenes act is composable only where a reading stands** —
/// the scene list is the reading's — so every job in a group carries an
/// anchored name (`Rendering` S6). Each name's provenance is that reading's
/// identity (J7's first naming act).
pub fn the_all_scenes_act(standing: Option<&RecordedReading>) -> Option<Vec<NamedAnswer>> {
    let reading = standing?;
    Some(
        reading
            .settings
            .scenes
            .iter()
            .map(|scene| NamedAnswer {
                name: scene.name.clone(),
                provenance: Provenance::ARecordedReading(reading.identity()),
            })
            .collect(),
    )
}

/// **Y1a: one thing is composed at submission beside the payload and is not
/// part of it** — for each setting the payload carries an override for, the
/// file's value for that setting as the recorded reading in force at
/// submission gave it, **and for the frame range, the samples, the percentage
/// and the engine whether or not the payload overrides them** (the 2026-10-07
/// correction, [`the_companion`]). **Into the local job record on the desktop,
/// and nowhere else** (`Queue & History` N14): not in the payload, not in the
/// manifest, never sent to a machine. **Display material, refused as
/// anything else**: nothing composes an override from it, nothing validates
/// against it, and it is not J7's provenance.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Companion {
    pub frame_range: Option<TheFilesFrameRange>,
    /// The count of the engine in force under the file — the user's pick
    /// where one stands, else the file's own engine where it is one of the
    /// two offered.
    pub sample_count: Option<u64>,
    /// **The file's own percentage and never a pixel size** (Q2a's page-from-
    /// history paragraph).
    pub resolution_percentage: Option<u32>,
    pub engine: Option<ExactName>,
    pub device: Option<DeviceSetting>,
    pub view_layer_pins: Vec<(ExactName, TheFilesSwitch)>,
    /// **Per setting, mirroring the override store's own shape** — Simplify
    /// has been per-setting since 2026-08-29, Row 472, and Y1a's rule is *one
    /// value per override; never the whole reading*. Answering the
    /// subdivision cap alone used to put the file's switch, child-particle cap
    /// and texture cap in the record beside it, for three settings nobody
    /// touched, which `Queue & History` N14 then draws under the file's-value
    /// convention.
    pub simplify: TheFilesSimplify,
}

/// **Y1a for Simplify: the file's value for each setting the payload carries
/// an override for, and no others.** The four fields mirror
/// [`crate::store::SimplifyEntries`].
#[derive(Debug, Clone, PartialEq, Default)]
pub struct TheFilesSimplify {
    pub use_simplify: Option<bool>,
    pub subdivision_render: Option<u32>,
    pub child_particles_render: Option<f64>,
    pub texture_limit_render: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TheFilesFrameRange {
    pub start: i64,
    pub end: i64,
    pub step: i64,
}

/// The file's own value for a pinned switch: one value, or animated — the
/// per-frame answer being Q3's and not a value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TheFilesSwitch {
    Static(bool),
    Animated,
}

/// **Y1a, performed.** `None` where no reading stood: the entry draws the
/// file's side as absent rather than guessing, and **a later read never
/// fills it in** — the value is *what the file said when this job was
/// submitted*, and a reading taken afterwards is a different fact.
///
/// **The four settings the entry draws as the file's own — the frame range,
/// the samples, the percentage and the engine — are composed whether or not
/// the payload overrides them** (Y1a as corrected 2026-10-07, the Creator's
/// second notes, piece 16). Beside an override the value is the file's,
/// named beside the user's; on a setting left alone it is the value the job
/// ran with. Until then a setting left alone kept nothing, and the entry read
/// *not known* for the frames and the engine of a job submitted with the file
/// read. **The device, the pins and Simplify stay one value per override**:
/// `Queue & History` E12a answers the device, and the entry draws neither of
/// the others.
pub fn the_companion(
    payload: &Payload,
    standing: Option<&RecordedReading>,
    scene: Option<&ExactName>,
) -> Option<Companion> {
    let reading = standing?;
    // A job carrying no scene name renders the file's showing scene, and
    // which that is the reading does not say; nothing is composed.
    let name = scene?;
    let scene: &SceneReading = reading
        .settings
        .scenes
        .iter()
        .find(|held| held.name.as_bytes() == name.as_bytes())?;
    let engine_in_force = match payload.engine.as_ref().map(|entry| entry.pick) {
        Some(pick) => Some(pick),
        None => crate::engine::what_the_page_shows(scene, reading).into_offered(),
    };
    Some(Companion {
        frame_range: Some(TheFilesFrameRange {
            start: scene.frame_start,
            end: scene.frame_end,
            step: scene.frame_step,
        }),
        sample_count: engine_in_force.map(|engine| match engine {
            OfferedEngine::Cycles => scene.sample_counts.cycles,
            OfferedEngine::Eevee => scene.sample_counts.eevee,
        }),
        resolution_percentage: Some(scene.resolution_percentage),
        engine: Some(scene.engine_as_read.clone()),
        device: payload.device.map(|_| scene.device),
        view_layer_pins: payload
            .view_layer_pins
            .iter()
            .filter_map(|pin| {
                let layer = scene
                    .view_layers
                    .iter()
                    .find(|layer| layer.name.as_bytes() == pin.layer.as_bytes())?;
                let switch = match &layer.animation {
                    SwitchAnimation::None { value } => TheFilesSwitch::Static(*value),
                    _ => TheFilesSwitch::Animated,
                };
                Some((pin.layer.clone(), switch))
            })
            .collect(),
        simplify: TheFilesSimplify {
            use_simplify: payload
                .simplify
                .use_simplify
                .map(|_| scene.simplify.use_simplify),
            subdivision_render: payload
                .simplify
                .subdivision_render
                .map(|_| scene.simplify.subdivision_render),
            child_particles_render: payload
                .simplify
                .child_particles_render
                .map(|_| scene.simplify.child_particles_render),
            texture_limit_render: payload
                .simplify
                .texture_limit_render
                .as_ref()
                .map(|_| scene.simplify.texture_limit_render.clone()),
        },
    })
}

impl crate::engine::WhatThePageShows {
    fn into_offered(self) -> Option<OfferedEngine> {
        match self {
            crate::engine::WhatThePageShows::Offered(engine) => Some(engine),
            crate::engine::WhatThePageShows::TheFilesOwn(_) => None,
        }
    }
}

/// **Y5: the pin pairs and the severing are part of the launch material**,
/// riding the same material as the redirect and the remap; **Y6: the known
/// evidence bound rides with the mechanism** — everything shown working
/// through the launch-script lever was shown at small sizes, and nothing at a
/// size a real job reaches was tried (§13 item 6).
pub const EVERYTHING_IMPOSED_THROUGH_THE_LEVER_IS_SHOWN_AT_SMALL_SIZES_ONLY: bool = true;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::file_half::tests::{a_blender, a_recorded_reading};
    use crate::frames::parse_the_frame_field;
    use crate::store::Pin;

    fn kitchen() -> NamedAnswer {
        NamedAnswer {
            name: ExactName::as_the_reading_produced_it("Kitchen"),
            provenance: Provenance::ARecordedReading(a_recorded_reading().identity()),
        }
    }

    fn an_untouched_page(
        scene: Option<&NamedAnswer>,
        standing: Option<&RecordedReading>,
    ) -> Payload {
        compose(Composition {
            version_pick: &a_blender(),
            scene,
            answers: &SceneEntries::default(),
            standing,
            stale: false,
        })
    }

    // `Render Settings` Y1c: an old job stays on; a saved off choice survives.
    #[test]
    fn simulation_choice_reads_old_jobs_on_and_keeps_off() {
        let mut carried = an_untouched_page(None, None).as_carried();
        carried.0.as_object_mut().unwrap().remove("prepare_simulations");
        let old = Payload::read_back(&carried).unwrap();
        assert!(old.prepare_simulations);
        carried.0["prepare_simulations"] = serde_json::json!(false);
        let off = Payload::read_back(&carried).unwrap();
        assert_eq!(off.as_carried().0.get("prepare_simulations"), Some(&serde_json::json!(false)));
    }

    #[test]
    fn an_untouched_page_composes_the_version_pick_and_nothing_else() {
        let payload = an_untouched_page(None, None);
        assert_eq!(payload.version_pick, VersionPick::of(&a_blender()));
        assert_eq!(payload.scene, None);
        assert_eq!(payload.frame_range, None);
        assert_eq!(payload.sample_count, None);
        assert_eq!(payload.resolution_percentage, None);
        assert_eq!(payload.engine, None);
        assert_eq!(payload.device, None);
        assert!(payload.view_layer_pins.is_empty());
        assert!(payload.simplify.is_empty());
        assert_eq!(
            payload.the_manifests_frame_range(),
            manifest::FrameRange::TheFilesOwn
        );
        assert_eq!(payload.the_manifests_scene_field(), None);
    }

    #[test]
    fn no_reading_value_is_ever_in_the_payload_however_the_file_reads() {
        // A reading stands, with values for everything; an untouched page
        // still composes no entry from it.
        let reading = a_recorded_reading();
        let payload = an_untouched_page(None, Some(&reading));
        assert_eq!(payload.sample_count, None);
        assert_eq!(payload.resolution_percentage, None);
        assert_eq!(payload.device, None);
        assert_eq!(payload.engine, None);
    }

    #[test]
    fn the_users_answers_compose_one_entry_each_and_names_anchor_against_the_reading() {
        let reading = a_recorded_reading();
        let scene = kitchen();
        let mut answers = SceneEntries {
            prepare_simulations: None,
            frame_range: Some(parse_the_frame_field("1-5").unwrap()),
            frame_range_text: Some("1-5".into()),
            sample_count: Some(128),
            resolution_percentage: Some(50),
            engine_pick: Some(OfferedEngine::Cycles),
            device: Some(DeviceChoice::TheProcessor),
            pins: vec![Pin {
                layer: NamedAnswer {
                    name: ExactName::as_the_reading_produced_it("Detail"),
                    provenance: scene.provenance.clone(),
                },
                on: true,
            }],
            simplify: SimplifyEntries::default(),
            what_a_loaded_range_stands_on: None,
        };
        answers.simplify.use_simplify = Some(true);
        let payload = compose(Composition {
            version_pick: &a_blender(),
            scene: Some(&scene),
            answers: &answers,
            standing: Some(&reading),
            stale: false,
        });
        assert!(matches!(
            payload.scene.as_ref().unwrap().anchor,
            AnchorState::Anchored { .. }
        ));
        assert_eq!(payload.sample_count, Some(128));
        assert_eq!(
            payload.engine.as_ref().unwrap().identifier,
            Some(ExactName::as_the_reading_produced_it("CYCLES"))
        );
        assert!(matches!(
            payload.view_layer_pins[0].anchor,
            AnchorState::Anchored { .. }
        ));
        assert_eq!(
            payload.the_manifests_frame_range(),
            manifest::FrameRange::TheUsersParsedIntegers(vec![1, 2, 3, 4, 5])
        );
        assert!(matches!(
            payload.the_manifests_scene_field().unwrap().anchor,
            manifest::AnchorState::Anchored { .. }
        ));
        assert_eq!(payload.the_manifests_blender().version, "4.5.12");

        // Stale: to-anchor, and the engine rides without an identifier.
        let stale = compose(Composition {
            version_pick: &a_blender(),
            scene: Some(&scene),
            answers: &answers,
            standing: Some(&reading),
            stale: true,
        });
        assert!(matches!(
            stale.scene.as_ref().unwrap().anchor,
            AnchorState::ToAnchor { .. }
        ));
        assert_eq!(stale.engine.as_ref().unwrap().identifier, None);
        assert!(matches!(
            stale.the_manifests_scene_field().unwrap().anchor,
            manifest::AnchorState::ToAnchor { .. }
        ));
    }

    #[test]
    fn the_payload_is_carried_into_the_manifests_field_and_reads_back_as_itself() {
        let reading = a_recorded_reading();
        let scene = kitchen();
        let answers = SceneEntries {
            sample_count: Some(128),
            ..Default::default()
        };
        let payload = compose(Composition {
            version_pick: &a_blender(),
            scene: Some(&scene),
            answers: &answers,
            standing: Some(&reading),
            stale: false,
        });
        let carried = payload.as_carried();
        assert_eq!(Payload::read_back(&carried), Some(payload));
    }

    #[test]
    fn the_all_scenes_act_needs_a_reading_and_every_name_is_the_readings_own() {
        assert_eq!(the_all_scenes_act(None), None);
        let reading = a_recorded_reading();
        let scenes = the_all_scenes_act(Some(&reading)).unwrap();
        assert_eq!(scenes.len(), 2);
        assert!(scenes
            .iter()
            .all(|scene| matches!(scene.provenance, Provenance::ARecordedReading(_))));
    }

    /// **Y1a as corrected 2026-10-07** (the Creator's second notes, piece
    /// 16): the four settings the entry draws as the file's own are composed
    /// whether or not the payload overrides them — the Creator's job, the
    /// samples and the percentage typed and the frames and the engine left
    /// alone, read *not known* for both. The device, the pins and Simplify
    /// stay one value per override.
    #[test]
    fn the_companion_carries_the_files_frames_samples_percentage_and_engine_and_others_per_override() {
        let reading = a_recorded_reading();
        let scene = kitchen();
        let answers = SceneEntries {
            sample_count: Some(128),
            resolution_percentage: Some(50),
            ..Default::default()
        };
        let payload = compose(Composition {
            version_pick: &a_blender(),
            scene: Some(&scene),
            answers: &answers,
            standing: Some(&reading),
            stale: false,
        });
        let companion = the_companion(&payload, Some(&reading), Some(&scene.name)).unwrap();
        // The file's engine is Cycles, so the count in force is Cycles'.
        assert_eq!(companion.sample_count, Some(64));
        assert_eq!(companion.resolution_percentage, Some(100));
        // Left alone: the file's own, the value the job runs with.
        let held = reading
            .settings
            .scenes
            .iter()
            .find(|held| held.name.as_bytes() == scene.name.as_bytes())
            .unwrap();
        assert_eq!(
            companion.frame_range,
            Some(TheFilesFrameRange {
                start: held.frame_start,
                end: held.frame_end,
                step: held.frame_step,
            })
        );
        assert_eq!(companion.engine, Some(held.engine_as_read.clone()));
        // No override: no companion value for the rest.
        assert_eq!(companion.device, None);
        assert!(companion.view_layer_pins.is_empty());
        assert_eq!(companion.simplify, TheFilesSimplify::default());
        // **Nothing where no reading stood, or no scene was named.**
        assert_eq!(the_companion(&payload, None, Some(&scene.name)), None);
        assert_eq!(the_companion(&payload, Some(&reading), None), None);
    }

    #[test]
    fn the_companion_answers_one_simplify_setting_per_override_and_never_the_whole_read_back() {
        // Y1a: one value per override, never the whole reading. The
        // subdivision cap alone was overridden, so the file's switch,
        // child-particle cap and texture cap stay out of the record.
        let reading = a_recorded_reading();
        let scene = kitchen();
        let mut answers = SceneEntries::default();
        answers.simplify.subdivision_render = Some(2);
        let payload = compose(Composition {
            version_pick: &a_blender(),
            scene: Some(&scene),
            answers: &answers,
            standing: Some(&reading),
            stale: false,
        });
        let companion = the_companion(&payload, Some(&reading), Some(&scene.name)).unwrap();
        assert_eq!(companion.simplify.subdivision_render, Some(6));
        assert_eq!(companion.simplify.use_simplify, None);
        assert_eq!(companion.simplify.child_particles_render, None);
        assert_eq!(companion.simplify.texture_limit_render, None);
    }

    #[test]
    fn a_pin_anchors_against_the_jobs_scenes_own_layers_and_never_another_scenes() {
        // C8. A layer living only in another scene of the file does not
        // anchor: the reading cannot vouch for it as this job's layer.
        let mut reading = a_recorded_reading();
        reading.settings.scenes[1].view_layers =
            vec![crate::file_half::tests::a_layer("Attic", true)];
        let scene = kitchen();
        let a_pin_on = |layer: &str| SceneEntries {
            pins: vec![Pin {
                layer: NamedAnswer {
                    name: ExactName::as_the_reading_produced_it(layer),
                    provenance: scene.provenance.clone(),
                },
                on: true,
            }],
            ..Default::default()
        };
        let composed = |answers: &SceneEntries| {
            compose(Composition {
                version_pick: &a_blender(),
                scene: Some(&scene),
                answers,
                standing: Some(&reading),
                stale: false,
            })
        };
        assert!(matches!(
            composed(&a_pin_on("Detail")).view_layer_pins[0].anchor,
            AnchorState::Anchored { .. }
        ));
        assert!(matches!(
            composed(&a_pin_on("Attic")).view_layer_pins[0].anchor,
            AnchorState::ToAnchor { .. }
        ));
        // And a scene's own name is not a view layer's.
        assert!(matches!(
            composed(&a_pin_on("Kitchen")).view_layer_pins[0].anchor,
            AnchorState::ToAnchor { .. }
        ));
    }

    #[test]
    fn where_no_reading_stood_nothing_is_composed_and_the_entry_says_nothing() {
        let scene = kitchen();
        let answers = SceneEntries {
            sample_count: Some(128),
            ..Default::default()
        };
        let payload = compose(Composition {
            version_pick: &a_blender(),
            scene: Some(&scene),
            answers: &answers,
            standing: None,
            stale: false,
        });
        assert_eq!(the_companion(&payload, None, Some(&scene.name)), None);
    }
}

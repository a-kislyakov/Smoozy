//! P1 step 7's composition as one pure function — the command line and the
//! launch material of §1, composed from the manifest, the map in force, the
//! remap material and the reckoning's integer list — together with L29 and
//! L30, which bound what it may be composed from.
//!
//! **L29: everything the launch needs is on the machine before the job
//! exists, and a re-run reads it rather than recomposing it.** The chain,
//! with nobody connected and Smoozy closed: the immutable submission
//! manifest → the remap material at `remaps/<job identity>` → the launch
//! material composed fresh per attempt from exactly those inputs → the line
//! composed per §1. **This is Row 15**: the redirect, the cards, the remap
//! and the animation form are all carried by the manifest and the asset
//! area, and none of them is something Smoozy applies by hand at render
//! time. **Kept by the signature**: [`AttemptInputs`] takes the manifest's
//! values, the map's, the material's and the reckoning's, and nothing from a
//! connection or a desktop.
//!
//! **L30: a re-run that cannot read the remap material its manifest names
//! must not render, and the attempt fails before the render instruction
//! with that stated.** Composing the material without it would render the
//! artist's own recorded locations in force and finish at success with
//! nothing pointing where it should. The check is P1 step 2's read; here it
//! is held by the type — [`AttemptInputs::remap`] is the material and not an
//! option of one, so an unreadable material never reaches composition.
//!
//! **Markers used in this file that belong to another document, each with
//! the document it belongs to.** `Job files and outputs` D10, M1, M2, M8.
//! `Dependency Upload & Remap` M10, M11. `Render Settings` D1, E1, E4.

use smoozy_dependencies_machine::remap::RemapMaterial;
use smoozy_job_files_machine::manifest::SubmissionManifest;
use smoozy_render_settings_machine::device::{the_resolved_choice, DeviceChoice};
use smoozy_render_settings_machine::engine::the_job_will_use_cycles;
use smoozy_render_settings_machine::file_half::RecordedReading;
use smoozy_render_settings_machine::payload::Payload;
use smoozy_render_settings_machine::reading::SceneReading;
use smoozy_state::identities::Identity;

use crate::frames::{ExplicitFrameList, NoList};
use crate::line::{LaunchLine, LineInputs, NotALine};
use crate::material::{self, LaunchMaterial, MaterialInputs, RedirectAssignment};
use crate::technology::{the_technology_the_manifest_names, NothingToName, Technology};
use crate::versions::TheFourVersions;

/// **What one attempt's composition is given, and it is exactly L29's
/// chain.**
pub struct AttemptInputs<'a> {
    pub job: &'a Identity,
    /// The immutable manifest, read and never recomposed (M2).
    pub manifest: &'a SubmissionManifest,
    /// The payload read back out of the manifest's carried field.
    pub payload: &'a Payload,
    /// **The remap material the manifest names, read at P1 step 2** — a
    /// material, never an absence (L30).
    pub remap: &'a RemapMaterial,
    /// The map in force for this attempt, as its redirect pairs and its
    /// pair for the regular output path (P1 steps 3 and 4; D10).
    pub redirect_pairs: &'a [RedirectAssignment],
    pub regular_output_path: &'a [u8],
    /// The reading in force, and the job's scene in it.
    pub reading_in_force: &'a RecordedReading,
    pub the_jobs_scene: &'a SceneReading,
    /// The engine identifier anchored at P1a, where the payload names an
    /// engine.
    pub anchored_engine: Option<String>,
    /// The reckoning's own integers, in the asked order (P1 step 6).
    pub the_reckonings_list: &'a [i64],
    /// Where the material was placed and read back (L12).
    pub launch_material_path: String,
    /// **Where L8b's script is placed and read back** (L12).
    pub load_time_script_path: String,
    /// **`Job files and outputs` R4 item 8, read off the step-4 reading of
    /// this attempt: whether the job's scene holds a simulation that is not
    /// baked** — which is what L10b's stepping is gated on. Added 2026-10-02.
    pub the_scene_holds_a_simulation_not_baked: bool,
    /// **L10c, `Job files and outputs` R4c, read off the step-4 reading of
    /// this attempt: whether it switched the job's scene from a video to a
    /// still picture**, so the material makes the same switch. Added
    /// 2026-10-27.
    pub the_check_is_written_as_a_picture: bool,
    /// **L17a, `Job files and outputs` I19: whether any destination of the
    /// map in force is video-shaped** — which turns the render instruction
    /// into the first frame, the last, the step and the animation
    /// instruction. Added 2026-10-17, the Creator's eleventh notes, the video
    /// piece; the step since the video-range piece.
    pub a_destination_is_video_shaped: bool,
}

/// **What step 7 composes, and it is written to the attempt record before
/// the render starts** (`Job files and outputs` M8).
#[derive(Debug, Clone, PartialEq)]
pub struct TheComposedAttempt {
    pub line: LaunchLine,
    pub material: LaunchMaterial,
    /// **L8b: what the handler the script before the file registers sets**
    /// — the places `Job files and outputs` D1a sets as the file loads, out
    /// of the same map in force as the material's.
    pub load_time: crate::load_time::TheLoadTimeDocument,
    /// The frame list in asked order, for the attempt record.
    pub frames: ExplicitFrameList,
    /// The technology named, where one was — recorded so `Queue & History`
    /// E12a's ask and this can never be confused: this is what was **sent**.
    pub technology_named: Option<Technology>,
}

/// Why the attempt could not be composed — every one is K14's *failed
/// before the render instruction*, loud and total.
#[derive(Debug, Clone, PartialEq)]
pub enum CannotComposeTheAttempt {
    /// The manifest names a version outside the four this contract is
    /// composed for.
    AVersionThisContractIsNotComposedFor {
        version: String,
    },
    /// L23's lookup answered with nothing — no card, no row, a make it
    /// could not serve — on a job set to use the cards.
    NoTechnologyToName(NothingToName),
    /// The reckoning's list had no frame to name.
    NoFrameToName(NoList),
    TheMaterial(material::CannotCompose),
    TheLine(NotALine),
}

impl CannotComposeTheAttempt {
    /// **Why the attempt could not be composed, in standard software words**
    /// (Rule 9) — beside the type, so the ending that carries it records a
    /// sentence. Added 2026-09-23, step 10c's fifth pass on Vast, fault 5:
    /// until then this reached the attempt record as this type's own debugging
    /// rendering.
    ///
    /// **A version outside the set says so in the artist's words** —
    /// *Blender 5.5.0 is not supported yet.* (until 2026-10-28, *Smoozy cannot
    /// render with Blender 5.5.0 yet.*), the version as read
    /// (`Specs/Server Setup`, the sentence added 2026-10-18 for the Creator's
    /// thirteenth notes, note 8). It said *Smoozy cannot compose a render for
    /// Blender 5.4.0*, with no full stop, until then; 5.4 joined the set the
    /// same day, read off its alpha at the desk.
    pub fn in_words(&self) -> String {
        match self {
            CannotComposeTheAttempt::AVersionThisContractIsNotComposedFor { version } => {
                format!("Blender {version} is not supported yet.")
            }
            CannotComposeTheAttempt::NoTechnologyToName(nothing) => nothing.in_words(),
            CannotComposeTheAttempt::NoFrameToName(why) => why.in_words().to_string(),
            CannotComposeTheAttempt::TheMaterial(why) => why.in_words(),
            CannotComposeTheAttempt::TheLine(why) => why.in_words(),
        }
    }
}

/// **P1 step 7, performed.** Nothing here reaches a machine; every input
/// is a value the worker already holds.
pub fn compose_the_attempt(
    inputs: AttemptInputs<'_>,
) -> Result<TheComposedAttempt, CannotComposeTheAttempt> {
    let version = smoozy_server_setup_machine::blender::Version::parse(&inputs.manifest.blender.version)
        .and_then(TheFourVersions::of)
        .ok_or_else(
            || CannotComposeTheAttempt::AVersionThisContractIsNotComposedFor {
                version: inputs.manifest.blender.version.clone(),
            },
        )?;

    // L21: the technology is named exactly where a Cycles job is set to use
    // the cards — the resolved choice, which is the user's own entry or the
    // scene's read value (`Render Settings` D1). Where the file asks for the
    // processor nothing is named and nothing overrides the file. **And under any
    // other engine nothing is named, L21's bullet of 2026-11-02**: the engine
    // is the user's pick or the file's own by the reading in force
    // (`Render Settings` E1, E4), and both are in hand here, so it is never
    // unknown.
    let a_cycles_job_on_the_cards = the_resolved_choice(inputs.payload.device, inputs.the_jobs_scene)
        == DeviceChoice::TheCards
        && the_job_will_use_cycles(
            inputs.payload.engine.as_ref().map(|entry| entry.pick),
            inputs.the_jobs_scene,
            inputs.reading_in_force,
        );
    let technology_named = if a_cycles_job_on_the_cards {
        Some(
            // **L23's make comes off the manifest** (`Job files and outputs` M1b),
            // because the hardware reading is `Servers`' and is taken on the
            // desktop — the machine cannot take one. **This
            // took a `Cards` argument until 2026-09-20 and every caller on
            // the machine had to pass *the cards could not be established*,
            // so every cards job ended at L23's loud failure before the
            // render.**
            the_technology_the_manifest_names(&inputs.manifest.card_make)
                .map_err(CannotComposeTheAttempt::NoTechnologyToName)?,
        )
    } else {
        None
    };

    let frames = ExplicitFrameList::from_the_reckonings_integers(inputs.the_reckonings_list)
        .map_err(CannotComposeTheAttempt::NoFrameToName)?;

    let material = material::compose(MaterialInputs {
        job: inputs.job,
        version,
        redirect_pairs: inputs.redirect_pairs,
        remap: inputs.remap,
        payload: inputs.payload,
        reading_in_force: inputs.reading_in_force,
        the_jobs_scene: inputs.the_jobs_scene,
        regular_output_path: inputs.regular_output_path,
        // **L10b**, where `Job files and outputs` R4 item 8 says yes, over the
        // reckoning's own list.
        the_stepping: material::TheStepping::for_the_list(
            inputs.payload.prepare_simulations && inputs.the_scene_holds_a_simulation_not_baked,
            inputs.the_reckonings_list,
        ),
        // **L10c**, where `Job files and outputs` R4c's reading switched.
        the_check_is_written_as_a_picture: inputs.the_check_is_written_as_a_picture,
    })
    .map_err(CannotComposeTheAttempt::TheMaterial)?;

    let line = LaunchLine::compose(LineInputs {
        version,
        blend_path: inputs.manifest.blend_path_on_the_machine.clone(),
        // L15: exactly where the manifest carries a scene name, byte for
        // byte as the reading recorded it.
        scene: Some(inputs.manifest.scene.name.clone()).filter(|name| !name.is_empty()),
        engine: inputs.anchored_engine,
        // **The map's own bytes, carried rather than converted here.** The
        // line's own gate refuses a value that is not text and names it; the
        // conversion that used to sit here replaced any byte that is not
        // valid text and left the read-back to fail the attempt with the
        // wrong cause recorded (`Job files and outputs` R4a).
        regular_output_path: inputs.regular_output_path.to_vec(),
        launch_material_path: inputs.launch_material_path,
        load_time_script_path: inputs.load_time_script_path,
        frames: frames.clone(),
        a_destination_is_video_shaped: inputs.a_destination_is_video_shaped,
        // **L17b: the job's scene's saved range, as the reading in force
        // recorded it** — what a frame below zero is written from. Added
        // 2026-10-17, the video-range piece.
        the_files_saved_range: crate::frames::TheFilesSavedRange {
            start: inputs.the_jobs_scene.frame_start,
            end: inputs.the_jobs_scene.frame_end,
        },
        technology: technology_named,
    })
    .map_err(CannotComposeTheAttempt::TheLine)?;

    Ok(TheComposedAttempt {
        line,
        material,
        load_time: crate::load_time::for_the_render(inputs.redirect_pairs, &inputs.remap.copies_owed),
        frames,
        technology_named,
    })
}

/// **L29 as a value: a re-run reads the manifest and never recomposes one
/// (M2), and every input of the launch is durable on the machine.**
pub const EVERYTHING_THE_LAUNCH_NEEDS_IS_ON_THE_MACHINE_BEFORE_THE_JOB_EXISTS: bool = true;

/// **A submission manifest as the tests write one, for both folders of this
/// module** — `smoozy-rendering`'s submitting tests build from it too. Behind
/// the `testing` switch for that crate's tests, as `material::testing` is.
/// Moved here from this file's tests on 2026-09-19, unchanged.
#[cfg(any(test, feature = "testing"))]
pub mod testing {
    use super::*;
    use crate::material::testing::{a_blender, an_untouched_payload, four_values};
    use smoozy_job_files_machine::layout::{MachineRoot, TheLayout, WhereThePackageAreaSits};
    use smoozy_job_files_machine::manifest::{
        AnchorState, CarriedAndNotInterpretedHere, FrameRange, SceneField, TheBlenderToRenderBy,
        TheCardMakeAtSubmission, TheRecordedReading,
    };
    use smoozy_job_files_machine::publication::AJobsPaths;

    /// **An ordinary rented GPU machine's manifest**: one NVIDIA make, which
    /// is what `Job files and outputs` M1b carries and what L23's table is
    /// keyed on.
    pub fn a_manifest(job: &Identity, version: &str, scene: &str) -> SubmissionManifest {
        a_manifest_with_make(
            job,
            version,
            scene,
            TheCardMakeAtSubmission::TheMakes(vec!["NVIDIA".to_string()]),
        )
    }

    /// The same, with the card make named — `Job files and outputs` M1b's
    /// field, which is L23's only input on the machine.
    pub fn a_manifest_with_make(
        job: &Identity,
        version: &str,
        scene: &str,
        card_make: TheCardMakeAtSubmission,
    ) -> SubmissionManifest {
        let layout = TheLayout::of(
            MachineRoot::chosen("/ephemeral/smoozy").unwrap(),
            WhereThePackageAreaSits::UnderTheRoot,
        );
        let paths = AJobsPaths::composed(&layout, job, Some("Kitchen")).unwrap();
        SubmissionManifest::composed(
            job,
            None,
            TheBlenderToRenderBy {
                version: version.to_string(),
                build_path: a_blender().path,
            },
            SceneField {
                name: scene.to_string(),
                anchor: AnchorState::Anchored {
                    the_reading_it_came_from: "reading-1".to_string(),
                },
            },
            FrameRange::TheUsersParsedIntegers(vec![1, 2, 3]),
            an_untouched_payload().as_carried(),
            RemapMaterial::manifest_reference(&layout, job),
            CarriedAndNotInterpretedHere(serde_json::json!("everything")),
            "/ephemeral/smoozy/assets/kitchen.blend".to_string(),
            four_values(),
            TheRecordedReading::ThereIsNone,
            card_make,
            smoozy_job_files_machine::manifest::WhereTheJobOutputFolderIs::named_by(&paths),
        )
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    pub(crate) use super::testing::*;
    use crate::material::tests::{
        a_blender, a_job, a_reading, a_remap, a_scene, an_untouched_payload,
    };
    use smoozy_job_files_machine::manifest::TheCardMakeAtSubmission;
    use smoozy_render_settings_machine::file_half::version_text;

    /// **`Job files and outputs` M1b's field as the desktop would have
    /// composed it**, for a reading
    /// carrying cards of one make.
    pub(crate) fn a_make(make: &str) -> TheCardMakeAtSubmission {
        TheCardMakeAtSubmission::TheMakes(vec![make.to_string()])
    }

    fn compose_for(
        version: &str,
        device: Option<DeviceChoice>,
        card_make: TheCardMakeAtSubmission,
        frames: &[i64],
    ) -> Result<TheComposedAttempt, CannotComposeTheAttempt> {
        let job = a_job();
        let manifest = a_manifest_with_make(&job, version, "Kitchen", card_make);
        let remap = a_remap(&job, false);
        let mut payload = an_untouched_payload();
        payload.device = device;
        let reading = a_reading();
        let scene = a_scene("Kitchen");
        compose_the_attempt(AttemptInputs {
            job: &job,
            manifest: &manifest,
            payload: &payload,
            remap: &remap,
            redirect_pairs: &[],
            regular_output_path: b"/ephemeral/smoozy/jobs/j1/frame_####",
            reading_in_force: &reading,
            the_jobs_scene: &scene,
            anchored_engine: None,
            the_reckonings_list: frames,
            launch_material_path: "/ephemeral/smoozy/job-records/j1/material.py".to_string(),
            load_time_script_path: "/ephemeral/smoozy/job-records/j1/load-time.py".to_string(),
            the_scene_holds_a_simulation_not_baked: false,
            the_check_is_written_as_a_picture: false,
            a_destination_is_video_shaped: false,
        })
    }

    #[test]
    fn a_cards_job_names_the_technology_looked_up_from_the_make_and_nothing_else() {
        let composed = compose_for(
            &version_text(a_blender().version),
            None,
            a_make("NVIDIA"),
            &[1, 2, 3],
        )
        .unwrap();
        assert_eq!(composed.technology_named, Some(Technology::Optix));
        let words = composed.line.as_words().unwrap();
        assert_eq!(words[words.len() - 1], "OPTIX");
        assert_eq!(composed.frames.in_the_asked_order(), &[1, 2, 3]);
        assert!(composed.material.contains_no_failure_handling());
    }

    /// **One job composed under the engine named**: the user's pick, the file's
    /// own as the reading gives it, and the make, with the scene saying the cards
    /// (`Rendering` L21's bullet of 2026-11-02).
    fn compose_under(
        version: &str,
        pick: Option<smoozy_render_settings_machine::engine::OfferedEngine>,
        the_files_own: &str,
        card_make: TheCardMakeAtSubmission,
        reading: &RecordedReading,
    ) -> Result<TheComposedAttempt, CannotComposeTheAttempt> {
        use smoozy_render_settings_machine::engine::{the_readings_identifier_for, EngineEntry};
        use smoozy_render_settings_machine::names::ExactName;
        let job = a_job();
        let manifest = a_manifest_with_make(&job, version, "Kitchen", card_make);
        let remap = a_remap(&job, false);
        let mut payload = an_untouched_payload();
        payload.engine = pick.map(|pick| EngineEntry { pick, identifier: None });
        let mut scene = a_scene("Kitchen");
        scene.engine_as_read = ExactName::as_the_reading_produced_it(the_files_own);
        compose_the_attempt(AttemptInputs {
            job: &job,
            manifest: &manifest,
            payload: &payload,
            remap: &remap,
            redirect_pairs: &[],
            regular_output_path: b"/ephemeral/smoozy/jobs/j1/frame_####",
            reading_in_force: reading,
            the_jobs_scene: &scene,
            anchored_engine: pick
                .and_then(|pick| the_readings_identifier_for(pick, reading))
                .map(|identifier| identifier.as_text().to_string()),
            the_reckonings_list: &[1],
            launch_material_path: "/ephemeral/smoozy/job-records/j1/material.py".to_string(),
            load_time_script_path: "/ephemeral/smoozy/job-records/j1/load-time.py".to_string(),
            the_scene_holds_a_simulation_not_baked: false,
            the_check_is_written_as_a_picture: false,
            a_destination_is_video_shaped: false,
        })
    }

    /// **`Rendering` L21's bullet of 2026-11-02: under any engine but Cycles no
    /// technology is named, in every version**, whether the engine is the user's
    /// pick or the file's own, and whatever the scene's Cycles device says. **No
    /// device argument and no bare double dash on the line**, and L23's lookup is
    /// not asked, so a machine with no card to name refuses nothing.
    #[test]
    fn an_eevee_job_on_the_cards_names_no_technology_in_any_version() {
        use smoozy_render_settings_machine::engine::OfferedEngine;
        use smoozy_render_settings_machine::names::ExactName;
        for (version, eevee) in [
            ("4.5.12", "BLENDER_EEVEE_NEXT"),
            ("5.0.1", "BLENDER_EEVEE"),
            ("5.1.2", "BLENDER_EEVEE"),
            ("5.2.3", "BLENDER_EEVEE"),
            ("5.3.0", "BLENDER_EEVEE"),
            ("5.4.0", "BLENDER_EEVEE"),
        ] {
            let mut reading = a_reading();
            reading.settings.offered_engine_identifiers.eevee =
                Some(ExactName::as_the_reading_produced_it(eevee));
            for (what, pick, the_files_own, make) in [
                ("the user's pick over a Cycles file", Some(OfferedEngine::Eevee), "CYCLES", a_make("NVIDIA")),
                ("the file's own", None, eevee, a_make("NVIDIA")),
                ("the file's own on a machine with no card", None, eevee, TheCardMakeAtSubmission::NoCard),
                ("the file's own on a make with no row", None, eevee, a_make("Matrox")),
                ("a third engine the file names", None, "BLENDER_WORKBENCH", a_make("NVIDIA")),
            ] {
                let composed = compose_under(version, pick, the_files_own, make, &reading)
                    .unwrap_or_else(|why| panic!("{version}, {what}: {}", why.in_words()));
                assert_eq!(composed.technology_named, None, "{version}, {what}");
                let words = composed.line.as_words().unwrap();
                assert_eq!(composed.line.the_window().2, None, "{version}, {what}: {words:?}");
                assert!(
                    !composed.line.placed().iter().any(|placed| matches!(placed, crate::line::Argument::Device { .. })),
                    "{version}, {what}: {words:?}"
                );
                assert!(!words.iter().any(|word| word == "OPTIX" || word == "--"), "{version}, {what}: {words:?}");
            }
        }
    }

    /// **A build that registers no Cycles identifier cannot resolve a job to
    /// Cycles**: what the reading gives counts as a third engine
    /// (`Render Settings` E4), as the page shows it.
    #[test]
    fn a_files_engine_the_readings_cycles_identifier_does_not_match_names_no_technology() {
        let mut reading = a_reading();
        reading.settings.offered_engine_identifiers.cycles = None;
        let composed = compose_under(
            &version_text(a_blender().version),
            None,
            "CYCLES",
            a_make("NVIDIA"),
            &reading,
        )
        .unwrap();
        assert_eq!(composed.technology_named, None);
    }

    /// **L21's other side: a Cycles job is as it was, in every version**, its
    /// line ending in the bare double dash, the argument and the technology,
    /// whether Cycles is the user's pick over an EEVEE file or the file's own.
    #[test]
    fn a_cycles_job_on_the_cards_ends_its_line_as_it_always_did() {
        use smoozy_render_settings_machine::engine::OfferedEngine;
        for version in ["4.5.12", "5.0.1", "5.1.2", "5.2.3", "5.3.0", "5.4.0"] {
            for (pick, the_files_own) in [
                (Some(OfferedEngine::Cycles), "BLENDER_EEVEE"),
                (None, "CYCLES"),
            ] {
                let composed =
                    compose_under(version, pick, the_files_own, a_make("NVIDIA"), &a_reading()).unwrap();
                assert_eq!(composed.technology_named, Some(Technology::Optix), "{version}");
                let words = composed.line.as_words().unwrap();
                let Some(crate::line::Argument::Device { version: spelled_for, .. }) = composed.line.placed().last() else {
                    panic!("{version}: {words:?}");
                };
                let the_argument = crate::spellings::DEVICE_ARGUMENT.for_version(*spelled_for).text().unwrap();
                assert_eq!(words[words.len() - 3..], ["--", the_argument, "OPTIX"], "{version}");
            }
        }
    }

    /// **L23's last paragraph**: a cards job on a machine with no card to
    /// name submits like any other and the attempt ends before the render
    /// instruction, naming what could not be served.
    #[test]
    fn a_cards_job_on_a_machine_with_no_technology_to_name_fails_before_the_render_instruction() {
        let refused = compose_for(
            &version_text(a_blender().version),
            None,
            TheCardMakeAtSubmission::NoCard,
            &[1],
        )
        .unwrap_err();
        assert_eq!(
            refused,
            CannotComposeTheAttempt::NoTechnologyToName(NothingToName::TheReadingReturnedNoCard)
        );
        let refused = compose_for(
            &version_text(a_blender().version),
            None,
            a_make("Matrox"),
            &[1],
        )
        .unwrap_err();
        assert!(matches!(
            refused,
            CannotComposeTheAttempt::NoTechnologyToName(NothingToName::AMakeWithNoRow { .. })
        ));
    }

    /// **L21: where the file asks for the processor, no technology is named
    /// and the cards are not even looked at.**
    #[test]
    fn a_processor_job_names_no_technology_whatever_the_cards_reading_says() {
        let mut scene = a_scene("Kitchen");
        scene.device = smoozy_render_settings_machine::reading::DeviceSetting::TheProcessor;
        let job = a_job();
        // **The manifest's make says the cards could not be established** and
        // the composition never looks at it, which is L21's own rule.
        let manifest = a_manifest_with_make(
            &job,
            "5.2.0",
            "Kitchen",
            TheCardMakeAtSubmission::CouldNotBeEstablished,
        );
        let remap = a_remap(&job, false);
        let payload = an_untouched_payload();
        let reading = a_reading();
        let composed = compose_the_attempt(AttemptInputs {
            job: &job,
            manifest: &manifest,
            payload: &payload,
            remap: &remap,
            redirect_pairs: &[],
            regular_output_path: b"/out/frame_####",
            reading_in_force: &reading,
            the_jobs_scene: &scene,
            anchored_engine: None,
            the_reckonings_list: &[4],
            launch_material_path: "/m.py".to_string(),
            load_time_script_path: "/l.py".to_string(),
            the_scene_holds_a_simulation_not_baked: false,
            the_check_is_written_as_a_picture: false,
            a_destination_is_video_shaped: false,
        })
        .unwrap();
        assert_eq!(composed.technology_named, None);
        assert_eq!(composed.line.the_window().2, None);
    }

    /// **The Creator's decision B, 2026-10-02 (`Job files and outputs` D1a):
    /// the copies the remap material owes reach the load-time handler**, so a
    /// geometry-nodes bake folder a sent copy names is set as the file loads
    /// and one no copy names is not.
    #[test]
    fn the_remaps_copies_owed_decide_which_geometry_nodes_bakes_the_handler_sets() {
        use smoozy_job_files_machine::reading::AnAddedWritePlace;
        let job = a_job();
        let manifest =
            a_manifest_with_make(&job, &version_text(a_blender().version), "Kitchen", a_make("NVIDIA"));
        let pairs = vec![
            crate::load_time::tests::an_added_place(
                AnAddedWritePlace::TheGeometryNodesBakeFolder,
                Some(b"//sent/"),
                b"/job/sent",
            ),
            crate::load_time::tests::an_added_place(
                AnAddedWritePlace::TheGeometryNodesBakeFolder,
                Some(b"//beside/"),
                b"/job/beside",
            ),
        ];
        let payload = an_untouched_payload();
        let reading = a_reading();
        let scene = a_scene("Kitchen");
        let compose = |remap: &RemapMaterial| {
            compose_the_attempt(AttemptInputs {
                job: &job,
                manifest: &manifest,
                payload: &payload,
                remap,
                redirect_pairs: &pairs,
                regular_output_path: b"/out/frame_####",
                reading_in_force: &reading,
                the_jobs_scene: &scene,
                anchored_engine: None,
                the_reckonings_list: &[1],
                launch_material_path: "/m.py".to_string(),
                load_time_script_path: "/l.py".to_string(),
                the_scene_holds_a_simulation_not_baked: false,
                the_check_is_written_as_a_picture: false,
                a_destination_is_video_shaped: false,
            })
            .unwrap()
        };
        assert!(compose(&a_remap(&job, false)).load_time.places.is_empty());
        let sent = a_remap(&job, false)
            .with_the_copies_owed(vec![crate::load_time::tests::a_sent_geometry_nodes_bake(b"//sent/")]);
        let places = compose(&sent).load_time.places;
        assert_eq!(places.len(), 1);
        assert_eq!(
            places[0].to,
            crate::load_time::WhereTheHandlerPointsIt::TheJobFolders(vec![
                crate::load_time::AFolderForARecordedLocation {
                    recorded: Some(b"//sent/".to_vec()),
                    folder: b"/job/sent".to_vec(),
                }
            ])
        );
    }

    /// **L10b: `Job files and outputs` R4 item 8 decides whether the
    /// material steps, over the reckoning's own list.**
    #[test]
    fn item_8_decides_whether_the_material_steps_over_the_reckonings_list() {
        let job = a_job();
        let manifest =
            a_manifest_with_make(&job, &version_text(a_blender().version), "Kitchen", a_make("NVIDIA"));
        let remap = a_remap(&job, false);
        let payload = an_untouched_payload();
        let reading = a_reading();
        let scene = a_scene("Kitchen");
        let stepping_of = |says: bool| {
            compose_the_attempt(AttemptInputs {
                job: &job,
                manifest: &manifest,
                payload: &payload,
                remap: &remap,
                redirect_pairs: &[],
                regular_output_path: b"/out/frame_####",
                reading_in_force: &reading,
                the_jobs_scene: &scene,
                anchored_engine: None,
                the_reckonings_list: &[9, 5, 17],
                launch_material_path: "/m.py".to_string(),
                load_time_script_path: "/l.py".to_string(),
                the_scene_holds_a_simulation_not_baked: says,
                the_check_is_written_as_a_picture: false,
                a_destination_is_video_shaped: false,
            })
            .unwrap()
            .material
            .entries()
            .iter()
            .find_map(|entry| match entry {
                crate::material::Entry::Stepping(stepping) => Some(*stepping),
                _ => None,
            })
        };
        assert_eq!(stepping_of(false), None);
        assert_eq!(
            stepping_of(true),
            Some(crate::material::TheStepping {
                the_lists_lowest: 5,
                the_lists_highest: 17
            })
        );
    }

    #[test]
    fn a_version_outside_the_four_composes_nothing_and_names_itself() {
        let refused = compose_for("4.4.0", None, a_make("NVIDIA"), &[1]).unwrap_err();
        assert_eq!(
            refused,
            CannotComposeTheAttempt::AVersionThisContractIsNotComposedFor {
                version: "4.4.0".to_string()
            }
        );
    }

    /// **5.3 composes, on the cards and on the processor, and a version past
    /// it still names itself.** The pass before 0.1.9 on Vast, new fault 1:
    /// the 5.3.0 Alpha installed, and every job on it failed with *Smoozy
    /// cannot compose a render for Blender 5.3.0*. Its names were read off the
    /// alpha at the desk (`crate::spellings::THE_5_3_LOOKUP`).
    #[test]
    fn the_five_three_alpha_composes_and_five_four_names_itself() {
        let cards = compose_for("5.3.0", Some(DeviceChoice::TheCards), a_make("NVIDIA"), &[1, 3])
            .expect("5.3 composes on the cards");
        let words = cards.line.as_words().expect("the line is words");
        assert_eq!(
            &words[..6],
            &["--background", "--enable-autoexec", "--python-exit-code", "63", "--log-level", "info"]
        );
        let dash = words.iter().position(|word| word == "--").expect("the bare double dash");
        assert_eq!(words[dash + 1], ["--cycles", "-device"].concat());
        compose_for("5.3.0", Some(DeviceChoice::TheProcessor), a_make("NVIDIA"), &[1])
            .expect("5.3 composes on the processor");
    }

    /// **5.4 composes, on the cards and on the processor, and a version past
    /// it still names itself.** The Creator's thirteenth notes, note 8: a 5.2.2
    /// file on the 5.4 alpha went straight to failed. Its names were read off
    /// the alpha at the desk (`crate::spellings::THE_5_4_LOOKUP`).
    #[test]
    fn the_five_four_alpha_composes_and_five_five_names_itself() {
        let cards = compose_for("5.4.0", Some(DeviceChoice::TheCards), a_make("NVIDIA"), &[1, 3])
            .expect("5.4 composes on the cards");
        let words = cards.line.as_words().expect("the line is words");
        assert_eq!(
            &words[..6],
            &["--background", "--enable-autoexec", "--python-exit-code", "63", "--log-level", "info"]
        );
        let dash = words.iter().position(|word| word == "--").expect("the bare double dash");
        assert_eq!(words[dash + 1], ["--cycles", "-device"].concat());
        assert_eq!(words[dash + 2], "OPTIX");
        let on_the_processor = compose_for("5.4.0", Some(DeviceChoice::TheProcessor), a_make("NVIDIA"), &[1])
            .expect("5.4 composes on the processor");
        let words = on_the_processor.line.as_words().expect("the line is words");
        assert!(!words.iter().any(|word| word == "--"), "nothing is named for the processor");
        assert_eq!(
            compose_for("5.5.0", None, a_make("NVIDIA"), &[1]).unwrap_err(),
            CannotComposeTheAttempt::AVersionThisContractIsNotComposedFor {
                version: "5.5.0".to_string()
            }
        );
    }

    /// **The thirteenth notes, note 8: a job on a Blender newer than the set
    /// fails in the artist's words, the version as read** — `Specs/Server
    /// Setup`'s sentence, *Smoozy cannot render with Blender 5.5.0 yet.*
    #[test]
    fn a_version_past_the_set_fails_saying_smoozy_cannot_render_with_it_yet() {
        let refused = compose_for("5.5.0", None, a_make("NVIDIA"), &[1]).unwrap_err();
        assert_eq!(refused.in_words(), "Blender 5.5.0 is not supported yet.");
    }

    #[test]
    fn an_empty_reckoning_composes_nothing() {
        let refused = compose_for("5.2.0", None, a_make("NVIDIA"), &[]).unwrap_err();
        assert_eq!(
            refused,
            CannotComposeTheAttempt::NoFrameToName(NoList::NothingToName)
        );
        const _: () = assert!(EVERYTHING_THE_LAUNCH_NEEDS_IS_ON_THE_MACHINE_BEFORE_THE_JOB_EXISTS);
    }

    /// **The map step 10c's sixth pass on Vast recorded for Blender's own
    /// factory cube**, as the machine wrote it into the job record — the
    /// regular output path and all four added write places, the bake output
    /// path read back as `//` and so mapped to the job output folder itself.
    /// Nothing in it was edited.
    const THE_SIXTH_PASS_CUBES_MAP: &str = r#"
{"pairs":[{"destination":{"kind":"TheRegularOutputPath","recorded_location":[47,116,109,112,
47],"format":{"as_the_file_records_it":[80,78,71],"the_ending_it_writes":[46,112,110,103],
"is_video_shaped":false},"origin":"TheScenesOutputPath","scene":"TheJobsOwnScene",
"the_view_layers_that_feed_it":[[86,105,101,119,76,97,121,101,114]]},
"outcome":{"UnderTheJobsFolder":{"new_folder":[47,114,111,111,116,47,115,109,111,111,122,
121,47,106,111,98,115,47,49,52,56,49,48,56,57,56,55,97,51,102,56,49,98,54,99,100,50,54,50,
53,102,54,54,54,53,49,57,56,101,97,45,112,49,50,118,54,32,99,117,98,101,32,99,112,117,47,
116,109,112],"further_values":[]}}},
{"destination":{"kind":{"AnAddedWritePlace":"TheBakeOutputPath"},"recorded_location":[47,
47],"format":null,"origin":{"AnAddedWritePlaceNamed":"TheBakeOutputPath"},
"scene":"TheJobsOwnScene","the_view_layers_that_feed_it":[]},
"outcome":{"UnderTheJobsFolder":{"new_folder":[47,114,111,111,116,47,115,109,111,111,122,
121,47,106,111,98,115,47,49,52,56,49,48,56,57,56,55,97,51,102,56,49,98,54,99,100,50,54,50,
53,102,54,54,54,53,49,57,56,101,97,45,112,49,50,118,54,32,99,117,98,101,32,99,112,117],
"further_values":[]}}},{"destination":{"kind":{"AnAddedWritePlace":"ThePointCache"},
"recorded_location":null,"format":null,"origin":{"AnAddedWritePlaceNamed":"ThePointCache"},
"scene":"TheJobsOwnScene","the_view_layers_that_feed_it":[]},
"outcome":{"UnderTheJobsFolder":{"new_folder":[47,114,111,111,116,47,115,109,111,111,122,
121,47,106,111,98,115,47,49,52,56,49,48,56,57,56,55,97,51,102,56,49,98,54,99,100,50,54,50,
53,102,54,54,54,53,49,57,56,101,97,45,112,49,50,118,54,32,99,117,98,101,32,99,112,117,47,
112,111,105,110,116,99,97,99,104,101],
"further_values":[{"setting":"point_cache.use_disk_cache","value":"On"},
{"setting":"point_cache.use_external","value":"On"},{"setting":"point_cache.name",
"value":{"NonEmptyText":"smoozy"}}]}}},
{"destination":{"kind":{"AnAddedWritePlace":"TheGeometryNodesBakeFolder"},
"recorded_location":null,"format":null,
"origin":{"AnAddedWritePlaceNamed":"TheGeometryNodesBakeFolder"},"scene":"TheJobsOwnScene",
"the_view_layers_that_feed_it":[]},"outcome":{"UnderTheJobsFolder":{"new_folder":[47,114,
111,111,116,47,115,109,111,111,122,121,47,106,111,98,115,47,49,52,56,49,48,56,57,56,55,97,
51,102,56,49,98,54,99,100,50,54,50,53,102,54,54,54,53,49,57,56,101,97,45,112,49,50,118,54,
32,99,117,98,101,32,99,112,117,47,103,101,111,109,101,116,114,121,45,110,111,100,101,115,45,
98,97,107,101],"further_values":[]}}},
{"destination":{"kind":{"AnAddedWritePlace":"TheSequencerProxyFolder"},
"recorded_location":null,"format":null,
"origin":{"AnAddedWritePlaceNamed":"TheSequencerProxyFolder"},"scene":"TheJobsOwnScene",
"the_view_layers_that_feed_it":[]},"outcome":{"UnderTheJobsFolder":{"new_folder":[47,114,
111,111,116,47,115,109,111,111,122,121,47,106,111,98,115,47,49,52,56,49,48,56,57,56,55,97,
51,102,56,49,98,54,99,100,50,54,50,53,102,54,54,54,53,49,57,56,101,97,45,112,49,50,118,54,
32,99,117,98,101,32,99,112,117,47,115,101,113,117,101,110,99,101,114,45,112,114,111,120,
121],"further_values":[]}}}]}
"#;

    /// **A factory cube renders, on either device.** On step 10c's sixth pass
    /// on Vast this exact map ended both jobs before the render instruction:
    /// the cards job at the card make, and the processor job at *Smoozy has no
    /// name for the folder setting of one of this file's write places (the bake
    /// output folder)* — which is every job, because the default scene carries
    /// the bake output path.
    ///
    /// **The make is the one the desktop's reading now gives an NVIDIA card**,
    /// read off `Servers — values` rather than written here, so this test and
    /// the desktop cannot hold two spellings of it.
    #[test]
    fn the_sixth_pass_cube_composes_on_the_cards_and_on_the_processor() {
        use smoozy_job_files_machine::mapping::TheMap;
        use smoozy_job_files_machine::reading::{AnAddedWritePlace, TheVersionInHand};
        let map: TheMap = serde_json::from_str(THE_SIXTH_PASS_CUBES_MAP).expect("the pass's map");
        let settled = smoozy_job_files_machine::redirect::compose_the_launch_material(
            &map,
            TheVersionInHand::of(5, 2),
            "148108987a3f81b6cd2625f6665198ea",
            smoozy_job_files_machine::manifest::CarriedAndNotInterpretedHere(serde_json::json!({})),
        );
        let regular = settled
            .the_regular_output_path
            .expect("the regular output path is in the map")
            .value;
        let the_make = a_make(smoozy_servers_values::hardware::NVIDIA);
        for device in [None, Some(DeviceChoice::TheProcessor)] {
            let job = a_job();
            let manifest = a_manifest_with_make(&job, "5.2.2", "Kitchen", the_make.clone());
            let remap = a_remap(&job, false);
            let mut payload = an_untouched_payload();
            payload.device = device;
            payload.sample_count = device.map(|_| 1024);
            let reading = a_reading();
            let scene = a_scene("Kitchen");
            let composed = compose_the_attempt(AttemptInputs {
                job: &job,
                manifest: &manifest,
                payload: &payload,
                remap: &remap,
                redirect_pairs: &settled.assignments,
                regular_output_path: &regular,
                reading_in_force: &reading,
                the_jobs_scene: &scene,
                anchored_engine: None,
                the_reckonings_list: &[1, 2, 3],
                launch_material_path: "/root/smoozy/job-records/j/material-1.py".to_string(),
                load_time_script_path: "/root/smoozy/job-records/j/load-time-1.py".to_string(),
                the_scene_holds_a_simulation_not_baked: false,
                the_check_is_written_as_a_picture: false,
                a_destination_is_video_shaped: false,
            })
            .unwrap_or_else(|why| panic!("{device:?}: {}", why.in_words()));
            assert_eq!(
                composed.technology_named,
                device.is_none().then_some(Technology::Optix),
                "{device:?}"
            );
            let set: Vec<AnAddedWritePlace> = composed
                .material
                .entries()
                .iter()
                .filter_map(|entry| match entry {
                    material::Entry::Redirect(assignment) => {
                        assignment.destination.as_an_added_write_place()
                    }
                    _ => None,
                })
                .collect();
            // **Three, since 2026-10-02**: the geometry-nodes bake folder is
            // set as the file loads and never by the material (`Job files and
            // outputs` D1a), and which of its bakes is the Creator's decision
            // B. The other three are set here, none passed over.
            assert_eq!(
                set,
                vec![
                    AnAddedWritePlace::TheBakeOutputPath,
                    AnAddedWritePlace::ThePointCache,
                    AnAddedWritePlace::TheSequencerProxyFolder,
                ],
                "every added write place the material sets is set, not passed over"
            );
        }
    }

    /// **L17a through the composition: the map's video shape reaches the
    /// line**, a whole unbroken range composes as the first frame, the last,
    /// the step and the animation instruction, a stepped one keeps its step
    /// (the video-range piece), and a gapped one ends the attempt with the line's own words.
    #[test]
    fn a_video_job_composes_as_one_run_and_a_gapped_one_is_refused_in_words() {
        let job = a_job();
        let manifest =
            a_manifest_with_make(&job, &version_text(a_blender().version), "Kitchen", a_make("NVIDIA"));
        let remap = a_remap(&job, false);
        let payload = an_untouched_payload();
        let reading = a_reading();
        let scene = a_scene("Kitchen");
        let compose = |list: &[i64], video: bool| {
            compose_the_attempt(AttemptInputs {
                job: &job,
                manifest: &manifest,
                payload: &payload,
                remap: &remap,
                redirect_pairs: &[],
                regular_output_path: b"/out/_preview",
                reading_in_force: &reading,
                the_jobs_scene: &scene,
                anchored_engine: None,
                the_reckonings_list: list,
                launch_material_path: "/m.py".to_string(),
                load_time_script_path: "/l.py".to_string(),
                the_scene_holds_a_simulation_not_baked: false,
                the_check_is_written_as_a_picture: false,
                a_destination_is_video_shaped: video,
            })
        };
        let whole: Vec<i64> = (1..=90).collect();
        let composed = compose(&whole, true).unwrap();
        let words = composed.line.as_words().unwrap();
        let at = words.iter().position(|word| word == "/m.py").unwrap();
        assert_eq!(&words[at + 1..at + 8], &["-s", "1", "-e", "90", "-j", "1", "-a"]);
        assert!(!words.contains(&"--render-frame".to_string()));
        // **The attempt record still carries the list** (M8), whole.
        assert_eq!(composed.frames.in_the_asked_order(), whole.as_slice());

        let stepped = compose(&[1, 3, 5, 7, 9], true).unwrap().line.as_words().unwrap();
        let at = stepped.iter().position(|word| word == "/m.py").unwrap();
        assert_eq!(&stepped[at + 1..at + 8], &["-s", "1", "-e", "9", "-j", "2", "-a"]);

        let pictures = compose(&whole, false).unwrap().line.as_words().unwrap();
        let at = pictures.iter().position(|word| word == "--render-frame").unwrap();
        assert_eq!(pictures[at + 1], "1..90");

        let refused = compose(&[1, 2, 3, 10, 11, 12], true).unwrap_err();
        assert!(matches!(
            refused,
            CannotComposeTheAttempt::TheLine(NotALine::AVideosFramesHaveGaps { .. })
        ));
        assert_eq!(
            refused.in_words(),
            "this job writes a video, and its frames have gaps or are out of order"
        );
    }

    /// **L17b through the composition: a frame below zero is written from the
    /// job's scene's own saved range, as the reading in force recorded it** —
    /// the scene the line names, and never another scene of the file.
    #[test]
    fn frames_below_zero_are_written_from_the_jobs_scenes_saved_range() {
        let job = a_job();
        let manifest =
            a_manifest_with_make(&job, &version_text(a_blender().version), "Kitchen", a_make("NVIDIA"));
        let remap = a_remap(&job, false);
        let payload = an_untouched_payload();
        let reading = a_reading();
        let mut scene = a_scene("Kitchen");
        scene.frame_start = -5;
        scene.frame_end = 5;
        let compose = |list: &[i64], video: bool| {
            compose_the_attempt(AttemptInputs {
                job: &job,
                manifest: &manifest,
                payload: &payload,
                remap: &remap,
                redirect_pairs: &[],
                regular_output_path: b"/out/_preview",
                reading_in_force: &reading,
                the_jobs_scene: &scene,
                anchored_engine: None,
                the_reckonings_list: list,
                launch_material_path: "/m.py".to_string(),
                load_time_script_path: "/l.py".to_string(),
                the_scene_holds_a_simulation_not_baked: false,
                the_check_is_written_as_a_picture: false,
                a_destination_is_video_shaped: video,
            })
        };
        let pictures = compose(&[-3, -2, -1, 0], false).unwrap().line.as_words().unwrap();
        let at = pictures.iter().position(|word| word == "/m.py").unwrap();
        assert_eq!(&pictures[at + 1..at + 3], &["--render-frame", "+2..+4,0"]);
        let video = compose(&[-3, -2, -1, 0], true).unwrap().line.as_words().unwrap();
        let at = video.iter().position(|word| word == "/m.py").unwrap();
        assert_eq!(&video[at + 1..at + 8], &["-s", "+2", "-e", "0", "-j", "1", "-a"]);
        let refused = compose(&[-6, -5], false).unwrap_err();
        assert_eq!(
            refused.in_words(),
            "frame -6 is below 0 and outside the scene's frame range, so it cannot be rendered"
        );
    }

    /// **L15 on the line: the scene argument carries the manifest's bytes.**
    #[test]
    fn the_scene_argument_is_the_manifests_own_bytes() {
        let composed = compose_for("5.2.0", None, a_make("NVIDIA"), &[1]).unwrap();
        let words = composed.line.as_words().unwrap();
        let at = words.iter().position(|word| word == "-S").unwrap();
        assert_eq!(words[at + 1], "Kitchen");
    }
}

//! Submitting — §4.1 S1–S3, S3a; §4.2 S4, S4a, S4b, S5, S5a, S6, S7, S8;
//! §4.4's composition half S13 (with S14–S19 cited); §4.5 S20. **The act,
//! the unit, the identities, the gate, the manifest, and the first-frame
//! pair.**
//!
//! **S1: submitting is the user's act, made on a connection, and it is the
//! only way a job comes into being.** The machine is never chosen: a job
//! renders where its `.blend` already is. Submitting starts no transfer of
//! the user's files; what the act moves is Smoozy's own submission
//! material. Submitting does not consult paid access.
//!
//! **S2: one act, one group; one scene, one job; and the partition unit is
//! the scene-job.** The act is partial across scene-jobs and atomic within
//! one — Row 2 and the spec's two promises held together by `State,
//! identity and recovery` §9.6's partition unit, declared at §7.
//!
//! **S3: every identity is issued on the desktop before anything is sent**
//! — the job identity is `Queue & History` Y1–Y2's whole, cited; the
//! submission command's operation identity is the idempotency key. Fact 8's
//! ground is exactly this ordering: 230 usable cut connections, zero
//! disagreements.
//!
//! **S4: one scene-job's submission is an ordered unit** — the remap
//! material placed, then the job's material published, then the job record
//! written — and where an earlier step fails the later is not attempted,
//! naming what stopped it. **S4a: units against one machine are serialised
//! at the desktop**, which is what fixes the line's order. **S4b: a unit
//! that dies between the publication and the record leaves published
//! material that is never a job, and its removal is this document's
//! recorded debt.**
//!
//! **Markers used in this file that belong to another document, each with
//! the document it belongs to.** `Queue & History`
//! Y1, Y2, Y3, Q3, Q7, X3, N3. `Job files and outputs` M1, M2, M5, M7,
//! M1a, N10. `Dependency Upload & Remap` M10, M11. `State, identity and
//! recovery` §9.3, §9.4, §12. `Server protocol and transfers` C4, C8, S28,
//! S29. `Server Setup` A9a, B1. `Render Settings` E1, E3, Y9, Y10,
//! Y16. `Telegram` V1a, V2, G4. `Rented-server runtime` U1, U2, U5, U9,
//! U10, U11, U22. `Security and licence` §11.
//!
//! **`Server Setup` A5 is not listed above and is written
//! out at each site**, because this plan defines an A5 of its own.

use smoozy_dependencies::remap::RemapMaterial;
use smoozy_job_files::layout::TheLayout;
use smoozy_job_files::manifest::{
    CarriedAndNotInterpretedHere, SubmissionManifest, TheBlenderToRenderBy,
    TheCardMakeAtSubmission, TheRecordedReading,
};
use smoozy_job_files::publication::AJobsPaths;
use smoozy_queue_history::identity::issue_a_job_identity;
use smoozy_render_settings::engine::EngineEntry;
use smoozy_render_settings::names::{ExactName, NamedAnswer};
use smoozy_render_settings::payload::Payload;
use smoozy_render_settings::reporting::{the_hand_over, ReportingChoice};
use smoozy_server_protocol::transfers::SourceReading;
use smoozy_server_setup::addons::{TheReadBack, WhatMayBeClaimed};
use smoozy_state::identities::{Identity, IdentityKind, NoRandomness};
use smoozy_state::outcomes::{Recorded, UnitOutcome};

/// **S3: the identities, issued on the desktop before anything is sent,
/// in this order** — the operation identity for the submission command,
/// which is the idempotency key, then the job identity, with the
/// provisional local record written first (`State, identity and recovery`
/// §7 step 2 — the write is the acting half's).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheIdentities {
    pub submission_command: Identity,
    pub job: Identity,
}

pub fn issue_the_identities() -> Result<TheIdentities, NoRandomness> {
    Ok(TheIdentities {
        submission_command: Identity::issue(IdentityKind::Operation)?,
        job: issue_a_job_identity()?,
    })
}

/// **S2: one act, one group — the group identity issued when the act is
/// composed, for an all-scenes act.** A single-scene act has no group
/// (`Job files and outputs` M1's group field is the group identity where
/// the job came from an all-scenes submission).
pub fn the_group_identity(scenes_in_the_act: usize) -> Result<Option<Identity>, NoRandomness> {
    if scenes_in_the_act > 1 {
        Ok(Some(Identity::issue(IdentityKind::Group)?))
    } else {
        Ok(None)
    }
}

/// **S3a's reading of the target build, as `Server Setup` §12 supplies it**
/// — `Server Setup` A5's read-back, under that clause's claim discipline and
/// no more.
/// **Where no read-back and no recorded reading of that build stands, the
/// desktop holds nothing to judge the pick against** (§9 item 27).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatTheDesktopHoldsAboutTheBuild {
    /// `Server Setup` A5's read-back of that build's add-ons.
    TheReadBack(TheReadBack),
    /// A recorded reading's own registered identifier for the pick is
    /// anchored on the payload's engine entry (`Render Settings` Q2 item 4,
    /// E1) — carried on the entry itself, so this variant says only that
    /// nothing else stands.
    NothingButTheEntry,
}

/// What the gate says.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TheGate {
    /// The scene-job proceeds to S4's unit.
    Proceed,
    /// **Refused before anything of S4's unit runs**: no remap material
    /// placed, no material published, no job record written, no byte of
    /// the scene-job reaches the machine — naming the engine and the build.
    Refused { engine: String, build: String },
    /// **§9 item 27**: nothing stands to judge the pick against, so the
    /// gate does not fire; `Render Settings` E2 fails the pick at the
    /// attempt instead, before the render instruction.
    NothingToJudgeAgainstSoTheAttemptDecides,
}

/// **S3a, performed: a scene-job whose engine pick the build it will render
/// with does not carry is refused before anything of it is sent.** It fires
/// where the payload names an engine — the user's own pick, never the
/// file's — **and** what the desktop holds about the target build says
/// that engine is not there. **No list of the engines a version is supposed
/// to ship is consulted, and this crate composes none** (`Server Setup`
/// B1's bar).
pub fn the_engine_gate(
    payload: &Payload,
    held: &WhatTheDesktopHoldsAboutTheBuild,
    the_target_build: &TheBlenderToRenderBy,
) -> TheGate {
    let Some(EngineEntry { pick, identifier }) = payload.engine.as_ref() else {
        // The file's own recorded engine stands and no engine argument is
        // composed (L19, `Render Settings` E3).
        return TheGate::Proceed;
    };
    if identifier.is_some() {
        // The recorded reading of that build anchored the pick: the build
        // registers the engine.
        return TheGate::Proceed;
    }
    match held {
        WhatTheDesktopHoldsAboutTheBuild::NothingButTheEntry => {
            TheGate::NothingToJudgeAgainstSoTheAttemptDecides
        }
        WhatTheDesktopHoldsAboutTheBuild::TheReadBack(read_back) => {
            let name = the_add_on_name_of(*pick);
            match smoozy_server_setup::addons::what_the_gate_is_given(name, read_back) {
                WhatMayBeClaimed::InstalledAndSwitchedOnLoadableAtThatMoment => TheGate::Proceed,
                WhatMayBeClaimed::NotInstalled => TheGate::Refused {
                    engine: name.to_string(),
                    build: the_target_build.build_path.clone(),
                },
            }
        }
    }
}

/// The add-on name the read-back lists an offered engine under. **This is
/// the name the listing carries and not an engine identifier** — the
/// identifier is a reading's own bytes (L19) and is never composed here.
fn the_add_on_name_of(pick: smoozy_render_settings::engine::OfferedEngine) -> &'static str {
    match pick {
        smoozy_render_settings::engine::OfferedEngine::Cycles => "cycles",
        smoozy_render_settings::engine::OfferedEngine::Eevee => "eevee",
    }
}

/// **S4: the ordered unit's three units, in order — the enumeration the
/// act's §9.4 outcome record holds durably before the first is attempted.**
pub const THE_ORDERED_UNIT: [&str; 3] = [
    "the remap material placed",
    "the job's material published",
    "the job record written",
];

/// **S4a, performed: whether the next scene-job's unit may begin** — only
/// when the previous unit's submission command has a recorded outcome the
/// desktop has read. A unit ending refused holds nothing back; a unit left
/// unaccounted by a drop ends the act's run.
pub fn may_the_next_unit_begin(the_previous_units_record: Option<&Recorded>) -> bool {
    match the_previous_units_record {
        None => true,
        Some(record) => record.the_actions_outcome().is_some(),
    }
}

/// **S4b: the cleanup debt a dead unit leaves — named from the act's own
/// record of what its units published, never from enumerating the
/// machine.** The material set at the job record directory's path, the
/// remap material at `remaps/<job identity>`, and the job output folder
/// where it is empty. Taken by a new action with its own operation
/// identity at the next connection; partial across the three, atomic within
/// each; an already-absent unit recorded *done*.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CleanupDebt {
    pub the_material_set_at: String,
    pub the_remap_material_at: Vec<u8>,
    /// **Absent where the manifest named no folder** — one chosen at render
    /// start (`Job files and outputs` P21) — because no folder was made at
    /// submission for the debt to name.
    pub the_job_output_folder_where_empty: Option<String>,
}

/// **S4b, performed**: where S9's resolution returned C8's third answer —
/// *no record of that identity* — for a scene-job whose material units had
/// completed, the debt names exactly what the units record as published.
pub fn the_cleanup_debt(
    the_units_outcomes: &Recorded,
    paths: &AJobsPaths,
    the_folder_the_manifest_names: Option<&str>,
    layout: &TheLayout,
    job: &Identity,
) -> Option<CleanupDebt> {
    let published = |unit: &str| {
        the_units_outcomes
            .units
            .iter()
            .any(|(name, outcome)| name == unit && matches!(outcome, Some(UnitOutcome::Done)))
    };
    let record_written = published(THE_ORDERED_UNIT[2]);
    if record_written || !published(THE_ORDERED_UNIT[1]) {
        return None;
    }
    Some(CleanupDebt {
        the_material_set_at: paths.job_record_directory.clone(),
        the_remap_material_at: RemapMaterial::location_in(layout, job).as_bytes().to_vec(),
        the_job_output_folder_where_empty: the_folder_the_manifest_names.map(str::to_string),
    })
}

/// **S5a: the per-job reporting field — one value from the closed set of
/// three, held as the identifier and never the wording; the user's per-job
/// answer where they gave one, and otherwise the app-wide item, read once
/// at composition.** Present on every manifest whether or not Telegram is
/// switched on anywhere (`Telegram` V1a), because the manifest is written
/// once and never rewritten and an absence could never be repaired.
///
/// **A mis-citation in the plan, noted and not repaired**: S5a records that
/// `Telegram` V1a cites *the closed set V2 names* and that V2 names no such
/// set — the members, their order and their words are Design's. That is
/// `Telegram`'s to repair; this crate builds to what V1a requires.
pub fn the_reporting_field(
    the_users_per_job_answer: Option<ReportingChoice>,
    the_app_wide_value: ReportingChoice,
) -> CarriedAndNotInterpretedHere {
    let value = the_hand_over(the_users_per_job_answer, the_app_wide_value);
    CarriedAndNotInterpretedHere(
        serde_json::to_value(value).expect("a reporting choice is a value that always renders"),
    )
}

/// What the manifest reads back as, for the role worker — never an
/// absence. **The fourth member reads back with its minutes** (`Telegram`
/// V1a, 2026-10-19): `{"APictureEveryFewMinutes":{"minutes":5}}`.
pub fn the_reporting_choice_on(manifest: &SubmissionManifest) -> Option<ReportingChoice> {
    serde_json::from_value(manifest.reporting_settings.0.clone()).ok()
}

/// **The card make field of `Job files and outputs` M1b, composed from the
/// hardware reading `Servers` holds** (L23, L29).
///
/// **The make alone crosses, never the reading.** No card identity, no bus
/// location, no memory figure and no moment goes on the manifest —
/// `Job files and outputs` M3 allows this field as a snapshot deliberately
/// taken and nothing wider, and L23's
/// table is keyed on the make and on nothing else.
///
/// **Each distinct make once, in the reading's own order.** A reading whose
/// cards are of more than one make carries both, and the machine's own lookup
/// refuses rather than picking a side (L23; [`crate`] has no say in it here).
///
/// **A row with no held reading is `Servers` H1's *never read*** — an ordinary
/// case rather than a gap, and it answers L23's lookup with nothing, which is
/// that clause's own already-written ending. **Nothing is guessed and no
/// submission is withheld.**
pub fn the_card_make_field(
    held: Option<&smoozy_render_settings::readings::HeldHardware>,
) -> TheCardMakeAtSubmission {
    use smoozy_render_settings::readings::HeldHardware;
    use smoozy_servers::hardware::Cards;
    let Some(HeldHardware::Held { reading, .. }) = held else {
        return TheCardMakeAtSubmission::NoReadingStood;
    };
    match &reading.cards {
        Cards::TheCards(cards) => {
            let mut makes: Vec<String> = Vec::new();
            for card in cards {
                let make = card.make.as_key().to_string();
                if !makes.contains(&make) {
                    makes.push(make);
                }
            }
            TheCardMakeAtSubmission::TheMakes(makes)
        }
        Cards::NoCard => TheCardMakeAtSubmission::NoCard,
        Cards::PresentAndUnreadable(_) => TheCardMakeAtSubmission::PresentAndUnreadable,
        Cards::CouldNotBeEstablished => TheCardMakeAtSubmission::CouldNotBeEstablished,
    }
}

/// **S5's inputs: what `Render Settings` hands over and what this document
/// adds** — the job and group identities, the frame range as integers, the
/// reference to the remap material, the job output folder's path, and the
/// per-job reporting field. The payload itself is carried and not
/// interpreted.
pub struct ManifestInputs<'a> {
    pub job: &'a Identity,
    pub group: Option<&'a Identity>,
    pub payload: &'a Payload,
    pub layout: &'a TheLayout,
    /// **The job output folder's field** (`Job files and outputs` M1): named
    /// by the paths the placement makes it from, or chosen at render start
    /// (`Job files and outputs` P21) — the desktop's own case, since it writes
    /// no recorded reading into a manifest.
    pub job_output_folder: smoozy_job_files::manifest::WhereTheJobOutputFolderIs,
    pub blend_path_on_the_machine: String,
    pub file_identity_at_submission: SourceReading,
    pub recorded_reading: TheRecordedReading,
    pub reporting: CarriedAndNotInterpretedHere,
    /// **The card make the hardware reading `Servers` holds gave for the
    /// machine at submission** (`Job files and outputs` M1b).
    ///
    /// **This document's L23 looks the render technology up from it and the
    /// machine cannot take the reading itself**, `Servers` H14 and `Servers`
    /// H15 putting it on the desktop once per primary connection — so the manifest is the
    /// carriage and this is where it is put on.
    pub card_make: TheCardMakeAtSubmission,
}

/// **S5, performed: the manifest composed on the desktop, and nothing in
/// it changes afterwards** (`Job files and outputs` M1, M2). S6: the scene
/// field is the payload's own, which obeys L15's provenance rule — the
/// manifest carries a scene exactly where the payload does, and the
/// unnamed branch is an empty name with its anchor stated.
pub fn compose_the_manifest(inputs: ManifestInputs<'_>) -> SubmissionManifest {
    let scene = inputs.payload.the_manifests_scene_field().unwrap_or(
        smoozy_job_files::manifest::SceneField {
            name: String::new(),
            anchor: smoozy_job_files::manifest::AnchorState::ToAnchor {
                provenance: "no scene named — the file's showing scene renders (L15)".to_string(),
            },
        },
    );
    SubmissionManifest::composed(
        inputs.job,
        inputs.group,
        inputs.payload.the_manifests_blender(),
        scene,
        inputs.payload.the_manifests_frame_range(),
        inputs.payload.as_carried(),
        RemapMaterial::manifest_reference(inputs.layout, inputs.job),
        inputs.reporting,
        inputs.blend_path_on_the_machine,
        inputs.file_identity_at_submission,
        inputs.recorded_reading,
        inputs.card_make,
        inputs.job_output_folder,
    )
}

/// **S6: the all-scenes act names every scene from the same one reading**
/// — composable only where the file was read, every name the reading's own
/// bytes (`Render Settings` Y11, cited through `the_all_scenes_act`). This
/// checks the act's shape: every scene-job of a group carries a name, and
/// no two carry the same one.
pub fn every_scene_job_of_the_group_carries_its_own_read_name(scenes: &[NamedAnswer]) -> bool {
    let names: Vec<&ExactName> = scenes.iter().map(|scene| &scene.name).collect();
    !names.is_empty()
        && names.iter().enumerate().all(|(at, name)| {
            !names[..at]
                .iter()
                .any(|earlier| earlier.as_bytes() == name.as_bytes())
        })
}

/// **S7, S13: the first-frame check submits two ordinary jobs per scene —
/// the first frame of the range as its own job, and the whole range, its
/// first frame included, as a second job that arrives on the machine already
/// held.** Two jobs only where the range holds more than one frame: on a
/// one-frame range the act composes one ordinary job with no hold — the
/// alternative is a second job with an empty frame set, which L18 has no
/// frame to emit for.
///
/// *Corrected 2026-10-02 (decision C, Creator-stated, S13 as amended): the
/// second job was the rest of the range, starting one frame after the first.
/// It is now the range exactly as asked, so the first frame renders twice —
/// and a simulation never baked gets an unbroken run from the range's start.*
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TheFirstFrameSplit {
    OneJobWithNoHold {
        frames: Vec<i64>,
    },
    TwoJobs {
        the_first_frame: Vec<i64>,
        the_whole_range_held: Vec<i64>,
    },
}

pub fn the_first_frame_split(frames_in_asked_order: &[i64]) -> Option<TheFirstFrameSplit> {
    match frames_in_asked_order {
        [] => None,
        [only] => Some(TheFirstFrameSplit::OneJobWithNoHold {
            frames: vec![*only],
        }),
        [first, _, ..] => Some(TheFirstFrameSplit::TwoJobs {
            the_first_frame: vec![*first],
            the_whole_range_held: frames_in_asked_order.to_vec(),
        }),
    }
}

/// **S13: the hold instruction riding the second job's submission** — in
/// the same supervisor act that writes its job record, the supervisor
/// places the hold record: origin *submission*, subject the second job's
/// identity, reason the check, dependency the first job's identity with the
/// release predicate `Queue & History` H7 declares (`Rented-server runtime`
/// U1, U2). The job record takes state *held* and never enters the line
/// (U5).
///
/// **The dependency is optional from 2026-10-12** — S18a's hold for the driver
/// names none, and no release predicate with it. Held-job piece A.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheHoldInstruction {
    pub origin: &'static str,
    pub subject: Identity,
    pub reason: &'static str,
    pub dependency: Option<Identity>,
}

pub fn the_hold_instruction(
    the_second_job: &Identity,
    the_first_job: &Identity,
) -> TheHoldInstruction {
    TheHoldInstruction {
        origin: "submission",
        subject: the_second_job.clone(),
        reason: "the first-frame check",
        dependency: Some(the_first_job.clone()),
    }
}

/// **S18a: the reason *the driver*, this document's to declare as the check's
/// is** (S13; `Rented-server runtime` U2). The machine carries it and never
/// interprets it; the desktop reads it back off the queue reading to tell a
/// hold for the driver from every other (`Server Setup` R1c, `Queue & History`
/// H12). **Declared once, here.** Added 2026-10-12, held-job piece A.
pub const THE_DRIVER: &str = "the driver";

/// **S18a: a job that would meet a driver below its Blender's OptiX bar, on a
/// server where a swap is owed, is submitted held** — origin *submission*,
/// subject the job's identity, reason [`THE_DRIVER`], **no dependency and no
/// release predicate**. Nothing on the machine waits for a person: the
/// release is the desktop's, by `Rented-server runtime` U9's ordinary command,
/// when `Server Setup` R1c says. Which jobs are held is `Server Setup` R1c's
/// and is decided on the desktop. Added 2026-10-12, held-job piece A.
pub fn the_driver_hold_instruction(the_job: &Identity) -> TheHoldInstruction {
    TheHoldInstruction {
        origin: "submission",
        subject: the_job.clone(),
        reason: THE_DRIVER,
        dependency: None,
    }
}

/// **S18a: a job already held at submission carries that hold and no second
/// one.** Where the first-frame check meets the driver, the first job is held
/// for the driver and the second keeps S13's hold, whose dependency is the
/// first job, so it behaves as S15 already says. `held_behind` is S13's first
/// job, where this job is the check's second; `held_for_the_driver` is
/// `Server Setup` R1c's answer for this job. Added 2026-10-12, held-job piece A.
pub fn the_hold_this_job_carries(
    the_job: &Identity,
    held_behind: Option<&Identity>,
    held_for_the_driver: bool,
) -> Option<TheHoldInstruction> {
    match held_behind {
        Some(first) => Some(the_hold_instruction(the_job, first)),
        None => held_for_the_driver.then(|| the_driver_hold_instruction(the_job)),
    }
}

/// **S8: a refusal at any unit fails that scene-job whole and touches no
/// other** — the outcome record's unit outcomes for that scene-job, and no
/// other scene-job's.
pub fn a_refusal_fails_that_scene_job_whole_and_touches_no_other(
    records: &[Recorded],
    the_refused: usize,
) -> bool {
    records.iter().enumerate().all(|(at, record)| {
        if at == the_refused {
            record.what_could_not_be_done().len() == record.units.len()
                || record
                    .units
                    .iter()
                    .any(|(_, o)| matches!(o, Some(UnitOutcome::Failed)))
        } else {
            record
                .units
                .iter()
                .all(|(_, o)| !matches!(o, Some(UnitOutcome::NotAttempted { .. })))
        }
    })
}

/// **S14–S17, S19: cited, not decided here.** The user's yes is an ordinary
/// release command with its own operation identity naming the hold identity
/// (`Rented-server runtime` U9–U11); the negative case is the machine's
/// own act; release starts a fresh attempt that takes the pipeline like any
/// other; nothing expires a hold (U22). What this document owns of Rows 4
/// and 23 is exactly S13's composition and S18's outcome.
pub const S14_TO_S17_ARE_CITATIONS_OF_RENTED_SERVER_RUNTIME_SECTION_8: bool = true;

/// **S20: every scene-group is whole on one machine**, taken as this
/// shape's own rule — a submission act never composes one group's jobs
/// against more than one machine (`Telegram` G4's resolution).
pub fn every_scene_group_is_whole_on_one_machine(machines_of_the_groups_jobs: &[Identity]) -> bool {
    machines_of_the_groups_jobs
        .windows(2)
        .all(|pair| pair[0] == pair[1])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::material::testing::{a_blender, a_reading, an_untouched_payload, four_values};
    use smoozy_job_files::layout::{MachineRoot, WhereThePackageAreaSits};
    use smoozy_render_settings::engine::OfferedEngine;
    use smoozy_render_settings::names::Provenance;
    use smoozy_render_settings::payload::{compose, Composition};
    use smoozy_render_settings::store::SceneEntries;
    use smoozy_state::outcomes::{Keeps, Refused, Writing};

    /// A keeper that holds its records in a list — `smoozy-state`'s own is
    /// test-gated, so this crate carries a copy for its tests and for
    /// nothing durable, as `Queue & History` does.
    #[derive(Default)]
    struct AList {
        records: Vec<Recorded>,
    }

    impl Keeps for AList {
        fn enumerate(&mut self, action: &str, units: &[&str]) -> Result<Identity, Refused> {
            let identity = smoozy_state::outcomes::issue()
                .map_err(|_| Refused::TheStore("no randomness".into()))?;
            self.records.push(Recorded {
                identity: identity.clone(),
                action: action.to_string(),
                units: units.iter().map(|unit| (unit.to_string(), None)).collect(),
            });
            Ok(identity)
        }

        fn a_unit_ended(
            &mut self,
            record: &Identity,
            unit: &str,
            outcome: UnitOutcome,
        ) -> Result<(), Refused> {
            let Some(held) = self
                .records
                .iter_mut()
                .find(|held| held.identity == *record)
            else {
                return Err(Refused::TheStore("no such record".into()));
            };
            let Some(entry) = held.units.iter_mut().find(|(owed, _)| owed == unit) else {
                return Err(Refused::ThatUnitWasNotOwed);
            };
            if entry.1.is_some() {
                return Err(Refused::ThatUnitHasAlreadyEnded);
            }
            entry.1 = Some(outcome);
            Ok(())
        }

        fn read(&self, record: &Identity) -> Result<Option<Recorded>, Refused> {
            Ok(self
                .records
                .iter()
                .find(|held| held.identity == *record)
                .cloned())
        }

        fn the_user_removed_it(&mut self, record: &Identity) -> Result<(), Refused> {
            self.records.retain(|held| held.identity != *record);
            Ok(())
        }
    }

    /// **The card make field, over every state a held reading can be in**
    /// (`Job files and outputs` M1b).
    ///
    /// **The make alone crosses**: the test reads the written manifest's own
    /// bytes and holds that no card identity, bus location or memory figure
    /// goes with it, which is what `Job files and outputs` M3 allows this
    /// field on.
    #[test]
    fn the_card_make_field_carries_the_make_and_nothing_else_of_the_reading() {
        use smoozy_render_settings::readings::{HeldHardware, HeldState};
        use smoozy_servers::hardware::{
            Card, CardMake, Cards, HardwareReading, MachineMemory, WhyUnreadable,
        };
        use std::time::{Duration, SystemTime};

        let held = |cards: Cards| {
            HeldHardware::Held {
                state: HeldState::Live,
                reading: HardwareReading::taken(
                    cards,
                    MachineMemory::CouldNotBeEstablished,
                    None,
                    SystemTime::UNIX_EPOCH + Duration::from_secs(1_756_800_000),
                    "row-1",
                    SystemTime::UNIX_EPOCH + Duration::from_secs(1_756_900_000),
                    None,
                ),
            }
        };
        // **Each card named the way a driver names one**, so the make that
        // crosses is the key and never the card's own name.
        let a_card = |make: &str, at: usize| Card {
            unique_identifier: format!("GPU-6e3a0c-{at}"),
            bus_location: format!("0000:0{at}:00.0"),
            make: CardMake::of_a_make(make, &format!("{make} Model {at}")),
            total_memory_bytes: Some(10_737_418_240),
        };

        // **Two cards of one make give one make**, each distinct make once.
        let two = held(Cards::TheCards(vec![a_card("NVIDIA", 0), a_card("NVIDIA", 1)]));
        assert_eq!(
            the_card_make_field(Some(&two)),
            TheCardMakeAtSubmission::TheMakes(vec!["NVIDIA".to_string()])
        );
        // **Two makes are both carried**, and the machine refuses rather than
        // picking a side.
        let mixed = held(Cards::TheCards(vec![a_card("NVIDIA", 0), a_card("AMD", 1)]));
        assert_eq!(
            the_card_make_field(Some(&mixed)),
            TheCardMakeAtSubmission::TheMakes(vec!["NVIDIA".to_string(), "AMD".to_string()])
        );
        // **The three stated absences a reading can carry.**
        assert_eq!(
            the_card_make_field(Some(&held(Cards::NoCard))),
            TheCardMakeAtSubmission::NoCard
        );
        assert_eq!(
            the_card_make_field(Some(&held(Cards::PresentAndUnreadable(
                WhyUnreadable::TheDriverIsAbsent
            )))),
            TheCardMakeAtSubmission::PresentAndUnreadable
        );
        assert_eq!(
            the_card_make_field(Some(&held(Cards::CouldNotBeEstablished))),
            TheCardMakeAtSubmission::CouldNotBeEstablished
        );
        // **`Servers` H1's *never read*, and a row with no held reading at
        // all** — one state, stated rather than guessed.
        assert_eq!(
            the_card_make_field(Some(&HeldHardware::NeverRead)),
            TheCardMakeAtSubmission::NoReadingStood
        );
        assert_eq!(
            the_card_make_field(None),
            TheCardMakeAtSubmission::NoReadingStood
        );

        // **Nothing of the reading but the make reaches the manifest.**
        let job = Identity::issue(IdentityKind::Job).unwrap();
        let layout = a_layout();
        let paths = AJobsPaths::composed(&layout, &job, None).unwrap();
        let manifest = compose_the_manifest(ManifestInputs {
            job: &job,
            group: None,
            payload: &an_untouched_payload(),
            layout: &layout,
            job_output_folder: smoozy_job_files::manifest::WhereTheJobOutputFolderIs::named_by(&paths),
            blend_path_on_the_machine: "/ephemeral/smoozy/assets/kitchen.blend".to_string(),
            file_identity_at_submission: four_values(),
            recorded_reading: TheRecordedReading::ThereIsNone,
            reporting: the_reporting_field(None, ReportingChoice::Everything),
            card_make: the_card_make_field(Some(&two)),
        });
        let written = String::from_utf8(manifest.as_written()).unwrap();
        assert!(written.contains("NVIDIA"), "the make is carried");
        assert!(!written.contains("Model"), "the driver's name for a card does not cross");
        assert!(!written.contains("GPU-6e3a0c"), "no card identity crosses");
        assert!(!written.contains("0000:0"), "no bus location crosses");
        assert!(!written.contains("10737418240"), "no memory figure crosses");
    }

    /// **The card make is one value from end to end, driven from the
    /// driver's own row.** Step 10c's sixth pass on Vast: the reading made the
    /// make from the driver's name for the card, the manifest carried
    /// `"TheMakes": ["NVIDIA GeForce GTX 1650"]`, the machine's table is keyed
    /// on the maker, and the job ended before the render with *Smoozy does not
    /// know how to render on a NVIDIA GeForce GTX 1650*. **Every piece of this
    /// walk is the real one**: `Servers`' own parser of the driver's answer,
    /// `Servers`' own reading of the cards, this crate's manifest field, the
    /// manifest written and read back, and the machine's own lookup.
    #[tokio::test]
    async fn the_card_make_is_one_value_from_the_drivers_row_to_the_machines_table() {
        use smoozy_render_settings::readings::{HeldHardware, HeldState};
        use smoozy_rendering_machine::technology::{
            the_technology_the_manifest_names, Technology,
        };
        use smoozy_server_protocol::transport::ssh::CommandFailure;
        use smoozy_server_protocol::transport::{CommandResult, CommandString};
        use smoozy_servers::hardware::{
            read_the_cards, BusScan, HardwareReading, MachineKind, MachineMemory,
        };
        use smoozy_servers::readings::{ask_the_driver, CommandChannel};
        use std::time::{Duration, SystemTime};

        /// The one command this walk sends, answered as the pass's machine
        /// answered it.
        struct TheDriverAnswers;
        impl CommandChannel for TheDriverAnswers {
            async fn run(
                &mut self,
                _command: CommandString,
            ) -> Result<CommandResult, CommandFailure> {
                Ok(CommandResult {
                    output: b"GPU-6e3a0c1d, 00000000:01:00.0, NVIDIA GeForce GTX 1650, 4096 MiB\n"
                        .to_vec(),
                    complaint: Vec::new(),
                    status: Some(0),
                })
            }
        }

        let driver = ask_the_driver(&mut TheDriverAnswers).await;
        let cards = read_the_cards(BusScan::Populated, driver, MachineKind::AContainer);
        let held = HeldHardware::Held {
            state: HeldState::Live,
            reading: HardwareReading::taken(
                cards,
                MachineMemory::CouldNotBeEstablished,
                None,
                SystemTime::UNIX_EPOCH + Duration::from_secs(1_756_800_000),
                "row-1",
                SystemTime::UNIX_EPOCH + Duration::from_secs(1_756_900_000),
                None,
            ),
        };

        let job = Identity::issue(IdentityKind::Job).unwrap();
        let layout = a_layout();
        let paths = AJobsPaths::composed(&layout, &job, None).unwrap();
        let manifest = compose_the_manifest(ManifestInputs {
            job: &job,
            group: None,
            payload: &an_untouched_payload(),
            layout: &layout,
            job_output_folder: smoozy_job_files::manifest::WhereTheJobOutputFolderIs::named_by(&paths),
            blend_path_on_the_machine: "/ephemeral/smoozy/assets/cube.blend".to_string(),
            file_identity_at_submission: four_values(),
            recorded_reading: TheRecordedReading::ThereIsNone,
            reporting: the_reporting_field(None, ReportingChoice::Everything),
            card_make: the_card_make_field(Some(&held)),
        });
        let on_the_machine =
            smoozy_job_files::manifest::SubmissionManifest::read_back(&manifest.as_written())
                .expect("the manifest reads back");
        assert_eq!(
            the_technology_the_manifest_names(&on_the_machine.card_make),
            Ok(Technology::Optix),
            "the machine found no technology for {:?}",
            on_the_machine.card_make
        );
        // **The card's own name stays on the desktop**, where it is shown.
        assert!(!String::from_utf8(manifest.as_written())
            .unwrap()
            .contains("GTX 1650"));
    }

    fn a_layout() -> TheLayout {
        TheLayout::of(
            MachineRoot::chosen("/ephemeral/smoozy").unwrap(),
            WhereThePackageAreaSits::UnderTheRoot,
        )
    }

    fn a_build() -> TheBlenderToRenderBy {
        TheBlenderToRenderBy {
            version: "5.2.0".to_string(),
            build_path: a_blender().path,
        }
    }

    #[test]
    fn the_identities_are_issued_on_the_desktop_and_a_group_only_for_more_than_one_scene() {
        let identities = issue_the_identities().unwrap();
        assert_eq!(
            identities.submission_command.kind(),
            IdentityKind::Operation
        );
        assert_eq!(identities.job.kind(), IdentityKind::Job);
        assert_eq!(the_group_identity(1).unwrap(), None);
        assert_eq!(
            the_group_identity(3).unwrap().map(|g| g.kind()),
            Some(IdentityKind::Group)
        );
    }

    /// **S3a: the gate fires only on the user's own pick, only where the
    /// build's read-back says the engine is not there.**
    #[test]
    fn the_engine_gate_refuses_a_users_pick_the_build_does_not_carry_and_no_other() {
        let mut payload = an_untouched_payload();
        let read_back = TheReadBack {
            installed_listing: vec!["cycles".to_string()],
            switched_on_listing: vec!["cycles".to_string()],
            the_reporting_switch_on_raised: false,
        };
        let held = WhatTheDesktopHoldsAboutTheBuild::TheReadBack(read_back.clone());
        // No pick: the file's own engine stands and the gate does not fire.
        assert_eq!(
            the_engine_gate(&payload, &held, &a_build()),
            TheGate::Proceed
        );
        // A pick the build lists.
        payload.engine = Some(EngineEntry {
            pick: OfferedEngine::Cycles,
            identifier: None,
        });
        assert_eq!(
            the_engine_gate(&payload, &held, &a_build()),
            TheGate::Proceed
        );
        // A pick the build does not list: refused, naming the engine and
        // the build.
        payload.engine = Some(EngineEntry {
            pick: OfferedEngine::Eevee,
            identifier: None,
        });
        assert_eq!(
            the_engine_gate(&payload, &held, &a_build()),
            TheGate::Refused {
                engine: "eevee".to_string(),
                build: a_blender().path
            }
        );
        // Anchored on the reading of that build: the build registers it.
        payload.engine = Some(EngineEntry {
            pick: OfferedEngine::Eevee,
            identifier: Some(ExactName::as_the_reading_produced_it("BLENDER_EEVEE")),
        });
        assert_eq!(
            the_engine_gate(&payload, &held, &a_build()),
            TheGate::Proceed
        );
        // §9 item 27: nothing to judge against.
        payload.engine = Some(EngineEntry {
            pick: OfferedEngine::Eevee,
            identifier: None,
        });
        assert_eq!(
            the_engine_gate(
                &payload,
                &WhatTheDesktopHoldsAboutTheBuild::NothingButTheEntry,
                &a_build()
            ),
            TheGate::NothingToJudgeAgainstSoTheAttemptDecides
        );
    }

    #[test]
    fn the_unit_is_three_in_order_and_the_next_begins_only_on_a_read_outcome() {
        assert_eq!(
            THE_ORDERED_UNIT,
            [
                "the remap material placed",
                "the job's material published",
                "the job record written"
            ]
        );
        assert!(may_the_next_unit_begin(None));
        let mut keeper = AList::default();
        let mut writing =
            Writing::open(&mut keeper, "one scene-job's submission", &THE_ORDERED_UNIT).unwrap();
        let record = writing.read().unwrap().unwrap();
        assert!(!may_the_next_unit_begin(Some(&record)), "still in hand");
        for unit in THE_ORDERED_UNIT {
            writing.a_unit_ended(unit, UnitOutcome::Done).unwrap();
        }
        assert!(may_the_next_unit_begin(Some(
            &writing.read().unwrap().unwrap()
        )));
    }

    /// **S4b: the debt names exactly what the units record as published,
    /// and only where the record was never written.**
    #[test]
    fn a_unit_that_died_between_the_publication_and_the_record_leaves_a_named_debt() {
        let job = Identity::issue(IdentityKind::Job).unwrap();
        let layout = a_layout();
        let paths = AJobsPaths::composed(&layout, &job, Some("Kitchen")).unwrap();
        let mut keeper = AList::default();
        let mut writing =
            Writing::open(&mut keeper, "one scene-job's submission", &THE_ORDERED_UNIT).unwrap();
        writing
            .a_unit_ended(THE_ORDERED_UNIT[0], UnitOutcome::Done)
            .unwrap();
        writing
            .a_unit_ended(THE_ORDERED_UNIT[1], UnitOutcome::Done)
            .unwrap();
        let record = writing.read().unwrap().unwrap();
        let debt = the_cleanup_debt(&record, &paths, Some(&paths.job_output_folder), &layout, &job).unwrap();
        assert_eq!(debt.the_material_set_at, paths.job_record_directory);
        assert_eq!(
            debt.the_job_output_folder_where_empty.as_deref(),
            Some(paths.job_output_folder.as_str())
        );
        // **A folder chosen at render start was never made here**, so the debt
        // names none (`Job files and outputs` P21, M7 as narrowed).
        assert_eq!(
            the_cleanup_debt(&record, &paths, None, &layout, &job)
                .unwrap()
                .the_job_output_folder_where_empty,
            None
        );
        assert!(String::from_utf8_lossy(&debt.the_remap_material_at).ends_with(&job.as_text()));
        // The record written: a job, no debt.
        writing
            .a_unit_ended(THE_ORDERED_UNIT[2], UnitOutcome::Done)
            .unwrap();
        assert_eq!(
            the_cleanup_debt(&writing.read().unwrap().unwrap(), &paths, None, &layout, &job),
            None
        );
        // The material never published: nothing to remove.
        let mut keeper = AList::default();
        let mut writing =
            Writing::open(&mut keeper, "one scene-job's submission", &THE_ORDERED_UNIT).unwrap();
        writing
            .a_unit_ended(THE_ORDERED_UNIT[0], UnitOutcome::Failed)
            .unwrap();
        writing
            .the_rest_were_not_attempted(THE_ORDERED_UNIT[0])
            .unwrap();
        assert_eq!(
            the_cleanup_debt(&writing.read().unwrap().unwrap(), &paths, None, &layout, &job),
            None
        );
    }

    /// **S5a, and the field is on every manifest and never an absence.**
    #[test]
    fn the_reporting_field_is_the_per_job_answer_else_the_app_wide_value_and_reads_back() {
        let silent = the_reporting_field(Some(ReportingChoice::Nothing), ReportingChoice::Everything);
        assert_eq!(silent.0, serde_json::json!("Nothing"));
        assert_eq!(serde_json::from_value::<ReportingChoice>(silent.0).unwrap(), ReportingChoice::Nothing);
        let field = the_reporting_field(None, ReportingChoice::OnlyWhatNeedsAnAnswer);
        let read: ReportingChoice = serde_json::from_value(field.0.clone()).unwrap();
        assert_eq!(read, ReportingChoice::OnlyWhatNeedsAnAnswer);
        let field = the_reporting_field(
            Some(ReportingChoice::Everything),
            ReportingChoice::OnlyWhatNeedsAnAnswer,
        );
        let read: ReportingChoice = serde_json::from_value(field.0.clone()).unwrap();
        assert_eq!(read, ReportingChoice::Everything);
    }

    /// **`Telegram` V1a's fourth member rides the field with its minutes**,
    /// as the role worker reads it, and reads back through
    /// [`the_reporting_choice_on`] (2026-10-19, the Creator's want: a picture
    /// every so many minutes).
    #[test]
    fn a_picture_every_few_minutes_rides_the_field_with_its_minutes() {
        let field = the_reporting_field(
            Some(ReportingChoice::APictureEveryFewMinutes { minutes: 12 }),
            ReportingChoice::Everything,
        );
        assert_eq!(field.0, serde_json::json!({"APictureEveryFewMinutes": {"minutes": 12}}));
        let read: ReportingChoice = serde_json::from_value(field.0.clone()).unwrap();
        assert_eq!(read, ReportingChoice::APictureEveryFewMinutes { minutes: 12 });
    }

    #[test]
    fn the_manifest_is_composed_from_the_payload_and_what_this_document_adds() {
        let reading = a_reading();
        let scene = NamedAnswer {
            name: ExactName::as_the_reading_produced_it("Kitchen"),
            provenance: Provenance::ARecordedReading(reading.identity()),
        };
        let answers = SceneEntries {
            frame_range: Some(
                smoozy_render_settings::frames::parse_the_frame_field("1-3").unwrap(),
            ),
            ..Default::default()
        };
        let payload = compose(Composition {
            version_pick: &a_blender(),
            scene: Some(&scene),
            answers: &answers,
            standing: Some(&reading),
            stale: false,
        });
        let job = Identity::issue(IdentityKind::Job).unwrap();
        let group = Identity::issue(IdentityKind::Group).unwrap();
        let layout = a_layout();
        let paths = AJobsPaths::composed(&layout, &job, Some("Kitchen")).unwrap();
        let manifest = compose_the_manifest(ManifestInputs {
            job: &job,
            group: Some(&group),
            payload: &payload,
            layout: &layout,
            job_output_folder: smoozy_job_files::manifest::WhereTheJobOutputFolderIs::named_by(&paths),
            blend_path_on_the_machine: "/ephemeral/smoozy/assets/kitchen.blend".to_string(),
            file_identity_at_submission: four_values(),
            recorded_reading: TheRecordedReading::ThereIsNone,
            reporting: the_reporting_field(None, ReportingChoice::Everything),
            card_make: TheCardMakeAtSubmission::TheMakes(vec!["NVIDIA".to_string()]),
        });
        assert_eq!(manifest.job, job.as_text());
        assert_eq!(manifest.group, Some(group.as_text()));
        assert_eq!(manifest.scene.name, "Kitchen");
        assert_eq!(
            manifest.frame_range,
            smoozy_job_files::manifest::FrameRange::TheUsersParsedIntegers(vec![1, 2, 3])
        );
        assert_eq!(
            manifest.job_output_folder.the_path_it_names(),
            Some(paths.job_output_folder.as_str())
        );
        assert_eq!(
            the_reporting_choice_on(&manifest),
            Some(ReportingChoice::Everything)
        );
        assert_eq!(
            Payload::read_back(&manifest.settings_payload),
            Some(payload)
        );
        assert!(String::from_utf8_lossy(&manifest.as_written()).contains(&job.as_text()));
    }

    /// **`Rendering` A1, A1c and L10 end to end: Simplify entries a page
    /// loaded from a failed entry's kept setup reach the launch material, and
    /// a page that drew no offer sends none.** Store → `Render Settings` Y1's
    /// payload → this document's manifest → the bytes the machine reads → the
    /// render worker's own read-back → L10's composition. The 38's piece 2
    /// proved this before wiring the route (hole 4 of the map).
    #[test]
    fn simplify_loaded_from_a_failed_entry_reaches_the_launch_material_and_no_other_load_does() {
        use crate::material::testing::compose_with;
        use crate::material::{Entry, SimplifyValue};
        use smoozy_render_settings::file_half::FileKey;
        use smoozy_render_settings::store::{KeptSetup, KeptVersion, Store};
        let reading = a_reading();
        let scene = NamedAnswer {
            name: ExactName::as_the_reading_produced_it("Kitchen"),
            provenance: Provenance::AKeptSetup {
                the_old_job: "job-1".to_string(),
            },
        };
        let mut kept_entries = SceneEntries::default();
        kept_entries.simplify.use_simplify = Some(true);
        kept_entries.simplify.subdivision_render = Some(2);
        kept_entries.simplify.child_particles_render = Some(0.5);
        let kept = KeptSetup {
            scene: scene.clone(),
            entries: kept_entries,
            version: KeptVersion::ResolvedAutomatically,
        };
        let simplify_in_the_material = |offer_drawn: bool| {
            let key = FileKey {
                server: Identity::issue(IdentityKind::LocalServer).unwrap(),
                path: "/ephemeral/smoozy/assets/kitchen.blend".to_string(),
            };
            let mut store = Store::new();
            store.load_a_kept_setup(key.clone(), kept.clone(), offer_drawn);
            let answers = store.the_answers_composition_reads(&key, Some(&scene.name));
            let payload = compose(Composition {
                version_pick: &a_blender(),
                scene: Some(&scene),
                answers: &answers,
                standing: Some(&reading),
                stale: false,
            });
            let job = Identity::issue(IdentityKind::Job).unwrap();
            let layout = a_layout();
            let paths = AJobsPaths::composed(&layout, &job, Some("Kitchen")).unwrap();
            let manifest = compose_the_manifest(ManifestInputs {
                job: &job,
                group: None,
                payload: &payload,
                layout: &layout,
                job_output_folder: smoozy_job_files::manifest::WhereTheJobOutputFolderIs::named_by(&paths),
                blend_path_on_the_machine: key.path.clone(),
                file_identity_at_submission: four_values(),
                recorded_reading: TheRecordedReading::ThereIsNone,
                reporting: the_reporting_field(None, ReportingChoice::Everything),
                card_make: TheCardMakeAtSubmission::TheMakes(vec!["NVIDIA".to_string()]),
            });
            // **What the machine reads**: the bytes, read back as the render
            // worker reads them.
            let on_the_machine = SubmissionManifest::read_back(&manifest.as_written()).unwrap();
            let read = Payload::read_back(&on_the_machine.settings_payload).unwrap();
            let remap = crate::material::testing::a_remap(&job, false);
            let material = compose_with(&read, &remap, &job).unwrap();
            material
                .entries()
                .iter()
                .filter_map(|entry| match entry {
                    Entry::Simplify { property, value } => Some((*property, value.clone())),
                    _ => None,
                })
                .collect::<Vec<_>>()
        };
        let drawn = simplify_in_the_material(true);
        assert!(drawn.contains(&("scene.render.use_simplify", SimplifyValue::Switch(true))));
        assert!(drawn.contains(&(
            "scene.render.simplify_subdivision_render",
            SimplifyValue::WholeNumber(2)
        )));
        assert!(drawn.contains(&(
            "scene.render.simplify_child_particles_render",
            SimplifyValue::Fraction(0.5)
        )));
        assert!(simplify_in_the_material(false).is_empty());
    }

    #[test]
    fn a_job_with_no_scene_name_carries_an_empty_name_with_the_branch_stated() {
        let job = Identity::issue(IdentityKind::Job).unwrap();
        let layout = a_layout();
        let paths = AJobsPaths::composed(&layout, &job, None).unwrap();
        let manifest = compose_the_manifest(ManifestInputs {
            job: &job,
            group: None,
            payload: &an_untouched_payload(),
            layout: &layout,
            job_output_folder: smoozy_job_files::manifest::WhereTheJobOutputFolderIs::named_by(&paths),
            blend_path_on_the_machine: "/ephemeral/smoozy/assets/kitchen.blend".to_string(),
            file_identity_at_submission: four_values(),
            recorded_reading: TheRecordedReading::ThereIsNone,
            reporting: the_reporting_field(None, ReportingChoice::Everything),
            card_make: TheCardMakeAtSubmission::TheMakes(vec!["NVIDIA".to_string()]),
        });
        assert_eq!(manifest.scene.name, "");
        assert!(matches!(
            manifest.scene.anchor,
            smoozy_job_files::manifest::AnchorState::ToAnchor { .. }
        ));
    }

    #[test]
    fn the_all_scenes_act_carries_one_read_name_per_scene_job_and_no_repeat() {
        let reading = a_reading();
        let scenes = smoozy_render_settings::payload::the_all_scenes_act(Some(&reading)).unwrap();
        assert!(every_scene_job_of_the_group_carries_its_own_read_name(
            &scenes
        ));
        let mut repeated = scenes.clone();
        repeated.push(scenes[0].clone());
        assert!(!every_scene_job_of_the_group_carries_its_own_read_name(
            &repeated
        ));
        assert!(!every_scene_job_of_the_group_carries_its_own_read_name(&[]));
    }

    /// **S13: two jobs only where the range holds more than one frame, and
    /// the held one is the whole range, its first frame included** (decision
    /// C, 2026-10-02).
    #[test]
    fn the_first_frame_check_splits_a_range_and_leaves_a_one_frame_range_whole() {
        assert_eq!(
            the_first_frame_split(&[40, 22, 23]),
            Some(TheFirstFrameSplit::TwoJobs {
                the_first_frame: vec![40],
                the_whole_range_held: vec![40, 22, 23]
            })
        );
        assert_eq!(
            the_first_frame_split(&[7]),
            Some(TheFirstFrameSplit::OneJobWithNoHold { frames: vec![7] })
        );
        assert_eq!(the_first_frame_split(&[]), None);
        let first = Identity::issue(IdentityKind::Job).unwrap();
        let second = Identity::issue(IdentityKind::Job).unwrap();
        let hold = the_hold_instruction(&second, &first);
        assert_eq!(hold.origin, "submission");
        assert_eq!(hold.subject, second);
        assert_eq!(hold.dependency, Some(first));
        const _: () = assert!(S14_TO_S17_ARE_CITATIONS_OF_RENTED_SERVER_RUNTIME_SECTION_8);
    }

    /// **S18a: the hold for the driver names no dependency, and a job the
    /// check already holds carries that hold and no second one.** Held-job
    /// piece A.
    #[test]
    fn the_driver_hold_names_no_dependency_and_never_rides_beside_the_checks() {
        let first = Identity::issue(IdentityKind::Job).unwrap();
        let second = Identity::issue(IdentityKind::Job).unwrap();
        let held = the_driver_hold_instruction(&first);
        assert_eq!(held.origin, "submission");
        assert_eq!(held.subject, first);
        assert_eq!(held.reason, THE_DRIVER);
        assert_eq!(held.dependency, None);
        assert_ne!(THE_DRIVER, the_hold_instruction(&second, &first).reason);

        // The pair: the first held for the driver, the second keeps S13's.
        assert_eq!(the_hold_this_job_carries(&first, None, true), Some(held));
        assert_eq!(
            the_hold_this_job_carries(&second, Some(&first), true),
            Some(the_hold_instruction(&second, &first))
        );
        assert_eq!(the_hold_this_job_carries(&first, None, false), None);
    }

    #[test]
    fn a_refusal_fails_one_scene_job_whole_and_touches_no_other() {
        let mut keeper = AList::default();
        let mut refused =
            Writing::open(&mut keeper, "one scene-job's submission", &THE_ORDERED_UNIT).unwrap();
        refused
            .a_unit_ended(THE_ORDERED_UNIT[0], UnitOutcome::Failed)
            .unwrap();
        refused
            .the_rest_were_not_attempted(THE_ORDERED_UNIT[0])
            .unwrap();
        let refused = refused.read().unwrap().unwrap();
        let mut other_keeper = AList::default();
        let mut other = Writing::open(
            &mut other_keeper,
            "one scene-job's submission",
            &THE_ORDERED_UNIT,
        )
        .unwrap();
        for unit in THE_ORDERED_UNIT {
            other.a_unit_ended(unit, UnitOutcome::Done).unwrap();
        }
        let other = other.read().unwrap().unwrap();
        assert!(a_refusal_fails_that_scene_job_whole_and_touches_no_other(
            &[refused, other],
            0
        ));
        let _ = keeper.read(&Identity::issue(IdentityKind::Operation).unwrap());
    }

    #[test]
    fn every_scene_group_is_whole_on_one_machine_by_construction() {
        let machine = Identity::issue(IdentityKind::LocalServer).unwrap();
        assert!(every_scene_group_is_whole_on_one_machine(&[
            machine.clone(),
            machine.clone(),
            machine.clone()
        ]));
        let other = Identity::issue(IdentityKind::LocalServer).unwrap();
        assert!(!every_scene_group_is_whole_on_one_machine(&[
            machine, other
        ]));
    }
}

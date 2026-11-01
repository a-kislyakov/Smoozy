//! The attempt pipeline — §2 P1, P1a, P2, P3: **the order as data, and the
//! two judgements inside it that are this half's** — P1a's name validation
//! and P1 step 6's rule for which attempt record's list is read.
//!
//! **P1: every render attempt runs one pipeline, in one order, and the
//! order is the contract's.** The nine steps are each a clause another
//! document already fixes, sequenced once here; performing them — admitting
//! the operation, reading the remap material, checking the four values,
//! taking the reading, placing a hold, walking the folder, launching,
//! copying the streams, reaping — is the render worker's and is the acting
//! half's. **What is here is the sequence itself**, so a worker walks
//! [`THE_NINE_STEPS`] rather than a list kept in its own head, and the two
//! steps whose whole content is a judgement over values in hand.
//!
//! **P1a: one step sits between step 4 and step 7, and the numbering is not
//! renumbered for it.** Every name-shaped entry of the payload — the scene,
//! a pinned view layer, the engine identifier — is checked against the
//! reading in force for this attempt, and **a name the reading in force
//! does not carry fails the attempt before the render instruction, naming
//! what was not found** (`Render Settings` Y13–Y14). Without it every
//! name-shaped entry reaches the launch unvalidated, which is a paid render
//! of the wrong scene, the wrong layer set or the wrong engine, at success.
//!
//! **Markers used in this file that belong to another document, each with
//! the document it belongs to.** `Rented-server runtime` M1, U6,
//! U7, W12, M6. `Job files and outputs` C1, C6, C7, C9, C12, C13, C17,
//! C18, C18a, C20, C27, H1, H4, H5, R1, R2, R3, R7, R10, R12, R13, M8.
//! `Render Settings` Y12, Y13, Y14, E2.
//!
//! **`Rented-server runtime` W3 and A4 are not listed above and are written
//! out at each site**, because this plan defines a W3 and an A4 of its own.

use smoozy_job_files_machine::attempts::AttemptRecord;
use smoozy_render_settings_machine::engine::anchor_the_engine;
use smoozy_render_settings_machine::file_half::RecordedReading;
use smoozy_render_settings_machine::names::{anchor_at_the_attempt, ExactName, NotFound, WhatIsAnchored};
use smoozy_render_settings_machine::payload::Payload;

/// One step of P1, with the clauses it performs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Step {
    pub number: u8,
    pub what: &'static str,
    pub clauses: &'static str,
}

/// **P1's nine steps, in the contract's order.**
pub const THE_NINE_STEPS: [Step; 9] = [
    Step {
        number: 1,
        what: "admission and start — the supervisor admits the render operation at the one admission point under §7's classes, writes the operation record, creates the two sinks, starts the render worker",
        clauses: "`Rented-server runtime` W3, `Rented-server runtime` M1, `Rented-server runtime` A4",
    },
    Step {
        number: 2,
        what: "the remap material is read; where it cannot be read the attempt fails before anything further, naming the material",
        clauses: "L30",
    },
    Step {
        number: 3,
        what: "the fresh four-value check against the manifest's recorded identity; a match with a map carried makes that map the map in force and nothing is recomputed",
        clauses: "`Job files and outputs` R10, R12, R7; `Rented-server runtime` U7",
    },
    Step {
        number: 4,
        what: "the reading, where one is owed — one headless invocation, all payloads out of one read, the map composed, both written to the job and attempt records; the version confirmation rides it",
        clauses: "`Job files and outputs` R1, R2, R13; L14",
    },
    Step {
        number: 5,
        what: "the three hold conditions evaluated off the recorded reading under the two triggers; where one holds the attempt ends *held for an answer* and no render was started",
        clauses: "`Job files and outputs` H1–H5; `Rented-server runtime` U6; K14",
    },
    Step {
        number: 6,
        what: "the reckoning — the walk against the inventory for the map in force, the zero-byte rule, the last frame asked for re-rendered anyway, the whole range where no previous record can be read, no reckoning where a destination is video-shaped; the previous list read from the latest attempt record that carries one",
        clauses: "`Job files and outputs` §10 C6, C7, C12, C13, C17, C18, C18a, C20",
    },
    Step {
        number: 7,
        what: "composition and launch — the line and the material of §1 from the manifest, the map in force, the remap material and the reckoning's list; the attempt record written durably before the render starts; the render process recorded with its start-time pair",
        clauses: "L8, L10, L5; `Job files and outputs` M8",
    },
    Step {
        number: 8,
        what: "the copy loop — both streams held and copied, bytes counted, the marker observed, the reader hosted as a tee, live readings reported at a bounded interval, the device line observed in passing",
        clauses: "K13",
    },
    Step {
        number: 9,
        what: "the ending step — the exit status reaped verbatim, the one walk and the completeness statement, the report re-attempted until accepted; the supervisor writes the recorded outcome",
        clauses: "`Job files and outputs` C7, C27; `Rented-server runtime` M6, W12; K14",
    },
];

/// **P1a sits between step 4 and step 7**, after the reading it validates
/// against and before the composition that uses the names.
pub const P1A_SITS_AFTER_STEP_4_AND_BEFORE_STEP_7: (u8, u8) = (4, 7);

/// The names the launch is composed from, every one the reading in force's
/// own bytes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidatedNames {
    pub scene: Option<ExactName>,
    pub engine: Option<ExactName>,
    pub pinned_layers: Vec<ExactName>,
}

/// **P1a, performed: every name-shaped entry of the payload checked against
/// the reading in force, the bytes sent being that reading's own.** The
/// first name not found fails the attempt, naming what was not found.
pub fn validate_the_names(
    payload: &Payload,
    reading_in_force: &RecordedReading,
) -> Result<ValidatedNames, NotFound> {
    let scene = match &payload.scene {
        Some(entry) => Some(anchor_at_the_attempt(
            &entry.name,
            &WhatIsAnchored::AScene,
            reading_in_force,
        )?),
        None => None,
    };
    let engine = match &payload.engine {
        Some(entry) => Some(anchor_the_engine(entry.pick, reading_in_force)?),
        None => None,
    };
    // **`Render Settings` C8: a pin's layer is the job's scene's**, so it is
    // looked up in that scene's own list and never across the file. A job
    // carrying no scene name carries no pins, by `Render Settings` J3.
    let mut pinned_layers = Vec::with_capacity(payload.view_layer_pins.len());
    for pin in &payload.view_layer_pins {
        let Some(the_jobs_scene) = payload.scene.as_ref().map(|entry| entry.name.clone()) else {
            break;
        };
        pinned_layers.push(anchor_at_the_attempt(
            &pin.layer,
            &WhatIsAnchored::AViewLayerOf { the_jobs_scene },
            reading_in_force,
        )?);
    }
    Ok(ValidatedNames {
        scene,
        engine,
        pinned_layers,
    })
}

/// **P1 step 6's rule for C18's list, stated by when the record was written
/// rather than by how the attempt ended** (`Job files and outputs` C18a):
/// an attempt that ended at a hold wrote its record with no frame list, so
/// the reckoning walks past it; an attempt that failed before step 7 wrote
/// no record at all; an attempt that failed at the read-back or a raised
/// pair had already written its list at step 7, and that is the record
/// read. **So the previous list is the latest record that carries one.**
pub fn the_previous_attempts_frame_list(records_oldest_first: &[AttemptRecord]) -> Option<&[i64]> {
    records_oldest_first
        .iter()
        .rev()
        .map(|record| record.frame_list_in_the_order_it_was_written.as_slice())
        .find(|list| !list.is_empty())
}

/// **P2: nothing in the pipeline needs a connection and nothing in it needs
/// Smoozy** — every input is durable on the machine (L29), every actor is
/// the supervisor or its worker, and every telling reaches a person through
/// the journal.
pub const NOTHING_IN_THE_PIPELINE_NEEDS_A_CONNECTION_OR_SMOOZY: bool = true;

/// **P3: where the supervisor is not answering at the moment a hold would
/// be placed, the attempt places no hold at all** — it ends as an
/// interrupted attempt and reconciliation takes it. Nothing invents a
/// second waiting mechanism and nothing writes a hold record without a
/// supervisor-issued identity.
pub const NO_HOLD_IS_PLACED_WITHOUT_A_SUPERVISOR_ISSUED_IDENTITY: bool = true;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::material::tests::{a_reading, an_untouched_payload, four_values};
    use smoozy_job_files_machine::attempts::TheMapInForce;
    use smoozy_render_settings_machine::engine::{EngineEntry, OfferedEngine};
    use smoozy_render_settings_machine::names::AnchorState;
    use smoozy_render_settings_machine::payload::SceneEntry;
    use smoozy_render_settings_machine::view_layers::PinEntry;

    #[test]
    fn the_nine_steps_are_numbered_one_to_nine_in_order_and_p1a_sits_between_four_and_seven() {
        for (at, step) in THE_NINE_STEPS.iter().enumerate() {
            assert_eq!(step.number as usize, at + 1);
            assert!(!step.what.is_empty());
            assert!(!step.clauses.is_empty());
        }
        assert_eq!(P1A_SITS_AFTER_STEP_4_AND_BEFORE_STEP_7, (4, 7));
        const _: () = assert!(NOTHING_IN_THE_PIPELINE_NEEDS_A_CONNECTION_OR_SMOOZY);
        const _: () = assert!(NO_HOLD_IS_PLACED_WITHOUT_A_SUPERVISOR_ISSUED_IDENTITY);
    }

    fn to_anchor(_name: &str) -> AnchorState {
        AnchorState::ToAnchor {
            provenance: smoozy_render_settings_machine::names::Provenance::AKeptSetup {
                the_old_job: "job-0".to_string(),
            },
        }
    }

    #[test]
    fn every_name_the_reading_carries_validates_and_the_bytes_sent_are_the_readings_own() {
        let reading = a_reading();
        let mut payload = an_untouched_payload();
        payload.scene = Some(SceneEntry {
            name: ExactName::as_the_reading_produced_it("Kitchen"),
            anchor: to_anchor("Kitchen"),
        });
        payload.engine = Some(EngineEntry {
            pick: OfferedEngine::Eevee,
            identifier: None,
        });
        payload.view_layer_pins.push(PinEntry {
            layer: ExactName::as_the_reading_produced_it("ViewLayer"),
            anchor: to_anchor("ViewLayer"),
            on: true,
        });
        let validated = validate_the_names(&payload, &reading).unwrap();
        assert_eq!(
            validated.scene,
            Some(ExactName::as_the_reading_produced_it("Kitchen"))
        );
        assert_eq!(
            validated.engine,
            Some(ExactName::as_the_reading_produced_it("BLENDER_EEVEE"))
        );
        assert_eq!(
            validated.pinned_layers,
            vec![ExactName::as_the_reading_produced_it("ViewLayer")]
        );
    }

    /// **A name the reading in force does not carry fails the attempt,
    /// naming it** — a paid render of the wrong scene at success is what
    /// this stands against.
    #[test]
    fn a_scene_name_the_reading_does_not_carry_fails_the_attempt_naming_it() {
        let reading = a_reading();
        let mut payload = an_untouched_payload();
        payload.scene = Some(SceneEntry {
            name: ExactName::as_the_reading_produced_it("Attic"),
            anchor: to_anchor("Attic"),
        });
        let refused = validate_the_names(&payload, &reading).unwrap_err();
        assert_eq!(refused.what, WhatIsAnchored::AScene);
        assert_eq!(refused.name, ExactName::as_the_reading_produced_it("Attic"));
        // A case near-miss is not a match anywhere.
        payload.scene = Some(SceneEntry {
            name: ExactName::as_the_reading_produced_it("kitchen"),
            anchor: to_anchor("kitchen"),
        });
        assert!(validate_the_names(&payload, &reading).is_err());
    }

    #[test]
    fn an_engine_pick_the_build_does_not_register_fails_the_attempt() {
        let mut reading = a_reading();
        reading.settings.offered_engine_identifiers.cycles = None;
        let mut payload = an_untouched_payload();
        payload.engine = Some(EngineEntry {
            pick: OfferedEngine::Cycles,
            identifier: None,
        });
        let refused = validate_the_names(&payload, &reading).unwrap_err();
        assert_eq!(refused.what, WhatIsAnchored::AnEngine);
    }

    fn a_record(number: u32, list: Vec<i64>) -> AttemptRecord {
        AttemptRecord {
            number,
            frame_list_in_the_order_it_was_written: list,
            map_in_force: TheMapInForce::NoneWasComposedForThisAttempt,
            file_identity_at_this_attempt: four_values(),
            the_file_changed_before_this_attempt: false,
            a_frame_an_earlier_attempt_wrote_was_kept: false,
        }
    }

    /// **C18a: the latest record that carries a list**, walking past a
    /// hold's record with none.
    #[test]
    fn the_previous_list_is_the_latest_record_that_carries_one() {
        let records = vec![a_record(1, vec![1, 2, 3]), a_record(2, Vec::new())];
        assert_eq!(
            the_previous_attempts_frame_list(&records),
            Some(&[1, 2, 3][..])
        );
        assert_eq!(the_previous_attempts_frame_list(&[]), None);
        assert_eq!(
            the_previous_attempts_frame_list(&[a_record(1, Vec::new())]),
            None
        );
        let two = vec![a_record(1, vec![1, 2]), a_record(2, vec![2])];
        assert_eq!(the_previous_attempts_frame_list(&two), Some(&[2][..]));
    }
}

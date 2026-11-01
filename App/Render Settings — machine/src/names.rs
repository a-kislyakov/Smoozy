//! A name-shaped answer, what it is held against, and anchoring at the
//! attempt — `Render Settings` §2.2 J7–J9 and §9.4 Y12–Y14.
//!
//! **Constraint:** Row 145. A ticked scene and a pinned view layer are stated
//! by name; the spec states outright that nothing checks the name still
//! exists; the cost of a wrong name is settled and silent — *a scene name no
//! scene bears is named and then ignored and the run reports success*
//! (*Settings 12*); and `Rendering` L15 refuses to send anything but the exact
//! bytes a reading of the file produced.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Rendering` L15, L19, K14, P1a. `Job files and
//! outputs` R13. `Queue & History` N10.

use serde::{Deserialize, Serialize};

use crate::file_half::{ReadingIdentity, RecordedReading};

/// **A name exactly as a reading of the file produced it — never typed, never
/// normalised, never matched case-insensitively, never re-derived** (J7,
/// `Rendering` L15).
///
/// The one constructor takes what a reading's own document carried, and the
/// type offers no way to change a byte of it. Comparison is on the bytes and
/// on nothing looser.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ExactName(String);

impl ExactName {
    /// The bytes a reading's document carried for the name.
    pub fn as_the_reading_produced_it(text: &str) -> ExactName {
        ExactName(text.to_string())
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }

    /// The text, for a manifest field or a diary line. **Not for comparison**:
    /// two names are the same when [`ExactName::as_bytes`] agree.
    pub fn as_text(&self) -> &str {
        &self.0
    }
}

/// **J7: the naming act is one of exactly two**, and the provenance records
/// which.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Provenance {
    /// The user ticked or pinned a thing drawn from a recorded reading; the
    /// bytes are that reading's own and this is that reading's identity.
    ARecordedReading(ReadingIdentity),
    /// The answer arrived from a history entry's kept setup, where the bytes
    /// are the old job's manifest's — themselves a reading's bytes by
    /// `Rendering` L15's provenance.
    AKeptSetup {
        /// The old job's identity, as text.
        the_old_job: String,
    },
}

/// **J7: a name-shaped answer is held against the name's exact bytes plus a
/// recorded provenance.** The page offers no way to type a scene or view-layer
/// name, so there is no third naming act: every name in the store is, by
/// provenance, bytes some reading of some file produced.
///
/// **J8: the answer outlives the reading that produced it** — that is the
/// spec's grant — so the held object is this pair and never the read's own
/// result; the standing reading is only ever what the answer is *anchored* to
/// at use.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NamedAnswer {
    pub name: ExactName,
    pub provenance: Provenance,
}

/// **Y12: every name-shaped payload entry carries an anchor state.**
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AnchorState {
    /// The bytes are a standing reading's own, and that reading's identity is
    /// recorded beside them.
    Anchored {
        the_reading_it_came_from: ReadingIdentity,
    },
    /// Carrying J7's provenance; anchored at the attempt.
    ToAnchor { provenance: Provenance },
}

/// **Y12: anchoring at submission happens exactly when an un-stale reading of
/// the file stands; everything else anchors at the attempt.**
///
/// **The kind is the caller's to state**, so each call site asks about its own
/// kind: a reading carrying a *scene* of that name is no reason to anchor a
/// *view layer*, and answering on any hit is how a wrong kind used to anchor.
///
/// `stale` is Q13's own test, taken by the caller against the four values now
/// standing and the version in force (`Job files and outputs` R13's rule).
pub fn anchor_at_submission(
    answer: &NamedAnswer,
    what: &WhatIsAnchored,
    standing: Option<&RecordedReading>,
    stale: bool,
) -> AnchorState {
    match standing {
        Some(reading) if !stale && reading.carries(&answer.name, what) => AnchorState::Anchored {
            the_reading_it_came_from: reading.identity(),
        },
        _ => AnchorState::ToAnchor {
            provenance: answer.provenance.clone(),
        },
    }
}

/// Which kind of name is being anchored — one of the three Y14 names, and the
/// one it names in a failure.
///
/// **A view layer carries the job's scene, and that is C8.** A pin's layer is
/// the job's scene's, per J3's per-scene key, and no pair walks the file's
/// scene list — so the kind cannot be stated without saying which scene, and
/// a name living in some other scene of the file is not a match.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhatIsAnchored {
    AScene,
    AViewLayerOf { the_jobs_scene: ExactName },
    AnEngine,
}

/// **Y14: a name the reading in force does not carry fails the attempt before
/// the render instruction, naming what was not found.** This adds one reason
/// to `Rendering` K14's failed-before-the-render-instruction family, and the
/// validation is that document's P1a step.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NotFound {
    pub what: WhatIsAnchored,
    pub name: ExactName,
}

/// **Y13: at every attempt, the name sent is validated against the reading in
/// force for that attempt, and the bytes sent are that reading's own.**
///
/// For the ordinary pre-loaded case this is a byte-compare in a document
/// already in hand; for a to-anchor name, and for any attempt where a fresh
/// reading ran, the lookup is against the fresh document. The bytes returned
/// are taken from the reading's own list rather than echoed from the answer,
/// which is what *the bytes sent are that reading's own* means.
///
/// **J9 is what this buys**: the one silent failure the research records for a
/// wrong scene name — named, ignored, wrong scene rendered at success — becomes
/// a failed attempt with the reason recorded, while every surviving-answer
/// grant the spec makes stands: no prompt, no gate, no check at the moment of
/// answering or of submitting.
pub fn anchor_at_the_attempt(
    name: &ExactName,
    what: &WhatIsAnchored,
    reading_in_force: &RecordedReading,
) -> Result<ExactName, NotFound> {
    let found = match what {
        WhatIsAnchored::AScene => reading_in_force
            .settings
            .scenes
            .iter()
            .map(|scene| &scene.name)
            .find(|held| held.as_bytes() == name.as_bytes()),
        // **C8: the job's scene's own view-layer list, and never the file's.**
        // Where the reading does not carry that scene, the layer is not found
        // either — which is Y14's own shape, and is what the attempt fails on.
        WhatIsAnchored::AViewLayerOf { the_jobs_scene } => reading_in_force
            .settings
            .scenes
            .iter()
            .find(|scene| scene.name.as_bytes() == the_jobs_scene.as_bytes())
            .into_iter()
            .flat_map(|scene| scene.view_layers.iter())
            .map(|layer| &layer.name)
            .find(|held| held.as_bytes() == name.as_bytes()),
        WhatIsAnchored::AnEngine => reading_in_force
            .settings
            .offered_engine_identifiers
            .both()
            .into_iter()
            .find(|held| held.as_bytes() == name.as_bytes()),
    };
    found.cloned().ok_or(NotFound {
        what: what.clone(),
        name: name.clone(),
    })
}

impl RecordedReading {
    /// Whether this reading carries the name **as the kind being asked
    /// about** — the test [`anchor_at_submission`] anchors on.
    ///
    /// The kind is the caller's, because the three lists are three different
    /// questions: a reading carrying a scene called *Detail* says nothing
    /// about whether it carries a view layer called *Detail*, and anchoring a
    /// pin on the scene's hit would put a reading behind an answer that
    /// reading cannot vouch for.
    pub fn carries(&self, name: &ExactName, what: &WhatIsAnchored) -> bool {
        anchor_at_the_attempt(name, what, self).is_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::file_half::tests::a_recorded_reading;

    fn kitchen() -> NamedAnswer {
        NamedAnswer {
            name: ExactName::as_the_reading_produced_it("Kitchen"),
            provenance: Provenance::AKeptSetup {
                the_old_job: "job-1".to_string(),
            },
        }
    }

    #[test]
    fn a_name_is_compared_on_its_bytes_and_never_case_folded() {
        let read = ExactName::as_the_reading_produced_it("Kitchen");
        let folded = ExactName::as_the_reading_produced_it("kitchen");
        assert_ne!(read, folded);
        assert_ne!(read.as_bytes(), folded.as_bytes());
    }

    fn a_layer_of(scene: &str) -> WhatIsAnchored {
        WhatIsAnchored::AViewLayerOf {
            the_jobs_scene: ExactName::as_the_reading_produced_it(scene),
        }
    }

    #[test]
    fn an_answer_anchors_at_submission_only_against_an_unstale_reading_that_carries_it() {
        let reading = a_recorded_reading();
        let anchored =
            anchor_at_submission(&kitchen(), &WhatIsAnchored::AScene, Some(&reading), false);
        assert!(matches!(anchored, AnchorState::Anchored { .. }));

        // Stale: to-anchor, carrying the provenance.
        let to_anchor =
            anchor_at_submission(&kitchen(), &WhatIsAnchored::AScene, Some(&reading), true);
        assert_eq!(
            to_anchor,
            AnchorState::ToAnchor {
                provenance: kitchen().provenance
            }
        );

        // No reading: to-anchor.
        assert!(matches!(
            anchor_at_submission(&kitchen(), &WhatIsAnchored::AScene, None, false),
            AnchorState::ToAnchor { .. }
        ));

        // A reading that stands and does not carry the name: to-anchor, never
        // anchored to a reading that cannot vouch for it.
        let elsewhere = NamedAnswer {
            name: ExactName::as_the_reading_produced_it("Attic"),
            provenance: kitchen().provenance,
        };
        assert!(matches!(
            anchor_at_submission(&elsewhere, &WhatIsAnchored::AScene, Some(&reading), false),
            AnchorState::ToAnchor { .. }
        ));
    }

    #[test]
    fn a_name_anchors_at_submission_only_as_the_kind_it_is_being_asked_about() {
        // The reading carries a scene called `Kitchen`. Asked as a view layer
        // of that scene, it does not carry it — and the pin must not anchor on
        // the scene's hit.
        let reading = a_recorded_reading();
        assert!(matches!(
            anchor_at_submission(&kitchen(), &a_layer_of("Kitchen"), Some(&reading), false),
            AnchorState::ToAnchor { .. }
        ));
        assert!(reading.carries(&kitchen().name, &WhatIsAnchored::AScene));
        assert!(!reading.carries(&kitchen().name, &a_layer_of("Kitchen")));
    }

    #[test]
    fn anchoring_at_the_attempt_returns_the_readings_own_bytes_or_names_what_was_not_found() {
        let reading = a_recorded_reading();
        let scene = ExactName::as_the_reading_produced_it("Kitchen");
        assert_eq!(
            anchor_at_the_attempt(&scene, &WhatIsAnchored::AScene, &reading),
            Ok(scene.clone())
        );
        let layer = ExactName::as_the_reading_produced_it("ViewLayer");
        assert!(anchor_at_the_attempt(&layer, &a_layer_of("Kitchen"), &reading).is_ok());
        let engine = ExactName::as_the_reading_produced_it("CYCLES");
        assert!(anchor_at_the_attempt(&engine, &WhatIsAnchored::AnEngine, &reading).is_ok());

        let missing = ExactName::as_the_reading_produced_it("Attic");
        assert_eq!(
            anchor_at_the_attempt(&missing, &WhatIsAnchored::AScene, &reading),
            Err(NotFound {
                what: WhatIsAnchored::AScene,
                name: missing.clone(),
            })
        );
        // A scene's name is not a view layer's, and a case near-miss is not a
        // match anywhere.
        assert!(anchor_at_the_attempt(&scene, &a_layer_of("Kitchen"), &reading).is_err());
        let near = ExactName::as_the_reading_produced_it("kitchen");
        assert!(anchor_at_the_attempt(&near, &WhatIsAnchored::AScene, &reading).is_err());
    }

    #[test]
    fn a_pinned_layer_is_looked_up_in_the_jobs_scene_alone_and_never_the_whole_file() {
        // C8. The reading carries two scenes; a layer living only in the other
        // one is not this job's, and the attempt fails naming it rather than
        // pinning a layer of a scene nobody is rendering.
        let mut reading = a_recorded_reading();
        reading.settings.scenes[1].view_layers =
            vec![crate::file_half::tests::a_layer("Attic", true)];
        let other_scenes_layer = ExactName::as_the_reading_produced_it("Attic");
        assert!(
            anchor_at_the_attempt(&other_scenes_layer, &a_layer_of("Exterior"), &reading).is_ok()
        );
        assert_eq!(
            anchor_at_the_attempt(&other_scenes_layer, &a_layer_of("Kitchen"), &reading),
            Err(NotFound {
                what: a_layer_of("Kitchen"),
                name: other_scenes_layer.clone(),
            })
        );
        // And where the reading does not carry the job's scene at all, the
        // layer is not found either.
        let layer = ExactName::as_the_reading_produced_it("ViewLayer");
        assert!(anchor_at_the_attempt(&layer, &a_layer_of("NoSuchScene"), &reading).is_err());
    }
}

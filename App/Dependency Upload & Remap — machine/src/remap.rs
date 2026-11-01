//! The remap material and its pairs — `Dependency Upload & Remap` §6.1 to
//! §6.3's values and the pure judgements over them: a pair and its match text,
//! whether a value matches, what the marker says for a pair, what each kind
//! takes, and the material's document with where it is kept.
//!
//! **What composes the pairs is not here**: [`compose_pair`] makes one pair,
//! and walking the record's located answers into pairs stays in
//! `smoozy-dependencies`' `remap`, because it takes the record and the landing
//! rule. That module re-exports everything here at the paths it always had.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Job files and outputs` D7, D8.

use serde::{Deserialize, Serialize};
use smoozy_job_files_machine::layout::TheLayout;
use smoozy_job_files_machine::manifest::CarriedAndNotInterpretedHere;
use smoozy_render_settings_machine::file_half::{version_text, BlenderIdentity};
use smoozy_state::identities::{Identity, IdentityKind};

use crate::bytes::{MachineLocation, RecordedLocation};
use crate::check::Kind;
use crate::landing::{the_asset_area, REMAPS};

/// **M4a: a pair's match text — the value with every backslash (byte `0x5C`)
/// replaced by a forward slash (byte `0x2F`). That is the whole of it.**
/// Nothing else is changed and everything else is compared verbatim: the
/// leading `//` shorthand stays, a doubled separator stays doubled, a `..`
/// step stays, letter case stays as recorded, a trailing space stays, and no
/// Unicode form is normalised, no path is resolved or tidied, no folding of
/// any kind is applied.
///
/// **Why the separator has to go** (Fact 128, observed in a test): opening a
/// `.blend` re-spells the separators in a recorded place toward the opening
/// machine's own convention and a plain save writes that in — so a user
/// checking a file on Windows and a rented Linux machine rendering it read the
/// same recorded place differently, and composed on the exact bytes every pair
/// of such a file would miss at once. **What it costs, said rather than
/// smoothed**: a backslash is a legal character in a file name on Mac and on
/// Linux, so this rule cannot tell such a name from a Windows separator —
/// Blender's own re-spelling already destroys that distinction.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MatchText(Vec<u8>);

impl MatchText {
    pub fn of(value: &[u8]) -> MatchText {
        MatchText(
            value
                .iter()
                .map(|byte| if *byte == 0x5C { 0x2F } else { *byte })
                .collect(),
        )
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

/// **M4: the new location a pair names** — the mirror of the resolved asset
/// (5.2), or the **fixed copy's** location where the reference is a linked
/// `.blend` (6.5), or the machine location of a file already up that the
/// reference was pointed at (Q9).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NewLocation {
    AMirror(MachineLocation),
    AFixedCopy(MachineLocation),
    ACopyAlreadyUp(MachineLocation),
}

/// **M4: one pair of the remap material** — the data-block kind and the
/// setting that holds the location; the recorded location's **expected
/// bytes** exactly as the check read them, **and the pair's match text
/// derived from those bytes by M4a's one rule**; and the new location.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Pair {
    pub kind: Kind,
    /// The setting is a property path rather than an instance (M8).
    pub setting: String,
    pub expected: RecordedLocation,
    pub match_text: MatchText,
    pub new_location: MachineLocation,
}

/// Why a pair is not composed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotComposed {
    /// **M4: a library pair never names a copy that has not been fixed.**
    /// Where the reference is a linked `.blend`, the new location is that
    /// file's fixed copy whichever route the file arrived by.
    ALibraryPairMustNameAFixedCopy,
    /// **M4: the fixed-copy alternative is for a linked `.blend` and never
    /// for an ordinary asset.**
    OnlyALibraryPairNamesAFixedCopy,
}

/// **M4 and M4a: a pair, composed once at composition from the expected bytes
/// the check read.** Pairs cover the top file's direct references only:
/// everything below the top was fixed where it sits.
pub fn compose_pair(
    kind: Kind,
    setting: &str,
    expected: &RecordedLocation,
    new: NewLocation,
) -> Result<Pair, NotComposed> {
    let new_location = match (kind, new) {
        (Kind::Library, NewLocation::AFixedCopy(fixed)) => fixed,
        (Kind::Library, _) => return Err(NotComposed::ALibraryPairMustNameAFixedCopy),
        (_, NewLocation::AFixedCopy(_)) => {
            return Err(NotComposed::OnlyALibraryPairNamesAFixedCopy)
        }
        (_, NewLocation::AMirror(at)) | (_, NewLocation::ACopyAlreadyUp(at)) => at,
    };
    Ok(Pair {
        kind,
        setting: setting.to_string(),
        expected: expected.clone(),
        match_text: MatchText::of(expected.as_bytes()),
        new_location,
    })
}

/// **M8: whether a data-block's current value matches the pair** — the value
/// put through M4a's separator rule, compared against the pair's match text
/// **byte for byte**; equal is a match and anything else is not. Applied to
/// every data-block of the pair's kind whose named setting matches, not to
/// one.
pub fn a_current_value_matches(pair: &Pair, current_value: &[u8]) -> bool {
    MatchText::of(current_value) == pair.match_text
}

/// **M9d: whether assignment alone is established for a pair's kind** —
/// composed from the pair's kind alone, so a kind nobody has posed reads *not
/// established* by construction rather than by omission.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AssignmentAlone {
    /// The image kind (Fact 99 — proved by render and not by a flag) and the
    /// library kind with its reload (Fact 101).
    Established,
    /// **Every other kind** — the five M9b names (cache file, sound, font,
    /// movie clip, volume) and the further kinds whose rewrite is inferred
    /// rather than watched (M19, §12 item 2). **Permanently unresolved** (§12
    /// item 1): no space is left here for an answer.
    NotEstablished,
}

/// **M9 and M9d, per kind.** What the field claims: that the assignment was
/// made, and that its sufficiency for that kind was never observed. What it
/// does not claim: that the render read the wrong file, or that anything
/// failed.
pub fn assignment_alone_is(kind: Kind) -> AssignmentAlone {
    match kind {
        Kind::Image | Kind::Library => AssignmentAlone::Established,
        _ => AssignmentAlone::NotEstablished,
    }
}

/// **M8 and M9: which kinds take a reload after the assignment.** For a
/// library, the assignment is followed by the reload, because assigning a
/// library's path changes what is recorded at once and leaves the old data in
/// place until reloaded — and assignment plus reload populates a library that
/// never loaded (Fact 101, §12 item 17 closed). **For an image the material
/// assigns and does not reload** (Fact 99). **M9c: the mechanism takes a
/// per-kind reload step by one edit here — a property of the shape, and not
/// a space kept for an answer, because no per-kind answer is coming.**
pub fn takes_a_reload_after_assignment(kind: Kind) -> bool {
    matches!(kind, Kind::Library)
}

/// **M8's per-pair outcome as the marker carries it** — applied and to how
/// many data-blocks, with M9d's field, or not applied. **Not applied means
/// zero data-blocks of the kind matched, a countable fact** — which covers an
/// edited reference and, per M8a, a data-block nobody was using that a plain
/// save dropped with its recorded place: **a known and benign cause**, and a
/// builder must not read every *not applied* as a re-pointed reference.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PairOutcome {
    Applied {
        data_blocks: usize,
        assignment_alone: AssignmentAlone,
        reloaded: bool,
    },
    NotApplied,
}

/// **M8's judgement over one pair**, given the current values of the named
/// setting on every data-block of the pair's kind: how many match, and what
/// the marker says for it.
pub fn the_outcome_of(
    pair: &Pair,
    current_values_on_every_data_block_of_the_kind: &[Vec<u8>],
) -> PairOutcome {
    let data_blocks = current_values_on_every_data_block_of_the_kind
        .iter()
        .filter(|value| a_current_value_matches(pair, value))
        .count();
    if data_blocks == 0 {
        PairOutcome::NotApplied
    } else {
        PairOutcome::Applied {
            data_blocks,
            assignment_alone: assignment_alone_is(pair.kind),
            reloaded: takes_a_reload_after_assignment(pair.kind),
        }
    }
}

/// **M5: every pair is applied by the per-setting route, in every version,
/// and no bulk route and no one-pass route is used for any part of it** — the
/// one route proved to reach every recorded location in the fixture (Fact
/// 77), where the one-pass route silently left every write location alone.
pub const EVERY_PAIR_IS_APPLIED_BY_THE_PER_SETTING_ROUTE: bool = true;

/// **M6: the material is composed per version and names the version it was
/// composed for**, exactly as the redirect's half is (`Job files and outputs`
/// D7, D8) — the setting names are resolved per version at composition, never
/// guessed at run time. 5.1 is inferred across the version boundary (G4).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComposedFor {
    pub build_path: String,
    pub version: String,
}

/// **M10: one remap-material document per job, immutable** — composed on the
/// desktop, from the record, at submission: the located answers and the
/// landing rule give the pairs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RemapMaterial {
    /// The job identity's textual form.
    pub job: String,
    pub composed_for: ComposedFor,
    pub pairs: Vec<Pair>,
    /// **M22c: the uploaded bakes the machine copies into the job folder
    /// before the render starts** — the instruction rides this material
    /// because the render worker already reads it at its step 2. **Absent from
    /// the written document where there is none**, so a material with no bake
    /// reads and writes exactly as it did before 2026-09-29, and one written
    /// before then reads as owing no copy.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub copies_owed: Vec<crate::placed_copies::TheCopyTheMachineOwes>,
}

impl RemapMaterial {
    pub fn composed(
        job: &Identity,
        composed_for: &BlenderIdentity,
        pairs: Vec<Pair>,
    ) -> RemapMaterial {
        debug_assert_eq!(job.kind(), IdentityKind::Job);
        RemapMaterial {
            job: job.as_text(),
            composed_for: ComposedFor {
                build_path: composed_for.path.clone(),
                version: version_text(composed_for.version),
            },
            pairs,
            copies_owed: Vec::new(),
        }
    }

    /// **M22c: the same material, carrying the copies the machine owes.**
    pub fn with_the_copies_owed(
        self,
        copies_owed: Vec<crate::placed_copies::TheCopyTheMachineOwes>,
    ) -> RemapMaterial {
        RemapMaterial {
            copies_owed,
            ..self
        }
    }

    /// **M10: in the asset area at `remaps/<job identity's textual form>`**,
    /// placed through the transfer contract before the job's submission
    /// material is published.
    pub fn location_in(layout: &TheLayout, job: &Identity) -> MachineLocation {
        MachineLocation::composed(
            format!("{}/{REMAPS}/{}", the_asset_area(layout), job.as_text()).into_bytes(),
        )
    }

    /// The document as placed on the machine — one JSON document, the form
    /// Build chose for the manifest and the attempt record too.
    pub fn as_written(&self) -> Vec<u8> {
        serde_json::to_vec_pretty(self)
            .expect("the material is composed of values that always render")
    }

    pub fn read_back(bytes: &[u8]) -> Option<RemapMaterial> {
        serde_json::from_slice(bytes).ok()
    }

    /// **M10: the manifest carries the reference to it** — `Job files and
    /// outputs` M1's carried-and-not-interpreted field, holding the
    /// material's machine location.
    pub fn manifest_reference(layout: &TheLayout, job: &Identity) -> CarriedAndNotInterpretedHere {
        let at = RemapMaterial::location_in(layout, job);
        CarriedAndNotInterpretedHere(serde_json::json!({
            "remap_material": String::from_utf8_lossy(at.as_bytes()),
        }))
    }
}

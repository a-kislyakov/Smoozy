//! The launch material — L10, L10a, L11, L12, L13: one document per
//! attempt, composed on the machine from the manifest, carrying four pair
//! families and the marker, applied inside the window.
//!
//! **L10: the contents, in order of application** — the marker's opening;
//! the redirect pairs; the remap pairs, with the library reload after
//! assignment; the settings pairs, every payload value with no argument in
//! the manifest's version; **the skip-existing-frames switch, set off — a
//! value this contract imposes on every attempt, in every version, and one
//! the payload never carries**; Simplify's switch and three settings as
//! settings pairs, exactly where the payload carries them; each pair's
//! outcome appended to the marker as it is applied; the read-back of the
//! regular output path, failing the attempt where it does not match the
//! map's pair; and the version the material was composed for. **No output
//! format and no media type is among them** — **but one: L10c's, added
//! 2026-10-27**, which writes a first-frame check of a job whose only output
//! is a video as a still picture, where the reading in force says it did.
//!
//! **L10a: two rules the material is composed under, both silent failures
//! closed.** The order in L8 is a rule and not an inference, and no
//! read-back can stand in for it — that is [`crate::line`]'s. **And every
//! settings pair is followed, in the same material, by a read of its own
//! name against the object it was assigned on, and a name that object does
//! not carry fails the attempt before the render instruction** — because in
//! Blender 4.5 a name Cycles' settings do not carry is accepted with no
//! error of any kind and the value is gone the instant it is read back. The
//! check is written for every settings pair rather than for that one group
//! in that one version, and it does not compare values (§9 item 25 is its
//! bound).
//!
//! **L11: the material contains no failure handling of its own.** A pair
//! whose assignment raises ends the run before the render instruction, and
//! that ending is L8a's argument and nothing else. **Held by the type**:
//! [`Entry`] has no variant that catches, and a test reads this crate's
//! sources for one.
//!
//! **What is here and what is the acting half's.** The material's contents
//! and their order are decided here, as a typed document; rendering that
//! document into the text the render's Blender runs, placing it, and L12's
//! read-back of the placement under the render's login are the worker's.
//! **L13's bound is stated rather than smoothed**: there is no route that
//! runs before the load, so the remap is in force *inside the same
//! material, in the same window, before the render instruction — after the
//! load by construction, because that is the only place there is.*
//!
//! **Markers used in this file that belong to another document, each with
//! the document it belongs to.** `Job files and outputs` D5, D7, D8, D9,
//! D10, D13, D16, D18, D19, M1. `Dependency Upload & Remap` M8, M9b, M9c,
//! M9d, M10. `Render Settings` C1, C7, C8, C9, C10, D3, V4, V5, V6, V7, Y1,
//! Y5. `Queue & History` D6.

use smoozy_dependencies_machine::remap::{takes_a_reload_after_assignment, Pair, RemapMaterial};
use smoozy_job_files_machine::redirect::AnAssignment;
use smoozy_render_settings_machine::device::{the_processor_pair, DeviceChoice};
use smoozy_render_settings_machine::engine::{what_the_page_shows, OfferedEngine, WhatThePageShows};
use smoozy_render_settings_machine::file_half::RecordedReading;
use smoozy_render_settings_machine::payload::Payload;
use smoozy_render_settings_machine::reading::SceneReading;
use smoozy_render_settings_machine::samples::{
    the_imposition_set, the_percentage_pair, MissingSpelling, PairValue, SettingsPair,
};
use smoozy_render_settings_machine::spellings::{Spelling as AsRenderSettingsSpellsIt, VIEW_LAYER_USE};
use smoozy_state::identities::Identity;

use crate::spellings::{
    self, Spelling, REGULAR_OUTPUT_PATH_READ_BACK, SIMPLIFY_CHILD_PARTICLES_RENDER,
    SIMPLIFY_SUBDIVISION_RENDER, SIMPLIFY_SWITCH, SIMPLIFY_TEXTURE_LIMIT_RENDER,
    SKIP_EXISTING_FRAMES_OFF_IS_OVERWRITE_ON, SKIP_EXISTING_FRAMES_SWITCH,
};
use crate::versions::TheFourVersions;

/// **One redirect entry of the destination map, and it is the shape the
/// other side produces** — `Job files and outputs`' own `AnAssignment`, taken
/// whole rather than narrowed into a shape of this crate's.
///
/// **Replaced this crate's own two-field pair on 2026-09-05, out of Round 1
/// of its code audit.** That pair was one setting and one value, and three
/// things did not fit through it, each of them a silent wrong render:
///
/// - **The further values had nowhere to go.** `Job files and outputs` P10
///   composes settings *beside* the folder for a place that needs them, and
///   that crate records at `src/mapping.rs` what setting the folder alone
///   produces for the physics point cache: the render finished and reported
///   success, and the cache was written beside the `.blend` — 250 files,
///   outside the job's folder, uncounted, destroyed with the machine.
/// - **The destination's scene was dropped.** A destination belonging to a
///   scene pulled in behind a compositor would have been assigned against the
///   job's scene or not found at all.
/// - **The setting was a dotted path nothing composed**, and a path into a
///   collection is one the material's walk cannot resolve — see
///   [`Entry::Redirect`].
///
/// **Nothing in that crate changed for this**, and nothing here composes a
/// redirect setting's name: **§9 item 2**'s per-version property split of
/// `Job files and outputs` D8 — `base_path` in 4.5.9, `directory` in 5.0.1
/// and 5.2.0 — is that document's to resolve per version and arrives already
/// resolved on the assignment (`Job files and outputs` D9, D13).
pub type RedirectAssignment = AnAssignment;

/// A value a Simplify pair assigns (A1d's ranges are the page's; the value
/// here is the payload's entry as carried).
#[derive(Debug, Clone, PartialEq)]
pub enum SimplifyValue {
    Switch(bool),
    WholeNumber(u32),
    Fraction(f64),
    /// A pick over the file's own enumeration, as the identifier read back.
    Identifier(String),
}

/// **One entry of the material, in L10's order of application.** The set is
/// closed, and **there is no entry that catches a failure** (L11).
#[derive(Debug, Clone, PartialEq)]
pub enum Entry {
    /// **The marker's opening, printed on the render's own ordinary output
    /// as the material's first act, naming the job identity** (`Job files
    /// and outputs` D16).
    MarkerOpening { job: String },
    /// **L8b's check, before any pair: the material reads whether the
    /// handler the script before the file registered ran to its end, and
    /// raises where it did not.** Added 2026-10-02, simulation fix piece 4.
    ///
    /// **A raise inside the handler does not fail the run** — observed at the
    /// desk in all four versions with L8a's argument on the line: Blender
    /// printed the traceback, loaded the file, rendered and ended at 0. **This
    /// check is what turns it into a run that ends before the render
    /// instruction**, at L8a's status with no frame written (also observed, in
    /// all four). The handler's own record of what it set is printed on the
    /// marker here.
    TheLoadTimeHandlerFinished,
    /// A redirect assignment, each destination's own setting written
    /// individually, per version, with no bulk route for any part of it
    /// (`Job files and outputs` D9, `Job files and outputs` D13).
    ///
    /// **The material is handed the assignment and never a property path.**
    /// The walk that resolves a property path splits it on the full stop and
    /// asks for each piece by name, so a path reaching into a collection —
    /// a file output node looked up by its own name — makes it ask for an
    /// attribute whose whole name is the bracketed piece, and no scene
    /// carries one. **Carrying the destination instead is what stops such a
    /// path existing at all**: the acting half looks the node up by key, on
    /// the destination's own scene, and assigns the folder and its further
    /// values there.
    Redirect(RedirectAssignment),
    /// A remap pair, applied by `Dependency Upload & Remap` M8's rules
    /// exactly: per-setting assignment on every data-block of the pair's
    /// kind whose named setting, put through `Dependency Upload & Remap`
    /// M4a's separator rule, equals the pair's **match text** — derived
    /// once at composition from the recorded location's expected bytes,
    /// and the only thing a pair is ever matched on; a pair whose match
    /// text is found nowhere reported as not applied; a reference matching
    /// no pair left exactly as recorded.
    ///
    /// **Never matched on the expected bytes as read.** That is what
    /// `Dependency Upload & Remap` M4a forbids and exists to prevent: a
    /// file checked on Windows has every forward slash in a recorded place
    /// written back as a backslash, so byte matching makes every pair of
    /// that file miss at once, every one is reported not applied, and the
    /// render finishes and reports success.
    Remap(Pair),
    /// **The library reload after assignment** — for the library kind
    /// (Fact 101, `Dependency Upload & Remap` M9c).
    LibraryReloadAfterAssignment,
    /// A settings pair — every payload value with no argument in the
    /// manifest's version, assigned against the job's scene (`Render
    /// Settings` C8) — **followed by its own name check**.
    Settings(SettingsPair),
    /// A view-layer pin pair, the three steps `Render Settings` V4 names:
    /// resolve the layer by its bytes, sever the switch's animation, assign
    /// the pinned value to `use` (V5–V7). An unapplied pin ends the attempt.
    Pin { layer: Vec<u8>, on: bool },
    /// **L10a's name check, after every settings pair**: a read of the
    /// pair's own name against the object it was assigned on; a name the
    /// object does not carry raises, which L8a turns into the run ending
    /// before the render instruction with nothing written, the pair
    /// recorded in the marker as unapplied. The value is not compared.
    ///
    /// **The property is a plain dotted name off the scene and never a
    /// written-out display form.** See [`Entry::PinNameCheck`] for what that
    /// distinction cost before 2026-09-05.
    NameCheck { property: &'static str },
    /// **L10a's name check for a view-layer pin**, which is not the entry
    /// above and cannot be.
    ///
    /// The pin is assigned on the layer object [`Entry::Pin`] resolves by the
    /// layer's own bytes, so the check is a read of the pin switch's own name
    /// **against that same object**, resolved the same way. It carries the
    /// layer rather than a path.
    ///
    /// **Why it is its own entry. Added 2026-09-05 out of Round 1 of this
    /// crate's code audit, where it was the round's most serious finding.**
    /// The pin's check used to be an [`Entry::NameCheck`] whose property was
    /// `Render Settings`' `VIEW_LAYER_USE` **written out from the scene** —
    /// a display form carrying a placeholder that stands in for the layer's
    /// name and that nothing ever filled. The material's walk splits a
    /// property on the full stop and asks the scene for each piece, so it
    /// asked for an attribute whose whole name was the bracketed piece with
    /// the placeholder still in it. No scene carries one, so the walk raised,
    /// L8a's switch turned that into a non-zero exit, and **every job that
    /// pinned a view layer failed before the render instruction with nothing
    /// written** — on every attempt and on the machine's own re-run alike,
    /// since the manifest is immutable and the material composes the same way
    /// each time. **Nothing composes a real property path out of a written-out
    /// display form**, and [`tests::no_property_this_material_carries_is_one_the_walk_cannot_resolve`]
    /// is what holds that over every family rather than over this one.
    PinNameCheck {
        layer: Vec<u8>,
        /// The pin switch's own name — `Render Settings` V4's `use`, the
        /// cited spelling's `name` and never its written-out form.
        property: &'static str,
    },
    /// **The skip-existing-frames switch, imposed off on every attempt.**
    SkipExistingFramesOff {
        property: &'static str,
        overwrite_on: bool,
    },
    /// A Simplify pair, against A1d's render-half target.
    Simplify {
        property: &'static str,
        value: SimplifyValue,
    },
    /// **L10c: the job's scene written as a still picture**, where the
    /// reading in force says this is a first-frame check of a job whose only
    /// output is a video and that it switched the scene (`Job files and
    /// outputs` R4c). From 5.0 the media setting is set to a picture first and
    /// then the format; in 4.5 the format alone. **The scene is then read, and
    /// a scene that still writes a video raises**, which L8a's argument turns
    /// into a run ending before the render instruction. Added 2026-10-27, the
    /// nineteenth notes, notes 3 and 9.
    TheCheckWrittenAsAPicture {
        /// The media setting's own name and the picture's value, where the
        /// version has one — `Job files and outputs — machine`'s
        /// `THE_MEDIA_KIND` and `A_STILL_PICTURES_MEDIA_KIND`, cited there.
        media: Option<(&'static str, &'static str)>,
        /// The format — `THE_CHECKS_PICTURE_FORMAT`, cited there.
        format: &'static str,
    },
    /// **`Job files and outputs` P18a: every name a first-frame check writes
    /// is marked `check_` at the start of its last piece**, where the job's
    /// payload names its range job as *writes into* (`Rendering` S13). The
    /// text is `Job files and outputs — machine`'s own, the one the reading
    /// ran before it read the file, here told not to mark the regular output
    /// path or 4.5's multilayer folder property: those arrive already marked
    /// through the map. Added 2026-10-31, the twenty-first notes, note 6.
    TheChecksNamesMarked {
        /// The names it is set by, for this version — that crate's
        /// `the_names_the_mark_is_set_by`, every one cited there.
        names: serde_json::Value,
        /// The mark — that crate's `THE_CHECKS_MARK`.
        mark: &'static str,
    },
    /// **L10b: the scene stepped frame by frame from its start, after every
    /// pair and before the read-back**, where the reading in force says the
    /// job's scene holds a simulation that is not baked (`Job files and
    /// outputs` R4 item 8). Added 2026-10-02 on the Creator's decision A.
    ///
    /// **Observed at the desk before it was built, 2026-10-02, in all four
    /// versions**: stepped this way and then rendered by the ordinary
    /// command-line render of the list, an unbaked geometry-nodes simulation
    /// zone gave frames identical to references stepped one run per frame,
    /// from the start and from part-way; unrendered it is frame 1's picture on
    /// every frame. **L1 is unmoved**: nothing renders from the material.
    Stepping(TheStepping),
    /// **Each pair's outcome appended to the marker as it is applied** —
    /// set, and to how many data-blocks, or not applied with the bytes found
    /// (M8, D16). One entry after every pair family.
    MarkerOutcomesAppended,
    /// **The read-back**: the value in force for the regular output path —
    /// the one pair the material does not set — printed in the marker and
    /// **the attempt failed where it does not match the map's pair, as raw
    /// text** (D10).
    ReadBack {
        property: &'static str,
        must_equal: Vec<u8>,
    },
    /// The version the material was composed for (D7).
    ComposedFor { version: TheFourVersions },
}

/// **L10b: the frames the material steps through.** The scene is stepped
/// from its own `frame_start` — or the lowest frame of the attempt's list
/// where that is earlier — to the highest frame of the list, every frame in
/// order. **The list is the reckoning's, in the order written** (L18), so it
/// is read for its lowest and highest frames rather than its first and last
/// entries: a re-run's scattered missing frames and a *Carry on* job's tail
/// both step from the scene's start to the highest frame they render.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TheStepping {
    pub the_lists_lowest: i64,
    pub the_lists_highest: i64,
}

impl TheStepping {
    /// **Where `Job files and outputs` R4 item 8 says yes, the stepping for
    /// this list; where it says no, or the list is empty, none** — a file
    /// the field says no to pays nothing.
    pub fn for_the_list(the_scene_holds_a_simulation_not_baked: bool, list: &[i64]) -> Option<TheStepping> {
        if !the_scene_holds_a_simulation_not_baked {
            return None;
        }
        Some(TheStepping {
            the_lists_lowest: *list.iter().min()?,
            the_lists_highest: *list.iter().max()?,
        })
    }
}

/// **The material: one document per attempt.** The field is private and
/// the only constructor is [`compose`], so a material cannot be assembled
/// in another order.
#[derive(Debug, Clone, PartialEq)]
pub struct LaunchMaterial {
    entries: Vec<Entry>,
}

impl LaunchMaterial {
    pub fn entries(&self) -> &[Entry] {
        &self.entries
    }

    /// **L11 as a property of the value.** There is no entry that catches.
    pub fn contains_no_failure_handling(&self) -> bool {
        true
    }
}

/// What the material is composed from.
pub struct MaterialInputs<'a> {
    pub job: &'a Identity,
    pub version: TheFourVersions,
    /// The destination map's assignments, in force for this attempt (P1
    /// steps 3 and 4), exactly as `Job files and outputs` composed them.
    pub redirect_pairs: &'a [RedirectAssignment],
    /// The remap material the manifest names, read at P1 step 2 (L30).
    pub remap: &'a RemapMaterial,
    /// The immutable settings payload (M1).
    pub payload: &'a Payload,
    /// The reading in force for this attempt, and the job's scene in it —
    /// the manifest's named scene, or the file's showing scene where it
    /// names none (`Render Settings` C8).
    pub reading_in_force: &'a RecordedReading,
    pub the_jobs_scene: &'a SceneReading,
    /// The map's pair for the regular output path, as raw text (D10).
    pub regular_output_path: &'a [u8],
    /// **L10b: the stepping, where the reading in force says the job's scene
    /// holds a simulation that is not baked** — [`TheStepping::for_the_list`].
    pub the_stepping: Option<TheStepping>,
    /// **L10c: whether the reading in force switched the job's scene from a
    /// video to a still picture** (`Job files and outputs` R4c). Added
    /// 2026-10-27.
    pub the_check_is_written_as_a_picture: bool,
}

/// Why the material could not be composed. **Every one is a failed attempt
/// before the render instruction** (K14's failed limb), loud and total.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CannotCompose {
    /// A spelling has not been read, so a pair cannot be composed and the
    /// attempt fails here, naming it. **`Render Settings` §13 item 29's five
    /// landed on 2026-09-20 and nothing is owed today**; this stays because
    /// `Rendering` L10a's name check stands whatever the names turned out to
    /// be, and a spelling later found to differ where Smoozy renders comes
    /// back as an absence rather than as a guess.
    ASpellingIsNotYetRead(MissingSpelling),
    /// A spelling of this crate's own is not on record.
    ASpellingIsNotOnRecord(spellings::MissingSpelling),
    /// A count was typed and the engine in force is neither offered one,
    /// so no count pair has a setting to target.
    ATypedCountUnderAnEngineThePageDoesNotOffer,
    /// **A redirect assignment whose folder has no setting name on record**,
    /// naming the destination it belongs to.
    ///
    /// `Job files and outputs`' assignment carries the property where that
    /// document's D8 names one, which is the file output node's folder, and
    /// its four added write places' settings are that crate's
    /// `the_settings_of`. **Nothing here composes a redirect setting's name**,
    /// so an assignment with none — a node assignment arriving without its
    /// property, or the regular output path, which its own argument sets — is
    /// refused loudly and named, which is a failed attempt before the render
    /// instruction with nothing written.
    ///
    /// **Until 2026-09-26 this was every job.** The four places had no names on
    /// record, a factory cube carries the bake output path, and step 10c's sixth
    /// pass on Vast ended every job here on either device. **What passing a
    /// folder over in silence costs is worse and is observed**: `Job files and
    /// outputs` records at `src/mapping.rs` a render that finished and reported
    /// success with the cache beside the `.blend`, 250 files outside the job's
    /// folder, destroyed with the machine — which is why this stays a refusal.
    ARedirectWhoseSettingIsNotOnRecord { destination: String },
}

impl CannotCompose {
    /// **Why the launch material could not be composed, in standard software
    /// words** (Rule 9) — beside the type, so a recorded reason is a sentence.
    /// Added 2026-09-23, step 10c's fifth pass on Vast, fault 5.
    ///
    /// **A missing spelling names the setting and the clause that closes it**,
    /// because those two are what somebody acts on; neither is parsed back out.
    pub fn in_words(&self) -> String {
        match self {
            CannotCompose::ASpellingIsNotYetRead(missing) => format!(
                "Blender's name for {} is not yet known ({})",
                missing.the_setting, missing.the_clause
            ),
            CannotCompose::ASpellingIsNotOnRecord(missing) => format!(
                "Blender's name for {} is not on record ({})",
                missing.what, missing.the_clause
            ),
            CannotCompose::ATypedCountUnderAnEngineThePageDoesNotOffer => {
                "a sample count was set, but this engine has none".to_string()
            }
            CannotCompose::ARedirectWhoseSettingIsNotOnRecord { destination } => format!(
                "no known folder setting for one of this file's outputs ({destination})"
            ),
        }
    }
}

/// **L10, performed: the material composed in the order of application.**
pub fn compose(inputs: MaterialInputs<'_>) -> Result<LaunchMaterial, CannotCompose> {
    let text = |spelling: Spelling| {
        spelling
            .text()
            .map_err(CannotCompose::ASpellingIsNotOnRecord)
    };
    let mut entries = vec![
        Entry::MarkerOpening {
            job: inputs.job.as_text(),
        },
        // **L8b: the handler's check comes before any pair**, so nothing is
        // set over a file whose load-time places were not.
        Entry::TheLoadTimeHandlerFinished,
    ];

    // The redirect assignments (`Job files and outputs` D9, `Job files and
    // outputs` D13). **An assignment the material cannot apply is refused
    // here and never passed over**, which is the whole of what
    // [`CannotCompose::ARedirectWhoseSettingIsNotOnRecord`] is for.
    for assignment in inputs.redirect_pairs {
        // **`Job files and outputs` D1a: a place set as the file loads is
        // never set by the material** — the Replay fluid cache folder, which
        // writes beside the `.blend` the moment the file opens, and the
        // geometry-nodes bake folder, which is ignored when set after the load
        // (both observed at the desk, 2026-10-02). They are
        // [`crate::load_time`]'s, and **until 2026-10-02 the material set
        // every geometry-nodes bake folder after the load, which never took.**
        if let smoozy_job_files_machine::reading::DestinationKind::AnAddedWritePlace(place) =
            assignment.destination.kind
        {
            if smoozy_job_files_machine::redirect::is_set_as_the_file_loads(place) {
                continue;
            }
        }
        the_redirects_holder(assignment)?;
        entries.push(Entry::Redirect(assignment.clone()));
    }
    entries.push(Entry::MarkerOutcomesAppended);

    // The remap pairs, each library pair followed by its reload (M8, M9c).
    for pair in &inputs.remap.pairs {
        let reload = takes_a_reload_after_assignment(pair.kind);
        entries.push(Entry::Remap(pair.clone()));
        if reload {
            entries.push(Entry::LibraryReloadAfterAssignment);
        }
    }
    entries.push(Entry::MarkerOutcomesAppended);

    // The settings pairs — every payload value with no argument in the
    // manifest's version — each followed by its name check (L10a).
    for pair in the_settings_pairs(
        inputs.payload,
        inputs.reading_in_force,
        inputs.the_jobs_scene,
        inputs.version,
    )? {
        let property = pair.property;
        entries.push(Entry::Settings(pair));
        entries.push(Entry::NameCheck { property });
    }
    // **The pin's own name check carries the layer, not a path.** The switch's
    // written-out form is a display form with a placeholder in it, and nothing
    // fills the placeholder — see [`Entry::PinNameCheck`].
    let the_pin_switch = match VIEW_LAYER_USE {
        AsRenderSettingsSpellsIt::Cited { name, .. } => name,
        AsRenderSettingsSpellsIt::NotYetRead(missing) => {
            return Err(CannotCompose::ASpellingIsNotYetRead(missing))
        }
    };
    for pin in &inputs.payload.view_layer_pins {
        let layer = pin.layer.as_bytes().to_vec();
        entries.push(Entry::Pin {
            layer: layer.clone(),
            on: pin.on,
        });
        entries.push(Entry::PinNameCheck {
            layer,
            property: the_pin_switch,
        });
    }

    // The skip-existing-frames switch, imposed off on every attempt. There
    // is no condition on this push and no input that could remove it.
    let skip = text(SKIP_EXISTING_FRAMES_SWITCH)?;
    entries.push(Entry::SkipExistingFramesOff {
        property: skip,
        overwrite_on: SKIP_EXISTING_FRAMES_OFF_IS_OVERWRITE_ON,
    });
    entries.push(Entry::NameCheck { property: skip });

    // Simplify's switch and three settings, exactly where the payload
    // carries them (A1d's render-half targets).
    let simplify = &inputs.payload.simplify;
    let mut simplify_pairs: Vec<(&'static str, SimplifyValue)> = Vec::new();
    if let Some(on) = simplify.use_simplify {
        simplify_pairs.push((text(SIMPLIFY_SWITCH)?, SimplifyValue::Switch(on)));
    }
    if let Some(value) = simplify.subdivision_render {
        simplify_pairs.push((
            text(SIMPLIFY_SUBDIVISION_RENDER)?,
            SimplifyValue::WholeNumber(value),
        ));
    }
    if let Some(value) = simplify.child_particles_render {
        simplify_pairs.push((
            text(SIMPLIFY_CHILD_PARTICLES_RENDER)?,
            SimplifyValue::Fraction(value),
        ));
    }
    if let Some(value) = &simplify.texture_limit_render {
        simplify_pairs.push((
            text(SIMPLIFY_TEXTURE_LIMIT_RENDER)?,
            SimplifyValue::Identifier(value.clone()),
        ));
    }
    for (property, value) in simplify_pairs {
        entries.push(Entry::Simplify { property, value });
        entries.push(Entry::NameCheck { property });
    }
    entries.push(Entry::MarkerOutcomesAppended);

    // **L10c: the check's picture, after every pair and before the stepping
    // and the read-back**, where the reading in force switched the scene. The
    // names are `Job files and outputs — machine`'s, cited there; a version
    // with no name for one of them fails the composition rather than guess.
    if inputs.the_check_is_written_as_a_picture {
        use smoozy_job_files_machine::reading_script::{
            A_STILL_PICTURES_MEDIA_KIND, THE_CHECKS_PICTURE_FORMAT, THE_MEDIA_KIND,
        };
        let (major, minor) = match inputs.version {
            TheFourVersions::V4_5 => (4, 5),
            TheFourVersions::V5_0 => (5, 0),
            TheFourVersions::V5_1 => (5, 1),
            TheFourVersions::V5_2 => (5, 2),
            TheFourVersions::V5_3 => (5, 3),
            TheFourVersions::V5_4 => (5, 4),
        };
        let in_hand = smoozy_job_files_machine::reading::TheVersionInHand::of(major, minor);
        let not_on_record = |missing: smoozy_job_files_machine::reading_script::MissingSpelling| {
            CannotCompose::ASpellingIsNotOnRecord(spellings::MissingSpelling {
                what: missing.what,
                the_clause: missing.the_clause,
            })
        };
        let media = match (
            THE_MEDIA_KIND.for_version(in_hand).map_err(not_on_record)?,
            A_STILL_PICTURES_MEDIA_KIND.for_version(in_hand).map_err(not_on_record)?,
        ) {
            (Some(property), Some(value)) => Some((property, value)),
            _ => None,
        };
        let format = THE_CHECKS_PICTURE_FORMAT
            .for_version(in_hand)
            .map_err(not_on_record)?
            .ok_or(CannotCompose::ASpellingIsNotOnRecord(spellings::MissingSpelling {
                what: "the picture format a first-frame check is written in",
                the_clause: "Rendering L10c",
            }))?;
        entries.push(Entry::TheCheckWrittenAsAPicture { media, format });
    }

    // **`Job files and outputs` P18a: the check's names marked, after every
    // pair and before the read-back**, by the same text and the same names the
    // reading marked them with. A version with no name for one of them fails
    // the composition rather than guess.
    if inputs.payload.the_check_writes_into.is_some() {
        let in_hand = match inputs.version {
            TheFourVersions::V4_5 => (4, 5),
            TheFourVersions::V5_0 => (5, 0),
            TheFourVersions::V5_1 => (5, 1),
            TheFourVersions::V5_2 => (5, 2),
            TheFourVersions::V5_3 => (5, 3),
            TheFourVersions::V5_4 => (5, 4),
        };
        let in_hand = smoozy_job_files_machine::reading::TheVersionInHand::of(in_hand.0, in_hand.1);
        let names = smoozy_job_files_machine::reading_script::the_names_the_mark_is_set_by(in_hand)
            .map_err(|missing| {
                CannotCompose::ASpellingIsNotOnRecord(spellings::MissingSpelling {
                    what: missing.what,
                    the_clause: missing.the_clause,
                })
            })?;
        entries.push(Entry::TheChecksNamesMarked {
            names,
            mark: smoozy_job_files_machine::reading_script::THE_CHECKS_MARK,
        });
    }

    // **L10b: the stepping, after every pair and before the read-back**, where
    // the reading in force says the job's scene holds a simulation that is not
    // baked. Nothing is stepped where it says no.
    if let Some(stepping) = inputs.the_stepping {
        entries.push(Entry::Stepping(stepping));
    }

    // The read-back (D10), and the version (D7).
    entries.push(Entry::ReadBack {
        property: text(REGULAR_OUTPUT_PATH_READ_BACK)?,
        must_equal: inputs.regular_output_path.to_vec(),
    });
    entries.push(Entry::ComposedFor {
        version: inputs.version,
    });
    Ok(LaunchMaterial { entries })
}

/// **What one redirect assignment is set on, and by which name.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TheRedirectsHolder<'a> {
    /// **A file output node**, by its own name, and the folder's property on
    /// it — `Job files and outputs` D8's, per version, arriving resolved.
    AFileOutputNode {
        node: &'a [u8],
        property: &'a str,
        /// **`Job files and outputs` D8a, from 5.3: the effect whose group holds
        /// the node**, by its name as the file holds it.
        effect: Option<&'a [u8]>,
    },
    /// **One of `Job files and outputs` D20's four added write places**, and
    /// every setting that holds its folder, each on what carries it —
    /// `Job files and outputs`' own `the_settings_of`, read off four running
    /// Blenders on 2026-09-26 and the same in all four.
    AnAddedWritePlace {
        place: smoozy_job_files_machine::reading::AnAddedWritePlace,
        settings: &'static [smoozy_job_files_machine::redirect::ASettingOfAnAddedPlace],
    },
}

/// **What one redirect assignment is set on, and by which name** — the node's
/// own name and the folder's own setting, or an added write place's named
/// settings; none of it composed here.
///
/// **It is the one gate on the redirect family and [`compose`] runs it over
/// every assignment before any entry is pushed**, so an [`Entry::Redirect`]
/// inside a [`LaunchMaterial`] is always one the acting half can apply — the
/// same discipline [`crate::line::LaunchLine::from_placed`] keeps for the
/// line.
///
/// **The four added write places are set, and until 2026-09-26 every one was
/// refused.** No setting name for any of them was on record, and a factory
/// cube carries the bake output path, so on step 10c's sixth pass on Vast
/// every job ended here on either device. The names were then read off the
/// four Blenders at the desk — `Job files and outputs`' `the_settings_of`
/// carries them, each cited.
pub fn the_redirects_holder(
    assignment: &RedirectAssignment,
) -> Result<TheRedirectsHolder<'_>, CannotCompose> {
    use smoozy_job_files_machine::reading::DestinationKind;
    let refuse = || CannotCompose::ARedirectWhoseSettingIsNotOnRecord {
        // **The place in the artist's own terms and never this type's name** —
        // step 10c's fifth pass on Vast, fault 5.
        destination: assignment.destination.origin.in_words(),
    };
    let node = match &assignment.destination.kind {
        DestinationKind::AnOutputOfAFileOutputNode { node, .. }
        | DestinationKind::AWholeFileOutputNodeGatheringIntoOneFile { node, .. } => node.as_bytes(),
        // **`Job files and outputs` D10: the regular output path is settled
        // by its own command-line argument and the material never sets it**,
        // so it is never a redirect entry and one handed in here is refused
        // rather than assigned twice.
        DestinationKind::TheRegularOutputPath => return Err(refuse()),
        DestinationKind::AnAddedWritePlace(place) => {
            let settings = smoozy_job_files_machine::redirect::the_settings_of(*place);
            // **A place with no setting named is still refused, loudly**,
            // rather than passed over — the silent case is the 250 cache files
            // written beside the `.blend`.
            if settings.is_empty() {
                return Err(refuse());
            }
            return Ok(TheRedirectsHolder::AnAddedWritePlace {
                place: *place,
                settings,
            });
        }
    };
    let property = assignment.property.as_deref().ok_or_else(refuse)?;
    Ok(TheRedirectsHolder::AFileOutputNode {
        node,
        property,
        effect: assignment.destination.the_effect().map(|effect| effect.as_bytes()),
    })
}

/// **L10's settings-pair family: every value of the payload that has no
/// command-line argument in the manifest's version** — the sample count
/// with `Render Settings` C1's imposition set, the resolution percentage
/// (C7), and D3's processor override where the user overrode a cards scene.
/// **Which settings the payload carries is `Render Settings`'; that each is
/// delivered by argument where the version has one and by pair where it has
/// none is this contract's, resolved at composition.** The scene, the
/// engine, the output path and the frames have arguments in all four
/// versions and never ride here (*Settings 1*).
fn the_settings_pairs(
    payload: &Payload,
    reading: &RecordedReading,
    scene: &SceneReading,
    version: TheFourVersions,
) -> Result<Vec<SettingsPair>, CannotCompose> {
    let mut pairs = Vec::new();
    if let Some(count) = payload.sample_count {
        let engine = match payload.engine.as_ref().map(|entry| entry.pick) {
            Some(pick) => pick,
            None => match what_the_page_shows(scene, reading) {
                WhatThePageShows::Offered(engine) => engine,
                WhatThePageShows::TheFilesOwn(_) => {
                    return Err(CannotCompose::ATypedCountUnderAnEngineThePageDoesNotOffer)
                }
            },
        };
        let _: OfferedEngine = engine;
        pairs.extend(
            // `Render Settings` C1 item 3: which time limits there are to turn off
            // depends on the version in hand.
            the_imposition_set(engine, count, version.major_and_minor())
                .map_err(CannotCompose::ASpellingIsNotYetRead)?,
        );
    }
    if let Some(percentage) = payload.resolution_percentage {
        pairs.push(the_percentage_pair(percentage));
    }
    if let Some(pair) =
        the_processor_pair(payload.device, scene).map_err(CannotCompose::ASpellingIsNotYetRead)?
    {
        let _: DeviceChoice = DeviceChoice::TheProcessor;
        pairs.push(pair);
    }
    Ok(pairs)
}

/// **L10: no output format and no media type is among the pairs** — the
/// payload has nine entry kinds and neither, so this contract sets no
/// format and the ordering trap it once named is never reached. **The one
/// format the material sets is not a pair**: it is L10c's entry,
/// [`Entry::TheCheckWrittenAsAPicture`], which sets the media setting before
/// the format, as the trap requires, and reads the result back (2026-10-27).
pub const NO_FORMAT_AND_NO_MEDIA_TYPE_IS_EVER_SET: bool = true;

/// **L13: the remap is in force by the time the render loads the file,
/// bounded by the window and stated rather than smoothed** — inside the
/// same material, in the same window, before the render instruction, after
/// the load by construction.
pub const THE_REMAP_IS_IN_FORCE_INSIDE_THE_SAME_MATERIAL_AFTER_THE_LOAD: bool = true;

/// **L12: the material's placement is confirmed by reading it back under
/// the login the render runs as** (`Job files and outputs` D19). It closes
/// the one shape of the unreadable-script hazard that is Smoozy's own doing
/// and is not offered as making the failure loud; the marker's absence is
/// readable as *never established*. The read-back is the acting half's.
pub const THE_PLACEMENT_IS_CONFIRMED_BY_READING_IT_BACK_UNDER_THE_RENDERS_LOGIN: bool = true;

/// A settings pair's value, for tests and for the acting half's rendering.
pub fn the_value_of(pair: &SettingsPair) -> &PairValue {
    &pair.value
}

/// **The readings, payloads, jobs and remap material the tests build from,
/// written once for both folders of this module.**
///
/// `smoozy-rendering` keeps the tests over what stayed there — the submission,
/// the warning, the diary and the desk-side acts — and they build from these. A
/// test-only module cannot be reached from another crate, so this one is
/// compiled for this crate's own tests and, behind the `testing` switch, for
/// that crate's tests, which turn it on as a development dependency. **Nothing
/// outside a test ever turns it on**, and the program a rented machine runs
/// does not. Moved here from this file's tests on 2026-09-19, unchanged.
#[cfg(any(test, feature = "testing"))]
pub mod testing {
    use super::*;
    use smoozy_dependencies_machine::remap::{compose_pair, NewLocation};
    use smoozy_render_settings_machine::file_half::{BlenderIdentity, RecordedReading};
    use smoozy_render_settings_machine::names::ExactName;
    use smoozy_render_settings_machine::payload::VersionPick;
    use smoozy_render_settings_machine::reading::*;
    use smoozy_render_settings_machine::store::SimplifyEntries;
    use smoozy_framing::source_reading::SourceReading;
    use smoozy_state::identities::IdentityKind;
    use smoozy_state::observed::{Clock, Reading};
    use std::time::{Duration, SystemTime};

    pub fn a_blender() -> BlenderIdentity {
        BlenderIdentity {
            path: "/ephemeral/smoozy/blender/blender-5.2.0/blender".to_string(),
            version: crate::versions::testing::a_version(5, 2, 0),
        }
    }

    pub fn a_scene(name: &str) -> SceneReading {
        SceneReading {
            prepare_simulations: None,
            motion_blur: None,
            name: ExactName::as_the_reading_produced_it(name),
            frame_start: 1,
            frame_end: 10,
            frame_step: 1,
            frame_count: 10,
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
            eevee_time_limit: None,
            layer_samples_rule: LayerSamplesRule::Use,
            saved_with: "5.2.0".to_string(),
            view_layers: vec![ViewLayerReading {
                name: ExactName::as_the_reading_produced_it("ViewLayer"),
                renders_at: (1..=10).map(|frame| (frame, true)).collect(),
                animation: SwitchAnimation::None { value: true },
                own_sample_count: 0,
                overrides_the_scenes_count: false,
            }],
            pulls_in: Vec::new(),
            simplify: SimplifyReadBack {
                use_simplify: false,
                subdivision_render: 6,
                child_particles_render: 1.0,
                texture_limit_render: "OFF".to_string(),
            },
        }
    }

    pub fn four_values() -> SourceReading {
        SourceReading {
            size: 4096,
            inode: "912".to_string(),
            modification_time: 1_756_800_000,
            change_time: 1_756_800_001,
        }
    }

    pub fn a_moment() -> SystemTime {
        SystemTime::UNIX_EPOCH + Duration::from_secs(1_756_900_000)
    }

    pub fn a_reading() -> RecordedReading {
        RecordedReading {
            settings: SettingsHalf {
                scenes: vec![a_scene("Kitchen"), a_scene("Exterior")],
                offered_engine_identifiers: OfferedEngineIdentifiers {
                    cycles: Some(ExactName::as_the_reading_produced_it("CYCLES")),
                    eevee: Some(ExactName::as_the_reading_produced_it("BLENDER_EEVEE")),
                },
                announced_add_on_case: None,
                the_folders_at_the_machines_top: None,
            },
            taken_by: a_blender(),
            file_identity: four_values(),
            stored_at: Reading::taken((), a_moment(), Clock::ThisComputer),
        }
    }

    pub fn an_untouched_payload() -> Payload {
        Payload {
            prepare_simulations: true,
            version_pick: VersionPick::of(&a_blender()),
            scene: None,
            frame_range: None,
            sample_count: None,
            resolution_percentage: None,
            engine: None,
            device: None,
            view_layer_pins: Vec::new(),
            simplify: SimplifyEntries::default(),
            the_check_is_written_as_a_picture: false,
            the_check_writes_into: None,
        }
    }

    pub fn a_job() -> Identity {
        Identity::issue(IdentityKind::Job).unwrap()
    }

    pub fn a_remap(job: &Identity, with_a_library_pair: bool) -> RemapMaterial {
        use smoozy_dependencies_machine::bytes::{MachineLocation, RecordedLocation};
        use smoozy_dependencies_machine::check::Kind;
        let image = compose_pair(
            Kind::Image,
            "filepath",
            &RecordedLocation::as_the_reading_carried_it(b"//tex/wood.png"),
            NewLocation::AMirror(MachineLocation::composed(
                b"/ephemeral/smoozy/assets/mirror/tex/wood.png".to_vec(),
            )),
        )
        .unwrap();
        let mut pairs = vec![image];
        if with_a_library_pair {
            pairs.push(
                compose_pair(
                    Kind::Library,
                    "filepath",
                    &RecordedLocation::as_the_reading_carried_it(b"//props.blend"),
                    NewLocation::AFixedCopy(MachineLocation::composed(
                        b"/ephemeral/smoozy/assets/fixed/props.blend".to_vec(),
                    )),
                )
                .unwrap(),
            );
        }
        RemapMaterial::composed(job, &a_blender(), pairs)
    }

    /// **One redirect assignment of the shape `Job files and outputs` really
    /// produces** — a file output node's folder, keyed to the destination the
    /// reading found, with that document's own trailing separator on it.
    ///
    /// **This fixture used to write a dotted property path**,
    /// `scene.node_tree.nodes["File Output"].directory`, which is the shape
    /// the material's walk cannot resolve: it splits on the full stop and
    /// asks for an attribute whose whole name is the bracketed piece. The
    /// path is gone with the pair it belonged to.
    pub fn a_redirect_assignment(node: &str) -> RedirectAssignment {
        use smoozy_job_files_machine::reading::{
            ArtistsText, Destination, DestinationKind, WhereItComesFrom, WhichScene,
        };
        let named = || ArtistsText::of(node.as_bytes().to_vec());
        RedirectAssignment {
            destination: Destination {
                kind: DestinationKind::AWholeFileOutputNodeGatheringIntoOneFile {
                    the_effect: None,
                    node: named(),
                    the_nodes_base_file_name: None,
                    replaced: None,
                },
                recorded_location: None,
                format: None,
                origin: WhereItComesFrom::AFileOutputNodeNamed(named()),
                scene: WhichScene::TheJobsOwnScene,
                the_view_layers_that_feed_it: Vec::new(),
                the_location_with_its_folder_replaced: None,
            },
            folder: format!("/ephemeral/smoozy/jobs/j1/{node}/").into_bytes(),
            // **D8's per-version property, resolved by that document and
            // arriving already resolved.** Nothing here composes it.
            property: Some("directory".to_string()),
            further_values: Vec::new(),
        }
    }

    pub fn compose_with(
        payload: &Payload,
        remap: &RemapMaterial,
        job: &Identity,
    ) -> Result<LaunchMaterial, CannotCompose> {
        let reading = a_reading();
        let scene = a_scene("Kitchen");
        let redirects = [a_redirect_assignment("File Output")];
        compose(MaterialInputs {
            job,
            version: TheFourVersions::V5_2,
            redirect_pairs: &redirects,
            remap,
            payload,
            reading_in_force: &reading,
            the_jobs_scene: &scene,
            regular_output_path: b"/ephemeral/smoozy/jobs/j1/frame_####",
            the_stepping: None,
            the_check_is_written_as_a_picture: false,
        })
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    pub(crate) use super::testing::*;
    use smoozy_render_settings_machine::engine::EngineEntry;
    use smoozy_render_settings_machine::names::ExactName;

    /// **L10's order of application, read off the entries.**
    #[test]
    fn the_material_carries_l10s_families_in_l10s_order() {
        let job = a_job();
        let remap = a_remap(&job, true);
        let mut payload = an_untouched_payload();
        payload.resolution_percentage = Some(50);
        payload.simplify.use_simplify = Some(true);
        let material = compose_with(&payload, &remap, &job).unwrap();
        let kinds: Vec<&str> = material
            .entries()
            .iter()
            .map(|entry| match entry {
                Entry::MarkerOpening { .. } => "marker",
                Entry::TheLoadTimeHandlerFinished => "handler-check",
                Entry::Redirect(_) => "redirect",
                Entry::Remap(_) => "remap",
                Entry::LibraryReloadAfterAssignment => "reload",
                Entry::Settings(_) => "settings",
                Entry::Pin { .. } => "pin",
                Entry::NameCheck { .. } => "check",
                Entry::PinNameCheck { .. } => "pin-check",
                Entry::SkipExistingFramesOff { .. } => "skip-off",
                Entry::Simplify { .. } => "simplify",
                Entry::MarkerOutcomesAppended => "outcomes",
                Entry::Stepping(_) => "stepping",
                Entry::TheCheckWrittenAsAPicture { .. } => "picture",
                Entry::TheChecksNamesMarked { .. } => "marked",
                Entry::ReadBack { .. } => "read-back",
                Entry::ComposedFor { .. } => "version",
            })
            .collect();
        assert_eq!(
            kinds,
            vec![
                "marker",
                "handler-check",
                "redirect",
                "outcomes",
                "remap",
                "remap",
                "reload",
                "outcomes",
                "settings",
                "check",
                "skip-off",
                "check",
                "simplify",
                "check",
                "outcomes",
                "read-back",
                "version"
            ]
        );
        assert_eq!(
            material.entries()[0],
            Entry::MarkerOpening { job: job.as_text() }
        );
        assert!(material.contains_no_failure_handling());
    }

    /// **L10b: the stepping sits after every pair and before the read-back,
    /// and only where `Job files and outputs` R4 item 8 says yes.** The list
    /// is read for its lowest and highest frames, so a re-run's scattered
    /// frames step from the scene's start to the highest of them.
    #[test]
    fn the_stepping_comes_after_every_pair_and_before_the_read_back_where_item_8_says_yes() {
        assert_eq!(TheStepping::for_the_list(false, &[1, 2, 3]), None);
        assert_eq!(TheStepping::for_the_list(true, &[]), None);
        let stepping = TheStepping::for_the_list(true, &[17, 5, 9]).unwrap();
        assert_eq!(
            stepping,
            TheStepping {
                the_lists_lowest: 5,
                the_lists_highest: 17
            }
        );
        let job = a_job();
        let remap = a_remap(&job, true);
        let payload = an_untouched_payload();
        let reading = a_reading();
        let scene = a_scene("Kitchen");
        let redirects = [a_redirect_assignment("File Output")];
        let compose_it = |the_stepping| {
            compose(MaterialInputs {
                job: &job,
                version: TheFourVersions::V5_2,
                redirect_pairs: &redirects,
                remap: &remap,
                payload: &payload,
                reading_in_force: &reading,
                the_jobs_scene: &scene,
                regular_output_path: b"/ephemeral/smoozy/jobs/j1/frame_####",
                the_stepping,
                the_check_is_written_as_a_picture: false,
            })
            .unwrap()
        };
        assert!(!compose_it(None)
            .entries()
            .iter()
            .any(|entry| matches!(entry, Entry::Stepping(_))));
        let material = compose_it(Some(stepping));
        let entries = material.entries();
        let at = entries
            .iter()
            .position(|entry| *entry == Entry::Stepping(stepping))
            .expect("the stepping is in the material");
        assert_eq!(entries.iter().filter(|entry| matches!(entry, Entry::Stepping(_))).count(), 1);
        assert!(matches!(entries[at + 1], Entry::ReadBack { .. }), "{:?}", entries[at + 1]);
        assert!(entries[..at].iter().all(|entry| !matches!(entry, Entry::ReadBack { .. })));
        let last_pair = entries
            .iter()
            .rposition(|entry| {
                matches!(
                    entry,
                    Entry::Redirect(_)
                        | Entry::Remap(_)
                        | Entry::Settings(_)
                        | Entry::Pin { .. }
                        | Entry::SkipExistingFramesOff { .. }
                        | Entry::Simplify { .. }
                )
            })
            .unwrap();
        assert!(last_pair < at);
    }

    /// **L10c, the nineteenth notes, notes 3 and 9: where the reading in force
    /// switched a first-frame check of a video-only job to a picture, the
    /// material makes the same switch** — after every pair, before the
    /// stepping and the read-back; from 5.0 the media setting first, in 4.5
    /// the format alone; and nowhere else.
    #[test]
    fn a_check_written_as_a_picture_is_switched_after_the_pairs_and_before_the_read_back() {
        let job = a_job();
        let remap = a_remap(&job, true);
        let payload = an_untouched_payload();
        let reading = a_reading();
        let scene = a_scene("Kitchen");
        let redirects = [a_redirect_assignment("File Output")];
        let compose_it = |version, the_check_is_written_as_a_picture, the_stepping| {
            compose(MaterialInputs {
                job: &job,
                version,
                redirect_pairs: &redirects,
                remap: &remap,
                payload: &payload,
                reading_in_force: &reading,
                the_jobs_scene: &scene,
                regular_output_path: b"/ephemeral/smoozy/jobs/j1/frame_####",
                the_stepping,
                the_check_is_written_as_a_picture,
            })
            .unwrap()
        };
        let the_switch = |material: &LaunchMaterial| {
            material
                .entries()
                .iter()
                .filter(|entry| matches!(entry, Entry::TheCheckWrittenAsAPicture { .. }))
                .cloned()
                .collect::<Vec<_>>()
        };
        // **Nowhere but where the reading switched.**
        for version in TheFourVersions::ALL {
            assert!(the_switch(&compose_it(version, false, None)).is_empty(), "{version:?}");
        }
        // **From 5.0, the media setting first; 4.5 has none.**
        assert_eq!(
            the_switch(&compose_it(TheFourVersions::V5_2, true, None)),
            vec![Entry::TheCheckWrittenAsAPicture {
                media: Some(("media_type", "IMAGE")),
                format: "PNG"
            }]
        );
        assert_eq!(
            the_switch(&compose_it(TheFourVersions::V4_5, true, None)),
            vec![Entry::TheCheckWrittenAsAPicture {
                media: None,
                format: "PNG"
            }]
        );
        // **After every pair, before the stepping and the read-back.**
        let stepping = TheStepping::for_the_list(true, &[1]).unwrap();
        let material = compose_it(TheFourVersions::V5_3, true, Some(stepping));
        let entries = material.entries();
        let at = entries
            .iter()
            .position(|entry| matches!(entry, Entry::TheCheckWrittenAsAPicture { .. }))
            .expect("the switch is in the material");
        assert_eq!(entries[at + 1], Entry::Stepping(stepping));
        assert!(matches!(entries[at + 2], Entry::ReadBack { .. }));
        assert!(entries[at + 1..].iter().all(|entry| !matches!(
            entry,
            Entry::Redirect(_) | Entry::Remap(_) | Entry::Settings(_) | Entry::Simplify { .. }
        )));
    }

    /// **The fourth of the four named faults: a frame counted as already
    /// there.** The skip switch is imposed off on an untouched payload, and
    /// on every payload, with no input that could remove it.
    #[test]
    fn the_skip_existing_frames_switch_is_imposed_off_on_every_attempt_unconditionally() {
        let job = a_job();
        let remap = a_remap(&job, false);
        for payload in [an_untouched_payload(), {
            let mut with = an_untouched_payload();
            with.sample_count = None;
            with.simplify.use_simplify = Some(false);
            with
        }] {
            let material = compose_with(&payload, &remap, &job).unwrap();
            let imposed: Vec<&Entry> = material
                .entries()
                .iter()
                .filter(|entry| matches!(entry, Entry::SkipExistingFramesOff { .. }))
                .collect();
            assert_eq!(imposed.len(), 1);
            assert_eq!(
                imposed[0],
                &Entry::SkipExistingFramesOff {
                    property: "scene.render.use_overwrite",
                    overwrite_on: true
                }
            );
        }
    }

    /// **The payload cannot carry the switch.** Its written form is read
    /// for any field naming overwriting or skipping, and there is none.
    #[test]
    fn the_payload_has_no_field_for_the_skip_switch() {
        let written = serde_json::to_string(&an_untouched_payload()).unwrap();
        for word in ["overwrite", "skip", "placeholder"] {
            assert!(!written.to_lowercase().contains(word), "{written}");
        }
    }

    /// **L10a: every settings pair is followed by its own name check, and
    /// the check is on the name and not the value.**
    #[test]
    fn every_settings_pair_and_every_simplify_pair_is_followed_by_its_name_check() {
        let job = a_job();
        let remap = a_remap(&job, false);
        let mut payload = an_untouched_payload();
        payload.resolution_percentage = Some(25);
        payload.simplify.subdivision_render = Some(2);
        payload.simplify.texture_limit_render = Some("1024".to_string());
        let material = compose_with(&payload, &remap, &job).unwrap();
        let entries = material.entries();
        for (at, entry) in entries.iter().enumerate() {
            let property = match entry {
                Entry::Settings(pair) => pair.property,
                Entry::Simplify { property, .. } => property,
                Entry::SkipExistingFramesOff { property, .. } => property,
                _ => continue,
            };
            assert_eq!(
                entries[at + 1],
                Entry::NameCheck { property },
                "{entry:?} is not followed by its own name check"
            );
        }
        assert!(entries
            .iter()
            .any(|entry| matches!(entry, Entry::NameCheck { property } if property.starts_with("scene.cycles."))));
    }

    /// **Item 29 closed on 2026-09-20, so both compose** — a typed count and a
    /// processor override alike, each pair written out from the job's scene
    /// and each followed by its own name check. This test held the loud
    /// refusal until that date.
    ///
    /// **The refusal route is not deleted**: `CannotCompose::ASpellingIsNotYetRead`
    /// still exists and is still what an absence produces, because `Rendering`
    /// L10a's name check stands whatever the names turned out to be and no
    /// name in `Render Settings` C10 has been set on Linux.
    #[test]
    fn a_typed_count_and_a_processor_override_both_compose_now_the_spellings_are_read() {
        let job = a_job();
        let remap = a_remap(&job, false);
        let mut payload = an_untouched_payload();
        payload.sample_count = Some(128);
        let material = compose_with(&payload, &remap, &job).expect("C1's set composes");
        let properties: Vec<&str> = material
            .entries()
            .iter()
            .filter_map(|entry| match entry {
                Entry::Settings(pair) => Some(pair.property),
                _ => None,
            })
            .collect();
        for wanted in [
            "scene.cycles.samples",
            "scene.cycles.use_adaptive_sampling",
            "scene.cycles.time_limit",
            "scene.cycles.use_layer_samples",
            "scene.cycles.use_sample_subset",
        ] {
            assert!(properties.contains(&wanted), "{wanted} is not composed");
        }

        // And the processor override the same way (D3) — the scene's own
        // Cycles device setting, never the compositor's.
        let mut override_ = an_untouched_payload();
        override_.device = Some(DeviceChoice::TheProcessor);
        let material = compose_with(&override_, &remap, &job).expect("D3's pair composes");
        let properties: Vec<&str> = material
            .entries()
            .iter()
            .filter_map(|entry| match entry {
                Entry::Settings(pair) => Some(pair.property),
                _ => None,
            })
            .collect();
        assert!(properties.contains(&"scene.cycles.device"));
        assert!(!properties.contains(&"scene.render.compositor_device"));
    }

    #[test]
    fn a_typed_count_takes_the_users_pick_and_else_the_files_own_offered_engine() {
        let mut payload = an_untouched_payload();
        payload.sample_count = Some(8);
        payload.engine = Some(EngineEntry {
            pick: OfferedEngine::Eevee,
            identifier: None,
        });
        // With the pick, the count pair targets EEVEE's own count.
        let pairs = the_settings_pairs(&payload, &a_reading(), &a_scene("Kitchen"), TheFourVersions::V5_2)
            .expect("every name C1 items 1 to 5 assign against is read");
        assert!(pairs
            .iter()
            .any(|pair| pair.property == "scene.eevee.taa_render_samples"));
        // **`Render Settings` C1 item 3: EEVEE's own time limit is composed from
        // 5.3 and never before**, because L10a would fail an older job on it.
        assert!(!pairs.iter().any(|pair| pair.property == "scene.eevee.time_limit"));
        let from_5_3 = the_settings_pairs(&payload, &a_reading(), &a_scene("Kitchen"), TheFourVersions::V5_3)
            .expect("the name is read");
        assert!(from_5_3.iter().any(|pair| pair.property == "scene.eevee.time_limit"));
        assert!(!pairs
            .iter()
            .any(|pair| pair.property == "scene.cycles.samples"));
        // A file whose engine is neither offered one, and a typed count:
        // nothing to target.
        let mut scene = a_scene("Kitchen");
        scene.engine_as_read = ExactName::as_the_reading_produced_it("HYDRA_STORM");
        payload.engine = None;
        assert_eq!(
            the_settings_pairs(&payload, &a_reading(), &scene, TheFourVersions::V5_2).unwrap_err(),
            CannotCompose::ATypedCountUnderAnEngineThePageDoesNotOffer
        );
    }

    /// **The read-back compares the raw text handed in — the map's pair as
    /// text — and never a finished file name.**
    #[test]
    fn the_read_back_carries_the_maps_raw_text_and_sits_before_the_version_stamp() {
        let job = a_job();
        let remap = a_remap(&job, false);
        let material = compose_with(&an_untouched_payload(), &remap, &job).unwrap();
        let entries = material.entries();
        let read_back = &entries[entries.len() - 2];
        assert_eq!(
            read_back,
            &Entry::ReadBack {
                property: "scene.render.filepath",
                must_equal: b"/ephemeral/smoozy/jobs/j1/frame_####".to_vec()
            }
        );
        assert_eq!(
            entries[entries.len() - 1],
            Entry::ComposedFor {
                version: TheFourVersions::V5_2
            }
        );
    }

    /// **The first of the four named faults: a dependency that did not
    /// re-point.** Every pair of the remap material rides the material as
    /// handed, and a library pair takes its reload.
    #[test]
    fn every_remap_pair_rides_the_material_as_handed_and_a_library_pair_takes_its_reload() {
        let job = a_job();
        let remap = a_remap(&job, true);
        let material = compose_with(&an_untouched_payload(), &remap, &job).unwrap();
        let remapped: Vec<&Pair> = material
            .entries()
            .iter()
            .filter_map(|entry| match entry {
                Entry::Remap(pair) => Some(pair),
                _ => None,
            })
            .collect();
        assert_eq!(remapped.len(), remap.pairs.len());
        for (carried, handed) in remapped.iter().zip(&remap.pairs) {
            assert_eq!(*carried, handed);
        }
        let entries = material.entries();
        let library_at = entries
            .iter()
            .position(|entry| matches!(entry, Entry::Remap(pair) if pair.kind == smoozy_dependencies_machine::check::Kind::Library))
            .unwrap();
        assert_eq!(entries[library_at + 1], Entry::LibraryReloadAfterAssignment);
        let image_at = entries
            .iter()
            .position(|entry| matches!(entry, Entry::Remap(pair) if pair.kind == smoozy_dependencies_machine::check::Kind::Image))
            .unwrap();
        assert_ne!(entries[image_at + 1], Entry::LibraryReloadAfterAssignment);
    }

    /// **L11: no entry catches.** The crate's sources are read for the
    /// words a catch would be written with, in code rather than comments.
    #[test]
    fn no_line_of_this_crate_writes_a_catch_into_the_material() {
        for (path, text) in crate::markers::tests::every_source_of_this_crate() {
            for line in text.lines() {
                let trimmed = line.trim_start();
                if trimmed.starts_with("//") || trimmed.starts_with("///") {
                    continue;
                }
                // Joined from halves, so this test's own line is not a hit.
                for word in [
                    ["Catch", "Failure"].concat(),
                    ["Try", "Except"].concat(),
                    ["Catches", "A"].concat(),
                ] {
                    assert!(!trimmed.contains(&word), "{path}: {line}");
                }
            }
        }
        const _: () = assert!(NO_FORMAT_AND_NO_MEDIA_TYPE_IS_EVER_SET);
        const _: () = assert!(THE_REMAP_IS_IN_FORCE_INSIDE_THE_SAME_MATERIAL_AFTER_THE_LOAD);
        const _: () =
            assert!(THE_PLACEMENT_IS_CONFIRMED_BY_READING_IT_BACK_UNDER_THE_RENDERS_LOGIN);
    }

    #[test]
    fn a_pin_rides_as_three_steps_and_is_followed_by_the_switchs_name_check() {
        use smoozy_render_settings_machine::names::AnchorState;
        use smoozy_render_settings_machine::view_layers::PinEntry;
        let job = a_job();
        let remap = a_remap(&job, false);
        let mut payload = an_untouched_payload();
        payload.view_layer_pins.push(PinEntry {
            layer: ExactName::as_the_reading_produced_it("ViewLayer"),
            anchor: AnchorState::Anchored {
                the_reading_it_came_from: a_reading().identity(),
            },
            on: false,
        });
        let material = compose_with(&payload, &remap, &job).unwrap();
        let entries = material.entries();
        let at = entries
            .iter()
            .position(|entry| matches!(entry, Entry::Pin { .. }))
            .unwrap();
        assert_eq!(
            entries[at],
            Entry::Pin {
                layer: b"ViewLayer".to_vec(),
                on: false
            }
        );
        // **The check is against the layer the pin was assigned on, by that
        // layer's own bytes, and its property is the switch's bare name.**
        // It read `scene.view_layers["…"].use` until 2026-09-05 — the display
        // form, placeholder and all — which no scene carries, so every pinned
        // job raised at the walk and failed before the render instruction.
        assert_eq!(
            entries[at + 1],
            Entry::PinNameCheck {
                layer: b"ViewLayer".to_vec(),
                property: "use"
            }
        );
        let _ = the_value_of(&the_percentage_pair(50));
    }

    /// **No property this material hands the acting half is one the walk
    /// cannot resolve** — the class guard for the round's most serious
    /// finding, held over every family rather than over the one that was
    /// found.
    ///
    /// The walk splits a property on the full stop and asks for each piece by
    /// name, so a piece carrying a placeholder, a bracket or a quotation mark
    /// names an attribute nothing carries and raises — which L8a turns into a
    /// paid Blender start with nothing written, on every attempt of that job.
    /// **Two written-out forms `Render Settings` publishes are display forms
    /// of exactly that shape**, the view layer's sample count and its switch,
    /// and nothing there says so; this is what stops one reaching the walk
    /// from here, whichever family a later composition takes it through.
    #[test]
    fn no_property_this_material_carries_is_one_the_walk_cannot_resolve() {
        use smoozy_render_settings_machine::names::AnchorState;
        use smoozy_render_settings_machine::view_layers::PinEntry;
        let job = a_job();
        let remap = a_remap(&job, true);
        let mut payload = an_untouched_payload();
        payload.resolution_percentage = Some(50);
        payload.simplify.use_simplify = Some(true);
        payload.simplify.subdivision_render = Some(2);
        payload.simplify.child_particles_render = Some(0.5);
        payload.simplify.texture_limit_render = Some("1024".to_string());
        payload.view_layer_pins.push(PinEntry {
            layer: ExactName::as_the_reading_produced_it("ViewLayer"),
            anchor: AnchorState::Anchored {
                the_reading_it_came_from: a_reading().identity(),
            },
            on: true,
        });
        let material = compose_with(&payload, &remap, &job).unwrap();

        let resolvable = |property: &str, at: &str| {
            assert!(!property.is_empty(), "{at}: an empty property");
            for piece in property.split('.') {
                assert!(
                    !piece.is_empty()
                        && piece
                            .chars()
                            .all(|c| c.is_ascii_alphanumeric() || c == '_'),
                    "{at}: `{property}` has a piece the walk cannot ask for by name"
                );
            }
        };
        let mut seen = 0;
        for entry in material.entries() {
            match entry {
                Entry::Settings(pair) => resolvable(pair.property, "a settings pair"),
                Entry::NameCheck { property } => resolvable(property, "a name check"),
                Entry::PinNameCheck { property, .. } => resolvable(property, "a pin's check"),
                Entry::SkipExistingFramesOff { property, .. } => {
                    resolvable(property, "the skip switch")
                }
                Entry::Simplify { property, .. } => resolvable(property, "a Simplify pair"),
                Entry::ReadBack { property, .. } => resolvable(property, "the read-back"),
                // **The redirect carries no property path at all**: the node
                // travels as its own key and the folder's setting is one
                // name, which is item 9's answer rather than a stricter walk.
                // An added place's settings are plain names off each carrier.
                Entry::Redirect(assignment) => match the_redirects_holder(assignment).unwrap() {
                    TheRedirectsHolder::AFileOutputNode { property, .. } => {
                        resolvable(property, "a redirect assignment")
                    }
                    TheRedirectsHolder::AnAddedWritePlace { settings, .. } => {
                        for one in settings {
                            resolvable(one.setting, "an added write place's setting");
                        }
                    }
                },
                _ => continue,
            }
            seen += 1;
        }
        assert!(seen >= 12, "only {seen} properties were read");
    }

    /// **The added write places the material sets compose, each onto its own
    /// named settings, and the two `Job files and outputs` D1a sets as the
    /// file loads are not in it at all** (since 2026-10-02: the Replay fluid
    /// cache folder and the geometry-nodes bake folder are L8b's handler's) — and a node assignment that arrives without its folder's
    /// property is still refused, loudly, naming the destination, and never
    /// passed over, which is the case `Job files and outputs` records as a
    /// finished render reporting success with 250 files written beside the
    /// artist's `.blend`.
    ///
    /// **Until 2026-09-26 the first half of this test was the refusal**, and a
    /// factory cube carries the bake output path, so on step 10c's sixth pass
    /// on Vast every job ended here on either device.
    #[test]
    fn the_added_write_places_compose_and_a_redirect_with_no_setting_is_still_refused() {
        use smoozy_job_files_machine::reading::{
            AnAddedWritePlace, DestinationKind, WhereItComesFrom, EVERY_ADDED_WRITE_PLACE,
        };
        let job = a_job();
        let remap = a_remap(&job, false);
        let payload = an_untouched_payload();
        let reading = a_reading();
        let scene = a_scene("Kitchen");
        let redirects: Vec<RedirectAssignment> = EVERY_ADDED_WRITE_PLACE
            .into_iter()
            .map(|place| {
                let mut added = a_redirect_assignment("File Output");
                added.destination.kind = DestinationKind::AnAddedWritePlace(place);
                added.destination.origin = WhereItComesFrom::AnAddedWritePlaceNamed(place);
                added.property = None;
                added.further_values =
                    smoozy_job_files_machine::mapping::the_further_values_for(place);
                added
            })
            .collect();
        let material = compose(MaterialInputs {
            job: &job,
            version: TheFourVersions::V5_2,
            redirect_pairs: &redirects,
            remap: &remap,
            payload: &payload,
            reading_in_force: &reading,
            the_jobs_scene: &scene,
            regular_output_path: b"/ephemeral/smoozy/jobs/j1/frame_####",
            the_stepping: None,
            the_check_is_written_as_a_picture: false,
        })
        .expect("every added write place has its settings named");
        let set: Vec<AnAddedWritePlace> = material
            .entries()
            .iter()
            .filter_map(|entry| match entry {
                Entry::Redirect(assignment) => match the_redirects_holder(assignment).unwrap() {
                    TheRedirectsHolder::AnAddedWritePlace { place, .. } => Some(place),
                    TheRedirectsHolder::AFileOutputNode { .. } => None,
                },
                _ => None,
            })
            .collect();
        assert_eq!(
            set,
            EVERY_ADDED_WRITE_PLACE
                .into_iter()
                .filter(|place| !smoozy_job_files_machine::redirect::is_set_as_the_file_loads(*place))
                .collect::<Vec<_>>()
        );
        assert_eq!(set.len(), 3);
        assert!(!set.contains(&AnAddedWritePlace::TheGeometryNodesBakeFolder));
        assert!(!set.contains(&AnAddedWritePlace::TheReplayFluidCacheFolder));

        // **A node assignment with no property is refused and named** in the
        // artist's own terms — step 10c's fifth pass on Vast, fault 5.
        let mut bare = a_redirect_assignment("File Output");
        bare.property = None;
        let CannotCompose::ARedirectWhoseSettingIsNotOnRecord { destination } =
            the_redirects_holder(&bare).unwrap_err()
        else {
            unreachable!()
        };
        assert!(destination.contains("File Output"), "{destination}");
        assert!(!destination.contains("FileOutputNode"), "{destination}");
        // **The regular output path is `Job files and outputs` D10's own
        // argument and the material never sets it**, so one handed in as a
        // redirect is refused rather than assigned a second time.
        let mut regular = a_redirect_assignment("File Output");
        regular.destination.kind = DestinationKind::TheRegularOutputPath;
        assert!(the_redirects_holder(&regular).is_err());
    }
}

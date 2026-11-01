//! **§1.4 performed — L10's typed document rendered into the text the
//! render's Blender runs, L11's absence of failure handling, and L12's
//! placement confirmed by reading it back under the render's login.**
//!
//! # What is rendered here and what is not
//!
//! **[`crate::material`] decides what the material carries and in what
//! order.** Its [`crate::material::Entry`] list is L10's order of
//! application, composed once and never rewritten, and **this module writes
//! that list out and nothing else**: it adds no entry, drops none, and
//! re-orders none. A reader checking that the text does what the plan says
//! reads the list, not this file.
//!
//! **The remap half's text is `Dependency Upload & Remap`'s own** and is
//! taken whole from that crate's `the_material_python`. §11 of that document
//! hands over the application rules — per-setting assignment, the library
//! reload after assignment, matching on the pair's **match text** under
//! `Dependency Upload & Remap` M4a's separator rule and never on the
//! expected bytes as read, with not-applied reporting, every pair's outcome
//! in the marker — and it has written them.
//! **A second rendering of those rules here would be a second thing to keep
//! true in the one place a mistake is a finished render pointing at the
//! artist's own machine.** So the fragment is emitted once, where the first
//! remap entry sits in the order, and the remap entries after it are that
//! fragment's own work.
//!
//! # Three rules the text keeps, each a clause rather than a habit
//!
//! - **L11: no failure handling of its own.** *"A pair whose assignment
//!   raises ends the run before the render instruction; nothing catches it,
//!   because a caught failure is a successful exit status over a render that
//!   did not do what was asked."* **There is no `try` in the text**, and
//!   [`crate::material`]'s own guard already reads every source of this
//!   crate for the words one would be written with.
//! - **L10a's name check is written for every settings pair rather than for
//!   the one version and the one group where a wrong name is swallowed.** In
//!   4.5 a name Cycles' settings do not carry is accepted with no error of
//!   any kind and the value is gone the instant it is read back, so the
//!   assignment writes nothing, prints nothing and the render goes on to
//!   exit 0 at the file's own count. **The check is on the name and never on
//!   the value**, and its failure is the material's own raise, which L8a's
//!   argument turns into a run ending before the render instruction with
//!   nothing written.
//! - **`Render Settings` C8: every imposition is assigned against the job's
//!   scene**, the manifest's named scene or the file's showing scene where
//!   it names none, **and never against any other scene of the file**. The
//!   text binds one scene, once, by the reading's own bytes, and every
//!   property path is resolved against it.
//!
//! # Nothing of the job's is spliced into an expression
//!
//! **Every value the text needs travels as data rather than as text written
//! into code**: the scene's name, each property path, each value and the
//! read-back's expected bytes go in as one JSON document inside a raw
//! string, exactly as the remap fragment already does. A path that is not
//! text would break a spliced literal, and would break it silently.
//!
//! # L12, and what it does and does not close
//!
//! **The placement is confirmed by reading it back under the login the
//! render runs as**, which closes the one shape of the unreadable-script
//! hazard that is Smoozy's own doing. **It is not offered as making the
//! failure loud**: `Job files and outputs` D17 records a run whose launch
//! script sat in an unreadable folder printing a permission refusal,
//! rendering the factory scene anyway, writing a file and exiting 0, and
//! `Job files and outputs` D18 forbids any architecture document resting on
//! a launch script failing loudly. **The hazard itself is not closed and
//! nothing here claims it is** (§9 item 1's neighbour, `Job files and
//! outputs` §14 item 8).
//!
//! **Markers used in this file that belong to another document, each with
//! the document it belongs to.** `Job files and outputs` D16, D17, D18, D19.
//! `Dependency Upload & Remap` M8. `Render Settings` C8, C9.
//!
//! **The `Render Settings` V, the `Job files and outputs` D and the
//! `Rented-server runtime` X clauses this file cites are deliberately not on
//! that list and are written out at every site instead**, because this plan
//! defines a clause of each name and a bare one would resolve to a real and
//! wrong clause.

use std::io::Read;

use serde::Serialize;
use smoozy_dependencies_machine::remap::RemapMaterial;
use smoozy_job_files_machine::mapping::FurtherValueIs;
use smoozy_job_files_machine::reading_script::THE_CHECKS_MARK_IN_BLENDER;
use smoozy_job_files_machine::reading::{
    AnAddedWritePlace, Destination, TheVersionInHand, WhichScene,
};
use smoozy_render_settings_machine::reading::SceneReading;
use smoozy_render_settings_machine::samples::PairValue;
use smoozy_render_settings_machine::view_layers::{the_route_to_the_curve, RouteToTheCurve};
use smoozy_server_setup_machine::blender::Version;

use crate::material::{Entry, LaunchMaterial, SimplifyValue, TheRedirectsHolder};

/// **The head of every line the material writes onto `Job files and
/// outputs` D16's marker.**
///
/// A constant, so a reader of a captured stream can find the lines with
/// nobody connected — the same discipline `Dependency Upload & Remap`'s own
/// per-pair head already keeps, and the same reason.
pub const THE_MARKER_HEAD: &str = "SMOOZY-MATERIAL ";

/// One of `Job files and outputs` P10's further values, as the text receives
/// it. **Its setting is that document's own vocabulary and is carried, never
/// composed here.**
#[derive(Debug, Clone, PartialEq, Serialize)]
struct AFurtherValue {
    setting: String,
    value: serde_json::Value,
}

/// One setting of an added write place, as the text receives it: which
/// carriers the text walks to, and the setting's own name on each.
#[derive(Debug, Clone, PartialEq, Serialize)]
struct AnAddedPlacesSetting {
    carried_by: smoozy_job_files_machine::redirect::WhatCarriesTheSetting,
    setting: &'static str,
}

/// **One recorded location of an added write place, and its folder under the
/// job's folder** — `Job files and outputs` R4 item 2 and `Job files and
/// outputs` P10, as the text receives them. `recorded` is absent for the
/// folder the carriers that record no location take (`Job files and outputs`
/// P7a).
#[derive(Debug, Clone, PartialEq, Serialize)]
struct AFolderForARecordedLocation {
    recorded: Option<Vec<u8>>,
    folder: Vec<u8>,
}

/// One instruction of the text, as the text receives it.
///
/// **The list is the rendering of [`crate::material::Entry`] and carries no
/// judgement of its own.** Every variant is one entry of that list.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "do")]
enum AnInstruction {
    /// `Job files and outputs` D16's opening, naming the job identity.
    Marker { job: String },
    /// **L10b: the job's scene stepped frame by frame** from its own start —
    /// or `lowest` where that is earlier — to `highest`, and the count printed
    /// on the marker.
    Step { lowest: i64, highest: i64 },
    /// **L10c: the job's scene written as a still picture** — the media
    /// setting first where the version has one, then the format, then a read
    /// that raises where the scene still writes a video.
    WriteAsAPicture {
        media_property: Option<String>,
        media: Option<String>,
        format: String,
    },
    /// **`Job files and outputs` P18a: the check's names marked** by that
    /// crate's own text, with the names it is set by for this version, the
    /// regular output path and 4.5's multilayer folder left to the map.
    MarkTheCheck {
        names: serde_json::Value,
        mark: String,
    },
    /// **L8b's check: whether the handler the script before the file
    /// registered ran to its end**, read from the record it keeps under
    /// `record`, and a raise where it did not.
    LoadTimeHandlerFinished { record: &'static str },
    /// A redirect assignment — each destination's own setting, individually,
    /// **with the object it hangs off named rather than spelled into a
    /// path**, the destination's own scene where that is not the job's, and
    /// the further values the place needs applied with the folder.
    Redirect {
        /// The destination's own scene, by that scene's own bytes, where the
        /// destination was reached through a sequencer strip. Absent for the
        /// job's own scene, which is the ordinary case.
        ///
        /// **`Job files and outputs` R4 item 3 is why this is here**: a
        /// scene reached through a strip contributes its own destinations,
        /// and assigning one against the job's scene sets a setting on the
        /// wrong scene or fails to find it.
        scene: Option<Vec<u8>>,
        /// **The scene's compositing tree, by the name this version gives
        /// it** — `node_tree` in 4.5 and `compositing_node_group` from 5.0,
        /// where a scene has no `node_tree` at all. Read off `Job files and
        /// outputs — machine`'s own cited pair, never spelled here.
        ///
        /// **Until 2026-09-26 the text asked every version for `node_tree`**,
        /// so a job whose file carried a file output node would have raised
        /// here on 5.0, 5.1 and 5.2 and ended before the render with nothing
        /// written. No pass had reached it: every job before then ended
        /// earlier.
        tree: String,
        /// The file output node the setting hangs off, by its own name.
        /// **Looked up in the collection by key** — never reached by asking
        /// for an attribute called `nodes["…"]`, which is what a walk over a
        /// dotted path does and what no scene carries.
        node: Vec<u8>,
        /// The folder's own property, per version, as `Job files and
        /// outputs` D8 resolved it. Nothing here composes it.
        property: String,
        /// The folder, as bytes, already carrying that document's trailing
        /// separator where `Job files and outputs` D8 requires one.
        folder: Vec<u8>,
        /// **`Job files and outputs` P10's further values, applied with the
        /// folder and on the same object.** Setting the folder alone is that
        /// document's own observed silent no-op.
        further_values: Vec<AFurtherValue>,
    },
    /// **One of `Job files and outputs` D20's four added write places**: the
    /// folder written to every setting that holds it, on every object in the
    /// file that carries that setting, with the place's further values on
    /// the same objects.
    ///
    /// **A file that carries none of it has nothing set and nothing is
    /// written for it** — a factory cube has no simulation, no nodes modifier
    /// and no sequencer, and carries the bake settings alone. **No carrier is
    /// ever made**: a scene with no sequence editor is not given one.
    RedirectAnAddedWritePlace {
        /// The place, in the artist's own terms, for the marker.
        place: &'static str,
        /// Every setting that holds the place's folder, each with what carries
        /// it — `Job files and outputs`' own `the_settings_of`, cited there.
        settings: Vec<AnAddedPlacesSetting>,
        /// **Each recorded location's folder, exactly as the map gave it** —
        /// one entry per destination the map holds for this place
        /// (`Job files and outputs` R4 item 2, corrected 2026-10-02), the
        /// one with no recorded location being the folder for every carrier
        /// that records none. **Each carrier is set to the folder of its own
        /// recorded location**, read off the carrier as the file has it.
        ///
        /// **Until 2026-10-02 there was one folder for the whole place**, so a
        /// file whose geometry-nodes bakes sat in four folders had every
        /// carrier set to one of them, and the three other sent folders had no
        /// place to be copied to: every attempt ended before the render.
        folders: Vec<AFolderForARecordedLocation>,
        /// **Whether the folder is made before the render starts** —
        /// `Job files and outputs` D21b-i's *"the folder is created before the
        /// render is started, not left to Blender"*, which is that clause's
        /// for the point cache and no other place's. Made only where something
        /// in the file carries the setting.
        the_folder_is_made_first: bool,
        /// **`Job files and outputs` P10's further values that are switches,
        /// written on each carrier before its folder** — `Job files and outputs` D21b-i's order,
        /// *external* on, the folder, the name. For the point cache that is
        /// `use_external` alone; the folder follows, and the cache's own stem
        /// last.
        ///
        /// **The order was proved at the desk, 2026-10-02**, on a baked cloth
        /// whose 48 files sat beside the `.blend`: in this order no file
        /// moved at any step, in 4.5.12 and 5.2.2. (The folder first did not
        /// move a file either; the order is the clause's.)
        switched_on_before_the_folder: Vec<AFurtherValue>,
        /// `Job files and outputs` P10's other further values, on the same
        /// carriers, written after the folder. None for any place since
        /// 2026-10-02, when the point cache's name became each cache's own.
        further_values: Vec<AFurtherValue>,
        /// **`Job files and outputs` D21b-i, corrected 2026-10-02: the
        /// setting each carrier's own file stem is written to, last** — the
        /// point cache's `name`, read off `Job files and outputs — machine`'s
        /// own `the_stem_setting_for` and never spelled here. **The stem is
        /// read off the carrier before anything on it is written**: its name
        /// where it has one, else the one Blender gives it from its object.
        the_caches_own_stem_goes_to: Option<&'static str>,
    },
    /// A settings pair, a Simplify pair, the skip-existing-frames switch and
    /// a pin's assignment are one shape: a property path against the job's
    /// scene, and a value.
    Assign {
        property: String,
        value: serde_json::Value,
        /// What the marker calls it, so a reader can tell a Simplify pair
        /// from a sample count without reading the property path.
        family: &'static str,
    },
    /// A pin's three steps, which are `Render Settings` V4 to `Render
    /// Settings` V7's and are
    /// named rather than folded into an assignment.
    Pin {
        layer: Vec<u8>,
        on: bool,
        /// **`Render Settings` V9's route to the curve, per version**, read
        /// off that crate's own `the_route_to_the_curve` and never decided
        /// here. In 4.5 the action's curves hang directly off the action; in
        /// 5.0, 5.1 and 5.2 that route does not exist — asking for it raises
        /// — and the curve is reached by walking the action's layers, then
        /// its strips, then its channel bags, then its curves.
        the_curve_is_reached_by_the_walk: bool,
    },
    /// L10a's check: a read of the pair's own name against the object it was
    /// assigned on. **The value is not compared.**
    NameCheck { property: String },
    /// L10a's check for a pin, which resolves the layer by its own bytes the
    /// way the pin's assignment does and reads the switch's name off **that**
    /// object. **No path is composed and nothing is split**, which is what
    /// stops a written-out display form ever reaching the walk.
    PinNameCheck { layer: Vec<u8>, property: String },
    /// `Job files and outputs` D10's read-back, against the raw text handed in.
    ReadBack {
        property: String,
        must_equal: Vec<u8>,
    },
}

/// **The location the file records for an added write place's destination**,
/// as the reading carried it, or nothing for the carriers that record none.
fn the_recorded_location_of(
    assignment: &smoozy_job_files_machine::redirect::AnAssignment,
) -> Option<Vec<u8>> {
    assignment
        .destination
        .recorded_location
        .as_ref()
        .map(|location| location.as_bytes().to_vec())
}

/// **The text, composed from the typed document and from nothing else.**
///
/// `the_jobs_scene` is `Render Settings` C8's scene, by the reading's own
/// bytes; `remap` is the document the manifest names, read at §2 P1 step 2.
pub fn the_materials_text(
    material: &LaunchMaterial,
    the_manifests_version: Version,
    the_jobs_scene: &SceneReading,
    remap: &RemapMaterial,
) -> String {
    let mut instructions: Vec<AnInstruction> = Vec::new();
    // **Where each added write place's one instruction sits**, so a place's
    // second destination joins its table rather than writing the place again
    // (`Job files and outputs` R4 item 2).
    let mut the_instruction_of_each_place: Vec<(AnAddedWritePlace, usize)> = Vec::new();
    let mut where_the_remap_fragment_goes = None;
    for entry in material.entries() {
        match entry {
            Entry::MarkerOpening { job } => {
                instructions.push(AnInstruction::Marker { job: job.clone() })
            }
            Entry::TheLoadTimeHandlerFinished => {
                instructions.push(AnInstruction::LoadTimeHandlerFinished {
                    record: smoozy_job_files_machine::redirect::THE_LOAD_TIME_RECORD,
                })
            }
            Entry::Redirect(assignment) => {
                // **[`crate::material::compose`] is the only constructor of a
                // material and runs this same gate over every assignment**,
                // so an entry that reached here has somewhere to be set.
                let holder = crate::material::the_redirects_holder(assignment)
                    .expect("the material's own constructor refuses any other assignment");
                let further_values: Vec<AFurtherValue> = assignment
                    .further_values
                    .iter()
                    .map(|further| AFurtherValue {
                        setting: further.setting.clone(),
                        value: match &further.value {
                            FurtherValueIs::On => serde_json::Value::Bool(true),
                            FurtherValueIs::NonEmptyText(text) => {
                                serde_json::Value::from(text.clone())
                            }
                        },
                    })
                    .collect();
                if let TheRedirectsHolder::AnAddedWritePlace { place, .. } = &holder {
                    if let Some((_, at)) =
                        the_instruction_of_each_place.iter().find(|(seen, _)| seen == place)
                    {
                        if let AnInstruction::RedirectAnAddedWritePlace { folders, .. } =
                            &mut instructions[*at]
                        {
                            folders.push(AFolderForARecordedLocation {
                                recorded: the_recorded_location_of(assignment),
                                folder: assignment.folder.clone(),
                            });
                        }
                        continue;
                    }
                    the_instruction_of_each_place.push((*place, instructions.len()));
                }
                instructions.push(match holder {
                    TheRedirectsHolder::AFileOutputNode { node, property } => {
                        AnInstruction::Redirect {
                            scene: the_destinations_own_scene(&assignment.destination),
                            tree: the_scenes_compositing_tree(the_manifests_version).to_string(),
                            node: node.to_vec(),
                            property: property.to_string(),
                            folder: assignment.folder.clone(),
                            further_values,
                        }
                    }
                    TheRedirectsHolder::AnAddedWritePlace { place, settings } => {
                        // **`Job files and outputs` D21b-i's order: a switch
                        // before the folder, a text after it** — *external* on,
                        // the folder, the name.
                        let (switched_on_before_the_folder, further_values): (Vec<_>, Vec<_>) =
                            further_values
                                .into_iter()
                                .partition(|further| further.value == serde_json::Value::Bool(true));
                        AnInstruction::RedirectAnAddedWritePlace {
                            place: place.in_words(),
                            settings: settings
                                .iter()
                                .map(|one| AnAddedPlacesSetting {
                                    carried_by: one.carried_by,
                                    setting: one.setting,
                                })
                                .collect(),
                            folders: vec![AFolderForARecordedLocation {
                                recorded: the_recorded_location_of(assignment),
                                folder: assignment.folder.clone(),
                            }],
                            the_folder_is_made_first: place == AnAddedWritePlace::ThePointCache,
                            switched_on_before_the_folder,
                            further_values,
                            the_caches_own_stem_goes_to:
                                smoozy_job_files_machine::mapping::the_stem_setting_for(place),
                        }
                    }
                })
            }
            // **The remap's rules are that crate's and its fragment carries
            // every pair**, so the position of the first remap entry in
            // L10's order is where the fragment sits, and the entries after
            // it are the fragment's own work rather than a second route.
            Entry::Remap(_) | Entry::LibraryReloadAfterAssignment => {
                where_the_remap_fragment_goes.get_or_insert(instructions.len());
            }
            Entry::Settings(pair) => instructions.push(AnInstruction::Assign {
                property: pair.property.to_string(),
                value: as_json(&pair.value),
                family: "settings",
            }),
            Entry::Pin { layer, on } => instructions.push(AnInstruction::Pin {
                layer: layer.clone(),
                on: *on,
                // **`Render Settings` V9's route, read off that crate's
                // own function and never decided here.**
                the_curve_is_reached_by_the_walk: matches!(
                    the_route_to_the_curve(the_manifests_version),
                    RouteToTheCurve::ThroughLayersStripsChannelBagsAndCurves
                ),
            }),
            Entry::NameCheck { property } => instructions.push(AnInstruction::NameCheck {
                property: property.to_string(),
            }),
            Entry::PinNameCheck { layer, property } => {
                instructions.push(AnInstruction::PinNameCheck {
                    layer: layer.clone(),
                    property: property.to_string(),
                })
            }
            Entry::SkipExistingFramesOff {
                property,
                overwrite_on,
            } => instructions.push(AnInstruction::Assign {
                property: property.to_string(),
                value: serde_json::Value::Bool(*overwrite_on),
                family: "skip-existing-frames",
            }),
            Entry::Simplify { property, value } => instructions.push(AnInstruction::Assign {
                property: property.to_string(),
                value: simplify_as_json(value),
                family: "simplify",
            }),
            // The outcomes are appended by the text as each instruction is
            // carried out, so there is nothing to render for this entry: it
            // is the reporting the rendering already does.
            Entry::MarkerOutcomesAppended => {}
            Entry::TheCheckWrittenAsAPicture { media, format } => {
                instructions.push(AnInstruction::WriteAsAPicture {
                    media_property: media.map(|(property, _)| property.to_string()),
                    media: media.map(|(_, value)| value.to_string()),
                    format: format.to_string(),
                })
            }
            Entry::TheChecksNamesMarked { names, mark } => {
                instructions.push(AnInstruction::MarkTheCheck {
                    names: names.clone(),
                    mark: mark.to_string(),
                })
            }
            Entry::Stepping(stepping) => instructions.push(AnInstruction::Step {
                lowest: stepping.the_lists_lowest,
                highest: stepping.the_lists_highest,
            }),
            Entry::ReadBack {
                property,
                must_equal,
            } => instructions.push(AnInstruction::ReadBack {
                property: property.to_string(),
                must_equal: must_equal.clone(),
            }),
            // The version is the material's own record and `Job files and
            // outputs` D7's confirmation
            // is L14's act before the line is composed, not a thing the text
            // does.
            Entry::ComposedFor { .. } => {}
        }
    }

    let document = serde_json::json!({
        "scene": the_jobs_scene.name.as_text(),
        "instructions": instructions,
    });
    let rendered = serde_json::to_string(&document)
        .expect("the document is composed of values that always render");
    // The one thing that could end the raw string is a run of three single
    // quotes, which cannot appear in JSON output — its only string delimiter
    // is the double quote.
    debug_assert!(!rendered.contains("'''"));

    let mut text = format!(
        "{THE_HEAD}{THE_SHARED_WALK}{THE_REST_OF_THE_HEAD}{THE_CHECKS_MARK_IN_BLENDER}\nSMOOZY = json.loads(r'''{rendered}''')\n"
    );
    match where_the_remap_fragment_goes {
        Some(at) => {
            text.push_str(&format!("{THE_BODY}\nsmoozy_carry_out(SMOOZY, 0, {at})\n"));
            text.push_str(&smoozy_dependencies_machine::material::the_material_python(
                remap,
            ));
            text.push_str(&format!(
                "\nsmoozy_carry_out(SMOOZY, {at}, len(SMOOZY['instructions']))\n"
            ));
        }
        None => {
            text.push_str(&format!(
                "{THE_BODY}\nsmoozy_carry_out(SMOOZY, 0, len(SMOOZY['instructions']))\n"
            ));
        }
    }
    text
}

/// **The scene's compositing tree, by the name the manifest's version gives
/// it** — `Job files and outputs — machine`'s own pair, cited there to the
/// property-name lookup of 2026-09-20, and never spelled in this crate.
fn the_scenes_compositing_tree(version: Version) -> &'static str {
    smoozy_job_files_machine::reading_script::THE_SCENES_COMPOSITING_TREE
        .for_version(TheVersionInHand::of(version.major as u16, version.minor as u16))
        .ok()
        .flatten()
        .expect("both sides of the boundary are cited, so every version has a name")
}

/// **The scene a redirect assignment's destination belongs to, where it is
/// not the job's own** (`Job files and outputs` R4 item 3).
///
/// A scene reached through a sequencer strip contributes its own
/// destinations, so its assignments are made on **that** scene. `None` is the
/// job's own scene, which `Render Settings` C8 already binds for everything
/// else the material sets.
fn the_destinations_own_scene(destination: &Destination) -> Option<Vec<u8>> {
    match &destination.scene {
        WhichScene::TheJobsOwnScene => None,
        WhichScene::ReachedThroughASequencerStrip { scene, .. } => Some(scene.as_bytes().to_vec()),
    }
}

fn as_json(value: &PairValue) -> serde_json::Value {
    match value {
        PairValue::Number(number) => serde_json::Value::from(*number),
        PairValue::Identifier(text) => serde_json::Value::from(*text),
        PairValue::Switch(on) => serde_json::Value::Bool(*on),
    }
}

fn simplify_as_json(value: &SimplifyValue) -> serde_json::Value {
    match value {
        SimplifyValue::Switch(on) => serde_json::Value::Bool(*on),
        SimplifyValue::WholeNumber(number) => serde_json::Value::from(*number),
        SimplifyValue::Fraction(fraction) => serde_json::Value::from(*fraction),
        SimplifyValue::Identifier(text) => serde_json::Value::from(text.clone()),
    }
}

/// The part of the text that is the same for every attempt.
///
/// **The property paths are resolved against the job's scene and against
/// nothing else** (`Render Settings` C8), by walking the path's own
/// components off the scene object. The first component is the scene, which
/// is why nothing here has to know what any property is called.
const THE_HEAD: &str = r#"
import bpy, json, os, sys

# **L11: no failure handling of its own.** There is no `try` below. A pair
# whose assignment raises, a name the object does not carry, and a read-back
# that does not match each raise, and the raise is what L8a's
# failure-to-exit-status argument turns into a run ending before the render
# instruction with nothing written.

SMOOZY_MARKER_HEAD = "SMOOZY-MATERIAL "

def smoozy_say(line):
    print(SMOOZY_MARKER_HEAD + json.dumps(line))

def smoozy_scene(document):
    # **`Render Settings` C8**: the job's scene, by the reading's own bytes,
    # and never another scene of the file.
    return bpy.data.scenes[document["scene"]]

"#;

/// **The walk to what carries an added write place's setting, and the lookup
/// of a carrier's own folder** — one text in two scripts: the material, and
/// L8b's script before the file ([`crate::acting::load_time`]), so the
/// handler and the material can never walk to two different sets of
/// carriers. Taken out of [`THE_HEAD`] on 2026-10-02, simulation fix piece 4.
pub(crate) const THE_SHARED_WALK: &str = r#"
def smoozy_off(holder, dotted_name):
    # **Every piece of a plain dotted name, off an object already in hand.**
    # A piece is asked for by name and nothing else: a piece that reached into
    # a collection would be asked for as an attribute of that literal spelling
    # and no object carries one. Nothing composed on the desktop side ever
    # sends such a piece — a name that has to be looked up by key travels as
    # the key, in its own field.
    parts = dotted_name.split(".")
    for part in parts[:-1]:
        holder = getattr(holder, part)
    return holder, parts[-1]

def smoozy_carriers(scene, carried_by):
    # **What carries one setting of an added write place, every one of it in
    # the file** (`Job files and outputs` D20, `Job files and outputs` D21b). Every name below was read
    # off four running Blenders, 4.5.12, 5.0.1, 5.1.2 and 5.2.2, and is the same
    # in all four: the lookup of 2026-09-26, recorded at
    # `Audits/Build/Step 10c/The card make and the bake folders — what was
    # done.md`. **Nothing is made**: a scene with no sequence editor is not
    # given one, and an object with no modifier carries nothing.
    if carried_by == "TheScenesBakeSettings":
        return [scene.render.bake]
    if carried_by == "EverythingCarryingAPointCache":
        # The kinds that hold a point cache, and nothing else in any of the
        # four: a cloth or soft body modifier, a dynamic paint canvas's
        # surfaces, and a particle system — **and a scene's rigid body world,
        # which is never set** (`Job files and outputs` D21b-i, corrected
        # 2026-10-02): switching its disk cache on ended Blender with exit 139
        # in 4.5.12 and 5.2.2, so it is not walked to here at all.
        #
        # **Only a cache whose disk cache the file already has on is set**
        # (the same clause). A cache the file keeps in memory writes nothing
        # to disk, so it is not a place the job writes, and it is left exactly
        # as the file has it — turning its disk cache on dropped a cloth's
        # in-file bake. The switch itself is only read here, never written:
        # writing it where it is already on deletes every file the cache holds.
        found = []
        for thing in bpy.data.objects:
            for modifier in thing.modifiers:
                if hasattr(modifier, "point_cache"):
                    found.append(modifier)
                canvas = getattr(modifier, "canvas_settings", None)
                if canvas is not None:
                    found.extend(canvas.canvas_surfaces)
            found.extend(thing.particle_systems)
        return [each for each in found if each.point_cache.use_disk_cache]
    if carried_by == "EveryGeometryNodesModifier":
        return [
            modifier
            for thing in bpy.data.objects
            for modifier in thing.modifiers
            if hasattr(modifier, "bake_directory")
        ]
    if carried_by == "EveryGeometryNodesBake":
        return [
            bake
            for thing in bpy.data.objects
            for modifier in thing.modifiers
            if hasattr(modifier, "bake_directory")
            for bake in modifier.bakes
        ]
    if carried_by == "EveryReplayFluidDomain":
        # **`Job files and outputs` D20's fifth place, added 2026-10-02**:
        # every fluid domain whose cache type is Replay, and no other — read
        # off the four Blenders (`Job files and outputs — machine`'s
        # `THE_FLUID_CACHE_LOOKUP`). A domain of another cache type is left as
        # the file has it. **It is walked to as the file loads, by L8b's
        # handler, and never by the material.**
        return [
            modifier
            for thing in bpy.data.objects
            for modifier in thing.modifiers
            if modifier.type == "FLUID"
            and modifier.fluid_type == "DOMAIN"
            and modifier.domain_settings.cache_type == "REPLAY"
        ]
    if carried_by == "TheScenesSequenceEditor":
        if scene.sequence_editor is None:
            return []
        return [scene.sequence_editor]
    if carried_by == "EveryStripsProxySettings":
        if scene.sequence_editor is None:
            return []
        return [
            strip.proxy
            for strip in scene.sequence_editor.strips_all
            if getattr(strip, "proxy", None) is not None
        ]
    raise ValueError(carried_by)

def smoozy_the_folder_held_for(folders, recorded):
    # **`Job files and outputs` R4 item 2: the folder of a carrier's own
    # recorded location**, and for a carrier that records none the folder for
    # those — or nothing, where the table holds no such value. The one
    # matching rule both lookups below share.
    wanted = os.fsencode(recorded) if recorded else None
    for one in folders:
        has = bytes(one["recorded"]) if one["recorded"] is not None else None
        if has == wanted:
            return os.fsdecode(bytes(one["folder"]))
    return None

def smoozy_the_folder_for(folders, recorded):
    # **`Job files and outputs` R4 item 2: each carrier is set to the folder
    # of its own recorded location**, and a carrier that records none to the
    # folder for those. A value the reading did not find has no folder, and
    # the lookup raises (L11): the reading and the material disagreeing about
    # the file is not something to render past.
    folder = smoozy_the_folder_held_for(folders, recorded)
    if folder is None:
        raise LookupError("smoozy: no job folder for the recorded location %r" % recorded)
    return folder

"#;

/// The rest of the head.
const THE_REST_OF_THE_HEAD: &str = r#"
def smoozy_the_caches_own_stem(carrier, setting):
    # **`Job files and outputs` D21b-i, corrected 2026-10-02: each cache keeps
    # its own file stem** — its name where it has one, else the stem Blender
    # gives it, which is its object's name as two capital hexadecimal digits
    # per byte (read off a cloth, a cloth on an object whose name is not
    # ASCII, and a particle system, in 4.5.12 and 5.2.2). A baked cache
    # copied into the job's folder is read under its own stem and under no
    # other.
    holder, name = smoozy_off(carrier, setting)
    own = getattr(holder, name)
    if own:
        return own
    return smoozy_a_stem_a_name_can_hold(
        "".join("%02X" % one for one in os.fsencode(carrier.id_data.name))
    )

def smoozy_a_stem_a_name_can_hold(stem):
    # **`Job files and outputs` D21b-i, resolved 2026-10-02**: a cache's name
    # holds 63 bytes, and Blender's own stem for an object named longer than
    # 31 bytes is longer than that. Such a stem becomes its first 46 bytes, an
    # underscore and the 64-bit FNV-1a of the whole stem in 16 capital
    # hexadecimal digits — `Job files and outputs — machine`'s own
    # `the_stem_a_cache_name_can_hold`, which the copy renames the cache's
    # files by, so the two agree.
    raw = os.fsencode(stem)
    if len(raw) <= 63:
        return stem
    mark = 0xCBF29CE484222325
    for one in raw:
        mark = ((mark ^ one) * 0x100000001B3) & 0xFFFFFFFFFFFFFFFF
    return os.fsdecode(raw[:46]) + "_" + "%016X" % mark

def smoozy_holder_and_name(scene, property_path):
    # A property path's first component names the scene, so the walk starts
    # at the scene object and every further component is a step off it.
    return smoozy_off(scene, property_path.split(".", 1)[-1])

def smoozy_sever(scene, layer_name, by_the_walk):
    # **`Render Settings` V6: severing then assigning holds, on and off.**
    # The severing happens in the running Blender only; nothing saves the
    # file, so the artist's `.blend` is untouched.
    #
    # The switch's own data path, in the form that document's own sources
    # record for the driver removal:
    # `scene.driver_remove('view_layers["VL_Drv"].use')`.
    path = 'view_layers["%s"].use' % layer_name
    scene.driver_remove(path)
    animation = getattr(scene, "animation_data", None)
    action = getattr(animation, "action", None) if animation is not None else None
    if action is None:
        return
    if by_the_walk:
        # **`Render Settings` V9, from 5.0**: the action's own curves route
        # does not exist —
        # asking for it raises — and the curve is reached by walking the
        # action's layers, then its strips, then its channel bags, then its
        # curves, with the removal by the channel bag's own remove.
        #
        # Blender's own reference, read 2026-09-04:
        # `Action.layers` -> `ActionLayer.strips` ("The list of strips that
        # are on this animation layer"), `ActionKeyframeStrip.channelbags`,
        # `ActionChannelbag.fcurves` ("The individual F-Curves that animate
        # the slot"), and on that collection `find(data_path, *, index=0)`
        # ("Find an F-Curve") and `remove(fcurve)` ("Remove F-Curve").
        for animation_layer in action.layers:
            for strip in animation_layer.strips:
                for bag in getattr(strip, "channelbags", ()):
                    curve = bag.fcurves.find(path)
                    if curve is not None:
                        bag.fcurves.remove(curve)
    else:
        # **`Render Settings` V9, in 4.5**: the action's curves hang directly
        # off the action.
        curve = action.fcurves.find(path)
        if curve is not None:
            action.fcurves.remove(curve)
"#;

/// The part that carries the instructions out.
const THE_BODY: &str = r#"
def smoozy_carry_out(document, first, last):
    scene = smoozy_scene(document)
    for instruction in document["instructions"][first:last]:
        what = instruction["do"]
        if what == "Marker":
            # **`Job files and outputs` D16**: the marker's opening, printed
            # on the render's own ordinary output as the material's first
            # act, naming the job identity.
            smoozy_say({"opened": instruction["job"]})
        elif what == "LoadTimeHandlerFinished":
            # **L8b**: the handler the script before the file registered sets
            # the places `Job files and outputs` D1a sets as the file loads. A
            # raise inside it does not end the run — Blender prints it, loads
            # the file and renders (observed at the desk, all four versions) —
            # so its record is read here, before any pair, and a handler that
            # did not run to its end raises, which L8a's argument turns into a
            # run ending before the render instruction with nothing written.
            record = sys.modules.get(instruction["record"])
            smoozy_say({"set_as_the_file_loaded": getattr(record, "what_was_set", None)})
            if record is None or not getattr(record, "finished", False):
                raise RuntimeError("smoozy: the settings Smoozy makes as the file opens were not made")
        elif what == "Step":
            # **L10b, the Creator's decision A**: where the reading in force
            # says the job's scene holds a simulation that is not baked, the
            # scene is stepped frame by frame from its start — or the list's
            # lowest frame where that is earlier — to the list's highest, after
            # every pair and before the read-back. The render instruction is
            # still the command line's (L1): this renders nothing. Observed at
            # the desk in all four versions: the command-line render then reads
            # what the stepping computed, where unstepped an unbaked
            # simulation zone is frame 1's picture on every frame.
            start = min(scene.frame_start, instruction["lowest"])
            for frame in range(start, instruction["highest"] + 1):
                scene.frame_set(frame)
            smoozy_say({"stepped": instruction["highest"] - start + 1, "from": start, "to": instruction["highest"]})
        elif what == "WriteAsAPicture":
            # **L10c, the nineteenth notes, notes 3 and 9**: a first-frame
            # check of a job whose only output is a video, which the reading
            # in force switched (`Job files and outputs` R4c), is written as a
            # still picture. From 5.0 the formats a scene offers follow its
            # media setting, so that is set first; 4.5 has none. A scene that
            # still writes a video raises, which L8a's argument turns into a
            # run ending before the render instruction with nothing written.
            image_settings = scene.render.image_settings
            if instruction["media_property"] is not None:
                setattr(image_settings, instruction["media_property"], instruction["media"])
            image_settings.file_format = instruction["format"]
            smoozy_say({"written_as_a_picture": instruction["format"]})
            if scene.render.is_movie_format:
                raise ValueError("smoozy: the first-frame check still writes a video")
        elif what == "MarkTheCheck":
            # **`Job files and outputs` P18a, the twenty-first notes, note 6**:
            # a first-frame check that writes into its range job's folder has
            # every name it writes marked, by the text the reading marked its
            # names with before it read the file. The regular output path and
            # 4.5's multilayer folder are not marked here: they arrive marked
            # through the map.
            smoozy_the_check_is_marked(scene, instruction["names"], instruction["mark"], False)
            smoozy_say({"marked": instruction["mark"]})
        elif what == "Redirect":
            # **`Job files and outputs` D9, and `Job files and outputs` D13**:
            # each destination's own
            # setting, individually, and no bulk route for any part of it.
            #
            # **The node is looked up in its collection by key**, never by
            # asking for an attribute named after the collection with the key
            # written into it, which is what a walk over a dotted path would
            # do and what no scene carries.
            #
            # **The destination's own scene where it has one.**
            # `Job files and outputs` R4 item 3: a destination reached through
            # a sequencer strip belongs to that scene, and assigning it
            # against the job's scene sets the wrong scene's node or finds
            # none.
            on = scene
            if instruction["scene"] is not None:
                on = bpy.data.scenes[os.fsdecode(bytes(instruction["scene"]))]
            # **The tree by the name this version gives it**: `node_tree` in
            # 4.5, `compositing_node_group` from 5.0, where a scene carries no
            # `node_tree` at all.
            tree = getattr(on, instruction["tree"])
            node = tree.nodes[os.fsdecode(bytes(instruction["node"]))]
            setattr(node, instruction["property"], os.fsdecode(bytes(instruction["folder"])))
            # **`Job files and outputs` P10: the further values are applied
            # with the folder and on the same object.** The folder alone is
            # that document's own observed silent no-op.
            for further in instruction["further_values"]:
                holder, name = smoozy_off(node, further["setting"])
                setattr(holder, name, further["value"])
            smoozy_say({
                "redirect": instruction["property"],
                "node": instruction["node"],
                "further": len(instruction["further_values"]),
                "set": True,
            })
        elif what == "RedirectAnAddedWritePlace":
            # **`Job files and outputs` D20 and `Job files and outputs` D21b**: the place's folder on
            # every setting that holds it, on every object in the file that
            # carries that setting, whether or not the reading found a value
            # there; **and `Job files and outputs` P10's further values on the same objects**, because
            # the folder alone is that document's own observed silent no-op.
            # **Nothing is read back as evidence that it took** (`Job files and outputs` D21b-ii).
            how_many = 0
            for one in instruction["settings"]:
                for carrier in smoozy_carriers(scene, one["carried_by"]):
                    holder, name = smoozy_off(carrier, one["setting"])
                    # `Job files and outputs` R4 item 2: the folder of this
                    # carrier's own recorded location, read before anything
                    # on the carrier is written.
                    folder = smoozy_the_folder_for(instruction["folders"], getattr(holder, name))
                    stem = None
                    if instruction["the_caches_own_stem_goes_to"] is not None:
                        stem = smoozy_the_caches_own_stem(carrier, instruction["the_caches_own_stem_goes_to"])
                    if instruction["the_folder_is_made_first"]:
                        # `Job files and outputs` D21b-i: *"the folder is created before the render is
                        # started, not left to Blender"*.
                        os.makedirs(folder, exist_ok=True)
                    # `Job files and outputs` D21b-i's order: *external* on,
                    # the folder, the name.
                    for further in instruction["switched_on_before_the_folder"]:
                        holder, name = smoozy_off(carrier, further["setting"])
                        setattr(holder, name, further["value"])
                    holder, name = smoozy_off(carrier, one["setting"])
                    setattr(holder, name, folder)
                    for further in instruction["further_values"]:
                        holder, name = smoozy_off(carrier, further["setting"])
                        setattr(holder, name, further["value"])
                    if stem is not None:
                        holder, name = smoozy_off(carrier, instruction["the_caches_own_stem_goes_to"])
                        setattr(holder, name, stem)
                    how_many += 1
            smoozy_say({
                "redirect": instruction["place"],
                "folders": len(instruction["folders"]),
                "set_on": how_many,
                "further": len(instruction["switched_on_before_the_folder"])
                + len(instruction["further_values"]),
            })
        elif what == "Assign":
            holder, name = smoozy_holder_and_name(scene, instruction["property"])
            setattr(holder, name, instruction["value"])
            smoozy_say({
                "family": instruction["family"],
                "property": instruction["property"],
                "set": True,
            })
        elif what == "Pin":
            # `Render Settings` V4 to `Render Settings` V7's three steps, in
            # order: resolve the
            # layer by its bytes, sever the switch's animation, assign the
            # pinned value. **All three are load-bearing and the middle one
            # is the finding**: a plain assignment on a keyframed or driven
            # switch is silently overwritten by the file at every frame and
            # the run reports success.
            #
            # A layer the file does not carry raises here, which is what ends
            # the attempt before the render instruction (`Render Settings`
            # `Render Settings` V7).
            name = os.fsdecode(bytes(instruction["layer"]))
            layer = scene.view_layers[name]
            smoozy_sever(scene, name, instruction["the_curve_is_reached_by_the_walk"])
            layer.use = instruction["on"]
            smoozy_say({"pin": instruction["layer"], "on": instruction["on"]})
        elif what == "NameCheck":
            # **L10a**: a read of the pair's own name against the object it
            # was assigned on. The value is not compared and nothing is done
            # with it. A name the object does not carry raises.
            holder, name = smoozy_holder_and_name(scene, instruction["property"])
            if not hasattr(holder, name):
                smoozy_say({"unapplied": instruction["property"]})
                raise AttributeError(instruction["property"])
        elif what == "PinNameCheck":
            # **L10a, for a pin.** The layer is resolved by its own bytes, the
            # same way the pin's own assignment resolves it, and the switch's
            # name is read off that object. **No property path is composed and
            # nothing is split**: the switch's written-out form is a display
            # form with a placeholder standing for the layer's name, and a walk
            # given one asks for an attribute no scene carries.
            name = os.fsdecode(bytes(instruction["layer"]))
            layer = scene.view_layers[name]
            if not hasattr(layer, instruction["property"]):
                smoozy_say({"unapplied": instruction["property"]})
                raise AttributeError(instruction["property"])
        elif what == "ReadBack":
            # **`Job files and outputs` D10**: the one pair the material does
            # not set. The comparison is against the raw text handed in,
            # template marks and all, and never against a resolved file name.
            holder, name = smoozy_holder_and_name(scene, instruction["property"])
            in_force = os.fsencode(getattr(holder, name))
            smoozy_say({"read_back": list(in_force)})
            if in_force != bytes(instruction["must_equal"]):
                raise ValueError(instruction["property"])
        else:
            raise ValueError(what)
"#;

// ---------------------------------------------------------------------------
// L12 — the placement, and the read-back under the render's login
// ---------------------------------------------------------------------------

/// Writing the material and reading it back, behind a trait — **the one
/// place in this crate that writes a file on the machine.**
pub trait PlacesTheMaterial {
    /// The text, written where the launch line will name it.
    fn place(&self, at: &str, text: &str) -> Result<(), String>;

    /// **L12: the same path read back under the login the render runs as.**
    /// `None` where it could not be read at all.
    fn read_it_back(&self, at: &str) -> Option<Vec<u8>>;
}

/// What the confirmation found.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhetherThePlacementIsConfirmed {
    /// The material is at the path and reads back as what was written.
    ItIs,
    /// It could not be written at all — a failed attempt before the render
    /// instruction, naming what stopped it.
    ItCouldNotBePlaced { why: String },
    /// It was written and the login the render runs as could not read it —
    /// **the one shape of the unreadable-script hazard that is Smoozy's own
    /// doing**, closed here.
    TheRendersLoginCouldNotReadIt,
    /// It read back as something else. **Nothing repairs it and nothing
    /// writes it again**: a second write would be a second thing that could
    /// have gone wrong, and the attempt fails before the render instruction.
    ItReadBackAsSomethingElse,
}

impl WhetherThePlacementIsConfirmed {
    /// **What became of the placement, in standard software words** (Rule 9) —
    /// beside the type, so the ending that carries it records a sentence.
    /// Added 2026-09-23, step 10c's fifth pass on Vast, fault 5.
    pub fn in_words(&self) -> String {
        match self {
            WhetherThePlacementIsConfirmed::ItIs => {
                "it was written and read back as what was written".to_string()
            }
            WhetherThePlacementIsConfirmed::ItCouldNotBePlaced { why } => {
                format!("write failed: {why}")
            }
            WhetherThePlacementIsConfirmed::TheRendersLoginCouldNotReadIt => {
                "the render's user cannot read it".to_string()
            }
            WhetherThePlacementIsConfirmed::ItReadBackAsSomethingElse => {
                "it read back different".to_string()
            }
        }
    }
}

/// **L12, performed.**
pub fn place_and_confirm(
    places: &impl PlacesTheMaterial,
    at: &str,
    text: &str,
) -> WhetherThePlacementIsConfirmed {
    if let Err(why) = places.place(at, text) {
        return WhetherThePlacementIsConfirmed::ItCouldNotBePlaced { why };
    }
    match places.read_it_back(at) {
        None => WhetherThePlacementIsConfirmed::TheRendersLoginCouldNotReadIt,
        Some(bytes) if bytes == text.as_bytes() => WhetherThePlacementIsConfirmed::ItIs,
        Some(_) => WhetherThePlacementIsConfirmed::ItReadBackAsSomethingElse,
    }
}

/// The real end, on the machine.
///
/// **The read-back is a fresh open of the path rather than a memory of the
/// write**, which is the whole of what L12 asks: a write that succeeded and
/// a file the render's login can open are two different facts.
///
/// **The supervisor and every worker run as one login** (`Rented-server
/// runtime` X16), so the login this reads under **is** the login the
/// render runs as, by the machine's own arrangement rather than by anything
/// here.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TheWorkersOwnFilesystem;

impl PlacesTheMaterial for TheWorkersOwnFilesystem {
    fn place(&self, at: &str, text: &str) -> Result<(), String> {
        std::fs::write(at, text).map_err(|why| format!("{at}: {why}"))
    }

    fn read_it_back(&self, at: &str) -> Option<Vec<u8>> {
        let mut back = Vec::new();
        std::fs::File::open(at).ok()?.read_to_end(&mut back).ok()?;
        Some(back)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::material::tests::{a_job, a_remap, a_scene, an_untouched_payload, compose_with};
    use smoozy_render_settings_machine::view_layers::PinEntry;

    const FIVE_TWO: Version = Version {
        major: 5,
        minor: 2,
        patch: 0,
    };
    const FOUR_FIVE: Version = Version {
        major: 4,
        minor: 5,
        patch: 12,
    };

    /// A material with a pair of every family in it, composed by the rules
    /// half.
    fn a_material_with_a_pin() -> (LaunchMaterial, RemapMaterial, SceneReading) {
        use smoozy_render_settings_machine::names::{AnchorState, ExactName};
        let job = a_job();
        let remap = a_remap(&job, true);
        let mut payload = an_untouched_payload();
        payload.resolution_percentage = Some(50);
        payload.view_layer_pins.push(PinEntry {
            layer: ExactName::as_the_reading_produced_it("ViewLayer"),
            anchor: AnchorState::Anchored {
                the_reading_it_came_from: crate::material::tests::a_reading().identity(),
            },
            on: true,
        });
        let material = compose_with(&payload, &remap, &job).expect("a composed material");
        (material, remap, a_scene("Kitchen"))
    }

    #[test]
    fn the_text_carries_the_documents_own_order_and_binds_one_scene() {
        let (material, remap, scene) = a_material_with_a_pin();
        let text = the_materials_text(&material, FIVE_TWO, &scene, &remap);
        assert!(text.contains("smoozy_scene(document)"));
        assert!(text.contains(THE_MARKER_HEAD));
        // **The scene is in the document as data and never in an
        // expression.**
        assert!(text.contains(r#""scene":"#));
        assert!(!text.contains("bpy.data.scenes[\"Kitchen\"]"));
    }

    /// **The compositing tree is asked for by the name the version gives
    /// it.** From 5.0 a scene carries no `node_tree` at all, and until
    /// 2026-09-26 every version's text asked for it — so a file output node's
    /// redirect would have raised on 5.0, 5.1 and 5.2 and ended the job before
    /// the render.
    #[test]
    fn a_file_output_nodes_redirect_asks_for_the_tree_by_the_versions_own_name() {
        let (material, remap, scene) = a_material_with_a_pin();
        let from_five = the_materials_text(&material, FIVE_TWO, &scene, &remap);
        let in_four_five = the_materials_text(&material, FOUR_FIVE, &scene, &remap);
        assert!(from_five.contains(r#""tree":"compositing_node_group""#));
        assert!(in_four_five.contains(r#""tree":"node_tree""#));
        assert!(!from_five.contains("on.node_tree"));
    }

    /// **The four added write places reach the text as settings on what
    /// carries them**, each by its cited name, and only the point cache's
    /// folder is made first (`Job files and outputs` D21b-i).
    #[test]
    fn the_added_write_places_reach_the_text_on_what_carries_them() {
        use crate::material::tests::a_redirect_assignment;
        use crate::material::{compose, MaterialInputs};
        use crate::versions::TheFourVersions;
        use smoozy_job_files_machine::reading::{
            DestinationKind, WhereItComesFrom, EVERY_ADDED_WRITE_PLACE,
        };
        let job = a_job();
        let remap = a_remap(&job, false);
        let payload = an_untouched_payload();
        let reading = crate::material::tests::a_reading();
        let scene = a_scene("Kitchen");
        let redirects: Vec<_> = EVERY_ADDED_WRITE_PLACE
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
        .unwrap();
        let text = the_materials_text(&material, FIVE_TWO, &scene, &remap);
        // **Three since 2026-10-02**: `Job files and outputs` D1a's two are
        // set as the file loads by L8b's handler, and the material sets
        // neither — the geometry-nodes assignment it made after the load
        // never took.
        assert_eq!(text.matches(r#""do":"RedirectAnAddedWritePlace""#).count(), 3);
        for wanted in [
            r#"{"carried_by":"TheScenesBakeSettings","setting":"filepath"}"#,
            r#"{"carried_by":"EverythingCarryingAPointCache","setting":"point_cache.filepath"}"#,
            r#"{"carried_by":"TheScenesSequenceEditor","setting":"proxy_dir"}"#,
            r#"{"carried_by":"EveryStripsProxySettings","setting":"directory"}"#,
        ] {
            assert!(text.contains(wanted), "{wanted} is not in the text");
        }
        for not_wanted in [
            r#"{"carried_by":"EveryGeometryNodesModifier","setting":"bake_directory"}"#,
            r#"{"carried_by":"EveryGeometryNodesBake","setting":"directory"}"#,
            r#"{"carried_by":"EveryReplayFluidDomain","setting":"domain_settings.cache_directory"}"#,
        ] {
            assert!(!text.contains(not_wanted), "{not_wanted} is set after the load");
        }
        assert_eq!(text.matches(r#""the_folder_is_made_first":true"#).count(), 1);
        // **Every carrier the document names has a walk in the text**, read
        // off the document's own list rather than off one written here, so no
        // instruction reaches the text's own refusal of a carrier it does not
        // know.
        for place in EVERY_ADDED_WRITE_PLACE {
            for one in smoozy_job_files_machine::redirect::the_settings_of(place) {
                let carried_by = serde_json::to_string(&one.carried_by).unwrap();
                assert!(
                    text.contains(&format!("if carried_by == {carried_by}:")),
                    "{carried_by} has no walk"
                );
            }
        }
    }

    /// **`Job files and outputs` D21b-i, corrected 2026-10-02: the point
    /// cache's values are set only where the file already writes the cache to
    /// disk, never on a rigid-body world, and the disk-cache switch is never
    /// written.** At the desk, in 4.5.12 and 5.2.2, writing the switch deleted
    /// a baked cloth's 48 files beside the `.blend`, turning it on for a cache
    /// kept in memory dropped a cloth's in-file bake and wrote 150 files beside
    /// the `.blend`, and turning it on for a rigid-body world ended Blender
    /// with exit 139.
    #[test]
    fn the_point_cache_is_set_only_where_the_file_writes_it_to_disk_and_its_switch_is_never_written() {
        use crate::material::tests::a_redirect_assignment;
        use crate::material::{compose, MaterialInputs};
        use crate::versions::TheFourVersions;
        use smoozy_job_files_machine::reading::{DestinationKind, WhereItComesFrom};
        let job = a_job();
        let remap = a_remap(&job, false);
        let payload = an_untouched_payload();
        let reading = crate::material::tests::a_reading();
        let scene = a_scene("Kitchen");
        let place = AnAddedWritePlace::ThePointCache;
        let mut cache = a_redirect_assignment("File Output");
        cache.destination.kind = DestinationKind::AnAddedWritePlace(place);
        cache.destination.origin = WhereItComesFrom::AnAddedWritePlaceNamed(place);
        cache.property = None;
        cache.further_values = smoozy_job_files_machine::mapping::the_further_values_for(place);
        let material = compose(MaterialInputs {
            job: &job,
            version: TheFourVersions::V5_2,
            redirect_pairs: &[cache],
            remap: &remap,
            payload: &payload,
            reading_in_force: &reading,
            the_jobs_scene: &scene,
            regular_output_path: b"/ephemeral/smoozy/jobs/j1/frame_####",
            the_stepping: None,
            the_check_is_written_as_a_picture: false,
        })
        .unwrap();
        let text = the_materials_text(&material, FIVE_TWO, &scene, &remap);
        // **No rigid-body world is walked to by our own walk**, so none is ever
        // set. (The remap's own fragment reads one, as a reference the check
        // may have found; it writes no disk-cache switch either.)
        let walk = &THE_SHARED_WALK[THE_SHARED_WALK.find("def smoozy_carriers").unwrap()
            ..THE_SHARED_WALK.find("def smoozy_the_folder_for").unwrap()];
        assert!(text.contains(walk));
        assert!(!walk.contains("rigidbody_world"), "a rigid-body world is walked to");
        // **The switch is read, to leave memory caches alone, and never written.**
        assert!(text.contains("if each.point_cache.use_disk_cache]"));
        assert!(!text.contains(r#""setting":"point_cache.use_disk_cache""#));
        assert!(!text.contains("use_disk_cache ="));
        // **`Job files and outputs` D21b-i's order: *external* on, the folder, the name.**
        assert!(text.contains(
            r#""switched_on_before_the_folder":[{"setting":"point_cache.use_external","value":true}]"#
        ));
        assert!(text.contains(r#""further_values":[]"#));
        assert!(text.contains(r#""the_caches_own_stem_goes_to":"point_cache.name""#));
        let switches = text
            .find(r#"for further in instruction["switched_on_before_the_folder"]:"#)
            .expect("the switches are written");
        let folder = text.find("setattr(holder, name, folder)").expect("the folder is written");
        let the_stem = text[folder..]
            .find("setattr(holder, name, stem)")
            .expect("the cache's own stem is written");
        assert!(switches < folder && the_stem > 0);
    }

    /// **`Job files and outputs` R4 item 2, corrected 2026-10-02: a place
    /// with several recorded locations is one instruction carrying each
    /// location's folder, and every carrier is set to the folder of its own.**
    /// Until then the text set every carrier of a place to one folder, and a
    /// file whose geometry-nodes bakes sat in four folders, all four sent,
    /// ended every attempt before the render (*"this job's folder has no place
    /// for //bakePath"*, at the desk in 5.2.2). **Told on the sequencer proxy
    /// folder since 2026-10-02**, when the geometry-nodes bake folder left the
    /// material for L8b's handler; the table is the same for every place.
    #[test]
    fn each_carrier_of_a_place_is_set_to_the_folder_of_its_own_recorded_location() {
        use crate::material::tests::a_redirect_assignment;
        use crate::material::{compose, MaterialInputs};
        use crate::versions::TheFourVersions;
        use smoozy_job_files_machine::reading::{
            DestinationKind, RecordedLocation, WhereItComesFrom,
        };
        let job = a_job();
        let remap = a_remap(&job, false);
        let payload = an_untouched_payload();
        let reading = crate::material::tests::a_reading();
        let scene = a_scene("Kitchen");
        let place = AnAddedWritePlace::TheSequencerProxyFolder;
        let redirects: Vec<_> = [
            (Some(&b"//bakePath"[..]), &b"/job/bakePath"[..]),
            (Some(&b"//set-on-node"[..]), &b"/job/set-on-node"[..]),
            (None, &b"/job/geometry-nodes-bake"[..]),
        ]
        .into_iter()
        .map(|(recorded, folder)| {
            let mut added = a_redirect_assignment("File Output");
            added.destination.kind = DestinationKind::AnAddedWritePlace(place);
            added.destination.origin = WhereItComesFrom::AnAddedWritePlaceNamed(place);
            added.destination.recorded_location = recorded.map(RecordedLocation::of);
            added.property = None;
            added.folder = folder.to_vec();
            added.further_values = Vec::new();
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
        .unwrap();
        let text = the_materials_text(&material, FIVE_TWO, &scene, &remap);
        // **One instruction for the place, holding all three folders.**
        assert_eq!(text.matches(r#""do":"RedirectAnAddedWritePlace""#).count(), 1);
        let bytes = |text: &[u8]| serde_json::to_string(text).unwrap();
        let wanted = format!(
            r#""folders":[{{"folder":{},"recorded":{}}},{{"folder":{},"recorded":{}}},{{"folder":{},"recorded":null}}]"#,
            bytes(b"/job/bakePath"),
            bytes(b"//bakePath"),
            bytes(b"/job/set-on-node"),
            bytes(b"//set-on-node"),
            bytes(b"/job/geometry-nodes-bake"),
        );
        assert!(text.contains(&wanted), "the table is not in the text");
        // **The body looks each carrier's own value up before writing it**,
        // and a value with no folder raises rather than being set somewhere.
        assert!(text.contains(
            r#"folder = smoozy_the_folder_for(instruction["folders"], getattr(holder, name))"#
        ));
        assert!(text.contains("raise LookupError(\"smoozy: no job folder for the recorded location"));
    }

    /// **`Job files and outputs` D21b-i, corrected 2026-10-02: each cache
    /// keeps its own file stem**, read off the cache before anything on it is
    /// written and written to its `name` last — its name where it has one,
    /// else its object's name as two capital hexadecimal digits per byte,
    /// which is the stem Blender gave a cloth, a cloth on an object whose name
    /// is not ASCII, and a particle system at the desk. Under the one stem
    /// `smoozy` a baked cloth copied into the job's folder was not read.
    #[test]
    fn each_cache_keeps_its_own_file_stem() {
        let walk = &THE_REST_OF_THE_HEAD[THE_REST_OF_THE_HEAD
            .find("def smoozy_the_caches_own_stem")
            .unwrap()
            ..THE_REST_OF_THE_HEAD.find("def smoozy_holder_and_name").unwrap()];
        assert!(walk.contains("own = getattr(holder, name)"));
        assert!(walk.contains(r#""".join("%02X" % one for one in os.fsencode(carrier.id_data.name))"#));
        // **`Job files and outputs` D21b-i, resolved 2026-10-02: a stem the name cannot hold is made
        // one it can, by the same rule the copy renames the files by** —
        // `Job files and outputs — machine`'s own, whose test pins its value.
        assert!(walk.contains("return smoozy_a_stem_a_name_can_hold("));
        assert!(walk.contains(&format!(
            "if len(raw) <= {}:",
            smoozy_job_files_machine::mapping::THE_LONGEST_STEM_A_CACHE_NAME_HOLDS
        )));
        assert!(walk.contains("mark = 0xCBF29CE484222325"));
        assert!(walk.contains("mark = ((mark ^ one) * 0x100000001B3) & 0xFFFFFFFFFFFFFFFF"));
        assert!(walk.contains(r#"return os.fsdecode(raw[:46]) + "_" + "%016X" % mark"#));
        assert!(!THE_BODY.contains("smoozy\""), "no one stem for every cache");
        let read = THE_BODY
            .find("stem = smoozy_the_caches_own_stem(carrier,")
            .expect("the stem is read off the carrier");
        let switches = THE_BODY
            .find(r#"for further in instruction["switched_on_before_the_folder"]:"#)
            .expect("the switches are written");
        let written = THE_BODY
            .find("setattr(holder, name, stem)")
            .expect("the stem is written");
        assert!(read < switches && switches < written);
    }

    /// **L8b: the material's first act after its marker is to read whether
    /// the handler the script before the file registered ran to its end, and
    /// to raise where it did not** — before any pair, so nothing is set over a
    /// file whose load-time places were not. A raise inside the handler does
    /// not end the run (observed at the desk on 2026-10-02 in all four
    /// versions), so this raise is what does.
    #[test]
    fn the_material_raises_before_any_pair_where_the_load_time_handler_did_not_finish() {
        let (material, remap, scene) = a_material_with_a_pin();
        let text = the_materials_text(&material, FIVE_TWO, &scene, &remap);
        let marker = text.find(r#"{"do":"Marker""#).expect("the marker opens");
        let check = text
            .find(&format!(
                r#"{{"do":"LoadTimeHandlerFinished","record":"{}"}}"#,
                smoozy_job_files_machine::redirect::THE_LOAD_TIME_RECORD
            ))
            .expect("the handler's check is an instruction");
        let first_pair = text.find(r#"{"do":"Redirect"#).expect("a redirect pair");
        assert!(marker < check && check < first_pair);
        let branch = &THE_BODY[THE_BODY.find(r#"elif what == "LoadTimeHandlerFinished":"#).unwrap()
            ..THE_BODY.find(r#"elif what == "Redirect":"#).unwrap()];
        assert!(branch.contains(r#"record = sys.modules.get(instruction["record"])"#));
        assert!(branch.contains(r#"if record is None or not getattr(record, "finished", False):"#));
        assert!(branch.contains("raise RuntimeError("));
    }

    /// **L11, checked on the rendered text rather than argued about it.**
    #[test]
    fn the_rendered_text_carries_no_failure_handling_of_its_own() {
        let (material, remap, scene) = a_material_with_a_pin();
        let text = the_materials_text(&material, FIVE_TWO, &scene, &remap);
        for word in [
            ["try", ":"].concat(),
            ["except", " "].concat(),
            ["contextlib", ".suppress"].concat(),
        ] {
            assert!(!text.contains(&word), "{word}");
        }
        assert!(material.contains_no_failure_handling());
    }

    /// **`Render Settings` V9's route, per version, read off the text.** The
    /// two branches are different code, and a material composed for 4.5
    /// carries the direct route while one composed from 5.0 carries the
    /// walk — which is the one thing the pin branches on.
    #[test]
    fn the_pins_severing_takes_the_versions_own_route_to_the_curve() {
        let (material, remap, scene) = a_material_with_a_pin();
        let from_five = the_materials_text(&material, FIVE_TWO, &scene, &remap);
        let in_four_five = the_materials_text(&material, FOUR_FIVE, &scene, &remap);
        assert!(from_five.contains(r#""the_curve_is_reached_by_the_walk":true"#));
        assert!(in_four_five.contains(r#""the_curve_is_reached_by_the_walk":false"#));
        // Both routes are in the body, and the driver removal is in both.
        assert!(from_five.contains("channelbags"));
        assert!(from_five.contains("action.fcurves.find"));
        assert!(from_five.contains("scene.driver_remove(path)"));
    }

    /// **`Render Settings` V4's three steps sit in the text in that
    /// clause's own order**, the severing
    /// between the resolve and the assignment — the middle one being the
    /// finding, since a plain assignment on an animated switch is silently
    /// overwritten at every frame.
    #[test]
    fn the_pins_three_steps_are_in_the_plans_order() {
        let (material, remap, scene) = a_material_with_a_pin();
        let text = the_materials_text(&material, FIVE_TWO, &scene, &remap);
        let resolve = text
            .find("layer = scene.view_layers[name]")
            .expect("resolve");
        let sever = text.find("smoozy_sever(scene, name,").expect("sever");
        let assign = text.find("layer.use = instruction").expect("assign");
        assert!(resolve < sever && sever < assign);
    }

    /// **The remap's fragment is emitted once, at the place its first entry
    /// sits in L10's order** — and the instructions are carried out in two
    /// stretches around it, so the order of application is the document's.
    #[test]
    fn the_remaps_own_fragment_is_emitted_once_and_the_order_is_kept_around_it() {
        let (material, remap, scene) = a_material_with_a_pin();
        let text = the_materials_text(&material, FIVE_TWO, &scene, &remap);
        let ours = text.matches("def smoozy_carry_out").count();
        assert_eq!(ours, 1, "one body of ours");
        let theirs = text
            .matches(smoozy_dependencies_machine::material::THE_PER_PAIR_HEAD)
            .count();
        assert!(theirs >= 1, "the remap's own fragment is in the text");
        let calls = text.matches("smoozy_carry_out(SMOOZY,").count();
        assert_eq!(calls, 2, "two stretches, the fragment between them");
    }

    /// **L12 run for real**: written, read back under this login, and each
    /// of the three failing shapes told apart.
    #[test]
    fn the_placement_is_confirmed_by_a_fresh_read_and_the_three_failures_are_told_apart() {
        let root = std::env::temp_dir().join(format!("smoozy-material-{}", std::process::id()));
        std::fs::create_dir_all(&root).expect("the place is made");
        let at = root.join("material.py");
        let places = TheWorkersOwnFilesystem;
        assert_eq!(
            place_and_confirm(&places, at.to_str().unwrap(), "print('x')\n"),
            WhetherThePlacementIsConfirmed::ItIs
        );
        // A path that cannot be written at all.
        assert!(matches!(
            place_and_confirm(&places, "/no/such/place/material.py", "x"),
            WhetherThePlacementIsConfirmed::ItCouldNotBePlaced { .. }
        ));
        std::fs::remove_dir_all(&root).ok();
    }

    /// The two readings the confirmation separates, with the placement
    /// answering and the read-back answering differently — the fault
    /// introduced one at a time.
    #[test]
    fn a_placement_that_cannot_be_read_back_and_one_that_reads_back_wrong_are_two_answers() {
        struct Wrote(Option<Vec<u8>>);
        impl PlacesTheMaterial for Wrote {
            fn place(&self, _at: &str, _text: &str) -> Result<(), String> {
                Ok(())
            }
            fn read_it_back(&self, _at: &str) -> Option<Vec<u8>> {
                self.0.clone()
            }
        }
        assert_eq!(
            place_and_confirm(&Wrote(None), "at", "text"),
            WhetherThePlacementIsConfirmed::TheRendersLoginCouldNotReadIt
        );
        assert_eq!(
            place_and_confirm(&Wrote(Some(b"other".to_vec())), "at", "text"),
            WhetherThePlacementIsConfirmed::ItReadBackAsSomethingElse
        );
        assert_eq!(
            place_and_confirm(&Wrote(Some(b"text".to_vec())), "at", "text"),
            WhetherThePlacementIsConfirmed::ItIs
        );
    }

    /// **L10b in the text: the job's scene stepped from its own start — or
    /// the list's lowest frame where that is earlier — to the list's highest,
    /// with the count on the marker, and nothing rendered from the
    /// material** (L1). Where item 8 says no there is no step at all.
    #[test]
    fn the_material_steps_the_jobs_scene_from_its_start_and_renders_nothing() {
        use crate::material::tests::a_reading;
        use crate::material::{compose, MaterialInputs};
        use crate::versions::TheFourVersions;
        let job = a_job();
        let remap = a_remap(&job, false);
        let payload = an_untouched_payload();
        let reading = a_reading();
        let scene = a_scene("Kitchen");
        let text_with = |the_stepping| {
            let material = compose(MaterialInputs {
                job: &job,
                version: TheFourVersions::V5_2,
                redirect_pairs: &[],
                remap: &remap,
                payload: &payload,
                reading_in_force: &reading,
                the_jobs_scene: &scene,
                regular_output_path: b"/ephemeral/smoozy/jobs/j1/frame_####",
                the_stepping,
                the_check_is_written_as_a_picture: false,
            })
            .unwrap();
            the_materials_text(&material, FIVE_TWO, &scene, &remap)
        };
        let without = text_with(None);
        assert!(!without.contains(r#""do":"Step""#));
        let text = text_with(crate::material::TheStepping::for_the_list(true, &[12, 13, 24]));
        assert!(text.contains(r#"{"do":"Step","highest":24,"lowest":12}"#), "{text}");
        let branch = &THE_BODY[THE_BODY.find(r#"elif what == "Step":"#).unwrap()
            ..THE_BODY.find(r#"elif what == "Redirect":"#).unwrap()];
        assert!(branch.contains(r#"start = min(scene.frame_start, instruction["lowest"])"#));
        assert!(branch.contains(r#"for frame in range(start, instruction["highest"] + 1):"#));
        assert!(branch.contains("scene.frame_set(frame)"));
        assert!(branch.contains(r#"smoozy_say({"stepped": "#));
        for never in ["render.render(", "bpy.ops.render", "write_still"] {
            assert!(!text.contains(never), "{never}: nothing renders from the material");
        }
    }

    /// **L10c in the text**: the switch rides as one instruction carrying the
    /// version's own names, and the body sets the media setting before the
    /// format, then raises where the scene still writes a video. Nothing
    /// renders from it, and nothing is saved.
    #[test]
    fn the_checks_picture_rides_as_one_instruction_and_raises_where_it_did_not_take() {
        use crate::material::tests::a_reading;
        use crate::material::{compose, MaterialInputs};
        use crate::versions::TheFourVersions;
        let job = a_job();
        let remap = a_remap(&job, false);
        let payload = an_untouched_payload();
        let reading = a_reading();
        let scene = a_scene("Kitchen");
        let text_for = |version, manifests: Version, the_check_is_written_as_a_picture| {
            let material = compose(MaterialInputs {
                job: &job,
                version,
                redirect_pairs: &[],
                remap: &remap,
                payload: &payload,
                reading_in_force: &reading,
                the_jobs_scene: &scene,
                regular_output_path: b"/ephemeral/smoozy/jobs/j1/frame_####",
                the_stepping: None,
                the_check_is_written_as_a_picture,
            })
            .unwrap();
            the_materials_text(&material, manifests, &scene, &remap)
        };
        assert!(!text_for(TheFourVersions::V5_2, FIVE_TWO, false).contains(r#""do":"WriteAsAPicture""#));
        let five = text_for(TheFourVersions::V5_2, FIVE_TWO, true);
        assert!(
            five.contains(r#"{"do":"WriteAsAPicture","format":"PNG","media":"IMAGE","media_property":"media_type"}"#),
            "{five}"
        );
        let four = text_for(TheFourVersions::V4_5, FOUR_FIVE, true);
        assert!(
            four.contains(r#"{"do":"WriteAsAPicture","format":"PNG","media":null,"media_property":null}"#),
            "{four}"
        );
        let branch = &THE_BODY[THE_BODY.find(r#"elif what == "WriteAsAPicture":"#).unwrap()
            ..THE_BODY.find(r#"elif what == "Redirect":"#).unwrap()];
        let media = branch.find(r#"setattr(image_settings, instruction["media_property"], instruction["media"])"#);
        let format = branch.find(r#"image_settings.file_format = instruction["format"]"#);
        let check = branch.find("if scene.render.is_movie_format:");
        assert!(media.is_some() && format.is_some() && check.is_some());
        assert!(media < format && format < check, "the media setting, the format, then the read");
        assert!(branch.contains(r#"raise ValueError("smoozy: the first-frame check still writes a video")"#));
        for never in ["render.render(", "save_as_mainfile", "save_mainfile", "write_still"] {
            assert!(!five.contains(never), "{never}: nothing renders or saves from the material");
        }
    }

    /// **`Job files and outputs` P18a in the text: a check that writes into
    /// its range job's folder has its names marked**, by one instruction
    /// carrying the version's own names and the text the reading ran, told to
    /// leave the regular output path and 4.5's multilayer folder to the map.
    /// Nowhere else: a payload that names no range carries none. The
    /// twenty-first notes, note 6.
    #[test]
    fn twenty_first_6_a_check_writing_into_its_range_jobs_folder_has_its_names_marked() {
        use crate::material::tests::a_reading;
        use crate::material::{compose, Entry, MaterialInputs};
        use crate::versions::TheFourVersions;
        let job = a_job();
        let remap = a_remap(&job, false);
        let reading = a_reading();
        let scene = a_scene("Kitchen");
        let text_for = |version, manifests: Version, payload: &smoozy_render_settings_machine::payload::Payload| {
            let material = compose(MaterialInputs {
                job: &job,
                version,
                redirect_pairs: &[],
                remap: &remap,
                payload,
                reading_in_force: &reading,
                the_jobs_scene: &scene,
                regular_output_path: b"/ephemeral/smoozy/jobs/j1/check_frame_####",
                the_stepping: None,
                the_check_is_written_as_a_picture: false,
            })
            .unwrap();
            let marked = material
                .entries()
                .iter()
                .filter(|entry| matches!(entry, Entry::TheChecksNamesMarked { .. }))
                .count();
            (marked, the_materials_text(&material, manifests, &scene, &remap))
        };
        let ordinary = an_untouched_payload();
        let (marked, text) = text_for(TheFourVersions::V5_2, FIVE_TWO, &ordinary);
        assert_eq!(marked, 0);
        assert!(!text.contains(r#""do":"MarkTheCheck""#));
        let mut check = an_untouched_payload();
        check.the_check_writes_into = Some("the-range-job".to_string());
        let (marked, five) = text_for(TheFourVersions::V5_2, FIVE_TWO, &check);
        assert_eq!(marked, 1);
        assert!(five.contains(r#""do":"MarkTheCheck""#), "{five}");
        assert!(five.contains(r#""mark":"check_""#), "{five}");
        assert!(five.contains(r#""the_plain_nodes_names_start":"file_name""#), "{five}");
        assert!(five.contains("def smoozy_the_check_is_marked("));
        let (_, four) = text_for(TheFourVersions::V4_5, FOUR_FIVE, &check);
        assert!(four.contains(r#""the_plain_nodes_names_start":"path""#), "{four}");
        assert!(four.contains(r#""the_gathering_nodes_name_start":"base_path""#), "{four}");
        let branch = &THE_BODY[THE_BODY.find(r#"elif what == "MarkTheCheck":"#).unwrap()
            ..THE_BODY.find(r#"elif what == "Redirect":"#).unwrap()];
        assert!(branch.contains(
            r#"smoozy_the_check_is_marked(scene, instruction["names"], instruction["mark"], False)"#
        ));
    }
}

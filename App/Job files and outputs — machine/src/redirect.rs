//! §5 — the redirect, and the launch material that carries it.
//!
//! **D1: the redirect is carried out by material handed to Blender at launch,
//! which sets each destination's own value in the running Blender before the
//! render instruction runs. No copy of the artist's `.blend` is made.**
//!
//! **D2: the choice was open and five constraints closed it.** The launch lever
//! is not optional and already exists; a copy has to sit beside the original and
//! puts a file Smoozy made into the folder the user browses; everything that
//! names the file afterwards speaks of the uploaded file; the copy route's save
//! silently rewrites locations nobody asked to change; and nothing is bought by
//! copying, because either route has to write each destination's setting
//! individually and per version.
//!
//! **D5: the artist's `.blend` is not written to by anything here.** The redirect
//! sets values in a running Blender and nothing saves the file. Observed
//! deliberately on 2026-08-17: with the rewrite live in the running Blender, a
//! fresh process reading the same file off disk saw every original path and the
//! digest was unchanged; and the artist's own file was digested before and after
//! all seven runs and never changed, **including the two runs where nothing was
//! rewritten, which is the control showing the check would have noticed**.
//!
//! **D6: what is owned here is the material; how it is handed to Blender is
//! `Rendering`'s launch contract.** So this file composes the material as values
//! and renders none of it into whatever Blender is given — a location crosses as
//! bytes under R4a, and a text form composed here would be a second place a
//! location could be decoded, quoted or dropped.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Dependency Upload & Remap` M9d. `Rendering` L8a.

use serde::{Deserialize, Serialize};

use crate::mapping::{
    the_mapped_folder_and_its_name_front, the_name_front_of, FurtherValue, HowItMapped, TheMap,
    SEPARATOR,
};
use crate::reading::{
    AnAddedWritePlace, ArtistsText, Destination, DestinationKind, RecordedLocation,
    TheVersionInHand, WhichBranch,
};

/// **D8: the two values a file output node keeps are not named the same across
/// the boundary, and the redirect must know which.**
///
/// Observed on 2026-08-17, per version, on disk, by a separate process: the
/// property carrying a node's folder is **`base_path` in 4.5.9** and
/// **`directory` in 5.0.1 and 5.2.0**. **5.1 was not exercised** (§14 item 4).
pub const THE_NODES_FOLDER_PROPERTY_IN_FOUR_FIVE: &str = "base_path";
/// The same property, from 5.0.
pub const THE_NODES_FOLDER_PROPERTY_FROM_FIVE_ZERO: &str = "directory";

/// **D8, performed.**
pub fn the_nodes_folder_property(version: TheVersionInHand) -> &'static str {
    match version.branch() {
        WhichBranch::FourFive => THE_NODES_FOLDER_PROPERTY_IN_FOUR_FIVE,
        WhichBranch::FromFiveZero => THE_NODES_FOLDER_PROPERTY_FROM_FIVE_ZERO,
    }
}

/// **D8's second half: the redirect writes that property with a trailing
/// separator, always, whatever the node's gathering mode.** ~~Always~~ —
/// **corrected 2026-10-04 by piece 1b**: except where 4.5's gathering node
/// holds a name front after its folder, which the redirect keeps
/// ([`the_value_a_nodes_folder_property_takes`]). The separator turned the
/// artist's file name into a folder; the reasoning below is why the value is
/// still always under the job's folder.
///
/// *"In 4.5, when the node is gathering its outputs into one multi-layer file,
/// that base path is used as the start of the file name and not only as a folder
/// — without a trailing separator the file lands **beside** the folder rather
/// than in it."*
///
/// **The rule is unconditional because the mode is a per-node setting the
/// redirect would otherwise have to read and branch on**, and a separator on a
/// property being used as a folder changes nothing. **Without it the redirect
/// produces the exact harm it exists to prevent**: the job's own file written
/// outside the job's folder, one level up and beside it, on a machine where
/// nothing else looks for it — and I2's recipe then predicts a name that is never
/// written, so the already-there test and the completeness statement both report
/// the frame absent, the redirect failing and the reckoning calling it a missing
/// frame rather than a misplaced one.
///
/// **5.0, 5.1 and 5.2 name the property `directory` and nothing in the research
/// puts them in this shape; the separator is written in every version
/// regardless**, because a rule with a version branch is a rule with a version
/// bug.
pub fn a_folder_ended_with_a_separator(folder: &[u8]) -> Vec<u8> {
    let mut out = folder.to_vec();
    if out.last() != Some(&SEPARATOR) {
        out.push(SEPARATOR);
    }
    out
}

/// **The lookup the four added write places' setting names are cited to** —
/// four running Blenders at the desk, 4.5.12, 5.0.1, 5.1.2 and 5.2.2, each
/// started with no window and from factory settings, printing every property of
/// every kind it registers. Read the way `What Build inherits` item 8(a)'s
/// lookup of 2026-09-20 was read, on the same four copies. Its record is the
/// audit file named here.
pub const THE_ADDED_PLACES_LOOKUP: &str = "the added write places' lookup, 2026-09-26 \
     (Audits/Build/Step 10c/The card make and the bake folders — what was done.md)";

/// **The lookup the fifth place's names are cited to** — the same four
/// Blenders at the desk, 4.5.12, 5.0.1, 5.1.2 and 5.2.2, each started with no
/// window and from factory settings, printing the fluid modifier's and its
/// domain settings' own descriptions: `fluid_type`'s items `NONE`, `DOMAIN`,
/// `FLOW`, `EFFECTOR`; `cache_type`'s `REPLAY`, `MODULAR`, `ALL`; and
/// `cache_directory`, label *Cache directory*, *"Directory that contains fluid
/// cache files"*, a folder path — the same in all four.
pub const THE_FLUID_CACHE_LOOKUP: &str = "the fluid cache's lookup, 2026-10-02 \
     (Audits/Build/Step 10c/Simulation fix piece 4 — what landed.md)";

/// **What carries one setting of one of D20's added write places** — the
/// objects the material sets it on, every one it finds in the file.
///
/// **D21b: each place is a named setting, not a found value**, so the setting is
/// set wherever the file carries it, whether or not the walk read a value back.
/// A file that carries none of it — a factory cube has no simulation, no nodes
/// modifier and no sequencer — has nothing to set, and nothing is written for
/// it.
///
/// **Every name the walk to each of these takes was read off all four
/// Blenders** ([`THE_ADDED_PLACES_LOOKUP`]), and each is written beside its
/// variant. The acting half renders the walk; nothing there names a setting of
/// its own.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WhatCarriesTheSetting {
    /// **The job's scene's own bake settings** — `Scene.render`, then
    /// `RenderSettings.bake`, a `BakeSettings`. One per scene.
    TheScenesBakeSettings,
    /// **Everything in the file that carries a point cache**, as the objects
    /// that hold one: every scene's `rigidbody_world`; every object's
    /// `modifiers` that carry `point_cache` (a `ClothModifier` or a
    /// `SoftBodyModifier`, in all four); every surface of a dynamic paint
    /// modifier's `canvas_settings`, in its `canvas_surfaces`; and every
    /// object's `particle_systems`. **Those five are the whole of what holds a
    /// `PointCache` in all four versions** — the lookup listed every property
    /// of every kind whose value is one.
    ///
    /// **Not all five are set. Corrected 2026-10-02 (D21b-i).** The setting is
    /// written only on a cache whose disk cache the file already has on: a
    /// cache the file keeps in memory writes nothing to disk and is left as the
    /// file has it, and **a scene's `rigidbody_world` is never set** —
    /// switching its disk cache on ended Blender with exit 139 in 4.5.12 and
    /// 5.2.2 at the desk. The rigid-body world is still read
    /// (the remap's own walk); neither this crate's reading nor the material's
    /// walk to this carrier reaches it.
    EverythingCarryingAPointCache,
    /// **Every geometry nodes modifier** — every object's `modifiers` that
    /// carry `bake_directory`, which is the `NodesModifier` alone in all four.
    EveryGeometryNodesModifier,
    /// **Every bake of every geometry nodes modifier** — its `bakes`, each a
    /// `NodesModifierBake`, which has its own `directory` beside the
    /// modifier's.
    EveryGeometryNodesBake,
    /// **The job's scene's sequence editor, where it has one** —
    /// `Scene.sequence_editor`, which is empty on a scene that never opened the
    /// sequencer. None is made where there is none.
    TheScenesSequenceEditor,
    /// **Every fluid domain whose cache type is Replay** — every object's
    /// `modifiers` whose `type` is `FLUID` and whose `fluid_type` is `DOMAIN`,
    /// with `domain_settings` (a `FluidDomainSettings`) whose `cache_type` is
    /// `REPLAY`. Added 2026-10-02 for D20's fifth place; every name read off
    /// the four Blenders ([`THE_FLUID_CACHE_LOOKUP`]). **A domain of any other
    /// cache type is not this place and is left as the file has it.**
    EveryReplayFluidDomain,
    /// **Every strip's own proxy settings, where the strip has them** — every
    /// strip in the editor's `strips_all` (*"All strips, recursively including
    /// those inside metastrips"*), and its `proxy`, a `StripProxy`.
    EveryStripsProxySettings,
}

/// One setting of an added write place, on what carries it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ASettingOfAnAddedPlace {
    pub carried_by: WhatCarriesTheSetting,
    /// **The setting's own name on each carrier**, with Blender's own label
    /// and description quoted beside it in [`the_settings_of`].
    pub setting: &'static str,
}

/// **D21b's named settings, per place: every setting that holds the place's
/// folder, and what carries each.** Read off four running Blenders
/// ([`THE_ADDED_PLACES_LOOKUP`]) and **the same in all four versions**, so this
/// takes no version.
///
/// - **The bake output path.** `filepath` on the bake settings — label *File
///   Path*, *"Image filepath to use when saving externally"*, a file path in
///   all four. **It is a file path and the map gives it a folder**: what an
///   image bake then writes there is `Job files and outputs` §14 item 22's and
///   is not established — no render bakes an image on its own.
/// - **The point cache.** `point_cache.filepath` off each carrier — label
///   *File Path*, *"Cache file path"*, a folder. It is written off the carrier,
///   the same shape as D21b-i's two further values
///   ([`crate::mapping::the_further_values_for`]), so the folder and the values
///   it needs land on the same cache.
/// - **The geometry nodes bake folder.** `bake_directory` on the modifier —
///   label *Simulation Bake Directory*, *"Location on disk where the bake data
///   is stored"*; **and** `directory` on each of its bakes — label
///   *Directory*, the same description. A bake set to its own path
///   (`use_custom_path`, *"Specify a path where the baked data should be stored
///   manually"*) reads the second and not the first, so both are set.
/// - **The sequencer proxy folder.** `proxy_dir` on the editor — label *Proxy
///   Directory*; **and** `directory` on each strip's proxy — label *Directory*,
///   *"Location to store the proxy files"*. The editor's is read where proxies
///   are stored per project (`proxy_storage`), a strip's where it keeps its own
///   (`use_proxy_custom_directory`), so both are set.
///
/// **No switch is turned by any of these.** A place's further values are
/// [`crate::mapping::the_further_values_for`]'s, which is D21b-i's *external*
/// and name for the point cache — never its disk-cache switch, since 2026-10-02
/// — and nothing for the other three; the switches named above are
/// recorded because they decide which folder Blender reads, not set.
pub fn the_settings_of(place: AnAddedWritePlace) -> &'static [ASettingOfAnAddedPlace] {
    use WhatCarriesTheSetting::*;
    match place {
        AnAddedWritePlace::TheBakeOutputPath => &[ASettingOfAnAddedPlace {
            carried_by: TheScenesBakeSettings,
            setting: "filepath",
        }],
        AnAddedWritePlace::ThePointCache => &[ASettingOfAnAddedPlace {
            carried_by: EverythingCarryingAPointCache,
            setting: "point_cache.filepath",
        }],
        AnAddedWritePlace::TheGeometryNodesBakeFolder => &[
            ASettingOfAnAddedPlace {
                carried_by: EveryGeometryNodesModifier,
                setting: "bake_directory",
            },
            ASettingOfAnAddedPlace {
                carried_by: EveryGeometryNodesBake,
                setting: "directory",
            },
        ],
        AnAddedWritePlace::TheSequencerProxyFolder => &[
            ASettingOfAnAddedPlace {
                carried_by: TheScenesSequenceEditor,
                setting: "proxy_dir",
            },
            ASettingOfAnAddedPlace {
                carried_by: EveryStripsProxySettings,
                setting: "directory",
            },
        ],
        AnAddedWritePlace::TheReplayFluidCacheFolder => &[ASettingOfAnAddedPlace {
            carried_by: EveryReplayFluidDomain,
            setting: "domain_settings.cache_directory",
        }],
    }
}

/// **The name the load-time handler keeps its record under**, in the running
/// Blender's own list of loaded modules, which a file load does not clear —
/// read at the desk on 2026-10-02: a value a handler put there was read by the
/// script after the file, in all four versions. **What the handler set, and
/// whether it ran to its end, is read from there**, by the material after the
/// file (`Rendering` L8b).
pub const THE_LOAD_TIME_RECORD: &str = "smoozy_load_time";

/// **D1a: the places set as the file loads, and never by the material after
/// it.** Added 2026-10-02, out of step 10c's desk tests.
///
/// - **The Replay fluid cache folder.** A Replay domain writes frame 1's
///   cache beside the `.blend` the moment the file opens, before any script
///   after the file runs; set as the file loads, nothing was written there
///   (4.5.12 at the desk, the map's probe; and again in 4.5.12 and 5.2.2 by
///   simulation fix piece 4).
/// - **The geometry-nodes bake folder.** Set after the file has loaded it is
///   ignored and the folder the file recorded is read; set as the file loads,
///   it is read (4.5.12 and 5.2.2). **D1a: the material sets no
///   geometry-nodes bake folder under either form of decision B** — which of
///   them the load-time handler sets is the Creator's, and `Rendering`'s
///   load-time composition holds that set.
///
/// `Rendering` L8b owns the script and its place on the line; the places,
/// their values and the map are this document's.
pub fn is_set_as_the_file_loads(place: AnAddedWritePlace) -> bool {
    matches!(
        place,
        AnAddedWritePlace::TheReplayFluidCacheFolder | AnAddedWritePlace::TheGeometryNodesBakeFolder
    )
}

/// One value the material sets in the running Blender.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AnAssignment {
    /// **Which destination this sets**, keyed to the destination the reading
    /// found — or, for one of D20's four, to the named setting (P10, D21b).
    pub destination: Destination,
    /// **The folder it is set to**, under the job's folder, as bytes.
    ///
    /// **For a file output node's folder this already carries D8's trailing
    /// separator** — or, for 4.5's gathering node, the artist's name front
    /// after it (D8 as corrected 2026-10-04) — so nothing further along has to
    /// remember either.
    pub folder: Vec<u8>,
    /// **The property the folder is written to**, where the version names one
    /// (D8). Absent for the destinations whose setting is not a node property —
    /// **D20's four added write places, whose settings are [`the_settings_of`]
    /// the place**, each on what carries it rather than on one node.
    pub property: Option<String>,
    /// **P10's further values, applied with the folder** — for the physics point
    /// cache that is D21b-i's *external* and name with the folder (four values,
    /// the disk-cache switch first, until 2026-10-02), and setting the folder
    /// alone is the observed silent no-op.
    pub further_values: Vec<FurtherValue>,
}

/// **D11: the window anything delivered on the command line sits in, and it has
/// three edges.**
///
/// *"After the file, before the render instruction, and before any bare double
/// dash."* Outside that window it is silently ignored, with no message and no
/// failure.
///
/// **The third edge was added 2026-09-01** out of the repaired Blender research:
/// **after a bare double dash the line is Cycles' and Blender reads nothing of
/// its own there**, so the double dash closes the window as surely as the render
/// instruction does. **A redirect argument written past the third would be
/// dropped in silence with the render going ahead at exit 0.** `Rendering` owns
/// the line; this document states the window because the redirect's own argument
/// sits in it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TheEdgesOfTheWindow {
    AfterTheFile,
    BeforeTheRenderInstruction,
    BeforeAnyBareDoubleDash,
}

/// The three edges, as one list. **There is no fourth and there were only two
/// until the repaired research settled the third.**
pub const THE_THREE_EDGES: [TheEdgesOfTheWindow; 3] = [
    TheEdgesOfTheWindow::AfterTheFile,
    TheEdgesOfTheWindow::BeforeTheRenderInstruction,
    TheEdgesOfTheWindow::BeforeAnyBareDoubleDash,
];

/// **D10: the regular output path is settled by its own command-line argument
/// rather than by the material**, because it has one in every version and *"the
/// `.blend` on disk is left byte for byte as it was"*.
///
/// **It is still one of the destinations the map covers** — the map decides the
/// value, the argument delivers it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TheRegularOutputPathsArgument {
    /// **The raw text handed in, template marks and all.**
    ///
    /// *Writing 13* settles the ordering this rests on: with the file, then the
    /// setting, then the script — `Rendering` L8's own order — the script reads
    /// the setting the caller handed in. **The value the material reads back is
    /// the raw text handed in**, so the comparison is against this as text and
    /// never against a resolved file name.
    pub value: Vec<u8>,
}

/// **D16 and D16a: the marker the material's first act prints on the render's own
/// output.**
///
/// **D17 is why it exists and D17 is also what it does not claim.** Observed on
/// 2026-08-16: a run whose launch script sat in a folder the login could not read
/// printed `Permission denied`, **rendered the factory scene anyway** at the
/// wrong size on the wrong engine, wrote a file, and **exited 0** — and [Open
/// Threads] states outright that *"until it is answered, no architecture document
/// may rest on a launch script failing loudly."* **So nothing here rests on it.
/// The marker is a positive signal whose absence is readable; it does not make
/// the failure loud and is not offered as doing so.**
///
/// **The streams are captured and retained by `Rented-server runtime` under Row
/// 286**, so this opens no reporting channel of its own.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TheMarker {
    /// The job identity.
    pub job: String,
    /// **Each destination it set**, by the destination's own origin and the
    /// folder it was set to.
    pub what_it_set: Vec<(ArtistsText, Vec<u8>)>,
    /// **The value it read back for the regular output path** (D10).
    ///
    /// Absent only where the read-back could not be taken at all, which is a
    /// state and not an empty value.
    pub the_value_read_back_for_the_regular_output_path: Option<Vec<u8>>,
    /// **D16a: the fields `Dependency Upload & Remap` writes onto this same
    /// marker, per applied pair.**
    ///
    /// *"The remap material reports every pair in this same marker (`Dependency
    /// Upload & Remap` M8), so D16's list is short by two fields without them;
    /// both are settled in that document and neither is re-decided here."* The
    /// per-pair outcome, and whether assignment alone is *established* for that
    /// pair's kind (`Dependency Upload & Remap` M9d), are that document's own
    /// values and are **carried and not interpreted here**.
    pub what_the_remap_wrote_onto_it: Vec<crate::manifest::CarriedAndNotInterpretedHere>,
}

/// **D9: the material carries exactly this, and nothing that could be an
/// argument instead.**
///
/// - the **destination map** from the recorded reading — one new folder per
///   destination, keyed to the destination the reading found, **except D20's four
///   added write places, which are keyed to the named setting and are in the map
///   whether or not the walk read a value back (D21b)**; **and where a place
///   takes more than a folder to be set, the further values that place requires
///   go with its folder in the material and are applied with it**;
/// - the **remap material** `Dependency Upload & Remap` owns, applied by the same
///   lever at the same moment;
/// - the **marker** of §5.4;
/// - the **version it was composed for**, per D7.
///
/// **D9a: *nothing that could be an argument instead* is a rule about route, not
/// a ceiling on the list.** A value that **has** a command-line argument in the
/// version the job runs is delivered by that argument and never by the material;
/// a value that has none in that version may only be delivered by the material.
/// **`Rendering` L10 composes into this same material, for the version in hand,
/// every settings-payload value with no argument in that version**, Simplify's
/// switch with its three settings, and the skip-existing-frames switch — each
/// inside D9's rule rather than against it. **The composition is `Rendering`'s
/// and is not re-decided or copied here**; what D9 fixes is what *this* document
/// puts in, and that the material never carries a value an argument could have
/// carried.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TheLaunchMaterial {
    /// **D7: the material is composed for one named Blender version and names
    /// it.**
    pub composed_for: TheVersionInHand,
    /// The destination map applied as assignments, one per mappable destination
    /// the material sets.
    pub assignments: Vec<AnAssignment>,
    /// **D10's one pair the material does not set**: it reads this value back
    /// and fails the attempt where it does not match.
    pub the_regular_output_path: Option<TheRegularOutputPathsArgument>,
    /// The remap material, **carried and not interpreted here** — it is
    /// `Dependency Upload & Remap`'s and reaches Blender by the same lever at
    /// the same moment.
    pub remap_material: crate::manifest::CarriedAndNotInterpretedHere,
    /// The job identity the marker names.
    pub job: String,
}

/// **D7: the version is confirmed before Blender is launched.**
///
/// *"The render worker confirms that the Blender it is about to launch is the
/// version the manifest names, and where it is not **the attempt fails before the
/// render instruction** and no launch happens."* That requirement sits on the
/// launch, which is `Rendering`'s under D6.
///
/// **Where the material is handed to another version anyway it does nothing** —
/// which is a second line of defence and not the mechanism, **because D17 forbids
/// this document resting on a launch failing loudly.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhetherTheLaunchMayGoAhead {
    ItMay,
    /// The attempt fails before the render instruction and no launch happens.
    TheBlenderIsNotTheVersionTheManifestNames,
}

/// **D7, performed.**
pub fn whether_the_launch_may_go_ahead(
    the_version_the_manifest_names: TheVersionInHand,
    the_version_about_to_be_launched: TheVersionInHand,
) -> WhetherTheLaunchMayGoAhead {
    if the_version_the_manifest_names == the_version_about_to_be_launched {
        WhetherTheLaunchMayGoAhead::ItMay
    } else {
        WhetherTheLaunchMayGoAhead::TheBlenderIsNotTheVersionTheManifestNames
    }
}

/// **D9, composed from the map and from nothing else.**
///
/// **D13: the redirect reaches each destination's own setting individually, and
/// no bulk route is used for any part of it.** The bulk commands that make every
/// path relative or absolute, and the file's own list of outside files, *"reach
/// an ocean modifier's cache path and nothing else a file records for writing"*,
/// and the one-pass route that visits every recorded path **is absent in 4.5.9
/// altogether**, while in 5.0.1 and 5.2.0 it rewrote the four images, the text
/// and the library and **silently left every write location alone**, with nothing
/// raised, nothing printed and the save succeeding. **One place out of six is not
/// a bulk route and no part of the redirect is given to one.**
///
/// **D14: that is the negative the question named and it is why no route is
/// trusted without checking what it reached.** *"A redirect built on that route
/// alone would appear to work and leave every place the job writes pointing at
/// the artist's own machine."*
pub fn compose_the_launch_material(
    map: &TheMap,
    version: TheVersionInHand,
    job: &str,
    remap_material: crate::manifest::CarriedAndNotInterpretedHere,
) -> TheLaunchMaterial {
    let mut assignments = Vec::new();
    let mut the_regular_output_path = None;
    for pair in &map.pairs {
        let HowItMapped::UnderTheJobsFolder {
            new_folder,
            further_values,
        } = &pair.outcome
        else {
            // **P13: a job with any destination recorded as not mappable is held
            // under §9.1**, so nothing is composed for one. *"There is no partial
            // redirect that proceeds quietly."*
            continue;
        };
        match &pair.destination.kind {
            DestinationKind::TheRegularOutputPath => {
                // **D10a: the job's own scene's pair, and only that.** A map
                // written before `Job files and outputs` R4 item 3's correction
                // holds a pair for each strip scene, and the last one used to
                // win here: the live pass on real production files, a good
                // video ending *failed*.
                if pair.destination.scene != crate::reading::WhichScene::TheJobsOwnScene {
                    continue;
                }
                // **D10: the map decides the value, the argument delivers it.**
                the_regular_output_path = Some(TheRegularOutputPathsArgument {
                    value: the_arguments_value(new_folder, &pair.destination),
                });
            }
            DestinationKind::AnOutputOfAFileOutputNode { .. }
            | DestinationKind::AWholeFileOutputNodeGatheringIntoOneFile { .. } => {
                assignments.push(AnAssignment {
                    destination: pair.destination.clone(),
                    // **D8 as corrected 2026-10-04 (piece 1b): in 4.5 a node
                    // gathering into one file keeps the name front the
                    // artist wrote after the mapped folder**; every other
                    // node's folder takes the separator, as always.
                    folder: the_value_a_nodes_folder_property_takes(
                        new_folder,
                        &pair.destination,
                        version,
                    ),
                    property: Some(the_nodes_folder_property(version).to_string()),
                    further_values: further_values.clone(),
                });
            }
            DestinationKind::AnAddedWritePlace(_) => {
                assignments.push(AnAssignment {
                    destination: pair.destination.clone(),
                    folder: new_folder.clone(),
                    // **The settings are the place's own, on what carries
                    // them** — [`the_settings_of`], read off four Blenders
                    // and the same in all four — so there is no one node
                    // property to name here.
                    property: None,
                    further_values: further_values.clone(),
                });
            }
        }
    }
    TheLaunchMaterial {
        composed_for: version,
        assignments,
        the_regular_output_path,
        remap_material,
        job: job.to_string(),
    }
}

/// **The value the regular output path's own argument carries.**
///
/// P4's third bullet: the setting is one piece of text holding the folder and the
/// front of the name together, and Blender itself splits it at the last
/// separator. **So the argument's value is the mapped folder joined to the name
/// front the artist wrote**, and the name front is not touched (N12: Smoozy
/// writes no output name).
///
/// **Where the reading replaced the folder, a literal brace left in the mapped
/// folder is written doubled** (added 2026-10-18, `Job files and outputs` I2's
/// *The regular output path's folder is read the same way*), for the reason
/// [`the_value_a_nodes_folder_property_takes`] gives: Blender reads the value
/// for expressions again. **The front is handed on as recorded**, expressions
/// and all, since Blender replaces them in the name it is asked for (I1). An
/// unresolved folder, and a reading taken before that date, are handed on as
/// they are.
fn the_arguments_value(new_folder: &[u8], destination: &Destination) -> Vec<u8> {
    let front = destination
        .recorded_location
        .as_ref()
        .map(|location| crate::mapping::split_the_regular_output_path(location.as_bytes()).1)
        .unwrap_or_default();
    let the_reading_replaced_the_folder = matches!(
        destination.the_location_with_its_folder_replaced,
        Some(crate::reading::WithItsBracesReplaced::Replaced(_))
    );
    if the_reading_replaced_the_folder {
        the_mapped_folder_and_its_name_front(&crate::reading::with_its_braces_doubled(new_folder), front)
    } else {
        the_mapped_folder_and_its_name_front(new_folder, front)
    }
}

/// **The value a file output node's folder property is set to.**
///
/// **D8 as corrected 2026-10-04, by piece 1b, on the runner's ruling that what
/// lands is what the artist named.** In 4.5, a node gathering into one file
/// holds the folder and the front of its file name in that one property
/// ([`crate::mapping::it_holds_a_name_front`]), so the value is the mapped
/// folder, a separator, and the front exactly as the artist wrote it —
/// `//../Render/EXR/### - Hero - Box` becomes `<job>/EXR/### - Hero - Box`, and
/// Blender writes `<job>/EXR/001 - Hero - Box.exr`, the name the file left
/// alone writes (observed at the desk in 4.5.12). **Where the artist's
/// property ends with a separator the front is empty and the value is D8's
/// separator-ended folder**, so the harm D8 exists for — the file landing
/// beside the job's folder — cannot come back: the folder Blender is given is
/// always under the job's folder. Every other node's property, and every node
/// from 5.0, is a folder and takes [`a_folder_ended_with_a_separator`].
///
/// **Where the reading replaced the node's folder, a literal brace left in it
/// is written doubled** (added 2026-10-18, `Job files and outputs` I2's *A
/// node's folder is read the same way*). The folder was mapped with its
/// expressions put in, and Blender reads the value it is handed for
/// expressions again, so the one brace a doubled one became would otherwise
/// be read as the start of one; doubled, Blender writes it as the one brace
/// the inventory predicts (*Dependencies 5*: *"a literal brace written
/// doubled"*). An unresolved folder, and a reading taken before that date,
/// are handed on as they are.
pub fn the_value_a_nodes_folder_property_takes(
    new_folder: &[u8],
    destination: &Destination,
    version: TheVersionInHand,
) -> Vec<u8> {
    let value = match the_name_front_of(destination, version) {
        Some(front) => the_mapped_folder_and_its_name_front(new_folder, front),
        None => a_folder_ended_with_a_separator(new_folder),
    };
    let the_reading_replaced_the_folder = destination
        .the_replaced_text()
        .is_some_and(|replaced| replaced.the_folder.the_text().is_some());
    if the_reading_replaced_the_folder {
        crate::reading::with_its_braces_doubled(&value)
    } else {
        value
    }
}

/// **D10's read-back, and what it does with a mismatch.**
///
/// *"Running inside the same Blender inside the same window, the material reads
/// the value in force for the regular output path at the moment it runs, prints
/// it in D16's marker, and **fails the attempt where it does not match the map's
/// pair**."*
///
/// **That is what makes §12's unit atomic over a pair no material sets**, and it
/// is not the material handling a failure of its own (D18) — **it raises one**.
/// **The raise ends the run only because `Rendering` L8a places the
/// failure-to-exit-status argument at the head of the launch line** — observed
/// 2026-08-19 in 4.5.12 and 5.2.0, a launch script that raises otherwise printing
/// its traceback, rendering anyway and exiting 0.
///
/// **Two things the answer binds this to.** The value read back is the **raw text
/// handed in**, template marks and all, so the comparison is text against text.
/// And an imposed output path reaches the **active scene alone** (§14 item 19a):
/// where the named scene is not the one the path reached, **this read-back is
/// what stops the attempt**.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatTheReadBackFound {
    ItMatchesTheMapsPair,
    /// **The attempt fails.** It is a raise and never a caught failure (D18).
    ItDoesNotMatch,
    /// The read-back could not be taken at all. **The attempt fails on this
    /// too**: a pair that was neither set nor verified is a map applied in part,
    /// which §12 forbids.
    ItCouldNotBeTaken,
}

/// **D10's read-back, performed as text against text.**
pub fn what_the_read_back_found(
    the_maps_pair: Option<&TheRegularOutputPathsArgument>,
    what_was_in_force: Option<&RecordedLocation>,
) -> WhatTheReadBackFound {
    let Some(expected) = the_maps_pair else {
        // No regular output path in the map is no pair to verify.
        return WhatTheReadBackFound::ItMatchesTheMapsPair;
    };
    let Some(in_force) = what_was_in_force else {
        return WhatTheReadBackFound::ItCouldNotBeTaken;
    };
    if in_force.as_bytes() == expected.value.as_slice() {
        WhatTheReadBackFound::ItMatchesTheMapsPair
    } else {
        WhatTheReadBackFound::ItDoesNotMatch
    }
}

/// **D18: the material contains no failure handling of its own.**
///
/// *"A failure raised inside a script that catches it leaves a successful exit
/// status behind"*, so anything the material catches becomes a job that reports
/// success over a render that did not do what was asked.
///
/// **Held where it can be checked**: nothing in [`TheLaunchMaterial`] is a
/// recovery, a fallback or a default, and [`what_the_read_back_found`] returns a
/// finding for the caller to raise on rather than handling one.
pub const THE_MATERIAL_CONTAINS_NO_FAILURE_HANDLING_OF_ITS_OWN: bool = true;

/// **D15: one trap the material must avoid, recorded for Build.**
///
/// From 5.0, **adding an output to a file output node rebuilds the node's socket
/// list**, so a held socket reference goes stale and links land on the trailing
/// extend socket with no error. **Nothing in the redirect adds an output**; the
/// trap is recorded because anything that composes a node's values in the same
/// pass could.
pub const NOTHING_IN_THE_REDIRECT_ADDS_AN_OUTPUT_TO_A_NODE: bool = true;

/// **D19: the placement of the material is confirmed by reading it back under
/// the login the render will run as** (L12), which closes the one shape of D17's
/// hazard that is Smoozy's own doing.
///
/// **Under L13's one login the read-back is a check against a placement fault** —
/// material that did not land, or landed somewhere else — **and never against a
/// permission difference between two logins**, of which there are none. **It does
/// not close the hazard** (§14 item 8).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatTheConfirmationOfThePlacementFound {
    /// The bytes read back are the bytes written.
    TheMaterialIsWhereItWasPut,
    /// **A placement fault**: it did not land, or landed somewhere else.
    ItIsNotThere,
}

/// **D19, performed.** The comparison is over bytes, so nothing is decoded on the
/// way.
pub fn what_the_confirmation_found(
    written: &[u8],
    read_back: &[u8],
) -> WhatTheConfirmationOfThePlacementFound {
    if written == read_back {
        WhatTheConfirmationOfThePlacementFound::TheMaterialIsWhereItWasPut
    } else {
        WhatTheConfirmationOfThePlacementFound::ItIsNotThere
    }
}

/// **D21: one residue is not reached and it is not disclosed either, because it
/// is never found.**
///
/// A path held in a **custom property** is reached by no route this stage has —
/// the flat list and the relative sweep miss it, and the walk over Blender's own
/// path marks misses it too. **So it is not *found and not mapped*; it is not
/// found at all.** It cannot be recorded, cannot be shown, and cannot hold a job,
/// **and this document says so rather than claiming a disclosure it cannot
/// deliver** (§14 item 12).
///
/// **What the reading does find is disclosed**: a destination the map records as
/// *not mappable* is carried in the recorded reading with its reason and reaches
/// `Queue & History` as a field of it, so nothing the reading found is hidden.
pub const THE_CUSTOM_PROPERTY_RESIDUE_IS_NOT_FOUND_AND_IS_NOT_DISCLOSED: bool = true;

/// **D22: a location the redirect did not reach does not fail loudly either.**
///
/// *"A whole foreign folder tree shaped for the artist's own computer is built at
/// the top of the rented machine's disk and written into, silently, and the
/// render reports success."* Observed again on 2026-08-17: the rewritten copy
/// wrote ten files all under the new folder with `ls: cannot access
/// '/home/artist': No such file or directory`, **while the unrewritten fixture
/// built that foreign tree silently**.
pub const A_LOCATION_THE_REDIRECT_DID_NOT_REACH_DOES_NOT_FAIL_LOUDLY: bool = true;

/// **D3 and D4, recorded because they are what closed the choice.**
///
/// **D3: the condition the copy was parked against has gone.** There is no
/// command-line argument for a file output node's place in any version, *"but a
/// script handed to Blender at launch reaches the node's values, and the render
/// that follows uses them"* — hand-confirmed in all four versions on the Linux
/// machine.
///
/// **D4: setting values at launch does not change how the render is started, and
/// that clash is already settled.** *"Setting a value from a script is not the
/// same as starting the render from one."* `Rendering`'s Row 11 keeps the
/// reporting route; **this document adds nothing to it and takes nothing from
/// it.**
pub const THE_LAUNCH_MATERIAL_SETS_VALUES_AND_STARTS_NO_RENDER: bool = true;

/// **D12: from 5.2 a switch inside the file stops the scene's own frames being
/// written even when a path is handed in on the command line, while a node in the
/// same tree keeps writing.**
///
/// **The reading records that switch** (R4 item 1) **and the count and the
/// inventory are composed from what it says**, so the redirect never promises a
/// file that switch will prevent. The composition is
/// [`crate::inventory::compose_the_inventory`]'s and nothing here reads the
/// switch a second time.
pub const THE_REDIRECT_NEVER_PROMISES_A_FILE_THE_SAVE_OUTPUT_SWITCH_WILL_PREVENT: bool = true;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::inventory::tests::{a_node_destination, mapped, text, JOB_FOLDER};
    use crate::mapping::{compose_the_map, ADestinationToMap, FurtherValueIs};
    use crate::reading::tests::a_destination;
    use crate::reading::{AnAddedWritePlace, WhereItComesFrom, WhichScene};

    fn nothing_carried() -> crate::manifest::CarriedAndNotInterpretedHere {
        crate::manifest::CarriedAndNotInterpretedHere(serde_json::json!({}))
    }

    fn the_map_for(destinations: &[Destination], version: TheVersionInHand) -> TheMap {
        let held: Vec<ADestinationToMap<'_>> = destinations
            .iter()
            .map(|destination| ADestinationToMap {
                destination,
                the_longest_name_it_will_write: Some(b"beauty0010.png".to_vec()),
            })
            .collect();
        compose_the_map(
            &held,
            b"/ephemeral/smoozy/assets/set-1",
            JOB_FOLDER,
            version,)
    }

    #[test]
    fn a_nodes_folder_always_ends_with_a_separator_in_every_version() {
        // D8. Without it, in 4.5's gathering mode, the job's own file lands
        // beside the job's folder rather than in it — the exact harm the
        // redirect exists to prevent — and I2's recipe then predicts a name that
        // is never written.
        for version in [TheVersionInHand::of(4, 5), TheVersionInHand::of(5, 2)] {
            let node = a_node_destination("Beauty", "beauty", "/home/a/out", None);
            let map = the_map_for(&[node], version);
            let material = compose_the_launch_material(&map, version, "job-7", nothing_carried());
            let folder = &material.assignments[0].folder;
            assert_eq!(*folder.last().unwrap(), SEPARATOR, "{version:?}");
        }
        // And a folder that already ends with one does not gain a second.
        assert_eq!(a_folder_ended_with_a_separator(b"/j/out/"), b"/j/out/".to_vec());
    }

    fn a_gathering_node(node: &str, at: &str, base: Option<&str>) -> Destination {
        let mut destination = a_destination(
            DestinationKind::AWholeFileOutputNodeGatheringIntoOneFile {
                the_effect: None,
                node: text(node),
                the_nodes_base_file_name: base.map(text),
                replaced: None,
            },
            at,
            WhereItComesFrom::AFileOutputNodeNamed(text(node)),
        );
        destination.format = Some(crate::reading::Format {
            as_the_file_records_it: text("OPEN_EXR_MULTILAYER"),
            the_ending_it_writes: Some(text(".exr")),
            is_video_shaped: false,
        });
        destination
    }

    /// **D8 as corrected 2026-10-04 (piece 1b): in 4.5 a node gathering into
    /// one file keeps the artist's name after its mapped folder**, the map
    /// takes the part before as its folder, and I2's recipe predicts the name
    /// Blender writes — the three from one rule. Observed at the desk in
    /// 4.5.12: `//../Render/EXR/### - Hero - Box` writes
    /// `EXR/001 - Hero - Box.exr` left alone, and the separator alone made it
    /// `EXR/### - Hero - Box/0001.exr`. **From 5.0 nothing moves.**
    #[test]
    fn a_gathering_node_keeps_the_artists_file_name_in_every_version() {
        let png = || a_node_destination("PNG Box", "### - Hero - Box", "/home/a/Render/PNG", None);
        let cases = [
            (
                TheVersionInHand::of(4, 5),
                a_gathering_node("EXR Box", "/home/a/Render/EXR/### - Hero - Box", None),
                format!("{JOB_FOLDER}/EXR/### - Hero - Box"),
                format!("{JOB_FOLDER}/EXR/001 - Hero - Box.exr"),
            ),
            (
                TheVersionInHand::of(5, 2),
                a_gathering_node("EXR Box", "/home/a/Render/EXR", Some("### - Hero - Box")),
                format!("{JOB_FOLDER}/EXR/"),
                format!("{JOB_FOLDER}/EXR/001 - Hero - Box.exr"),
            ),
            // **A property the artist ended with a separator has no name
            // front**, and takes D8's separator-ended folder, as before.
            (
                TheVersionInHand::of(4, 5),
                a_gathering_node("EXR Box", "/home/a/Render/EXR/", None),
                format!("{JOB_FOLDER}/EXR/"),
                format!("{JOB_FOLDER}/EXR/0001.exr"),
            ),
        ];
        for (version, gathering, value, name) in cases {
            let reading = crate::inventory::tests::a_reading(
                vec![png(), gathering],
                vec![crate::inventory::tests::a_frame(1, &["Box"], None)],
                version,
            );
            let map = mapped(&reading, version);
            assert_eq!(
                map.the_folders_the_job_will_write_into(JOB_FOLDER),
                vec![b"PNG".to_vec(), b"EXR".to_vec()],
                "{version:?}"
            );
            let material = compose_the_launch_material(&map, version, "job-7", nothing_carried());
            assert_eq!(material.assignments[0].folder, format!("{JOB_FOLDER}/PNG/").into_bytes());
            assert_eq!(
                String::from_utf8_lossy(&material.assignments[1].folder),
                value,
                "{version:?}"
            );
            let crate::inventory::WhatTheInventoryIs::It(inventory) =
                crate::inventory::compose_the_inventory(&reading, &map)
            else {
                panic!("the inventory is composed");
            };
            let predicted: Vec<String> = inventory
                .entries
                .iter()
                .map(|entry| String::from_utf8_lossy(&entry.predicted_path).into_owned())
                .collect();
            assert!(predicted.contains(&name), "{version:?}: {predicted:?}");
            assert!(predicted.contains(&format!("{JOB_FOLDER}/PNG/001 - Hero - Box.png")));
        }
    }

    #[test]
    fn the_property_the_folder_is_written_to_moves_across_the_version_boundary() {
        // D8, observed on disk by a separate process, per version.
        assert_eq!(the_nodes_folder_property(TheVersionInHand::of(4, 5)), "base_path");
        for minor in [0u16, 1, 2] {
            assert_eq!(
                the_nodes_folder_property(TheVersionInHand::of(5, minor)),
                "directory"
            );
        }
    }

    #[test]
    fn the_regular_output_path_is_an_argument_and_never_an_assignment() {
        // D10: it has a command-line argument in every version, so D9a's rule
        // about route keeps it out of the material.
        let regular = a_destination(
            DestinationKind::TheRegularOutputPath,
            "/home/a/out/f_####",
            WhereItComesFrom::TheScenesOutputPath,
        );
        let version = TheVersionInHand::of(5, 2);
        let map = the_map_for(&[regular], version);
        let material = compose_the_launch_material(&map, version, "job-7", nothing_carried());
        assert!(material.assignments.is_empty());
        // The raw text the artist wrote is kept, template marks and all.
        assert_eq!(
            material.the_regular_output_path.as_ref().unwrap().value,
            format!("{JOB_FOLDER}/f_####").into_bytes()
        );
    }

    /// **`Job files and outputs` D10a, out of the live pass on real production
    /// files.** A map written before `Job files and outputs` R4 item 3's
    /// correction holds a pair for a strip scene's regular output path, nine of
    /// them on that file, and they came last: the job wrote into `tmp/` while
    /// the count looked in the job's own scene's folder.
    #[test]
    fn a_strip_scenes_pair_gives_no_argument_whichever_order_the_map_holds_it_in() {
        let own = a_destination(
            DestinationKind::TheRegularOutputPath,
            "/home/a/sync/board-v002.mp4",
            WhereItComesFrom::TheScenesOutputPath,
        );
        let mut strips = a_destination(
            DestinationKind::TheRegularOutputPath,
            "/tmp/",
            WhereItComesFrom::TheScenesOutputPath,
        );
        strips.scene = crate::reading::WhichScene::ReachedThroughASequencerStrip {
            scene: crate::reading::ArtistsText::of(b"Scene.036".to_vec()),
            strip: crate::reading::ArtistsText::of(b"Scene.036".to_vec()),
        };
        let version = TheVersionInHand::of(5, 3);
        for destinations in [[own.clone(), strips.clone()], [strips.clone(), own.clone()]] {
            let map = the_map_for(&destinations, version);
            let material = compose_the_launch_material(&map, version, "job-7", nothing_carried());
            let HowItMapped::UnderTheJobsFolder { new_folder, .. } = &map.the_pair_for(&own).expect("its pair").outcome else {
                panic!("it maps");
            };
            let value = material.the_regular_output_path.as_ref().expect("the argument").value.clone();
            assert_eq!(value, [new_folder.as_slice(), b"/board-v002.mp4"].concat());
            assert!(material.assignments.is_empty(), "{:?}", material.assignments);
        }
    }

    #[test]
    fn the_read_back_compares_raw_text_and_fails_the_attempt_on_a_mismatch() {
        // D10, and §12's *every pair is either set or verified, or the attempt
        // fails before the render instruction*.
        let pair = TheRegularOutputPathsArgument {
            value: b"/j/out/f_####".to_vec(),
        };
        assert_eq!(
            what_the_read_back_found(
                Some(&pair),
                Some(&RecordedLocation::of(b"/j/out/f_####".to_vec()))
            ),
            WhatTheReadBackFound::ItMatchesTheMapsPair
        );
        // A resolved file name is not the raw text and is not accepted for it.
        assert_eq!(
            what_the_read_back_found(
                Some(&pair),
                Some(&RecordedLocation::of(b"/j/out/f_0001.png".to_vec()))
            ),
            WhatTheReadBackFound::ItDoesNotMatch
        );
        // §14 item 19a's residue: the imposed path reached another scene, so the
        // value in force is the file's own.
        assert_eq!(
            what_the_read_back_found(
                Some(&pair),
                Some(&RecordedLocation::of(b"/home/a/out/f_####".to_vec()))
            ),
            WhatTheReadBackFound::ItDoesNotMatch
        );
        assert_eq!(
            what_the_read_back_found(Some(&pair), None),
            WhatTheReadBackFound::ItCouldNotBeTaken
        );
    }

    #[test]
    fn the_point_caches_values_travel_with_its_folder_in_the_material() {
        // D21b-i and P10: setting the folder alone is the observed silent no-op
        // — the render finished, reported success, and wrote 250 files beside
        // the `.blend`, outside the job's folder, destroyed with the machine.
        // **And the disk-cache switch is not among them** (corrected
        // 2026-10-02): writing it where it is already on deletes the cache's
        // files.
        let cache = Destination {
            kind: DestinationKind::AnAddedWritePlace(AnAddedWritePlace::ThePointCache),
            recorded_location: None,
            format: None,
            origin: WhereItComesFrom::AnAddedWritePlaceNamed(AnAddedWritePlace::ThePointCache),
            scene: WhichScene::TheJobsOwnScene,
            the_view_layers_that_feed_it: Vec::new(),
            the_location_with_its_folder_replaced: None,
        };
        let version = TheVersionInHand::of(5, 2);
        let map = the_map_for(&[cache], version);
        let material = compose_the_launch_material(&map, version, "job-7", nothing_carried());
        let further = &material.assignments[0].further_values;
        assert!(further
            .iter()
            .any(|value| value.setting.ends_with("use_external")
                && value.value == FurtherValueIs::On));
        assert!(!further
            .iter()
            .any(|value| value.setting.ends_with("use_disk_cache")));
    }

    /// **D21b: every one of the four has its setting named**, read off four
    /// Blenders, so nothing is left for the material to refuse. **Until
    /// 2026-09-26 none was**, and step 10c's sixth pass on Vast found that a
    /// factory cube carries the bake output path — so every job, on either
    /// device, ended before the render with *Smoozy has no name for the folder
    /// setting of one of this file's write places (the bake output folder)*.
    #[test]
    fn every_added_write_place_has_its_folder_setting_named_on_what_carries_it() {
        use crate::reading::EVERY_ADDED_WRITE_PLACE;
        for place in EVERY_ADDED_WRITE_PLACE {
            let settings = the_settings_of(place);
            assert!(!settings.is_empty(), "{place:?} has no setting named");
            for one in settings {
                assert!(
                    one.setting
                        .split('.')
                        .all(|piece| !piece.is_empty()
                            && piece.chars().all(|c| c.is_ascii_lowercase() || c == '_')),
                    "{place:?}: `{}` is not a plain name",
                    one.setting
                );
            }
        }
        // **The point cache's folder is written off the same carrier as
        // D21b-i's further values**, so all of them land on one cache.
        let folder = the_settings_of(AnAddedWritePlace::ThePointCache)[0].setting;
        for further in crate::mapping::the_further_values_for(AnAddedWritePlace::ThePointCache)
        {
            assert_eq!(
                further.setting.split('.').next(),
                folder.split('.').next(),
                "{} and {folder} are on two different objects",
                further.setting
            );
        }
        // **And the assignment still names no single node property** — the
        // settings are the place's, on what carries them.
        let bake = Destination {
            kind: DestinationKind::AnAddedWritePlace(AnAddedWritePlace::TheBakeOutputPath),
            recorded_location: None,
            format: None,
            origin: WhereItComesFrom::AnAddedWritePlaceNamed(AnAddedWritePlace::TheBakeOutputPath),
            scene: WhichScene::TheJobsOwnScene,
            the_view_layers_that_feed_it: Vec::new(),
            the_location_with_its_folder_replaced: None,
        };
        let version = TheVersionInHand::of(5, 2);
        let map = the_map_for(&[bake], version);
        let material = compose_the_launch_material(&map, version, "job-7", nothing_carried());
        assert_eq!(material.assignments.len(), 1);
        assert_eq!(material.assignments[0].property, None);
    }

    #[test]
    fn a_destination_that_is_not_mappable_is_composed_for_at_all() {
        // P13: there is no partial redirect that proceeds quietly over a
        // destination the rule covers, so nothing is set for one and the job is
        // held instead.
        let long = format!("/home/a/{}", "n".repeat(250));
        let destination = a_destination(
            DestinationKind::TheRegularOutputPath,
            &format!("{long}/f_"),
            WhereItComesFrom::TheScenesOutputPath,
        );
        let destinations = [destination];
        let held = [ADestinationToMap {
            destination: &destinations[0],
            the_longest_name_it_will_write: Some(b"x".repeat(300)),
        }];
        let version = TheVersionInHand::of(5, 2);
        let map = compose_the_map(
            &held,
            b"/ephemeral/smoozy/assets/set-1",
            JOB_FOLDER,
            version,);
        let material = compose_the_launch_material(&map, version, "job-7", nothing_carried());
        assert!(material.assignments.is_empty());
        assert!(material.the_regular_output_path.is_none());
    }

    #[test]
    fn the_launch_does_not_go_ahead_where_the_blender_is_not_the_version_the_manifest_names() {
        // D7, and where the material is handed to another version anyway it does
        // nothing — a second line of defence and not the mechanism, because D17
        // forbids resting on a launch failing loudly.
        assert_eq!(
            whether_the_launch_may_go_ahead(
                TheVersionInHand::of(5, 2),
                TheVersionInHand::of(5, 2)
            ),
            WhetherTheLaunchMayGoAhead::ItMay
        );
        assert_eq!(
            whether_the_launch_may_go_ahead(
                TheVersionInHand::of(5, 2),
                TheVersionInHand::of(4, 5)
            ),
            WhetherTheLaunchMayGoAhead::TheBlenderIsNotTheVersionTheManifestNames
        );
    }

    #[test]
    fn the_window_has_three_edges_and_not_two() {
        // D11. A redirect argument written past the third edge would be dropped
        // in silence with the render going ahead at exit 0.
        assert_eq!(THE_THREE_EDGES.len(), 3);
        assert!(THE_THREE_EDGES.contains(&TheEdgesOfTheWindow::BeforeAnyBareDoubleDash));
    }

    #[test]
    fn the_placement_is_confirmed_by_reading_the_material_back() {
        // D19, over bytes.
        assert_eq!(
            what_the_confirmation_found(b"material", b"material"),
            WhatTheConfirmationOfThePlacementFound::TheMaterialIsWhereItWasPut
        );
        assert_eq!(
            what_the_confirmation_found(b"material", b""),
            WhatTheConfirmationOfThePlacementFound::ItIsNotThere
        );
    }

    #[test]
    fn the_marker_names_the_job_what_was_set_and_the_value_read_back() {
        // D16, and D16a's two further fields, which ride this same marker.
        let marker = TheMarker {
            job: "job-7".to_string(),
            what_it_set: vec![(text("Beauty"), b"/j/out/".to_vec())],
            the_value_read_back_for_the_regular_output_path: Some(b"/j/f_####".to_vec()),
            what_the_remap_wrote_onto_it: vec![nothing_carried()],
        };
        assert_eq!(marker.what_it_set.len(), 1);
        assert!(marker
            .the_value_read_back_for_the_regular_output_path
            .is_some());
        assert_eq!(marker.what_the_remap_wrote_onto_it.len(), 1);
    }

    #[test]
    fn every_mappable_destination_is_in_the_material_and_the_map_is_not_recomputed() {
        // §12: the unit is **the whole map**, and a map applied in part is a job
        // writing to the artist's own machine.
        let version = TheVersionInHand::of(5, 2);
        let destinations = vec![
            a_destination(
                DestinationKind::TheRegularOutputPath,
                "/home/a/renders/frames/f_",
                WhereItComesFrom::TheScenesOutputPath,
            ),
            a_node_destination("Beauty", "beauty", "/home/a/renders/passes", None),
            a_node_destination("Other", "beauty", "/home/a/renders/more", None),
        ];
        let map = mapped(
            &crate::inventory::tests::a_reading(destinations.clone(), Vec::new(), version),
            version,
        );
        let material = compose_the_launch_material(&map, version, "job-7", nothing_carried());
        assert_eq!(material.assignments.len(), 2);
        assert!(material.the_regular_output_path.is_some());
        assert_eq!(material.composed_for, version);
    }
}

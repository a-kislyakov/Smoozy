//! **L8b — the one script before the file, and what its handler sets.** Added
//! 2026-10-02, simulation fix piece 4.
//!
//! **One script sits at the head of the line, before the file, and it only
//! registers a handler Blender runs as the file loads.** It sets nothing
//! itself. Which places the handler sets, and to what, is `Job files and
//! outputs`' — D1a and D20 — and arrives here as the destination map's own
//! assignments; **this file only chooses which of them ride the handler rather
//! than the material, and which geometry-nodes bakes the handler sets — the
//! Creator's decision B, ruled 2026-10-02: only those a sent copy names**.
//!
//! **Why it cannot be the material**, observed at the desk on 2026-10-02 in
//! 4.5.12 and 5.2.2: a geometry-nodes bake folder set after the file has
//! loaded is ignored and the folder the file recorded is read, while set by a
//! handler as the file loads it is read; and a Replay fluid cache writes frame
//! 1's files beside the `.blend` the moment the file opens, while set by the
//! handler nothing is written there.
//!
//! **A raise inside the handler does not fail the run**, observed at the desk
//! on 2026-10-02 in all four versions with `--python-exit-code 63` on the
//! line: Blender printed *"Error in bpy.app.handlers.load_post"* with the
//! traceback, went on loading the file, ran the script after it, rendered and
//! ended at 0. **So the material reads whether the handler finished, before
//! its pairs, and raises where it did not** ([`crate::material::Entry::TheLoadTimeHandlerFinished`]),
//! and L8a's argument turns that raise into the run ending before the render
//! instruction — the same run ended at 63 with no frame written, in all four.
//! The handler holds no failure handling of its own (L11).
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Job files and outputs` D1a, D20, P7a, R4.
//! `Dependency Upload & Remap` M22a.

use smoozy_job_files_machine::mapping::{FurtherValue, FurtherValueIs};
use smoozy_job_files_machine::reading::{AnAddedWritePlace, DestinationKind};
use smoozy_job_files_machine::redirect::{
    is_set_as_the_file_loads, the_settings_of, ASettingOfAnAddedPlace,
};

use smoozy_dependencies_machine::placed_copies::TheCopyTheMachineOwes;

use crate::material::RedirectAssignment;

/// **What the handler is handed: every place it sets, and nothing else.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheLoadTimeDocument {
    pub places: Vec<APlaceSetAsTheFileLoads>,
}

/// One place the handler sets, on every carrier of it in the file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct APlaceSetAsTheFileLoads {
    pub place: AnAddedWritePlace,
    /// Every setting that holds the place's folder, each with what carries it
    /// — `Job files and outputs`' own `the_settings_of`, cited there.
    pub settings: &'static [ASettingOfAnAddedPlace],
    /// **Where each carrier is pointed.**
    pub to: WhereTheHandlerPointsIt,
    /// The place's further values that are switches, written on each carrier
    /// before its folder — none for either place the render's handler sets.
    pub switched_on_before_the_folder: Vec<FurtherValue>,
    /// **Settings the handler keeps as the file had them before it writes
    /// anything**, besides the ones it writes — `Job files and outputs` R18:
    /// a read reports the file's own values, never the throwaway ones. For a
    /// point cache that is whether it is baked, which the reading reports and
    /// which pointing the cache elsewhere could change.
    pub kept_as_the_file_has_it: Vec<&'static str>,
    /// **Whether a carrier whose recorded value the render's table does not
    /// hold is left as the file has it** rather than raising. On for the
    /// geometry-nodes bake folder alone — the Creator's decision B, 2026-10-02:
    /// only the folders a sent bake was copied for are set (`Job files and
    /// outputs` D1a) — and off for the Replay folder, which still raises
    /// (L11). A read's throwaway folder takes every carrier, so it is off there.
    pub leaves_what_the_table_does_not_hold: bool,
}

/// Where the handler points one place's carriers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhereTheHandlerPointsIt {
    /// **The render's case: the map's own folders**, one per recorded
    /// location, the one with no location being the folder for every carrier
    /// that records none (`Job files and outputs` R4 item 2, P7a). **Each
    /// carrier is set to the folder of its own recorded location**, read off
    /// the carrier as the file has it, and a value the table does not hold
    /// raises (L11) — the reading and the handler disagreeing about the file
    /// is not something to render past.
    TheJobFolders(Vec<AFolderForARecordedLocation>),
    /// **A read's case — `Job files and outputs` R18: a throwaway folder in
    /// that run's own working area**, removed when the run ends. Each carrier
    /// gets a folder of its own under it, so no two caches write into one
    /// another, and a path that names one is told back to its own carrier's
    /// recorded value.
    AThrowawayFolder(Vec<u8>),
}

/// One recorded location and its folder under the job's folder.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AFolderForARecordedLocation {
    /// Absent for the carriers that record no location.
    pub recorded: Option<Vec<u8>>,
    pub folder: Vec<u8>,
}

/// **The two forms the Creator's decision B was put in**, which decided which
/// geometry-nodes bakes the handler sets (`Job files and outputs` D1a,
/// `Dependency Upload & Remap` M22a).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AFormOfDecisionB {
    /// **Keep the ruling of 2026-08-22**: every geometry-nodes bake folder set
    /// to its job-folder value, and every geometry-nodes bake on the machine —
    /// sent, or uploaded beside the `.blend` — copied there first. **Not the
    /// ruling.**
    EveryGeometryNodesBake,
    /// **Narrow it**: only the folders a sent bake was copied for are set, each
    /// to its own job-folder value; the rest are left as the file has them and
    /// read where they are, as Blender does at home.
    OnlyTheBakesACopyWasSentFor,
}

/// **Decision B, ruled by the Creator on 2026-10-02 (Creator-stated): only the
/// folders a sent bake was copied for** (`Job files and outputs` D1a). A
/// geometry-nodes bake uploaded beside the `.blend` is read where it is; a
/// bake sent by *Check dependencies* is copied into the job's folder and its
/// folder is set there by the handler.
pub const THE_CREATORS_RULING_ON_DECISION_B: AFormOfDecisionB =
    AFormOfDecisionB::OnlyTheBakesACopyWasSentFor;

/// **Which of the map's geometry-nodes assignments the handler sets: those
/// whose recorded location a sent copy names** — a [`TheCopyTheMachineOwes`]
/// for the geometry-nodes bake folder, matched on the recorded bytes exactly
/// as its own `where_it_goes` finds the folder the copy lands in, so the
/// handler points a carrier at the very folder the copy was placed in.
/// **Every other geometry-nodes assignment is left out**, and the handler
/// leaves those carriers as the file has them
/// ([`APlaceSetAsTheFileLoads::leaves_what_the_table_does_not_hold`]).
/// The assignment for the carriers that record no location is never named by
/// a copy, since a sent bake always has a recorded folder, so it is never set.
pub fn the_geometry_nodes_bakes_set_as_the_file_loads(
    every_geometry_nodes_assignment: &[&RedirectAssignment],
    copies_owed: &[TheCopyTheMachineOwes],
) -> Vec<RedirectAssignment> {
    every_geometry_nodes_assignment
        .iter()
        .filter(|assignment| {
            let Some(recorded) = assignment.destination.recorded_location.as_ref() else {
                return false;
            };
            copies_owed.iter().any(|copy| {
                copy.place == AnAddedWritePlace::TheGeometryNodesBakeFolder
                    && copy.recorded.as_bytes() == recorded.as_bytes()
            })
        })
        .map(|assignment| (*assignment).clone())
        .collect()
}

/// **The render's handler: the map's assignments for the places `Job files
/// and outputs` D1a sets as the file loads**, grouped one place to one entry,
/// each carrying its table of recorded location to folder.
///
/// **The Replay fluid cache folder rides it whole.** The geometry-nodes bake
/// folder rides it as [`the_geometry_nodes_bakes_set_as_the_file_loads`]
/// says — only where a sent copy names it (decision B). **No place here is
/// ever in the material**:
/// [`crate::material::compose`] passes over every assignment
/// [`is_set_as_the_file_loads`] names.
pub fn for_the_render(
    redirect_pairs: &[RedirectAssignment],
    copies_owed: &[TheCopyTheMachineOwes],
) -> TheLoadTimeDocument {
    let of = |wanted: AnAddedWritePlace| -> Vec<&RedirectAssignment> {
        redirect_pairs
            .iter()
            .filter(|assignment| {
                assignment.destination.kind == DestinationKind::AnAddedWritePlace(wanted)
            })
            .collect()
    };
    let mut places = Vec::new();
    for place in [
        AnAddedWritePlace::TheReplayFluidCacheFolder,
        AnAddedWritePlace::TheGeometryNodesBakeFolder,
    ] {
        debug_assert!(is_set_as_the_file_loads(place));
        let set: Vec<RedirectAssignment> = match place {
            AnAddedWritePlace::TheGeometryNodesBakeFolder => {
                the_geometry_nodes_bakes_set_as_the_file_loads(&of(place), copies_owed)
            }
            _ => of(place).into_iter().cloned().collect(),
        };
        if set.is_empty() {
            continue;
        }
        places.push(APlaceSetAsTheFileLoads {
            place,
            settings: the_settings_of(place),
            to: WhereTheHandlerPointsIt::TheJobFolders(
                set.iter()
                    .map(|assignment| AFolderForARecordedLocation {
                        recorded: assignment
                            .destination
                            .recorded_location
                            .as_ref()
                            .map(|location| location.as_bytes().to_vec()),
                        folder: assignment.folder.clone(),
                    })
                    .collect(),
            ),
            switched_on_before_the_folder: set
                .first()
                .map(|assignment| {
                    assignment
                        .further_values
                        .iter()
                        .filter(|further| further.value == FurtherValueIs::On)
                        .cloned()
                        .collect()
                })
                .unwrap_or_default(),
            kept_as_the_file_has_it: Vec::new(),
            // Decision B: a geometry-nodes carrier no sent copy names is read
            // where the file has it. The Replay folder's table holds every
            // folder the reading found, so a miss there still raises (L11).
            leaves_what_the_table_does_not_hold: place
                == AnAddedWritePlace::TheGeometryNodesBakeFolder,
        });
    }
    TheLoadTimeDocument { places }
}

/// **`Job files and outputs` R18: the handler every read of a user's file
/// carries** — the render's start-of-attempt reading, *Read the file*, and
/// `Dependency Upload & Remap`'s check. **It points every Replay fluid cache
/// and every disk cache at a throwaway folder** in that run's own working
/// area, which the run removes when it ends.
///
/// **Observed at the desk before this, 2026-10-02, in 4.5.12 and 5.2.2**: the
/// render-start reading and *Read the file* each wrote 96 files beside a
/// Replay smoke's `.blend` and 48 beside an unbaked cloth's, and the check
/// wrote the Replay smoke's frame 1 there.
///
/// - **The Replay fluid cache folder**, every domain whose cache type is
///   Replay — the same carriers as the render's handler.
/// - **The point cache**, on every cache whose disk cache the file has on —
///   the material's own carriers, never a rigid-body world's, and never the
///   disk-cache switch (`Job files and outputs` D21b-i). *External* is turned
///   on before the folder, as the material does, or the folder is not used;
///   whether the cache is baked is kept as the file had it.
///
/// **Every value the handler changes, and those it keeps, are kept as the
/// file had them**, and the reading and the check read those rather than the
/// throwaway ones — so what they report is the file's own.
pub fn for_a_reading(the_throwaway_folder: &[u8]) -> TheLoadTimeDocument {
    let throwaway = || WhereTheHandlerPointsIt::AThrowawayFolder(the_throwaway_folder.to_vec());
    TheLoadTimeDocument {
        places: vec![
            APlaceSetAsTheFileLoads {
                place: AnAddedWritePlace::TheReplayFluidCacheFolder,
                settings: the_settings_of(AnAddedWritePlace::TheReplayFluidCacheFolder),
                to: throwaway(),
                switched_on_before_the_folder: Vec::new(),
                kept_as_the_file_has_it: Vec::new(),
                leaves_what_the_table_does_not_hold: false,
            },
            APlaceSetAsTheFileLoads {
                place: AnAddedWritePlace::ThePointCache,
                settings: the_settings_of(AnAddedWritePlace::ThePointCache),
                to: throwaway(),
                switched_on_before_the_folder: smoozy_job_files_machine::mapping::the_further_values_for(
                    AnAddedWritePlace::ThePointCache,
                )
                .into_iter()
                .filter(|further| further.value == FurtherValueIs::On)
                .collect(),
                kept_as_the_file_has_it: vec!["point_cache.is_baked"],
                leaves_what_the_table_does_not_hold: false,
            },
        ],
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::material::tests::a_redirect_assignment;
    use smoozy_job_files_machine::reading::{RecordedLocation, WhereItComesFrom};

    pub(crate) fn an_added_place(
        place: AnAddedWritePlace,
        recorded: Option<&[u8]>,
        folder: &[u8],
    ) -> RedirectAssignment {
        let mut added = a_redirect_assignment("File Output");
        added.destination.kind = DestinationKind::AnAddedWritePlace(place);
        added.destination.origin = WhereItComesFrom::AnAddedWritePlaceNamed(place);
        added.destination.recorded_location = recorded.map(|bytes| RecordedLocation::of(bytes.to_vec()));
        added.property = None;
        added.folder = folder.to_vec();
        added.further_values = smoozy_job_files_machine::mapping::the_further_values_for(place);
        added
    }

    /// **L8b and `Job files and outputs` D1a: every Replay fluid cache
    /// destination rides the handler, each with its own folder**, and nothing
    /// of the other three places that are not set as the file loads does.
    #[test]
    fn the_replay_fluid_cache_folders_ride_the_handler_with_their_own_folders() {
        let pairs = vec![
            an_added_place(AnAddedWritePlace::TheBakeOutputPath, None, b"/job/bake"),
            an_added_place(AnAddedWritePlace::ThePointCache, None, b"/job/pointcache"),
            an_added_place(
                AnAddedWritePlace::TheReplayFluidCacheFolder,
                Some(b"//cache_fluid_4b85f272"),
                b"/job/cache_fluid_4b85f272",
            ),
            an_added_place(AnAddedWritePlace::TheReplayFluidCacheFolder, None, b"/job/fluid-cache"),
        ];
        let document = for_the_render(&pairs, &[]);
        assert_eq!(document.places.len(), 1);
        let replay = &document.places[0];
        assert_eq!(replay.place, AnAddedWritePlace::TheReplayFluidCacheFolder);
        assert_eq!(
            replay.to,
            WhereTheHandlerPointsIt::TheJobFolders(vec![
                AFolderForARecordedLocation {
                    recorded: Some(b"//cache_fluid_4b85f272".to_vec()),
                    folder: b"/job/cache_fluid_4b85f272".to_vec(),
                },
                AFolderForARecordedLocation {
                    recorded: None,
                    folder: b"/job/fluid-cache".to_vec(),
                },
            ])
        );
        assert_eq!(replay.settings[0].setting, "domain_settings.cache_directory");
        assert!(replay.switched_on_before_the_folder.is_empty());
        assert!(!replay.leaves_what_the_table_does_not_hold, "a Replay miss still raises");
    }

    /// **`Job files and outputs` R18: a read's handler points every Replay
    /// fluid cache and every disk cache at the run's own throwaway folder**,
    /// turns *external* on first and never the disk-cache switch, and keeps
    /// whether a cache is baked as the file had it.
    #[test]
    fn a_reads_handler_points_every_replay_and_disk_cache_at_the_throwaway_folder() {
        let document = for_a_reading(b"/ops/o1/reading-caches");
        let places: Vec<AnAddedWritePlace> = document.places.iter().map(|one| one.place).collect();
        assert_eq!(
            places,
            vec![
                AnAddedWritePlace::TheReplayFluidCacheFolder,
                AnAddedWritePlace::ThePointCache
            ]
        );
        for one in &document.places {
            assert_eq!(
                one.to,
                WhereTheHandlerPointsIt::AThrowawayFolder(b"/ops/o1/reading-caches".to_vec())
            );
            assert!(!one
                .switched_on_before_the_folder
                .iter()
                .any(|further| further.setting.ends_with("use_disk_cache")));
        }
        let cache = &document.places[1];
        assert_eq!(cache.settings[0].setting, "point_cache.filepath");
        assert_eq!(cache.switched_on_before_the_folder[0].setting, "point_cache.use_external");
        assert_eq!(cache.kept_as_the_file_has_it, vec!["point_cache.is_baked"]);
    }

    pub(crate) fn a_sent_geometry_nodes_bake(recorded: &[u8]) -> TheCopyTheMachineOwes {
        TheCopyTheMachineOwes {
            from: smoozy_dependencies_machine::bytes::MachineLocation::composed(
                b"/ephemeral/smoozy/assets/files/home/artist/bake_load_cache".to_vec(),
            ),
            place: AnAddedWritePlace::TheGeometryNodesBakeFolder,
            recorded: smoozy_dependencies_machine::bytes::RecordedLocation::as_the_reading_carried_it(
                recorded,
            ),
        }
    }

    /// **The Creator's decision B, 2026-10-02 — only the folders a sent bake
    /// was copied for** (`Job files and outputs` D1a, `Dependency Upload &
    /// Remap` M22a narrowed). Of a file's geometry-nodes bake folders, the
    /// handler sets the one a sent copy names, on its recorded bytes, and
    /// neither the one uploaded beside the `.blend` nor the carriers that
    /// record no folder; a copy owed for another place names nothing here;
    /// and the geometry-nodes place leaves what its table does not hold.
    #[test]
    fn only_the_geometry_nodes_bakes_a_copy_was_sent_for_ride_the_handler() {
        assert_eq!(
            THE_CREATORS_RULING_ON_DECISION_B,
            AFormOfDecisionB::OnlyTheBakesACopyWasSentFor
        );
        let pairs = vec![
            an_added_place(
                AnAddedWritePlace::TheGeometryNodesBakeFolder,
                Some(b"//bake_load_cache/"),
                b"/job/bake_load_cache",
            ),
            an_added_place(
                AnAddedWritePlace::TheGeometryNodesBakeFolder,
                Some(b"//beside_the_blend/"),
                b"/job/beside_the_blend",
            ),
            an_added_place(AnAddedWritePlace::TheGeometryNodesBakeFolder, None, b"/job/geometry-nodes-bake"),
        ];
        // Nothing sent: no geometry-nodes folder is set at all.
        assert!(for_the_render(&pairs, &[]).places.is_empty());
        // A copy owed for another place with the same bytes names nothing here.
        let mut elsewhere = a_sent_geometry_nodes_bake(b"//bake_load_cache/");
        elsewhere.place = AnAddedWritePlace::ThePointCache;
        assert!(for_the_render(&pairs, &[elsewhere]).places.is_empty());
        // The sent one, and only it.
        let document = for_the_render(&pairs, &[a_sent_geometry_nodes_bake(b"//bake_load_cache/")]);
        assert_eq!(document.places.len(), 1);
        let bakes = &document.places[0];
        assert_eq!(bakes.place, AnAddedWritePlace::TheGeometryNodesBakeFolder);
        assert_eq!(
            bakes.to,
            WhereTheHandlerPointsIt::TheJobFolders(vec![AFolderForARecordedLocation {
                recorded: Some(b"//bake_load_cache/".to_vec()),
                folder: b"/job/bake_load_cache".to_vec(),
            }])
        );
        assert!(bakes.leaves_what_the_table_does_not_hold);
        let settings: Vec<&str> = bakes.settings.iter().map(|one| one.setting).collect();
        assert!(settings.contains(&"bake_directory") && settings.contains(&"directory"), "{settings:?}");
        // Matched on the bytes exactly: a near miss names nothing.
        assert!(
            for_the_render(&pairs, &[a_sent_geometry_nodes_bake(b"//bake_load_cache")]).places.is_empty()
        );
    }
}

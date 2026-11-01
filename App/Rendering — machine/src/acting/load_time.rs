//! **L8b performed — the script before the file, rendered from
//! [`crate::load_time`]'s document.** Added 2026-10-02, simulation fix piece 4.
//!
//! **The script registers one handler and sets nothing itself.** The handler
//! is kept across the load (Blender's `persistent` mark, read off all four
//! versions at the desk) and runs as the file loads: it walks to every carrier
//! of each place it is handed — by the same walk as the material,
//! [`super::material::THE_SHARED_WALK`] — and points each at the folder of
//! its own recorded location. **It keeps a record of what it set and of
//! whether it ran to its end**, under `Job files and outputs`'
//! `THE_LOAD_TIME_RECORD`, and the material reads that record before any pair
//! and raises where the handler did not finish — because a raise inside a
//! handler does not end the run (observed at the desk on 2026-10-02 in all
//! four versions: Blender printed the traceback, loaded the file, rendered and
//! ended at 0).
//!
//! **L11: no failure handling of its own.** There is no `try` in the text.
//!
//! **Placed and read back like the material** (L12), at its own path, by the
//! same act.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Job files and outputs` D1a, R4.

use serde::Serialize;

use crate::load_time::{TheLoadTimeDocument, WhereTheHandlerPointsIt};

#[derive(Debug, Clone, PartialEq, Serialize)]
struct AFolderForARecordedLocation {
    recorded: Option<Vec<u8>>,
    folder: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
struct ASetting {
    carried_by: smoozy_job_files_machine::redirect::WhatCarriesTheSetting,
    setting: &'static str,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
struct ASwitch {
    setting: String,
    value: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
struct APlace {
    /// The place, in the artist's own terms, for the record.
    place: &'static str,
    settings: Vec<ASetting>,
    /// The render's table, or nothing for a read's throwaway folder.
    folders: Option<Vec<AFolderForARecordedLocation>>,
    /// A read's throwaway folder, or nothing for the render's table.
    throwaway: Option<Vec<u8>>,
    switched_on_before_the_folder: Vec<ASwitch>,
    kept_as_the_file_has_it: Vec<&'static str>,
    /// Decision B: a carrier the render's table does not hold is left as the
    /// file has it, rather than raising.
    leaves_what_the_table_does_not_hold: bool,
}

/// **The script's text, composed from the document and from nothing else.**
/// Every value travels as data inside one JSON document, as the material's
/// do; nothing of the job's is spliced into an expression.
pub fn the_load_time_scripts_text(document: &TheLoadTimeDocument) -> String {
    let places: Vec<APlace> = document
        .places
        .iter()
        .map(|one| APlace {
            place: one.place.in_words(),
            settings: one
                .settings
                .iter()
                .map(|setting| ASetting {
                    carried_by: setting.carried_by,
                    setting: setting.setting,
                })
                .collect(),
            folders: match &one.to {
                WhereTheHandlerPointsIt::TheJobFolders(folders) => Some(
                    folders
                        .iter()
                        .map(|folder| AFolderForARecordedLocation {
                            recorded: folder.recorded.clone(),
                            folder: folder.folder.clone(),
                        })
                        .collect(),
                ),
                WhereTheHandlerPointsIt::AThrowawayFolder(_) => None,
            },
            throwaway: match &one.to {
                WhereTheHandlerPointsIt::TheJobFolders(_) => None,
                WhereTheHandlerPointsIt::AThrowawayFolder(folder) => Some(folder.clone()),
            },
            kept_as_the_file_has_it: one.kept_as_the_file_has_it.clone(),
            leaves_what_the_table_does_not_hold: one.leaves_what_the_table_does_not_hold,
            switched_on_before_the_folder: one
                .switched_on_before_the_folder
                .iter()
                .map(|further| ASwitch {
                    setting: further.setting.clone(),
                    value: true,
                })
                .collect(),
        })
        .collect();
    let rendered = serde_json::to_string(&serde_json::json!({
        "record": smoozy_job_files_machine::redirect::THE_LOAD_TIME_RECORD,
        "places": places,
    }))
    .expect("the document is composed of values that always render");
    // JSON's only string delimiter is the double quote, so three single
    // quotes in a row never appear in it and the raw string cannot end early.
    debug_assert!(!rendered.contains("'''"));
    format!(
        "{THE_HEAD}\nSMOOZY_LOAD_TIME = json.loads(r'''{rendered}''')\n{}{THE_HANDLER}",
        super::material::THE_SHARED_WALK
    )
}

const THE_HEAD: &str = r#"
import bpy, json, os, sys
from bpy.app.handlers import persistent

# **`Rendering` L8b: this script sits before the file and sets nothing
# itself.** It registers one handler, kept across the load, which Blender runs
# as the file loads. **L11: no failure handling of its own** — there is no
# `try` below. A raise inside the handler does not end the run, so the
# material after the file reads the record kept here and raises where the
# handler did not run to its end.
"#;

const THE_HANDLER: &str = r#"
def smoozy_the_record():
    name = SMOOZY_LOAD_TIME["record"]
    record = sys.modules.get(name)
    if record is None:
        record = type(sys)(name)
        sys.modules[name] = record
    return record

@persistent
def smoozy_as_the_file_loads(*_):
    record = smoozy_the_record()
    record.finished = False
    record.what_was_set = []
    # **`Job files and outputs` R18**: every value this handler changes, and
    # every value it is asked to keep, as the file had it before anything was
    # written — keyed by the object it sits on and its own name — and every
    # folder it set, told back to the value it replaced. A read reports these
    # and never the folders set here.
    record.as_the_file_has_it = {}
    record.paths_put_back = {}
    for place in SMOOZY_LOAD_TIME["places"]:
        how_many = 0
        left = 0
        for one in place["settings"]:
            # The carriers are object-level, so no scene is handed in.
            for carrier in smoozy_carriers(None, one["carried_by"]):
                holder, name = smoozy_off(carrier, one["setting"])
                recorded = getattr(holder, name)
                if place["throwaway"] is not None:
                    # **A read**: a folder of this carrier's own under the
                    # run's throwaway folder, so no two caches share one.
                    folder = os.path.join(
                        os.fsdecode(bytes(place["throwaway"])), str(len(record.paths_put_back))
                    )
                elif not place["leaves_what_the_table_does_not_hold"]:
                    # **The render** — `Job files and outputs` R4 item 2: the
                    # folder of this carrier's own recorded location, read
                    # before anything on it is written; a value the table does
                    # not hold raises.
                    folder = smoozy_the_folder_for(place["folders"], recorded)
                else:
                    # **The Creator's decision B, 2026-10-02**: only the
                    # geometry-nodes bake folders a sent copy names are in the
                    # table, and every other carrier is read where the file
                    # has it — left exactly as it is, nothing kept or written.
                    folder = smoozy_the_folder_held_for(place["folders"], recorded)
                    if folder is None:
                        left += 1
                        continue
                kept = list(place["kept_as_the_file_has_it"])
                kept.extend(further["setting"] for further in place["switched_on_before_the_folder"])
                kept.append(one["setting"])
                for setting in kept:
                    kept_holder, kept_name = smoozy_off(carrier, setting)
                    record.as_the_file_has_it[(kept_holder.as_pointer(), kept_name)] = getattr(kept_holder, kept_name)
                os.makedirs(folder, exist_ok=True)
                for further in place["switched_on_before_the_folder"]:
                    switch_holder, switch_name = smoozy_off(carrier, further["setting"])
                    setattr(switch_holder, switch_name, further["value"])
                setattr(holder, name, folder)
                record.paths_put_back[os.path.normpath(folder)] = recorded
                how_many += 1
        record.what_was_set.append({
            "place": place["place"],
            "folders": len(place["folders"] or []),
            "set_on": how_many,
            "left_as_the_file_has_it": left,
        })
    record.finished = True

smoozy_the_record().finished = False
bpy.app.handlers.load_post.append(smoozy_as_the_file_loads)
"#;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::load_time::tests::an_added_place;
    use crate::load_time::for_the_render;
    use smoozy_job_files_machine::reading::AnAddedWritePlace;

    /// **L8b: the handler's text names each Replay fluid domain through the
    /// shared walk, carries the map's own table for it, and none for the
    /// geometry-nodes bakes where no bake was sent (decision B).** It registers on
    /// the load, keeps its record, and holds no failure handling.
    #[test]
    fn the_handler_sets_each_replay_domain_from_the_table_and_no_geometry_nodes_bake_unsent() {
        let pairs = vec![
            an_added_place(
                AnAddedWritePlace::TheReplayFluidCacheFolder,
                Some(b"//cache_fluid_4b85f272"),
                b"/job/cache_fluid_4b85f272",
            ),
            an_added_place(
                AnAddedWritePlace::TheGeometryNodesBakeFolder,
                Some(b"//bake_load_cache/"),
                b"/job/bake_load_cache",
            ),
        ];
        let text = the_load_time_scripts_text(&for_the_render(&pairs, &[]));
        assert!(text.contains(
            r#"{"carried_by":"EveryReplayFluidDomain","setting":"domain_settings.cache_directory"}"#
        ));
        assert!(
            !text.contains(r#""carried_by":"EveryGeometryNodes"#),
            "decision B: no bake was sent, so no geometry-nodes folder is set"
        );
        let bytes = |text: &[u8]| serde_json::to_string(text).unwrap();
        assert!(text.contains(&format!(
            r#""folders":[{{"folder":{},"recorded":{}}}]"#,
            bytes(b"/job/cache_fluid_4b85f272"),
            bytes(b"//cache_fluid_4b85f272"),
        )));
        // The walk is the material's own, so the two cannot walk differently.
        assert!(text.contains(super::super::material::THE_SHARED_WALK));
        assert!(text.contains(r#"if carried_by == "EveryReplayFluidDomain":"#));
        assert!(text.contains(r#"modifier.domain_settings.cache_type == "REPLAY""#));
        // Registered on the load and kept across it.
        assert!(text.contains("@persistent\ndef smoozy_as_the_file_loads(*_):"));
        assert!(text.contains("bpy.app.handlers.load_post.append(smoozy_as_the_file_loads)"));
        // Its record, under the name the material reads, finished last.
        assert!(text.contains(&format!(
            r#""record":"{}""#,
            smoozy_job_files_machine::redirect::THE_LOAD_TIME_RECORD
        )));
        assert!(text.contains(r#""throwaway":null"#));
        let set = text.find("setattr(holder, name, folder)").unwrap();
        let finished = text.find("record.finished = True").unwrap();
        assert!(set < finished);
        for word in [["try", ":"].concat(), ["except", " "].concat()] {
            assert!(!text.contains(&word), "{word}");
        }
    }

    /// **`Job files and outputs` R18: a read's handler points each carrier
    /// at a folder of its own under the throwaway folder, keeps every value it
    /// changes — and whether a cache is baked — as the file had it, before
    /// writing anything, and tells each folder back to the value it
    /// replaced**, under the names the reading's and the check's helper read.
    #[test]
    fn a_reads_handler_keeps_the_files_own_values_where_the_reads_look_for_them() {
        let text = the_load_time_scripts_text(&crate::load_time::for_a_reading(b"/ops/o1/reading-caches"));
        let bytes = serde_json::to_string(&b"/ops/o1/reading-caches"[..]).unwrap();
        assert_eq!(text.matches(&format!(r#""throwaway":{bytes}"#)).count(), 2);
        assert!(text.contains(r#""folders":null"#));
        assert!(text.contains(r#""kept_as_the_file_has_it":["point_cache.is_baked"]"#));
        assert!(text.contains(
            r#""switched_on_before_the_folder":[{"setting":"point_cache.use_external","value":true}]"#
        ));
        let helper = smoozy_job_files_machine::reading_script::THE_READ_AS_THE_FILE_HAS_IT;
        for name in ["as_the_file_has_it", "paths_put_back"] {
            assert!(text.contains(&format!("record.{name} = {{}}")), "{name}");
            assert!(helper.contains(&format!(r#"getattr(record, "{name}", None)"#)), "{name}");
        }
        let kept = text
            .find("record.as_the_file_has_it[(kept_holder.as_pointer(), kept_name)]")
            .unwrap();
        let first_write = text.find("setattr(switch_holder, switch_name").unwrap();
        assert!(kept < first_write, "the file's own values are kept before anything is written");
        assert!(text.contains("record.paths_put_back[os.path.normpath(folder)] = recorded"));
    }

    /// **The Creator's decision B, 2026-10-02 (`Job files and outputs` D1a):
    /// a geometry-nodes carrier no sent copy names is left as the file has it,
    /// and a Replay domain whose value the table does not hold still raises.**
    /// The geometry-nodes place carries the flag and the Replay place does
    /// not; the handler reads the flag before any lookup, takes the lookup
    /// that answers nothing where it is on and the one that raises where it is
    /// off, and leaves a carrier it skips untouched — nothing kept, nothing
    /// made, nothing set — and counts it on the record.
    #[test]
    fn a_geometry_nodes_bake_left_beside_is_left_alone_while_a_replay_miss_still_raises() {
        let pairs = vec![
            an_added_place(
                AnAddedWritePlace::TheReplayFluidCacheFolder,
                Some(b"//cache_fluid_4b85f272"),
                b"/job/cache_fluid_4b85f272",
            ),
            an_added_place(
                AnAddedWritePlace::TheGeometryNodesBakeFolder,
                Some(b"//bake_load_cache/"),
                b"/job/bake_load_cache",
            ),
            an_added_place(
                AnAddedWritePlace::TheGeometryNodesBakeFolder,
                Some(b"//beside/"),
                b"/job/beside",
            ),
        ];
        let sent = [crate::load_time::tests::a_sent_geometry_nodes_bake(b"//bake_load_cache/")];
        let text = the_load_time_scripts_text(&for_the_render(&pairs, &sent));
        let bytes = |text: &[u8]| serde_json::to_string(text).unwrap();
        let document: serde_json::Value = serde_json::from_str(
            &text[text.find("json.loads(r'''").unwrap() + "json.loads(r'''".len()..text.find("''')").unwrap()],
        )
        .unwrap();
        let place = |words: &str| {
            document["places"]
                .as_array()
                .unwrap()
                .iter()
                .find(|one| one["place"] == words)
                .cloned()
                .unwrap()
        };
        let replay = place("the Replay fluid cache folder");
        let bakes = place("the geometry nodes bake folder");
        assert_eq!(replay["leaves_what_the_table_does_not_hold"], false);
        assert_eq!(bakes["leaves_what_the_table_does_not_hold"], true);
        assert_eq!(
            bakes["folders"],
            serde_json::json!([{"folder": b"/job/bake_load_cache", "recorded": b"//bake_load_cache/"}])
        );
        assert!(!text.contains(&bytes(b"/job/beside")), "the bake beside is not in the table");
        assert!(text.contains(r#"{"carried_by":"EveryGeometryNodesModifier","setting":"bake_directory"}"#));
        assert!(text.contains(r#"{"carried_by":"EveryGeometryNodesBake","setting":"directory"}"#));
        // The handler's branches, in order: the raising lookup where the flag
        // is off, the quiet one where it is on, and a skip before anything is
        // kept or written.
        let off = text.find(r#"elif not place["leaves_what_the_table_does_not_hold"]:"#).unwrap();
        let raising = text.find(r#"folder = smoozy_the_folder_for(place["folders"], recorded)"#).unwrap();
        let quiet = text.find(r#"folder = smoozy_the_folder_held_for(place["folders"], recorded)"#).unwrap();
        let skip = text[quiet..].find("left += 1").map(|at| at + quiet).unwrap();
        let first_kept = text.find("kept = list(place[\"kept_as_the_file_has_it\"])").unwrap();
        assert!(off < raising && raising < quiet && quiet < skip && skip < first_kept);
        assert!(text[skip..first_kept].contains("continue"));
        assert!(text.contains(r#""left_as_the_file_has_it": left"#));
        // The quiet lookup and the raising one share one matching rule.
        let walk = super::super::material::THE_SHARED_WALK;
        let shared = walk.find("def smoozy_the_folder_for").unwrap();
        assert!(walk[shared..].contains("folder = smoozy_the_folder_held_for(folders, recorded)"));
        assert!(walk[shared..].contains("raise LookupError"));
    }
}

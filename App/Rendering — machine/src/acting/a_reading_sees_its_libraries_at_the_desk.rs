//! **`Job files and outputs` R1a and `Dependency Upload & Remap` M8b with real
//! Blenders: a job's reading opens the file from the folder the render opens it
//! from, so it sees what sits on a linked object.** Kept in the tree and ignored.
//! Run by hand at the desk, one Blender at a time:
//!
//! ```text
//! SMOOZY_DESK_OUT=<scratch folder> cargo test -p smoozy-rendering-machine a_reading_sees_its_libraries_at_the_desk -- --ignored --nocapture --test-threads=1
//! ```
//!
//! `SMOOZY_DESK_ONLY=53` narrows a run to one Blender. **What runs is the render
//! worker's driver, whole**, through the harness of
//! [`super::the_5_3_readings_at_the_desk`]. Each file is built here in the shape the
//! normal route leaves on a machine: the top file alone in its folder, its library
//! elsewhere, and a pair saying where.
//!
//! **Why it exists: the film's first job** (`Audits/Build/Blender 5.3/The film's
//! faults — what each one is.md`, fault 9). Step 4 read the top file away from its
//! libraries, found no bake setting, and the job ended with *this job's folder has
//! no place for* the bake. The whole account is `Audits/Build/Blender 5.3/A job's
//! reading and failing before a render — what landed.md`.
//!
//! **`the_films_own_scene_is_read_and_not_rendered`** needs `SMOOZY_DESK_FILM`, the
//! scene's folder, and reads it with the file's scripts off. Nothing is written
//! there and no frame is rendered: the render's words are dropped at the wrapper.
//!
//! **What this does not prove**: Linux, a card, or the whole route through Files and
//! the check, which only a rented machine can show.

use std::path::{Path, PathBuf};

use smoozy_dependencies_machine::bytes::{MachineLocation, RecordedLocation};
use smoozy_dependencies_machine::check::Kind;
use smoozy_dependencies_machine::placed_copies::TheCopyTheMachineOwes;
use smoozy_dependencies_machine::remap::{compose_pair, NewLocation, Pair};
use smoozy_job_files_machine::mapping::HowItMapped;
use smoozy_job_files_machine::reading::AnAddedWritePlace;
use std::os::unix::fs::PermissionsExt;

use super::the_5_3_readings_at_the_desk::{
    a_file_baked_into, how_much_of_the_picture_is_not_background, make, one_job_whole, run, the_blenders, the_folder,
    WhatCameBack, THE_SMALL_SCENE,
};
use crate::material::tests::an_untouched_payload;

fn a_library_pair(recorded: &str, at: &Path) -> Pair {
    compose_pair(
        Kind::Library,
        "filepath",
        &RecordedLocation::as_the_reading_carried_it(recorded.as_bytes()),
        NewLocation::AFixedCopy(MachineLocation::composed(at.display().to_string().into_bytes())),
    )
    .expect("a pair")
}

/// **A top file holding nothing of its own but one linked object**, the small
/// scene's camera and settings, its library recorded relative to it.
fn a_top_file_linking(library: &Path, object: &str) -> String {
    format!(
        r#"{THE_SMALL_SCENE}
for one in list(bpy.data.objects):
    if one.type == "MESH":
        bpy.data.objects.remove(one)
bpy.ops.wm.save_as_mainfile(filepath=out)
with bpy.data.libraries.load({library:?}, link=True, relative=True) as (there, here):
    here.objects = [{object:?}]
for one in here.objects:
    scene.collection.objects.link(one)
print("SMOOZY-DESK-MADE", bpy.app.version_string, [one.filepath for one in bpy.data.libraries], flush=True)
bpy.ops.wm.save_as_mainfile(filepath=out, relative_remap=True)
"#,
        library = library.display().to_string(),
    )
}

fn say(came: &WhatCameBack, what: &str) {
    let not_launched = came.not_launched.as_deref().map(|said| {
        // The step's own long description is the same each time, and it hides the why.
        said.rsplit_once("reason:").map_or(said, |(_, why)| why).trim().to_string()
    });
    println!("    {what}: not launched {not_launched:?}, ended at {:?}", came.the_renders_status);
    println!("    {what}: took {:?} to read, line {}", came.the_reading_took, came.the_line);
}

/// **The film's fault, small.** The bake setting sits on a linked object, and the
/// bake was sent, so the machine owes a copy. Through the old order the map held no
/// place and the job ended. Through the new one the copy lands and **the picture
/// shows the cube, which exists only where the bake is read**.
#[test]
#[ignore = "run by hand at the desk with Blender 5.2 and 5.3 beta, one at a time"]
fn a_bake_on_a_linked_object_finds_its_place_and_is_read() {
    const THE_FOLDER: &str = "//caches/shot/";
    let out = the_folder().join("linked-bake-copied");
    for (named, blender) in the_blenders(&["5.2", "5.3"]) {
        println!("{named}:");
        let here = out.join(named);
        let library = make(&blender, &a_file_baked_into(THE_FOLDER), &here.join("assets"), "L");
        let the_bake = here.join("assets/caches/shot");
        assert!(the_bake.is_dir(), "{named} baked: {}", the_bake.display());
        let top = make(&blender, &a_top_file_linking(&library, "Cube"), &here, "T");
        let root = here.join("run");
        let came = one_job_whole(
            &blender,
            &top,
            &root,
            "Scene",
            &an_untouched_payload(),
            vec![1],
            vec![TheCopyTheMachineOwes {
                from: MachineLocation::composed(the_bake.display().to_string().into_bytes()),
                place: AnAddedWritePlace::TheGeometryNodesBakeFolder,
                recorded: RecordedLocation::as_the_reading_carried_it(THE_FOLDER.as_bytes()),
            }],
            None,
            vec![a_library_pair("//assets/L.blend", &library)],
        );
        say(&came, "linked");
        assert_eq!(came.not_launched, None, "{named}: the map held a place for the bake");
        assert_eq!(came.the_renders_status, Some(0), "{named}");
        println!("    in the job's folder: {:?}", came.in_the_jobs_folder);
        let frame = came
            .in_the_jobs_folder
            .iter()
            .find(|name| name.ends_with("f_0001.png"))
            .unwrap_or_else(|| panic!("{named}: no frame"));
        let cube = how_much_of_the_picture_is_not_background(&blender, &root.join("job").join(frame), &here);
        println!("    {cube} pixels of cube");
        assert!(cube > 0, "{named}: the render read the bake through the job's folder");
    }
}

/// **`Job files and outputs` R4 item 8 through the new order: a simulation on a linked
/// object is seen**, so `Rendering` L10b's gate answers as it does for one in the top
/// file. Read alone the file holds no simulation.
#[test]
#[ignore = "run by hand at the desk with Blender 5.2 and 5.3 beta, one at a time"]
fn a_simulation_on_a_linked_object_is_seen() {
    let out = the_folder().join("linked-simulation");
    for (named, blender) in the_blenders(&["5.2", "5.3"]) {
        println!("{named}:");
        let here = out.join(named);
        let script = format!(
            r#"{THE_SMALL_SCENE}
obj = bpy.data.objects["Cube"]
tree = bpy.data.node_groups.new("G", "GeometryNodeTree")
tree.interface.new_socket("Geometry", in_out="INPUT", socket_type="NodeSocketGeometry")
tree.interface.new_socket("Geometry", in_out="OUTPUT", socket_type="NodeSocketGeometry")
source = tree.nodes.new("NodeGroupInput")
end = tree.nodes.new("NodeGroupOutput")
into = tree.nodes.new("GeometryNodeSimulationInput")
leaving = tree.nodes.new("GeometryNodeSimulationOutput")
into.pair_with_output(leaving)
tree.links.new(source.outputs[0], into.inputs["Geometry"])
tree.links.new(into.outputs["Geometry"], leaving.inputs["Geometry"])
tree.links.new(leaving.outputs["Geometry"], end.inputs[0])
modifier = obj.modifiers.new("GN", "NODES")
modifier.node_group = tree
print("SMOOZY-DESK-MADE", bpy.app.version_string, len(modifier.bakes), flush=True)
bpy.ops.wm.save_as_mainfile(filepath=out)
"#
        );
        let library = make(&blender, &script, &here.join("assets"), "L");
        let top = make(&blender, &a_top_file_linking(&library, "Cube"), &here, "T");
        let job = |root: &str, pairs: Vec<Pair>| {
            one_job_whole(&blender, &top, &here.join(root), "Scene", &an_untouched_payload(), vec![1], Vec::new(), None, pairs)
        };
        let alone = job("alone", Vec::new());
        say(&alone, "alone");
        assert_eq!(alone.the_renders_status, Some(0), "{named}: {:?}", alone.not_launched);
        assert!(
            !alone.the_composed.as_ref().expect("a reading").reading.the_scene_holds_a_simulation_not_baked,
            "{named}: with no pair the library is absent and nothing is seen"
        );
        let linked = job("linked", vec![a_library_pair("//assets/L.blend", &library)]);
        say(&linked, "linked");
        assert_eq!(linked.the_renders_status, Some(0), "{named}: {:?}", linked.not_launched);
        assert!(
            linked.the_composed.as_ref().expect("a reading").reading.the_scene_holds_a_simulation_not_baked,
            "{named}: read through the folder, the simulation on the linked object is seen"
        );
        // `Render Settings` Q4: the page's actual pre-load script, with no job
        // frames, reads the same linked simulation through the same folder.
        let page=here.join("page");
        std::fs::create_dir_all(&page).unwrap();
        let isolated=page.join("T.blend");
        std::fs::copy(&top,&isolated).unwrap();
        let pair=a_library_pair("//assets/L.blend",&library);
        let opening=page.join("opening");
        use std::os::unix::ffi::OsStrExt;
        let folder=smoozy_dependencies_machine::opening::the_folder_the_file_opens_from(
            &[pair],isolated.as_os_str().as_bytes(),opening.as_os_str().as_bytes()).unwrap();
        super::opening::make_the_opening_folder(&folder).unwrap();
        let read=|path: &[u8],label: &str| {
            let version=if named=="5.3" {(5,3)} else {(5,2)};
            let invocation=smoozy_job_files_machine::reading::TheInvocation {
                the_blender_build_path:blender.display().to_string(),
                the_blend_path_on_the_machine:smoozy_job_files_machine::reading::RecordedLocation::of(path.to_vec()),
                the_version_it_will_render_by:smoozy_job_files_machine::reading::TheVersionInHand::of(version.0,version.1),
                the_frames_the_job_covers:Vec::new(),the_check_is_written_as_a_picture:false,the_check_is_marked:false,
            };
            let script=page.join(format!("{label}.py"));
            std::fs::write(&script,smoozy_job_files_machine::reading_script::the_pre_load_reading_script(&invocation).unwrap()).unwrap();
            let (load,throwaway)=super::reading::write_the_reads_load_time_script(script.to_str().unwrap()).unwrap();
            let mut words=vec!["--factory-startup".to_string(),"--disable-autoexec".to_string()];
            words.extend(super::reading::the_words_of_a_reading(std::str::from_utf8(path).unwrap(),&load,script.to_str().unwrap()).unwrap());
            let (status,out,err)=run(&blender,&words);
            super::reading::remove_the_throwaway_folder(&throwaway);
            std::fs::write(page.join(format!("{label}.log")),&out).unwrap();
            assert_eq!(status,Some(0),"{named}: {}",String::from_utf8_lossy(&err));
            let printed=smoozy_job_files_machine::reading_script::the_printed_reading_in(&out).unwrap();
            assert!(printed.errors.is_empty(),"{:?}",printed.errors);
            printed
        };
        let alone=read(isolated.as_os_str().as_bytes(),"alone");
        let linked=read(&folder.the_file,"linked");
        assert!(!alone.the_scene_holds_a_simulation_not_baked,"control: the linked simulation is absent");
        assert!(linked.the_scene_holds_a_simulation_not_baked,"the page sees the job's simulation");
        assert_eq!(linked.settings.0["scenes"][0]["prepare_simulations"],true);
        std::fs::remove_dir_all(opening).unwrap();

    }
}

/// **A cloth baked in the library**, its cache at the default folder beside the
/// library, as Blender writes it. The script is the small scene's.
fn a_library_with_a_baked_cloth() -> String {
    format!(
        r#"{THE_SMALL_SCENE}
bpy.ops.object.select_all(action="DESELECT")
bpy.ops.mesh.primitive_plane_add(size=2, location=(0, 0, 3))
plane = bpy.context.active_object
plane.name = "Cloth"
modifier = plane.modifiers.new("Cloth", "CLOTH")
cache = modifier.point_cache
cache.frame_start = 1
cache.frame_end = 3
scene.frame_end = 3
bpy.ops.wm.save_as_mainfile(filepath=out)
cache.use_disk_cache = True
with bpy.context.temp_override(point_cache=cache, object=plane):
    print("SMOOZY-DESK-BAKED", bpy.ops.ptcache.bake(bake=True), flush=True)
print("SMOOZY-DESK-MADE", bpy.app.version_string, cache.is_baked, cache.use_disk_cache, repr(cache.filepath), flush=True)
bpy.ops.wm.save_as_mainfile(filepath=out)
"#
    )
}

/// **`Job files and outputs` D21b-i through the new order, on a linked object.**
/// *Observed at the desk, 2026-11-04, 5.2.3:* a linked cloth reads its bake from
/// `blendcache_` and the library's name, beside the library, and an external one
/// measures `//` from the library. So the reading must look there, or the new
/// destination sends the cache to an empty folder.
#[test]
#[ignore = "run by hand at the desk with Blender 5.2 and 5.3 beta, one at a time"]
fn a_baked_cache_on_a_linked_object_is_found_beside_its_library() {
    let out = the_folder().join("linked-cache");
    for (named, blender) in the_blenders(&["5.2", "5.3"]) {
        println!("{named}:");
        let here = out.join(named);
        let library = make(&blender, &a_library_with_a_baked_cloth(), &here.join("assets"), "L");
        let the_cache = here.join("assets/blendcache_L");
        assert!(the_cache.is_dir(), "{named} baked: {}", the_cache.display());
        let top = make(&blender, &a_top_file_linking(&library, "Cloth"), &here, "T");
        let came = one_job_whole(
            &blender,
            &top,
            &here.join("run"),
            "Scene",
            &an_untouched_payload(),
            vec![1],
            Vec::new(),
            None,
            vec![a_library_pair("//assets/L.blend", &library)],
        );
        say(&came, "linked");
        assert_eq!(came.the_renders_status, Some(0), "{named}: {:?}", came.not_launched);
        let found = &came.the_composed.as_ref().expect("a reading").reading.the_baked_caches_on_the_machine;
        println!("    found: {found:?}");
        assert_eq!(found.len(), 1, "{named}: {found:?}");
        assert_eq!(
            found[0].its_files_sit_in.as_bytes(),
            std::fs::canonicalize(&the_cache).unwrap().display().to_string().as_bytes(),
            "{named}: the real folder, beside the real library"
        );
        println!("    in the job's folder: {:?}", came.in_the_jobs_folder);
        assert!(
            came.in_the_jobs_folder.iter().any(|name| name.ends_with(".bphys")),
            "{named}: the cache was copied: {:?}",
            came.in_the_jobs_folder
        );
        for line in came.the_renders_output.lines().filter(|line| line.contains("physics cache")) {
            println!("    the material: {line}");
        }
    }
}

/// **The reading's outputs still resolve against the file's real folder, and a cache
/// beside the file is handed on by its real path.** The same top file, once read
/// where it sits and once through the folder: every destination of its own maps to
/// the same place. **And the copy's source is no link**: the machine's copy refuses
/// one (`the_copy_on_the_machine`), so a link handed on here would fail a job that
/// works today.
#[test]
#[ignore = "run by hand at the desk with Blender 5.2 and 5.3 beta, one at a time"]
fn the_outputs_and_a_cache_beside_the_file_read_the_same_through_the_folder() {
    let out = the_folder().join("linked-outputs");
    for (named, blender) in the_blenders(&["5.2", "5.3"]) {
        println!("{named}:");
        let here = out.join(named);
        let library = make(&blender, &a_file_baked_into("//caches/shot/"), &here.join("assets"), "L");
        let script = format!(
            r#"{THE_SMALL_SCENE}
scene.render.filepath = "//renders/nested/f_"
for one in list(bpy.data.objects):
    if one.type == "MESH" and one.name != "Cube":
        bpy.data.objects.remove(one)
plane = bpy.data.objects["Cube"]
plane.name = "OwnCloth"
modifier = plane.modifiers.new("Cloth", "CLOTH")
cache = modifier.point_cache
cache.frame_start = 1
cache.frame_end = 2
scene.frame_end = 2
bpy.ops.wm.save_as_mainfile(filepath=out)
cache.use_disk_cache = True
with bpy.context.temp_override(point_cache=cache, object=plane):
    print("SMOOZY-DESK-BAKED", bpy.ops.ptcache.bake(bake=True), flush=True)
with bpy.data.libraries.load({library:?}, link=True, relative=True) as (there, here):
    here.objects = ["Cube"]
for one in here.objects:
    scene.collection.objects.link(one)
print("SMOOZY-DESK-MADE", bpy.app.version_string, cache.is_baked, flush=True)
bpy.ops.wm.save_as_mainfile(filepath=out, relative_remap=True)
"#,
            library = library.display().to_string(),
        );
        let top = make(&blender, &script, &here, "T");
        let cache = here.join("blendcache_T");
        assert!(cache.is_dir(), "{named} baked: {}", cache.display());
        let job = |root: &str, pairs: Vec<Pair>| {
            one_job_whole(
                &blender,
                &top,
                &here.join(root),
                "Scene",
                &an_untouched_payload(),
                vec![1],
                Vec::new(),
                Some((&cache, "blendcache_T")),
                pairs,
            )
        };
        let alone = job("alone", Vec::new());
        say(&alone, "alone");
        let linked = job("linked", vec![a_library_pair("//assets/L.blend", &library)]);
        say(&linked, "linked");
        assert_eq!(alone.not_launched, None, "{named}");
        assert_eq!(linked.not_launched, None, "{named}");
        let the_output_of = |came: &WhatCameBack, root: &str| -> Vec<String> {
            let own = came.the_composed.as_ref().expect("a reading");
            let mut folders: Vec<String> = own
                .map
                .pairs
                .iter()
                .filter(|pair| pair.destination.as_an_added_write_place().is_none())
                .map(|pair| match &pair.outcome {
                    HowItMapped::UnderTheJobsFolder { new_folder, .. } => {
                        String::from_utf8_lossy(new_folder).replace(root, "<run>")
                    }
                    other => format!("{other:?}"),
                })
                .collect();
            folders.sort();
            folders
        };
        let the_real_folder = |root: &str| here.join(root).join("rendering/blendcache_T").display().to_string();
        let found = |came: &WhatCameBack| -> Vec<String> {
            came.the_composed
                .as_ref()
                .expect("a reading")
                .reading
                .the_baked_caches_on_the_machine
                .iter()
                .map(|one| String::from_utf8_lossy(one.its_files_sit_in.as_bytes()).into_owned())
                .collect()
        };
        println!("    alone found {:?}; linked found {:?}", found(&alone), found(&linked));
        assert_eq!(
            the_output_of(&alone, &here.join("alone").display().to_string()),
            the_output_of(&linked, &here.join("linked").display().to_string()),
            "{named}: the outputs of the top file map to the same places"
        );
        let real = |root: &str| std::fs::canonicalize(the_real_folder(root)).unwrap().display().to_string();
        assert_eq!(found(&linked), vec![real("linked")], "{named}: no link is handed to the copy");
        assert_eq!(found(&alone), vec![real("alone")], "{named}: as it always read");
        assert!(
            std::fs::symlink_metadata(&found(&linked)[0]).unwrap().is_dir(),
            "{named}: the copy on the machine refuses a link"
        );
    }
}

/// **A wrapper that reads and does not render, with the file's scripts off.** The
/// reading and the render's material run; the render's own instruction, and all that
/// follows it, is dropped. The film's scene asks for a lot of memory to render.
///
/// **It got this wrong once, on 2026-11-04**: it dropped `-f`, the line's word is
/// `--render-frame`, and three runs rendered the film's scene on the Creator's Mac
/// until they were stopped. So [`nothing_renders_through_the_wrapper`] now proves it on
/// a small file before the film is opened.
fn a_blender_that_does_not_render(blender: &Path, here: &Path) -> PathBuf {
    std::fs::create_dir_all(here).expect("a folder");
    let wrapper = here.join("blender-reads-only");
    std::fs::write(
        &wrapper,
        format!(
            r#"#!/bin/bash
words=()
while (($#)); do
    case "$1" in
        --enable-autoexec) ;;
        --render-frame|-s|-e|-j) shift ;;
        -a) ;;
        --) break ;;
        *) words+=("$1") ;;
    esac
    shift
done
exec '{}' --disable-autoexec "${{words[@]}}"
"#,
            blender.display()
        ),
    )
    .expect("the wrapper");
    std::fs::set_permissions(&wrapper, std::fs::Permissions::from_mode(0o755)).expect("runnable");
    wrapper
}

/// **The wrapper is proved on a small file first**: a job through it writes no frame.
/// It stops the test before the film's scene is opened, if a word on the line moves.
fn nothing_renders_through_the_wrapper(blender: &Path, reads_only: &Path, here: &Path) {
    let script = format!("{THE_SMALL_SCENE}\nbpy.ops.wm.save_as_mainfile(filepath=out)\n");
    let small = make(blender, &script, &here.join("the-wrapper"), "small");
    let came = one_job_whole(
        reads_only,
        &small,
        &here.join("the-wrapper/run"),
        "Scene",
        &an_untouched_payload(),
        vec![1],
        Vec::new(),
        None,
        Vec::new(),
    );
    assert_eq!(came.not_launched, None, "the small file reached its launch");
    assert!(
        !came.in_the_jobs_folder.iter().any(|name| name.ends_with(".png")),
        "the wrapper let a render through, so the film's scene is not opened: {:?}",
        came.in_the_jobs_folder
    );
    assert!(!came.the_line.is_empty());
}

const THE_LIBRARIES_LISTED: &str = r#"
import bpy
for one in bpy.data.libraries:
    print("SMOOZY-DESK-LIBRARY", one.filepath, flush=True)
"#;

/// **The film's own scene, read through the new order and rendered by nobody.** The
/// top file alone in a folder and each library that exists linked at its recorded
/// place, the way the normal route leaves them. It says how long a reading takes each
/// way, what it found, and that the film's bake finds its place and the render's
/// material sets it.
#[test]
#[ignore = "run by hand at the desk, naming the scene's folder in SMOOZY_DESK_FILM"]
fn the_films_own_scene_is_read_and_not_rendered() {
    const THE_BAKE: &str = "//caches/shots/100_0040/critter_evolved-movement_bubbles/";
    let Ok(film) = std::env::var("SMOOZY_DESK_FILM") else {
        println!("SMOOZY_DESK_FILM names no scene, so nothing was read");
        return;
    };
    let film = PathBuf::from(film);
    let the_top_file = film.join("100_0040-lighting.blend");
    let out = the_folder().join("the-film");
    for (named, blender) in the_blenders(&["5.0", "5.2", "5.3"]) {
        println!("{named}:");
        let here = out.join(named);
        let reads_only = a_blender_that_does_not_render(&blender, &here);
        nothing_renders_through_the_wrapper(&blender, &reads_only, &here);
        let lister = here.join("list.py");
        std::fs::write(&lister, THE_LIBRARIES_LISTED).expect("written");
        let (_, said, _) = run(
            &blender,
            &[
                "--factory-startup".to_string(),
                "--background".to_string(),
                "--disable-autoexec".to_string(),
                the_top_file.display().to_string(),
                "--python".to_string(),
                lister.display().to_string(),
            ],
        );
        let said = String::from_utf8_lossy(&said);
        let mut pairs = Vec::new();
        let mut the_absent = 0;
        for recorded in said.lines().filter_map(|line| line.strip_prefix("SMOOZY-DESK-LIBRARY ")) {
            let real = film.join(recorded.trim_start_matches("//"));
            if real.exists() {
                if !pairs.iter().any(|pair: &Pair| pair.match_text.as_bytes() == recorded.as_bytes()) {
                    pairs.push(a_library_pair(recorded, &real));
                }
            } else {
                the_absent += 1;
            }
        }
        println!("    {} libraries have a pair, {the_absent} entries are not in the folder", pairs.len());
        // **One frame, so the material steps nothing.** The reading's own time does
        // not turn on how many frames are asked, since nothing on this scene moves.
        let frames: Vec<i64> = vec![101];
        let job = |root: &str, pairs: Vec<Pair>, copies: Vec<TheCopyTheMachineOwes>| {
            one_job_whole(
                &reads_only,
                &the_top_file,
                &here.join(root),
                "100_0040-lighting",
                &an_untouched_payload(),
                frames.clone(),
                copies,
                None,
                pairs,
            )
        };
        let copy = TheCopyTheMachineOwes {
            from: MachineLocation::composed(film.join(THE_BAKE.trim_start_matches("//")).display().to_string().into_bytes()),
            place: AnAddedWritePlace::TheGeometryNodesBakeFolder,
            recorded: RecordedLocation::as_the_reading_carried_it(THE_BAKE.as_bytes()),
        };
        // **The film's first job, as it went: the top file alone, and a copy owed.**
        let alone = job("alone", Vec::new(), vec![copy.clone()]);
        say(&alone, "the top file alone");
        assert!(
            alone.not_launched.as_deref().is_some_and(|said| said.contains("AnUploadedBakeCouldNotBeCopied")),
            "{named}: read away from its libraries, the map holds no place: {:?}",
            alone.not_launched
        );
        let whole = job("through-the-folder", pairs, vec![copy]);
        say(&whole, "through the folder");
        let size = |root: &str| std::fs::metadata(here.join(root).join("reading.json")).map(|it| it.len()).unwrap_or(0);
        println!("    the reading's record: alone {} bytes, through the folder {} bytes", size("alone"), size("through-the-folder"));
        for (what, came) in [("alone", &alone), ("through the folder", &whole)] {
            if let Some(own) = came.the_composed.as_ref() {
                println!(
                    "    {what}: {} pairs in the map, simulation {}, baked caches {}",
                    own.map.pairs.len(),
                    own.reading.the_scene_holds_a_simulation_not_baked,
                    own.reading.the_baked_caches_on_the_machine.len()
                );
            }
        }
        println!("    held: {:?}", whole.holds());
        for line in whole.the_renders_output.lines().filter(|line| line.contains("set_on") || line.contains("Stepped") || line.contains("stepped")) {
            println!("    the material: {line}");
        }
        assert_eq!(whole.not_launched, None, "{named}: the bake found its place");
        assert!(
            !whole.the_renders_output.contains("Fra:") && !whole.the_renders_output.contains("Rendering single"),
            "{named}: nothing rendered"
        );
        assert!(
            whole.in_the_jobs_folder.iter().any(|name| name.contains("1372380190")),
            "{named}: the bake's folder was copied: {:?}",
            whole.in_the_jobs_folder.iter().take(5).collect::<Vec<_>>()
        );
    }
}

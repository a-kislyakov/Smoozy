//! **§3.2 and §3.5's own script — G8 and G19 — the thing that runs inside the
//! Blender the check's reading drives, reached at the path it has always been
//! reached at.**
//!
//! **It lives whole in `smoozy-dependencies-machine` from 2026-09-29, step 10c's
//! dependency wave piece 2**, with every Blender name it uses quoted there and
//! the launch line beside it: G1 makes the reading *"driven as a
//! file-inspection worker through the supervisor"*, so the machine's own
//! program composes the script and starts Blender itself, and that program may
//! reach that crate and never this one. **Its tests stay here**, because what
//! they hold it to — [`crate::check`]'s kinds, [`crate::check::THE_MAP`],
//! [`crate::remap`], the fixing's generator — is this crate's.
//!
//! [`crate::arriving_later`] names this act: *"the walk over Blender's own path
//! marks joined with the map and the flat list, the node-tree walk into every
//! node group for the import nodes, and the walk over the record of what uses
//! what for scene attribution."*

/// **The walk over the nested marks, at the path it has always been reached
/// at.** It lives in `smoozy-dependencies-machine`, because the remap material
/// the rented machine composes carries it verbatim. Split at step 10c piece 3.
/// **Read here by the tests alone** since the 38's piece 7, when the fixing's
/// script, its other reader, moved there too.
#[cfg(test)]
pub(crate) use smoozy_dependencies_machine::reading_script::THE_WALK_OVER_THE_NESTED_MARKS;

/// **The script**, at the path it has always been reached at. Moved to
/// `smoozy-dependencies-machine` at step 10c's dependency wave piece 2.
pub use smoozy_dependencies_machine::check_script::the_check_readings_script;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::acting::blender;

    /// The script as the machine gets it — the shared walk and the body, the
    /// one call [`super::the_check_readings_script`] makes.
    fn the_script() -> String {
        the_check_readings_script()
    }

    /// **A8's other half: the script is a constant too.** Nothing about a
    /// reading is composed per file, so the same bytes are written to every
    /// machine and no value of Smoozy's is ever spliced into it.
    #[test]
    fn the_script_carries_no_value_and_names_the_marks_the_reader_looks_for() {
        // The file the script reads is the one Blender was launched on, so
        // the script never names a path — not a carriage token, not a root,
        // not a location of the user's.
        assert!(!the_script().contains(".smoozy-carriage"));
        assert!(!the_script().contains("/root/"));
        assert!(the_script().contains(
            std::str::from_utf8(blender::THE_OPENING_SENTINEL).expect("the mark is text")
        ));
        assert!(the_script().contains(
            std::str::from_utf8(blender::THE_CLOSING_SENTINEL).expect("the mark is text")
        ));
    }

    /// **G6: the map is named and the flat list is named, and neither stands
    /// in for the other.** The clause names both by their exact Blender names,
    /// and this is that clause held against the script rather than against a
    /// memory of it.
    #[test]
    fn the_script_walks_both_routes_and_the_map_is_not_optional() {
        assert!(the_script().contains(crate::check::THE_MAP));
        assert!(the_script().contains(crate::check::THE_FLAT_LIST));
        // The map is asked for with the libraries in, which is G6's own reason
        // for preferring it: it reaches through a library where the flat list
        // does not.
        assert!(the_script().contains("include_libraries=True"));
        const { assert!(crate::check::THE_FLAT_LIST_IS_NEVER_SUBSTITUTED_FOR_THE_MAP) };
    }

    /// **M9a: nothing anywhere in this module reads `image.has_data`**, and the
    /// script is the one place a reader might reach for it.
    #[test]
    fn the_script_never_reads_the_flag_m9a_names() {
        assert!(!the_script().contains(concat!("has", "_data")));
        const { assert!(crate::remap::NOTHING_HERE_READS_HAS_DATA) };
    }

    /// **G8: the node walk steps into node groups, and it reads the node's own
    /// path value rather than any bulk route.**
    #[test]
    fn the_node_walk_steps_into_groups_and_reads_the_import_nodes_own_value() {
        assert!(the_script().contains("GeometryNodeImport"));
        assert!(the_script().contains("node_tree"));
        assert!(the_script().contains("compositing_node_group"));
    }

    /// **§12 item 20 and G6b: the reading observes no stand-in, and no space is
    /// left for an answer that is not coming.**
    #[test]
    fn the_reading_observes_no_stand_in_and_says_nothing_about_one() {
        assert!(the_script().contains("\"observed_stand_ins\": []"));
    }

    /// The three shapes the script names a kind in, held against one kind.
    ///
    /// **The per-setting walk** writes it as the second half of a collection
    /// entry; **the nested walk** yields it; **the node walk** writes it
    /// straight into the reference. Nothing else counts — the map's own
    /// `BY_TYPE` names kinds too, and a kind that appears only there is
    /// reached only by accident of an owner's type and is exactly the fault
    /// this test is about.
    fn the_script_can_produce(kind: crate::check::Kind) -> bool {
        let named = serde_json::to_string(&kind).expect("a kind renders");
        let named = named.trim_matches('"');
        let script = the_script();
        script.contains(&format!(", \"{named}\"),"))
            || script.contains(&format!("yield (\"{named}\","))
            || script.contains(&format!("\"kind\": \"{named}\","))
    }

    /// **G6's list, held against what the script can actually produce — the
    /// test the reading never had.**
    ///
    /// **G6 lists about twenty-seven kinds and [`crate::check::Kind`] carries
    /// twenty-six, and the script used to produce nine of them.** Eleven
    /// variants occurred nowhere outside the enum's own declaration, four more
    /// only inside constants describing them, and the weak reference only as
    /// G27's exclusion. **Nothing failed, nothing was reported, and a file with
    /// a fluid cache or an OSL script came back checked and clean.**
    ///
    /// So every kind must now be one of two things and say which: **reached by
    /// the script**, or **a named absence** in
    /// [`crate::check::THE_KINDS_NO_REFERENCE_IS_PRODUCED_FOR`] with what
    /// breaks written beside it. Neither, and it fails here; both, and it fails
    /// here too, because a kind that is reached and also called an absence
    /// means one of the two records is stale.
    ///
    /// **What it cannot see.** That a route actually works inside a running
    /// Blender — no machine here runs one — and that a property name is the
    /// right one, which is Rule 2's business and is answered by the quotations
    /// at [`THE_WALK_OVER_THE_NESTED_MARKS`] rather than by a test.
    /// **The two external-displacement modifiers are reached separately**, and
    /// each branch names its own type value.
    ///
    /// **This is the test that would have caught it**: one `Kind` covered both
    /// modifiers, so the guard above passed on the Multires branch alone while
    /// the Mesh Cache half was reached by nothing and named nowhere.
    #[test]
    fn both_external_displacement_modifiers_are_reached_and_each_branches_on_its_own_type() {
        let script = the_script();
        assert!(the_script_can_produce(
            crate::check::Kind::MultiresExternalDisplacement
        ));
        assert!(the_script_can_produce(
            crate::check::Kind::MeshCacheExternalDisplacement
        ));
        for value in ["\"OCEAN\"", "\"MULTIRES\"", "\"MESH_CACHE\""] {
            assert!(
                script.contains(&format!("of_type == {value}")),
                "the walk branches on {value}"
            );
        }
    }

    #[test]
    fn every_kind_is_either_reached_by_the_script_or_a_named_absence() {
        let mut faults = Vec::new();
        for kind in crate::check::Kind::ALL {
            let reached = the_script_can_produce(kind);
            let named = crate::check::THE_KINDS_NO_REFERENCE_IS_PRODUCED_FOR.contains(&kind);
            if !reached && !named {
                faults.push(format!(
                    "{kind:?} is on G6's list, the script produces no reference for it, \
                     and nothing anywhere says so — a file carrying one comes back \
                     checked and clean"
                ));
            }
            if reached && named {
                faults.push(format!(
                    "{kind:?} is reached by the script and also recorded as an absence"
                ));
            }
        }
        assert!(faults.is_empty(), "{faults:#?}");
    }

    /// **A pair reaches exactly what the check saw**, because the remap and the
    /// fixing walk the very generator the reading walked rather than a second
    /// copy of it.
    ///
    /// The nested kinds have no collection to be taken from — the holder is a
    /// node, a modifier, a strip, a surface or a cache layer — so the walk is
    /// the only route at both ends, and two copies of it would drift.
    #[test]
    fn the_reading_the_remap_and_the_fixing_walk_one_generator() {
        use crate::acting::{fixing_worker, material};
        use crate::bytes::{MachineLocation, RecordedLocation};
        use crate::check::Kind;
        use crate::remap::{compose_pair, NewLocation};

        let pair = compose_pair(
            Kind::FluidCacheFolder,
            "cache_directory",
            &RecordedLocation::as_the_reading_carried_it(b"//cache/fluid/"),
            NewLocation::AMirror(MachineLocation::composed(b"/root/a/cache/fluid/".to_vec())),
        )
        .expect("the pair composes");
        let material = material::the_material_python(&crate::remap::RemapMaterial::composed(
            &smoozy_state::identities::Identity::issue(smoozy_state::identities::IdentityKind::Job)
                .expect("randomness"),
            &smoozy_render_settings::file_half::BlenderIdentity {
                path: "/opt/blender/blender".to_string(),
                version: smoozy_server_setup::blender::Version {
                    major: 4,
                    minor: 5,
                    patch: 9,
                },
            },
            vec![pair.clone()],
        ));
        let fixing = fixing_worker::the_fixing_python(&[pair], b"/staging");
        for text in [&material, &fixing] {
            assert!(text.contains(THE_WALK_OVER_THE_NESTED_MARKS));
            assert!(text.contains("smoozy_nested_marks()"));
            assert!(text.contains("SMOOZY_NESTED_KINDS"));
        }
        assert!(the_script().contains(THE_WALK_OVER_THE_NESTED_MARKS));
    }

    /// Every line of a `try` statement's `except` body, with the line number
    /// the `except` sits on. Indentation is what closes a block, which is
    /// Python's own rule and is enough here because the script is this file's
    /// own constant and is not generated.
    fn the_except_bodies() -> Vec<(usize, String)> {
        let script = the_script();
        let lines: Vec<&str> = script.lines().collect();
        let mut out = Vec::new();
        for (at, line) in lines.iter().enumerate() {
            let indent = line.len() - line.trim_start().len();
            if !line.trim_start().starts_with("except") {
                continue;
            }
            for below in lines.iter().skip(at + 1) {
                if below.trim().is_empty() {
                    continue;
                }
                if below.len() - below.trim_start().len() <= indent {
                    break;
                }
                out.push((at + 1, below.trim().to_string()));
            }
        }
        out
    }

    /// **G3, held over the check reading's own script: no branch turns a
    /// missing answer into an empty one.**
    ///
    /// *"A run that returns no sentinel-framed document, or a failing status,
    /// is could not be read, never an empty list"*, and
    /// [`crate::acting`]'s own rule repeats it: **no branch here turns a
    /// missing answer into an empty one.** The loud ending exists —
    /// the check's own launch line carries `--python-exit-code 1`
    /// (`smoozy_dependencies_machine::check_script::the_words_of_the_check_reading`),
    /// [`super::checking`] judges over both streams and
    /// the status, and [`crate::check::judge`] turns a failing status into
    /// *could not be read* — **and a catch inside the script disarms all of
    /// it**, because the run then exits 0 with a well-formed document that is
    /// short.
    ///
    /// **Three catches did exactly that and nothing anywhere would have caught
    /// them.** [`super::material`] has had the equivalent test over the remap
    /// fragment since it was written; the reading's script had none, and that
    /// is why they survived. Two substituted an empty answer — the map, whose
    /// loss costs G6's own stated reason for having a map, and the flat list,
    /// which G27's exclusion is read against. **The third was a different
    /// mechanism and is the worse one**: it dropped one data-block's reference
    /// with `continue`, so the file came back *checked* and one reference
    /// short, with nothing saying so.
    ///
    /// **What this test forbids**, in an `except` body anywhere in the script:
    /// an assignment of an empty collection, and a `continue` or a `pass`.
    /// Those are the two mechanisms, named.
    ///
    /// **What it cannot see, said rather than glossed.** A catch that
    /// substitutes something that is not an empty literal — a value read from
    /// somewhere else, or a collection built above — reads as ordinary code to
    /// this scan. The count below is the guard against that: a fourth `try`
    /// cannot be added without somebody moving a number and saying what it is
    /// for.
    #[test]
    fn no_catch_in_the_reading_script_turns_a_missing_answer_into_an_empty_one() {
        let mut faults = Vec::new();
        for (at, code) in the_except_bodies() {
            if code == "continue" || code == "pass" {
                faults.push(format!(
                    "line {at}: the catch drops what it was reading and says nothing"
                ));
            }
            let substitutes = ["{}", "[]", "()", "set()", "dict()", "list()", "tuple()"]
                .iter()
                .any(|empty| code.ends_with(&format!("= {empty}")));
            if substitutes {
                faults.push(format!(
                    "line {at}: the catch substitutes an empty answer — `{code}`"
                ));
            }
        }
        assert!(!the_script().contains("contextlib.suppress"));
        assert!(faults.is_empty(), "{faults:#?}");
    }

    /// **The two catches the script is allowed, counted exactly and each named
    /// for what it is.**
    ///
    /// Both are G20a's, both belong to the walk over the record of what uses
    /// what, and **both end in an explicit named absence rather than in an
    /// empty answer**: the walk that could not be made reports
    /// *NotEstablished* on every reference, which is a value a reader has to
    /// handle, not a silence. **They are the shape the three removed ones
    /// were not.**
    ///
    /// The count is asserted exactly rather than as a ceiling, on the same
    /// discipline [`crate::markers`] uses for the file count: a ceiling cannot
    /// fail on a catch somebody added, and a catch added quietly is the whole
    /// fault this pair of tests exists for.
    ///
    /// **A third from 2026-11-02, G6d's**, around the walk of the file's own
    /// bytes: it ends in no list, so a row says what Blender read, as before.
    /// It is a figure to show and no part of what the render needs.
    #[test]
    fn the_script_carries_exactly_the_three_catches_that_name_their_absence() {
        let tries = the_script()
            .lines()
            .filter(|line| line.trim_start().starts_with("try:"))
            .count();
        assert_eq!(
            tries, 3,
            "a catch was added or removed without this count moving with it"
        );
        assert!(the_script().contains("    except Exception:\n        return []\n"));
        // Both end in G20a's named absence, and the script says the words.
        assert_eq!(the_script().matches("NotEstablished").count(), 2);
    }

    /// The body of the script's loop that begins at `opening`, up to the
    /// next line at the loop's own indentation or less — enough to say what
    /// one loop of this file's own constant does, and no more.
    fn the_loop_opened_by(opening: &str) -> String {
        let script = the_script();
        let lines: Vec<&str> = script.lines().collect();
        let at = lines
            .iter()
            .position(|line| line.trim_start().starts_with(opening))
            .unwrap_or_else(|| panic!("the script opens a loop with {opening:?}"));
        let indent = lines[at].len() - lines[at].trim_start().len();
        let mut body = String::new();
        for line in lines.iter().skip(at + 1) {
            if !line.trim().is_empty() && line.len() - line.trim_start().len() <= indent {
                break;
            }
            body.push_str(line);
            body.push('\n');
        }
        body
    }

    /// **G6 with DW0-1: what is carried inside the file is not a reference,
    /// and neither is a setting that records nothing — in the per-setting
    /// walk and in the map alike.** On the splash scenes every packed
    /// picture came back a *not found* row, and Render Result, a Viewer
    /// node, a generated picture and every text written inside Blender came
    /// back as one row with an empty location. **The map is held to the same
    /// two tests**, or what the walk leaves out comes straight back through
    /// it as a new reference.
    #[test]
    fn the_walk_and_the_map_both_leave_out_what_is_carried_inside_and_what_records_nothing() {
        let script = the_script();
        // The two tests, by the names the reference gives.
        assert!(script.contains("getattr(datablock, \"packed_file\", None) is not None"));
        assert!(script.contains("getattr(datablock, \"is_archive\", False)"));
        assert!(script.contains("recorded is None or recorded == \"\""));

        let walk = the_loop_opened_by("for collection, kind in COLLECTIONS:");
        assert!(
            walk.contains("if records_nothing(recorded) or carried_inside_the_file(datablock):"),
            "the per-setting walk keeps both tests:\n{walk}"
        );
        let map = the_loop_opened_by("for datablock, paths in mapped.items():");
        assert!(
            map.contains("if carried_inside_the_file(datablock):"),
            "the map keeps the packed test:\n{map}"
        );
        assert!(
            map.contains("if records_nothing(path):"),
            "the map keeps the empty-location test:\n{map}"
        );
        // The flat list produces no reference, and is still held to the
        // empty-location test — after each path the load-time handler set is
        // told back to the file's own (`Job files and outputs` R18, since
        // 2026-10-02).
        assert!(script.contains("for one in map(smoozy_a_path_as_the_file_has_it, bpy.utils.blend_paths())\n    if not records_nothing(one)"));
    }

    /// **G6 with DW0-1: a geometry-node bake that keeps its data inside the
    /// file is not a reference — decided in the check's own loop over the
    /// shared walk, and never inside the shared walk**, which the remap and
    /// the fixing walk too. A bake's own `INHERIT` reads through to its
    /// modifier.
    #[test]
    fn the_bake_test_is_in_the_checks_loop_and_never_in_the_shared_walk() {
        assert!(
            !THE_WALK_OVER_THE_NESTED_MARKS.contains("bake_target"),
            "the shared walk decides nothing about a bake's target"
        );
        let nested = the_loop_opened_by(
            "for kind, where, holder, attribute, owner in smoozy_nested_marks():",
        );
        assert!(nested.contains(
            "if kind == \"GeometryNodeBakeFolder\" and the_bake_keeps_its_data_inside_the_file(holder, attribute):"
        ));
        assert!(nested.contains("if records_nothing(recorded):"));
        let script = the_script();
        for piece in [
            "if getattr(holder, \"bake_target\", None) != \"PACKED\":",
            "if getattr(bake, \"bake_target\", None) == \"DISK\":",
            "if target == \"INHERIT\":",
            "modifier_of_bake.get(holder.as_pointer())",
            "modifier_of_bake[bake.as_pointer()] = modifier",
            "return target == \"PACKED\"",
        ] {
            assert!(script.contains(piece), "{piece}");
        }
    }

    /// **G9 with DW0-3: a picture's and a movie clip's source rides with the
    /// reference**, from the walk and from the map alike, and nothing else
    /// carries one.
    #[test]
    fn a_pictures_and_a_movie_clips_source_rides_with_the_reference() {
        let script = the_script();
        assert!(script.contains("if kind in (\"Image\", \"MovieClip\"):"));
        assert!(script.contains("return getattr(datablock, \"source\", None)"));
        assert!(the_loop_opened_by("for collection, kind in COLLECTIONS:")
            .contains("\"source\": stored_source(kind, datablock),"));
        assert!(
            the_loop_opened_by("for datablock, paths in mapped.items():")
                .contains("\"source\": stored_source(kind, datablock),")
        );
        // Every reference the script writes says whether it carries one.
        assert_eq!(
            script.matches("\"kind\": kind,").count()
                + script.matches("\"kind\": \"ImportNodePath\",").count(),
            script.matches("\"source\":").count(),
        );
    }
}

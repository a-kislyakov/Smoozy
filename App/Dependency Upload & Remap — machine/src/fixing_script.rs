//! **M14's script and launch line — the fixing, as the machine's own
//! file-inspection worker runs it.** Moved here on 2026-10-04, the 38's piece
//! 7, from `smoozy-dependencies`' `acting::fixing_worker`, when the fixing
//! became a file-inspection operation (O2, settled 2026-10-03): the machine's
//! own program now writes this script and starts Blender with these words, so
//! the script lives where that program can take it. **Nothing in it changed
//! but how the staging path reaches it**, which is the one thing a link and a
//! fixed file name carried until then.
//!
//! # What the script does, in M14's order
//!
//! **The machine file is opened headless** (the launch line below), **each
//! resolved reference is rewritten by M5's per-setting route** — the same head
//! and the same walk the remap material uses, so the fixing and the render
//! rewrite by one route rather than two — **every pair is reported** on the
//! material's own per-pair line, and **the result is saved to the staging path
//! as a copy with the save-time relative remapping off**. The publication, one
//! rename, is the desktop's (`smoozy-dependencies`' `acting::fixing_worker`),
//! because it reads this report first and publishes only a copy in which every
//! pair applied.
//!
//! # Every Blender name below, with the publication it was read from
//!
//! **Read from Blender's own published Python API reference, bundled with this
//! project's Blender tooling, on 2026-09-04**, by the builder who first wrote
//! this script; unchanged since.
//!
//! - **`bpy.ops.wm.save_as_mainfile(*, filepath="", …, compress=False,
//!   relative_remap=True, copy=False)`** — *"Save the current file in the
//!   desired location"*, with **`relative_remap`** *"Remap Relative, Remap
//!   relative paths when saving to a different directory"* and **`copy`**
//!   *"Save Copy, Save a copy of the actual working state but does not make
//!   saved file active"* (`api/bpy.ops.wm.rst`).
//!
//! **The flag's own condition is the case this act is always in.** Writing the
//! copy to a staging path **is** saving to a different directory, so the trap
//! M14 was written against — Fact 77's first recorded trap — is the ordinary
//! case here, and turning the flag off is the whole of what stops it.
//!
//! **The fixing assigns and does not reload.** M8's reload belongs to the remap
//! at render time, where the point is to make a library's data arrive in the
//! running Blender; here the point is what the saved file records.
//!
//! # The launch line, and why it carries no load-time script
//!
//! **The words are the ones M14's fixing has always been launched with**, read
//! from Blender's own manual (`manual/advanced/command_line/arguments.rst`, on
//! 2026-09-04, by the same builder): `-b`, `--factory-startup`, the file, then
//! `--python-exit-code 1` and `--python` with the script. **They differ from the
//! check's reading in one place and it is deliberate**: the reading carries
//! `Rendering` L8b's load-time script, whose handler points every disk cache at
//! a throwaway folder (`Job files and outputs` R18). **The fixing saves what
//! the running Blender holds**, so that handler's throwaway locations would be
//! written into the fixed copy and every cache the library reads would point at
//! a folder removed when the run ends. A reading writes nothing, so the handler
//! costs it nothing; here it would be a silently wrong fixed copy.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Job files and outputs` D18, R18. `Rendering` L8b.

use crate::remap::Pair;

/// **The fixing's own first line**, written before anything else the script
/// does — `Render Settings` Q8's *"Smoozy's own frame"*, which tells a run that
/// started the script from one that did not. Smoozy's own text.
pub const THE_FIXINGS_START_MARK: &str = "<<<SMOOZY-FIXING-STARTED>>>";

/// **Whether a captured stream shows the fixing's script started.**
pub fn the_fixing_started_in(stream: &[u8]) -> bool {
    let mark = THE_FIXINGS_START_MARK.as_bytes();
    stream.windows(mark.len()).any(|window| window == mark)
}

/// **M14's script for one fixed copy**: the pairs it writes, and the staging
/// path it saves to.
///
/// **Both travel as data and never as text spliced into code** — the pairs as
/// one JSON document inside a raw string, the staging path as its own bytes in
/// a JSON list, decoded by `os.fsdecode` the way every location the material
/// assigns is. A path that is not text crosses whole.
pub fn the_fixing_python(pairs: &[Pair], staging: &[u8]) -> String {
    let pairs =
        serde_json::to_string(pairs).expect("the pairs are composed of values that always render");
    let staging = serde_json::to_string(staging).expect("bytes always render");
    // JSON's only string delimiter is the double quote, so three single quotes
    // cannot appear in either document.
    debug_assert!(!pairs.contains("'''") && !staging.contains("'''"));
    format!(
        "import sys\nsys.stdout.write({start:?} + \"\\n\")\nsys.stdout.flush()\n{head}\n{walk}\nPAIRS = json.loads(r'''{pairs}''')\nSMOOZY_STAGING = os.fsdecode(bytes(json.loads(r'''{staging}''')))\n{body}",
        start = THE_FIXINGS_START_MARK,
        head = crate::material::THE_HEAD,
        walk = crate::reading_script::THE_WALK_OVER_THE_NESTED_MARKS,
        body = THE_FIXINGS_BODY,
    )
}

/// **The words after the build's own path, for one fixing**: the startup
/// options, the file, then the script. **Every value is a word of its own**,
/// handed to the kernel as an argument and never spliced into a line a shell
/// reads (A8, `Server protocol and transfers` §1.1).
pub fn the_words_of_the_fixing(the_blend_path_on_the_machine: &[u8], the_scripts_path: &[u8]) -> Vec<Vec<u8>> {
    vec![
        b"-b".to_vec(),
        b"--factory-startup".to_vec(),
        the_blend_path_on_the_machine.to_vec(),
        b"--python-exit-code".to_vec(),
        b"1".to_vec(),
        b"--python".to_vec(),
        the_scripts_path.to_vec(),
    ]
}

/// The fixing's own body — the rewrite, then the save.
const THE_FIXINGS_BODY: &str = r#"
for pair in PAIRS:
    wanted = bytes(pair["match_text"])
    new_location = os.fsdecode(bytes(pair["new_location"]))
    kind = pair["kind"]
    setting = pair["setting"]
    reached = 0
    found = []

    if kind == "ImportNodePath":
        for tree in smoozy_every_tree():
            for node in getattr(tree, "nodes", ()):
                if not getattr(node, "bl_idname", "").startswith("GeometryNodeImport"):
                    continue
                for socket in getattr(node, "inputs", ()):
                    if socket.name != "Path":
                        continue
                    if getattr(socket, "is_linked", False):
                        continue
                    current = socket.default_value
                    if smoozy_match_text(current) == wanted:
                        socket.default_value = new_location
                        reached = reached + 1
                    else:
                        found.append(list(os.fsencode(current)))
    elif kind in SMOOZY_NESTED_KINDS:
        # **The same walk the reading took**, so the fixing rewrites exactly
        # what the check saw — and the same walk the remap material takes, so
        # the two halves are one route rather than two.
        for one_kind, where, holder, attribute, owner in smoozy_nested_marks():
            if one_kind != kind or attribute != setting:
                continue
            current = getattr(holder, attribute)
            if smoozy_match_text(current) == wanted:
                setattr(holder, attribute, new_location)
                reached = reached + 1
            else:
                found.append(list(os.fsencode(current)))
    else:
        # **M5's per-setting route, and no reload.** M8's reload belongs to the
        # remap at render time, where the point is to make a library's data
        # arrive; here the point is what the saved file records, and a reload
        # before a save would load data this act never asked for.
        holder = getattr(bpy.data, SMOOZY_COLLECTIONS[kind])
        for datablock in holder:
            current = getattr(datablock, setting)
            if smoozy_match_text(current) == wanted:
                setattr(datablock, setting, new_location)
                reached = reached + 1
            else:
                found.append(list(os.fsencode(current)))

    smoozy_report(pair, reached > 0, reached, False, [] if reached > 0 else found)

# **M14's save.** `relative_remap=False` because writing the copy to a staging
# path is saving to a different directory, which is the flag's own documented
# condition and the trap Fact 77 recorded; `copy=True` because the machine file
# stays what it is and M2 forbids writing to it.
bpy.ops.wm.save_as_mainfile(
    filepath=SMOOZY_STAGING,
    relative_remap=False,
    copy=True,
)
"#;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bytes::{MachineLocation, RecordedLocation};
    use crate::check::Kind;
    use crate::remap::{compose_pair, NewLocation};

    fn a_pair() -> Pair {
        compose_pair(
            Kind::Image,
            "filepath",
            &RecordedLocation::as_the_reading_carried_it(b"//tex/METallic.png"),
            NewLocation::AMirror(MachineLocation::composed(
                b"/root/assets/files/Users/a/tex/METallic.png".to_vec(),
            )),
        )
        .expect("the pair composes")
    }

    fn the_code_of(script: &str) -> String {
        script
            .lines()
            .filter(|line| !line.trim_start().starts_with('#'))
            .collect::<Vec<&str>>()
            .join("\n")
    }

    /// **M14's save, and the two arguments the clause turns on** — read in
    /// the code and never the comments, which name both flags to say why
    /// (a fault put into it on 2026-09-04 proved a whole-text test passes with
    /// the flag turned back on).
    #[test]
    fn the_save_turns_off_the_relative_remapping_and_writes_a_copy() {
        let code = the_code_of(&the_fixing_python(&[a_pair()], b"/root/assets/fixed/s"));
        assert!(code.contains("relative_remap=False"), "{code}");
        assert!(!code.contains("relative_remap=True"), "{code}");
        assert!(code.contains("copy=True"), "{code}");
        assert!(code.contains("save_as_mainfile"), "{code}");
    }

    /// **The staging path crosses as its own bytes**, never as text spliced
    /// into code: a path that is not text, or that carries a quote, reaches
    /// the script whole. Until the 38's piece 7 it reached the script through
    /// a file at a fixed name beside it.
    #[test]
    fn the_staging_path_crosses_as_its_bytes() {
        let staging = b"/root/assets/fixed/a'b\xff/.smoozy-staging/c.blend".to_vec();
        let script = the_fixing_python(&[a_pair()], &staging);
        let list = serde_json::to_string(&staging).expect("it renders");
        assert!(script.contains(&format!("SMOOZY_STAGING = os.fsdecode(bytes(json.loads(r'''{list}''')))")));
        assert!(!script.contains("dependency-staging-path"), "no file at a fixed name");
        assert!(!script.contains("a'b"), "nothing of the path is spliced as text");
    }

    /// **The start mark is the script's very first act**, before the head
    /// imports Blender's own module, so a run that started the script is told
    /// from one that never did.
    #[test]
    fn the_start_mark_comes_before_anything_else() {
        let script = the_fixing_python(&[a_pair()], b"/s");
        let mark = script.find(THE_FIXINGS_START_MARK).expect("the mark is written");
        let head = script.find("import bpy").expect("the head");
        assert!(mark < head);
        assert!(the_fixing_started_in(format!("noise\n{THE_FIXINGS_START_MARK}\n").as_bytes()));
        assert!(!the_fixing_started_in(b"Blender quit\n"));
    }

    /// **`Job files and outputs` D18's discipline**: the fixing catches
    /// nothing, so a rewrite that could not be made is a raise.
    #[test]
    fn the_fixing_catches_nothing() {
        for line in the_code_of(&the_fixing_python(&[a_pair()], b"/s")).lines() {
            let code = line.trim_start();
            assert!(!code.starts_with("try"), "{code}");
            assert!(!code.starts_with("except"), "{code}");
        }
    }

    /// **The fixing assigns and does not reload**, which is the one place it
    /// differs from the remap at render time.
    #[test]
    fn the_fixing_does_not_reload() {
        assert!(!the_fixing_python(&[a_pair()], b"/s").contains("datablock.reload()"));
    }

    /// **One rewrite route rather than two**: the matching, the per-setting
    /// route and the reporting are the material's own, flushed line by line.
    #[test]
    fn the_fixing_and_the_material_share_one_rewrite_route() {
        let fixing = the_fixing_python(&[a_pair()], b"/s");
        assert!(fixing.contains("smoozy_match_text"));
        assert!(fixing.contains("SMOOZY_COLLECTIONS"));
        assert!(fixing.contains(crate::material::THE_PER_PAIR_HEAD));
        assert!(fixing.contains("sys.stdout.flush()"));
        assert!(fixing.contains(crate::reading_script::THE_WALK_OVER_THE_NESTED_MARKS));
    }

    /// **The launch line is M14's**, each value a word of its own, the file
    /// before the script, and **no load-time script** (this file's head).
    #[test]
    fn the_words_open_the_file_headless_then_run_the_script() {
        let words = the_words_of_the_fixing(b"/root/a b.blend", b"/area/fixing.py");
        assert_eq!(
            words,
            vec![
                b"-b".to_vec(),
                b"--factory-startup".to_vec(),
                b"/root/a b.blend".to_vec(),
                b"--python-exit-code".to_vec(),
                b"1".to_vec(),
                b"--python".to_vec(),
                b"/area/fixing.py".to_vec(),
            ]
        );
    }
}

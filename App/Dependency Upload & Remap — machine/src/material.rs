//! **§6.3 performed — M5, M8, M9, M9a and M9d.** The thing that applies each
//! pair inside the running Blender, and the per-pair lines it writes onto
//! `Job files and outputs` D16's marker.
//!
//! **This is the trap the whole step is about.** A dependency that did not
//! re-point costs a finished render, at full price, with nothing to see. M8
//! requires every pair reported with nothing failing quietly; M9d asks, per
//! pair, whether assignment alone is even established for that kind; and
//! `Job files and outputs` D18 forbids the material catching anything, on
//! D17's observation of a run that printed a permission refusal, rendered the
//! factory scene anyway, wrote a file and exited 0.
//!
//! # Where this runs and who composes it
//!
//! **M1: the route is `Job files and outputs` D9 and is not re-opened** — the
//! remap material rides the same launch material as the output redirect,
//! applied by the same lever at the same moment, inside the one window there
//! is: after the file, before the render instruction. **The launch material is
//! composed on the machine**, from the manifest and from the remap-material
//! document M10 placed there, so [`the_material_python`] is what the machine
//! calls with that document and what it gets back is the fragment to place in
//! the line.
//!
//! **What is composed on the desktop is the document and not this text** (M10):
//! the pairs, their expected bytes and their match texts are settled at
//! submission from the record, and this fragment only carries them out. Nothing
//! here re-derives a pair, and nothing here decides a setting name — M6 resolves
//! those per version at composition and this reads them.
//!
//! # Four rules the fragment keeps, and each is a clause rather than a habit
//!
//! - **M5: the per-setting route, in every version, and no bulk route and no
//!   one-pass route for any part of it.** The one-pass route is absent in 4.5
//!   outright, and in 5.0 and 5.2 it silently left every write location alone.
//!   **The route that is silently partial for the redirect is not given a second
//!   chance here**, so this fragment assigns one setting on one data-block at a
//!   time and never calls a sweep.
//! - **`Job files and outputs` D18: the material contains no failure handling of
//!   its own.** *"A failure raised inside a script that catches it leaves a
//!   successful exit status behind."* **There is no `try` in the fragment**, and
//!   a test asserts it: a kind with no collection, a setting that will not take
//!   a value, a library that will not reload — every one of them raises, and the
//!   raise is what `Rendering` L8a's failure-to-exit-status argument turns into
//!   a failed attempt rather than a paid wrong render.
//! - **M8: every pair is reported, and *not applied* is a countable fact.** A
//!   pair that matched zero data-blocks is reported as not applied **with the
//!   values that were found**, so a reader can tell an edited reference from a
//!   dropped one. M8a records the two known and benign causes of it: a reference
//!   the artist re-pointed, and a data-block nobody was using that a plain save
//!   dropped with its recorded place.
//! - **M9a: nothing reads `image.has_data`.** It reported false throughout while
//!   the render was reading the file correctly, so **anything that gated on it
//!   would call a working remap broken.** The marker reports what was assigned.
//!
//! # The one thing the fragment does differently per kind, and why
//!
//! **A library is assigned and then reloaded; an image is assigned and not
//! reloaded.** That is the per-kind difference Facts 99 and 101 were run to
//! find, and it is [`crate::remap::takes_a_reload_after_assignment`]'s, read
//! rather than decided here. **The relocate command is not used and is not
//! offered as a fallback**: it moved on a file linking one library and moved
//! nothing on a file linking two, in all four versions, so it is not a general
//! form for an ordinary multi-library file.
//!
//! **Every other kind is assigned and not reloaded, and the marker says the
//! sufficiency of that was never observed** — M9d's field, composed from the
//! pair's kind alone, so a kind nobody has posed reads *not established* by
//! construction rather than by omission. **§12 items 1 and 2 are permanently
//! unresolved and no space is left here for an answer**; what removes the
//! silence is the field, and it is written.
//!
//! # Every Blender name below, with the publication it was read from
//!
//! **Read from Blender's own published Python API reference, bundled with this
//! project's Blender tooling, on 2026-09-04.**
//!
//! - **`bpy.types.Library.reload()`** — *"Reload this library and all its linked
//!   data-blocks"* (`api/bpy.types.Library.rst`). **This is M8's reload.**
//! - **The eight collections and their `filepath`** — as
//!   [`super::reading_script`] quotes them, from each type's own page.
//! - **A group node's inner tree is `node_tree`, and a tree's nodes are
//!   `nodes`** (`api/bpy.types.NodeGroup.rst`, `api/bpy.types.NodeTree.rst`).
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Job files and outputs` D9, D16, D16a, D17, D18.
//! `Rendering` L8a.

use serde::{Deserialize, Serialize};

use crate::check::Kind;
use crate::remap::{AssignmentAlone, MatchText, Pair, PairOutcome, RemapMaterial};

/// **The head of every per-pair line the fragment writes.**
///
/// **It rides `Job files and outputs` D16's marker rather than opening a
/// channel of its own** — D16a records that in the clause where the marker is
/// defined. The head is a constant so a reader of a captured stream can find
/// the lines with nobody connected, which is what M8's *readable after the
/// fact* means.
pub const THE_PER_PAIR_HEAD: &str = "SMOOZY-REMAP-PAIR ";

/// One pair's outcome as the fragment writes it and as a reader takes it back
/// off the captured stream.
///
/// **The values found travel only for a pair that was not applied**, which is
/// the case M8 names them for; an applied pair says how many data-blocks it
/// reached and nothing about the rest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AsTheMarkerCarriesIt {
    pub kind: String,
    pub setting: String,
    /// **Which pair the line is about**, and it is the one field that makes
    /// M8's *reports every pair* checkable rather than assertable.
    ///
    /// A kind and a setting do not name a pair: a file with two images has two
    /// pairs, both `Image` and both `filepath`, and their two lines were
    /// indistinguishable. So a missing line could not be told from a present
    /// one, and the comparison M8 asks for could not be made at all. The match
    /// text is what M4a composes a pair on and what K3's keying makes unique,
    /// so it names the pair exactly.
    pub match_text: MatchText,
    pub applied: bool,
    pub data_blocks: usize,
    pub assignment_alone: AssignmentAlone,
    pub reloaded: bool,
    #[serde(default)]
    pub values_found: Vec<Vec<u8>>,
}

impl AsTheMarkerCarriesIt {
    /// The outcome as the rules half states it, so a reader of the stream and a
    /// reader of the record are reading one shape.
    pub fn outcome(&self) -> PairOutcome {
        if self.applied {
            PairOutcome::Applied {
                data_blocks: self.data_blocks,
                assignment_alone: self.assignment_alone,
                reloaded: self.reloaded,
            }
        } else {
            PairOutcome::NotApplied
        }
    }
}

/// **M8: what the remap did, read back off the render's own captured stream
/// with nobody connected.**
///
/// **A line that will not read back is counted rather than dropped**, and a
/// stream carrying no lines returns none — which is not the same as a material
/// that applied nothing, and no caller may read it as one. What says the
/// material ran at all is the marker's own head, which is
/// `Job files and outputs` D16's and not this module's.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct WhatTheMarkerCarried {
    pub lines: Vec<AsTheMarkerCarriesIt>,
    /// **A line carrying the marker's head whose body would not read back.**
    ///
    /// It used to be skipped in silence, which made it indistinguishable from
    /// a pair the material never reported — and those are opposite facts. One
    /// says the material spoke and the reader could not follow; the other says
    /// the material said nothing.
    pub lines_that_would_not_read_back: usize,
}

pub fn what_the_marker_carried(stream: &[u8]) -> WhatTheMarkerCarried {
    let mut out = WhatTheMarkerCarried::default();
    for line in stream.split(|byte| *byte == b'\n') {
        let Some(at) = find(line, THE_PER_PAIR_HEAD.as_bytes()) else {
            continue;
        };
        let body = &line[at + THE_PER_PAIR_HEAD.len()..];
        match serde_json::from_slice::<AsTheMarkerCarriesIt>(body) {
            Ok(one) => out.lines.push(one),
            Err(_) => out.lines_that_would_not_read_back += 1,
        }
    }
    out
}

/// **M8 held end to end: the pairs the material was given, against the lines
/// the marker carried.**
///
/// M8 says the material *"reports every pair in the marker, and fails nothing
/// quietly"*, and `Job files and outputs` D16a says the same. **Nothing
/// compared the two.** The reader above existed and had no caller anywhere, so
/// *every pair is reported* was a sentence rather than a thing that had been
/// checked — and a pair that produced no line at all looked exactly like a
/// stream nobody had read.
///
/// **This is the comparison, and its caller is whoever holds the render's own
/// captured stream** — the marker rides `Job files and outputs` D16 rather than
/// a channel of this module's, so the stream is `Rendering`'s and this function
/// is what it asks.
///
/// **What it does not do.** It judges nothing and fails nothing: it says what
/// the marker said and what it did not. Whether a pair with no line is a
/// failed render is `Rendering`'s ending to decide, and M9d's *not
/// established* stands on every applied pair whatever this returns.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhatTheMarkerSaidAboutThePairs {
    /// Every pair the material carried, in the material's own order, with the
    /// line the marker carried for it — or `None` where there was none.
    pub per_pair: Vec<(Pair, Option<AsTheMarkerCarriesIt>)>,
    /// **Pairs the marker never mentioned.** M8's promise is that this is
    /// empty; it is counted rather than assumed.
    pub pairs_the_marker_never_mentioned: usize,
    /// Lines carrying the marker's head that named no pair of this material.
    pub lines_naming_no_pair_of_this_material: usize,
    /// Lines carrying the marker's head whose body would not read back.
    pub lines_that_would_not_read_back: usize,
}

impl WhatTheMarkerSaidAboutThePairs {
    /// **Whether M8's promise was kept for this run**: every pair reported,
    /// nothing unreadable, and no line about a pair this material never
    /// carried.
    pub fn every_pair_was_reported(&self) -> bool {
        self.pairs_the_marker_never_mentioned == 0
            && self.lines_that_would_not_read_back == 0
            && self.lines_naming_no_pair_of_this_material == 0
    }
}

pub fn what_the_marker_said_about_the_pairs(
    pairs: &[Pair],
    stream: &[u8],
) -> WhatTheMarkerSaidAboutThePairs {
    let carried = what_the_marker_carried(stream);
    // **A pair is named by its kind, its setting and its match text**, because
    // the first two do not name one: two images are two pairs with the same
    // kind and the same setting.
    let mut unclaimed: Vec<Option<AsTheMarkerCarriesIt>> =
        carried.lines.into_iter().map(Some).collect();
    let mut per_pair = Vec::new();
    let mut never_mentioned = 0;
    for pair in pairs {
        let found = unclaimed.iter_mut().find(|line| {
            line.as_ref().is_some_and(|one| {
                one.kind == the_kinds_name(pair.kind)
                    && one.setting == pair.setting
                    && one.match_text == pair.match_text
            })
        });
        match found {
            Some(slot) => per_pair.push((pair.clone(), slot.take())),
            None => {
                never_mentioned += 1;
                per_pair.push((pair.clone(), None));
            }
        }
    }
    WhatTheMarkerSaidAboutThePairs {
        per_pair,
        pairs_the_marker_never_mentioned: never_mentioned,
        lines_naming_no_pair_of_this_material: unclaimed.iter().flatten().count(),
        lines_that_would_not_read_back: carried.lines_that_would_not_read_back,
    }
}

/// A kind's name as the fragment writes it, which is the enum's own variant
/// name and is how the two halves agree on one spelling.
fn the_kinds_name(kind: Kind) -> String {
    serde_json::to_string(&kind)
        .expect("a kind renders")
        .trim_matches('"')
        .to_string()
}

fn find(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    if needle.len() > haystack.len() {
        return None;
    }
    haystack
        .windows(needle.len())
        .position(|window| window == needle)
}

/// **The fragment the launch material carries, composed on the machine from
/// M10's document.**
///
/// **Every value the fragment needs travels as data rather than as text spliced
/// into code**: the pairs go in as one JSON document inside a raw string, and
/// nothing about a path, a name or a location is ever written into a Python
/// expression. A location that is not text would break a spliced literal and
/// would break it silently.
pub fn the_material_python(material: &RemapMaterial) -> String {
    let pairs = serde_json::to_string(&material.pairs)
        .expect("the pairs are composed of values that always render");
    // The pairs are JSON, so the one thing that could end the raw string is a
    // run of quotes the composer chose. Three single quotes cannot appear in
    // JSON output, whose only string delimiter is the double quote.
    debug_assert!(!pairs.contains("'''"));
    format!(
        "{}\n{}\nPAIRS = json.loads(r'''{}''')\n{}",
        THE_HEAD,
        super::reading_script::THE_WALK_OVER_THE_NESTED_MARKS,
        pairs,
        THE_BODY
    )
}

/// The part of the fragment that is the same for every job, and the same
/// for the fixing — `smoozy_dependencies::acting::fixing_worker` shares it,
/// so **the matching, the per-setting route and the reporting are one route
/// rather than two**.
pub const THE_HEAD: &str = r#"
import bpy, json, os, sys

# **`Job files and outputs` D18: no failure handling of its own.** There is no
# `try` below. A kind with no collection, a setting that will not take a value
# and a library that will not reload each raise, and the raise is what
# `Rendering` L8a's failure-to-exit-status argument turns into a failed attempt
# instead of a paid wrong render.

SMOOZY_REMAP_HEAD = "SMOOZY-REMAP-PAIR "

# The collection each kind's data-blocks live in. The kinds are
# `crate::check::Kind`'s own names. **A kind absent from this table is not
# thereby unreachable**: every other kind G6 lists that a route is established
# for sits on a node, a modifier, a strip, a surface or a cache layer rather
# than in a collection, and `SMOOZY_NESTED_KINDS` reaches those by re-walking
# the very generator the check's reading walked. What is absent from **both** is
# a brush icon, a packed file's record and a weak reference, each a named
# absence stated at `THE_WALK_OVER_THE_NESTED_MARKS`, and a path in a custom
# property, which is G7's.
SMOOZY_COLLECTIONS = {
    "Library": "libraries",
    "Image": "images",
    "Sound": "sounds",
    "Font": "fonts",
    "MovieClip": "movieclips",
    "Volume": "volumes",
    "CacheArchive": "cache_files",
    "Text": "texts",
}

def smoozy_match_text(value):
    # M4a: every backslash becomes a forward slash and **nothing else is
    # changed** — the leading `//` stays, a doubled separator stays doubled, a
    # `..` step stays, letter case stays, a trailing space stays, and no Unicode
    # form is normalised. Compared byte for byte.
    return os.fsencode(value).replace(b"\\", b"/")

def smoozy_report(pair, applied, data_blocks, reloaded, values_found):
    line = {
        "kind": pair["kind"],
        "setting": pair["setting"],
        # **Which pair this line is about.** A kind and a setting do not name
        # one — a file with two images has two pairs, both `Image` and both
        # `filepath` — so without this the reader cannot tell a missing line
        # from a present one and M8's *every pair is reported* cannot be
        # checked at all.
        "match_text": pair["match_text"],
        "applied": applied,
        "data_blocks": data_blocks,
        # M9d: composed from the pair's kind alone, so a kind nobody has posed
        # reads *not established* by construction rather than by omission.
        "assignment_alone": "Established" if pair["kind"] in ("Image", "Library") else "NotEstablished",
        "reloaded": reloaded,
        "values_found": values_found,
    }
    # **M8 and `Job files and outputs` D16a: the line is written and pushed out
    # at once.** An ordinary raise flushes what is buffered on the way out, so
    # the loss window is a hard kill or a power-off — **which is the ordinary
    # occasion here rather than an exotic one**: a provider's forced hardware
    # maintenance is recorded as exactly that. And block buffering does a second
    # thing even when nothing is lost: it lands these lines at the end of the
    # retained stream instead of in place, against D16's *printed on the
    # render's own ordinary output as the material's first act*.
    sys.stdout.write(SMOOZY_REMAP_HEAD + json.dumps(line) + "\n")
    sys.stdout.flush()

def smoozy_every_tree():
    seen = set()
    frontier = []
    for tree in bpy.data.node_groups:
        frontier.append(tree)
    for collection in ("materials", "worlds", "lights", "textures", "linestyles"):
        holder = getattr(bpy.data, collection, None)
        if holder is None:
            continue
        for owner in holder:
            tree = getattr(owner, "node_tree", None)
            if tree is not None:
                frontier.append(tree)
    for scene in bpy.data.scenes:
        # From 5.3 every effect's group, and the older one-group name is never
        # read: it answers only the active effect's, and warns.
        # `Job files and outputs` R4 item 1. Before 5.3 a scene has no such stack.
        effects = getattr(scene, "compositor_effects", None)
        if effects is None:
            trees = [getattr(scene, "compositing_node_group", None)]
        else:
            trees = [effect.node_group for effect in effects]
        for tree in trees:
            if tree is not None:
                frontier.append(tree)
    for obj in bpy.data.objects:
        for modifier in getattr(obj, "modifiers", ()):
            tree = getattr(modifier, "node_group", None)
            if tree is not None:
                frontier.append(tree)
    while frontier:
        tree = frontier.pop()
        key = id(tree)
        if key in seen:
            continue
        seen.add(key)
        yield tree
        for node in getattr(tree, "nodes", ()):
            inner = getattr(node, "node_tree", None)
            if inner is not None:
                frontier.append(inner)
"#;

/// The part of the fragment that carries the pairs out.
const THE_BODY: &str = r#"
for pair in PAIRS:
    wanted = bytes(pair["match_text"])
    new_location = os.fsdecode(bytes(pair["new_location"]))
    kind = pair["kind"]
    setting = pair["setting"]
    reached = 0
    found = []
    reloaded = False

    if kind == "ImportNodePath":
        # A node is this kind's data-block. **The same walk the reading took**,
        # so a pair reaches exactly what the check saw.
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
        # **The same walk the reading took**, re-run so a pair reaches exactly
        # what the check saw. The holder is a node, a modifier, a strip, a
        # surface or a cache layer rather than a data-block, so there is no
        # collection to take it from — and **M5's rule is kept whole**: one
        # setting on one object at a time, no bulk call.
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
        # **M5: the per-setting route.** One collection, one setting, one
        # data-block at a time, and no bulk call anywhere.
        holder = getattr(bpy.data, SMOOZY_COLLECTIONS[kind])
        for datablock in holder:
            current = getattr(datablock, setting)
            if smoozy_match_text(current) == wanted:
                reached = reached + 1
                # **M8b: a library the file already loaded from the new location
                # is left exactly as it is.** The opening folder put it there. A
                # reload now would throw away what the file's own start scripts
                # set on its data, and a new location would part it from the
                # recorded text a script may name it by.
                if kind == "Library" and os.path.realpath(bpy.path.abspath(current)) == os.path.realpath(new_location):
                    continue
                setattr(datablock, setting, new_location)
                # **M8 and Fact 101: for a library the assignment is followed by
                # the reload**, and it works on a library that never loaded.
                # **Fact 99: for an image the material assigns and does not
                # reload.** The relocate command is not used and is not offered
                # as a fallback.
                if kind == "Library":
                    datablock.reload()
                    reloaded = True
            else:
                found.append(list(os.fsencode(current)))

    # **M8: every pair is reported and nothing fails quietly.** An applied pair
    # says how many data-blocks it reached; a pair that reached none is *not
    # applied* with the values that were found, which is a countable fact rather
    # than an unidentified miss.
    smoozy_report(pair, reached > 0, reached, reloaded, [] if reached > 0 else found)
"#;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bytes::{MachineLocation, RecordedLocation};
    use crate::check::Kind;
    use crate::remap::{compose_pair, NewLocation};
    use smoozy_render_settings_machine::file_half::BlenderIdentity;
    use smoozy_server_setup_machine::blender::Version;
    use smoozy_state::identities::{Identity, IdentityKind};

    fn a_material() -> RemapMaterial {
        let job = Identity::issue(IdentityKind::Job).expect("randomness");
        let with = BlenderIdentity {
            path: "/opt/blender-4.5.9/blender".to_string(),
            version: Version {
                major: 4,
                minor: 5,
                patch: 9,
            },
        };
        let image = compose_pair(
            Kind::Image,
            "filepath",
            &RecordedLocation::as_the_reading_carried_it(b"//tex\\METallic.png"),
            NewLocation::AMirror(MachineLocation::composed(
                b"/root/assets/files/Users/a/tex/METallic.png".to_vec(),
            )),
        )
        .expect("the pair composes");
        let library = compose_pair(
            Kind::Library,
            "filepath",
            &RecordedLocation::as_the_reading_carried_it(b"//lib/rig.blend"),
            NewLocation::AFixedCopy(MachineLocation::composed(
                b"/root/assets/fixed/assets/files/Users/a/lib/rig.blend".to_vec(),
            )),
        )
        .expect("the pair composes");
        RemapMaterial::composed(&job, &with, vec![image, library])
    }

    /// **`Job files and outputs` D18, held rather than remembered: the fragment
    /// catches nothing.**
    ///
    /// *"A failure raised inside a script that catches it leaves a successful
    /// exit status behind"*, and D17 records a run that printed a permission
    /// refusal, rendered the factory scene anyway and exited 0. **This is the
    /// one test that stands between that and a paid wrong render**, so it reads
    /// the fragment's own text rather than trusting a habit.
    #[test]
    fn the_fragment_catches_nothing_and_swallows_nothing() {
        let text = the_material_python(&a_material());
        for line in text.lines() {
            let code = line.trim_start();
            if code.starts_with('#') {
                continue;
            }
            assert!(!code.starts_with("try"), "{code}");
            assert!(!code.starts_with("except"), "{code}");
            assert!(!code.contains("contextlib.suppress"), "{code}");
        }
    }

    /// **M5: the per-setting route and no other.** No bulk sweep and no
    /// one-pass call is named anywhere in the fragment — the route that is
    /// silently partial for the redirect gets no second chance here.
    ///
    /// **M21 is held by the same list, at the remap end.** Blender's own
    /// search and repair stand behind nothing here, and the two names that
    /// would reach for them — its missing-file report and its find-missing
    /// route — are forbidden by name below, along with both forms of the
    /// relocate command. **This is the one place the fragment could have
    /// quietly acquired Blender's own repair**, so the forbidding is written
    /// as a test rather than left as a habit.
    #[test]
    fn no_bulk_route_and_no_one_pass_route_is_named() {
        let whole = the_material_python(&a_material());
        // A comment may name a route to say why it is not taken; a line of code
        // may not.
        let text: String = whole
            .lines()
            .filter(|line| !line.trim_start().starts_with('#'))
            .collect::<Vec<&str>>()
            .join("\n");
        for forbidden in [
            "make_paths_absolute",
            "make_paths_relative",
            "file_path_foreach",
            "report_missing_files",
            "find_missing_files",
            "lib_relocate",
            "relocate",
        ] {
            assert!(!text.contains(forbidden), "{forbidden} is named");
        }
        const { assert!(crate::remap::EVERY_PAIR_IS_APPLIED_BY_THE_PER_SETTING_ROUTE) };
    }

    /// **M8 and `Job files and outputs` D16a: a pair's line is pushed out as it
    /// is written, not when the process happens to end.**
    ///
    /// **The loss window is not exotic here.** An ordinary raise flushes what
    /// is buffered on the way out, so what a buffered line is lost to is a hard
    /// kill or a power-off — and `App/Job files and outputs`'s own reckoning
    /// records a provider's forced hardware maintenance as *the ordinary
    /// occasion rather than a corner case*. **A lost line is worse than a
    /// missing render**: the pairs were applied, the render was paid for, and
    /// the record of what the remap did is gone.
    ///
    /// **Block buffering costs something even when nothing is lost**: it lands
    /// these lines at the end of the retained stream instead of in place,
    /// against D16's *printed on the render's own ordinary output as the
    /// material's first act*.
    ///
    /// The reading's own script has ended in a flush since it was written; this
    /// is the same discipline at the other end.
    #[test]
    fn every_pairs_line_is_flushed_as_it_is_written() {
        let text = the_material_python(&a_material());
        assert!(text.contains("sys.stdout.write(SMOOZY_REMAP_HEAD"));
        assert!(text.contains("sys.stdout.flush()"));
        // The unflushed form is gone rather than merely joined by a flush.
        assert!(!text.contains("print(SMOOZY_REMAP_HEAD"));
        // The fixing shares the head, so it is flushed by the same edit; that
        // half is held beside the fixing, in `smoozy-dependencies`' own
        // `acting::fixing_worker`, which this crate cannot reach.
    }

    /// **M9a: the flag is not read, and the fragment is where a reader would
    /// most plausibly reach for it.**
    #[test]
    fn the_fragment_never_reads_the_flag_m9a_names() {
        let text = the_material_python(&a_material());
        assert!(!text.contains(concat!("has", "_data")));
    }

    /// **M8 and Fact 101: a library is assigned and reloaded; Fact 99: an image
    /// is assigned and not reloaded.** The per-kind difference is in the
    /// fragment, and it is the rules half's answer rather than a second one.
    #[test]
    fn a_library_is_reloaded_after_its_assignment_and_an_image_is_not() {
        let text = the_material_python(&a_material());
        assert!(text.contains("datablock.reload()"));
        assert!(text.contains("if kind == \"Library\":"));
        assert!(crate::remap::takes_a_reload_after_assignment(Kind::Library));
        assert!(!crate::remap::takes_a_reload_after_assignment(Kind::Image));
    }

    /// **M8b: a library the file already loaded from the new location is left
    /// alone, and the test comes before the assignment**, because the assignment is
    /// what would part it from the recorded text.
    #[test]
    fn a_library_already_in_place_is_neither_assigned_nor_reloaded() {
        let text = the_material_python(&a_material());
        let the_test = text
            .find("os.path.realpath(bpy.path.abspath(current)) == os.path.realpath(new_location)")
            .expect("the test is there");
        let the_assignment = text.find("setattr(datablock, setting, new_location)").expect("the assignment");
        let the_reload = text.find("datablock.reload()").expect("the reload");
        assert!(the_test < the_assignment && the_assignment < the_reload);
    }

    /// **M4a: the pair's match text is composed at composition and travels with
    /// it**, and the fragment applies the same one rule to the value it finds.
    /// A file checked on Windows and rendered on Linux reads the same recorded
    /// place differently, and matched on the exact bytes every pair of such a
    /// file would miss at once.
    #[test]
    fn the_match_text_is_the_separator_rule_and_nothing_else() {
        let material = a_material();
        assert_eq!(
            material.pairs[0].match_text.as_bytes(),
            b"//tex/METallic.png",
            "the backslash became a forward slash and nothing else moved"
        );
        assert_eq!(
            material.pairs[0].expected.as_bytes(),
            b"//tex\\METallic.png",
            "the expected bytes are exactly what the check read"
        );
        let text = the_material_python(&material);
        assert!(text.contains(r#"replace(b"\\", b"/")"#));
    }

    /// **M8: every pair is reported, and a reader takes the outcomes back off
    /// the captured stream with nobody connected.**
    #[test]
    fn every_pairs_outcome_reads_back_off_a_captured_stream() {
        let applied = AsTheMarkerCarriesIt {
            kind: "Library".to_string(),
            setting: "filepath".to_string(),
            match_text: MatchText::of(b"//lib/rig.blend"),
            applied: true,
            data_blocks: 2,
            assignment_alone: AssignmentAlone::Established,
            reloaded: true,
            values_found: Vec::new(),
        };
        let missed = AsTheMarkerCarriesIt {
            kind: "Volume".to_string(),
            setting: "filepath".to_string(),
            match_text: MatchText::of(b"//vol/one.vdb"),
            applied: false,
            data_blocks: 0,
            assignment_alone: AssignmentAlone::NotEstablished,
            reloaded: false,
            values_found: vec![b"//vol/other.vdb".to_vec()],
        };
        let stream = format!(
            "Blender 4.5.9\nSMOOZY-REMAP-PAIR {}\nFra:1 Mem:12M\nSMOOZY-REMAP-PAIR {}\nsaved\n",
            serde_json::to_string(&applied).expect("renders"),
            serde_json::to_string(&missed).expect("renders"),
        );
        let back = what_the_marker_carried(stream.as_bytes());
        assert_eq!(back.lines, vec![applied.clone(), missed.clone()]);
        assert_eq!(back.lines_that_would_not_read_back, 0);
        assert_eq!(
            back.lines[0].outcome(),
            PairOutcome::Applied {
                data_blocks: 2,
                assignment_alone: AssignmentAlone::Established,
                reloaded: true,
            }
        );
        assert_eq!(back.lines[1].outcome(), PairOutcome::NotApplied);
    }

    /// **A stream with no lines carries no outcomes, and that is not the same
    /// as a material that applied nothing.** Nothing here turns a missing
    /// answer into an empty one — **and a line that will not read back is now
    /// counted rather than dropped**, because *the material spoke and the
    /// reader could not follow* and *the material said nothing* are opposite
    /// facts that used to look identical.
    #[test]
    fn a_stream_with_no_lines_carries_no_outcomes_and_a_broken_line_says_so() {
        let none = what_the_marker_carried(b"Fra:1 Mem:12M\nsaved\n");
        assert!(none.lines.is_empty());
        assert_eq!(none.lines_that_would_not_read_back, 0);

        let broken = what_the_marker_carried(b"SMOOZY-REMAP-PAIR not a document\n");
        assert!(broken.lines.is_empty());
        assert_eq!(broken.lines_that_would_not_read_back, 1);
    }

    /// **M8 end to end, and the comparison that had never been made.**
    ///
    /// *"Reports every pair in the marker, and fails nothing quietly."* The
    /// reader existed and had no caller anywhere in `App/`, so the promise was
    /// a sentence. This holds a material's own pairs against the stream a run
    /// left behind.
    #[test]
    fn every_pair_the_material_carried_is_found_in_the_marker_or_counted_as_missing() {
        let material = a_material();
        let lines: Vec<String> = material
            .pairs
            .iter()
            .map(|pair| {
                serde_json::to_string(&AsTheMarkerCarriesIt {
                    kind: serde_json::to_string(&pair.kind)
                        .expect("renders")
                        .trim_matches('"')
                        .to_string(),
                    setting: pair.setting.clone(),
                    match_text: pair.match_text.clone(),
                    applied: true,
                    data_blocks: 1,
                    assignment_alone: crate::remap::assignment_alone_is(pair.kind),
                    reloaded: crate::remap::takes_a_reload_after_assignment(pair.kind),
                    values_found: Vec::new(),
                })
                .expect("renders")
            })
            .collect();

        let whole = format!(
            "Fra:1\nSMOOZY-REMAP-PAIR {}\nSMOOZY-REMAP-PAIR {}\nsaved\n",
            lines[0], lines[1]
        );
        let said = what_the_marker_said_about_the_pairs(&material.pairs, whole.as_bytes());
        assert!(said.every_pair_was_reported());
        assert_eq!(said.per_pair.len(), 2);
        assert!(said.per_pair.iter().all(|(_, line)| line.is_some()));

        // One pair's line missing, and it is the pair that is named rather than
        // a count of two against one.
        let short = format!("Fra:1\nSMOOZY-REMAP-PAIR {}\nsaved\n", lines[0]);
        let said = what_the_marker_said_about_the_pairs(&material.pairs, short.as_bytes());
        assert!(!said.every_pair_was_reported());
        assert_eq!(said.pairs_the_marker_never_mentioned, 1);
        assert!(said.per_pair[1].1.is_none());
        assert_eq!(said.per_pair[1].0.kind, Kind::Library);

        // A line the reader cannot follow is not a pair that went unreported.
        let mangled = format!(
            "SMOOZY-REMAP-PAIR {}\nSMOOZY-REMAP-PAIR nonsense\n",
            lines[0]
        );
        let said = what_the_marker_said_about_the_pairs(&material.pairs, mangled.as_bytes());
        assert_eq!(said.lines_that_would_not_read_back, 1);
        assert_eq!(said.pairs_the_marker_never_mentioned, 1);
        assert!(!said.every_pair_was_reported());
    }

    /// **Two pairs of the same kind and setting are two pairs**, and the kind
    /// and the setting alone never named one.
    ///
    /// This is why the line carries the match text: a file with two images has
    /// two `Image`/`filepath` lines, and without the match text a marker that
    /// reported one of them twice and the other never read as complete.
    #[test]
    fn two_pairs_of_one_kind_are_told_apart_and_a_repeated_line_does_not_cover_a_missing_one() {
        let first = compose_pair(
            Kind::Image,
            "filepath",
            &RecordedLocation::as_the_reading_carried_it(b"//tex/one.png"),
            NewLocation::AMirror(MachineLocation::composed(b"/root/one.png".to_vec())),
        )
        .expect("the pair composes");
        let second = compose_pair(
            Kind::Image,
            "filepath",
            &RecordedLocation::as_the_reading_carried_it(b"//tex/two.png"),
            NewLocation::AMirror(MachineLocation::composed(b"/root/two.png".to_vec())),
        )
        .expect("the pair composes");
        let one_line = serde_json::to_string(&AsTheMarkerCarriesIt {
            kind: "Image".to_string(),
            setting: "filepath".to_string(),
            match_text: first.match_text.clone(),
            applied: true,
            data_blocks: 1,
            assignment_alone: AssignmentAlone::Established,
            reloaded: false,
            values_found: Vec::new(),
        })
        .expect("renders");
        let stream = format!("SMOOZY-REMAP-PAIR {one_line}\nSMOOZY-REMAP-PAIR {one_line}\n");
        let said = what_the_marker_said_about_the_pairs(&[first, second], stream.as_bytes());
        assert_eq!(said.pairs_the_marker_never_mentioned, 1);
        assert_eq!(said.lines_naming_no_pair_of_this_material, 1);
        assert!(!said.every_pair_was_reported());
    }

    /// **Nothing about a path is ever written into a Python expression.** The
    /// pairs travel as one JSON document, so a location that is not text
    /// survives whole rather than breaking a spliced literal silently.
    #[test]
    fn the_pairs_travel_as_data_and_never_as_code() {
        let job = Identity::issue(IdentityKind::Job).expect("randomness");
        let with = BlenderIdentity {
            path: "/opt/blender/blender".to_string(),
            version: Version {
                major: 5,
                minor: 2,
                patch: 0,
            },
        };
        // A recorded location holding a quote, a newline and a byte that is not
        // text — every one of which would end a spliced literal.
        let nasty = [
            b'/', b'a', b'"', b'\n', 0xff, b'/', b'x', b'.', b'p', b'n', b'g',
        ];
        let pair = compose_pair(
            Kind::Image,
            "filepath",
            &RecordedLocation::as_the_reading_carried_it(&nasty),
            NewLocation::AMirror(MachineLocation::composed(b"/root/x.png".to_vec())),
        )
        .expect("the pair composes");
        let text = the_material_python(&RemapMaterial::composed(&job, &with, vec![pair]));
        assert!(!text.contains("\u{fffd}"));
        // The value is inside the JSON document as byte numbers and nowhere in
        // the code above or below it.
        assert!(text.contains("255"));
        assert!(!text.contains("/a\"\n"));
    }
}

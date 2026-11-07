//! **§3.2 and §3.5's own script — G8 and G19 — the thing that runs inside the
//! Blender the reading drives.**
//!
//! `smoozy-dependencies`' own list of the acting half (its `arriving_later`) names this act: *"the walk over Blender's own path
//! marks joined with the map and the flat list, the node-tree walk into every
//! node group for the import nodes, and the walk over the record of what uses
//! what for scene attribution."* What it returns is
//! `smoozy-dependencies`' `check::ReadingDocument`; the routes it must take are
//! that crate's `check::THE_MAP` and `check::THE_FLAT_LIST`; the rule for
//! what attribution means is G20a's.
//!
//! # Every Blender name below, with the publication it was read from
//!
//! **Rule 2 binds here hardest, because a name written from memory is a reading
//! that fails on every file.** Every name is quoted from Blender's own
//! published Python API reference and user manual, bundled with this project's
//! Blender tooling, **read on 2026-09-04**. The path after each quote is the
//! page inside that bundle.
//!
//! - **`bpy.data.file_path_map(*, subset=None, key_types=None,
//!   include_libraries=False)`** — *"Returns a mapping of all ID data-blocks in
//!   current `bpy.data` to a set of all file paths used by them."*, returning
//!   *"dictionary of `bpy.types.ID` instances, with sets of file path strings as
//!   their values"*, with *"include_libraries: Include library file paths of
//!   linked data. False by default."* (`api/bpy.types.BlendData.rst`). **This is
//!   G6's map, and the flag is turned on**: G6's own reason for the map is that
//!   *"where a linked library points at a picture of its own, the map reaches
//!   through the library and the flat list does not"*, and a library's own
//!   recorded location is a reference the render must resolve like any other.
//! - **`bpy.utils.blend_paths(*, absolute=False, packed=False, local=False)`** —
//!   *"Returns a list of paths to external files referenced by the loaded .blend
//!   file."* (`api/bpy.utils.rst`). **This is G6's flat list, called with its
//!   own defaults**, which is the form G27's observation was taken in.
//! - **`bpy.types.ID.library_weak_reference`** — *"Weak reference to a
//!   data-block in another library .blend file (used to re-use already appended
//!   data instead of appending new copies) (readonly)"*, of type
//!   `LibraryWeakReference`, whose `filepath` is *"Path to the library .blend
//!   file"* and whose `id_name` is *"Full ID name in the library .blend file
//!   (including the two leading 'id type' chars)"*
//!   (`api/bpy.types.ID.rst`, `api/bpy.types.LibraryWeakReference.rst`). **This
//!   is G27's mark.**
//! - **`bpy.data.user_map(*, subset=None, key_types=None, value_types=None)`** —
//!   *"Returns a mapping of all ID data-blocks in current `bpy.data` to a set of
//!   all data-blocks using them."*, returning *"dictionary that maps data-blocks
//!   ID's to their users"* (`api/bpy.types.BlendData.rst`). **This is G19's
//!   record of what uses what**, and the walk outwards over it is G19's walk.
//! - **The eight data-block collections that carry a recorded location.** The
//!   reference *"carries no classification of which collections hold
//!   external-path data-blocks"*, so the set below is derived from which types'
//!   own pages carry a `filepath`: `Library.filepath` *"Path to the library
//!   .blend file"*, `Image.filepath` *"Image/Movie file name"*,
//!   `Sound.filepath` *"Sound sample file used by this Sound data-block"*,
//!   `VectorFont.filepath`, `MovieClip.filepath` *"Filename of the movie or
//!   sequence file"*, `Volume.filepath` *"Volume file used by this Volume
//!   data-block"*, `CacheFile.filepath` *"Path to external displacements
//!   file"*, `Text.filepath` *"Filename of the text file"*
//!   (`api/bpy.types.<Type>.rst`).
//! - **`ID.is_missing`** — *"This data-block is a place-holder for missing
//!   linked data (i.e. it is [an override of] a linked data that could not be
//!   found anymore)"* (`api/bpy.types.ID.rst`).
//! - **The node trees a file holds, each by the property whose type is a node
//!   tree**: `bpy.data.node_groups` *"Node group data-blocks"*,
//!   `Material.node_tree` *"Node tree for node based materials"*,
//!   `World.node_tree`, `Light.node_tree`, `Texture.node_tree`,
//!   `FreestyleLineStyle.node_tree`, `Scene.compositing_node_group`
//!   *"Compositor Nodes"*, `NodesModifier.node_group` *"Node group that controls
//!   what this modifier does"*, and a group node's own `node_tree`
//!   (`api/bpy.types.NodeTree.rst`'s own References list, and each type's page).
//!   **The reference carries no recipe for enumerating every tree in a file**,
//!   so the enumeration below is that list walked and nothing wider.
//! - **The import nodes** — `GeometryNodeImportSTL` *"Import a mesh from an STL
//!   file"*, `GeometryNodeImportOBJ`, `GeometryNodeImportPLY`,
//!   `GeometryNodeImportCSV` *"Import geometry from an CSV file"*,
//!   `GeometryNodeImportText`, `GeometryNodeImportVDB` *"Import volume data from
//!   a .vdb file"* (`api/bpy.types.GeometryNode*.rst`). **That is G8's six, and
//!   the reference carries no seventh.** **The path is not a property**: the
//!   API reference carries none on any of those classes, and the manual
//!   documents it as an input socket named `Path` — *"Path / The path to the
//!   .stl file to be imported."* and one such line per node
//!   (`manual/modeling/geometry_nodes/input/import/*.rst`).
//!
//! # G6's two tests: what is carried inside the file, and a setting that records nothing
//!
//! **Added 2026-09-29, dependency wave piece 1, out of the splash scenes at the
//! desk**, where every packed picture came back a *not found* row and Render
//! Result, a Viewer node, a generated grid and every text written inside
//! Blender came back as one row with an empty location that nothing could ever
//! find. G6 now says *"What is carried inside the file is not a reference, and
//! neither is a setting that records nothing"*, and that *"the property that
//! says so is quoted at the site from Blender's own published reference, for
//! each version, and a version that carries no such property produces the
//! reference as before."* These are those quotations. **The words are the
//! bundled reference's, read 2026-09-29** (it is 5.1's: its index page reads
//! *"Blender 5.1 Python API Documentation"*); **which versions carry each
//! name was read the same day off each version's own published reference in
//! `Blender Sources/api_<version>/`**, 4.5, 5.0, 5.1 and 5.2.
//!
//! - **`Image.packed_file`** — *"First packed file of the image (readonly)"*,
//!   of type `PackedFile | None`; **`Sound.packed_file`**,
//!   **`VectorFont.packed_file`**, **`Volume.packed_file`** and
//!   **`Library.packed_file`** — each *"(readonly)"*, of the same type
//!   (`api/bpy.types.<Type>.rst`). **In all four versions.** A data-block
//!   holding one is carried inside the `.blend` and produces no reference.
//!   **`MovieClip`, `CacheFile` and `Text` carry no `packed_file` in any of
//!   the four**, so a reference is produced for them as before — the clause's
//!   own rule for a version without the property, met here by a type.
//! - **`Library.is_archive`** — *"This library is an 'archive' storage for
//!   packed linked IDs originally linked from its 'archive parent' library.
//!   (default False, readonly)"* (`api/bpy.types.Library.rst`). **This is
//!   G6's *"from 5.0 the library that holds packed linked data"*.** Present in
//!   5.0, 5.1 and 5.2; **4.5's reference carries no such property**, so in 4.5
//!   the reference is produced as before, which is the clause's own rule.
//! - **`NodesModifier.bake_target`** — *"Where to store the baked data
//!   (default `'PACKED'`)"*, whose values are *"`PACKED` Packed -- Pack the
//!   baked data into the .blend file."* and *"`DISK` Disk -- Store the baked
//!   data in a directory on disk."* (`api/bpy.types.NodesModifier.rst`); and
//!   **`NodesModifierBake.bake_target`** — the same sentence with *"(default
//!   `'INHERIT'`)"* and a third value, *"`INHERIT` Inherit from Modifier --
//!   Use setting from the modifier."* (`api/bpy.types.NodesModifierBake.rst`).
//!   **In all four versions.** G6's *"a geometry-node bake whose bake target
//!   keeps its data inside the file"*.
//! - **`bpy_struct.as_pointer()`** — *"Returns the memory address which holds
//!   a pointer to Blender's internal data"* (`api/bpy.types.bpy_struct.rst`).
//!   **In all four versions.** It is how a bake's own `INHERIT` is read
//!   through to the modifier that holds it: the shared walk hands the check a
//!   bake and not its modifier, and the walk itself is not changed to say so
//!   (below).
//! - **`bpy.utils.blend_paths`'s `packed` parameter, and the four references
//!   do not agree about it.** 5.1's and 5.2's read *"packed: When true include
//!   file paths for packed data."*; **4.5's reads *"packed (bool) – When true
//!   skip file paths for packed data."*** for the same default of `False`.
//!   **Nothing here turns on which is right**, because the flat list produces
//!   no reference: it is read only for G27's exclusion. It is held to the
//!   empty-location test like the map, and what the 4.5 reading actually
//!   returned for a file of seventy packed pictures is recorded in piece 1's
//!   account.
//!
//! **Where the bake test sits, and why not in the shared walk.** The test is
//! in this script's own loop over `smoozy_nested_marks()` and never inside
//! [`crate::reading_script::THE_WALK_OVER_THE_NESTED_MARKS`]: the remap and the fixing walk the same
//! generator to assign, and a mark the generator stopped yielding would change
//! what they walk. **So the modifier-level folder and each bake's own folder
//! are both still yielded, and the check alone decides that a packed one is
//! not a reference.** The modifier-level folder is packed where the modifier
//! says `PACKED` and no bake of it says `DISK`; a bake's own folder is packed
//! where its own target says `PACKED`, or says `INHERIT` and its modifier says
//! `PACKED`. **Anything else, a property missing included, is a reference as
//! before** — the wide side, where a folder found and sent costs bandwidth
//! and a folder missed renders wrong.
//!
//! **A picture's and a movie clip's source is carried with the reference**
//! (G9 with DW0-3): **`Image.source`** — *"Where the image comes from (default
//! `'FILE'`)"*, whose values include *"`FILE` Single Image -- Single image
//! file."*, *"`SEQUENCE` Image Sequence -- Multiple image files, as a
//! sequence."*, *"`MOVIE` Movie -- Movie file."* and *"`TILED` UDIM Tiles --
//! Tiled UDIM image texture."* (`api/bpy.types.Image.rst`); and
//! **`MovieClip.source`** — *"Where the clip comes from (default
//! `'SEQUENCE'`, readonly)"*, with *"`SEQUENCE` Image Sequence -- Multiple
//! image files, as a sequence."* and *"`MOVIE` Movie File -- Movie file."*
//! (`api/bpy.types.MovieClip.rst`). **In all four versions, with the same
//! values.** The script carries the value as Blender stores it and decides
//! nothing; `smoozy-dependencies`' `check::StoredSource` reads it.
//!
//! # Three decisions taken here, each recorded rather than assumed
//!
//! **The import nodes are matched by the prefix of their type name and not by a
//! list.** G8 says *"six kinds of import node in 4.5 to 5.2 and seven in 5.3, the
//! seventh being Import SPZ, plus the sound route in 5.2 and 5.3"*, and **the
//! reference read above carries no sound import node at all**. A fixed list
//! would silently miss the next one, as a list of six missed Import SPZ; the
//! prefix reaches whatever a version actually holds, and
//! what it reaches is **observed from the file** rather than inferred from a
//! version number. It errs wide in the direction G22 already errs in for a
//! set — a kind found and sent costs bandwidth, a kind missed renders wrong.
//!
//! **A socket whose value comes from a link is reported as having no value
//! rather than being guessed at.** An import node's `Path` socket may be driven
//! by another node, and then there is no stored text to read. Nothing invents
//! one; the node contributes no reference, which is G7's own shape for a
//! location no route reaches.
//!
//! # G27a: a linked file the Blender reading it ships. Added 2026-10-07, the Creator's second notes, piece 12
//!
//! **The reading answers one thing and decides nothing**: for each library
//! whose recorded location runs through a `datafiles/assets/` folder, whether
//! this Blender holds a file at the same remainder under its own. It carries
//! those libraries' recorded locations as `shipped_with_this_blender`, and
//! `smoozy-dependencies`' `check` leaves their rows out.
//!
//! - **`bpy.utils.system_resource("DATAFILES")`**, the folder the test looks
//!   under. **Not in the published reference of any of the four versions**
//!   (`Blender Sources/api_<version>/bpy.utils.txt` carries `resource_path`
//!   and `user_resource` and not this), so it is **observed, not documented**:
//!   read off the desk's four Blenders on 2026-10-07, each answered its own
//!   application's `…/Resources/<version>/datafiles`, and the same call with a
//!   folder that is not there answered an empty string, which the test reads
//!   as *not held*. `Job files and outputs`' multi-layer picture reaches
//!   Blender's own colour configuration by the same call, read the same way
//!   on 2026-10-03. **What it answers on Linux is a rental's to show** (the
//!   piece's account names the watch).
//! - **Both separators are read** in the recorded location, because a file
//!   saved on Windows records its own; the remainder may hold no empty, `.` or
//!   `..` step, so it can never reach outside that folder.
//! - **Where the test cannot say, the row stays** — no folder, no file, a
//!   remainder it will not walk. The wide side: a row shown costs the user a
//!   look, and a row dropped could be a missing file of theirs.
//!
//! **`observed_stand_ins` is always empty, and that is the answer rather than an
//! omission.** G6a's statement rests on the reading being able to observe a
//! linked item that *"keeps the name and the library and holds nothing"*, and
//! **§12 item 20 records that this is not established and is permanently
//! unresolved** — the Creator ruled on 2026-08-31 that there will be no further
//! Blender research. G6b states what to do: *"where the reading can see it, the
//! statement appears; where it cannot, no statement appears and the user is
//! exactly where they were before the ruling."* **So the script observes none,
//! and no test of the shape is invented from a fact nobody holds.** The
//! mechanism that would carry a statement is
//! `smoozy-dependencies`' `check::linked_item_statements` and it is unchanged; what it
//! produces is nothing, honestly. **No space is left for an answer, because
//! none is coming.**
//!
//! # What the document carries a location as
//!
//! **A list of byte values, and never text.** G3a makes a recorded location
//! *"bytes and… not required to be text"*, and `Job files and outputs` R4a
//! requires it to cross *"as the bytes the file holds, never decoded,
//! substituted, dropped or made the cause of the reading failing"*. Blender
//! hands a path back as a Python string, so the script puts it back through
//! `os.fsencode` — the reverse of the decoding that produced it — and writes
//! the bytes out one number at a time. That is what
//! [`crate::bytes::RecordedLocation`] already reads back, so **one
//! representation covers the wire and the record**, and there is no second one
//! for a reader to get wrong.
//!
//! # Why it is in this crate. Moved 2026-09-29, step 10c's dependency wave piece 2
//!
//! **`Dependency Upload & Remap` G1 makes the reading *"driven as a
//! file-inspection worker through the supervisor"*, and `Render Settings` Q4
//! says the bare-connection invocation *"is the mechanism's shape, not a route
//! around the admission point"*.** Until this piece the desktop wrote this
//! script to the machine itself, planted a link to the Blender program and ran
//! a bare command naming the link — the route both clauses rule out, and the
//! one that crashed Blender at the desk: started through a link to its program,
//! Blender 5.2.2 on this Mac could not find its own scripts and ended at 139.
//! **So the machine's own program composes the script now**, in the
//! file-inspection worker, and starts Blender by the build's own path; the
//! script had to be in a crate that program may reach, and this is the one its
//! shared walk already lives in. `smoozy-dependencies` re-exports it at the
//! path it has always been reached at, where its tests still hold it.
//!
//! # The launch line, argument by argument, each from Blender's own manual
//!
//! **Read from the Blender manual bundled with this project's Blender tooling,
//! `manual/advanced/command_line/arguments.rst`, on 2026-09-04.** Its usage
//! line is `blender [args ...] [file] [args ...]`, and its *Argument Order*
//! section says *"Arguments are executed in the order they are given."*
//!
//! - `-b, --background` — *"Run in background (often used for UI-less
//!   rendering)."* G1's *headless*.
//! - `--factory-startup` — *"Skip reading the startup.blend in the users home
//!   directory."* **The check's reading is the file's own and not the
//!   machine's preferences'**, and it is the condition every observation this
//!   module rests on was taken under. The file's reading launches the other
//!   way, and says why at its own site (`Job files and outputs` R7).
//! - `--enable-autoexec` — *"Enable automatic Python script execution."*
//!   **Added 2026-10-08, the Creator's ruling**: *"Yes to running scripts when
//!   reading."* The check's reading lets the file's own scripts run, as every
//!   render does (`Rendering` L9, which spells it the same, long form, from
//!   the security page's worked example). It stands after the factory startup
//!   and before the load-time script and the file. Observed at the desk on
//!   2026-10-08, 4.5.12 and 5.1.2: a file whose text block is registered to
//!   run on load ran it with `-b --factory-startup --enable-autoexec` and did
//!   not without the switch.
//! - `--python-exit-code <code>` — *"Set the exit-code in [0..255] to exit if a
//!   Python exception is raised (only for scripts executed from the command
//!   line), zero disables."* A script that raised is a failing status rather
//!   than a silent zero, which is the half of G3 a reader of the status can act
//!   on.
//! - `--python <filepath>` — *"Run the given Python script file."*
//!
//! **The file comes after the startup options and before the script**, the
//! order `Rendering` L8 composes a launch line in. **Nothing follows a bare
//! double dash**, the position where an argument Blender does not know is
//! swallowed without a word.
//!
//! **Markers used in this file that belong to another document, each with the
//! document it belongs to.** `Job files and outputs` R4a, R7. `Rendering` L8.
//!
//! **`Render Settings` Q4 is not listed above and is written out at its site**,
//! because this plan defines a Q4 of its own.

/// **The script** — composed by the machine's own program in the
/// file-inspection worker, written into that operation's own folder, and
/// named on [`the_words_of_the_check_reading`].
///
/// **It carries no value of Smoozy's**: the file it reads is the one Blender
/// was launched on, so nothing is spliced in and the script is the same bytes
/// for every reading. **It is composed of two constants rather than written as
/// one** only so that the shared walk has one home; nothing about a file
/// changes it.
pub fn the_check_readings_script() -> String {
    format!(
        "{}\n{}\n{THE_CHECK_READINGS_BODY}",
        crate::reading_script::THE_WALK_OVER_THE_NESTED_MARKS,
        // **`Job files and outputs` R18, added 2026-10-02**: the check is a
        // read of the user's file and carries the load-time handler that
        // points every Replay fluid cache and every disk cache at a throwaway
        // folder, so every setting it reports goes through the handler's
        // record and comes out as the file has it.
        smoozy_job_files_machine::reading_script::THE_READ_AS_THE_FILE_HAS_IT,
    )
}

/// **G1 and `Job files and outputs` R1: the document comes back between
/// sentinels because Blender's own chatter shares the stream.** The marks are
/// this reading's own and not the file's reading's, so a stream that somehow
/// carried both could never have one document read as the other. **The text is
/// Build's own choice**, which the plan fixes nowhere. The script writes the
/// same two, which a test holds.
pub const THE_OPENING_SENTINEL: &[u8] = b"<<<SMOOZY-DEPENDENCIES>>>";
/// The closing mark, and the same is true of it.
pub const THE_CLOSING_SENTINEL: &[u8] = b"<<<END-SMOOZY-DEPENDENCIES>>>";

/// The bytes between the two marks, or nothing where both marks are not there.
///
/// **G3: a stream cut short before the closing mark carries no document rather
/// than an empty one.** Nothing is guessed from a stream that does not carry
/// both marks.
pub fn between_the_sentinels(stream: &[u8]) -> Option<&[u8]> {
    let opens = find(stream, THE_OPENING_SENTINEL)? + THE_OPENING_SENTINEL.len();
    let closes = find(&stream[opens..], THE_CLOSING_SENTINEL)?;
    Some(&stream[opens..opens + closes])
}

/// **Whether this reading's own start mark stands anywhere in a stream** —
/// `Render Settings` Q8's *"the second from the third by the start sentinel,
/// which is Smoozy's own frame"*, which the worker reports.
pub fn the_start_mark_is_in(stream: &[u8]) -> bool {
    find(stream, THE_OPENING_SENTINEL).is_some()
}

fn find(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    if needle.len() > haystack.len() {
        return None;
    }
    haystack
        .windows(needle.len())
        .position(|window| window == needle)
}

/// **The words after the build's own path**, for one check's reading: the
/// startup options, then the file, then the script — the order above, each
/// argument quoted there.
///
/// **Every value is a word of its own and is handed to the kernel as an
/// argument**, never spliced into a line a shell reads (`Dependency Upload &
/// Remap` A8 and `Server protocol and transfers` §1.1). The `.blend`'s path is
/// the machine's own and crosses as the bytes it is, never decoded.
///
/// **`Rendering` L8b's script sits before the file, added 2026-10-02**: the
/// load-time script whose handler points every Replay fluid cache and every
/// disk cache at the run's own throwaway folder (`Job files and outputs` R18).
/// Opening a Replay file with nothing else wrote its frame 1 beside the
/// `.blend` (observed at the desk, 4.5.12 and 5.2.2).
pub fn the_words_of_the_check_reading(
    the_blend_path_on_the_machine: &[u8],
    the_load_time_scripts_path: &[u8],
    the_scripts_path: &[u8],
) -> Vec<Vec<u8>> {
    vec![
        b"-b".to_vec(),
        b"--factory-startup".to_vec(),
        // **The file's own scripts run, as in every render** (`Rendering` L9).
        b"--enable-autoexec".to_vec(),
        b"--python".to_vec(),
        the_load_time_scripts_path.to_vec(),
        the_blend_path_on_the_machine.to_vec(),
        b"--python-exit-code".to_vec(),
        b"1".to_vec(),
        b"--python".to_vec(),
        the_scripts_path.to_vec(),
    ]
}

const THE_CHECK_READINGS_BODY: &str = r#"
import bpy, json, os, sys

OPENS = "<<<SMOOZY-DEPENDENCIES>>>"
CLOSES = "<<<END-SMOOZY-DEPENDENCIES>>>"

def as_bytes(value):
    # G3a and `Job files and outputs` R4a: the location crosses as the bytes the
    # file holds. `os.fsencode` is the reverse of the decoding that produced the
    # string, so a name that is not text survives whole rather than being
    # substituted or dropped.
    if isinstance(value, bytes):
        raw = value
    else:
        raw = os.fsencode(value)
    return list(raw)

# The eight collections whose own type carries a recorded location, with the
# kind name `crate::check::Kind` uses for each.
COLLECTIONS = [
    ("libraries", "Library"),
    ("images", "Image"),
    ("sounds", "Sound"),
    ("fonts", "Font"),
    ("movieclips", "MovieClip"),
    ("volumes", "Volume"),
    ("cache_files", "CacheArchive"),
    ("texts", "Text"),
]

references = []

# ------------------------------------------------ G6's two tests (DW0-1)
#
# What is carried inside the file is not a reference, and neither is a setting
# that records nothing. Every name here is quoted in this file's Rust header;
# a version whose data-block carries no such property produces the reference
# as before, which is what the defaults below are for.
def carried_inside_the_file(datablock):
    if getattr(datablock, "packed_file", None) is not None:
        return True
    # From 5.0: the library that holds packed linked data.
    if getattr(datablock, "is_archive", False):
        return True
    return False

def records_nothing(recorded):
    # **G6e**: `<builtin>` is how a font records Blender's own: no file.
    return recorded is None or recorded == "" or recorded == "<builtin>"

def the_default_cache_folder(cache):
    # **G6, added 2026-10-02**: where a point cache keeps its cache on disk,
    # does not read it from elsewhere, and records no path, Blender writes it
    # to `blendcache_` and the file's own name without its ending, beside the
    # `.blend` (`Job files and outputs` D21c). That folder, written in
    # Blender's own `//` shorthand; else nothing. **Only a cache marked
    # baked** — G6's *a row where a baked result exists to send*; files left
    # there by an unbaked cache are not a result to send. **Whether the folder
    # is beside this copy of the file is not asked** (G6 as amended after
    # 0.1.8): this copy is the machine's, and a `.blend` uploaded alone has
    # its folder only on the user's computer, where the search finds it.
    # **`Job files and outputs` R18**: as the file has them, never the
    # throwaway values the load-time handler set.
    if not hasattr(cache, "use_disk_cache"):
        return None
    if not cache.use_disk_cache or smoozy_as_the_file_has_it(cache, "use_external"):
        return None
    if not smoozy_as_the_file_has_it(cache, "is_baked"):
        return None
    blend = bpy.data.filepath
    if not blend:
        return None
    return "//blendcache_" + os.path.splitext(os.path.basename(blend))[0] + "/"

# G9 with DW0-3: a picture's and a movie clip's source as Blender stores it,
# carried and not judged here.
def stored_source(kind, datablock):
    if kind in ("Image", "MovieClip"):
        return getattr(datablock, "source", None)
    return None

# A bake's own `INHERIT` reads through to its modifier, and the shared walk
# hands this loop the bake and not the modifier — so the modifier is found
# here, by Blender's own pointer, and the walk is left as the remap and the
# fixing need it.
modifier_of_bake = {}
for obj in bpy.data.objects:
    for modifier in getattr(obj, "modifiers", ()):
        for bake in getattr(modifier, "bakes", ()):
            modifier_of_bake[bake.as_pointer()] = modifier

def the_bake_keeps_its_data_inside_the_file(holder, attribute):
    if attribute == "bake_directory":
        # The modifier's own folder: packed where the modifier says so and no
        # bake of it says otherwise.
        if getattr(holder, "bake_target", None) != "PACKED":
            return False
        for bake in getattr(holder, "bakes", ()):
            if getattr(bake, "bake_target", None) == "DISK":
                return False
        return True
    target = getattr(holder, "bake_target", None)
    if target == "INHERIT":
        modifier = modifier_of_bake.get(holder.as_pointer())
        target = getattr(modifier, "bake_target", None) if modifier is not None else None
    return target == "PACKED"

# ---------------------------------------------------------------- G19's walk
#
# One call, then the walk outwards. The map is asked for once and walked many
# times, which is the shape the map is for.
try:
    user_map = bpy.data.user_map()
except Exception:
    user_map = None

scene_names = set(scene.name for scene in bpy.data.scenes)

def scenes_reaching(datablock):
    # G20a: the walk does not stop at the first scene, and *established* means
    # the walk completed. A walk that could not be made says nothing rather
    # than guessing.
    if user_map is None:
        return "NotEstablished"
    reached = set()
    seen = set()
    frontier = [datablock]
    while frontier:
        one = frontier.pop()
        key = id(one)
        if key in seen:
            continue
        seen.add(key)
        try:
            users = user_map.get(one, ())
        except Exception:
            return "NotEstablished"
        for user in users:
            if isinstance(user, bpy.types.Scene):
                reached.add(user.name)
            frontier.append(user)
    return {"Established": sorted(reached)}

# ------------------------------------------------- G6's per-setting walk
#
# The walk over Blender's own path marks on its settings. Each collection is
# the one its own type's `filepath` belongs to, so the setting written into the
# pair is a property path and never an instance (M4, M8).
for collection, kind in COLLECTIONS:
    holder = getattr(bpy.data, collection, None)
    if holder is None:
        continue
    for datablock in holder:
        # G3, in the third of its shapes and the only one that did not
        # substitute an empty answer: a catch here used to drop this one
        # data-block's reference and carry on. The file came back **checked**,
        # one reference short, with nothing anywhere saying so — worse than an
        # empty list, because an empty list at least looks wrong. There is no
        # catch: a `filepath` that will not read raises, and the whole reading
        # is *could not be read* with the version named.
        recorded = datablock.filepath
        # G6's two tests. Neither is a catch: a data-block that is carried
        # inside, or that records no location, is not a reference at all,
        # and the `filepath` above has already been read without one.
        if records_nothing(recorded) or carried_inside_the_file(datablock):
            continue
        references.append({
            "kind": kind,
            "datablock": datablock.name,
            "setting": "filepath",
            "recorded": as_bytes(recorded),
            "scenes": scenes_reaching(datablock),
            "source": stored_source(kind, datablock),
        })

# ------------------------------------------------------- G8's node-tree walk
#
# Every node tree the reference names a property for, then into every group
# node's own tree. The visited set is by identity, so a group reached twice is
# walked once and a tree that comes back round stops there.
def every_tree():
    seen = set()
    frontier = []
    for tree in bpy.data.node_groups:
        frontier.append((tree, tree.name))
    for collection, name in (
        ("materials", "node_tree"),
        ("worlds", "node_tree"),
        ("lights", "node_tree"),
        ("textures", "node_tree"),
        ("linestyles", "node_tree"),
    ):
        holder = getattr(bpy.data, collection, None)
        if holder is None:
            continue
        for owner in holder:
            tree = getattr(owner, name, None)
            if tree is not None:
                frontier.append((tree, owner.name))
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
                frontier.append((tree, scene.name))
    for obj in bpy.data.objects:
        for modifier in getattr(obj, "modifiers", ()):
            tree = getattr(modifier, "node_group", None)
            if tree is not None:
                frontier.append((tree, obj.name))
    while frontier:
        tree, owner_name = frontier.pop()
        key = id(tree)
        if key in seen:
            continue
        seen.add(key)
        yield tree, owner_name
        for node in getattr(tree, "nodes", ()):
            inner = getattr(node, "node_tree", None)
            if inner is not None:
                frontier.append((inner, owner_name))

# G8's own reason for this walk: in 4.5 an import node's path takes no part in
# the file's own path housekeeping, so neither bulk route reaches it and the
# node itself is the only route there is.
for tree, owner_name in every_tree():
    for node in getattr(tree, "nodes", ()):
        kind_name = getattr(node, "bl_idname", "")
        if not kind_name.startswith("GeometryNodeImport"):
            continue
        socket = None
        for one in getattr(node, "inputs", ()):
            if one.name == "Path":
                socket = one
                break
        if socket is None:
            continue
        # A socket fed by a link has no stored value to read, and nothing is
        # invented for it.
        if getattr(socket, "is_linked", False):
            continue
        recorded = getattr(socket, "default_value", None)
        if recorded is None or recorded == "":
            continue
        references.append({
            "kind": "ImportNodePath",
            "datablock": owner_name + "/" + node.name,
            "setting": "inputs[\"Path\"].default_value",
            "recorded": as_bytes(recorded),
            "scenes": scenes_reaching(tree),
            "source": None,
        })

# ------------------------------------- G6's marks that are not a data-block's
#
# The eight collections above reach nine of G6's kinds. Every kind below sits
# on a node, a modifier, a strip, a surface or a cache layer, and no walk over
# `bpy.data`'s own collections reaches any of them. The walk is
# `THE_WALK_OVER_THE_NESTED_MARKS` and the remap re-walks the same generator.
for kind, where, holder, attribute, owner in smoozy_nested_marks():
    # **`Job files and outputs` R18**: as the file has it, never the throwaway
    # folder the load-time handler set.
    recorded = smoozy_as_the_file_has_it(holder, attribute)
    # **G6's one exception to the rule below, added 2026-10-02**: a physics
    # disk cache that records no path writes to the default folder beside the
    # `.blend` (`Job files and outputs` D21c, *Dependencies 10*), so where the
    # cache is baked that folder is a reference like any other cache folder,
    # found on the user's computer or not found.
    if kind == "PhysicsPointCache" and records_nothing(recorded):
        recorded = the_default_cache_folder(holder)
    # An empty setting records no location, so there is nothing to find and
    # nothing to point anywhere — the same rule the import-node walk above
    # already keeps. Most of these properties default to the empty string, so
    # without this every file would carry a row for a bake nobody baked.
    if records_nothing(recorded):
        continue
    # G6: a geometry-node bake whose bake target keeps its data inside the
    # file is not a reference. **Decided here, in the check's own loop, and
    # never inside the shared walk**, which the remap and the fixing walk too.
    if kind == "GeometryNodeBakeFolder" and the_bake_keeps_its_data_inside_the_file(holder, attribute):
        continue
    references.append({
        "kind": kind,
        "datablock": where,
        "setting": attribute,
        "recorded": as_bytes(recorded),
        "scenes": scenes_reaching(owner),
        "source": None,
    })

# ---------------------------------------------------- G6's map and flat list
#
# The map is not optional and the flat list is never substituted for it. What
# the map reaches beyond the per-setting walk above joins the references as its
# own owner's; what the flat list carries joins the document as the flat list,
# which is what G27's exclusion is read against.
BY_TYPE = {
    "Library": "Library",
    "Image": "Image",
    "Sound": "Sound",
    "VectorFont": "Font",
    "MovieClip": "MovieClip",
    "Volume": "Volume",
    "CacheFile": "CacheArchive",
    "Text": "Text",
}

# **G3: a reading that did not complete is not a reading, and this call is not
# caught.** An empty map is not a file with no references — it is a reading
# that lost the whole of one route, and G6 says which route: *"where a linked
# library points at a picture of its own, the map reaches through the library
# and the flat list does not"*. Substituting an empty map would report every
# such reference as absent, the panel would show a clean file, and the render
# would be silently missing a library's own picture. The call raises, the run
# exits non-zero on `--python-exit-code 1`, and `crate::check::judge` turns
# that into *could not be read*.
mapped = bpy.data.file_path_map(include_libraries=True)
already = set()
for one in references:
    already.add(tuple(one["recorded"]))
# **G6c: a packed library's own location is no reference, whichever data-block
# the map names it under.** The map hands back a linked data-block's library as
# its path where it has no file of its own, so a text block in a packed library
# came through as a *Text* row naming the library. Only that location: a linked
# picture's own outside file still comes through, since packing a linked library
# does not carry that library's own outside files (*Node setups 2*).
for library in bpy.data.libraries:
    if carried_inside_the_file(library) and not records_nothing(library.filepath):
        already.add(tuple(as_bytes(smoozy_a_path_as_the_file_has_it(library.filepath))))
for datablock, paths in mapped.items():
    # **The kind is the owner's own type and is never guessed**, because the map
    # hands back a path with no word about which setting it came off. A modifier's
    # path arrives under its **object**, and an object could be carrying a fluid
    # cache, an ocean bake, a multi-resolution displacement or a geometry-node
    # bake — so a kind taken from the map here would be a coin toss, and a pair
    # of the wrong kind writes into the wrong setting.
    #
    # **What makes that safe is that the two walks above reach these by name.**
    # The per-setting walk takes the eight collections and the nested walk takes
    # every other kind G6 lists that a route is established for, each with the
    # setting it came off. So a path the map holds under an owner this table does
    # not name is one those walks have already contributed, and it is dropped
    # here as a duplicate rather than lost.
    #
    # **What is genuinely left behind, named rather than covered by a clause that
    # says the opposite.** A path under an owner neither walk reaches: a brush's
    # custom icon, for which the reference establishes no property at all; the
    # record kept with a packed file and the weak reference kept after appending,
    # both deliberate and both stated at `THE_WALK_OVER_THE_NESTED_MARKS`; and a
    # path in a custom property, which is G7's own case and §12 item 13's silence.
    # **G7 and item 13 are cited for those and for nothing on G6's list**, because
    # item 13's Holds is *"every kind on the list, and the check's own honesty,
    # which never claims the list is all of them"* — a kind on the list dropped
    # here would be the very thing that clause promises does not happen.
    kind = BY_TYPE.get(type(datablock).__name__)
    if kind is None:
        continue
    # G6: the map is held to the same two tests as the walk above, or a packed
    # picture the walk left out comes straight back through here as a new
    # reference, and so does an empty location.
    if carried_inside_the_file(datablock):
        continue
    for path in paths:
        # **`Job files and outputs` R18**: a throwaway folder told back.
        path = smoozy_a_path_as_the_file_has_it(path)
        if records_nothing(path):
            continue
        raw = as_bytes(path)
        if tuple(raw) in already:
            continue
        already.add(tuple(raw))
        references.append({
            "kind": kind,
            "datablock": getattr(datablock, "name", ""),
            "setting": "filepath",
            "recorded": raw,
            "scenes": scenes_reaching(datablock),
            "source": stored_source(kind, datablock),
        })

# ------------------------------------------- G6d, a library as the file writes it
#
# As a file loads, Blender writes a library's relative location again against the
# folder the file sits in, stopping at the root (observed, `Sources` §10), so it
# is the file's bytes that say what the file holds. Only for the row and the first
# look: the key stays what this Blender read, since the remap meets that text.
# **Everything here is a figure to show**, so a file this cannot walk says what
# Blender read, as before: the bundled reader absent, or a block that will not
# read. The block heads are read off the two forms the file's own opening names
# (`App/File Browser/src/blend_version.rs`), observed at the desk in six versions.
def smoozy_the_libraries_as_the_file_writes_them():
    import re, struct
    found = []
    try:
        with open(bpy.data.filepath, "rb") as raw:
            compressed = raw.read(4) == b"\x28\xb5\x2f\xfd"
            raw.seek(0)
            source = raw
            if compressed:
                import zstandard
                source = zstandard.ZstdDecompressor().stream_reader(raw, read_across_frames=True)
            def take(count):
                out = bytearray()
                while len(out) < count:
                    piece = source.read(min(count - len(out), 1 << 20))
                    if not piece:
                        break
                    out += piece
                return bytes(out)
            opening = take(12)
            if opening[:7] != b"BLENDER":
                return found
            if opening[7:9].isdigit():
                take(int(opening[7:9]) - 12)
                head, code_at, length_at = "<4siQqq", 0, 3
            else:
                order = "<" if opening[8:9] == b"v" else ">"
                pointer = "Q" if opening[7:8] == b"-" else "I"
                head, code_at, length_at = order + "4si" + pointer + "ii", 0, 1
            size = struct.calcsize(head)
            while True:
                block = take(size)
                if len(block) < size:
                    return found
                fields = struct.unpack(head, block)
                code, length = fields[code_at], fields[length_at]
                if code == b"ENDB" or length < 0:
                    return found
                if code[:2] == b"LI" and code[2:] == b"\0\0":
                    text = re.search(rb"(?:^|\0)(//[^\0]+)\0", take(length))
                    if text:
                        found.append(os.fsdecode(text.group(1)).replace("\\", "/"))
                elif compressed:
                    while length > 0:
                        piece = take(min(length, 1 << 20))
                        if not piece:
                            return found
                        length -= len(piece)
                else:
                    source.seek(length, 1)
    except Exception:
        return []

def smoozy_where_it_resolves(location):
    return os.path.normpath(bpy.path.abspath(location))

the_relative_libraries = [
    one for one in references
    if one["kind"] == "Library" and bytes(one["recorded"]).startswith(b"//")
]
if the_relative_libraries and bpy.data.filepath:
    the_files_own = smoozy_the_libraries_as_the_file_writes_them()
    for one in the_relative_libraries:
        recorded = os.fsdecode(bytes(one["recorded"]))
        for own in the_files_own:
            if own != recorded and smoozy_where_it_resolves(own) == smoozy_where_it_resolves(recorded):
                one["the_files_own_text"] = as_bytes(own)
                break

# ------------------------------------- G27a's presence test, on this Blender
#
# A library whose recorded location runs through a `datafiles/assets/` folder,
# where this Blender holds a file at the same place under its own folder of
# that name, is a file Blender ships. The reading answers only the presence;
# `smoozy-dependencies`' `check` leaves the row out. Never a list of names, an
# application path or a version string: the remainder is looked for, and
# nothing else is matched.
def smoozy_the_remainder_under_assets(recorded):
    steps = recorded.replace("\\", "/").split("/")
    for at in range(len(steps) - 2, -1, -1):
        if steps[at] == "datafiles" and steps[at + 1] == "assets":
            rest = steps[at + 2:]
            if rest and all(step not in ("", ".", "..") for step in rest):
                return rest
            return None
    return None

def smoozy_this_blender_holds(rest):
    own = bpy.utils.system_resource("DATAFILES")
    if not own:
        return False
    return os.path.isfile(os.path.join(own, "assets", *rest))

shipped = []
for one in references:
    if one["kind"] != "Library":
        continue
    rest = smoozy_the_remainder_under_assets(os.fsdecode(bytes(one["recorded"])))
    if rest is not None and smoozy_this_blender_holds(rest):
        shipped.append(one["recorded"])

# **G3 again, same shape:** an empty flat list is not a file with no outside
# files. G27's exclusion is read against this list, so an empty one silently
# turns every one of the application's own entries into something the exclusion
# never sees. Not caught. **G6's empty-location test holds here too**; the
# packed test cannot, because the list carries no owner, and it produces no
# reference either way — see this file's Rust header.
flat = [
    as_bytes(one)
    for one in map(smoozy_a_path_as_the_file_has_it, bpy.utils.blend_paths())
    if not records_nothing(one)
]

# ------------------------------------------------------ G27's weak references
weak = []
for collection, _kind in COLLECTIONS:
    holder = getattr(bpy.data, collection, None)
    if holder is None:
        continue
    for datablock in holder:
        reference = getattr(datablock, "library_weak_reference", None)
        if reference is not None:
            weak.append({
                "filepath": as_bytes(reference.filepath),
                "id_name": reference.id_name,
            })
for collection in ("materials", "objects", "meshes", "node_groups", "worlds", "scenes"):
    holder = getattr(bpy.data, collection, None)
    if holder is None:
        continue
    for datablock in holder:
        reference = getattr(datablock, "library_weak_reference", None)
        if reference is not None:
            weak.append({
                "filepath": as_bytes(reference.filepath),
                "id_name": reference.id_name,
            })

document = {
    "references": references,
    "flat_list": flat,
    "weak_references": weak,
    "shipped_with_this_blender": shipped,
    # §12 item 20: whether this reading can observe a linked stand-in at all is
    # permanently unresolved, so it observes none and says nothing. Never
    # inferred from an absence.
    "observed_stand_ins": [],
}

sys.stdout.write(OPENS)
sys.stdout.write(json.dumps(document))
sys.stdout.write(CLOSES)
sys.stdout.flush()
"#;

#[cfg(test)]
mod tests {
    use super::*;

    /// **G6, added 2026-10-02: a physics disk cache that records no path is
    /// listed at its default folder beside the `.blend`** in Blender's own
    /// shorthand, where it is baked — whether or not the folder is beside the
    /// machine's copy (after 0.1.8, the checks' leftovers). Before, the check skipped
    /// every setting that records nothing, so the commonest physics cache was
    /// never found or sent (at the desk, 4.5.12 and 5.2.2: a baked cloth at its
    /// default folder gave the check one row, the bake output path).
    #[test]
    fn a_physics_cache_at_its_default_folder_is_looked_for() {
        let text = the_check_readings_script();
        assert!(text.contains(
            "if kind == \"PhysicsPointCache\" and records_nothing(recorded):\n        recorded = the_default_cache_folder(holder)"
        ));
        let rule = &text[text.find("def the_default_cache_folder(").unwrap()..];
        let rule = &rule[..rule.find("\n\n").unwrap()];
        assert!(rule.contains("use_disk_cache"));
        assert!(rule.contains("use_external"));
        assert!(rule.contains("is_baked"));
        assert!(rule.contains(
            r#"return "//blendcache_" + os.path.splitext(os.path.basename(blend))[0] + "/""#
        ));
        // **After 0.1.8, the checks' leftovers, item 1: the folder is not
        // looked for beside the machine's copy.** The check runs there, and a
        // `.blend` uploaded alone has its folder only on the user's computer,
        // so the check before 0.1.8 listed nothing for it.
        assert!(!rule.contains("os.path.isdir("), "{rule}");
        assert!(!rule.contains("os.path.exists("), "{rule}");
    }

    /// **G6c: a packed library's own location is taken before the map is
    /// walked**, so no data-block brings it back as a row, and **only that
    /// location**: a linked picture's own outside file still comes through.
    /// Run for real in `a_packed_librarys_text_block_is_no_row_at_the_desk`.
    #[test]
    fn a_packed_librarys_own_location_is_taken_before_the_map_is_walked() {
        let text = the_check_readings_script();
        let taken = text
            .find("for library in bpy.data.libraries:\n    if carried_inside_the_file(library) and not records_nothing(library.filepath):")
            .expect("the libraries are asked");
        assert!(taken < text.find("for datablock, paths in mapped.items():").expect("the map's walk"));
        assert!(!text.contains("getattr(datablock, \"library\""), "no data-block is carried for its library");
    }

    /// **G6d: only for a library's row, and never over the key.**
    #[test]
    fn a_librarys_own_text_sits_beside_its_recorded_location() {
        let text = the_check_readings_script();
        assert!(text.contains("one[\"the_files_own_text\"] = as_bytes(own)"));
        let at = text.find("def smoozy_the_libraries_as_the_file_writes_them():").expect("the reader");
        assert!(at < text.find("document = {").expect("the document"));
    }

    /// **G27a: the reading answers presence on its own Blender, for libraries
    /// alone, and carries the answer under the name the reader takes it by**
    /// — and names no file, no application and no version of its own.
    #[test]
    fn the_reading_asks_its_own_blender_for_a_shipped_library_by_presence() {
        let text = the_check_readings_script();
        let test = &text[text.find("def smoozy_this_blender_holds(").unwrap()..];
        let test = &test[..test.find("\n\n").unwrap()];
        assert!(test.contains(r#"bpy.utils.system_resource("DATAFILES")"#), "{test}");
        assert!(test.contains("if not own:\n        return False"), "{test}");
        assert!(test.contains(r#"os.path.isfile(os.path.join(own, "assets", *rest))"#), "{test}");
        let walk = &text[text.find("def smoozy_the_remainder_under_assets(").unwrap()..];
        let walk = &walk[..walk.find("\n\n").unwrap()];
        assert!(walk.contains(r#"steps[at] == "datafiles" and steps[at + 1] == "assets""#), "{walk}");
        assert!(walk.contains(r#"step not in ("", ".", "..")"#), "{walk}");
        assert!(text.contains(
            "for one in references:\n    if one[\"kind\"] != \"Library\":\n        continue\n    rest = smoozy_the_remainder_under_assets(os.fsdecode(bytes(one[\"recorded\"])))\n    if rest is not None and smoozy_this_blender_holds(rest):\n        shipped.append(one[\"recorded\"])"
        ));
        assert!(text.contains(r#""shipped_with_this_blender": shipped,"#));
        for named in ["essentials", "Applications", "Blender.app", "nodes/"] {
            assert!(!test.contains(named) && !walk.contains(named), "{named}");
        }
    }

    /// **The script writes the marks the reader looks for**, and no others.
    #[test]
    fn the_script_writes_the_two_marks_the_reader_looks_for() {
        let script = the_check_readings_script();
        for mark in [THE_OPENING_SENTINEL, THE_CLOSING_SENTINEL] {
            let mark = std::str::from_utf8(mark).expect("the mark is text");
            assert!(script.contains(&format!("\"{mark}\"")), "{mark}");
        }
        assert!(script.contains(crate::reading_script::THE_WALK_OVER_THE_NESTED_MARKS));
    }

    /// **G3: both marks or no document**, and the start mark alone is seen
    /// without a document being read.
    #[test]
    fn a_document_needs_both_marks_and_the_start_mark_is_seen_on_its_own() {
        let whole = [
            b"chatter ".as_slice(),
            THE_OPENING_SENTINEL,
            b"{}",
            THE_CLOSING_SENTINEL,
            b" more chatter",
        ]
        .concat();
        assert_eq!(between_the_sentinels(&whole), Some(&b"{}"[..]));
        let cut = [b"chatter ".as_slice(), THE_OPENING_SENTINEL, b"{\"refe"].concat();
        assert_eq!(between_the_sentinels(&cut), None);
        assert!(the_start_mark_is_in(&cut));
        assert!(!the_start_mark_is_in(b"Blender quit"));
        assert_eq!(between_the_sentinels(b""), None);
    }

    /// **The order the manual permits and `Rendering` L8 composes in**, and
    /// nothing after a bare double dash.
    #[test]
    fn the_file_stands_after_the_startup_options_and_before_the_script() {
        let words = the_words_of_the_check_reading(
            b"/root/a b.blend",
            b"/area/script-load-time.py",
            b"/area/script.py",
        );
        let at = |word: &[u8]| words.iter().position(|one| one == word).expect("the word");
        assert!(at(b"--factory-startup") < at(b"/root/a b.blend"));
        // **`Rendering` L8b's script stands right before the file**
        // (`Job files and outputs` R18), and the check's own script after it.
        assert_eq!(at(b"/area/script-load-time.py") + 1, at(b"/root/a b.blend"));
        assert_eq!(words[at(b"/area/script-load-time.py") - 1], b"--python");
        assert!(at(b"/root/a b.blend") < at(b"/area/script.py"));
        assert_eq!(words[at(b"/area/script.py") - 1], b"--python");
        assert_eq!(words[at(b"--python-exit-code") + 1], b"1");
        assert!(!words.iter().any(|word| word == b"--"));
    }

    /// **The check's reading lets the file's own scripts run, as every render
    /// does** (the Creator, 2026-10-08: *"Yes to running scripts when
    /// reading."*; `Rendering` L9) — the switch once, among the startup
    /// options and before the file and both scripts.
    #[test]
    fn the_check_reading_lets_the_files_own_scripts_run_once_before_the_file() {
        let words = the_words_of_the_check_reading(b"/root/a.blend", b"/l.py", b"/s.py");
        let switch: &[u8] = b"--enable-autoexec";
        assert_eq!(words.iter().filter(|word| *word == switch).count(), 1, "{words:?}");
        let at = |word: &[u8]| words.iter().position(|one| one == word).expect("the word");
        assert!(at(b"--factory-startup") < at(switch));
        assert!(at(switch) < at(b"/l.py"));
        assert!(at(switch) < at(b"/root/a.blend"));
    }

    /// **`Job files and outputs` R18: the check reads every setting the
    /// load-time handler changes as the file has it**, and tells a throwaway
    /// folder in the map or the flat list back to the value it replaced.
    #[test]
    fn the_check_reads_what_the_handler_changed_as_the_file_has_it() {
        let text = the_check_readings_script();
        assert!(text.contains(smoozy_job_files_machine::reading_script::THE_READ_AS_THE_FILE_HAS_IT));
        for read in [
            "recorded = smoozy_as_the_file_has_it(holder, attribute)",
            r#"smoozy_as_the_file_has_it(cache, "use_external")"#,
            r#"smoozy_as_the_file_has_it(cache, "is_baked")"#,
            "path = smoozy_a_path_as_the_file_has_it(path)",
            "map(smoozy_a_path_as_the_file_has_it, bpy.utils.blend_paths())",
        ] {
            assert!(text.contains(read), "{read}");
        }
        assert!(!text.contains("recorded = getattr(holder, attribute)"));
    }

    /// **A path that is not text is carried whole** (`Job files and outputs`
    /// R4a's rule, met at the launch).
    #[test]
    fn a_path_that_is_not_text_is_one_word_whole() {
        let raw = [b'/', 0xff, 0xfe, b'.', b'b', b'l', b'e', b'n', b'd'];
        let words = the_words_of_the_check_reading(&raw, b"/l.py", b"/s.py");
        assert!(words.iter().any(|word| word.as_slice() == raw));
    }

    /// **`Job files and outputs` R4 item 1: the check reaches every effect's
    /// group from 5.3, and asks for the older name only where a scene has no
    /// stack.** Run for real at the desk (`packed_linked_data_at_the_desk.rs`).
    #[test]
    fn the_check_reaches_every_effect_and_reads_the_older_name_only_before_the_stack() {
        for script in [
            the_check_readings_script(),
            crate::material::THE_HEAD.to_string(),
            crate::reading_script::THE_WALK_OVER_THE_NESTED_MARKS.to_string(),
        ] {
            let stack = script.find("effects = getattr(scene, \"compositor_effects\", None)").expect("the stack is asked for");
            let older = script.find("trees = [getattr(scene, \"compositing_node_group\", None)]").expect("the older name is kept");
            assert!(stack < older);
            assert!(script.contains("trees = [effect.node_group for effect in effects]"));
        }
    }
}

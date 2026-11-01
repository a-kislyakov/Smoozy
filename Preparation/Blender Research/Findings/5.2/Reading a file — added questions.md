# Reading a file — added questions (Blender 5.2)

This document answers question 9 of the sheet, and no other.

Sources consulted: the Blender 5.2 manual (`Blender Sources/manual_5.2`), the Blender 5.2 Python API documentation (`Blender Sources/api_5.2`), the Blender 5.2 release notes (`Blender Sources/release_notes_5.2`) and Cycles' own files (`Blender Sources/cycles_5.2`), all read on 2026-07-30 (the date the snapshot was taken). Blender's requirements page (`Blender Sources/requirements/requirements_2026-08-01.txt`), read 2026-08-01.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

---

## 9. What separate means exist for listing and rewriting the locations recorded in a `.blend` file, and which kinds of recorded location does each of those means reach?

**Short answer:** I group what the sources describe into ten distinct means. The grouping and the count are mine — no source lists them together or counts them. Two list only (Means 2 and 3). One lists and rewrites in one pass (Means 1). One reads and writes a single recorded location at a time (Means 4). Three rewrite in bulk, each over a different set (Means 5, 6 and 7). One reads the library paths recorded in another `.blend`, when that file is loaded as a library (Means 8). One is the interface's own path field (Means 9). One is a command-line argument that sets the render output path (Means 10).

Their reach differs sharply. The broadest wordings are Means 1 and 2 ("the file paths used by all ID data-blocks"), Means 3 ("external files referenced by the loaded .blend file") and Means 5 ("all paths to external files"); the rest are tied to one kind of location — libraries, relative paths, one data-block at a time, or the render output path. Setting those wordings against each other is my step and not something any source does, so treat every comparison in this answer as **inferred** even where the wording either side of it is documented.

Each means below carries its own label, quote, source and date.

---

### Means 1 — `bpy.data.file_path_foreach()` — lists **and** rewrites in one pass

**LABEL: documented.**

Exact words:

> "file_path_foreach(visit_path_fn, *, subset=None, visit_types=None, flags={'SKIP_PACKED', 'SKIP_WEAK_REFERENCES'})
>
> Call visit_path_fn for the file paths used by all ID data-blocks in current bpy.data.
>
> For list of valid set members for visit_types, see: bpy.types.KeyingSetPath.id_type."

and, on the rewriting half:

> "visit_path_fn (Callable[[bpy.types.ID, str, bpy.types.BlendDataPathMeta], str|None]) – function that takes three parameters: the data-block, a file path, and a bpy.types.BlendDataPathMeta metadata object. The function should return either None or a str. In the latter case, the visited file path will be replaced with the returned string."

and on which paths may be filtered in or out:

> "subset (set[str] | None) – When given, only these data-blocks and their used file paths will be visited.
>
> visit_types (set[str] | None) – When given, only visit data-blocks of these types. Ignored if subset is also given.
>
> flags (set[str]) – Set of flags that influence which data-blocks are visited."

**Citation check note (Opus 5, 2026-08-06):** the first of those three lines used to read `subset (Sequence[bpy.types.ID] | None)`. That type belongs to the *other* method on the same page — `file_path_map`'s `subset`, whose wording also differs ("will be included as keys/values in the map"). `file_path_foreach`'s own `subset` reads `set[str] | None`. Corrected against the page.

Source: Python API documentation, `https://docs.blender.org/api/5.2/bpy.types.BlendData.html`. Date read: 2026-07-30.

**Which kinds it reaches.** "the file paths used by all ID data-blocks in current bpy.data" — the broadest wording of any means here. That ranking is my comparison across the ten, not a statement any source makes: **inferred**. The flag list spells out the categories it can be told to include or skip:

> "SKIP_LINKED: Skip Linked. Skip paths of linked IDs.
>
> SKIP_PACKED: Skip Packed. Skip paths when their matching data is packed.
>
> SKIP_WEAK_REFERENCES: Skip Weak References. Skip weak reference paths. Those paths are typically 'nice to have' extra information, but are not used as actual source of data by the current .blend file.
>
> SKIP_MULTIFILE: Skip Multi-file. Skip paths where a single dir is used with an array of files, eg. sequence strip images or point-caches. In this case only the first file path is processed. This is needed for directory manipulation callbacks which might otherwise modify the same directory multiple times.
>
> RELOAD_EDITED: Reload Edited. Reload data when the path is edited.
>
> RESOLVE_TOKEN: Resolve Tokens. Resolve tokens within a virtual filepath to a single, concrete, filepath. Currently only used for UDIM tiles.
>
> EXPAND_TOKENS: Expand Tokens. Expand template tokens in virtual file paths and invoke the callback once for each concrete file. Currently only used for UDIM tiles.
>
> EXPAND_SEQUENCES: Expand Sequences. Expand image and volume sequences, invoking the callback once per file on disk.
>
> EXPAND_CACHES: Expand Caches. Visit cache files, like texture cache files associated with images."

Source: Python API documentation, `https://docs.blender.org/api/5.2/bpy_types_enum_items/file_path_foreach_flag_items.html`. Date read: 2026-07-30.

Note the default in the signature: `flags={'SKIP_PACKED', 'SKIP_WEAK_REFERENCES'}`. Unless told otherwise it skips paths whose data is packed, and skips weak references.

**Not every visited path may be rewritten.** The metadata object says which:

> "class bpy.types.BlendDataPathMeta
>
> Metadata about a file path visited by bpy.types.BlendData.file_path_foreach.
>
> is_cache — True when the path is a cache file, like the image texture cache. These paths can not be edited.
>
> is_expanded — True when the path was expanded from a UDIM tile or sequence frame. These paths can not be edited.
>
> is_readonly — True when the path is read-only and can not be edited."

Source: Python API documentation, `https://docs.blender.org/api/5.2/bpy.types.BlendDataPathMeta.html`. Date read: 2026-07-30.

So this means lists more than it can rewrite: expanded per-tile and per-frame paths, and cache paths, are visited but read-only.

**What 5.2 changed here.** The release notes state it outright:

> "## Path Iterator
>
> Three new options for `bpy.data.file_path_foreach` so render farms and other scripts can find all relevant image files for rendering, including the new Cycles texture cache.
>
> - `EXPAND_TOKENS`: visit each UDIM tile path.
> - `EXPAND_SEQUENCES`: visit each image and volume sequence file path.
> - `EXPAND_CACHES`: visit cache files, like texture cache files associated with images.
>
> The previously reserved metadata field is now provided to callers, with `is_expanded` and `is_cache` booleans to indicate the file kind. Additionally `is_readonly` indicates if the path can be edited, which is not possible for expanded paths."

Source: release notes, `https://developer.blender.org/docs/release_notes/5.2/python_api/`. Date read: 2026-07-30.

The same notes record one kind of location that this means was *not* reaching until it was fixed:

> "GeoNodes simulation cache files not reported via foreach_path callback [[5523f921fc8]] - Backported to 5.1.1"

Source: release notes, `https://developer.blender.org/docs/release_notes/5.2/bugfixes/`. Date read: 2026-07-30.

---

### Means 2 — `bpy.data.file_path_map()` — lists only

**LABEL: documented.**

> "file_path_map(*, subset=None, key_types=None, include_libraries=False)
>
> Returns a mapping of all ID data-blocks in current bpy.data to a set of all file paths used by them.
>
> For list of valid set members for key_types, see: bpy.types.KeyingSetPath.id_type.
>
> subset (Sequence[bpy.types.ID] | None) – When given, only these data-blocks and their used file paths will be included as keys/values in the map.
>
> key_types (set[str] | None) – When given, filter the keys mapped by ID types. Ignored if subset is also given.
>
> include_libraries (bool) – Include library file paths of linked data. False by default.
>
> Returns: dictionary of bpy.types.ID instances, with sets of file path strings as their values."

Source: Python API documentation, `https://docs.blender.org/api/5.2/bpy.types.BlendData.html`. Date read: 2026-07-30.

**Which kinds it reaches.** All file paths used by all ID data-blocks, and it tells you which data-block each path belongs to. Library paths of linked data are excluded unless `include_libraries=True` is passed. There is no documented way to write through it — the description says only "Returns a mapping".

---

### Means 3 — `bpy.utils.blend_paths()` — lists only

**LABEL: documented.**

> "bpy.utils.blend_paths(*, absolute=False, packed=False, local=False)
>
> Returns a list of paths to external files referenced by the loaded .blend file.
>
> absolute (bool) – When true the paths returned are made absolute.
>
> packed (bool) – When true include file paths for packed data.
>
> local (bool) – When true skip linked library paths.
>
> Returns: path list."

Source: Python API documentation, `https://docs.blender.org/api/5.2/bpy.utils.html`. Date read: 2026-07-30.

**Which kinds it reaches.** "external files referenced by the loaded .blend file" — the wording is broad and the documentation does not break it down further. What it does say precisely: paths for packed data are left out unless asked for, and linked library paths are included unless `local=True`. The documented return is a "path list", and nothing is said about which data-block a path came from. That this is narrower than what Means 2 hands back is my comparison of the two wordings: **inferred**.

---

### Means 4 — the `filepath` property on individual data-blocks — one recorded location at a time

**LABEL: documented** that the property exists and what it holds. **LABEL: inferred** that it can be written — see the step written out below.

Two examples of the exact wording, which follows the same pattern throughout:

> "filepath — Path to the library .blend file (default "", never None, blend relative // prefix supported) — Type: str"

Source: Python API documentation, `https://docs.blender.org/api/5.2/bpy.types.Library.html`. Date read: 2026-07-30.

> "filepath — Image/Movie file name (default "", never None, blend relative // prefix supported) — Type: str
>
> filepath_raw — Image/Movie file name (without data refreshing) (default "", never None, blend relative // prefix supported) — Type: str"

Source: Python API documentation, `https://docs.blender.org/api/5.2/bpy.types.Image.html`. Date read: 2026-07-30.

**Reading is documented; writing is a step.** The quoted entries give the property, what it holds and its type, and say nothing about setting it. The documentation does mark properties that cannot be set — the same pages carry entries in the form "is_missing — … (default False, readonly)" — so a reader takes the absence of that mark as permission to write. That step is mine and the sources do not spell it out, so the writing half of this means is **inferred**. A hand test below is written to settle it.

**Which kinds it reaches.** Whatever type carries such a property, and nothing else — it is a per-object handle, not a sweep. Searching the API documentation for a `filepath` property turned it up on these types, among others: `Library`, `LibraryWeakReference`, `Image`, `ImagePackedFile`, `Sound`, `MovieClip`, `MovieStrip`, `StripProxy`, `VectorFont`, `Text`, `CacheFile`, `CacheFileLayer`, `Volume`, `VolumeGrids`, `PointCache`, `PointCacheItem`, `MeshCacheModifier`, `MultiresModifier`, `OceanModifier`, `ShaderNodeScript`, `ShaderNodeTexIES`, `RenderSettings`, `BakeSettings`, `CollectionExport`, `BlendImportContextLibrary`, `Depsgraph`, `RenderResult`, `RenderLayer`, `Theme`, `Preferences`, `AddonPreferences`, `Operator`, `FileHandler`, `WindowManager`, and the `BlendData*` collections. (This list is what the search returned; it is the set of pages containing the *word* `filepath`, not a claim about which of them a `.blend` records.)

**Citation check note (Opus 5, 2026-08-06).** Eight of the names above do not carry a `filepath` **property** at all — `Depsgraph`, `RenderResult`, `RenderLayer`, `Preferences`, `AddonPreferences`, `Operator`, `FileHandler` and `WindowManager` carry the word only as a method parameter or inside example code. The sentence used to say "the set of pages containing such a property", which does not hold for those eight, and it has been narrowed to the word. Twenty-six of the names do carry the property. The 5.2 page count for a bare `filepath` property heading is 27, and the one this list leaves out is `BlendData.filepath` — the open file's own location, marked read-only.

`Image` additionally carries a second, differently-behaving handle — `filepath_raw`, described as setting the name "without data refreshing".

---

### Means 5 — the File ▸ External Data operators — rewrite or report in bulk

**LABEL: documented.**

Four operators, quoted from the API documentation:

> "bpy.ops.file.make_paths_absolute() — Make all paths to external files absolute"
>
> "bpy.ops.file.make_paths_relative() — Make all paths to external files relative to current .blend"
>
> "bpy.ops.file.report_missing_files() — Report all missing external files"
>
> "bpy.ops.file.find_missing_files(*, find_all=False, directory='', ...) — Try to find missing external files
>
> find_all (bool) – Find All, Find all files in the search path (not just missing) (optional)
>
> directory (str) – Directory, Directory of the file (optional, never None)"

Source: Python API documentation, `https://docs.blender.org/api/5.2/bpy.ops.file.html`. Date read: 2026-07-30.

The manual describes the same four under the File menu, and adds what they do:

> "External data, like texture images and other resources, can be stored inside the blend-file (packed) or as separate files (unpacked). Blender keeps track of all unpacked resources via a relative or absolute path."
>
> "Make Paths Relative — Make all paths to external files relative to the current blend-file."
>
> "Make Paths Absolute — Make all paths to external files absolute (= full path from the system's root)."
>
> "Report Missing Files — This option is useful to check if there are links to unpacked files that no longer exist. After selecting this option, a warning message will appear in the Info editor's header. If no warning is shown, there are no missing external files."
>
> "Find Missing Files — In case you have broken links in a blend-file, this can help you to fix the problem. A File Browser will show up. Select the desired directory (or a file within that directory), and a search will be performed in it, recursively in all contained directories. Every missing file found in the search will be recovered. Those recoveries will be done as absolute paths, so if you want to have relative paths you will need to select Make Paths Relative."
>
> "Note — Recovered files might need to be reloaded. You can do that one by one, or you can save the blend-file and reload it again, so that all external files are reloaded at once."

Source: manual, `https://docs.blender.org/manual/en/5.2/interface/window_system/topbar.html`. Date read: 2026-07-30.

**Which kinds they reach.** "all paths to external files" is as far as the documentation goes; it does not enumerate the kinds, and it does not say whether write targets such as the render output path are included. `Report Missing Files` covers "links to unpacked files that no longer exist" — that is, unpacked external references, and its output is a warning in the Info editor's header rather than a list you can take away. `Find Missing Files` both searches and rewrites, and the manual states plainly that what it writes back is absolute.

---

### Means 6 — library relocation and reload — reaches linked `.blend` paths only

**LABEL: documented.**

> "bpy.ops.wm.lib_relocate(*, library='', filepath='', directory='', filename='', files=None, ...) — Relocate the given library to one or several others
>
> library (str) – Library, Library to relocate (optional, never None)
>
> filepath (str) – File Path, Path to file (optional, never None)"
>
> "bpy.ops.wm.lib_reload(*, library='', filepath='', ...) — Reload the given library"

Source: Python API documentation, `https://docs.blender.org/api/5.2/bpy.ops.wm.html`. Date read: 2026-07-30.

The same operations exist as Outliner entries — `bpy.ops.outliner.lib_relocate()` and `bpy.ops.outliner.id_linked_relocate()` (Python API documentation, `https://docs.blender.org/api/5.2/bpy.ops.outliner.html`, read 2026-07-30) — and `Library` itself carries a method:

> "reload() — Reload this library and all its linked data-blocks"

Source: Python API documentation, `https://docs.blender.org/api/5.2/bpy.types.Library.html`. Date read: 2026-07-30.

The manual describes the interface side:

> "Relocate Library — Editor: Outliner — Menu: Context menu ‣ Relocate — When the Outliner is in the Blender File Display Mode, a right-click on a library (a linked blend-file) will show the Relocate option. It allows to replace it by a different file. This can be used to either fix a broken linked library (e.g. because the file was moved or renamed), or to switch to a variation of the same data in a different file."
>
> "Broken Libraries — If Blender cannot find a library while loading a blend-file, it will create placeholder data-blocks to replace missing linked ones. That way, references to the missing data are not lost, and by relocating the missing library, the lost data can be automatically restored."
>
> "Relocate Linked ID — Menu: Context menu ‣ ID Data ‣ Relocate — A right-click a linked ID will show the Relocate option in the ID Data sub-menu. It allows to relocate a directly linked ID by another one, from the same library or from a different one. … Note — This operation is only available on data-blocks that are directly linked, and are not a dependency of any other linked data."

Source: manual, `https://docs.blender.org/manual/en/5.2/files/linked_libraries/link_append.html`. Date read: 2026-07-30.

**Which kinds it reaches.** Library paths — the recorded location of a linked `.blend` — and, through `id_linked_relocate`, the identity of a single directly-linked data-block. It reaches nothing else. The quoted words describe it as replacing a library by a different file. Whether that comes to anything different from writing `Library.filepath` is not compared by any source here, and the hand test below is written to settle it.

Related, for spotting which library references are broken:

> "is_missing — This data-block is a place-holder for missing linked data (i.e. it is [an override of] a linked data that could not be found anymore) (default False, readonly)"

Source: Python API documentation, `https://docs.blender.org/api/5.2/bpy.types.ID.html`. Date read: 2026-07-30.

---

### Means 7 — saving with Remap Relative — rewrites relative paths only

**LABEL: documented.**

> "bpy.ops.wm.save_as_mainfile(*, filepath='', … relative_remap=True, copy=False, …)"
>
> "relative_remap (bool) – Remap Relative, Remap relative paths when saving to a different directory (optional)"
>
> "bpy.ops.wm.save_mainfile(*, filepath='', … relative_remap=False, exit=False, …)"

Source: Python API documentation, `https://docs.blender.org/api/5.2/bpy.ops.wm.html`. Date read: 2026-07-30. (Note the differing defaults: `True` for Save As, `False` for Save.)

> "Remap Relative — This option remaps Relative Paths (such as linked libraries and images) when saving a file in a new location."
>
> "Relative Paths — Many blend-files reference external images or other linked blend-files. A path tells Blender where to look for these files. If the external files are moved, the blend-file that references them will not look right. When you specify one of these external files, the default option is to make the path relative. Blender stores a partial path evaluated relative to the directory location of the referencing blend-file."
>
> "Relative paths are the default but this can be changed in the File tab of the Preferences."
>
> "Hint — If it is necessary to relocate a blend-file relative to its linked resources, use Blender's File Save As function which has an option to Remap Relative file links."

Source: manual, `https://docs.blender.org/manual/en/5.2/files/blend/open_save.html`. Date read: 2026-07-30.

**Which kinds it reaches.** Only paths that are recorded as relative, and only when the file is saved to a different directory. Absolute paths are outside its reach by definition of what it does. The manual's examples of what it covers are "linked libraries and images".

---

### Means 8 — `bpy.data.libraries.load()` — lists the library paths recorded in *another* `.blend`

**LABEL: documented.**

> "load(filepath, *, link=False, pack=False, relative=False, …)
>
> Returns a context manager which exposes a pair of library objects (input and output) on entering.
>
> The input contains the data-blocks available in the loaded blend-file library and should be treated as read-only. …
>
> Each library object contains:
>
> Attributes matching bpy.data, which are lists of strings representing linkable data-blocks.
>
> A libraries attribute, which for the input is a list of all other libraries used by the loaded one, as named tuples (filepath, is_archive). The filepath may be absolute, or relative to the loaded blend-file. The output libraries attribute is always None.
>
> A version attribute, representing the version of the loaded library blend-file (for the input) or the version of the current Blender (for the output).
>
> Notes: Not all data-block types are linkable (e.g. WindowManager, Library, …). Packed linked data-blocks are not linkable and are not listed in the input libraries attribute."

Source: Python API documentation, `https://docs.blender.org/api/5.2/bpy.types.BlendDataLibraries.html`. Date read: 2026-07-30.

The release notes name this as new in 5.2 and show what comes back:

> "## API Improvements to `libraries.load()`
>
> Some limited library info is now exposed to the `bpy.data.libraries.load()` input context manager, giving in particular access to the blend-file paths of the libraries used by the loaded one"
>
> ```
> >>> with D.libraries.load("/home/path/to/singularity/pro/shots/050_contact/050_0070/050_0070-anim.blend") as (input, output):
> ...     for l in input.libraries:
> ...         print(l)
> ...
> bpy_lib.libraries(filepath='//../../../assets/scripts/update_bake_path.blend', is_archive=False)
> bpy_lib.libraries(filepath='//../../../assets/nodes/star_backdrop.blend', is_archive=False)
> …
> ```

Source: release notes, `https://developer.blender.org/docs/release_notes/5.2/python_api/`. Date read: 2026-07-30.

**Which kinds it reaches.** Library paths only — "the blend-file paths of the libraries used by the loaded one" — and only for reading: "The input … should be treated as read-only." The quoted words also make plain that the other file is loaded as a library, not merely inspected from outside. That this is the only means here reaching a file other than the open one is my comparison across the ten: **inferred**. Packed linked data-blocks are excluded, in so many words.

---

### Means 9 — the path field in the interface, per location

**LABEL: documented.**

> "You know that a data is packed when you see a little 'gift box' icon displayed next to its path."
>
> "Selective Packing — A single file can be packed by clicking on the little 'gift box' icon to the left of its file-path UI widget."

Source: manual, `https://docs.blender.org/manual/en/5.2/files/blend/packed_data.html`. Date read: 2026-07-30.

> "When relative paths are supported, the File Browser provides a Relative Path checkbox, when entering the path into a text field, use a double slash prefix (//) to make it so."

Source: manual, `https://docs.blender.org/manual/en/5.2/files/blend/open_save.html`. Date read: 2026-07-30.

The Outliner offers a mode that lists properties wholesale:

> "Data API: Lists every data-block in the file along with any properties that it might have."

Source: manual, `https://docs.blender.org/manual/en/5.2/editors/outliner/interface.html`. Date read: 2026-07-30.

**Which kinds it reaches.** One path at a time, wherever a path widget is shown. That the Data API mode therefore lists path properties too is not stated in those words — the manual says "any properties", and reading paths into that is a step I am taking, so treat that specific point as **inferred**.

---

### Means 10 — the `-o` / `--render-output` command-line argument

**LABEL: documented.**

> "-o, --render-output <path>
>
> Set the render path and file name.
>
> Use // at the start of the path to render relative to the blend-file.
>
> You can use path templating features such as {blend_name} in the path. See Blender's documentation on path templates for more details.
>
> The # characters are replaced by the frame number, and used to define zero padding."

And, on ordering:

> "blender --background --render-output /tmp test.blend --render-frame 1
>
> …will not render to /tmp because loading the blend-file overwrites the render output that was set."

Source: manual, `https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html`. Date read: 2026-07-30.

**Which kinds it reaches.** Exactly one recorded location: the render output path. It sets, it does not list. The quoted ordering rule states that loading a `.blend` overwrites what the argument set, so the argument must come after the file on the command line.

---

### One thing that changes what a listed location *means*

**LABEL: documented.** A recorded location may hold a template rather than a finished path, and reading it back gives you the template:

> "Path templates substitute template expressions (written {abcd}, where 'abcd' is a variable name) in a filepath when the path is used."
>
> "This substitution happens internally at time of use (e.g. while rendering), so the template syntax will remain as-is in the path field."
>
> "Note — Currently path templates are only supported for the render output path in Scene properties and the output paths in the compositor's File Output node. More filepaths will support templates in future releases."

Source: manual, `https://docs.blender.org/manual/en/5.2/files/file_paths.html`. Date read: 2026-07-30.

Means 1 has flags that bear directly on this — `RESOLVE_TOKEN` and `EXPAND_TOKENS`, both described as "Currently only used for UDIM tiles" — but the manual's template feature and those flags are described separately, and the sources do not join them up.

---

### Ruled out — things that look like a means but are not

- **`bpy.path.abspath()`, `bpy.path.relpath()`, `bpy.path.native_pathsep()`, `bpy.path.resolve_ncase()`** — string helpers. "Returns the absolute path relative to the current blend file using the '//' prefix", "Returns the path relative to the current blend file using the '//' prefix", "Replace the path separator with the system's native os.sep", "Resolve a case insensitive path on a case sensitive system, returning a string with the path if found else return the original path." Each is documented as returning a string; none is documented as reading from or writing to what a `.blend` records. (Python API documentation, `https://docs.blender.org/api/5.2/bpy.path.html`, read 2026-07-30.)
- **`bpy_extras.io_utils.path_reference()` / `path_reference_copy()`** — "Return a filepath relative to a destination directory, for use with exporters" and "Execute copying files of path_reference". These serve export, not the recorded locations. (Python API documentation, `https://docs.blender.org/api/5.2/bpy_extras.io_utils.html`, read 2026-07-30.)
- **Packing and unpacking** (`bpy.ops.file.pack_all`, `unpack_all`, `unpack_item`, `pack_libraries`, `unpack_libraries`, and Automatically Pack Resources) change whether a location is used at all, and the unpack options are described in terms of paths ("Use files in original location", "Create <local file path>"), but the manual presents them as packing operations, not as listing or rewriting. Named here only so it is clear they were considered. (Manual, `https://docs.blender.org/manual/en/5.2/files/blend/packed_data.html`, read 2026-07-30.)
- **Per-type load operators** — `bpy.ops.image.open`, `bpy.ops.image.replace` ("Replace current image by another one from disk"), `bpy.ops.clip.open`, `bpy.ops.cachefile.open`, and their kin all take a `filepath` and a `relative_path` argument. They set a location as a side effect of loading, one data-block at a time, and are a narrower restatement of Means 4. (Python API documentation, `https://docs.blender.org/api/5.2/bpy.ops.image.html`, read 2026-07-30.)

---

### NOT COVERED

- **The exact reach of Means 5 is not settled by the sources.** "all paths to external files" is not broken down. Whether Make Paths Relative / Absolute, Report Missing Files and Find Missing Files touch the render output path, the compositor's File Output node paths, point caches, simulation bake directories, sequencer strip directories or OSL script paths is not stated anywhere I searched.
- **Whether Means 1 and 2 reach the render output path is not settled.** They are documented as covering "file paths used by all ID data-blocks"; a Scene is an ID and carries `RenderSettings.filepath`, but the documentation does not say whether a write target counts as a "file path used by" the data-block. I am not willing to call this either way.
- **Nothing in the sources compares these means against each other**, or says which one a script should prefer. Where I have said one is wider than another, that is a comparison of the quoted wording only.
- **None of the ten is described as reading a `.blend` on disk without Blender running.** Means 8 does reach a file other than the open one, but the sources describe it as loading that file as a library from inside Blender. Whether anything at all outside Blender can read these locations is not addressed anywhere I searched.
- **Nothing here settles the order in which paths are visited**, whether the same path can be visited twice, or what happens if a rewrite produces an invalid path.
- **Cycles' own files contributed nothing to this answer.** I searched them and found no path-listing or path-rewriting facility (see the record below).

---

### Outside the interface

**Brought up to date by the hand tests of 2026-08-06 and 2026-08-07 (see the two observed sections below).** Every means named in this answer that has a Python form was run with no window at all, on the rented Linux machine, in this version. The two bulk sweeps, the missing-file search, the library relocation, saving elsewhere with the remap option, and the write call all returned `{'FINISHED'}` and did what they claim, with no interface present. Two operators did **not** on that machine: `bpy.ops.image.replace` and `bpy.ops.sequencer.change_path` both refused, naming a failed context check. The Mac sitting of 2026-08-07 took those two further, and they part company. **`bpy.ops.image.replace` needs a window**: with one on screen it returned `{'FINISHED'}` and rewrote the recorded path; with no window it refused on the Mac too, in every version. **`bpy.ops.sequencer.change_path` does not need a window**: with no window at all, given the file's name and the sequencer's own scene in the context, it returned `{'FINISHED'}` and rewrote the recorded folder and file, in every version. So this line is answered for everything here except that one operator, and that one is answered the other way. The words below are the researcher's own and are left as they were written.


Mixed, and only partly documented.

- **Means 1 is documented as intended for use outside the interface.** The release note says the new options exist "so render farms and other scripts can find all relevant image files for rendering" (`https://developer.blender.org/docs/release_notes/5.2/python_api/`, read 2026-07-30). That is an explicit statement about scripted, non-interactive use. **LABEL: documented.**
- **Means 10 is a command-line argument**, so it exists only outside the interface. The manual's own example uses `blender --background`. **LABEL: documented.**
- **Means 2, 3, 4 and 8 are plain Python calls on `bpy.data`, `bpy.utils` and data-blocks.** The documentation describes no interface involvement in any of them. Whether they behave identically with Blender started in background is not stated. **LABEL: not in the sources.**
- **Means 5 and 6 are operators, and some of them are described as opening a window.** The manual says of Find Missing Files: "A File Browser will show up." Both `file.find_missing_files` and `wm.lib_relocate` accept a `directory` (and `filepath`, `files`) argument, which is what a file browser would otherwise fill in; a caller supplying those arguments directly would not need the browser. That last point is a step I am taking from the argument list, not something the sources state — **LABEL: inferred** for that step, documented for the argument list itself. Whether these operators succeed at all with no window is **not in the sources**.
- **Means 7 depends on saving**, which the command line can do; the sources do not discuss it outside the interface. **LABEL: not in the sources.**
- **Means 9 is the interface**, and has no counterpart outside it.

An important documented consequence for anything driven from outside: "loading the blend-file overwrites the render output that was set" (manual, `https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html`, read 2026-07-30). Order of arguments matters.

### Operating system

**Brought up to date by the hand tests of 2026-08-06 and 2026-08-07 (see the two observed sections below).** Everything in this answer that could be run was run on the rented Linux machine, so **Linux is settled** for it. **macOS is now answered for the tests the Mac sitting ran**, and they are the ones this line turns on: the same file, built the same way, lists the same paths on the two machines — twenty-five of the twenty-seven printed lines are identical character for character. Two are not. The flat path list carries one extra entry on the Mac, a path into Blender's own application folder. And **whether capital letters in a name matter is answered differently on the two machines**: asked for its own image file with the name written in capitals, the Mac found it and the Linux machine did not. *(Citation check, 2026-08-06: narrowed. That check asks the operating system directly — `os.path.exists` on a name written in capitals — with none of Blender's own path machinery in it. It settles that the two file systems differ on letter case; it settles nothing about the means in this answer.)* Note what is still only Linux: the entries this round marked as running on both machines were in fact run on Linux alone, so for those the macOS half is untested rather than settled. **Windows stays untouched**: no Windows machine has run it, so the cross-drive question the sources raise was not answered by any test here. The words below are the researcher's own and are left as they were written.


**LABEL: not in the sources**, for every one of the ten means. Nothing in the manual, the API documentation, the release notes or Cycles' files says any of these behaves differently on one operating system than another.

Two nearby facts that are documented, and that bear on paths generally rather than on any of the ten means:

> "native_pathsep(path) — Replace the path separator with the system's native os.sep."
>
> "resolve_ncase(path) — Resolve a case insensitive path on a case sensitive system, returning a string with the path if found else return the original path."

Source: Python API documentation, `https://docs.blender.org/api/5.2/bpy.path.html`. Date read: 2026-07-30. The existence of both is a documented acknowledgement that separators and case sensitivity differ between systems, but neither says what any of the ten means does about it.

The requirements page names the operating systems Blender is offered for — Windows, macOS and Linux, with "Blender is cross-platform, it runs on every major operating system" — but it names Blender 4.5 LTS, 4.1 and "Blender 5.0 and later" rather than 5.2 in so many words, and it says nothing about file paths. (`https://www.blender.org/download/requirements/`, read 2026-08-01.)

---

### Record of where I looked

**Searched across the whole of `manual_5.2`, `api_5.2`, `release_notes_5.2` and `cycles_5.2` at once**, for these terms:

`blend_paths`, `file_path_foreach`, `file_path_map`, `foreach_path`, `path_reference`, `find_missing_files` / "Find Missing Files", `report_missing_files` / "Report Missing", `make_paths_relative` / "Make Paths Relative", `make_paths_absolute` / "Make Paths Absolute", "External Data", "Relocate", `lib_relocate`, `lib_reload`, `relative_remap` / "Remap Relative", "external files", `is_missing`, `blend_render_info`, `--command`, `FILE_PATH`, `DIR_PATH`, `texture_cache`, `StringProperty`, and `^filepath` as a property heading across all 2,191 API pages.

**Pages opened in full or in part:**

- API: `bpy.types.BlendData.txt` (both `file_path_foreach` and `file_path_map`), `bpy.types.BlendDataPathMeta.txt`, `bpy.types.BlendDataLibraries.txt`, `bpy.types.Library.txt`, `bpy.types.Image.txt`, `bpy.types.ID.txt`, `bpy.types.KeyingSetPath.txt`, `bpy.utils.txt`, `bpy.path.txt`, `bpy_extras.io_utils.txt`, `bpy.ops.file.txt`, `bpy.ops.wm.txt`, `bpy.ops.outliner.txt`, `bpy.ops.image.txt`, `bpy.ops.clip.txt`, `bpy.ops.cachefile.txt`, `bpy_types_enum_items/file_path_foreach_flag_items.txt`, `bpy_types_enum_items/id_type_items.txt`.
- Manual: `interface/window_system/topbar.txt`, `files/blend/packed_data.txt`, `files/blend/open_save.txt`, `files/file_paths.txt`, `files/linked_libraries/link_append.txt`, `files/linked_libraries/index.txt`, `editors/outliner/interface.txt`, `advanced/command_line/arguments.txt`, `advanced/command_line/extension_arguments.txt`.
- Release notes: `python_api.md`, `bugfixes.md`, `cycles.md`, plus a grep for "path" across all twenty subject files.
- Cycles: `properties.py`, `operators.py`, `maketx.py` (the whole file), and a directory listing of `cycles_5.2`.
- Requirements: `requirements_2026-08-01.txt`, read in full at the top and searched for version names.

**Pictures.** I checked the three pages that carry the relevant menus for images. `topbar.txt` references only `_images/interface_window-system_topbar_menus.png`, `_images/interface_window-system_topbar_workspaces.png` and `_images/interface_window-system_topbar_scenes-layers.png` — none of them a picture of the External Data submenu. `packed_data.txt` and `link_append.txt` reference no images at all. `files/file_paths.txt` references two pictures of an invalid path template, which do not bear on this question. So no picture was available to settle anything here.

**Ruled out and why:**

- `blend_render_info` — searched for across the manual, API and release notes; **no hits at all**. There is no documented standalone reader of this kind in the sources.
- `blender --command` subcommands — the only ones documented are the extension commands (`extension list`, `sync`, `update`, `install`, `install-file`, `remove`, `repo-list`, `repo-add`, `repo-remove`, `build`, `validate`, `server-generate`) plus Cycles' `maketx`. None lists or rewrites recorded locations. `maketx` is a Cycles command whose only path argument is `--cache-dir`, "Output directory for tx files, relative to source image or absolute" (`cycles_5.2/maketx.py`, the `--cache-dir` argument in `argparse_create()`, read 2026-07-30). Reading the file shows that the argument exists and what its help text says; what the command does when it runs is a step beyond reading the file, so any claim about that is **inferred**. Nothing in the file describes a facility for listing or rewriting recorded locations.
- Cycles' `properties.py` — searched for `StringProperty`, `FILE_PATH` and `DIR_PATH`. The only `StringProperty` entries are `id` and `name` on a device, and there are no path-typed properties at all. Cycles' `operators.py` has five `subtype='FILE_PATH'` properties (lines 41, 47, 123, 129, 135), which belong to its own operators rather than to any general path facility.
- Command-line arguments other than `-o` — searched `arguments.txt` for "path" and "relative". The other hits are `-P/--python <filepath>`, `--log-file <filepath>`, the splash-screen images, the OpenColorIO configuration override, and the 3D-mouse socket path. None reads or rewrites the locations a `.blend` records.
- The Outliner's other display modes — `editors/outliner/interface.txt` describes "Blender File: Lists all data in the current blend-file", with no mention of paths; "Library Overrides"; and "Unused Data". Only the Data API mode is described as listing properties.

---

### Observed — the hand tests of 2026-08-06

Everything in this section was run on **the rented Linux machine — Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB on driver 580.95.05, an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory and no display server at all**. The Blender was `/opt/blender/5.2/blender`, which reported itself as **5.2.0 LTS**, hash `fbe6228777e7`. Every run used `-b` (no window) and `--factory-startup`, and every run had `BLENDER_USER_RESOURCES` pointed at `/root/reading-a-file/res-5.2`, a settings folder made for this round and for nothing else — never at the real one. All work was done inside `/root/reading-a-file/`. The machine is shared with other topics' tests, so nothing here says how long anything took. Its graphics card could not be questioned during this sitting (`nvidia-smi` answered `Failed to initialize NVML: Driver/library version mismatch`); no test below touches the card. The full list of tests, with the set-up and the command for each, is `Audits/Blender Research/Reading a file/Hand tests — added questions.md`. **Nothing below was run on the Creator's Mac**, so nothing below settles macOS; a second sitting does that half.

#### What the machinery is called here, and what its switches are (list entry 1)

Two scripts were run with no file loaded, asking whether each named call exists and what it accepts. What came back, word for word:

```
PROBE version 5.2.0 LTS
PROBE has bpy.utils.blend_paths = True
PROBE has bpy.data.file_path_map = True
PROBE has bpy.data.file_path_foreach = True
PROBE has bpy.data.user_map = True
PROBE has bpy.data.all_ids = True
PROBE blend_paths positional = ERROR TypeError blend_paths() takes no positional arguments
PROBE Property.is_path_output = True
PROBE Property.is_path_supports_blend_relative = True
PROBE Property.is_path_supports_templates = True
PROBE Property.subtype = True
PROBE file_path_foreach flags = ERROR KeyError 'bpy_prop_collection[key]: key "file_path_foreach" not found'
PROBE file_path_map params = ERROR KeyError 'bpy_prop_collection[key]: key "file_path_map" not found'
PROBE bpy.types.BlendDataPathMeta = True
PROBE2 version 5.2.0 LTS
PROBE2 file_path_foreach flag SKIP_LINKED = ACCEPTED
PROBE2 file_path_foreach flag SKIP_PACKED = ACCEPTED
PROBE2 file_path_foreach flag SKIP_WEAK_REFERENCES = ACCEPTED
PROBE2 file_path_foreach flag SKIP_MULTIFILE = ACCEPTED
PROBE2 file_path_foreach flag RELOAD_EDITED = ACCEPTED
PROBE2 file_path_foreach flag RESOLVE_TOKEN = ACCEPTED
PROBE2 file_path_foreach flag EXPAND_TOKENS = ACCEPTED
PROBE2 file_path_foreach flag EXPAND_SEQUENCES = ACCEPTED
PROBE2 file_path_foreach flag EXPAND_CACHES = ACCEPTED
PROBE2 callback arg types = []
PROBE2 path_remap 'NONE' = ACCEPTED
PROBE2 path_remap 'RELATIVE' = ACCEPTED
PROBE2 path_remap 'RELATIVE_ALL' = ACCEPTED
PROBE2 path_remap 'ABSOLUTE' = ACCEPTED
PROBE2 path_remap False = REJECTED ValueError expected a string, got bool
PROBE2 CompositorNodeOutputFile base_path = False directory = True
PROBE2 Camera.custom_filepath = True
PROBE2 Text.filepath = True
PROBE2 TextureNodeOutput.filepath = True
PROBE2 Theme.filepath = True
PROBE2 Volume.filepath = True
PROBE2 NodesModifier.bake_directory = True
PROBE2 RenderSettings.filepath is_path_output = True blend_relative = True templates = True subtype = FILE_PATH
PROBE2 Text.filepath is_path_output = False blend_relative = False subtype = NONE
PROBE2 Camera.custom_filepath is_path_output = False blend_relative = False subtype = FILE_PATH
```

#### The file the next five tests were run on (list entries 3 to 8)

A file was built in a fresh empty folder holding **twenty-one recorded locations, every one written out in full from the drive root**, all pointing into an `assets` folder beside it: an image, a sound, a font, a text file, a volume, a movie clip, an Alembic cache file, a linked library, the render output path, the bake output path, the compositor File Output node's base path, a cloth point cache, a fluid domain cache folder, a geometry-nodes bake folder, a mesh-cache modifier file, an ocean modifier folder, an OSL script node, an IES texture node, a camera's own shader path, a sequencer image strip's folder with the sequencer's proxy folder, and a custom property on an object holding a path-shaped string. Two limits on it, recorded because they bound what follows: the multi-resolution modifier's path would not take a value — the assignment was accepted without error and the property read back empty every time — so that kind is **not** in the file; and three of the files the paths name were never created, on purpose, since what is being asked is which paths the machinery reaches and not whether the file behind them opens.

#### Which kinds each listing means names (list entry 3)

The file was opened and every kind read back by name, then each listing means called. What came back:

```
READ | Image.filepath                           | /root/reading-a-file/work/many/5.2/assets/probe.png
READ | Image.filepath_raw                       | /root/reading-a-file/work/many/5.2/assets/probe.png
READ | Sound.filepath                           | /root/reading-a-file/work/many/5.2/assets/probe.wav
READ | VectorFont.filepath                      | /root/reading-a-file/work/many/5.2/assets/probe.woff2
READ | Text.filepath                            | /root/reading-a-file/work/many/5.2/assets/probe.txt
READ | Volume.filepath                          | /root/reading-a-file/work/many/5.2/assets/probe.vdb
READ | MovieClip.filepath                       | /root/reading-a-file/work/many/5.2/assets/probe.png
READ | CacheFile.filepath                       | /root/reading-a-file/work/many/5.2/assets/probe.abc
READ | Library.filepath                         | /root/reading-a-file/work/many/5.2/lib.blend
READ | RenderSettings.filepath                  | /root/reading-a-file/work/many/5.2/assets/render_out/
READ | BakeSettings.filepath                    | /root/reading-a-file/work/many/5.2/assets/bake_out/
READ | CompositorNodeOutputFile base path       | /root/reading-a-file/work/many/5.2/assets/node_out/
READ | PointCache.filepath (cloth)              | /root/reading-a-file/work/many/5.2/assets/pointcache/
READ | FluidDomainSettings.cache_directory      | /root/reading-a-file/work/many/5.2/assets/fluidcache/
READ | NodesModifier.bake_directory             | /root/reading-a-file/work/many/5.2/assets/gnbake/
READ | MultiresModifier.filepath                | 
READ | MeshCacheModifier.filepath               | /root/reading-a-file/work/many/5.2/assets/meshcache.mdd
READ | OceanModifier.filepath                   | /root/reading-a-file/work/many/5.2/assets/oceanbake/
READ | ShaderNodeScript.filepath                | /root/reading-a-file/work/many/5.2/assets/probe.osl
READ | ShaderNodeTexIES.filepath                | /root/reading-a-file/work/many/5.2/assets/probe.ies
READ | Camera.custom_filepath                   | /root/reading-a-file/work/many/5.2/assets/camshader.osl
READ | custom property on an object             | /root/reading-a-file/work/many/5.2/assets/custom_prop.png
READ | ImageStrip.directory                     | /root/reading-a-file/work/many/5.2/assets/stripdir/
READ | StripProxy.directory (sequencer)         | /root/reading-a-file/work/many/5.2/assets/proxydir/
```

And the listing means:

```
LIST blend_paths()                 = ['/opt/blender/5.2/5.2/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/root/reading-a-file/work/many/5.2/assets/camshader.osl', '/root/reading-a-file/work/many/5.2/assets/fluidcache/', '/root/reading-a-file/work/many/5.2/assets/meshcache.mdd', '/root/reading-a-file/work/many/5.2/assets/oceanbake/', '/root/reading-a-file/work/many/5.2/assets/probe.abc', '/root/reading-a-file/work/many/5.2/assets/probe.ies', '/root/reading-a-file/work/many/5.2/assets/probe.osl', '/root/reading-a-file/work/many/5.2/assets/probe.png', '/root/reading-a-file/work/many/5.2/assets/probe.png', '/root/reading-a-file/work/many/5.2/assets/probe.txt', '/root/reading-a-file/work/many/5.2/assets/probe.vdb', '/root/reading-a-file/work/many/5.2/assets/probe.wav', '/root/reading-a-file/work/many/5.2/assets/probe.woff2', '/root/reading-a-file/work/many/5.2/assets/stripdir/probe.png', '/root/reading-a-file/work/many/5.2/lib.blend']
LIST blend_paths(packed=True)      = ['/opt/blender/5.2/5.2/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/root/reading-a-file/work/many/5.2/assets/camshader.osl', '/root/reading-a-file/work/many/5.2/assets/fluidcache/', '/root/reading-a-file/work/many/5.2/assets/meshcache.mdd', '/root/reading-a-file/work/many/5.2/assets/oceanbake/', '/root/reading-a-file/work/many/5.2/assets/probe.abc', '/root/reading-a-file/work/many/5.2/assets/probe.ies', '/root/reading-a-file/work/many/5.2/assets/probe.osl', '/root/reading-a-file/work/many/5.2/assets/probe.png', '/root/reading-a-file/work/many/5.2/assets/probe.png', '/root/reading-a-file/work/many/5.2/assets/probe.txt', '/root/reading-a-file/work/many/5.2/assets/probe.vdb', '/root/reading-a-file/work/many/5.2/assets/probe.wav', '/root/reading-a-file/work/many/5.2/assets/probe.woff2', '/root/reading-a-file/work/many/5.2/assets/stripdir/probe.png', '/root/reading-a-file/work/many/5.2/lib.blend']
LIST blend_paths(local=True)       = ['/opt/blender/5.2/5.2/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/root/reading-a-file/work/many/5.2/assets/camshader.osl', '/root/reading-a-file/work/many/5.2/assets/fluidcache/', '/root/reading-a-file/work/many/5.2/assets/meshcache.mdd', '/root/reading-a-file/work/many/5.2/assets/oceanbake/', '/root/reading-a-file/work/many/5.2/assets/probe.abc', '/root/reading-a-file/work/many/5.2/assets/probe.ies', '/root/reading-a-file/work/many/5.2/assets/probe.osl', '/root/reading-a-file/work/many/5.2/assets/probe.png', '/root/reading-a-file/work/many/5.2/assets/probe.png', '/root/reading-a-file/work/many/5.2/assets/probe.txt', '/root/reading-a-file/work/many/5.2/assets/probe.vdb', '/root/reading-a-file/work/many/5.2/assets/probe.wav', '/root/reading-a-file/work/many/5.2/assets/probe.woff2', '/root/reading-a-file/work/many/5.2/assets/stripdir/probe.png', '/root/reading-a-file/work/many/5.2/lib.blend']
LIST blend_paths(absolute=True)    = ['/opt/blender/5.2/5.2/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/root/reading-a-file/work/many/5.2/assets/camshader.osl', '/root/reading-a-file/work/many/5.2/assets/fluidcache/', '/root/reading-a-file/work/many/5.2/assets/meshcache.mdd', '/root/reading-a-file/work/many/5.2/assets/oceanbake/', '/root/reading-a-file/work/many/5.2/assets/probe.abc', '/root/reading-a-file/work/many/5.2/assets/probe.ies', '/root/reading-a-file/work/many/5.2/assets/probe.osl', '/root/reading-a-file/work/many/5.2/assets/probe.png', '/root/reading-a-file/work/many/5.2/assets/probe.png', '/root/reading-a-file/work/many/5.2/assets/probe.txt', '/root/reading-a-file/work/many/5.2/assets/probe.vdb', '/root/reading-a-file/work/many/5.2/assets/probe.wav', '/root/reading-a-file/work/many/5.2/assets/probe.woff2', '/root/reading-a-file/work/many/5.2/assets/stripdir/probe.png', '/root/reading-a-file/work/many/5.2/lib.blend']
LIST file_path_map()               = ['/root/reading-a-file/work/many/5.2/assets/camshader.osl', '/root/reading-a-file/work/many/5.2/assets/fluidcache/', '/root/reading-a-file/work/many/5.2/assets/meshcache.mdd', '/root/reading-a-file/work/many/5.2/assets/oceanbake/', '/root/reading-a-file/work/many/5.2/assets/probe.abc', '/root/reading-a-file/work/many/5.2/assets/probe.ies', '/root/reading-a-file/work/many/5.2/assets/probe.osl', '/root/reading-a-file/work/many/5.2/assets/probe.png', '/root/reading-a-file/work/many/5.2/assets/probe.txt', '/root/reading-a-file/work/many/5.2/assets/probe.vdb', '/root/reading-a-file/work/many/5.2/assets/probe.wav', '/root/reading-a-file/work/many/5.2/assets/probe.woff2', '/root/reading-a-file/work/many/5.2/assets/stripdir/probe.png', '/root/reading-a-file/work/many/5.2/lib.blend']
LIST file_path_map(include_libraries=True) = ['/root/reading-a-file/work/many/5.2/assets/camshader.osl', '/root/reading-a-file/work/many/5.2/assets/fluidcache/', '/root/reading-a-file/work/many/5.2/assets/meshcache.mdd', '/root/reading-a-file/work/many/5.2/assets/oceanbake/', '/root/reading-a-file/work/many/5.2/assets/probe.abc', '/root/reading-a-file/work/many/5.2/assets/probe.ies', '/root/reading-a-file/work/many/5.2/assets/probe.osl', '/root/reading-a-file/work/many/5.2/assets/probe.png', '/root/reading-a-file/work/many/5.2/assets/probe.txt', '/root/reading-a-file/work/many/5.2/assets/probe.vdb', '/root/reading-a-file/work/many/5.2/assets/probe.wav', '/root/reading-a-file/work/many/5.2/assets/probe.woff2', '/root/reading-a-file/work/many/5.2/assets/stripdir/probe.png', '/root/reading-a-file/work/many/5.2/lib.blend']
LIST file_path_map by data-block   = [('FluidObj', ['/root/reading-a-file/work/many/5.2/assets/fluidcache/']), ('LibColl', ['/root/reading-a-file/work/many/5.2/lib.blend']), ('LibCube', ['/root/reading-a-file/work/many/5.2/lib.blend']), ('LibMesh', ['/root/reading-a-file/work/many/5.2/lib.blend']), ('ModObj', ['/root/reading-a-file/work/many/5.2/assets/meshcache.mdd', '/root/reading-a-file/work/many/5.2/assets/oceanbake/']), ('ProbeCam', ['/root/reading-a-file/work/many/5.2/assets/camshader.osl']), ('ProbeClip', ['/root/reading-a-file/work/many/5.2/assets/probe.png']), ('ProbeFont', ['/root/reading-a-file/work/many/5.2/assets/probe.woff2']), ('ProbeImage', ['/root/reading-a-file/work/many/5.2/assets/probe.png']), ('ProbeMat', ['/root/reading-a-file/work/many/5.2/assets/probe.ies', '/root/reading-a-file/work/many/5.2/assets/probe.osl']), ('ProbeSound', ['/root/reading-a-file/work/many/5.2/assets/probe.wav']), ('ProbeText', ['/root/reading-a-file/work/many/5.2/assets/probe.txt']), ('ProbeVolume', ['/root/reading-a-file/work/many/5.2/assets/probe.vdb']), ('Scene', ['/root/reading-a-file/work/many/5.2/assets/stripdir/probe.png']), ('lib.blend', ['/root/reading-a-file/work/many/5.2/lib.blend']), ('probe.abc', ['/root/reading-a-file/work/many/5.2/assets/probe.abc'])]
LIST foreach[default] CacheFile 'probe.abc' /root/reading-a-file/work/many/5.2/assets/probe.abc ro=False cache=False exp=False
LIST foreach[default] Camera 'ProbeCam' /root/reading-a-file/work/many/5.2/assets/camshader.osl ro=False cache=False exp=False
LIST foreach[default] Image 'ProbeImage' /root/reading-a-file/work/many/5.2/assets/probe.png ro=False cache=False exp=False
LIST foreach[default] Library 'lib.blend' /root/reading-a-file/work/many/5.2/lib.blend ro=False cache=False exp=False
LIST foreach[default] MovieClip 'ProbeClip' /root/reading-a-file/work/many/5.2/assets/probe.png ro=False cache=False exp=False
LIST foreach[default] Object 'FluidObj' /root/reading-a-file/work/many/5.2/assets/fluidcache/ ro=False cache=False exp=False
LIST foreach[default] Object 'ModObj' /root/reading-a-file/work/many/5.2/assets/meshcache.mdd ro=False cache=False exp=False
LIST foreach[default] Object 'ModObj' /root/reading-a-file/work/many/5.2/assets/oceanbake/ ro=False cache=False exp=False
LIST foreach[default] Scene 'Scene' /root/reading-a-file/work/many/5.2/assets/stripdir/probe.png ro=False cache=False exp=False
LIST foreach[default] ShaderNodeTree 'Shader Nodetree' /root/reading-a-file/work/many/5.2/assets/probe.ies ro=False cache=False exp=False
LIST foreach[default] ShaderNodeTree 'Shader Nodetree' /root/reading-a-file/work/many/5.2/assets/probe.osl ro=False cache=False exp=False
LIST foreach[default] Sound 'ProbeSound' /root/reading-a-file/work/many/5.2/assets/probe.wav ro=False cache=False exp=False
LIST foreach[default] Text 'ProbeText' /root/reading-a-file/work/many/5.2/assets/probe.txt ro=False cache=False exp=False
LIST foreach[default] VectorFont 'ProbeFont' /root/reading-a-file/work/many/5.2/assets/probe.woff2 ro=False cache=False exp=False
LIST foreach[default] Volume 'ProbeVolume' /root/reading-a-file/work/many/5.2/assets/probe.vdb ro=False cache=False exp=False
LIST foreach[default] COUNT 15
LIST foreach[flags=set()] CacheFile 'probe.abc' /root/reading-a-file/work/many/5.2/assets/probe.abc ro=False cache=False exp=False
LIST foreach[flags=set()] Camera 'ProbeCam' /root/reading-a-file/work/many/5.2/assets/camshader.osl ro=False cache=False exp=False
LIST foreach[flags=set()] Image 'ProbeImage' /root/reading-a-file/work/many/5.2/assets/probe.png ro=False cache=False exp=False
LIST foreach[flags=set()] Library 'lib.blend' /root/reading-a-file/work/many/5.2/lib.blend ro=False cache=False exp=False
LIST foreach[flags=set()] Material 'Material' /opt/blender/5.2/5.2/datafiles/assets/brushes/essentials_brushes-gp_draw.blend ro=False cache=False exp=False
LIST foreach[flags=set()] MovieClip 'ProbeClip' /root/reading-a-file/work/many/5.2/assets/probe.png ro=False cache=False exp=False
LIST foreach[flags=set()] Object 'FluidObj' /root/reading-a-file/work/many/5.2/assets/fluidcache/ ro=False cache=False exp=False
LIST foreach[flags=set()] Object 'ModObj' /root/reading-a-file/work/many/5.2/assets/meshcache.mdd ro=False cache=False exp=False
LIST foreach[flags=set()] Object 'ModObj' /root/reading-a-file/work/many/5.2/assets/oceanbake/ ro=False cache=False exp=False
LIST foreach[flags=set()] Scene 'Scene' /root/reading-a-file/work/many/5.2/assets/stripdir/probe.png ro=False cache=False exp=False
LIST foreach[flags=set()] ShaderNodeTree 'Shader Nodetree' /root/reading-a-file/work/many/5.2/assets/probe.ies ro=False cache=False exp=False
LIST foreach[flags=set()] ShaderNodeTree 'Shader Nodetree' /root/reading-a-file/work/many/5.2/assets/probe.osl ro=False cache=False exp=False
LIST foreach[flags=set()] Sound 'ProbeSound' /root/reading-a-file/work/many/5.2/assets/probe.wav ro=False cache=False exp=False
LIST foreach[flags=set()] Text 'ProbeText' /root/reading-a-file/work/many/5.2/assets/probe.txt ro=False cache=False exp=False
LIST foreach[flags=set()] VectorFont 'ProbeFont' /root/reading-a-file/work/many/5.2/assets/probe.woff2 ro=False cache=False exp=False
LIST foreach[flags=set()] Volume 'ProbeVolume' /root/reading-a-file/work/many/5.2/assets/probe.vdb ro=False cache=False exp=False
LIST foreach[flags=set()] COUNT 16
```

#### What Make Paths Relative reaches (list entry 4)

The same file, opened again. Every kind read, then `bpy.ops.file.make_paths_relative()`, then every kind read again. The operator's own answer and the state afterwards:

```
OP make_paths_relative = {'FINISHED'}
AFTER | Image.filepath                           | //assets/probe.png
AFTER | Image.filepath_raw                       | //assets/probe.png
AFTER | Sound.filepath                           | //assets/probe.wav
AFTER | VectorFont.filepath                      | //assets/probe.woff2
AFTER | Text.filepath                            | //assets/probe.txt
AFTER | Volume.filepath                          | //assets/probe.vdb
AFTER | MovieClip.filepath                       | //assets/probe.png
AFTER | CacheFile.filepath                       | //assets/probe.abc
AFTER | Library.filepath                         | //lib.blend
AFTER | RenderSettings.filepath                  | /root/reading-a-file/work/many/5.2/assets/render_out/
AFTER | BakeSettings.filepath                    | /root/reading-a-file/work/many/5.2/assets/bake_out/
AFTER | CompositorNodeOutputFile base path       | /root/reading-a-file/work/many/5.2/assets/node_out/
AFTER | PointCache.filepath (cloth)              | /root/reading-a-file/work/many/5.2/assets/pointcache/
AFTER | FluidDomainSettings.cache_directory      | //assets/fluidcache/
AFTER | NodesModifier.bake_directory             | /root/reading-a-file/work/many/5.2/assets/gnbake/
AFTER | MultiresModifier.filepath                | 
AFTER | MeshCacheModifier.filepath               | //assets/meshcache.mdd
AFTER | OceanModifier.filepath                   | //assets/oceanbake/
AFTER | ShaderNodeScript.filepath                | //assets/probe.osl
AFTER | ShaderNodeTexIES.filepath                | //assets/probe.ies
AFTER | Camera.custom_filepath                   | //assets/camshader.osl
AFTER | custom property on an object             | /root/reading-a-file/work/many/5.2/assets/custom_prop.png
AFTER | ImageStrip.directory                     | //assets/stripdir/
AFTER | StripProxy.directory (sequencer)         | /root/reading-a-file/work/many/5.2/assets/proxydir/
```

Reading that off, without adding anything to it: the ones whose recorded text changed were **CacheFile.filepath, Camera.custom_filepath, FluidDomainSettings.cache_directory, Image.filepath, Image.filepath_raw, ImageStrip.directory, Library.filepath, MeshCacheModifier.filepath, MovieClip.filepath, OceanModifier.filepath, ShaderNodeScript.filepath, ShaderNodeTexIES.filepath, Sound.filepath, Text.filepath, VectorFont.filepath, Volume.filepath**. The ones whose recorded text is the same after as before were **BakeSettings.filepath, CompositorNodeOutputFile base path, NodesModifier.bake_directory, PointCache.filepath (cloth), RenderSettings.filepath, StripProxy.directory (sequencer), custom property on an object**. (The multi-resolution modifier's line is empty in both and is not counted either way, for the reason given above.)

#### What Make Paths Absolute reaches (list entry 5)

The file the last test saved, opened again, and `bpy.ops.file.make_paths_absolute()` called on it:

```
OP make_paths_absolute = {'FINISHED'}
AFTER | Image.filepath                           | /root/reading-a-file/work/many/5.2/assets/probe.png
AFTER | Image.filepath_raw                       | /root/reading-a-file/work/many/5.2/assets/probe.png
AFTER | Sound.filepath                           | /root/reading-a-file/work/many/5.2/assets/probe.wav
AFTER | VectorFont.filepath                      | /root/reading-a-file/work/many/5.2/assets/probe.woff2
AFTER | Text.filepath                            | /root/reading-a-file/work/many/5.2/assets/probe.txt
AFTER | Volume.filepath                          | /root/reading-a-file/work/many/5.2/assets/probe.vdb
AFTER | MovieClip.filepath                       | /root/reading-a-file/work/many/5.2/assets/probe.png
AFTER | CacheFile.filepath                       | /root/reading-a-file/work/many/5.2/assets/probe.abc
AFTER | Library.filepath                         | /root/reading-a-file/work/many/5.2/lib.blend
AFTER | RenderSettings.filepath                  | /root/reading-a-file/work/many/5.2/assets/render_out/
AFTER | BakeSettings.filepath                    | /root/reading-a-file/work/many/5.2/assets/bake_out/
AFTER | CompositorNodeOutputFile base path       | /root/reading-a-file/work/many/5.2/assets/node_out/
AFTER | PointCache.filepath (cloth)              | /root/reading-a-file/work/many/5.2/assets/pointcache/
AFTER | FluidDomainSettings.cache_directory      | /root/reading-a-file/work/many/5.2/assets/fluidcache/
AFTER | NodesModifier.bake_directory             | /root/reading-a-file/work/many/5.2/assets/gnbake/
AFTER | MultiresModifier.filepath                | 
AFTER | MeshCacheModifier.filepath               | /root/reading-a-file/work/many/5.2/assets/meshcache.mdd
AFTER | OceanModifier.filepath                   | /root/reading-a-file/work/many/5.2/assets/oceanbake/
AFTER | ShaderNodeScript.filepath                | /root/reading-a-file/work/many/5.2/assets/probe.osl
AFTER | ShaderNodeTexIES.filepath                | /root/reading-a-file/work/many/5.2/assets/probe.ies
AFTER | Camera.custom_filepath                   | /root/reading-a-file/work/many/5.2/assets/camshader.osl
AFTER | custom property on an object             | /root/reading-a-file/work/many/5.2/assets/custom_prop.png
AFTER | ImageStrip.directory                     | /root/reading-a-file/work/many/5.2/assets/stripdir/
AFTER | StripProxy.directory (sequencer)         | /root/reading-a-file/work/many/5.2/assets/proxydir/
```

#### The one-pass path visitor, and whether returning a string rewrites (list entry 6)

The same file, with `bpy.data.file_path_foreach` called with its flags emptied and a function that returns each path with `/assets/` replaced by `/MOVED/`. What was visited, and what the twenty-one kinds read afterwards:

```
FEW returned = None
FEW visited ('CacheFile', 'probe.abc', '/root/reading-a-file/work/many/5.2/assets/probe.abc', '/root/reading-a-file/work/many/5.2/MOVED/probe.abc', False)
FEW visited ('Camera', 'ProbeCam', '/root/reading-a-file/work/many/5.2/assets/camshader.osl', '/root/reading-a-file/work/many/5.2/MOVED/camshader.osl', False)
FEW visited ('Image', 'ProbeImage', '/root/reading-a-file/work/many/5.2/assets/probe.png', '/root/reading-a-file/work/many/5.2/MOVED/probe.png', False)
FEW visited ('Library', 'lib.blend', '/root/reading-a-file/work/many/5.2/lib.blend', '/root/reading-a-file/work/many/5.2/lib.blend', False)
FEW visited ('Material', 'Material', '/opt/blender/5.2/5.2/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/opt/blender/5.2/5.2/datafiles/MOVED/brushes/essentials_brushes-gp_draw.blend', False)
FEW visited ('MovieClip', 'ProbeClip', '/root/reading-a-file/work/many/5.2/assets/probe.png', '/root/reading-a-file/work/many/5.2/MOVED/probe.png', False)
FEW visited ('Object', 'FluidObj', '/root/reading-a-file/work/many/5.2/assets/fluidcache/', '/root/reading-a-file/work/many/5.2/MOVED/fluidcache/', False)
FEW visited ('Object', 'ModObj', '/root/reading-a-file/work/many/5.2/assets/meshcache.mdd', '/root/reading-a-file/work/many/5.2/MOVED/meshcache.mdd', False)
FEW visited ('Object', 'ModObj', '/root/reading-a-file/work/many/5.2/assets/oceanbake/', '/root/reading-a-file/work/many/5.2/MOVED/oceanbake/', False)
FEW visited ('Scene', 'Scene', '/root/reading-a-file/work/many/5.2/assets/stripdir/probe.png', '/root/reading-a-file/work/many/5.2/MOVED/stripdir/probe.png', False)
FEW visited ('ShaderNodeTree', 'Shader Nodetree', '/root/reading-a-file/work/many/5.2/assets/probe.ies', '/root/reading-a-file/work/many/5.2/MOVED/probe.ies', False)
FEW visited ('ShaderNodeTree', 'Shader Nodetree', '/root/reading-a-file/work/many/5.2/assets/probe.osl', '/root/reading-a-file/work/many/5.2/MOVED/probe.osl', False)
FEW visited ('Sound', 'ProbeSound', '/root/reading-a-file/work/many/5.2/assets/probe.wav', '/root/reading-a-file/work/many/5.2/MOVED/probe.wav', False)
FEW visited ('Text', 'ProbeText', '/root/reading-a-file/work/many/5.2/assets/probe.txt', '/root/reading-a-file/work/many/5.2/MOVED/probe.txt', False)
FEW visited ('VectorFont', 'ProbeFont', '/root/reading-a-file/work/many/5.2/assets/probe.woff2', '/root/reading-a-file/work/many/5.2/MOVED/probe.woff2', False)
FEW visited ('Volume', 'ProbeVolume', '/root/reading-a-file/work/many/5.2/assets/probe.vdb', '/root/reading-a-file/work/many/5.2/MOVED/probe.vdb', False)
```

```
FEW AFTER | Image.filepath                           | /root/reading-a-file/work/many/5.2/MOVED/probe.png
FEW AFTER | Image.filepath_raw                       | /root/reading-a-file/work/many/5.2/MOVED/probe.png
FEW AFTER | Sound.filepath                           | /root/reading-a-file/work/many/5.2/MOVED/probe.wav
FEW AFTER | VectorFont.filepath                      | /root/reading-a-file/work/many/5.2/MOVED/probe.woff2
FEW AFTER | Text.filepath                            | /root/reading-a-file/work/many/5.2/MOVED/probe.txt
FEW AFTER | Volume.filepath                          | /root/reading-a-file/work/many/5.2/MOVED/probe.vdb
FEW AFTER | MovieClip.filepath                       | /root/reading-a-file/work/many/5.2/MOVED/probe.png
FEW AFTER | CacheFile.filepath                       | /root/reading-a-file/work/many/5.2/MOVED/probe.abc
FEW AFTER | Library.filepath                         | /root/reading-a-file/work/many/5.2/lib.blend
FEW AFTER | RenderSettings.filepath                  | /root/reading-a-file/work/many/5.2/assets/render_out/
FEW AFTER | BakeSettings.filepath                    | /root/reading-a-file/work/many/5.2/assets/bake_out/
FEW AFTER | CompositorNodeOutputFile base path       | /root/reading-a-file/work/many/5.2/assets/node_out/
FEW AFTER | PointCache.filepath (cloth)              | /root/reading-a-file/work/many/5.2/assets/pointcache/
FEW AFTER | FluidDomainSettings.cache_directory      | /root/reading-a-file/work/many/5.2/MOVED/fluidcache/
FEW AFTER | NodesModifier.bake_directory             | /root/reading-a-file/work/many/5.2/assets/gnbake/
FEW AFTER | MultiresModifier.filepath                | 
FEW AFTER | MeshCacheModifier.filepath               | /root/reading-a-file/work/many/5.2/MOVED/meshcache.mdd
FEW AFTER | OceanModifier.filepath                   | /root/reading-a-file/work/many/5.2/MOVED/oceanbake/
FEW AFTER | ShaderNodeScript.filepath                | /root/reading-a-file/work/many/5.2/MOVED/probe.osl
FEW AFTER | ShaderNodeTexIES.filepath                | /root/reading-a-file/work/many/5.2/MOVED/probe.ies
FEW AFTER | Camera.custom_filepath                   | /root/reading-a-file/work/many/5.2/MOVED/camshader.osl
FEW AFTER | custom property on an object             | /root/reading-a-file/work/many/5.2/assets/custom_prop.png
FEW AFTER | ImageStrip.directory                     | /root/reading-a-file/work/many/5.2/MOVED/stripdir/
FEW AFTER | StripProxy.directory (sequencer)         | /root/reading-a-file/work/many/5.2/assets/proxydir/
```

#### Whether walking the property marks finds every recorded location (list entry 7)

A walk over thirteen of `bpy.data`'s collections, four levels deep, collecting every text setting whose subtype is `FILE_PATH` or `DIR_PATH` or which carries one of the two path marks. The walk found the same distinct values in every version. The counts it printed here:

```
SWEEP COUNT 97
SWEEP distinct values 22
```

and the distinct values it reached, one line each, showing where the walk found them. The last two or three lines are settings that carry a path mark but hold something that is not a path — a data-block name, or the bare file-name part of an output setting — and the custom property is on the end, found only by listing custom properties by hand and not by the walk at all:

```
SWEEP bpy.data.materials['Material'].library_weak_reference.filepath = /opt/blender/5.2/5.2/datafiles/assets/brushes/essentials_brushes-gp_draw.blend
SWEEP bpy.data.scenes['Scene'].render.bake.filepath = /root/reading-a-file/work/many/5.2/assets/bake_out/
SWEEP bpy.data.cameras['ProbeCam'].custom_filepath = /root/reading-a-file/work/many/5.2/assets/camshader.osl
SWEEP custom property bpy.data.objects['ModObj']['probe_path'] = /root/reading-a-file/work/many/5.2/assets/custom_prop.png
SWEEP bpy.data.objects['FluidObj'].modifiers[].domain_settings.cache_directory = /root/reading-a-file/work/many/5.2/assets/fluidcache/
SWEEP bpy.data.objects['GNObj'].modifiers[].bake_directory = /root/reading-a-file/work/many/5.2/assets/gnbake/
SWEEP bpy.data.objects['ModObj'].modifiers[].filepath = /root/reading-a-file/work/many/5.2/assets/meshcache.mdd
SWEEP bpy.data.node_groups['ProbeComp'].nodes[].directory = /root/reading-a-file/work/many/5.2/assets/node_out/
SWEEP bpy.data.objects['ModObj'].modifiers[].filepath = /root/reading-a-file/work/many/5.2/assets/oceanbake/
SWEEP bpy.data.objects['ClothObj'].modifiers[].point_cache.filepath = /root/reading-a-file/work/many/5.2/assets/pointcache/
SWEEP bpy.data.cache_files['probe.abc'].filepath = /root/reading-a-file/work/many/5.2/assets/probe.abc
SWEEP bpy.data.materials['ProbeMat'].node_tree.nodes[].filepath = /root/reading-a-file/work/many/5.2/assets/probe.ies
SWEEP bpy.data.materials['ProbeMat'].node_tree.nodes[].filepath = /root/reading-a-file/work/many/5.2/assets/probe.osl
SWEEP bpy.data.images['ProbeImage'].filepath = /root/reading-a-file/work/many/5.2/assets/probe.png
SWEEP bpy.data.volumes['ProbeVolume'].filepath = /root/reading-a-file/work/many/5.2/assets/probe.vdb
SWEEP bpy.data.sounds['ProbeSound'].filepath = /root/reading-a-file/work/many/5.2/assets/probe.wav
SWEEP bpy.data.fonts['ProbeFont'].filepath = /root/reading-a-file/work/many/5.2/assets/probe.woff2
SWEEP bpy.data.scenes['Scene'].sequence_editor.proxy_dir = /root/reading-a-file/work/many/5.2/assets/proxydir/
SWEEP bpy.data.scenes['Scene'].render.filepath = /root/reading-a-file/work/many/5.2/assets/render_out/
SWEEP bpy.data.scenes['Scene'].sequence_editor.strips[].directory = /root/reading-a-file/work/many/5.2/assets/stripdir/
SWEEP bpy.data.libraries['lib.blend'].filepath = /root/reading-a-file/work/many/5.2/lib.blend
SWEEP bpy.data.materials['Material'].library_weak_reference.id_name = MAMaterial
SWEEP bpy.data.node_groups['ProbeComp'].nodes[].file_name = {blend_name}
```

#### The same file's paths, in a form that names no folder (list entry 8)

This is the half of the two-machine test that this machine can give. The Mac half is owed.

```
CROSS | Image.filepath                           | //assets/probe.png
CROSS | Image.filepath_raw                       | //assets/probe.png
CROSS | Sound.filepath                           | //assets/probe.wav
CROSS | VectorFont.filepath                      | //assets/probe.woff2
CROSS | Text.filepath                            | //assets/probe.txt
CROSS | Volume.filepath                          | //assets/probe.vdb
CROSS | MovieClip.filepath                       | //assets/probe.png
CROSS | CacheFile.filepath                       | //assets/probe.abc
CROSS | Library.filepath                         | //lib.blend
CROSS | RenderSettings.filepath                  | <HERE>/assets/render_out/
CROSS | BakeSettings.filepath                    | <HERE>/assets/bake_out/
CROSS | CompositorNodeOutputFile base path       | <HERE>/assets/node_out/
CROSS | PointCache.filepath (cloth)              | <HERE>/assets/pointcache/
CROSS | FluidDomainSettings.cache_directory      | //assets/fluidcache/
CROSS | NodesModifier.bake_directory             | <HERE>/assets/gnbake/
CROSS | MultiresModifier.filepath                | 
CROSS | MeshCacheModifier.filepath               | //assets/meshcache.mdd
CROSS | OceanModifier.filepath                   | //assets/oceanbake/
CROSS | ShaderNodeScript.filepath                | //assets/probe.osl
CROSS | ShaderNodeTexIES.filepath                | //assets/probe.ies
CROSS | Camera.custom_filepath                   | //assets/camshader.osl
CROSS | custom property on an object             | <HERE>/assets/custom_prop.png
CROSS | ImageStrip.directory                     | //assets/stripdir/
CROSS | StripProxy.directory (sequencer)         | <HERE>/assets/proxydir/
CROSS blend_paths   = ['//assets/camshader.osl', '//assets/fluidcache/', '//assets/meshcache.mdd', '//assets/oceanbake/', '//assets/probe.abc', '//assets/probe.ies', '//assets/probe.osl', '//assets/probe.png', '//assets/probe.png', '//assets/probe.txt', '//assets/probe.vdb', '//assets/probe.wav', '//assets/probe.woff2', '//assets/stripdir/probe.png', '//lib.blend']
CROSS file_path_map = [('FluidObj', ['//assets/fluidcache/']), ('ModObj', ['//assets/meshcache.mdd', '//assets/oceanbake/']), ('ProbeCam', ['//assets/camshader.osl']), ('ProbeClip', ['//assets/probe.png']), ('ProbeFont', ['//assets/probe.woff2']), ('ProbeImage', ['//assets/probe.png']), ('ProbeMat', ['//assets/probe.ies', '//assets/probe.osl']), ('ProbeSound', ['//assets/probe.wav']), ('ProbeText', ['//assets/probe.txt']), ('ProbeVolume', ['//assets/probe.vdb']), ('Scene', ['//assets/stripdir/probe.png']), ('lib.blend', ['//lib.blend']), ('probe.abc', ['//assets/probe.abc'])]
CROSS separator     = '/' | case check = False
```

#### The packed switch, and which listing means show a packed thing's path (list entry 9)

A fresh file with two images loaded from disk, the first of them packed:

```
PACK saved = {'FINISHED'}
PACK PackedImage.packed_file is None = False
PACK PackedImage.filepath = /root/reading-a-file/work/packed/5.2/assets/packed.png
PACK PlainImage.filepath  = /root/reading-a-file/work/packed/5.2/assets/plain.png
PACK blend_paths()            = ['/opt/blender/5.2/5.2/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/opt/blender/5.2/5.2/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/root/reading-a-file/work/packed/5.2/assets/plain.png']
PACK blend_paths(packed=True) = ['/opt/blender/5.2/5.2/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/opt/blender/5.2/5.2/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/root/reading-a-file/work/packed/5.2/assets/packed.png', '/root/reading-a-file/work/packed/5.2/assets/plain.png']
PACK file_path_map()          = [('PlainImage', ['/root/reading-a-file/work/packed/5.2/assets/plain.png'])]
PACK foreach[default] = ["Image 'PlainImage' /root/reading-a-file/work/packed/5.2/assets/plain.png"]
PACK foreach[flags=set()] = ["Image 'PackedImage' /root/reading-a-file/work/packed/5.2/assets/packed.png", "Image 'PlainImage' /root/reading-a-file/work/packed/5.2/assets/plain.png", "Material 'Dots Stroke' /opt/blender/5.2/5.2/datafiles/assets/brushes/essentials_brushes-gp_draw.blend", "Material 'Material' /opt/blender/5.2/5.2/datafiles/assets/brushes/essentials_brushes-gp_draw.blend"]
PACK foreach[SKIP_PACKED] = ["Image 'PlainImage' /root/reading-a-file/work/packed/5.2/assets/plain.png", "Material 'Dots Stroke' /opt/blender/5.2/5.2/datafiles/assets/brushes/essentials_brushes-gp_draw.blend", "Material 'Material' /opt/blender/5.2/5.2/datafiles/assets/brushes/essentials_brushes-gp_draw.blend"]
```

#### Whether unpacking rewrites the recorded location (list entry 10)

Two copies of that file, unpacked two ways, so neither run could affect the other. The `on disk` listing of the second run shows the folder the first run made; the two runs used different copies of the file and the data did not cross.

```
--- unpack USE_LOCAL ---
UNPACK before, filepath = /root/reading-a-file/work/packed/5.2/assets/packed.png | packed = True
UNPACK op(USE_LOCAL) = {'FINISHED'}
UNPACK after,  filepath = //textures/packed.png | packed = False
UNPACK on disk: <W>/packed.blend
UNPACK on disk: <W>/u1.blend
UNPACK on disk: <W>/u2.blend
UNPACK on disk: <W>/assets/packed.png
UNPACK on disk: <W>/assets/plain.png
UNPACK on disk: <W>/textures/packed.png
--- unpack USE_ORIGINAL ---
UNPACK before, filepath = /root/reading-a-file/work/packed/5.2/assets/packed.png | packed = True
UNPACK op(USE_ORIGINAL) = {'FINISHED'}
UNPACK after,  filepath = /root/reading-a-file/work/packed/5.2/assets/packed.png | packed = False
UNPACK on disk: <W>/packed.blend
UNPACK on disk: <W>/u1.blend
UNPACK on disk: <W>/u2.blend
UNPACK on disk: <W>/assets/packed.png
UNPACK on disk: <W>/assets/plain.png
UNPACK on disk: <W>/textures/packed.png
```

#### Whether the wider missing-file search repoints healthy links (list entry 11)

Two folders, each holding a file called `same.png`, and a file with both loaded and both links working. Both were proved to exist before the call — the `True` at the end of each `before` line is that check — and then the search was run over the first folder with its wider switch on:

```
FINDALL built FromOne = /root/reading-a-file/work/findall/5.2/one/same.png
FINDALL built FromTwo = /root/reading-a-file/work/findall/5.2/two/same.png
FINDALL save = {'FINISHED'}
FINDALL before FromOne = /root/reading-a-file/work/findall/5.2/one/same.png True
FINDALL before FromTwo = /root/reading-a-file/work/findall/5.2/two/same.png True
FINDALL op = {'FINISHED'}
FINDALL after  FromOne = /root/reading-a-file/work/findall/5.2/one/same.png
FINDALL after  FromTwo = /root/reading-a-file/work/findall/5.2/one/same.png
```

#### A path holding a template, through the listing means and both sweeps (list entry 12)

```
TMPL save = {'FINISHED'}
TMPL OPENED   render.filepath = /root/reading-a-file/work/template/5.2/assets/out_{blend_name}_####.png
TMPL OPENED   node base path  = /root/reading-a-file/work/template/5.2/assets/node_{blend_name}/
TMPL OPENED   image           = /root/reading-a-file/work/template/5.2/assets/probe.png
TMPL OPENED   blend_paths     = ['/opt/blender/5.2/5.2/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/root/reading-a-file/work/template/5.2/assets/probe.png']
TMPL OPENED   file_path_map   = [['/root/reading-a-file/work/template/5.2/assets/probe.png']]
TMPL OPENED   foreach         = ['/root/reading-a-file/work/template/5.2/assets/probe.png']
TMPL OPENED   frame_path(1)   = /root/reading-a-file/work/template/5.2/assets/out_tmpl_0001.png
TMPL op make_paths_relative = {'FINISHED'}
TMPL REL      render.filepath = /root/reading-a-file/work/template/5.2/assets/out_{blend_name}_####.png
TMPL REL      node base path  = /root/reading-a-file/work/template/5.2/assets/node_{blend_name}/
TMPL REL      image           = //assets/probe.png
TMPL REL      blend_paths     = ['//../../../../../opt/blender/5.2/5.2/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '//assets/probe.png']
TMPL REL      file_path_map   = [['//assets/probe.png']]
TMPL REL      foreach         = ['//assets/probe.png']
TMPL REL      frame_path(1)   = /root/reading-a-file/work/template/5.2/assets/out_tmpl_0001.png
TMPL op make_paths_absolute = {'FINISHED'}
TMPL ABS      render.filepath = /root/reading-a-file/work/template/5.2/assets/out_{blend_name}_####.png
TMPL ABS      node base path  = /root/reading-a-file/work/template/5.2/assets/node_{blend_name}/
TMPL ABS      image           = /root/reading-a-file/work/template/5.2/assets/probe.png
TMPL ABS      blend_paths     = ['/opt/blender/5.2/5.2/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/root/reading-a-file/work/template/5.2/assets/probe.png']
TMPL ABS      file_path_map   = [['/root/reading-a-file/work/template/5.2/assets/probe.png']]
TMPL ABS      foreach         = ['/root/reading-a-file/work/template/5.2/assets/probe.png']
TMPL ABS      frame_path(1)   = /root/reading-a-file/work/template/5.2/assets/out_tmpl_0001.png
```

#### The four remap choices on the write call (list entry 13)

A file holding one image recorded relative and one recorded in full form, both naming files that exist. Each of the four choices wrote a new file one folder below. Each written file was then **opened** rather than linked from, because linking turns a recorded relative path into a full one on the way in and would hide the thing being asked about. Each line gives the stored text, what it works out to, and whether a file is there:

```
WREAD NONE.blend = [('AbsImage', ('/root/reading-a-file/work/write/5.2/assets/abs.png', '/root/reading-a-file/work/write/5.2/assets/abs.png', True)), ('RelImage', ('//assets/rel.png', '/root/reading-a-file/work/write/5.2/out/assets/rel.png', False))]
WREAD RELATIVE.blend = [('AbsImage', ('/root/reading-a-file/work/write/5.2/assets/abs.png', '/root/reading-a-file/work/write/5.2/assets/abs.png', True)), ('RelImage', ('//../assets/rel.png', '/root/reading-a-file/work/write/5.2/out/../assets/rel.png', True))]
WREAD RELATIVE_ALL.blend = [('AbsImage', ('//../assets/abs.png', '/root/reading-a-file/work/write/5.2/out/../assets/abs.png', True)), ('RelImage', ('//../assets/rel.png', '/root/reading-a-file/work/write/5.2/out/../assets/rel.png', True))]
WREAD ABSOLUTE.blend = [('AbsImage', ('/root/reading-a-file/work/write/5.2/assets/abs.png', '/root/reading-a-file/work/write/5.2/assets/abs.png', True)), ('RelImage', ('/root/reading-a-file/work/write/5.2/assets/rel.png', '/root/reading-a-file/work/write/5.2/assets/rel.png', True))]
```

#### Whether the write call may be pointed at a file that already exists (list entry 14)

```
WRITE existing file size = 27
WRITE onto existing file = no error raised
WRITE existing file size before = 27 after = 139849
WRITE existing file first bytes after = b'BLENDER17-01'
```

#### What Save As with Remap Relative moves (list entry 15)

```
SAVEAS save2 = {'FINISHED'}
SAVEAS OPENED   RelImage = //assets/rel.png
SAVEAS OPENED   AbsImage = /root/reading-a-file/work/saveas/5.2/assets/abs.png
SAVEAS OPENED   render   = //renders/
SAVEAS op = {'FINISHED'}
SAVEAS AFTER    RelImage = //../assets/rel.png
SAVEAS AFTER    AbsImage = /root/reading-a-file/work/saveas/5.2/assets/abs.png
SAVEAS AFTER    render   = //renders/
SAVEAS new file location = /root/reading-a-file/work/saveas/5.2/elsewhere/moved.blend
```

#### Writing the library's own path, against relocating it (list entry 16)

Three files: `B.blend` holding an object called `CubeFromB`, `C.blend` holding one called `CubeFromC`, and `A.blend` linking `B`'s collection. Two separate runs from the same `A.blend` — the first assigns the path and then calls `reload()`, the second calls the relocate operator:

```
RELOC OPENED   library = /root/reading-a-file/work/reloc/5.2/B.blend
RELOC OPENED   objects = [('Camera', None), ('Cube', None), ('CubeFromB', '/root/reading-a-file/work/reloc/5.2/B.blend'), ('Light', None)]
RELOC ASSIGNED library = /root/reading-a-file/work/reloc/5.2/C.blend
RELOC ASSIGNED objects = [('Camera', None), ('Cube', None), ('CubeFromB', '/root/reading-a-file/work/reloc/5.2/C.blend'), ('Light', None)]
RELOC reload() = returned
RELOC RELOADED library = /root/reading-a-file/work/reloc/5.2/C.blend
RELOC RELOADED objects = [('Camera', None), ('Cube', None), ('CubeFromC', '/root/reading-a-file/work/reloc/5.2/C.blend'), ('Light', None)]
RELOC OPENED   library = /root/reading-a-file/work/reloc/5.2/B.blend
RELOC OPENED   objects = [('Camera', None), ('Cube', None), ('CubeFromB', '/root/reading-a-file/work/reloc/5.2/B.blend'), ('Light', None)]
RELOC op lib_relocate = {'FINISHED'}
RELOC RELOCATED library = //C.blend
RELOC RELOCATED objects = [('Camera', None), ('Cube', None), ('CubeFromC', '//C.blend'), ('Light', None)]
```

#### The render-output argument, with a save and without one (list entry 17)

```
CLI render.filepath in this run = //FROM_CLI/
CLI save = {'FINISHED'}
CLI render.filepath in this run = //FROM_CLI/
```

Then four runs on a fresh copy of the many-paths file, in this order. Run 1 makes the paths relative and quits without saving; run 2 opens the same file again; run 3 opens it with the argument given and saves nothing; run 4 opens it again with no argument. All four runs, whole:

```
RUN 1 --- make paths relative, then quit without saving
NOSAVE rel    image before = /root/reading-a-file/work/many/5.2/assets/probe.png
NOSAVE rel    render before = /root/reading-a-file/work/many/5.2/assets/render_out/
NOSAVE op = {'FINISHED'}
NOSAVE after, not saved, image = //../../many/5.2/assets/probe.png
RUN 2 --- the same file opened again
NOSAVE check  image before = /root/reading-a-file/work/many/5.2/assets/probe.png
NOSAVE check  render before = /root/reading-a-file/work/many/5.2/assets/render_out/
RUN 3 --- the same file with -o //NOSAVE_CLI/ and no save
NOSAVE check  image before = /root/reading-a-file/work/many/5.2/assets/probe.png
NOSAVE check  render before = //NOSAVE_CLI/
RUN 4 --- the same file opened again, no argument
NOSAVE check  image before = /root/reading-a-file/work/many/5.2/assets/probe.png
NOSAVE check  render before = /root/reading-a-file/work/many/5.2/assets/render_out/
```

#### Whether a bulk rewrite reaches the file on disk without a save (list entry 18)

Runs 1 and 2 of the four just above are this test. Run 1 shows the rewrite taking hold in the running Blender — the `after, not saved` line — and run 2 shows what the file on disk still holds when it is opened again.

#### Turning a stored path into a real one, and the library it came from (list entry 19)

A library in its own folder holding an image recorded as `//libtex.png`, and a file one folder up that links it. The two folders differ on purpose, or the two calls would have no reason to differ:

```
AP own stored          = //assets/own.png
AP own abspath()       = /root/reading-a-file/work/abs/5.2/assets/own.png
AP own relpath(abs)    = //assets/own.png
AP own file exists     = True
AP linked name         = LibTex library = /root/reading-a-file/work/abs/5.2/libdir/lib.blend
AP linked stored       = //libtex.png
AP linked abspath()    = /root/reading-a-file/work/abs/5.2/libtex.png
AP linked abspath(lib) = /root/reading-a-file/work/abs/5.2/libdir/libtex.png
AP linked exists plain = False
AP linked exists lib   = True
```

#### A reference held only inside a node group, a driver or a custom property (list entry 20)

This one answers question 7 of the sheet rather than question 9, and is written into `Preparation/Blender Research/Findings/5.2/Reading a file.md` as well. Four images: one used straight from a material, one used only by an image node inside a node group, one named only as a driver's target, one held only by a custom property that points at the data-block:

```
UM group   users = [('InnerGroup', 'ShaderNodeTree')]
UM group   scenes reached in 7 steps = ['TheScene']
UM group   user_map(value_types=SCENE) = {'Img_group': []}
UM group   users count = 2
UM plain   users = [('HolderMat', 'Material')]
UM plain   scenes reached in 6 steps = ['TheScene']
UM plain   user_map(value_types=SCENE) = {'Img_plain': []}
UM plain   users count = 2
UM driver  users = [('Holder', 'Object')]
UM driver  scenes reached in 4 steps = ['TheScene']
UM driver  user_map(value_types=SCENE) = {'Img_driver': []}
UM driver  users count = 1
UM custom  users = [('Holder', 'Object')]
UM custom  scenes reached in 4 steps = ['TheScene']
UM custom  user_map(value_types=SCENE) = {'Img_custom': []}
UM custom  users count = 2
UM file_path_map = [('Img_custom', ['/root/reading-a-file/work/um/5.2/assets/custom.png']), ('Img_driver', ['/root/reading-a-file/work/um/5.2/assets/driver.png']), ('Img_group', ['/root/reading-a-file/work/um/5.2/assets/group.png']), ('Img_plain', ['/root/reading-a-file/work/um/5.2/assets/plain.png'])]
```

#### What could not be done here (list entries 21 and 22, and the cannot-be-run section)

**Replacing the file an image or a strip is loaded from.** Both operators refuse to run with no window on this machine, when called as they are called here. That refusal is itself the result, and it is what makes this a Mac entry rather than a Linux one. Read it with the Mac section below, which asked the same two operators again: one of them wanted a window and the other wanted the file named and the sequencer's own scene put into the context, and once given those it ran with no window.

```
REPL before image = /root/reading-a-file/work/replace/5.2/first/a.png
REPL op image.replace = ERROR RuntimeError Operator bpy.ops.image.replace.poll() failed, context is incorrect
REPL after  image = /root/reading-a-file/work/replace/5.2/first/a.png
REPL before strip = /root/reading-a-file/work/replace/5.2/first/ | ['a.png']
REPL op sequencer.change_path = ERROR RuntimeError Operator bpy.ops.sequencer.change_path.poll() Context missing sequencer scene
REPL after  strip = /root/reading-a-file/work/replace/5.2/first/ | ['a.png']
```

**The Outliner's Data API view** cannot be reached here at all — the machine has no display server. It was reached on the Creator's Mac on 2026-08-07; the pictures and what they show are in the Mac section below.

**Anything on Windows.** No Windows machine has run it, so whether making paths relative works across two drives on Windows was not reached by any test here. That is why it is unanswered rather than untried. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

**What Blender Asset Tracer reaches.** It is not part of Blender and is not in `Blender Sources/`; reaching it would mean fetching software from outside the project. Not attempted, and recorded as out of scope.

**The multi-resolution modifier's own path.** The property would not take a value in any version, so no test here could act on it. Nothing above says whether it is reached; only that a path could not be put there to find out.

---

### Observed — the Mac half of the hand tests, 2026-08-07

Everything in this section was run on **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card**. The Blender was `/Applications/Blender.app/Contents/MacOS/Blender`, which reported itself as **5.2.0 LTS**, hash `fbe6228777e7` — the same version string and the same hash the rented Linux machine reported for its own copy of this version. Every run used `--factory-startup`, and every run had `BLENDER_USER_RESOURCES` pointed at `$T/res-5.2`, a settings folder made for this sitting before it was used and for nothing else, never at the real one. Below, `$T` stands for `/private/tmp/smoozy-test/raf`, a throwaway folder outside the project, and `$S` for `/Users/artist/Documents/Smoozy hand tests/Reading a file/added-questions/scripts`, where the scripts are kept. Only one Blender of this sitting ran at a time.

**Other topics were running Blender on this same Mac while this sitting ran**, so nothing below says how long anything took or how much memory anything used, and no test below measures either.

**The scripts were written again for this sitting.** The brief said not to touch the rented Linux machine at all, and the Linux half's scripts live there, so they were rebuilt from the procedure and the printouts recorded above. They were checked against the Linux printout before anything was read from them: the twenty-four readings of the many-paths file, and the map of data-blocks to paths, came back the same character for character. That is what makes the two lines that *do* differ readable as a difference between the machines rather than a difference between two scripts.

The full list of tests, with the set-up and the command for each, is `Audits/Blender Research/Reading a file/Hand tests — added questions.md`.

#### The same file's paths on the Mac, set beside the Linux printout (list entry 8)

The many-paths file was built again on the Mac by `$S/s_buildlib.py` and then `$S/s_build.py`, its paths were made relative by `$S/s_rel.py`, which answered `OP make_paths_relative = {'FINISHED'}`, and `$S/s_cross.py` read the result:

```
BLENDER_USER_RESOURCES="$T/res-5.2" "/Applications/Blender.app/Contents/MacOS/Blender" -b --factory-startup \
  "$T/work/many/5.2/many_rel.blend" --python "$S/s_cross.py"
```

What came back:

```
CROSS | Image.filepath                           | //assets/probe.png
CROSS | Image.filepath_raw                       | //assets/probe.png
CROSS | Sound.filepath                           | //assets/probe.wav
CROSS | VectorFont.filepath                      | //assets/probe.woff2
CROSS | Text.filepath                            | //assets/probe.txt
CROSS | Volume.filepath                          | //assets/probe.vdb
CROSS | MovieClip.filepath                       | //assets/probe.png
CROSS | CacheFile.filepath                       | //assets/probe.abc
CROSS | Library.filepath                         | //lib.blend
CROSS | RenderSettings.filepath                  | <HERE>/assets/render_out/
CROSS | BakeSettings.filepath                    | <HERE>/assets/bake_out/
CROSS | CompositorNodeOutputFile base path       | <HERE>/assets/node_out/
CROSS | PointCache.filepath (cloth)              | <HERE>/assets/pointcache/
CROSS | FluidDomainSettings.cache_directory      | //assets/fluidcache/
CROSS | NodesModifier.bake_directory             | <HERE>/assets/gnbake/
CROSS | MultiresModifier.filepath                | 
CROSS | MeshCacheModifier.filepath               | //assets/meshcache.mdd
CROSS | OceanModifier.filepath                   | //assets/oceanbake/
CROSS | ShaderNodeScript.filepath                | //assets/probe.osl
CROSS | ShaderNodeTexIES.filepath                | //assets/probe.ies
CROSS | Camera.custom_filepath                   | //assets/camshader.osl
CROSS | custom property on an object             | <HERE>/assets/custom_prop.png
CROSS | ImageStrip.directory                     | //assets/stripdir/
CROSS | StripProxy.directory (sequencer)         | <HERE>/assets/proxydir/
CROSS blend_paths   = ['//../../../../../../../../../../Applications/Blender.app/Contents/Resources/5.2/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '//assets/camshader.osl', '//assets/fluidcache/', '//assets/meshcache.mdd', '//assets/oceanbake/', '//assets/probe.abc', '//assets/probe.ies', '//assets/probe.osl', '//assets/probe.png', '//assets/probe.png', '//assets/probe.txt', '//assets/probe.vdb', '//assets/probe.wav', '//assets/probe.woff2', '//assets/stripdir/probe.png', '//lib.blend']
CROSS file_path_map = [('FluidObj', ['//assets/fluidcache/']), ('ModObj', ['//assets/meshcache.mdd', '//assets/oceanbake/']), ('ProbeCam', ['//assets/camshader.osl']), ('ProbeClip', ['//assets/probe.png']), ('ProbeFont', ['//assets/probe.woff2']), ('ProbeImage', ['//assets/probe.png']), ('ProbeMat', ['//assets/probe.ies', '//assets/probe.osl']), ('ProbeSound', ['//assets/probe.wav']), ('ProbeText', ['//assets/probe.txt']), ('ProbeVolume', ['//assets/probe.vdb']), ('Scene', ['//assets/stripdir/probe.png']), ('lib.blend', ['//lib.blend']), ('probe.abc', ['//assets/probe.abc'])]
CROSS separator     = '/' | case check = True
```

Set beside the Linux printout for this same version, line by line: **twenty-five of the twenty-seven lines are identical, character for character.** That is all twenty-four readings, and the map of data-blocks to paths. **Two lines differ.**

**The flat path list.** The Mac's list holds sixteen entries; the Linux list holds fifteen. The fifteen they share are identical. The one the Mac has and Linux does not is a path into the Blender application's own folder, ending `datafiles/assets/brushes/essentials_brushes-gp_draw.blend`, recorded relative to the file. Two further readings taken on the Mac, recorded because they bound what that line can be read as. The same file **before** its paths were made relative lists that path on the Mac in full form, exactly as the Linux printout above lists its own copy of it — so the difference appears only after the sweep and the save. And after the sweep the material it hangs off reads:

```
WEAK 5.2 Material '//../../../../../../../../../../Applications/Blender.app/Contents/Resources/5.2/datafiles/assets/brushes/essentials_brushes-gp_draw.blend'
WEAK 5.2 ProbeMat None
```

Nothing here says why the Linux listing has no such entry after its own sweep; it says only that the Mac's does.

**Whether capital letters in a name matter.** This line reads `case check = True` on the Mac and `case check = False` on Linux. What the check does, in full, is ask for the image's own file again with its name written in capital letters:

```
img_abs = bpy.path.abspath(bpy.data.images["ProbeImage"].filepath)
upper = os.path.join(os.path.dirname(img_abs), os.path.basename(img_abs).upper())
os.path.exists(upper)
```

The file on disk is called `probe.png`, and nothing called `PROBE.PNG` was ever written — asked for the folder's own listing of everything whose name matches `probe.png` in any spelling, the Mac answered `['probe.png']`, one file. The check itself answered:

```
CASE 5.2 asked for $T/work/many/5.2/assets/PROBE.PNG
CASE 5.2 exists     True
CASE 5.2 real name  ['probe.png']
```

**On this point the Mac's answer is the one to read, not the Linux one**: they are different answers about two different machines and neither speaks for the other. The Linux half's own script was not opened for this sitting, so whether it asked this in exactly these words is not established; the words above are the Mac's.

#### Replacing the file an image or a strip is loaded from, with a window (list entry 21)

The Linux half of this entry is recorded above as a refusal with no window. The Mac has a screen, so the file was built there with no window by `$S/s_replace.py` and then opened **with a window on screen**, and the two operators were called from a `--python` script through a context override. The script ends by stopping Blender itself, so nothing was left open on the Creator's desk.

```
BLENDER_USER_RESOURCES="$T/res-5.2" "/Applications/Blender.app/Contents/MacOS/Blender" -b --factory-startup \
  --python "$S/s_replace.py" -- build "$T/work/replace/5.2"
BLENDER_USER_RESOURCES="$T/res-5.2" "/Applications/Blender.app/Contents/MacOS/Blender" --factory-startup \
  "$T/work/replace/5.2/repl.blend" --python "$S/s_replace_window.py" -- "$T/work/replace/5.2"
```

The script puts `ProbeImage` in an Image editor and calls `bpy.ops.image.replace(filepath=<the second file>, relative_path=False)`; then it switches the area to the Sequence editor, selects the strip, and calls `bpy.ops.sequencer.change_path` three times — first exactly as the list wrote it, then with the file named as well, then with the sequencer's scene supplied in the context. All three calls and all three answers are printed. What came back:

```
REPL build save {'FINISHED'}
REPL window     = True
REPL version    = 5.2.0 LTS
REPL before image = /private/tmp/smoozy-test/raf/work/replace/5.2/first/a.png
REPL op image.replace = {'FINISHED'}
REPL after  image = /private/tmp/smoozy-test/raf/work/replace/5.2/second/b.png
REPL before strip = /private/tmp/smoozy-test/raf/work/replace/5.2/first/ | ['a.png']
REPL op sequencer.change_path (no files argument) = ERROR RuntimeError Operator bpy.ops.sequencer.change_path.poll() Context missing sequencer scene
REPL after  strip, first call = /private/tmp/smoozy-test/raf/work/replace/5.2/first/ | ['a.png']
REPL op sequencer.change_path (files named) = ERROR RuntimeError Operator bpy.ops.sequencer.change_path.poll() Context missing sequencer scene
REPL after  strip, second call = /private/tmp/smoozy-test/raf/work/replace/5.2/first/ | ['a.png']
REPL context has sequencer_scene = True
REPL op sequencer.change_path (sequencer scene supplied) = {'FINISHED'}
REPL after  strip, third call = /private/tmp/smoozy-test/raf/work/replace/5.2/second/ | ['b.png']
```

#### The same two operators with no window at all, on the Mac (list entry 23)

This one is not on the list as it was written; it was added because the run above showed the list had missed it. The Linux half concluded that both operators refuse to run with no window. But the windowed run above shows that what the sequencer operator wanted was the file named and the sequencer's own scene put into the context, neither of which the Linux run supplied. So the same two operators were asked again on the Mac **with no window at all**, and given both of those things:

```
BLENDER_USER_RESOURCES="$T/res-5.2" "/Applications/Blender.app/Contents/MacOS/Blender" -b --factory-startup \
  --python "$S/s_replace.py" -- build "$T/work/replacebg/5.2"
BLENDER_USER_RESOURCES="$T/res-5.2" "/Applications/Blender.app/Contents/MacOS/Blender" -b --factory-startup \
  "$T/work/replacebg/5.2/repl.blend" --python "$S/s_replace_nowindow.py" -- "$T/work/replacebg/5.2"
```

What came back:

```
REPLBG background = True
REPLBG version    = 5.2.0 LTS
REPLBG screen     = <bpy_struct, Screen("Layout") at 0x982c7d420>
REPLBG areas      = 4
REPLBG before image = <W>/5.2/first/a.png
REPLBG op image.replace = ERROR RuntimeError Operator bpy.ops.image.replace.poll() failed, context is incorrect
REPLBG after  image = <W>/5.2/first/a.png
REPLBG before strip = <W>/5.2/first/ | ['a.png']
REPLBG op sequencer.change_path = ERROR RuntimeError Operator bpy.ops.sequencer.change_path.poll() Context missing sequencer scene
REPLBG after  strip = <W>/5.2/first/ | ['a.png']
REPLBG op sequencer.change_path (sequencer scene supplied) = {'FINISHED'}
REPLBG after  strip, second call = <W>/5.2/second/ | ['b.png']
```

#### The Outliner's Data API view (list entry 22)

The Mac has a screen, so this could be reached there and nowhere else. `many_rel.blend` was opened with a window, the largest area was turned into the Outliner and put into its Data API view, the interface scale was set to 0.5 so that more rows would fit in one picture, and the area was photographed with Blender's own screenshot command. Nothing was clicked. A second run assigned a new path to the image while that same view was on screen, saved, reopened the file and read the value back.

```
BLENDER_USER_RESOURCES="$T/res-5.2" "/Applications/Blender.app/Contents/MacOS/Blender" --factory-startup --window-geometry 0 0 2600 1500 \
  "$T/work/many/5.2/many_rel.blend" --python "$S/s_outliner_window.py" -- shot "$T/out/outl/5.2-%s.png"
BLENDER_USER_RESOURCES="$T/res-5.2" "/Applications/Blender.app/Contents/MacOS/Blender" --factory-startup --window-geometry 0 0 2600 1500 \
  "$T/work/many/5.2/outl.blend" --python "$S/s_outliner_window.py" -- write "$T/out/outl/5.2-%s.png"
```

What came back:

```
OUTL version       = 5.2.0 LTS
OUTL background    = False
OUTL display_mode  = DATA_API
OUTL area          = 2477 x 1503
OUTL ui_scale asked for 0.5, got 0.5
OUTL screenshot_area two_levels = {'FINISHED'} 135189 bytes
OUTL screenshot_area four_levels = {'FINISHED'} 164207 bytes
OUTL view y before = -1451.0
OUTL outliner.scroll_page = {'FINISHED'} view y now -2902.0
OUTL screenshot_area after_scroll_page = {'FINISHED'} 134635 bytes
OUTL view2d.pan = {'FINISHED'} view y now -4302.0
OUTL screenshot_area after_pan = {'FINISHED'} 134833 bytes
OUTL filter_text set to 'Filepath'
OUTL screenshot_area after_filter = {'FINISHED'} 146276 bytes
OUTL version       = 5.2.0 LTS
OUTL background    = False
OUTL display_mode  = DATA_API
OUTL area          = 2477 x 1503
OUTL ui_scale asked for 0.5, got 0.5
OUTL before = //assets/probe.png
OUTL assignment raised nothing
OUTL after  = //assets/CHANGED.png
OUTL saved  = {'FINISHED'}
OUTL reopened = //assets/CHANGED.png
```

**What the pictures show.** With the tree opened two levels, the whole of `Blend-File Data` is on one page: a first column of names, a second column of values, and a third column that is empty. Every value is drawn as a control rather than as flat text — tick-boxes for the yes-or-no ones, an item count for each kind of data-block, and, for `Filename`, a **text field holding the file's own recorded path**, drawn greyed. With the tree opened four levels, a data-block's own properties are drawn the same way: `Name` as a field that is not greyed, `Full Name`, `Library` and `Library Weak Reference` as greyed fields, and drop-downs, tick-boxes and number fields for the rest. So a path value **is** shown in this view, and it is drawn as a field and not as flat text.

**What the pictures do not show, and why.** The image data-block's own `Filepath` row was not brought into a picture. Reaching it needs the tree scrolled a long way down. With the whole tree opened it is very long: counted in 4.5, at this interface scale, it runs to 556 screens of about 94 rows, and it was not counted again in the other three versions. Three ways of moving down it without clicking were tried, and their answers are in the printout above: `bpy.ops.outliner.scroll_page(up=False)` and `bpy.ops.view2d.pan(deltax=0, deltay=-1400)` both returned `{'FINISHED'}` and both moved the view — the reported view position changes with each — but a picture taken after them does not always follow, and setting `filter_text` after the first drawing did not reach the picture at all. Walking the tree a screen at a time did reach rows named `Custom File Path`, under the camera, and `File Path`, under the Alembic cache file — so path properties are listed under their data-blocks — but on those scrolled screens the value column drew empty, so those particular rows do not show whether their value is a field. Blender builds this tree while it draws, and the drawing loop is not running while a `--python` script holds it; that is the limit this ran into. **A note on the list as written:** it names the view's setting as `'DATAAPI'`; the value Blender accepts is `'DATA_API'`, and asking for the first one stops the script.

**The read-back.** Assigning `bpy.data.images["ProbeImage"].filepath = "//assets/CHANGED.png"` while that view was on screen raised nothing, read back as the new value, and still read as the new value after the file was saved, closed and opened again.

---

## What is still open for a person with Blender in front of them

The list the researcher wrote here has been pruned. Everything on it that a terminal could reach
was run on 2026-08-06 and is recorded in the observed section above; those entries have been taken
off. What is left is what genuinely remains, and each says what a negative result would look like.

The full list of tests for this round, with the exact set-up and command for each,
is `Audits/Blender Research/Reading a file/Hand tests — added questions.md`.

**1. Can the Outliner's Data API view be scrolled far enough to show an image's own path row?**
The view itself was reached and photographed on the Mac on 2026-08-07, and a path value is shown
there, drawn as a field. What was not reached is the image data-block's own row: the fully opened
tree runs to 556 screens, and the ways of moving down it from a script did not reach the picture,
for the reason recorded above. Anyone with a mouse can settle it in a moment by opening
`Images ▸ ProbeImage` in that view and looking at the `Filepath` row.
*Negative result:* no path value shown under the image data-block — which would mean the view does
not list paths at all, against what the manual is quoted as saying.

**2. Does the Data API view's path field accept typing into it?**
The read-back above shows the value can be changed through the same route the view uses. What no
test here shows is whether the field on screen can be typed into, which needs a keystroke.
*Negative result:* the field refuses the keystroke, or the value snaps back.

**3. Do the tests marked for both machines agree on the Mac?**
Every test in this round except one was marked as running on both machines and was in fact run on
the rented Linux machine alone. The one that was run on both agreed on twenty-five of its
twenty-seven lines. The rest are untested on macOS.
*Negative result:* a test comes out differently on the Mac, and the printouts say where.

**4. Does writing a path on its own reload the thing it points at?**
Answered above for a linked library — the data did not change until `reload()` was called — and
untried for anything else. Load an image, assign a different real file to its `filepath`, and see
whether the picture in the Image editor changes without any further step.
*Negative result:* the picture is the old one while the path is the new one.

**5. How does the missing-file search decide two files match?**
The wider form of it was shown above to repoint a healthy link at a same-named file in the folder
it was pointed at. What is still unknown is what it does when two candidates of the same name sit
at different depths under the search folder.
*Negative result:* it picks neither and leaves both alone.

### What was not run, and why

- **Anything on Windows**, including whether making paths relative works across two drives. No Windows machine has run it. This is untried rather than unanswerable: a
  Windows machine can be rented — see the equipment section of `Preparation/Blender
  Research/Method.md`.
- **What Blender Asset Tracer reaches.** It is not part of Blender and is not in `Blender Sources/`;
  reaching it would mean fetching software from outside the project.
- **Whether the multi-resolution modifier's own recorded path is reached by any of this.** The
  property would not take a value in any version, as recorded above, so no test could put a path
  there to find out.
- **What the image data-block's own `Filepath` row looks like in the Outliner's Data API view.**
  The view was reached and photographed on the Mac and it does show path values, drawn as fields;
  that particular row could not be scrolled into the picture from a script, for the reason recorded
  in the Mac section above. This is the one thing in this round that a mouse would settle in a
  moment and a script could not.

---

2026-08-06 — Opus 5 (Claude Code) — Reading a file (question 9), Blender 5.2

2026-08-07 — Opus 5 (Claude Code) — the Mac half of the hand tests

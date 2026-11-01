# Reading a file — added questions, second pass (Blender 4.5)

Sources consulted, and the date read:

- Blender's manual for 4.5 (`https://docs.blender.org/manual/en/4.5/`), read 2026-07-30
- Blender's Python API documentation for 4.5 (`https://docs.blender.org/api/4.5/`), read 2026-07-30
- Blender's release notes for 4.5 (`https://developer.blender.org/docs/release_notes/4.5/`), read 2026-07-30
- Cycles' own files as shipped with 4.5, read 2026-07-30
- Blender's requirements page was **not** used for this answer.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

Only question 9 was answered. The rest of the sheet was left alone.

---

## 9. What separate means exist for listing and rewriting the locations recorded in a `.blend` file, and which kinds of recorded location does each of those means reach?

**How I read the question.** It is ambiguous in two places, so I say plainly which reading I answered.

- "Separate means" — I read this as *distinct mechanisms* a person or a script can use. The other reading is "means that work separately from Blender", i.e. without loading the file into Blender; I cover that too, in means **L5** below, because the sources touch it once.
- "Kinds of recorded location" — I read this as *kinds of stored path* (the path to a linked library, to an image, to a sound, to a cache, the render output path, and so on). The other reading is the *form* a path is stored in (relative with the `//` prefix, absolute, or packed away); where the sources speak to that form I say so.

**LABEL: mixed.** The list of mechanisms is **documented**. The list of path kinds that exist in a file is **documented**. The pairing the question actually asks for — which kinds each bulk mechanism reaches — is **not in the sources** for the four bulk mechanisms, and **documented** only for the narrow ones. I keep the two apart below and label each item.

---

### The means, grouped by what they do

#### Listing

**L1 — `bpy.utils.blend_paths()`. LABEL: documented.**

> "bpy.utils.blend_paths(absolute=False, packed=False, local=False)
>
> Returns a list of paths to external files referenced by the loaded .blend file.
> […]
> absolute (bool) – When true the paths returned are made absolute.
>
> packed (bool) – When true skip file paths for packed data.
>
> local (bool) – When true skip linked library paths."

Source: Python API documentation, `https://docs.blender.org/api/4.5/bpy.utils.html`. Date read: 2026-07-30.

Kinds reached: the source says "external files referenced by the loaded .blend file" and does not name which kinds those are. The only kind-level detail it gives is the two switches: paths belonging to packed data can be skipped, and linked-library paths can be skipped. It returns bare strings — nothing says which data-block each path came from.

**L2 — `bpy.data.file_path_map()`. LABEL: documented.**

> "file_path_map(subset=None, key_types=None, include_libraries=False)
>
> Returns a mapping of all ID data-blocks in current bpy.data to a set of all file paths used by them.
> […]
> include_libraries (bool) – Include library file paths of linked data. False by default.
>
> Returns:
>
> dictionary of bpy.types.ID instances, with sets of file path strings as their values."

Source: Python API documentation, `https://docs.blender.org/api/4.5/bpy.types.BlendData.html`. Date read: 2026-07-30.

Kinds reached: stated only as "all file paths used by them", per data-block, for all ID data-blocks. It can be narrowed to a subset of data-blocks or to chosen ID types. Library paths of linked data are **left out unless asked for**. This is the only listing means in the sources that tells you which data-block a path belongs to.

**L3 — reading the individual path properties on data-blocks. LABEL: documented.**

Every stored path is an ordinary property that can be read (and written — see R3). The API documentation marks the ones that accept Blender's `//` prefix with the note "blend relative // prefix supported". Searching the whole of the 4.5 API documentation for that exact phrase gives 41 pages (corrected from 39 by the citation check, 2026-08-06; two of them are operator pages and five are collection `load()` pages, leaving 34 type pages of properties). Stripping out the collection `load()` methods that are not stored paths, and the one page that is application preferences rather than file contents, the recorded locations are:

| Type and property | What the documentation calls it |
| --- | --- |
| `Library.filepath` | "Path to the library .blend file" |
| `LibraryWeakReference.filepath` | "Path to the library .blend file" |
| `Image.filepath`, `Image.filepath_raw` | "Image/Movie file name" / "Image/Movie file name (without data refreshing)" |
| `ImagePackedFile.filepath` | (no description given) |
| `Sound.filepath` | "Sound sample file used by this Sound data-block" |
| `MovieClip.filepath` | "Filename of the movie or sequence file" |
| `MovieClipProxy.directory` | "Location to store the proxy files" |
| `VectorFont.filepath` | (no description given) |
| `Volume.filepath` | "Volume file used by this Volume data-block" |
| `VolumeGrids.frame_filepath` | "Volume file used for loading the volume at the current frame…" |
| `CacheFile.filepath`, `CacheFileLayer.filepath` | "Path to external displacements file" / "Path to the archive" |
| `MeshCacheModifier.filepath`, `MultiresModifier.filepath` | "Path to external displacements file" |
| `OceanModifier.filepath` | "Path to a folder to store external baked images" |
| `NodesModifier.bake_directory`, `NodesModifierBake.directory` | "Location on disk where the bake data is stored" |
| `PointCache.filepath`, `PointCacheItem.filepath` | "Cache file path" |
| `FluidDomainSettings.cache_directory` | "Directory that contains fluid cache files" |
| `DynamicPaintSurface.image_output_path` | "Directory to save the textures" |
| `RenderSettings.filepath` | "Directory/name to save animations, # characters define the position and padding of frame numbers" |
| `BakeSettings.filepath` | "Image filepath to use when saving externally" |
| `CompositorNodeOutputFile.base_path` | "Base output path for the image" |
| `ShaderNodeScript.filepath` | "Shader script path" |
| `ShaderNodeTexIES.filepath` | "IES light path" |
| `MovieStrip.filepath`, `ImageStrip.directory` | (no description given) / (no description given) |
| `StripProxy.filepath`, `StripProxy.directory`, `SequenceEditor.proxy_dir` | "Location of custom proxy file" / (no description) / (no description) |
| `Brush.icon_filepath` | "File path to brush icon" |
| `CollectionExport.filepath` | "The file path used for exporting" |
| `NodeSocketStringFilePath.default_value` | "Input value used for unconnected socket" |

Source: Python API documentation, the `bpy.types.*` page for each type, e.g. `https://docs.blender.org/api/4.5/bpy.types.Library.html`, `https://docs.blender.org/api/4.5/bpy.types.Image.html`, `https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html`. Date read: 2026-07-30.

Two further recorded locations carry no `//` marker in their type line, so they are stored paths that the documentation does not describe as blend-relative:

> "filepath
>
> Filename of the text file
>
> Type:
>
> string, default "", (never None)"

Source: `https://docs.blender.org/api/4.5/bpy.types.Text.html`. Date read: 2026-07-30.

> "custom_filepath
>
> Path to the shader defining the custom camera
>
> Type:
>
> string, default "", (never None)"

Source: `https://docs.blender.org/api/4.5/bpy.types.Camera.html`. Date read: 2026-07-30. `Camera.custom_filepath` is listed as new in this version: "bpy.types.Camera / Added / … bpy.types.Camera.custom_filepath" — Python API documentation, `https://docs.blender.org/api/4.5/change_log.html`, section "4.4 to 4.5". Date read: 2026-07-30.

One page carrying the same marker is *not* a recorded location in a `.blend` file — `PreferencesFilePaths` (`font_directory`, `sound_directory`, `texture_directory`, `render_output_directory`, `render_cache_directory`). Those are application preferences. I mention them only so they are not miscounted.

**L4 — the Outliner, in the interface. LABEL: documented.**

> "Blender File:
>
> Lists all data in the current blend-file. […]
>
> Data API:
>
> Lists every data-block in the file along with any properties that it might have."

Source: manual, `https://docs.blender.org/manual/en/4.5/editors/outliner/interface.html`. Date read: 2026-07-30.

Kinds reached: Blender File mode is where the manual sends you to act on a library (see R6). Data API mode is described as listing every data-block "along with any properties that it might have", which covers the path properties of L3. Note the step: the manual does not say "file paths" here — it says "any properties", and a stored path is a property. That much is documented; that every path in L3 is therefore visible there is **inferred**.

**L5 — a report of only the *missing* ones. LABEL: documented.**

> "Report Missing Files
>
> This option is useful to check if there are links to unpacked files that no longer exist.
> After selecting this option, a warning message will appear in the Info editor's header.
> If no warning is shown, there are no missing external files."

Source: manual, `https://docs.blender.org/manual/en/4.5/interface/window_system/topbar.html`. Date read: 2026-07-30.

Its Python name and one-line description:

> "bpy.ops.file.report_missing_files()
>
> Report all missing external files"

Source: Python API documentation, `https://docs.blender.org/api/4.5/bpy.ops.file.html`. Date read: 2026-07-30.

Kinds reached: "all missing external files" — not enumerated. One kind is named indirectly by the 4.5 release notes, in the list of fixed bugs:

> "Internal mode in IES texture node flagged in missing file report"

Source: release notes, `https://developer.blender.org/docs/release_notes/4.5/bugfixes/`. Date read: 2026-07-30. Reading this as evidence that the IES light path is among the kinds the report covers is a step past the words; that step is **inferred**.

**L6 — tooling outside Blender. LABEL: documented (that it exists), not in the sources (what it reaches).**

> "Blender itself is not allowed to write to other blend-files than the currently open one. […]
> Since there is already tooling that can manipulate blend-files outside of Blender itself
> (see Blender Asset Tracer),
> it's possible to also create an external tool for doing such edits across blend-files."

Source: manual, `https://docs.blender.org/manual/en/4.5/files/asset_libraries/introduction.html`. Date read: 2026-07-30.

This is the only place in the sources that names a means of touching a `.blend` file's contents without Blender opening it. The manual names it and says no more; which kinds of recorded location it reaches is not in the sources. The same passage carries a hard limit on every other means on this page: Blender writes to the file it has open, and to nothing else.

#### Rewriting

**R1 — Make Paths Relative / Make Paths Absolute, in bulk. LABEL: documented.**

> "Make Paths Relative
>
> Make all paths to external files relative to the current blend-file.
>
> Make Paths Absolute
>
> Make all paths to external files absolute (= full path from the system's root)."

Source: manual, `https://docs.blender.org/manual/en/4.5/interface/window_system/topbar.html` (menu `File ‣ External Data`). Date read: 2026-07-30.

Their Python names:

> "bpy.ops.file.make_paths_absolute()
>
> Make all paths to external files absolute"

> "bpy.ops.file.make_paths_relative()
>
> Make all paths to external files relative to current .blend"

Source: Python API documentation, `https://docs.blender.org/api/4.5/bpy.ops.file.html`. Date read: 2026-07-30.

Kinds reached: "all paths to external files". The sources do not break that down. Neither takes any argument, so neither can be aimed at a subset.

**R2 — Find Missing Files. LABEL: documented.**

> "Find Missing Files
>
> In case you have broken links in a blend-file, this can help you to fix the problem.
> A File Browser will show up. Select the desired directory (or a file within that directory),
> and a search will be performed in it, recursively in all contained directories.
> Every missing file found in the search will be recovered.
> Those recoveries will be done as absolute paths,
> so if you want to have relative paths you will need to select Make Paths Relative."

Source: manual, `https://docs.blender.org/manual/en/4.5/interface/window_system/topbar.html`. Date read: 2026-07-30.

> "bpy.ops.file.find_missing_files(*, find_all=False, directory='', … )
>
> Try to find missing external files
>
> […] find_all (boolean, (optional)) – Find All, Find all files in the search path (not just missing)
>
> directory (string, (optional, never None)) – Directory, Directory of the file"

Source: Python API documentation, `https://docs.blender.org/api/4.5/bpy.ops.file.html`. Date read: 2026-07-30.

Kinds reached: "missing external files", not enumerated. What *is* documented about the result: the rewritten paths come out absolute, and `find_all` widens the rewrite from missing files only to every file found in the search path.

**R3 — assigning to an individual path property. LABEL: documented.**

Each of the properties listed in L3 is given as a writable string property (they are not marked `readonly`), and the type line states that the `//` prefix is accepted, e.g.:

> "filepath
>
> Path to the library .blend file
>
> Type:
>
> string, default "", (never None, blend relative // prefix supported)"

Source: Python API documentation, `https://docs.blender.org/api/4.5/bpy.types.Library.html`. Date read: 2026-07-30.

Kinds reached: exactly one recorded location per assignment, and any of the kinds in L3. This is the only means in the sources with per-path aim.

The same fields are editable by hand in the interface, and the file browser can be opened onto any of them:

> "bpy.ops.buttons.file_browse(*, filepath='', … relative_path=True, …)
>
> Open a file browser, hold Shift to open the file, Alt to browse containing directory"

> "bpy.ops.buttons.directory_browse(*, directory='', … relative_path=True, …)
>
> Open a directory browser, hold Shift to open the file, Alt to browse containing directory"

Source: Python API documentation, `https://docs.blender.org/api/4.5/bpy.ops.buttons.html`. Date read: 2026-07-30. Both carry the `relative_path` option, which is the same choice the manual describes as the File Browser's "Relative Path checkbox" (manual, `https://docs.blender.org/manual/en/4.5/files/blend/open_save.html`, date read 2026-07-30). Note that these two are listed by name in the API's own list of operators "only intended for use in a specific context […] only called from the properties editor" (`https://docs.blender.org/api/4.5/info_gotchas_operators.html`, date read 2026-07-30).

**R4 — Remap Relative when saving. LABEL: documented.**

> "Remap Relative
>
> This option remaps Relative Paths
> (such as linked libraries and images) when saving a file in a new location."

Source: manual, `https://docs.blender.org/manual/en/4.5/files/blend/open_save.html`. Date read: 2026-07-30. The same page adds:

> "If it is necessary to relocate a blend-file relative to its linked resources,
> use Blender's File Save As
> function which has an option to Remap Relative file links."

The Python form is a parameter on both save operators:

> "relative_remap (boolean, (optional)) – Remap Relative, Remap relative paths when saving to a different directory"

Source: Python API documentation, `https://docs.blender.org/api/4.5/bpy.ops.wm.html`, on `bpy.ops.wm.save_as_mainfile` (default `True`) and `bpy.ops.wm.save_mainfile` (default `False`). Date read: 2026-07-30.

Kinds reached: relative paths only — absolute ones are untouched by the wording "remaps Relative Paths". The manual's parenthesis "(such as linked libraries and images)" is an example, not a list.

**R5 — `bpy.data.libraries.write(..., path_remap=...)`. LABEL: documented.**

> "write(filepath, datablocks, path_remap=False, fake_user=False, compress=False)
>
> Write data-blocks into a blend file.
> […]
> path_remap (str) –
> Optionally remap paths when writing the file:
>
> NONE No path manipulation (default).
>
> RELATIVE Remap paths that are already relative to the new location.
>
> RELATIVE_ALL Remap all paths to be relative to the new location.
>
> ABSOLUTE Make all paths absolute on writing."

Source: Python API documentation, `https://docs.blender.org/api/4.5/bpy.types.BlendDataLibraries.html`. Date read: 2026-07-30.

Kinds reached: "paths" without a breakdown, but with a documented distinction the other means do not offer — `RELATIVE` touches only the already-relative ones, `RELATIVE_ALL` and `ABSOLUTE` touch all. This rewrites into a **newly written** file, not into the file you have open, and only for the data-blocks handed to it (plus, per the same page, "Indirectly referenced data-blocks will be expanded and written too").

**R6 — Relocate a library, or a linked data-block. LABEL: documented.**

> "Relocate Library
> […]
> When the Outliner is in the Blender File Display Mode,
> a right-click on a library (a linked blend-file) will show the Relocate option.
> It allows to replace it by a different file.
> This can be used to either fix a broken linked library (e.g. because the file was moved
> or renamed), or to switch to a variation of the same data in a different file."

> "Relocate Linked ID
> […]
> A right-click a linked ID will show the Relocate option in the ID Data sub-menu. It allows to
> relocate a directly linked ID by another one, from the same library or from a different one.
> […]
> Note
>
> This operation is only available on data-blocks that are directly linked, and are not
> a dependency of any other linked data."

Source: manual, `https://docs.blender.org/manual/en/4.5/files/linked_libraries/link_append.html`. Date read: 2026-07-30.

The Python forms are `bpy.ops.wm.lib_relocate(*, library='', filepath='', … relative_path=True, …)` — "Relocate the given library to one or several others" — and `bpy.ops.wm.id_linked_relocate(*, id_session_uid=0, filepath='', …)`, with `id_session_uid` described as "Linked ID Session UID, Unique runtime identifier for the linked ID to relocate"; also `bpy.ops.outliner.lib_relocate()` "Relocate the library under cursor" and `bpy.ops.outliner.lib_operation(*, type='DELETE')` with a `RELOCATE` member. Sources: `https://docs.blender.org/api/4.5/bpy.ops.wm.html` and `https://docs.blender.org/api/4.5/bpy.ops.outliner.html`. Date read: 2026-07-30.

Relocating a linked data-block is new in this version:

> "* Linked IDs can now also be relocated, if they are not a dependency of any other linked data"

Source: release notes, `https://developer.blender.org/docs/release_notes/4.5/core/`. Date read: 2026-07-30.

Kinds reached: exactly one kind — the path to a linked `.blend` (and, for the ID form, which data-block inside it). Nothing else.

`Library.filepath` can also simply be assigned (see L3/R3), and the type carries `reload()` — "Reload this library and all its linked data-blocks" (`https://docs.blender.org/api/4.5/bpy.types.Library.html`, date read 2026-07-30).

**R7 — the render output path from the command line. LABEL: documented.**

> "-o, --render-output <path>
>
> Set the render path and file name.
> Use // at the start of the path to render relative to the blend-file."

Source: manual, `https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html`. Date read: 2026-07-30.

Kinds reached: one kind only — the render output path (`RenderSettings.filepath`). The same page warns the order of arguments matters: "…will not render to /tmp because --render-frame 1 renders before the output path is set." Whether this writes the new value back into the file, or only applies for the run, is not stated on that page; the page is about a session, not about saving.

**R8 — packing and unpacking, which changes whether the recorded path is used at all. LABEL: documented.**

Packing does not rewrite paths, but it decides whether an external location is consulted, and unpacking can write files to, and then use, a different location:

> "Use files in current directory (create when necessary)
>
> Unpacks all files in the same directory // as the blend-file,
> grouping them in proper folders (like ''textures'' for instance)."

and the limit on which kinds can be packed at all:

> "Not all external files can be packed
>
> Some typically heavy external files, like videos from the Sequence Editor
> or Movie Clips, cannot be packed in a blend-file."

Source: manual, `https://docs.blender.org/manual/en/4.5/files/blend/packed_data.html`. Date read: 2026-07-30. Relevant because `bpy.utils.blend_paths()` (L1) has a switch to skip packed data — so packing state and path listing interact.

---

### What I ruled out, and why

- **`bpy.path`** (`abspath`, `relpath`, `basename`, `resolve_ncase`, `native_pathsep`, `reduce_dirs`, and the rest, `https://docs.blender.org/api/4.5/bpy.path.html`, date read 2026-07-30). These convert a path string and hand it back; nothing in the page says they read from or write to what the file records. `abspath` is documented as the way to turn a stored path into one Python can use, including the library case: "bpy.path.abspath(image.filepath, library=image.library)" (`https://docs.blender.org/api/4.5/info_gotchas_file_paths_and_encoding.html`, date read 2026-07-30). A helper for the other means, not a means.
- **`bpy_extras.io_utils.path_reference()` and `path_reference_copy()`**. Documented as "Return a filepath relative to a destination directory, **for use with exporters**" (`https://docs.blender.org/api/4.5/bpy_extras.io_utils.html`, date read 2026-07-30). It rewrites paths on their way into an exported file, not the locations recorded in the `.blend`. Out of scope on my reading of the question.
- **Path templates** (`{blend_name}`, `{fps}`, `{resolution_x}`). These are not a means of rewriting a stored location; they are substituted at time of use and the stored text stays as written: "This substitution happens internally at time of use (e.g. while rendering), so the template syntax will remain as-is in the path field." (`https://docs.blender.org/manual/en/4.5/files/file_paths.html`, date read 2026-07-30). The feature is new in this version and is limited: "Currently this is only enabled on the Scene's render output path and the output paths of the compositor's File Output node." (`https://developer.blender.org/docs/release_notes/4.5/core/`, date read 2026-07-30). Worth knowing because a stored path may hold template text rather than a literal location, and because `scene.render.frame_path()` "used to be infallible but can now fail if there are errors processing Path Templates in the render output path" (`https://developer.blender.org/docs/release_notes/4.5/python_api/`, date read 2026-07-30).
- **Cycles' own files.** I searched all of `cycles_4.5` for `FILE_PATH`, `DIR_PATH`, `blend_paths`, `make_paths` and `file_path_map`. The only path-typed properties are on two operators — `CYCLES_OT_denoise_animation` (`input_filepath`, `output_filepath`) and `CYCLES_OT_merge_images` (`input_filepath1`, `input_filepath2`, `output_filepath`), all `subtype='FILE_PATH'` in `operators.py`, lines 42–52 and 124–140. These are operator arguments, not settings stored in the file, and Cycles adds no listing or rewriting mechanism of its own. **LABEL: observed** (I read the files; I ran nothing).

One thing Cycles' files do settle, and it bears on the *kinds* in L3. `osl.py` resolves two of the recorded locations through Blender's blend-relative helper:

> `    script_path = bpy.path.abspath(filepath, library=library)`

(`cycles_4.5/osl.py`, line 169, inside `update_external_script`), reached with the shader script node's own path —

> `        ok, oso_path, oso_file_remove = update_external_script(report, node.filepath, node.id_data.library)`

(`cycles_4.5/osl.py`, line 250) — and with the camera's custom shader path —

> `        ok, oso_path, oso_file_remove = update_external_script(report, cam.custom_filepath, cam.library)`

(`cycles_4.5/osl.py`, line 306). **LABEL: observed** for the lines themselves. That `Camera.custom_filepath` therefore accepts a `//` path despite its API page not carrying the "blend relative // prefix supported" note is a step past the lines, and is **inferred**.

---

### Where I looked, and what I searched for

Searched across the whole of `manual_4.5`, `api_4.5`, `release_notes_4.5` and `cycles_4.5`:

`blend_paths`, `file_path_map`, `make_paths_relative`, `make_paths_absolute`, `find_missing_files`, `report_missing_files`, `pack_all`, `unpack_all`, `autopack`, `lib_relocate`, `lib_reload`, `lib_operation`, `id_linked_relocate`, `relocate`, `path_remap`, `relative_remap`, `path_reference`, `Remap Relative`, `Relative Paths`, `relative path`, `External Data`, `Report Missing Files`, `Find Missing Files`, `external file`, `remap`, `asset tracer`, `blender-asset-tracer`, `different drive`, `drive letter`, `same drive`, `background mode`, `--background`, `FILE_PATH`, `DIR_PATH`, and the exact phrase `blend relative // prefix supported`.

Pages opened in full or in part:

- Manual: `files/file_paths`, `files/blend/open_save`, `files/blend/packed_data`, `files/linked_libraries/index`, `files/linked_libraries/link_append`, `files/asset_libraries/introduction`, `interface/window_system/topbar`, `editors/outliner/interface`, `editors/file_browser`, `editors/preferences/save_load`, `advanced/command_line/arguments`.
- Python API: `bpy.utils`, `bpy.path`, `bpy.ops.file`, `bpy.ops.wm`, `bpy.ops.outliner`, `bpy.ops.buttons`, `bpy.types.BlendData`, `bpy.types.BlendDataLibraries`, `bpy.types.Library`, `bpy_extras.io_utils`, `info_gotchas_file_paths_and_encoding`, `info_gotchas_operators`, `change_log`, and the `bpy.types.*` page of every type that carries a blend-relative path property (41 pages found by the phrase search — corrected from 39 by the citation check, 2026-08-06; the table in L3 is drawn from the 34 of them that carry a stored path property).
- Release notes: `core`, `python_api`, `pipeline_assets_io`, `bugfixes`.
- Cycles: `operators.py`, `properties.py`, `osl.py`, `camera.py`, and a whole-folder search.
- Directory listing of the manual's `files/` section, to be sure no path page was missed.

Ruled out as not containing an answer: the import/export pages (`files/import_export/*`, `addons/import_export/*`) — they describe writing paths into exported files of other formats, not the locations recorded in a `.blend`. The `requirements` page — it is about hardware and operating systems.

**The gap I could not close.** I searched hard for a page that pairs the bulk mechanisms with the path kinds they cover — a list of what "all paths to external files" includes, or what `blend_paths()` counts as an external file. There is no such page in these sources. For L1, L5, R1 and R2 the coverage is stated only as "all", and I have not found any document that expands it. That is a real "not in the sources", not a gap in the searching, and it is the part of the question that matters most.

---

### Outside the interface

**Brought up to date by the hand tests of 2026-08-06 and 2026-08-07 (see the two observed sections below).** Every means named in this answer that has a Python form was run with no window at all, on the rented Linux machine, in this version. The two bulk sweeps, the missing-file search, the library relocation, saving elsewhere with the remap option, and the write call all returned `{'FINISHED'}` and did what they claim, with no interface present. Two operators did **not** on that machine: `bpy.ops.image.replace` and `bpy.ops.sequencer.change_path` both refused, naming a failed context check. The Mac sitting of 2026-08-07 took those two further, and they part company. **`bpy.ops.image.replace` needs a window**: with one on screen it returned `{'FINISHED'}` and rewrote the recorded path; with no window it refused on the Mac too, in every version. **`bpy.ops.sequencer.change_path` does not need a window**: with no window at all, given the file's name and the sequencer's own scene in the context, it returned `{'FINISHED'}` and rewrote the recorded folder and file, in every version. So this line is answered for everything here except that one operator, and that one is answered the other way. The words below are the researcher's own and are left as they were written.


Partly documented, partly not, and it splits by mechanism.

Documented that Blender can be driven from outside: "-b, --background / Run in background (often used for UI-less rendering)", "-P, --python <filepath>", and "-c, --command <command> / Run a command which consumes all remaining arguments. […] This implies --background mode." (manual, `https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html`, date read 2026-07-30). R7 (`-o, --render-output`) is a command-line means by definition and has no interface form.

L1, L2, L3/R3 (assigning to a property) and R5 are plain Python calls. Nothing in their pages ties them to a window, an editor or a selection.

R1, R2, R6 and the interface half of R3 are **operators**, and operators are the ones that can fail away from the interface. The API says so in general terms:

> "When calling an operator it gives an error like this:
>
> >>> bpy.ops.action.clean(threshold=0.001)
> RuntimeError: Operator bpy.ops.action.clean.poll() failed, context is incorrect
> […]
> Typically operators check for the active area type, a selection or active object they can operate on, but some operators are more strict when they run."

Source: `https://docs.blender.org/api/4.5/info_gotchas_operators.html`. Date read: 2026-07-30. That same page names `bpy.ops.buttons.file_browse` among the operators "only intended for use in a specific context […] only called from the properties editor", so the interface half of R3 does **not** hold outside the interface.

Whether `bpy.ops.file.make_paths_relative`, `make_paths_absolute`, `report_missing_files`, `find_missing_files` and `wm.lib_relocate` pass their checks with no window open is **not in the sources**. Two hints, and I label them as hints: `find_missing_files` and `lib_relocate` both take a `directory`/`filepath` argument, so they need not be given a directory by hand — but the manual describes `find_missing_files` as "A File Browser will show up", which is an interface behaviour. Concluding from the arguments that they run headless is **inferred**, and I would not act on it without the test in the list below.

R4 differs by which operator you call: `save_as_mainfile` defaults `relative_remap` to `True`, `save_mainfile` defaults it to `False`. A script must set it, not assume it.

L4 (the Outliner) and L5's warning "in the Info editor's header" are interface-only by description. L6 is outside Blender entirely.

### Operating system

**Brought up to date by the hand tests of 2026-08-06 and 2026-08-07 (see the two observed sections below).** Everything in this answer that could be run was run on the rented Linux machine, so **Linux is settled** for it. **macOS is now answered for the tests the Mac sitting ran**, and they are the ones this line turns on: the same file, built the same way, lists the same paths on the two machines — twenty-five of the twenty-seven printed lines are identical character for character. Two are not. The flat path list carries one extra entry on the Mac, a path into Blender's own application folder. And **whether capital letters in a name matter is answered differently on the two machines**: asked for its own image file with the name written in capitals, the Mac found it and the Linux machine did not. *(Citation check, 2026-08-06: narrowed. That check asks the operating system directly — `os.path.exists` on a name written in capitals — with none of Blender's own path machinery in it. It settles that the two file systems differ on letter case; it settles nothing about the means in this answer.)* Note what is still only Linux: the entries this round marked as running on both machines were in fact run on Linux alone, so for those the macOS half is untested rather than settled. **Windows stays untouched**: no Windows machine has run it, so the cross-drive question the sources raise was not answered by any test here. The words below are the researcher's own and are left as they were written.


The sources do not say that any of these means differs by operating system, and I found no page that says one does. What they do say, nearby:

- Paths are the one exception to Blender's UTF-8 rule: "Paths are an exception to this rule since the existence of non-UTF-8 paths on the user's file system cannot be ignored." — `https://docs.blender.org/api/4.5/info_gotchas_file_paths_and_encoding.html`, date read 2026-07-30. So a listed path can be one Python refuses to print, on any system where such names exist.
- `bpy.path.basename` is documented as "Equivalent to os.path.basename, but skips a "//" prefix. / Use for Windows compatibility." and `bpy.path.resolve_ncase` as "Resolve a case insensitive path on a case sensitive system" — `https://docs.blender.org/api/4.5/bpy.path.html`, date read 2026-07-30. Both exist because path handling is not uniform across systems, but neither statement is about the means above.
- The nearest documented drive-boundary limit belongs to the exporters, not to these means: the OBJ and FBX "Relative" path mode "Uses relative paths in every case (except when on a different drive on Windows)." — `https://docs.blender.org/manual/en/4.5/files/import_export/obj.html` and `https://docs.blender.org/manual/en/4.5/addons/import_export/scene_fbx.html`, date read 2026-07-30. Whether R1 (Make Paths Relative) hits the same wall on Windows is **not in the sources**.
- A few command-line arguments are marked as belonging to particular systems ("/? Print this help text and exit (Windows only)", "-r, --register … (Windows & Linux only)"), but none of them concern paths recorded in a file — `https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html`, date read 2026-07-30.

**Not covered.** This answer settles what mechanisms the 4.5 documentation names, and what each one's own page claims. It does **not** settle: which path kinds `blend_paths()`, `file_path_map()`, Make Paths Relative/Absolute, Report Missing Files and Find Missing Files actually visit; whether those five agree with each other; whether any of them reaches paths held inside a *linked* library rather than the open file; whether the operators run with no window; whether `-o` leaves a changed value in a file that is then saved; what Blender Asset Tracer reaches; and whether a stored path holding template text is treated as a path by any of these means.

---

### Observed — the hand tests of 2026-08-06

Everything in this section was run on **the rented Linux machine — Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB on driver 580.95.05, an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory and no display server at all**. The Blender was `/opt/blender/4.5/blender`, which reported itself as **4.5.12 LTS**, hash `84afd5f785f7`. Every run used `-b` (no window) and `--factory-startup`, and every run had `BLENDER_USER_RESOURCES` pointed at `/root/reading-a-file/res-4.5`, a settings folder made for this round and for nothing else — never at the real one. All work was done inside `/root/reading-a-file/`. The machine is shared with other topics' tests, so nothing here says how long anything took. Its graphics card could not be questioned during this sitting (`nvidia-smi` answered `Failed to initialize NVML: Driver/library version mismatch`); no test below touches the card. The full list of tests, with the set-up and the command for each, is `Audits/Blender Research/Reading a file/Hand tests — added questions.md`. **Nothing below was run on the Creator's Mac**, so nothing below settles macOS; a second sitting does that half.

#### What the machinery is called here, and what its switches are (list entry 1)

Two scripts were run with no file loaded, asking whether each named call exists and what it accepts. What came back, word for word:

```
PROBE version 4.5.12 LTS
PROBE has bpy.utils.blend_paths = True
PROBE has bpy.data.file_path_map = True
PROBE has bpy.data.file_path_foreach = False
PROBE has bpy.data.user_map = True
PROBE has bpy.data.all_ids = False
PROBE blend_paths positional = ERROR TypeError blend_paths() takes no positional arguments
PROBE Property.is_path_output = True
PROBE Property.is_path_supports_blend_relative = True
PROBE Property.is_path_supports_templates = True
PROBE Property.subtype = True
PROBE file_path_foreach flags = ERROR KeyError 'bpy_prop_collection[key]: key "file_path_foreach" not found'
PROBE file_path_map params = ERROR KeyError 'bpy_prop_collection[key]: key "file_path_map" not found'
PROBE bpy.types.BlendDataPathMeta = False
PROBE path_remap = ERROR KeyError 'bpy_prop_collection[key]: key "write" not found'
PROBE CompositorNodeOutputFile has base_path = True has directory = False
PROBE Camera.custom_filepath = True
PROBE Text.filepath = True
PROBE TextureNodeOutput.filepath = True
PROBE Theme.filepath = True
PROBE RenderSettings.filepath.is_path_output = True
PROBE RenderSettings.filepath.is_path_supports_blend_relative = True
PROBE RenderSettings.filepath.is_path_supports_templates = True
PROBE RenderSettings.filepath.subtype = FILE_PATH
PROBE2 version 4.5.12 LTS
PROBE2 file_path_foreach = NOT PRESENT
PROBE2 path_remap 'NONE' = ACCEPTED
PROBE2 path_remap 'RELATIVE' = ACCEPTED
PROBE2 path_remap 'RELATIVE_ALL' = ACCEPTED
PROBE2 path_remap 'ABSOLUTE' = ACCEPTED
PROBE2 path_remap False = REJECTED ValueError expected a string, got bool
PROBE2 CompositorNodeOutputFile base_path = True directory = False
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
READ | Image.filepath                           | /root/reading-a-file/work/many/4.5/assets/probe.png
READ | Image.filepath_raw                       | /root/reading-a-file/work/many/4.5/assets/probe.png
READ | Sound.filepath                           | /root/reading-a-file/work/many/4.5/assets/probe.wav
READ | VectorFont.filepath                      | /root/reading-a-file/work/many/4.5/assets/probe.woff2
READ | Text.filepath                            | /root/reading-a-file/work/many/4.5/assets/probe.txt
READ | Volume.filepath                          | /root/reading-a-file/work/many/4.5/assets/probe.vdb
READ | MovieClip.filepath                       | /root/reading-a-file/work/many/4.5/assets/probe.png
READ | CacheFile.filepath                       | /root/reading-a-file/work/many/4.5/assets/probe.abc
READ | Library.filepath                         | /root/reading-a-file/work/many/4.5/lib.blend
READ | RenderSettings.filepath                  | /root/reading-a-file/work/many/4.5/assets/render_out/
READ | BakeSettings.filepath                    | /root/reading-a-file/work/many/4.5/assets/bake_out/
READ | CompositorNodeOutputFile base path       | /root/reading-a-file/work/many/4.5/assets/node_out/
READ | PointCache.filepath (cloth)              | /root/reading-a-file/work/many/4.5/assets/pointcache/
READ | FluidDomainSettings.cache_directory      | /root/reading-a-file/work/many/4.5/assets/fluidcache/
READ | NodesModifier.bake_directory             | /root/reading-a-file/work/many/4.5/assets/gnbake/
READ | MultiresModifier.filepath                | 
READ | MeshCacheModifier.filepath               | /root/reading-a-file/work/many/4.5/assets/meshcache.mdd
READ | OceanModifier.filepath                   | /root/reading-a-file/work/many/4.5/assets/oceanbake/
READ | ShaderNodeScript.filepath                | /root/reading-a-file/work/many/4.5/assets/probe.osl
READ | ShaderNodeTexIES.filepath                | /root/reading-a-file/work/many/4.5/assets/probe.ies
READ | Camera.custom_filepath                   | /root/reading-a-file/work/many/4.5/assets/camshader.osl
READ | custom property on an object             | /root/reading-a-file/work/many/4.5/assets/custom_prop.png
READ | ImageStrip.directory                     | /root/reading-a-file/work/many/4.5/assets/stripdir/
READ | StripProxy.directory (sequencer)         | /root/reading-a-file/work/many/4.5/assets/proxydir/
```

And the listing means:

```
LIST blend_paths()                 = ['/opt/blender/4.5/4.5/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/root/reading-a-file/work/many/4.5/assets/camshader.osl', '/root/reading-a-file/work/many/4.5/assets/fluidcache/', '/root/reading-a-file/work/many/4.5/assets/meshcache.mdd', '/root/reading-a-file/work/many/4.5/assets/oceanbake/', '/root/reading-a-file/work/many/4.5/assets/probe.abc', '/root/reading-a-file/work/many/4.5/assets/probe.ies', '/root/reading-a-file/work/many/4.5/assets/probe.osl', '/root/reading-a-file/work/many/4.5/assets/probe.png', '/root/reading-a-file/work/many/4.5/assets/probe.png', '/root/reading-a-file/work/many/4.5/assets/probe.txt', '/root/reading-a-file/work/many/4.5/assets/probe.vdb', '/root/reading-a-file/work/many/4.5/assets/probe.wav', '/root/reading-a-file/work/many/4.5/assets/probe.woff2', '/root/reading-a-file/work/many/4.5/assets/stripdir/probe.png', '/root/reading-a-file/work/many/4.5/lib.blend']
LIST blend_paths(packed=True)      = ['/opt/blender/4.5/4.5/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/root/reading-a-file/work/many/4.5/assets/camshader.osl', '/root/reading-a-file/work/many/4.5/assets/fluidcache/', '/root/reading-a-file/work/many/4.5/assets/meshcache.mdd', '/root/reading-a-file/work/many/4.5/assets/oceanbake/', '/root/reading-a-file/work/many/4.5/assets/probe.abc', '/root/reading-a-file/work/many/4.5/assets/probe.ies', '/root/reading-a-file/work/many/4.5/assets/probe.osl', '/root/reading-a-file/work/many/4.5/assets/probe.png', '/root/reading-a-file/work/many/4.5/assets/probe.png', '/root/reading-a-file/work/many/4.5/assets/probe.txt', '/root/reading-a-file/work/many/4.5/assets/probe.vdb', '/root/reading-a-file/work/many/4.5/assets/probe.wav', '/root/reading-a-file/work/many/4.5/assets/probe.woff2', '/root/reading-a-file/work/many/4.5/assets/stripdir/probe.png', '/root/reading-a-file/work/many/4.5/lib.blend']
LIST blend_paths(local=True)       = ['/opt/blender/4.5/4.5/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/root/reading-a-file/work/many/4.5/assets/camshader.osl', '/root/reading-a-file/work/many/4.5/assets/fluidcache/', '/root/reading-a-file/work/many/4.5/assets/meshcache.mdd', '/root/reading-a-file/work/many/4.5/assets/oceanbake/', '/root/reading-a-file/work/many/4.5/assets/probe.abc', '/root/reading-a-file/work/many/4.5/assets/probe.ies', '/root/reading-a-file/work/many/4.5/assets/probe.osl', '/root/reading-a-file/work/many/4.5/assets/probe.png', '/root/reading-a-file/work/many/4.5/assets/probe.png', '/root/reading-a-file/work/many/4.5/assets/probe.txt', '/root/reading-a-file/work/many/4.5/assets/probe.vdb', '/root/reading-a-file/work/many/4.5/assets/probe.wav', '/root/reading-a-file/work/many/4.5/assets/probe.woff2', '/root/reading-a-file/work/many/4.5/assets/stripdir/probe.png', '/root/reading-a-file/work/many/4.5/lib.blend']
LIST blend_paths(absolute=True)    = ['/opt/blender/4.5/4.5/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/root/reading-a-file/work/many/4.5/assets/camshader.osl', '/root/reading-a-file/work/many/4.5/assets/fluidcache/', '/root/reading-a-file/work/many/4.5/assets/meshcache.mdd', '/root/reading-a-file/work/many/4.5/assets/oceanbake/', '/root/reading-a-file/work/many/4.5/assets/probe.abc', '/root/reading-a-file/work/many/4.5/assets/probe.ies', '/root/reading-a-file/work/many/4.5/assets/probe.osl', '/root/reading-a-file/work/many/4.5/assets/probe.png', '/root/reading-a-file/work/many/4.5/assets/probe.png', '/root/reading-a-file/work/many/4.5/assets/probe.txt', '/root/reading-a-file/work/many/4.5/assets/probe.vdb', '/root/reading-a-file/work/many/4.5/assets/probe.wav', '/root/reading-a-file/work/many/4.5/assets/probe.woff2', '/root/reading-a-file/work/many/4.5/assets/stripdir/probe.png', '/root/reading-a-file/work/many/4.5/lib.blend']
LIST file_path_map()               = ['/root/reading-a-file/work/many/4.5/assets/camshader.osl', '/root/reading-a-file/work/many/4.5/assets/fluidcache/', '/root/reading-a-file/work/many/4.5/assets/meshcache.mdd', '/root/reading-a-file/work/many/4.5/assets/oceanbake/', '/root/reading-a-file/work/many/4.5/assets/probe.abc', '/root/reading-a-file/work/many/4.5/assets/probe.ies', '/root/reading-a-file/work/many/4.5/assets/probe.osl', '/root/reading-a-file/work/many/4.5/assets/probe.png', '/root/reading-a-file/work/many/4.5/assets/probe.txt', '/root/reading-a-file/work/many/4.5/assets/probe.vdb', '/root/reading-a-file/work/many/4.5/assets/probe.wav', '/root/reading-a-file/work/many/4.5/assets/probe.woff2', '/root/reading-a-file/work/many/4.5/assets/stripdir/probe.png', '/root/reading-a-file/work/many/4.5/lib.blend']
LIST file_path_map(include_libraries=True) = ['/root/reading-a-file/work/many/4.5/assets/camshader.osl', '/root/reading-a-file/work/many/4.5/assets/fluidcache/', '/root/reading-a-file/work/many/4.5/assets/meshcache.mdd', '/root/reading-a-file/work/many/4.5/assets/oceanbake/', '/root/reading-a-file/work/many/4.5/assets/probe.abc', '/root/reading-a-file/work/many/4.5/assets/probe.ies', '/root/reading-a-file/work/many/4.5/assets/probe.osl', '/root/reading-a-file/work/many/4.5/assets/probe.png', '/root/reading-a-file/work/many/4.5/assets/probe.txt', '/root/reading-a-file/work/many/4.5/assets/probe.vdb', '/root/reading-a-file/work/many/4.5/assets/probe.wav', '/root/reading-a-file/work/many/4.5/assets/probe.woff2', '/root/reading-a-file/work/many/4.5/assets/stripdir/probe.png', '/root/reading-a-file/work/many/4.5/lib.blend']
LIST file_path_map by data-block   = [('FluidObj', ['/root/reading-a-file/work/many/4.5/assets/fluidcache/']), ('LibColl', ['/root/reading-a-file/work/many/4.5/lib.blend']), ('LibCube', ['/root/reading-a-file/work/many/4.5/lib.blend']), ('LibMesh', ['/root/reading-a-file/work/many/4.5/lib.blend']), ('ModObj', ['/root/reading-a-file/work/many/4.5/assets/meshcache.mdd', '/root/reading-a-file/work/many/4.5/assets/oceanbake/']), ('ProbeCam', ['/root/reading-a-file/work/many/4.5/assets/camshader.osl']), ('ProbeClip', ['/root/reading-a-file/work/many/4.5/assets/probe.png']), ('ProbeFont', ['/root/reading-a-file/work/many/4.5/assets/probe.woff2']), ('ProbeImage', ['/root/reading-a-file/work/many/4.5/assets/probe.png']), ('ProbeMat', ['/root/reading-a-file/work/many/4.5/assets/probe.ies', '/root/reading-a-file/work/many/4.5/assets/probe.osl']), ('ProbeSound', ['/root/reading-a-file/work/many/4.5/assets/probe.wav']), ('ProbeText', ['/root/reading-a-file/work/many/4.5/assets/probe.txt']), ('ProbeVolume', ['/root/reading-a-file/work/many/4.5/assets/probe.vdb']), ('Scene', ['/root/reading-a-file/work/many/4.5/assets/stripdir/probe.png']), ('lib.blend', ['/root/reading-a-file/work/many/4.5/lib.blend']), ('probe.abc', ['/root/reading-a-file/work/many/4.5/assets/probe.abc'])]
LIST file_path_foreach             = NOT PRESENT
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
AFTER | RenderSettings.filepath                  | /root/reading-a-file/work/many/4.5/assets/render_out/
AFTER | BakeSettings.filepath                    | /root/reading-a-file/work/many/4.5/assets/bake_out/
AFTER | CompositorNodeOutputFile base path       | /root/reading-a-file/work/many/4.5/assets/node_out/
AFTER | PointCache.filepath (cloth)              | /root/reading-a-file/work/many/4.5/assets/pointcache/
AFTER | FluidDomainSettings.cache_directory      | //assets/fluidcache/
AFTER | NodesModifier.bake_directory             | /root/reading-a-file/work/many/4.5/assets/gnbake/
AFTER | MultiresModifier.filepath                | 
AFTER | MeshCacheModifier.filepath               | //assets/meshcache.mdd
AFTER | OceanModifier.filepath                   | //assets/oceanbake/
AFTER | ShaderNodeScript.filepath                | //assets/probe.osl
AFTER | ShaderNodeTexIES.filepath                | //assets/probe.ies
AFTER | Camera.custom_filepath                   | //assets/camshader.osl
AFTER | custom property on an object             | /root/reading-a-file/work/many/4.5/assets/custom_prop.png
AFTER | ImageStrip.directory                     | //assets/stripdir/
AFTER | StripProxy.directory (sequencer)         | /root/reading-a-file/work/many/4.5/assets/proxydir/
```

Reading that off, without adding anything to it: the ones whose recorded text changed were **CacheFile.filepath, Camera.custom_filepath, FluidDomainSettings.cache_directory, Image.filepath, Image.filepath_raw, ImageStrip.directory, Library.filepath, MeshCacheModifier.filepath, MovieClip.filepath, OceanModifier.filepath, ShaderNodeScript.filepath, ShaderNodeTexIES.filepath, Sound.filepath, Text.filepath, VectorFont.filepath, Volume.filepath**. The ones whose recorded text is the same after as before were **BakeSettings.filepath, CompositorNodeOutputFile base path, NodesModifier.bake_directory, PointCache.filepath (cloth), RenderSettings.filepath, StripProxy.directory (sequencer), custom property on an object**. (The multi-resolution modifier's line is empty in both and is not counted either way, for the reason given above.)

#### What Make Paths Absolute reaches (list entry 5)

The file the last test saved, opened again, and `bpy.ops.file.make_paths_absolute()` called on it:

```
OP make_paths_absolute = {'FINISHED'}
AFTER | Image.filepath                           | /root/reading-a-file/work/many/4.5/assets/probe.png
AFTER | Image.filepath_raw                       | /root/reading-a-file/work/many/4.5/assets/probe.png
AFTER | Sound.filepath                           | /root/reading-a-file/work/many/4.5/assets/probe.wav
AFTER | VectorFont.filepath                      | /root/reading-a-file/work/many/4.5/assets/probe.woff2
AFTER | Text.filepath                            | /root/reading-a-file/work/many/4.5/assets/probe.txt
AFTER | Volume.filepath                          | /root/reading-a-file/work/many/4.5/assets/probe.vdb
AFTER | MovieClip.filepath                       | /root/reading-a-file/work/many/4.5/assets/probe.png
AFTER | CacheFile.filepath                       | /root/reading-a-file/work/many/4.5/assets/probe.abc
AFTER | Library.filepath                         | /root/reading-a-file/work/many/4.5/lib.blend
AFTER | RenderSettings.filepath                  | /root/reading-a-file/work/many/4.5/assets/render_out/
AFTER | BakeSettings.filepath                    | /root/reading-a-file/work/many/4.5/assets/bake_out/
AFTER | CompositorNodeOutputFile base path       | /root/reading-a-file/work/many/4.5/assets/node_out/
AFTER | PointCache.filepath (cloth)              | /root/reading-a-file/work/many/4.5/assets/pointcache/
AFTER | FluidDomainSettings.cache_directory      | /root/reading-a-file/work/many/4.5/assets/fluidcache/
AFTER | NodesModifier.bake_directory             | /root/reading-a-file/work/many/4.5/assets/gnbake/
AFTER | MultiresModifier.filepath                | 
AFTER | MeshCacheModifier.filepath               | /root/reading-a-file/work/many/4.5/assets/meshcache.mdd
AFTER | OceanModifier.filepath                   | /root/reading-a-file/work/many/4.5/assets/oceanbake/
AFTER | ShaderNodeScript.filepath                | /root/reading-a-file/work/many/4.5/assets/probe.osl
AFTER | ShaderNodeTexIES.filepath                | /root/reading-a-file/work/many/4.5/assets/probe.ies
AFTER | Camera.custom_filepath                   | /root/reading-a-file/work/many/4.5/assets/camshader.osl
AFTER | custom property on an object             | /root/reading-a-file/work/many/4.5/assets/custom_prop.png
AFTER | ImageStrip.directory                     | /root/reading-a-file/work/many/4.5/assets/stripdir/
AFTER | StripProxy.directory (sequencer)         | /root/reading-a-file/work/many/4.5/assets/proxydir/
```

#### The one-pass path visitor, and whether returning a string rewrites (list entry 6)

This call does not exist in this version, so the test could not be run here. What came back:

```
FEW file_path_foreach = NOT PRESENT
```

#### Whether walking the property marks finds every recorded location (list entry 7)

A walk over thirteen of `bpy.data`'s collections, four levels deep, collecting every text setting whose subtype is `FILE_PATH` or `DIR_PATH` or which carries one of the two path marks. The walk found the same distinct values in every version. The counts it printed here:

```
SWEEP COUNT 94
SWEEP distinct values 22
```

and the distinct values it reached, one line each, showing where the walk found them. The last two or three lines are settings that carry a path mark but hold something that is not a path — a data-block name, or the bare file-name part of an output setting — and the custom property is on the end, found only by listing custom properties by hand and not by the walk at all:

```
SWEEP bpy.data.materials['Material'].library_weak_reference.filepath = /opt/blender/4.5/4.5/datafiles/assets/brushes/essentials_brushes-gp_draw.blend
SWEEP bpy.data.scenes['Scene'].render.bake.filepath = /root/reading-a-file/work/many/4.5/assets/bake_out/
SWEEP bpy.data.cameras['ProbeCam'].custom_filepath = /root/reading-a-file/work/many/4.5/assets/camshader.osl
SWEEP custom property bpy.data.objects['ModObj']['probe_path'] = /root/reading-a-file/work/many/4.5/assets/custom_prop.png
SWEEP bpy.data.objects['FluidObj'].modifiers[].domain_settings.cache_directory = /root/reading-a-file/work/many/4.5/assets/fluidcache/
SWEEP bpy.data.objects['GNObj'].modifiers[].bake_directory = /root/reading-a-file/work/many/4.5/assets/gnbake/
SWEEP bpy.data.objects['ModObj'].modifiers[].filepath = /root/reading-a-file/work/many/4.5/assets/meshcache.mdd
SWEEP bpy.data.scenes['Scene'].node_tree.nodes[].base_path = /root/reading-a-file/work/many/4.5/assets/node_out/
SWEEP bpy.data.objects['ModObj'].modifiers[].filepath = /root/reading-a-file/work/many/4.5/assets/oceanbake/
SWEEP bpy.data.objects['ClothObj'].modifiers[].point_cache.filepath = /root/reading-a-file/work/many/4.5/assets/pointcache/
SWEEP bpy.data.cache_files['probe.abc'].filepath = /root/reading-a-file/work/many/4.5/assets/probe.abc
SWEEP bpy.data.materials['ProbeMat'].node_tree.nodes[].filepath = /root/reading-a-file/work/many/4.5/assets/probe.ies
SWEEP bpy.data.materials['ProbeMat'].node_tree.nodes[].filepath = /root/reading-a-file/work/many/4.5/assets/probe.osl
SWEEP bpy.data.images['ProbeImage'].filepath = /root/reading-a-file/work/many/4.5/assets/probe.png
SWEEP bpy.data.volumes['ProbeVolume'].filepath = /root/reading-a-file/work/many/4.5/assets/probe.vdb
SWEEP bpy.data.sounds['ProbeSound'].filepath = /root/reading-a-file/work/many/4.5/assets/probe.wav
SWEEP bpy.data.fonts['ProbeFont'].filepath = /root/reading-a-file/work/many/4.5/assets/probe.woff2
SWEEP bpy.data.scenes['Scene'].sequence_editor.proxy_dir = /root/reading-a-file/work/many/4.5/assets/proxydir/
SWEEP bpy.data.scenes['Scene'].render.filepath = /root/reading-a-file/work/many/4.5/assets/render_out/
SWEEP bpy.data.scenes['Scene'].sequence_editor.sequences[].directory = /root/reading-a-file/work/many/4.5/assets/stripdir/
SWEEP bpy.data.libraries['lib.blend'].filepath = /root/reading-a-file/work/many/4.5/lib.blend
SWEEP bpy.data.scenes['Scene'].node_tree.nodes[].file_slots[].path = Image
SWEEP bpy.data.materials['Material'].library_weak_reference.id_name = MAMaterial
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
PACK PackedImage.filepath = /root/reading-a-file/work/packed/4.5/assets/packed.png
PACK PlainImage.filepath  = /root/reading-a-file/work/packed/4.5/assets/plain.png
PACK blend_paths()            = ['/opt/blender/4.5/4.5/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/opt/blender/4.5/4.5/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/root/reading-a-file/work/packed/4.5/assets/plain.png']
PACK blend_paths(packed=True) = ['/opt/blender/4.5/4.5/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/opt/blender/4.5/4.5/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/root/reading-a-file/work/packed/4.5/assets/packed.png', '/root/reading-a-file/work/packed/4.5/assets/plain.png']
PACK file_path_map()          = [('PlainImage', ['/root/reading-a-file/work/packed/4.5/assets/plain.png'])]
PACK file_path_foreach = NOT PRESENT
```

#### Whether unpacking rewrites the recorded location (list entry 10)

Two copies of that file, unpacked two ways, so neither run could affect the other. The `on disk` listing of the second run shows the folder the first run made; the two runs used different copies of the file and the data did not cross.

```
--- unpack USE_LOCAL ---
UNPACK before, filepath = /root/reading-a-file/work/packed/4.5/assets/packed.png | packed = True
UNPACK op(USE_LOCAL) = {'FINISHED'}
UNPACK after,  filepath = //textures/packed.png | packed = False
UNPACK on disk: <W>/packed.blend
UNPACK on disk: <W>/u1.blend
UNPACK on disk: <W>/u2.blend
UNPACK on disk: <W>/assets/packed.png
UNPACK on disk: <W>/assets/plain.png
UNPACK on disk: <W>/textures/packed.png
--- unpack USE_ORIGINAL ---
UNPACK before, filepath = /root/reading-a-file/work/packed/4.5/assets/packed.png | packed = True
UNPACK op(USE_ORIGINAL) = {'FINISHED'}
UNPACK after,  filepath = /root/reading-a-file/work/packed/4.5/assets/packed.png | packed = False
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
FINDALL built FromOne = /root/reading-a-file/work/findall/4.5/one/same.png
FINDALL built FromTwo = /root/reading-a-file/work/findall/4.5/two/same.png
FINDALL save = {'FINISHED'}
FINDALL before FromOne = /root/reading-a-file/work/findall/4.5/one/same.png True
FINDALL before FromTwo = /root/reading-a-file/work/findall/4.5/two/same.png True
FINDALL op = {'FINISHED'}
FINDALL after  FromOne = /root/reading-a-file/work/findall/4.5/one/same.png
FINDALL after  FromTwo = /root/reading-a-file/work/findall/4.5/one/same.png
```

#### A path holding a template, through the listing means and both sweeps (list entry 12)

```
TMPL save = {'FINISHED'}
TMPL OPENED   render.filepath = /root/reading-a-file/work/template/4.5/assets/out_{blend_name}_####.png
TMPL OPENED   node base path  = /root/reading-a-file/work/template/4.5/assets/node_{blend_name}/
TMPL OPENED   image           = /root/reading-a-file/work/template/4.5/assets/probe.png
TMPL OPENED   blend_paths     = ['/opt/blender/4.5/4.5/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/root/reading-a-file/work/template/4.5/assets/probe.png']
TMPL OPENED   file_path_map   = [['/root/reading-a-file/work/template/4.5/assets/probe.png']]
TMPL OPENED   frame_path(1)   = /root/reading-a-file/work/template/4.5/assets/out_tmpl_0001.png
TMPL op make_paths_relative = {'FINISHED'}
TMPL REL      render.filepath = /root/reading-a-file/work/template/4.5/assets/out_{blend_name}_####.png
TMPL REL      node base path  = /root/reading-a-file/work/template/4.5/assets/node_{blend_name}/
TMPL REL      image           = //assets/probe.png
TMPL REL      blend_paths     = ['//../../../../../opt/blender/4.5/4.5/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '//assets/probe.png']
TMPL REL      file_path_map   = [['//assets/probe.png']]
TMPL REL      frame_path(1)   = /root/reading-a-file/work/template/4.5/assets/out_tmpl_0001.png
TMPL op make_paths_absolute = {'FINISHED'}
TMPL ABS      render.filepath = /root/reading-a-file/work/template/4.5/assets/out_{blend_name}_####.png
TMPL ABS      node base path  = /root/reading-a-file/work/template/4.5/assets/node_{blend_name}/
TMPL ABS      image           = /root/reading-a-file/work/template/4.5/assets/probe.png
TMPL ABS      blend_paths     = ['/opt/blender/4.5/4.5/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/root/reading-a-file/work/template/4.5/assets/probe.png']
TMPL ABS      file_path_map   = [['/root/reading-a-file/work/template/4.5/assets/probe.png']]
TMPL ABS      frame_path(1)   = /root/reading-a-file/work/template/4.5/assets/out_tmpl_0001.png
```

#### The four remap choices on the write call (list entry 13)

A file holding one image recorded relative and one recorded in full form, both naming files that exist. Each of the four choices wrote a new file one folder below. Each written file was then **opened** rather than linked from, because linking turns a recorded relative path into a full one on the way in and would hide the thing being asked about. Each line gives the stored text, what it works out to, and whether a file is there:

```
WREAD NONE.blend = [('AbsImage', ('/root/reading-a-file/work/write/4.5/assets/abs.png', '/root/reading-a-file/work/write/4.5/assets/abs.png', True)), ('RelImage', ('//assets/rel.png', '/root/reading-a-file/work/write/4.5/out/assets/rel.png', False))]
WREAD RELATIVE.blend = [('AbsImage', ('/root/reading-a-file/work/write/4.5/assets/abs.png', '/root/reading-a-file/work/write/4.5/assets/abs.png', True)), ('RelImage', ('//../assets/rel.png', '/root/reading-a-file/work/write/4.5/out/../assets/rel.png', True))]
WREAD RELATIVE_ALL.blend = [('AbsImage', ('//../assets/abs.png', '/root/reading-a-file/work/write/4.5/out/../assets/abs.png', True)), ('RelImage', ('//assets/rel.png', '/root/reading-a-file/work/write/4.5/out/assets/rel.png', False))]
WREAD ABSOLUTE.blend = [('AbsImage', ('/root/reading-a-file/work/write/4.5/assets/abs.png', '/root/reading-a-file/work/write/4.5/assets/abs.png', True)), ('RelImage', ('/root/reading-a-file/work/write/4.5/assets/rel.png', '/root/reading-a-file/work/write/4.5/assets/rel.png', True))]
```

#### Whether the write call may be pointed at a file that already exists (list entry 14)

```
WRITE existing file size = 27
WRITE onto existing file = no error raised
WRITE existing file size before = 27 after = 137016
WRITE existing file first bytes after = b'BLENDER-v405'
```

#### What Save As with Remap Relative moves (list entry 15)

```
SAVEAS save2 = {'FINISHED'}
SAVEAS OPENED   RelImage = //assets/rel.png
SAVEAS OPENED   AbsImage = /root/reading-a-file/work/saveas/4.5/assets/abs.png
SAVEAS OPENED   render   = //renders/
SAVEAS op = {'FINISHED'}
SAVEAS AFTER    RelImage = //../assets/rel.png
SAVEAS AFTER    AbsImage = /root/reading-a-file/work/saveas/4.5/assets/abs.png
SAVEAS AFTER    render   = //renders/
SAVEAS new file location = /root/reading-a-file/work/saveas/4.5/elsewhere/moved.blend
```

#### Writing the library's own path, against relocating it (list entry 16)

Three files: `B.blend` holding an object called `CubeFromB`, `C.blend` holding one called `CubeFromC`, and `A.blend` linking `B`'s collection. Two separate runs from the same `A.blend` — the first assigns the path and then calls `reload()`, the second calls the relocate operator:

```
RELOC OPENED   library = /root/reading-a-file/work/reloc/4.5/B.blend
RELOC OPENED   objects = [('Camera', None), ('Cube', None), ('CubeFromB', '/root/reading-a-file/work/reloc/4.5/B.blend'), ('Light', None)]
RELOC ASSIGNED library = /root/reading-a-file/work/reloc/4.5/C.blend
RELOC ASSIGNED objects = [('Camera', None), ('Cube', None), ('CubeFromB', '/root/reading-a-file/work/reloc/4.5/C.blend'), ('Light', None)]
RELOC reload() = returned
RELOC RELOADED library = /root/reading-a-file/work/reloc/4.5/C.blend
RELOC RELOADED objects = [('Camera', None), ('Cube', None), ('CubeFromC', '/root/reading-a-file/work/reloc/4.5/C.blend'), ('Light', None)]
RELOC OPENED   library = /root/reading-a-file/work/reloc/4.5/B.blend
RELOC OPENED   objects = [('Camera', None), ('Cube', None), ('CubeFromB', '/root/reading-a-file/work/reloc/4.5/B.blend'), ('Light', None)]
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
NOSAVE rel    image before = /root/reading-a-file/work/many/4.5/assets/probe.png
NOSAVE rel    render before = /root/reading-a-file/work/many/4.5/assets/render_out/
NOSAVE op = {'FINISHED'}
NOSAVE after, not saved, image = //../../many/4.5/assets/probe.png
RUN 2 --- the same file opened again
NOSAVE check  image before = /root/reading-a-file/work/many/4.5/assets/probe.png
NOSAVE check  render before = /root/reading-a-file/work/many/4.5/assets/render_out/
RUN 3 --- the same file with -o //NOSAVE_CLI/ and no save
NOSAVE check  image before = /root/reading-a-file/work/many/4.5/assets/probe.png
NOSAVE check  render before = //NOSAVE_CLI/
RUN 4 --- the same file opened again, no argument
NOSAVE check  image before = /root/reading-a-file/work/many/4.5/assets/probe.png
NOSAVE check  render before = /root/reading-a-file/work/many/4.5/assets/render_out/
```

#### Whether a bulk rewrite reaches the file on disk without a save (list entry 18)

Runs 1 and 2 of the four just above are this test. Run 1 shows the rewrite taking hold in the running Blender — the `after, not saved` line — and run 2 shows what the file on disk still holds when it is opened again.

#### Turning a stored path into a real one, and the library it came from (list entry 19)

A library in its own folder holding an image recorded as `//libtex.png`, and a file one folder up that links it. The two folders differ on purpose, or the two calls would have no reason to differ:

```
AP own stored          = //assets/own.png
AP own abspath()       = /root/reading-a-file/work/abs/4.5/assets/own.png
AP own relpath(abs)    = //assets/own.png
AP own file exists     = True
AP linked name         = LibTex library = /root/reading-a-file/work/abs/4.5/libdir/lib.blend
AP linked stored       = //libtex.png
AP linked abspath()    = /root/reading-a-file/work/abs/4.5/libtex.png
AP linked abspath(lib) = /root/reading-a-file/work/abs/4.5/libdir/libtex.png
AP linked exists plain = False
AP linked exists lib   = True
```

#### A reference held only inside a node group, a driver or a custom property (list entry 20)

This one answers question 7 of the sheet rather than question 9, and is written into `Preparation/Blender Research/Findings/4.5/Reading a file.md` as well. Four images: one used straight from a material, one used only by an image node inside a node group, one named only as a driver's target, one held only by a custom property that points at the data-block:

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
UM file_path_map = [('Img_custom', ['/root/reading-a-file/work/um/4.5/assets/custom.png']), ('Img_driver', ['/root/reading-a-file/work/um/4.5/assets/driver.png']), ('Img_group', ['/root/reading-a-file/work/um/4.5/assets/group.png']), ('Img_plain', ['/root/reading-a-file/work/um/4.5/assets/plain.png'])]
```

#### What could not be done here (list entries 21 and 22, and the cannot-be-run section)

**Replacing the file an image or a strip is loaded from.** Both operators refuse to run with no window on this machine, when called as they are called here. That refusal is itself the result, and it is what makes this a Mac entry rather than a Linux one. Read it with the Mac section below, which asked the same two operators again: one of them wanted a window and the other wanted the file named and the sequencer's own scene put into the context, and once given those it ran with no window.

```
REPL before image = /root/reading-a-file/work/replace/4.5/first/a.png
REPL op image.replace = ERROR RuntimeError Operator bpy.ops.image.replace.poll() failed, context is incorrect
REPL after  image = /root/reading-a-file/work/replace/4.5/first/a.png
REPL before strip = /root/reading-a-file/work/replace/4.5/first/ | ['a.png']
REPL op sequencer.change_path = ERROR RuntimeError Operator bpy.ops.sequencer.change_path.poll() failed, context is incorrect
REPL after  strip = /root/reading-a-file/work/replace/4.5/first/ | ['a.png']
```

**The Outliner's Data API view** cannot be reached here at all — the machine has no display server. It was reached on the Creator's Mac on 2026-08-07; the pictures and what they show are in the Mac section below.

**Anything on Windows.** No Windows machine has run it, so whether making paths relative works across two drives on Windows was not reached by any test here. That is why it is unanswered rather than untried. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

**What Blender Asset Tracer reaches.** It is not part of Blender and is not in `Blender Sources/`; reaching it would mean fetching software from outside the project. Not attempted, and recorded as out of scope.

**The multi-resolution modifier's own path.** The property would not take a value in any version, so no test here could act on it. Nothing above says whether it is reached; only that a path could not be put there to find out.

---

### Observed — the Mac half of the hand tests, 2026-08-07

Everything in this section was run on **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card**. The Blender was `/Applications/Blender 4.5.12.app/Contents/MacOS/Blender`, which reported itself as **4.5.12 LTS**, hash `84afd5f785f7` — the same version string and the same hash the rented Linux machine reported for its own copy of this version. Every run used `--factory-startup`, and every run had `BLENDER_USER_RESOURCES` pointed at `$T/res-4.5`, a settings folder made for this sitting before it was used and for nothing else, never at the real one. Below, `$T` stands for `/private/tmp/smoozy-test/raf`, a throwaway folder outside the project, and `$S` for `/Users/artist/Documents/Smoozy hand tests/Reading a file/added-questions/scripts`, where the scripts are kept. Only one Blender of this sitting ran at a time.

**Other topics were running Blender on this same Mac while this sitting ran**, so nothing below says how long anything took or how much memory anything used, and no test below measures either.

**The scripts were written again for this sitting.** The brief said not to touch the rented Linux machine at all, and the Linux half's scripts live there, so they were rebuilt from the procedure and the printouts recorded above. They were checked against the Linux printout before anything was read from them: the twenty-four readings of the many-paths file, and the map of data-blocks to paths, came back the same character for character. That is what makes the two lines that *do* differ readable as a difference between the machines rather than a difference between two scripts.

The full list of tests, with the set-up and the command for each, is `Audits/Blender Research/Reading a file/Hand tests — added questions.md`.

#### The same file's paths on the Mac, set beside the Linux printout (list entry 8)

The many-paths file was built again on the Mac by `$S/s_buildlib.py` and then `$S/s_build.py`, its paths were made relative by `$S/s_rel.py`, which answered `OP make_paths_relative = {'FINISHED'}`, and `$S/s_cross.py` read the result:

```
BLENDER_USER_RESOURCES="$T/res-4.5" "/Applications/Blender 4.5.12.app/Contents/MacOS/Blender" -b --factory-startup \
  "$T/work/many/4.5/many_rel.blend" --python "$S/s_cross.py"
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
CROSS blend_paths   = ['//../../../../../../../../../../Applications/Blender 4.5.12.app/Contents/Resources/4.5/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '//assets/camshader.osl', '//assets/fluidcache/', '//assets/meshcache.mdd', '//assets/oceanbake/', '//assets/probe.abc', '//assets/probe.ies', '//assets/probe.osl', '//assets/probe.png', '//assets/probe.png', '//assets/probe.txt', '//assets/probe.vdb', '//assets/probe.wav', '//assets/probe.woff2', '//assets/stripdir/probe.png', '//lib.blend']
CROSS file_path_map = [('FluidObj', ['//assets/fluidcache/']), ('ModObj', ['//assets/meshcache.mdd', '//assets/oceanbake/']), ('ProbeCam', ['//assets/camshader.osl']), ('ProbeClip', ['//assets/probe.png']), ('ProbeFont', ['//assets/probe.woff2']), ('ProbeImage', ['//assets/probe.png']), ('ProbeMat', ['//assets/probe.ies', '//assets/probe.osl']), ('ProbeSound', ['//assets/probe.wav']), ('ProbeText', ['//assets/probe.txt']), ('ProbeVolume', ['//assets/probe.vdb']), ('Scene', ['//assets/stripdir/probe.png']), ('lib.blend', ['//lib.blend']), ('probe.abc', ['//assets/probe.abc'])]
CROSS separator     = '/' | case check = True
```

Set beside the Linux printout for this same version, line by line: **twenty-five of the twenty-seven lines are identical, character for character.** That is all twenty-four readings, and the map of data-blocks to paths. **Two lines differ.**

**The flat path list.** The Mac's list holds sixteen entries; the Linux list holds fifteen. The fifteen they share are identical. The one the Mac has and Linux does not is a path into the Blender application's own folder, ending `datafiles/assets/brushes/essentials_brushes-gp_draw.blend`, recorded relative to the file. Two further readings taken on the Mac, recorded because they bound what that line can be read as. The same file **before** its paths were made relative lists that path on the Mac in full form, exactly as the Linux printout above lists its own copy of it — so the difference appears only after the sweep and the save. And after the sweep the material it hangs off reads:

```
WEAK 4.5 Material '//../../../../../../../../../../Applications/Blender 4.5.12.app/Contents/Resources/4.5/datafiles/assets/brushes/essentials_brushes-gp_draw.blend'
WEAK 4.5 ProbeMat None
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
CASE 4.5 asked for $T/work/many/4.5/assets/PROBE.PNG
CASE 4.5 exists     True
CASE 4.5 real name  ['probe.png']
```

**On this point the Mac's answer is the one to read, not the Linux one**: they are different answers about two different machines and neither speaks for the other. The Linux half's own script was not opened for this sitting, so whether it asked this in exactly these words is not established; the words above are the Mac's.

#### Replacing the file an image or a strip is loaded from, with a window (list entry 21)

The Linux half of this entry is recorded above as a refusal with no window. The Mac has a screen, so the file was built there with no window by `$S/s_replace.py` and then opened **with a window on screen**, and the two operators were called from a `--python` script through a context override. The script ends by stopping Blender itself, so nothing was left open on the Creator's desk.

```
BLENDER_USER_RESOURCES="$T/res-4.5" "/Applications/Blender 4.5.12.app/Contents/MacOS/Blender" -b --factory-startup \
  --python "$S/s_replace.py" -- build "$T/work/replace/4.5"
BLENDER_USER_RESOURCES="$T/res-4.5" "/Applications/Blender 4.5.12.app/Contents/MacOS/Blender" --factory-startup \
  "$T/work/replace/4.5/repl.blend" --python "$S/s_replace_window.py" -- "$T/work/replace/4.5"
```

The script puts `ProbeImage` in an Image editor and calls `bpy.ops.image.replace(filepath=<the second file>, relative_path=False)`; then it switches the area to the Sequence editor, selects the strip, and calls `bpy.ops.sequencer.change_path` three times — first exactly as the list wrote it, then with the file named as well, then with the sequencer's scene supplied in the context. All three calls and all three answers are printed. What came back:

```
REPL build save {'FINISHED'}
REPL window     = True
REPL version    = 4.5.12 LTS
REPL before image = /private/tmp/smoozy-test/raf/work/replace/4.5/first/a.png
REPL op image.replace = {'FINISHED'}
REPL after  image = /private/tmp/smoozy-test/raf/work/replace/4.5/second/b.png
REPL before strip = /private/tmp/smoozy-test/raf/work/replace/4.5/first/ | ['a.png']
REPL op sequencer.change_path (no files argument) = {'CANCELLED'}
REPL after  strip, first call = /private/tmp/smoozy-test/raf/work/replace/4.5/first/ | ['a.png']
REPL op sequencer.change_path (files named) = {'FINISHED'}
REPL after  strip, second call = /private/tmp/smoozy-test/raf/work/replace/4.5/second/ | ['b.png']
REPL context has sequencer_scene = False
REPL op sequencer.change_path (sequencer scene supplied) = {'FINISHED'}
REPL after  strip, third call = /private/tmp/smoozy-test/raf/work/replace/4.5/second/ | ['b.png']
```

#### The same two operators with no window at all, on the Mac (list entry 23)

This one is not on the list as it was written; it was added because the run above showed the list had missed it. The Linux half concluded that both operators refuse to run with no window. But the windowed run above shows that what the sequencer operator wanted was the file named and the sequencer's own scene put into the context, neither of which the Linux run supplied. So the same two operators were asked again on the Mac **with no window at all**, and given both of those things:

```
BLENDER_USER_RESOURCES="$T/res-4.5" "/Applications/Blender 4.5.12.app/Contents/MacOS/Blender" -b --factory-startup \
  --python "$S/s_replace.py" -- build "$T/work/replacebg/4.5"
BLENDER_USER_RESOURCES="$T/res-4.5" "/Applications/Blender 4.5.12.app/Contents/MacOS/Blender" -b --factory-startup \
  "$T/work/replacebg/4.5/repl.blend" --python "$S/s_replace_nowindow.py" -- "$T/work/replacebg/4.5"
```

What came back:

```
REPLBG background = True
REPLBG version    = 4.5.12 LTS
REPLBG screen     = <bpy_struct, Screen("Layout") at 0xab2deeea0>
REPLBG areas      = 4
REPLBG before image = <W>/4.5/first/a.png
REPLBG op image.replace = ERROR RuntimeError Operator bpy.ops.image.replace.poll() failed, context is incorrect
REPLBG after  image = <W>/4.5/first/a.png
REPLBG before strip = <W>/4.5/first/ | ['a.png']
REPLBG op sequencer.change_path = {'FINISHED'}
REPLBG after  strip = <W>/4.5/second/ | ['b.png']
REPLBG op sequencer.change_path (sequencer scene supplied) = {'FINISHED'}
REPLBG after  strip, second call = <W>/4.5/second/ | ['b.png']
```

#### The Outliner's Data API view (list entry 22)

The Mac has a screen, so this could be reached there and nowhere else. `many_rel.blend` was opened with a window, the largest area was turned into the Outliner and put into its Data API view, the interface scale was set to 0.5 so that more rows would fit in one picture, and the area was photographed with Blender's own screenshot command. Nothing was clicked. A second run assigned a new path to the image while that same view was on screen, saved, reopened the file and read the value back.

```
BLENDER_USER_RESOURCES="$T/res-4.5" "/Applications/Blender 4.5.12.app/Contents/MacOS/Blender" --factory-startup --window-geometry 0 0 2600 1500 \
  "$T/work/many/4.5/many_rel.blend" --python "$S/s_outliner_window.py" -- shot "$T/out/outl/4.5-%s.png"
BLENDER_USER_RESOURCES="$T/res-4.5" "/Applications/Blender 4.5.12.app/Contents/MacOS/Blender" --factory-startup --window-geometry 0 0 2600 1500 \
  "$T/work/many/4.5/outl.blend" --python "$S/s_outliner_window.py" -- write "$T/out/outl/4.5-%s.png"
```

What came back:

```
OUTL version       = 4.5.12 LTS
OUTL background    = False
OUTL display_mode  = DATA_API
OUTL area          = 2495 x 1505
OUTL ui_scale asked for 0.5, got 0.5
OUTL screenshot_area two_levels = {'FINISHED'} 131779 bytes
OUTL screenshot_area four_levels = {'FINISHED'} 167767 bytes
OUTL view y before = -1453.0
OUTL outliner.scroll_page = {'FINISHED'} view y now -2906.0
OUTL screenshot_area after_scroll_page = {'FINISHED'} 135351 bytes
OUTL view2d.pan = {'FINISHED'} view y now -4306.0
OUTL screenshot_area after_pan = {'FINISHED'} 145033 bytes
OUTL filter_text set to 'Filepath'
OUTL screenshot_area after_filter = {'FINISHED'} 144221 bytes
OUTL version       = 4.5.12 LTS
OUTL background    = False
OUTL display_mode  = DATA_API
OUTL area          = 2495 x 1505
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

2026-08-06 — Opus 5 (Claude Code) — Reading a file, Blender 4.5

2026-08-07 — Opus 5 (Claude Code) — the Mac half of the hand tests

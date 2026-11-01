# Dependencies — later questions, second pass (Blender 4.5)

**Sources consulted, and when.** Blender 4.5 manual, Blender 4.5 Python API documentation, Blender 4.5 release notes, and Cycles' own files for 4.5 — all as supplied in `Blender Sources/`, snapshot taken 2026-07-30. That snapshot date is the date read for every citation below.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

Only question 19 was assigned. The rest of the sheet was left alone.

---

## 19. What decides whether two recorded library locations are treated as the same library or as two different libraries?

**Reading taken.** The question is ambiguous, and I answer the reading I think is meant: a *library* here is a linked `.blend` file, and a *recorded library location* is the path string stored in the current file for such a linked `.blend`. So: if a file ends up holding two recorded paths, what makes Blender treat them as one library rather than two? The other reading is the Preferences list of "Asset Libraries", which are named directory entries rather than linked files; I cover what the sources say about that separately at the end, because it is a different mechanism.

**LABEL: not in the sources.**

The sources describe what a recorded library location *is*, how it may be written, and how a linked data-block is addressed using it. They nowhere state the rule by which two recorded locations are compared, nor whether comparison happens on the stored text or on the file the text resolves to. I could not find a single sentence in the manual, the Python API documentation, the release notes or Cycles' files that says when two locations collapse into one library and when they stay two.

What the sources *do* say, and which is as close as they get, follows.

### What is recorded, and in what form (documented)

A library is a data-block whose only location information is a single path string, which may be written blend-relative:

> "filepath
>
> Path to the library .blend file
>
> Type:
>
> string, default “”, (never None, blend relative // prefix supported)"

— Python API documentation, https://docs.blender.org/api/4.5/bpy.types.Library.html, read 2026-07-30.

The same page describes the type itself:

> "External .blend file from which data is linked"

— same page, same date.

That a stored path can be partial, and is completed against the referencing file's own folder, is stated in the manual:

> "When you specify one of these external files, the default option is to make the path relative.
> Blender stores a partial path evaluated relative to the directory location of the referencing blend-file."

— Manual, https://docs.blender.org/manual/en/4.5/files/blend/open_save.html, read 2026-07-30.

And the two spellings are interchangeable at the point of writing the path:

> "When relative paths are supported, the File Browser provides a Relative Path checkbox,
> when entering the path into a text field, use a double slash prefix (//) to make it so."

— same page, same date.

**Inferred (one step):** because the same file on disk can therefore be recorded either as `//lib.blend` or as a full path, two recorded locations can differ as text while naming the same file. The step I took is only that "a partial path evaluated relative to the directory" and a full path can denote one file; the sources do not take the next step and say what Blender then does with two such records.

### How a recorded location is used to address linked data (documented)

The Python API documentation shows the recorded path used verbatim — `//` prefix and all — as the second half of the key that picks out a linked data-block:

> "# Library object name lookup using a pair,
> # where the second argument is the library path matching bpy.types.Library.filepath.
> obj = bpy.data.objects["my_obj", "//my_lib.blend"]"

— Python API documentation, https://docs.blender.org/api/4.5/info_gotchas_internal_data_and_python_objects.html, read 2026-07-30.

This is the strongest hint in the sources that the *stored string* is what identifies a library in practice. It is still only a hint: the page is about looking data up once it is loaded, and says nothing about how two records are compared when a file is being read or when a second link is made.

Libraries are also shown to the user by their path rather than by any other identifier:

> "They’re also listed in the Outliner’s Blender File Display Mode,
> along with the path of the blend-file they originate from."

— Manual, https://docs.blender.org/manual/en/4.5/files/linked_libraries/link_append.html, read 2026-07-30.

### Why sameness matters, per the sources (documented)

The library a data-block came from is part of that data-block's name space:

> "Unique data-block ID name (within a same type and library)"

— Python API documentation, `ID.name`, https://docs.blender.org/api/4.5/bpy.types.ID.html, read 2026-07-30.

> "Data-blocks names are unique within their namespace. A data-block namespace is defined by its type, and the blendfile
> it is stored in."

— Manual, https://docs.blender.org/manual/en/4.5/files/data_blocks.html, read 2026-07-30.

The same manual page lists `Library` as a data-block type in its own right ("References to an external blend-file. Access from the Outliner's Blender File view."), with linking marked ✗ and packing marked ✓.

**Inferred (one step):** since libraries are data-blocks, and data-block names are unique per type per file, two libraries held at once in one file must carry two different names. The step is only the application of the general naming rule to the `Library` type. The sources never say how a library's name is derived from its recorded path, so this does not tell us what makes two locations distinct in the first place.

Blender also has a notion of "the same initial library" that it uses when appending, again without defining it:

> "Append (make local) also indirect dependencies of appended IDs coming from other libraries. NOTE: All IDs (including indirectly linked ones) coming from the same initial library are always made local."

— Python API documentation, `BlendImportContext.options`, `APPEND_RECURSIVE`, https://docs.blender.org/api/4.5/bpy.types.BlendImportContext.html, read 2026-07-30.

### The one place the sources do name a comparison key — and it is about appended data, not libraries (documented, then inferred)

For data that has been *appended* rather than linked, Blender records where it came from, and the record consists of exactly two fields — a library file path and a full data-block name:

> "Read-only external reference to a linked data-block and its library file"
>
> "filepath — Path to the library .blend file … (never None, blend relative // prefix supported)"
>
> "id_name — Full ID name in the library .blend file (including the two leading ‘id type’ chars)"

— Python API documentation, https://docs.blender.org/api/4.5/bpy.types.LibraryWeakReference.html, read 2026-07-30.

What that record is for:

> "Weak reference to a data-block in another library .blend file (used to re-use already appended data instead of appending new copies)"

— Python API documentation, `ID.library_weak_reference`, https://docs.blender.org/api/4.5/bpy.types.ID.html, read 2026-07-30.

The matching is exposed as an option on the link/append operator:

> "do_reuse_local_id (boolean, (optional)) – Re-Use Local Data, Try to re-use previously matching appended data-blocks instead of appending a new copy"

— Python API documentation, `bpy.ops.wm.link`, https://docs.blender.org/api/4.5/bpy.ops.wm.html, read 2026-07-30.

and described for the user as:

> "The first time an asset is used, it will be appended, including its dependencies,
> just like described previously. However, Blender will keep track of where it originated,
> and the next time the asset is used, as much data as possible will be reused."

— Manual, https://docs.blender.org/manual/en/4.5/editors/preferences/file_paths.html, read 2026-07-30 (the same wording also appears at https://docs.blender.org/manual/en/4.5/editors/asset_browser.html).

**Inferred (one step, and worth being cautious about):** for appended data, whether a previous import counts as "previously matching" is decided on the pair *library file path plus full data-block name*, because that pair is precisely and only what the weak reference stores. The step a reader has to supply is that the stored fields are the fields compared; no source says so. Even granting the step, this covers reuse of appended data, not whether two recorded locations become one `Library`.

### What I examined in Cycles' own files (observed)

Cycles treats each `Library` data-block as the unit, and matches data-blocks to libraries by the `library` pointer on the data-block rather than by any path text:

> `    # Map of versions used by libraries.`
> `    library_versions = {}`
> `    library_versions[bpy.data.version] = [None]`
> `    for library in bpy.data.libraries:`
> `        library_versions.setdefault(library.version, []).append(library)`

— `Blender Sources/cycles_4.5/version_update.py`, lines 97–101.

and, further down the same function:

> `        for scene in bpy.data.scenes:`
> `            if scene.library not in libraries:`
> `                continue`

— `Blender Sources/cycles_4.5/version_update.py`, lines 110–112 (the same `if … .library not in libraries` test recurs at lines 265, 277 and 299 for lights, worlds and materials).

This is an observation about what the file says: by the time Cycles sees the data, "which library" is already settled and is a data-block identity. It says nothing about how that settling was done, and I have not run anything.

### Related change recorded in the release notes (documented)

> "* Linked IDs can now also be relocated, if they are not a dependency of any other linked data (blender/blender!135936, blender/blender@0eba8caaf9, blender/blender@3fa35aa4ee)."

— Release notes, https://developer.blender.org/docs/release_notes/4.5/core/, read 2026-07-30.

The manual's matching description of relocation, for both a whole library and a single linked data-block:

> "It allows to replace it by a different file.
> This can be used to either fix a broken linked library (e.g. because the file was moved
> or renamed), or to switch to a variation of the same data in a different file."

> "It allows to
> relocate a directly linked ID by another one, from the same library or from a different one."

— Manual, https://docs.blender.org/manual/en/4.5/files/linked_libraries/link_append.html, read 2026-07-30.

Neither passage says what happens if the file you relocate onto is already linked in the same blend-file.

### The other reading: Asset Libraries in Preferences (documented)

These are named directory entries, and the sources are explicit that the name is not the identifier:

> "name — Identifier (not necessarily unique) for the asset library"
>
> "path — Path to a directory with .blend files to use as an asset library"

— Python API documentation, https://docs.blender.org/api/4.5/bpy.types.UserAssetLibrary.html, read 2026-07-30.

> "Name and on-drive directory paths of asset libraries.
> To make Blender aware of an asset library, add it to this list.
> The name is for your reference only, and will appear in asset library selectors.
> The path should point to the location of the asset library."

— Manual, https://docs.blender.org/manual/en/4.5/editors/preferences/file_paths.html, read 2026-07-30.

I opened the screenshot on that page, `_images/asset_browser-asset_library_preferences.png`, referenced from the same manual page (https://docs.blender.org/manual/en/4.5/editors/preferences/file_paths.html), read 2026-07-30. **Documented, from a picture.** Visible in it: a panel headed "Asset Libraries"; a two-column list headed "Name" and "Path"; two rows, "User Library" at `/home/sybren/Documents/Blender/Assets` and "Simon's Materials" at `/home/sybren/syncthi…ndles/simon-materials/`; a folder button and an X on each row, and a + below the last one. Nothing in the picture shows what happens if two rows carry the same path or the same name. **Inferred**, one step: that an entry is told apart from another by its name and its path — the picture shows each entry carrying both, not that either is what distinguishes it. *(Changed by the citation check, 2026-08-09: "entries are distinguished by a name and a path" used to sit inside the list of things visible; it is a step from the picture, so it is now labelled inferred and separated from what can be seen.)*

Even here, the sources do not say what makes two asset-library entries the same or different — only that the name need not be unique, which leaves the path as the only candidate and is not stated as a rule.

### Not covered

This answer does not settle any of the following, because nothing in the sources touches them:

- Whether comparison is done on the stored text or on the file the text resolves to.
- Whether `//lib.blend` and the full path to the same file give one library or two.
- Whether capital and small letters in a recorded location matter, and whether that differs by operating system.
- Whether cosmetic differences in a path — a trailing separator, `./`, `..`, forward versus back slashes, a repeated separator — make a difference.
- Whether a symbolic link, a hard link, a mounted network share reached by two names, or a drive letter versus a network path count as the same file.
- What happens when a library is relocated onto a path that is already linked in the same file, or when a link is made a second time to a file already linked.
- How a `Library` data-block's *name* is derived from its path, and whether two libraries whose names would clash get a numeric suffix the way other data-blocks do.
- Whether packing or unpacking linked libraries changes any of this.
- Whether the answer differs during file loading versus during an interactive link.

### Outside the interface

The sources do not say. Nothing I found states a rule at all, so there is nothing to test against a headless run. Two things are worth noting so the gap is not mistaken for a difference:

- Everything quoted above about `Library.filepath`, `ID.name`, `LibraryWeakReference` and `BlendImportContext` is documentation of data and of Python-facing properties, not of interface behaviour, and the same properties are reachable from a script driving Blender from outside. `bpy.data.libraries.load(filepath, link=…, relative=…)` and `bpy.data.libraries.write(…)` are documented as plain Python calls (https://docs.blender.org/api/4.5/bpy.types.BlendDataLibraries.html, read 2026-07-30), and `bpy.ops.wm.link`, `bpy.ops.wm.lib_reload(library='…')` and `bpy.ops.wm.lib_relocate(library='…')` are documented operators taking the library as a string parameter (https://docs.blender.org/api/4.5/bpy.ops.wm.html, read 2026-07-30).
- Reload and Relocate are documented in the manual only as Outliner right-click actions ("When the Outliner is in the Blender File Display Mode…", https://docs.blender.org/manual/en/4.5/files/linked_libraries/link_append.html, read 2026-07-30), but the operators above exist independently of the Outliner. That the operators behave identically without a window is not stated.

**Brought up to date by the hand tests, 2026-08-09.** Every test recorded in the *Observed* section below was run with Blender started with no window, and on the rented Linux machine there is no display server at all, so no window was possible there. All the results below therefore hold when Blender is driven from outside its own interface. What was **not** done is a comparison with the same steps carried out by clicking: neither machine ran a window this round, so nothing is said about whether the two differ. One thing that does turn on being driven from outside is written up under test 8 below — the relocate call returned `{'FINISHED'}` and did not move the library it named.

### Operating system

The sources do not say. There is no statement anywhere I searched that library-location matching differs between operating systems, and none that it is the same on all of them.

One adjacent, documented, operating-system-specific item did turn up, and I record it only because it concerns a recorded library path — but note it is an *asset* library path in Preferences, not a linked-library path, and it is a fixed crash, not a matching rule:

> " * Mac version blender crashes when Asset Libraries path contains BACK SLASH "\" [[efdda781755](https://projects.blender.org/blender/blender/commit/efdda781755ade5c079a081dba582feabcf2bf97)]"

— Release notes, https://developer.blender.org/docs/release_notes/4.5/bugfixes/, read 2026-07-30.

**Brought up to date by the hand tests, 2026-08-09.** Of the tests in the *Observed* section below, one came out differently on the two computers and the rest came out the same. The one that differs is capital and small letters, test 4: on the rented Linux machine the differently-spelled name is not a file and the link was refused, and on the Creator's Mac it is the same file and two library entries resulted. Windows was not tested at all, because no Windows machine has run it. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

---

## Record of where I looked

Everything below was searched inside `Blender Sources/` only, across `manual_4.5`, `api_4.5`, `release_notes_4.5` and `cycles_4.5`.

**Whole-tree text searches (case-insensitive), by term:**
`librar`; `same library`; `the same library`; `already linked`; `duplicate library`; `same blend-file`; `same file`; `same path`; `library path`; `unique`; `identified by`; `considered the same`; `point to the same`; `treated as the same`; `resolves to`; `one library`; `per library`; `distinct librar`; `two librar`; `case-insensitive`; `case sensit`; `canonical`; `normaliz`; `symlink`; `symbolic link`; `loaded once`; `only once`; `twice`; `more than once`; `second time`; `re-use`; `reuse`; `Reuse Data`; `reuse_liboverrides`; `already appended`; `Relative Path`; `lib_relocate`; `lib_reload`; `liboverride`; `bpy.data.libraries[`; `name_full`.

`case-insensitive`, `case sensit`, `canonical`, `normaliz`, `symlink` and `symbolic link` returned nothing about file paths anywhere in manual or API — the only hits were quaternion normalization, weight-paint normalization, and a command-line page noting that command-line *arguments* are case sensitive. That is the main reason I am confident the path-comparison rule is genuinely absent rather than merely hard to find.

**Manual pages opened in full or in part:**
- `files/linked_libraries/index` — https://docs.blender.org/manual/en/4.5/files/linked_libraries/index.html (contents list only; the three pages under it are the whole of the linked-library documentation)
- `files/linked_libraries/link_append` — https://docs.blender.org/manual/en/4.5/files/linked_libraries/link_append.html (read in full: Link, Append, Reload Library, Relocate Library, Broken Libraries, Relocate Linked ID, Make Local, Known Limitations)
- `files/data_blocks` — https://docs.blender.org/manual/en/4.5/files/data_blocks.html (read in full, including the data-block type table and the "Name & Rename" naming-collision rules)
- `files/blend/open_save` — https://docs.blender.org/manual/en/4.5/files/blend/open_save.html (Remap Relative, Relative Paths sections)
- `files/blend/packed_data` — https://docs.blender.org/manual/en/4.5/files/blend/packed_data.html (Pack/Unpack Linked Libraries)
- `files/file_paths` — https://docs.blender.org/manual/en/4.5/files/file_paths.html (path templates; ruled out — about substituting values into output paths, not about libraries)
- `files/asset_libraries/introduction` — https://docs.blender.org/manual/en/4.5/files/asset_libraries/introduction.html
- `editors/preferences/file_paths` — https://docs.blender.org/manual/en/4.5/editors/preferences/file_paths.html (Asset Libraries list, import methods)
- `editors/preferences/save_load` — https://docs.blender.org/manual/en/4.5/editors/preferences/save_load.html (Default To – Relative Paths)
- `editors/asset_browser` — https://docs.blender.org/manual/en/4.5/editors/asset_browser.html
- `editors/file_browser` — https://docs.blender.org/manual/en/4.5/editors/file_browser.html (Link/Append from Library, Relative Path)
- `editors/outliner/interface` and `editors/outliner/editing` — https://docs.blender.org/manual/en/4.5/editors/outliner/interface.html and …/editing.html (Blender File and Library Overrides display modes; no library-identity statement)
- `scene_layout/object/selecting` — https://docs.blender.org/manual/en/4.5/scene_layout/object/selecting.html (Select Linked ‣ Library: "Selects all objects that are in the same Library." — uses the notion, does not define it)
- `glossary/index` — https://docs.blender.org/manual/en/4.5/glossary/index.html (entries for Asset, Asset Library, Current File Asset Library; there is no glossary entry defining a linked library)
- `advanced/command_line/arguments` — https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html (searched for library switches; the only library-related one is `--disable-liboverride-auto-resync`, which concerns override resync, not location matching)
- `physics/baking` — https://docs.blender.org/manual/en/4.5/physics/baking.html (hit on "Use Library Path"; ruled out — that is about where a cache is written for a file linked into another, not about library identity)

**Manual pictures opened:**
- `_images/files_data-blocks_outliner-blender-file-view.png`, from https://docs.blender.org/manual/en/4.5/files/data_blocks.html, read 2026-07-30. **Documented, from a picture.** Visible: the Outliner's display-mode menu (Scenes, View Layer, Sequence, Blender File highlighted, Data API, Orphan Data), and below it a "Current File" tree listing the data-block types, with no linked library row anywhere in it. **Inferred** from that absence: it does not show how libraries are labelled, so it is ruled out. *(Changed by the citation check, 2026-08-09: labels added, as the Method now asks for pictures.)*
- `_images/asset_browser-asset_library_preferences.png`, from https://docs.blender.org/manual/en/4.5/editors/preferences/file_paths.html — described in the answer above.

**Python API pages opened:**
- `bpy.types.Library` — https://docs.blender.org/api/4.5/bpy.types.Library.html (read in full; the only location-bearing property is `filepath`)
- `bpy.types.LibraryWeakReference` — https://docs.blender.org/api/4.5/bpy.types.LibraryWeakReference.html (read in full)
- `bpy.types.BlendDataLibraries` — https://docs.blender.org/api/4.5/bpy.types.BlendDataLibraries.html (read in full: `load`, `write`, `remove`, `tag`; none mention matching or de-duplication)
- `bpy.types.BlendData` — https://docs.blender.org/api/4.5/bpy.types.BlendData.html (`libraries`, `file_path_map(… include_libraries …)`, `orphans_purge`)
- `bpy.types.ID` — https://docs.blender.org/api/4.5/bpy.types.ID.html (`name`, `name_full`, `library`, `library_weak_reference`, `is_missing`, `is_library_indirect`, `session_uid`)
- `bpy.types.BlendImportContext`, `bpy.types.BlendImportContextItem`, `bpy.types.BlendImportContextLibrary`, `bpy.types.BlendImportContextLibraries` — https://docs.blender.org/api/4.5/bpy.types.BlendImportContext.html and the three matching pages (read in full; `source_libraries` is "List of libraries to search and import that ID from. The ID will be imported from the first file in that list that contains it", which is about search order, not identity)
- `bpy.types.UserAssetLibrary` and `bpy.types.AssetLibraryCollection` — https://docs.blender.org/api/4.5/bpy.types.UserAssetLibrary.html, …/bpy.types.AssetLibraryCollection.html
- `bpy.types.AssetRepresentation` — https://docs.blender.org/api/4.5/bpy.types.AssetRepresentation.html (`full_library_path`, `full_path`)
- `bpy.path` — https://docs.blender.org/api/4.5/bpy.path.html (`abspath(path, *, start=None, library=None)`; documents resolution, says nothing about comparison)
- `info_gotchas_internal_data_and_python_objects` — https://docs.blender.org/api/4.5/info_gotchas_internal_data_and_python_objects.html ("Library Collisions" section; quoted above)
- `info_gotchas_file_paths_and_encoding` — https://docs.blender.org/api/4.5/info_gotchas_file_paths_and_encoding.html (relative paths of linked data are relative to the library, not the open file; relevant to resolution, not to identity)
- `bpy.ops.wm` — https://docs.blender.org/api/4.5/bpy.ops.wm.html (`link`, `append`, `lib_reload`, `lib_relocate`; all take the library as a plain string with no stated matching rule)
- `bpy.ops.outliner` — https://docs.blender.org/api/4.5/bpy.ops.outliner.html (`lib_operation` DELETE/RELOCATE/RELOAD — marked "Undocumented, consider contributing." — and `lib_relocate`)
- `bpy.ops.file` — https://docs.blender.org/api/4.5/bpy.ops.file.html (`make_paths_absolute`, `make_paths_relative`, `find_missing_files`, `report_missing_files`, `pack_libraries`, `unpack_all`; these change or search paths but no wording says what happens if two paths end up equal)
- `bpy.app` — https://docs.blender.org/api/4.5/bpy.app.html (checked the `__file__` / library-linked text note; ruled out)

**Release notes read:**
- https://developer.blender.org/docs/release_notes/4.5/core/ (read in full; "Linking & Library Overrides" quoted above)
- https://developer.blender.org/docs/release_notes/4.5/pipeline_assets_io/ (scanned; USD/glTF/FBX only, nothing on blend libraries)
- https://developer.blender.org/docs/release_notes/4.5/python_api/ (searched for library/link/path; only render-path and `//`-prefix property-subtype items)
- https://developer.blender.org/docs/release_notes/4.5/bugfixes/ (searched for "librar"; five hits, all quoted or ruled out above — override/action constraints, asset catalogs, the macOS back-slash crash, `bpy.data.libraries.load` from the current file, and an Outliner search field)
- https://developer.blender.org/docs/release_notes/4.5/user_interface/ (two library hits, both about asset tooltips; ruled out)

**Cycles files searched:**
Searched every file under `Blender Sources/cycles_4.5` for "librar". Hits: `version_update.py` (quoted above), `osl.py` (uses `bpy.path.abspath(filepath, library=library)` to resolve an OSL script path relative to its library — resolution, not identity), and a number of `source/…` C++ headers where the word appears in unrelated maths and OSL comments. Cycles exposes no setting bearing on library identity; there is nothing in `properties.py` or `ui.py` on the subject.

**What I ruled out and why.** Asset-library preferences pages were examined and kept only as the alternative reading, since an asset library is a registered directory rather than a linked file. Library *override* documentation (`files/linked_libraries/library_overrides`, the `IDOverrideLibrary*` API pages) was set aside: it concerns editing linked data, and nothing in it bears on which library a location denotes. Physics-cache "Use Library Path" and the OSL script path in Cycles both matched the word "library" but concern where a file is written or read, not whether two libraries are one.

---

## Observed — fifteen tests run by hand on 2026-08-09

**Label: observed.**

**On what kind of computer.** Two, and neither speaks for the other.

- **The rented Linux machine** — rented on Vast.ai, Ubuntu 22.04.5 LTS on kernel 6.8, one NVIDIA GeForce RTX 5060 with 8 GB on driver 580.95.05, twenty-three cores of an AMD EPYC 7K62, 49 GB of memory, and no display server at all. Blender 4.5.12 LTS, at `/opt/blender/4.5/blender`.
- **The Creator's Mac** — a MacBook Pro with an M1 Pro chip, macOS 26.5.2, Apple's own graphics. Blender 4.5.12 LTS, at `/Applications/Blender 4.5.12.app/Contents/MacOS/Blender`.

**What was done, in every test.** Blender was started with `--background --factory-startup`, pointed at a throwaway settings folder made immediately before the run, in a working folder made immediately before the run. Blend-files were built by script. Every link was made twice over in two separate runs — once with `bpy.ops.wm.link`, the operator the interface's File ▸ Link runs, and once with `bpy.data.libraries.load`, a plain Python call. **The two gave the same result in every test below**, on both computers. Neither call's arguments were guessed; they were read off the running Blender first.

**What was read, after every step.**

```python
print("HT|%s|library_count|%d" % (tag, len(bpy.data.libraries)))
for i, lib in enumerate(bpy.data.libraries):
    print("HT|%s|library|%d|name=%r|filepath=%r" % (tag, i, lib.name, lib.filepath))
for ob in bpy.data.objects:
    print("HT|%s|object|name=%r|name_full=%r|from=%r" % (
        tag, ob.name, ob.name_full, ob.library.filepath if ob.library else None))
```

That is the same set of library entries the Outliner's Blender File view lists in a window, read from a terminal instead. It is a different test from counting rows on a screen, and no screen was used in any of this.

**Every test also saved the file and opened it again in a fresh Blender**, and read the same lines again, so that a decision made when linking and a decision made when reading a file back could be told apart.

**The same fifteen tests were run in 5.0.1, 5.1.2 and 5.2.0 LTS as well.** Everything below came out the same in all four versions except test 12.

### 1. The control: the count can say one and can say two

*Done to:* a folder holding `libA.blend` with objects `A1` and `A2`, `libB.blend` with object `B1`, and an empty `host.blend`. *Done:* link `A1` from `libA.blend`; link `A2` from the same file with the path written exactly the same way; link `B1` from `libB.blend`.

The rented Linux machine printed:

```
HT|after-A1|library_count|1
HT|after-A1|library|0|name='libA.blend'|filepath='/root/ht-deps/e1-4.5-ops/libA.blend'
HT|after-A2-same-spelling|library_count|1
HT|after-B1-other-file|library_count|2
HT|after-B1-other-file|library|1|name='libB.blend'|filepath='/root/ht-deps/e1-4.5-ops/libB.blend'
HT|after-fresh-reopen|library_count|2
```

The Creator's Mac printed the same counts, `1, 1, 2, 2`. The two agreed. Every test below rests on this: the number can come out either way.

### 2. The same file recorded relative, then recorded absolute

*Done to:* `lib.blend` with objects `A1` and `A2`, and `host.blend` beside it. *Done:* link `A1` with the relative option on, then `A2` from the same file with it off.

The rented Linux machine printed:

```
HT|after-relative|library_count|1
HT|after-relative|library|0|name='lib.blend'|filepath='//lib.blend'
HT|after-absolute|library_count|1
HT|after-absolute|library|0|name='lib.blend'|filepath='//lib.blend'
HT|after-fresh-reopen|library_count|1
HT|after-fresh-reopen|library|0|name='lib.blend'|filepath='//lib.blend'
```

The Creator's Mac printed the same counts and the same `'//lib.blend'`. The two agreed.

### 3. The same file written with a step that goes nowhere

*Done:* link three objects from one file through `…/lib.blend`, `…/./lib.blend` and `…/sub/../lib.blend`.

The rented Linux machine printed:

```
HT|paths|plain='/root/ht-deps/e3-4.5-data/lib.blend'|dotted='/root/ht-deps/e3-4.5-data/./lib.blend'|updown='/root/ht-deps/e3-4.5-data/sub/../lib.blend'
HT|after-plain|library_count|1
HT|after-dot-step|library_count|1
HT|after-up-down-step|library_count|1
HT|after-fresh-reopen|library|0|name='lib.blend'|filepath='/root/ht-deps/e3-4.5-data/lib.blend'
```

The Creator's Mac printed the same counts and the same single recorded text. The two agreed.

### 4. Capital and small letters — the one test where the two computers disagree

*Done:* link `A1` through `lib.blend`, then `A2` through `LIB.blend`. A shell check was run first on each machine, so that a refusal could be told from a machine on which the name means nothing.

**On the rented Linux machine** the shell printed `ls: cannot access '/root/ht-deps/e4-4.5-data/LIB.blend': No such file or directory`, and Blender printed:

```
HT|exists|lower=True|upper=False
HT|after-lower|library_count|1
HT|link-upper|call|raised|OSError('load: /root/ht-deps/e4-4.5-data/LIB.blend failed to open blend file')
HT|after-upper|library_count|1
```

and, through the operator rather than the Python call:

```
HT|link-upper|call|raised|RuntimeError("Error: '/root/ht-deps/e4-4.5-ops/LIB.blend/Object/A2': nothing indicated\n")
```

**On the Creator's Mac** the shell listed both names as one file of the same size, and Blender printed:

```
HT|exists|lower=True|upper=True
HT|after-lower|library_count|1
HT|after-upper|library_count|2
HT|after-upper|library|0|name='lib.blend'|filepath='…/e4-4.5-data/lib.blend'
HT|after-upper|library|1|name='LIB.blend'|filepath='…/e4-4.5-data/LIB.blend'
HT|after-fresh-reopen|library_count|2
```

**The two did not agree, and the two results are not merged.** On the rented Linux machine the differently-spelled name is not a file and the link was refused. On the Creator's Mac it is the same file and two library entries resulted.

### 5. Two names on disk for one file, and two ways into one folder

*Done to:* `lib.blend` with objects `A1` to `A5`, plus `alias.blend` (a symbolic link to it), `hard.blend` (a hard link to it), and a symbolic link to the whole folder. On the rented Linux machine there was also a second mount of the folder, made with `mount --bind`. *Done:* link one object through each route.

The rented Linux machine's shell control printed `1808457 /root/ht-deps/e5-4.5-data/lib.blend` and `1808457 /root/ht-deps/e5-4.5-data/hard.blend` — one and the same file — and `MOUNT OK`. Blender printed:

```
HT|after-real|library_count|1
HT|after-symlink-to-file|library_count|2
HT|after-hardlink|library_count|3
HT|after-symlink-to-folder|library_count|4
HT|after-second-mount|library_count|5
HT|after-fresh-reopen|library|0|name='alias.blend'|filepath='/root/ht-deps/e5-4.5-data/alias.blend'
HT|after-fresh-reopen|library|1|name='hard.blend'|filepath='/root/ht-deps/e5-4.5-data/hard.blend'
HT|after-fresh-reopen|library|2|name='lib.blend'|filepath='/root/ht-deps/e5-4.5-data/lib.blend'
HT|after-fresh-reopen|library|3|name='lib.blend.001'|filepath='/root/ht-deps/e5-4.5-data-otherway/lib.blend'
HT|after-fresh-reopen|library|4|name='lib.blend.002'|filepath='/root/ht-deps/e5-4.5-data-bind/lib.blend'
```

The Creator's Mac cannot make a second mount, so it ran four of the five steps and printed the same counts up to `HT|after-fresh-reopen|library_count|4`, with the same four entries. The two agreed on the four steps both could do. The second mount is a Linux result only.

### 6. Two entries made to hold the very same recorded text

*Done to:* `host.blend` holding two libraries linked from two different files. *Done:* write the first one's recorded text into the second from Python, then save and open again.

The rented Linux machine printed:

```
HT|assignment|returned normally
HT|after-assignment|library_count|2
HT|after-assignment|library|0|name='lib1.blend'|filepath='/root/ht-deps/e6-4.5-data/lib1.blend'
HT|after-assignment|library|1|name='lib2.blend'|filepath='/root/ht-deps/e6-4.5-data/lib1.blend'
HT|after-reopen-in-process|library_count|1
HT|after-reopen-in-process|library|0|name='lib1.blend'|filepath='/root/ht-deps/e6-4.5-data/lib1.blend'
HT|after-fresh-reopen|library_count|1
```

The Creator's Mac printed the same counts, `2, 2, 1, 1`. The two agreed.

### 7. Rewriting every path in the file at once

*Done to:* the file test 2 left, holding one entry. *Done:* `bpy.ops.file.make_paths_absolute()`, then save and reopen, then `bpy.ops.file.make_paths_relative()`, then save and reopen.

The rented Linux machine printed:

```
HT|reopened|library|0|name='lib.blend'|filepath='//lib.blend'
HT|make_paths_absolute|returned={'FINISHED'}
HT|after-make-absolute|library|0|name='lib.blend'|filepath='/root/ht-deps/e7-4.5-data/lib.blend'
HT|make_paths_relative|returned={'FINISHED'}
HT|after-make-relative|library|0|name='lib.blend'|filepath='//lib.blend'
HT|reopened-after-relative|library_count|1
```

The Creator's Mac printed the same. The two agreed.

### 8. Relocating one library onto the file another library already uses — the call did not act on the library it named

*Done to:* `host.blend` with `lib1.blend` and `lib2.blend` linked. *Done:* `bpy.ops.wm.lib_relocate(library='lib2.blend', directory=…, filename='lib1.blend')`.

The rented Linux machine printed:

```
HT|two-libraries|library|0|name='lib1.blend'|filepath='/root/ht-deps/e8-4.5-data/lib1.blend'
HT|two-libraries|library|1|name='lib2.blend'|filepath='/root/ht-deps/e8-4.5-data/lib2.blend'
HT|relocate|library='lib2.blend'|returned={'FINISHED'}
HT|after-relocate|library_count|2
HT|after-relocate|library|0|name='lib1.blend'|filepath='//lib1.blend'
HT|after-relocate|library|1|name='lib2.blend'|filepath='/root/ht-deps/e8-4.5-data/lib2.blend'
HT|after-relocate|object|name='Q1'|name_full='Q1 [lib2.blend]'|from='/root/ht-deps/e8-4.5-data/lib2.blend'
```

The Creator's Mac printed the same. The two agreed.

Two further tests were run because of that. With the same set-up and a third file that no library records, the same call naming `lib2.blend` and pointing at that third file returned `{'FINISHED'}` and changed nothing at all, both when two calls were made in one run and when a single call was made in its own run, on both computers:

```
HT|calling|bpy.ops.wm.lib_relocate(library='lib2.blend', filepath='/root/ht-deps/e15-4.5-lib2-lib3/lib3.blend', directory='/root/ht-deps/e15-4.5-lib2-lib3/', filename='lib3.blend')
HT|relocate|returned={'FINISHED'}
HT|after|library|0|name='lib1.blend'|filepath='/root/ht-deps/e15-4.5-lib2-lib3/lib1.blend'
HT|after|library|1|name='lib2.blend'|filepath='/root/ht-deps/e15-4.5-lib2-lib3/lib2.blend'
```

So, with Blender started with no window, this call returns `{'FINISHED'}` whatever it is given; nothing moves when the target is a file no library records; and when the target is a file an existing library records, that existing library's recorded text is rewritten into the `//` form while the library named in the call is left where it was. **What this therefore does not settle:** what happens when a library really is relocated onto a file another library already records. The named library never moved, so there was no collision to watch. That is written up as still open below.

### 9. The same file reached directly and through another library

*Done to:* `lib.blend` with objects `A1` and `A2`; `mid.blend`, which links `A1` from it and holds it in a local collection; and `host.blend`. *Done:* link the collection from `mid.blend`, then link `A2` straight out of `lib.blend`.

The rented Linux machine printed:

```
HT|after-indirect|library_count|2
HT|after-indirect|library|0|name='lib.blend'|filepath='/root/ht-deps/e9-4.5-data/lib.blend'
HT|after-indirect|library|1|name='mid.blend'|filepath='/root/ht-deps/e9-4.5-data/mid.blend'
HT|after-direct|library_count|2
HT|after-direct|object|name='A1'|name_full='A1 [lib.blend]'|from='/root/ht-deps/e9-4.5-data/lib.blend'
HT|after-direct|object|name='A2'|name_full='A2 [lib.blend]'|from='/root/ht-deps/e9-4.5-data/lib.blend'
HT|after-fresh-reopen|library_count|2
```

`lib.blend` was already present after the first link, so the second half means something. The Creator's Mac printed the same. The two agreed.

### 10. Two libraries whose recorded text would have been the same

*Done to:* two folders side by side, each with its own `x.blend`; in the first, `mid.blend`, which links its neighbour with the relative option on; `host.blend` in the second folder. *Done:* link the second folder's `x.blend` with the relative option on, so `host.blend` also records `//x.blend`, then link the collection from `mid.blend`.

The set-up's own control printed, when `mid.blend` was built, `HT|built-mid.blend|library|0|name='x.blend'|filepath='//x.blend'`. The rented Linux machine then printed:

```
HT|after-own-relative|library_count|1
HT|after-mid|library_count|3
HT|after-mid|library|1|name='x.blend'|filepath='//x.blend'
HT|after-mid|library|2|name='x.blend.001'|filepath='//../one/x.blend'
HT|resolve|name='x.blend'|filepath='//x.blend'|parent=None|abspath='/root/ht-deps/e10-4.5-data/two/x.blend'
HT|resolve|name='x.blend.001'|filepath='//../one/x.blend'|parent='/root/ht-deps/e10-4.5-data/one/mid.blend'|abspath='/root/ht-deps/e10-4.5-data/one/../one/x.blend'
```

So the text that `mid.blend` holds as `//x.blend` appears in the host file as `//../one/x.blend`. The two identical texts the test was built to produce did not arise. The Creator's Mac printed the same. The two agreed.

### 11. Two different files that share a file name

*Done:* link an object from `p/lib.blend`, then one from `q/lib.blend`.

The rented Linux machine printed:

```
HT|after-p|library_count|1
HT|after-p|library|0|name='lib.blend'|filepath='/root/ht-deps/e11-4.5-data/p/lib.blend'
HT|after-q|library_count|2
HT|after-q|library|1|name='lib.blend.001'|filepath='/root/ht-deps/e11-4.5-data/q/lib.blend'
HT|after-fresh-reopen|library_count|2
```

The Creator's Mac printed the same. The two agreed. Together with test 1 this shows what this answer says above the sources never state: the printed `name` of a library data-block is the bare file name, and a numbered suffix is added when two would clash.

### 12. Packing linked data — the one place 4.5.12 LTS differs from the other three versions

*Done:* ask whether the three archive properties exist, then link with packing asked for.

On both computers, Blender 4.5.12 LTS printed:

```
HT|prop|is_archive|present=False
HT|prop|archive_parent_library|present=False
HT|prop|archive_libraries|present=False
HT|pack|raised|TypeError("'pack' is an invalid keyword argument for load()")
HT|after-pack|library_count|0
```

The same test in 5.0.1, 5.1.2 and 5.2.0 LTS printed `present=True` for all three properties, and the packing call was accepted and produced two library entries holding the same recorded text, one of them an archive of the other. None of that happens in 4.5.

### 13. Two files that record one library differently, both linked into a third

*Done to:* `lib.blend`; `a.blend`, which links from it with the relative option on; `b.blend`, which links from it with the option off; and an empty `c.blend`. *Done:* link a collection from `a.blend`, then one from `b.blend`.

The controls printed, when the two were built, `HT|built-a.blend|library|0|name='lib.blend'|filepath='//lib.blend'` and `HT|built-b.blend|library|0|name='lib.blend'|filepath='/root/ht-deps/e13-4.5-data/lib.blend'` — the two really did record the one library two different ways. The rented Linux machine then printed:

```
HT|after-a|library_count|2
HT|after-b|library_count|3
HT|after-b|library|0|name='a.blend'|filepath='/root/ht-deps/e13-4.5-data/a.blend'
HT|after-b|library|1|name='b.blend'|filepath='/root/ht-deps/e13-4.5-data/b.blend'
HT|after-b|library|2|name='lib.blend'|filepath='//lib.blend'
HT|after-fresh-reopen|library_count|3
```

`lib.blend` is listed once. The Creator's Mac printed the same. The two agreed.

### What the tests do not settle

- What happens when a library really is relocated onto a file another library already records. See test 8: the call never moved the library it named, so no collision arose.
- Anything about Windows. No Windows machine has run it, so a drive letter against a network name, back slashes against forward slashes, a directory junction, and the Windows side of the capitals test all stay untested rather than answered.
- Two different network mounts of one share. This needs a second computer serving a share, which the project does not have. The second mount of one folder on the rented Linux machine, in test 5, is a different condition.
- What any of this looks like in a window: how many rows the Outliner's Blender File view shows, what the Status Bar's memory figure does, and what the Info editor says after a relocate. The rented Linux machine has no display server, and no window was opened on either machine.
- The other reading of the question, about asset libraries registered in the Preferences. No test above touches it.

---

## Still open for a person with Blender 4.5 open

The researcher's own list has been pruned: everything the hand tests answered is gone. What is left is what a running Blender still has not settled.

1. **Relocating a library onto a file another library already records.** The tests reached this through `bpy.ops.wm.lib_relocate` in a Blender with no window, and the call never moved the library it was told to move. Doing it by right-clicking the entry in the Outliner's Blender File view, in a window, would settle it. Note first whether the count goes to one, then copy out anything Blender shows in the Info editor, word for word.
2. **What the Outliner's Blender File view shows for a file that holds two entries recording the same location.** The count and the recorded text are already known from a terminal; what is not known is how those two entries are labelled on screen when their recorded text is identical.
3. **Anything on Windows.** Untried — no Windows machine has run it — not answered.

---

2026-08-09 — Opus 5 (Claude Code) — Dependencies, Blender 4.5

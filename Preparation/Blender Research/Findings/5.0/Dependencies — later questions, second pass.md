# Dependencies — later questions, second pass (Blender 5.0)

**Sources consulted:** Blender 5.0 manual, Blender 5.0 Python API documentation, Blender 5.0 release notes, and Cycles' own files — all as supplied in `Blender Sources/` (manual_5.0, api_5.0, release_notes_5.0, cycles_5.0), snapshot taken 2026-07-30. Everything below was read on that snapshot; date read for every citation is 2026-07-30.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

Only question 19 was assigned. The rest of the sheet is untouched.

---

## 19. What decides whether two recorded library locations are treated as the same library or as two different libraries?

**Reading of the question.** I answer it about linked libraries — the external `.blend` files a file links data from, each recorded as a path inside the file. The question is ambiguous: "recorded library location" could also mean an *asset library* entry registered in the Preferences (a name plus a directory). I answer the linked-library reading, because that is the one where a location is "recorded" inside a file. The other reading is addressed briefly at the end of this answer.

**LABEL: not in the sources.**

No page in the manual, the Python API documentation or the release notes states the rule Blender uses to decide that two recorded library locations are one library or two. Nothing says whether the comparison is on the stored text of the path, on the path after it has been resolved to an absolute one, whether capitalisation is folded, or whether links/shortcuts on disk are followed. The record of where I looked is at the end of this answer.

What the sources *do* say, and which bears on the question, is below. Each item is labelled on its own.

### (a) A library is one external blend-file, and its recorded location is a single path property — **documented**

> "External .blend file from which data is linked"

> "filepath
> Path to the library .blend file
> Type:
> string, default “”, (never None, blend relative // prefix supported)"

Source: Python API documentation, https://docs.blender.org/api/5.0/bpy.types.Library.html — read 2026-07-30.

> "Collection of source libraries, i.e. blendfile paths"

Source: Python API documentation, https://docs.blender.org/api/5.0/bpy.types.BlendImportContextLibraries.html — read 2026-07-30.

> "source_library
> Library ID representing the blendfile from which the ID was imported. None until the ID has been linked or appended"

Source: Python API documentation, https://docs.blender.org/api/5.0/bpy.types.BlendImportContextItem.html — read 2026-07-30.

> "Library
> ✗
> ✓
> References to an external blend-file.
> Access from the Outliner's Blender File view."

Source: manual, https://docs.blender.org/manual/en/5.0/files/data_blocks.html — read 2026-07-30.

So a library, as Blender holds it, is a data-block that stands for one blend-file, and the only location it records is that one path string.

### (b) The library is the namespace boundary for linked names — **documented**

> "name
> Unique data-block ID name (within a same type and library)"

> "name_full
> Unique data-block ID name, including library one if any"

Source: Python API documentation, https://docs.blender.org/api/5.0/bpy.types.ID.html — read 2026-07-30.

> "Data-blocks names are unique within their namespace. A data-block namespace is defined by its type, and the blendfile it is stored in.
> This means that there can be for example an Object and a Mesh named the same, but there cannot be two local objects named the same in a blendfile. However, it is possible to have one local and several linked Objects sharing the same name."

Source: manual, https://docs.blender.org/manual/en/5.0/files/data_blocks.html — read 2026-07-30.

This matters because the practical consequence of "same library or two" is whether two linked things with the same name collide or coexist. The documentation says the namespace is the blend-file the data is stored in. It does not say how Blender decides that two recorded locations name the same blend-file.

### (c) The one place the documentation shows a library being identified, it identifies it by the recorded path text — **documented**

> "Library Collisions
> Blender keeps data names unique (bpy.types.ID.name) so you can't name two objects, meshes, scenes, etc., the same by accident. However, when linking in library data from another blend-file naming collisions can occur, so it's best to avoid referencing data by name at all."

> "# Library object name lookup using a pair,
> # where the second argument is the library path matching bpy.types.Library.filepath.
> obj = bpy.data.objects["my_obj", "//my_lib.blend"]"

> "# Local object name look up using a pair,
> # where the second argument excludes library data from being returned.
> obj = bpy.data.objects["my_obj", None]"

Source: Python API documentation, https://docs.blender.org/api/5.0/info_gotchas_internal_data_and_python_objects.html — read 2026-07-30.

The key given is "the library path matching `bpy.types.Library.filepath`", and the worked example passes the path in its recorded, relative form (`//my_lib.blend`). That is a lookup, not a statement about how Blender merges or splits libraries, and the page does not say whether the match is on the literal stored text or on something normalised first.

### (d) The same file on disk can legitimately be recorded in more than one form, and the form can be changed wholesale — **documented**

Relative and absolute are both allowed, and relative is the default:

> "When you specify one of these external files, the default option is to make the path relative. Blender stores a partial path evaluated relative to the directory location of the referencing blend-file."

> "When relative paths are supported, the File Browser provides a Relative Path checkbox, when entering the path into a text field, use a double slash prefix (//) to make it so.
> Relative paths are the default but this can be changed in the File tab of the Preferences."

Source: manual, https://docs.blender.org/manual/en/5.0/files/blend/open_save.html — read 2026-07-30.

Linking offers the choice per operation:

> "Relative Path
> Reference the external blend-file using a relative path rather than an absolute one."

Source: manual, https://docs.blender.org/manual/en/5.0/files/linked_libraries/link_append.html — read 2026-07-30.

And the whole file's paths can be flipped from one form to the other after the fact:

> "Make Paths Relative
> Make all paths to external files relative to the current blend-file.
> Make Paths Absolute
> Make all paths to external files absolute (= full path from the system's root)."

Source: manual, https://docs.blender.org/manual/en/5.0/interface/window_system/topbar.html — read 2026-07-30.

> "Remap Relative
> This option remaps Relative Paths (such as linked libraries and images) when saving a file in a new location."

Source: manual, https://docs.blender.org/manual/en/5.0/files/blend/open_save.html — read 2026-07-30.

The same choice exists from outside the interface:

> "relative (bool) – When True the path is stored relative to the open blend file."

> "path_remap (str) –
> Optionally remap paths when writing the file:
> NONE No path manipulation (default).
> RELATIVE Remap paths that are already relative to the new location.
> RELATIVE_ALL Remap all paths to be relative to the new location.
> ABSOLUTE Make all paths absolute on writing."

Source: Python API documentation, https://docs.blender.org/api/5.0/bpy.types.BlendDataLibraries.html — read 2026-07-30.

> "MAKE_PATHS_RELATIVE
> Make paths of used library blendfiles relative to current blendfile."

Source: Python API documentation, https://docs.blender.org/api/5.0/bpy.types.BlendImportContext.html — read 2026-07-30.

So two recordings of one file, one relative and one absolute, are a situation the documented features can easily produce. The sources never say what Blender then treats them as.

### (e) The same recorded text does not always mean the same file — **inferred**

> "When using Blender data from linked libraries there is an unfortunate complication since the path will be relative to the library rather than the open blend-file. When the data block may be from an external blend-file pass the library argument from the bpy.types.ID.
> >>> bpy.path.abspath(image.filepath, library=image.library)"

Source: Python API documentation, https://docs.blender.org/api/5.0/info_gotchas_file_paths_and_encoding.html — read 2026-07-30.

> "library (bpy.types.Library) – The library this path is from. This is only included for convenience, when the library is not None its path replaces start."

Source: Python API documentation, https://docs.blender.org/api/5.0/bpy.path.html — read 2026-07-30.

**The step I took:** the sources say a relative path stored on linked data is measured from the library file it came from, not from the open file. A relative path recorded inside library A and the identical text recorded inside library B therefore point at two different places on disk. I conclude that identical recorded text alone cannot be what makes two locations the same, once nesting is involved. The sources do not state this conclusion; they state the resolution rule I reasoned from.

### (f) Identity travels with the library data-block, and its recorded path can be changed underneath it — **documented**

> "RELOCATE
> Relocate – Select a new path for this library, and reload all its data."

Source: Python API documentation, https://docs.blender.org/api/5.0/bpy.ops.outliner.html — read 2026-07-30.

> "Relocate the given library to one or several others"
> "library (string, (optional, never None)) – Library, Library to relocate"

Source: Python API documentation, https://docs.blender.org/api/5.0/bpy.ops.wm.html — read 2026-07-30.

> "It allows to replace it by a different file. This can be used to either fix a broken linked library (e.g. because the file was moved or renamed), or to switch to a variation of the same data in a different file."

> "A right-click a linked ID will show the Relocate option in the ID Data sub-menu. It allows to relocate a directly linked ID by another one, from the same library or from a different one."

Source: manual, https://docs.blender.org/manual/en/5.0/files/linked_libraries/link_append.html — read 2026-07-30.

Two things are visible here. The operator takes the library as a string parameter labelled "Library", separate from the path arguments beside it — what that string is, the documentation does not say; and one library can be pointed at a new path, or at "one or several others". Nothing says what happens if the new path is the path another library in the same file already records.

*(Changed by the citation check, 2026-08-09: this used to read "The library is addressed by a name (the operator's `library` argument is a string), separately from its path". The page says only that the parameter is a string called "Library"; that it is a name, and that a name is a different thing from a path, is a step the page does not carry.)*

### (g) In 5.0, one source library can own extra library data-blocks that are not extra files — **documented**

> "Name collisions are averted by storing these packed IDs into a new type of 'archive' libraries (and their namespaces). These libraries:
> * Only contain packed IDs.
> * Are owned and managed by their 'real' library data-block, called an 'archive parent'."

> "These packed IDs are de-duplicated across blend-files, so e.g. if a shot file and several of its dependencies all use the same util geometry node, there will be a single copy of that geometry node in the shot file."

> "In case there are several versions of a same ID (e.g. linked at different moments from a same library, which has been modified in-between), there will be several packed IDs."

Source: release notes, https://developer.blender.org/docs/release_notes/5.0/core/ — read 2026-07-30.

Matching properties exist on the library type:

> "archive_libraries
> Archive libraries of packed IDs, generated (and owned) by this source library"

> "archive_parent_library
> Source library from which this archive of packed IDs was generated"

> "is_archive
> This library is an 'archive' storage for packed linked IDs originally linked from its 'archive parent' library."

Source: Python API documentation, https://docs.blender.org/api/5.0/bpy.types.Library.html — read 2026-07-30.

So in 5.0 the count of library data-blocks in a file is not the same as the count of distinct external files it points at: archive libraries are extra library data-blocks with their own namespaces, generated from one source library. The release note also says that two batches linked at different times from one and the same library produce several packed IDs — the library is "a same library" throughout, while the packed copies multiply.

### (h) For appended (not linked) data, sameness of source is recorded as a path plus a name — **documented**

> "library_weak_reference
> Weak reference to a data-block in another library .blend file (used to re-use already appended data instead of appending new copies)"

Source: Python API documentation, https://docs.blender.org/api/5.0/bpy.types.ID.html — read 2026-07-30.

> "Read-only external reference to a linked data-block and its library file"
> "filepath
> Path to the library .blend file"
> "id_name
> Full ID name in the library .blend file (including the two leading 'id type' chars)"

Source: Python API documentation, https://docs.blender.org/api/5.0/bpy.types.LibraryWeakReference.html — read 2026-07-30.

> "Append (Reuse Data)
> Specific to the Asset Browser.
> The first time an asset is used, it will be appended, including its dependencies, just like described previously. However, Blender will keep track of where it originated, and the next time the asset is used, as much data as possible will be reused."

Source: manual, https://docs.blender.org/manual/en/5.0/editors/preferences/file_paths.html — read 2026-07-30.

> "APPEND_LOCAL_ID_REUSE
> Try to re-use previously appended matching IDs when appending them again, instead of creating local duplicates."

Source: Python API documentation, https://docs.blender.org/api/5.0/bpy.types.BlendImportContext.html — read 2026-07-30.

The pair that gets recorded is therefore (library filepath, ID name). What counts as a match on the filepath half is not stated. Note also that these recorded paths are explicitly not treated as live references:

> "SKIP_WEAK_REFERENCES:
> Skip Weak References.
> Skip weak reference paths. Those paths are typically 'nice to have' extra information, but are not used as actual source of data by the current .blend file."

Source: Python API documentation, the "File Path Foreach Flag Items" enum listing, https://docs.blender.org/api/5.0/bpy_types_enum_items/file_path_foreach_flag_items.html — read 2026-07-30. That enum is the `flags` parameter of `bpy.types.BlendData.file_path_foreach`, https://docs.blender.org/api/5.0/bpy.types.BlendData.html.

### (i) From Cycles' own files: each entry in the list of libraries is treated as one library — **observed**

```
    # Map of versions used by libraries.
    library_versions = {}
    library_versions[bpy.data.version] = [None]
    for library in bpy.data.libraries:
        library_versions.setdefault(library.version, []).append(library)

    # Do versioning per library, since they might have different versions.
```
File: `cycles_5.0/version_update.py`, lines 97–103.

```
        for scene in bpy.data.scenes:
            if scene.library not in libraries:
                continue
```
File: `cycles_5.0/version_update.py`, lines 110–112.

What this shows, read as text: Cycles walks the file's list of libraries, treats each entry as one library with one version, and decides which library a data-block belongs to by comparing the data-block's `library` against those entries. Sameness is settled by which library data-block a linked thing points at, not by comparing paths. It does not show how the list of libraries came to have the entries it has — which is the actual question.

### The other reading (asset libraries registered in Preferences)

If "recorded library location" is read as a Preferences asset-library entry, the sources say the entry is a name plus a directory, and that the name is explicitly not required to be unique:

> "name
> Identifier (not necessarily unique) for the asset library"
> "path
> Path to a directory with .blend files to use as an asset library"

Source: Python API documentation, https://docs.blender.org/api/5.0/bpy.types.UserAssetLibrary.html — read 2026-07-30.

> "An asset library is a directory on your drive that is registered in the Preferences as an asset library. Registering it means that you give the library a name (like "Sprite Fright") and the location on drive (like /home/sybren/projects/sprite-fright/assets)."

Source: manual, https://docs.blender.org/manual/en/5.0/files/asset_libraries/introduction.html — read 2026-07-30.

> "Name and on-drive directory paths of asset libraries. To make Blender aware of an asset library, add it to this list. The name is for your reference only, and will appear in asset library selectors."

Source: manual, https://docs.blender.org/manual/en/5.0/editors/preferences/file_paths.html — read 2026-07-30.

Even here, nothing says what happens if two entries name the same directory, or whether two entries with the same name are then two libraries or one. **Label for this paragraph: documented** for the quotes, **not in the sources** for the identity rule.

### Not covered

- Whether the comparison is on the stored path text or on the path after resolving `//`, `..`, and the like.
- Whether upper and lower case are folded in the comparison, and whether that differs by operating system.
- Whether links, shortcuts, junctions, network shares written two ways, or two mount points onto one disk are seen through.
- What happens when a library is relocated onto a path that another library in the same file already records — merge, two libraries, or refusal.
- Whether one file can hold two library data-blocks recording the same path, and what its list of libraries would then show.
- Whether the answer differs between the moment of linking, the moment of loading the file, and a reload or relocate.
- Whether a library reached indirectly (through another library) and the same file linked directly become one library or two.

### Outside the interface

The material above is nearly all from the Python API documentation and describes the data model, not the window: the library type, its filepath, the ID/library pair lookup, the import context, the weak reference. None of it is described as depending on there being an interface, so I have no reason from the sources to think it differs; the sources do not say so either way.

Two edges worth naming, both **documented**:

- The manual describes Reload and Relocate as Outliner context-menu items ("When the Outliner is in the Blender File Display Mode…", https://docs.blender.org/manual/en/5.0/files/linked_libraries/link_append.html). The API documentation lists matching operators (`bpy.ops.wm.lib_reload`, `bpy.ops.wm.lib_relocate`, `bpy.ops.outliner.lib_operation`, https://docs.blender.org/api/5.0/bpy.ops.wm.html and https://docs.blender.org/api/5.0/bpy.ops.outliner.html). The sources do not say whether these run when Blender is started without a window.
- There is a command-line switch that changes library-override handling on file load: "**--disable-liboverride-auto-resync** — Do not perform library override automatic resync when loading a new blend-file." (manual, https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html). That is about overrides, not about library identity, but it is the only documented command-line control touching linked libraries I found.

The sources do not say whether library identity is decided the same way when Blender is driven from outside its own window.

**Brought up to date by the hand tests, 2026-08-09.** Every test recorded in the *Observed* section below was run with Blender started with no window, and on the rented Linux machine there is no display server at all, so no window was possible there. All the results below therefore hold when Blender is driven from outside its own interface. What was **not** done is a comparison with the same steps carried out by clicking: neither machine ran a window this round, so nothing is said about whether the two differ. One thing that does turn on being driven from outside is written up under test 8 below — the relocate call returned `{'FINISHED'}` and did not move the library it named.

### Operating system

The sources do not say that the identity rule differs by operating system — because they do not state the rule at all.

They do document, elsewhere, that path handling differs between systems, which is why this question has an operating-system edge at all. All **documented**:

- "bpy.path.resolve_ncase(path) — Resolve a case insensitive path on a case sensitive system, returning a string with the path if found else return the original path." (https://docs.blender.org/api/5.0/bpy.path.html)
- "bpy.path.native_pathsep(path) — Replace the path separator with the systems native os.sep." (same page)
- "bpy.path.basename(path) — Equivalent to os.path.basename, but skips a "//" prefix. Use for Windows compatibility." (same page)
- On Windows drives, from the OBJ exporter's path-mode options: "Uses relative paths in every case (except when on a different drive on Windows)." (https://docs.blender.org/manual/en/5.0/files/import_export/obj.html)
- "Windows (NTFS) only supports up to 255 chars in its file names." (https://developer.blender.org/docs/release_notes/5.0/core/)

None of these is stated to apply to deciding whether two library locations are one library.

**Brought up to date by the hand tests, 2026-08-09.** Of the tests in the *Observed* section below, one came out differently on the two computers and the rest came out the same. The one that differs is capital and small letters, test 4: on the rented Linux machine the differently-spelled name is not a file and the link was refused, and on the Creator's Mac it is the same file and two library entries resulted. Windows was not tested at all, because no Windows machine has run it. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

---

## Where I looked, for question 19

**Searched (whole-tree text search over `manual_5.0`, `api_5.0`, `release_notes_5.0`, and `cycles_5.0`):**

- library, libraries, librar\* (stem)
- "same library", "same file", "same blend-file", "same blend file", "same path", "same source", "same library file"
- "two libraries", "one library", "per library", "different librar", "separate librar", "another librar", "existing librar"
- "already linked", "already exist", "linked twice", "duplicate librar"
- "unique", "identical", "considered the same", "treated as the same", "regarded as", "one and the same", "distinguish"
- "merge", "merged", "de-duplicat", "deduplicat"
- "canonical", "normalize", "normalise", "symlink", "symbolic link"
- "case sensitive", "case-sensitive", "case-insensitive"
- "library path", "library file path", "relative path", "namespace"
- "relocate", "reload", "lib_relocate", "lib_reload", "make_paths_absolute", "make_paths_relative", "find_missing_files", "report_missing_files"
- "bpy.data.libraries", "my_lib.blend", "library path matching"

**Pages opened in full or in the relevant part:**

Manual — files/linked_libraries/index, files/linked_libraries/link_append, files/data_blocks, files/file_paths, files/blend/open_save (Relative Paths and Remap Relative), files/blend/rename, files/asset_libraries/introduction, editors/preferences/file_paths (Asset Libraries and Import Method), editors/asset_browser (Asset Details and import options), editors/outliner/interface (Display Modes and header), interface/window_system/topbar (External Data menu), advanced/command_line/arguments (library-related switches), glossary/index (Asset Library, Current File Asset Library — no entry for a linked library), scene_layout/object/selecting (Select Grouped ▸ Library).

Manual picture — `_images/files_data-blocks_outliner-blender-file-view.png`, cited from https://docs.blender.org/manual/en/5.0/files/data_blocks.html. **documented, from a picture:** the Blender File view of the Outliner is shown with a single top-level row reading "Current File", under which the data types are listed (Brushes, Cameras, Collections, Images, Lights, Materials, Meshes, Objects, Scenes, Screens, Texts, Window Managers, Workspaces, Worlds). The picture in this version shows no linked library row, so it settles nothing about how libraries are listed or labelled there. I did not read more into it.

Python API — bpy.types.Library, bpy.types.LibraryWeakReference, bpy.types.BlendDataLibraries, bpy.types.BlendData (file_path_foreach, file_path_map), bpy.types.ID, bpy.types.BlendImportContext, bpy.types.BlendImportContextItem, bpy.types.BlendImportContextLibrary, bpy.types.BlendImportContextLibraries, bpy.types.UserAssetLibrary, bpy.types.AssetLibraryCollection, bpy.types.AssetLibraryReference, bpy.path, bpy.ops.wm (link, append, lib_reload, lib_relocate, id_linked_relocate), bpy.ops.outliner (lib_operation, lib_relocate, id_linked_relocate), bpy.ops.file (make_paths_absolute, make_paths_relative, find_missing_files, report_missing_files), info_gotchas_internal_data_and_python_objects (Library Collisions), info_gotchas_file_paths_and_encoding, info_gotchas_crashes (Undo & Library Data), the File Path Foreach Flag enum item listing.

Release notes 5.0 — core (Packed Data, Longer Data-Block Names, Logging, Other), pipeline_io, python_api, assets, and a keyword pass over every other subject file for "librar".

Cycles' own files — searched all of `cycles_5.0` for "librar". Hits are in `osl.py` (uses `bpy.path.abspath(filepath, library=…)` for OSL script paths) and `version_update.py` (iterates `bpy.data.libraries`); every other hit is about maths or OSL shader libraries, not blend libraries. Cycles holds no code for deciding library identity, which is expected — it consumes libraries through `bpy.data`, it does not resolve them.

**Ruled out and why:**

- The Asset Browser and asset-catalog pages: they concern directories registered in Preferences, not locations recorded inside a blend-file. Kept only as the alternate reading.
- Library Overrides pages (manual and API `IDOverrideLibrary*`): they concern editing linked data and resyncing overrides, not which library is which. Searched, nothing on identity.
- `bpy.types.CacheFile`, image, sound and font path properties: those are external files a file points at, not libraries.
- Import/export pages (OBJ, FBX, USD, glTF, Alembic): their path-mode options were read only for what they say about relative and absolute forms and about Windows drives; they do not concern blend libraries.
- The `latest` address was never used or cited.

**I did not need the web, and did not go there.**

---

## Observed — fifteen tests run by hand on 2026-08-09

**Label: observed.**

**On what kind of computer.** Two, and neither speaks for the other.

- **The rented Linux machine** — rented on Vast.ai, Ubuntu 22.04.5 LTS on kernel 6.8, one NVIDIA GeForce RTX 5060 with 8 GB on driver 580.95.05, twenty-three cores of an AMD EPYC 7K62, 49 GB of memory, and no display server at all. Blender 5.0.1, at `/opt/blender/5.0/blender`.
- **The Creator's Mac** — a MacBook Pro with an M1 Pro chip, macOS 26.5.2, Apple's own graphics. Blender 5.0.1, at `/Applications/Blender 5.0.1.app/Contents/MacOS/Blender`.

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

**The same fifteen tests were run in 4.5.12 LTS, 5.1.2 and 5.2.0 LTS as well.** Everything below came out the same in all four versions except test 12.

### 1. The control: the count can say one and can say two

*Done to:* a folder holding `libA.blend` with objects `A1` and `A2`, `libB.blend` with object `B1`, and an empty `host.blend`. *Done:* link `A1` from `libA.blend`; link `A2` from the same file with the path written exactly the same way; link `B1` from `libB.blend`.

The rented Linux machine printed:

```
HT|after-A1|library_count|1
HT|after-A1|library|0|name='libA.blend'|filepath='/root/ht-deps/e1-5.0-ops/libA.blend'
HT|after-A2-same-spelling|library_count|1
HT|after-B1-other-file|library_count|2
HT|after-B1-other-file|library|1|name='libB.blend'|filepath='/root/ht-deps/e1-5.0-ops/libB.blend'
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
HT|paths|plain='/root/ht-deps/e3-5.0-data/lib.blend'|dotted='/root/ht-deps/e3-5.0-data/./lib.blend'|updown='/root/ht-deps/e3-5.0-data/sub/../lib.blend'
HT|after-plain|library_count|1
HT|after-dot-step|library_count|1
HT|after-up-down-step|library_count|1
HT|after-fresh-reopen|library|0|name='lib.blend'|filepath='/root/ht-deps/e3-5.0-data/lib.blend'
```

The Creator's Mac printed the same counts and the same single recorded text. The two agreed.

### 4. Capital and small letters — the one test where the two computers disagree

*Done:* link `A1` through `lib.blend`, then `A2` through `LIB.blend`. A shell check was run first on each machine, so that a refusal could be told from a machine on which the name means nothing.

**On the rented Linux machine** the shell printed `ls: cannot access '/root/ht-deps/e4-5.0-data/LIB.blend': No such file or directory`, and Blender printed:

```
HT|exists|lower=True|upper=False
HT|after-lower|library_count|1
HT|link-upper|call|raised|OSError('load: /root/ht-deps/e4-5.0-data/LIB.blend failed to open blend file')
HT|after-upper|library_count|1
```

and, through the operator rather than the Python call:

```
HT|link-upper|call|raised|RuntimeError("Error: '/root/ht-deps/e4-5.0-ops/LIB.blend/Object/A2': nothing indicated\n")
```

**On the Creator's Mac** the shell listed both names as one file of the same size, and Blender printed:

```
HT|exists|lower=True|upper=True
HT|after-lower|library_count|1
HT|after-upper|library_count|2
HT|after-upper|library|0|name='lib.blend'|filepath='…/e4-5.0-data/lib.blend'
HT|after-upper|library|1|name='LIB.blend'|filepath='…/e4-5.0-data/LIB.blend'
HT|after-fresh-reopen|library_count|2
```

**The two did not agree, and the two results are not merged.** On the rented Linux machine the differently-spelled name is not a file and the link was refused. On the Creator's Mac it is the same file and two library entries resulted.

### 5. Two names on disk for one file, and two ways into one folder

*Done to:* `lib.blend` with objects `A1` to `A5`, plus `alias.blend` (a symbolic link to it), `hard.blend` (a hard link to it), and a symbolic link to the whole folder. On the rented Linux machine there was also a second mount of the folder, made with `mount --bind`. *Done:* link one object through each route.

The rented Linux machine's shell control printed `1808491 /root/ht-deps/e5-5.0-data/lib.blend` and `1808491 /root/ht-deps/e5-5.0-data/hard.blend` — one and the same file — and `MOUNT OK`. Blender printed:

```
HT|after-real|library_count|1
HT|after-symlink-to-file|library_count|2
HT|after-hardlink|library_count|3
HT|after-symlink-to-folder|library_count|4
HT|after-second-mount|library_count|5
HT|after-fresh-reopen|library|0|name='alias.blend'|filepath='/root/ht-deps/e5-5.0-data/alias.blend'
HT|after-fresh-reopen|library|1|name='hard.blend'|filepath='/root/ht-deps/e5-5.0-data/hard.blend'
HT|after-fresh-reopen|library|2|name='lib.blend'|filepath='/root/ht-deps/e5-5.0-data/lib.blend'
HT|after-fresh-reopen|library|3|name='lib.blend.001'|filepath='/root/ht-deps/e5-5.0-data-otherway/lib.blend'
HT|after-fresh-reopen|library|4|name='lib.blend.002'|filepath='/root/ht-deps/e5-5.0-data-bind/lib.blend'
```

The Creator's Mac cannot make a second mount, so it ran four of the five steps and printed the same counts up to `HT|after-fresh-reopen|library_count|4`, with the same four entries. The two agreed on the four steps both could do. The second mount is a Linux result only.

### 6. Two entries made to hold the very same recorded text

*Done to:* `host.blend` holding two libraries linked from two different files. *Done:* write the first one's recorded text into the second from Python, then save and open again.

The rented Linux machine printed:

```
HT|assignment|returned normally
HT|after-assignment|library_count|2
HT|after-assignment|library|0|name='lib1.blend'|filepath='/root/ht-deps/e6-5.0-data/lib1.blend'
HT|after-assignment|library|1|name='lib2.blend'|filepath='/root/ht-deps/e6-5.0-data/lib1.blend'
HT|after-reopen-in-process|library_count|1
HT|after-reopen-in-process|library|0|name='lib1.blend'|filepath='/root/ht-deps/e6-5.0-data/lib1.blend'
HT|after-fresh-reopen|library_count|1
```

The Creator's Mac printed the same counts, `2, 2, 1, 1`. The two agreed.

### 7. Rewriting every path in the file at once

*Done to:* the file test 2 left, holding one entry. *Done:* `bpy.ops.file.make_paths_absolute()`, then save and reopen, then `bpy.ops.file.make_paths_relative()`, then save and reopen.

The rented Linux machine printed:

```
HT|reopened|library|0|name='lib.blend'|filepath='//lib.blend'
HT|make_paths_absolute|returned={'FINISHED'}
HT|after-make-absolute|library|0|name='lib.blend'|filepath='/root/ht-deps/e7-5.0-data/lib.blend'
HT|make_paths_relative|returned={'FINISHED'}
HT|after-make-relative|library|0|name='lib.blend'|filepath='//lib.blend'
HT|reopened-after-relative|library_count|1
```

The Creator's Mac printed the same. The two agreed.

### 8. Relocating one library onto the file another library already uses — the call did not act on the library it named

*Done to:* `host.blend` with `lib1.blend` and `lib2.blend` linked. *Done:* `bpy.ops.wm.lib_relocate(library='lib2.blend', directory=…, filename='lib1.blend')`.

The rented Linux machine printed:

```
HT|two-libraries|library|0|name='lib1.blend'|filepath='/root/ht-deps/e8-5.0-data/lib1.blend'
HT|two-libraries|library|1|name='lib2.blend'|filepath='/root/ht-deps/e8-5.0-data/lib2.blend'
HT|relocate|library='lib2.blend'|returned={'FINISHED'}
HT|after-relocate|library_count|2
HT|after-relocate|library|0|name='lib1.blend'|filepath='//lib1.blend'
HT|after-relocate|library|1|name='lib2.blend'|filepath='/root/ht-deps/e8-5.0-data/lib2.blend'
HT|after-relocate|object|name='Q1'|name_full='Q1 [lib2.blend]'|from='/root/ht-deps/e8-5.0-data/lib2.blend'
```

The Creator's Mac printed the same. The two agreed.

Two further tests were run because of that. With the same set-up and a third file that no library records, the same call naming `lib2.blend` and pointing at that third file returned `{'FINISHED'}` and changed nothing at all, both when two calls were made in one run and when a single call was made in its own run, on both computers:

```
HT|calling|bpy.ops.wm.lib_relocate(library='lib2.blend', filepath='/root/ht-deps/e15-5.0-lib2-lib3/lib3.blend', directory='/root/ht-deps/e15-5.0-lib2-lib3/', filename='lib3.blend')
HT|relocate|returned={'FINISHED'}
HT|after|library|0|name='lib1.blend'|filepath='/root/ht-deps/e15-5.0-lib2-lib3/lib1.blend'
HT|after|library|1|name='lib2.blend'|filepath='/root/ht-deps/e15-5.0-lib2-lib3/lib2.blend'
```

So, with Blender started with no window, this call returns `{'FINISHED'}` whatever it is given; nothing moves when the target is a file no library records; and when the target is a file an existing library records, that existing library's recorded text is rewritten into the `//` form while the library named in the call is left where it was. **What this therefore does not settle:** what happens when a library really is relocated onto a file another library already records. The named library never moved, so there was no collision to watch. That is written up as still open below.

### 9. The same file reached directly and through another library

*Done to:* `lib.blend` with objects `A1` and `A2`; `mid.blend`, which links `A1` from it and holds it in a local collection; and `host.blend`. *Done:* link the collection from `mid.blend`, then link `A2` straight out of `lib.blend`.

The rented Linux machine printed:

```
HT|after-indirect|library_count|2
HT|after-indirect|library|0|name='lib.blend'|filepath='/root/ht-deps/e9-5.0-data/lib.blend'
HT|after-indirect|library|1|name='mid.blend'|filepath='/root/ht-deps/e9-5.0-data/mid.blend'
HT|after-direct|library_count|2
HT|after-direct|object|name='A1'|name_full='A1 [lib.blend]'|from='/root/ht-deps/e9-5.0-data/lib.blend'
HT|after-direct|object|name='A2'|name_full='A2 [lib.blend]'|from='/root/ht-deps/e9-5.0-data/lib.blend'
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
HT|resolve|name='x.blend'|filepath='//x.blend'|parent=None|abspath='/root/ht-deps/e10-5.0-data/two/x.blend'
HT|resolve|name='x.blend.001'|filepath='//../one/x.blend'|parent='/root/ht-deps/e10-5.0-data/one/mid.blend'|abspath='/root/ht-deps/e10-5.0-data/one/../one/x.blend'
```

So the text that `mid.blend` holds as `//x.blend` appears in the host file as `//../one/x.blend`. The two identical texts the test was built to produce did not arise. The Creator's Mac printed the same. The two agreed.

### 11. Two different files that share a file name

*Done:* link an object from `p/lib.blend`, then one from `q/lib.blend`.

The rented Linux machine printed:

```
HT|after-p|library_count|1
HT|after-p|library|0|name='lib.blend'|filepath='/root/ht-deps/e11-5.0-data/p/lib.blend'
HT|after-q|library_count|2
HT|after-q|library|1|name='lib.blend.001'|filepath='/root/ht-deps/e11-5.0-data/q/lib.blend'
HT|after-fresh-reopen|library_count|2
```

The Creator's Mac printed the same. The two agreed. Together with test 1 this shows what this answer says above the sources never state: the printed `name` of a library data-block is the bare file name, and a numbered suffix is added when two would clash.

### 12. Packing linked data — the one place the versions differ

*Done:* ask whether the three archive properties exist, then link with packing asked for.

The rented Linux machine, Blender 5.0.1, printed:

```
HT|prop|is_archive|present=True
HT|prop|archive_parent_library|present=True
HT|prop|archive_libraries|present=True
HT|pack|returned normally
HT|after-pack|library_count|2
HT|after-pack|library|0|name='lib.blend'|filepath='/root/ht-deps/e12-5.0-data/lib.blend'
HT|after-pack|library|1|name='lib.blend.001'|filepath='/root/ht-deps/e12-5.0-data/lib.blend'
HT|after-pack|object|name='A1'|name_full='A1 [lib.blend.001]'|from='/root/ht-deps/e12-5.0-data/lib.blend'
HT|archive|name='lib.blend'|filepath='/root/ht-deps/e12-5.0-data/lib.blend'|is_archive=False|archive_parent=None|archives=['/root/ht-deps/e12-5.0-data/lib.blend']
HT|archive|name='lib.blend.001'|filepath='/root/ht-deps/e12-5.0-data/lib.blend'|is_archive=True|archive_parent='/root/ht-deps/e12-5.0-data/lib.blend'|archives=[]
```

So two library entries, holding **the same recorded text**, one with `is_archive=False` and one with `is_archive=True` whose archive parent is the first. The Creator's Mac printed the same two entries with the same two flags. The two agreed.

The same test in Blender 4.5.12 LTS printed `present=False` for all three properties and refused the call with `HT|pack|raised|TypeError("'pack' is an invalid keyword argument for load()")`, leaving no library at all. 5.0.1, 5.1.2 and 5.2.0 LTS all behaved as printed above.

### 13. Two files that record one library differently, both linked into a third

*Done to:* `lib.blend`; `a.blend`, which links from it with the relative option on; `b.blend`, which links from it with the option off; and an empty `c.blend`. *Done:* link a collection from `a.blend`, then one from `b.blend`.

The controls printed, when the two were built, `HT|built-a.blend|library|0|name='lib.blend'|filepath='//lib.blend'` and `HT|built-b.blend|library|0|name='lib.blend'|filepath='/root/ht-deps/e13-5.0-data/lib.blend'` — the two really did record the one library two different ways. The rented Linux machine then printed:

```
HT|after-a|library_count|2
HT|after-b|library_count|3
HT|after-b|library|0|name='a.blend'|filepath='/root/ht-deps/e13-5.0-data/a.blend'
HT|after-b|library|1|name='b.blend'|filepath='/root/ht-deps/e13-5.0-data/b.blend'
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

## Still open for a person with Blender 5.0 open

The researcher's own list has been pruned: everything the hand tests answered is gone. What is left is what a running Blender still has not settled.

1. **Relocating a library onto a file another library already records.** The tests reached this through `bpy.ops.wm.lib_relocate` in a Blender with no window, and the call never moved the library it was told to move. Doing it by right-clicking the entry in the Outliner's Blender File view, in a window, would settle it. Note first whether the count goes to one, then copy out anything Blender shows in the Info editor, word for word.
2. **What the Outliner's Blender File view shows for a file that holds two entries recording the same location.** The count and the recorded text are already known from a terminal; what is not known is how those two entries are labelled on screen when their recorded text is identical.
3. **Anything on Windows.** Untried — no Windows machine has run it — not answered.

---

2026-08-09 — Opus 5 (Claude Code) — Dependencies, Blender 5.0

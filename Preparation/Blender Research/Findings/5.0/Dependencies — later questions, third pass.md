# Dependencies — later questions, third pass (Blender 5.0)

**Sources consulted, and when.** Blender 5.0 user manual (`https://docs.blender.org/manual/en/5.0/`), Blender 5.0 Python API documentation (`https://docs.blender.org/api/5.0/`), Blender 5.0 release notes (`https://developer.blender.org/docs/release_notes/5.0/`), and the Cycles add-on's own files, all as supplied in `Blender Sources/` and all taken on **2026-07-30**. That snapshot date is my date read for everything below.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

I answered only question 19. I did not spawn any helper agents; all searching and reading below was done by me.

---

## 19. What decides whether two recorded library locations are treated as the same library or as two different libraries?

**LABEL: not in the sources.**

None of the three documentation sets, and none of the Cycles files, state a rule for deciding whether two recorded library locations denote one library or two. Nothing says whether Blender compares the recorded text of the two paths, or compares the files those paths resolve to; nothing says what happens if the same file on disk is recorded once as an absolute path and once as a `//` relative path; nothing says whether capitalisation, the direction of the slashes, a trailing separator, or a link/shortcut on disk changes the outcome. The record of where I looked is at the end of this answer.

What the sources *do* say bears on the question without settling it. I set out each piece with its own label, because they are not all of the same strength.

### (a) A library is a data-block, and its recorded location is one text field on it — **documented**

> "External .blend file from which data is linked"

and, for its `filepath`:

> "Path to the library .blend file"
>
> "string, default "", (never None, blend relative // prefix supported)"

Source: Python API documentation, `https://docs.blender.org/api/5.0/bpy.types.Library.html`. Read 2026-07-30.

So a recorded library location is a single string on a `Library` data-block, and that string may be in the `//` relative form.

### (b) A library is talked about as being a blendfile path — **documented**

> "Collection of source libraries, i.e. blendfile paths"

Source: Python API documentation, `https://docs.blender.org/api/5.0/bpy.types.BlendImportContextLibraries.html`. Read 2026-07-30.

This is the closest the sources come to saying what a library *is* for identity purposes: a path to a blendfile. It does not say how two such paths are compared.

### (c) When Python has to distinguish two data-blocks of the same name, the thing that tells them apart is the library's recorded path text — **documented**

> "# Library object name lookup using a pair,
> # where the second argument is the library path matching bpy.types.Library.filepath.
> obj = bpy.data.objects["my_obj", "//my_lib.blend"]"

Source: Python API documentation, "Library Collisions" section of `https://docs.blender.org/api/5.0/info_gotchas_internal_data_and_python_objects.html`. Read 2026-07-30.

The example uses the relative form `//my_lib.blend`, and says the second argument must *match* `Library.filepath`. So the handle by which a library is named in this lookup is the recorded string as recorded, not a resolved absolute path.

**Caution.** This is documentation of how to *look up* a data-block. It is not a statement that Blender decides library sameness by string comparison. Reading it that way would be inference, and weak inference, so I am not making it.

### (d) A library is a namespace, and names are unique within one — **documented**

> "Data-blocks names are unique within their namespace. A data-block namespace is defined by its type, and the blendfile it is stored in."

and

> "it is possible to have one local and several linked Objects sharing the same name"

Source: manual, `https://docs.blender.org/manual/en/5.0/files/data_blocks.html`. Read 2026-07-30.

Also, on the data-block name itself:

> "Unique data-block ID name (within a same type and library)"

Source: Python API documentation, `https://docs.blender.org/api/5.0/bpy.types.ID.html`. Read 2026-07-30.

The same manual page lists `Library` as one of the data-block types:

> "Library ... References to an external blend-file. Access from the Outliner’s Blender File view."

Source: manual, `https://docs.blender.org/manual/en/5.0/files/data_blocks.html`. Read 2026-07-30.

**Inferred, one step:** because `Library` is itself a data-block, and data-block names are unique within a type and file, the current file cannot hold two `Library` data-blocks bearing the same data-block name. The step a reader has to supply is that the general rule about data-block names applies to the `Library` type too; the page states the rule and lists `Library` in its table, but does not join the two up. What a `Library` data-block's *name* is — whether it is the file path, the bare file name, or something else — is not stated anywhere I found, so this does not tell you what two recorded locations have to have in common to collide.

### (e) A relative location means different things depending on which file recorded it — **documented**

> "library (bpy.types.Library) – The library this path is from. This is only included for convenience, when the library is not None its path replaces start."

Source: Python API documentation, `bpy.path.abspath`, `https://docs.blender.org/api/5.0/bpy.path.html`. Read 2026-07-30.

And, for the `//` form generally:

> "Blender stores a partial path evaluated relative to the directory location of the referencing blend-file."

Source: manual, "Relative Paths" section of `https://docs.blender.org/manual/en/5.0/files/blend/open_save.html`. Read 2026-07-30.

**Inferred, one step:** the same recorded text in the `//` form can point at two different files when it was recorded in two different files, and two different recorded texts can point at one file. The step is applying the stated resolution rule to two recordings at once; the sources describe resolving one path, not comparing two. This is why the question matters, and it is exactly the case the sources leave unanswered.

### (f) The recorded form is a choice made at link time, and can be rewritten in bulk afterwards — **documented**

At link time:

> "Relative Path
>
> Reference the external blend-file using a relative path rather than an absolute one."

Source: manual, `https://docs.blender.org/manual/en/5.0/files/linked_libraries/link_append.html`. Read 2026-07-30.

The same choice exists from Python, as `relative` on `bpy.data.libraries.load`:

> "relative (bool) – When True the path is stored relative to the open blend file."

Source: Python API documentation, `https://docs.blender.org/api/5.0/bpy.types.BlendDataLibraries.html`. Read 2026-07-30.

Afterwards, in bulk:

> "bpy.ops.file.make_paths_absolute()
>
> Make all paths to external files absolute"
>
> "bpy.ops.file.make_paths_relative()
>
> Make all paths to external files relative to current .blend"

Source: Python API documentation, `https://docs.blender.org/api/5.0/bpy.ops.file.html`. Read 2026-07-30.

And when saving elsewhere:

> "This option remaps Relative Paths (such as linked libraries and images) when saving a file in a new location."

Source: manual, "Remap Relative" in `https://docs.blender.org/manual/en/5.0/files/blend/open_save.html`. Read 2026-07-30.

So the recorded text for one and the same file on disk is not fixed: it depends on a checkbox at link time and can be rewritten later. What the sources never say is what happens if such a rewrite makes two previously different recordings identical, or makes two previously identical recordings different.

### (g) One case where two library entries deliberately exist for one source file — **documented**

Blender 5.0 introduced packed linked data, and with it a second kind of library data-block:

> "Name collisions are averted by storing these packed IDs into a new type of 'archive' libraries (and their namespaces). These libraries:
>
> * Only contain packed IDs.
> * Are owned and managed by their 'real' library data-block, called an 'archive parent'."

Source: release notes, `https://developer.blender.org/docs/release_notes/5.0/core/`. Read 2026-07-30.

The API side matches:

> "is_archive
>
> This library is an 'archive' storage for packed linked IDs originally linked from its 'archive parent' library."

and

> "archive_libraries
>
> Archive libraries of packed IDs, generated (and owned) by this source library"

Source: Python API documentation, `https://docs.blender.org/api/5.0/bpy.types.Library.html`. Read 2026-07-30.

The same release note also states that more than one packed copy of one data-block can exist:

> "In case there are several versions of a same ID (e.g. linked at different moments from a same library, which has been modified in-between), there will be several packed IDs."

Source: release notes, `https://developer.blender.org/docs/release_notes/5.0/core/`. Read 2026-07-30.

So in 5.0 there are library data-blocks whose separateness is decided by something other than a recorded location at all — an archive library is generated from, and owned by, its parent. This is a documented exception to any assumption that "one recorded location, one library" is the whole story.

### (h) Each library data-block is a distinct thing that other code compares by identity, not by path — **observed**

Cycles' own versioning code groups libraries and then tests membership on the library object itself:

```
    # Map of versions used by libraries.
    library_versions = {}
    library_versions[bpy.data.version] = [None]
    for library in bpy.data.libraries:
        library_versions.setdefault(library.version, []).append(library)

    # Do versioning per library, since they might have different versions.
```

and further down, for each data-block:

```
        for scene in bpy.data.scenes:
            if scene.library not in libraries:
                continue
```

Source: Cycles' own files, `cycles_5.0/version_update.py`, lines 97–101 and 110–112 (inside `do_versions`). Read 2026-07-30.

This is an **observation about the file**: Cycles walks `bpy.data.libraries` as a list of separate entries, reads a `version` from each, and matches a data-block to its library by comparing the library object, never by comparing paths. What it does *not* show is how that list came to have the entries it has — which is the question. Treating this as evidence about how Blender decides sameness would be a step past what the file says, and I am not taking it.

**Not covered.** Everything about the actual deciding rule. Specifically, the sources do not settle: whether comparison is on the recorded text or on the resolved file; whether the `//` form and the absolute form of one file count as one library or two; whether differing capitalisation, `/` versus `\`, a redundant `./` or `..` segment, or a trailing separator make a difference; whether a symbolic link or shortcut pointing at the same file counts as the same library; what happens when a relocate or a bulk path rewrite makes two recordings collide; whether the same library reached directly and reached indirectly through a second library is one entry or two; and what a `Library` data-block's own name is derived from. My answer also does not cover the possibility that Blender never merges at all and simply keeps whatever the file recorded — the sources do not rule that in or out either.

**Ambiguity flagged.** I read "library" as a linked `.blend` file — the `Library` data-block sense used throughout the manual's "Linked Libraries" pages — because the sheet's own preamble frames these questions as being about what a `.blend` file points at outside itself. The other available reading is "asset library", the named directories registered in the Preferences. On that other reading the sources say a little more, and I note it only because I chose against it: `UserAssetLibrary.name` is described as "Identifier (not necessarily unique) for the asset library" and `path` as "Path to a directory with .blend files to use as an asset library" (Python API documentation, `https://docs.blender.org/api/5.0/bpy.types.UserAssetLibrary.html`, read 2026-07-30) — so on that reading the name explicitly does *not* have to be unique, and what distinguishes two entries is still not stated.

**Outside the interface.** The sources do not say. Nothing I found states a rule for library identity at all, so nothing states one that varies between clicking and being driven from outside. What is documented is that the operations that create and rewrite these recordings are all reachable from outside the interface: `bpy.data.libraries.load(filepath, ..., relative=False)` records a library with the relative form chosen by an argument rather than a checkbox (Python API documentation, `https://docs.blender.org/api/5.0/bpy.types.BlendDataLibraries.html`), and `bpy.ops.wm.link`, `bpy.ops.wm.lib_relocate`, `bpy.ops.wm.lib_reload`, `bpy.ops.file.make_paths_relative` and `bpy.ops.file.make_paths_absolute` are all ordinary operators (Python API documentation, `https://docs.blender.org/api/5.0/bpy.ops.wm.html` and `https://docs.blender.org/api/5.0/bpy.ops.file.html`). Both read 2026-07-30. Whether the sameness decision behaves identically when a file is merely opened for a render, with nobody linking anything, is not addressed anywhere I looked.

**Operating system.** The sources do not say, for library identity specifically. Two general path facts are documented in the Python API and are the nearest thing to an operating-system difference that could bear on it:

> "bpy.path.resolve_ncase(path)
>
> Resolve a case insensitive path on a case sensitive system, returning a string with the path if found else return the original path."

and

> "bpy.path.native_pathsep(path)
>
> Replace the path separator with the systems native os.sep."

Source: Python API documentation, `https://docs.blender.org/api/5.0/bpy.path.html`. Read 2026-07-30.

These document that Blender's own tooling expects both case sensitivity and the path separator to differ between systems. They are helper functions offered to script authors; nothing states that Blender applies either of them when deciding whether two recorded library locations are the same. Drawing that conclusion would be inference beyond what the text supports, so I do not draw it.

### Record of where I looked

Searched, case-insensitively, across the whole of `manual_5.0`, `api_5.0` and `release_notes_5.0` at once, then followed up on every hit worth reading:

- Phrases aimed straight at the question: "same library", "two libraries", "duplicate librar", "distinct librar", "separate librar", "single librar", "one library", "already linked", "linked twice", "loaded twice", "same file twice", "same blend-file", "same blend file", "library data-block", "library path", "per-library".
- Words for merging or telling apart: "canonical", "normaliz", "normalis", "dedup", "de-dup", "collision", "conflict", "identif", "match", "uniq", "namespace" — the last six also run as a combined filter against every line in all three sources containing "librar".
- Words for the ways two paths can differ while meaning one file: "symlink", "symbolic link", "UNC", "drive letter", "trailing slash", "backslash", "case-sensitive", "case sensitive", "case-insensitive", "junction".
- Every line in all three sources containing both "librar" and "path".
- `grep -i librar` over the whole of `cycles_5.0` (Python and C++ alike).

Pages opened and read in full or in the relevant part:

- Manual: `files/linked_libraries/index`, `files/linked_libraries/link_append` (Link, Append, Reload Library, Relocate Library, Relocate Linked ID, Make Local, Known Limitations), `files/data_blocks` (whole page, including the data-block type table and "Name & Rename"), `files/blend/open_save` ("Remap Relative", "Relative Paths"), `files/blend/packed_data`, `files/file_paths`, `files/asset_libraries/introduction`, `editors/outliner/interface`, `editors/preferences/save_load`, `editors/preferences/file_paths`, `glossary/index` (the Asset Library and Current File Asset Library entries), `advanced/command_line/arguments` (every "librar" hit), `troubleshooting/python`.
- Manual picture: `_images/files_data-blocks_outliner-blender-file-view.png`, the Outliner's Blender File view, opened in case it showed how library entries are labelled. It shows only the display-mode menu and a "Current File" tree with no library entries in it, so it settles nothing here.
- Python API: `bpy.types.Library`, `bpy.types.BlendDataLibraries`, `bpy.types.LibraryWeakReference`, `bpy.types.BlendImportContext`, `bpy.types.BlendImportContextItem`, `bpy.types.BlendImportContextLibrary`, `bpy.types.BlendImportContextLibraries`, `bpy.types.ID`, `bpy.types.UserAssetLibrary`, `bpy.path` (whole module), `bpy.ops.wm` (`link`, `append`, `lib_reload`, `lib_relocate`), `bpy.ops.file` (`find_missing_files`, `make_paths_absolute`, `make_paths_relative`, `report_missing_files`), `info_gotchas_internal_data_and_python_objects` ("Library Collisions").
- Release notes: `core` (whole file — "Packed Data" is the only library-relevant part), `pipeline_io` (whole file), `assets` (every "librar" hit), `python_api` (searched for "librar", no hits at all).
- Cycles: `version_update.py` around lines 90–115, and `osl.py` lines 190–200 and 237, which use `bpy.path.abspath(..., library=...)` to resolve a script path against its library.

Ruled out, and why:

- `manual/troubleshooting/python` — its "duplicate libraries in your search path" is about duplicate Python shared-library files such as `_socket.pyd`, not about linked `.blend` libraries.
- Every `bpy_struct.is_property_overridable_library` / `property_overridable_library_set` hit across hundreds of API pages — these are inherited boilerplate on every RNA type and concern library overrides on properties, not library identity.
- `asset_library_identifier` on `bpy.ops.node`, `bpy.ops.poselib`, `bpy.ops.geometry`, `bpy.ops.object`, `bpy.ops.sequencer`, `bpy.ops.brush` — these identify a registered asset library in Preferences, not a linked `.blend`.
- `freestyle.types` "library path of the ViewShape" and `bpy.types.PointCacheItem.use_library_path` — different subjects (Freestyle view map, and where a disk cache is written).
- The FBX exporter's relative-path options in `addons/import_export/scene_fbx`, which do discuss paths on different Windows drives, but describe an exporter's own path writing and say nothing about linked libraries.
- Cycles' C++ and header files under `cycles_5.0/source` — every "librar" hit there is inside a URL in a comment citing a paper.

### Things a person with Blender 5.0 open could settle by hand, in a few minutes

Each of these is a direct test of the rule the sources do not state. In each case, do the work in a saved file, and read the result in the Outliner set to **Blender File** display mode (which lists linked blend-files) and by running `for l in bpy.data.libraries: print(repr(l.name), repr(l.filepath))` in the Python Console — the pair tells you both how many library entries exist and what each one recorded.

1. **Same file, two recorded forms.** Save `main.blend` in a folder that also contains `lib.blend`. File ‣ Link, pick an object from `lib.blend`, with **Relative Path** ticked. Then File ‣ Link again, pick a *different* object from the same `lib.blend`, with **Relative Path** unticked. Run the print above. One entry or two? Note both names and both filepaths.

2. **Rewriting the recording until two collide.** Starting from whatever state test 1 leaves you in, if you have two entries, run File ‣ External Data ‣ Make Paths Relative and print again. Do the two entries merge into one, do they both survive with the same `filepath`, or does one get a suffixed name? Then run Make Paths Absolute and print again.

3. **Capitalisation.** On the machine you have (note which operating system), link an object using the file browser normally, then in the Python Console set the library's `filepath` to the same path with the file name's capitalisation changed (e.g. `Lib.blend` for `lib.blend`), save, reopen, and print. One entry or two, and does the data still load?

4. **Slashes and redundant segments.** Same as test 3, but change only the separators (`/` versus `\` on Windows) or insert a redundant `assets/../assets/` segment into the recorded path. Save, reopen, print.

5. **A link on disk.** Make a symbolic link (or Windows shortcut/junction) `lib_alias.blend` pointing at `lib.blend`. Link one object through `lib.blend` and another through `lib_alias.blend`. Print. One entry or two?

6. **Relocating onto an existing library.** With two genuinely different libraries linked, right-click one in the Outliner's Blender File mode, choose **Relocate**, and point it at the file the *other* library already uses. Print. Do you end with one entry or two, and what happens to the data-blocks that were in the relocated one?

7. **Direct and indirect arrival.** Have `main.blend` link an object from `lib.blend`, and also link a collection from `mid.blend` where `mid.blend` itself links from `lib.blend`. Open `main.blend` and print. Is `lib.blend` one entry or two, and what does `Library.parent` report for each?

8. **Headless check.** Repeat whichever of the above gave the most surprising result with `blender --background main.blend --python-expr "import bpy; [print(repr(l.name), repr(l.filepath)) for l in bpy.data.libraries]"` and confirm the count and the recorded strings match what the interface showed.

---

9 August 2026 — Opus 5 (Claude Code) — Dependencies, Blender 5.0

# Dependencies — later questions, third pass (Blender 5.2)

Sources consulted, all read on **2026-07-30** (the snapshot date given to me):

- Blender 5.2 manual (`Blender Sources/manual_5.2`, pages under `https://docs.blender.org/manual/en/5.2/`)
- Blender 5.2 Python API documentation (`Blender Sources/api_5.2`, pages under `https://docs.blender.org/api/5.2/`)
- Blender 5.2 release notes (`Blender Sources/release_notes_5.2`, pages under `https://developer.blender.org/docs/release_notes/5.2/`)
- Cycles' own files as shipped with Blender 5.2 (`Blender Sources/cycles_5.2`)

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

I answered only question 19. I did not use a helper agent for any part of this work, so there is nobody else's reading to vouch for.

---

## 19. What decides whether two recorded library locations are treated as the same library or as two different libraries?

**Reading of the question.** I take "library" in the sense the sheet's topic uses it: a `.blend` file that another `.blend` file links data from — what Blender calls a Library data-block. "Recorded library location" I take to mean the path stored in the referencing file for such a library. The question was ambiguous in one way and I flag it rather than hide it: Blender uses the word "library" for a second, unrelated thing — an **asset library**, a directory or URL registered in the Preferences. I say at the end what the sources give for that other reading.

**LABEL: not in the sources.**

The rule itself — what test Blender applies to two recorded locations to decide they are one library or two — is not stated anywhere in the 5.2 manual, the 5.2 Python API documentation, the 5.2 release notes, or Cycles' own files. What the sources do give is the shape of the thing being compared, and one nearby matching rule for a different case. I set those out below, then the record of where I looked.

### What the sources do say (documented)

**A library is a data-block, and it holds one path.**

> "Library
>
> ✗
>
> ✓
>
> References to an external blend-file.
> Access from the Outliner's Blender File view."

Source: manual, `https://docs.blender.org/manual/en/5.2/files/data_blocks.html`, read 2026-07-30.

> "External .blend file from which data is linked"
>
> "filepath
>
> Path to the library .blend file (default "", never None, blend relative // prefix supported)"

Source: Python API, `https://docs.blender.org/api/5.2/bpy.types.Library.html`, read 2026-07-30.

The words "blend relative // prefix supported" are the load-bearing part for this question: the very same file on disk can be recorded either as a relative path beginning `//` or as an absolute path. So two recorded locations that differ as text can name the same file. The sources do not say what Blender does with that.

**The recorded location can be rewritten wholesale, in either direction.**

> "Make Paths Relative
>
> Make all paths to external files relative to the current blend-file.
>
> Make Paths Absolute
>
> Make all paths to external files absolute (= full path from the system's root)."

Source: manual, `https://docs.blender.org/manual/en/5.2/interface/window_system/topbar.html`, read 2026-07-30.

> "Remap Relative
>
> This option remaps Relative Paths
> (such as linked libraries and images) when saving a file in a new location."

Source: manual, `https://docs.blender.org/manual/en/5.2/files/blend/open_save.html`, read 2026-07-30.

These say the recorded form of a library location changes over the life of a file. They do not say whether changing the form changes which library a reference is taken to point at.

**A library is shown, and relocated, by its path.**

> "They're also listed in the Outliner's Blender File Display Mode,
> along with the path of the blend-file they originate from."

> "When the Outliner is in the Blender File Display Mode,
> a right-click on a library (a linked blend-file) will show the Relocate option.
> It allows to replace it by a different file."

Source: manual, `https://docs.blender.org/manual/en/5.2/files/linked_libraries/link_append.html`, read 2026-07-30.

> "RELOCATE
> Relocate – Select a new path for this library, and reload all its data."

Source: Python API, `https://docs.blender.org/api/5.2/bpy.ops.outliner.html`, read 2026-07-30.

So one library owns one path, and relocating swaps that one path. Nothing says what happens if the new path is one another library already holds.

**Data-block names are unique per type per file.**

> "A data-block's name is unique within its namespace. A data-block namespace is defined by its type, and the blendfile
> it is stored in."

Source: manual, `https://docs.blender.org/manual/en/5.2/files/data_blocks.html`, read 2026-07-30.

**INFERRED, with the step written out:** a Library is a data-block type (quoted above), and libraries recorded by a file are stored in that file, so two Library data-blocks in one `.blend` file cannot carry the same name. The step a reader must supply is only that the general namespace rule applies to the Library type like any other. What this does *not* settle, and the reason it is not the answer: the sources nowhere say what a Library data-block's name is, nor that it is derived from its path. So this bounds the answer without giving it.

**One library records one parent.**

> "parent
>
> (readonly)
>
> Type:
>
> Library | None"

Source: Python API, `https://docs.blender.org/api/5.2/bpy.types.Library.html`, read 2026-07-30.

I record this only as the shape of the data, not as an answer. Reading "one parent slot" as "a library file reached from two places must collapse to one entry" would be me supplying the conclusion, so I do not make that claim.

**One documented case where two library entries exist for what is on disk one file.**

> "is_archive
>
> This library is an 'archive' storage for packed linked IDs originally linked from its 'archive parent' library. (default False, readonly)"
>
> "archive_libraries
>
> Archive libraries of packed IDs, generated (and owned) by this source library (default None, readonly)"
>
> "archive_parent_library
>
> Source library from which this archive of packed IDs was generated (readonly)"

Source: Python API, `https://docs.blender.org/api/5.2/bpy.types.Library.html`, read 2026-07-30.

This is documented, and it matters here: an archive library is a separate Library from the source library it was generated from, and the relationship between them is recorded explicitly rather than being worked out from paths. Whatever decides sameness for ordinary libraries, packing linked data introduces libraries that are deliberately kept distinct from their source.

The release notes for 5.2 mention the same distinction being newly exposed to Python:

> "Some limited library info is now exposed to the `bpy.data.libraries.load()` input context manager, giving in particular access to the blend-file paths of the libraries used by the loaded one"

and the printed example lists each used library once, as a path plus a flag:

> "bpy_lib.libraries(filepath='//../../../assets/nodes/star_backdrop.blend', is_archive=False)"

Source: release notes, `https://developer.blender.org/docs/release_notes/5.2/python_api/`, read 2026-07-30.

The matching API page states the form of that list:

> "A libraries attribute, which for the input is a list of all other libraries
> used by the loaded one, as named tuples (filepath, is_archive). The
> filepath may be absolute, or relative to the loaded blend-file."

Source: Python API, `https://docs.blender.org/api/5.2/bpy.types.BlendDataLibraries.html`, read 2026-07-30.

Again: the location alone is what is reported, in whichever form it was recorded, and the documentation does not say the list is reduced or compared.

### The one nearby matching rule the sources do state — and it is for a different case

For **appended** (copied-in, not linked) data, there is a documented "is this the same thing I already have" mechanism, and it is keyed on a recorded location plus a name:

> "Read-only external reference to a linked data-block and its library file"
>
> "filepath
>
> Path to the library .blend file (default "", never None, blend relative // prefix supported)"
>
> "id_name
>
> Full ID name in the library .blend file (including the two leading 'id type' chars) (default "", never None)"

Source: Python API, `https://docs.blender.org/api/5.2/bpy.types.LibraryWeakReference.html`, read 2026-07-30.

> "APPEND_LOCAL_ID_REUSE
> Try to re-use previously appended matching IDs when appending them again, instead of creating local duplicates."

Source: Python API, `https://docs.blender.org/api/5.2/bpy.types.BlendImportContext.html`, read 2026-07-30.

> "reuse_local_id (bool) – If True,try to re-use previously appended matching ID on new append."

Source: Python API, `https://docs.blender.org/api/5.2/bpy.types.BlendDataLibraries.html`, read 2026-07-30.

**LABEL for what follows: inferred.** The step: the documentation says a weak reference holds exactly two things — the library file's path and the full ID name in it — and separately says an append can "re-use previously appended matching IDs". Joining those two into "the match is made on that recorded path together with that ID name" requires the reader to supply the link between the record and the matching, which the sources never write down. It is also explicitly about matching an *ID*, not about deciding whether two library locations are one library. And the sources are clear that these paths are side information rather than the file's real dependencies:

> "SKIP_WEAK_REFERENCES:
>
> Skip Weak References.
>
> Skip weak reference paths. Those paths are typically 'nice to have' extra information, but are not used as actual source of data by the current .blend file."

Source: Python API, `https://docs.blender.org/api/5.2/bpy_types_enum_items/file_path_foreach_flag_items.html`, read 2026-07-30.

### The contrast that makes the silence a real silence

Blender documents an identity-by-path switch for other kinds of external file. For images, sounds, fonts and movie clips the loading call carries:

> "check_existing (bool) – Using existing data-block if this file is already loaded (optional)"

Source: Python API, `https://docs.blender.org/api/5.2/bpy.types.BlendDataImages.html` (identical wording on `bpy.types.BlendDataSounds.html`, `bpy.types.BlendDataFonts.html`, `bpy.types.BlendDataMovieClips.html`), read 2026-07-30.

`bpy.data.libraries.load()` has no such parameter and no such sentence. Its full parameter list, quoted from the same source set, is `filepath, link, pack, relative, set_fake, recursive, reuse_local_id, assets_only, clear_asset_data, create_liboverrides, reuse_liboverrides, create_liboverrides_runtime` (Python API, `https://docs.blender.org/api/5.2/bpy.types.BlendDataLibraries.html`, read 2026-07-30). So Blender's documentation has the vocabulary for "this file is already loaded, reuse the existing data-block" and simply does not use it of libraries.

### Not covered

My answer does not settle any of the following, and I found nothing in the sources that does:

- Whether a relative record (`//assets/prop.blend`) and an absolute record of the same file are treated as one library or two.
- Whether difference of capital and small letters in a recorded library path makes two libraries.
- Whether a symbolic link, a hard link, a drive-letter path versus a network path, or two different mount points reaching one file count as one library.
- Whether a trailing separator, a `.` or `..` segment, or forward versus back slashes matter.
- Whether a file linked directly and also pulled in indirectly through another library ends up as one library entry or two.
- At what moment sameness is decided — when linking, when the file is loaded, or when it is saved.
- What happens if Relocate is pointed at a path another library in the same file already holds.
- What a Library data-block's name actually is, which the namespace rule above would otherwise turn into an answer.

### Outside the interface

The sources do not say. Nothing I read draws any distinction, for libraries or their paths, between Blender driven by a person and Blender started and driven from outside. The pieces involved are reachable without the interface — `bpy.types.Library` and its `filepath`, `bpy.data.libraries.load()`, `bpy.ops.wm.lib_relocate(library=…)` and `bpy.ops.wm.lib_reload(library=…)` are Python API entries (`https://docs.blender.org/api/5.2/bpy.types.Library.html`, `https://docs.blender.org/api/5.2/bpy.types.BlendDataLibraries.html`, `https://docs.blender.org/api/5.2/bpy.ops.wm.html`, read 2026-07-30) — but since the rule itself is not documented anywhere, there is nothing to say holds or fails to hold in either mode. The only command-line switch I found touching linked data at all is:

> "--disable-liboverride-auto-resync
>
> Do not perform library override automatic resync when loading a new blend-file."

Source: manual, `https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html`, read 2026-07-30. That concerns library overrides, not library identity.

### Operating system

The sources do not say, for this question. They do document, in general terms, that Blender treats paths differently across systems — which is exactly where an unstated identity rule could diverge:

> "bpy.path.resolve_ncase(path)
>
> Resolve a case insensitive path on a case sensitive system,
> returning a string with the path if found else return the original path."

> "bpy.path.basename(path)
>
> Equivalent to os.path.basename, but skips a "//" prefix.
>
> Use for Windows compatibility."

> "bpy.path.native_pathsep(path)
>
> Replace the path separator with the system's native os.sep."

Source: Python API, `https://docs.blender.org/api/5.2/bpy.path.html`, read 2026-07-30.

These are helper functions offered to scripts. Nothing in the sources connects them to how Blender itself compares two library locations, so I do not claim they do.

### The other reading of the question (asset libraries)

If "library" was meant in the Preferences sense, the sources give a little more, and I record it so the reading is not lost:

> "An asset library is a collection of Blender assets that is registered in the Preferences as an asset library."
>
> "Registering it means that you give the library a name (like "Sprite Fright")
> and the location on drive (like /home/user/projects/sprite-fright/assets)."
>
> "Unlike local asset libraries, online libraries are identified by a URL
> instead of a directory path and do not require manually managing asset files on disk."

Source: manual, `https://docs.blender.org/manual/en/5.2/files/asset_libraries/introduction.html`, read 2026-07-30.

> "name
>
> Identifier (not necessarily unique) for the asset library (default "", never None)"
>
> "path
>
> Path to a directory with .blend files to use as an asset library (default "", never None)"

Source: Python API, `https://docs.blender.org/api/5.2/bpy.types.UserAssetLibrary.html`, read 2026-07-30.

**LABEL for this part: documented**, for the fact that an asset library's name is explicitly "not necessarily unique" and that an online one is identified by URL rather than directory. **Not covered:** even here, the sources do not say whether two registered entries with different paths that reach the same directory are treated as one library or two.

### Record of where I looked

Searches run over the whole of `manual_5.2`, `api_5.2`, `release_notes_5.2` and `cycles_5.2` at once, case-insensitively (number of files matched in brackets where it helps):

- Phrases aimed straight at the rule: `same library` [2 files], `two libraries` [0], `duplicate library` [0], `library identity` [0], `matching library` [0], `existing library` [0], `reuse library` [0], `re-use library` [0], `same library file` [0], `single library` [0], `distinct librar` [0], `one library` [0], `per-library` [0], `library entry` [0], `library block` [0], `library data-block` [1 — an unrelated line in `bpy.types.BlendData`], `linked twice` [0], `linked from two` [0], `already linked` [0], `different librar` [2 — both about override property operations and outliner operators].
- The two `same library` hits: `scene_layout/object/selecting` (selecting objects, unrelated) and `files/linked_libraries/link_append` ("relocate a directly linked ID by another one, from the same library or from a different one" — about IDs, not about deciding library sameness).
- Path-comparison vocabulary: `canonical` [1 — the manual's own writing guide], `normalize`/`normalise` [only weight-painting and rotation pages], `symlink` [0], `symbolic link` [1 — asset catalogs, unrelated], `hard link` [0], `junction` [only geometry/mesh senses], `deduplicat` [only geometry nodes and modifier senses], `trailing slash` [0], `drive letter` [0], `network path` [0], `mounted` [only head-mounted displays], `path separator` [4 — wheels, OBJ export, an unrelated operator, `bpy.path.native_pathsep`], `backslash` [3, all unrelated], `resolved path` [1 — the return description of `bpy.path.resolve_ncase`], `case sensitive` [command-line render, outliner filtering, bone/object selecting, batch rename, `bpy.path.ensure_ext`, `SpaceOutliner`, `bpy.ops.object` — none about libraries].
- `already loaded` [6] — all four `check_existing` entries (images, sounds, fonts, movie clips), plus two script-loading entries in `bpy.utils` and `bpy_extras.image_utils`. None for libraries. This is the contrast I lean on above.
- `identified` [22] — only one is about libraries at all, the asset-library URL line quoted above.

Pages I opened in full or in the relevant part, and what I ruled out:

- `https://docs.blender.org/manual/en/5.2/files/linked_libraries/link_append.html` — the main Link/Append page, read end to end, including "Reload Library", "Relocate Library", "Broken Libraries", "Relocate Linked ID", "Make Local", "Known Limitations". Describes what the operations do; never states an identity test.
- `https://docs.blender.org/manual/en/5.2/files/linked_libraries/library_overrides.html` — scanned every line mentioning a path or a library. Covers overrides, hierarchies and resync; nothing on library identity.
- `https://docs.blender.org/manual/en/5.2/files/data_blocks.html` — read the Library row of the data-block table, the "Name & Rename" section in full, "Life Time"/"Protected"/"Sharing". Gives the namespace rule quoted above; does not say what a Library's name is.
- `https://docs.blender.org/manual/en/5.2/files/blend/open_save.html` — "Remap Relative" and the "Relative Paths" section in full. Explains relative versus absolute recording; silent on identity.
- `https://docs.blender.org/manual/en/5.2/files/blend/packed_data.html` and `https://docs.blender.org/manual/en/5.2/interface/window_system/topbar.html` — Pack/Unpack Linked Libraries, Make Paths Relative/Absolute, Report Missing Files, Find Missing Files. Ruled out: these change or search for paths, and never speak about two paths being one library.
- `https://docs.blender.org/manual/en/5.2/files/file_paths.html` — path templates, including `blend_name_lib` and `blend_dir_lib` which name the library a data-block comes from. Ruled out: substitution at use, nothing about identity.
- `https://docs.blender.org/manual/en/5.2/editors/outliner/interface.html` and the rest of the Outliner pages — checked the Blender File display mode description and the pictures listed on those pages (`editors_outliner_introduction_interface.png`, `editors_outliner_introduction_example.png`, `editors_outliner_interface_mode-icons.png` and the others). None of the Outliner pictures show the Blender File view with libraries listed, so no picture settles how a library is labelled. The Link & Append page carries no pictures at all.
- `https://docs.blender.org/manual/en/5.2/glossary/index.html` — searched for library entries. Only asset-library entries exist; there is no glossary entry for a linked library.
- `https://docs.blender.org/manual/en/5.2/editors/preferences/save_load.html` — the "Default To – Relative Paths" preference. Ruled out: sets the default recording form, says nothing about comparison.
- `https://docs.blender.org/manual/en/5.2/troubleshooting/python.html` — the only hit for "duplicate libraries" in the whole manual. Ruled out on reading: it is about duplicate Python `.pyd`/`.py` files on the system search path, nothing to do with `.blend` libraries.
- `https://docs.blender.org/manual/en/5.2/troubleshooting/network.html` — read in full (nine lines, about an SSL error reaching the extensions site). Ruled out.
- `https://docs.blender.org/manual/en/5.2/physics/baking.html` — "Use Library Path". Ruled out: about sharing a disk cache between linked copies, not about library identity.
- `https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html` — searched for every library mention; three hits, quoted or dismissed above.
- `https://docs.blender.org/manual/en/5.2/files/asset_libraries/introduction.html` — read for the second reading of the question.
- Python API: `bpy.types.Library`, `bpy.types.BlendDataLibraries`, `bpy.types.LibraryWeakReference`, `bpy.types.BlendImportContext`, `bpy.types.BlendImportContextItem`, `bpy.types.BlendImportContextLibrary`, `bpy.types.BlendImportContextLibraries`, `bpy.types.ID`, `bpy.types.BlendData` (including `file_path_map` and `file_path_foreach`), `bpy.types.BlendDataPathMeta`, `bpy.path`, `bpy.types.UserAssetLibrary`, and the library-related operators in `bpy.ops.wm` (`link`, `append`, `lib_reload`, `lib_relocate`, `id_linked_relocate`), `bpy.ops.outliner` (`lib_operation`, `lib_relocate`) and `bpy.ops.file` (`make_paths_absolute`, `make_paths_relative`, `find_missing_files`, `report_missing_files`, `pack_libraries`, `unpack_libraries`). All read; none states an identity rule.
- Release notes 5.2: searched every file for "librar" and "archive". Hits are in `assets.md` (online asset libraries), `core.md` (library override outliner operation, packed geonodes bakes), `python_api.md` (the `libraries.load()` addition quoted above), `grease_pencil.md` and `bugfixes.md`. `bugfixes.md` lists "Crash when relocating a linked library" as fixed; that is a crash fix and says nothing about identity. Nothing in the notes touches how library locations are compared.
- Cycles' own files: searched all of `cycles_5.2` for "librar" and for `.library`. Six hits, all in `osl.py` and `version_update.py`. Ruled out as not bearing on the question, with one thing worth recording as **observed**: `version_update.py` groups all linked libraries by their recorded Blender version and then decides which data to version by comparing each data-block's `library` against that group. The exact lines, from `Blender Sources/cycles_5.2/version_update.py`, lines 97–107:

  ```
      # Map of versions used by libraries.
      library_versions = {}
      library_versions[bpy.data.version] = [None]
      for library in bpy.data.libraries:
          library_versions.setdefault(library.version, []).append(library)

      # Do versioning per library, since they might have different versions.
  ```

  What this shows, and only this: Cycles treats each entry in `bpy.data.libraries` as one library, and identifies a data-block's library by object rather than by comparing paths. It does not show how those entries came to be, so it does not answer the question. Going further — saying this proves one entry per file on disk — would be a step past what the file says, so I do not take it.

I did not need the web, and did not use it.

### Things a person with Blender 5.2 open could settle in a few minutes

1. **Relative versus absolute record of one file.** Save `hub.blend` in a folder with `prop.blend` beside it. In `hub.blend`, use File ‣ Link… to link an object from `prop.blend` with the File Browser's **Relative Path** checkbox ticked. Then use File ‣ Link… again on the *same* `prop.blend`, this time with **Relative Path** unticked, linking a different object. Switch the Outliner to Blender File display mode and count how many `prop.blend` entries appear. Also run `len(bpy.data.libraries)` and print `[l.filepath for l in bpy.data.libraries]` in the Python console. One entry or two is the answer.
2. **Does saving collapse them?** With whatever state question 1 produced, save the file, quit Blender, reopen it, and count the entries in the Outliner's Blender File view again. Then run File ‣ External Data ‣ Make Paths Absolute, save, reopen and count once more. This tells you whether rewriting the recorded form merges two entries.
3. **Case.** On Windows or macOS with a case-insensitive disk, link from `Prop.blend` and then from `prop.blend` (same file, different capitalisation typed into the File Browser's path field). Count `bpy.data.libraries`. Repeat on Linux, where the two names would be different files, to see whether Blender's behaviour follows the disk.
4. **A link and a symbolic link.** Make a symbolic link `alias.blend` pointing at `prop.blend`. Link one object through `prop.blend` and another through `alias.blend`. Count `bpy.data.libraries` and print the filepaths.
5. **Direct plus indirect.** Make `mid.blend` link an object from `prop.blend`. Then in `hub.blend` link the object from `mid.blend` *and* link a second object directly from `prop.blend`. In the Outliner's Blender File view, see whether `prop.blend` appears once or twice, and check `bpy.data.libraries[…].parent` for each entry.
6. **Relocate onto an existing library.** With two libraries linked in one file, right-click one in the Outliner's Blender File view, choose Relocate, and point it at the file the *other* library already uses. Then count `bpy.data.libraries` and see whether Blender merges them, refuses, or leaves two entries with identical paths.
7. **What a library is called.** In the Python console run `[(l.name, l.name_full, l.filepath) for l in bpy.data.libraries]`. This settles directly whether a Library data-block's name is its path, which the documentation never states and which the namespace rule would then turn into the answer.
8. **From outside the interface.** Repeat step 1 as a script run with Blender started in background mode (`blender --background --python script.py`), using `bpy.data.libraries.load()` twice on the same file with `relative=True` then `relative=False`, and print `len(bpy.data.libraries)`. Compare with what the interface gave, to see whether the answer holds when Blender is driven from outside.

---

2026-08-09 — Opus 5 (Claude Code) — Dependencies, Blender 5.2

# Dependencies — later questions, second pass (Blender 5.2)

**Sources consulted, and when.** Blender's manual for 5.2 (`https://docs.blender.org/manual/en/5.2/`), Blender's Python API documentation for 5.2 (`https://docs.blender.org/api/5.2/`), the 5.2 release notes (`https://developer.blender.org/docs/release_notes/5.2/`), and Cycles' own files as shipped with 5.2. All four were taken on 2026-07-30, and that is my date read for everything below.

I read only the question sheet and files inside `Blender Sources/`. I did not open any other part of this project.

One thing I should state plainly rather than let stand behind that line: to check that the folder I was told to write into existed, I listed the file *names* in `Preparation/Blender Research/Findings/5.2/`. I opened none of them and read no content from any of them, including the earlier findings document I was told not to open. Nothing in this answer comes from anywhere but the sources named above.

Only question 19 was assigned to me. The rest of the sheet is untouched.

---

## 19. What decides whether two recorded library locations are treated as the same library or as two different libraries?

**How I read the question.** I take "recorded library location" to mean the path to an external `.blend` file that a blend-file stores when data is linked from it — the thing the Python API calls `Library.filepath`. The question then asks: if a file ends up holding two such paths, what makes Blender treat them as one library rather than two? The obvious hard case, and the one I hunted for, is two paths that are written differently but point at the same file on disk (one relative, one absolute; different capitalisation; one through a shortcut or a mapped drive).

**Ambiguity, flagged rather than resolved silently.** "Library" has a second meaning in Blender: an *asset library*, which is a folder or a web address registered in the Preferences and given a name. That is a different thing from a linked `.blend` file. I answer the linked-`.blend` reading, because this sheet is about what a `.blend` file points at outside itself, and asset libraries are registered in the application's Preferences rather than recorded in a `.blend`. I give what the sources say about the asset-library reading at the end of the answer, since under that reading the sources do answer.

### Label: **not in the sources**

Nothing in the 5.2 manual, the 5.2 Python API documentation, the 5.2 release notes, or Cycles' own files states the rule by which Blender decides that two recorded library paths are one library or two. There is no statement that the paths are compared as plain text, no statement that they are first turned into a single canonical form, no statement about capitalisation, and no statement about what happens when two differently written paths resolve to the same file on disk.

### What the sources *do* establish, and what each one does and does not carry

These are the pieces I could confirm. None of them adds up to the decision rule, and I am not going to pretend otherwise.

**(a) A library is a data-block, and it holds one path.** — *documented*

> "External .blend file from which data is linked"

> "filepath
> Path to the library .blend file (default "", never None, blend relative // prefix supported)"

Python API, `https://docs.blender.org/api/5.2/bpy.types.Library.html`, read 2026-07-30.

So each library present in a file is one `Library` data-block carrying one recorded path. That path may be written in the blend-relative `//` form or not.

**(b) A library reference *is* a blend-file path, in the documentation's own words.** — *documented*

> "Collection of source libraries, i.e. blendfile paths"

Python API, `https://docs.blender.org/api/5.2/bpy.types.BlendImportContextLibraries.html`, read 2026-07-30.

And, of a single one:

> "Library ID representing the blendfile from which the ID was imported. None until the ID has been linked or appended (readonly)"

Python API, `https://docs.blender.org/api/5.2/bpy.types.BlendImportContextItem.html`, read 2026-07-30.

This equates a library with a blend-file path, but it does not say how two paths are matched against each other.

**(c) Which library a data-block came from is part of that data-block's identity.** — *documented*

> "name
> Unique data-block ID name (within a same type and library) (default "", never None)"

> "name_full
> Unique data-block ID name, including library one if any (default "", readonly, never None)"

> "library
> Library file the data-block is linked from (readonly)"

Python API, `https://docs.blender.org/api/5.2/bpy.types.ID.html`, read 2026-07-30.

So names are unique only within a given library, and the full name of a data-block includes its library's name. Two libraries being treated as different therefore has real consequences: the same name in each is two different data-blocks. What decides it is still not said.

**(d) Library data-blocks live in the same namespace rules as everything else.** — *documented*

> "A data-block's name is unique within its namespace. A data-block namespace is defined by its type, and the blendfile it is stored in."

Manual, `https://docs.blender.org/manual/en/5.2/files/data_blocks.html`, read 2026-07-30.

The same page lists `Library` in its table of data-block types:

> "Library
> References to an external blend-file.
> Access from the Outliner's Blender File view."

Same page and date. So two `Library` data-blocks in one file cannot carry the same name. What the manual never says is what a library's *name* is — in particular, whether it is the recorded path. I looked for that statement and did not find it, so I will not assert it.

**(e) The Outliner groups linked data by the originating file's path.** — *documented*

> "Linked data-blocks are indicated with a chain icon in the Outliner.
> They're also listed in the Outliner's Blender File Display Mode,
> along with the path of the blend-file they originate from."

Manual, `https://docs.blender.org/manual/en/5.2/files/linked_libraries/link_append.html`, read 2026-07-30.

This tells you where you would *see* whether Blender ended up with one library or two. It does not tell you what decides it.

**(f) A recorded path can be rewritten in place, in bulk, without anything being said about merging.** — *documented*

> "Make Paths Relative
> Make all paths to external files relative to the current blend-file.
>
> Make Paths Absolute
> Make all paths to external files absolute (= full path from the system's root)."

Manual, `https://docs.blender.org/manual/en/5.2/interface/window_system/topbar.html`, read 2026-07-30.

And on saving elsewhere:

> "Remap Relative
>
> This option remaps Relative Paths
> (such as linked libraries and images) when saving a file in a new location."

Manual, `https://docs.blender.org/manual/en/5.2/files/blend/open_save.html`, read 2026-07-30.

These operations can plainly turn two differently written paths into the same written path. Whether the two libraries then become one, stay two, or collide over a name, is not stated anywhere I could find.

**(g) The relative form is resolved against the file the path was recorded in, not against the open file.** — *documented*

> "library (bpy.types.Library | None) – The library this path is from. This is only included for
> convenience, when the library is not None its path replaces start."

Python API, `https://docs.blender.org/api/5.2/bpy.path.html`, read 2026-07-30.

> "When using Blender data from linked libraries there is an unfortunate complication
> since the path will be relative to the library rather than the open blend-file."

Python API, `https://docs.blender.org/api/5.2/info_gotchas_file_paths_and_encoding.html`, read 2026-07-30.

This matters to the question because the same written text (`//assets/chair.blend`) recorded in two different files means two different files on disk. It establishes that a recorded path cannot be judged on its own text without knowing where it was recorded. It still does not say how Blender compares two of them.

**(h) A `check_existing` parameter is named on the movie-clip loader, and nothing of the kind is named on the library loader.** — *documented*, and the absence is what I am reporting

*(Changed by the citation check, 2026-08-09: this used to read "There is a documented 'is this the same file already' test for images and movie clips". Only the movie-clip page is quoted, so the images half has been cut for want of anything behind it. Note also that the words quoted say `check_existing` is **disabled** here; they show such a parameter exists on the load functions, not that a same-file test is in force.)*

> "Add a new movie clip to the main database from a file (while check_existing is disabled for consistency with other load functions, behavior with multiple movie-clips using the same file may incorrectly generate proxies)"

Python API, `https://docs.blender.org/api/5.2/bpy.types.BlendDataMovieClips.html`, read 2026-07-30.

The library loader, `bpy.data.libraries.load(...)`, has no such parameter — its full signature is documented at `https://docs.blender.org/api/5.2/bpy.types.BlendDataLibraries.html` (read 2026-07-30) and offers `filepath`, `link`, `pack`, `relative`, `set_fake`, `recursive`, `reuse_local_id`, `assets_only`, `clear_asset_data`, `create_liboverrides`, `reuse_liboverrides` and `create_liboverrides_runtime`, and nothing that names or controls matching against an already-loaded library.

**(i) The nearest documented matching rule is about data-blocks, not libraries.** — *documented*

> "library_weak_reference
> Weak reference to a data-block in another library .blend file (used to re-use already appended data instead of appending new copies) (readonly)"

Python API, `https://docs.blender.org/api/5.2/bpy.types.ID.html`, read 2026-07-30.

That reference carries exactly two fields:

> "filepath
> Path to the library .blend file (default "", never None, blend relative // prefix supported)"
>
> "id_name
> Full ID name in the library .blend file (including the two leading 'id type' chars) (default "", never None)"

Python API, `https://docs.blender.org/api/5.2/bpy.types.LibraryWeakReference.html`, read 2026-07-30.

So when Blender re-uses a previously appended data-block instead of appending a fresh copy, the two things it has recorded to recognise it by are the library file's path and the data-block's name. The documentation stops short of saying how the path is compared, and this is about appended data-blocks, not about whether two libraries are one.

**(j) Cycles keys "which library" off the library data-block, never off the path.** — *observed*

Cycles' own version-update pass, when it needs to do different work for data coming from different libraries, builds its groups out of the `Library` objects themselves and then tests membership by object, not by comparing any path text:

```
    # Map of versions used by libraries.
    library_versions = {}
    library_versions[bpy.data.version] = [None]
    for library in bpy.data.libraries:
        library_versions.setdefault(library.version, []).append(library)

    # Do versioning per library, since they might have different versions.
    max_need_versioning = (5, 2, 8)
    for version, libraries in library_versions.items():
        if version > max_need_versioning:
            continue

        # Scenes
        for scene in bpy.data.scenes:
            if scene.library not in libraries:
                continue
```

Cycles' own files, `cycles_5.2/version_update.py`, lines 97–112. Read 2026-07-30.

The step from that to any claim about behaviour would be *inferred*, so I will write the step out and label it as such: Cycles asks "which library is this scene from?" by comparing library objects, which means Cycles inherits whatever grouping Blender itself already arrived at and neither reinforces nor questions it. That is an **inferred** reading of the file, and it is as far as the file can be taken. It tells us nothing about how Blender arrived at that grouping — which is the question.

**(k) Nothing in the 5.2 release notes changes or explains this.** — *documented*

The only 5.2 release note touching library paths is:

> "Some limited library info is now exposed to the `bpy.data.libraries.load()` [input context manager](https://docs.blender.org/api/5.2/bpy.types.BlendDataLibraries.html#bpy.types.BlendDataLibraries.load), giving in particular access to the blend-file paths of the libraries used by the loaded one"

Release notes, `https://developer.blender.org/docs/release_notes/5.2/python_api/`, read 2026-07-30. The worked example printed on that same page lists a loaded file's libraries as a plain list of paths, several relative and one absolute-in-form, with no comment on how any of them are matched.

The Core section for 5.2 records only two library-related items, neither about identity:

> "## Library Override
> * A new operation has been added to the Outliner contextual menu (in Library Overrides view), to allow easy deletion of selected override rules…"

Release notes, `https://developer.blender.org/docs/release_notes/5.2/core/`, read 2026-07-30.

### The second reading, for completeness: asset libraries

If "recorded library location" is read as an *asset* library registered in the Preferences, the sources say what identifies one inside Blender — its name — but they do not say what makes two registered entries one library or two. — *documented*

*(Changed by the citation check, 2026-08-09: this used to read "the sources do answer, and the answer is that the name decides". The quote says the name is used to identify an asset library; it does not say the name is what decides whether two entries are the same one — which this answer itself concedes two paragraphs below.)*

> "The library name is only used within Blender to identify the asset library
> and can be changed at any time."

Manual, `https://docs.blender.org/manual/en/5.2/editors/preferences/asset_libraries.html`, read 2026-07-30.

> "Registering it means that you give the library a name (like "Sprite Fright")
> and the location on drive (like /home/user/projects/sprite-fright/assets)."

> "Unlike local asset libraries, online libraries are identified by a URL
> instead of a directory path and do not require manually managing asset files on disk."

Manual, `https://docs.blender.org/manual/en/5.2/files/asset_libraries/introduction.html`, read 2026-07-30.

So a registered asset library is a name plus a location, the name is what identifies it inside Blender, and an online one is identified by its web address instead of a folder. Whether two registered entries pointing at the same folder are treated as one is not stated. This is a different mechanism from the linked-`.blend` libraries the rest of this answer is about, and I am not carrying anything across between them.

### Where I looked

Searched across the whole of `manual_5.2`, `api_5.2`, `release_notes_5.2` and `cycles_5.2` unless noted.

*Terms searched, all case-insensitive:* "same library", "same initial library", "different librar", "two libraries", "duplicate librar", "distinct librar", "single library", "per library", "per-library", "library path", "library name", "library is identified", "libraries are", "name of the library", "already linked", "same file twice", "twice", "same path", "identical path", "unique path", "existing library", "reuse the library", "point to the same", "same file", "same blend-file", "same blend file", "treated as the same", "considered the same", "resolve to", "merge", "dedup", "de-duplicat", "identified by", "unique", "canonical", "normaliz", "case sensitive", "case-sensitive", "case-insensitive", "symlink", "symbolic link", "junction", "alias", "drive letter", "backslash", "trailing slash", "path separator", "network path", "relative path", "relocate", "linked librar", "library file", "linked blend-file", "indirectly linked", "indirect link", "is_library_indirect", "bpy.data.libraries", "libraries[".

*Pages I opened in full and ruled out:*

- Manual, Link & Append — `https://docs.blender.org/manual/en/5.2/files/linked_libraries/link_append.html`. The likeliest page by far. Covers Link, Append, Reload Library, Relocate Library, Relocate Linked ID, Make Local, Broken Libraries and Known Limitations. Says a library can be replaced by a different file and that a linked ID can be relocated "from the same library or from a different one", but never says what makes two recorded locations the same library. Ruled out.
- Manual, Linked Libraries contents page — `https://docs.blender.org/manual/en/5.2/files/linked_libraries/index.html`. Lists only the two pages above and the overrides page. Ruled out.
- Manual, Library Overrides — `https://docs.blender.org/manual/en/5.2/files/linked_libraries/library_overrides.html`. Read in full. About overriding and resyncing linked data; discusses hierarchies and identity of *overrides*, not of libraries. Ruled out.
- Manual, Data-Blocks — `https://docs.blender.org/manual/en/5.2/files/data_blocks.html`. Read in full, including the naming and namespace rules and the data-block type table. Establishes (d) above; carries no rule for libraries specifically. I also opened its one picture, `_images/files_data-blocks_outliner-blender-file-view.png`, read 2026-07-30. **Documented, from a picture.** Visible in it: the Outliner's display-mode menu (Scenes, View Layer, Sequence, Blender File highlighted, Data API, Orphan Data) and the Blender File view listing only "Current File" and its data-block categories. No library row is visible in it. **Inferred** from that absence: it settles nothing here. *(Changed by the citation check, 2026-08-09: labels added and the display-mode menu named, as the Method now asks for pictures.)*
- Manual, Save & Load / Relative Paths — `https://docs.blender.org/manual/en/5.2/files/blend/open_save.html`. Read in full. Explains the `//` prefix, Remap Relative, and that relative paths are the default. No comparison rule.
- Manual, Packed Data — `https://docs.blender.org/manual/en/5.2/files/blend/packed_data.html`. Read in full, on the chance that packing linked libraries forced the question. Ruled out.
- Manual, Topbar (the File ▸ External Data menu) — `https://docs.blender.org/manual/en/5.2/interface/window_system/topbar.html`. Carries Make Paths Relative / Absolute; no merge rule.
- Manual, Outliner Interface — `https://docs.blender.org/manual/en/5.2/editors/outliner/interface.html`. Describes the Blender File display mode in two sentences and says nothing about libraries. Ruled out. I also checked the `_images` folder for any picture of an Outliner row showing a library and its path; the only library-named images are asset-browser preference screenshots, so there was none to open.
- Manual, File Browser — `https://docs.blender.org/manual/en/5.2/editors/file_browser.html`. Only points back at the Linked Libraries page. Ruled out.
- Manual, Preferences ▸ Save & Load — `https://docs.blender.org/manual/en/5.2/editors/preferences/save_load.html`. Holds the default for relative paths and the remark that it "will be ignored if a path is already set". I considered whether that bore on the question, decided it is about the default value of the File Browser option rather than about library identity, and did not build anything on it.
- Manual, Preferences ▸ Asset Libraries — `https://docs.blender.org/manual/en/5.2/editors/preferences/asset_libraries.html` and Manual, Asset Libraries introduction — `https://docs.blender.org/manual/en/5.2/files/asset_libraries/introduction.html`. These answer the *other* reading, quoted above.
- Manual, Asset Browser — `https://docs.blender.org/manual/en/5.2/editors/asset_browser.html`. Checked the Import Method section, which contrasts Link with Append ("Dragging a material into the scene three times will result in three independent copies"). That is about data-blocks, not about library identity, so I did not build on it.
- Manual, Select Linked ▸ Library — `https://docs.blender.org/manual/en/5.2/scene_layout/object/selecting.html`. "Selects all objects that are in the same Library." Uses the notion without defining it. Ruled out.
- Manual, Command Line Arguments — `https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html`. Searched for every library mention. Only `--disable-liboverride-auto-resync` and two unrelated hits ("libmv library", bundled Python libraries). Nothing about library identity or path handling from the command line. Ruled out.
- Manual, Troubleshooting ▸ Python — `https://docs.blender.org/manual/en/5.2/troubleshooting/python.html`. Matched "duplicate libraries" but means Python modules on the search path, not Blender libraries. Ruled out.
- Python API, `Library` — `https://docs.blender.org/api/5.2/bpy.types.Library.html`. Read in full, every property including `archive_libraries`, `archive_parent_library`, `is_archive`, `parent`, `version`, `packed_file`, `users_id` and `reload()`. No identity rule.
- Python API, `LibraryWeakReference` — `https://docs.blender.org/api/5.2/bpy.types.LibraryWeakReference.html`. Read in full; gives (i) above.
- Python API, `BlendDataLibraries` — `https://docs.blender.org/api/5.2/bpy.types.BlendDataLibraries.html`. Read in full including `load()`, `write()`, `remove()` and all worked examples. No matching parameter, no identity rule.
- Python API, `BlendData` — `https://docs.blender.org/api/5.2/bpy.types.BlendData.html`. Read the `libraries`, `file_path_foreach` and `file_path_map` entries. `file_path_map` has an `include_libraries` switch and returns "sets of file path strings", but that is the shape of a Python return value and I refuse to read a library-identity rule out of it.
- Python API, `ID` — `https://docs.blender.org/api/5.2/bpy.types.ID.html`. Read in full; gives (c) and (i).
- Python API, `BlendImportContext`, `BlendImportContextItem`, `BlendImportContextLibraries`, `BlendImportContextLibrary` — the four pages at `https://docs.blender.org/api/5.2/bpy.types.BlendImportContext.html` and its siblings. Read in full. These are the newest and most detailed description of what happens during a link or append, listing every option including `APPEND_LOCAL_ID_REUSE` and the note that "All IDs (including indirectly linked ones) coming from the same initial library are always made local". They use the notion of "the same library" repeatedly and define it nowhere.
- Python API, `bpy.path` — `https://docs.blender.org/api/5.2/bpy.path.html`. Read in full, on the theory that a documented path-comparison helper would be the giveaway. There is `abspath`, `relpath`, `is_subdir`, `native_pathsep`, `reduce_dirs` ("remove duplicates and any directories nested in one of the other paths") and `resolve_ncase` ("Resolve a case insensitive path on a case sensitive system"). All are helpers offered to script authors. None is documented as being what Blender itself uses to decide library identity, and I will not claim it is.
- Python API, gotchas on file paths and encoding — `https://docs.blender.org/api/5.2/info_gotchas_file_paths_and_encoding.html`. Read in full; gives (g).
- Python API, `bpy.ops.wm` — `https://docs.blender.org/api/5.2/bpy.ops.wm.html`. Read the entries for `link`, `append`, `lib_reload`, `lib_relocate` and `id_linked_relocate` in full. `lib_reload` and `lib_relocate` both take a `library` string ("Library to relocate"), which shows a library is addressed by a name, but the documentation never says what that name is. `lib_relocate` is described as relocating "the given library to one or several others", which is the only hint anywhere that library count can change under an operation — and it says nothing about the reverse.
- Python API, `bpy.ops.outliner` — `https://docs.blender.org/api/5.2/bpy.ops.outliner.html`. `lib_operation` is marked "Undocumented, consider contributing." Its three modes are Delete, Relocate ("Select a new path for this library, and reload all its data") and Reload. Ruled out.
- Python API, `bpy.ops.file` — `https://docs.blender.org/api/5.2/bpy.ops.file.html`. Read `find_missing_files`, `report_missing_files`, `make_paths_absolute`, `make_paths_relative`. All one-line descriptions; none says what happens if the rewrite makes two paths identical.
- Python API, File Path Foreach Flag Items — `https://docs.blender.org/api/5.2/bpy_types_enum_items/file_path_foreach_flag_items.html`. Read in full, since it is the one place that enumerates the kinds of recorded path Blender knows about. It distinguishes linked, packed, weak-reference, multi-file and token paths. No identity rule.
- Release notes 5.2: read `index.md`, `core.md`, `pipeline_io.md`, `assets.md` and `python_api.md` in full, and searched `bugfixes.md` for every library mention. The bug-fix list names "Crash when relocating a linked library" and "Collection will be added to scene twice if it is appended, deleted, then linked under certain conditions", both as one-line titles with no description of behaviour. Nothing states an identity rule.
- Cycles' own files: searched all of `cycles_5.2` for "librar". Twenty-eight hits, of which all but the ones quoted in (j) and two calls of the form `bpy.path.abspath(filepath, library=library)` in `osl.py` (lines 199 and 239) are the string "library" inside unrelated web addresses in source comments, or `OSL_LIBRARY_VERSION_CODE` version checks. Nothing bearing on library identity.

I did not need the web, and did not go there.

### Not covered

- This answer does not settle the actual behaviour. It says the documentation is silent, not that Blender has no rule — it certainly has one, it just is not written down in these four sources.
- It therefore does not settle the specific cases that matter most in practice: one relative and one absolute path resolving to the same file; two paths differing only in capitalisation; two paths reaching the same file through a shortcut, a symbolic link, a mapped drive letter or two different network mounts; the same file reached once directly and once as a dependency of another library.
- It does not settle what happens after a bulk Make Paths Relative / Make Paths Absolute, or after Remap Relative on Save As, if two recorded library paths become the same text.
- It does not settle whether relocating one library onto the path of another already-present library merges them, replaces one, or produces two entries.
- It does not settle whether a missing library — for which the manual says Blender "will create placeholder data-blocks to replace missing linked ones" — is matched to a found one by anything other than the recorded path.
- It says nothing about how packed linked libraries and their "archive libraries" are matched, beyond the bare property descriptions on the `Library` page.
- The asset-library reading is answered only as far as "the name identifies it"; it does not settle what happens if two registered entries share a location, or a name.

### Outside the interface

The sources do not say. I found nothing anywhere in the 5.2 manual, API documentation or release notes that distinguishes how library locations are matched when Blender is driven from outside its own window — from the command line, in background mode, or from a script — as against being clicked at by a person. What I can report is the shape of the surrounding facts, and it is thin:

- The mechanism itself is not described for the interface case either, so there is no stated rule for a driven Blender to differ from.
- Everything that records or rewrites a library path is reachable without the interface: `bpy.data.libraries.load()`, `bpy.ops.wm.link`, `bpy.ops.wm.append`, `bpy.ops.wm.lib_reload`, `bpy.ops.wm.lib_relocate`, `bpy.ops.file.make_paths_absolute` and `bpy.ops.file.make_paths_relative` are all documented Python entry points at `https://docs.blender.org/api/5.2/` (read 2026-07-30). Whether they take the same path through Blender's matching as the menu items do is not stated.
- The one documented command-line switch that touches libraries at all changes override resync, not library identity: "--disable-liboverride-auto-resync … Do not perform library override automatic resync when loading a new blend-file", manual, `https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html`, read 2026-07-30.
- One documented fact does bite harder outside the interface than inside it, and it is worth flagging because it is about identity in a real sense: a relative path recorded in a library is resolved against *that library*, not against the open file — "the path will be relative to the library rather than the open blend-file" (Python API, `https://docs.blender.org/api/5.2/info_gotchas_file_paths_and_encoding.html`, read 2026-07-30). A script that resolves recorded paths itself, without passing the library, will get different answers from the ones Blender uses. That is a trap for anyone driving Blender from outside, and it is documented. It is not, however, an answer to the question.

**Brought up to date by the hand tests, 2026-08-09.** Every test recorded in the *Observed* section below was run with Blender started with no window, and on the rented Linux machine there is no display server at all, so no window was possible there. All the results below therefore hold when Blender is driven from outside its own interface. What was **not** done is a comparison with the same steps carried out by clicking: neither machine ran a window this round, so nothing is said about whether the two differ. One thing that does turn on being driven from outside is written up under test 8 below — the relocate call returned `{'FINISHED'}` and did not move the library it named.

### Operating system

The sources do not say. Nothing in the 5.2 manual, API documentation or release notes states that library matching behaves differently on Windows, macOS or Linux, and nothing states that it behaves the same either.

What the sources do carry, which bears on the question without answering it:

- The API documentation acknowledges that case sensitivity in paths differs between systems, by offering a helper for exactly that: "Resolve a case insensitive path on a case sensitive system, returning a string with the path if found else return the original path" (`bpy.path.resolve_ncase`, `https://docs.blender.org/api/5.2/bpy.path.html`, read 2026-07-30). That this helper exists shows Blender knows the problem is real. It does not say that library matching uses it.
- The documentation likewise acknowledges that path separators differ between systems: "Replace the path separator with the system's native os.sep" (`bpy.path.native_pathsep`, same page and date), and `bpy.path.basename` is described as "Equivalent to os.path.basename, but skips a "//" prefix. Use for Windows compatibility." Again, this is about helpers offered to script authors, not about the matching rule.
- The documentation states that recorded paths are not guaranteed to be valid UTF-8, because file systems differ: "Paths are an exception to this rule since the existence of non-UTF-8 paths on the user's file system cannot be ignored" (`https://docs.blender.org/api/5.2/info_gotchas_file_paths_and_encoding.html`, read 2026-07-30). Two recorded library paths could therefore differ as byte sequences while naming the same file on disk, and the sources say nothing about how that is handled.

I found no mention anywhere of drive letters, mapped network drives, UNC paths, symbolic links, junctions or aliases in connection with libraries.

**Brought up to date by the hand tests, 2026-08-09.** Of the tests in the *Observed* section below, one came out differently on the two computers and the rest came out the same. The one that differs is capital and small letters, test 4: on the rented Linux machine the differently-spelled name is not a file and the link was refused, and on the Creator's Mac it is the same file and two library entries resulted. Windows was not tested at all, because no Windows machine has run it. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

---

## Observed — fifteen tests run by hand on 2026-08-09

**Label: observed.**

**On what kind of computer.** Two, and neither speaks for the other.

- **The rented Linux machine** — rented on Vast.ai, Ubuntu 22.04.5 LTS on kernel 6.8, one NVIDIA GeForce RTX 5060 with 8 GB on driver 580.95.05, twenty-three cores of an AMD EPYC 7K62, 49 GB of memory, and no display server at all. Blender 5.2.0 LTS, at `/opt/blender/5.2/blender`.
- **The Creator's Mac** — a MacBook Pro with an M1 Pro chip, macOS 26.5.2, Apple's own graphics. Blender 5.2.0 LTS, at `/Applications/Blender.app/Contents/MacOS/Blender`.

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

**The same fifteen tests were run in 4.5.12 LTS, 5.0.1 and 5.1.2 as well.** Everything below came out the same in all four versions except test 12.

### 1. The control: the count can say one and can say two

*Done to:* a folder holding `libA.blend` with objects `A1` and `A2`, `libB.blend` with object `B1`, and an empty `host.blend`. *Done:* link `A1` from `libA.blend`; link `A2` from the same file with the path written exactly the same way; link `B1` from `libB.blend`.

The rented Linux machine printed:

```
HT|after-A1|library_count|1
HT|after-A1|library|0|name='libA.blend'|filepath='/root/ht-deps/e1-5.2-ops/libA.blend'
HT|after-A2-same-spelling|library_count|1
HT|after-B1-other-file|library_count|2
HT|after-B1-other-file|library|1|name='libB.blend'|filepath='/root/ht-deps/e1-5.2-ops/libB.blend'
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
HT|paths|plain='/root/ht-deps/e3-5.2-data/lib.blend'|dotted='/root/ht-deps/e3-5.2-data/./lib.blend'|updown='/root/ht-deps/e3-5.2-data/sub/../lib.blend'
HT|after-plain|library_count|1
HT|after-dot-step|library_count|1
HT|after-up-down-step|library_count|1
HT|after-fresh-reopen|library|0|name='lib.blend'|filepath='/root/ht-deps/e3-5.2-data/lib.blend'
```

The Creator's Mac printed the same counts and the same single recorded text. The two agreed.

### 4. Capital and small letters — the one test where the two computers disagree

*Done:* link `A1` through `lib.blend`, then `A2` through `LIB.blend`. A shell check was run first on each machine, so that a refusal could be told from a machine on which the name means nothing.

**On the rented Linux machine** the shell printed `ls: cannot access '/root/ht-deps/e4-5.2-data/LIB.blend': No such file or directory`, and Blender printed:

```
HT|exists|lower=True|upper=False
HT|after-lower|library_count|1
HT|link-upper|call|raised|OSError('load: /root/ht-deps/e4-5.2-data/LIB.blend failed to open blend file')
HT|after-upper|library_count|1
```

and, through the operator rather than the Python call:

```
HT|link-upper|call|raised|RuntimeError("Error: '/root/ht-deps/e4-5.2-ops/LIB.blend/Object/A2': nothing indicated\n")
```

**On the Creator's Mac** the shell listed both names as one file of the same size, and Blender printed:

```
HT|exists|lower=True|upper=True
HT|after-lower|library_count|1
HT|after-upper|library_count|2
HT|after-upper|library|0|name='lib.blend'|filepath='…/e4-5.2-data/lib.blend'
HT|after-upper|library|1|name='LIB.blend'|filepath='…/e4-5.2-data/LIB.blend'
HT|after-fresh-reopen|library_count|2
```

**The two did not agree, and the two results are not merged.** On the rented Linux machine the differently-spelled name is not a file and the link was refused. On the Creator's Mac it is the same file and two library entries resulted.

### 5. Two names on disk for one file, and two ways into one folder

*Done to:* `lib.blend` with objects `A1` to `A5`, plus `alias.blend` (a symbolic link to it), `hard.blend` (a hard link to it), and a symbolic link to the whole folder. On the rented Linux machine there was also a second mount of the folder, made with `mount --bind`. *Done:* link one object through each route.

The rented Linux machine's shell control printed `1808618 /root/ht-deps/e5-5.2-data/lib.blend` and `1808618 /root/ht-deps/e5-5.2-data/hard.blend` — one and the same file — and `MOUNT OK`. Blender printed:

```
HT|after-real|library_count|1
HT|after-symlink-to-file|library_count|2
HT|after-hardlink|library_count|3
HT|after-symlink-to-folder|library_count|4
HT|after-second-mount|library_count|5
HT|after-fresh-reopen|library|0|name='alias.blend'|filepath='/root/ht-deps/e5-5.2-data/alias.blend'
HT|after-fresh-reopen|library|1|name='hard.blend'|filepath='/root/ht-deps/e5-5.2-data/hard.blend'
HT|after-fresh-reopen|library|2|name='lib.blend'|filepath='/root/ht-deps/e5-5.2-data/lib.blend'
HT|after-fresh-reopen|library|3|name='lib.blend.001'|filepath='/root/ht-deps/e5-5.2-data-otherway/lib.blend'
HT|after-fresh-reopen|library|4|name='lib.blend.002'|filepath='/root/ht-deps/e5-5.2-data-bind/lib.blend'
```

The Creator's Mac cannot make a second mount, so it ran four of the five steps and printed the same counts up to `HT|after-fresh-reopen|library_count|4`, with the same four entries. The two agreed on the four steps both could do. The second mount is a Linux result only.

### 6. Two entries made to hold the very same recorded text

*Done to:* `host.blend` holding two libraries linked from two different files. *Done:* write the first one's recorded text into the second from Python, then save and open again.

The rented Linux machine printed:

```
HT|assignment|returned normally
HT|after-assignment|library_count|2
HT|after-assignment|library|0|name='lib1.blend'|filepath='/root/ht-deps/e6-5.2-data/lib1.blend'
HT|after-assignment|library|1|name='lib2.blend'|filepath='/root/ht-deps/e6-5.2-data/lib1.blend'
HT|after-reopen-in-process|library_count|1
HT|after-reopen-in-process|library|0|name='lib1.blend'|filepath='/root/ht-deps/e6-5.2-data/lib1.blend'
HT|after-fresh-reopen|library_count|1
```

The Creator's Mac printed the same counts, `2, 2, 1, 1`. The two agreed.

### 7. Rewriting every path in the file at once

*Done to:* the file test 2 left, holding one entry. *Done:* `bpy.ops.file.make_paths_absolute()`, then save and reopen, then `bpy.ops.file.make_paths_relative()`, then save and reopen.

The rented Linux machine printed:

```
HT|reopened|library|0|name='lib.blend'|filepath='//lib.blend'
HT|make_paths_absolute|returned={'FINISHED'}
HT|after-make-absolute|library|0|name='lib.blend'|filepath='/root/ht-deps/e7-5.2-data/lib.blend'
HT|make_paths_relative|returned={'FINISHED'}
HT|after-make-relative|library|0|name='lib.blend'|filepath='//lib.blend'
HT|reopened-after-relative|library_count|1
```

The Creator's Mac printed the same. The two agreed.

### 8. Relocating one library onto the file another library already uses — the call did not act on the library it named

*Done to:* `host.blend` with `lib1.blend` and `lib2.blend` linked. *Done:* `bpy.ops.wm.lib_relocate(library='lib2.blend', directory=…, filename='lib1.blend')`.

The rented Linux machine printed:

```
HT|two-libraries|library|0|name='lib1.blend'|filepath='/root/ht-deps/e8-5.2-data/lib1.blend'
HT|two-libraries|library|1|name='lib2.blend'|filepath='/root/ht-deps/e8-5.2-data/lib2.blend'
HT|relocate|library='lib2.blend'|returned={'FINISHED'}
HT|after-relocate|library_count|2
HT|after-relocate|library|0|name='lib1.blend'|filepath='//lib1.blend'
HT|after-relocate|library|1|name='lib2.blend'|filepath='/root/ht-deps/e8-5.2-data/lib2.blend'
HT|after-relocate|object|name='Q1'|name_full='Q1 [lib2.blend]'|from='/root/ht-deps/e8-5.2-data/lib2.blend'
```

The Creator's Mac printed the same. The two agreed.

Two further tests were run because of that. With the same set-up and a third file that no library records, the same call naming `lib2.blend` and pointing at that third file returned `{'FINISHED'}` and changed nothing at all, both when two calls were made in one run and when a single call was made in its own run, on both computers:

```
HT|calling|bpy.ops.wm.lib_relocate(library='lib2.blend', filepath='/root/ht-deps/e15-5.2-lib2-lib3/lib3.blend', directory='/root/ht-deps/e15-5.2-lib2-lib3/', filename='lib3.blend')
HT|relocate|returned={'FINISHED'}
HT|after|library|0|name='lib1.blend'|filepath='/root/ht-deps/e15-5.2-lib2-lib3/lib1.blend'
HT|after|library|1|name='lib2.blend'|filepath='/root/ht-deps/e15-5.2-lib2-lib3/lib2.blend'
```

So, with Blender started with no window, this call returns `{'FINISHED'}` whatever it is given; nothing moves when the target is a file no library records; and when the target is a file an existing library records, that existing library's recorded text is rewritten into the `//` form while the library named in the call is left where it was. **What this therefore does not settle:** what happens when a library really is relocated onto a file another library already records. The named library never moved, so there was no collision to watch. That is written up as still open below.

### 9. The same file reached directly and through another library

*Done to:* `lib.blend` with objects `A1` and `A2`; `mid.blend`, which links `A1` from it and holds it in a local collection; and `host.blend`. *Done:* link the collection from `mid.blend`, then link `A2` straight out of `lib.blend`.

The rented Linux machine printed:

```
HT|after-indirect|library_count|2
HT|after-indirect|library|0|name='lib.blend'|filepath='/root/ht-deps/e9-5.2-data/lib.blend'
HT|after-indirect|library|1|name='mid.blend'|filepath='/root/ht-deps/e9-5.2-data/mid.blend'
HT|after-direct|library_count|2
HT|after-direct|object|name='A1'|name_full='A1 [lib.blend]'|from='/root/ht-deps/e9-5.2-data/lib.blend'
HT|after-direct|object|name='A2'|name_full='A2 [lib.blend]'|from='/root/ht-deps/e9-5.2-data/lib.blend'
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
HT|resolve|name='x.blend'|filepath='//x.blend'|parent=None|abspath='/root/ht-deps/e10-5.2-data/two/x.blend'
HT|resolve|name='x.blend.001'|filepath='//../one/x.blend'|parent='/root/ht-deps/e10-5.2-data/one/mid.blend'|abspath='/root/ht-deps/e10-5.2-data/one/../one/x.blend'
```

So the text that `mid.blend` holds as `//x.blend` appears in the host file as `//../one/x.blend`. The two identical texts the test was built to produce did not arise. The Creator's Mac printed the same. The two agreed.

### 11. Two different files that share a file name

*Done:* link an object from `p/lib.blend`, then one from `q/lib.blend`.

The rented Linux machine printed:

```
HT|after-p|library_count|1
HT|after-p|library|0|name='lib.blend'|filepath='/root/ht-deps/e11-5.2-data/p/lib.blend'
HT|after-q|library_count|2
HT|after-q|library|1|name='lib.blend.001'|filepath='/root/ht-deps/e11-5.2-data/q/lib.blend'
HT|after-fresh-reopen|library_count|2
```

The Creator's Mac printed the same. The two agreed. Together with test 1 this shows what this answer says above the sources never state: the printed `name` of a library data-block is the bare file name, and a numbered suffix is added when two would clash.

### 12. Packing linked data — the one place the versions differ

*Done:* ask whether the three archive properties exist, then link with packing asked for.

The rented Linux machine, Blender 5.2.0 LTS, printed:

```
HT|prop|is_archive|present=True
HT|prop|archive_parent_library|present=True
HT|prop|archive_libraries|present=True
HT|pack|returned normally
HT|after-pack|library_count|2
HT|after-pack|library|0|name='lib.blend'|filepath='/root/ht-deps/e12-5.2-data/lib.blend'
HT|after-pack|library|1|name='lib.blend.001'|filepath='/root/ht-deps/e12-5.2-data/lib.blend'
HT|after-pack|object|name='A1'|name_full='A1 [lib.blend.001]'|from='/root/ht-deps/e12-5.2-data/lib.blend'
HT|archive|name='lib.blend'|filepath='/root/ht-deps/e12-5.2-data/lib.blend'|is_archive=False|archive_parent=None|archives=['/root/ht-deps/e12-5.2-data/lib.blend']
HT|archive|name='lib.blend.001'|filepath='/root/ht-deps/e12-5.2-data/lib.blend'|is_archive=True|archive_parent='/root/ht-deps/e12-5.2-data/lib.blend'|archives=[]
```

So two library entries, holding **the same recorded text**, one with `is_archive=False` and one with `is_archive=True` whose archive parent is the first. The Creator's Mac printed the same two entries with the same two flags. The two agreed.

The same test in Blender 4.5.12 LTS printed `present=False` for all three properties and refused the call with `HT|pack|raised|TypeError("'pack' is an invalid keyword argument for load()")`, leaving no library at all. 5.0.1, 5.1.2 and 5.2.0 LTS all behaved as printed above.

### 13. Two files that record one library differently, both linked into a third

*Done to:* `lib.blend`; `a.blend`, which links from it with the relative option on; `b.blend`, which links from it with the option off; and an empty `c.blend`. *Done:* link a collection from `a.blend`, then one from `b.blend`.

The controls printed, when the two were built, `HT|built-a.blend|library|0|name='lib.blend'|filepath='//lib.blend'` and `HT|built-b.blend|library|0|name='lib.blend'|filepath='/root/ht-deps/e13-5.2-data/lib.blend'` — the two really did record the one library two different ways. The rented Linux machine then printed:

```
HT|after-a|library_count|2
HT|after-b|library_count|3
HT|after-b|library|0|name='a.blend'|filepath='/root/ht-deps/e13-5.2-data/a.blend'
HT|after-b|library|1|name='b.blend'|filepath='/root/ht-deps/e13-5.2-data/b.blend'
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

## Still open for a person with Blender 5.2 open

The researcher's own list has been pruned: everything the hand tests answered is gone. What is left is what a running Blender still has not settled.

1. **Relocating a library onto a file another library already records.** The tests reached this through `bpy.ops.wm.lib_relocate` in a Blender with no window, and the call never moved the library it was told to move. Doing it by right-clicking the entry in the Outliner's Blender File view, in a window, would settle it. Note first whether the count goes to one, then copy out anything Blender shows in the Info editor, word for word.
2. **What the Outliner's Blender File view shows for a file that holds two entries recording the same location.** The count and the recorded text are already known from a terminal; what is not known is how those two entries are labelled on screen when their recorded text is identical.
3. **Anything on Windows.** Untried — no Windows machine has run it — not answered.

---

2026-08-09 — Opus 5 (Claude Code) — Dependencies, Blender 5.2

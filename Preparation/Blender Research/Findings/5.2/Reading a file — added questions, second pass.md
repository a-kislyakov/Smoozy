# Reading a file — added questions, second pass (Blender 5.2)

**Sources consulted, and when.** Blender 5.2's manual, Blender 5.2's Python API documentation, Blender 5.2's release notes, and Cycles' own files from Blender 5.2 — all as supplied in `Blender Sources/`, all taken on **2026-07-30**, which is the date read for every citation below except one. Blender's requirements page as supplied, taken on **2026-08-01**; it is cited with that date where it is cited at all.

I read only the question sheet and files inside `Blender Sources/`. I did not open any other part of this project.

*One disclosure, so that line can be trusted at face value.* The project's `CLAUDE.md` was placed into my context automatically by the tool I run in, before I did anything. I did not open it, I did not follow it, and I did not open `Preparation/Groundwork/Start Here.md` or anything else it points to. Its contents are a signpost to other documents and contain no statement about how Blender behaves, so nothing in this document can have come from it. I also read `Blender Sources/About this folder.md`, which sits inside the one folder I was told I may read.

**Scope of this pass.** Question 9 only. Questions 1–8 on the sheet were not looked at and are not answered here.

---

## 9. What separate means exist for listing and rewriting the locations recorded in a `.blend` file, and which kinds of recorded location does each of those means reach?

**LABEL: documented**, with two parts marked **inferred** where I say so inside the answer.

### Short answer

Blender 5.2 documents several distinct means. They are not one feature with several front doors — they have different reach, and some only list while others also rewrite. In rough order of reach:

| Means | Lists | Rewrites | What it reaches |
|---|---|---|---|
| `bpy.data.file_path_foreach` | yes | yes | paths used by every data-block in the open file, filterable by data-block type; switches for linked, packed, weak-reference, multi-file, UDIM tiles, sequences, cache files |
| `bpy.data.file_path_map` | yes | no | paths used by every data-block in the open file; library paths of linked data excluded unless asked for |
| `bpy.utils.blend_paths` | yes | no | "external files referenced by the loaded .blend file"; switches for absolute, packed, linked-library |
| The individual path settings themselves (e.g. an image's, a library's, the render output's) | yes | yes | exactly the one thing that owns the setting |
| `File ▸ External Data ▸ Make Paths Relative / Absolute` (`bpy.ops.file.make_paths_relative` / `make_paths_absolute`) | no | yes, in bulk | "all paths to external files" |
| `File ▸ External Data ▸ Report Missing Files / Find Missing Files` | reports | Find rewrites | unpacked external files that no longer exist |
| Outliner ▸ Relocate (`bpy.ops.wm.lib_relocate`, `lib_reload`) | no | yes | linked library files, and single linked data-blocks |
| Saving elsewhere with "Remap Relative" (`wm.save_as_mainfile`, `wm.save_mainfile`) | no | yes, on save | relative paths |
| `bpy.data.libraries.write(path_remap=…)` | no | yes, on write | all paths in the newly written file |
| `bpy.data.libraries.load(…)`, input side | yes | no | the library blend-file paths of a file that is **not** the open one |
| `--render-output` on the command line | no | yes, for the session | the render output path only |
| RNA property flags (`is_path_output`, `is_path_supports_blend_relative`, `subtype`) | tells you which settings *are* paths | no | any property, not the values |

Each is set out below with the words that carry it.

---

### A. The path iterator — `bpy.data.file_path_foreach`

This is the one means documented to both list and rewrite in the same pass.

> `file_path_foreach(visit_path_fn, *, subset=None, visit_types=None, flags={'SKIP_PACKED', 'SKIP_WEAK_REFERENCES'})`
>
> Call visit_path_fn for the file paths used by all ID data-blocks in current bpy.data.
>
> For list of valid set members for visit_types, see: bpy.types.KeyingSetPath.id_type.

and, on the rewriting:

> visit_path_fn (…) – function that takes three parameters: the data-block, a file path, and a bpy.types.BlendDataPathMeta metadata object. The function should return either None or a str. In the latter case, the visited file path will be replaced with the returned string.

> subset (set[str] | None) – When given, only these data-blocks and their used file paths will be visited.
>
> visit_types (set[str] | None) – When given, only visit data-blocks of these types. Ignored if subset is also given.

Source: Blender's Python API documentation, `https://docs.blender.org/api/5.2/bpy.types.BlendData.html`. Date read: 2026-07-30.

**What it reaches, and what it deliberately leaves out.** The switches are named and described:

> SKIP_LINKED: … Skip paths of linked IDs.
>
> SKIP_PACKED: … Skip paths when their matching data is packed.
>
> SKIP_WEAK_REFERENCES: … Skip weak reference paths. Those paths are typically 'nice to have' extra information, but are not used as actual source of data by the current .blend file.
>
> SKIP_MULTIFILE: … Skip paths where a single dir is used with an array of files, eg. sequence strip images or point-caches. In this case only the first file path is processed. This is needed for directory manipulation callbacks which might otherwise modify the same directory multiple times.
>
> RELOAD_EDITED: … Reload data when the path is edited.
>
> RESOLVE_TOKEN: … Resolve tokens within a virtual filepath to a single, concrete, filepath. Currently only used for UDIM tiles.
>
> EXPAND_TOKENS: … Expand template tokens in virtual file paths and invoke the callback once for each concrete file. Currently only used for UDIM tiles.
>
> EXPAND_SEQUENCES: … Expand image and volume sequences, invoking the callback once per file on disk.
>
> EXPAND_CACHES: … Visit cache files, like texture cache files associated with images.

Source: Blender's Python API documentation, `https://docs.blender.org/api/5.2/bpy_types_enum_items/file_path_foreach_flag_items.html`. Date read: 2026-07-30.

Note the two defaults: packed data and weak references are skipped unless you ask otherwise, and linked data-blocks' paths **are** visited unless you ask to skip them.

**Not every path it reaches can be rewritten.** The metadata object says which:

> is_cache — True when the path is a cache file, like the image texture cache. These paths can not be edited.
>
> is_expanded — True when the path was expanded from a UDIM tile or sequence frame. These paths can not be edited.
>
> is_readonly — True when the path is read-only and can not be edited.

Source: Blender's Python API documentation, `https://docs.blender.org/api/5.2/bpy.types.BlendDataPathMeta.html`. Date read: 2026-07-30.

**What 5.2 changed here.** The release notes state it outright:

> Three new options for `bpy.data.file_path_foreach` so render farms and other scripts can find all relevant image files for rendering, including the new Cycles texture cache. (blender/blender!159159)
>
> - `EXPAND_TOKENS`: visit each UDIM tile path.
> - `EXPAND_SEQUENCES`: visit each image and volume sequence file path.
> - `EXPAND_CACHES`: visit cache files, like texture cache files associated with images.
>
> The previously reserved metadata field is now provided to callers, with `is_expanded` and `is_cache` booleans to indicate the file kind. Additionally `is_readonly` indicates if the path can be edited, which is not possible for expanded paths.

Source: Blender's release notes, `https://developer.blender.org/docs/release_notes/5.2/python_api/`. Date read: 2026-07-30.

---

### B. The path map — `bpy.data.file_path_map`

Listing only. No rewriting is described.

> `file_path_map(*, subset=None, key_types=None, include_libraries=False)`
>
> Returns a mapping of all ID data-blocks in current bpy.data to a set of all file paths used by them.
>
> …
>
> include_libraries (bool) – Include library file paths of linked data. False by default.
>
> Returns: dictionary of bpy.types.ID instances, with sets of file path strings as their values.

Source: Blender's Python API documentation, `https://docs.blender.org/api/5.2/bpy.types.BlendData.html`. Date read: 2026-07-30.

The difference in reach from A worth naming: this one gives you paths grouped by the data-block that uses them, and it leaves out the library files themselves unless you ask.

---

### C. The flat path list — `bpy.utils.blend_paths`

Listing only.

> `bpy.utils.blend_paths(*, absolute=False, packed=False, local=False)`
>
> Returns a list of paths to external files referenced by the loaded .blend file.
>
> Parameters:
> absolute (bool) – When true the paths returned are made absolute.
> packed (bool) – When true include file paths for packed data.
> local (bool) – When true skip linked library paths.
>
> Returns: path list.

Source: Blender's Python API documentation, `https://docs.blender.org/api/5.2/bpy.utils.html`. Date read: 2026-07-30.

Its reach is stated only as "external files referenced by the loaded .blend file". The one thing it names explicitly is that linked library paths are in the list unless you switch them off, and packed data's paths are out unless you switch them on.

---

### D. The individual path settings

Every recorded location also sits on the thing that owns it, and each one can be read and written on its own. These are the kinds of recorded location the API documentation names, with the exact description each carries. Source for all of them: Blender's Python API documentation, page `https://docs.blender.org/api/5.2/bpy.types.<Name>.html` for the named type. Date read: 2026-07-30.

- `Image.filepath` — "Image/Movie file name (default "", never None, blend relative // prefix supported)"
- `Library.filepath` — "Path to the library .blend file (…)"
- `Sound.filepath` — "Sound sample file used by this Sound data-block (…)"
- `VectorFont.filepath` — "(default "", never None, blend relative // prefix supported)"
- `MovieClip.filepath` — "Filename of the movie or sequence file (…)"
- `CacheFile.filepath` — "Path to external displacements file (…)"
- `Text.filepath` — "Filename of the text file (default "", never None)" — note this one does **not** say the `//` prefix is supported
- `Volume.filepath` — "Volume file used by this Volume data-block (…)"
- `RenderSettings.filepath` (reached as `Scene.render`) — "Directory/name to save animations, # characters define the position and padding of frame numbers (default "//", never None, blend relative // prefix supported, Supports template expressions)"
- `PointCache.filepath` and `PointCacheItem.filepath` — "Cache file path (…)"
- `BakeSettings.filepath` — "Image filepath to use when saving externally (…)"
- `StripProxy.filepath` — "Location of custom proxy file (…)"
- `MovieStrip.filepath`
- `ImagePackedFile.filepath`
- `LibraryWeakReference.filepath` — "Path to the library .blend file (…)"
- `BlendImportContextLibrary.filepath` — "(default "", readonly, never None)", and the type itself is described as "Library (blendfile) reference in a BlendImportContext data. Currently only exposed as read-only data for the pre/post blendimport handlers"
- and on modifiers and nodes: `MeshCacheModifier.filepath`, `MultiresModifier.filepath`, `OceanModifier.filepath`, `ShaderNodeScript.filepath`, `ShaderNodeTexIES.filepath`, `TextureNodeOutput.filepath`, `CacheFileLayer.filepath`, `CollectionExport.filepath`, `Theme.filepath`

That the above is the **complete** list of kinds is **inferred**, not documented: I obtained it by searching every `bpy.types.*` page in the 5.2 API documentation for a property named exactly `filepath`. The step a reader must supply is that Blender records no external location under some other property name. I did find at least one such: `Image.filepath_raw` and `Image.filepath_from_user()` also exist, and the render output has a matching manual page under File Path:

> Choose the location to save rendered frames. … This setting expands Relative Paths where a // prefix represents the directory of the current blend-file.

Source: Blender's manual, `https://docs.blender.org/manual/en/5.2/render/output/properties/output.html`. Date read: 2026-07-30.

Reach of this means: exactly one location per setting. Nothing enumerates them for you — you have to know which data-block to ask.

---

### E. The bulk relative/absolute rewrite

Rewriting only, in bulk, with no listing.

> `bpy.ops.file.make_paths_absolute()` — Make all paths to external files absolute
>
> `bpy.ops.file.make_paths_relative()` — Make all paths to external files relative to current .blend

Source: Blender's Python API documentation, `https://docs.blender.org/api/5.2/bpy.ops.file.html`. Date read: 2026-07-30.

The manual describes the same two as menu entries:

> Make Paths Relative — Make all paths to external files relative to the current blend-file.
>
> Make Paths Absolute — Make all paths to external files absolute (= full path from the system's root).

Source: Blender's manual, `https://docs.blender.org/manual/en/5.2/interface/window_system/topbar.html`. Date read: 2026-07-30.

Reach is stated only as "all paths to external files". No list of which kinds that covers is given anywhere I searched.

---

### F. Reporting and repairing missing files

> `bpy.ops.file.report_missing_files()` — Report all missing external files
>
> `bpy.ops.file.find_missing_files(*, find_all=False, directory='', …)` — Try to find missing external files
>
> find_all (bool) – Find All, Find all files in the search path (not just missing) (optional)
>
> directory (str) – Directory, Directory of the file (optional, never None)

Source: Blender's Python API documentation, `https://docs.blender.org/api/5.2/bpy.ops.file.html`. Date read: 2026-07-30.

The manual says what each does, and — important for this question — says what kind of rewriting Find Missing Files performs:

> Report Missing Files — This option is useful to check if there are links to unpacked files that no longer exist. After selecting this option, a warning message will appear in the Info editor's header. If no warning is shown, there are no missing external files.
>
> Find Missing Files — In case you have broken links in a blend-file, this can help you to fix the problem. A File Browser will show up. Select the desired directory (or a file within that directory), and a search will be performed in it, recursively in all contained directories. Every missing file found in the search will be recovered. Those recoveries will be done as absolute paths, so if you want to have relative paths you will need to select Make Paths Relative.

Source: Blender's manual, `https://docs.blender.org/manual/en/5.2/interface/window_system/topbar.html`. Date read: 2026-07-30.

Reach: "unpacked files", i.e. external files that are not packed into the blend-file. Whether that includes linked libraries is not stated on this page.

---

### G. Library paths specifically — Relocate and Reload

Rewriting only, and only for library files (and single linked data-blocks).

> `bpy.ops.wm.lib_relocate(*, library='', filepath='', directory='', filename='', files=None, …, relative_path=True, …)`
>
> `bpy.ops.wm.lib_reload(*, library='', filepath='', directory='', filename='', …, relative_path=True, …)`

Source: Blender's Python API documentation, `https://docs.blender.org/api/5.2/bpy.ops.wm.html`. Date read: 2026-07-30.

The manual describes the same from the interface:

> When the Outliner is in the Blender File Display Mode, a right-click on a library (a linked blend-file) will show the Relocate option. It allows to replace it by a different file. This can be used to either fix a broken linked library (e.g. because the file was moved or renamed), or to switch to a variation of the same data in a different file.

and, for a single linked data-block:

> A right-click a linked ID will show the Relocate option in the ID Data sub-menu. It allows to relocate a directly linked ID by another one, from the same library or from a different one. … This operation is only available on data-blocks that are directly linked, and are not a dependency of any other linked data.

Source: Blender's manual, `https://docs.blender.org/manual/en/5.2/files/linked_libraries/link_append.html`. Date read: 2026-07-30.

The library itself also carries a direct setting and a reload action: `Library.filepath` — "Path to the library .blend file" — and `Library.reload()`. Source: `https://docs.blender.org/api/5.2/bpy.types.Library.html`. Date read: 2026-07-30.

The Outliner's Blender File mode is the listing side of this:

> Blender File: Lists all data in the current blend-file.

Source: Blender's manual, `https://docs.blender.org/manual/en/5.2/editors/outliner/interface.html`. Date read: 2026-07-30.

---

### H. Rewriting at the moment of saving or writing

Two documented ways, both of which rewrite as a side effect of producing a file.

Saving the open file somewhere else:

> relative_remap (bool) – Remap Relative, Remap relative paths when saving to a different directory (optional)

on both `bpy.ops.wm.save_as_mainfile` (where it defaults to on) and `bpy.ops.wm.save_mainfile` (where it defaults to off). Source: `https://docs.blender.org/api/5.2/bpy.ops.wm.html`. Date read: 2026-07-30.

The manual's version:

> Remap Relative — This option remaps Relative Paths (such as linked libraries and images) when saving a file in a new location.

Source: Blender's manual, `https://docs.blender.org/manual/en/5.2/files/blend/open_save.html`. Date read: 2026-07-30.

Writing a new blend-file out of chosen data-blocks:

> `write(filepath, datablocks, *, path_remap='NONE', fake_user=False, compress=False)` — Write data-blocks into a blend file.
>
> path_remap (str) – Optionally remap paths when writing the file:
> NONE No path manipulation (default).
> RELATIVE Remap paths that are already relative to the new location.
> RELATIVE_ALL Remap all paths to be relative to the new location.
> ABSOLUTE Make all paths absolute on writing.

Source: Blender's Python API documentation, `https://docs.blender.org/api/5.2/bpy.types.BlendDataLibraries.html`. Date read: 2026-07-30.

This last one is the only documented means whose reach is spelled out as four distinct rewriting behaviours rather than two.

---

### I. Listing the library paths of a file you have not opened

New wording in 5.2. The `load` context manager exposes, on its input side, the libraries that the *loaded* file uses:

> A `libraries` attribute, which for the input is a list of all other libraries used by the loaded one, as named tuples (filepath, is_archive). The filepath may be absolute, or relative to the loaded blend-file. The output libraries attribute is always None.

and a stated limit on its reach:

> Packed linked data-blocks are not linkable and are not listed in the input libraries attribute.

Source: Blender's Python API documentation, `https://docs.blender.org/api/5.2/bpy.types.BlendDataLibraries.html`. Date read: 2026-07-30.

The release notes name this as a 5.2 addition and show it in use:

> Some limited library info is now exposed to the `bpy.data.libraries.load()` input context manager, giving in particular access to the blend-file paths of the libraries used by the loaded one (blender/blender!157066, blender/blender@646199c89a).

with the worked example:

> `bpy_lib.libraries(filepath='//../../../assets/nodes/star_backdrop.blend', is_archive=False)`

Source: Blender's release notes, `https://developer.blender.org/docs/release_notes/5.2/python_api/`. Date read: 2026-07-30.

Reach: library blend-file paths only. Not images, not sounds, not caches. Listing only — it is described as read-only ("The input contains the data-blocks available in the loaded blend-file library and should be treated as read-only").

---

### J. From the command line

One documented command-line means that rewrites a recorded location:

> -o, --render-output <path>
>
> Set the render path and file name. Use // at the start of the path to render relative to the blend-file.
>
> You can use path templating features such as {blend_name} in the path.

Source: Blender's manual, `https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html`. Date read: 2026-07-30.

Reach: the render output path, and nothing else. I found no command-line argument in the 5.2 list that lists or rewrites any other kind of recorded location — see the record of looking below.

The other command-line entries relevant here are the vehicles for running the Python means above from outside: `-P, --python <filepath>` ("Run the given Python script file"), `--python-expr <expression>` ("Run the given expression as a Python script"), `--python-text <name>`, and `-c, --command <command>` ("Run a command which consumes all remaining arguments. … This implies --background mode"). Same page, same date.

---

### K. Finding out which settings *are* recorded locations

Not a means of listing values, but a documented means of telling a path setting from any other setting:

> is_path_output — Property is a filename, filepath or directory output (default False, readonly)
>
> is_path_supports_blend_relative — Property is a path which supports the "//" prefix, signifying the location as relative to the ".blend" file's directory (default False, readonly)
>
> is_path_supports_templates — Property is a path which supports the "{variable_name}" variable expression syntax, which substitutes the value of the referenced variable in place of the expression (default False, readonly)

Source: Blender's Python API documentation, `https://docs.blender.org/api/5.2/bpy.types.Property.html`. Date read: 2026-07-30.

Properties also carry a `subtype` — "Semantic interpretation of the property" — whose documented values include `FILE_PATH` ("File Path"), `DIR_PATH` ("Directory Path") and `FILE_NAME` ("File Name"). Source: `https://docs.blender.org/api/5.2/bpy_types_enum_items/property_subtype_items.html`. Date read: 2026-07-30.

That these flags could be walked to build your own list of every recorded location is **inferred**, not documented. The sources describe what each flag means; they never say to use them that way, and they do not say the flags are set consistently on every path setting.

---

### L. Outside Blender altogether

The manual acknowledges, in one sentence, that tooling exists outside Blender that edits blend-files:

> Since there is already tooling that can manipulate blend-files outside of Blender itself (see Blender Asset Tracer), it's possible to also create an external tool for doing such edits across blend-files.

It also states the rule that makes such a tool necessary:

> As described above, Blender itself is not allowed to write to other blend-files than the currently open one.

Source: Blender's manual, `https://docs.blender.org/manual/en/5.2/files/asset_libraries/introduction.html`. Date read: 2026-07-30.

That is the whole of what the 5.2 sources say about it. They do not say what Blender Asset Tracer reaches, how it is obtained, or whether it works with 5.2 files. Treat this as a pointer, not a documented capability.

---

### NOT COVERED

- **No source lists which kinds of recorded location the bulk operators actually touch.** "All paths to external files" is the only wording given for Make Paths Relative and Make Paths Absolute. Whether that includes point caches, render output, proxies, text files, or the library files themselves is nowhere stated.
- **Same gap for the path iterator and the path map.** "File paths used by all ID data-blocks" tells you the filtering is by data-block, not which settings inside a data-block are visited. In particular I cannot say from the sources whether the render output path (which lives on the Scene data-block) is visited.
- **`bpy.utils.blend_paths` and `file_path_map` are never compared.** Both list, both take options, and no source says whether they see the same set.
- **Nothing says whether a rewrite through any of these means is persisted.** All of them act on data in memory; the sources do not say the blend-file on disk changes without a save. The two exceptions are the save/write means in section H, which by their nature write a file.
- **Order and interaction are undocumented.** For example, Find Missing Files is documented to write absolute paths and the manual tells you to run Make Paths Relative afterwards — but nothing says what happens if two means are used at once, or during a render.
- **The template syntax (`{blend_name}` and so on) is a further wrinkle nobody's reach is described against.** The manual says templates are substituted "internally at time of use (e.g. while rendering), so the template syntax will remain as-is in the path field" (`https://docs.blender.org/manual/en/5.2/files/file_paths.html`, read 2026-07-30). Whether a listing means reports the written form or the substituted form is not stated for any of them, except that `RESOLVE_TOKEN` and `EXPAND_TOKENS` on the path iterator are documented for UDIM tiles only.
- **This answer does not settle whether any of these means can be *made* to reach a kind it does not reach by default.**

### Outside the interface

**Brought up to date by the hand tests of 2026-08-06 and 2026-08-07 (see the two observed sections below).** Every means named in this answer that has a Python form was run with no window at all, on the rented Linux machine, in this version. The two bulk sweeps, the missing-file search, the library relocation, saving elsewhere with the remap option, and the write call all returned `{'FINISHED'}` and did what they claim, with no interface present. Two operators did **not** on that machine: `bpy.ops.image.replace` and `bpy.ops.sequencer.change_path` both refused, naming a failed context check. The Mac sitting of 2026-08-07 took those two further, and they part company. **`bpy.ops.image.replace` needs a window**: with one on screen it returned `{'FINISHED'}` and rewrote the recorded path; with no window it refused on the Mac too, in every version. **`bpy.ops.sequencer.change_path` does not need a window**: with no window at all, given the file's name and the sequencer's own scene in the context, it returned `{'FINISHED'}` and rewrote the recorded folder and file, in every version. So this line is answered for everything here except that one operator, and that one is answered the other way. The words below are the researcher's own and are left as they were written.


Does this hold when Blender is driven from outside its own window?

**Partly documented, partly not, and the gap matters.**

Documented: Blender has a background mode — "-b, --background: Run in background (often used for UI-less rendering)" — and documented ways to run Python in it: `--python`, `--python-expr`, `--python-text`, and `-c, --command`, the last of which "implies --background mode". `--render-output` is a command-line argument and therefore only meaningful from outside the interface. All from `https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html`, read 2026-07-30. The 5.2 release notes describe the path iterator's new options as being "so render farms and other scripts can find all relevant image files for rendering" (`https://developer.blender.org/docs/release_notes/5.2/python_api/`, read 2026-07-30) — the plainest statement in the sources that this means is meant for unattended use.

Not documented: **no source I found states that any of these means works in background mode.** The data-side means (A, B, C, D, I, K, and `libraries.write`) are ordinary reads and writes of data with no described dependence on a window, but the sources do not say so, and I will not assert it.

The operator-based means (E, F, G, and the save operators in H) carry a specific documented risk. Blender's own guidance says operators can refuse to run depending on context:

> When calling an operator it gives an error like this: … RuntimeError: Operator bpy.ops.action.clean.poll() failed, context is incorrect … Certain operators in Blender are only intended for use in a specific context…

Source: `https://docs.blender.org/api/5.2/info_gotchas_operators.html`, read 2026-07-30. Nothing tells us whether the External Data operators are among those. Two of them are additionally described in the manual purely as interface actions — Find Missing Files ("A File Browser will show up") and Relocate ("a right-click on a library"). Both do have parameters that would let a script supply the answer the interface would ask for (`directory` on Find Missing Files; `filepath`, `directory`, `filename` on Relocate), but the sources do not say that supplying them avoids the file browser.

**Where it differs from inside:** the Outliner's Blender File mode and the File ▸ External Data menu are interface only. Their scripted equivalents are the operators named above; whether those behave identically is not stated. Section I (`libraries.load()` input side) has no interface equivalent described at all — it is a scripting-only means.

### Operating system

**Brought up to date by the hand tests of 2026-08-06 and 2026-08-07 (see the two observed sections below).** Everything in this answer that could be run was run on the rented Linux machine, so **Linux is settled** for it. **macOS is now answered for the tests the Mac sitting ran**, and they are the ones this line turns on: the same file, built the same way, lists the same paths on the two machines — twenty-five of the twenty-seven printed lines are identical character for character. Two are not. The flat path list carries one extra entry on the Mac, a path into Blender's own application folder. And **whether capital letters in a name matter is answered differently on the two machines**: asked for its own image file with the name written in capitals, the Mac found it and the Linux machine did not. *(Citation check, 2026-08-06: narrowed. That check asks the operating system directly — `os.path.exists` on a name written in capitals — with none of Blender's own path machinery in it. It settles that the two file systems differ on letter case; it settles nothing about the means in this answer.)* Note what is still only Linux: the entries this round marked as running on both machines were in fact run on Linux alone, so for those the macOS half is untested rather than settled. **Windows stays untouched**: no Windows machine has run it, so the cross-drive question the sources raise was not answered by any test here. The words below are the researcher's own and are left as they were written.


**Not in the sources**, for the means themselves. I found no statement in the 5.2 manual, API documentation, release notes or Cycles files that any of these means behaves differently on Windows, macOS or Linux.

What the sources do say, adjacent to the question and worth naming so nobody mistakes it for more:

- The path helper module carries three functions whose descriptions imply operating systems differ in how paths look: `bpy.path.native_pathsep` — "Replace the path separator with the system's native os.sep"; `bpy.path.basename` — "Equivalent to os.path.basename, but skips a "//" prefix. Use for Windows compatibility."; `bpy.path.resolve_ncase` — "Resolve a case insensitive path on a case sensitive system, returning a string with the path if found else return the original path." Source: `https://docs.blender.org/api/5.2/bpy.path.html`, read 2026-07-30. These are helpers a script may call; none of them lists or rewrites what a blend-file records, and none of them says anything about the means above.
- Blender warns that paths may not be valid text at all: "Paths are an exception to this rule since the existence of non-UTF-8 paths on the user's file system cannot be ignored." Source: `https://docs.blender.org/api/5.2/info_gotchas_file_paths_and_encoding.html`, read 2026-07-30.
- Two exporters — not the means in this answer — say relative paths cannot always be made: "Uses relative paths in every case (except when on a different drive on Windows)." Source: `https://docs.blender.org/manual/en/5.2/files/import_export/obj.html`, read 2026-07-30. Whether Make Paths Relative has the same limit is **not** stated. I flag it because it is the obvious place this answer could be wrong.
- Blender's requirements page confirms Blender runs on Windows, macOS and Linux (`https://www.blender.org/download/requirements/`, read 2026-08-01), but it does not name 5.2 in so many words, so I do not lean on it for this version.

---

### Where I looked

**How I searched.** I searched the whole of `Blender Sources/manual_5.2`, `api_5.2`, `release_notes_5.2` and `cycles_5.2` at once by text, before opening anything.

**Terms searched across all four sources:** `blend_paths`, `file_path_foreach`, `file_path_map`, `make_paths_relative`, `make_paths_absolute`, `find_missing_files`, `report_missing_files`, `lib_relocate`, `lib_reload`, `path_remap`, `relative_remap`, `Remap Relative`, `External Data`, `Relocate`, `Relative Paths`, `relative path`, `Make Paths Relative`, `Report Missing Files`, `Find Missing Files`, `file path`, `missing`, `blendfile`, `Asset Tracer`, `different drive`, `drive letter`, `case sensitive`, `case insensitive`, `pack`, `unpack`, `texture_cache`, `subtype='FILE_PATH'`, `FILE_PATH`, `DIR_PATH`, `is_path_output`.

**Structural sweeps.** I listed every `bpy.types.*` page in the 5.2 API documentation carrying a property named exactly `filepath` — 27 pages, 26 of them named in section D. **Citation check (Opus 5, 2026-08-06):** the sentence used to say all 27 were named. The one that is not is `BlendData.filepath` — "Path to the .blend file (default "", readonly, never None)" — which is the open file's own location rather than a location it records, and is read-only. I listed every command-line argument documented for 5.2 (about 130 entries) and read the description of every one whose name or text mentioned a path; only `--render-output` and the Python/command entry points were relevant. I listed every file in the manual's `files/` section and read `file_paths`, `blend/open_save`, `blend/packed_data`, `linked_libraries/link_append`, `linked_libraries/index`, `data_blocks` and `asset_libraries/introduction`.

**Pages opened in full:** manual — `files/file_paths`, `files/blend/packed_data`, plus the relevant stretches of `interface/window_system/topbar`, `files/blend/open_save`, `files/linked_libraries/link_append`, `editors/outliner/interface`, `render/output/properties/output`, `advanced/command_line/arguments`, `files/asset_libraries/introduction`. API — `bpy.path`, `bpy.utils` (the `blend_paths` entry), `bpy.types.BlendData` (the two path methods), `bpy.types.BlendDataPathMeta`, `bpy.types.BlendDataLibraries`, `bpy.types.BlendImportContextLibrary`, `bpy.types.BlendImportContextLibraries`, `bpy.types.Library`, `bpy.types.Property`, `bpy_types_enum_items/file_path_foreach_flag_items`, `bpy_types_enum_items/property_subtype_items`, `bpy_types_enum_items/id_type_items`, `info_gotchas_file_paths_and_encoding`, `info_gotchas_operators`, and the relevant entries in `bpy.ops.file` and `bpy.ops.wm`. Release notes — `python_api` in full, plus text searches of `core`, `pipeline_io`, `cycles`, `bugfixes` and `index`. Cycles — `maketx.py` in full, plus searches of `properties.py`, `operators.py`, `osl.py` for path settings.

**Ruled out, and why.** No picture in the manual's `_images` folder was needed: every claim here is carried by words, and the two pictures named on the File Paths page (`files_file_paths-invalid_path_template.png` and `files_file_paths-invalid_path_template_tooltip.png`) are about template errors, not about listing or rewriting. Cycles' own files were ruled out as a source for this question: the only path-related thing in them is `maketx.py`, a command (`blender --command maketx`) that *generates* texture cache files and can set `bpy.context.preferences.filepaths.texture_cache_directory`; it neither lists nor rewrites what a blend-file records. The requirements page was ruled out because it never names 5.2 in so many words. I did not use the web, and did not need to.

**One thing I looked for and did not find:** any means, of any kind, that lists the recorded locations of a blend-file **without opening it**, other than the library-paths-only listing in section I. I searched for it under `blend_paths`, `file_path`, `blendfile`, `Asset Tracer`, and through the whole command-line argument list. If it exists in 5.2, it is not in these sources.

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

2026-08-06 — Opus 5 (Claude Code) — Reading a file, Blender 5.2

2026-08-07 — Opus 5 (Claude Code) — the Mac half of the hand tests

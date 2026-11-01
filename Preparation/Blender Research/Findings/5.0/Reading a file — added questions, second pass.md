# Reading a file — added questions, second pass (Blender 5.0)

**Sources consulted, and when.**
- Blender 5.0 manual (plain-text snapshot), read 2026-07-30
- Blender 5.0 Python API documentation (plain-text snapshot), read 2026-07-30
- Blender 5.0 release notes (plain files), read 2026-07-30
- Cycles' own files for 5.0, read 2026-07-30
- Blender's requirements page (snapshot taken 2026-08-01) — opened, nothing in it bears on this question, so it is not cited below

**I read only the question sheet and files inside `Blender Sources/`. I did not open any other part of this project.**

Scope: question 9 only. Questions 1–8 on the sheet were not attempted.

---

## 9. What separate means exist for listing and rewriting the locations recorded in a `.blend` file, and which kinds of recorded location does each of those means reach?

### The question was ambiguous, and here is the reading I answered

"Locations recorded in a `.blend` file" can be read two ways.

- **The reading I answered (the wider one):** every file or folder path stored inside the file, whether the file reads from it (an image, a linked `.blend`, a cache) or writes to it (the render output path, the compositor's File Output folder).
- **The other reading (narrower):** only the paths of things the file points *at* — its external references — leaving output paths out.

I answered the wider reading because the question says "locations recorded", not "references". Where a means only reaches the narrower set, or where the sources do not say which set it reaches, I say so in the entry. This distinction matters: several of the bulk means below are described only as acting on "external files", and the sources never state whether an output path counts as one.

### Label

**Mixed.** Each named means below is **documented** — the sources describe it in so many words. The *coverage* column is **documented** only for the per-property means; for every bulk means it is **not in the sources**, because the documentation says "all paths to external files" and never enumerates which properties that phrase covers. One entry is **observed** (Cycles' files). The claim that the list below is *complete* is **not in the sources** — no page in the 5.0 manual or API documentation gathers these means into one list, so completeness is my search result, not a documented fact.

### The means, and what each reaches

There are, in the 5.0 sources, nine separate means. Four of them list, seven of them rewrite, and only one does both at a fine grain.

---

#### Means 1 — `bpy.utils.blend_paths()` — lists only, never rewrites

Exact words:

> `bpy.utils.blend_paths(*, absolute=False, packed=False, local=False)`
>
> Returns a list of paths to external files referenced by the loaded .blend file.
>
> Parameters:
>
> absolute (bool) – When true the paths returned are made absolute.
>
> packed (bool) – When true skip file paths for packed data.
>
> local (bool) – When true skip linked library paths.
>
> Returns:
>
> path list.
>
> Return type:
>
> list[str]

Source: Blender 5.0 Python API documentation, `https://docs.blender.org/api/5.0/bpy.utils.html`. Read 2026-07-30.

What it reaches: "external files referenced by the loaded .blend file". The two skip-flags tell you that, left at their defaults, the list **includes** paths belonging to packed data and **includes** linked library paths. That much is documented. What is *not* documented is any enumeration of which stored properties feed it, and whether the render output path is in or out. The word "referenced" points to the narrower reading, but the page does not say so, so I will not claim it.

This means cannot rewrite anything. It hands back a plain `list[str]` — a copy, with no link back to the property each string came from. So it tells you *that* a path exists and not *where in the file it lives*.

An ambiguity in the sources themselves: "skip linked library paths" could mean the paths of the library `.blend` files, or the paths recorded inside data that came from a library. The page does not settle it.

---

#### Means 2 — the path properties themselves, one per kind of thing — lists **and** rewrites, and is the only means whose reach is documented item by item

Every recorded location sits on a named, writable property. Reading the property lists the location; assigning to it rewrites the location. This is the only means with a documented, enumerable reach, because the reach *is* the list of properties.

In 5.0 the API documentation marks a path property with the note `blend relative // prefix supported`. Grouped by what kind of location they are:

**Citation check note (Opus 5, 2026-08-06).** The sentence that used to stand here said "Thirty-eight types carry at least one", and the list below was read as being those thirty-eight. The count and the list are not the same set, so the count has been cut and what is true of each is written out instead. The phrase appears on **40** pages of `api_5.0`: two are operator pages (`bpy.ops.view3d`, `bpy.ops.workspace`) and five are collection pages whose `load()` methods carry it (`BlendDataFonts`, `BlendDataImages`, `BlendDataMovieClips`, `BlendDataSounds`, `BlendDataTexts`), leaving **33 type pages** of stored path properties. The list below names all 33 of those, and **five more that carry no marker at all** — `Text`, `Camera`, `TextureNodeOutput`, `BlendImportContextLibrary` and `BlendData` — which are included because they are recorded locations even though the marker search does not find them. The first two are flagged as unmarked in the entries themselves.

**A linked `.blend` file**
- `Library.filepath` — "Path to the library .blend file"
- `LibraryWeakReference.filepath` — "Path to the library .blend file" (read-only object: "Read-only external reference to a linked data-block and its library file")
- `BlendImportContextLibrary.filepath` — read-only, and the page says it is "Currently only exposed as read-only data for the pre/post blendimport handlers"

**A media file the scene reads**
- `Image.filepath` — "Image/Movie file name"; `Image.filepath_raw` — "Image/Movie file name (without data refreshing)"
- `ImagePackedFile.filepath`
- `Sound.filepath` — "Sound sample file used by this Sound data-block"
- `MovieClip.filepath` — "Filename of the movie or sequence file"
- `Volume.filepath` — "Volume file used by this Volume data-block"; `VolumeGrids.frame_filepath` (read-only)
- `VectorFont.filepath`
- `CacheFile.filepath` and `CacheFileLayer.filepath` — "Path to the archive"
- `MovieStrip.filepath`, `ImageStrip.directory`
- `Text.filepath` — "Filename of the text file". Note: this one is marked `(never None)` **without** the `blend relative // prefix supported` note that all the others carry.

**A shader or light-data file**
- `ShaderNodeScript.filepath` — "Shader script path"
- `ShaderNodeTexIES.filepath` — "IES light path"
- `Camera.custom_filepath` — "Path to the shader defining the custom camera". Also marked `(never None)` only, without the blend-relative note.

**A cache or bake folder the file writes into and reads back**
- `PointCache.filepath` / `PointCacheItem.filepath` — "Cache file path"
- `FluidDomainSettings.cache_directory` — "Directory that contains fluid cache files"
- `NodesModifier` and `NodesModifierBake` bake directory — "Location on disk where the bake data is stored"
- `MultiresModifier.filepath` and `MeshCacheModifier.filepath` — "Path to external displacements file"
- `OceanModifier.filepath` — "Path to a folder to store external baked images"
- `DynamicPaintSurface` image output path — "Directory to save the textures"
- `StripProxy.directory` — "Location to store the proxy files"; `StripProxy.filepath` — "Location of custom proxy file"; `MovieClipProxy.directory` — "Location to store the proxy files"; `SequenceEditor.proxy_dir`

**A place the file writes finished work to**
- `RenderSettings.filepath` — "Directory/name to save animations, # characters define the position and padding of frame numbers", marked `(never None, blend relative // prefix supported, Supports template expressions)`
- `CompositorNodeOutputFile` base path — "The directory where the image will be written", also `Supports template expressions`
- `BakeSettings.filepath` — "Image filepath to use when saving externally"
- `CollectionExport.filepath` — "The file path used for exporting"
- `TextureNodeOutput.filepath`

**A node socket carrying a path**
- `NodeSocketStringFilePath` — "Input value used for unconnected socket"

**Not recorded in the `.blend` at all, despite appearing in the same search**
- `PreferencesFilePaths` (font, sound, texture, render-output and render-cache directories) is preferences, not file content. `BlendData.filepath` — "Path to the .blend file" — is the open file's own location and is marked `(readonly, never None)`.

Source for every quotation in this section: Blender 5.0 Python API documentation, the `bpy.types.<Name>` page for each type, e.g. `https://docs.blender.org/api/5.0/bpy.types.Library.html`, `https://docs.blender.org/api/5.0/bpy.types.Image.html`, `https://docs.blender.org/api/5.0/bpy.types.RenderSettings.html`, `https://docs.blender.org/api/5.0/bpy.types.LibraryWeakReference.html`, `https://docs.blender.org/api/5.0/bpy.types.Camera.html`, `https://docs.blender.org/api/5.0/bpy.types.BlendData.html`. Read 2026-07-30.

One thing this means reaches that no bulk means is documented to reach: the *literal stored text*. For two kinds of location the stored text is not the final path. The manual says:

> Currently path templates are only supported for the render output path in Scene properties and the output paths in the compositor's File Output node.

and

> This substitution happens internally at time of use (e.g. while rendering), so the template syntax will remain as-is in the path field.

Source: Blender 5.0 manual, `https://docs.blender.org/manual/en/5.0/files/file_paths.html`. Read 2026-07-30.

The 5.0 release notes record that this set grew:

> ## More Path Template Variables
>
> The following new path template variables have been introduced […]
>
> - `scene_name`
> - `camera_name`
> - `node_name`
> - `blend_name_lib`
> - `blend_dir`
> - `blend_dir_lib`

Source: Blender 5.0 release notes, `https://developer.blender.org/docs/release_notes/5.0/core/`. Read 2026-07-30.

---

#### Means 3 — Make Paths Relative / Make Paths Absolute — rewrites in bulk, lists nothing

Exact words, API:

> `bpy.ops.file.make_paths_absolute()`
>
> Make all paths to external files absolute
>
> `bpy.ops.file.make_paths_relative()`
>
> Make all paths to external files relative to current .blend

Source: `https://docs.blender.org/api/5.0/bpy.ops.file.html`. Read 2026-07-30.

Exact words, manual:

> Make Paths Relative
>
> Make all paths to external files relative to the current blend-file.
>
> Make Paths Absolute
>
> Make all paths to external files absolute (= full path from the system's root).

Source: Blender 5.0 manual, `https://docs.blender.org/manual/en/5.0/interface/window_system/topbar.html` (the `File ‣ External Data` menu). Read 2026-07-30.

What it reaches: "all paths to external files", four words that are never expanded anywhere in the 5.0 sources. Whether that phrase takes in the render output path, cache folders and proxy folders is **not in the sources**. Both operators take no parameters at all, so there is no way to aim them at a subset.

---

#### Means 4 — Report Missing Files — lists, and only the broken ones

Exact words, API:

> `bpy.ops.file.report_missing_files()`
>
> Report all missing external files

Exact words, manual:

> Report Missing Files
>
> This option is useful to check if there are links to unpacked files that no longer exist.
> After selecting this option, a warning message will appear in the Info editor's header.
> If no warning is shown, there are no missing external files.

Sources: `https://docs.blender.org/api/5.0/bpy.ops.file.html` and `https://docs.blender.org/manual/en/5.0/interface/window_system/topbar.html`. Read 2026-07-30.

What it reaches: "links to unpacked files that no longer exist" — so packed data is out by definition, and present files are out. It is a listing means with a very narrow filter, and its output goes to a place in the interface (the Info editor's header), not to a value you can pick up. The operator returns nothing described.

---

#### Means 5 — Find Missing Files — rewrites in bulk, by searching a folder

Exact words, API:

> `bpy.ops.file.find_missing_files(*, find_all=False, directory='', hide_props_region=True, …)`
>
> Try to find missing external files
>
> find_all (boolean, (optional)) – Find All, Find all files in the search path (not just missing)
>
> directory (string, (optional, never None)) – Directory, Directory of the file

Exact words, manual:

> Find Missing Files
>
> In case you have broken links in a blend-file, this can help you to fix the problem.
> A File Browser will show up. Select the desired directory (or a file within that directory),
> and a search will be performed in it, recursively in all contained directories.
> Every missing file found in the search will be recovered.
> Those recoveries will be done as absolute paths,
> so if you want to have relative paths you will need to select Make Paths Relative.

Sources: `https://docs.blender.org/api/5.0/bpy.ops.file.html` and `https://docs.blender.org/manual/en/5.0/interface/window_system/topbar.html`. Read 2026-07-30.

What it reaches: missing external files by default; with `find_all` turned on, "all files in the search path (not just missing)" — which is the one bulk means in 5.0 that will repoint locations that are not broken. It always writes absolute paths. It has a `directory` parameter, so it can be pointed at a folder by argument rather than by a person choosing one.

---

#### Means 6 — Save As with Remap Relative — rewrites, but only relative paths, and only while saving elsewhere

Exact words, API:

> `bpy.ops.wm.save_as_mainfile(*, filepath='', …, compress=False, relative_remap=True, copy=False)`
>
> Save the current file in the desired location
>
> relative_remap (boolean, (optional)) – Remap Relative, Remap relative paths when saving to a different directory

The same parameter is on `bpy.ops.wm.save_mainfile`, where its default is `False` rather than `True`.

Exact words, manual:

> Remap Relative
>
> This option remaps Relative Paths
> (such as linked libraries and images) when saving a file in a new location.

Sources: `https://docs.blender.org/api/5.0/bpy.ops.wm.html` and `https://docs.blender.org/manual/en/5.0/files/blend/open_save.html`. Read 2026-07-30.

I also opened the manual's picture of the Save dialog, `_images/files_blend_open-save_file-browser-save.png`, cited on that same page. It shows a "Save As Blender File" panel with three checkboxes — Compress, Remap Relative, Save Copy — with Compress and Remap Relative ticked and Save Copy unticked. That confirms Remap Relative is an option a person sees at save time. **documented**, from a manual picture.

What it reaches: relative paths only. Absolute paths are untouched, which the wording makes plain. The manual's examples are "linked libraries and images"; it says "such as", so that is not a closed list, and the sources do not close it.

Two documented limits sit near this:

> You cannot use relative paths into a new untitled blend-file.
> Save it before linking to external files.

and

> Relative paths are the default but this can be changed
> in the File tab of the Preferences.

Source: `https://docs.blender.org/manual/en/5.0/files/blend/open_save.html`. Read 2026-07-30.

---

#### Means 7 — `bpy.data.libraries.write(path_remap=…)` — rewrites while writing a *new* file

Exact words:

> `write(filepath, datablocks, *, path_remap=False, fake_user=False, compress=False)`
>
> Write data-blocks into a blend file.
>
> path_remap (str) –
> Optionally remap paths when writing the file:
>
> NONE No path manipulation (default).
>
> RELATIVE Remap paths that are already relative to the new location.
>
> RELATIVE_ALL Remap all paths to be relative to the new location.
>
> ABSOLUTE Make all paths absolute on writing.

Source: Blender 5.0 Python API documentation, `https://docs.blender.org/api/5.0/bpy.types.BlendDataLibraries.html`. Read 2026-07-30.

What it reaches: "all paths" in the file being written, with the same unexpanded "paths" as everywhere else. This is the only means with four separate remap modes, and the only one that distinguishes "paths that are already relative" from "all paths" in its own wording. It does not touch the currently open file; it writes a new one.

---

#### Means 8 — the library-only means: Relocate and Reload — rewrite exactly one kind of location

Exact words, API:

> `bpy.ops.wm.lib_relocate(*, library='', filepath='', directory='', filename='', files=None, …)`
>
> Relocate the given library to one or several others
>
> `bpy.ops.wm.lib_reload(*, library='', filepath='', …)`
>
> Reload the given library
>
> `bpy.ops.outliner.lib_relocate()`
>
> Relocate the library under cursor
>
> `bpy.ops.wm.id_linked_relocate(*, id_session_uid=0, filepath='', …)`
>
> Relocate a linked ID, i.e. select another ID to link, and remap its local usages to that newly linked data-block). Currently only designed as an internal operator, not directly exposed to the user

And on the library data-block itself:

> `reload()`
>
> Reload this library and all its linked data-blocks

Sources: `https://docs.blender.org/api/5.0/bpy.ops.wm.html`, `https://docs.blender.org/api/5.0/bpy.ops.outliner.html`, `https://docs.blender.org/api/5.0/bpy.types.Library.html`. Read 2026-07-30.

Exact words, manual:

> Relocate Library
>
> […] When the Outliner is in the Blender File Display Mode,
> a right-click on a library (a linked blend-file) will show the Relocate option.
> It allows to replace it by a different file.
> This can be used to either fix a broken linked library (e.g. because the file was moved
> or renamed), or to switch to a variation of the same data in a different file.
>
> Broken Libraries
>
> If Blender cannot find a library while loading a blend-file,
> it will create placeholder data-blocks to replace missing linked ones.
> That way, references to the missing data are not lost, and by relocating the missing library,
> the lost data can be automatically restored.

Source: `https://docs.blender.org/manual/en/5.0/files/linked_libraries/link_append.html`. Read 2026-07-30.

What it reaches: linked `.blend` files, and nothing else. `lib_relocate` takes a `library` name, so it can be aimed at one library at a time. `id_linked_relocate` reaches a single linked data-block rather than a whole library, and the API page itself warns it is internal and "not directly exposed to the user".

The Outliner's own Blender File mode is the listing partner to these:

> Blender File:
>
> Lists all data in the current blend-file.

Source: `https://docs.blender.org/manual/en/5.0/editors/outliner/interface.html`. Read 2026-07-30. Note the manual describes it as listing *data*, not paths — it does not say the paths are shown there, so I will not claim they are.

---

#### Means 9 — pack and unpack — changes whether a recorded location is used at all

Exact words, API:

> `bpy.ops.file.pack_all()`
>
> Pack all used external files into this .blend
>
> `bpy.ops.file.unpack_all(*, method='USE_LOCAL')`
>
> Unpack all files packed into this .blend to external ones
>
> method (enum in ['USE_LOCAL', 'WRITE_LOCAL', 'USE_ORIGINAL', 'WRITE_ORIGINAL', 'KEEP', 'REMOVE'], (optional)) – Method, How to unpack
>
> `bpy.ops.file.unpack_item(*, method='USE_LOCAL', id_name='', id_type=19785)`
>
> Unpack this file to an external file
>
> `bpy.ops.file.pack_libraries()`
>
> Store all data-blocks linked from other .blend files in the current .blend file. Library references are preserved so the linked data-blocks can be unpacked again
>
> `bpy.ops.file.unpack_libraries()`
>
> Restore all packed linked data-blocks to their original locations
>
> `bpy.ops.file.autopack_toggle()`
>
> Automatically pack all external files into the .blend file

Source: `https://docs.blender.org/api/5.0/bpy.ops.file.html`. Read 2026-07-30.

Exact words, manual, on unpacking:

> Use files in current directory (create when necessary)
>
> Unpacks all files in the same directory // as the blend-file,
> grouping them in proper folders (like ''textures'' for instance).

and

> Warning
>
> Not all external files can be packed
>
> Some typically heavy external files, like videos from the Sequence Editor
> or Movie Clips, cannot be packed in a blend-file.

Source: `https://docs.blender.org/manual/en/5.0/files/blend/packed_data.html`. Read 2026-07-30.

What it reaches: the manual says unpacking to the current directory puts files "in the same directory // as the blend-file", and `//` is Blender's relative prefix. That the *recorded* location therefore changes to a new one is a step past the quoted words — the manual describes where files are put, not what the path property afterwards holds. So: **inferred**, and the step is written out here. The documented part is that packing and unpacking change whether the recorded location is consulted, and that videos and movie clips cannot be packed at all.

Blender 5.0 added packed linked data, which brings a new kind of recorded library location into the file:

> Name collisions are averted by storing these packed IDs into a new type of 'archive' libraries (and their namespaces). These libraries:
>
> * Only contain packed IDs.
> * Are owned and managed by their 'real' library data-block, called an 'archive parent'.

Source: Blender 5.0 release notes, `https://developer.blender.org/docs/release_notes/5.0/core/`. Read 2026-07-30. The matching properties are on `Library`: `archive_libraries` — "Archive libraries of packed IDs, generated (and owned) by this source library", and `archive_parent_library` — "Source library from which this archive of packed IDs was generated", both marked `(readonly)` (`https://docs.blender.org/api/5.0/bpy.types.Library.html`).

---

#### Two things that look like means but are not

**`bpy.path.abspath` and `bpy.path.relpath` compute; they never rewrite.**

> `bpy.path.abspath(path, *, start=None, library=None)`
>
> Returns the absolute path relative to the current blend file
> using the "//" prefix.
>
> library (bpy.types.Library) – The library this path is from. This is only included for
> convenience, when the library is not None its path replaces start.
>
> `bpy.path.relpath(path, *, start=None)`
>
> Returns the path relative to the current blend file using the "//" prefix.

Source: `https://docs.blender.org/api/5.0/bpy.path.html`. Read 2026-07-30. These hand back a string. Nothing in the file changes unless you assign the result to a property yourself, which is Means 2.

**`-o` / `--render-output` sets one location from the command line.**

> -o, --render-output <path>
>
> Set the render path and file name.
> Use // at the start of the path to render relative to the blend-file.

Source: `https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html`. Read 2026-07-30. It reaches exactly one recorded location — the render output path — and the page does not say the change is written back to the `.blend`, only that it is set.

---

#### Cycles' own files — **observed**

Cycles adds no path setting of its own that is stored in a `.blend`. Searching every file in `cycles_5.0` for `FILE_PATH`, `DIR_PATH`, `filepath` and `abspath` returns only: operators' own input fields in `operators.py`, all `subtype='FILE_PATH'` — `input_filepath` / `output_filepath` on `CYCLES_OT_denoise_animation` at lines 37–47, and `input_filepath1` / `input_filepath2` / `output_filepath` on `CYCLES_OT_merge_images` at lines 119–135 — which belong to running operators and not to stored data; and `osl.py`, which *reads* two of the core path properties listed under Means 2. (**Citation check, 2026-08-06:** the three `CYCLES_OT_merge_images` arguments were missing from this sentence; the whole-folder search returns six `FILE_PATH` hits, five in `operators.py` and one in `osl.py`. The first-pass document for 5.0 already had them.)

Exact lines from `Blender Sources/cycles_5.0/osl.py`:

- line 197, inside `update_external_script`: `    script_path = bpy.path.abspath(filepath, library=library)`
- line 237, inside `update_internal_script`: `    osl_path = bpy.path.abspath(script.filepath, library=script.library)`
- line 278: `        ok, oso_path, oso_file_remove = update_external_script(report, node.filepath, node.id_data.library)`
- line 334: `        ok, oso_path, oso_file_remove = update_external_script(report, cam.custom_filepath, cam.library)`

And in `Blender Sources/cycles_5.0/properties.py`, the only two `StringProperty` declarations in the whole file are at lines 1563–1564, and neither is a path:

> `    id: StringProperty(name="ID", description="Unique identifier of the device")`
> `    name: StringProperty(name="Name", description="Name of the device")`

Read 2026-07-30.

What this shows: Cycles resolves `ShaderNodeScript.filepath` and `Camera.custom_filepath` through `bpy.path.abspath` with the owning data-block's `library` passed in — that is, it honours the `//` prefix and resolves it against the library the data came from, not only against the open file. That is what the code says. What Cycles does at render time with a path that turns out to be missing is a step beyond the file and I did not take it.

---

#### Outside Blender entirely

The manual names, in passing, that such tooling exists:

> Since there is already tooling that can manipulate blend-files outside of Blender itself
> (see Blender Asset Tracer),
> it's possible to also create an external tool for doing such edits across blend-files.

Source: Blender 5.0 manual, `https://docs.blender.org/manual/en/5.0/files/asset_libraries/introduction.html`. Read 2026-07-30.

What it reaches: the sources do not say. The name is mentioned once, in a "Future Development" section about the Asset Browser, with no description of what it lists or rewrites and no documentation of its own inside `Blender Sources/`. I record the mention because it is the only acknowledgement in the 5.0 sources that the locations in a `.blend` can be reached without Blender; I cannot say anything about its coverage from these sources.

The same page states the rule that shapes all of the above:

> As described above, Blender itself is not allowed to write to other blend-files
> than the currently open one.

---

### Not covered

- **Which properties the bulk means actually touch.** This is the biggest gap and it is a real one. "All paths to external files" (Means 3), "external files referenced by" (Means 1) and "all paths" (Means 7) are never expanded into a list of property kinds anywhere in the 5.0 manual, API documentation or release notes. So I cannot say from the sources whether Make Paths Relative reaches `RenderSettings.filepath`, `FluidDomainSettings.cache_directory`, `StripProxy.directory`, `NodeSocketStringFilePath`, `Text.filepath` or `Camera.custom_filepath`. Note that the last two are the two properties marked without the `blend relative // prefix supported` note, which is suggestive but not a statement.
- **Whether the bulk means agree with each other.** Three of them use three different phrasings. Nothing says they cover the same set.
- **Order of operations.** Whether `find_all` in Find Missing Files can repoint a path to the wrong file when two files of the same name exist under the search folder is not addressed.
- **Drivers, custom properties and add-on-stored paths.** A path stashed in a custom property is a string like any other; nothing in the sources says whether any bulk means sees it. I found no page addressing this.
- **What a rewrite does to data already loaded.** The manual notes for Find Missing Files that "Recovered files might need to be reloaded", but says nothing equivalent for the other rewriting means.
- **Reading locations out of a `.blend` that is not open.** Every means above except the external-tooling mention operates on the loaded file. `bpy.data.libraries.load()` opens a `.blend` to read names of data-blocks from it, but nothing on its page says it exposes that file's own recorded paths.
- **Question 9 asks about "separate means" and I have listed nine.** No source enumerates them, so I cannot rule out a tenth that no page I searched names.

---

### Outside the interface

**Brought up to date by the hand tests of 2026-08-06 and 2026-08-07 (see the two observed sections below).** Every means named in this answer that has a Python form was run with no window at all, on the rented Linux machine, in this version. The two bulk sweeps, the missing-file search, the library relocation, saving elsewhere with the remap option, and the write call all returned `{'FINISHED'}` and did what they claim, with no interface present. Two operators did **not** on that machine: `bpy.ops.image.replace` and `bpy.ops.sequencer.change_path` both refused, naming a failed context check. The Mac sitting of 2026-08-07 took those two further, and they part company. **`bpy.ops.image.replace` needs a window**: with one on screen it returned `{'FINISHED'}` and rewrote the recorded path; with no window it refused on the Mac too, in every version. **`bpy.ops.sequencer.change_path` does not need a window**: with no window at all, given the file's name and the sequencer's own scene in the context, it returned `{'FINISHED'}` and rewrote the recorded folder and file, in every version. So this line is answered for everything here except that one operator, and that one is answered the other way. The words below are the researcher's own and are left as they were written.


Does this hold when Blender is driven from outside its own window? **Partly, and the sources are thinner here than one would like.**

What the sources do say:

- The command line has `-b, --background` — "Run in background (often used for UI-less rendering)" — and `-P, --python <filepath>`, `--python-expr <expression>` and `--python-text <name>` for running code, all on `https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html` (read 2026-07-30). So Means 1, 2, 3, 5, 6, 7, 8 and 9 all have a Python form that can in principle be reached this way.
- **Means 2 (the properties) is the one that plainly does not need a window.** They are data on data-blocks, with no operator, no context and no editor named on their pages.
- **Means 4 (Report Missing Files) is documented to put its result in the interface**: "a warning message will appear in the Info editor's header". Where that message goes with no Info editor — console, standard output, nowhere — is **not in the sources**.
- **Means 5 (Find Missing Files) is documented to open a File Browser**: "A File Browser will show up." It also has a `directory` parameter, which is what a caller from outside would supply instead. Whether supplying `directory` skips the browser is not stated on either page.
- **Part of Means 8 is Outliner-bound by documentation.** The manual's Reference block for Relocate Library names "Editor: Outliner" and requires "the Outliner is in the Blender File Display Mode". `bpy.ops.outliner.lib_relocate()` is described as acting on "the library under cursor" — a cursor is an interface thing. The `bpy.ops.wm.lib_relocate` form takes a `library` name and a `filepath` as arguments instead, so it is the one to reach for from outside; but no page states it works without a window.
- **Means 6 has a documented difference by entry point.** `relative_remap` defaults to `True` on `save_as_mainfile` and `False` on `save_mainfile`. Anything driving Blender from outside gets whichever default the call it makes carries, and the manual's picture shows a person at the interface gets it ticked.

What the sources do not say: no page in the 5.0 manual or API documentation states, for any of these operators, whether it succeeds or fails in background mode. I searched for it and did not find it. That is the honest answer, and it is a gap worth closing by hand — see the tests below.

### Operating system

**Brought up to date by the hand tests of 2026-08-06 and 2026-08-07 (see the two observed sections below).** Everything in this answer that could be run was run on the rented Linux machine, so **Linux is settled** for it. **macOS is now answered for the tests the Mac sitting ran**, and they are the ones this line turns on: the same file, built the same way, lists the same paths on the two machines — twenty-five of the twenty-seven printed lines are identical character for character. Two are not. The flat path list carries one extra entry on the Mac, a path into Blender's own application folder. And **whether capital letters in a name matter is answered differently on the two machines**: asked for its own image file with the name written in capitals, the Mac found it and the Linux machine did not. *(Citation check, 2026-08-06: narrowed. That check asks the operating system directly — `os.path.exists` on a name written in capitals — with none of Blender's own path machinery in it. It settles that the two file systems differ on letter case; it settles nothing about the means in this answer.)* Note what is still only Linux: the entries this round marked as running on both machines were in fact run on Linux alone, so for those the macOS half is untested rather than settled. **Windows stays untouched**: no Windows machine has run it, so the cross-drive question the sources raise was not answered by any test here. The words below are the researcher's own and are left as they were written.


Does this hold on every operating system? **The sources name path handling as an area where systems differ, but say nothing about these nine means differing.**

What the sources say, all from `https://docs.blender.org/api/5.0/bpy.path.html`, read 2026-07-30:

> `bpy.path.basename(path)`
>
> Equivalent to os.path.basename, but skips a "//" prefix.
>
> Use for Windows compatibility.

> `bpy.path.native_pathsep(path)`
>
> Replace the path separator with the systems native os.sep.

> `bpy.path.resolve_ncase(path)`
>
> Resolve a case insensitive path on a case sensitive system,
> returning a string with the path if found else return the original path.

Three helpers exist precisely because separators differ, case sensitivity differs, and the `//` prefix is not a native construct on any system. That is a documented statement that path text is system-sensitive.

What I could not find: any statement that `blend_paths`, Make Paths Relative/Absolute, Find Missing Files, Remap Relative, `path_remap`, Relocate or pack/unpack behave differently on one system. I searched the whole manual for "drive letter", "different drive", "path separator", "backslash", "case sensitive" and "case-sensitive". The only hits about drives are in the OBJ and FBX exporter pages — "Uses relative paths in every case (except when on a different drive on Windows)" (`https://docs.blender.org/manual/en/5.0/files/import_export/obj.html` and `https://docs.blender.org/manual/en/5.0/addons/import_export/scene_fbx.html`, read 2026-07-30). Those describe exporters writing their own files, not any of the nine means above, so they do not speak for this answer. I note them only so the search record is complete: the one place the 5.0 manual admits a Windows drive-boundary limit on making a path relative is in an exporter, and nothing equivalent is said about Make Paths Relative on the `.blend` itself. **That silence is a gap, not a clearance.**

The requirements page (`https://www.blender.org/download/requirements/`, snapshot read 2026-08-01) names supported systems only; it says nothing about path handling.

---

### Where I looked

**Files searched, whole-tree, in `Blender Sources/api_5.0`, `manual_5.0`, `release_notes_5.0`, `cycles_5.0`, `requirements`.**

Terms searched across the API documentation: `blend_paths`, `make_paths`, `filepath`, `filepath_raw`, `find_missing_files`, `report_missing_files`, `pack_all`, `unpack_all`, `pack_libraries`, `unpack_libraries`, `autopack`, `lib_relocate`, `lib_reload`, `id_linked_relocate`, `relative_remap`, `path_remap`, `remap`, `blend relative // prefix supported`, `external files`, `directory`, `frame_path`, `filepath_from_user`. I also swept the generated index files `genindex-all.txt`, `genindex-B.txt`, `genindex-F.txt`, `genindex-M.txt` for every `filepath` and `paths` entry, which is how I satisfied myself the property list under Means 2 is the complete set the documentation exposes under those names.

Terms searched across the manual: `External Data`, `Make Paths Absolute`, `Make Paths Relative`, `Find Missing Files`, `Report Missing Files`, `Relative Paths`, `relative path`, `Remap Relative`, `Relocate`, `Blender File`, `missing file`, `missing external`, `path template`, `template expressions`, `asset tracer`, `blendfile.py`, `blend file format`, `DNA struct`, `drive letter`, `different drive`, `path separator`, `backslash`, `case sensitive`, `background`.

Manual pages opened in full or in part: `interface/window_system/topbar.txt` (the External Data menu), `files/blend/packed_data.txt`, `files/blend/open_save.txt`, `files/linked_libraries/link_append.txt`, `files/file_paths.txt`, `files/asset_libraries/introduction.txt`, `editors/outliner/interface.txt`, `editors/file_browser.txt`, `editors/preferences/save_load.txt`, `advanced/command_line/arguments.txt`.

Picture opened: `manual_5.0/_images/files_blend_open-save_file-browser-save.png`, cited from `files/blend/open_save.txt`. I checked whether the External Data menu itself is pictured anywhere — it is not; the only pictures on `topbar.txt` are of the menu bar, the workspace tabs and the scene/view-layer selectors, none of which show the External Data entries.

API pages opened in full: `bpy.utils`, `bpy.path`, `bpy.ops.file`, `bpy.ops.wm` (the relevant regions), `bpy.ops.outliner`, `bpy.ops.image`, `bpy.types.Library`, `bpy.types.LibraryWeakReference`, `bpy.types.BlendImportContextLibrary`, `bpy.types.BlendDataLibraries`, `bpy.types.BlendData`, `bpy.types.Image`, `bpy.types.RenderSettings`, `bpy.types.Camera`, plus every type carrying a blend-relative path property.

Release notes read: all of `release_notes_5.0/`, with `core.md` and `pipeline_io.md` read closely and `python_api.md`, `bugfixes.md` and `corrective_releases.md` searched for path terms.

Cycles: every file in `cycles_5.0` searched for `FILE_PATH`, `DIR_PATH`, `filepath`, `abspath`; `osl.py`, `operators.py` and `properties.py` opened at the matching places.

**Ruled out and why:**
- `PreferencesFilePaths` — its properties are preferences, not content of a `.blend`, so they are outside the question.
- `BlendData.filepath` — the open file's own location, marked read-only, and not a location the file points at.
- Keying set paths, motion paths, `GeometryNodeEdgePathsToCurves`, `XrUserPaths` and similar — the word "path" in these means something else entirely; they matched my index sweep and were discarded on reading.
- The requirements page — carries nothing on this subject.
- The wider web — not consulted. I did not need it.

---

### Observed — the hand tests of 2026-08-06

Everything in this section was run on **the rented Linux machine — Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB on driver 580.95.05, an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory and no display server at all**. The Blender was `/opt/blender/5.0/blender`, which reported itself as **5.0.1**, hash `a3db93c5b259`. Every run used `-b` (no window) and `--factory-startup`, and every run had `BLENDER_USER_RESOURCES` pointed at `/root/reading-a-file/res-5.0`, a settings folder made for this round and for nothing else — never at the real one. All work was done inside `/root/reading-a-file/`. The machine is shared with other topics' tests, so nothing here says how long anything took. Its graphics card could not be questioned during this sitting (`nvidia-smi` answered `Failed to initialize NVML: Driver/library version mismatch`); no test below touches the card. The full list of tests, with the set-up and the command for each, is `Audits/Blender Research/Reading a file/Hand tests — added questions.md`. **Nothing below was run on the Creator's Mac**, so nothing below settles macOS; a second sitting does that half.

#### What the machinery is called here, and what its switches are (list entry 1)

Two scripts were run with no file loaded, asking whether each named call exists and what it accepts. What came back, word for word:

```
PROBE version 5.0.1
PROBE has bpy.utils.blend_paths = True
PROBE has bpy.data.file_path_map = True
PROBE has bpy.data.file_path_foreach = True
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
PROBE CompositorNodeOutputFile has base_path = False has directory = True
PROBE Camera.custom_filepath = True
PROBE Text.filepath = True
PROBE TextureNodeOutput.filepath = True
PROBE Theme.filepath = True
PROBE RenderSettings.filepath.is_path_output = True
PROBE RenderSettings.filepath.is_path_supports_blend_relative = True
PROBE RenderSettings.filepath.is_path_supports_templates = True
PROBE RenderSettings.filepath.subtype = FILE_PATH
PROBE2 version 5.0.1
PROBE2 file_path_foreach flag SKIP_LINKED = ACCEPTED
PROBE2 file_path_foreach flag SKIP_PACKED = ACCEPTED
PROBE2 file_path_foreach flag SKIP_WEAK_REFERENCES = ACCEPTED
PROBE2 file_path_foreach flag SKIP_MULTIFILE = ACCEPTED
PROBE2 file_path_foreach flag RELOAD_EDITED = ACCEPTED
PROBE2 file_path_foreach flag RESOLVE_TOKEN = ACCEPTED
PROBE2 file_path_foreach flag EXPAND_TOKENS = REJECTED ValueError flags: 'EXPAND_TOKENS' not found in ('SKIP_LINKED', 'SKIP_PACKED', 'RESOLVE_TOKEN', 'SKIP_WEAK_REFERENCES', 'SKIP_MULTIFILE', 'RELOAD_EDITED')
PROBE2 file_path_foreach flag EXPAND_SEQUENCES = REJECTED ValueError flags: 'EXPAND_SEQUENCES' not found in ('SKIP_LINKED', 'SKIP_PACKED', 'RESOLVE_TOKEN', 'SKIP_WEAK_REFERENCES', 'SKIP_MULTIFILE', 'RELOAD_EDITED')
PROBE2 file_path_foreach flag EXPAND_CACHES = REJECTED ValueError flags: 'EXPAND_CACHES' not found in ('SKIP_LINKED', 'SKIP_PACKED', 'RESOLVE_TOKEN', 'SKIP_WEAK_REFERENCES', 'SKIP_MULTIFILE', 'RELOAD_EDITED')
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
READ | Image.filepath                           | /root/reading-a-file/work/many/5.0/assets/probe.png
READ | Image.filepath_raw                       | /root/reading-a-file/work/many/5.0/assets/probe.png
READ | Sound.filepath                           | /root/reading-a-file/work/many/5.0/assets/probe.wav
READ | VectorFont.filepath                      | /root/reading-a-file/work/many/5.0/assets/probe.woff2
READ | Text.filepath                            | /root/reading-a-file/work/many/5.0/assets/probe.txt
READ | Volume.filepath                          | /root/reading-a-file/work/many/5.0/assets/probe.vdb
READ | MovieClip.filepath                       | /root/reading-a-file/work/many/5.0/assets/probe.png
READ | CacheFile.filepath                       | /root/reading-a-file/work/many/5.0/assets/probe.abc
READ | Library.filepath                         | /root/reading-a-file/work/many/5.0/lib.blend
READ | RenderSettings.filepath                  | /root/reading-a-file/work/many/5.0/assets/render_out/
READ | BakeSettings.filepath                    | /root/reading-a-file/work/many/5.0/assets/bake_out/
READ | CompositorNodeOutputFile base path       | /root/reading-a-file/work/many/5.0/assets/node_out/
READ | PointCache.filepath (cloth)              | /root/reading-a-file/work/many/5.0/assets/pointcache/
READ | FluidDomainSettings.cache_directory      | /root/reading-a-file/work/many/5.0/assets/fluidcache/
READ | NodesModifier.bake_directory             | /root/reading-a-file/work/many/5.0/assets/gnbake/
READ | MultiresModifier.filepath                | 
READ | MeshCacheModifier.filepath               | /root/reading-a-file/work/many/5.0/assets/meshcache.mdd
READ | OceanModifier.filepath                   | /root/reading-a-file/work/many/5.0/assets/oceanbake/
READ | ShaderNodeScript.filepath                | /root/reading-a-file/work/many/5.0/assets/probe.osl
READ | ShaderNodeTexIES.filepath                | /root/reading-a-file/work/many/5.0/assets/probe.ies
READ | Camera.custom_filepath                   | /root/reading-a-file/work/many/5.0/assets/camshader.osl
READ | custom property on an object             | /root/reading-a-file/work/many/5.0/assets/custom_prop.png
READ | ImageStrip.directory                     | /root/reading-a-file/work/many/5.0/assets/stripdir/
READ | StripProxy.directory (sequencer)         | /root/reading-a-file/work/many/5.0/assets/proxydir/
```

And the listing means:

```
LIST blend_paths()                 = ['/opt/blender/5.0/5.0/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/root/reading-a-file/work/many/5.0/assets/camshader.osl', '/root/reading-a-file/work/many/5.0/assets/fluidcache/', '/root/reading-a-file/work/many/5.0/assets/meshcache.mdd', '/root/reading-a-file/work/many/5.0/assets/oceanbake/', '/root/reading-a-file/work/many/5.0/assets/probe.abc', '/root/reading-a-file/work/many/5.0/assets/probe.ies', '/root/reading-a-file/work/many/5.0/assets/probe.osl', '/root/reading-a-file/work/many/5.0/assets/probe.png', '/root/reading-a-file/work/many/5.0/assets/probe.png', '/root/reading-a-file/work/many/5.0/assets/probe.txt', '/root/reading-a-file/work/many/5.0/assets/probe.vdb', '/root/reading-a-file/work/many/5.0/assets/probe.wav', '/root/reading-a-file/work/many/5.0/assets/probe.woff2', '/root/reading-a-file/work/many/5.0/assets/stripdir/probe.png', '/root/reading-a-file/work/many/5.0/lib.blend']
LIST blend_paths(packed=True)      = ['/opt/blender/5.0/5.0/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/root/reading-a-file/work/many/5.0/assets/camshader.osl', '/root/reading-a-file/work/many/5.0/assets/fluidcache/', '/root/reading-a-file/work/many/5.0/assets/meshcache.mdd', '/root/reading-a-file/work/many/5.0/assets/oceanbake/', '/root/reading-a-file/work/many/5.0/assets/probe.abc', '/root/reading-a-file/work/many/5.0/assets/probe.ies', '/root/reading-a-file/work/many/5.0/assets/probe.osl', '/root/reading-a-file/work/many/5.0/assets/probe.png', '/root/reading-a-file/work/many/5.0/assets/probe.png', '/root/reading-a-file/work/many/5.0/assets/probe.txt', '/root/reading-a-file/work/many/5.0/assets/probe.vdb', '/root/reading-a-file/work/many/5.0/assets/probe.wav', '/root/reading-a-file/work/many/5.0/assets/probe.woff2', '/root/reading-a-file/work/many/5.0/assets/stripdir/probe.png', '/root/reading-a-file/work/many/5.0/lib.blend']
LIST blend_paths(local=True)       = ['/opt/blender/5.0/5.0/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/root/reading-a-file/work/many/5.0/assets/camshader.osl', '/root/reading-a-file/work/many/5.0/assets/fluidcache/', '/root/reading-a-file/work/many/5.0/assets/meshcache.mdd', '/root/reading-a-file/work/many/5.0/assets/oceanbake/', '/root/reading-a-file/work/many/5.0/assets/probe.abc', '/root/reading-a-file/work/many/5.0/assets/probe.ies', '/root/reading-a-file/work/many/5.0/assets/probe.osl', '/root/reading-a-file/work/many/5.0/assets/probe.png', '/root/reading-a-file/work/many/5.0/assets/probe.png', '/root/reading-a-file/work/many/5.0/assets/probe.txt', '/root/reading-a-file/work/many/5.0/assets/probe.vdb', '/root/reading-a-file/work/many/5.0/assets/probe.wav', '/root/reading-a-file/work/many/5.0/assets/probe.woff2', '/root/reading-a-file/work/many/5.0/assets/stripdir/probe.png', '/root/reading-a-file/work/many/5.0/lib.blend']
LIST blend_paths(absolute=True)    = ['/opt/blender/5.0/5.0/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/root/reading-a-file/work/many/5.0/assets/camshader.osl', '/root/reading-a-file/work/many/5.0/assets/fluidcache/', '/root/reading-a-file/work/many/5.0/assets/meshcache.mdd', '/root/reading-a-file/work/many/5.0/assets/oceanbake/', '/root/reading-a-file/work/many/5.0/assets/probe.abc', '/root/reading-a-file/work/many/5.0/assets/probe.ies', '/root/reading-a-file/work/many/5.0/assets/probe.osl', '/root/reading-a-file/work/many/5.0/assets/probe.png', '/root/reading-a-file/work/many/5.0/assets/probe.png', '/root/reading-a-file/work/many/5.0/assets/probe.txt', '/root/reading-a-file/work/many/5.0/assets/probe.vdb', '/root/reading-a-file/work/many/5.0/assets/probe.wav', '/root/reading-a-file/work/many/5.0/assets/probe.woff2', '/root/reading-a-file/work/many/5.0/assets/stripdir/probe.png', '/root/reading-a-file/work/many/5.0/lib.blend']
LIST file_path_map()               = ['/root/reading-a-file/work/many/5.0/assets/camshader.osl', '/root/reading-a-file/work/many/5.0/assets/fluidcache/', '/root/reading-a-file/work/many/5.0/assets/meshcache.mdd', '/root/reading-a-file/work/many/5.0/assets/oceanbake/', '/root/reading-a-file/work/many/5.0/assets/probe.abc', '/root/reading-a-file/work/many/5.0/assets/probe.ies', '/root/reading-a-file/work/many/5.0/assets/probe.osl', '/root/reading-a-file/work/many/5.0/assets/probe.png', '/root/reading-a-file/work/many/5.0/assets/probe.txt', '/root/reading-a-file/work/many/5.0/assets/probe.vdb', '/root/reading-a-file/work/many/5.0/assets/probe.wav', '/root/reading-a-file/work/many/5.0/assets/probe.woff2', '/root/reading-a-file/work/many/5.0/assets/stripdir/probe.png', '/root/reading-a-file/work/many/5.0/lib.blend']
LIST file_path_map(include_libraries=True) = ['/root/reading-a-file/work/many/5.0/assets/camshader.osl', '/root/reading-a-file/work/many/5.0/assets/fluidcache/', '/root/reading-a-file/work/many/5.0/assets/meshcache.mdd', '/root/reading-a-file/work/many/5.0/assets/oceanbake/', '/root/reading-a-file/work/many/5.0/assets/probe.abc', '/root/reading-a-file/work/many/5.0/assets/probe.ies', '/root/reading-a-file/work/many/5.0/assets/probe.osl', '/root/reading-a-file/work/many/5.0/assets/probe.png', '/root/reading-a-file/work/many/5.0/assets/probe.txt', '/root/reading-a-file/work/many/5.0/assets/probe.vdb', '/root/reading-a-file/work/many/5.0/assets/probe.wav', '/root/reading-a-file/work/many/5.0/assets/probe.woff2', '/root/reading-a-file/work/many/5.0/assets/stripdir/probe.png', '/root/reading-a-file/work/many/5.0/lib.blend']
LIST file_path_map by data-block   = [('FluidObj', ['/root/reading-a-file/work/many/5.0/assets/fluidcache/']), ('LibColl', ['/root/reading-a-file/work/many/5.0/lib.blend']), ('LibCube', ['/root/reading-a-file/work/many/5.0/lib.blend']), ('LibMesh', ['/root/reading-a-file/work/many/5.0/lib.blend']), ('ModObj', ['/root/reading-a-file/work/many/5.0/assets/meshcache.mdd', '/root/reading-a-file/work/many/5.0/assets/oceanbake/']), ('ProbeCam', ['/root/reading-a-file/work/many/5.0/assets/camshader.osl']), ('ProbeClip', ['/root/reading-a-file/work/many/5.0/assets/probe.png']), ('ProbeFont', ['/root/reading-a-file/work/many/5.0/assets/probe.woff2']), ('ProbeImage', ['/root/reading-a-file/work/many/5.0/assets/probe.png']), ('ProbeMat', ['/root/reading-a-file/work/many/5.0/assets/probe.ies', '/root/reading-a-file/work/many/5.0/assets/probe.osl']), ('ProbeSound', ['/root/reading-a-file/work/many/5.0/assets/probe.wav']), ('ProbeText', ['/root/reading-a-file/work/many/5.0/assets/probe.txt']), ('ProbeVolume', ['/root/reading-a-file/work/many/5.0/assets/probe.vdb']), ('Scene', ['/root/reading-a-file/work/many/5.0/assets/stripdir/probe.png']), ('lib.blend', ['/root/reading-a-file/work/many/5.0/lib.blend']), ('probe.abc', ['/root/reading-a-file/work/many/5.0/assets/probe.abc'])]
LIST foreach[default] CacheFile 'probe.abc' /root/reading-a-file/work/many/5.0/assets/probe.abc
LIST foreach[default] Camera 'ProbeCam' /root/reading-a-file/work/many/5.0/assets/camshader.osl
LIST foreach[default] Image 'ProbeImage' /root/reading-a-file/work/many/5.0/assets/probe.png
LIST foreach[default] Library 'lib.blend' /root/reading-a-file/work/many/5.0/lib.blend
LIST foreach[default] MovieClip 'ProbeClip' /root/reading-a-file/work/many/5.0/assets/probe.png
LIST foreach[default] Object 'FluidObj' /root/reading-a-file/work/many/5.0/assets/fluidcache/
LIST foreach[default] Object 'ModObj' /root/reading-a-file/work/many/5.0/assets/meshcache.mdd
LIST foreach[default] Object 'ModObj' /root/reading-a-file/work/many/5.0/assets/oceanbake/
LIST foreach[default] Scene 'Scene' /root/reading-a-file/work/many/5.0/assets/stripdir/probe.png
LIST foreach[default] ShaderNodeTree 'Shader Nodetree' /root/reading-a-file/work/many/5.0/assets/probe.ies
LIST foreach[default] ShaderNodeTree 'Shader Nodetree' /root/reading-a-file/work/many/5.0/assets/probe.osl
LIST foreach[default] Sound 'ProbeSound' /root/reading-a-file/work/many/5.0/assets/probe.wav
LIST foreach[default] Text 'ProbeText' /root/reading-a-file/work/many/5.0/assets/probe.txt
LIST foreach[default] VectorFont 'ProbeFont' /root/reading-a-file/work/many/5.0/assets/probe.woff2
LIST foreach[default] Volume 'ProbeVolume' /root/reading-a-file/work/many/5.0/assets/probe.vdb
LIST foreach[default] COUNT 15
LIST foreach[flags=set()] CacheFile 'probe.abc' /root/reading-a-file/work/many/5.0/assets/probe.abc
LIST foreach[flags=set()] Camera 'ProbeCam' /root/reading-a-file/work/many/5.0/assets/camshader.osl
LIST foreach[flags=set()] Image 'ProbeImage' /root/reading-a-file/work/many/5.0/assets/probe.png
LIST foreach[flags=set()] Library 'lib.blend' /root/reading-a-file/work/many/5.0/lib.blend
LIST foreach[flags=set()] Material 'Material' /opt/blender/5.0/5.0/datafiles/assets/brushes/essentials_brushes-gp_draw.blend
LIST foreach[flags=set()] MovieClip 'ProbeClip' /root/reading-a-file/work/many/5.0/assets/probe.png
LIST foreach[flags=set()] Object 'FluidObj' /root/reading-a-file/work/many/5.0/assets/fluidcache/
LIST foreach[flags=set()] Object 'ModObj' /root/reading-a-file/work/many/5.0/assets/meshcache.mdd
LIST foreach[flags=set()] Object 'ModObj' /root/reading-a-file/work/many/5.0/assets/oceanbake/
LIST foreach[flags=set()] Scene 'Scene' /root/reading-a-file/work/many/5.0/assets/stripdir/probe.png
LIST foreach[flags=set()] ShaderNodeTree 'Shader Nodetree' /root/reading-a-file/work/many/5.0/assets/probe.ies
LIST foreach[flags=set()] ShaderNodeTree 'Shader Nodetree' /root/reading-a-file/work/many/5.0/assets/probe.osl
LIST foreach[flags=set()] Sound 'ProbeSound' /root/reading-a-file/work/many/5.0/assets/probe.wav
LIST foreach[flags=set()] Text 'ProbeText' /root/reading-a-file/work/many/5.0/assets/probe.txt
LIST foreach[flags=set()] VectorFont 'ProbeFont' /root/reading-a-file/work/many/5.0/assets/probe.woff2
LIST foreach[flags=set()] Volume 'ProbeVolume' /root/reading-a-file/work/many/5.0/assets/probe.vdb
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
AFTER | RenderSettings.filepath                  | /root/reading-a-file/work/many/5.0/assets/render_out/
AFTER | BakeSettings.filepath                    | /root/reading-a-file/work/many/5.0/assets/bake_out/
AFTER | CompositorNodeOutputFile base path       | /root/reading-a-file/work/many/5.0/assets/node_out/
AFTER | PointCache.filepath (cloth)              | /root/reading-a-file/work/many/5.0/assets/pointcache/
AFTER | FluidDomainSettings.cache_directory      | //assets/fluidcache/
AFTER | NodesModifier.bake_directory             | /root/reading-a-file/work/many/5.0/assets/gnbake/
AFTER | MultiresModifier.filepath                | 
AFTER | MeshCacheModifier.filepath               | //assets/meshcache.mdd
AFTER | OceanModifier.filepath                   | //assets/oceanbake/
AFTER | ShaderNodeScript.filepath                | //assets/probe.osl
AFTER | ShaderNodeTexIES.filepath                | //assets/probe.ies
AFTER | Camera.custom_filepath                   | //assets/camshader.osl
AFTER | custom property on an object             | /root/reading-a-file/work/many/5.0/assets/custom_prop.png
AFTER | ImageStrip.directory                     | //assets/stripdir/
AFTER | StripProxy.directory (sequencer)         | /root/reading-a-file/work/many/5.0/assets/proxydir/
```

Reading that off, without adding anything to it: the ones whose recorded text changed were **CacheFile.filepath, Camera.custom_filepath, FluidDomainSettings.cache_directory, Image.filepath, Image.filepath_raw, ImageStrip.directory, Library.filepath, MeshCacheModifier.filepath, MovieClip.filepath, OceanModifier.filepath, ShaderNodeScript.filepath, ShaderNodeTexIES.filepath, Sound.filepath, Text.filepath, VectorFont.filepath, Volume.filepath**. The ones whose recorded text is the same after as before were **BakeSettings.filepath, CompositorNodeOutputFile base path, NodesModifier.bake_directory, PointCache.filepath (cloth), RenderSettings.filepath, StripProxy.directory (sequencer), custom property on an object**. (The multi-resolution modifier's line is empty in both and is not counted either way, for the reason given above.)

#### What Make Paths Absolute reaches (list entry 5)

The file the last test saved, opened again, and `bpy.ops.file.make_paths_absolute()` called on it:

```
OP make_paths_absolute = {'FINISHED'}
AFTER | Image.filepath                           | /root/reading-a-file/work/many/5.0/assets/probe.png
AFTER | Image.filepath_raw                       | /root/reading-a-file/work/many/5.0/assets/probe.png
AFTER | Sound.filepath                           | /root/reading-a-file/work/many/5.0/assets/probe.wav
AFTER | VectorFont.filepath                      | /root/reading-a-file/work/many/5.0/assets/probe.woff2
AFTER | Text.filepath                            | /root/reading-a-file/work/many/5.0/assets/probe.txt
AFTER | Volume.filepath                          | /root/reading-a-file/work/many/5.0/assets/probe.vdb
AFTER | MovieClip.filepath                       | /root/reading-a-file/work/many/5.0/assets/probe.png
AFTER | CacheFile.filepath                       | /root/reading-a-file/work/many/5.0/assets/probe.abc
AFTER | Library.filepath                         | /root/reading-a-file/work/many/5.0/lib.blend
AFTER | RenderSettings.filepath                  | /root/reading-a-file/work/many/5.0/assets/render_out/
AFTER | BakeSettings.filepath                    | /root/reading-a-file/work/many/5.0/assets/bake_out/
AFTER | CompositorNodeOutputFile base path       | /root/reading-a-file/work/many/5.0/assets/node_out/
AFTER | PointCache.filepath (cloth)              | /root/reading-a-file/work/many/5.0/assets/pointcache/
AFTER | FluidDomainSettings.cache_directory      | /root/reading-a-file/work/many/5.0/assets/fluidcache/
AFTER | NodesModifier.bake_directory             | /root/reading-a-file/work/many/5.0/assets/gnbake/
AFTER | MultiresModifier.filepath                | 
AFTER | MeshCacheModifier.filepath               | /root/reading-a-file/work/many/5.0/assets/meshcache.mdd
AFTER | OceanModifier.filepath                   | /root/reading-a-file/work/many/5.0/assets/oceanbake/
AFTER | ShaderNodeScript.filepath                | /root/reading-a-file/work/many/5.0/assets/probe.osl
AFTER | ShaderNodeTexIES.filepath                | /root/reading-a-file/work/many/5.0/assets/probe.ies
AFTER | Camera.custom_filepath                   | /root/reading-a-file/work/many/5.0/assets/camshader.osl
AFTER | custom property on an object             | /root/reading-a-file/work/many/5.0/assets/custom_prop.png
AFTER | ImageStrip.directory                     | /root/reading-a-file/work/many/5.0/assets/stripdir/
AFTER | StripProxy.directory (sequencer)         | /root/reading-a-file/work/many/5.0/assets/proxydir/
```

#### The one-pass path visitor, and whether returning a string rewrites (list entry 6)

The same file, with `bpy.data.file_path_foreach` called with its flags emptied and a function that returns each path with `/assets/` replaced by `/MOVED/`. What was visited, and what the twenty-one kinds read afterwards:

```
FEW returned = None
FEW visited ('CacheFile', 'probe.abc', '/root/reading-a-file/work/many/5.0/assets/probe.abc', '/root/reading-a-file/work/many/5.0/MOVED/probe.abc', None)
FEW visited ('Camera', 'ProbeCam', '/root/reading-a-file/work/many/5.0/assets/camshader.osl', '/root/reading-a-file/work/many/5.0/MOVED/camshader.osl', None)
FEW visited ('Image', 'ProbeImage', '/root/reading-a-file/work/many/5.0/assets/probe.png', '/root/reading-a-file/work/many/5.0/MOVED/probe.png', None)
FEW visited ('Library', 'lib.blend', '/root/reading-a-file/work/many/5.0/lib.blend', '/root/reading-a-file/work/many/5.0/lib.blend', None)
FEW visited ('Material', 'Material', '/opt/blender/5.0/5.0/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/opt/blender/5.0/5.0/datafiles/MOVED/brushes/essentials_brushes-gp_draw.blend', None)
FEW visited ('MovieClip', 'ProbeClip', '/root/reading-a-file/work/many/5.0/assets/probe.png', '/root/reading-a-file/work/many/5.0/MOVED/probe.png', None)
FEW visited ('Object', 'FluidObj', '/root/reading-a-file/work/many/5.0/assets/fluidcache/', '/root/reading-a-file/work/many/5.0/MOVED/fluidcache/', None)
FEW visited ('Object', 'ModObj', '/root/reading-a-file/work/many/5.0/assets/meshcache.mdd', '/root/reading-a-file/work/many/5.0/MOVED/meshcache.mdd', None)
FEW visited ('Object', 'ModObj', '/root/reading-a-file/work/many/5.0/assets/oceanbake/', '/root/reading-a-file/work/many/5.0/MOVED/oceanbake/', None)
FEW visited ('Scene', 'Scene', '/root/reading-a-file/work/many/5.0/assets/stripdir/probe.png', '/root/reading-a-file/work/many/5.0/MOVED/stripdir/probe.png', None)
FEW visited ('ShaderNodeTree', 'Shader Nodetree', '/root/reading-a-file/work/many/5.0/assets/probe.ies', '/root/reading-a-file/work/many/5.0/MOVED/probe.ies', None)
FEW visited ('ShaderNodeTree', 'Shader Nodetree', '/root/reading-a-file/work/many/5.0/assets/probe.osl', '/root/reading-a-file/work/many/5.0/MOVED/probe.osl', None)
FEW visited ('Sound', 'ProbeSound', '/root/reading-a-file/work/many/5.0/assets/probe.wav', '/root/reading-a-file/work/many/5.0/MOVED/probe.wav', None)
FEW visited ('Text', 'ProbeText', '/root/reading-a-file/work/many/5.0/assets/probe.txt', '/root/reading-a-file/work/many/5.0/MOVED/probe.txt', None)
FEW visited ('VectorFont', 'ProbeFont', '/root/reading-a-file/work/many/5.0/assets/probe.woff2', '/root/reading-a-file/work/many/5.0/MOVED/probe.woff2', None)
FEW visited ('Volume', 'ProbeVolume', '/root/reading-a-file/work/many/5.0/assets/probe.vdb', '/root/reading-a-file/work/many/5.0/MOVED/probe.vdb', None)
```

```
FEW AFTER | Image.filepath                           | /root/reading-a-file/work/many/5.0/MOVED/probe.png
FEW AFTER | Image.filepath_raw                       | /root/reading-a-file/work/many/5.0/MOVED/probe.png
FEW AFTER | Sound.filepath                           | /root/reading-a-file/work/many/5.0/MOVED/probe.wav
FEW AFTER | VectorFont.filepath                      | /root/reading-a-file/work/many/5.0/MOVED/probe.woff2
FEW AFTER | Text.filepath                            | /root/reading-a-file/work/many/5.0/MOVED/probe.txt
FEW AFTER | Volume.filepath                          | /root/reading-a-file/work/many/5.0/MOVED/probe.vdb
FEW AFTER | MovieClip.filepath                       | /root/reading-a-file/work/many/5.0/MOVED/probe.png
FEW AFTER | CacheFile.filepath                       | /root/reading-a-file/work/many/5.0/MOVED/probe.abc
FEW AFTER | Library.filepath                         | /root/reading-a-file/work/many/5.0/lib.blend
FEW AFTER | RenderSettings.filepath                  | /root/reading-a-file/work/many/5.0/assets/render_out/
FEW AFTER | BakeSettings.filepath                    | /root/reading-a-file/work/many/5.0/assets/bake_out/
FEW AFTER | CompositorNodeOutputFile base path       | /root/reading-a-file/work/many/5.0/assets/node_out/
FEW AFTER | PointCache.filepath (cloth)              | /root/reading-a-file/work/many/5.0/assets/pointcache/
FEW AFTER | FluidDomainSettings.cache_directory      | /root/reading-a-file/work/many/5.0/MOVED/fluidcache/
FEW AFTER | NodesModifier.bake_directory             | /root/reading-a-file/work/many/5.0/assets/gnbake/
FEW AFTER | MultiresModifier.filepath                | 
FEW AFTER | MeshCacheModifier.filepath               | /root/reading-a-file/work/many/5.0/MOVED/meshcache.mdd
FEW AFTER | OceanModifier.filepath                   | /root/reading-a-file/work/many/5.0/MOVED/oceanbake/
FEW AFTER | ShaderNodeScript.filepath                | /root/reading-a-file/work/many/5.0/MOVED/probe.osl
FEW AFTER | ShaderNodeTexIES.filepath                | /root/reading-a-file/work/many/5.0/MOVED/probe.ies
FEW AFTER | Camera.custom_filepath                   | /root/reading-a-file/work/many/5.0/MOVED/camshader.osl
FEW AFTER | custom property on an object             | /root/reading-a-file/work/many/5.0/assets/custom_prop.png
FEW AFTER | ImageStrip.directory                     | /root/reading-a-file/work/many/5.0/MOVED/stripdir/
FEW AFTER | StripProxy.directory (sequencer)         | /root/reading-a-file/work/many/5.0/assets/proxydir/
```

#### Whether walking the property marks finds every recorded location (list entry 7)

A walk over thirteen of `bpy.data`'s collections, four levels deep, collecting every text setting whose subtype is `FILE_PATH` or `DIR_PATH` or which carries one of the two path marks. The walk found the same distinct values in every version. The counts it printed here:

```
SWEEP COUNT 97
SWEEP distinct values 22
```

and the distinct values it reached, one line each, showing where the walk found them. The last two or three lines are settings that carry a path mark but hold something that is not a path — a data-block name, or the bare file-name part of an output setting — and the custom property is on the end, found only by listing custom properties by hand and not by the walk at all:

```
SWEEP bpy.data.materials['Material'].library_weak_reference.filepath = /opt/blender/5.0/5.0/datafiles/assets/brushes/essentials_brushes-gp_draw.blend
SWEEP bpy.data.scenes['Scene'].render.bake.filepath = /root/reading-a-file/work/many/5.0/assets/bake_out/
SWEEP bpy.data.cameras['ProbeCam'].custom_filepath = /root/reading-a-file/work/many/5.0/assets/camshader.osl
SWEEP custom property bpy.data.objects['ModObj']['probe_path'] = /root/reading-a-file/work/many/5.0/assets/custom_prop.png
SWEEP bpy.data.objects['FluidObj'].modifiers[].domain_settings.cache_directory = /root/reading-a-file/work/many/5.0/assets/fluidcache/
SWEEP bpy.data.objects['GNObj'].modifiers[].bake_directory = /root/reading-a-file/work/many/5.0/assets/gnbake/
SWEEP bpy.data.objects['ModObj'].modifiers[].filepath = /root/reading-a-file/work/many/5.0/assets/meshcache.mdd
SWEEP bpy.data.node_groups['ProbeComp'].nodes[].directory = /root/reading-a-file/work/many/5.0/assets/node_out/
SWEEP bpy.data.objects['ModObj'].modifiers[].filepath = /root/reading-a-file/work/many/5.0/assets/oceanbake/
SWEEP bpy.data.objects['ClothObj'].modifiers[].point_cache.filepath = /root/reading-a-file/work/many/5.0/assets/pointcache/
SWEEP bpy.data.cache_files['probe.abc'].filepath = /root/reading-a-file/work/many/5.0/assets/probe.abc
SWEEP bpy.data.materials['ProbeMat'].node_tree.nodes[].filepath = /root/reading-a-file/work/many/5.0/assets/probe.ies
SWEEP bpy.data.materials['ProbeMat'].node_tree.nodes[].filepath = /root/reading-a-file/work/many/5.0/assets/probe.osl
SWEEP bpy.data.images['ProbeImage'].filepath = /root/reading-a-file/work/many/5.0/assets/probe.png
SWEEP bpy.data.volumes['ProbeVolume'].filepath = /root/reading-a-file/work/many/5.0/assets/probe.vdb
SWEEP bpy.data.sounds['ProbeSound'].filepath = /root/reading-a-file/work/many/5.0/assets/probe.wav
SWEEP bpy.data.fonts['ProbeFont'].filepath = /root/reading-a-file/work/many/5.0/assets/probe.woff2
SWEEP bpy.data.scenes['Scene'].sequence_editor.proxy_dir = /root/reading-a-file/work/many/5.0/assets/proxydir/
SWEEP bpy.data.scenes['Scene'].render.filepath = /root/reading-a-file/work/many/5.0/assets/render_out/
SWEEP bpy.data.scenes['Scene'].sequence_editor.strips[].directory = /root/reading-a-file/work/many/5.0/assets/stripdir/
SWEEP bpy.data.libraries['lib.blend'].filepath = /root/reading-a-file/work/many/5.0/lib.blend
SWEEP bpy.data.materials['Material'].library_weak_reference.id_name = MAMaterial
SWEEP bpy.data.node_groups['ProbeComp'].nodes[].file_name = file_name
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
PACK PackedImage.filepath = /root/reading-a-file/work/packed/5.0/assets/packed.png
PACK PlainImage.filepath  = /root/reading-a-file/work/packed/5.0/assets/plain.png
PACK blend_paths()            = ['/opt/blender/5.0/5.0/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/opt/blender/5.0/5.0/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/root/reading-a-file/work/packed/5.0/assets/plain.png']
PACK blend_paths(packed=True) = ['/opt/blender/5.0/5.0/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/opt/blender/5.0/5.0/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/root/reading-a-file/work/packed/5.0/assets/packed.png', '/root/reading-a-file/work/packed/5.0/assets/plain.png']
PACK file_path_map()          = [('PlainImage', ['/root/reading-a-file/work/packed/5.0/assets/plain.png'])]
PACK foreach[default] = ["Image 'PlainImage' /root/reading-a-file/work/packed/5.0/assets/plain.png"]
PACK foreach[flags=set()] = ["Image 'PackedImage' /root/reading-a-file/work/packed/5.0/assets/packed.png", "Image 'PlainImage' /root/reading-a-file/work/packed/5.0/assets/plain.png", "Material 'Dots Stroke' /opt/blender/5.0/5.0/datafiles/assets/brushes/essentials_brushes-gp_draw.blend", "Material 'Material' /opt/blender/5.0/5.0/datafiles/assets/brushes/essentials_brushes-gp_draw.blend"]
PACK foreach[SKIP_PACKED] = ["Image 'PlainImage' /root/reading-a-file/work/packed/5.0/assets/plain.png", "Material 'Dots Stroke' /opt/blender/5.0/5.0/datafiles/assets/brushes/essentials_brushes-gp_draw.blend", "Material 'Material' /opt/blender/5.0/5.0/datafiles/assets/brushes/essentials_brushes-gp_draw.blend"]
```

#### Whether unpacking rewrites the recorded location (list entry 10)

Two copies of that file, unpacked two ways, so neither run could affect the other. The `on disk` listing of the second run shows the folder the first run made; the two runs used different copies of the file and the data did not cross.

```
--- unpack USE_LOCAL ---
UNPACK before, filepath = /root/reading-a-file/work/packed/5.0/assets/packed.png | packed = True
UNPACK op(USE_LOCAL) = {'FINISHED'}
UNPACK after,  filepath = //textures/packed.png | packed = False
UNPACK on disk: <W>/packed.blend
UNPACK on disk: <W>/u1.blend
UNPACK on disk: <W>/u2.blend
UNPACK on disk: <W>/assets/packed.png
UNPACK on disk: <W>/assets/plain.png
UNPACK on disk: <W>/textures/packed.png
--- unpack USE_ORIGINAL ---
UNPACK before, filepath = /root/reading-a-file/work/packed/5.0/assets/packed.png | packed = True
UNPACK op(USE_ORIGINAL) = {'FINISHED'}
UNPACK after,  filepath = /root/reading-a-file/work/packed/5.0/assets/packed.png | packed = False
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
FINDALL built FromOne = /root/reading-a-file/work/findall/5.0/one/same.png
FINDALL built FromTwo = /root/reading-a-file/work/findall/5.0/two/same.png
FINDALL save = {'FINISHED'}
FINDALL before FromOne = /root/reading-a-file/work/findall/5.0/one/same.png True
FINDALL before FromTwo = /root/reading-a-file/work/findall/5.0/two/same.png True
FINDALL op = {'FINISHED'}
FINDALL after  FromOne = /root/reading-a-file/work/findall/5.0/one/same.png
FINDALL after  FromTwo = /root/reading-a-file/work/findall/5.0/one/same.png
```

#### A path holding a template, through the listing means and both sweeps (list entry 12)

```
TMPL save = {'FINISHED'}
TMPL OPENED   render.filepath = /root/reading-a-file/work/template/5.0/assets/out_{blend_name}_####.png
TMPL OPENED   node base path  = /root/reading-a-file/work/template/5.0/assets/node_{blend_name}/
TMPL OPENED   image           = /root/reading-a-file/work/template/5.0/assets/probe.png
TMPL OPENED   blend_paths     = ['/opt/blender/5.0/5.0/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/root/reading-a-file/work/template/5.0/assets/probe.png']
TMPL OPENED   file_path_map   = [['/root/reading-a-file/work/template/5.0/assets/probe.png']]
TMPL OPENED   foreach         = ['/root/reading-a-file/work/template/5.0/assets/probe.png']
TMPL OPENED   frame_path(1)   = /root/reading-a-file/work/template/5.0/assets/out_tmpl_0001.png
TMPL op make_paths_relative = {'FINISHED'}
TMPL REL      render.filepath = /root/reading-a-file/work/template/5.0/assets/out_{blend_name}_####.png
TMPL REL      node base path  = /root/reading-a-file/work/template/5.0/assets/node_{blend_name}/
TMPL REL      image           = //assets/probe.png
TMPL REL      blend_paths     = ['//../../../../../opt/blender/5.0/5.0/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '//assets/probe.png']
TMPL REL      file_path_map   = [['//assets/probe.png']]
TMPL REL      foreach         = ['//assets/probe.png']
TMPL REL      frame_path(1)   = /root/reading-a-file/work/template/5.0/assets/out_tmpl_0001.png
TMPL op make_paths_absolute = {'FINISHED'}
TMPL ABS      render.filepath = /root/reading-a-file/work/template/5.0/assets/out_{blend_name}_####.png
TMPL ABS      node base path  = /root/reading-a-file/work/template/5.0/assets/node_{blend_name}/
TMPL ABS      image           = /root/reading-a-file/work/template/5.0/assets/probe.png
TMPL ABS      blend_paths     = ['/opt/blender/5.0/5.0/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/root/reading-a-file/work/template/5.0/assets/probe.png']
TMPL ABS      file_path_map   = [['/root/reading-a-file/work/template/5.0/assets/probe.png']]
TMPL ABS      foreach         = ['/root/reading-a-file/work/template/5.0/assets/probe.png']
TMPL ABS      frame_path(1)   = /root/reading-a-file/work/template/5.0/assets/out_tmpl_0001.png
```

#### The four remap choices on the write call (list entry 13)

A file holding one image recorded relative and one recorded in full form, both naming files that exist. Each of the four choices wrote a new file one folder below. Each written file was then **opened** rather than linked from, because linking turns a recorded relative path into a full one on the way in and would hide the thing being asked about. Each line gives the stored text, what it works out to, and whether a file is there:

```
WREAD NONE.blend = [('AbsImage', ('/root/reading-a-file/work/write/5.0/assets/abs.png', '/root/reading-a-file/work/write/5.0/assets/abs.png', True)), ('RelImage', ('//assets/rel.png', '/root/reading-a-file/work/write/5.0/out/assets/rel.png', False))]
WREAD RELATIVE.blend = [('AbsImage', ('/root/reading-a-file/work/write/5.0/assets/abs.png', '/root/reading-a-file/work/write/5.0/assets/abs.png', True)), ('RelImage', ('//../assets/rel.png', '/root/reading-a-file/work/write/5.0/out/../assets/rel.png', True))]
WREAD RELATIVE_ALL.blend = [('AbsImage', ('//../assets/abs.png', '/root/reading-a-file/work/write/5.0/out/../assets/abs.png', True)), ('RelImage', ('//assets/rel.png', '/root/reading-a-file/work/write/5.0/out/assets/rel.png', False))]
WREAD ABSOLUTE.blend = [('AbsImage', ('/root/reading-a-file/work/write/5.0/assets/abs.png', '/root/reading-a-file/work/write/5.0/assets/abs.png', True)), ('RelImage', ('/root/reading-a-file/work/write/5.0/assets/rel.png', '/root/reading-a-file/work/write/5.0/assets/rel.png', True))]
```

#### Whether the write call may be pointed at a file that already exists (list entry 14)

```
WRITE existing file size = 27
WRITE onto existing file = no error raised
WRITE existing file size before = 27 after = 137245
WRITE existing file first bytes after = b'BLENDER17-01'
```

#### What Save As with Remap Relative moves (list entry 15)

```
SAVEAS save2 = {'FINISHED'}
SAVEAS OPENED   RelImage = //assets/rel.png
SAVEAS OPENED   AbsImage = /root/reading-a-file/work/saveas/5.0/assets/abs.png
SAVEAS OPENED   render   = //renders/
SAVEAS op = {'FINISHED'}
SAVEAS AFTER    RelImage = //../assets/rel.png
SAVEAS AFTER    AbsImage = /root/reading-a-file/work/saveas/5.0/assets/abs.png
SAVEAS AFTER    render   = //renders/
SAVEAS new file location = /root/reading-a-file/work/saveas/5.0/elsewhere/moved.blend
```

#### Writing the library's own path, against relocating it (list entry 16)

Three files: `B.blend` holding an object called `CubeFromB`, `C.blend` holding one called `CubeFromC`, and `A.blend` linking `B`'s collection. Two separate runs from the same `A.blend` — the first assigns the path and then calls `reload()`, the second calls the relocate operator:

```
RELOC OPENED   library = /root/reading-a-file/work/reloc/5.0/B.blend
RELOC OPENED   objects = [('Camera', None), ('Cube', None), ('CubeFromB', '/root/reading-a-file/work/reloc/5.0/B.blend'), ('Light', None)]
RELOC ASSIGNED library = /root/reading-a-file/work/reloc/5.0/C.blend
RELOC ASSIGNED objects = [('Camera', None), ('Cube', None), ('CubeFromB', '/root/reading-a-file/work/reloc/5.0/C.blend'), ('Light', None)]
RELOC reload() = returned
RELOC RELOADED library = /root/reading-a-file/work/reloc/5.0/C.blend
RELOC RELOADED objects = [('Camera', None), ('Cube', None), ('CubeFromC', '/root/reading-a-file/work/reloc/5.0/C.blend'), ('Light', None)]
RELOC OPENED   library = /root/reading-a-file/work/reloc/5.0/B.blend
RELOC OPENED   objects = [('Camera', None), ('Cube', None), ('CubeFromB', '/root/reading-a-file/work/reloc/5.0/B.blend'), ('Light', None)]
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
NOSAVE rel    image before = /root/reading-a-file/work/many/5.0/assets/probe.png
NOSAVE rel    render before = /root/reading-a-file/work/many/5.0/assets/render_out/
NOSAVE op = {'FINISHED'}
NOSAVE after, not saved, image = //../../many/5.0/assets/probe.png
RUN 2 --- the same file opened again
NOSAVE check  image before = /root/reading-a-file/work/many/5.0/assets/probe.png
NOSAVE check  render before = /root/reading-a-file/work/many/5.0/assets/render_out/
RUN 3 --- the same file with -o //NOSAVE_CLI/ and no save
NOSAVE check  image before = /root/reading-a-file/work/many/5.0/assets/probe.png
NOSAVE check  render before = //NOSAVE_CLI/
RUN 4 --- the same file opened again, no argument
NOSAVE check  image before = /root/reading-a-file/work/many/5.0/assets/probe.png
NOSAVE check  render before = /root/reading-a-file/work/many/5.0/assets/render_out/
```

#### Whether a bulk rewrite reaches the file on disk without a save (list entry 18)

Runs 1 and 2 of the four just above are this test. Run 1 shows the rewrite taking hold in the running Blender — the `after, not saved` line — and run 2 shows what the file on disk still holds when it is opened again.

#### Turning a stored path into a real one, and the library it came from (list entry 19)

A library in its own folder holding an image recorded as `//libtex.png`, and a file one folder up that links it. The two folders differ on purpose, or the two calls would have no reason to differ:

```
AP own stored          = //assets/own.png
AP own abspath()       = /root/reading-a-file/work/abs/5.0/assets/own.png
AP own relpath(abs)    = //assets/own.png
AP own file exists     = True
AP linked name         = LibTex library = /root/reading-a-file/work/abs/5.0/libdir/lib.blend
AP linked stored       = //libtex.png
AP linked abspath()    = /root/reading-a-file/work/abs/5.0/libtex.png
AP linked abspath(lib) = /root/reading-a-file/work/abs/5.0/libdir/libtex.png
AP linked exists plain = False
AP linked exists lib   = True
```

#### A reference held only inside a node group, a driver or a custom property (list entry 20)

This one answers question 7 of the sheet rather than question 9, and is written into `Preparation/Blender Research/Findings/5.0/Reading a file.md` as well. Four images: one used straight from a material, one used only by an image node inside a node group, one named only as a driver's target, one held only by a custom property that points at the data-block:

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
UM file_path_map = [('Img_custom', ['/root/reading-a-file/work/um/5.0/assets/custom.png']), ('Img_driver', ['/root/reading-a-file/work/um/5.0/assets/driver.png']), ('Img_group', ['/root/reading-a-file/work/um/5.0/assets/group.png']), ('Img_plain', ['/root/reading-a-file/work/um/5.0/assets/plain.png'])]
```

#### What could not be done here (list entries 21 and 22, and the cannot-be-run section)

**Replacing the file an image or a strip is loaded from.** Both operators refuse to run with no window on this machine, when called as they are called here. That refusal is itself the result, and it is what makes this a Mac entry rather than a Linux one. Read it with the Mac section below, which asked the same two operators again: one of them wanted a window and the other wanted the file named and the sequencer's own scene put into the context, and once given those it ran with no window.

```
REPL before image = /root/reading-a-file/work/replace/5.0/first/a.png
REPL op image.replace = ERROR RuntimeError Operator bpy.ops.image.replace.poll() failed, context is incorrect
REPL after  image = /root/reading-a-file/work/replace/5.0/first/a.png
REPL before strip = /root/reading-a-file/work/replace/5.0/first/ | ['a.png']
REPL op sequencer.change_path = ERROR RuntimeError Operator bpy.ops.sequencer.change_path.poll() failed, context is incorrect
REPL after  strip = /root/reading-a-file/work/replace/5.0/first/ | ['a.png']
```

**The Outliner's Data API view** cannot be reached here at all — the machine has no display server. It was reached on the Creator's Mac on 2026-08-07; the pictures and what they show are in the Mac section below.

**Anything on Windows.** No Windows machine has run it, so whether making paths relative works across two drives on Windows was not reached by any test here. That is why it is unanswered rather than untried. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

**What Blender Asset Tracer reaches.** It is not part of Blender and is not in `Blender Sources/`; reaching it would mean fetching software from outside the project. Not attempted, and recorded as out of scope.

**The multi-resolution modifier's own path.** The property would not take a value in any version, so no test here could act on it. Nothing above says whether it is reached; only that a path could not be put there to find out.

---

### Observed — the Mac half of the hand tests, 2026-08-07

Everything in this section was run on **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card**. The Blender was `/Applications/Blender 5.0.1.app/Contents/MacOS/Blender`, which reported itself as **5.0.1**, hash `a3db93c5b259` — the same version string and the same hash the rented Linux machine reported for its own copy of this version. Every run used `--factory-startup`, and every run had `BLENDER_USER_RESOURCES` pointed at `$T/res-5.0`, a settings folder made for this sitting before it was used and for nothing else, never at the real one. Below, `$T` stands for `/private/tmp/smoozy-test/raf`, a throwaway folder outside the project, and `$S` for `/Users/artist/Documents/Smoozy hand tests/Reading a file/added-questions/scripts`, where the scripts are kept. Only one Blender of this sitting ran at a time.

**Other topics were running Blender on this same Mac while this sitting ran**, so nothing below says how long anything took or how much memory anything used, and no test below measures either.

**The scripts were written again for this sitting.** The brief said not to touch the rented Linux machine at all, and the Linux half's scripts live there, so they were rebuilt from the procedure and the printouts recorded above. They were checked against the Linux printout before anything was read from them: the twenty-four readings of the many-paths file, and the map of data-blocks to paths, came back the same character for character. That is what makes the two lines that *do* differ readable as a difference between the machines rather than a difference between two scripts.

The full list of tests, with the set-up and the command for each, is `Audits/Blender Research/Reading a file/Hand tests — added questions.md`.

#### The same file's paths on the Mac, set beside the Linux printout (list entry 8)

The many-paths file was built again on the Mac by `$S/s_buildlib.py` and then `$S/s_build.py`, its paths were made relative by `$S/s_rel.py`, which answered `OP make_paths_relative = {'FINISHED'}`, and `$S/s_cross.py` read the result:

```
BLENDER_USER_RESOURCES="$T/res-5.0" "/Applications/Blender 5.0.1.app/Contents/MacOS/Blender" -b --factory-startup \
  "$T/work/many/5.0/many_rel.blend" --python "$S/s_cross.py"
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
CROSS blend_paths   = ['//../../../../../../../../../../Applications/Blender 5.0.1.app/Contents/Resources/5.0/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '//assets/camshader.osl', '//assets/fluidcache/', '//assets/meshcache.mdd', '//assets/oceanbake/', '//assets/probe.abc', '//assets/probe.ies', '//assets/probe.osl', '//assets/probe.png', '//assets/probe.png', '//assets/probe.txt', '//assets/probe.vdb', '//assets/probe.wav', '//assets/probe.woff2', '//assets/stripdir/probe.png', '//lib.blend']
CROSS file_path_map = [('FluidObj', ['//assets/fluidcache/']), ('ModObj', ['//assets/meshcache.mdd', '//assets/oceanbake/']), ('ProbeCam', ['//assets/camshader.osl']), ('ProbeClip', ['//assets/probe.png']), ('ProbeFont', ['//assets/probe.woff2']), ('ProbeImage', ['//assets/probe.png']), ('ProbeMat', ['//assets/probe.ies', '//assets/probe.osl']), ('ProbeSound', ['//assets/probe.wav']), ('ProbeText', ['//assets/probe.txt']), ('ProbeVolume', ['//assets/probe.vdb']), ('Scene', ['//assets/stripdir/probe.png']), ('lib.blend', ['//lib.blend']), ('probe.abc', ['//assets/probe.abc'])]
CROSS separator     = '/' | case check = True
```

Set beside the Linux printout for this same version, line by line: **twenty-five of the twenty-seven lines are identical, character for character.** That is all twenty-four readings, and the map of data-blocks to paths. **Two lines differ.**

**The flat path list.** The Mac's list holds sixteen entries; the Linux list holds fifteen. The fifteen they share are identical. The one the Mac has and Linux does not is a path into the Blender application's own folder, ending `datafiles/assets/brushes/essentials_brushes-gp_draw.blend`, recorded relative to the file. Two further readings taken on the Mac, recorded because they bound what that line can be read as. The same file **before** its paths were made relative lists that path on the Mac in full form, exactly as the Linux printout above lists its own copy of it — so the difference appears only after the sweep and the save. And after the sweep the material it hangs off reads:

```
WEAK 5.0 Material '//../../../../../../../../../../Applications/Blender 5.0.1.app/Contents/Resources/5.0/datafiles/assets/brushes/essentials_brushes-gp_draw.blend'
WEAK 5.0 ProbeMat None
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
CASE 5.0 asked for $T/work/many/5.0/assets/PROBE.PNG
CASE 5.0 exists     True
CASE 5.0 real name  ['probe.png']
```

**On this point the Mac's answer is the one to read, not the Linux one**: they are different answers about two different machines and neither speaks for the other. The Linux half's own script was not opened for this sitting, so whether it asked this in exactly these words is not established; the words above are the Mac's.

#### Replacing the file an image or a strip is loaded from, with a window (list entry 21)

The Linux half of this entry is recorded above as a refusal with no window. The Mac has a screen, so the file was built there with no window by `$S/s_replace.py` and then opened **with a window on screen**, and the two operators were called from a `--python` script through a context override. The script ends by stopping Blender itself, so nothing was left open on the Creator's desk.

```
BLENDER_USER_RESOURCES="$T/res-5.0" "/Applications/Blender 5.0.1.app/Contents/MacOS/Blender" -b --factory-startup \
  --python "$S/s_replace.py" -- build "$T/work/replace/5.0"
BLENDER_USER_RESOURCES="$T/res-5.0" "/Applications/Blender 5.0.1.app/Contents/MacOS/Blender" --factory-startup \
  "$T/work/replace/5.0/repl.blend" --python "$S/s_replace_window.py" -- "$T/work/replace/5.0"
```

The script puts `ProbeImage` in an Image editor and calls `bpy.ops.image.replace(filepath=<the second file>, relative_path=False)`; then it switches the area to the Sequence editor, selects the strip, and calls `bpy.ops.sequencer.change_path` three times — first exactly as the list wrote it, then with the file named as well, then with the sequencer's scene supplied in the context. All three calls and all three answers are printed. What came back:

```
REPL build save {'FINISHED'}
REPL window     = True
REPL version    = 5.0.1
REPL before image = /private/tmp/smoozy-test/raf/work/replace/5.0/first/a.png
REPL op image.replace = {'FINISHED'}
REPL after  image = /private/tmp/smoozy-test/raf/work/replace/5.0/second/b.png
REPL before strip = /private/tmp/smoozy-test/raf/work/replace/5.0/first/ | ['a.png']
REPL op sequencer.change_path (no files argument) = ERROR RuntimeError Operator bpy.ops.sequencer.change_path.poll() failed, context is incorrect
REPL after  strip, first call = /private/tmp/smoozy-test/raf/work/replace/5.0/first/ | ['a.png']
REPL op sequencer.change_path (files named) = ERROR RuntimeError Operator bpy.ops.sequencer.change_path.poll() failed, context is incorrect
REPL after  strip, second call = /private/tmp/smoozy-test/raf/work/replace/5.0/first/ | ['a.png']
REPL context has sequencer_scene = True
REPL op sequencer.change_path (sequencer scene supplied) = {'FINISHED'}
REPL after  strip, third call = /private/tmp/smoozy-test/raf/work/replace/5.0/second/ | ['b.png']
```

#### The same two operators with no window at all, on the Mac (list entry 23)

This one is not on the list as it was written; it was added because the run above showed the list had missed it. The Linux half concluded that both operators refuse to run with no window. But the windowed run above shows that what the sequencer operator wanted was the file named and the sequencer's own scene put into the context, neither of which the Linux run supplied. So the same two operators were asked again on the Mac **with no window at all**, and given both of those things:

```
BLENDER_USER_RESOURCES="$T/res-5.0" "/Applications/Blender 5.0.1.app/Contents/MacOS/Blender" -b --factory-startup \
  --python "$S/s_replace.py" -- build "$T/work/replacebg/5.0"
BLENDER_USER_RESOURCES="$T/res-5.0" "/Applications/Blender 5.0.1.app/Contents/MacOS/Blender" -b --factory-startup \
  "$T/work/replacebg/5.0/repl.blend" --python "$S/s_replace_nowindow.py" -- "$T/work/replacebg/5.0"
```

What came back:

```
REPLBG background = True
REPLBG version    = 5.0.1
REPLBG screen     = <bpy_struct, Screen("Layout") at 0x72953e820>
REPLBG areas      = 4
REPLBG before image = <W>/5.0/first/a.png
REPLBG op image.replace = ERROR RuntimeError Operator bpy.ops.image.replace.poll() failed, context is incorrect
REPLBG after  image = <W>/5.0/first/a.png
REPLBG before strip = <W>/5.0/first/ | ['a.png']
REPLBG op sequencer.change_path = ERROR RuntimeError Operator bpy.ops.sequencer.change_path.poll() failed, context is incorrect
REPLBG after  strip = <W>/5.0/first/ | ['a.png']
REPLBG op sequencer.change_path (sequencer scene supplied) = {'FINISHED'}
REPLBG after  strip, second call = <W>/5.0/second/ | ['b.png']
```

#### The Outliner's Data API view (list entry 22)

The Mac has a screen, so this could be reached there and nowhere else. `many_rel.blend` was opened with a window, the largest area was turned into the Outliner and put into its Data API view, the interface scale was set to 0.5 so that more rows would fit in one picture, and the area was photographed with Blender's own screenshot command. Nothing was clicked. A second run assigned a new path to the image while that same view was on screen, saved, reopened the file and read the value back.

```
BLENDER_USER_RESOURCES="$T/res-5.0" "/Applications/Blender 5.0.1.app/Contents/MacOS/Blender" --factory-startup --window-geometry 0 0 2600 1500 \
  "$T/work/many/5.0/many_rel.blend" --python "$S/s_outliner_window.py" -- shot "$T/out/outl/5.0-%s.png"
BLENDER_USER_RESOURCES="$T/res-5.0" "/Applications/Blender 5.0.1.app/Contents/MacOS/Blender" --factory-startup --window-geometry 0 0 2600 1500 \
  "$T/work/many/5.0/outl.blend" --python "$S/s_outliner_window.py" -- write "$T/out/outl/5.0-%s.png"
```

What came back:

```
OUTL version       = 5.0.1
OUTL background    = False
OUTL display_mode  = DATA_API
OUTL area          = 2477 x 1510
OUTL ui_scale asked for 0.5, got 0.5
OUTL screenshot_area two_levels = {'FINISHED'} 133342 bytes
OUTL screenshot_area four_levels = {'FINISHED'} 165810 bytes
OUTL view y before = -1458.0
OUTL outliner.scroll_page = {'FINISHED'} view y now -2916.0
OUTL screenshot_area after_scroll_page = {'FINISHED'} 135045 bytes
OUTL view2d.pan = {'FINISHED'} view y now -4316.0
OUTL screenshot_area after_pan = {'FINISHED'} 135685 bytes
OUTL filter_text set to 'Filepath'
OUTL screenshot_area after_filter = {'FINISHED'} 147286 bytes
OUTL version       = 5.0.1
OUTL background    = False
OUTL display_mode  = DATA_API
OUTL area          = 2477 x 1510
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

2026-08-06 — Opus 5 (Claude Code) — Reading a file, Blender 5.0

2026-08-07 — Opus 5 (Claude Code) — the Mac half of the hand tests

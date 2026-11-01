# Reading a file — added questions (Blender 4.5)

**Sources consulted, and when.** Blender 4.5 manual, Blender 4.5 Python API documentation, Blender 4.5 release notes, and the Cycles add-on's own files — all read on **2026-07-30** (the snapshot date given to me). One manual image was opened (`_images/files_blend_open-save_file-browser-save.png`), same date. I did not need the requirements page for this question and did not consult it.

I read only the question sheet and files inside `Blender Sources/`. I did not open any other part of this project.

One thing I must add so the line above is exact: to confirm the folder I was told to save into exists, I ran a directory listing on `Preparation/Blender Research/Findings/4.5/` and the names of the files already sitting there appeared on screen. I opened none of them, and I did not read past the first three names in the listing. No content from any of them entered this document.

Only question 9 was asked of me, and only question 9 is answered.

---

## 9. What separate means exist for listing and rewriting the locations recorded in a `.blend` file, and which kinds of recorded location does each of those means reach?

**How I read the question.** "Locations recorded in a `.blend` file" = the file paths stored inside the file — where it says its images, sounds, linked blend-files, caches, fonts, scripts and render outputs live. "Listing" = getting those paths out. "Rewriting" = changing them. There is a second possible reading — "locations" meaning where in the file's internal structure something sits — which I do not think is meant, and which I have not answered. Saying so rather than choosing silently.

**Audit note (Opus 5, 2026-08-06):** that second reading is a part of the question left unanswered, so it is now recorded under *Not covered* below rather than only mentioned here.

**LABEL: documented**, for the existence and stated reach of each means. Where I have gone one step past the words, I say so and re-label that step **inferred**. The Cycles finding is **observed**.

### The short answer

The sources describe **eight separate means**, and they do not all reach the same paths. Three sweep the whole file at once; five work on one thing at a time or at a chosen moment. No single one of them is documented as reaching every path the file can hold.

**Audit note (Opus 5, 2026-08-06):** the number eight is this researcher's own grouping, not something any source counts. Several of the eight hold more than one command, and the Outliner listing in section C is a ninth way of listing that the count leaves out. Read *eight* as how the answer below is arranged, not as a fact from the sources.

---

### A. Whole-file means

**A1. `bpy.utils.blend_paths()` — lists, never rewrites.**

> "Returns a list of paths to external files referenced by the loaded .blend file."
>
> "absolute (bool) – When true the paths returned are made absolute."
> "packed (bool) – When true skip file paths for packed data."
> "local (bool) – When true skip linked library paths."

Blender 4.5 Python API documentation, `https://docs.blender.org/api/4.5/bpy.utils.html`, read 2026-07-30.

Reach, as documented: "external files referenced by" the file. The `local` option tells you what is in the default result for one kind — **inferred** (one step: an option to *skip* linked-library paths only makes sense if they are in the list when you do not ask to skip them). So: linked library paths in.

**Citation check note (Opus 5, 2026-08-06):** the same step used to be taken for packed data as well, and it has been cut, because this document's own printout contradicts it. Under *The packed switch* below, `blend_paths()` came back **without** the packed image's path and `blend_paths(packed=True)` came back **with** it — so in this version the option adds a packed path rather than skipping one, whatever the word "skip" on the page says, and the default list does not hold it. What the packed option does is therefore left to the test and not read off the page. The wording says nothing about paths the file writes *to* rather than reads *from* — the render output path, the compositor File Output node's base path, cache directories. I cannot tell from these words whether those are included, and I am not going to guess.

That this one only lists and cannot rewrite — the "never rewrites" in the heading — is **inferred**, one step, from the words "Returns a list of paths". The quoted words do not say it changes nothing.

**A2. `File ▸ External Data ▸ Make Paths Relative` / `Make Paths Absolute` — rewrites in bulk, lists nothing.**

> "Make Paths Relative
> Make all paths to external files relative to the current blend-file.
>
> Make Paths Absolute
> Make all paths to external files absolute (= full path from the system's root)."

Blender 4.5 manual, `https://docs.blender.org/manual/en/4.5/interface/window_system/topbar.html`, read 2026-07-30.

The same two as Python operators:

> "bpy.ops.file.make_paths_absolute()
> Make all paths to external files absolute"
>
> "bpy.ops.file.make_paths_relative()
> Make all paths to external files relative to current .blend"

Blender 4.5 Python API documentation, `https://docs.blender.org/api/4.5/bpy.ops.file.html`, read 2026-07-30.

Reach: "all paths to external files". That this changes only the *form* of a path — relative or absolute — and not where it points, and so cannot repair a path aimed at the wrong place, is **inferred**, one step: "relative" and "absolute" describe how a path is written, and the quoted words say nothing about what these two commands leave alone.

**A3. `Report Missing Files` / `Find Missing Files` — one lists the broken ones, the other rewrites them.**

> "Report Missing Files
> This option is useful to check if there are links to unpacked files that no longer exist. After selecting this option, a warning message will appear in the Info editor's header. If no warning is shown, there are no missing external files.
>
> Find Missing Files
> In case you have broken links in a blend-file, this can help you to fix the problem. A File Browser will show up. Select the desired directory (or a file within that directory), and a search will be performed in it, recursively in all contained directories. Every missing file found in the search will be recovered. Those recoveries will be done as absolute paths, so if you want to have relative paths you will need to select Make Paths Relative."

Blender 4.5 manual, `https://docs.blender.org/manual/en/4.5/interface/window_system/topbar.html`, read 2026-07-30.

As Python operators:

> "bpy.ops.file.report_missing_files()
> Report all missing external files"
>
> "bpy.ops.file.find_missing_files(*, find_all=False, directory='', …)
> Try to find missing external files"
>
> "find_all (boolean, (optional)) – Find All, Find all files in the search path (not just missing)"
> "directory (string, (optional, never None)) – Directory, Directory of the file"

Blender 4.5 Python API documentation, `https://docs.blender.org/api/4.5/bpy.ops.file.html`, read 2026-07-30.

Reach: "missing external files" only — that paths which currently resolve are left alone is **inferred**, one step, from the word "missing". Unless `find_all` is turned on, which the wording says widens it to "all files in the search path (not just missing)". Report puts a warning in the Info editor's header; that it hands back nothing else — no list you can take away — is **inferred**, one step, from the manual describing only the warning. Find rewrites to absolute paths, and the manual says plainly you must run Make Paths Relative afterwards if you want relative ones. Whether either one reaches *linked library* paths — as opposed to images, sounds and the like — is not stated anywhere I found; the manual treats broken libraries separately, under Relocate (B1 below).

---

### B. Per-item and per-moment means

**B1. Library `Relocate` and `Reload` — linked blend-files only.**

> "When the Outliner is in the Blender File Display Mode, a right-click on a library (a linked blend-file) will show the Relocate option. It allows to replace it by a different file. This can be used to either fix a broken linked library (e.g. because the file was moved or renamed), or to switch to a variation of the same data in a different file."
>
> "Broken Libraries
> If Blender cannot find a library while loading a blend-file, it will create placeholder data-blocks to replace missing linked ones. That way, references to the missing data are not lost, and by relocating the missing library, the lost data can be automatically restored."
>
> "A right-click a linked ID will show the Relocate option in the ID Data sub-menu. It allows to relocate a directly linked ID by another one, from the same library or from a different one."

Blender 4.5 manual, `https://docs.blender.org/manual/en/4.5/files/linked_libraries/link_append.html`, read 2026-07-30.

As Python operators:

> "bpy.ops.wm.lib_relocate(*, library='', filepath='', directory='', filename='', files=None, …, relative_path=True, …)
> Relocate the given library to one or several others"
>
> "bpy.ops.wm.lib_reload(*, library='', filepath='', …, relative_path=True, …)
> Reload the given library"

Blender 4.5 Python API documentation, `https://docs.blender.org/api/4.5/bpy.ops.wm.html`, read 2026-07-30.

Reach: one library at a time, named by the `library` argument. Note `relative_path=True` in the signature — the replacement path can be recorded relative.

**Audit note (Opus 5, 2026-08-06):** the third quote above is not about a whole library — it says a linked *data-block* can be relocated on its own, from the same library or a different one. So the manual documents a per-data-block reach here that the reach line does not mention, and the "nothing else" that stood at the end of it has been cut. The operator paragraph below covers the same ground from the Python side and reports it as internal, which sits awkwardly beside a manual that describes it as a right-click. That tension is in the sources, not in this answer.

There is also an internal one for a single linked data-block, and the documentation warns you off it:

> "bpy.ops.wm.id_linked_relocate(…)
> Relocate a linked ID, i.e. select another ID to link, and remap its local usages to that newly linked data-block). Currently only designed as an internal operator, not directly exposed to the user"

Same page, same date.

**B2. The per-data-block path property, through Python — reads and writes one path at a time, and reaches the widest set of the means found here.**

Stored paths appear as named properties on data-blocks, and most of the ones found are both readable and writable. The documentation marks many such properties with the phrase "blend relative // prefix supported". Searching the whole 4.5 API documentation for that phrase turns up **41 pages** (corrected from 39 by the citation check, 2026-08-06), carrying these properties:

`BakeSettings.filepath`, `Brush.icon_filepath`, `CacheFile.filepath`, `CacheFileLayer.filepath`, `CollectionExport.filepath`, `CompositorNodeOutputFile.base_path`, `DynamicPaintSurface.image_output_path`, `FluidDomainSettings.cache_directory`, `Image.filepath`, `Image.filepath_raw`, `ImagePackedFile.filepath`, `ImageStrip.directory`, `Library.filepath`, `LibraryWeakReference.filepath`, `MeshCacheModifier.filepath`, `MovieClip.filepath`, `MovieClipProxy.directory`, `MovieStrip.filepath`, `MultiresModifier.filepath`, `NodeSocketStringFilePath.default_value`, `NodesModifier.bake_directory`, `NodesModifierBake.directory`, `OceanModifier.filepath`, `PointCache.filepath`, `PointCacheItem.filepath`, `RenderSettings.filepath`, `SequenceEditor.proxy_dir`, `ShaderNodeScript.filepath`, `ShaderNodeTexIES.filepath`, `Sound.filepath`, `StripProxy.directory`, `StripProxy.filepath`, `VectorFont.filepath`, `Volume.filepath`, `VolumeGrids.frame_filepath`, and five on `PreferencesFilePaths` (`font_directory`, `render_cache_directory`, `render_output_directory`, `sound_directory`, `texture_directory`).

Blender 4.5 Python API documentation, the `bpy.types.*` pages of those names, under `https://docs.blender.org/api/4.5/`, read 2026-07-30.

**Audit note (Opus 5, 2026-08-06), three things about that list.**

*The count does not add up.* ~~The search is reported as finding 39 pages. What is named above is 40 properties spread over 34 pages — `Image` and `StripProxy` each carry two, and `PreferencesFilePaths` is one page for five. So five pages the search found are not named anywhere, and nobody reading this can tell which. That search has been sent back to be run again by a fresh researcher, with every page named.~~

**Closed by the citation check (Opus 5, 2026-08-06), which ran the search again on the same snapshot.** The phrase appears on **41** pages of `api_4.5`, not 39. Seven of the 41 are not named above, and here they are: two operator pages, `bpy.ops.view3d` and `bpy.ops.workspace`, where the phrase sits on an operator argument rather than a stored path; and five collection pages whose `load()` methods carry it — `bpy.types.BlendDataFonts`, `BlendDataImages`, `BlendDataMovieClips`, `BlendDataSounds` and `BlendDataTexts`. That leaves 34 `bpy.types.*` pages of actual properties, which are exactly the 34 named above. So the list is complete for property-bearing type pages; only the count was wrong.

*The list is what one phrase finds, not every path a file holds.* The `Text` property further down carries no such marker and was only caught by chance. So the marker finds a set of paths; it is not established that it finds them all, and this means cannot be called the widest-reaching on this evidence alone. The original wording — that every kind of stored path is a named property, readable and writable — has been cut, because two things in this same section contradict it: one of the listed properties is read-only, and `Text` carries no marker.

*Readable and writable is not uniform.* One of the forty is marked read-only, as recorded below.

Two of these need care. The five `PreferencesFilePaths` ones are settings of the application, not of any `.blend` — **inferred**, one step, from the type being `Preferences…`. And `VolumeGrids.frame_filepath` is marked "(readonly)", so it lists but cannot be rewritten:

> "frame_filepath
> Volume file used for loading the volume at the current frame. Empty if the volume has not be loaded or the frame only exists in memory.
> Type:
> string, default "", (readonly, never None, blend relative // prefix supported)"

`https://docs.blender.org/api/4.5/bpy.types.VolumeGrids.html`, read 2026-07-30.

Sample wording for the ordinary, writable case:

> "filepath
> Image/Movie file name
> Type:
> string, default "", (never None, blend relative // prefix supported)"

`https://docs.blender.org/api/4.5/bpy.types.Image.html`, read 2026-07-30.

> "filepath
> Path to the library .blend file
> Type:
> string, default "", (never None, blend relative // prefix supported)"

`https://docs.blender.org/api/4.5/bpy.types.Library.html`, read 2026-07-30.

Two paths that matter here and that the whole-file sweeps do not obviously cover are on this list and are writable:

> "filepath
> Directory/name to save animations, # characters define the position and padding of frame numbers
> Type:
> string, default "", (never None, blend relative // prefix supported, Supports template expressions)"

`https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html`, read 2026-07-30.

> "base_path
> Base output path for the image
> Type:
> string, default "", (never None, blend relative // prefix supported, Supports template expressions)"

`https://docs.blender.org/api/4.5/bpy.types.CompositorNodeOutputFile.html`, read 2026-07-30.

One path is **not** marked blend-relative and so did not appear in the sweep above — the text data-block's:

> "filepath
> Filename of the text file
> Type:
> string, default "", (never None)"

`https://docs.blender.org/api/4.5/bpy.types.Text.html`, read 2026-07-30.

A trap the documentation calls out for this means specifically: a path read off a linked data-block is relative to *its own* library file, not to the open one.

> "When using Blender data from linked libraries there is an unfortunate complication since the path will be relative to the library rather than the open blend-file. When the data block may be from an external blend-file pass the library argument from the bpy.types.ID."
>
> ">>> bpy.path.abspath(image.filepath, library=image.library)"

Blender 4.5 Python API documentation, `https://docs.blender.org/api/4.5/info_gotchas_file_paths_and_encoding.html`, read 2026-07-30. The conversion helpers themselves — `bpy.path.abspath`, `bpy.path.relpath` and the rest — are documented at `https://docs.blender.org/api/4.5/bpy.path.html`, read 2026-07-30. They convert a path you hand them; they do not find or change anything stored in the file.

**B3. Saving elsewhere, with `Remap Relative` — rewrites relative paths at the moment of saving.**

> "Remap Relative
> This option remaps Relative Paths (such as linked libraries and images) when saving a file in a new location."

Blender 4.5 manual, `https://docs.blender.org/manual/en/4.5/files/blend/open_save.html`, read 2026-07-30. The same page again, as advice: "If it is necessary to relocate a blend-file relative to its linked resources, use Blender's File Save As function which has an option to Remap Relative file links."

As Python:

> "relative_remap (boolean, (optional)) – Remap Relative, Remap relative paths when saving to a different directory"

— present on both `bpy.ops.wm.save_as_mainfile` (defaulting to `True`) and `bpy.ops.wm.save_mainfile` (defaulting to `False`). Blender 4.5 Python API documentation, `https://docs.blender.org/api/4.5/bpy.ops.wm.html`, read 2026-07-30.

Reach: relative paths only, and only when the destination directory differs. Absolute paths are outside its scope — **inferred**, one step, from the words "Remap relative paths".

I also opened the manual's picture of the Save As file browser, `_images/files_blend_open-save_file-browser-save.png` (part of `https://docs.blender.org/manual/en/4.5/files/blend/open_save.html`, read 2026-07-30). **LABEL for this bit: documented.** The left-hand "Save As Blender File" panel shows three tick-boxes — Compress, Remap Relative, Save Copy — with Compress and Remap Relative ticked and Save Copy unticked. What is visible in the picture is that the option sits in the save panel and is ticked in that screenshot; I do not read a default out of a screenshot.

**Audit note (Opus 5, 2026-08-06):** this one claim rests on a picture, and a picture has no words to quote. Anyone checking it has to open the image named above and look. The label was left as the researcher set it; whether a described picture counts as documented is a question for the research method rather than for this document.

**B4. `bpy.data.libraries.write(..., path_remap=...)` — rewrites paths into a *new* file.**

> "write(filepath, datablocks, path_remap=False, fake_user=False, compress=False)
> Write data-blocks into a blend file."
>
> "path_remap (str) –
> Optionally remap paths when writing the file:
> NONE No path manipulation (default).
> RELATIVE Remap paths that are already relative to the new location.
> RELATIVE_ALL Remap all paths to be relative to the new location.
> ABSOLUTE Make all paths absolute on writing."

Blender 4.5 Python API documentation, `https://docs.blender.org/api/4.5/bpy.types.BlendDataLibraries.html`, read 2026-07-30.

Reach: the paths carried by the data-blocks being written out, plus whatever is dragged along — "Indirectly referenced data-blocks will be expanded and written too" (same page). This is the only means in the sources that offers all four choices — leave alone, remap-already-relative, remap-everything-relative, make-everything-absolute — in one place. It does not touch the file you are working in; it writes a different one.

**B5. Single-item replace operators and the path field itself.**

> "bpy.ops.image.replace(*, filepath='', …, relative_path=True, …)
> Replace current image by another one from disk"

`https://docs.blender.org/api/4.5/bpy.ops.image.html`, read 2026-07-30.

> "bpy.ops.sequencer.change_path(*, filepath='', directory='', files=None, …, relative_path=True, …, use_placeholders=False)"

`https://docs.blender.org/api/4.5/bpy.ops.sequencer.html`, read 2026-07-30.

And typing into the path field, with the choice of form made right there:

> "When relative paths are supported, the File Browser provides a Relative Path checkbox, when entering the path into a text field, use a double slash prefix (//) to make it so."
>
> "Relative paths are the default but this can be changed in the File tab of the Preferences."
>
> "You cannot use relative paths into a new untitled blend-file. Save it before linking to external files."

Blender 4.5 manual, `https://docs.blender.org/manual/en/4.5/files/blend/open_save.html`, read 2026-07-30. The File Browser's side panel carries the same option — "Relative Path — See Relative Paths." — `https://docs.blender.org/manual/en/4.5/editors/file_browser.html`, read 2026-07-30.

Reach: exactly one recorded location per use.

**Audit note (Opus 5, 2026-08-06):** for the two operators, that is **inferred**, one step. "Replace current image by another one from disk" says an image is swapped; it does not say in so many words that the path recorded against that image is rewritten. Both operators carry a `relative_path` argument, which is what the step leans on. A test to settle it has been added to the list at the end.

---

### C. Listing without rewriting, in the interface

The Outliner has a mode that lists everything in the file with its properties:

> "Blender File:
> Lists all data in the current blend-file. On the right side of the list, a shield icon shows the number of users – clicking it adds or removes a fake user.
>
> Data API:
> Lists every data-block in the file along with any properties that it might have."

Blender 4.5 manual, `https://docs.blender.org/manual/en/4.5/editors/outliner/interface.html`, read 2026-07-30.

Reach: "every data-block … along with any properties" — which would take in every path property named in B2. The manual says *lists*. It does not say you can edit a property there, and I am not going to claim you can. The Blender File mode is where the library Relocate and Reload of B1 are reached from (see B1's quotes, both of which begin "When the Outliner is in the Blender File Display Mode").

---

### D. Two adjacent things that are not this, but are easy to mistake for it

**Packing.** `Pack Resources`, `Unpack Resources`, `Pack Linked Libraries`, `Unpack Linked Libraries`, and their operators `bpy.ops.file.pack_all`, `unpack_all`, `unpack_item`, `pack_libraries`, `unpack_libraries`, `autopack_toggle`. These move file *contents* in and out of the `.blend`. That the recorded path survives packing is **inferred**, two steps: `ImagePackedFile.filepath` exists and is blend-relative-capable, and `bpy.utils.blend_paths` has an option to skip "file paths for packed data", which it would not need if packed data had no path. That second step is the same one A1 takes, and it is labelled there too. Unpacking offers a choice between the original recorded location and a local one:

> "Use files in current directory (create when necessary)
> Unpacks all files in the same directory // as the blend-file, grouping them in proper folders (like ''textures'' for instance)."
>
> "Use files in original location (create when necessary)
> Unpacks all files in their original location."
>
> "Remove Pack
> Just mark the file as unpacked, without actually writing it or reloading it from the drive."

Blender 4.5 manual, `https://docs.blender.org/manual/en/4.5/files/blend/packed_data.html`, read 2026-07-30. Also on that page, a limit worth knowing: "Some typically heavy external files, like videos from the Sequence Editor or Movie Clips, cannot be packed in a blend-file."

Whether unpacking to the current directory rewrites the stored path is not stated in those words. I am leaving it there.

**Path templates.** In 4.5 some recorded paths are not literal:

> "Path templates substitute template expressions (written {abcd}, where "abcd" is a variable name) in a filepath when the path is used."
>
> "This substitution happens internally at time of use (e.g. while rendering), so the template syntax will remain as-is in the path field."
>
> "Currently path templates are only supported for the render output path in Scene properties and the output paths in the compositor's File Output node."

Blender 4.5 manual, `https://docs.blender.org/manual/en/4.5/files/file_paths.html`, read 2026-07-30. The release notes say the same and mark it as new:

> "Some file paths in Blender can now contain "template expressions" like … Currently this is only enabled on the Scene's render output path and the output paths of the compositor's File Output node."

Blender 4.5 release notes, `https://developer.blender.org/docs/release_notes/4.5/core/`, read 2026-07-30. And a consequence for anyone reading those two paths out:

> "`scene.render.frame_path()` used to be infallible but can now fail if there are errors processing Path Templates in the render output path."

Blender 4.5 release notes, `https://developer.blender.org/docs/release_notes/4.5/python_api/`, read 2026-07-30.

So for those two paths, what is stored is the template as written, not the path that will actually be used — that is stated outright ("the template syntax will remain as-is in the path field"). Whether every one of the listing means above hands back that stored text unchanged is not stated; the quote is about the path field.

---

### E. What Cycles adds

**LABEL: observed.** I read the Cycles add-on's own files and searched them for path settings. `cycles_4.5/properties.py` contains only two `StringProperty` declarations in the whole file, both on a preferences group:

```
class CyclesDeviceSettings(bpy.types.PropertyGroup):
    id: StringProperty(name="ID")
    name: StringProperty(name="Name")
```

`Blender Sources/cycles_4.5/properties.py`, lines 1548–1550, read 2026-07-30. Neither is a path.

Searching the Cycles folder for `FILE_PATH` and `DIR_PATH` returns five hits, all in `cycles_4.5/operators.py`, and all of them are arguments to two operators rather than anything stored:

```
    input_filepath: StringProperty(
        name='Input Filepath',
        description='File path for image to denoise. If not specified, uses the render file path and frame range from the scene',
        default='',
        subtype='FILE_PATH')
```

`Blender Sources/cycles_4.5/operators.py`, lines 42–46 (within `class CYCLES_OT_denoise_animation`), read 2026-07-30. The other four are `output_filepath` on the same operator (lines 48–52) and `input_filepath1` / `input_filepath2` / `output_filepath` on `class CYCLES_OT_merge_images` (line 118), at lines 124–140 of the same file.

What this settles: the two searches run — for the two path markers across the whole Cycles folder, and for text settings in `properties.py` — turned up no Cycles setting that holds a path. What it does not settle: whether a render *actually* stores nothing path-shaped — that would be a step past what the file says, and I have not taken it. Nor would either search have caught a path kept as a plain text setting in some Cycles file other than `properties.py`.

---

### The two required lines

**Brought up to date by the hand tests of 2026-08-06 and 2026-08-07 (see the two observed sections below).** Every means named in this answer that has a Python form was run with no window at all, on the rented Linux machine, in this version. The two bulk sweeps, the missing-file search, the library relocation, saving elsewhere with the remap option, and the write call all returned `{'FINISHED'}` and did what they claim, with no interface present. Two operators did **not** on that machine: `bpy.ops.image.replace` and `bpy.ops.sequencer.change_path` both refused, naming a failed context check. The Mac sitting of 2026-08-07 took those two further, and they part company. **`bpy.ops.image.replace` needs a window**: with one on screen it returned `{'FINISHED'}` and rewrote the recorded path; with no window it refused on the Mac too, in every version. **`bpy.ops.sequencer.change_path` does not need a window**: with no window at all, given the file's name and the sequencer's own scene in the context, it returned `{'FINISHED'}` and rewrote the recorded folder and file, in every version. So this line is answered for everything here except that one operator, and that one is answered the other way. The words below are the researcher's own and are left as they were written.


**Outside the interface.** Partly answered, mostly not.

What is documented: every means in section A, B1, B3, B4 and B5 exists as a Python call, and Python can be handed to a Blender that nobody is sitting at — the manual documents `--python-expr <expression>` ("Run the given expression as a Python script") at `https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html`, read 2026-07-30. Section C's Outliner modes are interface-only by their nature. `Find Missing Files` and library `Relocate` are described in the manual as putting a File Browser on screen — "A File Browser will show up" — but both take a `directory` or `filepath` argument in their Python form, which is what you would supply instead.

The sources do not say whether these operators run successfully with no window. The one thing the documentation does say about operators run from scripts is a general warning: "Certain operators in Blender are only intended for use in a specific context… If the operator should logically be able to run but fails when accessed from a script it should be reported to the bug tracker" (`https://docs.blender.org/api/4.5/info_gotchas_operators.html`, read 2026-07-30). That is not an answer about these particular ones, and I will not turn it into one.

One genuinely outside-the-interface means of rewriting a recorded location, documented, with a documented trap:

> "-o, --render-output <path>
> Set the render path and file name.
> Use // at the start of the path to render relative to the blend-file."
>
> "blender --background --render-output /tmp test.blend --render-frame 1
> …will not render to /tmp because loading the blend-file overwrites the render output that was set."

Blender 4.5 manual, `https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html`, read 2026-07-30. Reach: the render output path, one setting, for that run only. Argument order matters and the manual gives the failing case explicitly.

**Operating system.**

**Brought up to date by the hand tests of 2026-08-06 and 2026-08-07 (see the two observed sections below).** Everything in this answer that could be run was run on the rented Linux machine, so **Linux is settled** for it. **macOS is now answered for the tests the Mac sitting ran**, and they are the ones this line turns on: the same file, built the same way, lists the same paths on the two machines — twenty-five of the twenty-seven printed lines are identical character for character. Two are not. The flat path list carries one extra entry on the Mac, a path into Blender's own application folder. And **whether capital letters in a name matter is answered differently on the two machines**: asked for its own image file with the name written in capitals, the Mac found it and the Linux machine did not. *(Citation check, 2026-08-06: narrowed. That check asks the operating system directly — `os.path.exists` on a name written in capitals — with none of Blender's own path machinery in it. It settles that the two file systems differ on letter case; it settles nothing about the means in this answer.)* Note what is still only Linux: the entries this round marked as running on both machines were in fact run on Linux alone, so for those the macOS half is untested rather than settled. **Windows stays untouched**: no Windows machine has run it, so the cross-drive question the sources raise was not answered by any test here. The words below are the researcher's own and are left as they were written.
 The sources say nothing about any of these eight means behaving differently per operating system. What they do say is that *paths themselves* are OS-sensitive, and Blender ships helpers for it: `bpy.path.basename` — "Equivalent to os.path.basename, but skips a "//" prefix. Use for Windows compatibility"; `bpy.path.native_pathsep` — "Replace the path separator with the systems native os.sep"; `bpy.path.resolve_ncase` — "Resolve a case insensitive path on a case sensitive system, returning a string with the path if found else return the original path" (all `https://docs.blender.org/api/4.5/bpy.path.html`, read 2026-07-30). The existence of these three tells you case-sensitivity and separator differences are real and must be handled by whoever writes the paths; it does not tell you that `Make Paths Relative` or `Find Missing Files` behave differently anywhere. Also documented: paths may not be valid UTF-8, and reading them can raise an error — "Paths are an exception to this rule since the existence of non-UTF-8 paths on the user's file system cannot be ignored" (`https://docs.blender.org/api/4.5/info_gotchas_file_paths_and_encoding.html`, read 2026-07-30).

### Not covered

- Whether `bpy.utils.blend_paths()` includes the render output path, the compositor File Output base path, or cache and bake directories. Its wording is "external files referenced by", which reads like inputs; the sources do not settle it.
- The same gap for `Make Paths Relative` / `Make Paths Absolute`: "all paths to external files" is not defined anywhere I found, so I cannot say whether output and cache paths are in or out.
- Whether `Find Missing Files` reaches linked-library paths, or only images, sounds, fonts and the like. Not stated.
- Whether the Outliner's Data API mode lets you *edit* a path, or only see it. The manual says "Lists".
- Whether unpacking with "Use files in current directory" rewrites the stored path or leaves it pointing at the original location.
- Whether any of these means works with no window open. Not stated for any of them.
- Anything about reading or rewriting a path **without a Blender**. Every means above is a thing Blender does. The sources describe no other tool, and I did not look outside them.
- Ordering and interaction: what happens if two means disagree, or what order a bulk operation visits paths in.

Added by the audit (Opus 5, 2026-08-06):

- The second reading of the question — "locations" meaning where inside the file's own structure a thing sits, rather than the file paths it records. This answer takes only the first reading. The second is unanswered, and whether it is wanted at all is not this document's to decide.
- Whether the list of path properties in B2 is complete. It is what one marker phrase finds, and `Text.filepath` shows that not every stored path carries that marker.
- ~~Which five pages the B2 search found that are not named in it.~~ Closed by the citation check, 2026-08-06 — see the audit note in B2. There are seven, and they are named there.
- Whether `bpy.ops.image.replace` and `bpy.ops.sequencer.change_path` rewrite the path recorded against the item, as opposed to swapping what is loaded. Not stated in the quoted words.
- Whether a Cycles setting outside `properties.py` holds a path kept as plain text with no path marker on it.

### Where I looked

Searched the whole of `manual_4.5`, `api_4.5`, `release_notes_4.5` and `cycles_4.5` (case-insensitive, recursive) for: `blend_paths`, `make_paths`, `missing_files`, `External Data`, `Relative Paths`, `relative path`, `path_remap`, `path remap`, `repath`, `relink`, `remap`, `Relocate`, `broken link`, `missing file`, `Missing Files`, `list of paths`, `external files`, `dependencies`, `blend relative // prefix supported`, `lib_relocate`, `lib_reload`, `relative_remap`, `FILE_PATH`, `DIR_PATH`, `StringProperty`, `background`. Also listed every operator whose name contains `path` across all `bpy.ops.*` pages, and every `bpy.types.*` page carrying the marker phrase `blend relative // prefix supported` (39 pages, all named in B2).

`repath` and `path remap` returned nothing anywhere — that is why those words appear nowhere in the answer.

Pages opened in full: manual — `interface/window_system/topbar`, `files/file_paths`, `files/blend/index`, `files/blend/open_save`, `files/blend/packed_data`, `files/linked_libraries/link_append`, `editors/outliner/interface`, `editors/file_browser`, `advanced/command_line/arguments`. API — `bpy.utils`, `bpy.path`, `bpy.ops.file`, `bpy.ops.wm` (the library and save sections), `bpy.ops.image`, `bpy.ops.sequencer`, `bpy.ops.outliner`, `bpy.types.Library`, `bpy.types.BlendDataLibraries`, `bpy.types.BlendData`, `bpy.types.Image`, `bpy.types.ImagePackedFile`, `bpy.types.Sound`, `bpy.types.Text`, `bpy.types.CacheFile`, `bpy.types.PointCacheItem`, `bpy.types.VolumeGrids`, `bpy.types.RenderSettings`, `bpy.types.CompositorNodeOutputFile`, `info_gotchas_file_paths_and_encoding`, `info_gotchas_operators`. Release notes — `core`, `python_api`, `bugfixes` (searched only). Cycles — `properties.py`, `operators.py`. Image — `_images/files_blend_open-save_file-browser-save.png`.

Ruled out and why: `manual_4.5/editors/preferences/file_paths` and `bpy.types.PreferencesFilePaths` — application settings, not recorded in any `.blend`. `manual_4.5/files/import_export/*` — importer and exporter options, which record nothing back into the file. `bpy_extras.io_utils` / `bpy_extras.image_utils` — helpers for people writing exporters, not a means of listing or rewriting what a file already holds. `bpy.ops.outliner.id_remap` — remaps which data-block is used, not which file path is recorded. `bpy.ops.wm.path_open` — opens a path in the operating system's file manager; changes nothing.

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

2026-08-06 — Opus 5 (Claude Code) — Reading a file (question 9), Blender 4.5

2026-08-07 — Opus 5 (Claude Code) — the Mac half of the hand tests

# Reading a file — added questions (Blender 5.0)

Sources consulted, and when:

- Blender manual 5.0, plain-text copy in `Blender Sources/manual_5.0`, read 2026-07-30.
- Blender Python API documentation 5.0, plain-text copy in `Blender Sources/api_5.0`, read 2026-07-30.
- Blender 5.0 release notes, in `Blender Sources/release_notes_5.0`, read 2026-07-30.
- Cycles' own files for 5.0, in `Blender Sources/cycles_5.0`, read 2026-07-30.
- Blender's requirements page, `Blender Sources/requirements/requirements_2026-08-01.txt`, taken 2026-08-01.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

Scope: this document answers question 9 only. Questions 1 to 8 on the sheet were not touched.

---

## 9. What separate means exist for listing and rewriting the locations recorded in a `.blend` file, and which kinds of recorded location does each of those means reach?

**Label: mixed — see each means below. The overall shape of the answer is documented; the coverage of the blanket means is not in the sources.**

### The reading I took

The question is ambiguous. I answer the reading I think is meant: "locations recorded in a `.blend` file" means the file paths a `.blend` stores that point at things outside itself — linked library files, images, sounds, movies, caches, fonts, volumes, and the folders it is set up to write into. The other reading would be internal addresses inside the file's own byte layout (where a data-block sits in the file); I did not answer that reading, and the sources I was given describe it only in passing.

Throughout, Blender writes such a location either in full from the drive root, or beginning with `//`, which the manual defines as the folder of the referencing `.blend`:

> "Blender stores a partial path evaluated relative to the directory location of the referencing blend-file."
> Manual, https://docs.blender.org/manual/en/5.0/files/blend/open_save.html — read 2026-07-30

That matters below, because several of the means exist only to convert between those two forms.

---

### The means, one by one

I found nine distinct means in the sources, plus three near-neighbours that people would reasonably expect to be on the list but are not (they are at the end).

---

#### 1. `bpy.utils.blend_paths()` — lists; nothing documented writes back through it

**Label: documented.**

> "bpy.utils.blend_paths(*, absolute=False, packed=False, local=False)
> Returns a list of paths to external files referenced by the loaded .blend file.
> Parameters:
> absolute (bool) – When true the paths returned are made absolute.
> packed (bool) – When true skip file paths for packed data.
> local (bool) – When true skip linked library paths.
> Returns: path list."
> Python API, https://docs.blender.org/api/5.0/bpy.utils.html — read 2026-07-30

What it reaches: "external files referenced by the loaded .blend file". The sources give no list of which kinds of reference that covers. The `local` option is the one clue that holds: linked library paths can be left out. **Inferred step:** that they are therefore inside the set when the option is not used. The quoted words say what the option leaves out; they do not say what the default set holds, and a reader has to supply that step.

**Citation check note (Opus 5, 2026-08-06):** the same step used to be taken for packed data too, and it has been cut. This document's own printout under *The packed switch* below shows `blend_paths()` coming back **without** the packed image's path and `blend_paths(packed=True)` coming back **with** it — the opposite of what the word "skip" on the page would give. What the packed option does is left to that test and not read off the page.

It returns strings. There is nothing in the documentation that lets you write a changed string back through this call, and no companion call that does.

---

#### 2. File ▸ External Data ▸ Report Missing Files — lists the broken ones only

**Label: documented.**

> "bpy.ops.file.report_missing_files()
> Report all missing external files"
> Python API, https://docs.blender.org/api/5.0/bpy.ops.file.html — read 2026-07-30

> "Report Missing Files
> This option is useful to check if there are links to unpacked files that no longer exist.
> After selecting this option, a warning message will appear in the Info editor's header.
> If no warning is shown, there are no missing external files."
> Manual, https://docs.blender.org/manual/en/5.0/interface/window_system/topbar.html — read 2026-07-30

What it reaches: "links to unpacked files that no longer exist". Not a full listing — only the subset that is missing. The result is delivered as a warning message in an editor header, not as data.

---

#### 3. Make Paths Relative / Make Paths Absolute — rewrites the whole file at once

**Label: documented.**

> "bpy.ops.file.make_paths_absolute()
> Make all paths to external files absolute
> bpy.ops.file.make_paths_relative()
> Make all paths to external files relative to current .blend"
> Python API, https://docs.blender.org/api/5.0/bpy.ops.file.html — read 2026-07-30

> "Make Paths Relative
> Make all paths to external files relative to the current blend-file.
> Make Paths Absolute
> Make all paths to external files absolute (= full path from the system's root)."
> Manual, https://docs.blender.org/manual/en/5.0/interface/window_system/topbar.html — read 2026-07-30

What it reaches: the sources say "all paths to external files" and stop there. No page in the manual or the API documentation enumerates what that set contains. This is the single biggest gap I found, and it applies equally to means 1, 2 and 4.

Neither call takes arguments, so there is no way to aim it at a subset.

---

#### 4. Find Missing Files — searches a folder and rewrites what it finds

**Label: documented.**

> "bpy.ops.file.find_missing_files(*, find_all=False, directory='', …)
> Try to find missing external files
> Parameters:
> find_all (boolean, (optional)) – Find All, Find all files in the search path (not just missing)
> directory (string, (optional, never None)) – Directory, Directory of the file"
> Python API, https://docs.blender.org/api/5.0/bpy.ops.file.html — read 2026-07-30

> "Find Missing Files
> In case you have broken links in a blend-file, this can help you to fix the problem.
> A File Browser will show up. Select the desired directory (or a file within that directory),
> and a search will be performed in it, recursively in all contained directories.
> Every missing file found in the search will be recovered.
> Those recoveries will be done as absolute paths,
> so if you want to have relative paths you will need to select Make Paths Relative."
> Manual, https://docs.blender.org/manual/en/5.0/interface/window_system/topbar.html — read 2026-07-30

The manual adds:

> "Recovered files might need to be reloaded. You can do that one by one, or
> you can save the blend-file and reload it again, so that all external files are reloaded at once."
> Same page, same date.

What it reaches: "missing external files", by default; with `find_all` on, "all files in the search path (not just missing)". Again no enumeration of kinds. It always writes back the full form, never the `//` form.

---

#### 5. Save As with Remap Relative — rewrites while writing a new copy

**Label: documented.**

> "relative_remap (boolean, (optional)) – Remap Relative, Remap relative paths when saving to a different directory"
> Python API, on both `bpy.ops.wm.save_as_mainfile` (default `True`) and `bpy.ops.wm.save_mainfile` (default `False`),
> https://docs.blender.org/api/5.0/bpy.ops.wm.html — read 2026-07-30

> "Remap Relative
> This option remaps Relative Paths
> (such as linked libraries and images) when saving a file in a new location."
> Manual, https://docs.blender.org/manual/en/5.0/files/blend/open_save.html — read 2026-07-30

What it reaches: the quoted words describe it as remapping relative paths. The manual names linked libraries and images as examples ("such as"), so its own wording is explicitly not a full list. **Inferred step:** that locations recorded in full form are therefore untouched by it. Nothing quoted says what it does with those, and a reader has to supply that step.

---

#### 6. `bpy.data.libraries.write(…, path_remap=…)` — rewrites into a file being written out

**Label: documented.**

> "write(filepath, datablocks, *, path_remap=False, fake_user=False, compress=False)
> Write data-blocks into a blend file.
> …
> path_remap (str) –
> Optionally remap paths when writing the file:
> NONE No path manipulation (default).
> RELATIVE Remap paths that are already relative to the new location.
> RELATIVE_ALL Remap all paths to be relative to the new location.
> ABSOLUTE Make all paths absolute on writing."
> Python API, https://docs.blender.org/api/5.0/bpy.types.BlendDataLibraries.html — read 2026-07-30

What it reaches: the paths carried by the set of data-blocks handed to it, plus whatever those pull in — the same page notes "Indirectly referenced data-blocks will be expanded and written too." This is the only means in the sources that offers all four choices (leave alone, remap the relative ones, force all relative, force all absolute) in one place. It writes to a file path handed to it. Nothing quoted says whether that file may already exist, or what becomes of the open one.

---

#### 7. The Outliner's Blender File view, with Relocate and Reload — libraries only

**Label: documented.**

Listing:

> "Linked data-blocks are indicated with a chain icon in the Outliner.
> They're also listed in the Outliner's Blender File Display Mode,
> along with the path of the blend-file they originate from."
> Manual, https://docs.blender.org/manual/en/5.0/files/linked_libraries/link_append.html — read 2026-07-30

Rewriting:

> "When the Outliner is in the Blender File Display Mode,
> a right-click on a library (a linked blend-file) will show the Relocate option.
> It allows to replace it by a different file.
> This can be used to either fix a broken linked library (e.g. because the file was moved
> or renamed), or to switch to a variation of the same data in a different file."
> Same page, same date.

> "Broken Libraries
> If Blender cannot find a library while loading a blend-file,
> it will create placeholder data-blocks to replace missing linked ones.
> That way, references to the missing data are not lost, and by relocating the missing library,
> the lost data can be automatically restored."
> Same page, same date.

The operators behind it:

> "bpy.ops.outliner.lib_relocate()
> Relocate the library under cursor"
> Python API, https://docs.blender.org/api/5.0/bpy.ops.outliner.html — read 2026-07-30

> "bpy.ops.wm.lib_relocate(*, library='', filepath='', directory='', filename='', files=None, …)
> Relocate the given library to one or several others"
> "bpy.ops.wm.lib_reload(*, library='', filepath='', …)
> Reload the given library"
> Python API, https://docs.blender.org/api/5.0/bpy.ops.wm.html — read 2026-07-30

What it reaches: whole linked library files — and, through the narrower means below, a single linked data-block. **Inferred step:** `bpy.ops.wm.lib_relocate` names the library and the new file as arguments, so it does not require the cursor to be over anything; the quoted words do not say that.

A second, narrower one sits beside it:

> "A right-click a linked ID will show the Relocate option in the ID Data sub-menu. It allows to
> relocate a directly linked ID by another one, from the same library or from a different one."
> Manual, https://docs.blender.org/manual/en/5.0/files/linked_libraries/link_append.html — read 2026-07-30

Its operator is flagged as not for outside use:

> "bpy.ops.wm.id_linked_relocate(…)
> Relocate a linked ID, i.e. select another ID to link, and remap its local usages to that newly linked data-block). Currently only designed as an internal operator, not directly exposed to the user"
> Python API, https://docs.blender.org/api/5.0/bpy.ops.wm.html — read 2026-07-30

---

#### 8. Reading and writing each path property directly through Python

**Label: documented.**

This is the means whose reach the sources come closest to pinning down, because each path is a named property on a named type. Library paths, for example:

> "filepath
> Path to the library .blend file
> Type:
> string, default "", (never None, blend relative // prefix supported)"
> Python API, https://docs.blender.org/api/5.0/bpy.types.Library.html — read 2026-07-30

Every such property carries the same marker, `(never None, blend relative // prefix supported)`. I searched the whole of the 5.0 API documentation for that exact marker text. It appears on the following properties.

What that marker establishes, and what it does not: it says these properties accept the `//` form. That the set of properties carrying it is the same as the set of locations a `.blend` records is a step the sources do not take — as the `Text.filepath` note below shows, a property can hold a file name without carrying the marker. So read the list as *properties documented to accept the `//` form*, which is the nearest thing the sources offer, rather than as a complete list of recorded locations. I searched for the marker and not for the properties that lack it.

Data-block paths:

- `Image.filepath` — "Image/Movie file name"; and `Image.filepath_raw`
- `ImagePackedFile.filepath`
- `Sound.filepath` — "Sound sample file used by this Sound data-block"
- `MovieClip.filepath` — "Filename of the movie or sequence file"
- `MovieClipProxy.directory`
- `Volume.filepath` — "Volume file used by this Volume data-block"
- `VolumeGrids.frame_filepath` — marked `readonly`
- `VectorFont.filepath`
- `CacheFile.filepath` — "Path to external displacements file"; and `CacheFileLayer.filepath`
- `Library.filepath`, and `LibraryWeakReference.filepath`

Output and cache folders:

- `RenderSettings.filepath` — "Directory/name to save animations, # characters define the position and padding of frame numbers"; also marked "Supports template expressions"
- `BakeSettings.filepath`
- `CompositorNodeOutputFile.directory` — "The directory where the image will be written"; also "Supports template expressions"
- `PointCache.filepath` — "Cache file path"; and `PointCacheItem.filepath`
- `FluidDomainSettings.cache_directory`
- `NodesModifier.bake_directory`, and `NodesModifierBake.directory`
- `DynamicPaintSurface.image_output_path`
- `CollectionExport.filepath`

Video sequencer:

- `ImageStrip.directory`
- `MovieStrip.filepath`
- `StripProxy.directory` and `StripProxy.filepath`
- `SequenceEditor.proxy_dir`

Modifiers and nodes:

- `MeshCacheModifier.filepath`
- `MultiresModifier.filepath`
- `OceanModifier.filepath`
- `ShaderNodeScript.filepath`
- `ShaderNodeTexIES.filepath`
- `NodeSocketStringFilePath.default_value`

Two things this enumeration shows that nothing else in the sources says:

- `PreferencesFilePaths` also carries five such properties (`font_directory`, `render_cache_directory`, `render_output_directory`, `sound_directory`, `texture_directory`). Those are settings of the program, not locations recorded in a `.blend`, so they are outside the question.
- `Text.filepath` — "Filename of the text file" — does **not** carry the marker. Its type line is `string, default "", (never None)`, with no mention of the `//` prefix. Python API, https://docs.blender.org/api/5.0/bpy.types.Text.html — read 2026-07-30.

One complication the documentation calls out for anyone reading these properties:

> "When using Blender data from linked libraries there is an unfortunate complication
> since the path will be relative to the library rather than the open blend-file.
> When the data block may be from an external blend-file pass the library argument from the bpy.types.ID.
> >>> bpy.path.abspath(image.filepath, library=image.library)"
> Python API, https://docs.blender.org/api/5.0/info_gotchas_file_paths_and_encoding.html — read 2026-07-30

So a recorded location read off a linked data-block is not anchored to the file you opened.

Two supporting helpers, which convert a path string but touch nothing in the file:

> "bpy.path.abspath(path, *, start=None, library=None)
> Returns the absolute path relative to the current blend file using the "//" prefix."
> "bpy.path.relpath(path, *, start=None)
> Returns the path relative to the current blend file using the "//" prefix."
> Python API, https://docs.blender.org/api/5.0/bpy.path.html — read 2026-07-30

---

#### 9. The path field in the interface, on any property that has one

**Label: documented.**

> "bpy.ops.buttons.file_browse(*, filepath='', …, relative_path=True, …)
> Open a file browser, hold Shift to open the file, Alt to browse containing directory"
> "bpy.ops.buttons.directory_browse(*, directory='', …, relative_path=True, …)
> Open a directory browser, hold Shift to open the file, Alt to browse containing directory"
> Python API, https://docs.blender.org/api/5.0/bpy.ops.buttons.html — read 2026-07-30

> "When relative paths are supported, the File Browser provides a Relative Path checkbox,
> when entering the path into a text field, use a double slash prefix (//) to make it so."
> Manual, https://docs.blender.org/manual/en/5.0/files/blend/open_save.html — read 2026-07-30

There are type-specific versions of the same idea, for example:

> "bpy.ops.image.replace(*, filepath='', …)
> Replace current image by another one from disk"
> Python API, https://docs.blender.org/api/5.0/bpy.ops.image.html — read 2026-07-30

What it reaches: one property at a time, whichever field the button belongs to. The Outliner puts every property in one place, though the words quoted below say only that it lists them, not that a path can be changed there:

> "Data API:
> Lists every data-block in the file along with any properties that it might have."
> Manual, https://docs.blender.org/manual/en/5.0/editors/outliner/interface.html — read 2026-07-30

---

### Three things that look like means but are not

**Packing.** Packing puts the referenced content inside the `.blend` rather than rewriting where it points.

> "Pack Resources
> Pack all used external files into the blend-file. After running this operator and saving the
> blend-file, the external files will no longer be used"
> Manual, https://docs.blender.org/manual/en/5.0/interface/window_system/topbar.html — read 2026-07-30

For linked `.blend` files, 5.0 keeps the recorded location alive across packing:

> "bpy.ops.file.pack_libraries()
> Store all data-blocks linked from other .blend files in the current .blend file. Library references are preserved so the linked data-blocks can be unpacked again"
> Python API, https://docs.blender.org/api/5.0/bpy.ops.file.html — read 2026-07-30

The 5.0 release notes describe this as new work:

> "Packed linked data are a key part of an improved asset workflow in Blender."
> "Name collisions are averted by storing these packed IDs into a new type of 'archive' libraries (and their namespaces)."
> Release notes, https://developer.blender.org/docs/release_notes/5.0/core/ — read 2026-07-30

The 5.0 API documentation lists `archive_libraries` and `archive_parent_library` on the `Library` type, and `is_linked_packed` on `ID` — "This data-block is linked and packed into the .blend file". The words of the first two are not quoted here, and nothing quoted says whether any of the three is new in 5.0. Python API, https://docs.blender.org/api/5.0/bpy.types.Library.html and https://docs.blender.org/api/5.0/bpy.types.ID.html — read 2026-07-30.

**The command line's `-o`.** It sets where a run writes, without altering what the file records.

> "-o, --render-output <path>
> Set the render path and file name.
> Use // at the start of the path to render relative to the blend-file."
> Manual, https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html — read 2026-07-30

**Exporter path handling.** `bpy_extras.io_utils.path_reference` produces a path for a file being exported ("Return a filepath relative to a destination directory, for use with exporters", Python API, https://docs.blender.org/api/5.0/bpy_extras.io_utils.html — read 2026-07-30). It does not read or write anything recorded in the `.blend`.

---

### Whether anything outside Blender can do this

**Label: not in the sources, with one pointer.**

The manual states the boundary plainly, and points at outside tooling without documenting it:

> "As described above, Blender itself is not allowed to write to other blend-files
> than the currently open one."
> "Since there is already tooling that can manipulate blend-files outside of Blender itself
> (see Blender Asset Tracer),
> it's possible to also create an external tool for doing such edits across blend-files."
> Manual, https://docs.blender.org/manual/en/5.0/files/asset_libraries/introduction.html — read 2026-07-30

That passage sits under a heading "Future Development", and the sources say nothing further about what that tool is, whether it ships with Blender, or which kinds of recorded location it reaches. I did not go outside the sources to find out.

The 5.0 release notes do confirm that outside readers of the format exist and were affected this release:

> "This change affects the structure of the file header
> as well as the header of each block in the file.
> So any code that is parsing .blend files has to be updated."
> Release notes, https://developer.blender.org/docs/release_notes/5.0/core/ — read 2026-07-30

---

### What Cycles adds

**Label: observed, with one inferred step.**

I searched Cycles' own files in `Blender Sources/cycles_5.0` for properties that record a location. `properties.py` contains three uses of `StringProperty` in total, and only two of them define a property:

> `    id: StringProperty(name="ID", description="Unique identifier of the device")`
> `    name: StringProperty(name="Name", description="Name of the device")`
> `Blender Sources/cycles_5.0/properties.py`, lines 1563 and 1564 — read 2026-07-30

Neither is a path. The only path-typed properties in Cycles' files are arguments of two operators:

> `    input_filepath: StringProperty(`
> `        name='Input Filepath',`
> `        description='File path for image to denoise. If not specified, uses the render file path and frame range from the scene',`
> `        default='',`
> `        subtype='FILE_PATH')`
> `Blender Sources/cycles_5.0/operators.py`, lines 37 to 41 (class `CYCLES_OT_denoise_animation`) — read 2026-07-30

and the three on `CYCLES_OT_merge_images` at lines 119 to 135 of the same file, whose words are not quoted here.

Inferred step: these are properties of operators rather than of data-blocks, and Cycles' settings classes define no path property at all, so Cycles contributes no additional kind of recorded location for any of the means above to reach. A reader has to supply the step that operator arguments are not stored in the `.blend`; the files themselves only show where the properties are declared.

---

### Not covered

- **Which kinds each blanket means actually reaches.** For means 1, 2, 3 and 4, the sources say "external files" and never define the set. My enumeration in means 8 comes from the property documentation, not from any statement about those operators, so I cannot claim the two sets are the same. Whether Make Paths Relative touches, say, `RenderSettings.filepath`, a point cache folder, or a geometry-nodes bake folder is not stated anywhere I searched.
- **`Text.filepath`.** I can say it lacks the `//` marker in its documentation. I cannot say from the sources what any of the blanket means does with it.
- **Order and interaction.** Nothing says what happens if two means are applied in sequence, or whether Find Missing Files can undo a Make Paths Relative.
- **Failure behaviour.** Nothing says what any of these do to a path that cannot be converted — for example a full path on a different drive when asked to become relative.
- **Path templates.** Where a location contains a template expression, what is recorded is the template text, not the resolved path: "This substitution happens internally at time of use (e.g. while rendering), so the template syntax will remain as-is in the path field" (Manual, https://docs.blender.org/manual/en/5.0/files/file_paths.html — read 2026-07-30). What the listing and rewriting means do with template text is not stated.
- **Read-only inspection without loading.** All nine means operate on a file Blender has already loaded. The question of reading locations out of an unopened file is question 1's ground and I did not go into it.
- **The second reading of the question** — internal byte offsets inside the file — is not answered here.

---

### Outside the interface

**Brought up to date by the hand tests of 2026-08-06 and 2026-08-07 (see the two observed sections below).** Every means named in this answer that has a Python form was run with no window at all, on the rented Linux machine, in this version. The two bulk sweeps, the missing-file search, the library relocation, saving elsewhere with the remap option, and the write call all returned `{'FINISHED'}` and did what they claim, with no interface present. Two operators did **not** on that machine: `bpy.ops.image.replace` and `bpy.ops.sequencer.change_path` both refused, naming a failed context check. The Mac sitting of 2026-08-07 took those two further, and they part company. **`bpy.ops.image.replace` needs a window**: with one on screen it returned `{'FINISHED'}` and rewrote the recorded path; with no window it refused on the Mac too, in every version. **`bpy.ops.sequencer.change_path` does not need a window**: with no window at all, given the file's name and the sequencer's own scene in the context, it returned `{'FINISHED'}` and rewrote the recorded folder and file, in every version. So this line is answered for everything here except that one operator, and that one is answered the other way. The words below are the researcher's own and are left as they were written.


**Label: partly documented, partly not in the sources.**

Documented that scripts run with no interface at all:

> "blender --background --python myscript.py"
> Python API, https://docs.blender.org/api/5.0/info_tips_and_tricks.html — read 2026-07-30

Where each means stands:

- Means 1 (`bpy.utils.blend_paths`), 6 (`bpy.data.libraries.write`) and 8 (property reads and writes, `bpy.path.abspath`/`relpath`) are plain Python calls. Nothing in the sources ties them to a window. Nothing in the sources states outright that they work without one either.
- Means 3 (`make_paths_relative` / `make_paths_absolute`) takes no arguments and no context is named for it in the documentation.
- Means 4 (`find_missing_files`) is documented as putting up a File Browser when chosen from the menu, but the operator carries a `directory` argument. Whether supplying that argument makes it run without a browser is **not stated**; I would be inferring it.
- Means 2 (`report_missing_files`) delivers its result as "a warning message ... in the Info editor's header". There is no Info editor when there is no interface, and the sources do not say where the message goes instead. This is the weakest of the nine for use from outside.
- Means 7's Outliner entries need the Outliner, but `bpy.ops.wm.lib_relocate` and `bpy.ops.wm.lib_reload` both take `library` and `filepath` arguments, so they identify their target without a cursor. Whether they still poll successfully without a window is not stated.
- Means 5 (`save_as_mainfile`) is an operator with all its inputs as arguments.
- Means 9 is an interface means by definition and has no counterpart outside it.

The general warning about driving operators from scripts applies to all of them:

> "Operators' poll function can fail where an API function would raise an exception giving details on exactly why."
> "Certain operators in Blender are only intended for use in a specific context"
> Python API, https://docs.blender.org/api/5.0/info_gotchas_operators.html — read 2026-07-30

So: for six of the nine — means 1, 3, 5, 6, 7 and 8 — the sources give no reason to expect a difference and no assurance either; for means 4 the sources leave open whether supplying the directory avoids the browser; for means 2 there is a documented reason to expect one; for means 9 the answer is that it does not exist outside the interface.

### Operating system

**Brought up to date by the hand tests of 2026-08-06 and 2026-08-07 (see the two observed sections below).** Everything in this answer that could be run was run on the rented Linux machine, so **Linux is settled** for it. **macOS is now answered for the tests the Mac sitting ran**, and they are the ones this line turns on: the same file, built the same way, lists the same paths on the two machines — twenty-five of the twenty-seven printed lines are identical character for character. Two are not. The flat path list carries one extra entry on the Mac, a path into Blender's own application folder. And **whether capital letters in a name matter is answered differently on the two machines**: asked for its own image file with the name written in capitals, the Mac found it and the Linux machine did not. *(Citation check, 2026-08-06: narrowed. That check asks the operating system directly — `os.path.exists` on a name written in capitals — with none of Blender's own path machinery in it. It settles that the two file systems differ on letter case; it settles nothing about the means in this answer.)* Note what is still only Linux: the entries this round marked as running on both machines were in fact run on Linux alone, so for those the macOS half is untested rather than settled. **Windows stays untouched**: no Windows machine has run it, so the cross-drive question the sources raise was not answered by any test here. The words below are the researcher's own and are left as they were written.


**Label: not in the sources for the means themselves; documented for the ground under them.**

No page I searched says any of these nine means behaves differently on Windows, macOS or Linux.

What the sources do say about paths across operating systems:

> "bpy.path.basename(path)
> Equivalent to os.path.basename, but skips a "//" prefix.
> Use for Windows compatibility."
> "bpy.path.resolve_ncase(path)
> Resolve a case insensitive path on a case sensitive system, returning a string with the path if found else return the original path."
> "bpy.path.native_pathsep(path)
> Replace the path separator with the systems native os.sep."
> Python API, https://docs.blender.org/api/5.0/bpy.path.html — read 2026-07-30

> "Paths are an exception to this rule since the existence of non-UTF-8 paths on the user's file system cannot be ignored."
> Python API, https://docs.blender.org/api/5.0/info_gotchas_file_paths_and_encoding.html — read 2026-07-30

Those three helpers exist because separators, letter case and text encoding of paths differ between systems. That is about the paths, not about the nine means, and I am not carrying it further than the words allow.

Which systems are in scope for 5.0, from the requirements page:

> "Note: Blender 4.5 LTS is the last release to support Intel and macOS 11.2 (Big Sur). Blender 5.0 and later require Apple Silicon running macOS 13 (Ventura) or later."
> https://www.blender.org/download/requirements/ — read 2026-08-01

That page names 5.0 in so many words only in that sentence. Its Windows and Linux tables carry no version and describe whichever release was current on 2026-08-01, so I do not read them as speaking for 5.0.

---

### Where I looked

Searched across `Blender Sources/manual_5.0`, `Blender Sources/api_5.0`, `Blender Sources/release_notes_5.0` and `Blender Sources/cycles_5.0`, for: `blend_paths`, `make_paths_absolute`, `make_paths_relative`, `find_missing_files`, `report_missing_files`, `lib_relocate`, `lib_reload`, `id_linked_relocate`, `relative_remap`, `remap`, `path_remap`, `path_reference`, `blend_render_info`, `asset tracer`, `blendfile.py`, `external tool`, `Relative Paths`, `relative path`, `Make Paths Absolute`, `Make Paths Relative`, `all paths to external`, `paths to external files`, `list of paths`, `missing file`, `broken link`, `placeholder`, `Blender File`, `Relocate`, `Reload Library`, `filepath`, `abspath`, `relpath`, `--render-output`, `--python-expr`, `--command`, and the exact marker string `blend relative // prefix supported`.

Pages opened in full or in part: manual — `interface/window_system/topbar`, `files/blend/open_save`, `files/blend/packed_data`, `files/file_paths`, `files/data_blocks`, `files/linked_libraries/link_append`, `files/asset_libraries/introduction`, `editors/outliner/interface`, `editors/file_browser`, `editors/preferences/save_load`, `render/output/properties/output`, `advanced/command_line/arguments`. API — `bpy.utils`, `bpy.path`, `bpy.ops.file`, `bpy.ops.wm`, `bpy.ops.outliner`, `bpy.ops.buttons`, `bpy.ops.image`, `bpy.types.Library`, `bpy.types.BlendDataLibraries`, `bpy.types.ID`, `bpy.types.Text`, `bpy.types.Image`, `bpy.types.RenderSettings`, `bpy.types.CompositorNodeOutputFile`, `bpy_extras.io_utils`, `info_gotchas_file_paths_and_encoding`, `info_gotchas_operators`, `info_tips_and_tricks`, `info_advanced_blender_as_bpy`. Release notes — `core`, `pipeline_io`, `python_api`, `bugfixes`, `corrective_releases`. Cycles — `properties.py`, `operators.py`, `osl.py`.

Ruled out and why: `bpy.ops.object.paths_*` and `bpy.ops.pose.paths_*` are motion paths of moving objects, nothing to do with files. `PreferencesFilePaths` holds program settings, not anything recorded in a `.blend`. The USD, OBJ, FBX and glTF exporter pages have their own relative-path options; those govern what the exported file records, not what the `.blend` records. No picture in the manual was needed for this answer — the wording of the External Data menu is fully spelled out in text, so I did not open `_images`.

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

2026-08-06 — Opus 5 (Claude Code) — Reading a file (added questions), Blender 5.0

2026-08-07 — Opus 5 (Claude Code) — the Mac half of the hand tests

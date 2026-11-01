# Reading a file — added questions, second pass (Blender 5.1)

**Sources consulted, and when.** Blender's 5.1 manual, Blender's 5.1 Python API documentation, Blender's 5.1 release notes, and Cycles' own files for 5.1 — all read on 2026-07-30 (the date the copies in `Blender Sources/` were taken). Blender's requirements page, read as of 2026-08-01. I did not use the web.

`I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.`

One disclosure, so the line above is exact rather than merely true in spirit. I did not open the project's `CLAUDE.md`, but its text was placed into my working context automatically by the tool I run inside, before I read anything. That file is a signpost only: it names `Preparation/Groundwork/Start Here.md` and states it holds no rules of its own. I did not follow it, and I opened nothing it points at. It contains no claim about how Blender behaves, so nothing below can be an echo of it.

Scope: question 9 only. Questions 1–8 on the sheet were not touched.

---

## 9. What separate means exist for listing and rewriting the locations recorded in a `.blend` file, and which kinds of recorded location does each of those means reach?

### How I read the question

I read "locations recorded in a `.blend` file" as the file paths a `.blend` stores — the paths pointing at images, movies, sounds, fonts, caches, linked `.blend` files, scripts and so on, plus the paths the file stores for where it will write. I read "means" as separate mechanisms a person or a program can use, not separate buttons on the same mechanism.

The question is ambiguous in one place, and I flag it rather than choose silently. "Rewriting" could mean *changing the path Blender is using right now, in memory* or *changing what is written back into the `.blend` file on disk*. Everything below changes the first; the second follows only if the file is then saved. Where the sources say so, I say so. The other reading — that "means" refers only to tools that edit a `.blend` on disk without Blender — is answered at point 12, and the sources are nearly silent there.

### First, a constraint that shapes every answer below

**Label: documented.**

Blender will not write into a `.blend` other than the one it currently has open. So every in-Blender means below is really "open the file, change the paths in memory, save the file".

> "Blender is not allowed to write to other blend-files than the one you have currently open, or the special .asset.blend files explained above."

Source: the manual, <https://docs.blender.org/manual/en/5.1/files/asset_libraries/introduction.html>. Read 2026-07-30.

### The kinds of recorded location that exist

**Label: documented.**

Before the means, the kinds. The Python API documents a path field on each of these, and I quote the description each carries. All of these are stored in the `.blend`.

- `Library.filepath` — "Path to the library .blend file (default "", never None, blend relative // prefix supported)" — <https://docs.blender.org/api/5.1/bpy.types.Library.html>
- `Image.filepath` — "Image/Movie file name (default "", never None, blend relative // prefix supported)" — <https://docs.blender.org/api/5.1/bpy.types.Image.html>
- `MovieClip.filepath` — "Filename of the movie or sequence file" — <https://docs.blender.org/api/5.1/bpy.types.MovieClip.html>
- `Sound.filepath` — "Sound sample file used by this Sound data-block" — <https://docs.blender.org/api/5.1/bpy.types.Sound.html>
- `VectorFont.filepath` — "(default "", never None, blend relative // prefix supported)" — <https://docs.blender.org/api/5.1/bpy.types.VectorFont.html>
- `Volume.filepath` — "Volume file used by this Volume data-block" — <https://docs.blender.org/api/5.1/bpy.types.Volume.html>
- `CacheFile.filepath` — "Path to external displacements file" — <https://docs.blender.org/api/5.1/bpy.types.CacheFile.html>
- `CacheFileLayer.filepath` — "Path to the archive" — <https://docs.blender.org/api/5.1/bpy.types.CacheFileLayer.html>
- `PointCache.filepath` — "Cache file path" — <https://docs.blender.org/api/5.1/bpy.types.PointCache.html>
- `MeshCacheModifier.filepath` and `MultiresModifier.filepath` — both "Path to external displacements file" — <https://docs.blender.org/api/5.1/bpy.types.MeshCacheModifier.html>, <https://docs.blender.org/api/5.1/bpy.types.MultiresModifier.html>
- `OceanModifier.filepath` — "Path to a folder to store external baked images" — <https://docs.blender.org/api/5.1/bpy.types.OceanModifier.html>
- `ShaderNodeScript.filepath` — "Shader script path" — <https://docs.blender.org/api/5.1/bpy.types.ShaderNodeScript.html>
- `ShaderNodeTexIES.filepath` — "IES light path" — <https://docs.blender.org/api/5.1/bpy.types.ShaderNodeTexIES.html>
- `MovieStrip.filepath` and `ImageStrip.directory` — both "(default "", never None, blend relative // prefix supported)" — <https://docs.blender.org/api/5.1/bpy.types.MovieStrip.html>, <https://docs.blender.org/api/5.1/bpy.types.ImageStrip.html>
- `StripProxy.filepath` — "Location of custom proxy file" — <https://docs.blender.org/api/5.1/bpy.types.StripProxy.html>
- `RenderSettings.filepath` — "Directory/name to save animations, # characters define the position and padding of frame numbers (default "", never None, blend relative // prefix supported, Supports template expressions)" — <https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html>
- `BakeSettings.filepath` — "Image filepath to use when saving externally" — <https://docs.blender.org/api/5.1/bpy.types.BakeSettings.html>
- `CompositorNodeOutputFile.directory` — "The directory where the image will be written (default "", never None, blend relative // prefix supported, Supports template expressions)" — <https://docs.blender.org/api/5.1/bpy.types.CompositorNodeOutputFile.html>
- `CollectionExport.filepath` — "The file path used for exporting" — <https://docs.blender.org/api/5.1/bpy.types.CollectionExport.html>
- `ImagePackedFile.filepath` — the path a packed image remembers — <https://docs.blender.org/api/5.1/bpy.types.ImagePackedFile.html>
- `LibraryWeakReference.filepath` — "Path to the library .blend file" — <https://docs.blender.org/api/5.1/bpy.types.LibraryWeakReference.html>
- `Text.filepath` — "Filename of the text file (default "", never None)" — <https://docs.blender.org/api/5.1/bpy.types.Text.html>
- `TextureNodeOutput.filepath` — "(default "", never None)" — <https://docs.blender.org/api/5.1/bpy.types.TextureNodeOutput.html>
- `Theme.filepath` — "The path to the preset loaded into this theme (if any) (default "", never None)" — <https://docs.blender.org/api/5.1/bpy.types.Theme.html>

Date read for all of the above: 2026-07-30.

One thing to notice in that list, and it matters for the means below. **Label: inferred.** Almost every entry carries the words "blend relative // prefix supported". Three of the entries above do not: `Text.filepath`, `TextureNodeOutput.filepath` and `Theme.filepath`. **Citation check (Opus 5, 2026-08-06):** so does a fourth recorded location that this list leaves out altogether — `Camera.custom_filepath`, "Path to the shader defining the custom camera (default "", never None)" (`https://docs.blender.org/api/5.1/bpy.types.Camera.html`, read 2026-07-30). The first-pass document for this version names it. It is added here so the count and the list agree. The step I took: the documentation states that flag on the others and omits it on those three, so those three cannot hold a path relative to the `.blend`. A reader has to supply the assumption that the omission is deliberate rather than an oversight, so this is inferred, not documented.

---

### The means

#### 1. `bpy.utils.blend_paths()` — listing only

**Label: documented.**

> "bpy.utils.blend_paths(*, absolute=False, packed=False, local=False)
>
> Returns a list of paths to external files referenced by the loaded .blend file.
>
> absolute (bool) – When true the paths returned are made absolute.
>
> packed (bool) – When true include file paths for packed data.
>
> local (bool) – When true skip linked library paths."

Source: the Python API documentation, <https://docs.blender.org/api/5.1/bpy.utils.html>. Read 2026-07-30.

**Which kinds it reaches.** Documented only at this level: "external files referenced by the loaded .blend file". The three switches tell us three specific things about its reach — it can be asked to return the paths as absolute, it can be asked to include paths belonging to packed data, and it can be asked to skip the paths of linked libraries (which means that by default it includes them). It returns a plain list of strings, so it identifies no owner and cannot be written back through.

**Not covered.** The documentation does not enumerate which of the kinds listed above appear in the returned list. In particular it does not say whether the render output path, the compositor File Output directory, bake output paths or collection-export paths are counted as "external files referenced" — those are places the file writes *to*, not files it reads, and the wording leaves it open.

#### 2. The path field on each data-block — listing and rewriting, one kind at a time

**Label: documented.**

Every property in the list above is a normal Python property on a normal data-block, reachable through `bpy.data`, and none of them are marked read-only except `BlendImportContextLibrary.filepath` ("(default "", readonly, never None)"). The same fields are the path widgets in the interface.

The API documentation shows this being used:

> ">>> bpy.path.abspath(image.filepath)"

and, for a path belonging to a linked data-block:

> ">>> bpy.path.abspath(image.filepath, library=image.library)"

Source: the Python API documentation, <https://docs.blender.org/api/5.1/info_gotchas_file_paths_and_encoding.html>. Read 2026-07-30.

**Which kinds it reaches.** Exactly the kind you name, and only that one. This is the only means that reaches every kind in the list, because you go to each kind yourself. It is also the only means that gives you the owner of the path as well as the path.

**Not covered.** There is no documented single collection that holds "all data-blocks with a path". You must know the kinds in advance, or find them by the means at point 11.

#### 3. Make Paths Absolute / Make Paths Relative — bulk rewriting

**Label: documented.**

> "bpy.ops.file.make_paths_absolute()
>
> Make all paths to external files absolute"

> "bpy.ops.file.make_paths_relative()
>
> Make all paths to external files relative to current .blend"

Source: the Python API documentation, <https://docs.blender.org/api/5.1/bpy.ops.file.html>. Read 2026-07-30.

The manual gives the same two under the File menu:

> "Make Paths Relative
>
> Make all paths to external files relative to the current blend-file.
>
> Make Paths Absolute
>
> Make all paths to external files absolute (= full path from the system's root)."

Source: the manual, <https://docs.blender.org/manual/en/5.1/interface/window_system/topbar.html> (File ‣ External Data). Read 2026-07-30.

**Which kinds it reaches.** Documented as "all paths to external files", with no list. Neither operator takes any argument, so there is no documented way to limit it to one kind or to see which paths it changed.

**Not covered.** Which of the kinds above are actually visited. Whether "external files" here includes output paths. What happens to the three kinds that do not support the `//` prefix.

#### 4. Report Missing Files — listing, but only the broken ones

**Label: documented.**

> "bpy.ops.file.report_missing_files()
>
> Report all missing external files"

Source: the Python API documentation, <https://docs.blender.org/api/5.1/bpy.ops.file.html>. Read 2026-07-30.

The manual says where the result appears:

> "Report Missing Files
>
> This option is useful to check if there are links to unpacked files that no longer exist. After selecting this option, a warning message will appear in the Info editor's header. If no warning is shown, there are no missing external files."

Source: the manual, <https://docs.blender.org/manual/en/5.1/interface/window_system/topbar.html>. Read 2026-07-30.

**Which kinds it reaches.** "All missing external files" — again undivided. It lists only paths that do not resolve, so it is not a way to list the paths that are fine.

**Not covered.** The operator returns only an operator result, not a list of paths; the manual describes the output as a warning message in a header. The sources do not describe any structured, machine-readable form of that report.

#### 5. Find Missing Files — bulk rewriting of the broken ones

**Label: documented.**

> "bpy.ops.file.find_missing_files(*, find_all=False, directory='', …)
>
> Try to find missing external files
>
> find_all (bool) – Find All, Find all files in the search path (not just missing) (optional)
>
> directory (str) – Directory, Directory of the file (optional, never None)"

Source: the Python API documentation, <https://docs.blender.org/api/5.1/bpy.ops.file.html>. Read 2026-07-30.

The manual describes what it does to the recorded paths:

> "In case you have broken links in a blend-file, this can help you to fix the problem. A File Browser will show up. Select the desired directory (or a file within that directory), and a search will be performed in it, recursively in all contained directories. Every missing file found in the search will be recovered. Those recoveries will be done as absolute paths, so if you want to have relative paths you will need to select Make Paths Relative."

Source: the manual, <https://docs.blender.org/manual/en/5.1/interface/window_system/topbar.html>. Read 2026-07-30.

**Which kinds it reaches.** "Missing external files", undivided — and with `find_all` switched on, files that are not missing as well. The rewritten paths are absolute.

**Not covered.** Whether it reaches the same set of kinds as Make Paths Relative. Whether it reports which paths it changed.

#### 6. Relocating a linked library, or a single linked data-block — library paths only

**Label: documented.**

> "bpy.ops.wm.lib_relocate(*, library='', filepath='', directory='', filename='', …)
>
> Relocate the given library to one or several others
>
> library (str) – Library, Library to relocate (optional, never None)"

> "bpy.ops.wm.lib_reload(*, library='', filepath='', …)
>
> Reload the given library"

Source: the Python API documentation, <https://docs.blender.org/api/5.1/bpy.ops.wm.html>. Read 2026-07-30.

The same from the Outliner:

> "bpy.ops.outliner.lib_relocate()
>
> Relocate the library under cursor"

> "bpy.ops.outliner.id_linked_relocate()
>
> Replace the active linked ID (and its dependencies if any) by another one, from the same or a different library"

Source: the Python API documentation, <https://docs.blender.org/api/5.1/bpy.ops.outliner.html>. Read 2026-07-30.

There is also a lower-level one, and the documentation warns about it:

> "Relocate a linked ID, i.e. select another ID to link, and remap its local usages to that newly linked data-block). Currently only designed as an internal operator, not directly exposed to the user"

Source: `bpy.ops.wm.id_linked_relocate`, the Python API documentation, <https://docs.blender.org/api/5.1/bpy.ops.wm.html>. Read 2026-07-30.

The manual describes the interface route and says plainly what it is for:

> "When the Outliner is in the Blender File Display Mode, a right-click on a library (a linked blend-file) will show the Relocate option. It allows to replace it by a different file. This can be used to either fix a broken linked library (e.g. because the file was moved or renamed), or to switch to a variation of the same data in a different file."

and

> "If Blender cannot find a library while loading a blend-file, it will create placeholder data-blocks to replace missing linked ones. That way, references to the missing data are not lost, and by relocating the missing library, the lost data can be automatically restored."

Source: the manual, <https://docs.blender.org/manual/en/5.1/files/linked_libraries/link_append.html>. Read 2026-07-30.

**Which kinds it reaches.** One kind only: the path of a linked `.blend` (`Library.filepath`), or the choice of which data-block inside a library is used. It touches nothing else.

**Not covered.** These operators are aimed at a named library or at whatever is under the cursor; the sources do not describe a form that rewrites every library path in one call.

#### 7. Remapping at the moment of saving or writing

**Label: documented.** Two separate forms.

Saving the open file elsewhere:

> "relative_remap (bool) – Remap Relative, Remap relative paths when saving to a different directory (optional)"

Source: `bpy.ops.wm.save_as_mainfile`, the Python API documentation, <https://docs.blender.org/api/5.1/bpy.ops.wm.html>. Read 2026-07-30.

The manual on the same option, and it names two of the kinds:

> "Remap Relative
>
> This option remaps Relative Paths (such as linked libraries and images) when saving a file in a new location."

Source: the manual, <https://docs.blender.org/manual/en/5.1/files/blend/open_save.html>. Read 2026-07-30.

Writing selected data-blocks out to a new `.blend` from Python:

> "write(filepath, datablocks, *, path_remap='NONE', fake_user=False, compress=False)
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
> ABSOLUTE Make all paths absolute on writing."

Source: `bpy.types.BlendDataLibraries.write`, the Python API documentation, <https://docs.blender.org/api/5.1/bpy.types.BlendDataLibraries.html>. Read 2026-07-30.

The 5.1 release notes record a fix to that very option:

> "Bpy.data.libraries.write with path_remap='RELATIVE_ALL' does not recalculate existing relative paths"

Source: the release notes, <https://developer.blender.org/docs/release_notes/5.1/bugfixes/>. Read 2026-07-30.

**Which kinds each reaches.** `relative_remap` is documented as reaching "Relative Paths (such as linked libraries and images)" — a named pair with an "such as", so not an exhaustive list. `path_remap` is documented as reaching "paths" and "all paths", undivided. Both act on the copy being written, not on the file you started from.

**Not covered.** Neither is a listing means — you cannot see what was remapped. Neither documents the full set of kinds. `relative_remap` is documented as taking effect only "when saving to a different directory".

#### 8. Packing and unpacking — changing whether a location is used at all

**Label: documented.**

> "bpy.ops.file.pack_all()
>
> Pack all used external files into this .blend"

> "bpy.ops.file.unpack_all(*, method='USE_LOCAL')
>
> Unpack all files packed into this .blend to external ones"

> "bpy.ops.file.unpack_item(*, method='USE_LOCAL', id_name='', id_type=19785)
>
> Unpack this file to an external file"

> "bpy.ops.file.autopack_toggle()
>
> Automatically pack all external files into the .blend file"

> "bpy.ops.file.pack_libraries()
>
> Store all data-blocks linked from other .blend files in the current .blend file. Library references are preserved so the linked data-blocks can be unpacked again"

> "bpy.ops.file.unpack_libraries()
>
> Restore all packed linked data-blocks to their original locations"

Source: the Python API documentation, <https://docs.blender.org/api/5.1/bpy.ops.file.html>. Read 2026-07-30.

Unpacking decides where the file lands, and therefore what the recorded path becomes. The manual's four choices:

> "Use files in current directory (create when necessary)
>
> Unpacks all files in the same directory // as the blend-file, grouping them in proper folders (like ''textures'' for instance). However, if the final file exists already, it will use that file, instead of unpacking it."

> "Use files in original location (create when necessary)
>
> Unpacks all files in their original location."

Source: the manual, <https://docs.blender.org/manual/en/5.1/files/blend/packed_data.html>. Read 2026-07-30.

**Which kinds it reaches.** Here the sources are unusually clear that the reach is *not* everything:

> "Not all external files can be packed
>
> Some typically heavy external files, like videos from the Sequence Editor or Movie Clips, cannot be packed in a blend-file."

Source: the manual, <https://docs.blender.org/manual/en/5.1/files/blend/packed_data.html>. Read 2026-07-30.

`unpack_item` reaches a single named data-block, through `id_name` and `id_type`. `pack_libraries` and `unpack_libraries` reach linked `.blend` files only. Per-kind pack and unpack operators also exist — `bpy.ops.image.pack`, `bpy.ops.image.unpack`, `bpy.ops.sound.pack`, `bpy.ops.sound.unpack` (from <https://docs.blender.org/api/5.1/bpy.ops.image.html> and <https://docs.blender.org/api/5.1/bpy.ops.sound.html>, read 2026-07-30).

**Not covered.** Packing is a way of making the recorded location stop mattering, not a way of listing or rewriting it. Note also that the manual says packing takes effect on the next save: "Actual packing will happen on the next save of the blend-file."

#### 9. Per-kind "replace this file" operators

**Label: documented.**

> "bpy.ops.image.replace(*, filepath='', …, relative_path=True, …)
>
> Replace current image by another one from disk"

> "bpy.ops.image.file_browse(*, filepath='', …)
>
> Open an image file browser, hold Shift to open the file, Alt to browse containing directory"

Source: the Python API documentation, <https://docs.blender.org/api/5.1/bpy.ops.image.html>. Read 2026-07-30.

`bpy.ops.sound.open`, `bpy.ops.sound.open_mono` and `bpy.ops.font.open` are the equivalent openers for their kinds (from <https://docs.blender.org/api/5.1/bpy.ops.sound.html> and <https://docs.blender.org/api/5.1/bpy.ops.font.html>, read 2026-07-30).

**Which kinds it reaches.** One data-block of one kind, the one that is active. The `relative_path` argument decides how the new location is recorded.

**Not covered.** These are rewriting means only, and they act on whatever is active, which the sources tie to the editor context.

#### 10. The command line, for the render output path

**Label: documented.**

> "-o, --render-output <path>
>
> Set the render path and file name. Use // at the start of the path to render relative to the blend-file.
>
> You can use path templating features such as {blend_name} in the path."

Source: the manual, <https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html>. Read 2026-07-30.

And an ordering trap that is documented on the same page:

> "blender --background --render-output /tmp test.blend --render-frame 1
>
> …will not render to /tmp because loading the blend-file overwrites the render output that was set."

Source: as above.

**Which kinds it reaches.** Exactly one: the render output path (`RenderSettings.filepath`). Nothing else.

**Not covered.** This changes the path in the running session. The sources do not say it is written back into the `.blend`, and nothing on that page suggests it is.

Alongside it, the API offers a way to *read out* what the output path resolves to:

> "frame_path(*, frame=-2147483648, preview=False, view='')
>
> Return the absolute path to the filename to be written for a given frame"

Source: `bpy.types.RenderSettings.frame_path`, the Python API documentation, <https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html>. Read 2026-07-30.

#### 11. Walking the data through the property system — a general way to find every path field

**Label: documented.**

Blender marks its properties, and three of the marks are about paths:

> "is_path_output
>
> Property is a filename, filepath or directory output (default False, readonly)"

> "is_path_supports_blend_relative
>
> Property is a path which supports the "//" prefix, signifying the location as relative to the ".blend" file's directory (default False, readonly)"

> "is_path_supports_templates
>
> Property is a path which supports the "{variable_name}" variable expression syntax, which substitutes the value of the referenced variable in place of the expression (default False, readonly)"

Source: `bpy.types.Property`, the Python API documentation, <https://docs.blender.org/api/5.1/bpy.types.Property.html>. Read 2026-07-30.

And string properties carry a subtype naming what they hold:

> "FILE_PATH:
>
> File Path.
>
> DIR_PATH:
>
> Directory Path.
>
> FILE_NAME:
>
> File Name."

Source: the Python API documentation, <https://docs.blender.org/api/5.1/bpy_types_enum_items/property_subtype_string_items.html>. Read 2026-07-30.

The same information is browsable in the interface:

> "Data API:
>
> Lists every data-block in the file along with any properties that it might have."

Source: the manual, Outliner display modes, <https://docs.blender.org/manual/en/5.1/editors/outliner/interface.html>. Read 2026-07-30.

**Which kinds it reaches. Label: inferred.** In principle every path-valued property on every data-block, including ones added by add-ons, because the marks are on the properties themselves rather than on a fixed list. The step I took: the documentation says these flags exist and describes what they mean, and separately says the Data API view lists every data-block with its properties; it does not say anywhere that walking the properties and collecting the ones marked as paths is a supported way to find every recorded location. A reader has to join those two facts. So: inferred, and moderately so.

**Not covered.** The sources give no worked example of such a walk, do not say whether it reaches paths held inside custom properties, and do not say whether it reaches paths on data that is linked from another file.

There is one place where a recorded location does live in a custom property rather than a normal path field. **Label: observed** — from Cycles' own files:

> `elif datatype is str and metadata.get('widget') == 'filename':`
> `    ui.update(subtype='FILE_PATH')`

File: `cycles_5.1/osl.py`, lines 147–148, inside the function `osl_param_ensure_property(ccam, param)`. Read 2026-07-30.

What that file says is that a string parameter of an OSL camera shader whose metadata marks it as a filename is stored as a custom property with the `FILE_PATH` subtype. That is a location recorded in the `.blend` that is not one of the named fields in the list at the top.

Cycles' files also show how one recorded location is turned into a real one at use time:

> `script_path = bpy.path.abspath(filepath, library=library)`

File: `cycles_5.1/osl.py`, line 199, in `update_external_script(report, filepath, library)`; called from `update_script_node(node, report)` at line 280 as `update_external_script(report, node.filepath, node.id_data.library)`. Read 2026-07-30.

I have not run anything, so I make no claim about what happens at render time; that would be a step beyond what the file says.

#### 12. Tools outside Blender

**Label: documented, but thin.**

The manual states that such tooling exists and names one:

> "Since there is already tooling that can manipulate blend-files outside of Blender itself (see Blender Asset Tracer), it's possible to also create an external tool for doing such edits across blend-files."

Source: the manual, <https://docs.blender.org/manual/en/5.1/files/asset_libraries/introduction.html>. Read 2026-07-30.

**Which kinds it reaches. Label: not in the sources.** The sentence is a passing reference inside a section about future development of the asset system. Nothing in the manual, the API documentation, the release notes or Cycles' files describes what Blender Asset Tracer is, how it is obtained, what it lists, or what it can rewrite. I searched all four for "asset tracer", "Blender Asset Tracer", "blender-asset-tracer" and "bat_pack": the only hits anywhere are the two sentences on that one manual page. So the honest answer is that this means exists by the manual's own word, and its reach is undocumented here.

#### Not a means, but easy to mistake for one

**Label: documented.** `bpy.path.abspath` and `bpy.path.relpath` convert a path string; they touch nothing in the file.

> "bpy.path.abspath(path, *, start=None, library=None)
>
> Returns the absolute path relative to the current blend file using the "//" prefix."

> "bpy.path.relpath(path, *, start=None)
>
> Returns the path relative to the current blend file using the "//" prefix."

Source: the Python API documentation, <https://docs.blender.org/api/5.1/bpy.path.html>. Read 2026-07-30.

Likewise `bpy_extras.io_utils.path_reference` and `path_reference_copy` (<https://docs.blender.org/api/5.1/bpy_extras.io_utils.html>, read 2026-07-30) are helpers for exporters deciding how to refer to a file they are writing out, not a way of listing or rewriting what a `.blend` records.

And the Preferences hold their own file paths (<https://docs.blender.org/manual/en/5.1/editors/preferences/file_paths.html>) — those are settings of the installation, not locations recorded in a `.blend`.

---

### One piece of evidence that the bulk means do not reach everything

**Label: documented (the quotes), inferred (what I draw from them).**

Blender's 5.1 corrective-release notes list two fixes naming an internal path-visiting callback:

> "Correct path for foreach_path callback of object's particle systems. (blender/blender@6a713d7797)"

> "GeoNodes simulation cache files not reported via foreach_path callback. (blender/blender#155953)"

Source: the release notes, <https://developer.blender.org/docs/release_notes/5.1/corrective_releases/>. Read 2026-07-30.

Those two sentences are documented statements that a callback named `foreach_path` exists, that it is what "reports" paths, and that two kinds of path were being reported wrongly or not at all until these fixes. **The step I took, and it is a real step:** nothing in the sources says that the operators at points 3, 4, 5 and 7 are built on that callback. So all I can claim as documented is that Blender has one shared path-visiting mechanism whose coverage has had gaps. The claim that "Make Paths Relative therefore missed geometry-nodes simulation cache files" is inferred, and I would not rely on it without checking by hand.

Note also that both of these are listed among *corrective* releases, which means the state of 5.1 depends on which corrective release you have.

---

### Not covered by this answer, overall

- No source enumerates, for any bulk means, which kinds of recorded location it visits. Every bulk operator is documented with an undivided phrase — "all paths to external files", "all missing external files", "all paths". That is the single largest gap.
- Nothing tells you which paths a bulk operator changed. There is no documented listing that comes back from a rewrite.
- Nothing says whether the render output path, the compositor File Output directory, bake output paths or collection-export paths are treated as "external files" by the bulk operators. They are recorded locations, but they are places written to rather than files referenced.
- The interaction between path templates and these means is undocumented. The manual says templates are substituted "internally at time of use" and that "the template syntax will remain as-is in the path field" (<https://docs.blender.org/manual/en/5.1/files/file_paths.html>, read 2026-07-30), but nothing says what Make Paths Relative or `path_remap` do to a path containing `{blend_name}`.
- Nothing describes reading or rewriting the locations recorded in a `.blend` **without loading it into Blender**, beyond the one passing mention of Blender Asset Tracer.

### Where I looked, for the parts I am calling absent

Searched across the whole of `manual_5.1`, `api_5.1`, `release_notes_5.1` and `cycles_5.1`:

- `blend_paths` — three hits, all the one API page plus two index pages.
- `make_paths_absolute`, `make_paths_relative`, `find_missing_files`, `report_missing_files` — `bpy.ops.file`, the index pages, and one manual page (the File menu on the topbar page). The manual has no dedicated page for these.
- `Make Paths Absolute`, `Make Paths Relative`, `Find Missing Files`, `Report Missing Files`, `External Data` — only `interface/window_system/topbar.txt` and `files/blend/packed_data.txt`.
- `foreach_path` — only `release_notes_5.1/corrective_releases.md`. ~~Nothing in the API documentation, so there is no exposed form of it.~~ **Corrected by the citation check (Opus 5, 2026-08-06): there is an exposed form, and this search missed it by looking for the internal name.** `bpy.data.file_path_foreach(visit_path_fn, *, subset=None, visit_types=None, flags={'SKIP_PACKED', 'SKIP_WEAK_REFERENCES'})` — "Call visit_path_fn for the file paths used by all ID data-blocks in current bpy.data" — is documented at `https://docs.blender.org/api/5.1/bpy.types.BlendData.html`, read 2026-07-30, on the same page as `file_path_map`. The observed section below reports it present and working in 5.1. Neither call appears in the means list above; both are sent back for a fresh researcher to write up.
- `blend_render_info` — no hits anywhere. There is no documented standalone reader in these sources.
- `asset tracer`, `Blender Asset Tracer`, `blender-asset-tracer`, `bat_pack` — only the one manual page named above.
- `path_remap` — `bpy.types.BlendDataLibraries` and one bugfix line. `relative_remap` — `bpy.ops.wm` only. `remap_relative`, `repath` — no hits.
- `remap` across the manual — the hits are Remap Users in the Outliner (replacing a data-block, not a path), remapping of time and of colour. Checked `editors/outliner/editing.txt` directly: "Remap Users — Replaces all usages of the selected data-block by a different one." Not a path means. Ruled out.
- `broken link`, `missing file`, `search path`, `external files`, `dependencies` across the manual — the path-relevant hits are the topbar page, `files/blend/packed_data.txt`, `files/blend/open_save.txt`, `files/linked_libraries/link_append.txt`, `editors/preferences/save_load.txt` and `editors/preferences/file_paths.txt`, all used above. Checked `editors/video_sequencer/sequencer/navigating.txt` ("Reloads external files and refreshes the current frame preview") and ruled it out: reloading, not listing or rewriting.
- Every `bpy.types.*` page in `api_5.1` was scanned for a property named exactly `filepath`; 27 pages carry one. **Citation check (Opus 5, 2026-08-06): 25 of the 27 are listed above, not all 27.** The two that are not are `BlendData.filepath` — "Path to the .blend file (default "", readonly, never None)", which is the open file's own location and read-only — and `PointCacheItem.filepath`.
- Every `bpy.ops.file.*` entry was read end to end; the full set is in `bpy.ops.file`.
- `cycles_5.1` was searched for `FILE_PATH` and `DIR_PATH`: six hits, five of them operators' own arguments in `cycles_5.1/operators.py` (which are settings of a running operator, not stored in the `.blend`), and one in `cycles_5.1/osl.py` quoted above. **Citation check (Opus 5, 2026-08-06):** the five split two ways — two on `CYCLES_OT_denoise_animation` at lines 41 and 47, and three on `CYCLES_OT_merge_images` at lines 123, 129 and 135. The sentence used to put all five on the Denoise-Animation operator. Cycles' add-on files declare no scene-level or object-level file-path property of their own.
- Blender's requirements page (<https://www.blender.org/download/requirements/>, read 2026-08-01) does not name Blender 5.1 anywhere, so I have taken nothing from it.

---

### Outside the interface

**Brought up to date by the hand tests of 2026-08-06 and 2026-08-07 (see the two observed sections below).** Every means named in this answer that has a Python form was run with no window at all, on the rented Linux machine, in this version. The two bulk sweeps, the missing-file search, the library relocation, saving elsewhere with the remap option, and the write call all returned `{'FINISHED'}` and did what they claim, with no interface present. Two operators did **not** on that machine: `bpy.ops.image.replace` and `bpy.ops.sequencer.change_path` both refused, naming a failed context check. The Mac sitting of 2026-08-07 took those two further, and they part company. **`bpy.ops.image.replace` needs a window**: with one on screen it returned `{'FINISHED'}` and rewrote the recorded path; with no window it refused on the Mac too, in every version. **`bpy.ops.sequencer.change_path` does not need a window**: with no window at all, given the file's name and the sequencer's own scene in the context, it returned `{'FINISHED'}` and rewrote the recorded folder and file, in every version. So this line is answered for everything here except that one operator, and that one is answered the other way. The words below are the researcher's own and are left as they were written.


**Label: mixed — see each part.**

**Documented.** Blender can be run without its interface, and Python scripts can be run in that state:

> "blender myscene.blend --background --python myscript.py"

Source: the Python API documentation, <https://docs.blender.org/api/5.1/info_tips_and_tricks.html>. Read 2026-07-30. The manual's entry for the switch: "-b, --background — Run in background (often used for UI-less rendering)." (<https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html>, read 2026-07-30.) A script can tell which state it is in: "bpy.app.background — Boolean, True when blender is running without a user interface (started with -b)" (<https://docs.blender.org/api/5.1/bpy.app.html>, read 2026-07-30).

**Documented for point 10.** The `-o` switch is a command-line switch and the manual's own examples of it use `--background`.

**Not in the sources for points 3, 4, 5, 6, 8 and 9.** None of the operator pages say whether these operators run when there is no interface. Two specific worries the sources raise but do not settle:

- Find Missing Files is described in the manual as opening a file browser — "A File Browser will show up." It also has a `directory` argument, which reads as a way to supply the answer the browser would have collected, but the documentation does not say that calling it with `directory` set works with no interface present. Unsettled.
- Relocating a library is described in the manual only as an Outliner right-click. The `bpy.ops.wm.lib_relocate` form takes `library` and `filepath` as arguments, which reads as a way to do it without the Outliner, but again the documentation does not say so. And `bpy.ops.wm.id_linked_relocate` carries an explicit warning: "Currently only designed as an internal operator, not directly exposed to the user."

**Points 1, 2, 7 and 11 have no documented dependence on the interface at all** — they are module functions, data properties and a write method, none of which the documentation ties to a window, an editor or a cursor. That is an absence of a stated dependence rather than a statement that they work headless, so treat it as unsettled too.

This gap matters, and the hand-tests below are aimed straight at it.

### Operating system

**Brought up to date by the hand tests of 2026-08-06 and 2026-08-07 (see the two observed sections below).** Everything in this answer that could be run was run on the rented Linux machine, so **Linux is settled** for it. **macOS is now answered for the tests the Mac sitting ran**, and they are the ones this line turns on: the same file, built the same way, lists the same paths on the two machines — twenty-five of the twenty-seven printed lines are identical character for character. Two are not. The flat path list carries one extra entry on the Mac, a path into Blender's own application folder. And **whether capital letters in a name matter is answered differently on the two machines**: asked for its own image file with the name written in capitals, the Mac found it and the Linux machine did not. *(Citation check, 2026-08-06: narrowed. That check asks the operating system directly — `os.path.exists` on a name written in capitals — with none of Blender's own path machinery in it. It settles that the two file systems differ on letter case; it settles nothing about the means in this answer.)* Note what is still only Linux: the entries this round marked as running on both machines were in fact run on Linux alone, so for those the macOS half is untested rather than settled. **Windows stays untouched**: no Windows machine has run it, so the cross-drive question the sources raise was not answered by any test here. The words below are the researcher's own and are left as they were written.


**Label: not in the sources**, with two documented exceptions.

Nothing in the manual, the API documentation, the release notes or Cycles' files says that any of the twelve means behaves differently on one operating system than another. I searched the pages used above for operating-system names and found no such statement.

Two things the sources do say about paths and operating systems, which sit underneath every means:

> "bpy.path.basename(path)
>
> Equivalent to os.path.basename, but skips a "//" prefix.
>
> Use for Windows compatibility."

> "bpy.path.resolve_ncase(path)
>
> Resolve a case insensitive path on a case sensitive system, returning a string with the path if found else return the original path."

> "bpy.path.native_pathsep(path)
>
> Replace the path separator with the system's native os.sep."

Source: the Python API documentation, <https://docs.blender.org/api/5.1/bpy.path.html>. Read 2026-07-30.

Those three exist because path text, separators and letter case differ between systems. **Label: inferred** — from the existence and wording of `resolve_ncase` in particular, a path recorded on a system that ignores letter case may not resolve on one that does not, which would change what Report Missing Files and Find Missing Files report. The step: the documentation describes a helper for that mismatch but never connects it to those operators. I have not tested it.

Blender's requirements page does not name 5.1, so under my instructions it says nothing about 5.1 and I have taken nothing from it.

---

### Observed — the hand tests of 2026-08-06

Everything in this section was run on **the rented Linux machine — Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB on driver 580.95.05, an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory and no display server at all**. The Blender was `/opt/blender/5.1/blender`, which reported itself as **5.1.2**, hash `ec6e62d40fa9`. Every run used `-b` (no window) and `--factory-startup`, and every run had `BLENDER_USER_RESOURCES` pointed at `/root/reading-a-file/res-5.1`, a settings folder made for this round and for nothing else — never at the real one. All work was done inside `/root/reading-a-file/`. The machine is shared with other topics' tests, so nothing here says how long anything took. Its graphics card could not be questioned during this sitting (`nvidia-smi` answered `Failed to initialize NVML: Driver/library version mismatch`); no test below touches the card. The full list of tests, with the set-up and the command for each, is `Audits/Blender Research/Reading a file/Hand tests — added questions.md`. **Nothing below was run on the Creator's Mac**, so nothing below settles macOS; a second sitting does that half.

#### What the machinery is called here, and what its switches are (list entry 1)

Two scripts were run with no file loaded, asking whether each named call exists and what it accepts. What came back, word for word:

```
PROBE version 5.1.2
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
PROBE2 version 5.1.2
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
READ | Image.filepath                           | /root/reading-a-file/work/many/5.1/assets/probe.png
READ | Image.filepath_raw                       | /root/reading-a-file/work/many/5.1/assets/probe.png
READ | Sound.filepath                           | /root/reading-a-file/work/many/5.1/assets/probe.wav
READ | VectorFont.filepath                      | /root/reading-a-file/work/many/5.1/assets/probe.woff2
READ | Text.filepath                            | /root/reading-a-file/work/many/5.1/assets/probe.txt
READ | Volume.filepath                          | /root/reading-a-file/work/many/5.1/assets/probe.vdb
READ | MovieClip.filepath                       | /root/reading-a-file/work/many/5.1/assets/probe.png
READ | CacheFile.filepath                       | /root/reading-a-file/work/many/5.1/assets/probe.abc
READ | Library.filepath                         | /root/reading-a-file/work/many/5.1/lib.blend
READ | RenderSettings.filepath                  | /root/reading-a-file/work/many/5.1/assets/render_out/
READ | BakeSettings.filepath                    | /root/reading-a-file/work/many/5.1/assets/bake_out/
READ | CompositorNodeOutputFile base path       | /root/reading-a-file/work/many/5.1/assets/node_out/
READ | PointCache.filepath (cloth)              | /root/reading-a-file/work/many/5.1/assets/pointcache/
READ | FluidDomainSettings.cache_directory      | /root/reading-a-file/work/many/5.1/assets/fluidcache/
READ | NodesModifier.bake_directory             | /root/reading-a-file/work/many/5.1/assets/gnbake/
READ | MultiresModifier.filepath                | 
READ | MeshCacheModifier.filepath               | /root/reading-a-file/work/many/5.1/assets/meshcache.mdd
READ | OceanModifier.filepath                   | /root/reading-a-file/work/many/5.1/assets/oceanbake/
READ | ShaderNodeScript.filepath                | /root/reading-a-file/work/many/5.1/assets/probe.osl
READ | ShaderNodeTexIES.filepath                | /root/reading-a-file/work/many/5.1/assets/probe.ies
READ | Camera.custom_filepath                   | /root/reading-a-file/work/many/5.1/assets/camshader.osl
READ | custom property on an object             | /root/reading-a-file/work/many/5.1/assets/custom_prop.png
READ | ImageStrip.directory                     | /root/reading-a-file/work/many/5.1/assets/stripdir/
READ | StripProxy.directory (sequencer)         | /root/reading-a-file/work/many/5.1/assets/proxydir/
```

And the listing means:

```
LIST blend_paths()                 = ['/opt/blender/5.1/5.1/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/root/reading-a-file/work/many/5.1/assets/camshader.osl', '/root/reading-a-file/work/many/5.1/assets/fluidcache/', '/root/reading-a-file/work/many/5.1/assets/meshcache.mdd', '/root/reading-a-file/work/many/5.1/assets/oceanbake/', '/root/reading-a-file/work/many/5.1/assets/probe.abc', '/root/reading-a-file/work/many/5.1/assets/probe.ies', '/root/reading-a-file/work/many/5.1/assets/probe.osl', '/root/reading-a-file/work/many/5.1/assets/probe.png', '/root/reading-a-file/work/many/5.1/assets/probe.png', '/root/reading-a-file/work/many/5.1/assets/probe.txt', '/root/reading-a-file/work/many/5.1/assets/probe.vdb', '/root/reading-a-file/work/many/5.1/assets/probe.wav', '/root/reading-a-file/work/many/5.1/assets/probe.woff2', '/root/reading-a-file/work/many/5.1/assets/stripdir/probe.png', '/root/reading-a-file/work/many/5.1/lib.blend']
LIST blend_paths(packed=True)      = ['/opt/blender/5.1/5.1/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/root/reading-a-file/work/many/5.1/assets/camshader.osl', '/root/reading-a-file/work/many/5.1/assets/fluidcache/', '/root/reading-a-file/work/many/5.1/assets/meshcache.mdd', '/root/reading-a-file/work/many/5.1/assets/oceanbake/', '/root/reading-a-file/work/many/5.1/assets/probe.abc', '/root/reading-a-file/work/many/5.1/assets/probe.ies', '/root/reading-a-file/work/many/5.1/assets/probe.osl', '/root/reading-a-file/work/many/5.1/assets/probe.png', '/root/reading-a-file/work/many/5.1/assets/probe.png', '/root/reading-a-file/work/many/5.1/assets/probe.txt', '/root/reading-a-file/work/many/5.1/assets/probe.vdb', '/root/reading-a-file/work/many/5.1/assets/probe.wav', '/root/reading-a-file/work/many/5.1/assets/probe.woff2', '/root/reading-a-file/work/many/5.1/assets/stripdir/probe.png', '/root/reading-a-file/work/many/5.1/lib.blend']
LIST blend_paths(local=True)       = ['/opt/blender/5.1/5.1/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/root/reading-a-file/work/many/5.1/assets/camshader.osl', '/root/reading-a-file/work/many/5.1/assets/fluidcache/', '/root/reading-a-file/work/many/5.1/assets/meshcache.mdd', '/root/reading-a-file/work/many/5.1/assets/oceanbake/', '/root/reading-a-file/work/many/5.1/assets/probe.abc', '/root/reading-a-file/work/many/5.1/assets/probe.ies', '/root/reading-a-file/work/many/5.1/assets/probe.osl', '/root/reading-a-file/work/many/5.1/assets/probe.png', '/root/reading-a-file/work/many/5.1/assets/probe.png', '/root/reading-a-file/work/many/5.1/assets/probe.txt', '/root/reading-a-file/work/many/5.1/assets/probe.vdb', '/root/reading-a-file/work/many/5.1/assets/probe.wav', '/root/reading-a-file/work/many/5.1/assets/probe.woff2', '/root/reading-a-file/work/many/5.1/assets/stripdir/probe.png', '/root/reading-a-file/work/many/5.1/lib.blend']
LIST blend_paths(absolute=True)    = ['/opt/blender/5.1/5.1/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/root/reading-a-file/work/many/5.1/assets/camshader.osl', '/root/reading-a-file/work/many/5.1/assets/fluidcache/', '/root/reading-a-file/work/many/5.1/assets/meshcache.mdd', '/root/reading-a-file/work/many/5.1/assets/oceanbake/', '/root/reading-a-file/work/many/5.1/assets/probe.abc', '/root/reading-a-file/work/many/5.1/assets/probe.ies', '/root/reading-a-file/work/many/5.1/assets/probe.osl', '/root/reading-a-file/work/many/5.1/assets/probe.png', '/root/reading-a-file/work/many/5.1/assets/probe.png', '/root/reading-a-file/work/many/5.1/assets/probe.txt', '/root/reading-a-file/work/many/5.1/assets/probe.vdb', '/root/reading-a-file/work/many/5.1/assets/probe.wav', '/root/reading-a-file/work/many/5.1/assets/probe.woff2', '/root/reading-a-file/work/many/5.1/assets/stripdir/probe.png', '/root/reading-a-file/work/many/5.1/lib.blend']
LIST file_path_map()               = ['/root/reading-a-file/work/many/5.1/assets/camshader.osl', '/root/reading-a-file/work/many/5.1/assets/fluidcache/', '/root/reading-a-file/work/many/5.1/assets/meshcache.mdd', '/root/reading-a-file/work/many/5.1/assets/oceanbake/', '/root/reading-a-file/work/many/5.1/assets/probe.abc', '/root/reading-a-file/work/many/5.1/assets/probe.ies', '/root/reading-a-file/work/many/5.1/assets/probe.osl', '/root/reading-a-file/work/many/5.1/assets/probe.png', '/root/reading-a-file/work/many/5.1/assets/probe.txt', '/root/reading-a-file/work/many/5.1/assets/probe.vdb', '/root/reading-a-file/work/many/5.1/assets/probe.wav', '/root/reading-a-file/work/many/5.1/assets/probe.woff2', '/root/reading-a-file/work/many/5.1/assets/stripdir/probe.png', '/root/reading-a-file/work/many/5.1/lib.blend']
LIST file_path_map(include_libraries=True) = ['/root/reading-a-file/work/many/5.1/assets/camshader.osl', '/root/reading-a-file/work/many/5.1/assets/fluidcache/', '/root/reading-a-file/work/many/5.1/assets/meshcache.mdd', '/root/reading-a-file/work/many/5.1/assets/oceanbake/', '/root/reading-a-file/work/many/5.1/assets/probe.abc', '/root/reading-a-file/work/many/5.1/assets/probe.ies', '/root/reading-a-file/work/many/5.1/assets/probe.osl', '/root/reading-a-file/work/many/5.1/assets/probe.png', '/root/reading-a-file/work/many/5.1/assets/probe.txt', '/root/reading-a-file/work/many/5.1/assets/probe.vdb', '/root/reading-a-file/work/many/5.1/assets/probe.wav', '/root/reading-a-file/work/many/5.1/assets/probe.woff2', '/root/reading-a-file/work/many/5.1/assets/stripdir/probe.png', '/root/reading-a-file/work/many/5.1/lib.blend']
LIST file_path_map by data-block   = [('FluidObj', ['/root/reading-a-file/work/many/5.1/assets/fluidcache/']), ('LibColl', ['/root/reading-a-file/work/many/5.1/lib.blend']), ('LibCube', ['/root/reading-a-file/work/many/5.1/lib.blend']), ('LibMesh', ['/root/reading-a-file/work/many/5.1/lib.blend']), ('ModObj', ['/root/reading-a-file/work/many/5.1/assets/meshcache.mdd', '/root/reading-a-file/work/many/5.1/assets/oceanbake/']), ('ProbeCam', ['/root/reading-a-file/work/many/5.1/assets/camshader.osl']), ('ProbeClip', ['/root/reading-a-file/work/many/5.1/assets/probe.png']), ('ProbeFont', ['/root/reading-a-file/work/many/5.1/assets/probe.woff2']), ('ProbeImage', ['/root/reading-a-file/work/many/5.1/assets/probe.png']), ('ProbeMat', ['/root/reading-a-file/work/many/5.1/assets/probe.ies', '/root/reading-a-file/work/many/5.1/assets/probe.osl']), ('ProbeSound', ['/root/reading-a-file/work/many/5.1/assets/probe.wav']), ('ProbeText', ['/root/reading-a-file/work/many/5.1/assets/probe.txt']), ('ProbeVolume', ['/root/reading-a-file/work/many/5.1/assets/probe.vdb']), ('Scene', ['/root/reading-a-file/work/many/5.1/assets/stripdir/probe.png']), ('lib.blend', ['/root/reading-a-file/work/many/5.1/lib.blend']), ('probe.abc', ['/root/reading-a-file/work/many/5.1/assets/probe.abc'])]
LIST foreach[default] CacheFile 'probe.abc' /root/reading-a-file/work/many/5.1/assets/probe.abc
LIST foreach[default] Camera 'ProbeCam' /root/reading-a-file/work/many/5.1/assets/camshader.osl
LIST foreach[default] Image 'ProbeImage' /root/reading-a-file/work/many/5.1/assets/probe.png
LIST foreach[default] Library 'lib.blend' /root/reading-a-file/work/many/5.1/lib.blend
LIST foreach[default] MovieClip 'ProbeClip' /root/reading-a-file/work/many/5.1/assets/probe.png
LIST foreach[default] Object 'FluidObj' /root/reading-a-file/work/many/5.1/assets/fluidcache/
LIST foreach[default] Object 'ModObj' /root/reading-a-file/work/many/5.1/assets/meshcache.mdd
LIST foreach[default] Object 'ModObj' /root/reading-a-file/work/many/5.1/assets/oceanbake/
LIST foreach[default] Scene 'Scene' /root/reading-a-file/work/many/5.1/assets/stripdir/probe.png
LIST foreach[default] ShaderNodeTree 'Shader Nodetree' /root/reading-a-file/work/many/5.1/assets/probe.ies
LIST foreach[default] ShaderNodeTree 'Shader Nodetree' /root/reading-a-file/work/many/5.1/assets/probe.osl
LIST foreach[default] Sound 'ProbeSound' /root/reading-a-file/work/many/5.1/assets/probe.wav
LIST foreach[default] Text 'ProbeText' /root/reading-a-file/work/many/5.1/assets/probe.txt
LIST foreach[default] VectorFont 'ProbeFont' /root/reading-a-file/work/many/5.1/assets/probe.woff2
LIST foreach[default] Volume 'ProbeVolume' /root/reading-a-file/work/many/5.1/assets/probe.vdb
LIST foreach[default] COUNT 15
LIST foreach[flags=set()] CacheFile 'probe.abc' /root/reading-a-file/work/many/5.1/assets/probe.abc
LIST foreach[flags=set()] Camera 'ProbeCam' /root/reading-a-file/work/many/5.1/assets/camshader.osl
LIST foreach[flags=set()] Image 'ProbeImage' /root/reading-a-file/work/many/5.1/assets/probe.png
LIST foreach[flags=set()] Library 'lib.blend' /root/reading-a-file/work/many/5.1/lib.blend
LIST foreach[flags=set()] Material 'Material' /opt/blender/5.1/5.1/datafiles/assets/brushes/essentials_brushes-gp_draw.blend
LIST foreach[flags=set()] MovieClip 'ProbeClip' /root/reading-a-file/work/many/5.1/assets/probe.png
LIST foreach[flags=set()] Object 'FluidObj' /root/reading-a-file/work/many/5.1/assets/fluidcache/
LIST foreach[flags=set()] Object 'ModObj' /root/reading-a-file/work/many/5.1/assets/meshcache.mdd
LIST foreach[flags=set()] Object 'ModObj' /root/reading-a-file/work/many/5.1/assets/oceanbake/
LIST foreach[flags=set()] Scene 'Scene' /root/reading-a-file/work/many/5.1/assets/stripdir/probe.png
LIST foreach[flags=set()] ShaderNodeTree 'Shader Nodetree' /root/reading-a-file/work/many/5.1/assets/probe.ies
LIST foreach[flags=set()] ShaderNodeTree 'Shader Nodetree' /root/reading-a-file/work/many/5.1/assets/probe.osl
LIST foreach[flags=set()] Sound 'ProbeSound' /root/reading-a-file/work/many/5.1/assets/probe.wav
LIST foreach[flags=set()] Text 'ProbeText' /root/reading-a-file/work/many/5.1/assets/probe.txt
LIST foreach[flags=set()] VectorFont 'ProbeFont' /root/reading-a-file/work/many/5.1/assets/probe.woff2
LIST foreach[flags=set()] Volume 'ProbeVolume' /root/reading-a-file/work/many/5.1/assets/probe.vdb
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
AFTER | RenderSettings.filepath                  | /root/reading-a-file/work/many/5.1/assets/render_out/
AFTER | BakeSettings.filepath                    | /root/reading-a-file/work/many/5.1/assets/bake_out/
AFTER | CompositorNodeOutputFile base path       | /root/reading-a-file/work/many/5.1/assets/node_out/
AFTER | PointCache.filepath (cloth)              | /root/reading-a-file/work/many/5.1/assets/pointcache/
AFTER | FluidDomainSettings.cache_directory      | //assets/fluidcache/
AFTER | NodesModifier.bake_directory             | /root/reading-a-file/work/many/5.1/assets/gnbake/
AFTER | MultiresModifier.filepath                | 
AFTER | MeshCacheModifier.filepath               | //assets/meshcache.mdd
AFTER | OceanModifier.filepath                   | //assets/oceanbake/
AFTER | ShaderNodeScript.filepath                | //assets/probe.osl
AFTER | ShaderNodeTexIES.filepath                | //assets/probe.ies
AFTER | Camera.custom_filepath                   | //assets/camshader.osl
AFTER | custom property on an object             | /root/reading-a-file/work/many/5.1/assets/custom_prop.png
AFTER | ImageStrip.directory                     | //assets/stripdir/
AFTER | StripProxy.directory (sequencer)         | /root/reading-a-file/work/many/5.1/assets/proxydir/
```

Reading that off, without adding anything to it: the ones whose recorded text changed were **CacheFile.filepath, Camera.custom_filepath, FluidDomainSettings.cache_directory, Image.filepath, Image.filepath_raw, ImageStrip.directory, Library.filepath, MeshCacheModifier.filepath, MovieClip.filepath, OceanModifier.filepath, ShaderNodeScript.filepath, ShaderNodeTexIES.filepath, Sound.filepath, Text.filepath, VectorFont.filepath, Volume.filepath**. The ones whose recorded text is the same after as before were **BakeSettings.filepath, CompositorNodeOutputFile base path, NodesModifier.bake_directory, PointCache.filepath (cloth), RenderSettings.filepath, StripProxy.directory (sequencer), custom property on an object**. (The multi-resolution modifier's line is empty in both and is not counted either way, for the reason given above.)

#### What Make Paths Absolute reaches (list entry 5)

The file the last test saved, opened again, and `bpy.ops.file.make_paths_absolute()` called on it:

```
OP make_paths_absolute = {'FINISHED'}
AFTER | Image.filepath                           | /root/reading-a-file/work/many/5.1/assets/probe.png
AFTER | Image.filepath_raw                       | /root/reading-a-file/work/many/5.1/assets/probe.png
AFTER | Sound.filepath                           | /root/reading-a-file/work/many/5.1/assets/probe.wav
AFTER | VectorFont.filepath                      | /root/reading-a-file/work/many/5.1/assets/probe.woff2
AFTER | Text.filepath                            | /root/reading-a-file/work/many/5.1/assets/probe.txt
AFTER | Volume.filepath                          | /root/reading-a-file/work/many/5.1/assets/probe.vdb
AFTER | MovieClip.filepath                       | /root/reading-a-file/work/many/5.1/assets/probe.png
AFTER | CacheFile.filepath                       | /root/reading-a-file/work/many/5.1/assets/probe.abc
AFTER | Library.filepath                         | /root/reading-a-file/work/many/5.1/lib.blend
AFTER | RenderSettings.filepath                  | /root/reading-a-file/work/many/5.1/assets/render_out/
AFTER | BakeSettings.filepath                    | /root/reading-a-file/work/many/5.1/assets/bake_out/
AFTER | CompositorNodeOutputFile base path       | /root/reading-a-file/work/many/5.1/assets/node_out/
AFTER | PointCache.filepath (cloth)              | /root/reading-a-file/work/many/5.1/assets/pointcache/
AFTER | FluidDomainSettings.cache_directory      | /root/reading-a-file/work/many/5.1/assets/fluidcache/
AFTER | NodesModifier.bake_directory             | /root/reading-a-file/work/many/5.1/assets/gnbake/
AFTER | MultiresModifier.filepath                | 
AFTER | MeshCacheModifier.filepath               | /root/reading-a-file/work/many/5.1/assets/meshcache.mdd
AFTER | OceanModifier.filepath                   | /root/reading-a-file/work/many/5.1/assets/oceanbake/
AFTER | ShaderNodeScript.filepath                | /root/reading-a-file/work/many/5.1/assets/probe.osl
AFTER | ShaderNodeTexIES.filepath                | /root/reading-a-file/work/many/5.1/assets/probe.ies
AFTER | Camera.custom_filepath                   | /root/reading-a-file/work/many/5.1/assets/camshader.osl
AFTER | custom property on an object             | /root/reading-a-file/work/many/5.1/assets/custom_prop.png
AFTER | ImageStrip.directory                     | /root/reading-a-file/work/many/5.1/assets/stripdir/
AFTER | StripProxy.directory (sequencer)         | /root/reading-a-file/work/many/5.1/assets/proxydir/
```

#### The one-pass path visitor, and whether returning a string rewrites (list entry 6)

The same file, with `bpy.data.file_path_foreach` called with its flags emptied and a function that returns each path with `/assets/` replaced by `/MOVED/`. What was visited, and what the twenty-one kinds read afterwards:

```
FEW returned = None
FEW visited ('CacheFile', 'probe.abc', '/root/reading-a-file/work/many/5.1/assets/probe.abc', '/root/reading-a-file/work/many/5.1/MOVED/probe.abc', None)
FEW visited ('Camera', 'ProbeCam', '/root/reading-a-file/work/many/5.1/assets/camshader.osl', '/root/reading-a-file/work/many/5.1/MOVED/camshader.osl', None)
FEW visited ('Image', 'ProbeImage', '/root/reading-a-file/work/many/5.1/assets/probe.png', '/root/reading-a-file/work/many/5.1/MOVED/probe.png', None)
FEW visited ('Library', 'lib.blend', '/root/reading-a-file/work/many/5.1/lib.blend', '/root/reading-a-file/work/many/5.1/lib.blend', None)
FEW visited ('Material', 'Material', '/opt/blender/5.1/5.1/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/opt/blender/5.1/5.1/datafiles/MOVED/brushes/essentials_brushes-gp_draw.blend', None)
FEW visited ('MovieClip', 'ProbeClip', '/root/reading-a-file/work/many/5.1/assets/probe.png', '/root/reading-a-file/work/many/5.1/MOVED/probe.png', None)
FEW visited ('Object', 'FluidObj', '/root/reading-a-file/work/many/5.1/assets/fluidcache/', '/root/reading-a-file/work/many/5.1/MOVED/fluidcache/', None)
FEW visited ('Object', 'ModObj', '/root/reading-a-file/work/many/5.1/assets/meshcache.mdd', '/root/reading-a-file/work/many/5.1/MOVED/meshcache.mdd', None)
FEW visited ('Object', 'ModObj', '/root/reading-a-file/work/many/5.1/assets/oceanbake/', '/root/reading-a-file/work/many/5.1/MOVED/oceanbake/', None)
FEW visited ('Scene', 'Scene', '/root/reading-a-file/work/many/5.1/assets/stripdir/probe.png', '/root/reading-a-file/work/many/5.1/MOVED/stripdir/probe.png', None)
FEW visited ('ShaderNodeTree', 'Shader Nodetree', '/root/reading-a-file/work/many/5.1/assets/probe.ies', '/root/reading-a-file/work/many/5.1/MOVED/probe.ies', None)
FEW visited ('ShaderNodeTree', 'Shader Nodetree', '/root/reading-a-file/work/many/5.1/assets/probe.osl', '/root/reading-a-file/work/many/5.1/MOVED/probe.osl', None)
FEW visited ('Sound', 'ProbeSound', '/root/reading-a-file/work/many/5.1/assets/probe.wav', '/root/reading-a-file/work/many/5.1/MOVED/probe.wav', None)
FEW visited ('Text', 'ProbeText', '/root/reading-a-file/work/many/5.1/assets/probe.txt', '/root/reading-a-file/work/many/5.1/MOVED/probe.txt', None)
FEW visited ('VectorFont', 'ProbeFont', '/root/reading-a-file/work/many/5.1/assets/probe.woff2', '/root/reading-a-file/work/many/5.1/MOVED/probe.woff2', None)
FEW visited ('Volume', 'ProbeVolume', '/root/reading-a-file/work/many/5.1/assets/probe.vdb', '/root/reading-a-file/work/many/5.1/MOVED/probe.vdb', None)
```

```
FEW AFTER | Image.filepath                           | /root/reading-a-file/work/many/5.1/MOVED/probe.png
FEW AFTER | Image.filepath_raw                       | /root/reading-a-file/work/many/5.1/MOVED/probe.png
FEW AFTER | Sound.filepath                           | /root/reading-a-file/work/many/5.1/MOVED/probe.wav
FEW AFTER | VectorFont.filepath                      | /root/reading-a-file/work/many/5.1/MOVED/probe.woff2
FEW AFTER | Text.filepath                            | /root/reading-a-file/work/many/5.1/MOVED/probe.txt
FEW AFTER | Volume.filepath                          | /root/reading-a-file/work/many/5.1/MOVED/probe.vdb
FEW AFTER | MovieClip.filepath                       | /root/reading-a-file/work/many/5.1/MOVED/probe.png
FEW AFTER | CacheFile.filepath                       | /root/reading-a-file/work/many/5.1/MOVED/probe.abc
FEW AFTER | Library.filepath                         | /root/reading-a-file/work/many/5.1/lib.blend
FEW AFTER | RenderSettings.filepath                  | /root/reading-a-file/work/many/5.1/assets/render_out/
FEW AFTER | BakeSettings.filepath                    | /root/reading-a-file/work/many/5.1/assets/bake_out/
FEW AFTER | CompositorNodeOutputFile base path       | /root/reading-a-file/work/many/5.1/assets/node_out/
FEW AFTER | PointCache.filepath (cloth)              | /root/reading-a-file/work/many/5.1/assets/pointcache/
FEW AFTER | FluidDomainSettings.cache_directory      | /root/reading-a-file/work/many/5.1/MOVED/fluidcache/
FEW AFTER | NodesModifier.bake_directory             | /root/reading-a-file/work/many/5.1/assets/gnbake/
FEW AFTER | MultiresModifier.filepath                | 
FEW AFTER | MeshCacheModifier.filepath               | /root/reading-a-file/work/many/5.1/MOVED/meshcache.mdd
FEW AFTER | OceanModifier.filepath                   | /root/reading-a-file/work/many/5.1/MOVED/oceanbake/
FEW AFTER | ShaderNodeScript.filepath                | /root/reading-a-file/work/many/5.1/MOVED/probe.osl
FEW AFTER | ShaderNodeTexIES.filepath                | /root/reading-a-file/work/many/5.1/MOVED/probe.ies
FEW AFTER | Camera.custom_filepath                   | /root/reading-a-file/work/many/5.1/MOVED/camshader.osl
FEW AFTER | custom property on an object             | /root/reading-a-file/work/many/5.1/assets/custom_prop.png
FEW AFTER | ImageStrip.directory                     | /root/reading-a-file/work/many/5.1/MOVED/stripdir/
FEW AFTER | StripProxy.directory (sequencer)         | /root/reading-a-file/work/many/5.1/assets/proxydir/
```

#### Whether walking the property marks finds every recorded location (list entry 7)

A walk over thirteen of `bpy.data`'s collections, four levels deep, collecting every text setting whose subtype is `FILE_PATH` or `DIR_PATH` or which carries one of the two path marks. The walk found the same distinct values in every version. The counts it printed here:

```
SWEEP COUNT 97
SWEEP distinct values 22
```

and the distinct values it reached, one line each, showing where the walk found them. The last two or three lines are settings that carry a path mark but hold something that is not a path — a data-block name, or the bare file-name part of an output setting — and the custom property is on the end, found only by listing custom properties by hand and not by the walk at all:

```
SWEEP bpy.data.materials['Material'].library_weak_reference.filepath = /opt/blender/5.1/5.1/datafiles/assets/brushes/essentials_brushes-gp_draw.blend
SWEEP bpy.data.scenes['Scene'].render.bake.filepath = /root/reading-a-file/work/many/5.1/assets/bake_out/
SWEEP bpy.data.cameras['ProbeCam'].custom_filepath = /root/reading-a-file/work/many/5.1/assets/camshader.osl
SWEEP custom property bpy.data.objects['ModObj']['probe_path'] = /root/reading-a-file/work/many/5.1/assets/custom_prop.png
SWEEP bpy.data.objects['FluidObj'].modifiers[].domain_settings.cache_directory = /root/reading-a-file/work/many/5.1/assets/fluidcache/
SWEEP bpy.data.objects['GNObj'].modifiers[].bake_directory = /root/reading-a-file/work/many/5.1/assets/gnbake/
SWEEP bpy.data.objects['ModObj'].modifiers[].filepath = /root/reading-a-file/work/many/5.1/assets/meshcache.mdd
SWEEP bpy.data.node_groups['ProbeComp'].nodes[].directory = /root/reading-a-file/work/many/5.1/assets/node_out/
SWEEP bpy.data.objects['ModObj'].modifiers[].filepath = /root/reading-a-file/work/many/5.1/assets/oceanbake/
SWEEP bpy.data.objects['ClothObj'].modifiers[].point_cache.filepath = /root/reading-a-file/work/many/5.1/assets/pointcache/
SWEEP bpy.data.cache_files['probe.abc'].filepath = /root/reading-a-file/work/many/5.1/assets/probe.abc
SWEEP bpy.data.materials['ProbeMat'].node_tree.nodes[].filepath = /root/reading-a-file/work/many/5.1/assets/probe.ies
SWEEP bpy.data.materials['ProbeMat'].node_tree.nodes[].filepath = /root/reading-a-file/work/many/5.1/assets/probe.osl
SWEEP bpy.data.images['ProbeImage'].filepath = /root/reading-a-file/work/many/5.1/assets/probe.png
SWEEP bpy.data.volumes['ProbeVolume'].filepath = /root/reading-a-file/work/many/5.1/assets/probe.vdb
SWEEP bpy.data.sounds['ProbeSound'].filepath = /root/reading-a-file/work/many/5.1/assets/probe.wav
SWEEP bpy.data.fonts['ProbeFont'].filepath = /root/reading-a-file/work/many/5.1/assets/probe.woff2
SWEEP bpy.data.scenes['Scene'].sequence_editor.proxy_dir = /root/reading-a-file/work/many/5.1/assets/proxydir/
SWEEP bpy.data.scenes['Scene'].render.filepath = /root/reading-a-file/work/many/5.1/assets/render_out/
SWEEP bpy.data.scenes['Scene'].sequence_editor.strips[].directory = /root/reading-a-file/work/many/5.1/assets/stripdir/
SWEEP bpy.data.libraries['lib.blend'].filepath = /root/reading-a-file/work/many/5.1/lib.blend
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
PACK PackedImage.filepath = /root/reading-a-file/work/packed/5.1/assets/packed.png
PACK PlainImage.filepath  = /root/reading-a-file/work/packed/5.1/assets/plain.png
PACK blend_paths()            = ['/opt/blender/5.1/5.1/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/opt/blender/5.1/5.1/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/root/reading-a-file/work/packed/5.1/assets/plain.png']
PACK blend_paths(packed=True) = ['/opt/blender/5.1/5.1/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/opt/blender/5.1/5.1/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/root/reading-a-file/work/packed/5.1/assets/packed.png', '/root/reading-a-file/work/packed/5.1/assets/plain.png']
PACK file_path_map()          = [('PlainImage', ['/root/reading-a-file/work/packed/5.1/assets/plain.png'])]
PACK foreach[default] = ["Image 'PlainImage' /root/reading-a-file/work/packed/5.1/assets/plain.png"]
PACK foreach[flags=set()] = ["Image 'PackedImage' /root/reading-a-file/work/packed/5.1/assets/packed.png", "Image 'PlainImage' /root/reading-a-file/work/packed/5.1/assets/plain.png", "Material 'Dots Stroke' /opt/blender/5.1/5.1/datafiles/assets/brushes/essentials_brushes-gp_draw.blend", "Material 'Material' /opt/blender/5.1/5.1/datafiles/assets/brushes/essentials_brushes-gp_draw.blend"]
PACK foreach[SKIP_PACKED] = ["Image 'PlainImage' /root/reading-a-file/work/packed/5.1/assets/plain.png", "Material 'Dots Stroke' /opt/blender/5.1/5.1/datafiles/assets/brushes/essentials_brushes-gp_draw.blend", "Material 'Material' /opt/blender/5.1/5.1/datafiles/assets/brushes/essentials_brushes-gp_draw.blend"]
```

#### Whether unpacking rewrites the recorded location (list entry 10)

Two copies of that file, unpacked two ways, so neither run could affect the other. The `on disk` listing of the second run shows the folder the first run made; the two runs used different copies of the file and the data did not cross.

```
--- unpack USE_LOCAL ---
UNPACK before, filepath = /root/reading-a-file/work/packed/5.1/assets/packed.png | packed = True
UNPACK op(USE_LOCAL) = {'FINISHED'}
UNPACK after,  filepath = //textures/packed.png | packed = False
UNPACK on disk: <W>/packed.blend
UNPACK on disk: <W>/u1.blend
UNPACK on disk: <W>/u2.blend
UNPACK on disk: <W>/assets/packed.png
UNPACK on disk: <W>/assets/plain.png
UNPACK on disk: <W>/textures/packed.png
--- unpack USE_ORIGINAL ---
UNPACK before, filepath = /root/reading-a-file/work/packed/5.1/assets/packed.png | packed = True
UNPACK op(USE_ORIGINAL) = {'FINISHED'}
UNPACK after,  filepath = /root/reading-a-file/work/packed/5.1/assets/packed.png | packed = False
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
FINDALL built FromOne = /root/reading-a-file/work/findall/5.1/one/same.png
FINDALL built FromTwo = /root/reading-a-file/work/findall/5.1/two/same.png
FINDALL save = {'FINISHED'}
FINDALL before FromOne = /root/reading-a-file/work/findall/5.1/one/same.png True
FINDALL before FromTwo = /root/reading-a-file/work/findall/5.1/two/same.png True
FINDALL op = {'FINISHED'}
FINDALL after  FromOne = /root/reading-a-file/work/findall/5.1/one/same.png
FINDALL after  FromTwo = /root/reading-a-file/work/findall/5.1/one/same.png
```

#### A path holding a template, through the listing means and both sweeps (list entry 12)

```
TMPL save = {'FINISHED'}
TMPL OPENED   render.filepath = /root/reading-a-file/work/template/5.1/assets/out_{blend_name}_####.png
TMPL OPENED   node base path  = /root/reading-a-file/work/template/5.1/assets/node_{blend_name}/
TMPL OPENED   image           = /root/reading-a-file/work/template/5.1/assets/probe.png
TMPL OPENED   blend_paths     = ['/opt/blender/5.1/5.1/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/root/reading-a-file/work/template/5.1/assets/probe.png']
TMPL OPENED   file_path_map   = [['/root/reading-a-file/work/template/5.1/assets/probe.png']]
TMPL OPENED   foreach         = ['/root/reading-a-file/work/template/5.1/assets/probe.png']
TMPL OPENED   frame_path(1)   = /root/reading-a-file/work/template/5.1/assets/out_tmpl_0001.png
TMPL op make_paths_relative = {'FINISHED'}
TMPL REL      render.filepath = /root/reading-a-file/work/template/5.1/assets/out_{blend_name}_####.png
TMPL REL      node base path  = /root/reading-a-file/work/template/5.1/assets/node_{blend_name}/
TMPL REL      image           = //assets/probe.png
TMPL REL      blend_paths     = ['//../../../../../opt/blender/5.1/5.1/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '//assets/probe.png']
TMPL REL      file_path_map   = [['//assets/probe.png']]
TMPL REL      foreach         = ['//assets/probe.png']
TMPL REL      frame_path(1)   = /root/reading-a-file/work/template/5.1/assets/out_tmpl_0001.png
TMPL op make_paths_absolute = {'FINISHED'}
TMPL ABS      render.filepath = /root/reading-a-file/work/template/5.1/assets/out_{blend_name}_####.png
TMPL ABS      node base path  = /root/reading-a-file/work/template/5.1/assets/node_{blend_name}/
TMPL ABS      image           = /root/reading-a-file/work/template/5.1/assets/probe.png
TMPL ABS      blend_paths     = ['/opt/blender/5.1/5.1/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '/root/reading-a-file/work/template/5.1/assets/probe.png']
TMPL ABS      file_path_map   = [['/root/reading-a-file/work/template/5.1/assets/probe.png']]
TMPL ABS      foreach         = ['/root/reading-a-file/work/template/5.1/assets/probe.png']
TMPL ABS      frame_path(1)   = /root/reading-a-file/work/template/5.1/assets/out_tmpl_0001.png
```

#### The four remap choices on the write call (list entry 13)

A file holding one image recorded relative and one recorded in full form, both naming files that exist. Each of the four choices wrote a new file one folder below. Each written file was then **opened** rather than linked from, because linking turns a recorded relative path into a full one on the way in and would hide the thing being asked about. Each line gives the stored text, what it works out to, and whether a file is there:

```
WREAD NONE.blend = [('AbsImage', ('/root/reading-a-file/work/write/5.1/assets/abs.png', '/root/reading-a-file/work/write/5.1/assets/abs.png', True)), ('RelImage', ('//assets/rel.png', '/root/reading-a-file/work/write/5.1/out/assets/rel.png', False))]
WREAD RELATIVE.blend = [('AbsImage', ('/root/reading-a-file/work/write/5.1/assets/abs.png', '/root/reading-a-file/work/write/5.1/assets/abs.png', True)), ('RelImage', ('//../assets/rel.png', '/root/reading-a-file/work/write/5.1/out/../assets/rel.png', True))]
WREAD RELATIVE_ALL.blend = [('AbsImage', ('//../assets/abs.png', '/root/reading-a-file/work/write/5.1/out/../assets/abs.png', True)), ('RelImage', ('//../assets/rel.png', '/root/reading-a-file/work/write/5.1/out/../assets/rel.png', True))]
WREAD ABSOLUTE.blend = [('AbsImage', ('/root/reading-a-file/work/write/5.1/assets/abs.png', '/root/reading-a-file/work/write/5.1/assets/abs.png', True)), ('RelImage', ('/root/reading-a-file/work/write/5.1/assets/rel.png', '/root/reading-a-file/work/write/5.1/assets/rel.png', True))]
```

#### Whether the write call may be pointed at a file that already exists (list entry 14)

```
WRITE existing file size = 27
WRITE onto existing file = no error raised
WRITE existing file size before = 27 after = 138141
WRITE existing file first bytes after = b'BLENDER17-01'
```

#### What Save As with Remap Relative moves (list entry 15)

```
SAVEAS save2 = {'FINISHED'}
SAVEAS OPENED   RelImage = //assets/rel.png
SAVEAS OPENED   AbsImage = /root/reading-a-file/work/saveas/5.1/assets/abs.png
SAVEAS OPENED   render   = //renders/
SAVEAS op = {'FINISHED'}
SAVEAS AFTER    RelImage = //../assets/rel.png
SAVEAS AFTER    AbsImage = /root/reading-a-file/work/saveas/5.1/assets/abs.png
SAVEAS AFTER    render   = //renders/
SAVEAS new file location = /root/reading-a-file/work/saveas/5.1/elsewhere/moved.blend
```

#### Writing the library's own path, against relocating it (list entry 16)

Three files: `B.blend` holding an object called `CubeFromB`, `C.blend` holding one called `CubeFromC`, and `A.blend` linking `B`'s collection. Two separate runs from the same `A.blend` — the first assigns the path and then calls `reload()`, the second calls the relocate operator:

```
RELOC OPENED   library = /root/reading-a-file/work/reloc/5.1/B.blend
RELOC OPENED   objects = [('Camera', None), ('Cube', None), ('CubeFromB', '/root/reading-a-file/work/reloc/5.1/B.blend'), ('Light', None)]
RELOC ASSIGNED library = /root/reading-a-file/work/reloc/5.1/C.blend
RELOC ASSIGNED objects = [('Camera', None), ('Cube', None), ('CubeFromB', '/root/reading-a-file/work/reloc/5.1/C.blend'), ('Light', None)]
RELOC reload() = returned
RELOC RELOADED library = /root/reading-a-file/work/reloc/5.1/C.blend
RELOC RELOADED objects = [('Camera', None), ('Cube', None), ('CubeFromC', '/root/reading-a-file/work/reloc/5.1/C.blend'), ('Light', None)]
RELOC OPENED   library = /root/reading-a-file/work/reloc/5.1/B.blend
RELOC OPENED   objects = [('Camera', None), ('Cube', None), ('CubeFromB', '/root/reading-a-file/work/reloc/5.1/B.blend'), ('Light', None)]
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
NOSAVE rel    image before = /root/reading-a-file/work/many/5.1/assets/probe.png
NOSAVE rel    render before = /root/reading-a-file/work/many/5.1/assets/render_out/
NOSAVE op = {'FINISHED'}
NOSAVE after, not saved, image = //../../many/5.1/assets/probe.png
RUN 2 --- the same file opened again
NOSAVE check  image before = /root/reading-a-file/work/many/5.1/assets/probe.png
NOSAVE check  render before = /root/reading-a-file/work/many/5.1/assets/render_out/
RUN 3 --- the same file with -o //NOSAVE_CLI/ and no save
NOSAVE check  image before = /root/reading-a-file/work/many/5.1/assets/probe.png
NOSAVE check  render before = //NOSAVE_CLI/
RUN 4 --- the same file opened again, no argument
NOSAVE check  image before = /root/reading-a-file/work/many/5.1/assets/probe.png
NOSAVE check  render before = /root/reading-a-file/work/many/5.1/assets/render_out/
```

#### Whether a bulk rewrite reaches the file on disk without a save (list entry 18)

Runs 1 and 2 of the four just above are this test. Run 1 shows the rewrite taking hold in the running Blender — the `after, not saved` line — and run 2 shows what the file on disk still holds when it is opened again.

#### Turning a stored path into a real one, and the library it came from (list entry 19)

A library in its own folder holding an image recorded as `//libtex.png`, and a file one folder up that links it. The two folders differ on purpose, or the two calls would have no reason to differ:

```
AP own stored          = //assets/own.png
AP own abspath()       = /root/reading-a-file/work/abs/5.1/assets/own.png
AP own relpath(abs)    = //assets/own.png
AP own file exists     = True
AP linked name         = LibTex library = /root/reading-a-file/work/abs/5.1/libdir/lib.blend
AP linked stored       = //libtex.png
AP linked abspath()    = /root/reading-a-file/work/abs/5.1/libtex.png
AP linked abspath(lib) = /root/reading-a-file/work/abs/5.1/libdir/libtex.png
AP linked exists plain = False
AP linked exists lib   = True
```

#### A reference held only inside a node group, a driver or a custom property (list entry 20)

This one answers question 7 of the sheet rather than question 9, and is written into `Preparation/Blender Research/Findings/5.1/Reading a file.md` as well. Four images: one used straight from a material, one used only by an image node inside a node group, one named only as a driver's target, one held only by a custom property that points at the data-block:

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
UM file_path_map = [('Img_custom', ['/root/reading-a-file/work/um/5.1/assets/custom.png']), ('Img_driver', ['/root/reading-a-file/work/um/5.1/assets/driver.png']), ('Img_group', ['/root/reading-a-file/work/um/5.1/assets/group.png']), ('Img_plain', ['/root/reading-a-file/work/um/5.1/assets/plain.png'])]
```

#### What could not be done here (list entries 21 and 22, and the cannot-be-run section)

**Replacing the file an image or a strip is loaded from.** Both operators refuse to run with no window on this machine, when called as they are called here. That refusal is itself the result, and it is what makes this a Mac entry rather than a Linux one. Read it with the Mac section below, which asked the same two operators again: one of them wanted a window and the other wanted the file named and the sequencer's own scene put into the context, and once given those it ran with no window.

```
REPL before image = /root/reading-a-file/work/replace/5.1/first/a.png
REPL op image.replace = ERROR RuntimeError Operator bpy.ops.image.replace.poll() failed, context is incorrect
REPL after  image = /root/reading-a-file/work/replace/5.1/first/a.png
REPL before strip = /root/reading-a-file/work/replace/5.1/first/ | ['a.png']
REPL op sequencer.change_path = ERROR RuntimeError Operator bpy.ops.sequencer.change_path.poll() Context missing sequencer scene
REPL after  strip = /root/reading-a-file/work/replace/5.1/first/ | ['a.png']
```

**The Outliner's Data API view** cannot be reached here at all — the machine has no display server. It was reached on the Creator's Mac on 2026-08-07; the pictures and what they show are in the Mac section below.

**Anything on Windows.** No Windows machine has run it, so whether making paths relative works across two drives on Windows was not reached by any test here. That is why it is unanswered rather than untried. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

**What Blender Asset Tracer reaches.** It is not part of Blender and is not in `Blender Sources/`; reaching it would mean fetching software from outside the project. Not attempted, and recorded as out of scope.

**The multi-resolution modifier's own path.** The property would not take a value in any version, so no test here could act on it. Nothing above says whether it is reached; only that a path could not be put there to find out.

---

### Observed — the Mac half of the hand tests, 2026-08-07

Everything in this section was run on **the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card**. The Blender was `/Applications/Blender 5.1.2.app/Contents/MacOS/Blender`, which reported itself as **5.1.2**, hash `ec6e62d40fa9` — the same version string and the same hash the rented Linux machine reported for its own copy of this version. Every run used `--factory-startup`, and every run had `BLENDER_USER_RESOURCES` pointed at `$T/res-5.1`, a settings folder made for this sitting before it was used and for nothing else, never at the real one. Below, `$T` stands for `/private/tmp/smoozy-test/raf`, a throwaway folder outside the project, and `$S` for `/Users/artist/Documents/Smoozy hand tests/Reading a file/added-questions/scripts`, where the scripts are kept. Only one Blender of this sitting ran at a time.

**Other topics were running Blender on this same Mac while this sitting ran**, so nothing below says how long anything took or how much memory anything used, and no test below measures either.

**The scripts were written again for this sitting.** The brief said not to touch the rented Linux machine at all, and the Linux half's scripts live there, so they were rebuilt from the procedure and the printouts recorded above. They were checked against the Linux printout before anything was read from them: the twenty-four readings of the many-paths file, and the map of data-blocks to paths, came back the same character for character. That is what makes the two lines that *do* differ readable as a difference between the machines rather than a difference between two scripts.

The full list of tests, with the set-up and the command for each, is `Audits/Blender Research/Reading a file/Hand tests — added questions.md`.

#### The same file's paths on the Mac, set beside the Linux printout (list entry 8)

The many-paths file was built again on the Mac by `$S/s_buildlib.py` and then `$S/s_build.py`, its paths were made relative by `$S/s_rel.py`, which answered `OP make_paths_relative = {'FINISHED'}`, and `$S/s_cross.py` read the result:

```
BLENDER_USER_RESOURCES="$T/res-5.1" "/Applications/Blender 5.1.2.app/Contents/MacOS/Blender" -b --factory-startup \
  "$T/work/many/5.1/many_rel.blend" --python "$S/s_cross.py"
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
CROSS blend_paths   = ['//../../../../../../../../../../Applications/Blender 5.1.2.app/Contents/Resources/5.1/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', '//assets/camshader.osl', '//assets/fluidcache/', '//assets/meshcache.mdd', '//assets/oceanbake/', '//assets/probe.abc', '//assets/probe.ies', '//assets/probe.osl', '//assets/probe.png', '//assets/probe.png', '//assets/probe.txt', '//assets/probe.vdb', '//assets/probe.wav', '//assets/probe.woff2', '//assets/stripdir/probe.png', '//lib.blend']
CROSS file_path_map = [('FluidObj', ['//assets/fluidcache/']), ('ModObj', ['//assets/meshcache.mdd', '//assets/oceanbake/']), ('ProbeCam', ['//assets/camshader.osl']), ('ProbeClip', ['//assets/probe.png']), ('ProbeFont', ['//assets/probe.woff2']), ('ProbeImage', ['//assets/probe.png']), ('ProbeMat', ['//assets/probe.ies', '//assets/probe.osl']), ('ProbeSound', ['//assets/probe.wav']), ('ProbeText', ['//assets/probe.txt']), ('ProbeVolume', ['//assets/probe.vdb']), ('Scene', ['//assets/stripdir/probe.png']), ('lib.blend', ['//lib.blend']), ('probe.abc', ['//assets/probe.abc'])]
CROSS separator     = '/' | case check = True
```

Set beside the Linux printout for this same version, line by line: **twenty-five of the twenty-seven lines are identical, character for character.** That is all twenty-four readings, and the map of data-blocks to paths. **Two lines differ.**

**The flat path list.** The Mac's list holds sixteen entries; the Linux list holds fifteen. The fifteen they share are identical. The one the Mac has and Linux does not is a path into the Blender application's own folder, ending `datafiles/assets/brushes/essentials_brushes-gp_draw.blend`, recorded relative to the file. Two further readings taken on the Mac, recorded because they bound what that line can be read as. The same file **before** its paths were made relative lists that path on the Mac in full form, exactly as the Linux printout above lists its own copy of it — so the difference appears only after the sweep and the save. And after the sweep the material it hangs off reads:

```
WEAK 5.1 Material '//../../../../../../../../../../Applications/Blender 5.1.2.app/Contents/Resources/5.1/datafiles/assets/brushes/essentials_brushes-gp_draw.blend'
WEAK 5.1 ProbeMat None
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
CASE 5.1 asked for $T/work/many/5.1/assets/PROBE.PNG
CASE 5.1 exists     True
CASE 5.1 real name  ['probe.png']
```

**On this point the Mac's answer is the one to read, not the Linux one**: they are different answers about two different machines and neither speaks for the other. The Linux half's own script was not opened for this sitting, so whether it asked this in exactly these words is not established; the words above are the Mac's.

#### Replacing the file an image or a strip is loaded from, with a window (list entry 21)

The Linux half of this entry is recorded above as a refusal with no window. The Mac has a screen, so the file was built there with no window by `$S/s_replace.py` and then opened **with a window on screen**, and the two operators were called from a `--python` script through a context override. The script ends by stopping Blender itself, so nothing was left open on the Creator's desk.

```
BLENDER_USER_RESOURCES="$T/res-5.1" "/Applications/Blender 5.1.2.app/Contents/MacOS/Blender" -b --factory-startup \
  --python "$S/s_replace.py" -- build "$T/work/replace/5.1"
BLENDER_USER_RESOURCES="$T/res-5.1" "/Applications/Blender 5.1.2.app/Contents/MacOS/Blender" --factory-startup \
  "$T/work/replace/5.1/repl.blend" --python "$S/s_replace_window.py" -- "$T/work/replace/5.1"
```

The script puts `ProbeImage` in an Image editor and calls `bpy.ops.image.replace(filepath=<the second file>, relative_path=False)`; then it switches the area to the Sequence editor, selects the strip, and calls `bpy.ops.sequencer.change_path` three times — first exactly as the list wrote it, then with the file named as well, then with the sequencer's scene supplied in the context. All three calls and all three answers are printed. What came back:

```
REPL build save {'FINISHED'}
REPL window     = True
REPL version    = 5.1.2
REPL before image = /private/tmp/smoozy-test/raf/work/replace/5.1/first/a.png
REPL op image.replace = {'FINISHED'}
REPL after  image = /private/tmp/smoozy-test/raf/work/replace/5.1/second/b.png
REPL before strip = /private/tmp/smoozy-test/raf/work/replace/5.1/first/ | ['a.png']
REPL op sequencer.change_path (no files argument) = ERROR RuntimeError Operator bpy.ops.sequencer.change_path.poll() Context missing sequencer scene
REPL after  strip, first call = /private/tmp/smoozy-test/raf/work/replace/5.1/first/ | ['a.png']
REPL op sequencer.change_path (files named) = ERROR RuntimeError Operator bpy.ops.sequencer.change_path.poll() Context missing sequencer scene
REPL after  strip, second call = /private/tmp/smoozy-test/raf/work/replace/5.1/first/ | ['a.png']
REPL context has sequencer_scene = True
REPL op sequencer.change_path (sequencer scene supplied) = {'FINISHED'}
REPL after  strip, third call = /private/tmp/smoozy-test/raf/work/replace/5.1/second/ | ['b.png']
```

#### The same two operators with no window at all, on the Mac (list entry 23)

This one is not on the list as it was written; it was added because the run above showed the list had missed it. The Linux half concluded that both operators refuse to run with no window. But the windowed run above shows that what the sequencer operator wanted was the file named and the sequencer's own scene put into the context, neither of which the Linux run supplied. So the same two operators were asked again on the Mac **with no window at all**, and given both of those things:

```
BLENDER_USER_RESOURCES="$T/res-5.1" "/Applications/Blender 5.1.2.app/Contents/MacOS/Blender" -b --factory-startup \
  --python "$S/s_replace.py" -- build "$T/work/replacebg/5.1"
BLENDER_USER_RESOURCES="$T/res-5.1" "/Applications/Blender 5.1.2.app/Contents/MacOS/Blender" -b --factory-startup \
  "$T/work/replacebg/5.1/repl.blend" --python "$S/s_replace_nowindow.py" -- "$T/work/replacebg/5.1"
```

What came back:

```
REPLBG background = True
REPLBG version    = 5.1.2
REPLBG screen     = <bpy_struct, Screen("Layout") at 0xc14573220>
REPLBG areas      = 4
REPLBG before image = <W>/5.1/first/a.png
REPLBG op image.replace = ERROR RuntimeError Operator bpy.ops.image.replace.poll() failed, context is incorrect
REPLBG after  image = <W>/5.1/first/a.png
REPLBG before strip = <W>/5.1/first/ | ['a.png']
REPLBG op sequencer.change_path = ERROR RuntimeError Operator bpy.ops.sequencer.change_path.poll() Context missing sequencer scene
REPLBG after  strip = <W>/5.1/first/ | ['a.png']
REPLBG op sequencer.change_path (sequencer scene supplied) = {'FINISHED'}
REPLBG after  strip, second call = <W>/5.1/second/ | ['b.png']
```

#### The Outliner's Data API view (list entry 22)

The Mac has a screen, so this could be reached there and nowhere else. `many_rel.blend` was opened with a window, the largest area was turned into the Outliner and put into its Data API view, the interface scale was set to 0.5 so that more rows would fit in one picture, and the area was photographed with Blender's own screenshot command. Nothing was clicked. A second run assigned a new path to the image while that same view was on screen, saved, reopened the file and read the value back.

```
BLENDER_USER_RESOURCES="$T/res-5.1" "/Applications/Blender 5.1.2.app/Contents/MacOS/Blender" --factory-startup --window-geometry 0 0 2600 1500 \
  "$T/work/many/5.1/many_rel.blend" --python "$S/s_outliner_window.py" -- shot "$T/out/outl/5.1-%s.png"
BLENDER_USER_RESOURCES="$T/res-5.1" "/Applications/Blender 5.1.2.app/Contents/MacOS/Blender" --factory-startup --window-geometry 0 0 2600 1500 \
  "$T/work/many/5.1/outl.blend" --python "$S/s_outliner_window.py" -- write "$T/out/outl/5.1-%s.png"
```

What came back:

```
OUTL version       = 5.1.2
OUTL background    = False
OUTL display_mode  = DATA_API
OUTL area          = 2477 x 1503
OUTL ui_scale asked for 0.5, got 0.5
OUTL screenshot_area two_levels = {'FINISHED'} 133064 bytes
OUTL screenshot_area four_levels = {'FINISHED'} 164320 bytes
OUTL view y before = -1451.0
OUTL outliner.scroll_page = {'FINISHED'} view y now -2902.0
OUTL screenshot_area after_scroll_page = {'FINISHED'} 134743 bytes
OUTL view2d.pan = {'FINISHED'} view y now -4302.0
OUTL screenshot_area after_pan = {'FINISHED'} 134833 bytes
OUTL filter_text set to 'Filepath'
OUTL screenshot_area after_filter = {'FINISHED'} 146276 bytes
OUTL version       = 5.1.2
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

2026-08-06 — Opus 5 (Claude Code) — Reading a file, Blender 5.1

2026-08-07 — Opus 5 (Claude Code) — the Mac half of the hand tests

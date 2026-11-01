# Reading a file — added questions (Blender 5.1)

Sources consulted, and when: Blender's manual for 5.1, Blender's Python API documentation for 5.1, and Blender's 5.1 release notes — all as supplied in `Blender Sources/`, read on 2026-07-30 (the snapshot date). Cycles' own files for 5.1, read on 2026-07-30. Blender's requirements page was not needed and was not used.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

Scope: question 9 only. No other question on the sheet was answered.

---

## 9. What separate means exist for listing and rewriting the locations recorded in a `.blend` file, and which kinds of recorded location does each of those means reach?

### The reading I answered, and the other reading

The question is ambiguous in one way and I say so plainly. "Means" could be read narrowly, as the commands a person clicks inside Blender, or broadly, as anything at all that can list or rewrite those locations — clicks, Python calls, command-line switches, and tools that are not Blender. **I answered the broad reading**, because the question says "separate means" rather than "menu items", and because the sources describe several non-click routes. The narrow reading would drop the Python entries, the command-line entry and the outside-Blender entry below, and would leave only the File ▸ External Data menu, the Outliner context menu, the path fields in the editors, and the Save As option.

A second, smaller ambiguity: "the locations recorded in a `.blend` file" — I read this as the file paths a `.blend` stores that point at something else (or at where something should be written), not as the storage offsets inside the file format. Nothing in the sources describes the latter.

---

### First, the kinds of recorded location the sources name

The answer below refers back to this list. Every entry comes from the 5.1 Python API documentation, read 2026-07-30, and each page's address is `https://docs.blender.org/api/5.1/<type>.html`. Each is a separate property that holds a path.

**Not every entry below carries quoted words.** Where a description is quoted, the quote is what shows the property holds a path. Where only a name and an address are given — `ImagePackedFile.filepath`, `MovieClipProxy.directory`, `MovieStrip.filepath`, `ImageStrip.directory`, `StripProxy.directory`, `StripProxy.filepath`, `SequenceEditor.proxy_dir`, `NodesModifierBake.directory`, `CacheFileLayer.filepath`, `MeshCacheModifier.filepath`, `MultiresModifier.filepath`, `OceanModifier.filepath`, `BakeSettings.filepath`, `CollectionExport.filepath`, `ShaderNodeScript.filepath`, `ShaderNodeTexIES.filepath`, `NodeSocketStringFilePath.default_value` — nothing is quoted, and those entries rest on the page having been read rather than on words set down here.

The API documentation marks path properties with a standard phrase. The meaning of that phrase is stated in the API's own list of property flags:

> "PATH_SUPPORTS_BLEND_RELATIVE:
>
> Relative Path Support.
>
> This path supports relative prefix “//” which is expanded the directory where the current “.blend” file is located.."

— Python API, `https://docs.blender.org/api/5.1/bpy_types_enum_items/property_flag_items.html`, read 2026-07-30.

**Linked library files**

- `Library.filepath` — "Path to the library .blend file (default “”, never None, blend relative // prefix supported)" (`bpy.types.Library.html`)
- `LibraryWeakReference.filepath` — "Path to the library .blend file (default “”, never None, blend relative // prefix supported)", on a class described as "Read-only external reference to a linked data-block and its library file" (`bpy.types.LibraryWeakReference.html`)
- `BlendImportContextLibrary.filepath` — "(default “”, readonly, never None)", on a class described as "Library (blendfile) reference in a BlendImportContext data. Currently only exposed as read-only data for the pre/post blendimport handlers" (`bpy.types.BlendImportContextLibrary.html`)

**Resources that can be packed.** The manual's glossary defines the word:

> "Resource
>
> External files such as images, sounds, fonts and volumes files that can be packed into a blend-file."

— Manual, `https://docs.blender.org/manual/en/5.1/glossary/index.html`, read 2026-07-30.

- `Image.filepath` — "Image/Movie file name (default “”, never None, blend relative // prefix supported)"; and `Image.filepath_raw` — "Image/Movie file name (without data refreshing) (default “”, never None, blend relative // prefix supported)" (`bpy.types.Image.html`)
- `ImagePackedFile.filepath` (`bpy.types.ImagePackedFile.html`)
- `Sound.filepath` — "Sound sample file used by this Sound data-block (default “”, never None, blend relative // prefix supported)" (`bpy.types.Sound.html`)
- `VectorFont.filepath` — "(default “”, never None, blend relative // prefix supported)" (`bpy.types.VectorFont.html`)
- `Volume.filepath` — "Volume file used by this Volume data-block (default “”, never None, blend relative // prefix supported)"; and `VolumeGrids.frame_filepath` — "Volume file used for loading the volume at the current frame. Empty if the volume has not be loaded or the frame only exists in memory. (default “”, readonly, never None, blend relative // prefix supported)" (`bpy.types.Volume.html`, `bpy.types.VolumeGrids.html`)

**Video and clip material, which the manual says cannot be packed**

> "Not all external files can be packed
>
> Some typically heavy external files, like videos from the Sequence Editor or Movie Clips, cannot be packed in a blend-file."

— Manual, `https://docs.blender.org/manual/en/5.1/files/blend/packed_data.html`, read 2026-07-30.

- `MovieClip.filepath` — "Filename of the movie or sequence file (default “”, never None, blend relative // prefix supported)" (`bpy.types.MovieClip.html`)
- `MovieClipProxy.directory` (`bpy.types.MovieClipProxy.html`)
- `MovieStrip.filepath`, `ImageStrip.directory` (`bpy.types.MovieStrip.html`, `bpy.types.ImageStrip.html`)
- `StripProxy.directory`, `StripProxy.filepath`, `SequenceEditor.proxy_dir` (`bpy.types.StripProxy.html`, `bpy.types.SequenceEditor.html`)

**Caches and baked data**

- `PointCache.filepath` and `PointCacheItem.filepath` — "Cache file path (default “”, never None, blend relative // prefix supported)" (`bpy.types.PointCache.html`, `bpy.types.PointCacheItem.html`)
- `FluidDomainSettings.cache_directory` — "Directory that contains fluid cache files (default “”, never None, blend relative // prefix supported)" (`bpy.types.FluidDomainSettings.html`)
- `NodesModifier.bake_directory` — "Location on disk where the bake data is stored (default “”, never None, blend relative // prefix supported)"; `NodesModifierBake.directory` (`bpy.types.NodesModifier.html`, `bpy.types.NodesModifierBake.html`)
- `CacheFile.filepath` — "Path to external displacements file (default “”, never None, blend relative // prefix supported)"; `CacheFileLayer.filepath` (`bpy.types.CacheFile.html`, `bpy.types.CacheFileLayer.html`)
- `MeshCacheModifier.filepath`, `MultiresModifier.filepath`, `OceanModifier.filepath` (`bpy.types.MeshCacheModifier.html`, `bpy.types.MultiresModifier.html`, `bpy.types.OceanModifier.html`)

**Places the file is set up to write to**

- `RenderSettings.filepath` — "Directory/name to save animations, # characters define the position and padding of frame numbers (default “”, never None, blend relative // prefix supported, Supports template expressions)" (`bpy.types.RenderSettings.html`)
- `CompositorNodeOutputFile.directory` — "The directory where the image will be written (default “”, never None, blend relative // prefix supported, Supports template expressions)" (`bpy.types.CompositorNodeOutputFile.html`)
- `BakeSettings.filepath` (`bpy.types.BakeSettings.html`)
- `DynamicPaintSurface.image_output_path` — "Directory to save the textures (default “”, never None, blend relative // prefix supported)" (`bpy.types.DynamicPaintSurface.html`)
- `CollectionExport.filepath` (`bpy.types.CollectionExport.html`)

**Scripts and other pointed-at files**

- `Text.filepath` — "Filename of the text file (default “”, never None)". Note: no "blend relative // prefix supported" in this one (`bpy.types.Text.html`)
- `ShaderNodeScript.filepath`, `ShaderNodeTexIES.filepath` (`bpy.types.ShaderNodeScript.html`, `bpy.types.ShaderNodeTexIES.html`)
- `NodeSocketStringFilePath.default_value` (`bpy.types.NodeSocketStringFilePath.html`)
- `Camera.custom_filepath` — "Path to the shader defining the custom camera (default “”, never None)". Note again: no "blend relative // prefix supported" (`bpy.types.Camera.html`)

Not a recorded location in a `.blend`: `PreferencesFilePaths.render_output_directory`, `.texture_directory`, `.font_directory`, `.sound_directory`, `.render_cache_directory` all carry the same path marker but sit on Preferences, described as "Default paths for external files" (`bpy.types.PreferencesFilePaths.html`). They are settings of the installed Blender, not of a file. **That exclusion is inferred, not read** — the step is that a property sitting on Preferences belongs to the installed Blender rather than to any one file. No quoted words say that Preferences are not stored in a `.blend`.

**Label for this list: documented**, for the property names and for the descriptions quoted above. Two smaller things in it are not. The Preferences exclusion just above is **inferred**. And so is the premise that every entry holds something the file *records*: `VolumeGrids.frame_filepath`'s own description says it is "Empty if the volume has not be loaded or the frame only exists in memory", which reads as the state of a loaded volume rather than as something a `.blend` stores.

---

### The means

#### A. `bpy.utils.blend_paths()` — lists, does not rewrite

> "bpy.utils.blend_paths(*, absolute=False, packed=False, local=False)
>
> Returns a list of paths to external files referenced by the loaded .blend file.
>
> Parameters:
>
> absolute (bool) – When true the paths returned are made absolute.
>
> packed (bool) – When true include file paths for packed data.
>
> local (bool) – When true skip linked library paths.
>
> Returns:
>
> path list.
>
> Return type:
>
> list[str]"

— Python API, `https://docs.blender.org/api/5.1/bpy.utils.html`, read 2026-07-30.

Which kinds it reaches: the sources say "external files", and name two kinds explicitly through the switches — **paths for packed data** (off by default) and **linked library paths** (on by default, switched off by `local`). Beyond those two, the sources do not itemise what counts as an "external file" here, and in particular say nothing about whether the output paths, the cache paths or `Text.filepath` are in the returned list. Its documented return is a list of strings; nothing quoted says which data-block each string came from, and no route from a returned string back to a data-block is documented. That it therefore cannot be used to write anything back is a step a reader supplies. **Label: documented for what is quoted; inferred that it cannot be used to write; the full membership of the list is not in the sources.**

#### B. The data-block's own path property, read and written directly

Every property in the list above is a normal Python attribute on a data-block. **That those not marked `readonly` can be read and assigned is inferred, not read** — the step is that the `readonly` marker in a property description means the property cannot be written, and that its absence means it can. Nothing quoted in this document says what the marker means, and nothing quoted says that assigning to one of these properties changes what the `.blend` records. Everything else in this section rests on that step. This is the only means the sources describe that reaches a **named, individual** location rather than "all of them".

Which kinds it reaches: exactly the kinds listed above, one property at a time — library files, images (including a second, non-refreshing route via `filepath_raw`), sounds, fonts, volumes, movie clips and strips and their proxies, every cache and bake directory, every output path, script and IES and text paths. It reaches `Text.filepath` and `Camera.custom_filepath`, which do not carry the blend-relative marker. It does not reach the read-only ones (`VolumeGrids.frame_filepath`, `BlendImportContextLibrary.filepath`).

One complication the API documentation states about reading these values:

> "When using Blender data from linked libraries there is an unfortunate complication since the path will be relative to the library rather than the open blend-file. When the data block may be from an external blend-file pass the library argument from the bpy.types.ID."

— Python API, `https://docs.blender.org/api/5.1/info_gotchas_file_paths_and_encoding.html`, read 2026-07-30.

And one about reading output paths in particular:

> "This substitution happens internally at time of use (e.g. while rendering), so the template syntax will remain as-is in the path field."

— Manual, `https://docs.blender.org/manual/en/5.1/files/file_paths.html`, read 2026-07-30. The same page adds: "Currently path templates are only supported for the render output path in Scene properties and the output paths in the compositor's File Output node."

So a read of `RenderSettings.filepath` or `CompositorNodeOutputFile.directory` returns whatever template text is stored, not the path that will be used. **Label: documented** for the two complications quoted just above; **inferred** that these properties can be assigned at all, which is the whole of this means.

`bpy.path` is a companion to this means, not a means of its own: it converts path strings (`abspath`, `relpath`, `native_pathsep`, `resolve_ncase`) but writes nothing back — "This module has a similar scope to os.path, containing utility functions for dealing with paths in Blender." (`https://docs.blender.org/api/5.1/bpy.path.html`, read 2026-07-30).

#### C. RNA introspection — finds which properties are paths at all

The API documentation exposes, on every property definition, flags that say whether that property is a path:

> "is_path_output
>
> Property is a filename, filepath or directory output (default False, readonly)"

> "is_path_supports_blend_relative
>
> Property is a path which supports the “//” prefix, signifying the location as relative to the “.blend” file's directory (default False, readonly)"

> "is_path_supports_templates
>
> Property is a path which supports the “{variable_name}” variable expression syntax, which substitutes the value of the referenced variable in place of the expression (default False, readonly)"

— Python API, `https://docs.blender.org/api/5.1/bpy.types.Property.html`, read 2026-07-30. Alongside these, `subtype` — "Semantic interpretation of the property (default 'NONE', readonly)" — takes the values "FILE_PATH: File Path.", "DIR_PATH: Directory Path.", "FILE_NAME: File Name." (`https://docs.blender.org/api/5.1/bpy_types_enum_items/property_subtype_items.html`, read 2026-07-30). A struct's properties are themselves listed: `Struct.properties` — "Properties in the struct (default None, readonly)" (`https://docs.blender.org/api/5.1/bpy.types.Struct.html`).

Which kinds it reaches: any property of any type, which makes it the widest **discovery** route in the sources — but it discovers *which properties are paths*, not *which data-blocks in this file hold one*. Turning it into a list of the locations actually recorded in a given file needs a walk over the file's data-blocks, which the sources describe nowhere as a ready-made call. Rewriting then happens through means B. **Label: documented for the flags; that this can be composed into a whole-file sweep is inferred — the step is that the flags are readable on every property and every property is reachable from its data-block, but no source states the composition.**

The Outliner offers the same thing to a person clicking:

> "Data API:
>
> Lists every data-block in the file along with any properties that it might have."

— Manual, `https://docs.blender.org/manual/en/5.1/editors/outliner/interface.html`, read 2026-07-30.

#### D. File ▸ External Data ▸ Make Paths Relative / Make Paths Absolute — rewrites in bulk

> "Make Paths Relative
>
> Make all paths to external files relative to the current blend-file.
>
> Make Paths Absolute
>
> Make all paths to external files absolute (= full path from the system's root)."

— Manual, `https://docs.blender.org/manual/en/5.1/interface/window_system/topbar.html`, read 2026-07-30.

The same two as Python calls:

> "bpy.ops.file.make_paths_absolute()
>
> Make all paths to external files absolute"

> "bpy.ops.file.make_paths_relative()
>
> Make all paths to external files relative to current .blend"

— Python API, `https://docs.blender.org/api/5.1/bpy.ops.file.html`, read 2026-07-30.

Which kinds it reaches: **the sources say only "all paths to external files"** and do not itemise. They do not say whether the render output path, the compositor File Output directory, the cache and bake directories, `Text.filepath` or `Camera.custom_filepath` are included or skipped. Neither operator takes any argument, so there is no selectivity: it is all or nothing, with no way to target one data-block. **Label: documented that they exist and what they claim; the set of locations they touch is not in the sources.**

#### E. File ▸ External Data ▸ Report Missing Files — lists, and only the missing

> "Report Missing Files
>
> This option is useful to check if there are links to unpacked files that no longer exist. After selecting this option, a warning message will appear in the Info editor's header. If no warning is shown, there are no missing external files."

— Manual, same topbar page, read 2026-07-30. As Python: "bpy.ops.file.report_missing_files() / Report all missing external files" (`https://docs.blender.org/api/5.1/bpy.ops.file.html`, read 2026-07-30).

Which kinds it reaches: "all missing external files" — again not itemised. Note what the manual says about *where the answer goes*: a warning in the Info editor's header. The API page gives the operator's return as "Result of the operator call." (`https://docs.blender.org/api/5.1/bpy.ops.file.html`, read 2026-07-30); that this is the same success/failure set every operator returns is my own reading of the page, not something it states. **Label: documented** for what the operator is and for where the manual says its answer appears; **not in the sources** for any way of getting the list of missing files back as data — the searches recorded at the end of this answer turned up none.

#### F. File ▸ External Data ▸ Find Missing Files — rewrites in bulk, by searching

> "Find Missing Files
>
> In case you have broken links in a blend-file, this can help you to fix the problem. A File Browser will show up. Select the desired directory (or a file within that directory), and a search will be performed in it, recursively in all contained directories. Every missing file found in the search will be recovered. Those recoveries will be done as absolute paths, so if you want to have relative paths you will need to select Make Paths Relative."

— Manual, same topbar page, read 2026-07-30. And:

> "Recovered files might need to be reloaded. You can do that one by one, or you can save the blend-file and reload it again, so that all external files are reloaded at once."

As Python:

> "bpy.ops.file.find_missing_files(*, find_all=False, directory='', …)
>
> Try to find missing external files
>
> Parameters:
>
> find_all (bool) – Find All, Find all files in the search path (not just missing) (optional)
>
> directory (str) – Directory, Directory of the file (optional, never None)"

— Python API, `https://docs.blender.org/api/5.1/bpy.ops.file.html`, read 2026-07-30.

Which kinds it reaches: "missing external files", not itemised, and with `find_all` it goes wider — "not just missing". It writes **absolute** paths, stated outright. It is the one bulk rewriter that takes a target directory as an argument. Whether supplying that argument is enough to run it with no file browser is **not stated in the sources** — the standing line below leaves that same question open for this operator, and it is the first thing on the hand-check list. **Label: documented.**

#### G. Relocating a library — rewrites one library's path

> "bpy.ops.wm.lib_relocate(*, library='', filepath='', directory='', filename='', files=None, … relative_path=True, …)
>
> Relocate the given library to one or several others
>
> Parameters:
>
> library (str) – Library, Library to relocate (optional, never None)"

— Python API, `https://docs.blender.org/api/5.1/bpy.ops.wm.html`, read 2026-07-30.

> "bpy.ops.outliner.lib_relocate()
>
> Relocate the library under cursor"

and, in the same file, the enumerated library operation: "RELOCATE / Relocate – Select a new path for this library, and reload all its data." — Python API, `https://docs.blender.org/api/5.1/bpy.ops.outliner.html`, read 2026-07-30.

The manual's version, for a person clicking:

> "When the Outliner is in the Blender File Display Mode, a right-click on a library (a linked blend-file) will show the Relocate option. It allows to replace it by a different file. This can be used to either fix a broken linked library (e.g. because the file was moved or renamed), or to switch to a variation of the same data in a different file."

and

> "If Blender cannot find a library while loading a blend-file, it will create placeholder data-blocks to replace missing linked ones. That way, references to the missing data are not lost, and by relocating the missing library, the lost data can be automatically restored."

— Manual, `https://docs.blender.org/manual/en/5.1/files/linked_libraries/link_append.html`, read 2026-07-30.

Which kinds it reaches: **linked library paths only**, one named library at a time, and it reloads that library's data as part of the act. It has a `relative_path` argument. What that argument does is **inferred from its name** — its description is not among the words quoted here, and nothing quoted says the new path can be written either relative or absolute. A sibling exists for a single linked data-block rather than a whole library — `bpy.ops.wm.id_linked_relocate`, described as "Relocate a linked ID, i.e. select another ID to link, and remap its local usages to that newly linked data-block). Currently only designed as an internal operator, not directly exposed to the user" (`https://docs.blender.org/api/5.1/bpy.ops.wm.html`, read 2026-07-30) — that one changes which data-block is used, and the sources do not say it rewrites a stored path. **Label: documented**, except the reading of `relative_path` above, which is **inferred** from the argument's name.

#### H. Remapping on save — rewrites in bulk, as a side effect of writing the file

> "relative_remap (bool) – Remap Relative, Remap relative paths when saving to a different directory (optional)"

— on both `bpy.ops.wm.save_as_mainfile()` and `bpy.ops.wm.save_mainfile()`, Python API, `https://docs.blender.org/api/5.1/bpy.ops.wm.html`, read 2026-07-30. The page's signature for `save_as_mainfile` carries `relative_remap=True` as its default; the words carrying that default are not quoted above, so this one line rests on the page having been read rather than on words set down here.

The manual's version:

> "Remap Relative
>
> This option remaps Relative Paths (such as linked libraries and images) when saving a file in a new location."

and

> "If it is necessary to relocate a blend-file relative to its linked resources, use Blender's File Save As function which has an option to Remap Relative file links."

— Manual, `https://docs.blender.org/manual/en/5.1/files/blend/open_save.html`, read 2026-07-30.

Which kinds it reaches: the quoted words name **relative paths** and say nothing at all about absolute ones. Reading that as *only* relative paths is a step a reader supplies — silence about absolute paths is not a statement that they are left alone. The manual names linked libraries and images as examples ("such as"), which is not a complete list. **Label: documented** that the option exists and that it remaps relative paths; **inferred** that it leaves absolute ones untouched; the rest of what it reaches is **not in the sources**.

#### I. Writing a new `.blend` from Python with a remap rule

> "write(filepath, datablocks, *, path_remap='NONE', fake_user=False, compress=False)
>
> Write data-blocks into a blend file.
>
> …
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

— Python API, `https://docs.blender.org/api/5.1/bpy.types.BlendDataLibraries.html`, read 2026-07-30. The same page notes: "Indirectly referenced data-blocks will be expanded and written too."

Which kinds it reaches: "all paths" / "paths that are already relative", not itemised — but this is the one means with a *stated four-way choice* between leaving alone, remapping only the already-relative, remapping everything to relative, and making everything absolute. It rewrites into a **new file**, not the open one.

The 5.1 release notes record a fix to this exact option:

> "Bpy.data.libraries.write with path_remap='RELATIVE_ALL' does not recalculate existing relative paths"

— Release notes, `https://developer.blender.org/docs/release_notes/5.1/bugfixes/`, read 2026-07-30, in the Python section of the bugfix list. The page opens: "There are a total of 573 commits in 5.1 that fixed issues that existed in Blender 5.0 or older." **Label: documented.**

#### J. Packing and unpacking — removes and restores the reliance on a recorded location

> "bpy.ops.file.pack_all() — Pack all used external files into this .blend"
> "bpy.ops.file.unpack_all(*, method='USE_LOCAL') — Unpack all files packed into this .blend to external ones … method (Literal['USE_LOCAL', 'WRITE_LOCAL', 'USE_ORIGINAL', 'WRITE_ORIGINAL', 'KEEP', 'REMOVE'])"
> "bpy.ops.file.unpack_item(*, method='USE_LOCAL', id_name='', id_type=19785) — Unpack this file to an external file"
> "bpy.ops.file.pack_libraries() — Store all data-blocks linked from other .blend files in the current .blend file. Library references are preserved so the linked data-blocks can be unpacked again"
> "bpy.ops.file.unpack_libraries() — Restore all packed linked data-blocks to their original locations"
> "bpy.ops.file.autopack_toggle() — Automatically pack all external files into the .blend file"

— Python API, `https://docs.blender.org/api/5.1/bpy.ops.file.html`, read 2026-07-30.

The manual's unpack choices name the two destinations:

> "Use files in current directory (create when necessary)
>
> Unpacks all files in the same directory // as the blend-file, grouping them in proper folders (like ''textures'' for instance)."

> "Use files in original location (create when necessary)
>
> Unpacks all files in their original location."

and for a single file:

> "Create <local file path>
>
> Unpack the file at the proposed path, which is local to the current blend-file."

> "Remove Pack
>
> Just mark the file as unpacked, without actually writing it or reloading it from the drive."

— Manual, `https://docs.blender.org/manual/en/5.1/files/blend/packed_data.html`, read 2026-07-30.

Which kinds it reaches: **resources** — by the glossary, "images, sounds, fonts and volumes files" — plus, separately, linked libraries via the two `*_libraries` operators. Explicitly **not** videos from the Sequence Editor or Movie Clips.

Is this a means of *rewriting* recorded locations? The unpack choices offer a local destination and an original destination, which reads as the recorded location changing when the local one is chosen — but no source in 5.1 says outright "the stored path is rewritten to the new location". The nearest direct statement is a 5.1 bugfix entry about packing clearing a path:

> "External Multires .btx filepath not cleared when re-packing, reports as missing even when empty"

— Release notes, `https://developer.blender.org/docs/release_notes/5.1/bugfixes/`, read 2026-07-30. **Label: documented that the operators exist and what they pack/unpack; inferred that unpacking to a local directory rewrites the recorded location — the step is that the manual offers "current directory" and "original location" as alternative unpack destinations and Blender must record which one is in use, but no source states the write-back.**

#### K. `-o` / `--render-output` on the command line — rewrites one kind, for that run

> "-o, --render-output <path>
>
> Set the render path and file name.
>
> Use // at the start of the path to render relative to the blend-file."

and, on ordering:

> "blender --background --render-output /tmp test.blend --render-frame 1
>
> …will not render to /tmp because loading the blend-file overwrites the render output that was set."

— Manual, `https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html`, read 2026-07-30.

Which kinds it reaches: **the render output path only**. The second quote is the useful part: it establishes that this switch sets the same value that loading a `.blend` supplies, and that load order decides which wins. The sources do not say the switch writes anything back to the `.blend` on disk — only that it sets the value in the running Blender. **Label: documented.**

#### L. Tools that are not Blender

The manual states that such tools exist, and names one:

> "Since there is already tooling that can manipulate blend-files outside of Blender itself (see Blender Asset Tracer), it's possible to also create an external tool for doing such edits across blend-files."

— Manual, `https://docs.blender.org/manual/en/5.1/files/asset_libraries/introduction.html`, read 2026-07-30, in a section headed "Future Development" ▸ "Cross Blend-File Editing". The same passage states the rule this works around: "As described above, Blender itself is not allowed to write to other blend-files than the currently open one."

Which kinds it reaches: **the sources do not say.** They name the existence of the tool and nothing about which paths it lists or rewrites. Whatever the reach is, it is not established here. **Label: documented that such tooling exists and is named; its reach is not in the sources.**

#### M. Two smaller listing routes worth naming

- `bpy.ops.buttons.file_browse(...)` — "Open a file browser, hold Shift to open the file, Alt to browse containing directory" and `bpy.ops.buttons.directory_browse(...)` (`https://docs.blender.org/api/5.1/bpy.ops.buttons.html`, read 2026-07-30). This is the operator behind the folder button on any path field, so it reaches whichever single path field it was invoked from.
- Kind-specific replacements: `bpy.ops.image.replace(*, filepath='', … relative_path=True, …)` — "Replace current image by another one from disk" (`https://docs.blender.org/api/5.1/bpy.ops.image.html`) and `bpy.ops.sequencer.change_path(*, filepath='', directory='', … relative_path=True, use_placeholders=False)`, whose description in 5.1 reads in full: "Undocumented, consider contributing." (`https://docs.blender.org/api/5.1/bpy.ops.sequencer.html`). Both read 2026-07-30. Each reaches one kind — the active image, and the selected strip.
- During linking only: `BlendImportContextLibrary.filepath`, read-only, "Currently only exposed as read-only data for the pre/post blendimport handlers" (`https://docs.blender.org/api/5.1/bpy.types.BlendImportContextLibrary.html`, read 2026-07-30).

**Label for M: documented.**

#### N. Not a means — one recorded location that sits outside all of the above, observed in Cycles' files

Cycles stores the parameters of a custom OSL camera shader as custom properties, and gives a string parameter whose OSL metadata marks it as a filename the `FILE_PATH` subtype:

```
    elif datatype is str and metadata.get('widget') == 'filename':
        ui.update(subtype='FILE_PATH')
```

— `Blender Sources/cycles_5.1/osl.py`, lines 147–148 (corrected from 148–149 by the citation check, 2026-08-06), inside `def osl_param_ensure_property(ccam, param):` which begins at line 92. The owner is a custom-property group on the camera: `custom_props = cam.cycles_custom` (`osl.py`, line 333) passed as `osl_param_ensure_property(custom_props, param)` (`osl.py`, line 355). Read 2026-07-30.

Cycles passes the camera's own shader path through `bpy.path.abspath` before using it, rather than reading the stored string raw:

```
    script_path = bpy.path.abspath(filepath, library=library)
```

— `Blender Sources/cycles_5.1/osl.py`, line 199, inside `def update_external_script(report, filepath, library):` at line 192 (both corrected from 197 and 190 by the citation check, 2026-08-06); called at line 336 as `ok, oso_path, oso_file_remove = update_external_script(report, cam.custom_filepath, cam.library)`. Read 2026-07-30.

**What that call does is not established here.** Nothing quoted in this document says what `bpy.path.abspath` returns, or that it expands the `//` prefix. Reading a line of Cycles' files shows that the call is made; it does not show what a running render makes of it.

**Label: observed** for what Cycles' files say — that these lines are there, in these files, at these places. Nothing here is observed about what a render does with them.

Set against the documented fact that `Camera.custom_filepath` is described as "Path to the shader defining the custom camera (default “”, never None)" — with no "blend relative // prefix supported" — this matters for means C: **inferred**, and the step is this — a sweep that finds path properties by the `is_path_supports_blend_relative` flag would not find `Camera.custom_filepath` or `Text.filepath`, because the API documentation does not list that marker on either; yet Cycles puts the camera one through `bpy.path.abspath` rather than reading it raw, which suggests the stored string is not always taken as it stands. No source states what any bulk means does with these two, so this is a gap I am flagging, not a finding about behaviour.

For completeness: the file-path arguments in `Blender Sources/cycles_5.1/operators.py` (lines 37–47 and 119–135 — the second range corrected from 123–135 by the citation check, 2026-08-06, e.g. `input_filepath: StringProperty(name='Input Filepath', description='File path for image to denoise. If not specified, uses the render file path and frame range from the scene', default='', subtype='FILE_PATH')`) are operator arguments for the denoise and merge tools, not locations recorded in the `.blend`. Read 2026-07-30.

---

### Label (overall)

**documented**, for the existence, naming and stated wording of means A–M, with the exceptions each entry names above. **Observed**, for what Cycles' files say under N — and that is all N is; it names a location, not a means. **Not in the sources**, for the central half of the question — *which kinds of recorded location each bulk means reaches*. Blender's documentation names the bulk means with a blanket phrase ("all paths to external files", "all missing external files", "all paths", "Relative Paths (such as linked libraries and images)") and nowhere enumerates the set. Only the narrow means state their reach exactly: relocation reaches libraries, `-o` reaches the render output, `blend_paths()` names packed data and library paths as its two switchable groups, and packing reaches the glossary's "resources" and explicitly not videos or movie clips.

### Not covered

- Whether Make Paths Relative / Absolute, Report Missing Files, Find Missing Files, Remap Relative or `path_remap` touch the **output** paths (render output, File Output node directory, bake path, dynamic paint output), the **cache and bake** directories, `Text.filepath`, or `Camera.custom_filepath`. No source says either way. This is the single largest gap in the answer.
- What `bpy.utils.blend_paths()` actually contains beyond the two groups its switches name.
- ~~Whether any of these means can report *which data-block* a listed path came from. `blend_paths()` returns bare strings; `report_missing_files` is documented as putting a warning in the Info editor's header, with no data return described.~~

  **Corrected by the citation check (Opus 5, 2026-08-06).** This was wrong, and it was wrong because a means was missed. `bpy.data.file_path_map()` is documented in this very version — `https://docs.blender.org/api/5.1/bpy.types.BlendData.html`, read 2026-07-30 — as "Returns a mapping of all ID data-blocks in current bpy.data to a set of all file paths used by them", returning a "dictionary of bpy.types.ID instances, with sets of file path strings as their values", with `include_libraries` described as "Include library file paths of linked data. False by default." That is exactly a listing that names the owning data-block. The observed section below uses the call; the documented half of this answer never named it, and its means list A–N leaves it out along with `bpy.data.file_path_foreach`. Both are gaps in the search, not in the sources, and both are sent back for a fresh researcher to write up properly — this check does not write a new means entry.
- Whether packing or unpacking rewrites the stored path, and to what. Only the Multires `.btx` bugfix line touches this, and only for that one case.
- What Blender Asset Tracer lists or rewrites. The manual names it and stops.
- The order of operations when several means are combined (e.g. relocate then make-relative), and whether any of them are undoable.
- Whether any means reaches paths held inside **custom properties**, which is where Cycles puts OSL camera filename parameters.

### Where I looked, and what I searched for

Everything below is inside `Blender Sources/`, in the `5.1` copies only.

Searched across the whole 5.1 manual, the whole 5.1 API documentation, the whole 5.1 release notes and all of Cycles' 5.1 files for: `blend_paths`, `make_paths`, `make_paths_absolute`, `make_paths_relative`, `path_remap`, `relative_remap`, `remap_relative`, `Remap Relative`, `find_missing`, `find_missing_files`, `report_missing`, `report_missing_files`, `lib_relocate`, `relocate`, `remap`, `is_path_output`, `is_path_supports_blend_relative`, `path map`, `remap path`, `rewrite path`, `repath`, `path resolve`, `path_reference`, `external files`, `External Data`, `missing file`, `broken link`, `missing librar`, `Blender Asset Tracer`, `blendfile module`, `blend relative // prefix supported`, `FILE_PATH`, `DIR_PATH`, `relative path`, `filepath`, `render-output`, `Relative Path`.

Pages opened in full or in part:

- Manual: `files/file_paths`, `files/blend/packed_data`, `files/blend/open_save`, `files/linked_libraries/link_append`, `files/asset_libraries/introduction`, `interface/window_system/topbar`, `editors/outliner/interface`, `glossary/index`, `advanced/command_line/arguments`. I also listed `files/`, `files/blend/`, `files/linked_libraries/`, `files/media/` to be sure no page on this subject was missed.
- Python API: `bpy.utils`, `bpy.path`, `bpy.ops.file`, `bpy.ops.wm`, `bpy.ops.outliner`, `bpy.ops.buttons`, `bpy.ops.image`, `bpy.ops.sequencer`, `bpy.types.BlendDataLibraries`, `bpy.types.Library`, `bpy.types.LibraryWeakReference`, `bpy.types.BlendImportContextLibrary`, `bpy.types.Image`, `bpy.types.Sound`, `bpy.types.MovieClip`, `bpy.types.VectorFont`, `bpy.types.CacheFile`, `bpy.types.Volume`, `bpy.types.VolumeGrids`, `bpy.types.Text`, `bpy.types.Camera`, `bpy.types.RenderSettings`, `bpy.types.CompositorNodeOutputFile`, `bpy.types.PointCache`, `bpy.types.PointCacheItem`, `bpy.types.NodesModifier`, `bpy.types.FluidDomainSettings`, `bpy.types.SequenceEditor`, `bpy.types.DynamicPaintSurface`, `bpy.types.MovieStrip`, `bpy.types.ImageStrip`, `bpy.types.StripProxy`, `bpy.types.ID`, `bpy.types.Property`, `bpy.types.StringProperty`, `bpy.types.Struct`, `bpy_types_enum_items/property_flag_items`, `bpy_types_enum_items/property_subtype_items`, `info_gotchas_file_paths_and_encoding`, `info_gotchas_operators`, `info_advanced_blender_as_bpy`.
- Release notes 5.1: `index`, `core`, `python_api`, `pipeline_io`, `bugfixes`. Every other 5.1 release-note file was searched for path terms and produced nothing.
- Cycles 5.1: listed the folder, then read `osl.py` and `operators.py` around every `FILE_PATH` and `filepath` hit. `properties.py`, `engine.py`, `camera.py`, `ui.py`, `presets.py`, `version_update.py`, `__init__.py` were searched for `FILE_PATH`, `DIR_PATH` and `filepath` and hold none that record a location in the file.

Ruled out, and why:

- `bpy_extras.io_utils.path_reference` / `path_reference_copy` — these are exporter helpers that decide how to reference a file in an *exported* format; they do not list or rewrite what a `.blend` records. Read 2026-07-30.
- `PreferencesFilePaths.*` and the `editors/preferences/file_paths` manual page — settings of the installed Blender, not locations recorded in a file.
- `bpy.ops.outliner.id_remap` and `id_linked_relocate` — these change which data-block is used, and the sources do not describe them as rewriting a stored path.
- No pictures were needed. The only image the relevant manual pages point at is `_images/files_file_paths-invalid_path_template.png` and its tooltip companion on the Path Templates page, which show a red-highlighted field for a malformed template — nothing about listing or rewriting.
- I did not use the web, and did not need to.

### Outside the interface

**Brought up to date by the hand tests of 2026-08-06 and 2026-08-07 (see the two observed sections below).** Every means named in this answer that has a Python form was run with no window at all, on the rented Linux machine, in this version. The two bulk sweeps, the missing-file search, the library relocation, saving elsewhere with the remap option, and the write call all returned `{'FINISHED'}` and did what they claim, with no interface present. Two operators did **not** on that machine: `bpy.ops.image.replace` and `bpy.ops.sequencer.change_path` both refused, naming a failed context check. The Mac sitting of 2026-08-07 took those two further, and they part company. **`bpy.ops.image.replace` needs a window**: with one on screen it returned `{'FINISHED'}` and rewrote the recorded path; with no window it refused on the Mac too, in every version. **`bpy.ops.sequencer.change_path` does not need a window**: with no window at all, given the file's name and the sequencer's own scene in the context, it returned `{'FINISHED'}` and rewrote the recorded folder and file, in every version. So this line is answered for everything here except that one operator, and that one is answered the other way. The words below are the researcher's own and are left as they were written.


This is where the answer thins out, and honestly so.

What the sources say directly: Blender as a Python module, and background mode, are close relatives —

> "For the most part using Blender as a Python module is equivalent to running a script in background-mode (passing the command-line arguments --background or -b), however there are some differences to be aware of."

— Python API, `https://docs.blender.org/api/5.1/info_advanced_blender_as_bpy.html`, read 2026-07-30. The same page adds "Only a single .blend file can be edited at a time." and points at `BlendDataLibraries.load()` and `.write()` as the alternative that "operates on reading and writing ID data-blocks".

Means B, C and I are plain Python on data — nothing in the sources ties them to a window, and means I is documented as writing a file directly.

Means D, E, F, G, H and J are **operators**, and the sources warn generally about operators outside their usual setting:

> "Operators' poll function can fail where an API function would raise an exception giving details on exactly why."

and

> "Certain operators in Blender are only intended for use in a specific context, some operators for example are only called from the properties editor…"

— Python API, `https://docs.blender.org/api/5.1/info_gotchas_operators.html`, read 2026-07-30, which names `bpy.ops.buttons.file_browse()` among its examples of exactly that problem. Means M's browse operators are therefore the ones the sources flag as interface-bound.

Several of these operators are documented as opening a File Browser when a person runs them — Find Missing Files ("A File Browser will show up") and Relocate ("Select a new path for this library") — but both also take the destination as an argument (`directory=` on Find Missing Files, `filepath=`/`directory=`/`library=` on `wm.lib_relocate`), which is what an argument-taking operator needs in order to run without one. **Whether they actually run with no window is not stated anywhere in the sources**, and I will not guess. It is the first thing on the hand-check list below.

Means K is a command-line switch, so it exists only outside the interface — and the sources note it is unavailable to the Python-module form: "Functionality controlled by command line arguments (shown by calling blender --help) isn't accessible." (same page, read 2026-07-30).

Means L is by definition outside Blender entirely.

One difference that is documented: means E puts its answer in the Info editor's header. With no window there is no Info editor, and the sources do not say where the report goes instead.

### Operating system

**Brought up to date by the hand tests of 2026-08-06 and 2026-08-07 (see the two observed sections below).** Everything in this answer that could be run was run on the rented Linux machine, so **Linux is settled** for it. **macOS is now answered for the tests the Mac sitting ran**, and they are the ones this line turns on: the same file, built the same way, lists the same paths on the two machines — twenty-five of the twenty-seven printed lines are identical character for character. Two are not. The flat path list carries one extra entry on the Mac, a path into Blender's own application folder. And **whether capital letters in a name matter is answered differently on the two machines**: asked for its own image file with the name written in capitals, the Mac found it and the Linux machine did not. *(Citation check, 2026-08-06: narrowed. That check asks the operating system directly — `os.path.exists` on a name written in capitals — with none of Blender's own path machinery in it. It settles that the two file systems differ on letter case; it settles nothing about the means in this answer.)* Note what is still only Linux: the entries this round marked as running on both machines were in fact run on Linux alone, so for those the macOS half is untested rather than settled. **Windows stays untouched**: no Windows machine has run it, so the cross-drive question the sources raise was not answered by any test here. The words below are the researcher's own and are left as they were written.


The sources say nothing about any of these means differing by operating system. What they do say about paths in general — the first three from the Python API at `https://docs.blender.org/api/5.1/bpy.path.html`, the fourth at the address given with it — all read 2026-07-30:

- `bpy.path.native_pathsep(path)` — "Replace the path separator with the system's native os.sep."
- `bpy.path.basename(path)` — "Equivalent to os.path.basename, but skips a “//” prefix. Use for Windows compatibility."
- `bpy.path.resolve_ncase(path)` — "Resolve a case insensitive path on a case sensitive system, returning a string with the path if found else return the original path."
- "Paths are an exception to this rule since the existence of non-UTF-8 paths on the user's file system cannot be ignored." — `https://docs.blender.org/api/5.1/info_gotchas_file_paths_and_encoding.html`

All four are about handling path *strings* across systems, and are companions to means B rather than statements about means D–L. Every one of the means above is described in the sources without any operating-system qualification. That is not the same as a statement that they behave identically everywhere — **the sources do not say.**

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

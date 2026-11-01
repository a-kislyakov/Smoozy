# Reading a file — Blender 5.0 — second pass (questions 4, 7, 8)

Sources consulted: Blender 5.0 manual (text and images), Blender 5.0 Python API documentation, Blender 5.0 release notes, and Cycles' own files as shipped with Blender 5.0 — all read on **2026-07-30** (the snapshot date; used as the date read throughout).

I read only the question sheet and files inside `Blender Sources/`. I did not open any other part of this project.

I did not use any helper agents. Everything below I searched and read myself.

---

## 4. Can the places a file is set up to write to be read out of it?

**Reading of the question.** I take "the places a file is set up to write to" to mean the file paths and directories stored in a `.blend` that name where Blender will put things it writes — the render output path, the compositor's File Output node paths, bake and cache destinations, and so on. I take "read out of it" to mean read back as data by something other than a person looking at the panel. The other reading — "read out of the file *without* a Blender" — is a different question, and I answer only the reading above; where the distinction matters I say so.

### Answer

**Yes** — every such place is an ordinary readable property of the loaded file's data, and Blender 5.0 additionally marks these properties in a machine-readable way so they can be found generically rather than one by one. Two caveats are documented: the stored value may contain unexpanded template expressions, and it may carry the `//` relative prefix.

---

**4a. Render output path.**

LABEL: **documented**

> filepath
>
> Directory/name to save animations, # characters define the position and padding of frame numbers
>
> Type:
>
> string, default “”, (never None, blend relative // prefix supported, Supports template expressions)

Source: Python API documentation, https://docs.blender.org/api/5.0/bpy.types.RenderSettings.html — read 2026-07-30.

The manual's Output panel describes the same field:

> File Path
>
> Choose the location to save rendered frames.

and

> This setting expands Relative Paths
> where a // prefix represents the directory of the current blend-file.

Source: manual, https://docs.blender.org/manual/en/5.0/render/output/properties/output.html — read 2026-07-30.

I opened the picture on that page (`_images/render_output_properties_output_panel.png`, shown on the page above). LABEL: **documented**. What is visible: a panel headed "Output" whose first control is a text field containing `/tmp/` with a folder-browse button, followed by Overwrite, Placeholders, File Extensions, Cache Result, File Format (PNG), Color, Color Depth and Compression. Nothing more should be read into it than that the output location is a single editable text field at the top of the panel.

There is also a documented call that turns the stored path into the concrete path for a frame:

> frame_path(*, frame=-2147483648, preview=False, view='')
>
> Return the absolute path to the filename to be written for a given frame

Source: Python API documentation, https://docs.blender.org/api/5.0/bpy.types.RenderSettings.html — read 2026-07-30.

**4b. Compositor File Output node paths.**

LABEL: **documented**

> directory
>
> The directory where the image will be written
>
> Type:
>
> string, default “”, (never None, blend relative // prefix supported, Supports template expressions)
>
> file_name
>
> The base name of the file. Other information might be included in the final file name depending on the node options
>
> Type:
>
> string, default “”, (never None, Supports template expressions)

and

> file_output_items
>
> Type:
>
> NodeCompositorFileOutputItems bpy_prop_collection of NodeCompositorFileOutputItem, (readonly)

Source: Python API documentation, https://docs.blender.org/api/5.0/bpy.types.CompositorNodeOutputFile.html — read 2026-07-30.

The release notes state the split explicitly:

> - File Output node now takes `directory` and `file_name` as two different inputs
>   - Removed:
>     - `bpy.types.CompositorNodeOutputFile.file_slots`
>     - `bpy.types.CompositorNodeOutputFile.layer_slots`
>     - `bpy.types.CompositorNodeOutputFile.base_path`
>   - Added:
>     - `bpy.types.CompositorNodeOutputFile.directory`
>     - `bpy.types.CompositorNodeOutputFile.file_name`
>     - `bpy.types.CompositorNodeOutputFile.file_output_items`

Source: release notes, https://developer.blender.org/docs/release_notes/5.0/python_api/ — read 2026-07-30.

The node's final assembled paths are also shown in the interface:

> Output Paths
>
> Reference
>
> Panel:
>
> Sidebar ‣ Node ‣ Properties ‣ Output Paths
>
> Displays the final file path and extension for each input/layer, based on current settings.

Source: manual, https://docs.blender.org/manual/en/5.0/compositing/types/output/file_output.html — read 2026-07-30.

I opened the picture on that page (`_images/node-types_CompositorNodeOutputFile.webp`). LABEL: **documented**. What is visible: a node titled "File Output" with two stacked text fields, the upper containing `/tmp/` with a folder-browse button and the lower containing `file_name`, and one unlabelled input socket on the left.

One thing that matters for finding these nodes at all: in 5.0 the compositor tree hangs off a different property than before.

> - `scene.node_tree` was removed, use `scene.compositing_node_group` instead

Source: release notes, https://developer.blender.org/docs/release_notes/5.0/python_api/ — read 2026-07-30. The corresponding property reads:

> compositing_node_group
>
> Compositor Nodes
>
> Type:
>
> NodeTree

Source: Python API documentation, https://docs.blender.org/api/5.0/bpy.types.Scene.html — read 2026-07-30.

**4c. There is a generic marker for "this property is an output path".**

LABEL: **documented**

> is_path_output
>
> Property is a filename, filepath or directory output
>
> Type:
>
> boolean, default False, (readonly)

alongside

> is_path_supports_blend_relative
>
> Property is a path which supports the “//” prefix, signifying the location as relative to the “.blend” file’s directory

and

> is_path_supports_templates
>
> Property is a path which supports the “{variable_name}” variable expression syntax, which substitutes the value of the referenced variable in place of the expression

Source: Python API documentation, https://docs.blender.org/api/5.0/bpy.types.Property.html — read 2026-07-30.

The same three appear as property flags:

> OUTPUT_PATH:
>
> Output Path.
>
> PATH_SUPPORTS_BLEND_RELATIVE:
>
> Relative Path Support.
>
> This path supports relative prefix “//” which is expanded the directory where the current “.blend” file is located..
>
> SUPPORTS_TEMPLATES:
>
> Variable expression support.

Source: Python API documentation, https://docs.blender.org/api/5.0/bpy_types_enum_items/property_flag_items.html — read 2026-07-30.

Properties of a type can be listed:

> properties
>
> Properties in the struct
>
> Type:
>
> bpy_prop_collection of Property, (readonly)

Source: Python API documentation, https://docs.blender.org/api/5.0/bpy.types.Struct.html — read 2026-07-30.

**4d. The full set of stored paths I could find, by the flag text that marks them.**

LABEL: **documented** (each quoted description below is the API documentation's own wording; I found them by searching the whole API documentation for the phrase `blend relative // prefix supported`, which appears in exactly 38 type pages).

The ones that name a place Blender writes to:

- `RenderSettings.filepath` — "Directory/name to save animations…" (render output)
- `CompositorNodeOutputFile.directory` / `.file_name` — "The directory where the image will be written" / "The base name of the file…"
- `BakeSettings.filepath` — "Image filepath to use when saving externally"
- `NodesModifier.bake_directory` and `NodesModifierBake.directory` — "Location on disk where the bake data is stored"
- `FluidDomainSettings.cache_directory` — "Directory that contains fluid cache files"
- `PointCacheItem.filepath` — "Cache file path" (with `PointCacheItem.use_external`, "Read cache from an external location")
- `OceanModifier.filepath` — "Path to a folder to store external baked images"
- `MultiresModifier.filepath` — "Path to external displacements file" (with `is_external`, "Store multires displacements outside the .blend file, to save memory")
- `DynamicPaintSurface.image_output_path` — "Directory to save the textures"
- `SequenceEditor.proxy_dir`, `StripProxy.directory` and `MovieClipProxy.directory` — "Location to store the proxy files"
- `CollectionExport.filepath` — "The file path used for exporting"

Sources: the corresponding pages under https://docs.blender.org/api/5.0/ (for example https://docs.blender.org/api/5.0/bpy.types.BakeSettings.html, https://docs.blender.org/api/5.0/bpy.types.NodesModifier.html, https://docs.blender.org/api/5.0/bpy.types.FluidDomainSettings.html, https://docs.blender.org/api/5.0/bpy.types.PointCacheItem.html, https://docs.blender.org/api/5.0/bpy.types.OceanModifier.html, https://docs.blender.org/api/5.0/bpy.types.MultiresModifier.html, https://docs.blender.org/api/5.0/bpy.types.DynamicPaintSurface.html, https://docs.blender.org/api/5.0/bpy.types.SequenceEditor.html, https://docs.blender.org/api/5.0/bpy.types.StripProxy.html, https://docs.blender.org/api/5.0/bpy.types.MovieClipProxy.html, https://docs.blender.org/api/5.0/bpy.types.CollectionExport.html) — read 2026-07-30.

The collection exporters are described in the manual as a set-up-once, re-run-later arrangement:

> Each collection can be exported to a number of various file formats.
> These exporters are available globally, see Importing & Exporting Files,
> however, this panel streamlines the process of re-exporting the same asset(s) repeatedly.

Source: manual, https://docs.blender.org/manual/en/5.0/scene_layout/collections/collections.html — read 2026-07-30. They hang off the collection:

> exporters
>
> Export Handlers configured for the collection

Source: Python API documentation, https://docs.blender.org/api/5.0/bpy.types.Collection.html — read 2026-07-30.

**4e. What you read back is the stored text, not necessarily the final path.**

LABEL: **documented**

> This substitution happens internally at time of use (e.g. while rendering), so
> the template syntax will remain as-is in the path field.

and

> Currently path templates are only supported for the render output path in
> Scene properties and the output paths in the compositor’s File Output
> node. More filepaths will support templates in future releases.

Source: manual, https://docs.blender.org/manual/en/5.0/files/file_paths.html — read 2026-07-30.

The available variables include ones that depend on the scene and camera in use, so the same stored text can resolve differently:

> Available only in render output paths (including in the compositor File Output node):
> …
> scene_name:
>
> Name of the current scene.
>
> camera_name:
>
> Name of the current render camera.

Source: same page — read 2026-07-30. The release notes record these as newly present:

> ## More Path Template Variables
>
> The following new path template variables have been introduced
> …
> - `scene_name`
> - `camera_name`
> - `node_name`
> - `blend_name_lib`
> - `blend_dir`
> - `blend_dir_lib`

Source: release notes, https://developer.blender.org/docs/release_notes/5.0/core/ — read 2026-07-30.

And a stored path that is malformed as a template is documented as blocking the write:

> Depending on the path, template errors may prevent certain actions. For
> example, if there are errors in the render output path, then rendering an
> animation will fail with an error message indicating the path errors.

Source: manual, https://docs.blender.org/manual/en/5.0/files/file_paths.html — read 2026-07-30.

For the `//` prefix, the API documentation supplies the expansion call, and warns that for data from a linked file the prefix is relative to that file, not the open one:

> When using Blender data from linked libraries there is an unfortunate complication
> since the path will be relative to the library rather than the open blend-file.
> When the data block may be from an external blend-file pass the library argument from the bpy.types.ID.
>
> >>> bpy.path.abspath(image.filepath, library=image.library)

Source: Python API documentation, https://docs.blender.org/api/5.0/info_gotchas_file_paths_and_encoding.html — read 2026-07-30.

**4f. Cycles' own files show the render output path being read back and turned into per-frame paths.**

LABEL: **observed**

Exact lines, from `Blender Sources/cycles_5.0/operators.py`, inside `CYCLES_OT_denoise_animation.execute()` (lines 72, 77, 80–81):

```
            in_filepath = scene.render.filepath
```
```
            original_filepath = scene.render.filepath

            for frame in range(scene.frame_start, scene.frame_end + 1):
                scene.render.filepath = in_filepath
                filepath = scene.render.frame_path(frame=frame)
```

Source: Cycles' own files, `cycles_5.0/operators.py`, lines 72 and 77–81 — read 2026-07-30.

What this shows by reading alone (**observed**): Cycles' shipped code reads the render output path off the loaded scene, and calls `frame_path(frame=…)` to get a per-frame path. What it does *not* show is what those calls return at run time — I did not run Blender. The step from "the code reads this property" to "this is how you enumerate a file's output destinations" is a step, so treat the general claim as **inferred** from 4a–4d rather than observed here.

### Not covered

- Whether `frame_path()` expands template expressions, or returns them unexpanded. The API text says only "Return the absolute path to the filename to be written for a given frame"; the manual says substitution happens "at time of use". Neither settles whether `frame_path()` counts as a time of use.
- Whether `bpy.data.file_path_map()` (see question 8) includes output paths as well as input ones. Its wording is "a set of all file paths used by them", which I cannot stretch to cover write destinations without supplying the assumption myself.
- Whether every writer in Blender carries an `is_path_output` flag. I found the flag documented and I found the twelve-odd path properties above by text search, but the sources nowhere publish a complete list of output-path properties, so I cannot claim my list is exhaustive.
- Reading these places out of a `.blend` **without** a Blender to load it. Nothing I read describes doing that; the release notes do say the file format's block headers changed in 5.0 and that "any code that is parsing .blend files has to be updated" (https://developer.blender.org/docs/release_notes/5.0/core/, read 2026-07-30), but that is a statement about parsers, not a way to read output paths.
- Whether the value read back is the value that will actually be used at render time: the command line can set it after loading (see below), and the sources do not describe any record of that override inside the file.

### Outside the interface

Holds. LABEL: **inferred**, from two documented things. First, background operation is documented: `-b, --background` — "Run in background (often used for UI-less rendering)" (manual, https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html, read 2026-07-30) — and the API documentation states that using Blender as an importable module "is equivalent to running a script in background-mode (passing the command-line arguments --background or -b)" (https://docs.blender.org/api/5.0/info_advanced_blender_as_bpy.html, read 2026-07-30). Second, none of the properties above is documented as belonging to the interface or to a window/screen type; they hang off scene, node, modifier and collection data. The step I supplied: data properties that are not interface data are readable wherever `bpy.data` is readable.

One documented difference that matters when driving Blender from outside: the command line can replace the render output path, and order decides who wins.

> blender --background --render-output /tmp test.blend --render-frame 1
>
> …will not render to /tmp because loading the blend-file overwrites the render output that was set.

Source: manual, https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html — read 2026-07-30. Also documented for that argument:

> You can use path templating features such as {blend_name} in the path.

Source: same page — read 2026-07-30.

A second documented difference concerns the File Output node specifically:

> - The **File Output** node now only appends the frame number to the image name when doing an
>   animation render. Rendering through the command line always does an animation render, so no change
>   of behavior is expected. Rendering using the `render.render()` operator when `animation=False`
>   will see the change of behavior…

Source: release notes, https://developer.blender.org/docs/release_notes/5.0/compositor/ — read 2026-07-30. This is about the file names produced, not about reading the stored path, but it means the same stored path yields different final names depending on how the render was started.

### Operating system

The sources do not say that reading these properties differs by operating system, and I found no statement that it does. Three OS-dependent points around the *values* are documented:

- `bpy.path.native_pathsep(path)` — "Replace the path separator with the systems native os.sep." (https://docs.blender.org/api/5.0/bpy.path.html, read 2026-07-30)
- `bpy.path.resolve_ncase(path)` — "Resolve a case insensitive path on a case sensitive system, returning a string with the path if found else return the original path." (same page, read 2026-07-30)
- Windows file-name length: "Windows (NTFS) only supports up to 255 chars in its file names. This limit can now more easily be reached with longer data-block names, when they are used as part of a generated file name." (release notes, https://developer.blender.org/docs/release_notes/5.0/core/, read 2026-07-30)

LABEL for the OS line as a whole: **not in the sources** that reading differs; **documented** that the three points above are OS-sensitive.

### Where I looked (question 4)

Searched across the whole of `manual_5.0`, `api_5.0` and `release_notes_5.0` for: `filepath`, `base_path`, `directory`, `output path`, `Output Path`, `output directory`, `destination`, `where to save`, `written to`, `write to disk`, `save path`, `cache_directory`, `bake_directory`, `proxy_dir`, `filepath_raw`, `use_external`, `template expression`, `path template`, `path templating`, `blend_name`, `file output`, `is_path_output`, `PATH_SUPPORTS`, `OUTPUT_PATH`, and the exact API phrase `blend relative // prefix supported` (which gave the 38-page inventory in 4d).

Pages opened in full: manual `files/file_paths.txt`, `render/output/properties/output.txt`, `render/output/animation.txt`, `compositing/types/output/file_output.txt`, `advanced/command_line/arguments.txt` (output-related parts), `advanced/command_line/render.txt`, `scene_layout/collections/collections.txt`; API `bpy.types.RenderSettings.txt`, `bpy.types.CompositorNodeOutputFile.txt`, `bpy.types.NodeCompositorFileOutputItem.txt`, `bpy.types.Property.txt`, `bpy.types.Struct.txt`, `bpy.types.Scene.txt` (compositing property), `bpy.utils.txt`, `bpy.path.txt`, `info_gotchas_file_paths_and_encoding.txt`, `bpy_types_enum_items/property_flag_items.txt`, plus the path-property section of each of the twelve types listed in 4d; release notes `core.md`, `compositor.md`, `python_api.md`, `pipeline_io.md`. Images opened: `render_output_properties_output_panel.png`, `node-types_CompositorNodeOutputFile.webp`.

Cycles' files: searched all `.py` under `cycles_5.0` for `filepath`, `directory`, `output_path`, `FILE_PATH`, `DIR_PATH`; read `operators.py` lines 25–120.

Ruled out and why: `PreferencesFilePaths` (a preference, not stored in the `.blend`); `Image.filepath`, `Sound.filepath`, `MovieClip.filepath`, `VectorFont.filepath`, `Text.filepath`, `CacheFile.filepath`, `Volume.filepath`, `ImageStrip`/`MovieStrip`, `ShaderNodeScript.filepath`, `ShaderNodeTexIES.filepath`, `MeshCacheModifier.filepath` (places the file reads *from*, not writes to — they belong to a different question); `bpy.ops.wm.*_export` operators (an export you invoke, not a place recorded in the file — except via `CollectionExport`, which I did include).

### What the hand tests settled for question 4

**Observed (2026-08-04).** Tests were run on the Creator's own computer — a MacBook Pro with an M1 Pro chip running macOS 26.5.2 — with Blender **5.0.1** (hash a3db93c5b259), started with no window, with `--factory-startup`, and with a throwaway settings folder. The full record, with the exact commands and the raw output, is in `Preparation/Blender Research/Findings/5.0/Reading a file.md` under question 4. In short:

- `frame_path()` returns a path with the templates worked out, while the stored string keeps its braces. This settles the first item under "Not covered" above.
- A malformed template makes `frame_path()` raise. `{scene_name}` resolved to the scene's name.
- Neither `bpy.utils.blend_paths()`, nor `bpy.data.file_path_map()`, nor a callback given to `bpy.data.file_path_foreach()` returned the render output path or the File Output node's paths on a file that had both set. This settles the second item under "Not covered" above.
- The output-path marker came back `True` for `RenderSettings.filepath` and `False` for `Image.filepath`, `Library.filepath` and `PointCache.filepath`.
- The File Output node's stored destinations read back as `directory` plus `file_name` plus one item per input. An input named `passes/diffuse` was stored as `passes_diffuse` — the slash did not survive. A render of one frame wrote a single file, `out/shot0001.exr`, rather than one file per input.
- All of it was done with no window, so the "Outside the interface" line above holds by test as well as by inference.

**Observed (2026-08-05).** The Creator then opened the node's sidebar in Blender's own window, on the same computer, and photographed it. In 5.0 that sidebar does hold a section headed `Output Paths`, and it holds **one** line for the whole node — `/Users/artist/Documents/Smoozy hand tests/Reading a file/13/out/shot.exr` — rather than one line per input. The same panel shows the two path fields reading `//out/` and `shot`, a two-button row reading `Image` and `Multi-Layer EXR` with `Multi-Layer EXR` selected, and a `Layers` list holding `passes_diffuse` and `beauty`. The full record is in `Preparation/Blender Research/Findings/5.0/Reading a file.md` under question 4. It was run on a Mac, so it settles macOS.

**Observed (2026-08-06).** Every one of those tests was then run again on a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH — with the same Blender build, no window, `--factory-startup`, and this topic's own settings folder. The sidebar was reached there too, by driving Blender's window on that machine's own display and taking Blender's own screenshot. **Every result agreed with the Mac's**, point for point: the stored braces and the worked-out path, the raise on a malformed template, `{scene_name}` resolving to the scene's name, the three whole-file path lists returning neither output path, the four `is_path_output` answers, the node's `directory` and `file_name` with the slash coming back as `passes_diffuse`, a render writing one file for the whole node, and a sidebar that **does** hold a panel headed `Output Paths`. **One thing that Linux run did not read:** that panel was closed there and on the day of the run nothing on that machine could click it open, so what it holds rested on the Creator's screenshot alone.

> **Note from the citation check of the Linux hand tests, 2026-08-06.** A tool for moving the pointer was put on that machine afterwards and the check tried to open the panel with it, without success it could vouch for — a click on that machine could not be confirmed against any state Blender will report. So the panel's contents still rest on the Creator's Mac screenshot alone. The full record is in `Preparation/Blender Research/Findings/5.0/Reading a file.md` under question 4, kept separate from the Mac's. So **Windows** is the only operating system this question leaves open.

---

## 7. Can something a file points at be traced to the scene that uses it?

**Reading of the question.** I take "something a file points at" to mean anything the blend references — a data-block, or an external file such as an image, a cache, or a linked library — and "the scene that uses it" to mean the Scene data-block (or data-blocks) whose contents depend on it. The other reading, "the 3D scene" in the loose sense of the set of objects, is not meaningfully different for the answer. I flag it because a Scene data-block is a precise thing and "scene" is not.

### Answer

**Yes, by walking users backwards, and the sources give the tools for each step, but nothing does the whole walk in one call.**

**7a. There is a documented, general, reverse lookup: who uses this?**

LABEL: **documented**

> user_map(*, subset=None, key_types=None, value_types=None)
>
> Returns a mapping of all ID data-blocks in current bpy.data to a set of all data-blocks using them.
>
> For list of valid set members for key_types & value_types, see: bpy.types.KeyingSetPath.id_type.
>
> Parameters:
>
> subset (Sequence[bpy.types.ID]) – When passed, only these data-blocks and their users will be included as keys/values in the map.
>
> key_types (set[str]) – Filter the keys mapped by ID types.
>
> value_types (set[str]) – Filter the values in the set by ID types.
>
> Returns:
>
> dictionary that maps data-blocks ID’s to their users.

Source: Python API documentation, https://docs.blender.org/api/5.0/bpy.types.BlendData.html — read 2026-07-30.

`LIBRARY` and `SCENE` are both valid members of the id-type list this refers to (https://docs.blender.org/api/5.0/bpy_types_enum_items/id_type_items.html, read 2026-07-30), so a library can be a key and a scene a value.

That this is one step, not the whole chain, is **inferred**: the quoted text says the map is from a data-block to "all data-blocks using them", so an image whose only user is a material yields the material, not the scene; reaching the scene means applying the map again to the material, and again. The step I supplied is that repetition, which the documentation does not describe.

**7b. There is a documented one-shot answer for objects.**

LABEL: **documented**

> users_scene
>
> The scenes this object is in.
>
> Type:
>
> tuple of Scene
>
> Note
>
> Takes O(len(bpy.data.scenes) * len(bpy.data.objects)) time.
>
> (readonly)

and, one rung down,

> users_collection
>
> The collections this object is in.

Source: Python API documentation, https://docs.blender.org/api/5.0/bpy.types.Object.html — read 2026-07-30.

**7c. There are documented reverse lookups for a few other kinds.**

LABEL: **documented**

For a library:

> users_id
>
> ID data-blocks that use this library
>
> Type:
>
> tuple of bpy.types.ID
>
> Note
>
> Takes O(n) time, where n is the total number of all
> linkable ID types in bpy.data.

Source: Python API documentation, https://docs.blender.org/api/5.0/bpy.types.Library.html — read 2026-07-30.

For a collection:

> users_dupli_group
>
> The collection instance objects this collection is used in

Source: https://docs.blender.org/api/5.0/bpy.types.Collection.html — read 2026-07-30.

For a texture:

> users_material
>
> Materials that use this texture

and

> users_object_modifier
>
> Object modifiers that use this texture

Source: https://docs.blender.org/api/5.0/bpy.types.ImageTexture.html — read 2026-07-30 (the same two appear on every texture type page).

**7d. Going the other way — asking a scene what it depends on — is also documented.**

LABEL: **documented**

> depsgraph
>
> Dependencies in the scene data
>
> Type:
>
> Depsgraph, (readonly)

Source: Python API documentation, https://docs.blender.org/api/5.0/bpy.types.ViewLayer.html — read 2026-07-30.

> ids
>
> All evaluated data-blocks
>
> Type:
>
> bpy_prop_collection of ID, (readonly)

and

> scene
>
> Original scene dependency graph is built for

Source: https://docs.blender.org/api/5.0/bpy.types.Depsgraph.html — read 2026-07-30.

That this gives an answer to question 7 (test each scene's dependency graph for the thing in hand) is **inferred**; the step I supplied is the test itself. Note the documented wording is "All *evaluated* data-blocks", and `ID.is_evaluated` is documented as "Whether this ID is runtime-only, evaluated data-block, or actual data from .blend file" (https://docs.blender.org/api/5.0/bpy.types.ID.html, read 2026-07-30) — so what comes out of `Depsgraph.ids` is not the same objects as what sits in `bpy.data`, and matching them up needs `ID.original`, documented as "Actual data-block from .blend file (Main database) that generated that evaluated one" (same page).

**7e. Paths, not just data-blocks, can be mapped to the data-blocks that use them.**

LABEL: **documented**

> file_path_map(*, subset=None, key_types=None, include_libraries=False)
>
> Returns a mapping of all ID data-blocks in current bpy.data to a set of all file paths used by them.
> …
> include_libraries (bool) – Include library file paths of linked data. False by default.
>
> Returns:
>
> dictionary of bpy.types.ID instances, with sets of file path strings as their values.

Source: Python API documentation, https://docs.blender.org/api/5.0/bpy.types.BlendData.html — read 2026-07-30.

This runs data-block → paths. Turning it round to get path → data-block, and then walking that data-block's users up to a scene, is **inferred** — the step I supplied is the inversion and the walk.

**7f. What the interface offers.**

LABEL: **documented**

The interface shows a *count*, not the identities:

> Every data-block has its usage counted (reference count), when there is more than one,
> you can see the number of current users of a data-block to the right of its name in the interface.

Source: manual, https://docs.blender.org/manual/en/5.0/files/data_blocks.html — read 2026-07-30. The same in the Outliner's Blender File mode:

> Lists all data in the current blend-file. On the right side of the list, a shield icon shows the number
> of users – clicking it adds or removes a fake user.

Source: manual, https://docs.blender.org/manual/en/5.0/editors/outliner/interface.html — read 2026-07-30. And the matching API property:

> users
>
> Number of times this data-block is referenced

Source: https://docs.blender.org/api/5.0/bpy.types.ID.html — read 2026-07-30.

**7g. Cycles' own files show the "which library did this come from" direction in use.**

LABEL: **observed**

Exact lines, from `Blender Sources/cycles_5.0/version_update.py`, lines 100–101 and 109–111:

```
    for library in bpy.data.libraries:
        library_versions.setdefault(library.version, []).append(library)
```
```
        for scene in bpy.data.scenes:
            if scene.library not in libraries:
                continue
```

Source: Cycles' own files, `cycles_5.0/version_update.py`, lines 100–101 and 109–111 — read 2026-07-30.

What this shows by reading alone: shipped code enumerates every scene in the loaded data and asks each one which library it came from. It does **not** show the reverse walk from an arbitrary referenced thing up to a scene; I did not run anything.

### Not covered

- The sources describe no single call that answers "which scenes use this?" for anything other than an Object. For everything else the answer is assembled by repeated use of `user_map()`, and the sources do not describe that assembly, nor say whether it terminates cleanly, nor what happens with cycles in the usage graph.
- Whether `user_map()` follows every kind of reference — for example a reference held only inside a node group, a driver expression, or a modifier's node tree. The wording is "all data-blocks using them" and nothing qualifies it, but nothing enumerates what counts as "using" either.
- What happens when the thing you start from is an external file rather than a data-block (say an EXR on disk). `file_path_map()` gets you from data-blocks to paths; the sources do not describe a path-first lookup.
- Cost. Two of the reverse lookups carry documented complexity notes (`Object.users_scene` is O(scenes × objects); `Library.users_id` is O(all linkable IDs)); `user_map()` carries none, so I cannot say what a repeated walk costs.
- Whether a scene that is not in any window still counts as "the scene that uses it" — the sources draw no such distinction, and I did not assume one.

### Outside the interface

Holds. LABEL: **inferred**. `user_map()`, `file_path_map()`, `users_scene`, `users_id` and `ViewLayer.depsgraph` are all properties and methods of `bpy.data` and of data-block types, none documented as needing a window, screen or context. The step I supplied: things reachable from `bpy.data` are reachable in background mode, which the API documentation supports generally — using Blender as a module "is equivalent to running a script in background-mode (passing the command-line arguments --background or -b)" (https://docs.blender.org/api/5.0/info_advanced_blender_as_bpy.html, read 2026-07-30).

The one part that is documented as interface-bound is the *display* of user counts (7f) — a shield icon and a number in the Outliner and in ID widgets. That has no equivalent outside the window; the API equivalent is `ID.users`, a number only.

One caveat that is documented and does bite outside the interface: the dependency-graph route depends on evaluation. The API documentation's own example notes that requesting an evaluated object "ensure[s] the dependency graph is fully evaluated. This might be expensive if changes were made to the scene" (https://docs.blender.org/api/5.0/bpy.types.Depsgraph.html, read 2026-07-30).

### Operating system

The sources do not say. I found no statement in the manual, the API documentation or the release notes that user tracing, `user_map()`, `file_path_map()` or the dependency graph behaves differently on any operating system, and nothing that would suggest it. LABEL: **not in the sources**.

### Where I looked (question 7)

Searched across `manual_5.0`, `api_5.0` and `release_notes_5.0` for: `user_map`, `users_`, `users of`, `used by`, `number of users`, `Show users`, `usages`, `Remap Users`, `trace`, `reverse`, `depsgraph`, `dependency`, `is_evaluated`, `original`, `file_path_map`, `path_map`, `id_type`.

Pages opened in full or in the relevant part: API `bpy.types.BlendData.txt` (whole method list), `bpy.types.ID.txt` (whole property and method list), `bpy.types.Object.txt` (users section), `bpy.types.Collection.txt`, `bpy.types.Library.txt`, `bpy.types.ImageTexture.txt`, `bpy.types.Depsgraph.txt`, `bpy.types.ViewLayer.txt`, `bpy.types.Scene.txt`, `bpy.data.txt`, `bpy_types_enum_items/id_type_items.txt`, `info_gotchas_internal_data_and_python_objects.txt`, `info_gotchas_crashes.txt`; manual `files/data_blocks.txt` (whole), `editors/outliner/interface.txt` (whole), `interface/controls/templates/data_block.txt`.

I also listed every operator in `bpy.ops.outliner.txt` looking for anything that reports or selects the users of a data-block. What is there is `id_remap` / "Remap Users – Make all users of selected data-blocks to use instead current (clicked) one" — a *change* operation, not a report. Ruled out on that basis.

Ruled out and why: `ID.users` (a count, so it cannot identify a scene); the Outliner "Data API" display mode, documented only as "Lists every data-block in the file along with any properties that it might have" — a flat listing, not a usage trace; `Library.needs_liboverride_resync` and the override troubleshooting operators (about overrides going stale, not about tracing usage).

Cycles' files: searched all `.py` under `cycles_5.0` for `user_map`, `users_scene`, `depsgraph`, `library`. Only `version_update.py` was relevant (quoted above); the `depsgraph` hits in `__init__.py` and `engine.py` are the render-engine entry points and say nothing about tracing usage.

### What the hand tests settled for question 7

**Observed (2026-08-04).** A test was run on the Creator's own computer — a MacBook Pro with an M1 Pro chip running macOS 26.5.2 — with Blender **5.0.1** (hash a3db93c5b259), started with no window. The full record, with the exact script and the raw output, is in `Preparation/Blender Research/Findings/5.0/Reading a file.md` under question 7. In short: on a file with two scenes, an image used through a material on a cube that sits in one scene only, and a second image used by nothing, the repeated `user_map()` walk reached the right scene in four steps — image to material, material to mesh, mesh to object, object to scene — and reached nothing at all from the unused image. Asking the map directly for scene-typed users of the image, with `value_types={'SCENE'}`, returned an empty set. The whole test ran with no window, so the walk holds outside Blender's interface. It was run on macOS.

**Observed (2026-08-06).** The same test was run again on a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH — with the same Blender build and no window. **Every printed line agreed with the Mac's**: the same four steps to the scene, nothing at all reached from the unused image, and an empty set from asking the map directly for scene-typed users. One thing had to be repaired before it would run at all: as the Mac's script stands, the unused image was not in the saved file on that machine and the script raised `KeyError: 'bpy_prop_collection[key]: key "UnusedTex" not found'`. Marking that image with a fake user so it survives the save fixed it. Both the error and the repaired run are recorded in `Preparation/Blender Research/Findings/5.0/Reading a file.md` under question 7; why the two machines' runs differed on that point was not established, and nothing says it belongs to the operating system. So **Windows** is the only operating system this question leaves open.

---

## 8. Can a file's full chain of references be read while some of the referenced files are absent?

**Reading of the question.** I take "chain of references" to mean references that go more than one step — a file links a library, which itself links another library, and so on — and "read" to mean enumerate with their recorded locations. I answer for library chains primarily, since that is where a chain exists at all, and note the plain external-file case where it differs.

### Answer

**Partly — and the part that is documented is the near end of the chain, not the far end.**

What is documented: when a linked file is missing, the references to it are *not* dropped; Blender substitutes placeholders and keeps the recorded location, and those placeholders are flagged as such. What is **not in the sources**: whether anything beyond a missing file — the references recorded inside *it* — can be read while it is absent.

**8a. Missing linked files do not erase the reference.**

LABEL: **documented**

> Broken Libraries
>
> If Blender cannot find a library while loading a blend-file,
> it will create placeholder data-blocks to replace missing linked ones.
> That way, references to the missing data are not lost, and by relocating the missing library,
> the lost data can be automatically restored.

Source: manual, https://docs.blender.org/manual/en/5.0/files/linked_libraries/link_append.html — read 2026-07-30.

**8b. The placeholders are flagged, so they can be told apart when read.**

LABEL: **documented**

> is_missing
>
> This data-block is a place-holder for missing linked data (i.e. it is [an override of] a linked data that could not be found anymore)
>
> Type:
>
> boolean, default False, (readonly)

Source: Python API documentation, https://docs.blender.org/api/5.0/bpy.types.ID.html — read 2026-07-30. Every data-block type inherits it. The linking machinery names the same idea:

> USE_PLACEHOLDERS
> Generate a placeholder (empty ID) if not found in any library files.

Source: https://docs.blender.org/api/5.0/bpy.types.BlendImportContext.html — read 2026-07-30.

**8c. The libraries themselves are readable data-blocks, with their recorded location and a parent.**

LABEL: **documented**

> External .blend file from which data is linked

> filepath
>
> Path to the library .blend file
>
> Type:
>
> string, default “”, (never None, blend relative // prefix supported)

> parent
>
> Type:
>
> Library, (readonly)

> version
>
> Version of Blender the library .blend was saved with

Source: Python API documentation, https://docs.blender.org/api/5.0/bpy.types.Library.html — read 2026-07-30. Note that `parent` carries **no description at all** on that page — I have quoted it in full, and I am not going to say what it points at, because the sources do not.

They are enumerable:

> libraries
>
> Library data-blocks
>
> Type:
>
> BlendDataLibraries bpy_prop_collection of Library, (readonly)

Source: https://docs.blender.org/api/5.0/bpy.types.BlendData.html — read 2026-07-30.

And the manual places them in the interface:

> Library
>
> ✗
>
> ✓
>
> References to an external blend-file.
> Access from the Outliner’s Blender File view.

Source: manual, https://docs.blender.org/manual/en/5.0/files/data_blocks.html — read 2026-07-30.

**8d. Two documented calls that hand back all referenced paths at once.**

LABEL: **documented**

> bpy.utils.blend_paths(*, absolute=False, packed=False, local=False)
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

Source: Python API documentation, https://docs.blender.org/api/5.0/bpy.utils.html — read 2026-07-30.

That linked-library paths are included in the default result is **inferred**: the only way `local=True` can "skip linked library paths" is if they are otherwise present. The step is that reading of the parameter.

And, per data-block:

> file_path_map(*, subset=None, key_types=None, include_libraries=False)
>
> Returns a mapping of all ID data-blocks in current bpy.data to a set of all file paths used by them.
> …
> include_libraries (bool) – Include library file paths of linked data. False by default.

Source: https://docs.blender.org/api/5.0/bpy.types.BlendData.html — read 2026-07-30.

Neither description says whether the paths returned are only those of files that exist, or all recorded paths including missing ones. That is one of the gaps below.

**8e. The interface can be asked to report what is missing.**

LABEL: **documented**

> Report Missing Files
>
> This option is useful to check if there are links to unpacked files that no longer exist.
> After selecting this option, a warning message will appear in the Info editor’s header.
> If no warning is shown, there are no missing external files.

Source: manual, https://docs.blender.org/manual/en/5.0/interface/window_system/topbar.html — read 2026-07-30. The matching operators:

> bpy.ops.file.report_missing_files()
>
> Report all missing external files

> bpy.ops.file.find_missing_files(*, find_all=False, directory='', …)
>
> Try to find missing external files
>
> Parameters:
>
> find_all (boolean, (optional)) – Find All, Find all files in the search path (not just missing)

Source: Python API documentation, https://docs.blender.org/api/5.0/bpy.ops.file.html — read 2026-07-30.

**8f. In 5.0 there is a documented way to make the chain survive the files going away.**

LABEL: **documented**

> Packed linked data are a key part of an improved asset workflow in Blender.
>
> Packed IDs remain considered as linked data (i.e. they cannot be edited), but they are stored in the current blendfile. This means that they:
>
> * Are not lost in case the library data becomes unavailable.
> * Are not changed in case the library data is updated.

and

> Name collisions are averted by storing these packed IDs into a new type of 'archive' libraries (and their namespaces). These libraries:
>
> * Only contain packed IDs.
> * Are owned and managed by their 'real' library data-block, called an 'archive parent'.

Source: release notes, https://developer.blender.org/docs/release_notes/5.0/core/ — read 2026-07-30.

The matching readable properties exist:

> archive_libraries
>
> Archive libraries of packed IDs, generated (and owned) by this source library

> archive_parent_library
>
> Source library from which this archive of packed IDs was generated

> is_archive
>
> This library is an ‘archive’ storage for packed linked IDs originally linked from its ‘archive parent’ library.

> packed_file
>
> Type:
>
> PackedFile, (readonly)

Source: Python API documentation, https://docs.blender.org/api/5.0/bpy.types.Library.html — read 2026-07-30. And the operation that sets it up:

> Pack Linked Libraries
> …
> Mark all linked library files in the current blend-file as packed.
> Actual packing will happen on the next save of the blend-file.

Source: manual, https://docs.blender.org/manual/en/5.0/files/blend/packed_data.html — read 2026-07-30.

There is also a documented flag on every data-block for this case:

> is_linked_packed
>
> This data-block is linked and packed into the .blend file

Source: https://docs.blender.org/api/5.0/bpy.types.ID.html — read 2026-07-30.

**8g. The far end of the chain: not in the sources.**

LABEL: **not in the sources**

I could not find any statement, in the manual, the API documentation or the release notes, about whether the references recorded *inside* a missing library can be read while that library is absent — that is, whether a blend file records its indirect dependencies' own dependencies, or only what it links directly plus whatever was resolved at load time. `Library.parent` exists and is readable, but carries no description; nothing says whether a `Library` entry is created for a library that was never reachable because its own parent was missing.

Nor did I find any statement that library loading is or is not recursive across a missing link, or that the Outliner's Blender File view shows libraries that could not be reached.

The nearest thing in the sources is negative rather than positive: the manual warns that references pointing back into the current file "will likely result in missing links" (https://docs.blender.org/manual/en/5.0/files/linked_libraries/link_append.html, read 2026-07-30), which is about circularity, not about absence.

The searches behind this "not in the sources" are listed below.

### Not covered

- Everything in 8g: the depth of the readable chain past a missing file.
- Whether `bpy.utils.blend_paths()` and `bpy.data.file_path_map()` return paths for references that are missing, or silently omit them. Nothing in either description says.
- Whether the same holds for missing *non*-library references (a missing image, a missing cache). 8a is written about libraries specifically; `ID.is_missing` is written about "linked data", not about images with a dead path. The sources do document a separate report/find pair for "external files" generally (8e), which suggests the two cases are handled differently, but I am not going to state how.
- What is *reported* when a file with missing links is opened — whether anything appears on the console or in a log, and in what form. The release notes describe a reworked logging system with `--log-level info`, `--log-list-categories` and a `blend` category visible in their example output (https://developer.blender.org/docs/release_notes/5.0/core/, read 2026-07-30), but nothing states that missing libraries are logged, or under what category.
- Whether reading the chain out of a blend file without loading it is possible. `bpy.types.BlendDataLibraries.load()` is documented as exposing "attributes matching bpy.data which are lists of strings to be linked" (https://docs.blender.org/api/5.0/bpy.types.BlendDataLibraries.html, read 2026-07-30), and `bpy.data` has a `libraries` member — but the `load()` text does not say that `libraries` is among the attributes exposed, and I will not assume it.
- The order of events: whether the chain is fully readable immediately after load, or only after the load handlers have run.

### Outside the interface

Holds for the reading, with one documented exception. LABEL: **inferred** for the reading part: `bpy.data.libraries`, `Library.filepath`, `Library.parent`, `ID.is_missing`, `bpy.utils.blend_paths()` and `bpy.data.file_path_map()` are all plain data access, and the API documentation states that module use "is equivalent to running a script in background-mode" (https://docs.blender.org/api/5.0/info_advanced_blender_as_bpy.html, read 2026-07-30). The step supplied: data access that does not name a window works where there is no window.

The documented exception is the *fixing*, not the reading. Relocate and Reload are described only as Outliner context-menu actions:

> When the Outliner is in the Blender File Display Mode,
> a right-click on a library (a linked blend-file) will show the Relocate option.

Source: manual, https://docs.blender.org/manual/en/5.0/files/linked_libraries/link_append.html — read 2026-07-30. There are matching operators, `bpy.ops.wm.lib_relocate(*, library='', filepath='', …)` — "Relocate the given library to one or several others" — and `bpy.ops.wm.lib_reload(...)` — "Reload the given library" (https://docs.blender.org/api/5.0/bpy.ops.wm.html, read 2026-07-30), but the sources do not say whether they run without a window. Likewise `Find Missing Files` is documented as opening a File Browser (8e), which has no meaning without one.

One further documented point about driving Blender from outside: only one file at a time.

> Single Blend File Restriction
>
> Only a single .blend file can be edited at a time.
>
> Hint
>
> As with the application it’s possible to start multiple instances,
> each with their own bpy and therefor Blender state.

Source: https://docs.blender.org/api/5.0/info_advanced_blender_as_bpy.html — read 2026-07-30. The same page points at `BlendDataLibraries.load()`, `BlendDataLibraries.write()` and `BlendData.temp_data()` as a partial alternative, adding that this "operates on reading and writing ID data-blocks and isn’t a complete substitute for loading .blend files".

### Operating system

The sources do not say that any of this differs by operating system. Two documented points touch the values rather than the mechanism: `bpy.path.resolve_ncase(path)` — "Resolve a case insensitive path on a case sensitive system…" — and `bpy.path.native_pathsep(path)` — "Replace the path separator with the systems native os.sep." (both https://docs.blender.org/api/5.0/bpy.path.html, read 2026-07-30). A recorded library path that resolves on one system's file naming rules and not on another's would be a missing library on the second and not the first, but the sources do not say that, and I am not asserting it. LABEL: **not in the sources**.

### Where I looked (question 8)

Searched across `manual_5.0`, `api_5.0` and `release_notes_5.0` for: `missing`, `missing file`, `missing librar`, `missing linked`, `missing data`, `Report Missing`, `Find Missing`, `is_missing`, `placeholder`, `USE_PLACEHOLDERS`, `broken`, `absent`, `unavailable`, `not found`, `cannot be found`, `could not be found`, `cannot find`, `indirect`, `indirectly linked`, `indirect librar`, `nested librar`, `library of a library`, `library hierarchy`, `dependency chain`, `recursively`, `chain`, `parent`, `relocate`, `reload`, `pack linked`, `packed linked`, `archive librar`, `is_archive`, `blend_paths`, `file_path_map`, `libraries.load`, `temp_data`, and `librar` across all release-note subjects.

Pages opened in full or in the relevant part: manual `files/linked_libraries/index.txt` and `link_append.txt` (whole), `files/linked_libraries/library_overrides.txt` (skimmed for missing-file handling), `files/data_blocks.txt` (whole), `files/blend/packed_data.txt`, `files/blend/open_save.txt`, `interface/window_system/topbar.txt` (External Data menu), `editors/outliner/interface.txt` (whole), `troubleshooting/` (all files, searched for `librar`); API `bpy.types.Library.txt` (whole), `bpy.types.BlendDataLibraries.txt` (whole), `bpy.types.BlendData.txt` (whole method list), `bpy.types.ID.txt` (whole), `bpy.types.BlendImportContext.txt`, `bpy.types.BlendImportContextItem.txt`, `bpy.types.BlendImportContextLibrary.txt`, `bpy.app.handlers.txt` (blend_import handlers), `bpy.ops.file.txt` (missing-file operators), `bpy.ops.wm.txt` (`lib_relocate`, `lib_reload`), `bpy.ops.outliner.txt` (full operator list), `bpy.utils.txt`, `bpy.path.txt`, `info_advanced_blender_as_bpy.txt` (whole), `info_gotchas_file_paths_and_encoding.txt` (whole); release notes `core.md` (whole), `assets.md`, `python_api.md`, `pipeline_io.md`, `corrective_releases.md` (searched), `bugfixes.md` (searched for `librar`, `missing`).

Cycles' files: searched all `.py` under `cycles_5.0` for `library`, `libraries`, `missing`. The only hits are `version_update.py` (enumerating `bpy.data.libraries` and each ID's `.library`, quoted under question 7) and `osl.py` (passing `library=` to `bpy.path.abspath`). Neither says anything about missing files, so neither supports an answer here.

Ruled out and why: the Outliner "Library Overrides" display mode and the override-troubleshooting operators (about overrides needing resync, a different failure than a missing file); `Library.needs_liboverride_resync`, same reason; `bpy.ops.file.unpack_libraries` — "Restore all packed linked data-blocks to their original locations" — a write, not a read; the `bugfixes.md` entries mentioning "Missing Linked Data Block" and "Missing Data-Block" node groups, which are crash-fix titles and describe no behaviour I could quote as an answer.

---

### What the hand tests settled for question 8

**Observed (2026-08-04).** Tests were run on the Creator's own computer — a MacBook Pro with an M1 Pro chip running macOS 26.5.2 — with Blender **5.0.1** (hash a3db93c5b259), started with no window. The full record, with the exact scripts and the raw output, is in `Preparation/Blender Research/Findings/5.0/Reading a file.md` under question 8. In short:

- A three-file chain was built: `c.blend` holds a cube, `b.blend` links it into a local collection, `a.blend` links that collection. With all three present, opening `a.blend` listed both `b.blend` and `c.blend`, and `Library.parent` came back `None` for both while Blender's own log said `parent '<direct>'` for both.
- With `c.blend` — the far end — renamed away, both libraries were still listed, `c.blend` carried `is_missing` `True`, the linked object carried `is_missing` `True`, and `blend_paths()` still held both paths.
- With `b.blend` — the middle — renamed away instead, only `b.blend` was listed. `c.blend` was named nowhere at all, the objects list was empty, and `blend_paths()` held only `//b.blend`. This settles 8g above: the readable chain stops at the first absent file.
- Blender printed all of this to the terminal with no window, naming the missing file and the missing data-block, so a missing library does reach a background run. This settles the fourth item under "Not covered" above.
- `bpy.data.libraries.load()` on a path that does not exist raised `OSError load: … failed to open blend file`. On a present file it exposed a `version` attribute but no `libraries` attribute in this version, and it listed only that file's own local data-blocks — a linked object inside it did not appear. This settles the last item under "Not covered" above.

The tests were run on macOS.

**Observed (2026-08-06).** All of it was run again on a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH — with the same Blender build and no window. **Every result agreed with the Mac's**, line for line: both libraries listed with everything present and `Library.parent` coming back `None` while Blender's log said `parent '<direct>'`; both still listed with the far file gone, the far one and the linked object marked missing; only the middle file listed with the middle one gone, the far file named nowhere and the object list empty; all of it printed to the terminal with no window; and the loader raising `OSError load: … failed to open blend file` on a path that does not exist while exposing a `version` attribute reading `(5, 0, 119)` and no `libraries` attribute. The full record is in `Preparation/Blender Research/Findings/5.0/Reading a file.md` under question 8, kept separate from the Mac's. So **Windows** is the only operating system this question leaves open.

---

## Things a person with Blender 5.0 open could settle by hand in a few minutes

The hand-test step ran on 2026-08-04 and settled every item that stood here. What it found is written into `Preparation/Blender Research/Findings/5.0/Reading a file.md` as observed findings, and summarised in the three sections above. New equipment arrived on 2026-08-06 and the same tests were run again on a rented Linux machine; that is written into the same document, beside the Mac's results and never merged with them.

Two things are left, and neither is a test that was skipped:

1. **Anything on Windows.** No Windows machine has run it, so every operating-system line above is settled for macOS and Linux and open for Windows. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**
2. **What the File Output node's Output Paths panel holds, on Linux.** Still open. It was read on the Mac. On the Linux run of 2026-08-06 the panel was closed and nothing on that machine could then click it open. A tool for moving the pointer was put there afterwards, and the citation check of that run tried again and could not confirm that a click reaches Blender on that machine.

---

2026-08-04 — Opus 5 (Claude Code) — Reading a file, Blender 5.0

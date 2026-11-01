# Reading a file — Blender 4.5 — second pass (questions 4 and 5)

**Sources consulted, and when.** Blender 4.5 manual (`Blender Sources/manual_4.5`, including two pictures in `_images`), Blender 4.5 Python API documentation (`Blender Sources/api_4.5`), Blender 4.5 release notes (`Blender Sources/release_notes_4.5`), and Cycles' own add-on files (`Blender Sources/cycles_4.5`). All four are the snapshot taken on **2026-07-30**, which is the date read for every finding below.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

One qualification I owe you, so that line is exactly true. Before writing this file I ran a directory listing of the folder I was told to save into, to check it existed. I saw a list of file names and nothing else — no file was opened, and no content from any of them entered this work. That is the only touch of anything outside `Blender Sources/` and the question sheet.

Two helper agents were used, both carrying the full warning verbatim and both restricted to the same two readable things. Each declared it opened nothing outside `Blender Sources/` and started no helpers of its own. I used them only to sweep for material I then opened myself: **every quotation in this document I read in the source file with my own eyes, at the file and line given, before writing it down.** Nothing here rests on a helper's word alone. I vouch for both on that basis.

---

## 4. Can the places a file is set up to write to be read out of it?

**LABEL: documented** (for the main answer), with two parts marked **inferred** and one part **not in the sources**, each flagged where it appears.

### The reading I answered, and the other reading

The question is ambiguous and I am flagging it rather than choosing silently.

- **The reading I answered:** given a `.blend`, can the output destinations that file has been set up with be got back out of it as values you can look at?
- **The other reading:** can they be got out *without a Blender* — by reading the file's bytes directly. I answer that one too, separately, at the end. It is **not in the sources**.

### The main answer

Yes. Every output destination I could find is an ordinary readable property of the loaded file's data, and the API documentation states each one.

**The scene's render output path.**

> filepath
>
> Directory/name to save animations, # characters define the position and padding of frame numbers
>
> Type:
>
> string, default “”, (never None, blend relative // prefix supported, Supports template expressions)

Source: Python API documentation, https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html (lines 212–218 of `bpy.types.RenderSettings.txt`). Date read: 2026-07-30.

The manual describes the same field from the interface side:

> File Path
>
> Choose the location to save rendered frames.
>
> When rendering an animation, the frame number is appended at the end of the file name with four padded zeros (e.g. image0001.png). You can set a custom padding size by adding the appropriate number of # anywhere in the file name (e.g. image_##_test.png translates to image_01_test.png).
>
> This setting expands Relative Paths where a // prefix represents the directory of the current blend-file.

Source: manual, https://docs.blender.org/manual/en/4.5/render/output/properties/output.html. Date read: 2026-07-30.

**LABEL: documented** (from a picture) — I opened `_images/render_output_properties_output_panel.png`, the picture the Output page names. It shows the Output panel with a single path field reading `/tmp/`, above checkboxes labelled Overwrite, Placeholders, File Extensions, Cache Result, and a File Format row set to PNG. That confirms the output destination is one plain text field in that panel. Source: manual, https://docs.blender.org/manual/en/4.5/render/output/properties/output.html. Date read: 2026-07-30.

**The resolved path for one frame.** Reading the stored string is not the same as knowing the file name that will actually be written, because of `#` padding, extensions and views. There is a documented call that does that arithmetic for you:

> frame_path(\*, frame=-2147483648, preview=False, view='')
>
> Return the absolute path to the filename to be written for a given frame
>
> Parameters:
>
> frame (int in [-inf, inf], (optional)) – Frame number to use, if unset the current frame will be used
>
> preview (boolean, (optional)) – Preview, Use preview range
>
> view (string, (optional, never None)) – View, The name of the view to use to replace the “%” chars
>
> Returns:
>
> File Path, The resulting filepath from the scenes render settings

Source: Python API documentation, https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html (lines 956–970 of `bpy.types.RenderSettings.txt`). Date read: 2026-07-30.

**LABEL: observed** — Cycles' own add-on files use exactly this pair to work out where frames were written. The file is `Blender Sources/cycles_4.5/operators.py`, in `CYCLES_OT_denoise_animation.execute`, lines 77–99:

```
            in_filepath = scene.render.filepath
            if out_filepath == '':
                out_filepath = in_filepath

            # Backup since we will overwrite the scene path temporarily
            original_filepath = scene.render.filepath

            for frame in range(scene.frame_start, scene.frame_end + 1):
                scene.render.filepath = in_filepath
                filepath = scene.render.frame_path(frame=frame)
                in_filepaths.append(filepath)

                if not os.path.isfile(filepath):
                    scene.render.filepath = original_filepath
                    err_msg = tip_("Frame '%s' not found, animation must be complete") % filepath
                    self.report({'ERROR'}, err_msg)
                    return {'CANCELLED'}
```

Two things this file *says*, which is solid ground: the property `scene.render.filepath` is read from and assigned to, and `scene.render.frame_path(frame=frame)` is called per frame and its result handed to `os.path.isfile`. What a run would *do* is a step beyond that; I have not taken it.

**The output paths on the compositor's File Output node.** There is a base path on the node and a sub-path per slot.

> base_path
>
> Base output path for the image
>
> Type:
>
> string, default “”, (never None, blend relative // prefix supported, Supports template expressions)

Source: Python API documentation, https://docs.blender.org/api/4.5/bpy.types.CompositorNodeOutputFile.html (lines 17–23). Date read: 2026-07-30.

> path
>
> Subpath used for this slot
>
> Type:
>
> string, default “”, (never None, Supports template expressions)

Source: Python API documentation, https://docs.blender.org/api/4.5/bpy.types.NodeOutputFileSlotFile.html (lines 17–21). Date read: 2026-07-30.

The manual states how the two combine:

> Base Path
>
> Unlike the render output filepath, this node uses a base directory and an image name, by default the output path is composed of: {base path}/{file name}{frame number}.{extension}.
>
> Besides being split into two settings, in all other respects, this setting is treated the same as the render output path.

Source: manual, https://docs.blender.org/manual/en/4.5/compositing/types/output/file_output.html. Date read: 2026-07-30.

**A stored output path is not always the literal destination.** In 4.5 these two fields can hold expressions that are only worked out when the path is used, so what you read back may not be the final text.

> Path templates substitute template expressions (written {abcd}, where “abcd” is a variable name) in a filepath when the path is used.
>
> […]
>
> This substitution happens internally at time of use (e.g. while rendering), so the template syntax will remain as-is in the path field.
>
> Note
>
> Currently path templates are only supported for the render output path in Scene properties and the output paths in the compositor’s File Output node. More filepaths will support templates in future releases.

Source: manual, https://docs.blender.org/manual/en/4.5/files/file_paths.html. Date read: 2026-07-30.

The release notes say the same as a statement of change:

> ## Path Templates
>
> Some file paths in Blender can now contain "template expressions" like
> `{blend_name}` and `{resolution_x}`, which get replaced by the values they
> reference when the path is used.
>
> Currently this is only enabled on the Scene's render output path and the output
> paths of the compositor's File Output node.

Source: release notes, https://developer.blender.org/docs/release_notes/4.5/core/. Date read: 2026-07-30.

The three variables the manual lists are `blend_name`, `fps`, and `resolution_x` / `resolution_y`, and it adds "The set of available variables will expand over time." (same manual page).

**LABEL: inferred** — `frame_path()` resolves template expressions, so it, and not the raw string, is what tells you the real destination. The step: the release notes say

> - `scene.render.frame_path()` used to be infallible but can now fail if there
>   are errors processing [Path
>   Templates](https://docs.blender.org/manual/en/4.5/files/file_paths.html#path-templates)
>   in the render output path.

(Source: release notes, https://developer.blender.org/docs/release_notes/4.5/python_api/. Date read: 2026-07-30.) For `frame_path()` to fail on errors *processing* templates, `frame_path()` must be the thing processing them. The reader has to supply that one step, so this is inferred, not documented. The sources nowhere say outright "frame_path expands templates".

**The other places a file records that it will write to.** Each is a documented, readable property. All quotations are from the Python API documentation, read 2026-07-30.

| What it is | Property | Exact words | Address |
|---|---|---|---|
| Physics point cache on disk | `PointCache.filepath` | "Cache file path" — "string, default “”, (never None, blend relative // prefix supported)" | https://docs.blender.org/api/4.5/bpy.types.PointCache.html |
| The same, per cache item | `PointCacheItem.filepath` | "Cache file path" — "string, default “”, (never None, blend relative // prefix supported)" | https://docs.blender.org/api/4.5/bpy.types.PointCacheItem.html |
| Fluid (Mantaflow) cache | `FluidDomainSettings.cache_directory` | "Directory that contains fluid cache files" — "string, default “”, (never None, blend relative // prefix supported)" | https://docs.blender.org/api/4.5/bpy.types.FluidDomainSettings.html |
| Geometry-nodes bake, modifier-wide | `NodesModifier.bake_directory` | "Location on disk where the bake data is stored" — "string, default “”, (never None, blend relative // prefix supported)" | https://docs.blender.org/api/4.5/bpy.types.NodesModifier.html |
| Geometry-nodes bake, per bake | `NodesModifierBake.directory` | "Location on disk where the bake data is stored" — "string, default “”, (never None, blend relative // prefix supported)" | https://docs.blender.org/api/4.5/bpy.types.NodesModifierBake.html |
| Baking to external images | `BakeSettings.filepath` | "Image filepath to use when saving externally" — "string, default “”, (never None, blend relative // prefix supported)" | https://docs.blender.org/api/4.5/bpy.types.BakeSettings.html |
| Multiresolution external displacements | `MultiresModifier.filepath` | "Path to external displacements file" — "string, default “”, (never None, blend relative // prefix supported)" | https://docs.blender.org/api/4.5/bpy.types.MultiresModifier.html |
| Ocean modifier bakes | `OceanModifier.filepath` | "Path to a folder to store external baked images" — "string, default “”, (never None, blend relative // prefix supported)" | https://docs.blender.org/api/4.5/bpy.types.OceanModifier.html |
| Dynamic Paint image output | `DynamicPaintSurface.image_output_path` | "Directory to save the textures" — "string, default “”, (never None, blend relative // prefix supported)" | https://docs.blender.org/api/4.5/bpy.types.DynamicPaintSurface.html |
| Sequencer custom proxy | `StripProxy.filepath` | "Location of custom proxy file" — "string, default “”, (never None, blend relative // prefix supported)" | https://docs.blender.org/api/4.5/bpy.types.StripProxy.html |
| Sequencer proxy folder | `StripProxy.directory` | "Location to store the proxy files" — "string, default “”, (never None, blend relative // prefix supported)" | https://docs.blender.org/api/4.5/bpy.types.StripProxy.html |
| Sequencer-wide proxy folder | `SequenceEditor.proxy_dir` | (no description given) — "string, default “”, (never None, blend relative // prefix supported)" | https://docs.blender.org/api/4.5/bpy.types.SequenceEditor.html |
| Movie clip proxy folder | `MovieClipProxy.directory` | "Location to store the proxy files" — "string, default “”, (never None, blend relative // prefix supported)" | https://docs.blender.org/api/4.5/bpy.types.MovieClipProxy.html |
| Per-collection exporters | `CollectionExport.filepath` | "The file path used for exporting" — "string, default “”, (never None, blend relative // prefix supported)" | https://docs.blender.org/api/4.5/bpy.types.CollectionExport.html |

Whether a recorded cache path is actually used is a separate stored setting, and it is readable too:

> use_disk_cache
>
> Save cache files to disk (.blend file must be saved first)

> use_external
>
> Read cache from an external location

> use_library_path
>
> Use this file’s path for the disk cache when library linked into another file (for local bakes per scene file, disable this option)

Source: Python API documentation, https://docs.blender.org/api/4.5/bpy.types.PointCache.html (lines 118–140). Date read: 2026-07-30.

The manual describes the same three from the interface, and — this matters — gives the naming rule Blender uses when you have not set a path at all:

> External
>
> Allows you to read the cache from a drive using a user-specified file path.
>
> […]
>
> Index
>
> The index number of cache files. (The last two digits of the files name.)
>
> Path
>
> Select the directory path to the cache files.
>
> Disk Cache
>
> The cache of a baked simulation will be stored inside the blend-file when you save it. When Disk Cache is checked, Blender will save the cache separately to the drive in a folder named blendcache_[filename] alongside the blend-file. (The blend-file must be saved first.)

Source: manual, https://docs.blender.org/manual/en/4.5/physics/baking.html. Date read: 2026-07-30. That last line is the one to hold on to: a file can be set up to write somewhere that is recorded nowhere in it as a path, but derived from the blend-file's own name.

The same page gives the rule for unnamed caches:

> If the cache does not have a name (which is the default), the filename of the cache will depend on the object it is attached to, although this is not immediately obvious. For example, a cache on an object Cube will be stored in 43756265_xxxxxx_yy.bphys, where 43756265 is determined by the object name.

Same source and date.

Two more of the destinations above have manual pages worth the exact words. For the ocean bake:

> Cache Path
>
> Folder to store the baked EXR files in. The sequences will be in the form disp_####.exr, normal_####.exr, and foam_####.exr, where #### is the four digit frame number. If the cache path folder does not exist, it will be created.

Source: manual, https://docs.blender.org/manual/en/4.5/modeling/modifiers/physics/ocean.html. Date read: 2026-07-30.

And for geometry-nodes bakes, where the destination is a choice before it is a path:

> Bake Target
>
> Specifies where baked data should be stored. This can be overridden for individual bakes.
>
> Packed:
>
> The baked data is packed into the .blend file. So no separate file is necessary.
>
> Disk:
>
> The baked data is stored in a separate directory on disk.
>
> Bake Path
>
> Location on disk where the baked data for Simulation Zones and Bake Nodes are stored.

Source: manual, https://docs.blender.org/manual/en/4.5/modeling/modifiers/generate/geometry_nodes.html. Date read: 2026-07-30.

One destination is not a separate setting at all — rendering from the viewport reuses the scene's:

> Additionally, some output settings are used too:
>
> Resolution
>
> Aspect
>
> Output path
>
> File format

Source: manual, https://docs.blender.org/manual/en/4.5/editors/3dview/viewport_render.html. Date read: 2026-07-30.

The collection exporters are reachable from the collection itself:

> exporters
>
> Export Handlers configured for the collection
>
> Type:
>
> bpy_prop_collection of CollectionExport, (readonly)

Source: Python API documentation, https://docs.blender.org/api/4.5/bpy.types.Collection.html (lines 57–63). Date read: 2026-07-30. (The collection of exporters is read-only as a collection; the `filepath` on each entry is not marked read-only.) The manual describes them:

> Each collection can be exported to a number of various file formats. These exporters are available globally, see Importing & Exporting Files, however, this panel streamlines the process of re-exporting the same asset(s) repeatedly.

Source: manual, https://docs.blender.org/manual/en/4.5/scene_layout/collections/collections.html. Date read: 2026-07-30.

**Is there one call that lists them all?** Not one that returns the paths. But 4.5's own property-introspection carries a flag that marks a property as an output path, and that flag is readable:

> is_path_output
>
> Property is a filename, filepath or directory output
>
> Type:
>
> boolean, default False, (readonly)

Source: Python API documentation, https://docs.blender.org/api/4.5/bpy.types.Property.html (lines 100–106). Date read: 2026-07-30.

Two neighbours of it are readable in the same way, and matter for making sense of what you read:

> is_path_supports_blend_relative
>
> Property is a path which supports the “//” prefix, signifying the location as relative to the “.blend” file’s directory

> is_path_supports_templates
>
> Property is a path which supports the “{variable_name}” variable expression syntax, which substitutes the value of the referenced variable in place of the expression

Same page and date. The matching flag names an author sets are documented too — `OUTPUT_PATH: Output Path.`, `PATH_SUPPORTS_BLEND_RELATIVE`, `SUPPORTS_TEMPLATES` — in https://docs.blender.org/api/4.5/bpy_types_enum_items.html (file `api_4.5/bpy_types_enum_items/property_flag_items.txt`, lines 41–53), read 2026-07-30.

**How many path properties are there at all?** The API documentation stamps every blend-relative path property with the same phrase, so the whole set can be counted. Searching all of `api_4.5` for the exact words `blend relative // prefix supported` returns **47 occurrences across 41 files** (39 under `bpy.types.`, plus one each in `bpy.ops.view3d.txt` and `bpy.ops.workspace.txt`). Five of the 47 are in `PreferencesFilePaths`, which is the application's settings and not the file's. Date read: 2026-07-30.

That count is a floor, not a ceiling: at least two path properties do not carry the phrase and so are not in it — `Text.filepath` ("string, default “”, (never None)") and `NodeOutputFileSlotFile.path` ("string, default “”, (never None, Supports template expressions)"). So it bounds the blend-relative ones, not the paths.

### The other reading: without a Blender

**LABEL: not in the sources.**

Nothing in the manual, the API documentation, the release notes or the Cycles files describes the layout of a `.blend` file, or any way to pull an output path out of one without Blender loading it.

Where I looked, and what I searched for. I searched the whole of `manual_4.5`, `api_4.5` and `release_notes_4.5` (text and markdown only, excluding the `_images` folder and the generated index pages) for: `SDNA`, `DNA`, `blend-file format`, `file format specification`, `without opening`, `without running Blender`, `third-party`, `parse`, `inspect a blend`, `query the file`, `read the render output`, `list of output`, `output destination`. `SDNA`, `blend-file format`, `file format specification`, `without running Blender`, `inspect a blend`, `query the file`, `read the render output`, `list of output` and `output destination` returned nothing at all. `without opening` returned three hits, all about menus and drivers (`interface/controls/buttons/menus.txt`, `animation/drivers/usage.txt`). `third-party` returned hits about add-ons, renderers, UDIM painting and audio editing — nothing about reading blend-files. The nearest thing in the sources to touching a `.blend` from outside is about compression, not content:

> The used compression algorithm is Zstandard. It is not unique to Blender so files can be compressed/decompressed with external tools.

Source: manual, https://docs.blender.org/manual/en/4.5/files/blend/open_save.html. Date read: 2026-07-30. That describes unwrapping the file, not reading anything out of it.

One passage comes close, and it is worth reading carefully because of what it does and does not say. The manual, discussing why Blender will not edit other files, says such outside tooling exists:

> Cross Blend-File Editing
>
> As described above, Blender itself is not allowed to write to other blend-files than the currently open one. This rule helps to limit complexities; for example, it is hard to reliably implement an undo system when manipulating other files. The rule does get in the way of mass-updating assets when they are stored in various blend-files.
>
> Since there is already tooling that can manipulate blend-files outside of Blender itself (see Blender Asset Tracer), it’s possible to also create an external tool for doing such edits across blend-files. Such a tool might even be implemented via Blender’s application templates system, or as an add-on; the rule above applies to Blender itself, not to its add-ons.

Source: manual, https://docs.blender.org/manual/en/4.5/files/asset_libraries/introduction.html. Date read: 2026-07-30.

So the manual states plainly that tooling which manipulates blend-files from outside Blender exists, and names one. But that is the whole of it: nothing in my sources says what that tool can read, whether it can read an output path, how, or with what reliability. The answer stays **not in the sources** for the mechanism; what changes is that the sources do not treat the idea as impossible.

I also ruled out `bpy.data.libraries.load()` as a way to peek at another file's output paths without opening it. The documentation says what it hands you:

> Returns a context manager which exposes 2 library objects on entering. Each object has attributes matching bpy.data which are lists of strings to be linked.

Source: Python API documentation, https://docs.blender.org/api/4.5/bpy.types.BlendDataLibraries.html. Date read: 2026-07-30. **LABEL: inferred** — since what it exposes is lists of *names*, it does not give you any property of the data in that file, so it cannot give you that file's render output path. The step: "lists of strings" plus "attributes matching bpy.data" means names only, and a path is a property, not a name.

### One thing worth knowing alongside the answer

What the file records is not necessarily where a render lands, because the command line can override it after the file is loaded:

> -o, --render-output <path>
>
> Set the render path and file name.
> Use // at the start of the path to render relative to the blend-file.
>
> You can use path templating features such as {blend_name} in the path.

and, on ordering:

> blender --background --render-output /tmp test.blend --render-frame 1
>
> …will not render to /tmp because loading the blend-file overwrites the render output that was set.

Source: manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html. Date read: 2026-07-30.

There is also an application-level default that is *not* in the file at all, so do not mistake it for one:

> Render Output
>
> Where rendered images/videos are saved.

Source: manual, https://docs.blender.org/manual/en/4.5/editors/preferences/file_paths.html, in the Preferences section. Date read: 2026-07-30.

### Where I looked, for the whole of question 4

Searches were run across all of `manual_4.5`, `api_4.5`, `release_notes_4.5` and `cycles_4.5` unless noted.

- **The census that found the destinations.** Searched every `bpy.types.*.txt` for a property line reading exactly `filepath`, which gave 27 types; then searched all of `api_4.5` for the exact phrase `blend relative // prefix supported`, which gave 47 occurrences across 41 files. Those two sweeps are where the table above comes from. Also searched for `cache_directory`, `bake_directory`, `simulation_bake_directory`, `image_output_path`, `proxy_dir`, `directory`, `render_output_directory`.
- **Templates.** Searched for `template expression`, which returned exactly nine lines across the three documentation sources: three type-lines in `api_4.5` (`RenderSettings`, `CompositorNodeOutputFile`, `NodeOutputFileSlotFile`), the `property_flag_items` entry, four lines in `manual_4.5/files/file_paths.txt`, and one in `release_notes_4.5/core.md`. That is what makes the "only these two settings" claim checkable. Also searched `path_template`, which returned nothing.
- **The output-path flag.** Searched for `OUTPUT_PATH`, `is_path_output`, `is_path_supports_templates`, `is_path_supports_blend_relative`. `OUTPUT_PATH` appears in exactly one file in all of `api_4.5` — the `property_flag_items` enum. Searched every `bpy.types.*.txt` for the phrase `output path`, which returned exactly two lines, quoted under Not covered below.
- **Manual pages opened.** `render/output/properties/output.txt`, `render/output/properties/index.txt` (to see what else sits under Output), `compositing/types/output/file_output.txt`, `files/file_paths.txt`, `physics/baking.txt`, `physics/fluid/type/domain/cache.txt`, `modeling/modifiers/physics/ocean.txt`, `modeling/modifiers/generate/geometry_nodes.txt`, `editors/3dview/viewport_render.txt`, `scene_layout/collections/collections.txt`, `interface/window_system/topbar.txt`, `editors/preferences/file_paths.txt`, `advanced/command_line/arguments.txt`, `files/blend/open_save.txt`.
- **Pictures opened.** `_images/render_output_properties_output_panel.png`.
- **Cycles' files.** Searched all of `cycles_4.5` for `filepath` and `directory`. The only path handling in them is in `operators.py` (quoted above) and `osl.py`. Cycles' `properties.py` holds no output path of its own — I checked.
- **Ruled out.** `bpy.data.libraries.load()`, for the reason given below. Preferences (`PreferencesFilePaths`), because the manual presents them as application defaults rather than something stored in the file.

### Not covered

- I cannot show my list of output destinations is complete. I found them by hunting property by property; the sources publish no list of "all the places a file can be set up to write to", so there may be others I did not think to look for.
- The documentation does **not** say which properties actually carry the `OUTPUT_PATH` flag. The API pages annotate `blend relative // prefix supported` and `Supports template expressions` in the type line, but never `output path`. I searched every `bpy.types.*.txt` file for the phrase "output path" and got only two hits, both plain prose (`CompositorNodeOutputFile.base_path`'s description, and a graphviz debug argument on `Depsgraph`). So whether `RenderSettings.filepath` is flagged as an output path is unverified from the sources.
- Whether `frame_path()` resolves template expressions is inferred, not stated.
- Nothing in the sources says what `frame_path()` returns when the output path contains a template error, beyond that it "can now fail".
- I did not establish whether the File Output node's slot paths have any equivalent of `frame_path()`. I found none.
- **The manual does not document the File Output node's per-slot paths at all.** Its page for the node describes only Base Path and File Format, and ends "More options can be set in the Sidebar region." (https://docs.blender.org/manual/en/4.5/compositing/types/output/file_output.html, read 2026-07-30). Searching the whole of `manual_4.5` for `Subpath`, `File Slot`, `input slot` and `Add Input` found nothing about them. Everything I know about those slot paths comes from the API documentation, not the manual.
- Not every destination is a recorded path. The physics disk cache falls back to a folder derived from the blend-file's name, and an unnamed cache to a number derived from the object's name, as quoted above. Reading the file's path properties would not find either. I did not check whether other features do the same, and I would expect some do.

### Outside the interface

Reading these values needs Blender to have loaded the file, but not Blender's window. The documented way is the command line:

> blender --background --python myscript.py
>
> You might want to run this with a blend-file so the script has some data to operate on.
>
> blender myscene.blend --background --python myscript.py

Source: Python API documentation, https://docs.blender.org/api/4.5/info_tips_and_tricks.html. Date read: 2026-07-30.

There is a second documented way, Blender built as an importable Python module:

> Blender supports being built as a Python module, allowing import bpy to be added to any Python script, providing access to Blender’s features.
>
> […]
>
> For the most part using Blender as a Python module is equivalent to running a script in background-mode (passing the command-line arguments --background or -b), however there are some differences to be aware of.

and, on how many files at once:

> Single Blend File Restriction
>
> Only a single .blend file can be edited at a time.
>
> Hint
>
> As with the application it’s possible to start multiple instances, each with their own bpy and therefor Blender state.

Source: Python API documentation, https://docs.blender.org/api/4.5/info_advanced_blender_as_bpy.html. Date read: 2026-07-30.

A script can tell which of the three situations it is in:

> bpy.app.background
>
> Boolean, True when blender is running without a user interface (started with -b)

> bpy.app.module
>
> Boolean, True when running Blender as a python module

Source: Python API documentation, https://docs.blender.org/api/4.5/bpy.app.html (lines 168–178). Date read: 2026-07-30.

Where it differs from inside the window: only one part of what the manual describes is interface-only, and it is the reporting of template errors, not the reading of the path —

> When there are template errors in a path, the path field will be highlighted red in the UI:
>
> […]
>
> Hovering over the path field will pop up a tooltip that contains a list of the template errors encountered in that path:

Source: manual, https://docs.blender.org/manual/en/4.5/files/file_paths.html. Date read: 2026-07-30. The sources do not say how, or whether, those same errors are surfaced with no window. They do say that when rendering an animation with errors in the output path it "will fail with an error message indicating the path errors" (same page).

### Operating system

The sources do not say the reading of these values differs by operating system, and I found no statement that it does. What they do say is that paths themselves are not identical across systems, which affects what you get back:

> bpy.path.native_pathsep(path)
>
> Replace the path separator with the systems native os.sep.

> bpy.path.basename(path)
>
> Equivalent to os.path.basename, but skips a “//” prefix.
>
> Use for Windows compatibility.

> bpy.path.resolve_ncase(path)
>
> Resolve a case insensitive path on a case sensitive system, returning a string with the path if found else return the original path.

Source: Python API documentation, https://docs.blender.org/api/4.5/bpy.path.html. Date read: 2026-07-30.

The one command-line difference the sources name is unrelated to output paths: `/?` to print help is marked "(Windows only)" (https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html, read 2026-07-30). The hand tests were run on **two** machines — the Creator's Mac and a rented Linux machine — and agreed, so macOS and Linux are both settled for what they cover. **Windows stays open**, because no Windows machine has run it. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

### What the hand tests settled for question 4

**Observed (2026-08-04).** Four tests were run on the Creator's own computer — a MacBook Pro with an M1 Pro chip running macOS 26.5.2 — with Blender **4.5.12 LTS** (hash 84afd5f785f7), started with no window, with `--factory-startup`, and with a throwaway settings folder. They settled four things left open above, and the full record of each, with the exact commands and the raw output, is in `Preparation/Blender Research/Findings/4.5/Reading a file.md` under question 4. In short:

- Reading a stored output path back gives the stored text with any `{…}` left in place, and `frame_path()` returns a path with the templates worked out. This settles the step marked inferred above.
- A malformed template makes `frame_path()` raise, rather than return anything.
- `{scene_name}` is not a variable this version knows: assigning it printed `Unknown variable referenced in template expression '{scene_name}'`.
- The output-path marker `is_path_output` came back `True` for `RenderSettings.filepath` and `False` for `Image.filepath`, `Library.filepath` and `PointCache.filepath`. This settles the "unverified from the sources" note above.
- Neither `bpy.utils.blend_paths()` nor `bpy.data.file_path_map()` returned the render output path or the File Output node's paths on a file that had both set.
- The File Output node's stored destinations read back as `base_path` plus one `path` per slot, and a render wrote one image per slot, with a slash in a slot's name making a folder on disk.

**Observed (2026-08-05).** The Creator then opened the node's sidebar in Blender's own window, on the same computer, and photographed it. In 4.5 that sidebar holds two panels, `Node` and `Properties`, and **no panel headed Output Paths**. The Properties panel holds `Base Path:` above a field reading `//out/`, `Save as Render`, `File Format` set to `PNG`, `Color`, `Color Depth`, `Compression` and `Color Management` settings, an `Add Input` button, a list reading `Image`, `passes/diffuse`, `beauty`, a `File Subpath:` field reading `beauty`, and `Format: Use Node Format`. So this version shows no assembled final path anywhere in that sidebar. The full record is in `Preparation/Blender Research/Findings/4.5/Reading a file.md` under question 4. It was run on a Mac, so it settles macOS.

**Observed (2026-08-06).** Every one of those tests was then run again on a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH — with the same Blender build, no window, `--factory-startup`, and this topic's own settings folder. The sidebar was reached there too, by driving Blender's window on that machine's own display and taking Blender's own screenshot. **Every result agreed with the Mac's**, point for point: the stored text with the braces left in place, the worked-out path from `frame_path()`, the raise on a malformed template, `Unknown variable referenced in template expression '{scene_name}'`, the four `is_path_output` answers, the two whole-file path lists returning neither output path, the node's `base_path` and slot paths with a slash making a folder on disk, and a sidebar holding no panel headed Output Paths. The full record is in `Preparation/Blender Research/Findings/4.5/Reading a file.md` under question 4, kept separate from the Mac's. So Windows is the only operating system this question leaves open.

---

## 5. Can the locations a file records for the things it points at be read out and written back, and by what?

**LABEL: documented.**

Yes to both. The manual states the arrangement plainly:

> External data, like texture images and other resources, can be stored inside the blend-file (packed) or as separate files (unpacked). Blender keeps track of all unpacked resources via a relative or absolute path.

Source: manual, https://docs.blender.org/manual/en/4.5/interface/window_system/topbar.html. Date read: 2026-07-30.

and, on what the recorded path means:

> Many blend-files reference external images or other linked blend-files. A path tells Blender where to look for these files. If the external files are moved, the blend-file that references them will not look right.
>
> When you specify one of these external files, the default option is to make the path relative. Blender stores a partial path evaluated relative to the directory location of the referencing blend-file. This choice helps when you need to reorganize folders or move your files.
>
> […]
>
> When relative paths are supported, the File Browser provides a Relative Path checkbox, when entering the path into a text field, use a double slash prefix (//) to make it so.

Source: manual, https://docs.blender.org/manual/en/4.5/files/blend/open_save.html. Date read: 2026-07-30.

### By what — reading

**All of them at once, as a list.** There is one call for this:

> bpy.utils.blend_paths(absolute=False, packed=False, local=False)
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

Source: Python API documentation, https://docs.blender.org/api/4.5/bpy.utils.html (lines 12–30). Date read: 2026-07-30.

**LABEL: inferred** — this one is read-only. The step: it returns `list[str]`, a plain list of strings, with no stated connection back to the data-blocks the strings came from; changing an entry in a returned list of strings cannot reach back into the file. The documentation does not say "read-only" in so many words.

**One at a time, through the thing that points.** Each kind of referencing data-block carries its own path property. I checked every type in the 4.5 API documentation with a top-level `filepath` property. The full set is: `BlendData`, `BlendImportContextLibrary`, `BakeSettings`, `CacheFileLayer`, `CollectionExport`, `CacheFile`, `ImagePackedFile`, `Image`, `Library`, `LibraryWeakReference`, `MeshCacheModifier`, `MovieClip`, `MovieStrip`, `MultiresModifier`, `OceanModifier`, `PointCache`, `PointCacheItem`, `RenderSettings`, `ShaderNodeTexIES`, `Sound`, `StripProxy`, `ShaderNodeScript`, `Text`, `Theme`, `TextureNodeOutput`, `Volume`, `VectorFont`.

The ones that record something the file points *at*, with their exact words (all from the Python API documentation, read 2026-07-30):

| Type | Exact words | Address |
|---|---|---|
| `Image.filepath` | "Image/Movie file name" — "string, default “”, (never None, blend relative // prefix supported)" | https://docs.blender.org/api/4.5/bpy.types.Image.html |
| `Image.filepath_raw` | "Image/Movie file name (without data refreshing)" — "string, default “”, (never None, blend relative // prefix supported)" | https://docs.blender.org/api/4.5/bpy.types.Image.html |
| `Library.filepath` | "Path to the library .blend file" — "string, default “”, (never None, blend relative // prefix supported)" | https://docs.blender.org/api/4.5/bpy.types.Library.html |
| `Sound.filepath` | "Sound sample file used by this Sound data-block" — "string, default “”, (never None, blend relative // prefix supported)" | https://docs.blender.org/api/4.5/bpy.types.Sound.html |
| `MovieClip.filepath` | "Filename of the movie or sequence file" — "string, default “”, (never None, blend relative // prefix supported)" | https://docs.blender.org/api/4.5/bpy.types.MovieClip.html |
| `CacheFile.filepath` | "Path to external displacements file" — "string, default “”, (never None, blend relative // prefix supported)" | https://docs.blender.org/api/4.5/bpy.types.CacheFile.html |
| `Volume.filepath` | "Volume file used by this Volume data-block" — "string, default “”, (never None, blend relative // prefix supported)" | https://docs.blender.org/api/4.5/bpy.types.Volume.html |
| `VectorFont.filepath` | (no description given) — "string, default “”, (never None, blend relative // prefix supported)" | https://docs.blender.org/api/4.5/bpy.types.VectorFont.html |
| `Text.filepath` | "Filename of the text file" — "string, default “”, (never None)" | https://docs.blender.org/api/4.5/bpy.types.Text.html |
| `MovieStrip.filepath` | (no description given) — "string, default “”, (never None, blend relative // prefix supported)" | https://docs.blender.org/api/4.5/bpy.types.MovieStrip.html |
| `ImagePackedFile.filepath` | (no description given) — "string, default “”, (never None, blend relative // prefix supported)" | https://docs.blender.org/api/4.5/bpy.types.ImagePackedFile.html |
| `ImageStrip.directory` | (no description given) — "string, default “”, (never None, blend relative // prefix supported)" | https://docs.blender.org/api/4.5/bpy.types.ImageStrip.html |
| `CacheFileLayer.filepath` | "Path to the archive" — "string, default “”, (never None, blend relative // prefix supported)" | https://docs.blender.org/api/4.5/bpy.types.CacheFileLayer.html |
| `MeshCacheModifier.filepath` | "Path to external displacements file" — "string, default “”, (never None, blend relative // prefix supported)" | https://docs.blender.org/api/4.5/bpy.types.MeshCacheModifier.html |
| `ShaderNodeScript.filepath` | "Shader script path" — "string, default “”, (never None, blend relative // prefix supported)" | https://docs.blender.org/api/4.5/bpy.types.ShaderNodeScript.html |
| `ShaderNodeTexIES.filepath` | "IES light path" — "string, default “”, (never None, blend relative // prefix supported)" | https://docs.blender.org/api/4.5/bpy.types.ShaderNodeTexIES.html |
| `Brush.icon_filepath` | "File path to brush icon" — "string, default “”, (never None, blend relative // prefix supported)" | https://docs.blender.org/api/4.5/bpy.types.Brush.html |
| `NodeSocketStringFilePath.default_value` | "Input value used for unconnected socket" — "string, default “”, (never None, blend relative // prefix supported)" | https://docs.blender.org/api/4.5/bpy.types.NodeSocketStringFilePath.html |

One is explicitly read-only, and is the exception that proves the rule about how the documentation marks them:

> frame_filepath
>
> Volume file used for loading the volume at the current frame. Empty if the volume has not be loaded or the frame only exists in memory.
>
> Type:
>
> string, default “”, (readonly, never None, blend relative // prefix supported)

Source: Python API documentation, https://docs.blender.org/api/4.5/bpy.types.VolumeGrids.html (lines 33–39). Date read: 2026-07-30.

`LibraryWeakReference` is described as read-only in its own class text, though its `filepath` line carries no `readonly` marker:

> class bpy.types.LibraryWeakReference(bpy_struct)
>
> Read-only external reference to a linked data-block and its library file
>
> filepath
>
> Path to the library .blend file

Source: Python API documentation, https://docs.blender.org/api/4.5/bpy.types.LibraryWeakReference.html (lines 5–15). Date read: 2026-07-30. Getting to it from a data-block: `ID.library` ("Library file the data-block is linked from", `Library, (readonly)`) and `ID.library_weak_reference` ("Weak reference to a data-block in another library .blend file (used to re-use already appended data instead of appending new copies)", `LibraryWeakReference, (readonly)`) — https://docs.blender.org/api/4.5/bpy.types.ID.html (lines 76–90), read 2026-07-30.

**How long a recorded path may be.** The manual states the limits:

> Text Fields
>
> Fixed strings are used internally, and while it is not useful to list all limits, here are some common limits. Used for data-block names, modifiers, vertex groups, UV layers…
>
> directory:
>
> 767
>
> file-name:
>
> 255
>
> file-path:
>
> 1023

Source: manual, https://docs.blender.org/manual/en/4.5/advanced/limits.html. Date read: 2026-07-30.

The file's own path stands out as the one that is explicitly read-only:

> filepath
>
> Path to the .blend file
>
> Type:
>
> string, default “”, (readonly, never None)

Source: Python API documentation, https://docs.blender.org/api/4.5/bpy.types.BlendData.html (lines 65–71). Date read: 2026-07-30.

**LABEL: inferred** — the properties in the table above can be written to. The step: the documentation marks read-only properties by putting `readonly` in the type line, as it does for `BlendData.filepath` just above and for `Property.is_path_output`; none of the properties in the table carries that word, so they are not read-only. The reader has to supply the convention, so this is inferred rather than documented.

**Making sense of a relative path.** Two documented helpers turn the recorded form into an absolute one and back:

> bpy.path.abspath(path, \*, start=None, library=None)
>
> Returns the absolute path relative to the current blend file using the “//” prefix.
>
> Parameters:
>
> start (str | bytes) – Relative to this path, when not set the current filename is used.
>
> library (bpy.types.Library) – The library this path is from. This is only included for convenience, when the library is not None its path replaces start.

> bpy.path.relpath(path, \*, start=None)
>
> Returns the path relative to the current blend file using the “//” prefix.

Source: Python API documentation, https://docs.blender.org/api/4.5/bpy.path.html. Date read: 2026-07-30.

**LABEL: observed** — Cycles' own files use exactly this, including the `library` argument, when it needs the real location of a script a node points at. File `Blender Sources/cycles_4.5/osl.py`, line 209:

```
        osl_path = bpy.path.abspath(script.filepath, library=script.library)
```

and at line 169, inside `update_external_script`:

```
    script_path = bpy.path.abspath(filepath, library=library)
```

That is what the file says. What happens when it runs is a step beyond, and I have not taken it.

The API documentation has a page devoted to exactly this trap, and it is the most useful thing I found for anyone who means to read paths back out:

> Relative File Paths
>
> Blender’s relative file paths are not compatible with standard Python modules such as sys and os. Built-in Python functions don’t understand Blender’s // prefix which denotes the blend-file path.
>
> A common case where you would run into this problem is when exporting a material with associated image paths:
>
> \>\>\> bpy.path.abspath(image.filepath)
>
> When using Blender data from linked libraries there is an unfortunate complication since the path will be relative to the library rather than the open blend-file. When the data block may be from an external blend-file pass the library argument from the bpy.types.ID.
>
> \>\>\> bpy.path.abspath(image.filepath, library=image.library)
>
> These returns the absolute path which can be used with native Python modules.

Source: Python API documentation, https://docs.blender.org/api/4.5/info_gotchas_file_paths_and_encoding.html (lines 3–18). Date read: 2026-07-30. In plain terms: a recorded path read back out of a linked data-block is relative to the file it came from, not to the file you have open, and reading it without saying so gives the wrong answer.

**In the interface.** The Outliner has two display modes that show the file's contents, one of which lists properties as well:

> Blender File:
>
> Lists all data in the current blend-file. On the right side of the list, a shield icon shows the number of users – clicking it adds or removes a fake user.
>
> Data API:
>
> Lists every data-block in the file along with any properties that it might have.

Source: manual, https://docs.blender.org/manual/en/4.5/editors/outliner/interface.html. Date read: 2026-07-30.

For linked .blend files the Blender File mode shows the path outright:

> Linked data-blocks are indicated with a chain icon in the Outliner. They’re also listed in the Outliner’s Blender File Display Mode, along with the path of the blend-file they originate from.

Source: manual, https://docs.blender.org/manual/en/4.5/files/linked_libraries/link_append.html. Date read: 2026-07-30. The manual's table of data-block types says the same about the Library type: "References to an external blend-file. Access from the Outliner’s Blender File view." (https://docs.blender.org/manual/en/4.5/files/data_blocks.html, read 2026-07-30).

Whether a thing still has a path in use, rather than being packed inside, is shown beside the path itself:

> You know that a data is packed when you see a little “gift box” icon displayed next to its path.

and

> A single file can be packed by clicking on the little “gift box” icon to the left of its file-path UI widget.

Source: manual, https://docs.blender.org/manual/en/4.5/files/blend/packed_data.html. Date read: 2026-07-30.

And a data-block's own panel shows and edits its path directly. For an image:

> Common Options
>
> File
>
> Used for replacing or packing files.
>
> Pack
>
> Embed the resource into the current blend-file. See Packed Data.
>
> Path
>
> Path to the linked file.

Source: manual, https://docs.blender.org/manual/en/4.5/editors/image/image_settings.html. Date read: 2026-07-30. (I opened the picture that page names, `_images/editors_image_image-settings_movie-image-panel.png`. It shows the Image panel with Source set to Generated, so the Path field is not visible in it. It settles nothing about the Path field and I am not using it as evidence.)

The same pattern holds for the other kinds. All quotations below are from the manual, read 2026-07-30:

- **Sound, in a sequencer strip** — "File Path / Path to the file used by the selected sound data-block." https://docs.blender.org/manual/en/4.5/editors/video_sequencer/sequencer/sidebar/strip.html
- **Any sequencer strip's own source** — "The Source panel shows (and lets you change) the file which the strip points to, as well as how this file should be displayed." with "Directory / The folder containing the source file for the strip." and "Filename / The full name of the source file." Same page and address.
- **Volume** — "OpenVDB File / File Path / The VDB file to use." https://docs.blender.org/manual/en/4.5/modeling/volumes/properties.html
- **Alembic or USD cache, via a modifier** — "Cache File / Data-block menu to select the Alembic or USD file. / File Path / Path to Alembic or USD file. / Object Path / The path to the Alembic or USD object inside the archive or stage." https://docs.blender.org/manual/en/4.5/modeling/modifiers/modify/mesh_sequence_cache.html
- **Mesh Cache modifier** — "File Path / Path to the external cache file containing the animation data." https://docs.blender.org/manual/en/4.5/modeling/modifiers/modify/mesh_cache.html
- **Text** — "The Text editor footer displays whether the text is saved internally or externally and if there are unsaved changes to an external file. For external files, this region also displays the file path to the text file." https://docs.blender.org/manual/en/4.5/editors/text_editor.html
- **Geometry-nodes import nodes** each take a path as an input socket, so a node tree records them too — "Path / The path to the .obj file to be imported." https://docs.blender.org/manual/en/4.5/modeling/geometry_nodes/input/import/obj.html. The matching pages in the same folder say the same for their formats: "The path to the .stl file to be imported.", "The path to the .ply file to be imported.", "The path to the VDB file to be imported.", "The path to the txt file.", and "The path to the CSV file. The file must be structured with one row per point and values separated by a delimiter."

Which kinds of thing can be packed rather than pointed at is set out in the manual's table of data-block types, whose "Pack" column is defined as "File Packing, supports file contents being packed into the blend-file (not applicable for most data-blocks which have no file reference)". Source: manual, https://docs.blender.org/manual/en/4.5/files/data_blocks.html. Date read: 2026-07-30. Its glossary puts the same idea in one line: "Resource — External files such as images, sounds, fonts and volumes files that can be packed into a blend-file." (https://docs.blender.org/manual/en/4.5/glossary/index.html, read 2026-07-30). The manual also warns that packing is not universal:

> Not all external files can be packed
>
> Some typically heavy external files, like videos from the Sequence Editor or Movie Clips, cannot be packed in a blend-file.

Source: manual, https://docs.blender.org/manual/en/4.5/files/blend/packed_data.html. Date read: 2026-07-30.

### By what — writing back

**All of them at once, by operator.**

> bpy.ops.file.make_paths_absolute()
>
> Make all paths to external files absolute

> bpy.ops.file.make_paths_relative()
>
> Make all paths to external files relative to current .blend

Source: Python API documentation, https://docs.blender.org/api/4.5/bpy.ops.file.html (lines 215–221). Date read: 2026-07-30.

The manual gives the same two as menu entries, alongside the rest of the External Data menu:

> Make Paths Relative
>
> Make all paths to external files relative to the current blend-file.
>
> Make Paths Absolute
>
> Make all paths to external files absolute (= full path from the system’s root).
>
> Report Missing Files
>
> This option is useful to check if there are links to unpacked files that no longer exist. After selecting this option, a warning message will appear in the Info editor’s header. If no warning is shown, there are no missing external files.
>
> Find Missing Files
>
> In case you have broken links in a blend-file, this can help you to fix the problem. A File Browser will show up. Select the desired directory (or a file within that directory), and a search will be performed in it, recursively in all contained directories. Every missing file found in the search will be recovered. Those recoveries will be done as absolute paths, so if you want to have relative paths you will need to select Make Paths Relative.

Source: manual, https://docs.blender.org/manual/en/4.5/interface/window_system/topbar.html. Date read: 2026-07-30. The matching operators are `bpy.ops.file.report_missing_files()` ("Report all missing external files") and `bpy.ops.file.find_missing_files(*, find_all=False, directory='', …)` ("Try to find missing external files"), with `find_all (boolean, (optional)) – Find All, Find all files in the search path (not just missing)` and `directory (string, (optional, never None)) – Directory, Directory of the file` — Python API documentation, https://docs.blender.org/api/4.5/bpy.ops.file.html, read 2026-07-30.

**One at a time, by assignment.** As above: each `filepath` property in the table is not marked read-only. `Image` in particular distinguishes writing the path with and without reloading the data — `filepath` versus `filepath_raw`, "Image/Movie file name (without data refreshing)" (same source and date as the table).

**At save time.** Two documented ways rewrite paths as the file is written:

> relative_remap (boolean, (optional)) – Remap Relative, Remap relative paths when saving to a different directory

Source: Python API documentation, https://docs.blender.org/api/4.5/bpy.ops.wm.html, on `bpy.ops.wm.save_as_mainfile`. Date read: 2026-07-30. The manual's wording for the same option:

> Remap Relative
>
> This option remaps Relative Paths (such as linked libraries and images) when saving a file in a new location.

Source: manual, https://docs.blender.org/manual/en/4.5/files/blend/open_save.html. Date read: 2026-07-30.

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

Source: Python API documentation, https://docs.blender.org/api/4.5/bpy.types.BlendDataLibraries.html, on `write()`. Date read: 2026-07-30.

**For linked .blend files specifically.** There is a dedicated set of tools, because a library is more than a path.

> bpy.ops.wm.lib_relocate(\*, library='', filepath='', directory='', filename='', files=None, …)
>
> Relocate the given library to one or several others
>
> Parameters:
>
> library (string, (optional, never None)) – Library, Library to relocate
>
> filepath (string, (optional, never None)) – File Path, Path to file
>
> directory (string, (optional, never None)) – Directory, Directory of the file
>
> filename (string, (optional, never None)) – File Name, Name of the file

> bpy.ops.wm.lib_reload(\*, library='', filepath='', directory='', filename='', …)
>
> Reload the given library

Source: Python API documentation, https://docs.blender.org/api/4.5/bpy.ops.wm.html (lines 1510–1596). Date read: 2026-07-30.

From the Outliner:

> bpy.ops.outliner.lib_relocate()
>
> Relocate the library under cursor

and `bpy.ops.outliner.lib_operation(*, type='DELETE')` with `type (enum in ['DELETE', 'RELOCATE', 'RELOAD'], (optional))` — Python API documentation, https://docs.blender.org/api/4.5/bpy.ops.outliner.html (lines 311–331), read 2026-07-30.

The manual describes the same from the interface, and says what it is for:

> Relocate Library
>
> Reference
>
> Editor:
>
> Outliner
>
> Menu:
>
> Context menu ‣ Relocate
>
> When the Outliner is in the Blender File Display Mode, a right-click on a library (a linked blend-file) will show the Relocate option. It allows to replace it by a different file. This can be used to either fix a broken linked library (e.g. because the file was moved or renamed), or to switch to a variation of the same data in a different file.
>
> Broken Libraries
>
> If Blender cannot find a library while loading a blend-file, it will create placeholder data-blocks to replace missing linked ones. That way, references to the missing data are not lost, and by relocating the missing library, the lost data can be automatically restored.

Source: manual, https://docs.blender.org/manual/en/4.5/files/linked_libraries/link_append.html. Date read: 2026-07-30.

The library data-block itself exposes its path and a reload:

> filepath
>
> Path to the library .blend file
>
> Type:
>
> string, default “”, (never None, blend relative // prefix supported)

> reload()
>
> Reload this library and all its linked data-blocks

Source: Python API documentation, https://docs.blender.org/api/4.5/bpy.types.Library.html. Date read: 2026-07-30.

And the release notes record a change here in 4.5:

> ## Linking & Library Overrides
>
> * Linked IDs can now also be relocated, if they are not a dependency of any other linked data (blender/blender!135936, blender/blender@0eba8caaf9, blender/blender@3fa35aa4ee).

Source: release notes, https://developer.blender.org/docs/release_notes/4.5/core/. Date read: 2026-07-30. The manual's matching page:

> Relocate Linked ID
>
> […]
>
> A right-click a linked ID will show the Relocate option in the ID Data sub-menu. It allows to relocate a directly linked ID by another one, from the same library or from a different one. This can be used to either fix a broken linked data-block (e.g. because the ID was renamed), or to switch to a variation of the same data.
>
> Note
>
> This operation is only available on data-blocks that are directly linked, and are not a dependency of any other linked data.

Source: manual, https://docs.blender.org/manual/en/4.5/files/linked_libraries/link_append.html. Date read: 2026-07-30.

**One hard limit on writing back.** All of the above rewrites paths in the file you have open. The manual states the boundary plainly:

> As described above, Blender itself is not allowed to write to other blend-files than the currently open one. This rule helps to limit complexities; for example, it is hard to reliably implement an undo system when manipulating other files. The rule does get in the way of mass-updating assets when they are stored in various blend-files.
>
> Since there is already tooling that can manipulate blend-files outside of Blender itself (see Blender Asset Tracer), it’s possible to also create an external tool for doing such edits across blend-files. Such a tool might even be implemented via Blender’s application templates system, or as an add-on; the rule above applies to Blender itself, not to its add-ons.

Source: manual, https://docs.blender.org/manual/en/4.5/files/asset_libraries/introduction.html. Date read: 2026-07-30. So: one file at a time, from inside Blender. The manual says outside tooling exists and names one, but my sources say nothing whatever about what it can do.

**What decides the form a newly recorded path takes.** It is a preference, not a per-file setting:

> Default To – Relative Paths
>
> Default value for Relative Paths when loading external files such as images, sounds, and linked libraries. It will be ignored if a path is already set.

Source: manual, https://docs.blender.org/manual/en/4.5/editors/preferences/save_load.html. Date read: 2026-07-30.

**Taking the paths away altogether.** Packing removes the need for a recorded location, and is its own set of operators — `bpy.ops.file.pack_all()` ("Pack all used external files into this .blend"), `bpy.ops.file.unpack_all(*, method='USE_LOCAL')` ("Unpack all files packed into this .blend to external ones"), `bpy.ops.file.autopack_toggle()` ("Automatically pack all external files into the .blend file"), `bpy.ops.file.pack_libraries()`, `bpy.ops.file.unpack_libraries()` ("Restore all packed linked data-blocks to their original locations") — Python API documentation, https://docs.blender.org/api/4.5/bpy.ops.file.html, read 2026-07-30. Unpacking is itself a path decision, and the manual lists the choices, which are all about where the path ends up pointing:

> Use files in current directory (create when necessary)
>
> Unpacks all files in the same directory // as the blend-file, grouping them in proper folders (like ''textures'' for instance). However, if the final file exists already, it will use that file, instead of unpacking it.
>
> Write files to current directory (overwrite existing files)
>
> As with previous option, but if the final file exists already, it will overwrite it.
>
> Use files in original location (create when necessary)
>
> Unpacks all files in their original location. However, if the final file exists already, it will use that file, instead of unpacking it.
>
> Write files to original location (overwrite existing files)
>
> As with previous option, but if the final file exists already, it will overwrite it.
>
> Disable AutoPack, keep all packed files
>
> Only deactivates the Automatically Pack Resources option.

Source: manual, https://docs.blender.org/manual/en/4.5/files/blend/packed_data.html. Date read: 2026-07-30.

### Where I looked, for the whole of question 5

- **Finding the read side.** Searched all four sources for `blend_paths`, which appears in exactly one documentation file (`api_4.5/bpy.utils.txt`) plus two generated index pages. Searched every `bpy.types.*.txt` for a property line reading exactly `filepath` (27 types, all listed above) and for the phrase `blend relative // prefix supported` (47 occurrences, 41 files). Read the whole of `api_4.5/bpy.path.txt` and `api_4.5/info_gotchas_file_paths_and_encoding.txt`.
- **Finding the write side.** Searched for `make_paths_relative`, `make_paths_absolute`, `find_missing_files`, `report_missing_files`, `pack_all`, `unpack_all`, `autopack_toggle`, `pack_libraries`, `unpack_libraries`, `relative_remap`, `path_remap`, `lib_relocate`, `lib_reload`, `lib_operation`, `id_linked_relocate`, `relocate`, `remap`, `repath`, `rewrite path`, `path remap`, `broken link`, `missing file`. `repath`, `rewrite path` and `path remap` returned nothing anywhere. `remap` returned mostly unrelated hits (colour ramps, keyframe remapping, frame-range remapping) plus the two real ones quoted above.
- **Manual pages opened.** `interface/window_system/topbar.txt`, `files/blend/open_save.txt`, `files/blend/packed_data.txt`, `files/file_paths.txt`, `files/data_blocks.txt`, `files/linked_libraries/link_append.txt`, `files/asset_libraries/introduction.txt`, `editors/outliner/interface.txt`, `editors/image/image_settings.txt`, `editors/image/editing.txt`, `editors/preferences/file_paths.txt`, `editors/preferences/save_load.txt`, `editors/text_editor.txt`, `editors/video_sequencer/sequencer/sidebar/strip.txt`, `modeling/volumes/properties.txt`, `modeling/modifiers/modify/mesh_cache.txt`, `modeling/modifiers/modify/mesh_sequence_cache.txt`, the six `modeling/geometry_nodes/input/import/*.txt` pages, `advanced/limits.txt`, `glossary/index.txt`.
- **Pictures opened.** `_images/editors_image_image-settings_movie-image-panel.png` (which turned out to settle nothing — noted above).
- **Cycles' files.** Searched all of `cycles_4.5` for `filepath`; the two path uses are in `osl.py` and `operators.py`, and the `osl.py` ones are quoted above.
- **Release notes.** Read `core.md` and `python_api.md` in full for anything on paths, libraries or relocation; searched all nineteen release-note files for `path`, `librar`, `relocate`, `missing` and `pack`.

### Not covered

- Whether `blend_paths()` covers every kind of external reference. The documentation says "external files referenced by the loaded .blend file" and no more; it does not enumerate what counts, and it gives no way to get from a returned string back to the data-block that holds it. If you need to know *which* thing a path belongs to, that call will not tell you.
- Whether writing a `filepath` property is enough on its own, or whether something must be reloaded afterwards. The manual hints at the problem for recovered files — "Recovered files might need to be reloaded. You can do that one by one, or you can save the blend-file and reload it again, so that all external files are reloaded at once" (https://docs.blender.org/manual/en/4.5/interface/window_system/topbar.html, read 2026-07-30) — and `Image` has the `filepath` / `filepath_raw` pair, but the sources do not state the rule for other types.
- Whether "all paths to external files" in `make_paths_relative` / `make_paths_absolute` means literally all of them, including caches and output paths. The wording says "external files"; it does not say what is in and what is out.
- Whether these path properties can be written on data that came from a linked library. `Library.is_editable` exists ("Datablocks in this library are editable despite being linked. Used by brush assets and their dependencies", and it is read-only), but the sources do not spell out the consequence for path properties.
- Nothing here tells you which recorded paths are currently broken, only that `report_missing_files` exists and reports into the Info editor's header.
- Reading and writing is confined to the open file. The manual's rule — "Blender itself is not allowed to write to other blend-files than the currently open one" — means there is no documented way to rewrite the paths recorded in a linked library from the file that links it. What the named outside tooling can do is entirely outside my sources.
- The manual documents no source-path field for the Movie Clip data-block itself. Its editor sidebar page covers only proxies and timecodes. The path exists in the API (`MovieClip.filepath`), so this is a gap in the manual rather than a missing feature.
- I did not establish what happens when a recorded path exceeds the stated 1023-character limit.

### Outside the interface

Mostly yes, with one clear exception.

The reading and the per-property writing are ordinary Python and work with no window, by the documented route `blender myscene.blend --background --python myscript.py` (https://docs.blender.org/api/4.5/info_tips_and_tricks.html) or with Blender built as a Python module, which "for the most part […] is equivalent to running a script in background-mode" (https://docs.blender.org/api/4.5/info_advanced_blender_as_bpy.html). Both read 2026-07-30.

The exception is **Find Missing Files**. The manual describes it as opening a file browser and waiting for a person:

> A File Browser will show up. Select the desired directory (or a file within that directory), and a search will be performed in it, recursively in all contained directories.

Source: manual, https://docs.blender.org/manual/en/4.5/interface/window_system/topbar.html. Date read: 2026-07-30. Its operator does take a `directory` argument, which suggests the directory could be supplied instead of chosen — but the sources nowhere say that calling it with arguments works without a window, and I am not going to guess. **Not in the sources.**

The same caution applies to `wm.lib_relocate` and `outliner.lib_relocate`: the first takes `library`, `filepath`, `directory` and `filename` as arguments, the second is documented as acting on "the library under cursor", which is an interface idea. The sources do not state which of them can be driven with no window.

One documented difference that matters if you drive Blender from outside: the command line can override the render output but not, as far as the sources say, any of these reference paths — and loading a blend-file resets what the command line had set (https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html, read 2026-07-30).

### Operating system

The sources do not say the reading or writing of these paths differs by operating system. They do say plainly that paths themselves behave differently across systems, and give three helpers for exactly that, quoted in full under question 4's operating-system line: `bpy.path.native_pathsep` ("Replace the path separator with the systems native os.sep"), `bpy.path.basename` ("Equivalent to os.path.basename, but skips a “//” prefix. Use for Windows compatibility."), and `bpy.path.resolve_ncase` ("Resolve a case insensitive path on a case sensitive system, returning a string with the path if found else return the original path"). Source: Python API documentation, https://docs.blender.org/api/4.5/bpy.path.html. Date read: 2026-07-30.

The practical consequence — that a path recorded on one system may not resolve on another — is not stated in the sources and I am not stating it.

One difference the sources *do* state outright, and it bites when reading paths out: a recorded path may not be text Blender can otherwise handle.

> To avoid the problem for Python integration and script authors we have decided that all strings in blend-files must be UTF-8, ASCII compatible. This means assigning strings with different encodings to an object name, for instance, will raise an error.
>
> Paths are an exception to this rule since the existence of non-UTF-8 paths on the user’s file system cannot be ignored. This means seemingly harmless expressions can raise errors, e.g:
>
> \>\>\> print(bpy.data.filepath)
> UnicodeEncodeError: 'ascii' codec can't encode characters in position 10-21: ordinal not in range(128)

and the advice that follows:

> Avoid manipulating file paths as strings directly, use os.path functions instead.
>
> Use os.fsencode() or os.fsdecode() instead of built-in string decoding functions when operating on paths.
>
> To print paths or to include them in the user interface use repr(path) first or "%r" % path with string formatting.

Source: Python API documentation, https://docs.blender.org/api/4.5/info_gotchas_file_paths_and_encoding.html (lines 20–61). Date read: 2026-07-30. The page says this arises from "the existence of non-UTF-8 paths on the user’s file system", which is a property of the file system rather than of Blender; it does not name which operating systems.

---

### What the hand tests settled for question 5

**Observed (2026-08-04).** Tests were run on the Creator's own computer — a MacBook Pro with an M1 Pro chip running macOS 26.5.2 — with Blender **4.5.12 LTS** (hash 84afd5f785f7), started with no window, with `--factory-startup`, and with a throwaway settings folder. The full record, with the exact commands and the raw output, is in `Preparation/Blender Research/Findings/4.5/Reading a file.md` under questions 5 and 6. In short:

- An image's `filepath` was read, assigned a new value, saved and read back after reopening the file. It took the new value and kept it. This settles the step marked inferred above, that a property without a `readonly` mark can be written.
- All of that was done with no window at all, so reading, assigning, saving and reopening hold outside Blender's interface.
- `bpy.utils.blend_paths()` returned the image's path and one further entry nothing in the file pointed at: the Blender application's own brushes file.
- `bpy.data.file_path_map()` exists in this version and returned the image's path, filed under the image. It returned nothing for the render output path or the File Output node's paths.
- `bpy.data.file_path_foreach` does not exist in this version.
- `bpy.ops.file.find_missing_files(directory=…)` ran with no window, rewrote the recorded path, and the change survived saving and reopening. The rewritten path came back in the relative `//` form.
- Both `pack_libraries` and `unpack_libraries` are present under `bpy.ops.file` in this version.

**Observed (2026-08-05).** The Creator ran the same search from the Python Console inside Blender's window, on the same computer, with a directory supplied. It returned `{'FINISHED'}`, no File Browser window opened, and the image's recorded path afterwards read `//../15/tex_5.2/probe.png`. They reported that console as identical in all four versions, this one included; it was not established why the recovered path names `tex_5.2` in a version other than 5.2, and nothing here explains it. The full record is in `Preparation/Blender Research/Findings/4.5/Reading a file.md` under question 6.

**Observed (2026-08-06).** All of that was run again on a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH — with the same Blender build, no window, `--factory-startup`, and this topic's own settings folder. **Every result agreed with the Mac's**: the path was read, assigned, saved and read back with the new value; all of it worked with no window; the list of external paths held the image's path and Blender's own brushes file; the path map returned the image's path and nothing for the output paths; `bpy.data.file_path_foreach` does not exist in this version; the search rewrote the path in the relative form and the change survived a save and a reopening; and both packing operators are present. The search was also run inside Blender's window on that machine, with a folder supplied: it returned `{'FINISHED'}` and **no File Browser opened**, as on the Mac. One thing differed and neither run explains it: the recovered path there read `//search/deeper/probe.png`, the plain relative path to where the file was, rather than the Mac's `//../15/tex_5.2/probe.png`. Both stand. The full record is in `Preparation/Blender Research/Findings/4.5/Reading a file.md` under questions 5 and 6, kept separate from the Mac's.

The tests were run on macOS and on Linux, so **Windows** is the only operating system left open here.

---

## Things a person with Blender 4.5 open could settle in a few minutes

The hand-test step ran on 2026-08-04 and settled every item that stood here for questions 4 and 5. What it found is written into `Preparation/Blender Research/Findings/4.5/Reading a file.md` as observed findings, and summarised in the two sections above. New equipment arrived on 2026-08-06 and the same tests were run again on a rented Linux machine; that is written into the same document, beside the Mac's results and never merged with them.

One thing is left, and it is not a test that was skipped:

1. **Anything on Windows.** No Windows machine has run it, so every operating-system line above is settled for macOS and Linux and open for Windows.

---

2026-08-04 — Opus 5 (Claude Code) — Reading a file, Blender 4.5

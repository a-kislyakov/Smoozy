# Dependencies — Blender 5.0 — second pass

**Sources consulted, and when.** Blender 5.0 manual (`Blender Sources/manual_5.0`, including its `_images` folder), Blender 5.0 Python API documentation (`Blender Sources/api_5.0`), Blender 5.0 release notes (`Blender Sources/release_notes_5.0`), and the Cycles add-on's own files (`Blender Sources/cycles_5.0`). All read on 2026-07-30 (the snapshot date given in my inputs).

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

This is a second pass. Only questions 1, 5, 10, 11, 13 and 15 are answered. For each one I have written down what I searched, what I searched for, what I opened, and what I ruled out.

A note on how I searched, so it does not have to be repeated sixteen times. Every search below was run across the whole of `manual_5.0`, the whole of `api_5.0`, and the whole of `release_notes_5.0` at once, unless I say otherwise. Where a search would have been swamped by the manual's picture files, I excluded the `_images` folder and say so.

---

## 1. What kinds of thing can a `.blend` file point at outside itself? Give the full list.

**LABEL: documented** for each individual kind listed below. **inferred** for the claim that the list is complete — see "Not covered".

### The short answer

The sources nowhere print a single list headed "everything a blend-file can point at". What they do give is three separate, overlapping enumerations, and a machine-readable one in the Python API that is the closest thing to a full list that exists in these sources. Put together, these are the kinds of outside thing a 5.0 blend-file can hold a pointer to.

### Enumeration A — the manual's data-block table

The manual's data-block table has a column for whether a type's file contents can be packed into the blend-file, and it introduces that column like this:

> "Pack
>
> File Packing, supports file contents being packed into the blend-file
> (not applicable for most data-blocks which have no file reference)."

— Manual, https://docs.blender.org/manual/en/5.0/files/data_blocks.html, read 2026-07-30

The types that carry a ✓ or ✗ in that column — the mark the legend ties to having a file reference — are Font, Image, Library, Movie Clip, Sounds and Text. Cache File carries a "—" and is listed here only because the API gives it a `filepath`; the table itself does not mark it as file-referencing. *(Citation check 2026-08-05: Cache File had been listed among the types the table marks as having a file reference; its Pack column reads "—".)*

> "Cache File … Used by Mesh Cache modifiers."
> "Font … References font files. Used by curve object-data of text objects."
> "Image … Image files. Used by shader nodes and textures."
> "Library … References to an external blend-file. Access from the Outliner's Blender File view."
> "Movie Clip … Reference to an image sequence or video file. Used in the Movie Clip editor."
> "Sounds … Reference to sound files. Used as data of speaker objects."
> "Text … Text data. Used by Python scripts and OSL shaders."

— Manual, https://docs.blender.org/manual/en/5.0/files/data_blocks.html, read 2026-07-30

### Enumeration B — the API's marked path properties

The Python API marks every property that holds a Blender file path with the phrase "blend relative // prefix supported". I searched all 2062 API pages for that phrase. Excluding the ones that belong to Preferences rather than to a blend-file, the complete set of marked properties is below, quoted with the API's own description of each:

- `BakeSettings.filepath` — "Image filepath to use when saving externally"
- `CacheFile.filepath` — "Path to external displacements file" (this description looks like a copy-paste slip in the docs; the type is documented elsewhere as the Alembic/USD archive — I quote it as written)
- `CacheFileLayer.filepath` — "Path to the archive"
- `CollectionExport.filepath` — "The file path used for exporting"
- `CompositorNodeOutputFile.directory` — "The directory where the image will be written" (its companion `file_name`, "The base name of the file. Other information might be included in the final file name depending on the node options", is marked "Supports template expressions" but not "blend relative")
- `DynamicPaintSurface.image_output_path` — "Directory to save the textures"
- `FluidDomainSettings.cache_directory` — "Directory that contains fluid cache files"
- `Image.filepath` and `Image.filepath_raw` — "Image/Movie file name" and "Image/Movie file name (without data refreshing)"
- `ImagePackedFile.filepath`
- `ImageStrip.directory`
- `Library.filepath` — "Path to the library .blend file"
- `LibraryWeakReference.filepath` — "Path to the library .blend file"
- `MeshCacheModifier.filepath` — "Path to external displacements file"
- `MovieClip.filepath` — "Filename of the movie or sequence file"
- `MovieClipProxy.directory` — "Location to store the proxy files"
- `MovieStrip.filepath`
- `MultiresModifier.filepath` — "Path to external displacements file"
- `NodeSocketStringFilePath.default_value` — "Input value used for unconnected socket"
- `NodesModifier.bake_directory` and `NodesModifierBake.directory` — "Location on disk where the bake data is stored"
- `OceanModifier.filepath` — "Path to a folder to store external baked images"
- `PointCache.filepath` and `PointCacheItem.filepath` — "Cache file path"
- `RenderSettings.filepath` — "Directory/name to save animations, # characters define the position and padding of frame numbers"
- `SequenceEditor.proxy_dir`
- `ShaderNodeScript.filepath` — "Shader script path"
- `ShaderNodeTexIES.filepath` — "IES light path"
- `Sound.filepath` — "Sound sample file used by this Sound data-block"
- `StripProxy.directory` and `StripProxy.filepath` — "Location to store the proxy files" and "Location of custom proxy file"
- `VectorFont.filepath`
- `Volume.filepath` — "Volume file used by this Volume data-block"
- `VolumeGrids.frame_filepath` — "Volume file used for loading the volume at the current frame. Empty if the volume has not be loaded or the frame only exists in memory."

— API, one page per type under https://docs.blender.org/api/5.0/ (for example https://docs.blender.org/api/5.0/bpy.types.Library.html, https://docs.blender.org/api/5.0/bpy.types.Volume.html), read 2026-07-30

Two path properties carry no "// prefix" marking but are still outward pointers:

- `Text.filepath` — "Filename of the text file" (API, https://docs.blender.org/api/5.0/bpy.types.Text.html)
- `Camera.custom_filepath` — "Path to the shader defining the custom camera" (API, https://docs.blender.org/api/5.0/bpy.types.Camera.html)

Both read 2026-07-30.

### Enumeration C — the geometry-node import nodes

> "Import Nodes
>
> Nodes used to import data from external file formats."

and the list of them: "CSV (.csv)", "Wavefront (.obj)", "Stanford PLY (.ply)", "STL (.stl)", "Text (.txt)", "OpenVDB (.vdb)".

— Manual, https://docs.blender.org/manual/en/5.0/modeling/geometry_nodes/input/import/index.html, read 2026-07-30

Each of these takes the file to read as a node input, described for the OBJ one as:

> "Path
>
> The path to the .obj file to be imported."

— Manual, https://docs.blender.org/manual/en/5.0/modeling/geometry_nodes/input/import/obj.html, read 2026-07-30

### The API's own "all the paths in this file" call

Blender 5.0 added a call whose whole purpose is to walk every outward file pointer in an open blend-file, which is the strongest evidence in these sources that the set is closed and known:

> "Add `bpy.data.file_path_foreach()`,
>   which calls a callback function for each file path used in the blend file"

— Release notes, https://developer.blender.org/docs/release_notes/5.0/python_api/, read 2026-07-30

> "file_path_foreach(visit_path_fn, *, subset=None, visit_types=None, flags={'SKIP_PACKED', 'SKIP_WEAK_REFERENCES'})
>
> Call visit_path_fn for the file paths used by all ID data-blocks in current bpy.data."

and its companion:

> "file_path_map(*, subset=None, key_types=None, include_libraries=False)
>
> Returns a mapping of all ID data-blocks in current bpy.data to a set of all file paths used by them."

— API, https://docs.blender.org/api/5.0/bpy.types.BlendData.html, read 2026-07-30

Note the second one's option "include_libraries (bool) – Include library file paths of linked data. False by default." Library files are counted separately from ordinary file paths.

### Things pointed at that are not files

Three more outward dependencies appear in the sources that are not file paths at all, but names looked up on the computer:

1. A colour management configuration. See question 15.
2. Colour space, display and view names. `ColorManagedDisplaySettings.display_device` is documented as "enum in ['NONE'], default 'NONE'" (API, https://docs.blender.org/api/5.0/bpy.types.ColorManagedDisplaySettings.html) — the list of valid values is not fixed in the documentation.
3. A render engine identifier. `RenderSettings.engine` — "Engine to use for rendering", "enum in ['BLENDER_EEVEE'], default 'BLENDER_EEVEE'" (API, https://docs.blender.org/api/5.0/bpy.types.RenderSettings.html). Cycles is an add-on, so this names something loaded from outside the file.

All read 2026-07-30.

### Where I looked

- Searched the whole manual for: "external file", "External Files", "external resource", "relative path", "Relative Paths", "//", "dependencies", "packed", "Pack Resources", "External Data".
- Opened in the manual: `files/index.txt`, `files/introduction.txt`, `files/data_blocks.txt`, `files/file_paths.txt`, `files/blend/open_save.txt`, `files/blend/packed_data.txt`, `files/linked_libraries/link_append.txt`, `files/linked_libraries/library_overrides.txt`, `interface/window_system/topbar.txt`, `modeling/geometry_nodes/input/import/index.txt` and `obj.txt`, `render/cycles/osl/index.txt`, `render/cycles/osl/camera.txt`.
- Opened the manual picture `_images/files_data-blocks_outliner-blender-file-view.png`. It shows the Outliner's display-mode menu and a "Current File" tree with Brushes, Cameras, Collections, Images, Lights, Materials, Meshes, Objects, Scenes, Screens, Texts, Window Managers, Workspaces, Worlds. It does not show any library entry, so it settles nothing about outward pointers. I mention it because I looked.
- Searched all API pages for the property name `filepath` (106 files hit), then narrowed to pages where `filepath` is a property rather than an operator argument. Separately searched for the marker phrases "blend relative", "Supports template expressions", and for property names matching `base_path`, `directory`, `cache_directory`, `*_filepath`, `*_directory`, `*_path`.
- Ruled out as not being blend-file dependencies: everything under `PreferencesFilePaths` (application settings, not file contents), `Theme.filepath`, `UserExtensionRepo.directory`, `ScriptDirectory.directory`, `AssetRepresentation.full_path`, `FileSelectParams.directory` — these belong to the application or to a browser session. Also ruled out `DriverTarget.data_path`, `FCurve.data_path`, `KeyingSetPath.data_path`, `IDOverrideLibraryProperty.rna_path`, `ViewerPath`, `MotionPath`, `XrUserPath` — despite the word "path" these are internal references, not locations on disk.
- Searched the release notes for "path", "librar", "link", "pack", "depend".
- Searched the Cycles files for `filepath`, `FILE_PATH`, `DIR_PATH`, `path`. The only outward pointers there are the OSL script path and the custom-camera shader path, both handled in `osl.py`.

**Not covered.** I cannot show the list is complete. There is no page in these sources that claims to enumerate every outward pointer, and the API marker phrase I leaned on ("blend relative // prefix supported") is a property annotation, not a promise of completeness — a path property that does not accept the `//` prefix would not appear in my sweep. The two I found that way (`Text.filepath`, `Camera.custom_filepath`) show that gap is real. The answer also does not cover add-ons and extensions the file may need, or Python scripts a file may run, beyond noting the render-engine identifier. It does not cover anything about whether these pointers are followed at render time.

**Outside the interface.** The list is a property of the file format and the data model, not of the window. `bpy.data.file_path_foreach()` and `bpy.data.file_path_map()` are ordinary Python API calls and the API documentation attaches no interface condition to them (API, https://docs.blender.org/api/5.0/bpy.types.BlendData.html, read 2026-07-30). The manual's `Pack Resources`, `Make Paths Relative` and `Report Missing Files` are described as menu items, and the sources do not say what, if anything, differs when Blender is driven from outside its own window.

**Operating system.** The sources do not say the list of kinds differs by operating system, and they do not say it is the same either. The one operating-system remark I found nearby concerns name lengths, not the list: "Windows (NTFS) only supports up to 255 chars in its file names. This limit can now more easily be reached with longer data-block names, when they are used as part of a generated file name." (Release notes, https://developer.blender.org/docs/release_notes/5.0/core/, read 2026-07-30).


**Observed — hand test entries 1, 2 and 20, run 2026-08-05 on the Creator's own computer** (MacBook Pro, M1 Pro chip, macOS 26.5.2, Apple's own graphics), with Blender 5.0.1 started from a terminal with no window, using a throwaway settings folder set through `BLENDER_USER_RESOURCES`. The full list of tests is at `Audits/Blender Research/Dependencies/Hand tests.md`.

**What was done.** A file was built by script holding an image, a sound, a font, an OpenVDB volume path, an OSL script path, an IES light-profile path and a text file loaded from disk, all in an `assets` folder beside the blend-file, with the render output path set to `//renders/frame_`. The file was reopened in the same version and `bpy.utils.blend_paths(absolute=False, packed=False, local=False)` and `bpy.data.file_path_map()` were printed.

**What came back.** `blend_paths` returned exactly these seven, and nothing else:

```
    '//assets/light.ies'
    '//assets/smoke.vdb'
    '//assets/shader.osl'
    '//assets/tex.png'
    '//assets/beep.wav'
    '//assets/notes.txt'
    '//assets/testfont.ttf'
```

`file_path_map()` returned one row per data-block. Every row was empty except these (the empty rows for the window manager, the workspaces, the screens, the scene, the objects, the mesh and the built-in font are dropped here; nothing else is):

```
    <bpy_struct, PointLight("L") ...> -> ["'//assets/light.ies'"]
    <bpy_struct, Volume("Vol") ...> -> ["'//assets/smoke.vdb'"]
    <bpy_struct, Material("Mat") ...> -> ["'//assets/shader.osl'"]
    <bpy_struct, Image("tex.png") ...> -> ["'//assets/tex.png'"]
    <bpy_struct, Sound("beep.wav") ...> -> ["'//assets/beep.wav'"]
    <bpy_struct, Text("notes.txt") ...> -> ["'//assets/notes.txt'"]
    <bpy_struct, VectorFont("Arial Regular") ...> -> ["'//assets/testfont.ttf'"]
    <bpy_struct, VectorFont("Bfont Regular") ...> -> []
```

`file_path_map(include_libraries=True)` gave the same list. The render output path `//renders/frame_` appeared in neither list. The text file's path was stored as `//assets/notes.txt`, so that property does take the double-slash form even though its documentation does not say so.

**Which machinery this version has.** Checked by name in all four versions. Present here and absent in 4.5: `bpy.data.file_path_foreach`, `Library.is_archive`, `Library.archive_libraries`, `bpy.data.colorspace` (`bpy.data.colorspace.is_missing_opencolorio_config` returned `False`). `WindowManager.reports` is absent here and in 4.5, and present in 5.2. Present in all four: `bpy.data.file_path_map`, `bpy.utils.blend_paths`, `bpy.ops.file.pack_libraries`, `bpy.ops.file.pack_all`, `bpy.ops.file.report_missing_files`, `bpy.ops.file.find_missing_files`, `bpy.ops.outliner.id_linked_relocate`, `Library.parent`, `ID.is_missing`, `ID.library_weak_reference`, `bpy.path.resolve_ncase`.

**One item on the list is not called the same thing in every version.** The properties of `bpy.types.CompositorNodeOutputFile` that are not already on `bpy.types.Node` are, in this version:

```
['file_output_items', 'active_item_index', 'directory', 'file_name', 'format', 'save_as_render']
```

In 4.5 the folder is `base_path`; in 5.0, 5.1 and 5.2 it is `directory`, with a separate `file_name`, and setting `base_path` there raises `AttributeError: 'CompositorNodeOutputFile' object has no attribute 'base_path'`.

**Both standing lines, brought up to date by this test.** *Driven from outside:* every reading above was taken with Blender running with no window, so the recording of these references and the two ways of listing them hold outside the interface. *Operating system:* macOS for this test, and the same test was run on a rented Linux machine on 2026-08-06 — that result is recorded below. Windows stays untested.

**What this does not settle.** Whether the seven kinds above are all the kinds a `.blend` can point at. The file was built to hold those seven; a kind nobody thought to put in it would not appear.


**Observed — the same test run again on Linux, 2026-08-06.** Every name above was asked for again in this version on a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH. **Every row came back the same as on the Creator's Mac.** Present here and absent in 4.5: nothing differs between the two machines at all. The file built to hold seven kinds returned the same seven paths on Linux:

```
    '//assets/beep.wav'
    '//assets/light.ies'
    '//assets/notes.txt'
    '//assets/shader.osl'
    '//assets/smoke.vdb'
    '//assets/testfont.ttf'
    '//assets/tex.png'
```

and `bpy.data.file_path_map()` returned the same seven non-empty rows. The render output path, `//renders/frame_`, appeared in neither list. The text file's path stored as `//assets/notes.txt`. The properties of `bpy.types.CompositorNodeOutputFile` came back exactly as recorded above.

**Two corrections that came out of the Linux run.** First, `bpy.utils.blend_paths` takes no `include_libraries` keyword in any of the four versions on either machine — passing one gives `TypeError: blend_paths() takes at most 3 keyword arguments (4 given)`, and its own documentation reads `blend_paths(*, absolute=False, packed=False, local=False)`. That keyword belongs to `bpy.data.file_path_map`, where it returned the same rows. Second, the IES light profile could not be set through `light.cycles.use_ies`, which does not exist in these builds — `AttributeError: 'CyclesLightSettings' object has no attribute 'use_ies'` — so an IES Texture node was used instead, and the path still appears in both lists.

**Something a Mac-written file carries that nothing had noticed.** Opening a blend-file saved on the Creator's Mac on the Linux machine, `bpy.utils.blend_paths` returned, among its paths, `'/Applications/Blender 5.0.1.app/Contents/Resources/5.0/datafiles/assets/brushes/essentials_brushes-gp_draw.blend'` — a path into the Mac's own Blender application, on a machine that has no such folder. It comes out of the file rather than out of the running Blender. Report Missing Files did **not** name it.

**Both standing lines, brought up to date by the Linux tests.** *Driven from outside:* every reading above was taken with Blender running with no window unless it says otherwise, so it holds outside the interface. *Operating system:* **now settled on two operating systems, macOS and Linux, and each is its own result** — what they agree on and where they differ is said above. **Windows is still untried: no Windows machine has run it.**

---

## 5. In what ways can a recorded location be written?

**LABEL: documented**, with one **observed** addition from Cycles' own files.

Six ways, as follows.

### (a) Absolute

> "Make Paths Absolute
>
> Make all paths to external files absolute (= full path from the system's root)."

— Manual, https://docs.blender.org/manual/en/5.0/interface/window_system/topbar.html, read 2026-07-30

### (b) Relative, written with a leading double slash

> "When you specify one of these external files, the default option is to make the path relative.
> Blender stores a partial path evaluated relative to the directory location of the referencing blend-file."

> "When relative paths are supported, the File Browser provides a Relative Path checkbox,
> when entering the path into a text field, use a double slash prefix (//) to make it so."

> "You cannot use relative paths into a new untitled blend-file.
> Save it before linking to external files."

— Manual, https://docs.blender.org/manual/en/5.0/files/blend/open_save.html, read 2026-07-30

The API says the same and names which properties accept it, with the marker "blend relative // prefix supported" (see question 1), and adds the important qualification that "relative" does not always mean relative to the open file:

> "When using Blender data from linked libraries there is an unfortunate complication
> since the path will be relative to the library rather than the open blend-file."

— API, https://docs.blender.org/api/5.0/info_gotchas_file_paths_and_encoding.html, read 2026-07-30

Which of the two is the default when a file is first written is a preference:

> "Default To – Relative Paths
>
> Default value for Relative Paths when loading external files
> such as images, sounds, and linked libraries. It will be ignored if a path is already set."

— Manual, https://docs.blender.org/manual/en/5.0/editors/preferences/save_load.html, read 2026-07-30

### (c) With hash marks standing for a frame number

> "When rendering an animation,
> the frame number is appended at the end of the file name with four padded zeros (e.g. image0001.png).
> You can set a custom padding size by adding the appropriate number of # anywhere in the file name
> (e.g. image_##_test.png translates to image_01_test.png)."

— Manual, https://docs.blender.org/manual/en/5.0/render/output/properties/output.html, read 2026-07-30

The same, stated for the command line:

> "The # characters are replaced by the frame number, and used to define zero padding.
>
> animation_##_test.png becomes animation_01_test.png
>
> test-######.png becomes test-000001.png
>
> When the filename does not contain #, the suffix #### is added to the filename."

— Manual, https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html, read 2026-07-30

### (d) With template expressions in curly braces

> "Path templates substitute template expressions (written {abcd}, where
> "abcd" is a variable name) in a filepath when the path is used."

> "This substitution happens internally at time of use (e.g. while rendering), so
> the template syntax will remain as-is in the path field."

The available variables in 5.0 are `blend_name`, `blend_dir`, `blend_name_lib`, `blend_dir_lib` in any supporting path, plus `fps`, `resolution_x`, `resolution_y`, `scene_name`, `camera_name` in render output paths, plus `node_name` in path properties on a node. Variables representing numbers may carry a format specifier after a colon, written with hash marks: `dance_{fps:###}.png`, `dance_{fps:.###}.png`, `dance_{fps:###.##}.png`. A literal brace is written doubled:

> "{{ translates to a single { in the final path.
>
> }} translates to a single } in the final path."

Where this form is allowed is narrow:

> "Currently path templates are only supported for the render output path in
> Scene properties and the output paths in the compositor's File Output
> node. More filepaths will support templates in future releases."

— Manual, https://docs.blender.org/manual/en/5.0/files/file_paths.html, read 2026-07-30

The API marks exactly three properties as accepting this form — `RenderSettings.filepath`, `CompositorNodeOutputFile.directory` and `CompositorNodeOutputFile.file_name` — with the words "Supports template expressions" (API, https://docs.blender.org/api/5.0/bpy.types.RenderSettings.html and https://docs.blender.org/api/5.0/bpy.types.CompositorNodeOutputFile.html, read 2026-07-30). I searched all 2062 API pages for that phrase; those two pages are the only hits, which matches the manual's statement above exactly.

The set of variables is stated as new in this version:

> "The following new path template variables have been introduced … `scene_name`, `camera_name`, `node_name`, `blend_name_lib`, `blend_dir`, `blend_dir_lib`"

— Release notes, https://developer.blender.org/docs/release_notes/5.0/core/, read 2026-07-30

### (e) With substitution tokens in angle brackets, for tiled textures

> "Substitution tokens are special sequences of characters in a filename
> that can be replaced with more meaningful and context aware information.
> In this case, tokens are identified by being text wrapped in angle bracket characters."

> "<UDIM>: A 4-digit notation calculated as 1001 + u-tile + v-tile * 10.
>
> <UVTILE>: A notation defined as u(u-tile + 1)_v(v-tile + 1)."

> "monster-basecolor.<UDIM>.png will load/save files like monster-basecolor.1021.png etc."

— Manual, https://docs.blender.org/manual/en/5.0/modeling/meshes/uv/workflows/udims.html, read 2026-07-30

The API confirms these are stored as written and resolved on use, and calls the stored form a "virtual filepath":

> "RESOLVE_TOKEN:
>
> Resolve Token.
>
> Resolve tokens within a virtual filepath to a single, concrete, filepath. Currently only used for UDIM tiles."

— API, https://docs.blender.org/api/5.0/bpy_types_enum_items/file_path_foreach_flag_items.html, read 2026-07-30

### (f) As a bare module name, resolved on a search path — **observed**

Cycles' own files show a third shape for the OSL script location, alongside absolute and relative: a name with no directory part at all, treated as a module to be found on a search path.

> ```
>     elif os.path.dirname(filepath) == "":
>         # module in search path
>         oso_path = filepath
>         ok = True
>     else:
>         # unknown
>         report({'ERROR'}, "External shader script must have .osl or .oso extension, or be a module name")
>         ok = False
> ```

— Cycles add-on file `osl.py`, function `update_external_script()`, lines 215–222, read 2026-07-30

Label for this: **observed** for what the file says (the branch exists, and the error message names "a module name" as a third accepted form). It would be **inferred**, not observed, to say what search path is used or that a render actually finds the module — the file does not say, and I did not run anything.

### What is not a way of writing a location

Packing removes the need for one:

> "Blender has the ability to encapsulate (incorporate)
> various kinds of data within the blend-file that is normally saved outside of the blend-file."

— Manual, https://docs.blender.org/manual/en/5.0/files/blend/packed_data.html, read 2026-07-30

### Where I looked

- Searched the whole manual and API for: "relative path", "Relative Paths", "absolute path", "// prefix", "double slash", "blend relative", "Supports template expressions", "path template", "# characters", "frame numbers", "UDIM", "UVTILE", "substitution token".
- Opened: `files/file_paths.txt`, `files/blend/open_save.txt`, `editors/file_browser.txt`, `editors/preferences/save_load.txt`, `editors/preferences/file_paths.txt`, `render/output/properties/output.txt`, `advanced/command_line/arguments.txt`, `modeling/meshes/uv/workflows/udims.txt`, `files/linked_libraries/link_append.txt`, `interface/window_system/topbar.txt`; API `bpy.path.txt`, `info_gotchas_file_paths_and_encoding.txt`, `bpy_types_enum_items/file_path_foreach_flag_items.txt`, `bpy.types.RenderSettings.txt`, `bpy.types.CompositorNodeOutputFile.txt`.
- Read `cycles_5.0/osl.py` in full.
- **Ruled out, with searches that found nothing.** I searched the manual and API (excluding the `_images` folder) for "backslash" and "drive letter" — the only hits are a Node Wrangler shortcut and an icon name, neither about paths. So the sources do not say whether a location may be written with backslashes or with a drive letter. I searched for a shell-style variable expansion inside path fields (`$` followed by capitals, within the `files` and `render/output` parts of the manual) and found none; environment variables appear only as things that configure Blender's own directories, never as something expanded inside a recorded path. I searched for "symbolic link" and "symlink" across manual and API: one passing mention in the asset catalogue page, nothing about recorded locations. I searched for "home directory" and a leading tilde: the only hit is the File Browser's bookmark list. So: no documented support for environment variables, `~`, or drive-letter/backslash conventions inside a recorded location.

**Not covered.** This answer does not settle what a location written in one of these ways *resolves to* — only how it may be written. It does not settle whether the six forms may be combined (for instance a relative path that also contains a UDIM token and hash marks), because the sources do not discuss combinations. It does not settle whether a path is normalised when stored. It does not settle what separator character is written to the file.

**Outside the interface.** Forms (a) to (d) are explicitly documented for use from outside the window. The command-line render-output argument accepts all of them:

> "-o, --render-output <path>
>
> Set the render path and file name.
> Use // at the start of the path to render relative to the blend-file.
>
> You can use path templating features such as {blend_name} in the path."

— Manual, https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html, read 2026-07-30

The same page warns that the order of arguments matters, which is a real difference from the interface: "blender --background test.blend --render-frame 1 --render-output "/tmp" … will not render to /tmp because --render-frame 1 renders before the output path is set", and "blender --background --render-output /tmp test.blend --render-frame 1 … will not render to /tmp because loading the blend-file overwrites the render output that was set." The working form is given as "blender --background test.blend --render-output /tmp --render-frame 1". Form (b)'s relative prefix is a property of the file, not the interface. Form (e) is a property of the stored image path. Form (f) is inside Cycles' Python and does not involve the window. The one interface-only thing here is the Relative Path checkbox in the File Browser, and the red highlighting and tooltip that report a broken template; the manual notes the underlying failure is not interface-only — "if there are errors in the render output path, then rendering an animation will fail with an error message indicating the path errors."

**Operating system.** The sources do not say. Two nearby remarks are the closest they come, and neither answers it: `bpy.path.basename` is documented as "Equivalent to os.path.basename, but skips a "//" prefix. Use for Windows compatibility.", and `bpy.path.native_pathsep` as "Replace the path separator with the systems native os.sep." (API, https://docs.blender.org/api/5.0/bpy.path.html, read 2026-07-30). Both imply the separator is a per-system matter but neither states what is stored.


**Observed — hand test entries 3, 14 and 15, run 2026-08-05 on the Creator's own computer** (MacBook Pro, M1 Pro chip, macOS 26.5.2, Apple's own graphics), with Blender 5.0.1 started from a terminal with no window, using a throwaway settings folder set through `BLENDER_USER_RESOURCES`. The full list of tests is at `Audits/Blender Research/Dependencies/Hand tests.md`.

**What was done.** A folder `proj` was made holding a blend-file and an `assets/tex.png`, with an `outer/tex.png` one level above it and a third copy in a folder in the home directory. Nine image data-blocks were made, each with `source` set to `FILE` and a different way of writing its location. What was stored was printed the instant each was assigned; the file was then saved twice, once with `bpy.ops.wm.save_as_mainfile(filepath=..., relative_remap=False)` and once with `relative_remap=True`; each was reopened with the environment variable `SMOOZY_DEP_TEST` set to the home-directory folder, and the stored text, `bpy.path.abspath` of it, and the image size after `reload()` were printed.

**What came back.** At the moment of assignment every form was stored exactly as typed. After saving — the same for both saves:

```
NAME=absolute       STORED='<BASE>/proj/assets/tex.png'          ABSPATH='<BASE>/proj/assets/tex.png' SIZE=(8, 8)
NAME=relative       STORED='//assets/tex.png'                    ABSPATH='<BASE>/proj/assets/tex.png' SIZE=(8, 8)
NAME=relative-up    STORED='//../outer/tex.png'                  ABSPATH='<BASE>/proj/../outer/tex.png' SIZE=(8, 8)
NAME=backslash-rel  STORED='//assets/tex.png'                    ABSPATH='<BASE>/proj/assets/tex.png' SIZE=(8, 8)
NAME=tilde          STORED='~/smoozy-path-test/tex.png'          ABSPATH='~/smoozy-path-test/tex.png' SIZE=(0, 0)
NAME=dollar-var     STORED='$SMOOZY_DEP_TEST/tex.png'            ABSPATH='$SMOOZY_DEP_TEST/tex.png' SIZE=(0, 0)
NAME=dollar-brace   STORED='${SMOOZY_DEP_TEST}/tex.png'          ABSPATH='${SMOOZY_DEP_TEST}/tex.png' SIZE=(0, 0)
NAME=drive-letter   STORED='C:/textures/tex.png'                 ABSPATH='C:/textures/tex.png' SIZE=(0, 0)
NAME=no-such-mount  STORED='/Volumes/NoSuchShare/tex.png'        ABSPATH='/Volumes/NoSuchShare/tex.png' SIZE=(0, 0)
```

A size of 8 by 8 means the file was found; 0 by 0 means it was not. The form typed for `backslash-rel` was `//assets\tex.png` and for `drive-letter` was `C:\textures\tex.png` — on saving, backslashes became forward slashes in both.

So, settled: a relative path may climb above the blend-file's own folder with `//../`; a home-directory shorthand is stored as typed and not expanded; an environment variable in either form is stored as typed and not expanded; backslashes are turned into forward slashes when the file is saved.

**The curly-brace template forms.** With the file saved as `out.blend`:

```
   render.filepath = '//templated/{blend_name}_frame_'
   frame_path(frame=1) = '<path>/templated/out_frame_0001.png'
   folder created? True   contents: ['out_frame_0001.png', 'out_frame_0002.png']
```

The same braces in an image texture path were left alone: stored `'//{blend_name}_tex.png'`, resolved `'<path>/{blend_name}_tex.png'`.

A broken one, `//broken/{blend_name_frame_`, stopped the render:

```
   frame_path ERR RuntimeError: Error: Parse errors in path '//broken/{blend_name_frame_':
- Invalid or incomplete template expression '{blend_name_frame_'.
   render ERR RuntimeError: Error: Parse errors in path '//broken/{blend_name_frame_':
- Invalid or incomplete template expression '{blend_name_frame_'.
   folder created? False   contents: None
```

and the same two lines were printed to the terminal on their own. All of this was the same in all four versions.

**A tile form Blender writes for itself.** An image loaded from `tiles/tile.1001.png` with `source` set to `TILED` had its path rewritten by Blender to `'//tiles/tile.<UDIM>.png'`, standing for tiles 1001 and 1002.

**Both standing lines, brought up to date by this test.** *Driven from outside:* settled for every form above — all of it was stored, resolved and rendered with no window, and the broken template fails there too, with its message printed to the terminal. *Operating system:* macOS for this test. The same test was run on a rented Linux machine on 2026-08-06 and that result is recorded below; Windows stays untested. What a Windows Blender writes into a path, and what a drive-lettered path made on Windows means elsewhere, was not tried: no Windows machine has run it, so it sits in the *cannot be run* section of `Audits/Blender Research/Dependencies/Hand tests — Linux.md`. What is settled is what a Mac does with such a string: it stores `C:/textures/tex.png`, treats it as absolute, and does not find the file.


**Observed — hand test entry 24, run by the Creator on 2026-08-05 on their own computer** (MacBook Pro, M1 Pro chip, macOS 26.5.2, Apple's own graphics), in Blender 5.0.1 with its window open. This is the half of the list that needs eyes on the screen; the whole list is at `Audits/Blender Research/Dependencies/Hand tests.md`. Because it was run on a Mac, it settles macOS and leaves Windows and Linux open.

**What was done.** The Output File Path was set to a template with a brace left open — `//out_{blend_name.png` — and the Creator hovered the field for its tooltip, then rendered.

**What came back.** Two screenshots per version, and the same thing in all four.

The field itself is filled **red**. The tooltip reads:

```
Output Path
Directory/name to save animations, # characters define the position and padding of frame numbers.
Value: //out_{blend_name.png
Path template error(s):
  - Invalid or incomplete template expression '{blend_name.png'.
Python: RenderSettings.filepath
bpy.data.scenes["Scene"].render.filepath
```

with the line `Path template error(s):` and the error under it drawn in red, and the two Python lines in grey.

On rendering, the Blender Render window opens and stays empty — a bare checkerboard, no image. The Info log carries an entry with a red error mark:

```
Parse errors in path '//out_{blend_name.png':
  - Invalid or incomplete template expression '{blend_name.png'.
```

and the same text appears on one line in the status bar at the bottom of the window:

```
Parse errors in path '//out_{blend_name.png': - Invalid or incomplete template expression '{blend_name.png'.
```

No floating error panel is visible over the render window in this version's screenshots. One is visible in the shots for 5.1 and 5.2; whether that is a difference or a matter of when the picture was taken is not reported.

**Both standing lines, brought up to date by this test.** *Driven from outside:* now settled on both sides. The terminal test recorded the same failure with no window, where both `render.frame_path()` and the render itself raised `RuntimeError: Error: Parse errors in path ... Invalid or incomplete template expression`, and no folder was made. Here the same words appear in three places on screen — the tooltip, the Info log and the status bar — and the render produces nothing. So a broken template stops the render either way, and says so either way. *Operating system:* macOS for this test. The same test was run on a rented Linux machine on 2026-08-06 and that result is recorded below; Windows stays untested.

**A note on the wording.** The template the Creator used, `//out_{blend_name.png`, is not the one the entry named. It is broken in the same way and produced the same class of message; the exact text above is what was on screen.


**Observed — run again on Linux, 2026-08-06.** The nine ways of writing a location were assigned, saved twice (once with path remapping off and once with it on) and read back again in this version on the rented Linux machine. **Every one behaved as it did on the Creator's Mac**, and the two saves gave the same result as each other:

```
NAME=absolute       STORED='…/e3/proj/assets/tex.png'   ABSPATH='…/e3/proj/assets/tex.png'   SIZE=(8, 8)
NAME=backslash-rel  STORED='//assets/tex.png'           ABSPATH='…/e3/proj/assets/tex.png'   SIZE=(8, 8)
NAME=dollar-brace   STORED='${SMOOZY_DEP_TEST}/tex.png' ABSPATH='${SMOOZY_DEP_TEST}/tex.png' SIZE=(0, 0)
NAME=dollar-var     STORED='$SMOOZY_DEP_TEST/tex.png'   ABSPATH='$SMOOZY_DEP_TEST/tex.png'   SIZE=(0, 0)
NAME=drive-letter   STORED='C:/textures/tex.png'        ABSPATH='C:/textures/tex.png'        SIZE=(0, 0)
NAME=no-such-mount  STORED='/Volumes/NoSuchShare/tex.png' ABSPATH='/Volumes/NoSuchShare/tex.png' SIZE=(0, 0)
NAME=relative       STORED='//assets/tex.png'           ABSPATH='…/e3/proj/assets/tex.png'   SIZE=(8, 8)
NAME=relative-up    STORED='//../outer/tex.png'         ABSPATH='…/e3/proj/../outer/tex.png' SIZE=(8, 8)
NAME=tilde          STORED='~/smoozy-path-test/tex.png' ABSPATH='~/smoozy-path-test/tex.png' SIZE=(0, 0)
```

The typed form for `backslash-rel` was `//assets\tex.png` and for `drive-letter` was `C:\textures\tex.png`. So on Linux too: backslashes become forward slashes on save, a home-directory shorthand is never expanded, an environment variable in either form is never expanded, and a drive letter is treated as an ordinary absolute path that finds nothing.

Templates behaved the same on both machines. `//templated/{blend_name}_frame_` produced the real file name and made the folder. The same braces in an image texture path were left alone. A broken template stopped both the path calculation and the render, made no folder, and gave word for word:

```
   frame_path ERR RuntimeError: Error: Parse errors in path '//broken/{blend_name_frame_':
- Invalid or incomplete template expression '{blend_name_frame_'.
```

**Which template variables this version really has, asked one at a time.** Each name was put into a render output path in turn and `frame_path` asked for. Nine were accepted: `blend_name` gave `out_0001.png`, `blend_dir` and `blend_dir_lib` each gave the file's own folder, `blend_name_lib` gave `out_0001.png`, `scene_name` gave `Scene_0001.png`, `camera_name` gave `Camera_0001.png`, `fps` gave `24.0_0001.png`, and `resolution_x` and `resolution_y` each gave `16_0001.png`. **Only `node_name` was refused** — `RuntimeError: Error: Parse errors in path '//tv/{node_name}_': - Unknown variable referenced in template expression '{node_name}'.` — which is the one the documentation restricts to a path property on a node rather than a render output path.

**One thing about the drive-lettered form that neither machine had noticed, and it happens on both.** Report Missing Files prints that path as `/c/textures/tex.png` — Blender rewrites `C:/` into `/c/` in its own warning — while the stored text and `bpy.path.abspath` both still read `C:/textures/tex.png`. The warning and the stored value disagree.

**With a window open on the Linux machine**, a broken template turns the output File Path field **solid red** with the text in white on the red, in 5.0, 5.1 and 5.2, and the terminal carries `operator | ERROR Parse errors in path … Invalid or incomplete template expression`. In 4.5 the Output panel sat below the visible part of the Properties editor, so that field was not photographed on Linux; the 4.5 picture is the Mac's. The tooltip needs a pointer resting on the field and was not photographed on Linux.

**Both standing lines, brought up to date by the Linux tests.** *Driven from outside:* every reading above was taken with Blender running with no window unless it says otherwise, so it holds outside the interface. *Operating system:* **now settled on two operating systems, macOS and Linux, and each is its own result** — what they agree on and where they differ is said above. **Windows is still untried: no Windows machine has run it.**

---

## 10. Are some of the things a file points at places it writes to rather than reads from, and where does such a write land?

**LABEL: documented.**

Yes. Many of them. Below is every outward pointer in the sources that is described as a place written to, with what the sources say about where the write lands.

### Rendered frames

> "File Path
>
> Choose the location to save rendered frames."

> "This setting expands Relative Paths
> where a // prefix represents the directory of the current blend-file."

— Manual, https://docs.blender.org/manual/en/5.0/render/output/properties/output.html, read 2026-07-30

Two options change what lands there rather than where:

> "Image Sequence – Overwrite Image Multi-Layer EXR
>
> Overwrite existing files when rendering."

> "Image Sequence – Overwrite Image Multi-Layer EXR
>
> Create empty placeholder frames while rendering."

(the manual prints the same heading twice; the API distinguishes them as `use_overwrite`, "Overwrite existing files while rendering", and `use_placeholder`, "Create empty placeholder files while rendering frames (similar to Unix 'touch')" — API, https://docs.blender.org/api/5.0/bpy.types.RenderSettings.html)

The same manual page is explicit that the destination may be a shared network location:

> "Primitive Render Farm
>
> An easy way to get multiple machines to share the rendering workload is to:
>
> Set up a shared directory over a network file system.
>
> Disable Overwrite, enable Placeholders in the Render Output panel.
>
> Start as many machines as you wish rendering to that directory."

— Manual, https://docs.blender.org/manual/en/5.0/render/output/properties/output.html, read 2026-07-30

### The compositor's File Output node

> "The File Output node writes one or more images to disk during rendering or compositing.
> Each connected input is saved to the specified directory and filename,
> with the frame number appended to create a sequence.
> For example: {Directory}/{File Name}{Socket Name}{frame number}.{extension}."

> "Directory
>
> The base output directory."

> "Output Paths
>
> … Displays the final file path and extension for each input/layer, based on current settings."

— Manual, https://docs.blender.org/manual/en/5.0/compositing/types/output/file_output.html, read 2026-07-30

### The render cache

> "Saving – Cache Result
>
> Saves the rendered view layers and their passes to a multi-layer OpenEXR image. …
>
> The image is stored in the Render Cache folder as specified in the
> File Paths Preferences."

— Manual, https://docs.blender.org/manual/en/5.0/render/output/properties/output.html, read 2026-07-30

So this write lands somewhere set in the application's preferences, not in the file: `render_cache_directory`, "Where to cache raw render results" (API, https://docs.blender.org/api/5.0/bpy.types.PreferencesFilePaths.html, read 2026-07-30).

### Physics caches and bakes

This is the clearest statement in the sources about where a write lands by default:

> "Disk Cache
>
> The cache of a baked simulation will be stored inside the blend-file when you save it.
> When Disk Cache is checked, Blender will save the cache separately to
> the drive in a folder named blendcache_[filename] alongside the blend-file.
> (The blend-file must be saved first.)"

The file names inside that folder are also documented:

> "Each cache can have a name. Double-click the cache entry to give it a name.
> If this name is given, any disk cache will be stored in files starting with that name.
> For example, a cache named MyCache will be stored in MyCache_xxxxxx_yy.bphys."

> "If the cache does not have a name (which is the default),
> the filename of the cache will depend on the object it is attached to,
> although this is not immediately obvious. For example, a cache on
> an object Cube will be stored in 43756265_xxxxxx_yy.bphys,
> where 43756265 is determined by the object name."

And a custom destination can be given instead:

> "External
>
> Allows you to read the cache from a drive using a user-specified file path."
>
> "Path
>
> Select the directory path to the cache files."

— Manual, https://docs.blender.org/manual/en/5.0/physics/baking.html, read 2026-07-30

The manual also records a failure mode for this write:

> "If you are not allowed to write to the required subdirectory caching will not take place,
> e.g. if your blend-file path is very long and your operating system
> has a limit on the path length that is supported."

— Manual, https://docs.blender.org/manual/en/5.0/physics/particles/emitter/cache.html, read 2026-07-30

### Fluid caches

> "Cache Directory
>
> Directory to store baked simulation files in. Inside this directory each simulation type
> (i.e. mesh, particles, noise) will have its own directory containing the simulation data."

and, separating fluids from the rest:

> "Fluid simulations use their own cache. All other physics simulations make use of
> the General Baking operators."

— Manual, https://docs.blender.org/manual/en/5.0/physics/fluid/type/domain/cache.html, read 2026-07-30

### Geometry-node bakes

> "Bake
>
> Preforms all necessary geometry calculations and saves the data on disk or packed into the .blend file."

> "Bake Target
>
> Specifies where the baked data should be stored.
>
> Inherit from Modifier: The setting is copied from the modifier that contains the bake node. This is the default.
>
> Packed: The baked data is packed into the .blend file. So no separate file is necessary.
>
> Disk: The baked data is stored in a separate directory on disk."

> "Bake Path
>
> Location on disk where the baked data is stored.
> Note, this path is also used for simulation zones."

— Manual, https://docs.blender.org/manual/en/5.0/modeling/geometry_nodes/geometry/operations/bake.html, read 2026-07-30

### Ocean modifier bakes

> "Cache Path
>
> Folder to store the baked EXR files in.
> The sequences will be in the form disp_####.exr, normal_####.exr,
> and foam_####.exr, where #### is the four digit frame number.
> If the cache path folder does not exist, it will be created."

— Manual, https://docs.blender.org/manual/en/5.0/modeling/modifiers/physics/ocean.html, read 2026-07-30

That last sentence is the only place in these sources where I found an explicit statement that Blender creates a missing output directory.

### Proxies, dynamic paint, external displacement, texture bakes, collection export

From the API, each described as a place written to (all read 2026-07-30):

- `MovieClipProxy.directory` and `StripProxy.directory` — "Location to store the proxy files"
- `SequenceEditor.proxy_dir`
- `DynamicPaintSurface.image_output_path` — "Directory to save the textures"
- `MultiresModifier.filepath` and `MeshCacheModifier.filepath` — "Path to external displacements file"
- `BakeSettings.filepath` — "Image filepath to use when saving externally"
- `CollectionExport.filepath` — "The file path used for exporting"

— API, one page per type under https://docs.blender.org/api/5.0/

### Unpacking

Unpacking is a write back out to disk, and the manual gives four choices of where it lands:

> "Use files in current directory (create when necessary)
>
> Unpacks all files in the same directory // as the blend-file,
> grouping them in proper folders (like ''textures'' for instance).
> However, if the final file exists already, it will use that file, instead of unpacking it."

> "Write files to current directory (overwrite existing files)"
> "Use files in original location (create when necessary)"
> "Write files to original location (overwrite existing files)"

and for linked libraries:

> "Unpack Linked Libraries
>
> Export previously packed data-blocks back to external blend-files. Existing blend-files are
> overwritten."

— Manual, https://docs.blender.org/manual/en/5.0/files/blend/packed_data.html and https://docs.blender.org/manual/en/5.0/interface/window_system/topbar.html, read 2026-07-30

### A write that lands beside an input file — **observed**

Cycles compiles an external OSL source file and writes the compiled result next to it:

> ```
>     elif script_ext == ".osl":
>         # compile .osl file
>         ok, oso_path = osl_compile(script_path, report)
>         oso_file_remove = True
>
>         if ok:
>             # copy .oso from temporary path to .osl directory
>             dst_path = script_path_noext + ".oso"
>             try:
>                 shutil.copy2(oso_path, dst_path)
>             except:
>                 report({'ERROR'}, rpt_("Failed to write .oso file next to external .osl file at {:s}").format(dst_path))
> ```

— Cycles add-on file `osl.py`, function `update_external_script()`, lines 203–214, read 2026-07-30

Label: **observed** for what the file says — the copy target is the source path with its extension changed, and the error text says "next to external .osl file". It would be **inferred** to say a render always triggers this; the file shows the function, not when it runs.

The same file also shows a temporary file being written during compilation:

> ```
> def osl_compile(input_path, report):
>     """compile .osl file with given filepath to temporary .oso file"""
>     import tempfile
>     output_file = tempfile.NamedTemporaryFile(mode='w', suffix=".oso", delete=False)
> ```

— Cycles add-on file `osl.py`, lines 13–16, read 2026-07-30

### The temporary directory

Not a pointer held in the file, but it is where several of these writes are said to land:

> "The temporary directory is used to store various files at run-time
> (including render layers, physics cache, copy-paste buffer and crash logs).
>
> The temporary directory is selected based on the following priority:
>
> User Preference (see File Paths).
>
> Environment variables (TEMP on Windows, TMP & TMP_DIR on other platforms).
>
> The /tmp/ directory."

— Manual, https://docs.blender.org/manual/en/5.0/advanced/blender_directory_layout.html, read 2026-07-30

The command-line page names these differently — "TEMP: Store temporary files here (MS-Windows)." and "TMPDIR: Store temporary files here (UNIX Systems). The path must reference an existing directory or it will be ignored." (Manual, https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html, read 2026-07-30). The two pages disagree on the non-Windows variable name. I report both as written and do not choose between them.

### Where a write lands, in general

The general rule is the relative-path rule, which applies to writes as much as reads: a leading `//` means the directory of the blend-file that holds the property, and for a property on library-linked data it means the directory of the library file rather than the open one (API, https://docs.blender.org/api/5.0/info_gotchas_file_paths_and_encoding.html, read 2026-07-30). Every write destination listed above except the render cache and the temporary directory is marked in the API with "blend relative // prefix supported".

### Where I looked

- Searched the whole manual for: "written to", "save", "bake", "cache", "output", "Disk Cache", "blendcache", "Cache Path", "Cache Directory", "overwrite", "placeholder", "will be created".
- Opened: `render/output/properties/output.txt`, `compositing/types/output/file_output.txt`, `physics/baking.txt`, `physics/particles/emitter/cache.txt`, `physics/fluid/type/domain/cache.txt`, `modeling/geometry_nodes/geometry/operations/bake.txt`, `modeling/modifiers/physics/ocean.txt`, `files/blend/packed_data.txt`, `interface/window_system/topbar.txt`, `advanced/blender_directory_layout.txt`, `advanced/command_line/arguments.txt`.
- In the API, went through every property I had collected in question 1 and kept the ones whose own description says "save", "store", "write", "output" or "bake".
- Read `cycles_5.0/osl.py` and `cycles_5.0/operators.py` in full. `operators.py` contains a denoise operator with "Input Filepath" and "Output Filepath" properties, but those are operator arguments, not something the file points at, so I left them out.
- Searched "blendcache" across manual, API and release notes: exactly one hit, quoted above.

**Not covered.** This does not settle what happens when the destination does not exist, except for the ocean modifier, which is the only case the sources address. It does not settle what happens when the destination is not writable, except for the physics-cache remark about path length. It does not settle whether relative write paths are resolved at the moment the render starts or per frame. It does not settle what the render output path defaults to in a fresh file — the API gives `RenderSettings.filepath` a default of the empty string, and the preference `render_output_directory` ("The default directory for rendering output, for new scenes") also defaults to the empty string, so the sources do not name a concrete default location. It does not settle whether packing a resource stops the corresponding write.

**Outside the interface.** The command-line page documents render output being set and written from outside the window, including the relative prefix and templates, and warns that argument order decides whether the setting takes effect at all (quoted under question 5). The render-farm hint above describes several machines each writing into one shared directory, which is inherently outside-the-window use. For the rest — bakes, caches, proxies, unpacking — the sources describe them as panel operations and say nothing about whether the destination behaves differently when Blender is driven from outside. That is a real gap for anything that depends on it.

**Operating system.** Partly. The temporary directory is explicitly per-system (TEMP on Windows, a different variable elsewhere, falling back to `/tmp/`), and the user and system directories are listed per system on the same page. The physics-cache page names an operating-system path-length limit as a reason a cache write can silently fail. Beyond those, the sources do not say whether write behaviour differs by operating system.


**Observed — hand test entries 13, 18 and 20, run 2026-08-05 on the Creator's own computer** (MacBook Pro, M1 Pro chip, macOS 26.5.2, Apple's own graphics), with Blender 5.0.1 started from a terminal with no window, using a throwaway settings folder set through `BLENDER_USER_RESOURCES`. The full list of tests is at `Audits/Blender Research/Dependencies/Hand tests.md`.

**What was done, part one — where a render lands when the file has never been saved.** A factory-fresh Blender with no file open, 16 by 16 output, Cycles, one sample, frames 1 to 2. The factory default `render.filepath` was printed, then it was set to `//never_saved_out/frame_`, then `bpy.path.abspath` of it and `render.frame_path(frame=1)` were printed, then `bpy.ops.render.render(animation=True)` was run.

**What came back**, the same in all four versions:

```
A. file never saved. bpy.data.filepath = ''
   factory default render.filepath = '/tmp/'
   set render.filepath = '//never_saved_out/frame_'
   bpy.path.abspath now  = 'never_saved_out/frame_'
   frame_path(frame=1)   = 'never_saved_out/frame_0001.png'
   render(animation=True) finished
   contents of the current working directory that look like output: ['never_saved_out']
```

In an unsaved file the `//` prefix resolves to nothing at all, leaving a plain relative path, and the frames land in the folder the terminal was started from.

**Part two — a folder that does not exist.** With the file saved and `render.filepath` set to `//does_not_exist_yet/frame_`, rendering the two-frame animation gave, in all four versions:

```
   render.filepath = '//does_not_exist_yet/frame_' -> '<path>/does_not_exist_yet/frame_'
   folder created? True   contents: ['frame_0001.png', 'frame_0002.png']
```

**Part three — the compiled OSL file.** Two folders were made, one writable and one set to `chmod 555`, each holding the same `simple_col.osl`. A file per folder was built with a Script node in External mode pointing at it, Cycles on CPU, `scene.cycles.shading_system = True`. Calling `bpy.ops.node.shader_script_update()` failed in all four versions and both folders with `RuntimeError: Operator bpy.ops.node.shader_script_update.poll() failed, context is incorrect`, so the compile could only be reached by rendering.

In this version no `.oso` appeared in either folder after the render, and nothing was printed about the script in either case; the render finished and wrote a frame each time. In 4.5.12 the same test did write `simple_col.oso` beside the source in the writable folder, and in the read-only folder printed `ERROR: error: Could not open "<path>/readonly/simple_col.oso"` and `Cycles shader graph: failed to read file <path>/readonly/simple_col.oso`.

**Part four — what the compositor's File Output node calls its folder.** The properties of `bpy.types.CompositorNodeOutputFile` that are not already on `bpy.types.Node`, in this version:

```
['file_output_items', 'active_item_index', 'directory', 'file_name', 'format', 'save_as_render']
```

In 4.5 the folder is `base_path`; in 5.0, 5.1 and 5.2 it is `directory`, with a separate `file_name`. Setting `base_path` in 5.x raises `AttributeError: 'CompositorNodeOutputFile' object has no attribute 'base_path'`, and `Scene.node_tree` does not exist there either — the compositor tree is reached through `Scene.compositing_node_group`, which is `None` until something is put there.

Whether that node creates a folder that does not exist is **not** settled for this version. Building a compositor node group by hand from a script and rendering produced no folder and no files, but a tree built that way may simply not be wired into the render, so the result carries nothing. It is entry 26 on the hand-test list and is the Creator's to answer through the window.

**Both standing lines, brought up to date by this test.** *Driven from outside:* settled for the render output path, for folder creation and for the unsaved-file case — all of it was done with no window. One thing is settled the other way: the operator that updates an external shader script cannot be called with no window, and says so. *Operating system:* macOS for this test. The same test was run on a rented Linux machine on 2026-08-06 and that result is recorded below; Windows stays untested.


**Observed — hand test entry 26, run by the Creator on 2026-08-05 on their own computer** (MacBook Pro, M1 Pro chip, macOS 26.5.2, Apple's own graphics), in Blender 5.0.1 with its window open. This is the half of the list that needs eyes on the screen; the whole list is at `Audits/Blender Research/Dependencies/Hand tests.md`. Because it was run on a Mac, it settles macOS and leaves Windows and Linux open.

**What was done.** A plain two-frame file was opened, Use Nodes was ticked in the Compositing workspace, a File Output node was added and connected to the Render Layers node's Image output, and its folder was set to `//fileoutput_new_folder/` — a folder that did not exist. Then Render ‣ Render Animation.

**What came back**, in the Creator's words: "It all renders. You can check out the files yourself." The folder beside the blend-file was then read from a terminal:

```
$ ls "/Users/artist/Documents/Smoozy hand tests/Dependencies/5.0/26/fileoutput_new_folder/"
file_name0001.exr
file_name0002.exr
```

**So the node does create a folder that does not exist, in this version**, and it wrote two files into it: `file_name0001.exr` and `file_name0002.exr`.

The names differ between versions, and all four were run: 4.5 wrote `Image0001.png` and `Image0002.png`; 5.0 and 5.1 wrote `file_name0001.exr` and `file_name0002.exr`; 5.2 wrote `file output0001.exr` and `file output0002.exr`, which is the blend-file's own name. The file type differs too — PNG in 4.5, OpenEXR in the three 5.x versions — with nothing about the format changed by hand in any of them.

**This closes what the terminal could not.** Building a compositor node group from a script and rendering wrote nothing in 5.0, 5.1 and 5.2, which was recorded as carrying no answer because a tree built that way may not be wired into the render. Done through the window, it writes.

**Both standing lines, brought up to date by this test.** *Driven from outside:* not settled. This was done with the window open, and the same thing attempted from a script with no window produced no files — but for a reason that was not established, so nothing here says what a background render does with a File Output node that a person set up. *Operating system:* macOS for this test. The same test was run on a rented Linux machine on 2026-08-06 and that result is recorded below; Windows stays untested.


**Observed — run again on Linux, 2026-08-06.** Where a render lands was tested again in this version on the rented Linux machine, and came back the same as on the Creator's Mac, including the factory default:

```
A. file never saved. bpy.data.filepath = ''
   factory default render.filepath = '/tmp/'
   set render.filepath = '//never_saved_out/frame_'
   bpy.path.abspath now  = 'never_saved_out/frame_'
   frame_path(frame=1)   = 'never_saved_out/frame_0001.png'
   contents of the working directory: ['never_saved_out']
   inside never_saved_out: ['frame_0001.png', 'frame_0002.png']

B. file saved, output path names a folder that does not exist
   folder created? True   contents: ['frame_0001.png', 'frame_0002.png']
```

The operator that updates an external shader script refuses with no window here too: `RuntimeError: Operator bpy.ops.node.shader_script_update.poll() failed, context is incorrect`.

**The compiled OSL file.** No `.oso` appeared in either a writable or a read-only folder, and nothing at all was printed about the script; the render finished and wrote a frame in every case. The same as on the Mac. The read-only folder was run both as plain root and with the two file-permission capabilities dropped, because root walks straight through file permissions; neither run produced a `.oso`.

**And this closes what the Mac could not settle: whether a File Output node writes when the file is rendered with no window, and why a script-built one wrote nothing.** The Mac test found that a node group built from a script wrote nothing in this version. The cause, found here: **the File Output node arrives with no output items and therefore no usable input socket**, so the link from Render Layers went nowhere — the socket it connected to has no name at all. Adding one output item first with `fo.file_output_items.new("RGBA", "MyPass")`, linking to that, **saving**, and then rendering in a separate run with no window gave:

```
   directory = '//fileoutput_new_folder/'  file_name = 'file_name'
   Saved: '/root/…/e26/fileoutput_new_folder/file_name0001.exr'
   Saved: '/root/…/e26/fileoutput_new_folder/file_name0002.exr'
```

It created the folder that was not there. The names and the file type are exactly what the Creator saw through the window on the Mac. So the node does write with no window; what the Mac saw was a node with nothing connected to it.

The accepted socket types were read back from the refusal `TypeError: NodeCompositorFileOutputItems.new(): error with argument 1, "socket_type" - enum "COLOR" not found in ('FLOAT', 'INT', 'BOOLEAN', 'VECTOR', 'RGBA', …)`.

**One thing about where a write lands that only the crossing could show.** A file whose render output path is a full path written on a Mac does not make a Linux render refuse. It creates the whole Mac-shaped folder tree at the root of the Linux disk and writes the frames into it, saying nothing. That run was made as the root user, who owns the root of that disk, so **what happens for a render user who cannot write there is not established.**

**Both standing lines, brought up to date by the Linux tests.** *Driven from outside:* every reading above was taken with Blender running with no window unless it says otherwise, so it holds outside the interface. *Operating system:* **now settled on two operating systems, macOS and Linux, and each is its own result** — what they agree on and where they differ is said above. **Windows is still untried: no Windows machine has run it.**

---

## 11. Can a reference point at a named thing inside another file, and what happens when that thing is renamed or is gone?

**LABEL: documented.**

### Yes, and the name is what is stored

Library linking is by data-block, and data-blocks are identified by name:

> "Their names are unique within a blend-file, for a given type."

> "Data-blocks names are unique within their namespace. A data-block namespace is defined by its type, and the blendfile
> it is stored in."

— Manual, https://docs.blender.org/manual/en/5.0/files/data_blocks.html, read 2026-07-30

The API states outright that the stored reference is a name plus a type marker plus the library's path:

> "LibraryWeakReference … Read-only external reference to a linked data-block and its library file"
>
> "filepath — Path to the library .blend file"
>
> "id_name — Full ID name in the library .blend file (including the two leading 'id type' chars)"

— API, https://docs.blender.org/api/5.0/bpy.types.LibraryWeakReference.html, read 2026-07-30

The Python interface for linking works in names, which is the same thing seen from the other side:

> "Returns a context manager which exposes 2 library objects on entering.
> Each object has attributes matching bpy.data which are lists of strings to be linked."
>
> ```
> # Load a single scene we know the name of.
> with bpy.data.libraries.load(filepath) as (data_src, data_dst):
>     data_dst.scenes = ["Scene"]
> ```

— API, https://docs.blender.org/api/5.0/bpy.types.BlendDataLibraries.html, read 2026-07-30

The geometry-nodes modifier stores the same shape of reference for data-blocks it needs:

> "id_name — Name that is mapped to the referenced data-block"
>
> "lib_name — Used when the data block is not local to the current .blend file but is linked from some library"

— API, https://docs.blender.org/api/5.0/bpy.types.NodesModifierDataBlock.html, read 2026-07-30

### Other kinds of named thing inside another file

Not only data-blocks. The sources document at least these:

- A named object inside an Alembic archive. "object_path — Path to the object in the Alembic archive used to lookup geometric data" (API, https://docs.blender.org/api/5.0/bpy.types.MeshSequenceCacheModifier.html); "object_path — Path to the object in the Alembic archive used to lookup the transform matrix" (API, https://docs.blender.org/api/5.0/bpy.types.TransformCacheConstraint.html); and the type "CacheObjectPath — Path of an object inside of an Alembic archive" (API, https://docs.blender.org/api/5.0/bpy.types.CacheObjectPath.html). All read 2026-07-30.
- A named cache, by file-name prefix. "The cache name in Caches List and the Index has to exactly match the external cache files name in order to work. The cache files name format is name_frame_index.bphys." (Manual, https://docs.blender.org/manual/en/5.0/physics/baking.html, read 2026-07-30). This one is a name matched against a file name rather than against contents.

### What happens when it is gone

The library file missing:

> "Broken Libraries
>
> If Blender cannot find a library while loading a blend-file,
> it will create placeholder data-blocks to replace missing linked ones.
> That way, references to the missing data are not lost, and by relocating the missing library,
> the lost data can be automatically restored."

— Manual, https://docs.blender.org/manual/en/5.0/files/linked_libraries/link_append.html, read 2026-07-30

The API names the flag on such a placeholder:

> "is_missing
>
> This data-block is a place-holder for missing linked data (i.e. it is [an override of] a linked data that could not be found anymore)"

— API, https://docs.blender.org/api/5.0/bpy.types.ID.html, read 2026-07-30

and names the option that produces them:

> "USE_PLACEHOLDERS
> Generate a placeholder (empty ID) if not found in any library files."

— API, https://docs.blender.org/api/5.0/bpy.types.BlendImportContext.html, read 2026-07-30

Note the wording there: "if not found in any library files". That covers the case where the file is present but the named thing inside it is not.

### What happens when it is renamed

The manual treats a rename as producing exactly the same broken state, and names the repair:

> "Relocate Linked ID
>
> … A right-click a linked ID will show the Relocate option in the ID Data sub-menu. It allows to
> relocate a directly linked ID by another one, from the same library or from a different one.
> This can be used to either fix a broken linked data-block (e.g. because the ID was renamed),
> or to switch to a variation of the same data."

with a limit that matters:

> "Note
>
> This operation is only available on data-blocks that are directly linked, and are not
> a dependency of any other linked data."

— Manual, https://docs.blender.org/manual/en/5.0/files/linked_libraries/link_append.html, read 2026-07-30

The corresponding operator is `bpy.ops.outliner.id_linked_relocate()` — "Replace the active linked ID (and its dependencies if any) by another one, from the same or a different library" (API, https://docs.blender.org/api/5.0/bpy.ops.outliner.html, read 2026-07-30).

For a renamed or moved *file* rather than a renamed data-block:

> "Relocate Library
>
> … It allows to replace it by a different file.
> This can be used to either fix a broken linked library (e.g. because the file was moved
> or renamed), or to switch to a variation of the same data in a different file."

— Manual, https://docs.blender.org/manual/en/5.0/files/linked_libraries/link_append.html, read 2026-07-30

### One thing that makes renames more likely to bite in 5.0

Names are stored with a numeric suffix when they collide, and the manual sets out the rules, including that renaming one data-block can rename another:

> "If the original root name is the same as in the new requested name, the renamed data-block gets the requested name,
> and the conflicting of data-block is renamed accordingly.
>
> E.g. assuming that there are three objects named Sphere, Cube and Cube.001, renaming Cube.001 to Cube
> will rename the data-block to Cube, and the other data-block to Cube.001."

— Manual, https://docs.blender.org/manual/en/5.0/files/data_blocks.html, read 2026-07-30

### Where I looked

- Searched the whole manual for: "missing", "Missing", "placeholder", "broken", "Broken Libraries", "renamed", "Relocate", "name collision".
- Opened: `files/linked_libraries/link_append.txt`, `files/linked_libraries/library_overrides.txt`, `files/data_blocks.txt`, `interface/window_system/topbar.txt`, `editors/outliner/interface.txt`, `editors/outliner/editing.txt`, `troubleshooting/startup.txt`.
- In the API, opened `bpy.types.ID.txt`, `bpy.types.Library.txt`, `bpy.types.LibraryWeakReference.txt`, `bpy.types.BlendDataLibraries.txt`, `bpy.types.BlendImportContext.txt`, `bpy.types.BlendImportContextItem.txt`, `bpy.types.NodesModifierDataBlock.txt`, `bpy.types.MeshSequenceCacheModifier.txt`, `bpy.types.TransformCacheConstraint.txt`, `bpy.types.CacheObjectPath.txt`, `bpy.ops.outliner.txt`.
- Searched the API for "is_missing", "placeholder", "relocate", "id_remap", "weak reference".
- Checked whether named things inside multi-layer images are stored by name: they are not. `ImageUser.multilayer_layer`, `.multilayer_pass` and `.multilayer_view` are all documented as "int in [0, 32767], default 0, (readonly)" (API, https://docs.blender.org/api/5.0/bpy.types.ImageUser.html, read 2026-07-30). So I make no claim about layer names.
- Searched the release notes for "missing", "placeholder", "rename". The only relevant hit is the longer-names change in `core`.

**Not covered.** The sources say a placeholder is created and say it can be repaired by relocating; they do not say what a render does with a placeholder in it, nor whether a render reports it. They do not say whether a rename inside a library is detected at all — only that relocating the linked ID is the fix. They do not say whether the placeholder survives a save and reload, nor what a placeholder looks like in the file. They do not say anything about renaming a named object inside an Alembic archive, or a named cache, beyond the exact-match requirement quoted. They do not say whether matching is case-sensitive.

**Outside the interface.** Placeholder creation happens "while loading a blend-file", which is not an interface action, so it should hold for a file opened from outside the window; and `USE_PLACEHOLDERS` is an option on the import context exposed to Python handlers, not a menu item. The repairs, though, are documented only as interface operations — Relocate Library and Relocate Linked ID are described as Outliner right-click menu items. The matching operators exist (`bpy.ops.outliner.lib_relocate()`, `bpy.ops.outliner.id_linked_relocate()`), but both are Outliner operators and the sources do not say whether they can be driven without an Outliner present. That is a gap worth knowing about.

**Operating system.** The sources do not say. Nothing in the pages I read attaches an operating system to any of this. The one indirect remark is that longer data-block names may run into the Windows file-name limit "when they are used as part of a generated file name" (Release notes, https://developer.blender.org/docs/release_notes/5.0/core/, read 2026-07-30), which touches names but not name-based lookup inside a library.


**Observed — hand test entries 7 and 8, run 2026-08-05 on the Creator's own computer** (MacBook Pro, M1 Pro chip, macOS 26.5.2, Apple's own graphics), with Blender 5.0.1 started from a terminal with no window, using a throwaway settings folder set through `BLENDER_USER_RESOURCES`. The full list of tests is at `Audits/Blender Research/Dependencies/Hand tests.md`.

**What was done.** `lib.blend` was built holding a collection `PropCol` with one object `Prop`; `master.blend` linked that collection with `bpy.ops.wm.link(..., instance_collections=True)` and added a camera and a sun. Three things were then tried in turn, each time reopening `master.blend` with no window and printing the libraries, the collections with their `is_missing`, the objects, and the number of evaluated instances: the whole `lib.blend` renamed out of the way; `lib.blend` put back and the collection inside it renamed to `PropCol_renamed` and saved; the name restored and the collection removed with `bpy.data.collections.remove(...)` and saved.

**What came back.** Identical in all four versions; only the shape of the log line differs (4.5 writes `Info:` and `Warning:` prefixes, 5.x writes a `reports          |` prefix).

With the whole library file gone:

```
Warning: Unable to open '<path>/lib.blend': No such file or directory
Info: Cannot find lib '<path>/lib.blend'
Info: LIB: Collection: 'PropCol' missing from '<path>/lib.blend', parent '<direct>'
Warning: 1 libraries and 1 linked data-blocks are missing (including 0 ObjectData), please check the Info and Outliner editors for details
```

**With the library present and the named item renamed inside it — and, separately, with the named item removed — the output was exactly the same in both cases:**

```
Info: LIB: Collection: 'PropCol' missing from '<path>/lib.blend', parent '<direct>'
Warning: 0 libraries and 1 linked data-blocks are missing (including 0 ObjectData), please check the Info and Outliner editors for details
```

so a rename does break the reference, it is reported, and nothing in what is reported tells a rename from a deletion. Note the count: `0 libraries` here against `1 libraries` when the whole file was gone.

In all three cases what is left in the file is a placeholder that keeps the name and the library, and holds nothing:

```
   name='PropCol'  library='//lib.blend' is_missing=True is_library_indirect=False objects=[]
```

Evaluated instances dropped from 4 to 3, and a Cycles render still finished and wrote a frame.

**Both standing lines, brought up to date by this test.** *Driven from outside:* settled. All of this happened with no window, and every message above was printed to the terminal — including the one that tells the reader to look in the Info and Outliner editors, which are not there. *Operating system:* macOS for this test. The same test was run on a rented Linux machine on 2026-08-06 and that result is recorded below; Windows stays untested.

**Still open.** Whether the Relocate repair is offered for a directly linked item and for one that arrived as a dependency is entry 23 on the hand-test list and is the Creator's to answer.


**Observed — hand test entry 23, run by the Creator on 2026-08-05 on their own computer** (MacBook Pro, M1 Pro chip, macOS 26.5.2, Apple's own graphics), in Blender 5.0.1 with its window open. This is the half of the list that needs eyes on the screen; the whole list is at `Audits/Blender Research/Dependencies/Hand tests.md`. Because it was run on a Mac, it settles macOS and leaves Windows and Linux open.

**What was done.** A file prepared for this test was opened. It links a collection called `MiddleCollection` from `middle.blend`, which itself links one called `DeepCollection` from `deep.blend` — so the file holds one thing that was linked directly and one that came along behind it. In an Outliner set to Blender File display mode, the Creator right-clicked each in turn and opened the ID Data submenu.

**What came back.** Screenshots per version. In all four versions the Outliner lists both `//middle.blend` and `//deep.blend` as libraries, with `MiddleCollection` under the first and `DeepCollection` under the second.

The ID Data submenu holds the same entries for both, in this order:

```
Unlink
Make Local
Delete
Remap Users
Copy
Paste
Add Fake User
Clear Fake User
Rename
Select Linked
Relocate
```

**Relocate is present for the indirectly linked collection as well as the directly linked one, and in none of the screenshots does it look greyed out** — its text is the same brightness as Unlink and Make Local above it. For comparison, three entries in the parent menu are visibly greyed in the same shots: Mark as Asset, Clear Asset and Clear Asset (Set Fake User).

**What the answer does not cover.** The screenshots show what the menu offers. Whether the entry does anything when it is clicked on an indirectly linked item was not tried and is not reported.

**Both standing lines, brought up to date by this test.** *Driven from outside:* not settled by this. What was looked at is a menu, which only exists with a window; the operator behind it, `bpy.ops.outliner.id_linked_relocate`, exists in all four versions as recorded under question 1, but what it does with no window was not tried. *Operating system:* macOS for this test. The same test was run on a rented Linux machine on 2026-08-06 and that result is recorded below; Windows stays untested.


**Observed — run again on Linux, 2026-08-06.** All three cases were built and read again in this version on the rented Linux machine, and the output matched the Creator's Mac word for word, allowing for the Linux paths. With the whole library file renamed out of the way:

```
Unable to open '/root/…/e7/lib.blend': No such file or directory
Cannot find lib '/root/…/e7/lib.blend'
LIB: Collection: 'PropCol' missing from '/root/…/e7/lib.blend', parent '<direct>'
1 libraries and 1 linked data-blocks are missing (including 0 ObjectData), please check the Info and Outliner editors for details
```

With the library present and the collection inside it **renamed**, and separately with it **removed**, the output was exactly the same in both cases:

```
LIB: Collection: 'PropCol' missing from '/root/…/e7/lib.blend', parent '<direct>'
0 libraries and 1 linked data-blocks are missing (including 0 ObjectData), please check the Info and Outliner editors for details
```

In every case what is left is a placeholder that keeps the name and the library and holds nothing:

```
   name='PropCol'  library='//lib.blend' is_missing=True is_library_indirect=False objects=[]
```

The render still finished and wrote a frame. **So a rename breaks the reference exactly as a deletion does, and nothing in what is reported tells the two apart, on Linux as on the Mac.**

**Relocate, with a window open on the Linux machine.** The operator's own test said it may run for both the directly linked collection and the one that arrived behind it — `id_linked_relocate.poll() = True` for `MiddleCollection` with `is_library_indirect=False` and for `DeepCollection` with `is_library_indirect=True`. That agrees with the Creator's screenshots showing the menu entry not greyed out in either case. **Whether it does anything when used on an indirectly linked item is still not settled**: calling it returned `{'PASS_THROUGH'}`, but the operator reads the Outliner's own selection, which cannot be set from a script, so neither result can be pinned to the item that was meant.

**Both standing lines, brought up to date by the Linux tests.** *Driven from outside:* every reading above was taken with Blender running with no window unless it says otherwise, so it holds outside the interface. *Operating system:* **now settled on two operating systems, macOS and Linux, and each is its own result** — what they agree on and where they differ is said above. **Windows is still untried: no Windows machine has run it.**

---

## 13. Does a file pointing at another `.blend` file bring that file's own references with it, how deep does that go, and can they be reached the same way?

**LABEL: documented**, with the "how deep" part **inferred**.

### Yes, dependencies come with it

The sources never say "linking pulls dependencies" in one sentence, but they say it repeatedly from the side of the options that undo it:

> "Localize All
>
> Also copy all indirectly linked data, instead of maintaining the links."

— Manual, https://docs.blender.org/manual/en/5.0/files/linked_libraries/link_append.html, read 2026-07-30

> "use_recursive (boolean, (optional)) – Localize All, Localize all appended data, including those indirectly linked from other libraries"

— API, https://docs.blender.org/api/5.0/bpy.ops.wm.html, read 2026-07-30

> "recursive (bool) – If True, also make indirect dependencies of appended libraries local."

— API, https://docs.blender.org/api/5.0/bpy.types.BlendDataLibraries.html, read 2026-07-30

> "APPEND_RECURSIVE
> Append (make local) also indirect dependencies of appended IDs coming from other libraries. NOTE: All IDs (including indirectly linked ones) coming from the same initial library are always made local."

— API, https://docs.blender.org/api/5.0/bpy.types.BlendImportContext.html, read 2026-07-30

> "INDIRECT_USAGE
> That item was added for an indirectly imported ID, as a dependency of another data-block."

— API, https://docs.blender.org/api/5.0/bpy.types.BlendImportContextItem.html, read 2026-07-30

And a flag on every data-block records which side it is on:

> "is_library_indirect
>
> Is this ID block linked indirectly"

— API, https://docs.blender.org/api/5.0/bpy.types.ID.html, read 2026-07-30

Writing a library file behaves the same way:

> "Note
>
> Indirectly referenced data-blocks will be expanded and written too."

— API, https://docs.blender.org/api/5.0/bpy.types.BlendDataLibraries.html (`write`), read 2026-07-30

### How deep

The one direct statement about depth in the sources is about dependency trees in general, not a stated limit:

> "These relationships can be represented as a tree, with a root data-block 'linking-in' all its
> dependencies, recursively."

— Manual, https://docs.blender.org/manual/en/5.0/files/linked_libraries/library_overrides.html, read 2026-07-30

The strongest structural evidence is that a library data-block itself has a parent library:

> "parent
>
> Type: Library, (readonly)"

— API, https://docs.blender.org/api/5.0/bpy.types.Library.html, read 2026-07-30

**inferred:** because a library's `parent` is itself a library, libraries can form a chain of more than two links, so a file's dependencies can be more than one file deep. The step a reader has to supply is that a chain of parents is possible at all — the API states only that the property exists and its type, not that chains longer than one occur or how long they may get. I could not find any stated depth limit anywhere in these sources, and my searches for one are recorded below.

There is one documented limit on the *shape* rather than the depth:

> "Circular Dependencies
>
> In general, dependencies should not go in both directions.
> Attempting to link or append data which links back to the current file will likely result in missing links."

— Manual, https://docs.blender.org/manual/en/5.0/files/linked_libraries/link_append.html, read 2026-07-30

### Can they be reached the same way? No, not in three respects

**First — their file paths resolve against a different file.** This is the most consequential difference in these sources:

> "When using Blender data from linked libraries there is an unfortunate complication
> since the path will be relative to the library rather than the open blend-file.
> When the data block may be from an external blend-file pass the library argument from the bpy.types.ID.
>
> ```
> >>> bpy.path.abspath(image.filepath, library=image.library)
> ```"

— API, https://docs.blender.org/api/5.0/info_gotchas_file_paths_and_encoding.html, read 2026-07-30

The path function itself confirms this:

> "library (bpy.types.Library) – The library this path is from. This is only included for
> convenience, when the library is not None its path replaces start."

— API, https://docs.blender.org/api/5.0/bpy.path.html, read 2026-07-30

The path template variables added in 5.0 exist for the same reason:

> "blend_name_lib:
>
> Like blend_name, except that for path properties on
> library-linked data-blocks it will be the name of the library blend file
> that the data-block comes from.
>
> blend_dir_lib:
>
> Like blend_dir, except that for path properties on
> library-linked data-blocks it will be the path to the library blend file
> that the data-block comes from."

— Manual, https://docs.blender.org/manual/en/5.0/files/file_paths.html, read 2026-07-30

**Second — indirectly linked data cannot be repaired the same way.** Quoted in full under question 11:

> "This operation is only available on data-blocks that are directly linked, and are not
> a dependency of any other linked data."

— Manual, https://docs.blender.org/manual/en/5.0/files/linked_libraries/link_append.html, read 2026-07-30

**Third — some things do not come along at all.**

> "Scene-Level Settings
>
> Scene-level settings such as the Rigid Body World will not
> be copied when linking objects. As an alternative, you can link the
> entire scene and use it as a Background Scene."

— Manual, https://docs.blender.org/manual/en/5.0/files/linked_libraries/link_append.html, read 2026-07-30

And a physics cache is shared or not depending on a switch:

> "Use Library Path
>
> Share the disk cache when the physics object is
> linked into another blend-file.
> When this option is enabled, linked versions of the object will reference the same disk cache.
> Otherwise linked versions of the object will use independent caches."

— Manual, https://docs.blender.org/manual/en/5.0/physics/baking.html, read 2026-07-30

### One 5.0 change that bears directly on this

Packed linked data changes where a chained dependency physically lives:

> "Packed IDs remain considered as linked data (i.e. they cannot be edited), but they are stored in the current blendfile. This means that they:
>
> * Are not lost in case the library data becomes unavailable.
> * Are not changed in case the library data is updated."

> "These packed IDs are de-duplicated across blend-files, so e.g. if a shot file and several of its dependencies all use the same util geometry node, there will be a single copy of that geometry node in the shot file."

> "Name collisions are averted by storing these packed IDs into a new type of 'archive' libraries (and their namespaces). These libraries:
>
> * Only contain packed IDs.
> * Are owned and managed by their 'real' library data-block, called an 'archive parent'."

— Release notes, https://developer.blender.org/docs/release_notes/5.0/core/, read 2026-07-30

The matching API properties are on `Library`: "archive_libraries — Archive libraries of packed IDs, generated (and owned) by this source library", "archive_parent_library — Source library from which this archive of packed IDs was generated", "is_archive — This library is an 'archive' storage for packed linked IDs originally linked from its 'archive parent' library." (API, https://docs.blender.org/api/5.0/bpy.types.Library.html, read 2026-07-30). And on `ID`: "is_linked_packed — This data-block is linked and packed into the .blend file."

The corresponding operators are "bpy.ops.file.pack_libraries() — Store all data-blocks linked from other .blend files in the current .blend file. Library references are preserved so the linked data-blocks can be unpacked again" and "bpy.ops.file.unpack_all(...)" (API, https://docs.blender.org/api/5.0/bpy.ops.file.html, read 2026-07-30).

### A cost the manual attaches to chained references

> "Compression & Memory Use
>
> Referencing compressed blend-files may need a lot of
> memory because they have to be loaded in their entirety, even if you only link/append
> a small part of them. Once the data-blocks are loaded, however, memory usage is the same."

— Manual, https://docs.blender.org/manual/en/5.0/files/linked_libraries/link_append.html, read 2026-07-30

### Where I looked

- Searched the whole manual for "indirect" (39 hits; almost all are about indirect *lighting*, and I read every one to be sure — the only linking-related ones are the two in `link_append.txt` and `library_overrides.txt`).
- Searched the whole API for "indirect", "recursive", "recursively", "nested librar", "library of a library", "chain of librar".
- Opened: `files/linked_libraries/link_append.txt` and `library_overrides.txt` in full, `files/data_blocks.txt`, `physics/baking.txt`, `editors/outliner/interface.txt`.
- Opened in the API: `bpy.types.Library.txt`, `bpy.types.ID.txt`, `bpy.types.BlendDataLibraries.txt`, `bpy.types.BlendImportContext.txt`, `bpy.types.BlendImportContextItem.txt`, `bpy.path.txt`, `info_gotchas_file_paths_and_encoding.txt`, `bpy.ops.wm.txt`, `bpy.ops.file.txt`, `bpy.ops.outliner.txt`.
- Read `release_notes_5.0/core.md` in full, and searched all release-note files for "librar", "link", "pack", "depend".
- Opened the manual picture `_images/files_data-blocks_outliner-blender-file-view.png` hoping it would show a nested library tree. It shows only a "Current File" listing with no library entries, so it settles nothing here.
- **Ruled out, with searches that found nothing.** No stated maximum depth. No statement of what happens at a given depth. No page describing the Outliner's Blender File mode as showing a library hierarchy — the only description is "Blender File: Lists all data in the current blend-file." (Manual, https://docs.blender.org/manual/en/5.0/editors/outliner/interface.html), which does not mention nesting.

**Not covered.** This does not settle how deep the chain may actually go, only that a chain is structurally possible. It does not settle whether every kind of reference is followed through a chain, or only data-block references — for instance whether an image referenced by a material in a library two levels away is loaded. It does not settle whether a broken link partway down the chain breaks the whole chain. It does not settle the order in which paths are resolved when the same file is reached by two different chains.

**Outside the interface.** The path-resolution difference is the one part of this that is clearly not interface-bound: it is a property of how a stored relative path is interpreted, and the API documents it for Python. `bpy.data.libraries.load()` with its `recursive` option, and the pre/post blend-import handlers that expose `BlendImportContext`, are Python and so available outside the window. The repairs, again, are documented only as Outliner menu items. The sources do not say whether the "cannot relocate an indirectly linked ID" restriction is an interface restriction or a deeper one; the wording "This operation is only available" reads as being about the menu, so I flag it rather than resolve it.

**Operating system.** The sources do not say. Nothing in the linking, library or packing pages attaches an operating system to any of this.


**Observed — hand test entry 9, run 2026-08-05 on the Creator's own computer** (MacBook Pro, M1 Pro chip, macOS 26.5.2, Apple's own graphics), with Blender 5.0.1 started from a terminal with no window, using a throwaway settings folder set through `BLENDER_USER_RESOURCES`. The full list of tests is at `Audits/Blender Research/Dependencies/Hand tests.md`.

**What was done.** A chain of files was built by script. `L1.blend` holds an object `Prop` with an image texture at `//tex/img.png` beside it, inside a collection `C1`. `L2.blend` links collection `C1` from `L1.blend` as an instance and puts the resulting empty in its own collection `C2`, and so on. The top file also gets a camera, a sun, 32 by 32 output and one Cycles sample. Chains of 7, 8, 9 and 20 files were built. The top file was opened with no window and every library printed with its `filepath` and its `parent`, every object with its library and `is_library_indirect`, and for the image `bpy.path.abspath(im.filepath)` both with and without `library=im.library`. Then one frame was rendered with Cycles and its centre pixel read.

**What came back.** The same in all four versions.

At 8 files (7 libraries) the whole chain resolved. The load printed one line per level:

```
Info: Read library: '<path>/L7/L7.blend', '//../L7/L7.blend', parent '<direct>'
Info: Read library: '<path>/L6/L6.blend', '//../L6/L6.blend', parent '<path>/L7/L7.blend'
… down to …
Info: Read library: '<path>/L1/L1.blend', '//../L1/L1.blend', parent '<path>/L2/L2.blend'
```

and the parent chain was complete:

```
libraries (7):
   name='L1.blend'   filepath='//../L1/L1.blend'       parent='//../L2/L2.blend'
   name='L2.blend'   filepath='//../L2/L2.blend'       parent='//../L3/L3.blend'
   name='L3.blend'   filepath='//../L3/L3.blend'       parent='//../L4/L4.blend'
   name='L4.blend'   filepath='//../L4/L4.blend'       parent='//../L5/L5.blend'
   name='L5.blend'   filepath='//../L5/L5.blend'       parent='//../L6/L6.blend'
   name='L6.blend'   filepath='//../L6/L6.blend'       parent='//../L7/L7.blend'
   name='L7.blend'   filepath='//../L7/L7.blend'       parent=None
```

Every object below the top was marked `is_library_indirect=True`. At 20 files, 19 libraries were recorded with a complete parent chain.

**Where a path inside a library is measured from.** The image belonging to the deepest file was found, and the two ways of resolving it differ:

```
   name='img.png'  stored='//tex/img.png'      library='//../L1/L1.blend'
      abspath WITHOUT library arg: '<path>/L8/tex/img.png'
      abspath WITH    library arg: '<path>/L8/../L1/tex/img.png'
      size=(32, 32)
```

Only the second names a real file. So a tool that forgets the library argument gathers the wrong path — and this held at 20 levels as well as at 8.

**How deep it goes in practice.** The recording goes at least 19 levels. The render does not. Centre pixel of the rendered frame, the same in all four versions:

```
files=7  LIBS 6  centre_rgba=(1.0, 0.6275, 0.1373, 1.0)
files=8  LIBS 7  centre_rgba=(1.0, 0.6275, 0.1373, 1.0)
files=9  LIBS 8  centre_rgba=(0.0, 0.0, 0.0, 1.0)
```

`(1.0, 0.6275, 0.1373, 1.0)` is the texture's own colour; `(0.0, 0.0, 0.0, 1.0)` is an empty frame. Nothing was printed about it at 9 files, and at 20 files the frame was also empty. The number of evaluated instances was 10 at both 8 and 20 files.

**Read this narrowly.** The chain here is built out of nested collection instances, which is one way of chaining and not the only one. What the test shows is that at 9 files, built this way, the object stops arriving in the render while the library records and the paths still resolve. It does not show which of the two — the linking or the instancing — set that boundary, and the run gives no message either way.

**Both standing lines, brought up to date by this test.** *Driven from outside:* settled. Everything above was built, opened, resolved and rendered with no window, including the library-relative resolution. *Operating system:* macOS for this test. The same test was run on a rented Linux machine on 2026-08-06 and that result is recorded below; Windows stays untested.

**Still open.** Whether the Relocate repair is offered for a directly linked item and refused for one that arrived as a dependency is entry 23 on the hand-test list and is the Creator's to answer.


**Observed — hand test entry 23, run by the Creator on 2026-08-05 on their own computer** (MacBook Pro, M1 Pro chip, macOS 26.5.2, Apple's own graphics), in Blender 5.0.1 with its window open. This is the half of the list that needs eyes on the screen; the whole list is at `Audits/Blender Research/Dependencies/Hand tests.md`. Because it was run on a Mac, it settles macOS and leaves Windows and Linux open.

**What was done.** A file prepared for this test was opened. It links a collection called `MiddleCollection` from `middle.blend`, which itself links one called `DeepCollection` from `deep.blend` — so the file holds one thing that was linked directly and one that came along behind it. In an Outliner set to Blender File display mode, the Creator right-clicked each in turn and opened the ID Data submenu.

**What came back.** Screenshots per version. In all four versions the Outliner lists both `//middle.blend` and `//deep.blend` as libraries, with `MiddleCollection` under the first and `DeepCollection` under the second.

The ID Data submenu holds the same entries for both, in this order:

```
Unlink
Make Local
Delete
Remap Users
Copy
Paste
Add Fake User
Clear Fake User
Rename
Select Linked
Relocate
```

**Relocate is present for the indirectly linked collection as well as the directly linked one, and in none of the screenshots does it look greyed out** — its text is the same brightness as Unlink and Make Local above it. For comparison, three entries in the parent menu are visibly greyed in the same shots: Mark as Asset, Clear Asset and Clear Asset (Set Fake User).

**What the answer does not cover.** The screenshots show what the menu offers. Whether the entry does anything when it is clicked on an indirectly linked item was not tried and is not reported.

**Both standing lines, brought up to date by this test.** *Driven from outside:* not settled by this. What was looked at is a menu, which only exists with a window; the operator behind it, `bpy.ops.outliner.id_linked_relocate`, exists in all four versions as recorded under question 1, but what it does with no window was not tried. *Operating system:* macOS for this test. The same test was run on a rented Linux machine on 2026-08-06 and that result is recorded below; Windows stays untested.


**Observed — run again on Linux, 2026-08-06, and one long-open question closed.** The chain was built and read again in this version on the rented Linux machine.

**The repeat agreed with the Creator's Mac completely.** At eight files the whole chain resolved, the parent chain was complete from the deepest file to the top, every object below the top was marked indirectly linked, and the image inside the deepest file loaded at 32 by 32. Where a path inside a library is measured from came back the same:

```
   name='img.png'  stored='//tex/img.png'  library='//../L1/L1.blend'
      abspath WITHOUT library arg: '/root/…/n8/L8/tex/img.png'
      abspath WITH    library arg: '/root/…/n8/L8/../L1/tex/img.png'
      size=(32, 32)
```

Only the second names a real file. At twenty files, nineteen libraries were recorded with a complete parent chain and the image still loaded. The render showed the same boundary:

```
files=7  libraries 6  centre=(0.6314, 0.4196, 0.2588, 1.0)   the texture's own colour
files=8  libraries 7  centre=(0.6314, 0.4196, 0.2588, 1.0)
files=9  libraries 8  centre=(0.0, 0.0, 0.0, 1.0)            an empty frame
files=20 libraries 19 centre=(0.0, 0.0, 0.0, 1.0)
```

**Blender does say something about it, which the Mac run recorded as silence.** From eight files upward the load prints, twice each time:

```
files=8   Warning: Maximum instance recursion level reached in "Prop" object.
files=9   Warning: Maximum instance recursion level reached in "C1" object.
files=20  Warning: Maximum instance recursion level reached in "C12" object.
```

**And this settles which of the two sets the boundary.** The same chain was built a second time with **no collection instancing at all** — each level links the level below and puts it in as a child collection rather than as an instance, so the number of linked files grows and the nesting of instances does not:

```
--- 8 files ---   libraries 7  instances 3   centre=(0.6314, 0.4196, 0.2588, 1.0)
--- 9 files ---   libraries 8  instances 3   centre=(0.6314, 0.4196, 0.2588, 1.0)
--- 20 files ---  libraries 19 instances 3   centre=(0.6314, 0.4196, 0.2588, 1.0)
```

Nineteen libraries deep, the object still arrives and the texture still renders, and nothing is printed. **So the boundary is set by the nesting of collection instances, not by the chain of linked files.** That was settled by a test, not by reading the message. *(Narrowed by the citation check of 2026-08-06: this second shape of the chain was built and run on the rented Linux machine only. The Creator's Mac never ran it, so the boundary is settled on Linux and is untested on macOS — the standing line below covers the answer as a whole, not this claim.)*

**Relocate is still not settled.** The operator's own test said it may run for the indirectly linked collection as well as the directly linked one, which agrees with the Creator's screenshots. Calling it on the indirectly linked one returned `{'PASS_THROUGH'}` and changed nothing, but the operator reads the Outliner's own selection, which cannot be set from a script, so that result cannot be pinned to the item that was meant.

**Both standing lines, brought up to date by the Linux tests.** *Driven from outside:* every reading above was taken with Blender running with no window unless it says otherwise, so it holds outside the interface. *Operating system:* **now settled on two operating systems, macOS and Linux, and each is its own result** — what they agree on and where they differ is said above. **Windows is still untried: no Windows machine has run it.**

---

## 15. Can a file rely on a colour management profile that lives on the computer rather than inside the file, and how does Blender find it?

**LABEL: documented.**

### Yes

> "Blender comes with a standard OpenColorIO configuration that
> contains a number of useful display devices and view transforms.
>
> However, OpenColorIO is also designed to give a consistent user experience across
> multiple applications, and for this, a single shared configuration file can be used.
> Blender will use the standard OCIO environment variable to read an OpenColorIO configuration
> other than the default Blender one."

> "There is also a BLENDER_OCIO environment variable to change the configuration
> for Blender only. It is recommended to use OCIO when possible for compatibility
> with other software and pipelines that may not be aware of BLENDER_OCIO. However
> sometimes there are incompatibilities in configuration files that make it difficult
> to share them across applications."

> "This covers most needs for working in an ACES pipeline. However for more complete
> support, the official ACES configurations
> can be manually installed and set through the OCIO environment variable."

— Manual, https://docs.blender.org/manual/en/5.0/render/color_management/opencolorio.html, read 2026-07-30

### How Blender finds it

Two environment variables, with a stated order between them:

> "BLENDER_OCIO:
>
> Path to override the OpenColorIO configuration file.
> If not set, the OCIO environment variable is used."

— Manual, https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html, read 2026-07-30

If neither is set, the configuration that ships with Blender is used, and the manual says where that lives:

> "./datafiles/colormanagement
>
> Default OpenColorIO configuration."

— Manual, https://docs.blender.org/manual/en/5.0/advanced/blender_directory_layout.html, read 2026-07-30

That folder sits under Blender's system directories, and those can themselves be moved: "BLENDER_SYSTEM_DATAFILES: Directory to replace bundled datafiles." (Manual, https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html), and there is a command-line argument to set it: "--env-system-datafiles — Set the BLENDER_SYSTEM_DATAFILES environment variable." (same page). Both read 2026-07-30.

The `BLENDER_OCIO` variable is stated as new in this version:

> "New `BLENDER_OCIO` environment variable, that may be used instead of `OCIO` to change the configuration only for Blender."

— Release notes, https://developer.blender.org/docs/release_notes/5.0/color_management/, read 2026-07-30

### What the file relies on, exactly

The file does not store the configuration or a path to it. It stores names that are looked up in whatever configuration is loaded. The API shows this: the properties that hold those names are enumerations whose documented value list is empty or trivial, because the values come from the configuration at run time.

> "display_device
>
> Display name. For viewing, this is the display device that will be emulated by limiting the gamut and HDR colors. For image and video output, this is the display space used for writing.
>
> Type: enum in ['NONE'], default 'NONE'"

— API, https://docs.blender.org/api/5.0/bpy.types.ColorManagedDisplaySettings.html, read 2026-07-30

> "name
>
> Color space in the image file, to convert to and from when saving and loading the image
>
> Type: enum in Color Space Convert Default Items, default 'NONE'"

— API, https://docs.blender.org/api/5.0/bpy.types.ColorManagedInputColorspaceSettings.html, read 2026-07-30

That the file depends on a configuration it does not contain is stated directly, in the flag Blender sets when the lookup fails:

> "is_missing_opencolorio_config
>
> A color space, view or display was not found, which likely means the OpenColorIO config used to create this blend file is missing"

— API, https://docs.blender.org/api/5.0/bpy.types.BlendFileColorspace.html, read 2026-07-30

And the release notes record that this became visible in 5.0:

> "When a blend file is loaded that was created with another OpenColorIO config, a warning is now shown."

— Release notes, https://developer.blender.org/docs/release_notes/5.0/color_management/, read 2026-07-30

### One thing the file does store

The working space is a file-level property with a fixed, closed set of values, so it does not depend on the configuration in the same way:

> "working_space
>
> Color space used for all scene linear colors in this file, and for compositing, shader and geometry nodes processing
>
> Type: enum in ['Linear Rec.709', 'Linear Rec.2020', 'ACEScg'], (readonly)"

— API, https://docs.blender.org/api/5.0/bpy.types.BlendFileColorspace.html, read 2026-07-30

> "Blend files now have a working color space. The default is Linear Rec.709 as before, with options for Linear Rec.2020 and ACEScg."
>
> "The working space affects the colors of all data-blocks in a file, and has a significant effect on rendering and compositing results. Generally the working space should be chosen at the start of a project and used for all blend files."
>
> "For linked blend files, colors will automatically be converted to match the current file."

— Release notes, https://developer.blender.org/docs/release_notes/5.0/color_management/, read 2026-07-30

The manual says the same and adds: "Blender can convert between different working spaces, however this is only an approximation and manual fix-ups are typically needed. When linking and appending data-blocks, colors are automatically converted to match the current file." (Manual, https://docs.blender.org/manual/en/5.0/render/color_management/color_spaces.html, read 2026-07-30).

### What a replacement configuration must provide

The manual sets out requirements, which is itself evidence that the configuration is an outside dependency the file cannot guarantee:

> "Use OpenColorIO v2 display_colorspaces and view_transforms with cie_xyz_d65_interchange
> intermediate display linear space. … In older configs without this, display emulation will be disabled and wide gamut and HDR display will
> not work well. Saving colorspace metadata for images and video also depends on this."

> "For every display, include a view transform without tone mapping. Blender will look for a view
> transform named Standard or Un-tone-mapped or the config wide default_view_transform.
> If not found, the first view transform of the display will be used."

The page also names the roles Blender expects a configuration to define: `scene_linear`, `data`, `aces_interchange`, `cie_xyz_d65_interchange`, `color_picking`, `default_sequencer`, `default_byte`, `default_float`.

— Manual, https://docs.blender.org/manual/en/5.0/render/color_management/opencolorio.html, read 2026-07-30

### Where I looked

- Searched the whole manual, API and release notes for: "OCIO", "OpenColorIO", "color management", "colormanagement", "config.ocio", "color profile", "colour profile", "display profile", "ICC", "working space", "Working Space", "BlendFileColorspace".
- Opened all four manual pages under `render/color_management/`: `index.txt`, `color_spaces.txt`, `displays_views.txt`, `opencolorio.txt`.
- Opened `advanced/command_line/arguments.txt` (environment-variable section in full) and `advanced/blender_directory_layout.txt` in full.
- Opened in the API: `bpy.types.BlendFileColorspace.txt`, `bpy.types.ColorManagedDisplaySettings.txt`, `bpy.types.ColorManagedInputColorspaceSettings.txt`, `bpy.types.ColorManagedViewSettings.txt`, `bpy.types.BlendData.txt`, `mathutils.txt`.
- Read `release_notes_5.0/color_management.md` in full.
- Searched the Cycles files for "OCIO" and "OpenColorIO": no hits. Cycles' own files say nothing about this.
- **Ruled out, with a search that found nothing.** I searched manual, API and release notes (excluding the `_images` folder) for "ICC", "color profile", "colour profile" and "display profile". The single hit is an unrelated sentence about a fluid flame colour ramp. So: there is nothing in these sources about ICC or monitor profiles as a dependency; the whole answer is about OpenColorIO configurations.

**Not covered.** This does not settle what Blender does at render time when the configuration is missing or does not contain a name the file uses — only that a flag is set and a warning is shown on load. It does not settle whether that warning appears anywhere other than the interface. It does not settle whether the configuration path is checked when a file is merely linked rather than opened. It does not settle what happens when two machines in the same pipeline have different configurations. It does not settle whether a configuration can be packed into a blend-file (I found no mention of packing in connection with colour management, and colour management does not appear in the data-block pack table).

**Outside the interface.** The lookup mechanism is environment variables, which is if anything better suited to outside-the-window use than to inside it: both `OCIO` and `BLENDER_OCIO` are read from the environment Blender is started in, and `--env-system-datafiles` sets the fallback location as a command-line argument. There is no documented way to set the configuration from inside the interface at all — the manual describes only environment variables and manual installation. The one part that is interface-bound is the reporting: the release note says a "warning is now shown", and the manual's instruction to change the working space is written as "go to Color Management > Working Space". The underlying flag `is_missing_opencolorio_config` is readable from Python, so a caller outside the window can check it; the warning display itself is not documented for that case.

**Operating system.** The sources do not say the mechanism differs by operating system, and the environment variables are named without any per-system qualification, unlike the temporary-directory variables on the same page which are explicitly split between Windows and other systems. The bundled configuration's location is stated as a relative path, "./datafiles/colormanagement", under system directories that the same page does list per operating system (Linux, macOS, Windows). One requirement in the release notes is per-system, but it concerns HDR display rather than finding the configuration: "macOS: Any Apple Silicon device. Linux: Use Wayland, and set the Vulkan backend in the Blender system preferences. Windows: Enable "Automatically manage color for apps" or "Use HDR" in the Windows display settings, and set the Vulkan backend in the Blender system preferences." (Release notes, https://developer.blender.org/docs/release_notes/5.0/color_management/, read 2026-07-30).


**Observed — hand test entry 19, run 2026-08-05 on the Creator's own computer** (MacBook Pro, M1 Pro chip, macOS 26.5.2, Apple's own graphics), with Blender 5.0.1 started from a terminal with no window, using a throwaway settings folder set through `BLENDER_USER_RESOURCES`. The full list of tests is at `Audits/Blender Research/Dependencies/Hand tests.md`.

**What was done.** A small OpenColorIO configuration was written holding one colour space `TinyLinear`, one display `TinyDisplay` and one view `TinyView`, saved as `tiny.ocio`. Each version was then started four ways with no window: with nothing set; with `OCIO` pointing at `tiny.ocio`; with `BLENDER_OCIO` pointing at it; and with `BLENDER_OCIO` pointing at a path that does not exist. Each time both environment variables as Blender sees them, `bpy.data.colorspace.is_missing_opencolorio_config`, the list of colour spaces offered, the scene view transform and a new image's colour space were printed. Separately, a file was saved with the bundled configuration in which an image's colour space is `ACEScg` and the scene's view transform is `Filmic`, then reopened under `tiny.ocio`, which has neither name.

**Which variable this version honours.** Both `OCIO` and `BLENDER_OCIO` work, and setting `BLENDER_OCIO` also set `OCIO` in the process environment. Blender does not put `OCIO` into its own environment at start-up; 5.1.2 and 5.2.0 do. In 4.5.12 `BLENDER_OCIO` is ignored altogether.

**How much the configuration decides.** The bundled configuration offers 25 colour spaces in this version (21 in 4.5.12, 25 in 5.0.1 and 5.1.2, 63 in 5.2.0). With `tiny.ocio` active the list dropped to `['TinyLinear', 'scene_linear']` and the view transform became `TinyView`.

**Pointing at a configuration that is not there.** All four fell back to the bundled configuration, offered the full list again, and printed an error naming the file:

```
00:00.003  color_management | ERROR OpenColorIO Error: Error could not read '<path>' OCIO profile.
```

`bpy.data.colorspace.is_missing_opencolorio_config` was `False` even then.

**A name the active configuration does not have.** Opening the saved file under `tiny.ocio` gave, in all four versions:

```
  image colorspace name now = ''
  scene view transform now  = 'TinyView'
```

with these lines printed (4.5's wording; 5.x carries the same text under a `color_management | WARNING` prefix):

```
Color management: scene view "Filmic" not found, setting default "TinyView".
Color management: image colorspace "ACEScg" not found, will use default instead.
WARN (bpy.rna): current value '-1' matches no enum in 'ColorManagedInputColorspaceSettings', '(null)', 'name'
```

So the file's own colour-space name is silently thrown away and comes back as an empty string, and the view transform is silently replaced. The render is not stopped.

Two further warnings appear in this version that 5.1.2 and 5.2.0 do not print: `Display "sRGB" used by scene output not found, setting to "TinyDisplay".` and `scene output view "Standard" not found, setting to "TinyView".`

**Both standing lines, brought up to date by this test.** *Driven from outside:* settled — the whole mechanism is an environment variable, every reading above was taken with no window, and every message reached the terminal. *Operating system:* macOS for this test. The same test was run on a rented Linux machine on 2026-08-06 and that result is recorded below; Windows stays untested.

**Still open.** What the colour-space field shows on screen when the name is gone is entry 25 on the hand-test list and is the Creator's to answer.


**Observed — hand test entry 25, run by the Creator on 2026-08-05 on their own computer** (MacBook Pro, M1 Pro chip, macOS 26.5.2, Apple's own graphics), in Blender 5.0.1 with its window open. This is the half of the list that needs eyes on the screen; the whole list is at `Audits/Blender Research/Dependencies/Hand tests.md`. Because it was run on a Mac, it settles macOS and leaves Windows and Linux open.

**What was done.** A file was prepared with the bundled colour configuration in which an image's colour space is `ACEScg` and the scene's View Transform is `Filmic`. The Creator started Blender from a terminal with the `OCIO` environment variable pointing at a small configuration holding one colour space, one display and one view, none of them those names, and opened the file. They then read the image's Color Space field and the scene's View Transform field.

**What came back**, in the Creator's own words:

> "4.5
> TinyLinear
> TinyView
> No error messages
>
> 5.0, 5.1, 5.2
> TinyLinear
> TinyView
> There's a yellow warning at the bottom right corner of the window that just says "Color Management". It can't be clicked and its tooltip says "Displays, views or color spaces in this file were missing and have been changed."
> Interestingly, when closing Blender I got a warning in the close dialog that said "Displays, views or color spaces in this file were missing and have been changed.
> Saving it with this OpenColorIO configuration may cause loss of data.""

The first name is what the image's Color Space field read; the second is what the View Transform read.

**So in this version the interface does say something.** A yellow warning reading `Color Management` sits in the bottom right corner of the window; it cannot be clicked, and hovering it gives `Displays, views or color spaces in this file were missing and have been changed.` The close dialog carries a second warning: `Displays, views or color spaces in this file were missing and have been changed. Saving it with this OpenColorIO configuration may cause loss of data.` In 4.5 there is no warning of any kind.

**Read alongside the terminal test.** With no window the same file under the same configuration reported `image colorspace name now = ''` — an empty string — and `scene view transform now = 'TinyView'`, with the substitutions named in lines beginning `Color management:`. On screen the Color Space field does not read as empty: it reads `TinyLinear`, the configuration's own default. The two readings are of different things — what the property returns to a script, and what the field displays — and both are recorded here as they were seen.

**Both standing lines, brought up to date by this test.** *Driven from outside:* the substitution happens either way, and the messages reach the terminal either way. What is new here is that from 5.0 onward the interface keeps a standing warning and warns again before the file is saved, and that 4.5 does neither. *Operating system:* macOS for this test. The same test was run on a rented Linux machine on 2026-08-06 and that result is recorded below; Windows stays untested.


**Observed — run again on Linux, 2026-08-06.** Every part of this was run again in this version on the rented Linux machine, and **every part agreed with the Creator's Mac.**

**Which variable is honoured.** Both work. Setting `BLENDER_OCIO` also set `OCIO` in the process environment. With nothing set, `OCIO` in Blender's own environment read `None` — **so this version does not put `OCIO` into its own environment at start-up.** The same as on the Mac.

**How many colour spaces the bundled configuration offers: 25**, the same as on the Mac. With the small configuration active the list dropped to two entries; here the second read `'Working Space'` where the Mac's record has `'scene_linear'`. Both readings are recorded and which reading each run took is not established.

Pointing at a configuration that does not exist fell back to the bundled one, offered the full list again, and printed `color_management | ERROR OpenColorIO Error: Error could not read '/root/…/no_such.ocio' OCIO profile.` `is_missing_opencolorio_config` returned `False` in every case tried, including that one.

**The two further warnings this version prints and 5.1 and 5.2 do not** appeared here as well: `WARNING Display "sRGB" used by scene output not found, setting to "TinyDisplay".` and `WARNING scene output view "Standard" not found, setting to "TinyView".`

**With a window open**, the status bar carried a **yellow panel with a white warning triangle reading `Color Management`**, to the right of the version number. That is what the Creator saw on the Mac, seen again here.

**Opening a file whose image colour space is `ACEScg` and whose view transform is `Filmic` under a configuration that has neither name** gave, on Linux as on the Mac:

```
  image colorspace name now = ''
  scene view transform now  = 'TinyView'
```

So the file's own colour-space name is silently thrown away and comes back as an empty string, the view transform is silently replaced, and the render is not stopped. **And the two readings disagree in the same way on both machines**: a script asking for the image's colour space gets an empty string while the field on screen shows the configuration's own substitute.

The tooltip and the close-dialog warning the Creator saw on the Mac need a pointer resting on a control and were not photographed on Linux.

**Both standing lines, brought up to date by the Linux tests.** *Driven from outside:* every reading above was taken with Blender running with no window unless it says otherwise, so it holds outside the interface. *Operating system:* **now settled on two operating systems, macOS and Linux, and each is its own result** — what they agree on and where they differ is said above. **Windows is still untried: no Windows machine has run it.**

---

## What a running Blender still has to settle

Every test on the first list was carried out on 2026-08-05 — twenty from a terminal and six by the
Creator with Blender's window open — and every one of them was **run again on a rented Linux machine on
2026-08-06**, along with four tests that machine made possible for the first time. The results of both
are written into the answers above, each saying what was done, what came back and on what kind of
computer. The two lists, with the exact commands, are at
`Audits/Blender Research/Dependencies/Hand tests.md` and
`Audits/Blender Research/Dependencies/Hand tests — Linux.md`. What is left is this.

**Not run with the equipment used.** There are two computers — the Creator's MacBook Pro
and a rented Linux container — and no Windows machine, no second machine of either kind, no folder on
network storage both can reach, and no graphics card of any make but Apple's and NVIDIA's. **A Windows machine can be rented and
none was rented for this**, so the Windows halves below stay untried; the rest stay unanswered: what a drive-lettered path written on Windows means elsewhere and the
reverse (questions 5 and 6); whether a file carrying a non-Latin path survives being moved to Windows
(question 9); and what a library or a texture on network storage means at either end (questions 2 and 7).
What could be reached from the two machines is recorded above under each of those questions.

**Small gaps left by tests that otherwise answered.** Under question 13, whether the Relocate entry does
anything when it is used on an indirectly linked item: the menu was photographed on the Mac and the
operator's own test was asked on Linux, but an Outliner selection cannot be set from a script, so neither
answer can be pinned to the item that was meant. Under questions 5, 15 and 16, three things that need a
pointer resting on a control — what the Font panel says, what the tooltip on a broken output path says,
and what the ID Data menu holds — were photographed on the Mac only; the reason they could not be
photographed on Linux is written into `Audits/Blender Research/Dependencies/Hand tests — Linux.md`. And
under question 10, what a render does when it genuinely cannot write its output folder: every Linux run
was made as a user who owns the root of the disk, so the write always succeeded.

**Closed since.** Under question 3, what sits beside a *packed* image's file-path field — a stack of
boxes where the unpacked one carries a plain folder. Under question 10, whether a File Output node set up
one way still writes when the file is rendered the other way — it does, in all four versions. Under
question 13, which of the two sets the boundary at nine files — the nesting of collection instances, not
the chain of linked files. All three were settled on the Linux machine on 2026-08-06.

**Not closed by any test.** Whether the list under question 1 is the whole list: the test file was built
to hold seven kinds, so a kind nobody thought to include would not have appeared.

---

2026-08-04 — Opus 5 (Claude Code) — Dependencies, Blender 5.0

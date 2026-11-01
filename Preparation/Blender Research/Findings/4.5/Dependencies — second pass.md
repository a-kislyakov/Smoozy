# Dependencies — Blender 4.5 — second pass

**Sources consulted, and when.** Blender's 4.5 manual, Blender's 4.5 Python API documentation, Blender's 4.5 release notes, and Cycles' own add-on files for 4.5 — all as the plain-file copies in `Blender Sources/`, taken 2026-07-30. I used 2026-07-30 as the date read throughout. I also opened one picture from the manual (`_images/files_data-blocks_id-types.png`).

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

Questions answered on this pass: 1, 5, 12, 13, 14.

---

## 1. What kinds of thing can a `.blend` file point at outside itself? Give the full list.

**Reading taken.** The question is slightly ambiguous. It could mean "which kinds of data-block carry a link to a file outside the blend" or "every distinct kind of outside target a blend file can record an address for". I answer the second, wider reading, because it is the one that a full list makes sense of. Under the narrower reading the answer would be only the data-block types marked as referencing files in the manual's data-block table (Font, Image, Movie Clip, Sound, Library, Text, Cache File, Volume).

**LABEL: documented (each item below), with the completeness of the list itself *inferred*.**

Every item below is quoted from a source. What is *not* documented anywhere I could find is that these together are *all* of them — no page in the manual or API says "here is the complete set of external references". I assembled the list by sweeping the API for the marker Blender puts on every path-typed property, so I can say the list is complete *for properties carrying that marker*, which is a step beyond what any single page states.

### (a) Other blend files — linked libraries

> "External .blend file from which data is linked"
> `filepath` — "Path to the library .blend file"
> "string, default “”, (never None, blend relative // prefix supported)"

— API, https://docs.blender.org/api/4.5/bpy.types.Library.html — read 2026-07-30

The manual's data-block table lists this as a data-block type of its own:

> "Library … References to an external blend-file. Access from the Outliner's Blender File view."

— Manual, https://docs.blender.org/manual/en/4.5/files/data_blocks.html — read 2026-07-30

There is also a weak, non-linking reference to another blend file kept after appending:

> "Weak reference to a data-block in another library .blend file (used to re-use already appended data instead of appending new copies)"

— API, https://docs.blender.org/api/4.5/bpy.types.ID.html — read 2026-07-30

### (b) The data-block types the manual itself marks as file-referencing

The manual's data-block table has a "Pack" column, described as:

> "Pack
> File Packing, supports file contents being packed into the blend-file (not applicable for most data-blocks which have no file reference)."

and the rows that carry a file reference are:

> "Font ✓ ✓ References font files. Used by curve object-data of text objects."
> "Image ✓ ✓ Image files. Used by shader nodes and textures."
> "Movie Clip ✓ ✗ Reference to an image sequence or video file. Used in the Movie Clip editor."
> "Sounds ✓ ✓ Reference to sound files. Used as data of speaker objects."
> "Text ✓ ✗ Text data. Used by Python scripts and OSL shaders."
> "Cache File ✓ — Used by Mesh Cache modifiers."

— Manual, https://docs.blender.org/manual/en/4.5/files/data_blocks.html — read 2026-07-30

I opened the picture on that same page, `_images/files_data-blocks_id-types.png`, which shows Blender's own "ID Type" list. Visible in it: Material, Mask, Lattice, Line Style, Library, Light, Key, Image, Collection, Grease Pencil, Font, Curve, Cache File, Camera, Brush, Armature, Action, Workspace, World, Window Manager, Texture, Text, Speaker, Sound, Scene, Light Probe, Particle, Palette, Paint Curve, Object, Node Tree, Movie Clip, Mesh, Metaball. **documented** (it is part of the manual). Note that neither the table nor the picture includes Volume, even though the API gives Volume a file path (below) — so the manual's table is not a complete list of file-referencing data-blocks.

### (c) The full sweep of path-typed properties in the API

Blender marks every path property that understands the blend-relative prefix with the same phrase. Searching the whole API for `blend relative // prefix supported` returns 41 pages. Excluding preferences (which are not stored in the blend file) and the file-browser's own transient fields, the external targets a blend file can record are:

| What is pointed at | Exact wording | API page |
|---|---|---|
| Another blend file (library) | "Path to the library .blend file" | `bpy.types.Library` |
| Image or movie for an Image data-block | "Image/Movie file name" and "Image/Movie file name (without data refreshing)" | `bpy.types.Image` |
| Font file | `filepath` (no description given) on "Vector font for Text objects" | `bpy.types.VectorFont` |
| Sound file | "Sound sample file used by this Sound data-block" | `bpy.types.Sound` |
| OpenVDB volume file | "Volume file used by this Volume data-block" | `bpy.types.Volume` |
| The volume file actually used this frame | "Volume file used for loading the volume at the current frame. Empty if the volume has not be loaded or the frame only exists in memory." | `bpy.types.VolumeGrids` |
| Movie or image sequence for a Movie Clip | "Filename of the movie or sequence file" | `bpy.types.MovieClip` |
| Movie Clip proxy files | "Location to store the proxy files" | `bpy.types.MovieClipProxy` |
| Alembic / USD archive | "Path to external displacements file" | `bpy.types.CacheFile` |
| Extra archive layers on a cache file | "Path to the archive" | `bpy.types.CacheFileLayer` |
| External displacements for Multires | "Path to external displacements file" | `bpy.types.MultiresModifier` |
| External cache for the Mesh Cache modifier | "Path to external displacements file" | `bpy.types.MeshCacheModifier` |
| Physics point cache on disk | "Cache file path" | `bpy.types.PointCache`, `bpy.types.PointCacheItem` |
| Fluid simulation cache | "Directory that contains fluid cache files" | `bpy.types.FluidDomainSettings` |
| Ocean modifier bake | "Path to a folder to store external baked images" | `bpy.types.OceanModifier` |
| Geometry-nodes bake | "Location on disk where the bake data is stored" | `bpy.types.NodesModifierBake`, `bpy.types.NodesModifier` |
| Dynamic Paint output textures | "Directory to save the textures" | `bpy.types.DynamicPaintSurface` |
| Render output | "Directory/name to save animations, # characters define the position and padding of frame numbers" | `bpy.types.RenderSettings` |
| Compositor File Output node | "Base output path for the image" | `bpy.types.CompositorNodeOutputFile` |
| Bake-to-file images | "Image filepath to use when saving externally" | `bpy.types.BakeSettings` |
| IES light profile | "IES light path" | `bpy.types.ShaderNodeTexIES` |
| OSL shader script | "Shader script path" | `bpy.types.ShaderNodeScript` |
| Brush icon | "File path to brush icon" | `bpy.types.Brush` |
| Sequencer image strip folder | `directory` | `bpy.types.ImageStrip` |
| Sequencer movie strip file | `filepath` | `bpy.types.MovieStrip` |
| Sequencer proxies | "Location to store the proxy files" / "Location of custom proxy file" | `bpy.types.StripProxy` |
| Sequencer's own proxy directory | (on `bpy.types.SequenceEditor`) | `bpy.types.SequenceEditor` |
| A collection's configured exporter output | "The file path used for exporting" | `bpy.types.CollectionExport` |
| A node socket that is itself a file path | (subtype) | `bpy.types.NodeSocketStringFilePath` |

All the above — API, `https://docs.blender.org/api/4.5/<page>.html`, read 2026-07-30.

One external reference is *not* marked blend-relative:

> `filepath` — "Filename of the text file"
> "string, default “”, (never None)"

— API, https://docs.blender.org/api/4.5/bpy.types.Text.html — read 2026-07-30

### (d) Things pointed at that are written rather than read

Also outside the file, and covered by the manual: the physics disk cache folder beside the blend file —

> "When Disk Cache is checked, Blender will save the cache separately to the drive in a folder named blendcache_[filename] alongside the blend-file. (The blend-file must be saved first.)"

— Manual, https://docs.blender.org/manual/en/4.5/physics/baking.html — read 2026-07-30

### (e) What Blender's own "find missing files" tool treats as external

The operator that hunts for missing external files exposes these file filters, which is Blender's own working notion of the kinds involved:

> "filter_blender … filter_backup … filter_image … filter_movie … filter_python … filter_font … filter_sound … filter_text … filter_archive … filter_btx … filter_collada … filter_alembic … filter_usd … filter_obj … filter_volume … filter_folder … filter_blenlib"

— API, https://docs.blender.org/api/4.5/bpy.ops.file.html — read 2026-07-30

### (f) Which of these can be brought inside the file instead

> "Blender has the ability to encapsulate (incorporate) various kinds of data within the blend-file that is normally saved outside of the blend-file."
> "Not all external files can be packed. Some typically heavy external files, like videos from the Sequence Editor or Movie Clips, cannot be packed in a blend-file."

— Manual, https://docs.blender.org/manual/en/4.5/files/blend/packed_data.html — read 2026-07-30

The API gives a `packed_file` property on exactly five data-block types: Image, Library, Sound, Volume, VectorFont (plus `packed_files` on Image, and `ImagePackedFile`). — API pages of those types, read 2026-07-30.

**Not covered.** No source states that this is the complete set, so I cannot promise nothing is missing. In particular I did not find, and therefore cannot list: whether a blend file records any address for a colour-management configuration (the manual says only that Blender reads the standard OCIO environment variable, https://docs.blender.org/manual/en/4.5/render/color_management.html, read 2026-07-30 — that is the machine's setting, not the file's); whether add-on or extension locations are ever recorded in the file (the sources put those in Preferences only); and whether drivers or Python expressions inside a file can name files the sources do not enumerate. The list also says nothing about which of these are read at render time versus only in the interface.

**Outside the interface.** The sources do not say. Every item above is a property or a data-block field, which exists identically whether Blender is driven by a person or by a script — the API pages that document them are the scripting interface itself, so at minimum the *recording* of these references is not interface-dependent. Whether all of them are *resolved* when Blender runs without its window is not stated.

**Operating system.** The sources do not say that any of these kinds is confined to one operating system. Two nearby remarks are the only OS-specific text I found on external files: the render format list notes some formats are "not available on all systems" (https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html), and `bpy.path.basename` exists to "skip a “//” prefix. Use for Windows compatibility" (https://docs.blender.org/api/4.5/bpy.path.html). Both read 2026-07-30.

**Where I looked.** Manual: `files/data_blocks.txt` (whole), `files/blend/packed_data.txt` (whole), `files/index.txt`, `files/introduction.txt`, `files/file_paths.txt`, `interface/window_system/topbar.txt` (External Data section), `editors/preferences/file_paths.txt` (whole), `physics/baking.txt`, `physics/fluid/type/domain/cache.txt`, `modeling/volumes/introduction.txt`, `modeling/volumes/properties.txt`, `modeling/modifiers/physics/ocean.txt`, `modeling/modifiers/modify/mesh_cache.txt`, `modeling/modifiers/generate/geometry_nodes.txt`, `modeling/geometry_nodes/geometry/operations/bake.txt`, `scene_layout/collections/collections.txt`, `editors/text_editor.txt`, `render/color_management.txt`. Picture opened: `_images/files_data-blocks_id-types.png`. API: swept all 1,851 pages for the string `blend relative // prefix supported` (41 pages hit), then read every hit; also swept every `bpy.types.*` page for property names `filepath`, `filepath_raw`, `directory`, `cache_directory`, `font_filepath`, `filename` and read the one-line description after each; read `bpy.types.Library`, `bpy.types.ID`, `bpy.types.Volume`, `bpy.types.VolumeGrids`, `bpy.types.CacheFile`, `bpy.types.Text`, `bpy.types.Image`, `bpy.ops.file`. Searched for `packed_file`/`packed_files` across all `bpy.types.*`. Release notes: searched all of `release_notes_4.5` for "filepath", "library", "linked", "relative path", "path template"; read `core.md` in full. Cycles: `osl.py`, `engine.py`, `operators.py`, `properties.py` searched for `filepath`, `FILE_PATH`, `DIR_PATH`. **Ruled out:** the Preferences path properties (`bpy.types.PreferencesFilePaths`, `PreferencesView.font_path_ui`, `ScriptDirectory`, `UserExtensionRepo`, `Theme.filepath`) — these are settings of the installed Blender, not of the blend file, so they are not things a `.blend` points at; and `bpy.types.FileSelectParams` — the file browser's own current folder, not stored in the file.


**Observed — hand test entries 1, 2 and 20, run 2026-08-05 on the Creator's own computer** (MacBook Pro, M1 Pro chip, macOS 26.5.2, Apple's own graphics), with Blender 4.5.12 LTS started from a terminal with no window, using a throwaway settings folder set through `BLENDER_USER_RESOURCES`. The full list of tests is at `Audits/Blender Research/Dependencies/Hand tests.md`.

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

**Which machinery this version has.** Checked by name in all four versions. Present here: `bpy.data.file_path_map`, `bpy.utils.blend_paths`, `bpy.ops.file.pack_libraries`, `bpy.ops.file.pack_all`, `bpy.ops.file.report_missing_files`, `bpy.ops.file.find_missing_files`, `bpy.ops.outliner.id_linked_relocate`, `Library.parent`, `ID.is_missing`, `ID.library_weak_reference`, `bpy.path.resolve_ncase`. **Absent here and present in 5.0, 5.1 and 5.2:** `bpy.data.file_path_foreach`, `Library.is_archive`, `Library.archive_libraries`, `bpy.data.colorspace` — asking for the last raised `AttributeError: 'BlendData' object has no attribute 'colorspace'`. `WindowManager.reports` is absent here and in 5.0 and 5.1, and present in 5.2.

**One item on the list is not called the same thing in every version.** The properties of `bpy.types.CompositorNodeOutputFile` that are not already on `bpy.types.Node` are, in this version:

```
['base_path', 'active_input_index', 'format', 'save_as_render', 'file_slots', 'layer_slots']
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

**Something a Mac-written file carries that nothing had noticed.** Opening a blend-file saved on the Creator's Mac on the Linux machine, `bpy.utils.blend_paths` returned, among its paths, `'/Applications/Blender 4.5.12.app/Contents/Resources/4.5/datafiles/assets/brushes/essentials_brushes-gp_draw.blend'` — a path into the Mac's own Blender application, on a machine that has no such folder. It comes out of the file rather than out of the running Blender. Report Missing Files did **not** name it.

**Both standing lines, brought up to date by the Linux tests.** *Driven from outside:* every reading above was taken with Blender running with no window unless it says otherwise, so it holds outside the interface. *Operating system:* **now settled on two operating systems, macOS and Linux, and each is its own result** — what they agree on and where they differ is said above. **Windows is still untried: no Windows machine has run it.**

---

## 5. In what ways can a recorded location be written?

**LABEL: documented** (each form quoted below), except item 9 which is **observed** and item 3 which is **documented in the API only**.

**1. As a full path from the system's root (absolute).**

> "Make Paths Absolute — Make all paths to external files absolute (= full path from the system's root)."

— Manual, https://docs.blender.org/manual/en/4.5/interface/window_system/topbar.html — read 2026-07-30

**2. As a partial path measured from the blend file's own folder, written with a double slash at the front.**

> "When you specify one of these external files, the default option is to make the path relative. Blender stores a partial path evaluated relative to the directory location of the referencing blend-file."
> "When relative paths are supported, the File Browser provides a Relative Path checkbox, when entering the path into a text field, use a double slash prefix (//) to make it so."
> "Relative paths are the default but this can be changed in the File tab of the Preferences."
> "You cannot use relative paths into a new untitled blend-file. Save it before linking to external files."

— Manual, https://docs.blender.org/manual/en/4.5/files/blend/open_save.html — read 2026-07-30

The same prefix, said again for render output:

> "This setting expands Relative Paths where a // prefix represents the directory of the current blend-file."

— Manual, https://docs.blender.org/manual/en/4.5/render/output/properties/output.html — read 2026-07-30

Not every path field accepts this form. The API marks the ones that do with the phrase "blend relative // prefix supported"; `bpy.types.Text.filepath` is marked only "(never None)" — no relative support stated.

**3. Relative, but measured from a *library* file rather than the open one.**

> "abspath(path, *, start=None, library=None) — Returns the absolute path relative to the current blend file using the “//” prefix."
> "library (bpy.types.Library) – The library this path is from. This is only included for convenience, when the library is not None its path replaces start."

— API, https://docs.blender.org/api/4.5/bpy.path.html — read 2026-07-30

**4. With template expressions in curly braces (render output path and File Output node only).**

> "Path templates substitute template expressions (written {abcd}, where “abcd” is a variable name) in a filepath when the path is used."
> "//my_render_dir/{blend_name}.png"
> "Currently path templates are only supported for the render output path in Scene properties and the output paths in the compositor's File Output node. More filepaths will support templates in future releases."
> "blend_name: The current blend file's name (without the .blend)." / "fps: The frames per second of the current scene." / "resolution_x / resolution_y"
> "dance_{fps:###}.png: format as an integer with at least 3 digits."
> "{{ translates to a single { in the final path. }} translates to a single } in the final path."

— Manual, https://docs.blender.org/manual/en/4.5/files/file_paths.html — read 2026-07-30

The release notes state this outright as a change:

> "Some file paths in Blender can now contain "template expressions" like `{blend_name}` and `{resolution_x}`, which get replaced by the values they reference when the path is used."
> "Currently this is only enabled on the Scene's render output path and the output paths of the compositor's File Output node."

— Release notes, https://developer.blender.org/docs/release_notes/4.5/core/ — read 2026-07-30

**5. With hash marks standing for the frame number.**

> "When rendering an animation, the frame number is appended at the end of the file name with four padded zeros (e.g. image0001.png). You can set a custom padding size by adding the appropriate number of # anywhere in the file name (e.g. image_##_test.png translates to image_01_test.png)."

— Manual, https://docs.blender.org/manual/en/4.5/render/output/properties/output.html — read 2026-07-30

> "The # characters are replaced by the frame number, and used to define zero padding."
> "When the filename does not contain #, the suffix #### is added to the filename."

— Manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html — read 2026-07-30

**6. With tile tokens in angle brackets, for images split across tiles.**

> "Substitution tokens are special sequences of characters in a filename that can be replaced with more meaningful and context aware information. In this case, tokens are identified by being text wrapped in angle bracket characters."
> "<UDIM>: A 4-digit notation calculated as 1001 + u-tile + v-tile * 10."
> "<UVTILE>: A notation defined as u(u-tile + 1)_v(v-tile + 1)."
> "monster-basecolor.<UDIM>.png will load/save files like monster-basecolor.1021.png etc."

— Manual, https://docs.blender.org/manual/en/4.5/modeling/meshes/uv/workflows/udims.html — read 2026-07-30

**7. With a per-cent sign standing for a view name (multi-view / stereo output).**

> "view (string, (optional, never None)) – View, The name of the view to use to replace the “%” chars"

— API, https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html (`frame_path`) — read 2026-07-30

**8. Split into a folder plus a separate file name per frame.** Sequencer image strips record the two apart:

> `directory` — "string, default “”, (never None, blend relative // prefix supported)"
> `elements` — "StripElements bpy_prop_collection of StripElement"

— API, https://docs.blender.org/api/4.5/bpy.types.ImageStrip.html — read 2026-07-30

> `filename` — "Name of the source file"

— API, https://docs.blender.org/api/4.5/bpy.types.StripElement.html — read 2026-07-30

**9. As a bare name with no folder at all, meaning "look on a search path".** **observed**, from Cycles' own files:

```
    script_path = bpy.path.abspath(filepath, library=library)
    script_path_noext, script_ext = os.path.splitext(script_path)

    if script_ext == ".oso":
        # it's a .oso file, no need to compile
        ok, oso_path = True, script_path
    elif script_ext == ".osl":
        ...
    elif os.path.dirname(filepath) == "":
        # module in search path
        oso_path = filepath
        ok = True
    else:
        # unknown
        report({'ERROR'}, "External shader script must have .osl or .oso extension, or be a module name")
```

— `Blender Sources/cycles_4.5/osl.py`, function `update_external_script`, lines 169–194 — read 2026-07-30

That file also shows the relative-to-library rule of item 3 in actual use: `bpy.path.abspath(filepath, library=library)`.

**10. As no location at all, because the contents were brought inside the file.**

> "Selective Packing — A single file can be packed by clicking on the little “gift box” icon to the left of its file-path UI widget."

— Manual, https://docs.blender.org/manual/en/4.5/files/blend/packed_data.html — read 2026-07-30

**11. Rewritten wholesale on save.** When writing a blend from a script, the form of every recorded location can be forced:

> "path_remap (str) – Optionally remap paths when writing the file:
> NONE No path manipulation (default).
> RELATIVE Remap paths that are already relative to the new location.
> RELATIVE_ALL Remap all paths to be relative to the new location.
> ABSOLUTE Make all paths absolute on writing."

— API, https://docs.blender.org/api/4.5/bpy.types.BlendDataLibraries.html — read 2026-07-30

And from the interface: "Remap Relative — This option remaps Relative Paths (such as linked libraries and images) when saving a file in a new location." (https://docs.blender.org/manual/en/4.5/files/blend/open_save.html), plus the two operators "Make all paths to external files absolute" and "Make all paths to external files relative to current .blend" (https://docs.blender.org/api/4.5/bpy.ops.file.html). Both read 2026-07-30.

**Not covered.** The sources do not say whether a relative path may climb above the blend file's folder (`//../`), whether a home-directory shorthand (`~`) is understood, whether environment variables are expanded inside a recorded location, or which slash character is stored on which system. I searched specifically for all four and found nothing (see below). The `$filepath`, `$line`, `$column` expansions that do exist are for the *external text-editor command line* in Preferences, not for locations stored in a blend file (https://docs.blender.org/manual/en/4.5/editors/preferences/file_paths.html, read 2026-07-30) — I mention them only so nobody mistakes them for a path form. Nothing here settles what any of these forms *mean* on a different machine; that is questions 6 and 7.

**Outside the interface.** Partly settled, and in Blender's favour. The double-slash form is documented on the command line itself — "Use // at the start of the path to render relative to the blend-file" and "You can use path templating features such as {blend_name} in the path" and the `#` rule, all in the command-line arguments page (https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html, read 2026-07-30). The only interface-only part I found is how a *broken* template is shown: "the path field will be highlighted red in the UI" and a hover tooltip lists the errors (https://docs.blender.org/manual/en/4.5/files/file_paths.html) — that is plainly a window-only signal, though the same page adds that "rendering an animation will fail with an error message indicating the path errors", which is not window-only. The tile tokens and the `%` view token are documented without reference to the interface either way; the sources do not say.

**Operating system.** The sources do not say that any of these forms is limited to one operating system. The only OS-flavoured note found is `bpy.path.basename` — "Equivalent to os.path.basename, but skips a “//” prefix. Use for Windows compatibility" — and `bpy.path.native_pathsep`, "Replace the path separator with the systems native os.sep" (both https://docs.blender.org/api/4.5/bpy.path.html, read 2026-07-30). Those show Blender expects separator differences between systems, but no page states how a recorded location is written on each.

**Where I looked.** Manual, searched whole tree for: "Relative Path", "relative path", "relative to the", "absolute path", "//../", "backslash", "forward slash", "path separator", "drive letter", "UDIM", "UVTILE", "%". Read in full: `files/file_paths.txt`, `files/blend/open_save.txt`, `modeling/meshes/uv/workflows/udims.txt`, `editors/preferences/file_paths.txt`, `advanced/command_line/arguments.txt`, `advanced/command_line/render.txt`, `render/output/properties/output.txt`, `render/output/properties/stereoscopy/usage.txt`, `editors/file_browser.txt`, `interface/window_system/topbar.txt`. API: read `bpy.path.txt` in full; read the path properties of `RenderSettings`, `Image`, `Text`, `ImageStrip`, `MovieStrip`, `StripElement`, `SceneRenderView`, `BlendDataLibraries`, `CompositorNodeOutputFile`; swept all pages for "blend relative // prefix supported" and "Supports template expressions". Release notes: read `core.md`; searched all for "path". Cycles: read `osl.py` lines 155–225. **Ruled out and why:** `advanced/extensions/addons.txt` mentions relative imports — that is Python module syntax, not a file location in a blend; `addons/import_export/scene_fbx.txt` has a "Path Mode" with relative/absolute/copy options — that is how the FBX *exporter* writes paths into a foreign format, not how a blend records them; `editors/preferences/file_paths.txt` `$filepath`/`$line`/`$column` — external editor command arguments, as noted above.


**Observed — hand test entries 3, 14 and 15, run 2026-08-05 on the Creator's own computer** (MacBook Pro, M1 Pro chip, macOS 26.5.2, Apple's own graphics), with Blender 4.5.12 LTS started from a terminal with no window, using a throwaway settings folder set through `BLENDER_USER_RESOURCES`. The full list of tests is at `Audits/Blender Research/Dependencies/Hand tests.md`.

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


**Observed — hand test entry 24, run by the Creator on 2026-08-05 on their own computer** (MacBook Pro, M1 Pro chip, macOS 26.5.2, Apple's own graphics), in Blender 4.5.12 LTS with its window open. This is the half of the list that needs eyes on the screen; the whole list is at `Audits/Blender Research/Dependencies/Hand tests.md`. Because it was run on a Mac, it settles macOS and leaves Windows and Linux open.

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

**Which template variables this version really has, asked one at a time.** Each name was put into a render output path in turn and `frame_path` asked for. Four were accepted — `blend_name` gave `out_0001.png`, `fps` gave `24.0_0001.png`, `resolution_x` and `resolution_y` each gave `16_0001.png`. Six were refused, each with the same shape of message: `RuntimeError: Error: Parse errors in path '//tv/{blend_dir}_': - Unknown variable referenced in template expression '{blend_dir}'.` — and the same for `blend_name_lib`, `blend_dir_lib`, `scene_name`, `camera_name` and `node_name`.

**One thing about the drive-lettered form that neither machine had noticed, and it happens on both.** Report Missing Files prints that path as `/c/textures/tex.png` — Blender rewrites `C:/` into `/c/` in its own warning — while the stored text and `bpy.path.abspath` both still read `C:/textures/tex.png`. The warning and the stored value disagree.

**With a window open on the Linux machine**, a broken template turns the output File Path field **solid red** with the text in white on the red, in 5.0, 5.1 and 5.2, and the terminal carries `operator | ERROR Parse errors in path … Invalid or incomplete template expression`. In 4.5 the Output panel sat below the visible part of the Properties editor, so that field was not photographed on Linux; the 4.5 picture is the Mac's. The tooltip needs a pointer resting on the field and was not photographed on Linux.

**Both standing lines, brought up to date by the Linux tests.** *Driven from outside:* every reading above was taken with Blender running with no window unless it says otherwise, so it holds outside the interface. *Operating system:* **now settled on two operating systems, macOS and Linux, and each is its own result** — what they agree on and where they differ is said above. **Windows is still untried: no Windows machine has run it.**

---

## 12. What does a render do when something the file points at is not there, and does it say so anywhere?

**LABEL: not in the sources**, for the general case — with several specific cases that *are* documented, listed below.

I could not find any statement in the manual, the API documentation, the release notes, or Cycles' files describing what a render does when, say, an image texture a material points at is absent: whether the render stops, continues with a substitute, what colour that substitute is, or what is printed. That gap is the answer, and it is the important part of it.

What *is* documented:

**(a) A missing linked blend file, at the moment the file is opened.**

> "Broken Libraries — If Blender cannot find a library while loading a blend-file, it will create placeholder data-blocks to replace missing linked ones. That way, references to the missing data are not lost, and by relocating the missing library, the lost data can be automatically restored."

— Manual, https://docs.blender.org/manual/en/4.5/files/linked_libraries/link_append.html — read 2026-07-30

The placeholder is visible to a script:

> "is_missing — This data-block is a place-holder for missing linked data (i.e. it is [an override of] a linked data that could not be found anymore)"

— API, https://docs.blender.org/api/4.5/bpy.types.ID.html — read 2026-07-30

And it is a switchable behaviour of the import machinery:

> "USE_PLACEHOLDERS — Generate a placeholder (empty ID) if not found in any library files."

— API, https://docs.blender.org/api/4.5/bpy.types.BlendImportContext.html — read 2026-07-30

The manual also warns that this rescue is not total:

> "While in most cases library overrides data is preserved across a loss of reference linked data (if e.g. the library file becomes unavailable or is relocated), there are some exceptions. The main one is probably posed (but not animated) armature objects, when their Armature obdata itself is not overridden. The Pose bones of an armature object are fully linked to the bones of its Armature obdata, if the later goes missing, the pose bones are definitively lost."

— Manual, https://docs.blender.org/manual/en/4.5/files/linked_libraries/library_overrides.html — read 2026-07-30

**(b) Missing media in the Video Sequencer — this one *is* about what gets rendered.**

> "Show Missing Media — Render missing images/movies with a solid magenta color. When disabled, missing content will render fully transparent."
> "Tip — Strips with missing content will be displayed as red in the timeline."

— Manual, https://docs.blender.org/manual/en/4.5/editors/video_sequencer/preview/sidebar.html — read 2026-07-30

That setting is off unless someone turns it on:

> "show_missing_media — Render missing images/movies with a solid magenta color — boolean, default False"

— API, https://docs.blender.org/api/4.5/bpy.types.SequenceEditor.html — read 2026-07-30

**LABEL for this sub-point: inferred.** Putting the two together: with the property left at its default, missing sequencer media renders fully transparent rather than magenta. The step a reader has to supply is that "when disabled" in the manual and "default False" in the API describe the same switch — the manual does not state the default.

**(c) Gaps inside an image sequence in the Sequencer.**

> "This allows you to render an image sequence with a few frames missing and still the image strip will have the correct range to account for the missing frames displayed as black."

— Manual, https://docs.blender.org/manual/en/4.5/video_editing/edit/montage/strips/image.html — read 2026-07-30

**(d) A volume file that will not load.**

> "error_message — If loading grids failed, error message with details — string, default “”, (readonly, never None)"
> "is_loaded — List of grids and metadata are loaded in memory"

— API, https://docs.blender.org/api/4.5/bpy.types.VolumeGrids.html — read 2026-07-30

This says a message exists and can be read back from the data-block. It does not say what the render then shows.

**(e) Asking Blender to check, as a separate action.**

> "Report Missing Files — This option is useful to check if there are links to unpacked files that no longer exist. After selecting this option, a warning message will appear in the Info editor's header. If no warning is shown, there are no missing external files."

— Manual, https://docs.blender.org/manual/en/4.5/interface/window_system/topbar.html — read 2026-07-30

> "bpy.ops.file.report_missing_files() — Report all missing external files"

— API, https://docs.blender.org/api/4.5/bpy.ops.file.html — read 2026-07-30

The Info editor is where such messages land:

> "The Info editor logs the executed operators as well as errors, warnings, and informational messages."

— Manual, https://docs.blender.org/manual/en/4.5/editors/info_editor.html — read 2026-07-30

**(f) A missing or unbuildable shader script.** **observed**, from Cycles' own files — these are the messages Cycles raises itself:

```
        report({'ERROR'}, "Failed to write .oso file next to external .osl file at " + dst_path)
...
        report({'ERROR'}, "External shader script must have .osl or .oso extension, or be a module name")
...
            report({'ERROR'}, "Can't read OSO bytecode to store in node at %r" % oso_path)
...
        report({'WARNING'}, "No text or file specified in node, nothing to compile")
...
            report({'ERROR'}, tip_("OSL query failed to open %s") % oso_path)
...
        report({'ERROR'}, "OSL script compilation failed, see console for errors")
```

— `Blender Sources/cycles_4.5/osl.py`, lines 186, 193, 235, 264, 282, 284 — read 2026-07-30

Matching the manual:

> "Compilation errors will be shown in the system console."

— Manual, https://docs.blender.org/manual/en/4.5/render/cycles/osl/index.html — read 2026-07-30

**(g) A malformed path (not the same as a missing file, but adjacent, and it does stop a render).**

> "Depending on the path, template errors may prevent certain actions. For example, if there are errors in the render output path, then rendering an animation will fail with an error message indicating the path errors."

— Manual, https://docs.blender.org/manual/en/4.5/files/file_paths.html — read 2026-07-30

**(h) One case where Cycles itself refuses to proceed on a missing file.** **observed** — in the Denoise Animation operator, which reads back already-rendered frames:

```
                if not os.path.isfile(filepath):
                    scene.render.filepath = original_filepath
                    err_msg = tip_("Frame '%s' not found, animation must be complete") % filepath
```

— `Blender Sources/cycles_4.5/operators.py`, lines 89–91 — read 2026-07-30

This is denoising, not rendering, so it does not answer the question — I include it only because it is the one place in Cycles' supplied files that handles a missing file at all.

**Not covered.** Nothing here settles the central case: a missing image texture, font, cache, or volume during an actual render. Not settled: whether the render stops or continues; what a missing texture looks like in the output; whether anything is printed to the terminal, and in what words; whether the process ends with a failure code; whether Cycles and EEVEE behave alike; whether a missing file discovered mid-animation stops the remaining frames. Also not settled: whether the placeholder data-blocks created for a missing library produce any message at load time, or fail silently.

**Outside the interface.** Three of the documented signals above are plainly interface-bound: the Info-editor warning from Report Missing Files, the red strips in the Sequencer timeline, and the red-highlighted path field. What survives outside the window is not stated for any of them. The one relevant statement I found about messages when Blender is driven from a command line is:

> "-q, --quiet — Suppress status printing (warnings & errors are still printed)."

— Manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html — read 2026-07-30

**LABEL: inferred** for the reading that warnings and errors reach the terminal at all when Blender runs from a command line — the quoted line only promises they are not suppressed by `-q`; the step a reader must supply is that something which "is still printed" is printed to the terminal. Whether a missing external file counts among those warnings is not stated anywhere.

**Operating system.** The sources do not say. Nothing in any of the pages above ties this behaviour to one operating system. The only nearby OS note is that on Windows the console must be opened deliberately — "To display the console again, go to Window ‣ Toggle System Console" (https://docs.blender.org/manual/en/4.5/advanced/command_line/launch/windows.html, read 2026-07-30) — which bears on whether a person *sees* a printed message on Windows, not on whether it is produced.

**Where I looked.** Manual, searched the whole tree for: "missing", "not found", "could not be found", "cannot be found", "does not exist", "cannot open", "unable to open", "failed to load", "failed to open", "fails to", "placeholder", "magenta", "pink", "broken", "unresolved", "dangling", "invalid path", "renders as", "as black", "will be ignored", "silently", "error message", "warning", "console", "system console". Read in full or in the relevant part: `interface/window_system/topbar.txt`, `files/linked_libraries/link_append.txt`, `files/linked_libraries/library_overrides.txt`, `files/file_paths.txt`, `editors/info_editor.txt`, `editors/video_sequencer/preview/sidebar.txt`, `video_editing/edit/montage/strips/image.txt`, `video_editing/setup/directory_structure.txt`, `render/shader_nodes/textures/image.txt`, `render/output/properties/output.txt`, `render/cycles/osl/index.txt`, the whole `troubleshooting/` folder listing plus its "missing"/"not found" hits, `advanced/command_line/arguments.txt`, `advanced/command_line/render.txt`, `advanced/command_line/launch/windows.txt`. API: searched all `bpy.types.*` for "missing" and for "error_message"; read `bpy.ops.file.txt` in full, `bpy.types.ID.txt`, `bpy.types.VolumeGrids.txt`, `bpy.types.SequenceEditor.txt`, `bpy.types.BlendImportContext.txt`, `bpy.types.Image.txt` (`has_data`, `reload`). Release notes: searched all for "missing" — the only hits were bug-fix entries, one of which is `bugfixes.md`: "Internal mode in IES texture node flagged in missing file report", which confirms a "missing file report" exists but describes a fix, not the behaviour. Cycles: searched all `.py` files for `report(`, `missing`, `isfile`, `exists`; read `osl.py` and `operators.py`. **Ruled out:** `render/cycles/gpu_rendering.txt` "Error Messages" section — it is about GPU memory and driver errors, not files; `bpy.types.ImagePaint` `missing_texture`/`missing_uvs` — these are texture-paint readiness flags about *unassigned* slots, not about files absent from disk.


**Observed — hand test entries 4, 5, 6, 7, 16 and 17, run 2026-08-05 on the Creator's own computer** (MacBook Pro, M1 Pro chip, macOS 26.5.2, Apple's own graphics), with Blender 4.5.12 LTS started from a terminal with no window, using a throwaway settings folder set through `BLENDER_USER_RESOURCES`. The full list of tests is at `Audits/Blender Research/Dependencies/Hand tests.md`.

**What was done.** A file was built with a plane whose Base Color comes from an Image Texture node pointing at a solid green 32 by 32 PNG beside the blend-file, a camera, a sun, 32 by 32 output, Standard view transform and one Cycles sample. The PNG was moved aside and the file rendered from a terminal with

```
"<application>" --background --factory-startup <file>.blend --engine CYCLES --render-output <folder>/miss_CYCLES_ --render-frame 1
```

and again with `--engine BLENDER_EEVEE_NEXT`. The centre pixel of each written frame was then read by loading it into a fresh Blender.

**What came back.** The render finished, a frame was written, the exit status was 0, and the plane came out magenta — in every version and both engines. Centre pixel with the texture missing: Cycles `(1.0, 0.1373, 1.0, 1.0)`, Eevee `(1.0, 0.1294, 1.0, 1.0)`. With the texture present the same pixel was `(0.1373, 1.0, 0.1373, 1.0)` and `(0.1294, 1.0, 0.1294, 1.0)`.

**What was said about it is where the versions part company.**

In this version Cycles said **nothing at all**. The whole run contains no mention of the missing file; the only line naming it is the ordinary progress line `Updating Images | Loading green.png`. EEVEE printed one line, `GPUTexture: Blender Texture Not Loaded!`, before the version banner.

For comparison, the same test in the other versions does name the file: 5.0.1 prints `cycles           | ERROR Image file '<path>/assets/green.png' does not exist.`, 5.1.2 the same without the quotation marks, and 5.2.0 `cycles           | WARNING Image file <path>/assets/green.png does not exist.` followed by `cycles           | ERROR Failed to load 1 image files`.

**A missing font, in the same shape of test.** A text object whose font file had been moved away still rendered, exit status 0, and the evaluated mesh dropped from 332 vertices and 334 polygons to 16 and 16 — so the letterforms are lost rather than falling back to the built-in font. What was printed:

```
WARN (bke.vfont): source/blender/blenkernel/intern/vfont.cc:225 BKE_vfont_data_ensure: Font file doesn't exist: //assets/testfont.ttf
```

four times, each preceded by two lines of `Error: Unable to pack file, source path '<path>/assets/testfont.ttf' not found`. In this version those `Unable to pack file` lines also appear on a plain open; in 5.0, 5.1 and 5.2 they appear only on the render.

**A missing linked library.** Covered under question 11 above: the load prints `Unable to open ... No such file or directory`, `Cannot find lib ...`, `LIB: Collection: 'PropCol' missing from ...` and `1 libraries and 1 linked data-blocks are missing ... please check the Info and Outliner editors for details`, all to the terminal, and the render still produces a frame.

**A missing volume file.** `grids.load()` returned `False` and `grids.error_message` was `'smoke.vdb not found'`.

**A partly-packed file with its files gone.** Nothing at all was printed when the file was opened. `bpy.ops.file.report_missing_files()` does work with no window, prints `Warning: Path '<path>/assets/loose.png' not found, from local data-block 'IMloose.png'` to the terminal, and returns `{'FINISHED'}`.

**The sequencer's missing-media switch.** On a sequence editor made with `scene.sequence_editor_create()`, `show_missing_media` came back **True**, while the property's declared default read from `bpy.types.SequenceEditor.bl_rna.properties["show_missing_media"].default` is **False**. Same in all four versions.

**Both standing lines, brought up to date by this test.** *Driven from outside:* settled, and this is the important part. A render with a missing dependency does not stop, does not change the exit status, and writes its frame; from 5.0 onward Cycles names the missing image on the terminal and in 4.5 it does not; a missing font and a missing library are named in every version. *Operating system:* macOS for this test. The same test was run on a rented Linux machine on 2026-08-06 and that result is recorded below; Windows stays untested.

**Still open.** What appears on screen — the Info editor, the status bar — is entry 21 on the hand-test list and is the Creator's to answer.


**Observed — hand test entry 21, run by the Creator on 2026-08-05 on their own computer** (MacBook Pro, M1 Pro chip, macOS 26.5.2, Apple's own graphics), in Blender 4.5.12 LTS with its window open. This is the half of the list that needs eyes on the screen; the whole list is at `Audits/Blender Research/Dependencies/Hand tests.md`. Because it was run on a Mac, it settles macOS and leaves Windows and Linux open.

**What was done.** A file prepared for this test was opened. It holds two planes: the left one uses an image that was packed into the file, the right one uses an image that was not, and neither image file exists on disk any more. The Creator looked at the two images in the Image editor and at the Image Texture node in the Shading workspace, then used File ‣ External Data ‣ Report Missing Files.

**What came back.** Four screenshots, one per version, and these words from the Creator:

> "Report Missing Files says this:
> Path '/Users/artist/Documents/Smoozy hand tests/Dependencies/4.5/21/assets/loose.png' not found, from local data-block 'IMloose.png'
> Obviously 4.5 is for version 4.5 here, but the message is the same in all versions."

So the version's own folder appears in the path and the rest of the message is the same in all four. It is the same wording as the warning printed to the terminal when the same operator is called with no window.

The screenshots show the same three things in all four versions. In the image browse list, `packed.png` carries a small blue preview of itself and `loose.png` carries none. In the 3D viewport the plane using the packed image is blue — its own colour — and the plane using the unpacked one is bright magenta. In the Image editor the unpacked image shows an empty checkerboard.

**What the answer does not cover.** The entry also asked what sits to the left of each image's file-path field, to see whether the packed one carries a marker the unpacked one does not. In all four screenshots the image selected is `loose.png`, the unpacked one, and its row carries no such marker; what the packed image's row looks like was not photographed and is not reported. That part stays open.

**Both standing lines, brought up to date by this test.** *Driven from outside:* the wording of the report is now known to be the same with a window as without one, which the sources did not say. What the sources describe as landing in the Info editor is a window-only route; with no window the same text goes to the terminal. *Operating system:* macOS for this test. The same test was run on a rented Linux machine on 2026-08-06 and that result is recorded below; Windows stays untested.

**Observed — hand test entry 22, run by the Creator on 2026-08-05 on their own computer** (MacBook Pro, M1 Pro chip, macOS 26.5.2, Apple's own graphics), in Blender 4.5.12 LTS with its window open. This is the half of the list that needs eyes on the screen; the whole list is at `Audits/Blender Research/Dependencies/Hand tests.md`. Because it was run on a Mac, it settles macOS and leaves Windows and Linux open.

**What was done.** A file prepared for this test was opened. It holds a text object reading "Smoozy" that used a font file which is no longer on disk. The Creator looked at the object in the 3D viewport, pressed F12, and looked at the Font panel in Object Data Properties.

**What came back.** Three screenshots per version, and these words from the Creator: "Nothing is marked as missing, at least in Object Data Properties. All versions."

The screenshots show the same thing in all four versions. In the viewport the text is a row of empty rectangles, one per character, drawn as outlines. The F12 render shows the same: six hollow white boxes on black, one for each letter of "Smoozy". The letters are gone; they are not replaced by the built-in font's letterforms.

The Font panel reads, in all four versions:

```
Regular        Arial Regular
Bold           Bfont Regular    3
Italic         Bfont Regular    3
Bold & Italic  Bfont Regular    3
```

The Regular slot still says `Arial Regular` in ordinary white text, with the usual shield, folder and X buttons beside it and no mark of any kind. The other three slots show `Bfont Regular` greyed, each with a user count of 3.

**Both standing lines, brought up to date by this test.** *Driven from outside:* this matches the terminal test exactly. There the evaluated mesh for two letters fell from 332 vertices to 16 — eight per letter, which is one rectangle each — and here that is what a person sees. So the behaviour is the same with a window and without one, and in neither case does the text fall back to the built-in font. *Operating system:* macOS for this test. The same test was run on a rented Linux machine on 2026-08-06 and that result is recorded below; Windows stays untested.

**Observed — hand test entry 24, run by the Creator on 2026-08-05 on their own computer** (MacBook Pro, M1 Pro chip, macOS 26.5.2, Apple's own graphics), in Blender 4.5.12 LTS with its window open. This is the half of the list that needs eyes on the screen; the whole list is at `Audits/Blender Research/Dependencies/Hand tests.md`. Because it was run on a Mac, it settles macOS and leaves Windows and Linux open.

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


**Observed — run again on Linux, 2026-08-06.** Every missing-thing case was built and run again in this version on the rented Linux machine.

**What a render does is the same on both machines.** With a missing image texture, a missing font, a missing linked library or a missing volume, the render finished, a frame was written, and the exit status was 0 in every case. The plane whose texture was missing came out magenta.

**What it says is also the same on both machines.** **Cycles said nothing at all about the missing image**, in this version, on Linux as on the Mac. The only line naming the file is the ordinary progress line `Updating Images | Loading tex.png`. EEVEE printed `GPUTexture: Blender Texture Not Loaded!`. The engine's own name on the command line is `BLENDER_EEVEE_NEXT` here.

The rest, all matching the Mac: a missing font still renders with the letterforms lost rather than falling back to the built-in font; a missing linked library still renders a frame with placeholders in place of what is gone; a missing volume answers `grids.load() = False` and `grids.error_message = 'smoke.vdb not found'`; a partly-packed file whose files are gone prints nothing at all when opened; and on a sequence editor made from a script the missing-media switch reads **on** while the property's declared default is **off**.

Report Missing Files works with no window on Linux and prints the same warning naming the path and the data-block. `WindowManager.reports` raised `AttributeError: 'WindowManager' object has no attribute 'reports'` here, with a window open as well as without one.

**Find Missing Files, which no Mac test reached.** It works with no window, returns `{'FINISHED'}`, **rewrites every repaired path as a full absolute path into the search folder**, and leaves alone the paths that already resolve. It does **not** match a name case-insensitively: with a file named `texture.png` in the search folder and the reference written `Texture.PNG` it printed `Warning: Could not find 'Texture.PNG' in '…'` and changed nothing.

**A broken output-path template, with a window open on the Linux machine.** In 5.0, 5.1 and 5.2 the File Path field is filled **solid red** with the text in white on the red, and the terminal carries `operator | ERROR Parse errors in path '//out_{blend_name.png': - Invalid or incomplete template expression '{blend_name.png'.` In 4.5 the Output panel sat below the visible part of the Properties editor and was not photographed on Linux.

**Both standing lines, brought up to date by the Linux tests.** *Driven from outside:* every reading above was taken with Blender running with no window unless it says otherwise, so it holds outside the interface. *Operating system:* **now settled on two operating systems, macOS and Linux, and each is its own result** — what they agree on and where they differ is said above. **Windows is still untried: no Windows machine has run it.**

---

## 13. Does a file pointing at another `.blend` file bring that file's own references with it, how deep does that go, and can they be reached the same way?

**LABEL: documented**, on all three parts, though the depth is documented by the shape of the data rather than by a sentence.

**Yes, it brings them.** Linking one data-block pulls in whatever that data-block itself needs, and those arrive marked as *indirect*:

> "is_library_indirect — Is this ID block linked indirectly"

— API, https://docs.blender.org/api/4.5/bpy.types.ID.html — read 2026-07-30

> "INDIRECT_USAGE — That item was added for an indirectly imported ID, as a dependency of another data-block."

— API, https://docs.blender.org/api/4.5/bpy.types.BlendImportContextItem.html — read 2026-07-30

The append side of the same machinery says the same thing from the other direction:

> "Localize All — Also copy all indirectly linked data, instead of maintaining the links."

— Manual, https://docs.blender.org/manual/en/4.5/files/linked_libraries/link_append.html — read 2026-07-30

> "APPEND_RECURSIVE — Append (make local) also indirect dependencies of appended IDs coming from other libraries. NOTE: All IDs (including indirectly linked ones) coming from the same initial library are always made local."

— API, https://docs.blender.org/api/4.5/bpy.types.BlendImportContext.html — read 2026-07-30

And when a blend file is written from a script:

> "write(filepath, datablocks, path_remap=False, fake_user=False, compress=False) — Write data-blocks into a blend file.
> Note — Indirectly referenced data-blocks will be expanded and written too."

— API, https://docs.blender.org/api/4.5/bpy.types.BlendDataLibraries.html — read 2026-07-30

The manual states the same for assets:

> "All of the asset and all its dependencies will be appended to the current file."
> "“Dependencies” in this case means everything the asset refers to. For an object, this can be its mesh and materials, but also other objects used by modifiers, constraints, or drivers."

— Manual, https://docs.blender.org/manual/en/4.5/editors/preferences/file_paths.html — read 2026-07-30

**How deep.** The sources do not name a limit, and they describe the structure as a chain and a tree rather than one level:

> "parent — Type: Library, (readonly)"

— API, https://docs.blender.org/api/4.5/bpy.types.Library.html — read 2026-07-30

**LABEL for the depth claim: inferred.** A library data-block having a *library* as its parent means one linked file can itself have come in through another linked file, and nothing bounds how many times that repeats. The step a reader supplies is that a parent-of-the-same-type field implies a chain of arbitrary length. The manual supports the same shape in words, for the dependency tree and for overrides:

> "These relationships can be represented as a tree, with a root data-block ‘linking-in’ all its dependencies, recursively."
> "Recursively chaining overrides (i.e. link and override overrides from another library file, etc.)."

— Manual, https://docs.blender.org/manual/en/4.5/files/linked_libraries/library_overrides.html — read 2026-07-30

Note that "recursively" in both quotations is the manual's own word.

**Can they be reached the same way? No — indirect ones are handled differently, in four documented respects.**

*They cannot be re-pointed individually.*

> "Note — This operation is only available on data-blocks that are directly linked, and are not a dependency of any other linked data."

— Manual, https://docs.blender.org/manual/en/4.5/files/linked_libraries/link_append.html (Relocate Linked ID) — read 2026-07-30

The release notes say this in the language of a change:

> "Linked IDs can now also be relocated, if they are not a dependency of any other linked data"

— Release notes, https://developer.blender.org/docs/release_notes/4.5/core/ — read 2026-07-30

*Nothing linked is editable without an override.*

> "Linked data-blocks are initially not editable. This even includes the location/rotation/scale of linked objects, which are locked to the transformation they have in the source file."
> "Since it is not editable, linked data cannot be protected with the Fake User option."

— Manual, https://docs.blender.org/manual/en/4.5/files/linked_libraries/link_append.html — read 2026-07-30

> "Note — Linked data cannot be protected that way."

— Manual, https://docs.blender.org/manual/en/4.5/files/data_blocks.html (Protected) — read 2026-07-30

*Their own outside references are measured from the file they came from, not from yours.* This is the part that matters most to anyone gathering files. The API gives the rule:

> "library (bpy.types.Library) – The library this path is from. This is only included for convenience, when the library is not None its path replaces start."

— API, https://docs.blender.org/api/4.5/bpy.path.html — read 2026-07-30

and Cycles uses exactly that rule when resolving a shader script that arrived through a link — **observed**:

```
    script_path = bpy.path.abspath(filepath, library=library)
```
```
    osl_path = bpy.path.abspath(script.filepath, library=script.library)
```

— `Blender Sources/cycles_4.5/osl.py`, lines 169 and 209 — read 2026-07-30

*Some references brought in by a link are shared or duplicated by an explicit choice.*

> "Use Library Path — Share the disk cache when the physics object is linked into another blend-file. When this option is enabled, linked versions of the object will reference the same disk cache. Otherwise linked versions of the object will use independent caches."

— Manual, https://docs.blender.org/manual/en/4.5/physics/baking.html — read 2026-07-30

Whole libraries, including the ones that came in indirectly, can be brought inside the file:

> "Pack Linked Libraries — Pack data-blocks that are linked from an external blend-file into the current one."

— Manual, https://docs.blender.org/manual/en/4.5/interface/window_system/topbar.html — read 2026-07-30

> "bpy.ops.file.pack_libraries() — Store all data-blocks linked from other .blend files in the current .blend file. Library references are preserved so the linked data-blocks can be unpacked again"

— API, https://docs.blender.org/api/4.5/bpy.ops.file.html — read 2026-07-30

One documented failure mode of chains:

> "Circular Dependencies — In general, dependencies should not go in both directions. Attempting to link or append data which links back to the current file will likely result in missing links."

— Manual, https://docs.blender.org/manual/en/4.5/files/linked_libraries/link_append.html — read 2026-07-30

**Not covered.** The sources do not say whether packing linked libraries also packs the *non-blend* files those libraries point at (their images, fonts, caches). They do not state a maximum depth, nor what happens if a chain is very long. They do not say whether an indirectly linked library file's own path can be seen or changed at all, only that the individual data-blocks from it cannot be relocated. They do not say what happens at render time if a library two steps down the chain is absent — question 12 covers that gap. And they do not say whether an indirectly linked data-block can be turned into a directly linked one.

**Outside the interface.** Well covered here, unusually. Every part of the answer above except "Relocate Linked ID" and "Pack Linked Libraries" is stated in the Python API, which is by definition reachable without the window: `ID.is_library_indirect`, `Library.parent`, `BlendImportContext` options, `BlendDataLibraries.load`/`.write`, `bpy.path.abspath(library=…)`, `bpy.ops.file.pack_libraries()`. The two menu items are documented only as menu entries in the manual, but `bpy.ops.file` lists operator equivalents for the packing ones. Whether library relocation is available outside the window is not stated in the pages I read.

**Operating system.** The sources do not say. Nothing in any of these pages ties linking, indirect linking, or library chains to a particular operating system.

**Where I looked.** Manual: read `files/linked_libraries/index.txt`, `files/linked_libraries/link_append.txt` (whole), `files/linked_libraries/library_overrides.txt` (opening sections, hierarchy section, resync section), `files/data_blocks.txt` (whole), `files/blend/packed_data.txt` (whole), `interface/window_system/topbar.txt` (External Data), `physics/baking.txt`, `editors/preferences/file_paths.txt` (Import Method), `editors/outliner/interface.txt` (display modes). Searched the whole manual for "indirect", "recursiv", "dependency", "dependencies", "library", "linked". API: read `bpy.types.Library.txt` (whole), `bpy.types.ID.txt`, `bpy.types.BlendDataLibraries.txt` (whole), `bpy.types.BlendImportContext.txt` (whole), `bpy.types.BlendImportContextItem.txt`, `bpy.types.LibraryWeakReference.txt`, `bpy.path.txt` (whole), `bpy.ops.file.txt` (whole); swept all `bpy.types.*` for "is_library_indirect", "library_weak_reference", "indirectly linked". Release notes: read `core.md` in full; searched all subject files for "library", "linked", "relocate". Cycles: read `osl.py` lines 155–225. **Ruled out:** the manual's many hits for "indirect" under `render/` — those are all about indirect *light*, not indirect linking, and I checked each of the `render/cycles/`, `render/eevee/`, `render/layers/`, `scene_layout/collections/` hits to confirm.


**Observed — hand test entry 9, run 2026-08-05 on the Creator's own computer** (MacBook Pro, M1 Pro chip, macOS 26.5.2, Apple's own graphics), with Blender 4.5.12 LTS started from a terminal with no window, using a throwaway settings folder set through `BLENDER_USER_RESOURCES`. The full list of tests is at `Audits/Blender Research/Dependencies/Hand tests.md`.

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


**Observed — hand test entry 23, run by the Creator on 2026-08-05 on their own computer** (MacBook Pro, M1 Pro chip, macOS 26.5.2, Apple's own graphics), in Blender 4.5.12 LTS with its window open. This is the half of the list that needs eyes on the screen; the whole list is at `Audits/Blender Research/Dependencies/Hand tests.md`. Because it was run on a Mac, it settles macOS and leaves Windows and Linux open.

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

**One thing in the record for this version that is not a finding about this question.** One of the screenshots filed under 4.5 for this entry is a warning reading `File written by newer Blender binary (502.44), expect loss of data!`. The file prepared for 4.5 was made by 4.5.12 and does not produce that warning: opening it from a terminal in 4.5.12 was tried afterwards and printed nothing of the kind. What that shot was taken from is not recorded, so the 4.5 result for this entry should be read with that in mind.

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

## 14. Does rendering one scene of a file pull only that scene's references?

**LABEL: not in the sources**, for the question as asked about *references to files on disk* — with a firm documented "no" for references to *other scenes*.

**Rendering is scoped to one scene, and that scene is chosen.**

> "bpy.ops.render.render(*, animation=False, write_still=False, use_viewport=False, layer='', scene='') — Render active scene"
> "scene (string, (optional, never None)) – Scene, Scene to render, current scene if not specified"

— API, https://docs.blender.org/api/4.5/bpy.ops.render.html — read 2026-07-30

> "-S, --scene <name> — Set the active scene <name> for rendering."

— Manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html — read 2026-07-30

**But three documented mechanisms make one scene's render reach into other scenes.**

*Background scenes, and they nest.*

> "Background Scene — Allows you to use a scene as a background, this is typically useful when you want to focus on animating the foreground for example, without background elements getting in the way."
> "Background Scenes can themselves have a Background Scene (they're recursively included)."

— Manual, https://docs.blender.org/manual/en/4.5/scene_layout/scene/properties.html — read 2026-07-30

*The compositor can render a different scene.*

> "Scene — The scene for which to render a view layer."
> "View Layer — The view layer to render. The button next to the dropdown re-renders it immediately."

— Manual, https://docs.blender.org/manual/en/4.5/compositing/types/input/scene/render_layers.html — read 2026-07-30

*The sequencer can render other scenes as part of the same job.*

> "Sequencer — Renders the output of the Video Sequence editor, instead of the view from the 3D scene's active camera. If the sequence contains Scene strips, these will also be rendered as part of the pipeline. If Compositing is also enabled, the Scene strip will be the output of the Compositor."

— Manual, https://docs.blender.org/manual/en/4.5/render/output/properties/post_processing.html — read 2026-07-30

> "Scene strips are a way to insert the render output of another scene into your sequence. Instead of rendering out a video, then inserting the video file, you can insert the scene directly."

— Manual, https://docs.blender.org/manual/en/4.5/video_editing/edit/montage/strips/scene.html — read 2026-07-30

**And scenes in one file share their contents anyway.**

> "Each blend-file can contain multiple scenes, which share other data such as objects and materials."
> "Linked Copy — This option creates a new scene with the same settings and contents as the active scene. However, instead of copying the objects, the new scene contains links to the collections in the old scene."

— Manual, https://docs.blender.org/manual/en/4.5/scene_layout/scene/introduction.html — read 2026-07-30

**What is evaluated is built per scene and per view layer.**

> "scene — Original scene dependency graph is built for"
> "view_layer — Original view layer dependency graph is built for"
> "ids — All evaluated data-blocks"
> "objects — Evaluated objects in the dependency graph"

— API, https://docs.blender.org/api/4.5/bpy.types.Depsgraph.html — read 2026-07-30

And Cycles is handed exactly one of those — **observed**:

```
def render(engine, depsgraph):
    import _cycles
    if hasattr(engine, "session"):
        _cycles.render(engine.session, depsgraph.as_pointer())
```

— `Blender Sources/cycles_4.5/engine.py`, lines 89–92 — read 2026-07-30

**LABEL for the following: inferred.** Putting those two together: what Cycles is given to render is a dependency graph built for one scene and one view layer, so what Cycles sees is bounded by that scene and view layer, not by the whole file. The steps a reader must supply are (i) that the object Cycles receives is the same kind of object the `Depsgraph` page describes, and (ii) that Cycles renders only what it is given. Neither is written down. This is about what is *evaluated and rendered* — it says nothing about what is *loaded from disk*.

**Not covered — and this is the heart of the question.** No source I found says whether opening a blend file and rendering one scene causes Blender to open, read, or hold in memory the external files (textures, caches, volumes, sounds) belonging only to *other* scenes in that file. The manual describes a blend file as a single database — "Each blend-file contains a database. This database contains all scenes, objects, meshes, textures, etc. that are in the file." (https://docs.blender.org/manual/en/4.5/files/introduction.html, read 2026-07-30) — but a database entry for an image is not the same as the image file being read, and nothing states which one happens when. The nearest statement about wholesale loading is about linking, not rendering:

> "Referencing compressed blend-files may need a lot of memory because they have to be loaded in their entirety, even if you only link/append a small part of them. Once the data-blocks are loaded, however, memory usage is the same."

— Manual, https://docs.blender.org/manual/en/4.5/files/linked_libraries/link_append.html — read 2026-07-30

That is about a *library* being read when you link from it, so it does not answer the question, and I am not stretching it to.

Also not settled: whether a background scene's, a Render Layers node's, or a scene strip's external files are gathered before the render begins or as each is reached; and whether an unused scene's data-blocks are dropped at load. The one adjacent statement about unused data is about *saving*, not loading — "This works by skipping zero user data-blocks when writing blend-files" (https://docs.blender.org/manual/en/4.5/files/data_blocks.html, read 2026-07-30).

**Outside the interface.** One documented difference, and it is directly on point for anything driving Blender from outside:

> "Render Single Layer — Only render the active view layer.
> Note — This option is ignored when rendering from the command line."

— Manual, https://docs.blender.org/manual/en/4.5/render/layers/view_layer.html — read 2026-07-30

So the scope of a render is *not* identical inside and outside the window: a narrowing set inside the interface is disregarded outside it. Choosing the scene, by contrast, is documented for both — `bpy.ops.render.render(scene=…)` and `-S, --scene <name>`. Whether background scenes, Render Layers nodes pointing elsewhere, and scene strips behave identically outside the window is not stated.

**Operating system.** The sources do not say. Nothing on any of these pages ties scene scope to an operating system.

**Where I looked.** Manual: read `scene_layout/scene/introduction.txt` (whole), `scene_layout/scene/properties.txt` (Scene panel), `compositing/types/input/scene/render_layers.txt` (whole), `video_editing/edit/montage/strips/scene.txt`, `render/output/properties/post_processing.txt` (whole), `render/layers/view_layer.txt`, `render/cycles/render_settings/performance.txt` (Persistent Data), `advanced/command_line/arguments.txt`, `advanced/command_line/render.txt`, `files/introduction.txt`, `files/data_blocks.txt` (Life Time), `files/linked_libraries/link_append.txt` (Compression & Memory Use). Searched the whole manual for: "background scene", "background_set", "all scenes", "every scene", "other scenes", "loaded into memory", "in their entirety", "loaded on demand", "lazily", "only loaded", "memory usage", "persistent data", "single layer". API: read `bpy.ops.render.txt` (render operator), `bpy.types.Depsgraph.txt` (properties), `bpy.types.Scene.txt` (searched for depsgraph), `bpy.types.RenderSettings.txt`. Release notes: searched all subject files for "scene" and "render" — nothing bearing on scope. Cycles: read `engine.py` in full and `properties.py`/`ui.py` searched for "scene" handling. **Ruled out:** `render/cycles/render_settings/performance.txt` Persistent Data — "When using multiple View Layers, only data from a single view layer is preserved to keep memory usage within bounds; however, objects shared between view layers are preserved" — this is about what is *kept between* renders, not about what one render pulls, so I did not use it as an answer; `render/cycles/osl/index.txt` "on-demand texture loading" — it appears only in a list of OptiX limitations and describes an OSL feature, not Blender's scene-level loading.


**Observed — hand test entry 12, run 2026-08-05 on the Creator's own computer** (MacBook Pro, M1 Pro chip, macOS 26.5.2, Apple's own graphics), with Blender 4.5.12 LTS started from a terminal with no window, using a throwaway settings folder set through `BLENDER_USER_RESOURCES`. The full list of tests is at `Audits/Blender Research/Dependencies/Hand tests.md`.

**What was done.** One file was built with two scenes. SceneA uses only `a.png`, SceneB uses only `b.png`; each has its own camera, sun and material, and both scenes were set to Cycles so no engine had to be named on the command line. `b.png` was moved out of the way, and the file rendered twice:

```
"<application>" --background --factory-startup <file>.blend --scene SceneA --render-output <folder>/onlyA_ --render-frame 1
"<application>" --background --factory-startup <file>.blend --scene SceneB --render-output <folder>/onlyB_ --render-frame 1
```

**What came back.** Rendering SceneA said nothing about `b.png`, in any version.

**In this version the test does not settle the question.** Rendering SceneB — the control, where the missing file certainly is reached — also said nothing, which matches the finding under question 12 that 4.5's Cycles never names a missing image. So 4.5's silence when SceneA is rendered carries no information either way. In 5.0.1, 5.1.2 and 5.2.0 the control did name the file, so there the silence means something.

This is one-sided evidence: it shows that a render of one scene does not complain about the other scene's missing file. It does not show that the file was never opened. A second attempt at a stronger reading was made and abandoned — reading `image.has_data` and `image.size` after rendering SceneA reported `has_data=False` for both images, and reading `size` returned `(32, 32)` for both because reading `size` itself loads the file. Neither can tell whether a render opened a file. A file-activity trace on macOS needs an administrator password, so it was not used.

**Both standing lines, brought up to date by this test.** *Driven from outside:* the whole test was done with no window, and `--scene` chose the scene there. *Operating system:* macOS for this test. The same test was run on a rented Linux machine on 2026-08-06 and that result is recorded below; Windows stays untested.


**Observed — run again on Linux, 2026-08-06.** A two-scene file was built and rendered again in this version on the rented Linux machine. **Rendering SceneA said nothing about SceneB's missing image**, the same as on the Creator's Mac. **And the control said nothing either**, exactly as on the Mac, because this version's Cycles never names a missing image at all. **So the silence carries no information and this test does not settle the question for 4.5, on either machine.**

The evidence is one-sided on Linux for the same reason it was on the Mac: it shows that a render of one scene does not complain about the other scene's missing file, not that the file was never opened. **Nothing about this question is settled by the second machine.**

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

2026-08-04 — Opus 5 (Claude Code) — Dependencies, Blender 4.5

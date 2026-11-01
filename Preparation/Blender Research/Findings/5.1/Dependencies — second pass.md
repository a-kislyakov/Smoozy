# Dependencies — Blender 5.1 — second pass (questions 1 and 5)

**Sources consulted, and when.** The Blender 5.1 manual (`Blender Sources/manual_5.1`, including four of its picture files), the Blender 5.1 Python API documentation (`Blender Sources/api_5.1`), the Blender 5.1 release notes (`Blender Sources/release_notes_5.1`), and Cycles' own files (`Blender Sources/cycles_5.1`). All read on **2026-07-30** (the snapshot date given to me), and that is the date read for every citation below.

**I read only the question sheet and files inside `Blender Sources/`. I did not open any other part of this project.**

One disclosure so that line is exact: to find out whether the folder I had to save into existed, I ran a directory listing of `Preparation/Blender Research/Findings/5.1/`. That showed me file *names* only. I opened none of them, including the existing `Dependencies.md`, and I read nothing about this project's plans, assumptions or expectations from anywhere.

---

## How I searched

Because this pass exists to record the looking, here is the method before the answers.

I searched the whole of `manual_5.1` and the whole of `api_5.1` at once, by text, rather than opening likely files. The API documentation turned out to carry a machine-generated phrase on every property that accepts a Blender path — `blend relative // prefix supported` — and searching for that one phrase across all 2,104 API files produced a near-mechanical census of every place in Blender's data where an outside location can be recorded. That census is the backbone of both answers below, and it is what makes the completeness claim in question 1 as strong as it can be made from these sources.

Search terms I used across the manual and the API (each run over every file in both, 2,224 and 2,104 files respectively):

`blend relative // prefix supported` · `Supports template expressions` · `filepath` · `packed_file` · `packed_files` · `path template` · `template` · `relative path` / `Relative Path` / `Relative Paths` · `absolute` · `External Data` · `external file` / `external files` / `files outside` / `outside the blend` · `Report Missing Files` · `Find Missing Files` · `Make Paths Relative` · `Make Paths Absolute` · `blend_paths` · `UDIM` · `UVTILE` · `path separator` · `forward slash` · `slash character` · `os.sep` · `backslash` · `UNC` · `drive letter` · `network drive` · `C:\` · `%APPDATA%` · `expanduser` · `~/` · `home directory` · `$` (within the file-handling and preferences sections) · `environment variable` · `OpenColorIO` / `OCIO` · `OpenVDB` · `blendcache` · `Bake Path` / `Bake Directory` · `cache directory` / `Cache Directory` · `blender_assets.cats.txt` · `catalog` · `Bfont` / `built-in font` / `font file` · `path_remap` · `relative_remap` · `use_relative_path` · `abspath` / `relpath` (in the Cycles files) · `FILE_PATH` / `DIR_PATH` / `FILE_NAME` (in the Cycles files).

Pictures I opened (all four from `manual_5.1/_images/`):
`files_data-blocks_outliner-blender-file-view.png`, `files_file_paths-invalid_path_template.png`, `files_file_paths-invalid_path_template_tooltip.png`, and I noted but could not usefully read `editors_preferences_section_file-paths.webp`.

Cycles files I read in full or in part: `osl.py`, `operators.py`, `engine.py` (path handling only), and I listed `shader/` and searched all `.py` files for path-related code.

---

## 1. What kinds of thing can a `.blend` file point at outside itself? Give the full list.

**LABEL: documented for each individual kind listed; inferred for the claim that the list is complete.**

I have to separate those two things, because the sources behave differently towards them.

### 1a. The sources never state a full list in one place

That is itself a finding, and I looked hard for such a statement. The nearest things to a single list are three, and each is explicitly partial:

The glossary defines the word Blender uses for these things, and gives four examples rather than a list:

> "Resource
>
> External files such as images, sounds, fonts and volumes files that can be packed into a blend-file."

Source: manual. https://docs.blender.org/manual/en/5.1/glossary/index.html — read 2026-07-30.

The topbar page describes the External Data menu in general terms without enumerating:

> "External data, like texture images and other resources, can be stored inside the blend-file (packed) or as separate files (unpacked). Blender keeps track of all unpacked resources via a relative or absolute path."

Source: manual. https://docs.blender.org/manual/en/5.1/interface/window_system/topbar.html — read 2026-07-30.

The Python API offers one call whose whole job is to produce the list at run time, but does not say what is in it:

> "bpy.utils.blend_paths(*, absolute=False, packed=False, local=False)
>
> Returns a list of paths to external files referenced by the loaded .blend file."

Source: Python API documentation. https://docs.blender.org/api/5.1/bpy.utils.html — read 2026-07-30.

So: no page in these sources says "here is everything a blend-file can point at". What follows is assembled by me from the API property census plus the manual, and the assembling is the inferred step.

### 1b. Data-block types that carry a file reference (documented)

The manual's data-block table has a "Pack" column, explained on the page as:

> "Pack
>
> File Packing, supports file contents being packed into the blend-file (not applicable for most data-blocks which have no file reference)."

The rows that carry anything other than "—" in that column are these, quoted exactly (type, Link, Pack, description):

> "Font ✓ ✓ References font files. Used by curve object-data of text objects."
> "Image ✓ ✓ Image files. Used by shader nodes and textures."
> "Library ✗ ✓ References to an external blend-file. Access from the Outliner's Blender File view."
> "Movie Clip ✓ ✗ Reference to an image sequence or video file. Used in the Movie Clip editor."
> "Sound ✓ ✓ Reference to sound files. Used as data of speaker objects."
> "Text ✓ ✗ Text data. Used by Python scripts and OSL shaders."

Source: manual. https://docs.blender.org/manual/en/5.1/files/data_blocks.html — read 2026-07-30.

**Caution on that table, and it matters.** It is not a reliable full list, and I can show that from the sources rather than assert it. Two data-block types carry a file path in the API but show "—" in that table: Cache File and Volume. (The legend does not define "—" as "no file reference"; it says packing is "not applicable for *most* data-blocks which have no file reference". *Citation check 2026-08-05: that stronger reading was removed.*) Volume is the sharper case — it is documented in the API as packable:

> "filepath
>
> Volume file used by this Volume data-block (default "", never None, blend relative // prefix supported)"

and, further down the same page:

> "packed_file
>
> (readonly)
> Type:
> PackedFile"

Source: Python API documentation. https://docs.blender.org/api/5.1/bpy.types.Volume.html — read 2026-07-30.

And the glossary entry quoted above names "volumes files" among the things that can be packed. So the manual's own table and the manual's own glossary disagree about Volume. I report both; I do not resolve it.

Cache File likewise:

> "filepath
>
> Path to external displacements file (default "", never None, blend relative // prefix supported)"

Source: Python API documentation. https://docs.blender.org/api/5.1/bpy.types.CacheFile.html — read 2026-07-30. (The description text there reads oddly — "external displacements file" is the wording used for the Multires and Mesh Cache modifier paths too — but it is what the page says. The manual's data-block table describes Cache File as "Used by Mesh Cache modifiers", and the Alembic and USD pages treat it as the archive reference.)

### 1c. The census: every place in the file where an outside location is recorded

This is the strongest evidence I have for completeness, because it comes from a mechanically generated phrase rather than from prose. Every one of the following is a property whose API description ends with `blend relative // prefix supported`. I searched all 2,104 API files; these are all the `bpy.types.*` hits. I have grouped them and quoted the description of each.

**Whole-file references (data-blocks):**

- `Image.filepath` — "Image/Movie file name (default "", never None, blend relative // prefix supported)". Also `Image.filepath_raw` — "Image/Movie file name (without data refreshing)". https://docs.blender.org/api/5.1/bpy.types.Image.html
- `MovieClip.filepath` — "Filename of the movie or sequence file". https://docs.blender.org/api/5.1/bpy.types.MovieClip.html
- `Sound.filepath` — "Sound sample file used by this Sound data-block". https://docs.blender.org/api/5.1/bpy.types.Sound.html
- `VectorFont.filepath` — description is bare: "(default "", never None, blend relative // prefix supported)". https://docs.blender.org/api/5.1/bpy.types.VectorFont.html
- `Library.filepath` — "Path to the library .blend file". https://docs.blender.org/api/5.1/bpy.types.Library.html
- `LibraryWeakReference.filepath` — "Path to the library .blend file", on a struct described as "Read-only external reference to a linked data-block and its library file". https://docs.blender.org/api/5.1/bpy.types.LibraryWeakReference.html
- `Volume.filepath` and `VolumeGrids.frame_filepath` — the latter "Volume file used for loading the volume at the current frame. Empty if the volume has not be loaded or the frame only exists in memory." https://docs.blender.org/api/5.1/bpy.types.Volume.html and https://docs.blender.org/api/5.1/bpy.types.VolumeGrids.html
- `CacheFile.filepath` and `CacheFileLayer.filepath` — the latter "Path to the archive". https://docs.blender.org/api/5.1/bpy.types.CacheFile.html and https://docs.blender.org/api/5.1/bpy.types.CacheFileLayer.html
- `Text.filepath` — **note the difference**: "Filename of the text file (default "", never None)". This one alone lacks the `blend relative // prefix supported` clause. https://docs.blender.org/api/5.1/bpy.types.Text.html

**Modifier and simulation references:**

- `MeshCacheModifier.filepath` — "Path to external displacements file". https://docs.blender.org/api/5.1/bpy.types.MeshCacheModifier.html
- `MultiresModifier.filepath` — "Path to external displacements file". https://docs.blender.org/api/5.1/bpy.types.MultiresModifier.html
- `OceanModifier.filepath` — "Path to a folder to store external baked images". https://docs.blender.org/api/5.1/bpy.types.OceanModifier.html
- `NodesModifier.bake_directory` and `NodesModifierBake.directory` — both "Location on disk where the bake data is stored". https://docs.blender.org/api/5.1/bpy.types.NodesModifier.html and https://docs.blender.org/api/5.1/bpy.types.NodesModifierBake.html
- `PointCache.filepath` and `PointCacheItem.filepath` — "Cache file path". https://docs.blender.org/api/5.1/bpy.types.PointCache.html and https://docs.blender.org/api/5.1/bpy.types.PointCacheItem.html
- `FluidDomainSettings.cache_directory` — "Directory that contains fluid cache files". https://docs.blender.org/api/5.1/bpy.types.FluidDomainSettings.html
- `DynamicPaintSurface.image_output_path` — "Directory to save the textures". https://docs.blender.org/api/5.1/bpy.types.DynamicPaintSurface.html

**Node references:**

- `ShaderNodeScript.filepath` — "Shader script path". https://docs.blender.org/api/5.1/bpy.types.ShaderNodeScript.html
- `ShaderNodeTexIES.filepath` — "IES light path", on a node described as "Match real world lights with IES files, which store the directional intensity distribution of light sources". https://docs.blender.org/api/5.1/bpy.types.ShaderNodeTexIES.html
- `CompositorNodeOutputFile.directory` — "The directory where the image will be written". https://docs.blender.org/api/5.1/bpy.types.CompositorNodeOutputFile.html
- `NodeSocketStringFilePath.default_value` — bare description. This is the socket type used by the Geometry Nodes import nodes. https://docs.blender.org/api/5.1/bpy.types.NodeSocketStringFilePath.html

On that last one, the manual documents six such nodes:

> "Import Nodes
>
> Nodes used to import data from external file formats. […]
>
> CSV (.csv)
> Wavefront (.obj)
> Stanford PLY (.ply)
> STL (.stl)
> Text (.txt)
> OpenVDB (.vdb)"

Source: manual. https://docs.blender.org/manual/en/5.1/modeling/geometry_nodes/input/import/index.html — read 2026-07-30. Each has a "Path" input; e.g. the VDB node's is documented as "The path to the VDB file to be imported" (https://docs.blender.org/manual/en/5.1/modeling/geometry_nodes/input/import/vdb.html).

**Sequencer and clip references:**

- `MovieStrip.filepath` — bare description. https://docs.blender.org/api/5.1/bpy.types.MovieStrip.html
- `ImageStrip.directory` — bare description. https://docs.blender.org/api/5.1/bpy.types.ImageStrip.html
- `StripProxy.directory` — "Location to store the proxy files" — and `StripProxy.filepath` — "Location of custom proxy file". https://docs.blender.org/api/5.1/bpy.types.StripProxy.html
- `SequenceEditor.proxy_dir` — bare description. https://docs.blender.org/api/5.1/bpy.types.SequenceEditor.html
- `MovieClipProxy.directory` — "Location to store the proxy files". https://docs.blender.org/api/5.1/bpy.types.MovieClipProxy.html

**Output references (places the file writes to, recorded in the file):**

- `RenderSettings.filepath` — "Directory/name to save animations, # characters define the position and padding of frame numbers (default "", never None, blend relative // prefix supported, Supports template expressions)". https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html
- `BakeSettings.filepath` — "Image filepath to use when saving externally". https://docs.blender.org/api/5.1/bpy.types.BakeSettings.html
- `CollectionExport.filepath` — "The file path used for exporting". https://docs.blender.org/api/5.1/bpy.types.CollectionExport.html

**One packed-data leftover:** `ImagePackedFile.filepath` — bare description, on a type the API describes no further. https://docs.blender.org/api/5.1/bpy.types.ImagePackedFile.html *(Citation check 2026-08-05: a gloss reading "this records where a packed image came from" was removed — the page does not say it.)*

(I excluded `PreferencesFilePaths` from this list — its five hits, `font_directory`, `texture_directory`, `sound_directory`, `render_cache_directory`, `render_output_directory`, are Preferences, not contents of a `.blend`.)

### 1d. Things a file depends on that are *not* recorded as a path in it

These are easy to miss because they will never appear in `blend_paths()`, and I found three in the sources.

**An OpenColorIO configuration.** The API documents a blend-file-level struct whose whole purpose is reporting this dependency:

> "class bpy.types.BlendFileColorspace(bpy_struct)
>
> Information about the color space used for data-blocks in a blend file
>
> is_missing_opencolorio_config
>
> A color space, view or display was not found, which likely means the OpenColorIO config used to create this blend file is missing (default False, readonly)"

Source: Python API documentation. https://docs.blender.org/api/5.1/bpy.types.BlendFileColorspace.html — read 2026-07-30.

And the manual says the configuration is located by environment variable, not by a path in the file:

> "Blender will use the standard OCIO environment variable to read an OpenColorIO configuration other than the default Blender one."

> "There is also a BLENDER_OCIO environment variable to change the configuration for Blender only."

Source: manual. https://docs.blender.org/manual/en/5.1/render/color_management/opencolorio.html — read 2026-07-30.

**An OSL shader looked up by bare module name in a search path.** The Script node's External mode:

> "The third option is to specify just the module name, which will be looked up in the shader search path.
>
> The shader search path is located in the same place as the scripts or configuration path, under:
>
> Linux:
> $HOME/.config/blender/5.1/shaders/
>
> Windows:
> C:\Users\$user\AppData\Roaming\Blender Foundation\Blender\5.1\shaders\
>
> macOS:
> /Users/$USER/Library/Application Support/Blender/5.1/shaders/"

Source: manual. https://docs.blender.org/manual/en/5.1/render/shader_nodes/utilities/script.html — read 2026-07-30.

The same mechanism is available for a custom Cycles camera:

> "This enables the selection of a text data-block or external file, similar to the Script node in shaders."

Source: manual. https://docs.blender.org/manual/en/5.1/render/cycles/osl/camera.html — read 2026-07-30.

**An asset catalog definition file.** Assets in a blend-file store a catalog identifier; the catalog itself lives in a separate text file in the asset library:

> "Asset catalogs are stored in Catalog Definition Files (CDFs). Blender 3.0 supports a single CDF per asset library. It is stored in blender_assets.cats.txt in the root directory of the asset library."

and, on the same page, an explicit acknowledgement that this dependency can break:

> "Each catalog has an optional simple name. This name is stored along with the UUID in each asset. The purpose is to make it possible for humans to recognize the catalog the asset was assigned to, even when the catalog definition file (see below) is lost."

Source: manual. https://docs.blender.org/manual/en/5.1/files/asset_libraries/catalogs.html — read 2026-07-30.

### 1e. Two supporting cross-checks

**Physics disk caches** get a location by convention rather than by a recorded path, unless the user sets one:

> "When Disk Cache is checked, Blender will save the cache separately to the drive in a folder named blendcache_[filename] alongside the blend-file. (The blend-file must be saved first.)"

and, for the recorded-path case:

> "External
>
> Allows you to read the cache from a drive using a user-specified file path."

Source: manual. https://docs.blender.org/manual/en/5.1/physics/baking.html — read 2026-07-30.

**The Find Missing Files operator's file-type filters** name the kinds of external file Blender's own recovery tool expects to encounter. This is weaker evidence — they are File Browser filter flags, not a statement about dependencies — but it lines up with the census. The signature is:

> "bpy.ops.file.find_missing_files(*, find_all=False, directory='', hide_props_region=True, check_existing=False, filter_blender=False, filter_backup=False, filter_image=False, filter_movie=False, filter_python=False, filter_font=False, filter_sound=False, filter_text=False, filter_archive=False, filter_btx=False, filter_alembic=False, filter_usd=False, filter_obj=False, filter_volume=False, filter_folder=False, filter_blenlib=False, filemode=9, display_type='DEFAULT', sort_method='')"

with the description "Try to find missing external files".

Source: Python API documentation. https://docs.blender.org/api/5.1/bpy.ops.file.html — read 2026-07-30.

### 1f. What I looked at and ruled out

- **Freestyle style modules.** I checked whether these are an external file reference. They are not: `FreestyleModuleSettings.script` is documented as "Python script to define a style module" with "Type: Text" — a Text data-block, not a path (https://docs.blender.org/api/5.1/bpy.types.FreestyleModuleSettings.html). A Text data-block may in turn have its own `filepath`, which is already in the census.
- **Add-ons and extensions as a dependency.** I searched the manual for `requires the add-on`, `add-on is not enabled`, `missing add-on` and found nothing. The sources do not describe a blend-file recording which add-ons it needs.
- **Application templates.** Searched; these are a Blender-installation concept, not a `.blend` reference.
- **The built-in font.** Text objects do not necessarily point outside at all: "Blender comes with a built-in font by default […] The built-in font is always present and shows in this list as 'Bfont'." (https://docs.blender.org/manual/en/5.1/modeling/texts/properties.html). External fonts are the alternative: "This font system has its own built-in font, but it can use external fonts too, including PostScript Type 1, OpenType and TrueType fonts." (https://docs.blender.org/manual/en/5.1/modeling/texts/introduction.html).
- **Release notes.** I searched all of `release_notes_5.1` for `filepath`, `path`, `relative path`, `template`, `UDIM`. Nothing there adds a new kind of external reference. Two entries touch the area and are worth recording as documented statements about change: "GeoNodes simulation cache files not reported via foreach_path callback." and "Correct path for foreach_path callback of object's particle systems." (https://developer.blender.org/docs/release_notes/5.1/corrective_releases/), and "External Multires .btx filepath not cleared when re-packing, reports as missing even when empty" (https://developer.blender.org/docs/release_notes/5.1/bugfixes/). All read 2026-07-30. These are fixes, and they incidentally confirm that geometry-nodes simulation caches, particle system paths and Multires `.btx` files are all part of the set Blender walks when it enumerates external references.
- **The Outliner picture.** I opened `files_data-blocks_outliner-blender-file-view.png` (from https://docs.blender.org/manual/en/5.1/files/data_blocks.html). It shows the Outliner's display-mode menu (Scenes, View Layer, Sequence, Blender File, Data API, Orphan Data) and a Blender File view listing "Current File" with Brushes, Cameras, Collections, Images, Lights, Materials, Meshes, Objects, Scenes, Screens, Texts, Window Managers, Workspaces, Worlds. **documented**, but it settles nothing about external references — it is a data-block listing of one particular file, and no Libraries entry happens to be present in it. I mention it only so the record is complete.

**Not covered.** This does not settle whether the census is genuinely exhaustive — it is exhaustive over properties whose API description carries the relative-path phrase, and I have shown one property (`Text.filepath`) that is an external reference without that phrase, so there could be others I did not catch. It does not settle which of these Blender actually walks when it packs, reports missing files, or renders — only that each records a location. It does not settle what a reference means at render time, only that it exists. It does not cover formats Blender reads through an importer at import time (OBJ, FBX, and so on), because those become internal data rather than a standing reference. And it does not address the two documented contradictions I found (the data-block table versus the API on Volume and Cache File); I only report them.

**Outside the interface.** Every item above is a property of data stored in the file, not of the window. The Python API is the source for most of the census, and Python is how a file is driven from outside; `bpy.utils.blend_paths()` is documented as returning "a list of paths to external files referenced by the loaded .blend file" with no mention of the interface. The manual documents the render output path being set from the command line (`-o, --render-output <path>`, https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html). **inferred** that the set of things a file can point at is therefore the same when Blender is driven from outside — the step is that a stored property does not change with how Blender was started. The sources nowhere state this outright, and there is one place I would not extend it: the OSL shader search path and the OpenColorIO configuration are found via the user configuration directory and environment variables, both of which a launcher can change, so those two dependencies can resolve differently outside the interface. That last point is **inferred** from the quoted environment-variable and search-path text.

**Operating system.** The kinds themselves are not stated to differ by operating system. Two of the items above are documented with per-system locations: the OSL shader search path is given separately for Linux, Windows and macOS (quoted in 1d), and the manual carries a note "Location of Fonts on Unix — Fonts are typically located under /usr/lib/fonts, or some variant like /usr/lib/X11/fonts, but not always." (https://docs.blender.org/manual/en/5.1/modeling/texts/properties.html). For everything else the sources do not say, and I will not guess.


**Observed — hand test entries 1, 2 and 20, run 2026-08-05 on the Creator's own computer** (MacBook Pro, M1 Pro chip, macOS 26.5.2, Apple's own graphics), with Blender 5.1.2 started from a terminal with no window, using a throwaway settings folder set through `BLENDER_USER_RESOURCES`. The full list of tests is at `Audits/Blender Research/Dependencies/Hand tests.md`.

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

**Something a Mac-written file carries that nothing had noticed.** Opening a blend-file saved on the Creator's Mac on the Linux machine, `bpy.utils.blend_paths` returned, among its paths, `'/Applications/Blender 5.1.2.app/Contents/Resources/5.1/datafiles/assets/brushes/essentials_brushes-gp_draw.blend'` — a path into the Mac's own Blender application, on a machine that has no such folder. It comes out of the file rather than out of the running Blender. Report Missing Files did **not** name it.

**Both standing lines, brought up to date by the Linux tests.** *Driven from outside:* every reading above was taken with Blender running with no window unless it says otherwise, so it holds outside the interface. *Operating system:* **now settled on two operating systems, macOS and Linux, and each is its own result** — what they agree on and where they differ is said above. **Windows is still untried: no Windows machine has run it.**

---

## 5. In what ways can a recorded location be written?

**Ambiguity, flagged rather than silently resolved.** "A recorded location" can be read as (a) a path stored in the file that points at something outside it, or (b) any path field in Blender's interface. I answer reading (a), because the sheet is about what a `.blend` file points at. Where a form only applies to a path the file *writes to* rather than reads from, I say so. Under reading (b) the answer would be the same set plus the Preferences path fields, which behave the same way (the Preferences page says "The default path // refers to the folder of the currently open blend-file", https://docs.blender.org/manual/en/5.1/editors/preferences/file_paths.html).

**LABEL: documented.**

There are six distinct forms, and they can be combined in one string.

### 5.1 Absolute

Defined by the manual only in passing, inside the description of the operator that produces it:

> "Make Paths Absolute
>
> Make all paths to external files absolute (= full path from the system's root)."

Source: manual. https://docs.blender.org/manual/en/5.1/interface/window_system/topbar.html — read 2026-07-30.

The API operator carries the shorter description "Make all paths to external files absolute" (https://docs.blender.org/api/5.1/bpy.ops.file.html).

### 5.2 Relative, written with a leading double slash `//`

This is the default form. The manual's dedicated section:

> "When you specify one of these external files, the default option is to make the path relative. Blender stores a partial path evaluated relative to the directory location of the referencing blend-file."

and the syntax itself:

> "When relative paths are supported, the File Browser provides a Relative Path checkbox, when entering the path into a text field, use a double slash prefix (//) to make it so."

and a stated limit on it:

> "You cannot use relative paths into a new untitled blend-file. Save it before linking to external files."

Source: manual. https://docs.blender.org/manual/en/5.1/files/blend/open_save.html — read 2026-07-30.

The Preferences page gives the same meaning in one line: "The default path // refers to the folder of the currently open blend-file" (https://docs.blender.org/manual/en/5.1/editors/preferences/file_paths.html).

The API states which properties accept it, using the clause `blend relative // prefix supported` on every such property (the census in answer 1 above lists them all), and gives the two conversion calls:

> "bpy.path.abspath(path, *, start=None, library=None)
>
> Returns the absolute path relative to the current blend file using the "//" prefix."

> "bpy.path.relpath(path, *, start=None)
>
> Returns the path relative to the current blend file using the "//" prefix."

Source: Python API documentation. https://docs.blender.org/api/5.1/bpy.path.html — read 2026-07-30.

**Important detail about what `//` is relative to.** It is not always the file you opened. The same API page documents:

> "library (bpy.types.Library | None) – The library this path is from. This is only included for convenience, when the library is not None its path replaces start."

That is, for a path stored on a data-block that came in from a linked library, the starting point is the library file, not the current file. **documented.**

Cycles' own code does exactly this, which is worth recording as an **observed** confirmation of the same rule in a renderer:

> `    script_path = bpy.path.abspath(filepath, library=library)`

Source: Cycles' own files, `Blender Sources/cycles_5.1/osl.py`, line 199, inside the function `update_external_script`. Read 2026-07-30.

> `    osl_path = bpy.path.abspath(script.filepath, library=script.library)`

Source: Cycles' own files, `Blender Sources/cycles_5.1/osl.py`, line 239, inside the function `update_internal_script`. Read 2026-07-30.

The path-template variables carry the same distinction, and the manual spells it out:

> "blend_name_lib:
>
> Like blend_name, except that for path properties on library-linked data-blocks it will be the name of the library blend file that the data-block comes from."

Source: manual. https://docs.blender.org/manual/en/5.1/files/file_paths.html — read 2026-07-30.

**One property is documented as an exception.** `Text.filepath` is described as "Filename of the text file (default "", never None)" — it alone, out of every path-bearing property I found, lacks the `blend relative // prefix supported` clause. Source: Python API documentation, https://docs.blender.org/api/5.1/bpy.types.Text.html — read 2026-07-30. I did not find any page that explains why, and I do not know what it means in practice.

**Writing a relative path is a choice made at several moments**, each documented separately: the File Browser's Relative Path checkbox (above); the Preferences default —

> "Default To – Relative Paths
>
> Default value for Relative Paths when loading external files such as images, sounds, and linked libraries. It will be ignored if a path is already set."

(https://docs.blender.org/manual/en/5.1/editors/preferences/save_load.html); the Save As option —

> "Remap Relative
>
> This option remaps Relative Paths (such as linked libraries and images) when saving a file in a new location."

(https://docs.blender.org/manual/en/5.1/files/blend/open_save.html); and, when writing a library from Python, a four-way choice:

> "path_remap (str) –
> Optionally remap paths when writing the file:
>
> NONE No path manipulation (default).
>
> RELATIVE Remap paths that are already relative to the new location.
>
> RELATIVE_ALL Remap all paths to be relative to the new location.
>
> ABSOLUTE Make all paths absolute on writing."

Source: Python API documentation. https://docs.blender.org/api/5.1/bpy.types.BlendDataLibraries.html — read 2026-07-30. All read 2026-07-30.

### 5.3 With `#` characters standing for the frame number

This applies to output paths.

> "When rendering an animation, the frame number is appended at the end of the file name with four padded zeros (e.g. image0001.png). You can set a custom padding size by adding the appropriate number of # anywhere in the file name (e.g. image_##_test.png translates to image_01_test.png)."

Source: manual. https://docs.blender.org/manual/en/5.1/render/output/properties/output.html — read 2026-07-30.

The API says the same on the property: "Directory/name to save animations, # characters define the position and padding of frame numbers" (https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html).

The File Output node uses the same device, with a note that it also affects single renders:

> "Both the File Name and the Image names (when saving multiple images) can contain #### to force the frame number to appear in a specific place in the filename. This also has the effect that a single render will have the frame number embedded. The frame number will be padded with zeros to fill the number of # characters."

Source: manual. https://docs.blender.org/manual/en/5.1/compositing/types/output/file_output.html — read 2026-07-30.

The command-line page documents the fallback when no `#` is written:

> "When the filename does not contain #, the suffix #### is added to the filename."

Source: manual. https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html — read 2026-07-30.

### 5.4 With `<UDIM>` or `<UVTILE>` substitution tokens

This applies to image paths.

> "File Substitution Tokens
>
> Substitution tokens are special sequences of characters in a filename that can be replaced with more meaningful and context aware information. In this case, tokens are identified by being text wrapped in angle bracket characters.
>
> This substitution is used while load loading or saving an image to automatically identify the tile associated with a particular texture in the UDIM array.
>
> The following tokens are supported:
>
> <UDIM>: A 4-digit notation calculated as 1001 + u-tile + v-tile * 10.
>
> <UVTILE>: A notation defined as u(u-tile + 1)_v(v-tile + 1).
>
> Examples:
>
> monster-basecolor.<UDIM>.png will load/save files like monster-basecolor.1021.png etc.
>
> monster-basecolor.<UVTILE>.png will load/save files like monster-basecolor.u1_v3.png etc."

Source: manual. https://docs.blender.org/manual/en/5.1/modeling/meshes/uv/workflows/udims.html — read 2026-07-30.

Note the wording "while loading or saving" — this form works in both directions, not only on output.

### 5.5 With path-template expressions in curly braces

This is a whole documented syntax of its own, and it is currently restricted.

> "Path templates substitute template expressions (written {abcd}, where "abcd" is a variable name) in a filepath when the path is used."

> "For example, if the currently open blend file is dance.blend and the render output path is set to this:
>
> //my_render_dir/{blend_name}.png
>
> Then when rendering, Blender will treat the path as:
>
> //my_render_dir/dance.png"

Crucially, on how it is *stored*:

> "This substitution happens internally at time of use (e.g. while rendering), so the template syntax will remain as-is in the path field."

The restriction, stated as a Note on the page:

> "Currently path templates are only supported for the render output path in Scene properties and the output paths in the compositor's File Output node. More filepaths will support templates in future releases."

The variables, quoted from the page's own headings:

Available in all paths that support template expressions — `blend_name` ("The currently open blend file's name (without the .blend)"), `blend_dir` ("Path up to (but not including) the currently open blend file"), `blend_name_lib`, `blend_dir_lib`.

Available only in render output paths (including in the compositor File Output node) — `fps`, `resolution_x` / `resolution_y`, `scene_name` ("Name of the current scene"), `camera_name` ("Name of the current render camera").

Available only in path properties on a node — `node_name` ("The name of the node that the path property is on").

Format specifiers, written after a colon:

> "dance_{fps:###}.png: format as an integer with at least 3 digits.
>
> dance_{fps:.###}.png: format as a floating point number with exactly 3 digits after the decimal point.
>
> dance_{fps:###.##}.png: format as a floating point number with at least 3 digits for the integer part and exactly 2 digits for the fractional part."

Escapes:

> "{{ translates to a single { in the final path.
>
> }} translates to a single } in the final path."

with the worked example "//my_render_{{dir}}/{blend_name}.png -> //my_render_{dir}/dance.png".

And what a malformed one does:

> "Paths that support templates can have template errors, which prevent the path from being processed."

> "Depending on the path, template errors may prevent certain actions. For example, if there are errors in the render output path, then rendering an animation will fail with an error message indicating the path errors."

Source for all of the above: manual. https://docs.blender.org/manual/en/5.1/files/file_paths.html — read 2026-07-30.

I opened the page's second picture, `_images/files_file_paths-invalid_path_template_tooltip.png`. **documented**, from the picture: it shows the Output panel's path field containing `//my_render_dir/{blend_name.png`, drawn with a red background, and a tooltip headed "Output Path" reading "Directory/name to save animations, # characters define the position and padding of frame numbers.", then "Value: //my_render_dir/{blend_name.png", then in red "Syntax error(s):" and "- Invalid or incomplete template expression '{blend_name.png'." What the picture shows and the text does not: the `//` prefix, the `{}` template and the `#` convention all live in the same single field, and the error is reported per-expression with the offending text quoted back.

The API marks the supported properties with the clause `Supports template expressions`. I searched all 2,104 API files for that phrase; there are exactly three hits, spread over two objects:

- `RenderSettings.filepath` — "Directory/name to save animations, # characters define the position and padding of frame numbers (default "", never None, blend relative // prefix supported, Supports template expressions)". https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html
- `CompositorNodeOutputFile.directory` — "The directory where the image will be written (default "", never None, blend relative // prefix supported, Supports template expressions)".
- `CompositorNodeOutputFile.file_name` — "The base name of the file. Other information might be included in the final file name depending on the node options (default "", never None, Supports template expressions)". Note that this one supports templates but does **not** carry the relative-path clause. https://docs.blender.org/api/5.1/bpy.types.CompositorNodeOutputFile.html

Both read 2026-07-30. This matches the manual's Note exactly.

### 5.6 As a bare module name, with no directory part

This form is documented only for OSL shaders, and it is a genuinely different way of writing a location — the file is found by search rather than by path.

> "The third option is to specify just the module name, which will be looked up in the shader search path."

Source: manual. https://docs.blender.org/manual/en/5.1/render/shader_nodes/utilities/script.html — read 2026-07-30.

Cycles' own code implements exactly that, and the condition it tests is worth quoting because it makes the rule precise — the form is recognised by the *absence* of a directory component:

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

Source: Cycles' own files, `Blender Sources/cycles_5.1/osl.py`, lines 217–224, inside the function `update_external_script`. Read 2026-07-30. **observed** (this is what the file says). *(Citation check 2026-08-05: the line range read 214–221; the quoted branch sits at 217–224.)* The step to "therefore a render will find the shader there" would be **inferred**, and I do not take it.

### 5.7 One more thing that shapes a written location, though it is not a form of its own

In the File Output node, a slash inside an *input socket name* becomes part of the path:

> "If an input name contains a slash character (/ or \), the path is interpreted as a directory hierarchy.
>
> Any missing directories are created automatically during output.
>
> This allows organizing outputs into folders directly from the node setup, for example: passes/diffuse or masks/object_01."

Source: manual. https://docs.blender.org/manual/en/5.1/compositing/types/output/file_output.html — read 2026-07-30. Note that both slash characters are accepted here, on any system, per the quoted words.

### 5.8 What I searched for and did not find

Recording these so the absences are real answers rather than gaps.

- **Environment variables inside a recorded path.** I searched `manual_5.1` and `api_5.1` for `environment variable`, `expanduser`, `%APPDATA%`, `~/`, `home directory`, and for `$` within the file-handling and Preferences sections. Environment variables appear in three unrelated roles — locating Blender's own directories (`BLENDER_USER_RESOURCES` and relatives, https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html), locating the OpenColorIO config (`OCIO`, `BLENDER_OCIO`), and as `$filepath` / `$line` / `$column` placeholders in the Preferences setting for an external text editor command. **Not in the sources** that a path stored in a `.blend` may contain an environment variable or a `~` home shorthand.
- **UNC paths, drive letters, and network locations.** I searched both source sets for `UNC`, `drive letter`, `network drive`, `C:\`, `\\`. The only near-hits are the FBX and OBJ exporters' path-mode option, which mentions "except when on a different drive on Windows" (https://docs.blender.org/manual/en/5.1/files/import_export/obj.html), and Blender-installation paths on the command-line launch page. **Not in the sources** for how a recorded location is written in these forms.
- **Which slash character is stored.** I searched for `path separator`, `forward slash`, `slash character`, `backslash`, `os.sep`. The only relevant statement is the API's `bpy.path.native_pathsep(path)` — "Replace the path separator with the system's native os.sep." (https://docs.blender.org/api/5.1/bpy.path.html), which tells you a conversion exists but not what is stored. **Not in the sources** which separator a `.blend` records.
- **Going up a level with `//../`.** I searched both source sets for `//..`. No hits. **Not in the sources.**
- **Release notes.** Searched all of `release_notes_5.1` for `path template`, `template`, `relative path`, `filepath`, `UDIM`. Path templates are not announced in the 5.1 release notes; the only 5.1 note touching them is a fix, "Crash when opening a file browser for path property with template variables" (https://developer.blender.org/docs/release_notes/5.1/bugfixes/, read 2026-07-30). Also there: "Bpy.data.libraries.write with path_remap='RELATIVE_ALL' does not recalculate existing relative paths" and "Node Wrangler Image Sequence Texture Node doesn't work with relative paths". All three are documented statements that these forms exist and were being corrected in 5.1; none of them changes the list of forms.

**Not covered.** This settles the forms in which a location may be *written*, and in some cases when the substitution happens, but not what any of them resolves to at render time on a given machine — the only substitution timing the sources state outright is the templates' ("This substitution happens internally at time of use"). It does not settle what character Blender actually stores as a separator, nor how an absolute path is spelled on Windows. It does not settle whether the `<UDIM>` / `<UVTILE>` tokens and the `{}` templates can be mixed in one path, since no page shows that. It does not settle the meaning of `Text.filepath` lacking the relative-path clause. And it does not settle which of these forms survive the Make Paths Relative / Make Paths Absolute operators unchanged.

**Outside the interface.** Three of the six forms are documented working from the command line, in one place, which is the strongest evidence available:

> "-o, --render-output <path>
>
> Set the render path and file name. Use // at the start of the path to render relative to the blend-file.
>
> You can use path templating features such as {blend_name} in the path. See Blender's documentation on path templates for more details.
>
> The # characters are replaced by the frame number, and used to define zero padding."

with a worked example, "blender -b animation.blend -o //render_ -F PNG -x 1 -a" and "//render_ becomes //render_####, writing frames as //render_0001.png".

Source: manual. https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html — read 2026-07-30. **documented** for `//`, `{}` and `#` outside the interface.

One documented difference at the edge, and it is about ordering rather than syntax: "Arguments are executed in the order they are given", and specifically "blender --background --render-output /tmp test.blend --render-frame 1 …will not render to /tmp because loading the blend-file overwrites the render output that was set." (same page). So a path supplied outside the interface can be silently replaced by the one recorded in the file, depending on argument order.

Two forms have no statement either way outside the interface: `<UDIM>` / `<UVTILE>`, and the bare OSL module name. The Relative Path checkbox is by its nature a File Browser control, but the underlying `//` prefix is a text form the manual explicitly says you can type ("when entering the path into a text field, use a double slash prefix (//) to make it so"), and the Python API exposes both conversions and the four-way `path_remap` choice. **The sources do not say** whether anything else differs.

**Operating system.** The sources do not say that any of these six forms differ by operating system, and I found no page that qualifies `//`, `#`, `<UDIM>`, `<UVTILE>` or `{}` by system. Two adjacent facts are stated per-system: the OSL shader search path that form 5.6 relies on is given separately for Linux, Windows and macOS (quoted in answer 1), so a bare module name resolves in a different directory on each; and the OBJ and FBX exporters' relative-path option carries the qualifier "Uses relative paths in every case (except when on a different drive on Windows)" (https://docs.blender.org/manual/en/5.1/files/import_export/obj.html, read 2026-07-30) — which is about an exporter writing paths into a non-`.blend` file, not about a `.blend`, but is the only statement in these sources linking relative paths to a Windows-specific limit. The existence of `bpy.path.native_pathsep` and of `bpy.path.basename` — "Equivalent to os.path.basename, but skips a "//" prefix. Use for Windows compatibility." (https://docs.blender.org/api/5.1/bpy.path.html) — shows Blender treats separators as a per-system concern, but neither says what is stored. Beyond that, **the sources do not say**.


**Observed — hand test entries 3, 14 and 15, run 2026-08-05 on the Creator's own computer** (MacBook Pro, M1 Pro chip, macOS 26.5.2, Apple's own graphics), with Blender 5.1.2 started from a terminal with no window, using a throwaway settings folder set through `BLENDER_USER_RESOURCES`. The full list of tests is at `Audits/Blender Research/Dependencies/Hand tests.md`.

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


**Observed — hand test entry 24, run by the Creator on 2026-08-05 on their own computer** (MacBook Pro, M1 Pro chip, macOS 26.5.2, Apple's own graphics), in Blender 5.1.2 with its window open. This is the half of the list that needs eyes on the screen; the whole list is at `Audits/Blender Research/Dependencies/Hand tests.md`. Because it was run on a Mac, it settles macOS and leaves Windows and Linux open.

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

In this version's screenshots a floating panel headed `Report: Error` also sits over the render window carrying the same two lines. No such panel is visible in the shots for 4.5 or 5.0; whether that is a difference or a matter of when the picture was taken is not reported.

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

2026-08-04 — Opus 5 (Claude Code) — Dependencies, Blender 5.1

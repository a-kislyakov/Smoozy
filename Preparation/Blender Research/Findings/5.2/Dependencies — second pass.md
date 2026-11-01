# Dependencies — Blender 5.2 — second pass

**Sources consulted, and the date.** Blender 5.2 manual (`Blender Sources/manual_5.2`, including its `_images` folder), Blender 5.2 Python API documentation (`Blender Sources/api_5.2`), Blender 5.2 release notes (`Blender Sources/release_notes_5.2`), and Cycles' own add-on files (`Blender Sources/cycles_5.2`). All read on **2026-07-30** (the snapshot date given in the inputs), which is the date read cited throughout.

**I read only the question sheet and files inside `Blender Sources/`. I did not open any other part of this project.**

This pass answers questions 1, 2, 3, 4, 5, 10, 11, 12, 13, 14, 15 and 16 only. Questions 6, 7, 8 and 9 were not assigned and are not answered here.

Two helper agents were used, one for questions 15 and 16 and one for questions 10 and 12. Each was given the same restriction, word for word, and each declared in writing that it read only the question sheet and files inside `Blender Sources/`, opened no other version folder, used no web source, and started no helpers of its own. Their records of where they looked are reproduced in their answers below. I vouch for both.

---

## 1. What kinds of thing can a `.blend` file point at outside itself? Give the full list.

**LABEL: documented** for the list below. Each item is quoted from a source that names it as an external file reference.

The sources do not give a single sentence saying "here is the complete list". They give three overlapping enumerations, and the list below is the union of them. Where I have joined the three, that joining is the only step I have taken; every item is quoted.

### The general statement

> "External data, like texture images and other resources,
> can be stored inside the blend-file (packed) or as separate files (unpacked).
> Blender keeps track of all unpacked resources via a relative or absolute path."

— manual, https://docs.blender.org/manual/en/5.2/interface/window_system/topbar.html (read 2026-07-30)

> "Resource
>
> External files such as images, sounds, fonts and volumes files that can be packed into a blend-file."

— manual, https://docs.blender.org/manual/en/5.2/glossary/index.html (read 2026-07-30)

And the machine-readable way to get the list out of a given file:

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
> local (bool) – When true skip linked library paths."

— Python API, https://docs.blender.org/api/5.2/bpy.utils.html (read 2026-07-30)

> "file_path_map(*, subset=None, key_types=None, include_libraries=False)
>
> Returns a mapping of all ID data-blocks in current bpy.data to a set of all file paths used by them."

— Python API, https://docs.blender.org/api/5.2/bpy.types.BlendData.html (read 2026-07-30)

### Enumeration A — the data-block table

The manual's data-block table has a "Pack" column defined as:

> "Pack
>
> File Packing, supports file contents being packed into the blend-file
> (not applicable for most data-blocks which have no file reference)."

The rows that name a file reference:

> "Cache File … Used by Mesh Cache modifiers."
> "Font … References font files. Used by curve object-data of text objects."
> "Image … Image files. Used by shader nodes and textures."
> "Library … References to an external blend-file. Access from the Outliner's Blender File view."
> "Movie Clip … Reference to an image sequence or video file. Used in the Movie Clip editor."
> "Sound … Reference to sound files. Used as data of speaker objects."
> "Text … Text data. Used by Python scripts and OSL shaders."

— manual, https://docs.blender.org/manual/en/5.2/files/data_blocks.html (read 2026-07-30)

### Enumeration B — every API type that carries a blend-relative file path

I searched the whole of `api_5.2` for the exact phrase "blend relative // prefix supported", which is how the API documentation marks a property that holds an external location. Forty-one pages carry it. Grouped by what they point at, with the description quoted:

Read from:

- `Library.filepath` — "Path to the library .blend file"
- `LibraryWeakReference.filepath` — "Path to the library .blend file"
- `Image.filepath` / `Image.filepath_raw` — "Image/Movie file name"
- `ImagePackedFile.filepath`
- `VectorFont.filepath` (fonts)
- `Sound.filepath` — "Sound sample file used by this Sound data-block"
- `MovieClip.filepath` — "Filename of the movie or sequence file"
- `Volume.filepath` — "Volume file used by this Volume data-block"
- `CacheFile.filepath` and `CacheFileLayer.filepath` — "Path to the archive"
- `MeshCacheModifier.filepath` and `MultiresModifier.filepath` — "Path to external displacements file"
- `ShaderNodeScript.filepath` — "Shader script path"
- `ShaderNodeTexIES.filepath` — "IES light path"
- `ImageStrip.directory`, `MovieStrip.filepath` (sequencer strips)
- `NodeSocketStringFilePath.default_value` and `NodeTreeInterfaceSocketStringFilePath` — a node socket whose value is a file path
- `BlendDataFonts.load`, `BlendDataImages.load`, `BlendDataMovieClips.load`, `BlendDataSounds.load`, `BlendDataTexts.load` — the loaders

Written to (see question 10):

- `RenderSettings.filepath` — "Directory/name to save animations"
- `CompositorNodeOutputFile.directory` — "The directory where the image will be written"
- `BakeSettings.filepath` — "Image filepath to use when saving externally"
- `NodesModifier.bake_directory` and `NodesModifierBake.directory` — "Location on disk where the bake data is stored"
- `PointCache.filepath` — "Cache file path", and `PointCacheItem`
- `FluidDomainSettings.cache_directory` — "Directory that contains fluid cache files"
- `DynamicPaintSurface.image_output_path` — "Directory to save the textures"
- `OceanModifier.filepath` — "Path to a folder to store external baked images"
- `StripProxy.directory` / `StripProxy.filepath`, `MovieClipProxy.directory`, `SequenceEditor.proxy_dir` — "Location to store the proxy files"
- `CollectionExport.filepath` — "The file path used for exporting"
- `PreferencesFilePaths` — the preferences, not stored in the file

— all Python API, addresses of the form `https://docs.blender.org/api/5.2/bpy.types.<Name>.html` (read 2026-07-30)

One type carries a path without the `//` marker:

> "filepath
>
> Filename of the text file (default "", never None)"

— `bpy.types.Text`, https://docs.blender.org/api/5.2/bpy.types.Text.html (read 2026-07-30)

### Enumeration C — the geometry-node import nodes

These reference a file at evaluation time, through a socket rather than a fixed property:

> "Import Nodes
>
> Nodes used to import data from external file formats.
>
> Geometry Nodes supports automatically creating import nodes by dragging supported file types into the node editor.
>
> When dropping a supported file into a Geometry Node tree,
> the corresponding import node is added automatically and configured with the file path.
>
> CSV (.csv)
>
> Wavefront (.obj)
>
> Stanford PLY (.ply)
>
> STL (.stl)
>
> Text (.txt)
>
> OpenVDB (.vdb)"

— manual, https://docs.blender.org/manual/en/5.2/modeling/geometry_nodes/input/import/index.html (read 2026-07-30)

**Not covered.** The three enumerations do not agree perfectly and I have not resolved the disagreements. The data-block table marks Cache File's Pack column with "—", while `CacheFile.filepath` exists in the API. The table's key does not say "—" means the type has no file reference; it says packing is "not applicable for *most* data-blocks which have no file reference", so the two sit awkwardly rather than flatly contradicting. I report both and do not choose. *(Citation check 2026-08-05: the key was being quoted as saying something it does not say.)* The list also does not settle what a `.blend` points at *transitively* — a linked library brings its own references (question 13). It does not cover things a file points at that are not files at all (a URL for an online asset library is a preference, not a file property). And an "external file reference" in the API sense includes paths Blender writes to as well as reads from; I have split them above but the sources do not draw that line themselves.

**Outside the interface.** Holds. `bpy.utils.blend_paths` and `BlendData.file_path_map` / `file_path_foreach` are Python functions with no stated interface dependency, and the release notes describe the path iterator as being for exactly this use:

> "Three new options for `bpy.data.file_path_foreach` so render farms and other scripts can find all relevant image files for rendering, including the new Cycles texture cache."

— release notes, https://developer.blender.org/docs/release_notes/5.2/python_api/ (read 2026-07-30)

**Operating system.** The sources do not say. Nothing in any of the three enumerations is qualified by operating system.


**Observed — hand test entries 1, 2 and 20, run 2026-08-05 on the Creator's own computer** (MacBook Pro, M1 Pro chip, macOS 26.5.2, Apple's own graphics), with Blender 5.2.0 LTS started from a terminal with no window, using a throwaway settings folder set through `BLENDER_USER_RESOURCES`. The full list of tests is at `Audits/Blender Research/Dependencies/Hand tests.md`.

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

**Which machinery this version has.** Checked by name in all four versions. Present here and absent in 4.5: `bpy.data.file_path_foreach`, `Library.is_archive`, `Library.archive_libraries`, `bpy.data.colorspace` (`bpy.data.colorspace.is_missing_opencolorio_config` returned `False`). `WindowManager.reports` is present here and absent in 4.5, 5.0 and 5.1. Present in all four: `bpy.data.file_path_map`, `bpy.utils.blend_paths`, `bpy.ops.file.pack_libraries`, `bpy.ops.file.pack_all`, `bpy.ops.file.report_missing_files`, `bpy.ops.file.find_missing_files`, `bpy.ops.outliner.id_linked_relocate`, `Library.parent`, `ID.is_missing`, `ID.library_weak_reference`, `bpy.path.resolve_ncase`.

**One item on the list is not called the same thing in every version.** The properties of `bpy.types.CompositorNodeOutputFile` that are not already on `bpy.types.Node` are, in this version:

```
['file_output_items', 'active_item_index', 'directory', 'file_name', 'format', 'save_as_render', 'use_file_extension']
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

**Something a Mac-written file carries that nothing had noticed.** Opening a blend-file saved on the Creator's Mac on the Linux machine, `bpy.utils.blend_paths` returned, among its paths, `'/Applications/Blender 5.2.0.app/Contents/Resources/5.2/datafiles/assets/brushes/essentials_brushes-gp_draw.blend'` — a path into the Mac's own Blender application, on a machine that has no such folder. It comes out of the file rather than out of the running Blender. Report Missing Files did **not** name it.

**Both standing lines, brought up to date by the Linux tests.** *Driven from outside:* every reading above was taken with Blender running with no window unless it says otherwise, so it holds outside the interface. *Operating system:* **now settled on two operating systems, macOS and Linux, and each is its own result** — what they agree on and where they differ is said above. **Windows is still untried: no Windows machine has run it.** **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

---

## 2. What kinds of library can a file point at, including one that does not sit on the same computer?

**LABEL: documented.**

The word "library" carries two distinct meanings in the sources, and the answer differs sharply between them. I answer both, because the question's phrase "does not sit on the same computer" only applies to the second.

### Library in the linking sense — another `.blend` file, and only that

> "class bpy.types.Library(ID)
>
> External .blend file from which data is linked"

> "filepath
>
> Path to the library .blend file (default "", never None, blend relative // prefix supported)"

— Python API, https://docs.blender.org/api/5.2/bpy.types.Library.html (read 2026-07-30)

> "Library
>
> ✗
>
> ✓
>
> References to an external blend-file.
> Access from the Outliner's Blender File view."

— manual, https://docs.blender.org/manual/en/5.2/files/data_blocks.html (read 2026-07-30)

There is one further kind of `Library` in 5.2, generated by Blender rather than chosen by the user:

> "is_archive
>
> This library is an 'archive' storage for packed linked IDs originally linked from its 'archive parent' library. (default False, readonly)"

> "archive_libraries
>
> Archive libraries of packed IDs, generated (and owned) by this source library (default None, readonly)"

— Python API, https://docs.blender.org/api/5.2/bpy.types.Library.html (read 2026-07-30)

### Library in the asset sense — a local directory, or a URL

> "An asset library is a collection of Blender assets that is registered in the Preferences as an asset library."
>
> "Asset libraries may be stored locally as directories containing blend-files
> or hosted online and downloaded on demand."

> "Unlike local asset libraries, online libraries are identified by a URL
> instead of a directory path and do not require manually managing asset files on disk."

— manual, https://docs.blender.org/manual/en/5.2/files/asset_libraries/introduction.html (read 2026-07-30)

> "path
>
> Path to a directory with .blend files to use as an asset library (default "", never None)"
>
> "remote_url
>
> Remote URL to the asset library (default "", never None)"
>
> "use_remote_url
>
> Synchronize the asset library with a remote URL (default False, readonly)"

— Python API, https://docs.blender.org/api/5.2/bpy.types.UserAssetLibrary.html (read 2026-07-30)

"Remote" is defined, and it is a network connection, not necessarily a distant machine:

> "A Remote Asset Library is an Asset Library with a few extra files, stored on a server so that it can be accessed remotely.
>
> In this case, "Remote" means "reached through a network connection". It can be served from your own computer, it doesn't have to be far away."

— manual, https://docs.blender.org/manual/en/5.2/files/asset_libraries/remote_asset_libraries.html (read 2026-07-30)

This is new in 5.2:

> "## Online Asset Libraries
>
> It is now possible to register remotely hosted asset libraries, browse them from within Blender, and download assets as needed. This requires _Allow Internet Access_ to be enabled in the Preferences."

— release notes, https://developer.blender.org/docs/release_notes/5.2/assets/ (read 2026-07-30)

### The joint that matters: a file cannot point at a remote library

The sources state twice, plainly, that an online asset library cannot be the target of a link. What ends up in the file is either an appended copy or a packed one:

> "Assets cannot be linked directly and must instead
> be appended or packed."
>
> "The library contents depend on the availability of the remote server.
>
> An internet connection is required to download assets that are not already cached locally."

— manual, https://docs.blender.org/manual/en/5.2/files/asset_libraries/introduction.html (read 2026-07-30)

> "Note
>
> Linking is not supported for online asset libraries."

— manual, https://docs.blender.org/manual/en/5.2/editors/preferences/asset_libraries.html (read 2026-07-30)

And where a downloaded asset lands:

> "Assets are downloaded to the local cache directory."

— manual, https://docs.blender.org/manual/en/5.2/files/asset_libraries/remote_asset_libraries.html (read 2026-07-30)

> "The cache directory is used to store persistent caches locally. Currently it is only used for the indexing of
> Asset Libraries. The operating system is not expected to clear this automatically.
>
> The following path will be used:
>
> Linux:
>
> $XDG_CACHE_HOME/blender/ if $XDG_CACHE_HOME is set, otherwise $HOME/.cache/blender/
>
> macOS:
>
> /Library/Caches/Blender/
>
> Windows:
>
> %USERPROFILE%\AppData\Local\Blender Foundation\Blender\Cache\"

— manual, https://docs.blender.org/manual/en/5.2/advanced/blender_directory_layout.html (read 2026-07-30)

A remote library is also required to be self-contained:

> "Each .blend file should be independent. Any external dependency, such as .png files for textures, must be packed into the blend file."

— manual, https://docs.blender.org/manual/en/5.2/files/asset_libraries/remote_asset_libraries.html (read 2026-07-30)

**Not covered.** Whether a linked-library path may point at a file on a network share or mounted volume. I searched the manual for `UNC`, `network share`, `network drive`, `shared drive`, `networked`, `drive letter`, and `C:\` and found nothing about path forms of that kind — the only `network drive` hit is `advanced/deploying_blender.txt`, about installing Blender itself on "a read-only network drive or in a system directory", not about library paths. So: the sources are silent on whether a library may live on another computer via the file system. What they *do* settle is that the only network-addressed library form Blender offers (an online asset library, by URL) explicitly cannot be linked.

The answer also does not settle what the archive-library mechanism does at file level, only that it exists.

**Outside the interface.** Partly. The remote-library tooling is documented as command-line driven:

> "$ blender -b -c asset_listing generate /path/to/asset-library"

— manual, https://docs.blender.org/manual/en/5.2/files/asset_libraries/remote_asset_libraries.html (read 2026-07-30)

Linking to a `.blend` library is available from Python via `bpy.data.libraries.load(filepath, link=True)` with no stated interface dependency. But asset libraries are registered in the Preferences, not in the file, so which libraries a machine knows about is a property of that machine, not of the file. The sources do not say what happens to an online asset library when Blender runs without internet access beyond "An internet connection is required to download assets that are not already cached locally."

**Operating system.** The local cache directory differs per operating system, quoted above. Nothing else in this answer is qualified by operating system.


**Observed — hand test entry 1, run 2026-08-05 on the Creator's own computer** (MacBook Pro, M1 Pro chip, macOS 26.5.2, Apple's own graphics), with Blender 5.2.0 LTS started from a terminal with no window, using a throwaway settings folder set through `BLENDER_USER_RESOURCES`. The full list of tests is at `Audits/Blender Research/Dependencies/Hand tests.md`.

**What was done.** `Library.is_archive` and `Library.archive_libraries` were looked for by name, in all four versions.

**What came back.** Both exist here, and in the other 5.x versions. Neither exists in 4.5.12.

**Both standing lines, brought up to date by this test.** *Driven from outside:* the reading was taken with no window. *Operating system:* macOS for this test. The same test was run on a rented Linux machine on 2026-08-06 and that result is recorded below; Windows stays untested.

**What this does not settle, and cannot be settled here.** Whether a library that does not sit on the same computer can be reached. That needs a second computer and a shared place both can reach; the Creator has one computer and no share, so it is in the *cannot be run* section of `Audits/Blender Research/Dependencies/Hand tests.md` and stays unanswered. The one adjacent thing that was tested is in question 7 below: a path naming a volume that is not mounted is stored and resolved unchanged, and the file is simply not found.


**Observed — run again on Linux, 2026-08-06.** The two archive-library names were asked for again in this version on the rented Linux machine, and came back exactly as on the Creator's Mac: absent in 4.5, present in 5.0, 5.1 and 5.2. Nothing about which kinds of library exist differs between the two machines.

**The half about a library that does not sit on the same computer is still not settled**, and now for a narrower reason: there are two machines but no folder on network storage that both can reach. What the Linux machine does with a path naming a volume that is not mounted is the same as the Mac: `/Volumes/NoSuchShare/tex.png` is kept exactly as written, resolved exactly as written, and the file is not found. Report Missing Files names it.

**Both standing lines, brought up to date by the Linux tests.** *Driven from outside:* every reading above was taken with Blender running with no window unless it says otherwise, so it holds outside the interface. *Operating system:* **now settled on two operating systems, macOS and Linux, and each is its own result** — what they agree on and where they differ is said above. **Windows is still untried: no Windows machine has run it.**

---

## 3. Can a file hold the things it depends on inside itself partly rather than wholly, and what does a partly-held file report when it is read?

**LABEL: documented** for the first half. **documented** for one specific, precise thing a partly-held file reports; **not in the sources** for what it reports more generally.

### Yes — partial holding is offered at four different grains

**Whole-file, all at once:**

> "Pack Resources
>
> …
>
> Mark all eligible external resource files used by the blend-file as packed.
> Actual packing will happen on the next save of the blend-file."

**But not everything is eligible:**

> "Warning
>
> Not all external files can be packed
>
> Some typically heavy external files, like videos from the Sequence Editor
> or Movie Clips, cannot be packed in a blend-file."

**One file at a time — the explicit answer to "partly rather than wholly":**

> "Selective Packing
>
> A single file can be packed by clicking on the little "gift box" icon to the left of its file-path UI widget."

> "Selective Unpacking
>
> A single file can be unpacked by clicking on the little "gift box" icon to the left of its file-path UI widget."

**Linked libraries are packed separately from resources — a different menu item:**

> "Pack Linked Libraries
>
> …
>
> Mark all linked library files in the current blend-file as packed."

— all from the manual, https://docs.blender.org/manual/en/5.2/files/blend/packed_data.html (read 2026-07-30)

**Per-bake, inside geometry nodes:**

> "Bake Target
>
> Specifies where the baked data should be stored.
>
> Inherit from Modifier:
>
> The setting is copied from the modifier that contains the bake node. This is the default.
>
> Packed:
>
> The baked data is packed into the .blend file. So no separate file is necessary.
>
> Disk:
>
> The baked data is stored in a separate directory on disk."

— manual, https://docs.blender.org/manual/en/5.2/modeling/geometry_nodes/geometry/operations/bake.html (read 2026-07-30)

New in 5.2:

> "* Added support for geonodes packed bakes in nodes modifiers (blender/blender!158148, blender/blender@329061c239)."

— release notes, https://developer.blender.org/docs/release_notes/5.2/core/ (read 2026-07-30)

**Per-tile and per-view, inside a single image:** an image holds a *collection* of packed files, each tagged with which tile and which view it is:

> "packed_file
>
> First packed file of the image (readonly)"
>
> "packed_files
>
> Collection of packed images (default None, readonly)"

— Python API, https://docs.blender.org/api/5.2/bpy.types.Image.html (read 2026-07-30)

> "class bpy.types.ImagePackedFile(bpy_struct)
>
> filepath
> …
> packed_file
> …
> tile_number
>
> (in [-inf, inf], default 0, readonly)
> …
> view
>
> (in [-inf, inf], default 0, readonly)"

— Python API, https://docs.blender.org/api/5.2/bpy.types.ImagePackedFile.html (read 2026-07-30)

**Per-asset, on import:**

> "PACK:
>
> Pack.
>
> Import the asset as linked data-block, and pack it in the current file (ensures that it remains unchanged in case the library data is modified, is not available anymore, etc.)."

— Python API, Asset Import Method Items, https://docs.blender.org/api/5.2/bpy.types.UserAssetLibrary.html (read 2026-07-30; the enum page itself is `bpy_types_enum_items/asset_import_method_items`, which has no separate address)

And the all-or-nothing switch is separate from the acts of packing:

> "use_autopack
>
> Automatically pack all external data into .blend file (default False)"

— Python API, https://docs.blender.org/api/5.2/bpy.types.BlendData.html (read 2026-07-30)

### What a partly-held file reports when it is read

The one thing the sources say outright is about reading such a file *as a library from outside*. Packed linked data is not visible there:

> "Notes:
>
> Not all data-block types are linkable (e.g. WindowManager, Library, …).
>
> Packed linked data-blocks are not linkable and are not listed in the input
> libraries attribute."

— Python API, `BlendDataLibraries.load`, https://docs.blender.org/api/5.2/bpy.types.BlendDataLibraries.html (read 2026-07-30)

The libraries that *are* listed come back tagged with whether they are archives:

> "A libraries attribute, which for the input is a list of all other libraries
> used by the loaded one, as named tuples (filepath, is_archive). The
> filepath may be absolute, or relative to the loaded blend-file. The output
> libraries attribute is always None."

— Python API, same page (read 2026-07-30). The release notes give a worked example, in which one entry is `is_archive=True` and the rest `is_archive=False`:

> "bpy_lib.libraries(filepath='//../../../assets/scripts/update_bake_path.blend', is_archive=False)
> …
> bpy_lib.libraries(filepath='//../../../../../local/blender/linux/5.0/datafiles/assets/nodes/procedural_hair_node_assets.blend', is_archive=True)"

— release notes, https://developer.blender.org/docs/release_notes/5.2/python_api/ (read 2026-07-30)

Within an opened file, packed and unpacked can be told apart per data-block:

> "is_linked_packed
>
> This data-block is linked and packed into the .blend file (default False, readonly)"

— Python API, https://docs.blender.org/api/5.2/bpy.types.ID.html (read 2026-07-30)

and the path enumerator can be told to include or exclude packed things:

> "SKIP_PACKED:
>
> Skip Packed.
>
> Skip paths when their matching data is packed."

— Python API, File Path Foreach Flag Items (read 2026-07-30); the default for `file_path_foreach` is `flags={'SKIP_PACKED', 'SKIP_WEAK_REFERENCES'}`, and `bpy.utils.blend_paths` has `packed (bool) – When true include file paths for packed data`.

In the interface, packing is shown by an icon, and 5.2 added a second one for indirect use:

> "You know that a data is packed when you see a little "gift box" icon displayed next to its path."

— manual, https://docs.blender.org/manual/en/5.2/files/blend/packed_data.html (read 2026-07-30)

> "  * `PACKAGE_INDIRECT`. A dimmed version of `PACKAGE` used by indirectly used packed data-blocks. (blender/blender@5d7403c8ce23)"

— release notes, https://developer.blender.org/docs/release_notes/5.2/user_interface/ (read 2026-07-30)

**Not covered.** Whether opening a partly-packed file prints or reports anything at all — a message, a count, a warning — is **not in the sources**. I searched the manual and API for `partially packed`, `partly`, `some files are packed`, and read `files/blend/packed_data.txt`, `interface/window_system/topbar.txt`, `files/blend/open_save.txt` and `editors/preferences/save_load.txt` in full; none describes a report on open. What is documented is that you can *ask* — `Report Missing Files` (question 12), `blend_paths`, `file_path_map`, `is_linked_packed`. The sources also do not say what a packed image reports if only some of its UDIM tiles are packed, nor whether a packed linked library still records its original path for relocation.

**Outside the interface.** Partly. Everything in the "what it reports" section above is Python and has no stated interface dependency; `bpy.data.libraries.load` reads a file without opening it. The packing *operators* are documented under a menu (`File ‣ External Data ‣ …`) and the gift-box icon is an interface affordance; whether the operators run headless the sources do not say, though they are exposed as `bpy.ops.file.pack_all` / `unpack_all` in the API. One documented constraint on when packing takes effect: "Actual packing will happen on the next save of the blend-file."

**Operating system.** The sources do not say. Nothing in this answer is qualified by operating system.


**Observed — hand test entries 1 and 5, run 2026-08-05 on the Creator's own computer** (MacBook Pro, M1 Pro chip, macOS 26.5.2, Apple's own graphics), with Blender 5.2.0 LTS started from a terminal with no window, using a throwaway settings folder set through `BLENDER_USER_RESOURCES`. The full list of tests is at `Audits/Blender Research/Dependencies/Hand tests.md`.

**What was done.** A file was built with two images on disk, `packed.png` and `loose.png`, both used by one material. `packed.png` was packed with `image.pack()` and the file saved. Both PNG files were then moved out of the way. The file was reopened with no window and, before Blender was asked anything, each image's `filepath`, `packed_file`, `size` and `has_data` were printed. Then `bpy.ops.file.report_missing_files()` was called, then `bpy.data.window_managers[0].reports` was read, then `bpy.utils.blend_paths` twice.

**What came back.** Nothing at all was printed when the file was opened.

```
   image=loose.png filepath='//assets/loose.png' packed_file=None size=(0, 0) has_data=False
   image=packed.png filepath='//assets/packed.png' packed_file=bpy.data.images['packed.png']...PackedFile size=(32, 32) has_data=True
```

So the packed copy is used and the packed image survives its original file being gone, while the unpacked one reports a size of zero.

`bpy.ops.file.report_missing_files()` printed to the terminal and returned:

```
Warning: Path '<path>/assets/loose.png' not found, from local data-block 'IMloose.png'
   returned: {'FINISHED'}
```

```
   packed=False: ['//assets/loose.png']
   packed=True:  ['//assets/loose.png', '//assets/packed.png']
```

`WindowManager.reports` exists in this version, and its length was 0 — the warning went to the terminal, not into that collection. In 4.5.12, 5.0.1 and 5.1.2 the property does not exist at all and reading it raised `AttributeError: 'WindowManager' object has no attribute 'reports'`.

A Cycles render of the same file finished with exit status 0.

**Both standing lines, brought up to date by this test.** *Driven from outside:* settled. A partly-packed file says nothing of its own accord when opened with no window; Report Missing Files does work with no window and its warning is printed to the terminal, naming the path and the data-block. *Operating system:* macOS for this test. The same test was run on a rented Linux machine on 2026-08-06 and that result is recorded below; Windows stays untested.

**Still open.** What the interface shows — the gift-box icons and where the message lands on screen — is entry 21 on the hand-test list and is the Creator's to answer.


**Observed — hand test entry 21, run by the Creator on 2026-08-05 on their own computer** (MacBook Pro, M1 Pro chip, macOS 26.5.2, Apple's own graphics), in Blender 5.2.0 LTS with its window open. This is the half of the list that needs eyes on the screen; the whole list is at `Audits/Blender Research/Dependencies/Hand tests.md`. Because it was run on a Mac, it settles macOS and leaves Windows and Linux open.

**What was done.** A file prepared for this test was opened. It holds two planes: the left one uses an image that was packed into the file, the right one uses an image that was not, and neither image file exists on disk any more. The Creator looked at the two images in the Image editor and at the Image Texture node in the Shading workspace, then used File ‣ External Data ‣ Report Missing Files.

**What came back.** Four screenshots, one per version, and these words from the Creator:

> "Report Missing Files says this:
> Path '/Users/artist/Documents/Smoozy hand tests/Dependencies/4.5/21/assets/loose.png' not found, from local data-block 'IMloose.png'
> Obviously 4.5 is for version 4.5 here, but the message is the same in all versions."

So the version's own folder appears in the path and the rest of the message is the same in all four. It is the same wording as the warning printed to the terminal when the same operator is called with no window.

The screenshots show the same three things in all four versions. In the image browse list, `packed.png` carries a small blue preview of itself and `loose.png` carries none. In the 3D viewport the plane using the packed image is blue — its own colour — and the plane using the unpacked one is bright magenta. In the Image editor the unpacked image shows an empty checkerboard.

**What the answer does not cover.** The entry also asked what sits to the left of each image's file-path field, to see whether the packed one carries a marker the unpacked one does not. In all four screenshots the image selected is `loose.png`, the unpacked one, and its row carries no such marker; what the packed image's row looks like was not photographed and is not reported. That part stays open.

**Both standing lines, brought up to date by this test.** *Driven from outside:* the wording of the report is now known to be the same with a window as without one, which the sources did not say. What the sources describe as landing in the Info editor is a window-only route; with no window the same text goes to the terminal. *Operating system:* macOS for this test. The same test was run on a rented Linux machine on 2026-08-06 and that result is recorded below; Windows stays untested.


**Observed — run again on Linux, 2026-08-06.** A partly-packed file whose files are gone was opened again in this version on the rented Linux machine, with no window. **Nothing at all was printed when it was opened**, and the state read back was the same as on the Creator's Mac:

```
   image=loose.png filepath='//assets/loose.png' packed_file=None size=(0, 0) has_data=False
   image=packed.png filepath='//assets/packed.png' packed_file=<bpy_struct, PackedFile...> size=(32, 32) has_data=True
```

`bpy.ops.file.report_missing_files()` returned `{'FINISHED'}` and printed one line naming the missing path and the local data-block that wants it. Listing the file's paths without packed data returned the unpacked one alone and with packed data returned both. The Cycles render finished with exit status 0.

**With a window open on the Linux machine, and this closes a gap the Mac left.** The Mac's screenshots all had the unpacked image selected, so what sits beside the *packed* image's file-path field was never seen. Photographed here and enlarged, the Image editor header reads, with the packed image selected: the image icon, the name field `packed.png`, a shield, a two-sheets copy button, **a stack of boxes**, and an X. With the unpacked image selected the row is identical except that where the stack of boxes was there is **a plain folder** — the ordinary open-a-file button. So the packed one carries a parcel and the unpacked one carries a folder.

The rest of the window picture matched the Mac: the plane using the packed image rendered in its own colour and the plane using the unpacked one bright magenta, and Report Missing Files put the same warning on screen that it prints to a terminal.

**Both standing lines, brought up to date by the Linux tests.** *Driven from outside:* every reading above was taken with Blender running with no window unless it says otherwise, so it holds outside the interface. *Operating system:* **now settled on two operating systems, macOS and Linux, and each is its own result** — what they agree on and where they differ is said above. **Windows is still untried: no Windows machine has run it.**

---

## 4. Can one thing a file points at mean several files on disk?

**LABEL: documented.** Yes, in at least five distinct ways, and the API has a name for it: a *virtual filepath*.

### The API states it directly

> "SKIP_MULTIFILE:
>
> Skip Multi-file.
>
> Skip paths where a single dir is used with an array of files, eg. sequence strip images or point-caches. In this case only the first file path is processed. This is needed for directory manipulation callbacks which might otherwise modify the same directory multiple times."
>
> "RESOLVE_TOKEN:
>
> Resolve Tokens.
>
> Resolve tokens within a virtual filepath to a single, concrete, filepath. Currently only used for UDIM tiles."
>
> "EXPAND_TOKENS:
>
> Expand Tokens.
>
> Expand template tokens in virtual file paths and invoke the callback once for each concrete file. Currently only used for UDIM tiles."
>
> "EXPAND_SEQUENCES:
>
> Expand Sequences.
>
> Expand image and volume sequences, invoking the callback once per file on disk."
>
> "EXPAND_CACHES:
>
> Expand Caches.
>
> Visit cache files, like texture cache files associated with images."

— Python API, File Path Foreach Flag Items, reachable from https://docs.blender.org/api/5.2/bpy.types.BlendData.html (read 2026-07-30)

and the per-path metadata that comes back:

> "is_expanded
>
> True when the path was expanded from a UDIM tile or sequence frame. These paths can not be edited."
>
> "is_cache
>
> True when the path is a cache file, like the image texture cache. These paths can not be edited."

— Python API, https://docs.blender.org/api/5.2/bpy.types.BlendDataPathMeta.html (read 2026-07-30)

The release notes say plainly why these exist:

> "Three new options for `bpy.data.file_path_foreach` so render farms and other scripts can find all relevant image files for rendering, including the new Cycles texture cache. (blender/blender!159159)
>
> - `EXPAND_TOKENS`: visit each UDIM tile path.
> - `EXPAND_SEQUENCES`: visit each image and volume sequence file path.
> - `EXPAND_CACHES`: visit cache files, like texture cache files associated with images."

— release notes, https://developer.blender.org/docs/release_notes/5.2/python_api/ (read 2026-07-30)

### The five ways, each documented

**(a) Image sequence.** One image data-block, many files:

> "SEQUENCE
> Image Sequence – Multiple image files, as a sequence."

— Python API, `Image.source`, https://docs.blender.org/api/5.2/bpy.types.Image.html (read 2026-07-30)

> "To load image sequence in any of the supported image file formats,
> the filename of the images must contain a digit to indicate the frame order
> (e.g. *-0001.jpg, *-0002.jpg, *-0003.jpg, etc, of any image format), indicating the frame."

— manual, https://docs.blender.org/manual/en/5.2/files/media/image_formats.html (read 2026-07-30)

**(b) UDIM tiles.** One path with a token in it, standing for a grid of files:

> "Substitution tokens are special sequences of characters in a filename
> that can be replaced with more meaningful and context aware information.
> In this case, tokens are identified by being text wrapped in angle bracket characters.
>
> This substitution is used while load loading or saving an image
> to automatically identify the tile associated with a particular texture in the UDIM array.
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

— manual, https://docs.blender.org/manual/en/5.2/modeling/meshes/uv/workflows/udims.html (read 2026-07-30)

The corresponding source value and collection:

> "TILED
> UDIM Tiles – Tiled UDIM image texture."
>
> "tiles
>
> Tiles of the image (default None, readonly)"

— Python API, https://docs.blender.org/api/5.2/bpy.types.Image.html (read 2026-07-30)

And the detection option on load:

> "Detect UDIMs
>
> Automatically looks for UDIM
> tiles in the directory of the selected image; if matches are found they are loaded into Blender as UDIMs.
> This works by detecting if the filename has a .xxxx (four digit number) before the file extension."

— manual, https://docs.blender.org/manual/en/5.2/files/media/image_formats.html (read 2026-07-30)

**(c) Volume sequence.** One VDB path, many `.vdb` files:

> "Sequence
>
> Loads further VDB files, one for each frame in an animation. Much like with image sequences,
> all the files should have a numerical suffix in their name; so if you selected smoke-000.vdb
> in the File Path, there should be a smoke-001.vdb, a smoke-002.vdb and so on."

— manual, https://docs.blender.org/manual/en/5.2/modeling/volumes/properties.html (read 2026-07-30)

**(d) Multiple views (stereo / multi-view).** One image reference, one file per view:

> "is_multiview
>
> Image has more than one view (default False, readonly)"

— Python API, https://docs.blender.org/api/5.2/bpy.types.Image.html (read 2026-07-30)

> "file_suffix
>
> Suffix added to the render images for this view (default "", never None)"

— Python API, https://docs.blender.org/api/5.2/bpy.types.SceneRenderView.html (read 2026-07-30)

> "The Individual option will produce separate images that
> (if saved in a lossless encoding such as PNG or OpenEXR)
> can be loaded back in production with no loss of data."

— manual, https://docs.blender.org/manual/en/5.2/render/output/properties/stereoscopy/usage.html (read 2026-07-30)

And on the output side the `%` character stands for the view:

> "view (str) – View, The name of the view to use to replace the "%" chars (optional, never None)"

— Python API, `RenderSettings.frame_path`, https://docs.blender.org/api/5.2/bpy.types.RenderSettings.html (read 2026-07-30)

**(e) One image implying a cache file beside it.** See `EXPAND_CACHES` above, and question 10.

The Image data-block's own overview confirms the one-to-many shape:

> "The Image data-block is a shallow wrapper around image or video file(s)
> (on disk, as packed data, or generated).
>
> All actual data like the pixel buffer, size, resolution etc. is
> cached in an imbuf.types.ImBuf image buffer (or several buffers
> in some cases, like UDIM textures, multi-views, animations…)."

— Python API, https://docs.blender.org/api/5.2/bpy.types.Image.html (read 2026-07-30)

**Not covered.** How the expansion behaves when some of the implied files are missing — whether `EXPAND_SEQUENCES` visits paths that do not exist, or only ones that do — is not stated. Nor is the frame range that a sequence expands over, in the general case. Nor whether the render output path's `#` padding (question 5) counts as the same kind of one-to-many, though it plainly produces many files.

**Outside the interface.** Holds, and this is the case the feature was built for — see the release-notes quote naming render farms and scripts. `EXPAND_TOKENS`, `EXPAND_SEQUENCES` and `EXPAND_CACHES` are Python flags with no stated interface dependency. The `Detect Sequences` and `Detect UDIMs` options quoted above are File Browser options, so they belong to the interface; but they only affect how a reference is *created*, not how an existing one expands.

**Operating system.** The sources do not say. Nothing in this answer is qualified by operating system.


**Observed — hand test entries 4, 15 and 16, run 2026-08-05 on the Creator's own computer** (MacBook Pro, M1 Pro chip, macOS 26.5.2, Apple's own graphics), with Blender 5.2.0 LTS started from a terminal with no window, using a throwaway settings folder set through `BLENDER_USER_RESOURCES`. The full list of tests is at `Audits/Blender Research/Dependencies/Hand tests.md`.

**What was done.** Two UDIM tiles `tiles/tile.1001.png` and `tiles/tile.1002.png`, and a three-frame sequence `seq/shot_0001.png` to `shot_0003.png`, were written to disk. One image data-block was loaded from `tile.1001.png` with `source` set to `TILED`, another from `shot_0001.png` with `source` set to `SEQUENCE`. Their stored paths and tile numbers were printed, then `bpy.utils.blend_paths()`, then `bpy.data.file_path_map()`, then a walk of `bpy.data.file_path_foreach`.

**What came back.**

```
   image=shot_0001.png  source=SEQUENCE  stored='//seq/shot_0001.png' tiles=[1001]
   image=tile.1001.png  source=TILED     stored='//tiles/tile.<UDIM>.png' tiles=[1001, 1002]
```

Blender rewrote the tiled image's path into the `<UDIM>` token form itself, and one recorded path stood for two files. `blend_paths()` and `file_path_map()` both returned the un-expanded text.

**This version can be asked to expand one path into the files it stands for.** The valid flags, learned by passing a deliberately invalid one, are `('SKIP_LINKED', 'SKIP_PACKED', 'SKIP_WEAK_REFERENCES', 'SKIP_MULTIFILE', 'RELOAD_EDITED', 'RESOLVE_TOKEN', 'EXPAND_TOKENS', 'EXPAND_SEQUENCES', 'EXPAND_CACHES')`. Walking `bpy.data.file_path_foreach` twice gave:

```
plain:
   visited '//volumes/smoke.vdb' is_expanded=False is_cache=False
   visited '//seq/shot_0001.png' is_expanded=False is_cache=False
   visited '//tiles/tile.<UDIM>.png' is_expanded=False is_cache=False
with EXPAND_TOKENS, EXPAND_SEQUENCES, EXPAND_CACHES added to the default flags:
   visited '//volumes/smoke.vdb' is_expanded=False is_cache=False
   visited '<path>/seq/shot_0001.png' is_expanded=True is_cache=False
   visited '<path>/seq/shot_0002.png' is_expanded=True is_cache=False
   visited '<path>/seq/shot_0003.png' is_expanded=True is_cache=False
   visited '<path>/tiles/tile.1001.png' is_expanded=True is_cache=False
   visited '<path>/tiles/tile.1002.png' is_expanded=True is_cache=False
```

So one recorded path expanded into three files for the sequence and two for the tiles. The three expand flags do not exist in 5.0.1 or 5.1.2, and the function itself does not exist in 4.5.12.

**A separate test, on a member that is not there.** A Volume data-block whose `filepath` was `//volumes/smoke.vdb`, with no such file on disk, gave in all four versions:

```
   grids.is_loaded before load = False
   grids.load() returned False
   grids.is_loaded after load  = False
   grids.error_message         = 'smoke.vdb not found'
   grids.frame_filepath        = ''
```

**Both standing lines, brought up to date by this test.** *Driven from outside:* settled. Everything above was read with no window: the token form is stored the same way, and the expansion in 5.2 is a plain function call. *Operating system:* macOS for this test. The same test was run on a rented Linux machine on 2026-08-06 and that result is recorded below; Windows stays untested.


**Observed — run again on Linux, 2026-08-06.** Built and read again in this version on the rented Linux machine. The storing came back the same as on the Creator's Mac, and Blender rewrote the tiled image's path into the token form on its own:

```
   image=shot_0001.png    source=SEQUENCE stored='…/seq/shot_0001.png' tiles=[1001]
   image=tile.1001.png    source=TILED    stored='…/tiles/tile.<UDIM>.png' tiles=[1001, 1002]
```

`blend_paths()` and `file_path_map()` both returned the un-expanded text. The invalid-flag probe returned `ValueError: flags: 'NOT_A_REAL_FLAG' not found in ('SKIP_LINKED', 'SKIP_PACKED', 'SKIP_WEAK_REFERENCES', 'SKIP_MULTIFILE', 'RELOAD_EDITED', 'RESOLVE_TOKEN', 'EXPAND_TOKENS', 'EXPAND_SEQUENCES', 'EXPAND_CACHES')`, and with the three expand options added the walk visited three sequence entries and two tile entries, each marked `is_expanded=True`, against three un-expanded entries without them. The same as on the Mac.

A Volume whose file is not there answered the same on both machines:

```
   grids.is_loaded before load = False
   grids.load() returned       = False
   grids.is_loaded after load  = False
   grids.error_message         = 'smoke.vdb not found'
   grids.frame_filepath        = ''
```

**Both standing lines, brought up to date by the Linux tests.** *Driven from outside:* every reading above was taken with Blender running with no window unless it says otherwise, so it holds outside the interface. *Operating system:* **now settled on two operating systems, macOS and Linux, and each is its own result** — what they agree on and where they differ is said above. **Windows is still untried: no Windows machine has run it.**

---

## 5. In what ways can a recorded location be written?

**LABEL: documented.** Six forms, three of which are meaningful only in paths Blender writes to.

### (a) Relative to the blend-file, marked by `//`

> "When you specify one of these external files, the default option is to make the path relative.
> Blender stores a partial path evaluated relative to the directory location of the referencing blend-file.
> This choice helps when you need to reorganize folders or move your files."
>
> "When relative paths are supported, the File Browser provides a Relative Path checkbox,
> when entering the path into a text field, use a double slash prefix (//) to make it so.
>
> Relative paths are the default but this can be changed
> in the File tab of the Preferences.
>
> Note
>
> You cannot use relative paths into a new untitled blend-file.
> Save it before linking to external files."

— manual, https://docs.blender.org/manual/en/5.2/files/blend/open_save.html (read 2026-07-30)

> "Hint
>
> The default path // refers to the folder of the currently open blend-file
> (see Relative Paths for details)."

— manual, https://docs.blender.org/manual/en/5.2/editors/preferences/file_paths.html (read 2026-07-30)

Crucially, "the blend-file" is not always the file you opened. For a path on a library-linked data-block it is the library:

> "bpy.path.abspath(path, *, start=None, library=None)
>
> Returns the absolute path relative to the current blend file
> using the "//" prefix.
>
> …
>
> library (bpy.types.Library | None) – The library this path is from. This is only included for
> convenience, when the library is not None its path replaces start."

— Python API, https://docs.blender.org/api/5.2/bpy.path.html (read 2026-07-30)

and the path-template variables make the same distinction:

> "blend_dir_lib:
>
> Like blend_dir, except that for path properties on
> library-linked data-blocks it will be the path to the library blend file
> that the data-block comes from."

— manual, https://docs.blender.org/manual/en/5.2/files/file_paths.html (read 2026-07-30)

The API records, per property, whether the `//` form is accepted at all. The marker is the phrase "blend relative // prefix supported"; forty-one API pages carry it (listed under question 1), and `bpy.types.Text.filepath` notably does not.

### (b) Absolute

> "Make Paths Absolute
>
> Make all paths to external files absolute (= full path from the system's root)."

— manual, https://docs.blender.org/manual/en/5.2/interface/window_system/topbar.html (read 2026-07-30)

The two forms are convertible in both directions, in bulk:

> "Make Paths Relative
>
> Make all paths to external files relative to the current blend-file."

— same page (read 2026-07-30)

and on save:

> "Remap Relative
>
> This option remaps Relative Paths
> (such as linked libraries and images) when saving a file in a new location."

— manual, https://docs.blender.org/manual/en/5.2/files/blend/open_save.html (read 2026-07-30)

`bpy.path.relpath` and `bpy.path.abspath` are the Python equivalents (same address as above).

### (c) With a substitution token in angle brackets — `<UDIM>`, `<UVTILE>`

Quoted in full under question 4.

### (d) With a template expression in curly braces — `{blend_name}`, and a format specifier

> "Path templates substitute template expressions (written {abcd}, where
> "abcd" is a variable name) in a filepath when the path is used.
>
> For example, if the currently open blend file is dance.blend and the render
> output path is set to this:
>
> //my_render_dir/{blend_name}.png
>
> Then when rendering, Blender will treat the path as:
>
> //my_render_dir/dance.png
>
> With {blend_name} getting replaced by the dance from dance.blend.
>
> This substitution happens internally at time of use (e.g. while rendering), so
> the template syntax will remain as-is in the path field."

The available variables are `blend_name`, `blend_dir`, `blend_name_lib`, `blend_dir_lib` everywhere templates are supported; `fps`, `resolution_x`, `resolution_y`, `scene_name`, `camera_name` in render output paths; and `node_name` in path properties on a node. Format specifiers are written after a colon:

> "dance_{fps:###}.png: format as an integer with at least 3 digits.
>
> dance_{fps:.###}.png: format as a floating point number with exactly 3
> digits after the decimal point.
>
> dance_{fps:###.##}.png: format as a floating point number with at least 3
> digits for the integer part and exactly 2 digits for the fractional part."

Literal braces are doubled:

> "{{ translates to a single { in the final path.
>
> }} translates to a single } in the final path."

And where they work is limited:

> "Note
>
> Currently path templates are only supported for the render output path in
> Scene properties and the output paths in the compositor's File Output
> node. More filepaths will support templates in future releases."

— all from the manual, https://docs.blender.org/manual/en/5.2/files/file_paths.html (read 2026-07-30)

A malformed template is a distinct failure state of the path itself:

> "Paths that support templates can have template errors, which prevent the
> path from being processed."
>
> "Note
>
> Depending on the path, template errors may prevent certain actions. For
> example, if there are errors in the render output path, then rendering an
> animation will fail with an error message indicating the path errors."

— same page (read 2026-07-30). The manual illustrates both the red field and the tooltip with screenshots, `_images/files_file_paths-invalid_path_template.png` and `_images/files_file_paths-invalid_path_template_tooltip.png`. I did not need to open them; the text states the behaviour.

The API marks which properties accept templates with the phrase "Supports template expressions" — it appears on `RenderSettings.filepath` and `CompositorNodeOutputFile.directory`, matching the manual's note.

### (e) With `#` characters standing for the frame number

> "-o, --render-output <path>
>
> Set the render path and file name.
> Use // at the start of the path to render relative to the blend-file.
>
> You can use path templating features such as {blend_name} in the path.
> See Blender's documentation on path templates for more details.
>
> The # characters are replaced by the frame number, and used to define zero padding.
>
> animation_##_test.png becomes animation_01_test.png
>
> test-######.png becomes test-000001.png
>
> When the filename does not contain #, the suffix #### is added to the filename.
>
> The frame number will be added at the end of the filename, eg:
>
> blender -b animation.blend -o //render_ -F PNG -x 1 -a
>
> //render_ becomes //render_####, writing frames as //render_0001.png"

— manual, https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html (read 2026-07-30)

> "filepath
>
> Directory/name to save animations, # characters define the position and padding of frame numbers (default "//", never None, blend relative // prefix supported, Supports template expressions)"

— Python API, https://docs.blender.org/api/5.2/bpy.types.RenderSettings.html (read 2026-07-30)

### (f) With `%` standing for the view name

> "view (str) – View, The name of the view to use to replace the "%" chars (optional, never None)"

— Python API, `RenderSettings.frame_path`, https://docs.blender.org/api/5.2/bpy.types.RenderSettings.html (read 2026-07-30)

This appears **only** in the API documentation. I searched the whole manual for `%`, `percent`, and the phrase "chars" and found no mention of it there.

### One more form, in a preference rather than a data-block

> "texture_cache_directory
>
> The directory for storing tx files generated from image files, for more efficient rendering. Paths may be absolute, or relative to the image file. Leave blank to store tx files in the same directory as image files (default "", never None, blend relative // prefix supported)"

— Python API, https://docs.blender.org/api/5.2/bpy.types.PreferencesFilePaths.html (read 2026-07-30)

This is a *third* base for a relative path — relative to the image file, not to the blend-file. The manual agrees:

> "Texture Cache
>
> Directory to store .tx texture cache files, relative to the
> source image file or as an absolute path.
> Used by the Cycles Texture Cache."

— manual, https://docs.blender.org/manual/en/5.2/editors/preferences/file_paths.html (read 2026-07-30)

**Observed**, from Cycles' own files, the same wording in the command-line helper:

> ```
>     parser.add_argument(
>         "--cache-dir",
>         dest="cache_dir",
>         type=str,
>         default="",
>         help="Output directory for tx files, relative to source image or absolute.",
>     )
> ```

— `Blender Sources/cycles_5.2/maketx.py`, lines 49–55 (read 2026-07-30). This is what the file *says*; that a run actually places the file there is a step beyond it and I have not taken it.

**Not covered.** Environment-variable expansion inside a recorded path: I searched the manual and API for `$user`, `$BLENDER`, `environment variable`, `%USERPROFILE%` and similar and found environment variables used only to configure Blender itself (`OCIO`, `BLENDER_OCIO`, `BLENDER_SYSTEM_DATAFILES`, `TEMP`/`TMPDIR`) and in a prose example of a shader directory, never as something expanded inside a stored path property. Home-directory `~` expansion: the manual uses `~/` in prose about installing Blender and about the manual's own build, never about a stored path. Path separators (forward versus back slash), drive letters, and maximum path length: I searched for `path separator`, `backslash`, `forward slash`, `drive letter` and `C:\` and found nothing that governs a stored location — the only `Path Separator` hit is an OBJ import option about splitting *object names*, not file paths. Case: not covered here; that is question 8, which I was not assigned.

The answer also does not settle what a path field does with a form it does not support — for instance a `//` prefix on `bpy.types.Text.filepath`, which the API does not mark as supporting it.

**Outside the interface.** Holds, and several of these forms exist *for* outside use. `//`, `#` and `{…}` are all documented on the command-line arguments page under `-o`. `bpy.path.abspath`, `bpy.path.relpath` and `RenderSettings.frame_path` are Python and have no stated interface dependency. The one place where the interface is doing work is the *reporting* of a template error — the red field and the tooltip. But the manual states the non-interface consequence too: "rendering an animation will fail with an error message indicating the path errors."

**Operating system.** The sources do not say, for any of the six forms. There is no per-operating-system qualifier on any of the quoted passages. The absence of any statement about drive letters or separators is itself the gap, not a finding.


**Observed — hand test entries 3, 14 and 15, run 2026-08-05 on the Creator's own computer** (MacBook Pro, M1 Pro chip, macOS 26.5.2, Apple's own graphics), with Blender 5.2.0 LTS started from a terminal with no window, using a throwaway settings folder set through `BLENDER_USER_RESOURCES`. The full list of tests is at `Audits/Blender Research/Dependencies/Hand tests.md`.

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


**Observed — hand test entry 24, run by the Creator on 2026-08-05 on their own computer** (MacBook Pro, M1 Pro chip, macOS 26.5.2, Apple's own graphics), in Blender 5.2.0 LTS with its window open. This is the half of the list that needs eyes on the screen; the whole list is at `Audits/Blender Research/Dependencies/Hand tests.md`. Because it was run on a Mac, it settles macOS and leaves Windows and Linux open.

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

## 10. Are some of the things a file points at places it writes to rather than reads from, and where does such a write land?

**LABEL: documented** that many recorded paths are write destinations, and for how `//` resolves. **not in the sources** for where a `//` render output path lands when the file has never been saved.

### Yes. At least ten kinds of recorded location are written to, not read from

**Render output.**

> "Choose the location to save rendered frames.
>
> When rendering an animation,
> the frame number is appended at the end of the file name with four padded zeros (e.g. image0001.png).
> You can set a custom padding size by adding the appropriate number of # anywhere in the file name
> (e.g. image_##_test.png translates to image_01_test.png).
>
> This setting expands Relative Paths
> where a // prefix represents the directory of the current blend-file."

— manual, https://docs.blender.org/manual/en/5.2/render/output/properties/output.html (read 2026-07-30)

> "frame_path(*, frame=-2147483648, preview=False, view='')
>
> Return the absolute path to the filename to be written for a given frame"

— Python API, https://docs.blender.org/api/5.2/bpy.types.RenderSettings.html (read 2026-07-30)

**Compositor File Output node** — and it creates folders:

> "The File Output node writes one or more images to disk during rendering or compositing.
> Each connected input is saved to the specified directory and filename,
> with the frame number appended to create a sequence.
> For example: {Directory}/{File Name}{Socket Name}{frame number}.{extension}."
>
> "If an input name contains a slash character (/ or \\),
> the path is interpreted as a directory hierarchy.
>
> Any missing directories are created automatically during output."

— manual, https://docs.blender.org/manual/en/5.2/compositing/types/output/file_output.html (read 2026-07-30)

**Physics disk cache** — lands beside the blend-file, under a derived name:

> "The cache of a baked simulation will be stored inside the blend-file when you save it.
> When Disk Cache is checked, Blender will save the cache separately to
> the drive in a folder named blendcache_[filename] alongside the blend-file.
> (The blend-file must be saved first.)"
>
> "Each cache can have a name. Double-click the cache entry to give it a name.
> If this name is given, any disk cache will be stored in files starting with that name.
> For example, a cache named MyCache will be stored in MyCache_xxxxxx_yy.bphys.
>
> If the cache does not have a name (which is the default),
> the filename of the cache will depend on the object it is attached to,
> although this is not immediately obvious. For example, a cache on
> an object Cube will be stored in 43756265_xxxxxx_yy.bphys,
> where 43756265 is determined by the object name."

— manual, https://docs.blender.org/manual/en/5.2/physics/baking.html (read 2026-07-30)

**Geometry-nodes bakes** — location follows where the blend-file was saved:

> "Bake simulations in geometry nodes modifiers.
> In order to bake the simulation, the blend-file must be saved to your computer.
> The location the file is saved determines where the baked data is also saved.
> The directory the baked data is saved to can be changed per modifier in the
> Internal Dependencies."

— manual, https://docs.blender.org/manual/en/5.2/physics/simulation_nodes.html (read 2026-07-30)

> "Data is not stored to disk until the Bake operation is ran.
>
> Important
>
> Blend-files must be saved to a disk before data can be baked."

— manual, https://docs.blender.org/manual/en/5.2/modeling/geometry_nodes/baking.html (read 2026-07-30)

**Fluid cache directory.**

> "Directory to store baked simulation files in. Inside this directory each simulation type
> (i.e. mesh, particles, noise) will have its own directory containing the simulation data."

— manual, https://docs.blender.org/manual/en/5.2/physics/fluid/type/domain/cache.html (read 2026-07-30)

**Ocean modifier cache** — creates the folder:

> "Folder to store the baked EXR files in.
> The sequences will be in the form disp_####.exr, normal_####.exr,
> and foam_####.exr, where #### is the four digit frame number.
> If the cache path folder does not exist, it will be created."

— manual, https://docs.blender.org/manual/en/5.2/modeling/modifiers/physics/ocean.html (read 2026-07-30)

**Sequencer proxies** — with a documented default beside the footage:

> "By default, all generated proxy videos are stored to
> the folder <path of original footage>/BL_proxy/<clip name>,
> but this can be changed to a custom directory using this option."

— manual, https://docs.blender.org/manual/en/5.2/editors/video_sequencer/sequencer/sidebar/proxy.html (read 2026-07-30)

**Cycles texture cache** — written *during rendering*, beside the image, not beside the blend-file:

> "Uses .tx files that store images in a tiled, mipmapped format optimized for
> loading just the tiles and resolutions needed for rendering. By default, files are
> stored in the blender_tx/ folder next to the image file. This can be changed
> with the Texture Cache
> file path preference.
>
> Auto Generate
>
> Automatically generate and update .tx files for images when rendering.
> Files are regenerated when the source image is modified."

and, separately on the same page:

> "Render high resolution images in tiles of this size, to reduce memory usage
> of render passes. Tiles are cached to disk while rendering to save memory."

— manual, https://docs.blender.org/manual/en/5.2/render/cycles/render_settings/performance.html (read 2026-07-30)

**Render cache.**

> "Saves the rendered view layers and their passes to a multi-layer OpenEXR image.
> The Compositor can then use this file to improve performance, especially for heavy compositing.
>
> The image is stored in the Render Cache folder as specified in the
> File Paths Preferences."

— manual, https://docs.blender.org/manual/en/5.2/render/output/properties/output.html (read 2026-07-30)

**And in the API, each with its own wording** (all `https://docs.blender.org/api/5.2/bpy.types.<Name>.html`, read 2026-07-30): `DynamicPaintSurface.image_output_path` "Directory to save the textures"; `BakeSettings.filepath` "Image filepath to use when saving externally"; `MovieClipProxy` "Location to store the proxy files"; `PointCache.filepath` "Cache file path"; `CollectionExport.filepath` "The file path used for exporting"; `PreferencesFilePaths.render_output_directory` "The default directory for rendering output, for new scenes"; `PreferencesFilePaths.render_cache_directory` "Where to cache raw render results".

### Where such a write lands

Three different bases, all documented, and they are not the same base:

1. **The blend-file's folder**, for anything with a `//` prefix — "This setting expands Relative Paths where a // prefix represents the directory of the current blend-file" (output properties, quoted above), and "The // at the start of each path in Blender means the directory of the currently opened blend-file, used to reference relative paths" (manual, https://docs.blender.org/manual/en/5.2/getting_started/configuration/introduction.html, read 2026-07-30). Physics disk caches land there by construction (`blendcache_[filename]`).
2. **Beside the source image**, for the Cycles texture cache — "the blender_tx/ folder next to the image file", and the preference is "relative to the source image file or as an absolute path".
3. **Beside the source footage**, for sequencer proxies by default — `<path of original footage>/BL_proxy/<clip name>`.

Templates are substituted at the moment of writing, not stored expanded:

> "This substitution happens internally at time of use (e.g. while rendering), so
> the template syntax will remain as-is in the path field."

— manual, https://docs.blender.org/manual/en/5.2/files/file_paths.html (read 2026-07-30)

### Writes that are not recorded in the file at all

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

— manual, https://docs.blender.org/manual/en/5.2/advanced/blender_directory_layout.html (read 2026-07-30)

> "The directory for storing temporary save files. The path must reference an existing directory
> or it will be ignored and the systems temporary directory will be used instead.
> When left blank, the systems temporary directory will be used."

— manual, https://docs.blender.org/manual/en/5.2/editors/preferences/file_paths.html (read 2026-07-30)

> "Closes Blender. The current scene is saved to a file called "quit.blend" in Blender's temporary directory"

— manual, https://docs.blender.org/manual/en/5.2/interface/window_system/topbar.html (read 2026-07-30)

`bpy.app.tempdir` — "String, the temp directory used by blender (read-only)" (Python API, https://docs.blender.org/api/5.2/bpy.app.html, read 2026-07-30).

**Not covered.** Where a `//` render output path lands when the blend-file has never been saved: **not in the sources**. The record behind that — the manual states the constraint for *linking* ("You cannot use relative paths into a new untitled blend-file. Save it before linking to external files.") and for *baking* ("Blend-files must be saved to a disk before data can be baked", "The blend-file must be saved in a directory so that simulation results can be saved"), but says nothing about the render output path in that state; the only related API is `BlendData.is_saved`, "Has the current session been saved to disk as a .blend file". Searches run for this: `untitled`, `unsaved`, `not been saved`, `must be saved`, `is_saved`, `is_dirty`.

Also not covered: whether the scene render output directory is auto-created (only the File Output node and the Ocean cache path are documented as creating folders); whether the write is atomic; the on-disk layout of the render cache folder; and where the tile spooling ("Tiles are cached to disk while rendering") actually writes — that sentence does not say. There is one unreconciled tension in the sources: the temporary directory is documented as holding "render layers, physics cache", while disk caches are documented as landing beside the blend-file. Nothing reconciles the two, and this answer does not either.

**Outside the interface.** Holds, and there are command-line equivalents, with a documented ordering trap:

> "blender --background test.blend --render-frame 1 --render-output "/tmp"
>
> …will not render to /tmp because --render-frame 1 renders before the output path is set.
>
> blender --background --render-output /tmp test.blend --render-frame 1
>
> …will not render to /tmp because loading the blend-file overwrites the render output that was set.
>
> blender --background test.blend --render-output /tmp --render-frame 1
>
> …works as expected."

— manual, https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html (read 2026-07-30)

**Observed**, from Cycles' own files, the texture cache is also generatable from outside, and refuses without a loaded file:

> ```
>     if not bpy.data.filepath:
>         parser.error("No image file path given and no blend file loaded.")
> ```

— `Blender Sources/cycles_5.2/maketx.py`, lines 93–94 (read 2026-07-30). What the file *says*. That a run behaves this way is a step beyond, and I have not taken it.

The sources do not say whether the temporary directory or auto-save behaves differently in background mode.

**Operating system.** Does **not** hold identically everywhere. The temporary directory is explicitly per-operating-system — "TEMP on Windows, TMP & TMP_DIR on other platforms", falling back to "/tmp/". The persistent cache directory likewise (quoted under question 2). The `//` resolution rule itself is not qualified by operating system anywhere in the sources.


**Observed — hand test entries 13, 18 and 20, run 2026-08-05 on the Creator's own computer** (MacBook Pro, M1 Pro chip, macOS 26.5.2, Apple's own graphics), with Blender 5.2.0 LTS started from a terminal with no window, using a throwaway settings folder set through `BLENDER_USER_RESOURCES`. The full list of tests is at `Audits/Blender Research/Dependencies/Hand tests.md`.

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
['file_output_items', 'active_item_index', 'directory', 'file_name', 'format', 'save_as_render', 'use_file_extension']
```

In 4.5 the folder is `base_path`; in 5.0, 5.1 and 5.2 it is `directory`, with a separate `file_name`. Setting `base_path` in 5.x raises `AttributeError: 'CompositorNodeOutputFile' object has no attribute 'base_path'`, and `Scene.node_tree` does not exist there either — the compositor tree is reached through `Scene.compositing_node_group`, which is `None` until something is put there.

Whether that node creates a folder that does not exist is **not** settled for this version. Building a compositor node group by hand from a script and rendering produced no folder and no files, but a tree built that way may simply not be wired into the render, so the result carries nothing. It is entry 26 on the hand-test list and is the Creator's to answer through the window.

**Both standing lines, brought up to date by this test.** *Driven from outside:* settled for the render output path, for folder creation and for the unsaved-file case — all of it was done with no window. One thing is settled the other way: the operator that updates an external shader script cannot be called with no window, and says so. *Operating system:* macOS for this test. The same test was run on a rented Linux machine on 2026-08-06 and that result is recorded below; Windows stays untested.


**Observed — hand test entry 26, run by the Creator on 2026-08-05 on their own computer** (MacBook Pro, M1 Pro chip, macOS 26.5.2, Apple's own graphics), in Blender 5.2.0 LTS with its window open. This is the half of the list that needs eyes on the screen; the whole list is at `Audits/Blender Research/Dependencies/Hand tests.md`. Because it was run on a Mac, it settles macOS and leaves Windows and Linux open.

**What was done.** A plain two-frame file was opened, Use Nodes was ticked in the Compositing workspace, a File Output node was added and connected to the Render Layers node's Image output, and its folder was set to `//fileoutput_new_folder/` — a folder that did not exist. Then Render ‣ Render Animation.

**What came back**, in the Creator's words: "It all renders. You can check out the files yourself." The folder beside the blend-file was then read from a terminal:

```
$ ls "/Users/artist/Documents/Smoozy hand tests/Dependencies/5.2/26/fileoutput_new_folder/"
file output0001.exr
file output0002.exr
```

**So the node does create a folder that does not exist, in this version**, and it wrote two files into it: `file output0001.exr` and `file output0002.exr`.

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

**And this closes what the Mac could not settle: whether a File Output node writes when the file is rendered with no window, and why a script-built one wrote nothing, and why the file is named after the blend-file.** The Mac test found that a node group built from a script wrote nothing in this version. The cause, found here: **the File Output node arrives with no output items and therefore no usable input socket**, so the link from Render Layers went nowhere. Adding one output item first with `fo.file_output_items.new("RGBA", "MyPass")`, linking to that, **saving**, and then rendering in a separate run with no window gave:

```
   directory = '//fileoutput_new_folder/'  file_name = '{blend_name}'
   Saved: '/root/…/e26/fileoutput_new_folder/file output0001.exr'
   Saved: '/root/…/e26/fileoutput_new_folder/file output0002.exr'
```

It created the folder that was not there. **And the reason the written file carries the blend-file's own name is that `file_name` holds the template `{blend_name}` by default** — which the Mac run could see the effect of but not the cause. The names and the file type are exactly what the Creator saw through the window on the Mac.

**One thing about where a write lands that only the crossing could show.** A file whose render output path is a full path written on a Mac does not make a Linux render refuse. It creates the whole Mac-shaped folder tree at the root of the Linux disk and writes the frames into it, saying nothing. That run was made as the root user, who owns the root of that disk, so **what happens for a render user who cannot write there is not established.**

**Both standing lines, brought up to date by the Linux tests.** *Driven from outside:* every reading above was taken with Blender running with no window unless it says otherwise, so it holds outside the interface. *Operating system:* **now settled on two operating systems, macOS and Linux, and each is its own result** — what they agree on and where they differ is said above. **Windows is still untried: no Windows machine has run it.**

---

## 11. Can a reference point at a named thing inside another file, and what happens when that thing is renamed or is gone?

**LABEL: documented** for the mechanism, for what a rename does, and for what happens when the thing is gone.

### Yes — the reference is by name, and the name is the whole identity

> "A data-block's name is unique within its namespace. A data-block namespace is defined by its type, and the blendfile
> it is stored in.
>
> This means that there can be for example an Object and a Mesh named the same, but there cannot be two local objects
> named the same in a blendfile. However, it is possible to have one local and several linked Objects sharing the same
> name."

— manual, https://docs.blender.org/manual/en/5.2/files/data_blocks.html (read 2026-07-30)

The API stores the pair (library file, name in that file) explicitly:

> "class bpy.types.LibraryWeakReference(bpy_struct)
>
> Read-only external reference to a linked data-block and its library file
>
> filepath
>
> Path to the library .blend file (default "", never None, blend relative // prefix supported)
>
> Type:
>
> str
>
> id_name
>
> Full ID name in the library .blend file (including the two leading 'id type' chars) (default "", never None)"

— Python API, https://docs.blender.org/api/5.2/bpy.types.LibraryWeakReference.html (read 2026-07-30)

> "library_weak_reference
>
> Weak reference to a data-block in another library .blend file (used to re-use already appended data instead of appending new copies) (readonly)"

— Python API, https://docs.blender.org/api/5.2/bpy.types.ID.html (read 2026-07-30)

And linking is done by naming:

> "```
> # Load a single scene we know the name of.
> with bpy.data.libraries.load(filepath) as (data_src, data_dst):
>     data_dst.scenes = ["Scene"]
> ```"

— Python API, https://docs.blender.org/api/5.2/bpy.types.BlendDataLibraries.html (read 2026-07-30)

`ID.name` is described as:

> "name
>
> Unique data-block ID name (within a same type and library) (default "", never None)"

— Python API, https://docs.blender.org/api/5.2/bpy.types.ID.html (read 2026-07-30)

### What a rename does — it breaks the reference, and there is a repair operator whose stated purpose is that case

> "Relocate Linked ID
>
> Reference
>
> Editor:
>
> Outliner
>
> Menu:
>
> Context menu ‣ ID Data ‣ Relocate
>
> A right-click a linked ID will show the Relocate option in the ID Data sub-menu. It allows to
> relocate a directly linked ID by another one, from the same library or from a different one.
> This can be used to either fix a broken linked data-block (e.g. because the ID was renamed),
> or to switch to a variation of the same data.
>
> Note
>
> This operation is only available on data-blocks that are directly linked, and are not
> a dependency of any other linked data."

— manual, https://docs.blender.org/manual/en/5.2/files/linked_libraries/link_append.html (read 2026-07-30)

The Python form:

> "bpy.ops.outliner.id_linked_relocate()
>
> Replace the active linked ID (and its dependencies if any) by another one, from the same or a different library"

— Python API, https://docs.blender.org/api/5.2/bpy.ops.outliner.html (read 2026-07-30)

Note the two things that quote settles at once: a rename in the library breaks the link ("fix a broken linked data-block (e.g. because the ID was renamed)"), and the repair is *not* available for indirectly linked data or for anything another linked thing depends on.

Also relevant to renames: Blender's own automatic renaming can move a name out from under a reference, because it appends a numeric suffix on collision:

> "When Blender has to name a new data-block or rename an existing one, it will check for name collisions. If a
> data-block with the same name already exists, the (re)named data-block will get a numeric extension added as a
> post-fix to its 'root name', like e.g. .001."
>
> "If the original root name is the same as in the new requested name, the renamed data-block gets the requested name,
> and the conflicting data-block is renamed accordingly.
>
> E.g. assuming that there are three objects named Sphere, Cube and Cube.001, renaming Cube.001 to Cube
> will rename the data-block to Cube, and the other data-block to Cube.001."

— manual, https://docs.blender.org/manual/en/5.2/files/data_blocks.html (read 2026-07-30). Note also the stated exception: "Blender will never rename another data-block when doing automatic naming."

### What happens when the thing is gone — a placeholder is made, and the reference survives

> "Broken Libraries
>
> If Blender cannot find a library while loading a blend-file,
> it will create placeholder data-blocks to replace missing linked ones.
> That way, references to the missing data are not lost, and by relocating the missing library,
> the lost data can be automatically restored."

— manual, https://docs.blender.org/manual/en/5.2/files/linked_libraries/link_append.html (read 2026-07-30)

The placeholder is flagged, and the flag is readable:

> "is_missing
>
> This data-block is a place-holder for missing linked data (i.e. it is [an override of] a linked data that could not be found anymore) (default False, readonly)"

— Python API, https://docs.blender.org/api/5.2/bpy.types.ID.html (read 2026-07-30)

Placeholder creation is a named option on the import machinery, so it is not unconditional:

> "USE_PLACEHOLDERS
> Generate a placeholder (empty ID) if not found in any library files."

— Python API, `BlendImportContext.options`, https://docs.blender.org/api/5.2/bpy.types.BlendImportContext.html (read 2026-07-30)

That wording — "if not found in **any library files**" — is the one place the sources address a single named thing being absent from a library that itself is present. The `Broken Libraries` passage addresses the whole library being absent. Both produce a placeholder.

The repair path at library level:

> "Relocate Library
>
> …
>
> When the Outliner is in the Blender File Display Mode,
> a right-click on a library (a linked blend-file) will show the Relocate option.
> It allows to replace it by a different file.
> This can be used to either fix a broken linked library (e.g. because the file was moved
> or renamed), or to switch to a variation of the same data in a different file."

— manual, https://docs.blender.org/manual/en/5.2/files/linked_libraries/link_append.html (read 2026-07-30)

### A related case: overrides, where a change in the library can delete local work

> "The relationships between linked data-blocks can change, resulting in outdated overrides.
> When this happens, overrides need to be resynced to match the new structure of their hierarchy.
> Overrides are automatically resynced if needed on blend-files opening."

> "Warning
>
> While resyncing a library override it is possible that edited overrides
> get deleted if they are changed in the original library.
> If this is the case, a warning message will be displayed stating how many overrides were deleted,
> if the deletion is undesirable the resync can be undone before saving the blend-file."

— manual, https://docs.blender.org/manual/en/5.2/files/linked_libraries/library_overrides.html (read 2026-07-30)

And the library-level flag for it:

> "needs_liboverride_resync
>
> True if this library contains library overrides that are linked in current blendfile, and that had to be recursively resynced on load (it is recommended to open and re-save that library blendfile then) (default False)"

— Python API, https://docs.blender.org/api/5.2/bpy.types.Library.html (read 2026-07-30)

**Not covered.** Whether a *rename inside a present library* produces a placeholder in the same way a missing library does — the `USE_PLACEHOLDERS` wording says "if not found in any library files", which covers it, but the manual's Broken Libraries paragraph is written only about a missing library, and no source states the rename case as behaviour. Whether anything is reported when a placeholder is created (a message, a count) is not stated; `is_missing` can be *queried*, but nothing says it is announced. Whether the placeholder renders as anything, or renders at all, is not stated here (see question 12). Whether `USE_PLACEHOLDERS` is on by default when a file is opened normally is not stated — the option is documented as read-only data exposed to import handlers.

**Outside the interface.** Mixed, and this matters.

- The *mechanism* is not interface-bound: `ID.is_missing`, `ID.library_weak_reference`, `bpy.data.libraries.load` by name are all Python.
- Placeholder creation on load is described as something Blender does "while loading a blend-file", with no interface qualifier.
- The *repairs* are interface-bound as documented. Both Relocate operations are given with `Editor: Outliner` and `Menu: Context menu ‣ …`, and `bpy.ops.outliner.lib_relocate()` is described as "Relocate the library under cursor" — a cursor being an interface thing. `bpy.ops.wm.lib_relocate` exists with a `library` string parameter, which does not require a cursor, but the sources do not say it works headless.
- The override-resync warning is "a warning message will be displayed", which is an interface place. Whether an equivalent reaches standard output in background mode, the sources do not say.

**Operating system.** The sources do not say. Nothing in this answer is qualified by operating system.


**Observed — hand test entries 7 and 8, run 2026-08-05 on the Creator's own computer** (MacBook Pro, M1 Pro chip, macOS 26.5.2, Apple's own graphics), with Blender 5.2.0 LTS started from a terminal with no window, using a throwaway settings folder set through `BLENDER_USER_RESOURCES`. The full list of tests is at `Audits/Blender Research/Dependencies/Hand tests.md`.

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


**Observed — hand test entry 23, run by the Creator on 2026-08-05 on their own computer** (MacBook Pro, M1 Pro chip, macOS 26.5.2, Apple's own graphics), in Blender 5.2.0 LTS with its window open. This is the half of the list that needs eyes on the screen; the whole list is at `Audits/Blender Research/Dependencies/Hand tests.md`. Because it was run on a Mac, it settles macOS and leaves Windows and Linux open.

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

## 12. What does a render do when something the file points at is not there, and does it say so anywhere?

**LABEL: mixed.** **documented** for four narrow cases. **observed** for what Cycles does with an image it cannot get. **not in the sources** for the general case — a missing texture, font, sound, cache or library at render time in the main pipeline — and for whether that is reported anywhere.

### Documented case 1 — a malformed path template fails the render, with a message

> "Note
>
> Depending on the path, template errors may prevent certain actions. For
> example, if there are errors in the render output path, then rendering an
> animation will fail with an error message indicating the path errors."
>
> "When there are template errors in a path, the path field will be highlighted red
> in the UI:"
>
> "Hovering over the path field will pop up a tooltip that contains a list of the
> template errors encountered in that path:"

— manual, https://docs.blender.org/manual/en/5.2/files/file_paths.html (read 2026-07-30)

Both screenshots on that page were opened. `_images/files_file_paths-invalid_path_template.png` shows the Output field filled dark red. `_images/files_file_paths-invalid_path_template_tooltip.png` shows the tooltip reading: "Output Path / Directory/name to save animations, # characters define the position and padding of frame numbers. / Value: //my_render_dir/{blend_name.png / Syntax error(s): / - Invalid or incomplete template expression '{blend_name.png'." The error lines are drawn in red. **documented** (the images are part of the manual). What is visible there is visible; nothing more is claimed from them.

### Documented case 2 — no valid output destination raises an error

> "When disabled, Blender does not write render output from this panel and instead
> relies on other systems. For animation renders, if the
> Video Sequencer is active and no alternative output is
> available, an error is raised. Otherwise, if compositing is enabled, active
> File Output Node nodes are used to save the result.
> If no such nodes exist, an error is reported."

— manual, https://docs.blender.org/manual/en/5.2/render/output/properties/output.html (read 2026-07-30)

> "If no valid output method is available, rendering operations will report errors."

— manual, https://docs.blender.org/manual/en/5.2/render/output/animation.html (read 2026-07-30)

### Documented case 3 — missing media in the sequencer renders magenta, or transparent, and by default transparent

> "Show Missing Media
>
> Render missing images/movies with a solid magenta color.
> When disabled, missing content will render fully transparent.
>
> Tip
>
> Strips with missing content will be displayed as red in the timeline."

— manual, https://docs.blender.org/manual/en/5.2/editors/video_sequencer/preview/sidebar.html (read 2026-07-30)

The default is off:

> "show_missing_media
>
> Render missing images/movies with a solid magenta color (default False)"

— Python API, https://docs.blender.org/api/5.2/bpy.types.SequenceEditor.html (read 2026-07-30)

So by default a missing sequencer strip renders as fully transparent, in the image, with no colour signal.

### Documented case 4 — a missing linked library becomes a placeholder at load

Quoted in full under question 11 ("Broken Libraries", and `ID.is_missing`). This is load-time behaviour, not render-time. What such a placeholder renders as is not stated.

### Documented — the check exists, but you have to run it

> "Report Missing Files
>
> This option is useful to check if there are links to unpacked files that no longer exist.
> After selecting this option, a warning message will appear in the Info editor's header.
> If no warning is shown, there are no missing external files."

— manual, https://docs.blender.org/manual/en/5.2/interface/window_system/topbar.html (read 2026-07-30)

> "bpy.ops.file.report_missing_files()
>
> Report all missing external files"
>
> "bpy.ops.file.find_missing_files(*, find_all=False, directory='', …)
>
> Try to find missing external files"

— Python API, https://docs.blender.org/api/5.2/bpy.ops.file.html (read 2026-07-30)

The wording matters: this is a check you invoke, not something a render does.

Where a message would land if one were emitted:

> "The Info editor logs the executed operators as well as errors, warnings,
> and informational messages."

— manual, https://docs.blender.org/manual/en/5.2/editors/info_editor.html (read 2026-07-30)

and a render engine has two channels — `RenderEngine.report(type, message)` "Report info, warning or error messages" and `RenderEngine.error_set(message)` "Set error message displaying after the render is finished" (Python API, https://docs.blender.org/api/5.2/bpy.types.RenderEngine.html, read 2026-07-30). Reports are readable from Python via `bpy.types.Report` (`message`, `type`), and the release notes add: "Blender reports are now exposed as a read-only list in WindowManager ID types. They also now have a session-wide unique identifier integer" (https://developer.blender.org/docs/release_notes/5.2/python_api/, read 2026-07-30).

### Observed — what Cycles does with an image it cannot get

**LABEL for this part: observed.** From Cycles' own files, there is a dedicated colour for it, and it is magenta.

> ```
> /* Color to use when images are not found. */
> #define IMAGE_MISSING_RGBA make_float4(1, 0, 1, 1)
> ```

— `Blender Sources/cycles_5.2/source/util/types_image.h`, lines 17–18 (read 2026-07-30)

It is returned wherever an image lookup cannot resolve:

> ```
>   if (image_texture_id == KERNEL_IMAGE_NONE) {
>     return IMAGE_MISSING_RGBA;
>   }
> ```

— `Blender Sources/cycles_5.2/source/kernel/device/gpu/image.h`, lines 93–94 (read 2026-07-30)

and, distinguishing a tile that failed to load from one merely not yet loaded:

> ```
>       return (tile_descriptor == KERNEL_TILE_LOAD_FAILED) ? IMAGE_MISSING_RGBA : tex.average_color;
> ```

— same file, line 113 (read 2026-07-30). The same constant is returned at lines 124 and 167 of that file, and at line 259 of `Blender Sources/cycles_5.2/source/kernel/util/image_3d.h`.

**The line this must not cross.** What those files *say* is that a constant named for missing images is defined as opaque magenta and returned on those branches. That a render of a scene with a missing texture actually comes out magenta is a step beyond the text, because I have not established which of those branches a real missing file takes, nor what happens before the kernel is reached. Stated as the inference it is: *if* a missing image file reaches those lookups, the sampled value is opaque magenta. Nothing in those files emits a report or a log line; the colour is the only signal at that level.

Elsewhere in the Cycles add-on, missing files on disk *are* reported — but only by its own operators, not by a render:

> ```
>                 if not os.path.isfile(filepath):
>                     scene.render.filepath = original_filepath
>                     err_msg = tip_("Frame '%s' not found, animation must be complete") % filepath
>                     self.report({'ERROR'}, err_msg)
>                     return {'CANCELLED'}
> ```

— `Blender Sources/cycles_5.2/operators.py`, lines 82–88 (read 2026-07-30), in the Denoise Animation operator.

> ```
>             report({'ERROR'}, rpt_("OSL query failed to open %s") % oso_path)
> ```

— `Blender Sources/cycles_5.2/osl.py`, line 312 (read 2026-07-30)

### Not in the sources — the general case

There is no statement anywhere in the 5.2 manual, API documentation or release notes about what a render does when an image texture, a font, a sound, a cache file, a linked library or a mesh-sequence cache referenced by the file is missing at render time, nor whether it says so anywhere. Specifically absent: any documented magenta or pink placeholder for missing *image textures* (the Image Texture node page says nothing about it); any documented exit code for a failed render (the only exit-code control documented is `--python-exit-code <code>`, "Set the exit-code in [0..255] to exit if a Python exception is raised (only for scripts executed from the command line)"); any statement of what is printed to standard output or standard error. `bpy.ops.render.render` is itself marked "Undocumented, consider contributing."

**Record of where I looked**, for that "not in the sources" claim. Grep terms run across `manual_5.2`, `api_5.2`, `release_notes_5.2` and `cycles_5.2`: `missing`, `missing file`, `Find Missing`, `Report Missing`, `magenta`, `pink`, `purple`, `checkerboard`, `checker`, `placeholder`, `cannot be found`, `could not be found`, `not found`, `does not exist`, `broken link`, `unresolved`, `no longer exist`, `cannot write`, `unable to write`, `write permission`, `read-only`, `fails`, `will fail`, `render will fail`, `an error`, `error message`, `status bar`, `System Console`, `Toggle System Console`, `console output`, `stdout`, `stderr`, `exit code`, `return code`, `batch render`, `background mode`, `report`, `has_reports`, `Wm Report Items`, `Report Type`, `is_missing`, `has_data`, `show_missing_media`, `error_set`, `render_write`, `write_still`, `IMAGE_MISSING_RGBA`, `Cannot open`, `failed to load`, `Failed to open`, `self.report`.

Pages opened and then ruled out: `render/shader_nodes/textures/image.txt` — this is the page where a documented missing-texture behaviour would live, and it is silent; that silence is the main basis for the claim. `editors/image/introduction.txt` (render slots are in-memory buffers, no disk path). `troubleshooting/3d_view.txt` (matched `unresolved`, but about depth-buffer precision). `physics/particles/emitter/cache.txt` and `physics/particles/mode.txt` (cross-references only). All of `release_notes_5.2/*` searched for `missing` and `path template`; the only relevant line is "Saving render output can now be disabled." in `rendering.md`. Also opened and used above: `interface/window_system/status_bar.txt`, `editors/info_editor.txt`, `bpy_types_enum_items/wm_report_items.txt`, `bpy.types.Report`, `bpy.types.RenderEngine`, `bpy.app.handlers`.

**Not covered.** Whether a render continues or aborts on a missing texture. Whether anything at all reaches the console. What a placeholder linked data-block renders as. What happens when a physics disk cache, fluid cache directory or geometry-nodes bake directory referenced by the file is absent at render time. Whether a render distinguishes "file missing" from "file present but unreadable". Whether a render output path that cannot be written to produces a report, and of what severity. Any exit code.

**Outside the interface.** This is where the answer is thinnest, and it matters most. The Status Bar and the Info editor are windows; in background mode there is none, and the sources do not say where reports go instead. What the sources do give for the command line:

> "-q, --quiet
>
> Suppress status printing (warnings & errors are still printed)."
>
> "--log-level <level>
>
> Set the logging verbosity level.
>
> fatal: Fatal errors only
> error: Errors only
> warning: Warnings
> info: Information about devices, files, configuration, operations
> debug: Verbose messages for developers
> trace: Very verbose code execution tracing"
>
> "--log-file <filepath>
>
> Set a file to output the log to."
>
> "--debug-exit-on-error
>
> Immediately exit when internal errors are detected."

— manual, https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html (read 2026-07-30)

For a machine watching a render from Python, the documented hooks are `bpy.app.handlers.render_write` — "on writing a render frame (directly after the frame is written)" — plus `render_complete`, `render_cancel`, `render_init` and `render_stats` (https://docs.blender.org/api/5.2/bpy.app.handlers.html, read 2026-07-30). Whether `render_cancel` fires on a missing-file failure, the sources do not say.

**Operating system.** Nothing makes the missing-file render behaviour itself operating-system specific. One display detail is Windows-only:

> "When Blender is started on a Microsoft Windows operating system, the Console Window
> (called the Command Prompt) is first created as a separate window on the desktop.
> The main Blender window will also appear and the Console Window will then be toggled off.
> To display the console again, go to Window ‣ Toggle System Console."

— manual, https://docs.blender.org/manual/en/5.2/advanced/command_line/launch/windows.html (read 2026-07-30)

So on Windows the console output exists but is hidden until toggled. Crash-log destinations also differ per operating system (https://docs.blender.org/manual/en/5.2/troubleshooting/crash.html). Beyond that, the sources do not say.


**Observed — hand test entries 4, 5, 6, 7, 16 and 17, run 2026-08-05 on the Creator's own computer** (MacBook Pro, M1 Pro chip, macOS 26.5.2, Apple's own graphics), with Blender 5.2.0 LTS started from a terminal with no window, using a throwaway settings folder set through `BLENDER_USER_RESOURCES`. The full list of tests is at `Audits/Blender Research/Dependencies/Hand tests.md`.

**What was done.** A file was built with a plane whose Base Color comes from an Image Texture node pointing at a solid green 32 by 32 PNG beside the blend-file, a camera, a sun, 32 by 32 output, Standard view transform and one Cycles sample. The PNG was moved aside and the file rendered from a terminal with

```
"<application>" --background --factory-startup <file>.blend --engine CYCLES --render-output <folder>/miss_CYCLES_ --render-frame 1
```

and again with `--engine BLENDER_EEVEE`. The centre pixel of each written frame was then read by loading it into a fresh Blender.

**What came back.** The render finished, a frame was written, the exit status was 0, and the plane came out magenta — in every version and both engines. Centre pixel with the texture missing: Cycles `(1.0, 0.1373, 1.0, 1.0)`, Eevee `(1.0, 0.1294, 1.0, 1.0)`. With the texture present the same pixel was `(0.1373, 1.0, 0.1373, 1.0)` and `(0.1294, 1.0, 0.1294, 1.0)`.

**What was said about it is where the versions part company.**

```
00:00.424  cycles           | WARNING Image file <path>/assets/green.png does not exist.
00:00.427  cycles           | ERROR Failed to load 1 image files
```

and EEVEE printed `gpu.texture      | ERROR Failed to create GPU texture from Blender image`.

In 4.5.12 the same test printed nothing at all from Cycles. 5.0.1 and 5.1.2 print one line, called an ERROR rather than a WARNING, and have no second line.

**A missing font, in the same shape of test.** A text object whose font file had been moved away still rendered, exit status 0, and the evaluated mesh dropped from 332 vertices and 334 polygons to 16 and 16 — so the letterforms are lost rather than falling back to the built-in font. What was printed:

```
00:00.415  geom.vfont       | WARNING Font file doesn't exist: //assets/testfont.ttf
```

four times, each preceded by two lines of `reports          | ERROR Unable to pack file, source path '<path>/assets/testfont.ttf' not found`. In 4.5.12 the same lines carry the file and line number of Blender's own source and also appear on a plain open.

**A missing linked library.** Covered under question 11 above: the load prints `Unable to open ... No such file or directory`, `Cannot find lib ...`, `LIB: Collection: 'PropCol' missing from ...` and `1 libraries and 1 linked data-blocks are missing ... please check the Info and Outliner editors for details`, all to the terminal, and the render still produces a frame.

**A missing volume file.** `grids.load()` returned `False` and `grids.error_message` was `'smoke.vdb not found'`.

**A partly-packed file with its files gone.** Nothing at all was printed when the file was opened. `bpy.ops.file.report_missing_files()` does work with no window, prints `Warning: Path '<path>/assets/loose.png' not found, from local data-block 'IMloose.png'` to the terminal, and returns `{'FINISHED'}`.

**The sequencer's missing-media switch.** On a sequence editor made with `scene.sequence_editor_create()`, `show_missing_media` came back **True**, while the property's declared default read from `bpy.types.SequenceEditor.bl_rna.properties["show_missing_media"].default` is **False**. Same in all four versions.

**Both standing lines, brought up to date by this test.** *Driven from outside:* settled, and this is the important part. A render with a missing dependency does not stop, does not change the exit status, and writes its frame; from 5.0 onward Cycles names the missing image on the terminal and in 4.5 it does not; a missing font and a missing library are named in every version. *Operating system:* macOS for this test. The same test was run on a rented Linux machine on 2026-08-06 and that result is recorded below; Windows stays untested.

**Still open.** What appears on screen — the Info editor, the status bar — is entry 21 on the hand-test list and is the Creator's to answer.


**Observed — hand test entry 21, run by the Creator on 2026-08-05 on their own computer** (MacBook Pro, M1 Pro chip, macOS 26.5.2, Apple's own graphics), in Blender 5.2.0 LTS with its window open. This is the half of the list that needs eyes on the screen; the whole list is at `Audits/Blender Research/Dependencies/Hand tests.md`. Because it was run on a Mac, it settles macOS and leaves Windows and Linux open.

**What was done.** A file prepared for this test was opened. It holds two planes: the left one uses an image that was packed into the file, the right one uses an image that was not, and neither image file exists on disk any more. The Creator looked at the two images in the Image editor and at the Image Texture node in the Shading workspace, then used File ‣ External Data ‣ Report Missing Files.

**What came back.** Four screenshots, one per version, and these words from the Creator:

> "Report Missing Files says this:
> Path '/Users/artist/Documents/Smoozy hand tests/Dependencies/4.5/21/assets/loose.png' not found, from local data-block 'IMloose.png'
> Obviously 4.5 is for version 4.5 here, but the message is the same in all versions."

So the version's own folder appears in the path and the rest of the message is the same in all four. It is the same wording as the warning printed to the terminal when the same operator is called with no window.

The screenshots show the same three things in all four versions. In the image browse list, `packed.png` carries a small blue preview of itself and `loose.png` carries none. In the 3D viewport the plane using the packed image is blue — its own colour — and the plane using the unpacked one is bright magenta. In the Image editor the unpacked image shows an empty checkerboard.

**What the answer does not cover.** The entry also asked what sits to the left of each image's file-path field, to see whether the packed one carries a marker the unpacked one does not. In all four screenshots the image selected is `loose.png`, the unpacked one, and its row carries no such marker; what the packed image's row looks like was not photographed and is not reported. That part stays open.

**Both standing lines, brought up to date by this test.** *Driven from outside:* the wording of the report is now known to be the same with a window as without one, which the sources did not say. What the sources describe as landing in the Info editor is a window-only route; with no window the same text goes to the terminal. *Operating system:* macOS for this test. The same test was run on a rented Linux machine on 2026-08-06 and that result is recorded below; Windows stays untested.

**Observed — hand test entry 22, run by the Creator on 2026-08-05 on their own computer** (MacBook Pro, M1 Pro chip, macOS 26.5.2, Apple's own graphics), in Blender 5.2.0 LTS with its window open. This is the half of the list that needs eyes on the screen; the whole list is at `Audits/Blender Research/Dependencies/Hand tests.md`. Because it was run on a Mac, it settles macOS and leaves Windows and Linux open.

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

**Observed — hand test entry 24, run by the Creator on 2026-08-05 on their own computer** (MacBook Pro, M1 Pro chip, macOS 26.5.2, Apple's own graphics), in Blender 5.2.0 LTS with its window open. This is the half of the list that needs eyes on the screen; the whole list is at `Audits/Blender Research/Dependencies/Hand tests.md`. Because it was run on a Mac, it settles macOS and leaves Windows and Linux open.

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


**Observed — run again on Linux, 2026-08-06.** Every missing-thing case was built and run again in this version on the rented Linux machine.

**What a render does is the same on both machines.** With a missing image texture, a missing font, a missing linked library or a missing volume, the render finished, a frame was written, and the exit status was 0 in every case. The plane whose texture was missing came out magenta.

**What it says is also the same on both machines.** Cycles printed two lines rather than one: `00:00.753  cycles           | WARNING Image file /root/…/assets/green.png does not exist.` followed by `00:00.758  cycles           | ERROR Failed to load 1 image files`. EEVEE printed `gpu.texture      | ERROR Failed to create GPU texture from Blender image`. The engine's own name on the command line is `BLENDER_EEVEE` here. The same as on the Mac.

The rest, all matching the Mac: a missing font still renders with the letterforms lost rather than falling back to the built-in font; a missing linked library still renders a frame with placeholders in place of what is gone; a missing volume answers `grids.load() = False` and `grids.error_message = 'smoke.vdb not found'`; a partly-packed file whose files are gone prints nothing at all when opened; and on a sequence editor made from a script the missing-media switch reads **on** while the property's declared default is **off**.

Report Missing Files works with no window on Linux and prints the same warning naming the path and the data-block. `WindowManager.reports` exists here and came back as an **empty list**, with a window open as well as without one — so even with an interface, the missing-files warning does not land in that collection.

**Find Missing Files, which no Mac test reached.** It works with no window, returns `{'FINISHED'}`, **rewrites every repaired path as a full absolute path into the search folder**, and leaves alone the paths that already resolve. It does **not** match a name case-insensitively: with a file named `texture.png` in the search folder and the reference written `Texture.PNG` it printed `Warning: Could not find 'Texture.PNG' in '…'` and changed nothing.

**A broken output-path template, with a window open on the Linux machine.** In 5.0, 5.1 and 5.2 the File Path field is filled **solid red** with the text in white on the red, and the terminal carries `operator | ERROR Parse errors in path '//out_{blend_name.png': - Invalid or incomplete template expression '{blend_name.png'.` In 4.5 the Output panel sat below the visible part of the Properties editor and was not photographed on Linux.

**Both standing lines, brought up to date by the Linux tests.** *Driven from outside:* every reading above was taken with Blender running with no window unless it says otherwise, so it holds outside the interface. *Operating system:* **now settled on two operating systems, macOS and Linux, and each is its own result** — what they agree on and where they differ is said above. **Windows is still untried: no Windows machine has run it.**

---

## 13. Does a file pointing at another `.blend` file bring that file's own references with it, how deep does that go, and can they be reached the same way?

**LABEL: documented** for "yes, it brings them" and for "no, they cannot be reached the same way". **not in the sources** for a stated depth limit.

### Yes — a library's own libraries come with it, and 5.2 exposes that list

> "A libraries attribute, which for the input is a list of all other libraries
> used by the loaded one, as named tuples (filepath, is_archive). The
> filepath may be absolute, or relative to the loaded blend-file."

— Python API, `BlendDataLibraries.load`, https://docs.blender.org/api/5.2/bpy.types.BlendDataLibraries.html (read 2026-07-30)

The release notes state that this is new, and show it returning several libraries at once for one loaded file:

> "## API Improvements to `libraries.load()`
>
> Some limited library info is now exposed to the `bpy.data.libraries.load()` [input context manager](https://docs.blender.org/api/5.2/bpy.types.BlendDataLibraries.html#bpy.types.BlendDataLibraries.load), giving in particular access to the blend-file paths of the libraries used by the loaded one (blender/blender!157066, blender/blender@646199c89a)."

— release notes, https://developer.blender.org/docs/release_notes/5.2/python_api/ (read 2026-07-30)

The thing that comes with it has a name — *indirectly linked* — and a flag:

> "is_library_indirect
>
> Is this ID block linked indirectly (default False, readonly)"

— Python API, https://docs.blender.org/api/5.2/bpy.types.ID.html (read 2026-07-30)

> "INDIRECT_USAGE
> That item was added for an indirectly imported ID, as a dependency of another data-block."

— Python API, `BlendImportContextItem.import_info`, https://docs.blender.org/api/5.2/bpy.types.BlendImportContextItem.html (read 2026-07-30)

Libraries know their parent:

> "parent
>
> (readonly)
>
> Type:
>
> Library | None"

— Python API, https://docs.blender.org/api/5.2/bpy.types.Library.html (read 2026-07-30)

And appending has an explicit switch for whether to follow the chain and localise it:

> "Localize All
>
> Also copy all indirectly linked data, instead of maintaining the links."

— manual, https://docs.blender.org/manual/en/5.2/files/linked_libraries/link_append.html (read 2026-07-30)

> "recursive (bool) – If True, also make indirect dependencies of appended libraries local."

— Python API, https://docs.blender.org/api/5.2/bpy.types.BlendDataLibraries.html (read 2026-07-30)

> "APPEND_RECURSIVE
> Append (make local) also indirect dependencies of appended IDs coming from other libraries. NOTE: All IDs (including indirectly linked ones) coming from the same initial library are always made local."

— Python API, https://docs.blender.org/api/5.2/bpy.types.BlendImportContext.html (read 2026-07-30)

That NOTE is worth reading twice: within one library, indirect data is *always* made local on append; the switch only governs data coming from libraries beyond it.

### How deep

No source states a depth, a limit, or a level count. What the sources do say is that the structures are recursive in shape and that resync is recursive in operation:

> "needs_liboverride_resync
>
> True if this library contains library overrides that are linked in current blendfile, and that had to be recursively resynced on load…"

— Python API, https://docs.blender.org/api/5.2/bpy.types.Library.html (read 2026-07-30)

> "Blender is also able to resync library overrides from external libraries, that are then linked into a
> working file. However, this is a costly process that needs to be fully redone every time the working
> file is loaded, since Blender cannot edit/modify the external library directly."

— manual, https://docs.blender.org/manual/en/5.2/files/linked_libraries/library_overrides.html (read 2026-07-30)

The only stated limit is on direction, not depth:

> "Circular Dependencies
>
> In general, dependencies should not go in both directions.
> Attempting to link or append data which links back to the current file will likely result in missing links."

— manual, https://docs.blender.org/manual/en/5.2/files/linked_libraries/link_append.html (read 2026-07-30)

A parallel, for a different kind of chain, is stated outright — background scenes:

> "Background Scenes can themselves have a Background Scene (they're recursively included)."

— manual, https://docs.blender.org/manual/en/5.2/scene_layout/scene/properties.html (read 2026-07-30)

I record that as a parallel, not as an answer about libraries.

### Can they be reached the same way? No.

Three documented differences between direct and indirect:

**(a) The repair operator refuses them.**

> "Note
>
> This operation is only available on data-blocks that are directly linked, and are not
> a dependency of any other linked data."

— manual, Relocate Linked ID, https://docs.blender.org/manual/en/5.2/files/linked_libraries/link_append.html (read 2026-07-30)

**(b) They are flagged differently in the data.** `ID.is_library_indirect`, quoted above.

**(c) They are drawn differently.** 5.2 added an icon specifically for it:

> "  * `PACKAGE_INDIRECT`. A dimmed version of `PACKAGE` used by indirectly used packed data-blocks. (blender/blender@5d7403c8ce23)"

— release notes, https://developer.blender.org/docs/release_notes/5.2/user_interface/ (read 2026-07-30)

There is also a case where indirect libraries are *not listed at all* — when the data has been packed:

> "Packed linked data-blocks are not linkable and are not listed in the input
> libraries attribute."

— Python API, https://docs.blender.org/api/5.2/bpy.types.BlendDataLibraries.html (read 2026-07-30)

Against those differences, what *is* the same: all linked data-blocks, direct or indirect, live in `bpy.data` and are reachable through it —

> "all_ids
>
> Read-only list of all IDs listed in Blender data-base (default None, readonly)"

— Python API, https://docs.blender.org/api/5.2/bpy.types.BlendData.html (read 2026-07-30), described in the release notes as "allowing to iterate over all data-blocks at once" (https://developer.blender.org/docs/release_notes/5.2/python_api/)

and both are shown in the same Outliner view:

> "Blender File:
>
> Lists all data in the current blend-file."

— manual, https://docs.blender.org/manual/en/5.2/editors/outliner/interface.html (read 2026-07-30)

**Not covered.** How deep the chain may go: **not in the sources**. Record of the search behind that claim — I grepped the whole manual and the whole API for `nested librar`, `chain of librar`, `library of librar`, `depth limit`, `levels of link`, and `recursive`. The only `recursive` hits relevant to libraries are the resync passages quoted above and the background-scene sentence; none states a depth or a limit. The nearest thing to a limit anywhere in the 5.2 release notes is about a different structure entirely — "The maximum level of nesting for IDProperties is now set to 1024 levels" (https://developer.blender.org/docs/release_notes/5.2/python_api/) — which is not about libraries and I do not offer it as an answer.

Also not covered: whether the whole of an indirectly-referenced library file is read into memory. The one memory statement is about linking generally, not about depth: "Referencing compressed blend-files may need a lot of memory because they have to be loaded in their entirety, even if you only link/append a small part of them. Once the data-blocks are loaded, however, memory usage is the same." (same link_append page). And whether a relative path recorded *inside* a library resolves against that library rather than the opened file — `bpy.path.abspath`'s `library` parameter and `blend_dir_lib` both point that way (question 5), but no source states it as the resolution rule for loading.

**Outside the interface.** Mostly holds, with one exception. The whole `libraries.load` mechanism, the `is_library_indirect` flag, `Library.parent`, and the `APPEND_RECURSIVE` option are Python and have no stated interface dependency; the release notes present the `libraries.load` improvement as tooling for scripts. The exception is the reach-them-the-same-way part: Relocate Linked ID is documented under `Editor: Outliner` / `Menu: Context menu`, and the `PACKAGE_INDIRECT` difference is by definition an interface one. So the *distinction* between direct and indirect exists in the data and is readable headless; the *repair* is documented only in the interface.

**Operating system.** The sources do not say. Nothing in this answer is qualified by operating system.


**Observed — hand test entry 9, run 2026-08-05 on the Creator's own computer** (MacBook Pro, M1 Pro chip, macOS 26.5.2, Apple's own graphics), with Blender 5.2.0 LTS started from a terminal with no window, using a throwaway settings folder set through `BLENDER_USER_RESOURCES`. The full list of tests is at `Audits/Blender Research/Dependencies/Hand tests.md`.

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


**Observed — hand test entry 23, run by the Creator on 2026-08-05 on their own computer** (MacBook Pro, M1 Pro chip, macOS 26.5.2, Apple's own graphics), in Blender 5.2.0 LTS with its window open. This is the half of the list that needs eyes on the screen; the whole list is at `Audits/Blender Research/Dependencies/Hand tests.md`. Because it was run on a Mac, it settles macOS and leaves Windows and Linux open.

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

The library list `bpy.data.libraries.load()` hands back was not exercised again on Linux; that half is as recorded above.


**Both standing lines, brought up to date by the Linux tests.** *Driven from outside:* every reading above was taken with Blender running with no window unless it says otherwise, so it holds outside the interface. *Operating system:* **now settled on two operating systems, macOS and Linux, and each is its own result** — what they agree on and where they differ is said above. **Windows is still untried: no Windows machine has run it.**

---

## 14. Does rendering one scene of a file pull only that scene's references?

**LABEL: not in the sources**, for the question as asked. What *is* documented is that a scene's references are not confined to that scene — I set that out below, with the record of where I looked.

### What is documented: a scene can be selected for render

> "-S, --scene <name>
>
> Set the active scene <name> for rendering."

— manual, https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html (read 2026-07-30)

> "scene (str) – Scene, Scene to render, current scene if not specified (optional, never None)"

— Python API, `bpy.ops.render.render`, https://docs.blender.org/api/5.2/bpy.ops.render.html (read 2026-07-30)

> "Render Image
>
> Render the active scene at the current frame."

— manual, https://docs.blender.org/manual/en/5.2/interface/window_system/topbar.html (read 2026-07-30)

### What is documented: one scene can reach into others, three ways

**Background scene:**

> "Background Scene
>
> Allows you to use a scene as a background,
> this is typically useful when you want to focus on animating the foreground for example,
> without background elements getting in the way."
>
> "Background Scenes can themselves have a Background Scene (they're recursively included)."

— manual, https://docs.blender.org/manual/en/5.2/scene_layout/scene/properties.html (read 2026-07-30)

**Scene strips in the sequencer:**

> "Scene strips are a way to insert the render output of another scene into your sequence.
> Instead of rendering out a video, then inserting the video file, you can insert the scene directly."

— manual, https://docs.blender.org/manual/en/5.2/video_editing/edit/montage/strips/scene.html (read 2026-07-30)

> "When used as strips, scenes are rendered dynamically during playback or rendering,
> and they share the same data-blocks unless they were created as full copies."

— manual, https://docs.blender.org/manual/en/5.2/scene_layout/scene/introduction.html (read 2026-07-30)

**The Render Layers compositor node, which names a scene:**

> "Properties
>
> Scene
>
> The scene for which to render a view layer."

— manual, https://docs.blender.org/manual/en/5.2/compositing/types/input/scene/render_layers.html (read 2026-07-30)

### What is documented: scenes share data by default

> "Linked Copy
>
> Creates a new scene that shares all data with the current one.
> The new scene contains links to the same collections, objects, and data-blocks as the original.
> Changes made in one scene automatically affect the other, since they reference the same data."
>
> "Full Copy
>
> Creates a fully independent scene with unique copies of all objects and their data."

— manual, https://docs.blender.org/manual/en/5.2/scene_layout/scene/introduction.html (read 2026-07-30)

> "Each blend-file contains a database.
> This database contains all scenes, objects, meshes, textures, etc. that are in the file."

— manual, https://docs.blender.org/manual/en/5.2/files/introduction.html (read 2026-07-30)

### Why the question itself is not answered

Nothing in the manual, the API documentation, the release notes or the Cycles files says what a render loads, opens, or reads from disk, scoped by scene. The sources describe what a scene *is* and what it can reference; they never describe the loading behaviour.

**Record of where I looked, for the "not in the sources" claim.** I grepped the whole of `manual_5.2` and `api_5.2`, case-insensitively, for each of: `only the active scene`, `active scene`, `scene to render`, `unused scenes`, `other scenes`, `all scenes are`, `loads all`, `whole file`, `entire blend`, `on demand`, `on-demand`, `lazily`, `lazy load`, `only loads`, `loaded in their entirety`, `entire file is loaded`, `loads the whole`. Nothing describes per-scene loading. Pages I opened and read in full or in substantial part and then ruled out: `scene_layout/scene/introduction.txt` (describes scene creation and sharing, not loading), `scene_layout/scene/properties.txt` (background scene), `video_editing/edit/montage/strips/scene.txt` (scene strips), `video_editing/sequencer_scene.txt`, `compositing/types/input/scene/render_layers.txt`, `advanced/command_line/render.txt` (the whole page — it describes arguments, never what gets loaded), `advanced/command_line/arguments.txt` (the render-options section in full), `files/introduction.txt`, `files/data_blocks.txt` (life-time and user counting, not loading), `editors/outliner/interface.txt` (Unused Data and Purge — about deletion, not selective loading), `files/linked_libraries/link_append.txt` (the memory note is about linking, not rendering). In the API I checked `bpy.ops.render`, `bpy.types.Scene`, `bpy.types.SceneStrip`, `bpy.types.BlendData` and `bpy.ops.wm` (open_mainfile) for any statement about scoped loading; there is none. The only near-miss is the compressed-file memory note quoted under question 13, which is about linking a library, not about rendering a scene.

**Ambiguity, flagged.** "That scene's references" can be read two ways, and I have answered the first. Reading A: does Blender read from disk only the external files that scene needs? Reading B: does the render only *use* that scene's data, ignoring other scenes' data in the same file? Reading B is partly answered by the quotes above — a render of one scene can reach into another scene, so even in Reading B the answer is "not necessarily". I chose Reading A because the sheet is about what a file points at outside itself.

**Not covered.** Everything the label says. In particular: whether an image used only by a non-rendered scene is opened during a render; whether a linked library used only by a non-rendered scene is loaded when the file is opened; and whether `-S` changes any of that.

**Outside the interface.** The scene-selection mechanism is documented for outside use — `-S, --scene` is a command-line argument and `bpy.ops.render.render(scene=…)` takes a scene name. But since the question itself is not answered, there is nothing here for which "outside the interface" could differ. The sources do not say.

**Operating system.** The sources do not say.


**Observed — hand test entry 12, run 2026-08-05 on the Creator's own computer** (MacBook Pro, M1 Pro chip, macOS 26.5.2, Apple's own graphics), with Blender 5.2.0 LTS started from a terminal with no window, using a throwaway settings folder set through `BLENDER_USER_RESOURCES`. The full list of tests is at `Audits/Blender Research/Dependencies/Hand tests.md`.

**What was done.** One file was built with two scenes. SceneA uses only `a.png`, SceneB uses only `b.png`; each has its own camera, sun and material, and both scenes were set to Cycles so no engine had to be named on the command line. `b.png` was moved out of the way, and the file rendered twice:

```
"<application>" --background --factory-startup <file>.blend --scene SceneA --render-output <folder>/onlyA_ --render-frame 1
"<application>" --background --factory-startup <file>.blend --scene SceneB --render-output <folder>/onlyB_ --render-frame 1
```

**What came back.** Rendering SceneA said nothing about `b.png`, in any version.

The control worked: rendering SceneB in the same file did name it — `cycles           | WARNING Image file <path>/assets/b.png does not exist.` and `cycles           | ERROR Failed to load 1 image files` — so the silence when SceneA is rendered is a real result and not a version that keeps quiet about everything. The same control in 4.5.12 said nothing either, so the test settles nothing there.

This is one-sided evidence: it shows that a render of one scene does not complain about the other scene's missing file. It does not show that the file was never opened. A second attempt at a stronger reading was made and abandoned — reading `image.has_data` and `image.size` after rendering SceneA reported `has_data=False` for both images, and reading `size` returned `(32, 32)` for both because reading `size` itself loads the file. Neither can tell whether a render opened a file. A file-activity trace on macOS needs an administrator password, so it was not used.

**Both standing lines, brought up to date by this test.** *Driven from outside:* the whole test was done with no window, and `--scene` chose the scene there. *Operating system:* macOS for this test. The same test was run on a rented Linux machine on 2026-08-06 and that result is recorded below; Windows stays untested.


**Observed — run again on Linux, 2026-08-06.** A two-scene file was built and rendered again in this version on the rented Linux machine. **Rendering SceneA said nothing about SceneB's missing image**, the same as on the Creator's Mac. The control — rendering the *other* scene, where the missing file certainly is reached — did name it: `cycles | WARNING Image file /root/…/assets/b.png does not exist.` followed by `cycles | ERROR Failed to load 1 image files`. So the silence in the first run means something here.

The evidence is one-sided on Linux for the same reason it was on the Mac: it shows that a render of one scene does not complain about the other scene's missing file, not that the file was never opened. **Nothing about this question is settled by the second machine.**

**Both standing lines, brought up to date by the Linux tests.** *Driven from outside:* every reading above was taken with Blender running with no window unless it says otherwise, so it holds outside the interface. *Operating system:* **now settled on two operating systems, macOS and Linux, and each is its own result** — what they agree on and where they differ is said above. **Windows is still untried: no Windows machine has run it.**

---

## 15. Can a file rely on a colour management profile that lives on the computer rather than inside the file, and how does Blender find it?

**LABEL: documented** for the mechanism and the lookup order. **inferred** for the step "and therefore the file depends on it" — the step is written out below.

### Yes. Blender ships one, and one on the computer can replace it

> "Blender comes with a standard OpenColorIO configuration that
> contains a number of useful display devices and view transforms.
>
> However, OpenColorIO is also designed to give a consistent user experience across
> multiple applications,
> and for this, a single shared configuration file can be used.
> Blender will use the standard OCIO environment variable to read an OpenColorIO configuration
> other than the default Blender one. More information about how to set up such a workflow
> can be found on the OpenColorIO website.
>
> There is also a BLENDER_OCIO environment variable to change the configuration
> for Blender only. It is recommended to use OCIO when possible for compatibility
> with other software and pipelines that may not be aware of BLENDER_OCIO. However
> sometimes there are incompatibilities in configuration files that make it difficult
> to share them across applications."

— manual, https://docs.blender.org/manual/en/5.2/render/color_management/opencolorio.html (read 2026-07-30)

### How Blender finds it — the precedence, stated outright

> "BLENDER_OCIO:
>
> Path to override the OpenColorIO configuration file.
> If not set, the OCIO environment variable is used."

— manual, https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html (read 2026-07-30)

And where the bundled one sits when neither variable is set:

> "./datafiles
>
> Data files loaded at runtime.
>
> Located in both user and system directories. User data files either override
> or add to system data files.
>
> ./datafiles/colormanagement
>
> Default OpenColorIO configuration."

— manual, https://docs.blender.org/manual/en/5.2/advanced/blender_directory_layout.html (read 2026-07-30). The same page gives the per-operating-system system directories (quoted under "Operating system" below), and the command-line page adds `BLENDER_SYSTEM_DATAFILES` — "Directory to replace bundled datafiles."

So the lookup order documented is: `BLENDER_OCIO`, else `OCIO`, else the bundled configuration under `./datafiles/colormanagement`.

### What the file itself holds is names, resolved against whatever config is active

> "The color spaces available in Blender depend on the active OCIO config.
> The default supported color spaces are described in detail at
> Default OpenColorIO Configuration."

— manual, https://docs.blender.org/manual/en/5.2/glossary/index.html (read 2026-07-30). The same sentence, shortened to "The list of color spaces depends on the active OCIO config", appears on https://docs.blender.org/manual/en/5.2/editors/image/image_settings.html, https://docs.blender.org/manual/en/5.2/files/media/image_formats.html and https://docs.blender.org/manual/en/5.2/movie_clip/movie_clips.html (all read 2026-07-30).

Corroborated by the API, where the display and view enums have no fixed members at all:

> "display_device
>
> Display name. For viewing, this is the display device that will be emulated by limiting the gamut and HDR colors. For image and video output, this is the display space used for writing. (default 'NONE')
>
> Type:
>
> Literal['NONE']"

— Python API, https://docs.blender.org/api/5.2/bpy.types.ColorManagedDisplaySettings.html (read 2026-07-30); `ColorManagedViewSettings.view_transform` is the same shape.

### The strongest single statement — a per-file flag whose whole meaning is "the config that made this file is missing"

> "class bpy.types.BlendFileColorspace(bpy_struct)
>
> Information about the color space used for data-blocks in a blend file
>
> is_missing_opencolorio_config
>
> A color space, view or display was not found, which likely means the OpenColorIO config used to create this blend file is missing (default False, readonly)"

— Python API, https://docs.blender.org/api/5.2/bpy.types.BlendFileColorspace.html (read 2026-07-30)

It hangs off the file's own data:

> "colorspace
>
> Information about the color space used for data-blocks in a blend file (readonly, never None)"

— Python API, https://docs.blender.org/api/5.2/bpy.types.BlendData.html (read 2026-07-30)

### The one colour-management thing that does *not* depend on an outside config

> "working_space
>
> Color space used for all scene linear colors in this file, and for compositing, shader and geometry nodes processing (readonly)
>
> Linear Rec.709
> Linear Rec.709 – Linear BT.709 with illuminant D65 white point.
>
> Linear Rec.2020
> Linear Rec.2020 – Linear BT.2020 with illuminant D65 white point.
>
> ACEScg
> ACEScg – Linear AP1 with ACES white point."

— Python API, https://docs.blender.org/api/5.2/bpy.types.BlendFileColorspace.html (read 2026-07-30)

> "The working space affects the colors of all data-blocks in a file, and has a significant effect on
> rendering and compositing results. Generally the working space should be chosen at the start of a
> project and used for all blend files."

— manual, https://docs.blender.org/manual/en/5.2/render/color_management/color_spaces.html (read 2026-07-30)

### The inferred step, written out

No source says in one sentence "a blend-file depends on an external OpenColorIO configuration". A reader must join two things: the set of colour spaces, displays and views is whatever the active config offers; and Blender exposes a per-file flag whose stated meaning is that a colour space, view or display named in this file was not found, which "likely means the OpenColorIO config used to create this blend file is missing". From those two it follows that the file records names, and that resolving them is done against a configuration living on the computer. That joining is mine, not the source's.

### What a hand-installed config must supply

> "OpenColorIO configurations do not strictly specify all information needed for Blender to work optimally.
> These guidelines help ensure a configuration works well:
>
> Use OpenColorIO v2 display_colorspaces and view_transforms with cie_xyz_d65_interchange
> intermediate display linear space. This is used to emulate the chosen display on the actual display.
> In older configurations without this, display emulation will be disabled and wide gamut and HDR display will
> not work well. Saving colorspace metadata for images and video also depends on this.
>
> For every display, include a view transform without tone mapping. Blender will look for a view
> transform named Standard or Un-tone-mapped or the config wide default_view_transform.
> If not found, the first view transform of the display will be used."

— manual, https://docs.blender.org/manual/en/5.2/render/color_management/opencolorio.html (read 2026-07-30)

That last sentence is the only documented fallback behaviour anywhere in this answer, and it is about a missing *view transform within a present config*, not a missing config.

### ICC profiles

The only mention of ICC in the whole of the 5.2 sources:

> "The icc_profile_name interchange attribute is supported, to embed ICC profiles when saving images."

— same page (read 2026-07-30). That is about writing images out, not about the file reading a profile off the computer. An operating-system display profile appears only as advice to the user: "Enabling 'Automatically manage color for apps' on Windows or using a good display profile can correct for this. On macOS, the default configuration is already set up for accurate color gamut." (manual, https://docs.blender.org/manual/en/5.2/render/color_management/system_configuration.html, read 2026-07-30).

**Not covered.** What Blender actually *does* when a named colour space, view or display is absent — whether the value is substituted, reset, or kept as text, and whether rendering fails. The flag exists; the behaviour is not written down. Whether `BLENDER_OCIO` / `OCIO` take the config file itself or the folder containing it: the two statements disagree in emphasis ("Path to override the OpenColorIO configuration file" versus "a single shared configuration file can be used") and no example path is given. When the config is read — once at startup, or again when a file loads. What happens if the path is set but the file is missing or malformed. Whether the config's name or path is written into the file: nothing says it is; the only per-file record documented is `working_space`, `working_space_interop_id` and the missing-config flag. Whether a user-supplied config in the *user* `datafiles/colormanagement` folder overrides the bundled one — the "user data files either override or add to system data files" sentence is stated for `./datafiles` generally, not for colour management.

For completeness, the whole of the 5.2 colour-management release note is: "Color space are now organized into menus, based on the `Family` attribute in OpenColorIO configurations. (blender/blender!151678) / New input color spaces for Apple, ARRI, Blackmagic Design, Canon and Sony cameras. (blender/blender!155619) / New color spaces for Adobe RGB, common gamma values, and wide gamut textures. (blender/blender!157702)" — https://developer.blender.org/docs/release_notes/5.2/rendering/ (read 2026-07-30). None of it changes the lookup.

**Outside the interface.** Partly documented. Both variables are environment variables of the process, documented under "Environment Variables" on the command-line arguments page, alongside `--env-system-datafiles`. So the mechanism is reachable from outside. Nothing says the lookup differs in background mode. `is_missing_opencolorio_config` is a Python property and so is readable from a script; nothing says it is printed anywhere.

Where it plainly must differ is the display half, and there the sources are silent. The display setting has two jobs at once — "For viewing, this is the display device that will be emulated… For image and video output, this is the display space used for writing." The writing half is what matters headless. Whether the viewing half is simply skipped when there is no window, no source says.

**Operating system.** Documented differences. The bundled config's location differs:

> "Linux
>
> Archive downloaded from blender.org:
>
> ./5.2/
>
> Linux distribution packages:
>
> /usr/share/blender/5.2/
>
> macOS
>
> ./Blender.app/Contents/Resources/5.2/
>
> Windows
>
> Zip file downloaded from blender.org:
>
> ./5.2/
>
> Installer downloaded from blender.org:
>
> %ProgramFiles%\Blender Foundation\Blender\5.2\"

— manual, https://docs.blender.org/manual/en/5.2/advanced/blender_directory_layout.html (read 2026-07-30)

And the display side is heavily per-operating-system:

> "macOS: Available on any Apple Silicon device.
>
> Linux: Use Wayland, and set the Vulkan backend in the Blender system preferences.
>
> Windows:
>
> For wide gamut, enable "Automatically manage color for apps" in the Windows
> display settings.
>
> For HDR, enable "Use HDR" in the Windows display settings.
>
> Set the Vulkan backend in the Blender system preferences."

— manual, https://docs.blender.org/manual/en/5.2/render/color_management/system_configuration.html (read 2026-07-30)

The two environment variables themselves carry no per-operating-system qualifier, unlike neighbouring entries on the same page which do ("SPNAV_SOCKET … (Unix only)", "TEMP … (MS-Windows)", "TMPDIR … (UNIX Systems)"). That absence is suggestive but is not a statement, so: the sources do not say explicitly that `OCIO` and `BLENDER_OCIO` behave identically on all three.

**Record of where the helper looked.** Grep terms across all four 5.2 source sets: `OpenColorIO`, `OCIO`, `colormanagement`, `color management`, `ICC`, `config`, `environment variable`, `working space`, `is_missing_opencolorio_config`, `missing_opencolorio`, `OpenColorIO config used to create`, `display_device`, `view_transform`, `colorspace`, `color_space`, `system_resource`, `DATAFILES`. Files read: `render/color_management/opencolorio.txt`, `system_configuration.txt`, `color_spaces.txt`, `displays_views.txt`, `index.txt`; `advanced/blender_directory_layout.txt` in full; the environment-variable section of `advanced/command_line/arguments.txt` in full; `bpy.types.BlendFileColorspace`, `bpy.types.ColorManagedDisplaySettings`, `bpy.types.ColorManagedViewSettings`, `bpy.types.ColorManagedInputColorspaceSettings`, `bpy.types.ColorManagedSequencerColorspaceSettings`, `bpy.types.BlendData`, `bpy.app`; release notes `rendering.md`, `core.md`, `pipeline_io.md`, `python_api.md`. Ruled out: `glossary/index.txt`, `editors/image/image_settings.txt`, `files/media/image_formats.txt`, `movie_clip/movie_clips.txt`, `compositing/types/color/convert_to_display.txt`, `convert_colorspace.txt`, `render/shader_nodes/textures/environment.txt`, `video_editing/edit/montage/strips/strip_properties.txt` — all repeat "the list of color spaces depends on the active OCIO config" and give no lookup mechanism. The whole `troubleshooting/` tree was listed; there is no colour-management page. `cycles_5.2` grepped for `ocio`, `colorspace`, `display_device`, `view_transform`; the only hits are a `--colorspace` argument in `maketx.py` and a kernel header include — nothing about config discovery, so no observed finding is available. `manual_5.2/_images/` searched for colour-management figures; only three exist and all are conceptual diagrams, not screenshots, so none could show a missing-config warning.


**Observed — hand test entry 19, run 2026-08-05 on the Creator's own computer** (MacBook Pro, M1 Pro chip, macOS 26.5.2, Apple's own graphics), with Blender 5.2.0 LTS started from a terminal with no window, using a throwaway settings folder set through `BLENDER_USER_RESOURCES`. The full list of tests is at `Audits/Blender Research/Dependencies/Hand tests.md`.

**What was done.** A small OpenColorIO configuration was written holding one colour space `TinyLinear`, one display `TinyDisplay` and one view `TinyView`, saved as `tiny.ocio`. Each version was then started four ways with no window: with nothing set; with `OCIO` pointing at `tiny.ocio`; with `BLENDER_OCIO` pointing at it; and with `BLENDER_OCIO` pointing at a path that does not exist. Each time both environment variables as Blender sees them, `bpy.data.colorspace.is_missing_opencolorio_config`, the list of colour spaces offered, the scene view transform and a new image's colour space were printed. Separately, a file was saved with the bundled configuration in which an image's colour space is `ACEScg` and the scene's view transform is `Filmic`, then reopened under `tiny.ocio`, which has neither name.

**Which variable this version honours.** Both `OCIO` and `BLENDER_OCIO` work. At start-up Blender sets `OCIO` in its own environment to the bundled configuration inside the application, read back as `'/Applications/Blender.app/Contents/Resources/5.2/datafiles/colormanagement/config.ocio'`. 4.5.12 ignores `BLENDER_OCIO` altogether, and neither 4.5.12 nor 5.0.1 sets `OCIO` at start-up.

**How much the configuration decides.** The bundled configuration offers 63 colour spaces in this version (21 in 4.5.12, 25 in 5.0.1 and 5.1.2, 63 in 5.2.0). With `tiny.ocio` active the list dropped to `['TinyLinear', 'scene_linear']` and the view transform became `TinyView`.

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

**Both standing lines, brought up to date by this test.** *Driven from outside:* settled — the whole mechanism is an environment variable, every reading above was taken with no window, and every message reached the terminal. *Operating system:* macOS for this test. The same test was run on a rented Linux machine on 2026-08-06 and that result is recorded below; Windows stays untested.

**Still open.** What the colour-space field shows on screen when the name is gone is entry 25 on the hand-test list and is the Creator's to answer.


**Observed — hand test entry 25, run by the Creator on 2026-08-05 on their own computer** (MacBook Pro, M1 Pro chip, macOS 26.5.2, Apple's own graphics), in Blender 5.2.0 LTS with its window open. This is the half of the list that needs eyes on the screen; the whole list is at `Audits/Blender Research/Dependencies/Hand tests.md`. Because it was run on a Mac, it settles macOS and leaves Windows and Linux open.

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

**Which variable is honoured.** Both work, and **at start-up Blender sets `OCIO` in its own environment** to the bundled configuration inside the application — here `'/opt/blender/blender-5.2.0-linux-x64/5.2/datafiles/colormanagement/config.ocio'`. The same behaviour as on the Mac, with the Linux path in place of the Mac one.

**How many colour spaces the bundled configuration offers: 63**, the same as on the Mac. With the small configuration active the list dropped to two entries; here the second read `'Working Space'` where the Mac's record has `'scene_linear'`. Both readings are recorded and which reading each run took is not established.

Pointing at a configuration that does not exist fell back to the bundled one and printed `color_management | ERROR OpenColorIO Error: Error could not read '/root/…/no_such.ocio' OCIO profile.` `is_missing_opencolorio_config` returned `False` in every case tried, including that one. The two extra warnings 5.0 prints did not appear.

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

## 16. How is a font installed on one kind of computer recorded in the file, and what does Blender do when a font it points at is not on the computer?

**LABEL: documented** for the first half — how it is recorded. **not in the sources** for the second half — what Blender does when the font is gone — beyond a generic mechanism that covers external files as a class.

### How it is recorded: as a data-block holding a file path, which may be relative

The data-block table marks Font as both linkable and packable, and the Pack column is defined as being about file references:

> "Pack
>
> File Packing, supports file contents being packed into the blend-file
> (not applicable for most data-blocks which have no file reference)."

> "Font
>
> ✓
>
> ✓
>
> References font files.
> Used by curve object-data of text objects."

— manual, https://docs.blender.org/manual/en/5.2/files/data_blocks.html (read 2026-07-30)

What it holds:

> "class bpy.types.VectorFont(ID)
>
> Vector font for Text objects
>
> filepath
>
> (default "", never None, blend relative // prefix supported)
>
> Type:
>
> str
>
> packed_file
>
> (readonly)
>
> Type:
>
> PackedFile | None
>
> pack()
>
> Pack the font into the current blend file
>
> unpack(*, method='USE_LOCAL')
>
> Unpack the font to the samples filename"

— Python API, https://docs.blender.org/api/5.2/bpy.types.VectorFont.html (read 2026-07-30)

Loading is by path, and the loader defaults to relative:

> "filepath (str) – path of the font to load (never None, blend relative // prefix supported)"

— Python API, https://docs.blender.org/api/5.2/bpy.types.BlendDataFonts.html (read 2026-07-30)

> "bpy.ops.font.open(*, filepath='', … relative_path=True, display_type='THUMBNAIL', sort_method='')
>
> Load a new font from a file"

— Python API, https://docs.blender.org/api/5.2/bpy.ops.font.html (read 2026-07-30). Note `relative_path=True`.

The interface path is "browse to a font file", and the manual points at where the operating system keeps installed fonts:

> "Blender comes with a built-in font by default that is displayed in
> each of the four font style data-block menus.
> The built-in font is always present and shows in this list as "Bfont".
> The data-block menu contains a list displaying the currently loaded fonts.
> Select one for each font style.
>
> To load a different Font, click one of the Load buttons
> in the Font panel and navigate to a font file.
> The File Browser will give all valid fonts a capital "F" icon.
>
> If you select a font that is unsupported by Blender, you will get the error Not a valid font.
>
> Note
>
> Location of Fonts on Unix
>
> Fonts are typically located under /usr/lib/fonts, or some variant like /usr/lib/X11/fonts,
> but not always. They may be in other locations as well,
> such as /usr/share/local or /usr/local/share, and possibly related sub-trees."

— manual, https://docs.blender.org/manual/en/5.2/modeling/texts/properties.html (read 2026-07-30)

Which file types count:

> "This font system has its own built-in font, but it can use external fonts too,
> including PostScript Type 1, OpenType and TrueType fonts."

— manual, https://docs.blender.org/manual/en/5.2/modeling/texts/introduction.html (read 2026-07-30)

So: **an installed system font is recorded exactly the same way as any other font — a path in a Font data-block, relative by default, absolute if you say so.** Nothing in the sources gives system fonts a special recording form, no font name, no family, no marker. That is the answer to the first half, and it carries a consequence worth stating plainly: relative recording cannot help for a font installed in a system folder, because that folder is not under the blend-file's folder. The sources do not draw that conclusion; I flag it rather than assert it.

The escape hatch:

> "Resource
>
> External files such as images, sounds, fonts and volumes files that can be packed into a blend-file."

— manual, https://docs.blender.org/manual/en/5.2/glossary/index.html (read 2026-07-30)

> "class bpy.types.PackedFile(bpy_struct)
>
> External file packed into the .blend file
>
> data
>
> Raw data (bytes, exact content of the embedded file) (default b"", readonly, never None)"

— Python API, https://docs.blender.org/api/5.2/bpy.types.PackedFile.html (read 2026-07-30)

### What Blender does when the font is gone: not in the sources

There is no statement anywhere in the 5.2 manual, API documentation or release notes about what Blender does when a font a file points at is absent. No documented fallback to the built-in Bfont, no documented message, no documented placeholder. What exists is the general external-file machinery, which by the Pack-column definition covers fonts:

> "Report Missing Files
>
> This option is useful to check if there are links to unpacked files that no longer exist.
> After selecting this option, a warning message will appear in the Info editor's header.
> If no warning is shown, there are no missing external files."

— manual, https://docs.blender.org/manual/en/5.2/interface/window_system/topbar.html (read 2026-07-30)

and the File Browser page names fonts explicitly among what it re-points:

> "Updating the locations of previously imported media (images, videos, fonts…)."

— manual, https://docs.blender.org/manual/en/5.2/editors/file_browser.html (read 2026-07-30)

Two things come close and do **not** answer it. `ID.is_missing` is about a data-block linked from another blend-file, not about a missing external font file. And "If you select a font that is unsupported by Blender, you will get the error Not a valid font" is about picking an unusable file at load time, not about a path that no longer resolves.

**Record of where the helper looked**, for that claim. Grep terms across all four 5.2 source sets: `font`, `bfont`, `VFont`, `VectorFont`, `missing font`, `font file`, `font_path_ui`, `font_path_ui_mono`, `Interface Font`, `built-in font`, `builtin font`, `builtin`, `<builtin>`, `default font`, `fallback font`, `system font`, `typeface`, `truetype`, `opentype`, `.ttf`, `.otf`, `fontconfig`, `missing file`, `find missing`, `report missing`, `is_missing`, `placeholder`, `place-holder`, `no longer exist`, `cannot be found`, `could not be found`, `not be found`, `relative path`, plus a combined pass of `font` against `missing|broken|absent|unavailable|fail|error|invalid|not found|fallback|substitut`. Every one of the 28 manual files containing the word "font" was enumerated and checked. Ruled out and why: `editors/preferences/interface.txt` and `PreferencesView.font_path_ui` — the *interface* font, a preference, not stored in a file. `editors/preferences/file_paths.txt` "Fonts / Default location to browse for text object font files" — a browse starting folder, and nothing says Blender searches there when a font is missing. `modeling/texts/editing.txt` line 207 "as they also serve as placeholders for you to load up other fonts manually" — a misleading match; this is about the Bold/Italic buttons standing in for separate font slots. `modeling/geometry_nodes/input/constant/font.txt` and `compositing/types/input/constant/font.txt` — both say "By default, Blender's built-in font is selected", which is about a node default, not a missing file. `bpy.ops.font.txt` read in full: the only file-touching operators are `font.open` and `font.unlink` — no reload, no repath, no missing-font operator. `bpy.types.TextCurve` — has `font`, `font_bold`, `font_bold_italic`, `font_italic`, all typed `VectorFont | None` with no descriptions; the `| None` shows a text object can hold no font, but nothing says that is what a missing file produces. `api_5.2/blf.txt` — the interface text-drawing module; it loads by path and documents "The new font's fontid or -1 if there was an error", which is the only documented font-load failure result anywhere in 5.2, but it is a different subsystem and does not answer the question. The hundreds of `filter_font (bool)` hits are file-browser filter flags on unrelated operators. `manual_5.2/_images/` was searched for font and text figures; there is no screenshot of the Font panel and none of a data-block menu with a missing entry, so nothing there could settle it. `cycles_5.2` grepped for `font`: two hits, an object-type list in `ui.py` and a comment in `shader/oslutil.h`; no observed finding available. The `troubleshooting/` tree has no font page. In the release notes the only font mentions are the new Font socket and Input Font node (https://developer.blender.org/docs/release_notes/5.2/compositor/ and https://developer.blender.org/docs/release_notes/5.2/user_interface/, read 2026-07-30); neither touches file references.

**Not covered.** What Blender does when a recorded font path does not resolve — whether the text renders in the built-in font, renders as nothing, or the data-block survives empty. Whether any warning is emitted, and where. Whether the built-in "Bfont" is written into the file as a path, as a marker, or not at all. Whether `bpy.data.fonts.load()` raises when the path does not exist. Whether the font is read once at load or re-read during a render.

**Outside the interface.** Mostly a gap. The path form is the same however Blender is driven — `bpy.data.fonts.load` takes a `//`-capable path, and `bpy.path.abspath` resolves it, both Python. `report_missing_files` and `find_missing_files` are callable from Python. But `report_missing_files` is documented as putting "a warning message… in the Info editor's header", an interface place, and whether an equivalent reaches standard output in background mode the sources do not say. `find_missing_files` is documented as opening a File Browser, though it does take a `directory` argument. Whether a background render with a missing font warns, fails, or silently substitutes: the sources do not say.

**Operating system.** The manual gives a Unix-specific note on where installed fonts live (quoted above); there is no equivalent note for Windows or macOS anywhere in the 5.2 manual, and the helper searched for one. So the manual tells you the system font location on one family of systems only. Whether a path recorded on one operating system resolves on another, and how separators or drive letters are handled, the sources do not say. The relative-path promise ("you could send someone a folder that contains a blend-file and a subfolder of external images that it references") is made without an operating-system qualifier, but it is about a file you carry with you, not about a font installed in a system folder.


**Observed — hand test entry 6, run 2026-08-05 on the Creator's own computer** (MacBook Pro, M1 Pro chip, macOS 26.5.2, Apple's own graphics), with Blender 5.2.0 LTS started from a terminal with no window, using a throwaway settings folder set through `BLENDER_USER_RESOURCES`. The full list of tests is at `Audits/Blender Research/Dependencies/Hand tests.md`.

**What was done.** A file was built with a text object whose body is `AB`, using a copy of the Mac's own `/System/Library/Fonts/Supplemental/Arial.ttf` placed in an `assets` folder beside the blend-file. The file was opened with the font present and every `bpy.data.fonts` entry, the text object's font and the vertex and polygon counts of the object's evaluated mesh were printed. The font file was then moved aside and the same readings taken, followed by a Cycles render from the command line.

**How the built-in font is recorded.** As the literal string `<builtin>`:

```
FONT name='Arial Regular' filepath='//assets/testfont.ttf' packed_file=None
FONT name='Bfont Regular' filepath='<builtin>' packed_file=None
```

A font taken from the operating system's own font folder is recorded as an ordinary file path, made relative to the blend-file like any other — here `//assets/testfont.ttf` after the copy was placed beside the file. There is no separate family name.

**What happens when it is gone.** The stored path is unchanged, the render finishes with exit status 0, and the rendered frame differs from the good one. The text object does **not** fall back to the built-in font: its evaluated mesh went from `vertices=332 polygons=334` with the font present to `vertices=16 polygons=16` with it gone, for the two letters `AB`.

What was printed while rendering:

```
00:00.415  geom.vfont       | WARNING Font file doesn't exist: //assets/testfont.ttf
```

four times, each preceded by two lines of `reports          | ERROR Unable to pack file, source path '<path>/assets/testfont.ttf' not found`. In 4.5.12 the warning carries the file and line number of Blender's own source, and the `Unable to pack file` lines also appear on a plain open rather than only on the render.

`bpy.ops.file.report_missing_files()` does name the font, in all four versions: `Warning: Path '<path>/assets/testfont.ttf' not found, from local data-block 'VFArial Regular'`.

**Both standing lines, brought up to date by this test.** *Driven from outside:* settled. All of this was done with no window; the warning reaches the terminal and the render is not stopped. *Operating system:* macOS for this test, with the same test run on a rented Linux machine on 2026-08-06 and recorded below; the font came from the Mac's own font folder. Where Windows and Linux keep their fonts, and what a font path written on one means on another, is untested and was not tried here.

**Still open.** What the text object looks like on screen and what the Font panel says is entry 22 on the hand-test list and is the Creator's to answer.


**Observed — hand test entry 22, run by the Creator on 2026-08-05 on their own computer** (MacBook Pro, M1 Pro chip, macOS 26.5.2, Apple's own graphics), in Blender 5.2.0 LTS with its window open. This is the half of the list that needs eyes on the screen; the whole list is at `Audits/Blender Research/Dependencies/Hand tests.md`. Because it was run on a Mac, it settles macOS and leaves Windows and Linux open.

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


**Observed — run again on Linux, 2026-08-06.** A text object whose font file is beside the blend-file, and then gone, was built and read again in this version on the rented Linux machine.

**How a font is recorded, and what happens when it is gone, is the same on both machines.** A font copied beside the blend-file is recorded as `//assets/testfont.ttf`, an ordinary external file path like any other. The built-in font is recorded as the literal string `<builtin>`. With the font present the evaluated mesh for `AB` had `vertices=332 polygons=334`; with it gone, `vertices=16 polygons=16`, and the stored path was unchanged. The render finished with exit status 0. Report Missing Files named the font: `Warning: Path '/root/…/assets/testfont.ttf' not found, from local data-block 'VFArial Regular'`. The message shapes were the same as on the Mac.

**With a window open on the Linux machine**, the text object `AB` is **two hollow rectangles, one per character**, in the viewport — the letterforms are gone and the built-in font does not stand in for them. That matches the vertex counts and the Creator's picture on the Mac. The Font panel in Object Data Properties is closed to start with and Blender offers no way to open a panel from a script, so **what that panel says was photographed on the Mac only.**

**This test disagrees with what the Mac recorded about *when* Blender complains, and the disagreement is written down rather than settled.** The Mac's record says the two `Unable to pack file` lines appear on a plain open in 4.5 and only on a render in 5.0, 5.1 and 5.2. On the Linux machine they appear **on a plain open in all four versions**. To find out whether that is the machine, the same file was opened three ways: the Linux-built file on Linux with nothing asked of it at all; those same files carried back to the Mac and opened there the same way; and the Creator's own prepared files — the ones the Mac result came from — opened on both machines. **In every one of those runs, all four versions printed the lines at load, on the Mac as well as on Linux.** So the two machines agree with each other and both disagree with the earlier record. The earlier Mac result stands as recorded and this one stands beside it. **What makes the lines appear at load in one run and not another is not established**, and no test here reached it.

**Still not established.** Where Windows keeps its fonts and what a font path written there means elsewhere. No Windows machine has run it.

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

2026-08-04 — Opus 5 (Claude Code) — Dependencies, Blender 5.2

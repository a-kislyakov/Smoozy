# Dependencies — findings, Blender 5.3

- Sources consulted: Blender 5.3 manual, Blender 5.3 Python API documentation and Blender 5.3 release notes (copies taken 2026-10-11, used as date read), and Cycles' own files for 5.3 (copy taken 2026-10-16, used as date read). All read from inside `Blender Sources/`.
- I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

Two honesty notes on that line. First, before I started, the tool I run in showed me, without my asking, the text of the project's `CLAUDE.md` signpost (it told Blender researchers to stop and contains no Blender facts) and a list of one-line titles from the user's saved memory. I did not open either, and nothing below comes from them. Second, I checked only whether the output file already existed (it did not) before writing it. I did not open it. Inside `Blender Sources/` I also read `About this folder.md`. I did not follow its link out of the folder.

How to read the labels: **documented** means the source says it outright. **observed** means it is read straight from Cycles' own files. **inferred** means it follows from what a source says, and the step is written out. **not in the sources** means I searched and found nothing. The search is recorded.

---

## 1. What kinds of thing can a `.blend` file point at outside itself? Give the full list.

**Label:** documented for each item below. The claim that the list is *full* is **inferred**. No page calls itself a complete list. I built the list by searching the whole API documentation for the phrase every file-path setting carries ("blend relative // prefix supported"). That phrase appears on 42 pages. I then added the manual's data-block table and the API's list of path kinds.

**Answer.** These are the things the sources show a file recording as a location outside itself:

1. **Other `.blend` files (libraries).** A library is "External .blend file from which data is linked". There are also "weak references": a note of which library file an appended or editable data-block came from.
2. **Data-blocks that stand for a file:** images (single images, image sequences, movies, UDIM tile sets), movie clips, sounds, fonts, text files, volumes (OpenVDB), and cache files (Alembic or USD archives, which can have several layers).
3. **Paths held inside other settings:**
   - Sequencer: the image-strip folder and its per-frame file names, movie strips, and proxy folders and files (per strip and for the whole project). Movie-clip proxy folders.
   - Modifiers: Mesh Cache file, Multiresolution external displacement file, Ocean bake folder, the Geometry Nodes bake folder (per modifier and per bake), the Dynamic Paint output folder, the fluid cache folder, and point-cache paths (disk cache and "external" cache).
   - Shading: OSL script files (Script node) and the OSL custom-camera shader file, IES light files, and any node socket of the "file path" kind. This includes the Geometry Nodes import nodes for CSV, OBJ, PLY, STL, TXT and VDB, each of which has a Path input.
   - Output: the render output path, the compositor File Output folder and file name, and the external-save path for texture baking.
   - Collection importers and exporters: their file paths.
4. **Cache files tied to images**, such as the Cycles texture cache (`.tx`) files. The API marks these as a separate kind of path.
5. **Things that are not stored as a path but still live outside the file** (partly **documented**, partly **inferred**, see the quotes):
   - the OpenColorIO configuration the file was made with (the file detects when it is missing, but whether the file records the configuration itself is not documented; see question 15)
   - the active project, if any (documented; see question 5)
   - the asset catalog definition file (an asset stores only a catalog ID, and the file that defines it lives in the asset library; that the .blend points at the definition file is inferred)
   - OSL shaders given only by a module name, which are looked up in a search path on the computer (documented)

Settings with the same "//" wording also exist under Preferences, such as the texture cache folder, render cache folder, render output default and font folder. Those belong to Blender's preferences, not to the `.blend` file, so they are not on the list.

**Evidence (all dated 2026-10-11 unless marked):**
- "External .blend file from which data is linked" — API, https://docs.blender.org/api/5.3/bpy.types.Library.html
- "Read-only external reference to a linked data-block and its library file" — API, https://docs.blender.org/api/5.3/bpy.types.LibraryWeakReference.html
- "Skip weak reference paths. Those paths are typically ‘nice to have’ extra information, but are not used as actual source of data by the current .blend file." and "Visit cache files, like texture cache files associated with images." — API, https://docs.blender.org/api/5.3/bpy_types_enum_items/file_path_foreach_flag_items.html
- "True when the path is a cache file, like the image texture cache. These paths can not be edited." — API, https://docs.blender.org/api/5.3/bpy.types.BlendDataPathMeta.html
- "Returns a list of paths to external files referenced by the loaded .blend file." — API, https://docs.blender.org/api/5.3/bpy.utils.html
- Path settings, each quoted from its API page (address pattern https://docs.blender.org/api/5.3/bpy.types.NAME.html):
  - Image "Image/Movie file name"
  - MovieClip "Filename of the movie or sequence file"
  - Sound "Sound sample file used by this Sound data-block"
  - VectorFont `filepath` (no description beyond the default)
  - Text "Filename of the text file"
  - Volume "Volume file used by this Volume data-block"
  - CacheFile `filepath`, and CacheFileLayer "Path to the archive"
  - ImageStrip `directory`, StripElement "Name of the source file", MovieStrip `filepath`
  - StripProxy "Location to store the proxy files" / "Location of custom proxy file"; SequenceEditor `proxy_dir`; MovieClipProxy "Location to store the proxy files"
  - MeshCacheModifier and MultiresModifier `filepath`
  - OceanModifier "Path to a folder to store external baked images"
  - NodesModifier `bake_directory` and NodesModifierBake `directory`: "Location on disk where the bake data is stored"
  - DynamicPaintSurface "Directory to save the textures"
  - FluidDomainSettings "Directory that contains fluid cache files"
  - PointCache "Cache file path"
  - ShaderNodeScript "Shader script path"; Camera `custom_filepath` "Path to the shader defining the custom camera"
  - ShaderNodeTexIES "IES light path"
  - NodeSocketStringFilePath `default_value`
  - RenderSettings "Directory/name to save animations, # characters define the position and padding of frame numbers"
  - CompositorNodeOutputFile "The directory where the image will be written"
  - BakeSettings "Image filepath to use when saving externally"
  - CollectionExport "The file path used for exporting"; CollectionImport "The file path used for importing"
- Data-block table, "Pack" column: Font ✓, Image ✓, Library ✓, Sound ✓, Movie Clip ✗, Text ✗. "Cache File … Used by Mesh Cache modifiers." — manual, https://docs.blender.org/manual/en/5.3/files/data_blocks.html
- "The path to the CSV file." (and matching "Path" inputs on the OBJ, PLY, STL, TXT and VDB import nodes) — manual, https://docs.blender.org/manual/en/5.3/modeling/geometry_nodes/input/import/csv.html
- "The third option is to specify just the module name, which will be looked up in the shader search path." — manual, https://docs.blender.org/manual/en/5.3/render/shader_nodes/utilities/script.html
- "This is what is stored in the asset, and what determines the “identity” of the catalog." and "It is stored in blender_assets.cats.txt in the root directory of the asset library." — manual, https://docs.blender.org/manual/en/5.3/files/asset_libraries/catalogs.html
- "A color space, view or display was not found, which likely means the OpenColorIO config used to create this blend file is missing" — API, https://docs.blender.org/api/5.3/bpy.types.BlendFileColorspace.html
- "The currently active Blender project, if any" — API, https://docs.blender.org/api/5.3/bpy.types.BlendData.html

**Not covered.** No source states that this list is complete. Paths added by add-ons are not covered. The sources do not say whether every item above is visited by Blender's own path tools (Report Missing Files, Make Paths Relative, and so on). One conflict: the manual's table marks Volume as not packable ("—"), but the API gives Volume a `packed_file` property.

**Where I looked.** Searched the API for "blend relative // prefix supported" (42 pages), "Supports template expressions", "filepath", "directory", and "packed_file". Searched the manual for "External Data", "Pack", "path" and "file path". Read the manual pages on file paths, packed data, data-blocks, linked libraries and asset libraries.

**Outside the interface.** *Brought up to date by the hand tests of 2026-10-17 — see* Observed *below. What follows is what the sources alone said.* The path settings are data and are reachable from Python. The listing tools `bpy.utils.blend_paths`, `BlendData.file_path_map` and `BlendData.file_path_foreach` are Python functions. The command-line page documents running Blender in the background with a Python script (`-b`, `-P`), so these tools are reachable without a person at the screen. That last step is **inferred**. The sources do not say the results differ outside the interface.

**Operating system.** *Brought up to date by the hand tests of 2026-10-17 — see* Observed *below.* The sources do not say the list differs between operating systems.

### Observed — hand tests on the Creator's Mac, 2026-10-17

**Label: observed.** Run on the Creator's Mac — a MacBook Pro with an Apple M1 Pro chip and its own 14-core graphics (Metal), 16 GB of memory, macOS 26.6.2 — in Blender 5.3.0 Beta, which said of itself `VERSION 5.3.0 Beta HASH 5d8151335b6b DATE 2026-10-16`. Unless an entry says otherwise, Blender was started from a terminal with no window, with `--factory-startup`, a settings folder of its own set through `BLENDER_USER_RESOURCES`, and its temporary folder pointed at the work folder through `TMPDIR`. In the machine's own words quoted below, the work folder `/private/tmp/claude-501/smoozy-hand-tests-5.3/Dependencies` is written `<W>`; nothing else is altered. The list these entries come from, with every command, is `Audits/Blender Research/Dependencies/Hand tests — 5.3.md`.

**Entry 1 — seven kinds of outside file recorded, and the two ways of listing them.** A folder `assets` was filled with a 32 by 32 PNG `tex.png`, a short WAV `beep.wav`, a copy of the Mac's `/System/Library/Fonts/Supplemental/Arial.ttf` named `testfont.ttf`, a text file `notes.txt`, an OSL file `shader.osl` and an IES file `light.ies`. A file `kinds.blend` was saved first, then given by script: an image loaded from `//assets/tex.png`, a sound, a font used by a text object, a text loaded from `//assets/notes.txt`, a Volume data-block whose path is `//assets/smoke.vdb` (no such file), a material with a Script node in External mode pointing at `//assets/shader.osl`, a point light with an IES Texture node pointing at `//assets/light.ies`, and the render output path `//renders/frame_`; then saved again. The factory-startup scene's own objects and material were left in it. The file was opened in a fresh run and read.

`bpy.utils.blend_paths(absolute=False, packed=False, local=False)` returned, sorted:

```
    '//../../../../../../Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/brushes/essentials_brushes-gp_draw.blend'
    '//assets/beep.wav'
    '//assets/light.ies'
    '//assets/notes.txt'
    '//assets/shader.osl'
    '//assets/smoke.vdb'
    '//assets/testfont.ttf'
    '//assets/tex.png'
```

`bpy.data.file_path_map()` gave these non-empty rows, and every other row empty (the empty ones were the cameras, collections, line style, the `Render Result` and `Viewer Node` images, the factory `Material`, meshes, objects, palette, the other light, scene, screens, text curve, `Bfont Regular`, window manager, workspaces and world):

```
    VectorFont('Arial Regular') -> ['//assets/testfont.ttf']
    PointLight('L') -> ['//assets/light.ies']
    Material('Mat') -> ['//assets/shader.osl']
    Volume('Vol') -> ['//assets/smoke.vdb']
    Sound('beep.wav') -> ['//assets/beep.wav']
    Text('notes.txt') -> ['//assets/notes.txt']
    Image('tex.png') -> ['//assets/tex.png']
```

`bpy.data.file_path_map(include_libraries=True)` gave the same rows. The render output path appeared in neither listing:

```
render.filepath '//renders/frame_'
text filepath stored '//assets/notes.txt'
```

**The eighth path, into the Blender application itself.** It is a weak reference carried by the factory-startup material `Material`, which nothing in the build touched:

```
LIBRARIES []
ID materials 'Material' library None weakref ('//../../../../../../Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/brushes/essentials_brushes-gp_draw.blend', 'MAMaterial')
```

So `blend_paths` lists a weak reference that `file_path_map` does not, and the file records it relative to itself, climbing six folders to the application. Asked for absolute paths, `blend_paths(absolute=True)` gave `['/Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/assets/brushes/essentials_brushes-gp_draw.blend']`.

**Which machinery this version has**, each asked by name:

```
HAS bpy.data.file_path_foreach True
HAS bpy.data.file_path_map True
HAS bpy.utils.blend_paths True
HAS Library.is_archive True
HAS Library.archive_libraries True
HAS Library.archive_parent_library True
HAS Library.parent True
HAS bpy.data.colorspace True
HAS WindowManager.reports True
HAS ID.is_missing True
HAS ID.library_weak_reference True
HAS bpy.ops.file.pack_libraries True
HAS bpy.ops.file.pack_all True
HAS bpy.ops.file.report_missing_files True
HAS bpy.ops.file.find_missing_files True
HAS bpy.ops.outliner.id_linked_relocate True
HAS bpy.path.resolve_ncase True
bpy.data.colorspace.is_missing_opencolorio_config False
CompositorNodeOutputFile own properties: ['file_output_items', 'active_item_index', 'directory', 'file_name', 'format', 'save_as_render', 'use_file_extension']
```

**Entry 4 — whether a volume can be packed.** The sources disagree here: the manual's data-block table marks Volume as not packable, and the API gives Volume a `packed_file`. A real OpenVDB file was made first by baking a small gas simulation (domain resolution 16, frames 1 to 2, cache format `OPENVDB`) and copying its second frame, `fluidcache/data/fluid_data_0002.vdb`, 53,796 bytes, to `vol/smoke.vdb`. A new file `volpack.blend` was saved, given a Volume data-block with the path `//vol/smoke.vdb` and an object using it, and Pack All was called:

```
Volume has packed_file property: True
grids.load() -> True | grids: ['density', 'flame', 'shadow', 'temperature', 'velocity'] | error: ''
packed_file before: None
Info: Packed 1 file(s)
pack_all() -> {'FINISHED'}
packed_file after pack_all: <bpy_struct, PackedFile at 0xb1cffb2e8>
```

The file was saved, `smoke.vdb` moved to another folder, and the file opened again in a fresh run:

```
REOPENED, vol/smoke.vdb exists on disk: False
filepath: '//vol/smoke.vdb' | packed_file: <bpy_struct, PackedFile at 0xab2fdc5e0>
grids.load() -> False | grids: [] | error: 'smoke.vdb not found'
blend_paths(packed=False): []
blend_paths(packed=True):  ['//vol/smoke.vdb']
```

So the volume holds a packed copy, and the listing treats it as packed, but asking it to load its grids with the file gone fails with `smoke.vdb not found`. Then the same file was rendered three times from a script, with a camera and a black world added, Cycles, 64 by 64, 8 samples: once with `smoke.vdb` gone, once with it put back, and once with it put back and the volume object hidden from the render. Nothing was printed about the volume in any run. Compared pixel by pixel:

```
gone against present:     PICTURES size=(64, 64) largest channel difference=0.294118 channels differing=1299 of 16384 — DIFFERENT PICTURE
gone against no-volume:   PICTURES size=(64, 64) largest channel difference=0.000000 channels differing=0 of 16384 — SAME PICTURE
present against no-volume: PICTURES size=(64, 64) largest channel difference=0.294118 channels differing=1299 of 16384 — DIFFERENT PICTURE
```

With its file gone, the packed volume rendered exactly as if the volume were not in the scene at all, and nothing said so. With the file there, it rendered. So in this build Pack All marks a volume as packed and the mark survives a save, but neither loading its grids nor a render with no window reaches anything from it once the file on disk is gone. Whether the saved `.blend` holds the volume's bytes at all was not read.

**What this does not settle.** Whether these kinds are all the kinds a `.blend` can point at: the file was built to hold seven, and a kind nobody put in it would not appear. Entries 40 and 49 put further kinds into files — an Alembic cache, a movie clip, a movie strip and others — and are recorded under questions 12 and 20.

**Entry 49 — the two ways of listing every recorded place.** Recorded in full under question 20. In short: for a file recording places of many kinds, `bpy.utils.blend_paths()` and `bpy.data.file_path_map(include_libraries=True)` both returned 27 strings in the run that built it; once the file had been opened from disk they returned 24 and 25, the extra one in `file_path_map` being `'//lib.blend'`.

**Standing lines, as the hand tests leave them.** *Driven from outside:* every reading above was taken with Blender running with no window, so the recording and both listings hold outside the interface. *Operating system:* macOS only, on the machine named above. Linux and Windows were not run in this sitting: no machine running either was available.

---

## 2. What kinds of library can a file point at, including one that does not sit on the same computer?

**Label:** documented, except where marked inferred.

**Answer.**
- **A linked `.blend` library.** This is the basic kind.
- **An "archive" library.** This holds linked data that has been packed into the file. It is created from, and belongs to, the original library.
- **A library whose data stays editable even though linked.** The API says this is used for brush assets.
- **Asset libraries (in Blender).** These are repository locations where assets are found: the current file, bundled Essentials, custom folders of `.blend` files set in Preferences, project asset libraries, extension libraries, or online ("remote") libraries named by a web address. However, as noted below, a `.blend` file cannot link directly from online asset libraries.
- **Not on the same computer.** The only kind the sources describe is the online or remote asset library, "reached through a network connection". The manual says assets from such libraries **cannot be linked**. They can only be appended or packed, so the file ends up holding them rather than pointing at the remote source. That last step is **inferred** from the append and pack descriptions. A `.blend` file pointing at a library outside its own machine via a network address is therefore **not in the sources**.
- A `.blend` library sitting on a network drive is not described anywhere. The File Browser does list "network mounts", so such a location can be picked like any folder. That is **inferred**: the sources say nothing about linking from it.

**Evidence (2026-10-11):**
- "This library is an ‘archive’ storage for packed linked IDs originally linked from its ‘archive parent’ library." and "Data-blocks in this library are editable despite being linked. Used by brush assets and their dependencies." — API, https://docs.blender.org/api/5.3/bpy.types.Library.html
- "Asset libraries may be stored locally as directories containing blend-files or hosted online and downloaded on demand." and "Unlike local asset libraries, online libraries are identified by a URL instead of a directory path" and "Assets cannot be linked directly and must instead be appended or packed." — manual, https://docs.blender.org/manual/en/5.3/files/asset_libraries/introduction.html
- "In this case, “Remote” means “reached through a network connection”. It can be served from your own computer, it doesn’t have to be far away." — manual, https://docs.blender.org/manual/en/5.3/files/asset_libraries/remote_asset_libraries.html
- "Linking is not supported for online asset libraries." and "Pack — Imports the asset as linked data and immediately packs it into the current blend-file." — manual, https://docs.blender.org/manual/en/5.3/editors/preferences/asset_libraries.html
- Library types "Current File", "Essentials", "Online Essentials", "Custom" — API, https://docs.blender.org/api/5.3/bpy_types_enum_items/asset_library_type_items.html
- "Signifies if the asset library is defined by a project" and "Identifier of the extension defining this asset library" — API, https://docs.blender.org/api/5.3/bpy.types.UserAssetLibrary.html
- "Enable asset libraries served over HTTP/HTTPS (default False, readonly)" — API, https://docs.blender.org/api/5.3/bpy.types.PreferencesExperimental.html
- Release note: "Asset library servers can now require authentication to access and download their content." and "Asset Library extensions are now supported." — https://developer.blender.org/docs/release_notes/5.3/assets/
- Release note: "Project asset libraries to easily browse all assets in a project" — https://developer.blender.org/docs/release_notes/5.3/core/
- "Volumes — Drives and network mounts." — manual, https://docs.blender.org/manual/en/5.3/editors/file_browser.html

**Not covered.** Whether a library's recorded location can be a web address (no source says so). How a `.blend` library on a network share behaves. What path the file records for an asset taken from an online library and packed. What the `use_remote_asset_libraries` switch means in practice, beyond the quoted words.

**Where I looked.** Asset library pages (introduction, remote, catalogs), the Asset Browser page, the asset library preferences page, the API for Library, UserAssetLibrary, AssetLibrary and PreferencesExperimental, the 5.3 assets/core/python_api release notes, and searches for "remote", "URL", "network" and "online".

**Outside the interface.** *Brought up to date by the hand tests of 2026-10-17 — see* Observed *below. What follows is what the sources alone said.* `bpy.data.libraries.load(…, link=…, pack=…)` links or packs from Python. Online downloads need "Online Access". The command line has `--online-mode` and `--offline-mode` ("Allow internet access, overriding the preference" / "Disallow internet access…"), at https://docs.blender.org/manual/en/5.3/advanced/command_line/arguments.html. Whether online asset libraries work at all without the interface is not stated.

**Operating system.** *Brought up to date by the hand tests of 2026-10-17 — see* Observed *below.* Not stated as differing.

### Observed — hand tests on the Creator's Mac, 2026-10-17

**Label: observed.** Same machine, Blender and way of starting it as under question 1; `<W>` is the work folder.

**Entry 1 — the archive-library properties.** Asked for by name in a running 5.3 with no window: `Library.is_archive`, `Library.archive_libraries` and `Library.archive_parent_library` all came back `True` (present). Entry 31, under question 19, links with packing asked for and shows what those properties hold.

**Entry 5 — a web address given as a library or as an image's location.** With no window, `bpy.data.libraries.load("https://example.invalid/lib.blend", link=True)` was asked for an object:

```
libraries.load('https://example.invalid/lib.blend', link=True) RAISED OSError: load: https:/example.invalid/lib.blend failed to open blend file
libraries now: []
```

The error message prints the address with one slash after `https:`. An image given the location `https://example.invalid/tex.png` in a saved file kept that text after a save and a fresh open, resolved to the same text, and loaded nothing (`SIZE=(0, 0)`); Report Missing Files printed it as `https:/example.invalid/tex.png`. The address `example.invalid` is one that cannot exist, so whether Blender would have tried to reach a real one is not shown.

**Standing lines, as the hand tests leave them.** *Driven from outside:* the readings were taken with no window. *Operating system:* macOS only. A library that does not sit on the same computer, and an online asset library, were not tried: this sitting had one computer, no network share, and no asset-library server — see the *Cannot be run* section of the list.

---

## 3. Can a file hold the things it depends on inside itself partly rather than wholly, and what does a partly-held file report when it is read?

**Label:** documented that partial holding exists and is sometimes forced. What is reported is documented item by item. There is no single "this file is partly packed" report. That absence is **not in the sources**.

**Answer.**
- **Partly held is normal and sometimes unavoidable.** Single files can be packed one at a time. Some kinds can never be packed: sequencer videos, movie clips and text files. Geometry Nodes bakes are packed or on disk, chosen per bake. Linked data can be packed or left linked.
- **What reading a partly packed file reports, per item:**
  - Each packable data-block shows whether it is packed (`packed_file` is filled or empty). Linked data shows `is_linked_packed`. In the interface a "gift box" icon marks packed items.
  - Blender's path-listing tools leave packed items out by default. `bpy.utils.blend_paths` includes them only on request. `file_path_foreach` skips them by default.
  - Report Missing Files speaks only of *unpacked* files that are missing. That packed items are therefore never reported as missing is **inferred** from the wording.
  - When the file is read as a library by another file, its packed linked data is neither linkable nor listed.
- A packed image can hold one packed part per UDIM tile and per view (`tile_number`, `view`). That this means partial packing within one image is **inferred**.

**Evidence (2026-10-11):**
- "A single file can be packed by clicking on the little “gift box” icon to the left of its file-path UI widget." and "Not all external files can be packed. Some typically heavy external files, like videos from the Sequence Editor or Movie Clips, cannot be packed in a blend-file." — manual, https://docs.blender.org/manual/en/5.3/files/blend/packed_data.html
- Table rows "Movie Clip ✓ ✗" and "Text ✓ ✗" (Link, Pack) — manual, https://docs.blender.org/manual/en/5.3/files/data_blocks.html
- "Packed: The baked data is packed into the .blend file. So no separate file is necessary. Disk: The baked data is stored in a separate directory on disk." — manual, https://docs.blender.org/manual/en/5.3/modeling/geometry_nodes/geometry/operations/bake.html
- "packed (bool) – When true include file paths for packed data." — API, https://docs.blender.org/api/5.3/bpy.utils.html
- `file_path_foreach(…, flags={'SKIP_PACKED', 'SKIP_WEAK_REFERENCES'})` — API, https://docs.blender.org/api/5.3/bpy.types.BlendData.html; "Skip paths when their matching data is packed." — https://docs.blender.org/api/5.3/bpy_types_enum_items/file_path_foreach_flag_items.html
- "This data-block is linked and packed into the .blend file" — API, https://docs.blender.org/api/5.3/bpy.types.ID.html
- "This option is useful to check if there are links to unpacked files that no longer exist." — manual, https://docs.blender.org/manual/en/5.3/interface/window_system/topbar.html
- "Packed linked data-blocks are not linkable and are not listed in the input libraries attribute." — API, https://docs.blender.org/api/5.3/bpy.types.BlendDataLibraries.html
- ImagePackedFile has `filepath`, `packed_file`, `tile_number`, `view` — API, https://docs.blender.org/api/5.3/bpy.types.ImagePackedFile.html
- Release note: "A linked packed ID is now considered directly linked if it's used by a local ID, _or by any other packed ID also directly linked_" — https://developer.blender.org/docs/release_notes/5.3/core/

**Not covered.** Any file-level flag or message saying "partly packed". Whether a packed item keeps its original path. The Unpack option "Use <original file path>" suggests it does, but that is **inferred**. What the console prints on load.

**Where I looked.** Packed data page, topbar External Data menu, data-block table, the bake pages, and the API pages for PackedFile, ImagePackedFile, Image, ID, BlendData, bpy.utils, BlendDataLibraries and the path-flag list. Searched "packed", "pack", "gift box" and "linked packed".

**Outside the interface.** *Brought up to date by the hand tests of 2026-10-17 — see* Observed *below. What follows is what the sources alone said.* All the reporting properties above are reachable from Python in a background run. The gift-box icon is interface only. Whether background loading prints anything about packing is not stated.

**Operating system.** *Brought up to date by the hand tests of 2026-10-17 — see* Observed *below.* Not stated as differing.

### Observed — hand tests on the Creator's Mac, 2026-10-17

**Label: observed.** Same machine, Blender and way of starting it as under question 1; `<W>` is the work folder.

**Entry 2 — a partly packed file whose files are gone, opened with no window.** Two 32 by 32 PNGs, `assets/packed.png` (blue) and `assets/loose.png` (red), were each used by a plane's material in a file `partpack.blend`, with a camera and a sun. `packed.png` was packed with `image.pack()` and the file saved. Both PNG files were then moved to another folder. The file was opened with no window; before anything else was asked, each image's `filepath` and `packed_file` were printed, then its `size` and `has_data`. Then `bpy.ops.file.report_missing_files()` was called, `WindowManager.reports` was read, and `bpy.utils.blend_paths` was called with and without packed data. What came back, in full — the only line before the readings is Blender's own `Read blend` line:

```
00:00.317  blend            | Read blend: "<W>/e02/partpack.blend"
--- readings before anything is asked ---
   image=loose.png filepath='//assets/loose.png' packed_file=None
   image=packed.png filepath='//assets/packed.png' packed_file=bpy.data.images['packed.png']...PackedFile
   image=loose.png size=(0, 0) has_data=False
   image=packed.png size=(32, 32) has_data=True
--- report_missing_files ---
Warning: Path '<W>/e02/assets/loose.png' not found, from local data-block 'IMloose.png'
   returned: {'FINISHED'}
   len(WindowManager.reports) = 0
   packed=False: ['//assets/loose.png']
   packed=True:  ['//assets/loose.png', '//assets/packed.png']
```

The packed image keeps its original recorded path `//assets/packed.png` and loads at 32 by 32 from the packed copy. A Cycles render of the same file from the command line (`--engine CYCLES --render-output <W>/e02/out/frame_ --render-frame 1`) finished with exit status 0 and wrote its frame; of everything it printed, the lines naming an image were:

```
00:00.334  render           | Fra: 1 | Mem: 0M | Updating Images | Loading packed.png
00:00.363  cycles           | WARNING Image file <W>/e02/assets/loose.png does not exist.
00:00.368  render           | Fra: 1 | Mem: 1M | Updating Images | Loading loose.png
00:00.368  cycles           | ERROR Failed to load 1 image files
00:00.372  render           | Saved: '<W>/e02/out/frame_0001.png'
```

**Entry 39 — through the window: the partly packed file of entry 2, both PNG files gone.** `partpack.blend` was opened with its window from the terminal on the Mac and driven by timers in a script; pictures were taken with Blender's own screenshot command after forced redraws. Opening printed `image.gpu        | ERROR Image file "<W>/e02/assets/loose.png" does not exist` to the terminal once the viewport drew in Material Preview. On screen:

- *The 3D viewport*, in Material Preview seen from above, showed the plane using the packed image in its own blue and the plane using the unpacked one in bright magenta.
- *The Image editor*, showing `packed.png`, drew a small blue square — the image itself; showing `loose.png`, it drew an empty checkerboard.
- *The Image editor's header* read, for `packed.png`: the image icon, the name `packed.png`, a shield, a two-sheets copy button, **a stack of boxes**, and an X; for `loose.png` the same row with **a plain folder** where the stack of boxes was. Each was photographed in a run of its own, twice, and both pictures agreed.
- *Report Missing Files*, called inside the window both plainly and as `bpy.ops.file.report_missing_files('INVOKE_DEFAULT')`, returned `{'FINISHED'}` and printed `Warning: Path '<W>/e02/assets/loose.png' not found, from local data-block 'IMloose.png'` to the terminal; nothing about it appeared in the status bar in the pictures taken a second later, and `WindowManager.reports` stayed empty.

**Entry 31 — packed linked data.** Recorded in full under question 19. An object linked from `lib.blend` with `bpy.data.libraries.load(…, link=True, pack=True)` came in through an archive library entry (`is_archive=True`) and read `is_linked_packed=True` after a save and reopen; with `lib.blend` then moved away it read `is_missing=False`, and nothing was printed about the library.

**Entry 4 — a packed volume.** Recorded in full under question 1. In short: `bpy.ops.file.pack_all()` packed a Volume data-block's real OpenVDB file (`Info: Packed 1 file(s)`, and `packed_file` filled); after the file on disk was moved away and the `.blend` reopened, `packed_file` was still filled and `blend_paths(packed=True)` listed `//vol/smoke.vdb`, but `grids.load()` returned `False` with `'smoke.vdb not found'`, and a Cycles render with no window came out pixel for pixel the same as a render with the volume hidden, printing nothing about it.

**Entry 51 — a file left partly packed by Automatically Pack Resources.** Recorded in full under question 20. In short: with `bpy.data.use_autopack = True`, a plain save of a file holding four places that reach nothing printed `Error: Unable to pack file, source path '…' not found` for each of the four, raised `RuntimeError`, and wrote the file anyway, with twelve items packed and those four not. Opened again with no window, that file printed no warning or error.

**Standing lines, as the hand tests leave them.** *Driven from outside:* a partly packed file says nothing of its own accord when opened with no window; Report Missing Files works with no window, prints its warning to the terminal naming the path and the data-block, puts nothing into `WindowManager.reports`, and names only the unpacked image. Inside the window (entry 39) the packed image is shown by its own picture and a stack-of-boxes button, the unpacked one by a checkerboard, a magenta plane and a folder button. A packed volume (entry 4) keeps its packed mark but is not used once its file is gone. *Operating system:* macOS only, on the machine named under question 1.

---

## 4. Can one thing a file points at mean several files on disk?

**Label:** documented, except where marked inferred.

**Answer.** Yes, in many places:
- an image set to Image Sequence
- an image made of UDIM tiles, using `<UDIM>` or `<UVTILE>` in its name
- a movie clip pointing at an image sequence
- a volume set to Sequence ("one for each frame")
- a cache file (Alembic/USD) set to Sequence, or with several layers
- a sequencer image strip (one folder plus a list of per-frame file names)
- point caches (one file per frame)
- the fluid cache folder (a sub-folder per simulation type)
- Geometry Nodes bake folders
- multi-view output (one file per view by default)
- the render output path, which becomes one file per frame

Blender's own path tool can either treat such an entry as one path or expand it to every file on disk. It also marks paths that were "expanded".

**Evidence (2026-10-11):**
- "SEQUENCE — Image Sequence – Multiple image files, as a sequence." and "TILED — UDIM Tiles – Tiled UDIM image texture." — API, https://docs.blender.org/api/5.3/bpy.types.Image.html
- "monster-basecolor.<UDIM>.png will load/save files like monster-basecolor.1021.png etc." — manual, https://docs.blender.org/manual/en/5.3/modeling/meshes/uv/workflows/udims.html
- "Loads further VDB files, one for each frame in an animation." — manual, https://docs.blender.org/manual/en/5.3/modeling/volumes/properties.html
- "Whether the cache is separated in a series of files" and CacheFileLayer "Layer of the cache, used to load or override data from the first the first layer" — API, https://docs.blender.org/api/5.3/bpy.types.CacheFile.html and https://docs.blender.org/api/5.3/bpy.types.CacheFileLayer.html
- "The cache files name format is name_frame_index.bphys." — manual, https://docs.blender.org/manual/en/5.3/physics/baking.html
- "Inside this directory each simulation type (i.e. mesh, particles, noise) will have its own directory containing the simulation data." — manual, https://docs.blender.org/manual/en/5.3/physics/fluid/type/domain/cache.html
- "By default the system saves each view as an individual file, thus generating as many files as views to be rendered." — manual, https://docs.blender.org/manual/en/5.3/render/output/properties/stereoscopy/usage.html
- "Skip paths where a single dir is used with an array of files, eg. sequence strip images or point-caches." and "Expand image and volume sequences, invoking the callback once per file on disk." and "Expand template tokens in virtual file paths and invoke the callback once for each concrete file. Currently only used for UDIM tiles." — API, https://docs.blender.org/api/5.3/bpy_types_enum_items/file_path_foreach_flag_items.html
- "True when the path was expanded from a UDIM tile or sequence frame. These paths can not be edited." — API, https://docs.blender.org/api/5.3/bpy.types.BlendDataPathMeta.html

**Not covered.** Whether a multi-view *input* image set to "Individual" reads one file per view (the sources describe this only for output). How Blender finds the frames of a sequence beyond "must contain a digit".

**Where I looked.** Image, UDIM, volume, cache file, point cache, fluid cache, stereoscopy and sequencer image-strip pages; the API path flags and BlendDataPathMeta. Searched "sequence", "UDIM", "tile", "per frame" and "individual".

**Outside the interface.** *Brought up to date by the hand tests of 2026-10-17 — see* Observed *below. What follows is what the sources alone said.* These are stored settings, so they hold the same way in a background run. That is **inferred**. The expansion tools are Python functions.

**Operating system.** *Brought up to date by the hand tests of 2026-10-17 — see* Observed *below.* Not stated as differing.

### Observed — hand tests on the Creator's Mac, 2026-10-17

**Label: observed.** Same machine, Blender and way of starting it as under question 1; `<W>` is the work folder.

**Entry 3 — one recorded path standing for several files.** Two UDIM tiles `tiles/tile.1001.png` and `tiles/tile.1002.png` and a three-frame sequence `seq/shot_0001.png` to `shot_0003.png` were written to disk beside a new file `multi.blend`. One image was loaded from `//tiles/tile.1001.png` with its `source` set to `TILED`, another from `//seq/shot_0001.png` with `source` set to `SEQUENCE`, and the file saved:

```
   image=shot_0001.png  source=SEQUENCE stored='//seq/shot_0001.png' tiles=[1001]
   image=tile.1001.png  source=TILED    stored='//tiles/tile.<UDIM>.png' tiles=[1001, 1002]
blend_paths(): ['//seq/shot_0001.png', '//tiles/tile.<UDIM>.png', '//volumes/smoke.vdb']
file_path_map() non-empty: [('Vol', ['//volumes/smoke.vdb']), ('shot_0001.png', ['//seq/shot_0001.png']), ('tile.1001.png', ['//tiles/tile.<UDIM>.png'])]
```

Blender rewrote the tiled image's path into the `<UDIM>` form by itself, and one recorded path stood for two files. Both listings return the un-expanded text. (`//volumes/smoke.vdb` is a Volume data-block put in the same file for the reading under question 12.)

`bpy.data.file_path_foreach` was asked for a flag that does not exist, to make it name the real ones, and then walked with its default flags and with the three expand flags added. The default was read off the function's own description in this build: `file_path_foreach(visit_path_fn, *, subset=None, visit_types=None, flags={'SKIP_PACKED', 'SKIP_WEAK_REFERENCES'})`.

```
invalid flag probe: ValueError flags: 'NOT_A_REAL_FLAG' not found in ('SKIP_LINKED', 'SKIP_PACKED', 'SKIP_WEAK_REFERENCES', 'SKIP_MULTIFILE', 'RELOAD_EDITED', 'RESOLVE_TOKEN', 'EXPAND_TOKENS', 'EXPAND_SEQUENCES', 'EXPAND_CACHES')
plain:
   visited '//volumes/smoke.vdb' is_expanded=False is_cache=False
   visited '//seq/shot_0001.png' is_expanded=False is_cache=False
   visited '//tiles/tile.<UDIM>.png' is_expanded=False is_cache=False
default flags of file_path_foreach: ['SKIP_PACKED', 'SKIP_WEAK_REFERENCES']
with EXPAND_TOKENS, EXPAND_SEQUENCES, EXPAND_CACHES added to the default flags:
   visited '//volumes/smoke.vdb' is_expanded=False is_cache=False
   visited '<W>/e03/seq/shot_0001.png' is_expanded=True is_cache=False
   visited '<W>/e03/seq/shot_0002.png' is_expanded=True is_cache=False
   visited '<W>/e03/seq/shot_0003.png' is_expanded=True is_cache=False
   visited '<W>/e03/tiles/tile.1001.png' is_expanded=True is_cache=False
   visited '<W>/e03/tiles/tile.1002.png' is_expanded=True is_cache=False
```

So one recorded path expanded into three files for the sequence and two for the tiles, each given as a full path and marked `is_expanded=True`.

Two attempts to read the default flags failed before the description was read, and are kept as they happened: asking Blender's own property table for `file_path_foreach` gave `KeyError: 'bpy_prop_collection[key]: key "file_path_foreach" not found'`, and asking Python's `inspect.signature` gave `ValueError: no signature found for builtin <built-in method file_path_foreach of BlendData object at 0x1110c2cc0>`. Neither changed anything that was read.

**Standing lines, as the hand tests leave them.** *Driven from outside:* every reading was taken with no window; the token form is stored the same way, and the expansion is a plain function call. *Operating system:* macOS only, on the machine named under question 1.

---

## 5. In what ways can a recorded location be written?

**Label:** documented, except where marked.

**Answer.**
- **Relative to the `.blend` file**, starting with `//`.
- **Absolute**: "full path from the system's root".
- **Relative to a library file.** Paths inside linked data are relative to *that library*, not to the open file.
- **With path templates** in curly braces. The variables are `{blend_name}`, `{blend_dir}`, `{blend_name_lib}`, `{blend_dir_lib}`, `{fps}`, `{resolution_x}`/`{resolution_y}`, `{scene_name}`, `{camera_name}` and `{node_name}`. Number format specifiers are written as `{fps:###}`, and a literal brace is written as `{{` or `}}`.
  - The manual says templates work in the render output path and the compositor File Output node.
  - The API also marks Geometry Nodes bake folders as accepting them. The 5.3 notes add "Bake path template support", `{project_root}` and custom project variables.
- **Frame numbers** written as `#` (the number of `#` sets the padding). If there is none, `####` is added.
- **A view name** replacing `%` in the render output path.
- **UDIM tile tokens** `<UDIM>` and `<UVTILE>`, and numbered image sequences.
- **An OSL shader as a bare module name** (no folder), looked up in the shader search path.

Outside the file, but nearby: the texture cache folder may be "relative to the source image file or as an absolute path". A project's colour configuration may be an absolute path, a `{project_root}` path, or an `ocio://` built-in name.

**Evidence (2026-10-11):**
- "Blender stores a partial path evaluated relative to the directory location of the referencing blend-file." and "when entering the path into a text field, use a double slash prefix (//) to make it so." — manual, https://docs.blender.org/manual/en/5.3/files/blend/open_save.html
- "Make all paths to external files absolute (= full path from the system’s root)." — manual, https://docs.blender.org/manual/en/5.3/interface/window_system/topbar.html
- "When using Blender data from linked libraries there is an unfortunate complication since the path will be relative to the library rather than the open blend-file." — API, https://docs.blender.org/api/5.3/info_gotchas_file_paths_and_encoding.html
- "Path templates substitute template expressions (written {abcd}, where “abcd” is a variable name) in a filepath when the path is used." and "Currently path templates are only supported for the render output path in Scene properties and the output paths in the compositor’s File Output node." and "{{ translates to a single { in the final path." — manual, https://docs.blender.org/manual/en/5.3/files/file_paths.html
- "Location on disk where the bake data is stored (default “”, never None, blend relative // prefix supported, Supports template expressions)" — API, https://docs.blender.org/api/5.3/bpy.types.NodesModifier.html
- Release notes: "Bake path template support" — https://developer.blender.org/docs/release_notes/5.3/geometry_nodes/; "`{project_root}` and custom project variables for path templates." — https://developer.blender.org/docs/release_notes/5.3/core/
- "Filepath – A string interpreted as a filepath. Will be used as-is (unescaped) when substituted into part of a filepath." — API, https://docs.blender.org/api/5.3/bpy.types.ProjectVariableString.html
- "When the filename does not contain #, the suffix #### is added to the filename." — manual, https://docs.blender.org/manual/en/5.3/advanced/command_line/arguments.html
- "view (str) – View, The name of the view to use to replace the “%” chars" — API, https://docs.blender.org/api/5.3/bpy.types.RenderSettings.html
- "<UDIM>: A 4-digit notation calculated as 1001 + u-tile + v-tile * 10." — manual, https://docs.blender.org/manual/en/5.3/modeling/meshes/uv/workflows/udims.html
- "the filename of the images must contain a digit to indicate the frame order" — manual, https://docs.blender.org/manual/en/5.3/files/media/image_formats.html
- "specify just the module name, which will be looked up in the shader search path." — manual, https://docs.blender.org/manual/en/5.3/render/shader_nodes/utilities/script.html
- "May be an absolute file path, project relative path with {project_root}, or an ocio:// built-in OpenColorIO config" — API, https://docs.blender.org/api/5.3/bpy.types.BlenderProject.html

**Not covered.** Path separators, drive letters and network-share forms (see questions 6 and 7). Whether templates work in any field beyond those named. The manual names only two, while the API and the release notes name more. That gap is the sources' own.

**Where I looked.** File paths page, open/save page, topbar, command-line arguments, UDIM, image formats, Script node, the gotchas page, the API for every "Supports template expressions" property, the project API pages, and the 5.3 release notes. Searched "template", "relative", "absolute", "#", "token" and "variable".

**Outside the interface.** *Brought up to date by the hand tests of 2026-10-17 — see* Observed *below. What follows is what the sources alone said.* The command line accepts `//` and templates in `-o` ("Use // at the start of the path to render relative to the blend-file." and "You can use path templating features such as {blend_name} in the path."), at https://docs.blender.org/manual/en/5.3/advanced/command_line/arguments.html. Template errors show as a red field in the interface. Outside it, "rendering an animation will fail with an error message indicating the path errors" (file paths page). That this message is what a background run sees is **inferred**.

**Operating system.** *Brought up to date by the hand tests of 2026-10-17 — see* Observed *below.* Not stated as differing. See question 6.

### Observed — hand tests on the Creator's Mac, 2026-10-17

**Label: observed.** Same machine, Blender and way of starting it as under question 1; `<W>` is the work folder.

**Entry 5 — thirteen ways of writing a location.** A folder `<W>/e05/proj` was made holding `assets/tex.png` (8 by 8), with `<W>/e05/outer/tex.png` one level above it and a third copy at `<W>/home-copy/tex.png`. In a file saved in `proj`, thirteen image data-blocks were made, each with `source` set to `FILE` and its location typed a different way. Each was stored exactly as typed at the moment it was assigned:

```
ASSIGNED absolute         TYPED='<W>/e05/proj/assets/tex.png' STORED='<W>/e05/proj/assets/tex.png'
ASSIGNED relative         TYPED='//assets/tex.png' STORED='//assets/tex.png'
ASSIGNED relative-up      TYPED='//../outer/tex.png' STORED='//../outer/tex.png'
ASSIGNED backslash-rel    TYPED='//assets\\tex.png' STORED='//assets\\tex.png'
ASSIGNED tilde            TYPED='~/smoozy-path-test/tex.png' STORED='~/smoozy-path-test/tex.png'
ASSIGNED dollar-var       TYPED='$SMOOZY_DEP_TEST/tex.png' STORED='$SMOOZY_DEP_TEST/tex.png'
ASSIGNED dollar-brace     TYPED='${SMOOZY_DEP_TEST}/tex.png' STORED='${SMOOZY_DEP_TEST}/tex.png'
ASSIGNED drive-letter     TYPED='C:\\textures\\tex.png' STORED='C:\\textures\\tex.png'
ASSIGNED no-such-mount    TYPED='/Volumes/NoSuchShare/tex.png' STORED='/Volumes/NoSuchShare/tex.png'
ASSIGNED two-slash-server TYPED='//server/share/tex.png' STORED='//server/share/tex.png'
ASSIGNED backslash-unc    TYPED='\\\\server\\share\\tex.png' STORED='\\\\server\\share\\tex.png'
ASSIGNED smb-url          TYPED='smb://server/share/tex.png' STORED='smb://server/share/tex.png'
ASSIGNED web-url          TYPED='https://example.invalid/tex.png' STORED='https://example.invalid/tex.png'
```

(Each line prints the text as Python writes it, so `\\` in these lines is one backslash.) The file was saved twice with `bpy.ops.wm.save_as_mainfile`, once as `remap_off.blend` with `relative_remap=False` and once as `remap_on.blend` with `relative_remap=True`, both into `proj`. Each was opened in a fresh run with `SMOOZY_DEP_TEST` set to `<W>/home-copy`, and for each image the stored text, `bpy.path.abspath` of it and its size after `reload()` were printed; a size of 8 by 8 means the file was found, 0 by 0 that it was not. **The two files gave the same output, line for line:**

```
NAME=absolute         STORED='<W>/e05/proj/assets/tex.png' ABSPATH='<W>/e05/proj/assets/tex.png' SIZE=(8, 8)
NAME=backslash-rel    STORED='//assets/tex.png' ABSPATH='<W>/e05/proj/assets/tex.png' SIZE=(8, 8)
NAME=backslash-unc    STORED='\\\\server/share/tex.png' ABSPATH='\\\\server/share/tex.png' SIZE=(0, 0)
NAME=dollar-brace     STORED='${SMOOZY_DEP_TEST}/tex.png' ABSPATH='${SMOOZY_DEP_TEST}/tex.png' SIZE=(0, 0)
NAME=dollar-var       STORED='$SMOOZY_DEP_TEST/tex.png' ABSPATH='$SMOOZY_DEP_TEST/tex.png' SIZE=(0, 0)
NAME=drive-letter     STORED='C:/textures/tex.png' ABSPATH='C:/textures/tex.png' SIZE=(0, 0)
NAME=no-such-mount    STORED='/Volumes/NoSuchShare/tex.png' ABSPATH='/Volumes/NoSuchShare/tex.png' SIZE=(0, 0)
NAME=relative         STORED='//assets/tex.png' ABSPATH='<W>/e05/proj/assets/tex.png' SIZE=(8, 8)
NAME=relative-up      STORED='//../outer/tex.png' ABSPATH='<W>/e05/proj/../outer/tex.png' SIZE=(8, 8)
NAME=smb-url          STORED='smb://server/share/tex.png' ABSPATH='smb://server/share/tex.png' SIZE=(0, 0)
NAME=tilde            STORED='~/smoozy-path-test/tex.png' ABSPATH='~/smoozy-path-test/tex.png' SIZE=(0, 0)
NAME=two-slash-server STORED='//server/share/tex.png' ABSPATH='<W>/e05/proj/server/share/tex.png' SIZE=(0, 0)
NAME=web-url          STORED='https://example.invalid/tex.png' ABSPATH='https://example.invalid/tex.png' SIZE=(0, 0)
```

So, as read back after a save and a fresh open: a relative path may climb above the file's folder with `//../`; a home-folder shorthand is kept as typed and not expanded; an environment variable in either form is kept and not expanded, though the variable was set; a backslash after the first character becomes a forward slash — `//assets\tex.png` becomes `//assets/tex.png` and `C:\textures\tex.png` becomes `C:/textures/tex.png` — while the two leading backslashes of `\\server\share\tex.png` are kept and the rest turn, giving `\\server/share/tex.png`; and a location typed as `//server/share/tex.png` is taken as relative to the file, resolving to `<W>/e05/proj/server/share/tex.png`. Whether the re-spelling happens at the save or at the open is not told apart by this entry; entry 49, under question 20, tells it apart.

`bpy.ops.file.report_missing_files()` on either file then printed, returning `{'FINISHED'}`:

```
Warning: Path '\\server/share/tex.png' not found, from local data-block 'IMbackslash-unc'
Warning: Path '${SMOOZY_DEP_TEST}/tex.png' not found, from local data-block 'IMdollar-brace'
Warning: Path '$SMOOZY_DEP_TEST/tex.png' not found, from local data-block 'IMdollar-var'
Warning: Path '/c/textures/tex.png' not found, from local data-block 'IMdrive-letter'
Warning: Path '/Volumes/NoSuchShare/tex.png' not found, from local data-block 'IMno-such-mount'
Warning: Path 'smb:/server/share/tex.png' not found, from local data-block 'IMsmb-url'
Warning: Path '~/smoozy-path-test/tex.png' not found, from local data-block 'IMtilde'
Warning: Path '<W>/e05/proj/server/share/tex.png' not found, from local data-block 'IMtwo-slash-server'
Warning: Path 'https:/example.invalid/tex.png' not found, from local data-block 'IMweb-url'
```

Three of those warnings print the path differently from the stored text: `C:/textures/tex.png` is printed `/c/textures/tex.png`, `smb://server/share/tex.png` is printed `smb:/server/share/tex.png`, and `https://example.invalid/tex.png` is printed `https:/example.invalid/tex.png`. The stored text and `bpy.path.abspath` kept the original in each case.

**Entry 6 — path templates.** A file was saved as `<W>/e06/out.blend` (Cycles, 16 by 16, one sample, frames 1 to 2) and driven from a script with no window.

*In the render output path.* With `//templated/{blend_name}_frame_`:

```
   render.filepath = '//templated/{blend_name}_frame_'
   frame_path(frame=1) = '<W>/e06/templated/out_frame_0001.png'
00:00.388  render           | Saved: '<W>/e06/templated/out_frame_0001.png'
00:00.389  render           | Saved: '<W>/e06/templated/out_frame_0002.png'
   folder created? True   contents: ['out_frame_0001.png', 'out_frame_0002.png']
```

*In an image's path*, the same braces were left alone: `stored '//{blend_name}_tex.png'  resolved '<W>/e06/{blend_name}_tex.png'`.

*A broken template*, `//broken/{blend_name_frame_`, stopped both the path calculation and the render, and made no folder. The script printed:

```
   frame_path ERR RuntimeError: Error: Parse errors in path '//broken/{blend_name_frame_':
- Invalid or incomplete template expression '{blend_name_frame_'.

Error: Parse errors in path '//broken/{blend_name_frame_':
- Invalid or incomplete template expression '{blend_name_frame_'.
   render ERR RuntimeError: Error: Parse errors in path '//broken/{blend_name_frame_':
- Invalid or incomplete template expression '{blend_name_frame_'.

   folder created? False   contents: None
```

(the unindented `Error:` pair is what Blender printed on its own when the render was asked for). Entry 9, recorded under question 17, rendered a file whose output path is `//tmpl/{blend_name.png` from the command line: a single frame and an animation each stopped with exit status 1 and printed `reports          | ERROR Parse errors in path '//tmpl/{blend_name.png':` followed by `- Invalid or incomplete template expression '{blend_name.png'.`

*Each variable in turn*, put into `//tv/{<name>}_` and `frame_path(frame=1)` asked for:

```
   blend_name     -> '<W>/e06/tv/out_0001.png'
   blend_dir      -> '/private/tmp/claude-501/smoozy-hand-tests-5.3/Dependencies/e06/tv/private/tmp/claude-501/smoozy-hand-tests-5.3/Dependencies/e06/_0001.png'
   blend_name_lib -> '<W>/e06/tv/out_0001.png'
   blend_dir_lib  -> '/private/tmp/claude-501/smoozy-hand-tests-5.3/Dependencies/e06/tv/private/tmp/claude-501/smoozy-hand-tests-5.3/Dependencies/e06/_0001.png'
   scene_name     -> '<W>/e06/tv/Scene_0001.png'
   camera_name    -> '<W>/e06/tv/Cam_0001.png'
   fps            -> '<W>/e06/tv/24.0_0001.png'
   resolution_x   -> '<W>/e06/tv/16_0001.png'
   resolution_y   -> '<W>/e06/tv/16_0001.png'
   node_name      -> RuntimeError: Error: Parse errors in path '//tv/{node_name}_': - Unknown variable referenced in template expression '{node_name}'. 
   project_root   -> RuntimeError: Error: Parse errors in path '//tv/{project_root}_': - Unknown variable referenced in template expression '{project_root}'. 
```

(The two `blend_dir` lines are given exactly as printed, without the `<W>` shortening, because the work folder appears twice in each: once as the place `//tv/` resolves to, and once as what the variable gave.) So nine variables were accepted in the render output path. `node_name` was refused, and so was `project_root`, which the 5.3 release notes add; no project was active in this run, and whether it is accepted when one is was not tried.

*In a geometry-nodes bake folder.* A cube carrying a simulation zone was put in a saved file with frames 1 to 3, its modifier's bake target set to `DISK`, and its bake folder set three ways in three separate files; then the zone was baked with `bpy.ops.object.geometry_node_bake_single(session_uid=…, modifier_name=…, bake_id=…)`, the operator's arguments having been read off the running Blender first. With a plain folder `//gnbake/plain/`, the control:

```
   geometry_node_bake_single -> {'FINISHED'}
   bake_directory stored: '//gnbake/plain/'
   ON DISK gnbake/plain/1516041663/blobs/ files: 3
   ON DISK gnbake/plain/1516041663/meta/ files: 3
```

With `//gnbake/{blend_name}/`, in a file named `baketmpl2.blend`:

```
   geometry_node_bake_single -> {'FINISHED'}
   bake_directory stored: '//gnbake/{blend_name}/'
   ON DISK gnbake/baketmpl2/1864379878/blobs/ files: 3
   ON DISK gnbake/baketmpl2/1864379878/meta/ files: 3
```

With `//gnbake/{blend_name}_{scene_name}/`:

```
Info: Cannot determine bake location on disk. Falling back to packed bake.
   geometry_node_bake_single -> {'FINISHED'}
   bake_directory stored: '//gnbake/{blend_name}_{scene_name}/'
```

and no `gnbake` folder of any kind was made. So a bake folder does take `{blend_name}`, the folder written being named after the file; with `{scene_name}` added the bake was not written to disk at all and Blender said it fell back to packing, while the operator still returned `{'FINISHED'}`. The same happened in the first run of this entry, made in `out.blend` with the same two-variable folder.

**Entry 13 — a broken output-path template through Blender's window.** Two files were saved by script with Cycles, 32 by 32, one frame, and the Properties editor showing the Output tab: `bad.blend` with the output path `//out/{blend_name.png` and `good.blend` with `//out/{blend_name}.png`. Each was opened with its window from the terminal on the Mac (`--factory-startup --enable-event-simulate <file> --python <script>`) and driven by timers in the script; pictures were taken with Blender's own `bpy.ops.screen.screenshot`, at 3024 by 1724 pixels. A first-run setup panel covers the middle of the window on opening and was closed by a simulated Esc key. Neither run printed anything to the terminal about the path.

*The field.* In both pictures the field under *Output* reads the path as saved. Averaging the colour of the same strip of the field in the two pictures:

```
bad_output_tab   path-field strip average colour r=0.606 g=0.172 b=0.172 (7200 pixels)
good_output_tab  path-field strip average colour r=0.233 g=0.233 b=0.233 (7200 pixels)
```

The broken one is drawn red, with the text `//out/{blend_name.png` in white on it; the sound one is Blender's ordinary grey.

*The tooltip was not reached.* The pointer was moved onto the field with `Window.event_simulate`, in the units that a simulated click on the Render Region checkbox had first been shown to act in (`use_border before full-pixel click: False`, `after: True`), once directly and once with a small move before it, and the window was photographed after one, three and five seconds. No tooltip appeared in any picture. What the tooltip says stays open.

*A render started the way the menu does.* `bpy.ops.render.render('INVOKE_DEFAULT', animation=False, write_still=True)` was called from a timer:

```
render INVOKE -> {'RUNNING_MODAL'}
windows now: 2
WindowManager.reports: [('ERROR', "Parse errors in path '//out/{blend_name.png':\n- Invalid or incomplete template expression '{blend_name.png'.")]
out folder exists: False None
```

The second window, the render window, was 160 by 128 and its picture was plain grey with no image in it. The main window's status bar carried, after a red error mark, on one line:

```
Parse errors in path '//out/{blend_name.png':- Invalid or incomplete template expression '{blend_name.png'.
```

and with the 3D viewport switched to the Info editor, the Info log showed one entry with a red error mark:

```
Parse errors in path '//out/{blend_name.png':
  - Invalid or incomplete template expression '{blend_name.png'.
```

So in the window the error lands in three places a person can see — the red field, the status bar and the Info log — and in `WindowManager.reports`, which a script can read; no folder is made. In the runs with no window recorded above and under question 17, the same words were printed to the terminal instead.

**Entry 49 — every way of writing a location, through a save, an opening and a plain save.** Recorded in full under question 20. In short: fourteen images were set to twelve different spellings, and each was stored as typed at the moment of setting. The build's own save wrote the two backslash spellings, `//assets\tex.png` and `//assets\..\assets//TEX.PNG `, into the file as typed (both found in the file's text); the first opening of that file read them as `//assets/tex.png` and `//assets/../assets//TEX.PNG `, and a plain save then wrote the forward-slash forms into the file. The `..` segment, the doubled separator, the wrong capitals, the trailing space, the non-Latin name, the space in a name, the missing place, the place one folder up and the full path read the same after the opening and after the plain save as when they were set. Saving the same file into another folder (entry 50) rewrote the relative ones and shortened `..` and doubled separators.

**Standing lines, as the hand tests leave them.** *Driven from outside:* every form above was assigned, saved, read back and reported on with no window, and templates behaved the same there: a broken one stops the path calculation and the render and makes no folder. The difference with a window (entry 13) is where the words go — onto the field, the status bar and the Info log, and into `WindowManager.reports`, rather than onto the terminal. *Operating system:* macOS only, on the machine named under question 1. What a Windows Blender does with any of these forms was not run in this sitting: no Windows machine was available.

---

## 6. What does a location recorded on one kind of computer mean on another, including a kind that names its disks?

**Label:** **not in the sources** for the core question. Some nearby facts are documented. The conclusions drawn from them are **inferred**.

**Answer.** No source says what Blender does with a location written on one operating system when the file is opened on another. Nor does any source say this for a location carrying a drive letter. What the sources do say:
- Relative (`//`) paths let a folder holding the `.blend` file and its files be moved and sent to someone else. That this is the portable form between computers is **inferred**. The manual speaks only of moving and sending, not of changing operating system.
- Absolute paths are "full path from the system's root".
- For *exported* OBJ and FBX files, the manual warns that "absolute paths may only be correct on your own system". It also says a relative path cannot be made across drives on Windows. That the same limit applies to paths inside a `.blend` file is **inferred** only. The source speaks of export, not of `.blend` files.
- Blender's Python path helpers include one that swaps separators for the computer's own. This shows that separators differ between systems (**inferred**). It does not say Blender converts recorded paths on load.
- The compositor File Output node accepts either `/` or `\` in its *input names* as a folder break. That is documented for that node only.
- The OSL shader search path is a different folder on Linux, Windows and macOS.

**Evidence (2026-10-11):**
- "With a relative path, you can move the blend-file to a new location provided the externally linked files are moved along with it. For example, you could send someone a folder that contains a blend-file and a subfolder of external images that it references." — manual, https://docs.blender.org/manual/en/5.3/files/blend/open_save.html
- "Make all paths to external files absolute (= full path from the system’s root)." — manual, https://docs.blender.org/manual/en/5.3/interface/window_system/topbar.html
- "absolute paths may only be correct on your own system" and "Relative: Uses relative paths in every case (except when on a different drive on Windows)." — manual (OBJ export), https://docs.blender.org/manual/en/5.3/files/import_export/obj.html
- "Replace the path separator with the system’s native os.sep." and, for basename, "Use for Windows compatibility." — API, https://docs.blender.org/api/5.3/bpy.path.html
- "If an input name contains a slash character (/ or \), the path is interpreted as a directory hierarchy." — manual, https://docs.blender.org/manual/en/5.3/compositing/types/output/file_output.html
- Shader search path listed separately for "Linux", "Windows" and "macOS" — manual, https://docs.blender.org/manual/en/5.3/render/shader_nodes/utilities/script.html
- "Every missing file found in the search will be recovered. Those recoveries will be done as absolute paths" — manual, https://docs.blender.org/manual/en/5.3/interface/window_system/topbar.html

**Not covered.** What a path like a Windows drive-letter path means when opened on Linux or macOS, and the reverse. Whether Blender converts separators on load. Whether a relative path is stored with one separator style. How Find Missing Files behaves across systems.

**Where I looked.** Searched the manual, API and release notes for "drive letter", "drive", "UNC", "backslash", "forward slash", "separator", "native_pathsep", "Windows", "macOS", "Linux" and "Unix" near "path", "directory" or "drive", and for "C:\". Read the open/save, topbar, OBJ/FBX export, File Output node and Script node pages, plus `bpy.path`. The only drive-related statements found are the OBJ/FBX export lines quoted above.

**Outside the interface.** *Brought up to date by the hand tests of 2026-10-17 — see* Observed *below. What follows is what the sources alone said.* Nothing in the sources separates the two cases.

**Operating system.** *Brought up to date by the hand tests of 2026-10-17 — see* Observed *below.* This question is about operating systems, and the sources are silent on it beyond the points above.

### Observed — hand tests on the Creator's Mac, 2026-10-17

**Label: observed.** Same machine, Blender and way of starting it as under question 1; `<W>` is the work folder.

**Entry 5 — a drive-letter path and a path naming a disk, on a Mac.** Recorded in full under question 5. An image given the location `C:\textures\tex.png` in a file on the Mac, saved and opened again with no window, read back `STORED='C:/textures/tex.png' ABSPATH='C:/textures/tex.png' SIZE=(0, 0)`: the backslashes became forward slashes, the text was resolved as it stands, and no file was found. Report Missing Files printed it as `Warning: Path '/c/textures/tex.png' not found, from local data-block 'IMdrive-letter'`, which is not the stored text. An image given `/Volumes/NoSuchShare/tex.png` read back `STORED='/Volumes/NoSuchShare/tex.png' ABSPATH='/Volumes/NoSuchShare/tex.png' SIZE=(0, 0)`. Nothing was printed about either until Report Missing Files was asked.

**Entry 16 — what Find Missing Files writes back.** Recorded in full under question 8. On the Mac, in a run with no window, `bpy.ops.file.find_missing_files(directory=<W>/e16/csmount/search2, find_all=False)` repaired two missing images and wrote each back as `//search2/Texture.PNG` and `//search2/other.png` — relative to the blend-file — where the manual quoted above says recoveries "will be done as absolute paths".

**What this does not settle.** What a location written on one kind of computer means on another. That needs the file carried to a second computer running another system, and this sitting had only the Mac — see the *Cannot be run* section of `Audits/Blender Research/Dependencies/Hand tests — 5.3.md`.

**Standing lines, as the hand tests leave them.** *Driven from outside:* the readings above were taken with no window. *Operating system:* macOS only, and only for strings typed on the Mac; nothing here crosses to another system.

---

## 7. What does a location pointing somewhere other than the computer the file sits on mean, at either end?

**Ambiguity.** "At either end" could mean (a) the computer that wrote the file and the computer that reads it, or (b) the computer holding the file and the remote place the path points at. I answer both as far as the sources go.

**Label:** **not in the sources** for a `.blend` file's own paths pointing at a network location. Nearby facts are documented.

**Answer.**
- The sources say nothing about how an image, library or cache path that points at a network share is recorded or followed. Nor do they say whether it means the same thing on the computer that wrote it and on one that reads it.
- The File Browser lists "network mounts" as places to pick from.
- The manual's "Primitive Render Farm" hint has several machines render into one shared network folder. It relies on the Overwrite and Placeholders settings. That is about *output*.
- Online asset libraries are the one remote kind described. They are reached over the network, cached locally on download, and cannot be linked, only appended or packed. The `.blend` file therefore does not keep a remote pointer for them. That last step is **inferred**.
- The only network-share address form in the sources is for *extension repositories* on Windows (`file://HOST/share/…`). It is not about `.blend` paths.

**Evidence (2026-10-11):**
- "Volumes — Drives and network mounts." — manual, https://docs.blender.org/manual/en/5.3/editors/file_browser.html
- "Set up a shared directory over a network file system. Disable Overwrite, enable Placeholders in the Render Output panel. Start as many machines as you wish rendering to that directory." — manual, https://docs.blender.org/manual/en/5.3/render/output/properties/output.html
- "Online asset libraries provide assets that are downloaded on demand and cached locally." and "Assets cannot be linked directly and must instead be appended or packed." — manual, https://docs.blender.org/manual/en/5.3/files/asset_libraries/introduction.html
- "file://HOST/share/path/to/packages/index.json for network shares on Windows." — manual, https://docs.blender.org/manual/en/5.3/advanced/extensions/creating_repository/static_repository.html

**Not covered.** Everything about network paths stored in a `.blend` file: their written form, whether they survive moving to another computer, and how the reading computer resolves them. Also the manual defines `//` as the start of a relative path, and the sources do not say how a location that itself starts with two slashes is told apart from a relative one.

**Where I looked.** Searched for "network", "UNC", "share", "mount", "remote", "URL", "URI", "http" and "\\\\". Read the File Browser, render output, asset library, remote asset library, extension repository and deployment pages, and the API for UserAssetLibrary and BlendFileColorspace.

**Outside the interface.** *Brought up to date by the hand tests of 2026-10-17 — see* Observed *below. What follows is what the sources alone said.* Not stated.

**Operating system.** *Brought up to date by the hand tests of 2026-10-17 — see* Observed *below.* Not stated. The only OS-specific network form found is the Windows extension-repository line above.

### Observed — hand tests on the Creator's Mac, 2026-10-17

**Label: observed.** Same machine, Blender and way of starting it as under question 1; `<W>` is the work folder.

**Entry 5 — locations that look like other computers, written on a Mac.** Recorded in full under question 5. Four images in a file saved in `<W>/e05/proj` were given locations that name another place, the file saved and opened again with no window, and the stored text, `bpy.path.abspath` and the size after `reload()` read back:

```
NAME=no-such-mount    STORED='/Volumes/NoSuchShare/tex.png' ABSPATH='/Volumes/NoSuchShare/tex.png' SIZE=(0, 0)
NAME=two-slash-server STORED='//server/share/tex.png' ABSPATH='<W>/e05/proj/server/share/tex.png' SIZE=(0, 0)
NAME=backslash-unc    STORED='\\\\server/share/tex.png' ABSPATH='\\\\server/share/tex.png' SIZE=(0, 0)
NAME=smb-url          STORED='smb://server/share/tex.png' ABSPATH='smb://server/share/tex.png' SIZE=(0, 0)
```

The third was typed `\\server\share\tex.png`; Python prints its two leading backslashes doubled, so it read back as `\\server/share/tex.png`. So on this Mac a location typed `//server/share/…` is taken as relative to the file — the leading two slashes mean "beside the blend-file" — and resolves into a folder `server/share` beside it; a location typed with two leading backslashes keeps them and has its other backslashes turned to forward slashes; and none of the four is found, with nothing printed until Report Missing Files is asked, which then names each: `Warning: Path '<W>/e05/proj/server/share/tex.png' not found, from local data-block 'IMtwo-slash-server'`, `Warning: Path '\\server/share/tex.png' not found, from local data-block 'IMbackslash-unc'`, `Warning: Path 'smb:/server/share/tex.png' not found, from local data-block 'IMsmb-url'` and `Warning: Path '/Volumes/NoSuchShare/tex.png' not found, from local data-block 'IMno-such-mount'`.

**What this does not settle.** What a location pointing at a real place on another computer means at either end. That needs a second computer and a share both can reach, and this sitting had only the Mac. What a Windows Blender does with a `\\server\share` location was not run either: no Windows machine was available. See the *Cannot be run* section of `Audits/Blender Research/Dependencies/Hand tests — 5.3.md`.

**Standing lines, as the hand tests leave them.** *Driven from outside:* the readings were taken with no window. *Operating system:* macOS only, for strings typed on the Mac.

---

## 8. Does the difference between capital and small letters in a name matter, and does it matter the same on every kind of computer?

**Ambiguity.** "Name" could mean a *file or folder name in a recorded location*, or a *data-block name* (for example, the name of a linked object). I answer both.

**Label:** file names: **inferred** from one API line. Data-block names: **not in the sources**.

**Answer.**
- **File names.** Blender's Python path helpers include one to "Resolve a case insensitive path on a case sensitive system". From this I **infer** two things: some systems Blender runs on treat letter case in paths as significant and others do not, and a path recorded with the wrong case may not be found on the former. The source does not say Blender applies this repair itself when loading or rendering. It is offered to scripts. Another helper compares file extensions ignoring case by default.
- **Data-block names.** The manual says names are unique per type within a file. It does not say whether `Cube` and `cube` count as the same. The case-sensitive options found are for *searching* only (the Outliner filter and pattern select, both off by default).

**Evidence (2026-10-11):**
- "Resolve a case insensitive path on a case sensitive system, returning a string with the path if found else return the original path." and "case_sensitive (bool) – Check for matching case when comparing extensions." (default `case_sensitive=False`) — API, https://docs.blender.org/api/5.3/bpy.path.html
- "Their names are unique within a blend-file, for a given type." — manual, https://docs.blender.org/manual/en/5.3/files/data_blocks.html
- "Only use case sensitive matches of search string (default False)" — API, https://docs.blender.org/api/5.3/bpy.types.SpaceOutliner.html

**Not covered.** Whether Blender's own loading, linking, Find Missing Files or rendering ever ignores case. Which operating systems are case-sensitive (the sources do not name them). Whether data-block names differing only in case may coexist.

**Where I looked.** Searched the manual and API for "case sensitive", "case-sensitive", "case insensitive", "ignore case", "upper case", "lower case", "capital letter" and "resolve_ncase". Read `bpy.path`, the data-blocks page, the batch rename page and the Outliner filter.

**Outside the interface.** *Brought up to date by the hand tests of 2026-10-17 — see* Observed *below. What follows is what the sources alone said.* `bpy.path.resolve_ncase` is a Python function and works in a background run with a script. That is **inferred** from the command-line `-P` option.

**Operating system.** *Brought up to date by the hand tests of 2026-10-17 — see* Observed *below.* **Inferred** that it differs ("case sensitive system" implies others are not). Which systems is not stated.

### Observed — hand tests on the Creator's Mac, 2026-10-17

**Label: observed.** Same machine, Blender and way of starting it as under question 1; `<W>` is the work folder. Two kinds of disk were used on the one Mac: its ordinary disk, which `diskutil` reports as `File System Personality:   APFS`, and a disk image made in the work folder with `hdiutil create -size 64m -fs "Case-sensitive APFS" -volname SmoozyCS <W>/e16/cs.dmg` and attached at a folder of its own with `hdiutil attach <W>/e16/cs.dmg -mountpoint <W>/e16/csmount -nobrowse`, which `diskutil` reports as `File System Personality:   Case-sensitive APFS`. The image was detached afterwards.

**Entry 16 — the letter case of a file name, on the two kinds of disk.** On each disk a file `case.blend` was saved holding an image loaded from `//assets/Texture.PNG` (which existed then) and a second image `other` pointing at `//gone/other.png`; then the file on disk was renamed `texture.png`. Each `case.blend` was opened with no window and read. On the Mac's ordinary disk:

```
IMAGE Texture.PNG stored='//assets/Texture.PNG' abspath='<W>/e16/plain/assets/Texture.PNG' size=(32, 32) has_data=True
      os.path.exists(abspath) = True
      bpy.path.resolve_ncase(abspath) = '<W>/e16/plain/assets/Texture.PNG'
Warning: Path '<W>/e16/plain/gone/other.png' not found, from local data-block 'IMother'
```

On the case-sensitive disk image:

```
IMAGE Texture.PNG stored='//assets/Texture.PNG' abspath='<W>/e16/csmount/assets/Texture.PNG' size=(0, 0) has_data=False
      os.path.exists(abspath) = False
      bpy.path.resolve_ncase(abspath) = '<W>/e16/csmount/assets/texture.png'
Warning: Path '<W>/e16/csmount/gone/other.png' not found, from local data-block 'IMother'
Warning: Path '<W>/e16/csmount/assets/Texture.PNG' not found, from local data-block 'IMTexture.PNG'
```

So whether the capitals matter followed the disk: the same recorded text found the file on the ordinary disk and not on the case-sensitive one. Blender did not try the other capitals itself when loading, though `bpy.path.resolve_ncase` named the real small-letter file the moment it was asked.

*Find Missing Files on the case-sensitive disk.* `bpy.ops.file.find_missing_files(directory=<folder>, find_all=False)` was called, its arguments having been read off the running Blender first. With a search folder holding only `texture.png`:

```
search folder holds: ['texture.png']
Warning: Could not find 'other.png' in '<W>/e16/csmount/search'
Warning: Could not find 'Texture.PNG' in '<W>/e16/csmount/search'
find_missing_files -> {'FINISHED'}
   after: other stored='//gone/other.png'
   after: Texture.PNG stored='//assets/Texture.PNG'
```

It did not match `Texture.PNG` to `texture.png`, said so, and changed nothing. In a fresh run of the same file, with a search folder holding `Texture.PNG` and `other.png`:

```
search folder holds: ['Texture.PNG', 'other.png']
find_missing_files -> {'FINISHED'}
   after: other stored='//search2/other.png'
   after: Texture.PNG stored='//search2/Texture.PNG'
```

Both were repaired, and each was rewritten as a path relative to the blend-file pointing into the search folder, with nothing printed.

**Entry 17 — one file asked for by different capitals, on the Mac's ordinary disk.** A folder `assets` was made holding `tex.png` and a `lib.blend` holding one object `LibCube`. In a fresh empty session each time, the image was loaded by four spellings and the library linked from by four, with no window. (The script cut the folder out of the image error message before printing it; nothing else is shortened.)

```
on disk: ['lib.blend', 'tex.png']
--- an IMAGE asked for by three spellings of a name that exists once, as tex.png ---
  asked tex.png      -> size (32, 32) | has_data True | filepath 'tex.png'
  asked TEX.PNG      -> size (32, 32) | has_data True | filepath 'TEX.PNG'
  asked Tex.Png      -> size (32, 32) | has_data True | filepath 'Tex.Png'
  asked nosuch.png   -> RAISED RuntimeError: Error: Cannot read 'nosuch.png': No such file or directory
--- the same LIBRARY asked for by three spellings of lib.blend ---
  asked lib.blend      -> available ['LibCube'] | linked ['LibCube'] | recorded ['<W>/e17b/assets/lib.blend']
  asked LIB.BLEND      -> available ['LibCube'] | linked ['LibCube'] | recorded ['<W>/e17b/assets/LIB.BLEND']
  asked Lib.Blend      -> available ['LibCube'] | linked ['LibCube'] | recorded ['<W>/e17b/assets/Lib.Blend']
  asked nosuch.blend   -> RAISED OSError: load: <W>/e17b/assets/nosuch.blend failed to open blend file
```

On the Mac's ordinary disk a file name was found whatever its capitals, for an image and for a library alike, and the spelling asked for is the spelling recorded. The fourth line of each block is the control: a name that is not there fails and says so. (A first run printed the available list after it had been handed back filled with objects; it is kept in the run record of the list and changes nothing above.)

**Entry 18 — capitals in names inside a file.** A library `lib.blend` was saved holding one object `Cube`. A second file `names.blend` was built holding three objects `Cube`, `cube` and `CUBE`, three images `Texture`, `texture` and `TEXTURE`, and three materials `Mat`, `mat` and `MAT`, each with a fake user; it was saved and opened again. Then, from a fresh empty file each time, the library's object was asked for by three spellings, through `bpy.data.libraries.load(<lib>, link=True)` and through `bpy.ops.wm.link(filepath=<lib>/Object/<name>, directory=<lib>/Object/, filename=<name>)`. Last, in an empty session, `bpy.data.objects.new("Test", None)` was run and then `bpy.data.objects.new("test", None)`. All with no window:

```
LIB objects: ['Cube']
BEFORE SAVE objects: ['CUBE', 'Cube', 'cube']
AFTER RELOAD objects: ['CUBE', 'Cube', 'cube']
AFTER RELOAD images : ['TEXTURE', 'Texture', 'texture']
AFTER RELOAD mats   : ['MAT', 'Mat', 'mat']
LOOKUP objects.get('Cube') -> 'Cube'
LOOKUP objects.get('cube') -> 'cube'
LOOKUP objects.get('CUBE') -> 'CUBE'
LOOKUP objects.get('CuBe') -> None
LOOKUP images.get('Texture') -> 'Texture'
LOOKUP images.get('texture') -> 'texture'
LOOKUP images.get('TEXTURE') -> 'TEXTURE'
LOOKUP images.get('TeXtUrE') -> None
LINK available in library: ['Cube']
LINK asked 'Cube' -> got ['Cube']
   warning: UserWarning: load: '<W>/e18/lib.blend' does not contain objects["cube"]
LINK asked 'cube' -> got [None]
   warning: UserWarning: load: '<W>/e18/lib.blend' does not contain objects["CUBE"]
LINK asked 'CUBE' -> got [None]
wm.link 'Cube' -> {'FINISHED'} ; objects before [] after ['Cube']
wm.link 'cube' -> {'FINISHED'} ; objects before [] after []
wm.link 'CUBE' -> {'FINISHED'} ; objects before [] after []
objects.new('Test') then objects.new('test') -> ['Test', 'test']
```

So inside a file, names differing only in case are told apart exactly: all three of each kind survive a save and reopen, the second of `Test` and `test` is not renamed, and a lookup by other capitals finds nothing. Asking a library for the wrong capitals through `libraries.load` gives `None` and a warning naming the file; through `bpy.ops.wm.link` it answers `{'FINISHED'}`, prints nothing, and links nothing.

**Entry 49 — a place written with the wrong capitals, through a save, an opening and a plain save.** Recorded in full under question 20. In short: an image set to `//assets/TEX.PNG`, where the file on disk is `tex.png`, read `'//assets/TEX.PNG'` after the build's save, after a fresh opening and after a plain save, and its picture loaded at `(2, 2)` from the Mac's ordinary disk. Saved into another folder (entry 50) it became `'//../proj/assets/TEX.PNG'`, capitals kept.

**Standing lines, as the hand tests leave them.** *Driven from outside:* every reading here was taken with no window. *Operating system:* macOS only, on two kinds of disk on the one Mac; each is its own result. Linux's and Windows' own disks were not run in this sitting: no machine running either was available.

---

## 9. Do names written in characters outside the Latin alphabet survive being read out of a file, moved to another computer, and found again by a render?

**Label:** documented for how such names are stored, except where marked inferred. **Not in the sources** for surviving a move between computers and being found again by a render.

**Answer.**
- Every text inside a `.blend` file must be UTF-8. **Paths are the exception**: they may hold names that are not valid UTF-8, because such names exist on users' disks. Reading such a path from Python can raise an error unless handled as the guide shows.
- Data-block names are UTF-8, with a fixed length of 255 bytes. Non-Latin letters take more than one byte, so fewer fit.
- Fixed limits also apply to paths: directory 767, file name 255, full path 1023. The manual notes that "Multi-byte encoding means some Unicode characters use more than a single ASCII character". That non-Latin path names therefore reach these limits sooner is **inferred**.
- There is a repair tool that checks and fixes text in a file to be valid UTF-8. It is "needed for some old, 2.4x area files".
- One conflict in the manual: the limits page lists "identifier: 63" among fixed strings "Used for data-block names", while the data-blocks page says data-block names have "a fixed length of 255 bytes". I report both and do not pick one.

**Evidence (2026-10-11):**
- "To avoid the problem for Python integration and script authors we have decided that all strings in blend-files must be UTF-8, ASCII compatible." and "Paths are an exception to this rule since the existence of non-UTF-8 paths on the user’s file system cannot be ignored." — API, https://docs.blender.org/api/5.3/info_gotchas_file_paths_and_encoding.html
- "Data-block names have a fixed length of 255 bytes and are UTF-8 encoded. This is room for 255 basic ASCII characters, or less when using diacritics or non-latin glyphs, which use more than one byte per character." — manual, https://docs.blender.org/manual/en/5.3/files/data_blocks.html
- "directory: 767", "file-name: 255", "file-path: 1023", "identifier: 63" and "Multi-byte encoding means some Unicode characters use more than a single ASCII character." — manual, https://docs.blender.org/manual/en/5.3/advanced/limits.html
- "Check and fix all strings in current .blend file to be valid UTF-8 Unicode (needed for some old, 2.4x area files)" — API, https://docs.blender.org/api/5.3/bpy.ops.wm.html

**Not covered.** Whether a non-Latin file or folder name recorded on one computer matches the same name on another, for example if the two systems spell the same letters with different byte sequences. Whether a render then finds the file. Whether non-UTF-8 paths survive being moved between systems.

**Where I looked.** Searched the manual, API and release notes for "UTF-8", "utf8", "unicode", "ASCII", "non-ascii", "latin", "multi-byte", "encoding" and "special character". Read the gotchas page, the limits page, the data-blocks page and `bpy.path`.

**Outside the interface.** *Brought up to date by the hand tests of 2026-10-17 — see* Observed *below. What follows is what the sources alone said.* The UTF-8 rule and the path exception are stated for the Python layer and the file itself, so they apply in background runs. That is **inferred**.

**Operating system.** *Brought up to date by the hand tests of 2026-10-17 — see* Observed *below.* Not stated as differing.

### Observed — hand tests on the Creator's Mac, 2026-10-17

**Label: observed.** Same machine, Blender and way of starting it as under question 1; `<W>` is the work folder.

**Entry 19 — a non-Latin path, read, rendered and moved on the Mac.** A folder `Тестовая папка` holding a red 32 by 32 `текстура_日本語.png` was put beside `nonlatin.blend`, whose plane uses that image as its emission colour through a relative path; camera, sun, Cycles, 32 by 32, one sample. The file was opened with no window and read, and one frame rendered from the command line:

```
  repr(filepath)      = '//Тестовая папка/текстура_日本語.png'
  os.fsencode(...)    = b'//\xd0\xa2\xd0\xb5\xd1\x81\xd1\x82\xd0\xbe\xd0\xb2\xd0\xb0\xd1\x8f \xd0\xbf\xd0\xb0\xd0\xbf\xd0\xba\xd0\xb0/\xd1\x82\xd0\xb5\xd0\xba\xd1\x81\xd1\x82\xd1\x83\xd1\x80\xd0\xb0_\xe6\x97\xa5\xe6\x9c\xac\xe8\xaa\x9e.png'
  repr(abspath)       = '<W>/e19/before/Тестовая папка/текстура_日本語.png'
  size                = (32, 32)
render EXIT 0
00:00.357  render           | Saved: '<W>/e19/out/before_0001.png'
```

The blend-file and the folder were then moved together into another folder, `moved somewhere`, on the same Mac, and the same done again:

```
  repr(filepath)      = '//Тестовая папка/текстура_日本語.png'
  repr(abspath)       = '<W>/e19/moved somewhere/Тестовая папка/текстура_日本語.png'
  size                = (32, 32)
render EXIT 0
```

with the same bytes for `os.fsencode` as before. The centre pixel of each frame:

```
centre pixel before the move (1.0, 0.102, 0.102, 1.0)
centre pixel after the move  (1.0, 0.102, 0.102, 1.0)
```

which is the texture's own red. Nothing was printed about the path in any of these runs. The bytes are UTF-8 throughout.

*The same letters spelled two ways.* A file `café.png` was written to disk with the é as one character; the Mac's disk kept the name as `b'caf\xc3\xa9.png'`. In the moved `nonlatin.blend`, a second image was given the location `//café.png` typed with the é as a plain `e` followed by a combining accent:

```
  typed bytes  : b'//cafe\xcc\x81.png'
  stored bytes : b'//cafe\xcc\x81.png'
  size after reload: (32, 32) | has_data: True
  os.path.exists(abspath): True
```

So Blender stored the spelling it was given, byte for byte, and on the Mac's disk that spelling found the file whose name is held the other way.

*How long a data-block name can be.* The sources give two limits — 63 and 255 bytes. Three objects were made in an empty file, saved and opened again:

```
  ascii100     asked 100 chars 100 bytes -> got 100 chars 100 bytes
  cyrillic100  asked 100 chars 200 bytes -> got 100 chars 200 bytes
  ascii300     asked 300 chars 300 bytes -> got 255 chars 255 bytes
  after save and reopen: ascii100     100 chars 100 bytes
  after save and reopen: ascii300     255 chars 255 bytes
  after save and reopen: cyrillic100  100 chars 200 bytes
```

A name of 300 Latin letters was cut to 255 bytes when it was made; names of 100 and 200 bytes were kept whole through a save.

**What this does not settle.** Whether such a name survives being carried to a computer running another system, and whether a spelling held one way there finds a file held the other way. That needs a second computer, and this sitting had only the Mac — see the *Cannot be run* section of `Audits/Blender Research/Dependencies/Hand tests — 5.3.md`.

**Entry 49 — a non-Latin place, through a save, an opening and a plain save.** Recorded in full under question 20. In short: an image set to `//Тестовая папка/текстура_日本語.png` read the same, letter for letter, after the build's save, after a fresh opening and after a plain save, and its picture loaded at `(2, 2)`. Saved into another folder (entry 50) it became `'//../proj/Тестовая папка/текстура_日本語.png'`.

**Standing lines, as the hand tests leave them.** *Driven from outside:* every reading here was taken with no window, and every render was started from the command line. *Operating system:* macOS only, on the machine named under question 1.

---

## 10. Are some of the things a file points at places it writes to rather than reads from, and where does such a write land?

**Label:** documented, except where marked. The Cycles OSL and maketx items are **observed**.

**Answer.** Yes. These are the places the sources show being written to, and where the write lands:

| Place | Where it lands |
|---|---|
| Render output path | The given path. `//` means the `.blend` file's folder. The default is `//`. |
| Compositor File Output node | Its folder. Missing sub-folders are created. |
| Point-cache "Disk Cache" | A folder named `blendcache_[filename]` beside the `.blend` file. The file must be saved first. If that folder cannot be written, caching does not happen. |
| Point cache of linked data | With "Use Library Path", the library file's own path is used. |
| Fluid cache | Its cache folder, with one sub-folder per simulation type. |
| Geometry Nodes bakes set to Disk | The bake folder. The `.blend` file must be saved first. |
| Ocean bake, Dynamic Paint output, Multiresolution "save external" | Their own folders or files. |
| Texture baking set to External | The bake file path, default `//`. |
| Sequencer proxies | `<path of original footage>/BL_proxy/<clip name>`, a custom folder, or one project proxy folder. This lands next to the *source footage*, not next to the `.blend` file. |
| Cycles texture cache (`.tx`) | A `blender_tx/` folder next to each image, unless the Texture Cache preference names another folder. This also writes next to *read* files. |
| OSL Script node set to External with a `.osl` file | The compiled `.oso` is written into the same folder as the `.osl`. Cycles reports an error if that write fails. |
| Unpacking | Either the `.blend` file's folder, grouped into sub-folders such as "textures", or the original locations. Unpacking linked libraries overwrites existing `.blend` files. |
| Saving the `.blend` file | May also save modified images to their own files, depending on a preference. |
| Collection exporters | Their file paths. |

**Evidence (2026-10-11 unless marked):**
- "This setting expands Relative Paths where a // prefix represents the directory of the current blend-file." — manual, https://docs.blender.org/manual/en/5.3/render/output/properties/output.html; RenderSettings default "//" — API, https://docs.blender.org/api/5.3/bpy.types.RenderSettings.html
- "Any missing directories are created automatically during output." — manual, https://docs.blender.org/manual/en/5.3/compositing/types/output/file_output.html
- "When Disk Cache is checked, Blender will save the cache separately to the drive in a folder named blendcache_[filename] alongside the blend-file. (The blend-file must be saved first.)" and "Use Library Path — Share the disk cache when the physics object is linked into another blend-file." — manual, https://docs.blender.org/manual/en/5.3/physics/baking.html
- "Use this file’s path for the disk cache when library linked into another file (for local bakes per scene file, disable this option) (default True)" — API, https://docs.blender.org/api/5.3/bpy.types.PointCache.html
- "If you are not allowed to write to the required subdirectory caching will not take place" — manual, https://docs.blender.org/manual/en/5.3/physics/particles/emitter/cache.html
- "Directory to store baked simulation files in." — manual, https://docs.blender.org/manual/en/5.3/physics/fluid/type/domain/cache.html
- "Blend-files must be saved to a disk before data can be baked." — manual, https://docs.blender.org/manual/en/5.3/modeling/geometry_nodes/baking.html
- "EXTERNAL — External. Save the baking map in an external file." — API, https://docs.blender.org/api/5.3/bpy_types_enum_items/bake_save_mode_items.html
- "By default, all generated proxy videos are stored to the folder <path of original footage>/BL_proxy/<clip name>, but this can be changed to a custom directory using this option." — manual, https://docs.blender.org/manual/en/5.3/editors/video_sequencer/sequencer/sidebar/proxy.html
- "By default, files are stored in the blender_tx/ folder next to the image file. This can be changed with the Texture Cache file path preference." — manual, https://docs.blender.org/manual/en/5.3/render/cycles/render_settings/performance.html
- "Used to specify a .osl file from a drive, and this will then be automatically compiled into a .oso file in the same directory." — manual, https://docs.blender.org/manual/en/5.3/render/shader_nodes/utilities/script.html
- **Observed**, Cycles file `osl.py`, function `update_external_script`, lines 210–216 (read 2026-10-16): `# copy .oso from temporary path to .osl directory` / `dst_path = script_path_noext + ".oso"` / `shutil.copy2(oso_path, dst_path)` / `"Failed to write .oso file next to external .osl file at {:s}"`.
- "Unpacks all files in the same directory // as the blend-file, grouping them in proper folders (like ‘’textures’’ for instance)." — manual, https://docs.blender.org/manual/en/5.3/files/blend/packed_data.html; "Existing blend-files are overwritten." — manual, https://docs.blender.org/manual/en/5.3/interface/window_system/topbar.html
- "When saving a blend-file, Blender may also save modified external images depending on the Save Modified Images preference." — manual, https://docs.blender.org/manual/en/5.3/files/blend/open_save.html

**Not covered.** Where a Geometry Nodes bake lands when no custom path is set. Where the fluid cache lands when its folder is empty. Whether writes into a *library's* folder happen for any kind other than point caches with "Use Library Path". What happens when a write place is read-only, apart from point caches and OSL.

**Where I looked.** Render output, File Output node, physics baking, particle cache, fluid cache, Geometry Nodes bake pages, proxy page, Cycles performance page, Script node page, packed data page, open/save page and Save & Load preferences. The API for PointCache, NodesModifier(Bake), BakeSettings and RenderSettings. Cycles `osl.py` and `maketx.py`. Searched "write", "store", "save", "directory", "folder" and "alongside".

**Outside the interface.** *Brought up to date by the hand tests of 2026-10-17 — see* Observed *below. What follows is what the sources alone said.*
- The command line can set the output path (`-o`). The texture cache can be built from the command line ("`blender scene.blend --command maketx`", Cycles performance page). Cycles' `maketx.py` accepts `--cache-dir` ("Output directory for tx files, relative to source image or absolute.", **observed**, `maketx.py` lines 50–54).
- The OSL `.oso` write is in Cycles' Python code, which runs whenever the script node is compiled. Whether that happens in a background render is not stated.

**Operating system.** *Brought up to date by the hand tests of 2026-10-17 — see* Observed *below.* Not stated as differing. The particle cache page notes an operating-system limit on path length that can stop caching ("if your blend-file path is very long and your operating system has a limit on the path length that is supported"). It does not name which systems.

### Observed — hand tests on the Creator's Mac, 2026-10-17

**Label: observed.** Same machine, Blender and way of starting it as under question 1; `<W>` is the work folder.

**Entry 6 — where a templated write lands.** Recorded in full under question 5. A render output path `//templated/{blend_name}_frame_` in `out.blend` made the folder `templated` and wrote `out_frame_0001.png` and `out_frame_0002.png` into it. A geometry-nodes simulation baked with its modifier's bake folder set to `//gnbake/plain/` wrote three frames under `gnbake/plain/<bake id>/meta/` and `…/blobs/`; set to `//gnbake/{blend_name}/` in `baketmpl2.blend`, it wrote them under `gnbake/baketmpl2/<bake id>/`; set to `//gnbake/{blend_name}_{scene_name}/`, Blender printed `Info: Cannot determine bake location on disk. Falling back to packed bake.`, wrote no folder, and the operator still returned `{'FINISHED'}`.

**Entry 7 — where a render lands.** Cycles, 16 by 16, one sample, frames 1 to 2.

*A file never saved.* A factory-fresh Blender with no file open was started from the folder `<W>/e07/cwd`, its output path set from a script, and the animation rendered:

```
A. file never saved. bpy.data.filepath = ''
   working folder = <W>/e07/cwd
   factory default render.filepath = '/tmp/'
   set render.filepath = '//never_saved_out/frame_'
   bpy.path.abspath now  = 'never_saved_out/frame_'
   frame_path(frame=1)   = 'never_saved_out/frame_0001.png'
00:00.514  render           | Saved: 'never_saved_out/frame_0001.png'
00:00.555  render           | Saved: 'never_saved_out/frame_0002.png'
   render(animation=True) finished
   contents of the working folder: ['never_saved_out']
   inside never_saved_out: ['frame_0001.png', 'frame_0002.png']
```

In an unsaved file the `//` prefix resolves to nothing, leaving a plain relative path, and the frames land under the folder the terminal was started from.

*A folder that does not exist.* The file saved as `where.blend` with the output `//does_not_exist_yet/frame_`:

```
   folder exists before? False
00:00.435  render           | Saved: '<W>/e07/does_not_exist_yet/frame_0001.png'
00:00.474  render           | Saved: '<W>/e07/does_not_exist_yet/frame_0002.png'
   folder created? True   contents: ['frame_0001.png', 'frame_0002.png']
```

and from the command line, `--render-output <W>/e07/deep3/deep4/frame_ --render-frame 1`, two folders deep into folders that did not exist, exit status 0:

```
00:00.419  render           | Saved: '<W>/e07/deep3/deep4/frame_0001.png'
<W>/e07/deep3
<W>/e07/deep3/deep4
<W>/e07/deep3/deep4/frame_0001.png
```

Nothing was printed about making either folder.

*A File Output node built by script.* In a file saved as `fo_script.blend`, a compositor node group was made and set on the scene, a Render Layers node and a File Output node added, one output item added to the File Output node, the image linked to it, and its folder set:

```
D. File Output node own properties: ['file_output_items', 'active_item_index', 'directory', 'file_name', 'format', 'save_as_render', 'use_file_extension']
   Scene has node_tree? False | compositing_node_group = None
   new File Output node: items [] | inputs ['']
   after file_output_items.new: items ['MyPass'] | inputs ['MyPass', '']
   directory = '//fileoutput_new_folder/'  file_name = '{blend_name}'
   format: MULTI_LAYER_IMAGE OPEN_EXR_MULTILAYER
```

Setting and reading `Scene.compositing_node_group` printed, each time, `DeprecationWarning: 'Scene.compositing_node_group' is expected to be removed in Blender 6.0`. The file was saved and rendered in a separate run with `--render-anim`, exit status 0:

```
00:00.467  render           | Executing compositor
00:00.472  render           | Saved: '<W>/e07/fileoutput_new_folder/fo_script0001.exr'
00:00.534  render           | Saved: '<W>/e07/renders/frame_0001.png'
00:00.574  render           | Executing compositor
00:00.575  render           | Saved: '<W>/e07/fileoutput_new_folder/fo_script0002.exr'
00:00.576  render           | Saved: '<W>/e07/renders/frame_0002.png'
```

So the node made the folder that was not there, and named its files after the blend-file because its `file_name` holds `{blend_name}` from the start; the files are multi-layer OpenEXR. A File Output node made by script arrives with no output items and one unnamed input.

**Entry 8 — the compiled OSL file beside an external `.osl`.** A folder `writable` was made holding `simple_col.osl`, a shader with one colour output `Col` set to `color(0.2, 0.4, 0.8)`. In a file `osl_writable.blend` (Cycles on the processor, `scene.cycles.shading_system = True`, 16 by 16), a plane's material was given a Script node in External mode, and the folder was listed before and after the node's path was set:

```
folder before anything: ['simple_col.osl']
script node outputs before its path is set: [] | folder: ['simple_col.osl']
script node outputs after its path is set: ['Col'] | folder: ['simple_col.osl', 'simple_col.oso']
shader_script_update RAISED RuntimeError: Operator bpy.ops.node.shader_script_update.poll() failed, context is incorrect
removed simple_col.oso after the build, so the render starts without one
folder after build: ['simple_col.osl']
```

So setting the path, with no window, compiled the script, gave the node its `Col` output and wrote `simple_col.oso` beside the `.osl` at once; the update operator itself cannot be called with no window. `Col` was linked through an Emission shader to the material output, the file saved, and the `.oso` deleted. One frame was then rendered from the command line, exit status 0; nothing about the script, OSL or `.oso` was printed, and afterwards:

```
-rw-r--r--@ 1 artist     wheel   91 Oct 17 07:23 simple_col.osl
-rw-r--r--@ 1 artist     wheel  583 Oct 17 07:23 simple_col.oso
centre pixel (0.4824, 0.6667, 0.9059, 1.0)
```

So the render with no window compiled the external script again and wrote the `.oso` beside the `.osl`, and the rendered colour is the shader's own (its 0.2, 0.4, 0.8 shown through the Standard view transform). A first try of this entry added a second `Col` socket by hand before the compiled one had been seen, linked that one, and rendered white; it is kept in the run record of the list and carries nothing.

What happens when the `.osl` sits in a folder that cannot be written was not run: making the folder read-only needs `chmod 555`, which the tool this sitting ran in refused.

**Entry 15 — a File Output node added through the window's node editor.** A plain file `fo_window.blend` (Cycles, 16 by 16, one sample, frames 1 to 2, output `//renders/frame_`) was opened with its window from the terminal on the Mac and driven by timers in a script, after a first-run setup panel was closed with a simulated Esc key. The 3D viewport was switched to the Compositor node editor. That editor showed an empty canvas with a **New** button in its header and, along its foot, a shelf of effects headed *All*, *Camera & Lens Effects*, *Creative* and *Utilities*. Calling `bpy.ops.node.new_compositing_node_group()` in that editor returned `{'FINISHED'}` and made nothing — the scene's group, its compositor effects and the editor's tree all stayed empty; its own description in this build reads `Create a new compositing node group and initialize it with default nodes. Deprecated and will be removed in 6.0`. A simulated click on the header's **New** button then gave:

```
compositing_node_group after: <bpy_struct, CompositorNodeTree("Scene Compositor") at 0x8e6bd7308>
compositor_effects after: [('Scene Effect', 'Scene Compositor', True)]
node editor's tree after: <bpy_struct, CompositorNodeTree("Scene Compositor") at 0x8e6bd7308>
tree used: Scene Compositor | nodes: ['NodeGroupOutput', 'NodeGroupInput']
```

and the editor showed a path *Scene › Scene Effect › Scene Compositor* and two nodes, *Group Input* with an `Image` output linked to *Group Output*'s `Image` input. A File Output node was added in that editor with `bpy.ops.node.add_node(type="CompositorNodeOutputFile")`:

```
add_node(CompositorNodeOutputFile) -> {'FINISHED'}
nodes added: [('File Output', 'CompositorNodeOutputFile')]
File Output node: items [] | inputs [''] | directory '//renders/frame_' | file_name '{blend_name}' | format OPEN_EXR_MULTILAYER
```

So a File Output node added this way arrives with no output items, its folder filled in from the scene's render output path, and its file name the template `{blend_name}`. The rest was done by script, which says so in its own lines: a Render Layers node was added, one item added to the File Output node, the image linked to it, and its folder set to `//fileoutput_new_folder/`, a folder that did not exist. The animation was then rendered inside the window with `bpy.ops.render.render(animation=True)`:

```
00:06.617  render           | Saved: '<W>/e15/fileoutput_new_folder/fo_window0001.exr'
00:06.650  render           | Saved: '<W>/e15/renders/frame_0001.png'
00:06.689  render           | Saved: '<W>/e15/fileoutput_new_folder/fo_window0002.exr'
00:06.690  render           | Saved: '<W>/e15/renders/frame_0002.png'
render(animation=True) -> {'FINISHED'}
folder: True ['fo_window0001.exr', 'fo_window0002.exr']
```

The node made its folder and named its files after the blend-file, as it did from the command line under entry 7. Two earlier tries of this entry stopped at the empty group and are kept in the run record of the list.

*Not run:* what a render does when its output folder cannot be written. Making a folder read-only needs the terminal command `chmod 555`, and the tool this sitting ran in refused that command, so no read-only folder could be made — see the *Cannot be run* section of `Audits/Blender Research/Dependencies/Hand tests — 5.3.md`.

**Standing lines, as the hand tests leave them.** *Driven from outside:* entries 6, 7 and 8 were run with no window, every render started from a script or the command line; entry 15 was run inside the window and its File Output node wrote the same way the script-built one did with no window. Two things differ by route: the compositor's node group could be made by the editor's **New** button and not by the old operator, and an external OSL script's update operator cannot be called with no window, though setting the node's path compiles it anyway. *Operating system:* macOS only, on the machine named under question 1.

---

## 11. Can a reference point at a named thing inside another file, and what happens when that thing is renamed or is gone?

**Label:** documented, except where marked inferred.

**Answer.**
- **Yes.** Linking creates a reference to named data inside another `.blend` file. The full name is recorded, including a two-letter type prefix. Cache files (Alembic or USD) also point at a named object path inside the archive.
- **Renamed.** The manual gives a renamed data-block as an example of a *broken* link. Its fix is "Relocate Linked ID". So renaming in the source file breaks the link.
- **Gone.** A linked data-block that can no longer be found is replaced by a **placeholder**, flagged `is_missing`. When the whole library file cannot be found, Blender creates placeholders for the missing linked data, so that "references to the missing data are not lost". Relocating the library can bring the data back.
- From Python, a requested name that cannot be loaded comes back as `None`.
- With library overrides, most override data survives a lost reference. The noted exception is posed armatures, whose pose bones "are definitively lost".

**Evidence (2026-10-11):**
- "Link creates a reference to data in a source file such that changes made there will be reflected in the current file the next time it is reloaded." and "This can be used to either fix a broken linked data-block (e.g. because the ID was renamed), or to switch to a variation of the same data." and "If Blender cannot find a library while loading a blend-file, it will create placeholder data-blocks to replace missing linked ones. That way, references to the missing data are not lost, and by relocating the missing library, the lost data can be automatically restored." — manual, https://docs.blender.org/manual/en/5.3/files/linked_libraries/link_append.html
- "Full ID name in the library .blend file (including the two leading ‘id type’ chars)" — API, https://docs.blender.org/api/5.3/bpy.types.LibraryWeakReference.html
- "This data-block is a place-holder for missing linked data (i.e. it is [an override of] a linked data that could not be found anymore)" — API, https://docs.blender.org/api/5.3/bpy.types.ID.html
- "since loading the data replaces the strings for the data-blocks or None if the data-block could not be loaded." — API, https://docs.blender.org/api/5.3/bpy.types.BlendDataLibraries.html
- "The path to the Alembic or USD object inside the archive or stage." — manual, https://docs.blender.org/manual/en/5.3/modeling/modifiers/modify/mesh_sequence_cache.html
- "While in most cases library overrides data is preserved across a loss of reference linked data (if e.g. the library file becomes unavailable or is relocated), there are some exceptions." — manual, https://docs.blender.org/manual/en/5.3/files/linked_libraries/library_overrides.html

**Not covered.** What a placeholder looks like in a render. Whether any message is printed. What happens when an Alembic or USD object path no longer exists.

**Where I looked.** Link & Append page, library overrides page, mesh sequence cache page, the API for ID, Library, LibraryWeakReference, BlendDataLibraries and `bpy.ops.wm` (link, lib_relocate, id_linked_relocate). Searched "renamed", "broken", "placeholder", "missing" and "is_missing".

**Outside the interface.** *Brought up to date by the hand tests of 2026-10-17 — see* Observed *below. What follows is what the sources alone said.* Placeholders are made "while loading a blend-file", which happens however the file is opened. That is **inferred**. Relocating is shown as an Outliner action. `bpy.ops.wm.lib_relocate` and `bpy.ops.wm.id_linked_relocate` exist, but the latter is described as "Currently only designed as an internal operator, not directly exposed to the user".

**Operating system.** *Brought up to date by the hand tests of 2026-10-17 — see* Observed *below.* Not stated as differing.

### Observed — hand tests on the Creator's Mac, 2026-10-17

**Label: observed.** Same machine, Blender and way of starting it as under question 1; `<W>` is the work folder.

**Entry 20 — a linked collection when its library is gone, when it is renamed inside the library, and when it is removed.** `lib.blend` was saved holding a collection `PropCol` with one mesh object `Prop`. `master.blend` linked that collection with `bpy.ops.wm.link(..., instance_collections=True)`, which returned `{'FINISHED'}`, and was given a camera and a sun (Cycles, 32 by 32, one sample). Each case below opened `master.blend` with no window, printed everything Blender printed and then what the file held, called Report Missing Files, and rendered one frame from the command line.

*The control, with everything present:*

```
00:00.321  reports          | Read library: '<W>/e20/lib.blend', '//lib.blend', parent '<direct>'
libraries: [('lib.blend', '//lib.blend')]
   name='PropCol' library='//lib.blend' is_missing=False is_library_indirect=False objects=['Prop']
objects: [('Cam', None, False), ('PropCol', None, False), ('Sun', None, False), ('Prop', '//lib.blend', False)]
evaluated instances: 4
Info: No missing files
render EXIT 0
```

*(a) The whole library file renamed out of the way:*

```
00:00.321  reports          | Read library: '<W>/e20/lib.blend', '//lib.blend', parent '<direct>'
00:00.321  reports          | WARNING Unable to open '<W>/e20/lib.blend': No such file or directory
00:00.321  reports          | Cannot find lib '<W>/e20/lib.blend'
00:00.322  reports          | LIB: Collection: 'PropCol' missing from '<W>/e20/lib.blend', parent '<direct>'
00:00.322  reports          | WARNING 1 libraries and 1 linked data-blocks are missing (including 0 ObjectData), please check the Info and Outliner editors for details
libraries: [('lib.blend', '//lib.blend')]
   name='PropCol' library='//lib.blend' is_missing=True is_library_indirect=False objects=[]
objects: [('Cam', None, False), ('PropCol', None, False), ('Sun', None, False)]
evaluated instances: 3
Warning: Path '<W>/e20/lib.blend' not found, from local data-block 'LIlib.blend'
render EXIT 0
00:00.352  render           | Saved: '<W>/e20/out/a_0001.png'
```

*(b) The library put back, and the collection inside it renamed `PropCol_renamed` and saved:*

```
00:00.324  reports          | Read library: '<W>/e20/lib.blend', '//lib.blend', parent '<direct>'
00:00.327  reports          | LIB: Collection: 'PropCol' missing from '<W>/e20/lib.blend', parent '<direct>'
00:00.327  reports          | WARNING 0 libraries and 1 linked data-blocks are missing (including 0 ObjectData), please check the Info and Outliner editors for details
libraries: [('lib.blend', '//lib.blend')]
   name='PropCol' library='//lib.blend' is_missing=True is_library_indirect=False objects=[]
objects: [('Cam', None, False), ('PropCol', None, False), ('Sun', None, False)]
evaluated instances: 3
Info: No missing files
render EXIT 0
00:00.357  render           | Saved: '<W>/e20/out/b_0001.png'
```

*(c) The name put back, and the collection then removed inside the library with `bpy.data.collections.remove(...)` and saved:* the output was the same as (b), line for line apart from the times and the frame's name (`c_0001.png`).

So a renamed item and a removed one were reported in the same words, and neither the load messages nor Report Missing Files told them apart; Report Missing Files said `Info: No missing files` in both, because the library file itself was there. When the whole file was gone the count read `1 libraries` and Report Missing Files named the library; when only the item was gone it read `0 libraries`. In all three cases the file was left holding a placeholder with the name and the library and no objects, the linked object dropped out, evaluated instances fell from 4 to 3, and the render finished with exit status 0. Every message was printed to the terminal, including the one asking the reader to check the Info and Outliner editors.

**Entry 40 — a named object inside an Alembic file that is not there.** Recorded in full under question 12. With a Mesh Sequence Cache modifier's object path set to `/NoSuchObject` in an Alembic file that exists, a render from the command line exited 0, printed `Alembic: unknown how to handle objects of schema '', skipping object ''` and `object.modifier  | WARNING Object: "AbcCube", Modifier: "MeshSequenceCache", Could not create cache reader for file //assets/cube.abc` (twice each), and drew the object's own mesh in place of the cached one — the same picture as with the whole file gone.

**Entry 23 — the repair offered for a linked item.** Recorded in full under question 13. In the window's Outliner, the *Data-block* menu for a directly linked collection and for one that arrived behind it held the same eleven entries, ending in *Relocate*, drawn at full brightness for both.

**Standing lines, as the hand tests leave them.** *Driven from outside:* entries 20 and 40 were read with no window, and every render started from the command line; entry 23 is the window's view, where the Outliner's menu offers *Relocate* for a linked item. *Operating system:* macOS only, on the machine named under question 1.

---

## 12. What does a render do when something the file points at is not there, and does it say so anywhere?

**Label:** documented or observed where quoted, except where marked inferred. **Not in the sources** for most kinds and for any render-wide outcome.

**Answer.**
- **Linked library missing.** Placeholders stand in (see question 11).
- **Image missing (Cycles).**
  - Cycles' own files define a magenta colour "to use when images are not found". It is returned directly by the sky texture and by 3D (volume) image lookups. In the general image lookup, a texture-cache tile that failed to load returns the "missing" colour passed in. A tile not yet loaded returns the image's average colour.
  - The Image Texture node takes its "missing" colour from values handed to it. What those values are is set by code not in my sources, so the colour a missing image texture renders with in Cycles is **not settled** by what I could read.
- **Sequencer.** A file setting decides this, and is off by default. When on, missing images or movies render solid magenta. When off, they render fully transparent. Missing-media strips show red in the timeline. In 5.3 they also show their type icon. Image strips set to use placeholders render missing frames pink in the preview.
- **Volumes.** If loading grids fails, an error message is kept on the volume.
- **Colour configuration.** The file records whether the config failed to load or seemed missing (see question 15).
- **Does it say so?**
  - "Report Missing Files" puts a warning in the Info editor header, and "If no warning is shown, there are no missing external files."
  - The Info editor logs "errors, warnings, and informational messages".
  - On the command line, `-q` still prints "warnings & errors", and `--log` lets you choose categories.
  - Nothing says a render stops or fails because of a missing input file.
- **Fonts, sounds, caches, Alembic/USD, IES and OSL files.** Missing-file behaviour is not in the sources.

**Evidence:**
- **Observed**, Cycles file `source/util/types_image.h`, lines 17–19 (read 2026-10-16): `/* Color to use when images are not found. */` / `#define IMAGE_MISSING_RGBA make_float4(1, 0, 1, 1)` / `#define IMAGE_MISSING_RGB make_float3(1, 0, 1)`
- **Observed**, Cycles file `source/kernel/util/image.h`, function `kernel_image_interp`, line 361: `return (tile_descriptor == KERNEL_TILE_LOAD_FAILED) ? missing_rgba : tex.average_color;`. Line 180 of `source/kernel/svm/sky.h` passes `IMAGE_MISSING_RGBA`. Line 259 of `source/kernel/util/image_3d.h` returns `IMAGE_MISSING_RGBA`. In `source/kernel/svm/image.h` lines 69–75, the image node uses `const float3 missing = stack_load(stack, node.missing);`. Its value comes from outside the files I have.
- "Render missing images/movies with a solid magenta color (default False)" — API, https://docs.blender.org/api/5.3/bpy.types.SequenceEditor.html
- "When disabled, missing content will render fully transparent." and "Strips with missing content will be displayed as red in the timeline." — manual, https://docs.blender.org/manual/en/5.3/editors/video_sequencer/preview/sidebar.html
- "Placeholder frames are rendered in pink in the VSE preview." — manual, https://docs.blender.org/manual/en/5.3/video_editing/edit/montage/strips/image.html
- Release note: "Strips with missing/broken media now display the strip's type icon in the "thumbnail" area." — https://developer.blender.org/docs/release_notes/5.3/sequencer/
- "If loading grids failed, error message with details" — API, https://docs.blender.org/api/5.3/bpy.types.VolumeGrids.html
- "After selecting this option, a warning message will appear in the Info editor’s header. If no warning is shown, there are no missing external files." — manual, https://docs.blender.org/manual/en/5.3/interface/window_system/topbar.html
- "The Info editor logs the executed operators as well as errors, warnings, and informational messages." — manual, https://docs.blender.org/manual/en/5.3/editors/info_editor.html
- "-q, --quiet — Suppress status printing (warnings & errors are still printed)." — manual, https://docs.blender.org/manual/en/5.3/advanced/command_line/arguments.html

**Not covered.** Whether a render with missing inputs finishes, fails, or returns a non-zero exit code. Whether Report Missing Files runs by itself during a render. What colour a missing image renders with in Cycles (see above) and in EEVEE. Missing fonts, sounds, caches, Alembic/USD, IES and OSL files.

**Where I looked.** Searched the manual and API for "missing", "not found", "cannot be found", "could not be found", "failed to load", "magenta", "pink", "placeholder", "broken" and "error message". Searched Cycles' files for "missing", "MISSING" and "not found". Read the topbar, Info editor, sequencer preview sidebar, image strip, Image Texture node, link & append and troubleshooting pages.

**Outside the interface.** *Brought up to date by the hand tests of 2026-10-17 — see* Observed *below. What follows is what the sources alone said.*
- The Info editor and the timeline colours are interface only.
- Report Missing Files is an operator (`bpy.ops.file.report_missing_files`) and can be called from a script. Where its report goes in a background run is not stated.
- The sequencer setting and the Cycles colour are part of the render itself. That they apply the same way in a background render is **inferred**.

**Operating system.** *Brought up to date by the hand tests of 2026-10-17 — see* Observed *below.* Not stated as differing.

### Observed — hand tests on the Creator's Mac, 2026-10-17

**Label: observed.** Same machine, Blender and way of starting it as under question 1; `<W>` is the work folder.

**Entry 2 — a partly packed file whose files are gone.** Set up and recorded in full under question 3. In short: opening the file with no window printed nothing but Blender's own `Read blend` line; `bpy.ops.file.report_missing_files()` printed `Warning: Path '<W>/e02/assets/loose.png' not found, from local data-block 'IMloose.png'`, returned `{'FINISHED'}`, and left `WindowManager.reports` at length 0; and a Cycles render from the command line finished with exit status 0, wrote its frame, and printed `cycles           | WARNING Image file <W>/e02/assets/loose.png does not exist.` and `cycles           | ERROR Failed to load 1 image files`.

**Entry 3 — a volume whose file is not there.** A Volume data-block with the path `//volumes/smoke.vdb`, no such file, in a saved file `multi.blend`, read with no window:

```
   grids.is_loaded before load = False
   grids.load() returned       = False
   grids.is_loaded after load  = False
   grids.error_message         = 'smoke.vdb not found'
   grids.frame_filepath        = ''
```

Nothing was printed to the terminal about it.

**Entry 39 — the same partly packed file through the window.** Recorded in full under question 3. With both PNG files gone, the viewport drew the unpacked image's plane bright magenta and the packed one's in its own blue, and the Image editor drew an empty checkerboard for the unpacked image. Report Missing Files called from a script inside the window printed its warning to the terminal and put nothing in the status bar.

**Entry 37 — a missing image texture, in Cycles and in EEVEE.** A plane whose Base Color comes from an Image Texture node with a solid green 32 by 32 `assets/green.png`, a camera, a sun, 32 by 32, Standard view transform, one sample, saved as `missimg.blend`. It was rendered from the command line with `--engine CYCLES` and with `--engine BLENDER_EEVEE` (the only EEVEE name this build offers), first with the texture there and then with it moved to another folder. Every render wrote its frame and exited with status 0. With the texture there, neither engine printed anything beyond its ordinary progress lines. With it gone, Cycles printed:

```
00:00.352  cycles           | WARNING Image file <W>/e37/assets/green.png does not exist.
00:00.353  cycles           | ERROR Failed to load 1 image files
```

and EEVEE printed:

```
00:00.446  image.gpu        | ERROR Image file "<W>/e37/assets/green.png" does not exist
```

The centre pixel of each frame, read afterwards in a fresh Blender:

```
centre pixel present CYCLES (0.7412, 1.0, 0.7412, 1.0)
centre pixel present BLENDER_EEVEE (0.7333, 1.0, 0.7333, 1.0)
centre pixel missing CYCLES (1.0, 0.7412, 1.0, 1.0)
centre pixel missing BLENDER_EEVEE (1.0, 0.7333, 1.0, 1.0)
```

So with the texture missing the plane came out magenta in both engines — the green and red channels swapped places — and the render was not stopped.

**Entry 38 — the sequencer's missing-media switch.** In a new file with no window, a sequence editor was made with `scene.sequence_editor_create()` and the switch read, then its declared default and description read off the property:

```
show_missing_media on a new sequence editor = True
declared default = False
description = 'Render missing images/movies with a solid magenta color'
```

So the switch comes on in a sequence editor made this way, though the property says its default is off.

**Entry 35 — a missing font.** Recorded in full under question 16. A text object whose font file was gone rendered with exit status 0, with the letters lost (one rectangle each); the file printed `reports          | ERROR Unable to pack file, source path '<W>/e35/assets/testfont.ttf' not found` twice and `geom.vfont       | WARNING Font file doesn't exist: //assets/testfont.ttf` once, in two groups, on a plain open and again while rendering, and Report Missing Files named it: `Warning: Path '<W>/e35/assets/testfont.ttf' not found, from local data-block 'VFArial Regular'`.

**Entry 40 — other kinds missing at render: an Alembic file, an object path inside it, an IES file, an OSL file and a sound.** One file `kinds.blend` was built with no window: a cube exported by Blender to `assets/cube.abc` and imported back with a Mesh Sequence Cache modifier reading `//assets/cube.abc` at object path `/AbcCube/AbcCubeMesh` — and, so that the picture would show whether the cache was read, the object's own mesh then replaced with a much smaller cube (read back with no window: width `1.500` with the file there, `0.300` with it moved away); a plane coloured by an OSL Script node in External mode reading `//assets/shader.osl` (setting the path compiled it, writing `shader.oso` beside it — see question 10); a point light shaped by an IES Texture node reading `//assets/light.ies`, a narrow beam; and a sound strip in the sequencer reading `//assets/beep.wav`. Cycles on the processor with OSL on, 64 by 64, four samples. One frame was rendered from the command line with everything there, and then once for each case below, the file being put back each time. Every render wrote its frame and exited with status 0. Lines printed apart from the ordinary progress lines, and each picture against the full one:

```
=== full render EXIT 0 | frames saved: 1
=== abc_gone render EXIT 0 | frames saved: 1
Could not open as Ogawa file from provided streams.
Unable to open <W>/e40c/assets/cube.abc
00:00.307  object.modifier  | WARNING Object: "AbcCube", Modifier: "MeshSequenceCache", Could not create cache reader for file //assets/cube.abc
(the same three lines again)
=== objpath_gone render EXIT 0 | frames saved: 1
Alembic: unknown how to handle objects of schema '', skipping object ''
00:00.320  object.modifier  | WARNING Object: "AbcCube", Modifier: "MeshSequenceCache", Could not create cache reader for file //assets/cube.abc
(the same two lines again)
=== ies_gone render EXIT 0 | frames saved: 1
00:00.333  cycles           | ERROR Could not read IES file from <W>/e40c/assets/light.ies
=== osl_gone_oso_left render EXIT 0 | frames saved: 1
=== osl_and_oso_gone render EXIT 0 | frames saved: 1
ERROR: error: Input file "<W>/e40c/assets/shader.osl" not found
00:00.324  cycles           | ERROR Shader graph: failed to read file <W>/e40c/assets/shader.oso
=== wav_gone render EXIT 0 | frames saved: 1
abc_gone           against full: PICTURES size=(64, 64) largest channel difference=0.756863 channels differing=1824 of 16384 — DIFFERENT PICTURE
objpath_gone       against full: PICTURES size=(64, 64) largest channel difference=0.756863 channels differing=1824 of 16384 — DIFFERENT PICTURE
ies_gone           against full: PICTURES size=(64, 64) largest channel difference=0.074510 channels differing=3 of 16384 — DIFFERENT PICTURE
osl_gone_oso_left  against full: PICTURES size=(64, 64) largest channel difference=0.000000 channels differing=0 of 16384 — SAME PICTURE
osl_and_oso_gone   against full: PICTURES size=(64, 64) largest channel difference=0.517647 channels differing=3843 of 16384 — DIFFERENT PICTURE
wav_gone           against full: PICTURES size=(64, 64) largest channel difference=0.000000 channels differing=0 of 16384 — SAME PICTURE
```

(`abc_gone`: `cube.abc` moved away. `objpath_gone`: the file there, the modifier's object path set to `/NoSuchObject` and saved as `kinds_objpath.blend`. `ies_gone`: `light.ies` moved away. `osl_gone_oso_left`: `shader.osl` moved away, the compiled `shader.oso` left beside where it was. `osl_and_oso_gone`: both moved away. `wav_gone`: `beep.wav` moved away.)

So a missing Alembic file and a missing object path inside it gave the same picture as each other — the object's own stand-in mesh — and were named on the terminal, the object path case without naming the path. A missing IES file was named and changed the picture slightly. A missing `.osl` with its `.oso` still beside it changed nothing and printed nothing; with both gone the script was named and the picture changed. A missing sound printed nothing; this render writes no sound, so the picture could not have shown it. Before the cache case was rebuilt with a different stand-in mesh, a first build gave the same picture with the Alembic file gone as with it there, which said nothing about the cache and is kept only in the list's run record.

**Entry 21 — a missing library that arrived indirectly.** Recorded in full under question 13. A render of `top.blend`, whose library `mid.blend` links from a `base.blend` that had been moved away, finished with exit status 0 after printing `WARNING Unable to open '<W>/e21/base.blend': No such file or directory`, `Cannot find lib '<W>/e21/base.blend'`, `LIB: Collection: 'BaseCol' missing from '<W>/e21/base.blend', parent '<direct>'` and `WARNING 1 libraries and 1 linked data-blocks are missing …`.

**Entry 20 — a missing linked library, and a missing item inside a library.** Recorded in full under question 11. A file linking a collection from `lib.blend` rendered a frame from the command line with exit status 0 in every case: with `lib.blend` gone the load printed `WARNING Unable to open '<W>/e20/lib.blend': No such file or directory`, `Cannot find lib '<W>/e20/lib.blend'`, `LIB: Collection: 'PropCol' missing from '<W>/e20/lib.blend', parent '<direct>'` and `WARNING 1 libraries and 1 linked data-blocks are missing (including 0 ObjectData), please check the Info and Outliner editors for details`; with the collection renamed or removed inside the library the load printed the `LIB:` line and `WARNING 0 libraries and 1 linked data-blocks are missing …`, and Report Missing Files said `Info: No missing files`.

**Standing lines, as the hand tests leave them.** *Driven from outside:* with no window, every render above with something missing — an image (both engines), a font, a linked library, an item inside a library, an indirect library, an Alembic file or object path, an IES file, an OSL file, a sound — finished, wrote its frame and exited with status 0; what each printed is quoted above, and a volume, a sound and an `.osl` with its `.oso` still beside it printed nothing. Report Missing Files works with no window and prints to the terminal; inside the window, called from a script, it printed to the terminal and showed nothing in the status bar (entry 39). *Operating system:* macOS only, on the machine named under question 1. Linux and Windows were not run in this sitting.

---

## 13. Does a file pointing at another `.blend` file bring that file's own references with it, how deep does that go, and can they be reached the same way?

**Label:** documented that indirect references come along. **Not in the sources** for any depth limit. How they are reached is partly documented and partly **inferred**.

**Answer.**
- **Yes.** A library's own libraries come with it. They are "linked indirectly". When reading a library, Blender lists "all other libraries used by the loaded one". Their paths "may be absolute, or relative to the loaded blend-file", not to your file.
- Image and other paths inside linked data are likewise relative to the library.
- Appending can either keep these indirect links or copy them too ("Localize All").
- **How deep.** No limit is stated. The 5.3 notes show a four-step chain as a normal production case (`assets.blend -> stage.blend -> shot_anim.blend -> shot_lighting.blend`). The manual warns that links going back into the current file "will likely result in missing links".
- **Reached the same way?**
  - Partly. Relocate Linked ID works only on directly linked data, "not a dependency of any other linked data".
  - Blender's path tools visit the paths of linked data unless told to skip them ("Skip paths of linked IDs"). That they are therefore included by default is **inferred**. The library-path listing has its own on/off switch.
  - Each library has a `parent` entry, with no description. That it names the library that brought it in is **inferred**.
  - Linked data is not editable. Changing a library path by hand "can leave Blender data in an invalid state".
- **Point caches.** "Use Library Path" makes a linked object use the library's own cache folder.
- **5.3 change.** A packed linked item now counts as direct if a directly used packed item uses it. Only direct packed items are written.

**Evidence (2026-10-11):**
- "A libraries attribute, which for the input is a list of all other libraries used by the loaded one, as named tuples (filepath, is_archive). The filepath may be absolute, or relative to the loaded blend-file." — API, https://docs.blender.org/api/5.3/bpy.types.BlendDataLibraries.html
- "Is this ID block linked indirectly" — API, https://docs.blender.org/api/5.3/bpy.types.ID.html
- "the path will be relative to the library rather than the open blend-file." — API, https://docs.blender.org/api/5.3/info_gotchas_file_paths_and_encoding.html
- "Localize All — Also copy all indirectly linked data, instead of maintaining the links." and "This operation is only available on data-blocks that are directly linked, and are not a dependency of any other linked data." and "Attempting to link or append data which links back to the current file will likely result in missing links." — manual, https://docs.blender.org/manual/en/5.3/files/linked_libraries/link_append.html
- "Skip paths of linked IDs." — API, https://docs.blender.org/api/5.3/bpy_types_enum_items/file_path_foreach_flag_items.html; "include_libraries (bool) – Include library file paths of linked data. False by default." — API, https://docs.blender.org/api/5.3/bpy.types.BlendData.html
- Library `parent` "(readonly) Type: Library | None" and filepath "WARNING: Typically, this value should be considered read-only, changing this path without proper processing afterwards can leave Blender data in an invalid state" — API, https://docs.blender.org/api/5.3/bpy.types.Library.html
- "blend_dir_lib: Like blend_dir, except that for path properties on library-linked data-blocks it will be the path to the library blend file that the data-block comes from." — manual, https://docs.blender.org/manual/en/5.3/files/file_paths.html
- Release note: "This is especially useful in pipelines stacking several layers of linking in production files (e.g. `assets.blend -> stage.blend -> shot_anim.blend -> shot_lighting.blend`)." and "code writing linked data now only writes directly linked packed IDs" — https://developer.blender.org/docs/release_notes/5.3/core/

**Not covered.** A maximum depth. Whether Find Missing Files, Make Paths Relative or Absolute, or Report Missing Files reach paths inside indirect libraries. Whether a missing indirect library is placeholdered like a direct one. Whether the Outliner lists indirect libraries. I opened the manual's Outliner "Blender File" picture (`_images/files_data-blocks_outliner-blender-file-view.png`). It shows only "Current File" categories and no libraries, so it does not settle this.

**Where I looked.** Link & Append, file paths, library overrides, the Outliner pages and picture, the gotchas page, the API for Library, ID, BlendData, BlendDataLibraries, `bpy.ops.wm` and the path flags, and the 5.3 core notes. Searched "indirect", "recursive", "nested", "parent", "depth" and "dependency".

**Outside the interface.** *Brought up to date by the hand tests of 2026-10-17 — see* Observed *below. What follows is what the sources alone said.* The listing and path tools are Python and work in background runs (**inferred**). Relocate Linked ID is an Outliner action.

**Operating system.** *Brought up to date by the hand tests of 2026-10-17 — see* Observed *below.* Not stated as differing.

### Observed — hand tests on the Creator's Mac, 2026-10-17

**Label: observed.** Same machine, Blender and way of starting it as under question 1; `<W>` is the work folder.

**Entry 21 — a missing library that arrived indirectly.** Three files in one folder, built with no window: `base.blend` holding a collection `BaseCol` with a mesh object `BaseObj`; `mid.blend` linking `BaseCol` (relative) and holding it inside a local collection `MidCol`; `top.blend` linking `MidCol` (relative) into its scene, with a camera and a sun. At the end of the build, in the same run, `top.blend` listed its libraries as `[('base.blend', '//base.blend', 'mid.blend'), ('mid.blend', '//mid.blend', None)]` — name, recorded text, parent. `top.blend` was then opened in a fresh run with no window.

*With `base.blend` in place:*

```
00:00.305  reports          | Read library: '<W>/e21/mid.blend', '//mid.blend', parent '<direct>'
00:00.308  reports          | Read library: '<W>/e21/base.blend', '//base.blend', parent '<direct>'
   library name='base.blend' filepath='//base.blend' parent=None
   library name='mid.blend' filepath='//mid.blend' parent=None
   collection 'BaseCol' library='base.blend' is_missing=False is_library_indirect=True objects=['BaseObj']
   collection 'MidCol' library='mid.blend' is_missing=False is_library_indirect=False objects=[]
   object 'BaseObj' library='base.blend' is_missing=False is_library_indirect=True
Info: No missing files
blend_paths(): ['//base.blend', '//mid.blend']
```

*With `base.blend` moved to another folder:*

```
00:00.320  reports          | Read library: '<W>/e21/mid.blend', '//mid.blend', parent '<direct>'
00:00.323  reports          | Read library: '<W>/e21/base.blend', '//base.blend', parent '<direct>'
00:00.323  reports          | WARNING Unable to open '<W>/e21/base.blend': No such file or directory
00:00.323  reports          | Cannot find lib '<W>/e21/base.blend'
00:00.323  reports          | LIB: Collection: 'BaseCol' missing from '<W>/e21/base.blend', parent '<direct>'
00:00.324  reports          | WARNING 1 libraries and 1 linked data-blocks are missing (including 0 ObjectData), please check the Info and Outliner editors for details
   library name='base.blend' filepath='//base.blend' parent=None
   library name='mid.blend' filepath='//mid.blend' parent=None
   collection 'BaseCol' library='base.blend' is_missing=True is_library_indirect=True objects=[]
   collection 'MidCol' library='mid.blend' is_missing=False is_library_indirect=False objects=[]
Warning: Path '<W>/e21/base.blend' not found, from local data-block 'LIbase.blend'
report_missing_files -> {'FINISHED'}
blend_paths(): ['//base.blend', '//mid.blend']
```

and a render of one frame from the command line printed the same load messages and finished with exit status 0, writing its frame. So the missing indirect library got a placeholder — `BaseCol` with `is_missing=True` and `is_library_indirect=True`, holding nothing — the load named the missing file and counted it, and Report Missing Files named it too, as a data-block of the top file (`LIbase.blend`). In the reopened file both libraries read `parent=None`, and the load lines call each one's parent `'<direct>'`, though in the run that built the file `base.blend`'s parent read `mid.blend`.

**Entry 22 — how deep a chain of linked files goes, built two ways.** `L1.blend` holds a plane `Prop` whose emission colour is an image `//tex/img.png` beside it (32 by 32, colour `(1.0, 0.6275, 0.1373)`), in a collection `C1`. Each next file `Ln.blend`, in a folder of its own, links `C(n-1)` from the one below with the relative option on and puts it into a collection `Cn`, in one of two ways. *Instanced:* an empty `In` instances `C(n-1)` and sits in `Cn`. *Child:* `C(n-1)` is made a child collection of `Cn`, with no instancing. The top file also has a camera, a sun, 32 by 32 and one Cycles sample. Chains of 7, 8, 9 and 20 files were built in each way, each top file opened with no window and read, one frame rendered from the command line, and the frame's centre pixel read:

```
=== inst files=7 render EXIT 0 | centre pixel: (1.0, 0.6275, 0.1373, 1.0) | warnings on load: 0
=== inst files=8 render EXIT 0 | centre pixel: (1.0, 0.6275, 0.1373, 1.0) | warnings on load: 2
=== inst files=9 render EXIT 0 | centre pixel: (0.0, 0.0, 0.0, 1.0) | warnings on load: 2
=== inst files=20 render EXIT 0 | centre pixel: (0.0, 0.0, 0.0, 1.0) | warnings on load: 2
=== child files=7 render EXIT 0 | centre pixel: (1.0, 0.6275, 0.1373, 1.0) | warnings on load: 0
=== child files=8 render EXIT 0 | centre pixel: (1.0, 0.6275, 0.1373, 1.0) | warnings on load: 0
=== child files=9 render EXIT 0 | centre pixel: (1.0, 0.6275, 0.1373, 1.0) | warnings on load: 0
=== child files=20 render EXIT 0 | centre pixel: (1.0, 0.6275, 0.1373, 1.0) | warnings on load: 0
```

`(1.0, 0.6275, 0.1373, 1.0)` is the texture's own colour; `(0.0, 0.0, 0.0, 1.0)` an empty frame. So built with instances, the object stopped reaching the render at nine files and the frame was empty, with exit status 0; built with child collections, it reached the render at twenty files.

The warnings, in the instanced chains only, each printed twice on reading:

```
inst8:  Warning: Maximum instance recursion level reached in "Prop" object.
inst9:  Warning: Maximum instance recursion level reached in "I2" object.
inst20: Warning: Maximum instance recursion level reached in "I13" object.
```

and the nine-file render printed `Warning: Maximum instance recursion level reached in "I2" object.` Evaluated instances read 10 for every instanced chain from eight files up, and 3 for every child chain.

*How the chain is recorded.* At eight files, instanced, the load printed one line per level and the parent chain was complete:

```
00:00.317  reports          | Read library: '<W>/e22/inst8/L7/L7.blend', '//../L7/L7.blend', parent '<direct>'
00:00.319  reports          | Read library: '<W>/e22/inst8/L6/L6.blend', '//../L6/L6.blend', parent '<W>/e22/inst8/L7/L7.blend'
… down to …
00:00.333  reports          | Read library: '<W>/e22/inst8/L1/L1.blend', '//../L1/L1.blend', parent '<W>/e22/inst8/L2/L2.blend'
libraries (7):
   name='L1.blend' filepath='//../L1/L1.blend'     parent='//../L2/L2.blend'
   … each one's parent the next file up …
   name='L7.blend' filepath='//../L7/L7.blend'     parent=None
objects below the top: 7, all is_library_indirect: True, direct ones: []
```

At twenty files, instanced, nineteen libraries were recorded with the same complete chain. Built as child collections, every load line said `parent '<direct>'` and every library read `parent=None` — at eight files seven libraries, at twenty files nineteen — with the one object below the top marked `is_library_indirect: True`.

*Where a path inside a library is measured from*, the same in every chain:

```
   image 'img.png' stored='//tex/img.png' library='//../L1/L1.blend'
      abspath WITHOUT library arg: '<W>/e22/inst8/L8/tex/img.png'
      abspath WITH    library arg: '<W>/e22/inst8/L8/../L1/tex/img.png'
      size=(32, 32)
```

Only the second names a real file, and the image loaded at 32 by 32 in every chain, including the ones whose render came out empty.

**Entry 23 — through the window: the Outliner's Blender File view, and the menu for a direct and an indirect item.** Three files: `deep.blend` holding a collection `DeepCollection` with a cube `DeepObject`; `middle.blend` linking `DeepCollection` (relative) as a child of its own collection `MiddleCollection`; `window.blend`, the factory scene with `MiddleCollection` linked (relative) into it. Read with no window at the end of the build: `('DeepCollection', 'deep.blend', True)` and `('MiddleCollection', 'middle.blend', False)` — name, library, `is_library_indirect`. `window.blend` was opened with its window from the terminal on the Mac and driven by timers in a script; the 3D viewport was switched to an Outliner in Blender File display mode and opened out level by level with `bpy.ops.outliner.show_one_level()`; pictures were taken with Blender's own screenshot command.

*What the Outliner lists.* Unfiltered, the visible part of the tree showed *Current File* with a *Libraries* category holding `//deep.blend` and `//middle.blend`, then the file's own data-blocks by kind; the rest of the tree ran below the visible part and was not photographed. With the Outliner's filter set to show only collections, the whole tree fitted: *Current File* with the local `Collection`; then `//deep.blend` with nothing under it; then `//middle.blend` holding `MiddleCollection`, holding `DeepCollection`, holding `DeepObject` and its mesh `DeepMesh`, each with a link mark.

*What the menu offers.* Each collection's row was chosen with a simulated left click, which highlighted it. A simulated right click did not open a menu in any of three tries, so the menus were opened instead with `bpy.ops.wm.call_menu(name=…)`, using the menu names the running Blender lists (`OUTLINER_MT_collection` and `OUTLINER_MT_id_data` among them). The collection menu holds an entry *Data-block* with an arrow, in the place of the *ID Data* entry earlier versions' records name. The *Data-block* menu, opened for each collection, held the same entries for both, in this order:

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

*Relocate* is drawn at the same brightness as the entries above it for both the directly linked `MiddleCollection` and the indirectly linked `DeepCollection`, where in the collection menu *Mark as Asset*, *Clear Asset* and *Clear Asset (Set Fake User)* are drawn greyed. The operator behind it said, in the Outliner's context, `id_linked_relocate.poll() = True` with each collection chosen — and `True` with nothing chosen at all, so its answer does not tell the items apart. Whether *Relocate* does anything when used on the indirect item was not tried. Every menu picture was taken one step late in some runs (a menu opened by one call appeared in the next picture), so only pictures in which the chosen row and the menu could both be matched are reported here; the runs and their pictures are kept in the list's run record.

**Standing lines, as the hand tests leave them.** *Driven from outside:* entries 21 and 22 were run with no window, every render from the command line; entry 23 is the window's view of the same kind of chain. *Operating system:* macOS only, on the machine named under question 1.

---

## 14. Does rendering one scene of a file pull only that scene's references?

**Label:** **not in the sources** as asked. Related facts are documented. The conclusions drawn from them are **inferred**.

**Answer.**
- No source says whether Blender loads only what the rendered scene uses, or everything the file points at.
- Several documented features make one scene's render use *other* scenes, and so their references (**inferred**):
  - background scenes, which are included recursively
  - the compositor's Render Layers node, which can render a view layer of any scene
  - scenes used inside compositor node groups, which 5.3 newly renders
  - sequencer scene strips
- Libraries are read "while loading a blend-file". That this happens for the whole file, whatever scene is rendered, is **inferred**.
- A compressed library is loaded "in their entirety" even if little is used.
- A background-mode option exists to *not* build every view layer's dependency graph on load. That by default they are built on load is **inferred**, but the option does not say whether this covers all scenes or only the active one.

**Evidence (2026-10-11):**
- "Background Scenes can themselves have a Background Scene (they’re recursively included)." — manual, https://docs.blender.org/manual/en/5.3/scene_layout/scene/properties.html
- "The Render Layers node renders a View Layer from a scene" and "Scene — The scene for which to render a view layer." — manual, https://docs.blender.org/manual/en/5.3/compositing/types/input/scene/render_layers.html
- Release note: "The compositor now renders scenes referenced inside node groups." — https://developer.blender.org/docs/release_notes/5.3/compositor/
- "Scene strips are a way to insert the render output of another scene into your sequence." — manual, https://docs.blender.org/manual/en/5.3/video_editing/edit/montage/strips/scene.html
- "If Blender cannot find a library while loading a blend-file, it will create placeholder data-blocks" and "Referencing compressed blend-files may need a lot of memory because they have to be loaded in their entirety, even if you only link/append a small part of them." — manual, https://docs.blender.org/manual/en/5.3/files/linked_libraries/link_append.html
- "--disable-depsgraph-on-file-load — Background mode: Do not systematically build and evaluate ViewLayers’ dependency graphs when loading a blend-file in background mode (-b or -c options)." and "-S, --scene <name> — Set the active scene <name> for rendering." — manual, https://docs.blender.org/manual/en/5.3/advanced/command_line/arguments.html
- "True if the image data is loaded into memory" — API, https://docs.blender.org/api/5.3/bpy.types.Image.html (this suggests images can be present without their pixels loaded; when they load is not stated)

**Not covered.** Whether images, caches, volumes or fonts used only by other scenes are opened when one scene is rendered. Whether a missing file used only by another scene affects the render.

**Where I looked.** Searched for "on demand", "lazily", "loaded when", "only load", "entire", "entirety", "background scene", "scene strip", "Render Layers" and "depsgraph". Read the scene properties, Link & Append, compositor Render Layers, sequencer scene strip and command-line pages, and the 5.3 compositor notes.

**Outside the interface.** *Brought up to date by the hand tests of 2026-10-17 — see* Observed *below. What follows is what the sources alone said.* The command line picks the scene with `-S` and has the dependency-graph option above, which applies only in background mode. Beyond that, not stated.

**Operating system.** *Brought up to date by the hand tests of 2026-10-17 — see* Observed *below.* Not stated.

### Observed — hand tests on the Creator's Mac, 2026-10-17

**Label: observed.** Same machine, Blender and way of starting it as under question 1; `<W>` is the work folder.

**Entry 32 — rendering one scene of two.** One file `scenes.blend` with two scenes, each Cycles on the processor, one sample, 64 by 64, with its own camera, sun and plane. SceneA's plane uses `assets/a.png`; SceneB's uses `assets/b.png`, and SceneB also holds an object linked (relative) from `assets/lib.blend`. Each render was started from the command line with `--scene <name> --render-output <folder> --render-frame 1`.

*(a) What is said.* With `b.png` moved to another folder, the lines naming either image, or a warning or error, were:

```
--- render SceneA EXIT 0
00:00.454  render           | Saved: '<W>/e32/out/onlySceneA_0001.png'
--- render SceneB EXIT 0
00:00.334  cycles           | WARNING Image file <W>/e32/assets/b.png does not exist.
00:00.334  cycles           | ERROR Failed to load 1 image files
00:00.448  render           | Saved: '<W>/e32/out/onlySceneB_0001.png'
```

Rendering SceneA said nothing about `b.png`; rendering SceneB, the control, named it.

*(b) Which files the disk records as read.* First, a check that this disk keeps access times: a text file's access time was set to 1577833200 with Python's `os.utime`, the file read with `cat`, and its access time read again — it moved to 1792212811. Then, with `b.png` back, the access times of `a.png`, `b.png` and `lib.blend` were all set to 1577833200, one scene rendered, and the three read back; and the same for the other scene:

```
--- rendering SceneA ---
    access times set to 1577833200 for all three
    EXIT 0 | 1 frame saved
    a.png      access time now 1792212822  (moved)
    b.png      access time now 1577833200  (unchanged)
    lib.blend  access time now 1792212822  (moved)
--- rendering SceneB ---
    access times set to 1577833200 for all three
    EXIT 0 | 1 frame saved
    a.png      access time now 1577833200  (unchanged)
    b.png      access time now 1792212823  (moved)
    lib.blend  access time now 1792212823  (moved)
```

So the image used only by the scene not being rendered kept its old access time, in both directions; the image of the scene being rendered had its time moved; and the linked library, used only by SceneB, had its time moved whichever scene was rendered.

**What this does not settle.** It covers an image used by a scene's material and a linked library, and nothing else a file can point at; and it does not cover a scene reached from the rendered one as a background scene, through the compositor, or through a sequencer strip. A trace of every file a render opens, and the memory a render needs, were not taken — see the *Cannot be run* section of `Audits/Blender Research/Dependencies/Hand tests — 5.3.md`.

**Standing lines, as the hand tests leave them.** *Driven from outside:* the whole entry was run from the command line with no window, and `--scene` chose the scene. *Operating system:* macOS only, on the machine named under question 1.

---

## 15. Can a file rely on a colour management profile that lives on the computer rather than inside the file, and how does Blender find it?

**Label:** documented, except the order, which is partly **inferred**.

**Answer.**
- **Yes.** Colour handling uses an OpenColorIO configuration that lives outside the file. The file's colour spaces and views are names inside that configuration. If those names are not found, the file reports that the configuration it was made with "is missing". If the requested configuration fails to load, the file reports that too, and a built-in "fallback" configuration is used.
- **How Blender finds it.** The sources name five possible sources:
  - the `BLENDER_OCIO` environment variable
  - the standard `OCIO` environment variable
  - the active project's setting (Blender default, ACES 2.0 Studio, ACES 1.3 Studio, or a path or `ocio://` name)
  - the configuration bundled with Blender (in `datafiles/colormanagement`)
  - the fallback
- **Order.** `BLENDER_OCIO` wins over `OCIO` ("If not set, the OCIO environment variable is used"). Where the project setting sits in that order is not stated. The API lists the sources in the order above, but a list order is not a stated rule, so placing the project setting between them is **inferred** at most.
- **5.3 changes.**
  - Images can now carry a standard "interop ID" alongside the colour-space name, to work across configurations.
  - Saved sRGB images now include an ICC profile.
  - Projects can choose their own configuration.

**Evidence (2026-10-11):**
- "Blender will use the standard OCIO environment variable to read an OpenColorIO configuration other than the default Blender one." and "There is also a BLENDER_OCIO environment variable to change the configuration for Blender only." — manual, https://docs.blender.org/manual/en/5.3/render/color_management/opencolorio.html
- "BLENDER_OCIO: Path to override the OpenColorIO configuration file. If not set, the OCIO environment variable is used." — manual, https://docs.blender.org/manual/en/5.3/advanced/command_line/arguments.html
- `ocio_config_source`: "BLENDER_OCIO environment variable", "OCIO environment variable", "Project – The active project’s OpenColorIO configuration setting.", "Blender – The OpenColorIO configuration bundled with Blender.", "Fallback – The built-in fallback configuration, if loading other configurations failed."; `is_failed_opencolorio_config` "The requested OpenColorIO config failed to load"; `is_missing_opencolorio_config` "A color space, view or display was not found, which likely means the OpenColorIO config used to create this blend file is missing" — API, https://docs.blender.org/api/5.3/bpy.types.BlendFileColorspace.html
- "OpenColorIO configuration to use for this project" with options BLENDER, ACES_2_0_STUDIO, ACES_1_3_STUDIO, PATH — API, https://docs.blender.org/api/5.3/bpy.types.BlenderProject.html
- "./datafiles/colormanagement — Default OpenColorIO configuration." — manual, https://docs.blender.org/manual/en/5.3/advanced/blender_directory_layout.html
- "The list of color spaces depends on the active OCIO config." — manual, https://docs.blender.org/manual/en/5.3/editors/image/image_settings.html
- Release notes: "Improved interopability between blend files and assets created with different OpenColorIO configurations." — https://developer.blender.org/docs/release_notes/5.3/rendering/; "New `Image.colorspace_settings.interop_id` alongside the existing `name`." — https://developer.blender.org/docs/release_notes/5.3/python_api/; "Saving sRGB image files now includes an ICC profile." — https://developer.blender.org/docs/release_notes/5.3/pipeline_io/; "[OpenColorIO configuration] … to choose between the default Blender configuration, ACES 1.3, ACES 2.0 or a custom configuration." — https://developer.blender.org/docs/release_notes/5.3/core/

**Not covered.** Whether the `.blend` file records *which* configuration it was made with, by path or by name. The sources only say the file can detect that it "likely" is missing. What happens to a render when names are missing. Where the project setting stands against the environment variables. Whether a user `datafiles/colormanagement` overrides the bundled one. The layout page says only that user data files "either override or add to system data files".

**Where I looked.** Colour management pages (index, colour spaces, OpenColorIO, system configuration), the command-line arguments page, the directory layout page, the API for BlendFileColorspace, BlenderProject and ColorManagedInputColorspaceSettings, and the 5.3 core, rendering, pipeline and python_api notes. Searched "OCIO", "OpenColorIO", "colormanagement", "config" and "ICC".

**Outside the interface.** *Brought up to date by the hand tests of 2026-10-17 — see* Observed *below. What follows is what the sources alone said.* Environment variables are set outside Blender before it starts, so they apply equally to background runs. That is **inferred** from their being listed on the command-line page.

**Operating system.** *Brought up to date by the hand tests of 2026-10-17 — see* Observed *below.* Not stated as differing for the configuration. The display-side notes on wide gamut and HDR do differ by system, but they concern the screen, not the file.

### Observed — hand tests on the Creator's Mac, 2026-10-17

**Label: observed.** Same machine, Blender and way of starting it as under question 1; `<W>` is the work folder.

**Entry 33 — which colour configuration is used, and what a missing name does.** A small configuration `tiny.ocio` was written holding one colour space `TinyLinear`, one display `TinyDisplay` and one view `TinyView`. Blender 5.3 was started four ways with no window, and a script printed the two variables as Blender sees them, every property of `bpy.data.colorspace`, the colour spaces offered, the scene's view transform, and a new image's colour space and `interop_id`.

*Nothing set:*

```
OCIO as Blender sees it        = '/Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/colormanagement/config.ocio'
BLENDER_OCIO as Blender sees it = None
bpy.data.colorspace.working_space = 'Linear Rec.709'
bpy.data.colorspace.working_space_interop_id = 'lin_rec709_scene'
bpy.data.colorspace.is_missing_opencolorio_config = False
bpy.data.colorspace.is_failed_opencolorio_config = False
bpy.data.colorspace.ocio_config_path = '/Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/colormanagement/config.ocio'
bpy.data.colorspace.ocio_config_source = 'BLENDER'
colour spaces offered: 64, first two ['ACES 1.3 sRGB', 'ACES 2.0 sRGB']
scene view transform = 'AgX' | display device = 'sRGB'
new image colour space = 'sRGB' | interop_id = 'srgb_rec709_display'
```

*`OCIO` pointing at `tiny.ocio`:*

```
00:00.001  color_management | Using config from OCIO: "<W>/e33/tiny.ocio"
bpy.data.colorspace.working_space = 'TinyLinear'
bpy.data.colorspace.working_space_interop_id = 'data'
bpy.data.colorspace.ocio_config_path = '<W>/e33/tiny.ocio'
bpy.data.colorspace.ocio_config_source = 'OCIO'
colour spaces offered: 2, first two ['TinyLinear', 'scene_linear']
scene view transform = 'TinyView' | display device = 'TinyDisplay'
new image colour space = 'TinyLinear' | interop_id = 'data'
```

*`BLENDER_OCIO` pointing at `tiny.ocio`:* the same as the line above, with `Using config from BLENDER_OCIO: "<W>/e33/tiny.ocio"` printed, `OCIO as Blender sees it = '<W>/e33/tiny.ocio'` — Blender had set it — and `bpy.data.colorspace.ocio_config_source = 'BLENDER_OCIO'`.

*`BLENDER_OCIO` pointing at a file that does not exist:*

```
00:00.003  color_management | ERROR OpenColorIO Error: Error could not read '<W>/e33/no_such.ocio' OCIO profile.
00:00.003  color_management | ERROR Failed to load config from BLENDER_OCIO: <W>/e33/no_such.ocio
OCIO as Blender sees it        = '/Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/colormanagement/config.ocio'
BLENDER_OCIO as Blender sees it = '<W>/e33/no_such.ocio'
bpy.data.colorspace.is_missing_opencolorio_config = False
bpy.data.colorspace.is_failed_opencolorio_config = False
bpy.data.colorspace.ocio_config_source = 'BLENDER'
colour spaces offered: 64, first two ['ACES 1.3 sRGB', 'ACES 2.0 sRGB']
```

So both variables are honoured, a missing file is named and the bundled configuration used instead, and in that case the properties read `is_failed_opencolorio_config = False` and source `'BLENDER'`.

*A name the active configuration does not have.* A file `cm.blend` was saved with the bundled configuration, holding an image whose colour space is `ACEScg` (its `interop_id` then read `'lin_ap1_scene'`) and a scene view transform of `AgX` — the script's attempt to pick `Filmic` from a list of the view transforms on offer read that list as `['NONE']` and fell back to `AgX`. Opened under `OCIO` pointing at `tiny.ocio`:

```
00:00.001  color_management | Using config from OCIO: "<W>/e33/tiny.ocio"
00:00.309  color_management | WARNING sequencer colorspace "sRGB" not found, will use default instead.
00:00.309  color_management | WARNING image colorspace "ACEScg" not found, will use default instead.
00:00.310  bpy.rna          | WARNING current value '-1' matches no enum in 'ColorManagedInputColorspaceSettings', '(null)', 'name'
  image colorspace name now = ''
  image interop_id now      = 'NONE'
  scene view transform now  = 'TinyView'
  is_missing_opencolorio_config = True | is_failed = False | source = OCIO
```

and, as the control, opened with nothing set it read `'ACEScg'`, `'lin_ap1_scene'`, `'AgX'` and `is_missing_opencolorio_config = False`. So the image's colour-space name came back empty and its `interop_id` as `'NONE'`, the view transform was replaced with no line printed about it, and the file was flagged as made with a configuration that is missing. The render is not stopped by any of this.

*Whether the file names its configuration.* An uncompressed copy of `cm.blend` was saved (`compress=False`; the ordinary save is compressed and its text cannot be read), and `strings -a` run over it for `ocio`, `config`, `colormanagement`, `ACEScg`, `AgX`, `interop` and `lin_`. Among the matches were `?ACEScg` and `lin_ap1_scene`; nothing matched `ocio` or `colormanagement`, and every `config` match was a field name of Blender's own (`FreestyleConfig`, `keyconfigstr[64]` and the like). `strings` lists only runs of four or more printable characters, so a three-letter name such as `AgX` could not be seen this way.

**Entry 34 — through the window: colour-management names missing on screen.** Blender 5.3 was started with its window from the terminal on the Mac, with `OCIO` pointing at `tiny.ocio`, opening `cm.blend` from entry 33, and driven by timers in a script; pictures were taken with Blender's own screenshot command, in later runs after forcing a redraw with `bpy.ops.wm.redraw_timer(type='DRAW_WIN_SWAP', iterations=1)`, because without it a picture could show the window as it was a step earlier. The terminal carried the same three warnings as the run with no window (`sequencer colorspace "sRGB" not found`, `image colorspace "ACEScg" not found`, and the `bpy.rna` line about the value `-1`), and the script read `image colorspace name = '' | view = 'TinyView'`. On screen:

- *The status bar*, at its right-hand end after the version number, carried a yellow panel with a warning triangle reading **Color Management**, in every picture.
- *The image's Color Space field*, in the Image tab of the Image editor's sidebar, read **TinyLinear** — not empty, though a script reads the name as `''`.
- *The scene's Color Management panel*, in the Render tab of a Properties editor, read **Display** `TinyDisplay`, **View** `TinyView`, **Look** `None`, **Exposure** `0.000`, **Gamma** `1.000`, with closed sub-panels *Curves*, *White Balance*, *Working Space* and *Advanced*.
- *The close dialog.* After a change was made and an undo step pushed from the script (`is_dirty: True`), `bpy.ops.wm.quit_blender('INVOKE_DEFAULT')` brought up a dialog reading: **Save changes before closing?** / `cm.blend` / *Displays, views or color spaces in this file were missing and have been changed. Saving it with this OpenColorIO configuration may cause loss of data.* — with the buttons **Don't Save**, **Cancel** and **Save As...** (**Save As...** highlighted). The process was then ended without saving. Two earlier tries quit without a dialog: changing the frame or adding an image from the script left `is_dirty` `False`, and Blender quit and wrote its recovery file.

*The warning's tooltip was not reached.* The pointer was rested on the status-bar warning with `Window.event_simulate`, the panel drew lighter, and no tooltip appeared in pictures taken two and four seconds later. Its words stay open.

**Standing lines, as the hand tests leave them.** *Driven from outside:* with no window the same substitutions happen and the warnings reach the terminal, and `bpy.data.colorspace.is_missing_opencolorio_config` reads `True` for a script to see; with a window, a standing status-bar warning and the close-dialog warning are added. *Operating system:* macOS only, on the machine named under question 1.


---

## 16. How is a font installed on one kind of computer recorded in the file, and what does Blender do when a font it points at is not on the computer?

**Label:** recording: documented as a file path. That there is no lookup by font name is **inferred** from silence. Missing font: **not in the sources**.

**Answer.**
- **Recorded as a path to a font file.** A font is loaded from a file through the File Browser and stored as a font data-block with a file path, which accepts `//`. It can be packed into the file.
- The sources describe no recording of a font by its installed name. They only describe where font files typically live on Unix-like systems. That fonts are found only by path is **inferred** from the absence of any other mechanism.
- Blender's own built-in font, "Bfont", is always present. It is the default for the Font node.
- A font file Blender cannot read gives the error "Not a valid font".
- A sequencer text strip with no font set uses the interface font. That says nothing about a missing font file.
- **When the font file is not there.** Not stated.

**Evidence (2026-10-11):**
- "To load a different Font, click one of the Load buttons in the Font panel and navigate to a font file." and "The built-in font is always present and shows in this list as “Bfont”." and "If you select a font that is unsupported by Blender, you will get the error Not a valid font." and "Fonts are typically located under /usr/lib/fonts, or some variant like /usr/lib/X11/fonts, but not always." — manual, https://docs.blender.org/manual/en/5.3/modeling/texts/properties.html
- VectorFont `filepath` "(default “”, never None, blend relative // prefix supported)" and "pack() — Pack the font into the current blend file" — API, https://docs.blender.org/api/5.3/bpy.types.VectorFont.html
- "Font ✓ ✓ References font files." — manual, https://docs.blender.org/manual/en/5.3/files/data_blocks.html
- "Fonts — Default location to browse for text object font files." — manual, https://docs.blender.org/manual/en/5.3/editors/preferences/file_paths.html
- "Font of the text. Falls back to the UI font by default." — API, https://docs.blender.org/api/5.3/bpy.types.TextStrip.html

**Not covered.** What a text object, text strip or String to Curves result shows when its font file is missing. Whether Blender substitutes Bfont. Whether a render reports it.

**Where I looked.** Text object pages (introduction, properties, editing), the Font node, the text strip, the data-block table, the Preferences file paths page, and the API for VectorFont, BlendDataFonts and TextStrip. Searched "font" with "missing", "fallback", "fall back", "system font", "installed", "builtin" and "Bfont".

**Outside the interface.** *Brought up to date by the hand tests of 2026-10-17 — see* Observed *below. What follows is what the sources alone said.* Not stated.

**Operating system.** *Brought up to date by the hand tests of 2026-10-17 — see* Observed *below.* The only system-specific statement is the Unix font-location note. Font paths on other systems are not discussed.

### Observed — hand tests on the Creator's Mac, 2026-10-17

**Label: observed.** Same machine, Blender and way of starting it as under question 1; `<W>` is the work folder.

**Entry 35 — how a font is recorded, and a font file that is gone.** A copy of the Mac's `/System/Library/Fonts/Supplemental/Arial.ttf` was put at `assets/testfont.ttf` beside `font.blend`, which holds a text object reading `AB` in that font, with a camera, a sun, Cycles, 32 by 32. Opened with no window, with the font there:

```
FONT name='Arial Regular' filepath='//assets/testfont.ttf' packed_file=None
FONT name='Bfont Regular' filepath='<builtin>' packed_file=None
text object font: Arial Regular | bold: Bfont Regular
evaluated mesh: vertices=332 polygons=334
Info: No missing files
```

So a font taken from the system's own font folder is recorded as an ordinary path to its file, here relative to the blend-file, with no family name of its own; the built-in font is recorded as the literal text `<builtin>`. The font file was then moved to another folder. Opening the file and asking nothing at all printed:

```
00:00.327  reports          | ERROR Unable to pack file, source path '<W>/e35/assets/testfont.ttf' not found
00:00.327  reports          | ERROR Unable to pack file, source path '<W>/e35/assets/testfont.ttf' not found
00:00.327  geom.vfont       | WARNING Font file doesn't exist: //assets/testfont.ttf
00:00.327  reports          | ERROR Unable to pack file, source path '<W>/e35/assets/testfont.ttf' not found
00:00.327  reports          | ERROR Unable to pack file, source path '<W>/e35/assets/testfont.ttf' not found
00:00.327  geom.vfont       | WARNING Font file doesn't exist: //assets/testfont.ttf
```

and the readings, after the same six lines, were:

```
FONT name='Arial Regular' filepath='//assets/testfont.ttf' packed_file=None
FONT name='Bfont Regular' filepath='<builtin>' packed_file=None
text object font: Arial Regular | bold: Bfont Regular
evaluated mesh: vertices=16 polygons=16
Warning: Path '<W>/e35/assets/testfont.ttf' not found, from local data-block 'VFArial Regular'
```

A Cycles render of one frame from the command line printed the six lines on opening and the same six again once rendering had started, wrote its frame and exited with status 0. Its picture differed from the one rendered with the font present: `PICTURES size=(32, 32) largest channel difference=1.000000 channels differing=90 of 4096 — DIFFERENT PICTURE`.

So the stored path is kept, the text's own font stays `Arial Regular`, and its letters are lost: the mesh for two letters fell from 332 vertices to 16, rather than being drawn in the built-in font. The messages call the missing font a failure to *pack*, though nothing was being packed.

**Entry 36 — through the window: a text object whose font is gone.** A file `smoozy_font.blend` was built with no window: the factory scene with its cube removed, a text object `Txt` reading `Smoozy` in a copy of the Mac's Arial at `assets/testfont.ttf`, white emission on a black world, an orthographic camera looking straight down, Cycles, 320 by 120; saved with the text object active and the Properties editor on its Object Data tab; and then the font file moved to another folder. The file was opened with its window from the terminal on the Mac and driven by timers in a script; pictures were taken with Blender's own screenshot command after a forced redraw. The terminal carried the six lines of entry 35 on opening, and the same three-line group again each time the text was drawn or evaluated — eleven groups before the first picture.

- *The viewport* showed the text as a row of six hollow rectangles, one per letter, outlined in the orange of the selection.
- *A render* inside the window with `bpy.ops.render.render(write_still=True)` returned `{'FINISHED'}` and wrote `f12_.png`: six hollow light rectangles side by side on a dark ground, no letters.
- *The Font panel*, opened with a simulated click on its header, read: **Regular** `Arial R…` (the name cut by the panel's width), in ordinary white, with the shield, folder and X buttons beside it and no mark of any kind; **Bold**, **Italic** and **Bold &…** each `Bf…`, greyed, with a user count of `3`.

So the window draws what the evaluated mesh of entry 35 holds — one rectangle per letter — and the Font panel marks nothing as missing.

**Standing lines, as the hand tests leave them.** *Driven from outside:* entry 35 was run with no window, every message reached the terminal, and the render was not stopped; entry 36 shows the same outcome inside the window, with nothing on screen marking the font as missing. *Operating system:* macOS only, on the machine named under question 1, with a font from the Mac's own font folder. Where Windows and Linux keep fonts, and what a font path written on one means on another, were not run in this sitting.

---

## 17. What use does a `.blend` file make of the places it records for writing, besides writing to them?

**Label:** documented, except where marked **observed** or **inferred**.

**Answer.** These places are also *read*, checked or reused:
- **Render output path.**
  - Rendering checks for existing files when "Overwrite" is off and can write empty "placeholder" files. That is how the manual's multi-machine hint lets several computers share one folder. That the check is for existing files is **inferred** from the settings' wording.
  - The animation player plays back what is at the render output path.
  - Cycles' "Denoise Animation" reads the previously rendered frames from the render output path and frame range when no input file is given (**observed**).
  - Blender can give the exact file name for a given frame.
- **Point caches.** A disk cache is read back. "External" reads a cache from a user-chosen path. "Use Library Path" lets linked copies share the library's cache.
- **Fluid cache.** It is loaded from the cache folder, with a frame offset applied "only when loading it".
- **Geometry Nodes bakes.** The Bake node is for "saving and loading" geometry. The bake path is where it is loaded from.
- **Proxies.** Proxies are read for preview. An existing proxy file can be used, and "Overwrite" decides whether existing proxies are kept.
- **Texture cache.** `.tx` files are read while rendering, and regenerated when the source image changes.
- **Render cache.** "Open Cached Render" loads the last cached render back.
- **Unpacking.** Some options reuse a file that already exists at the target instead of writing it.

**Evidence (2026-10-11 unless marked):**
- "Overwrite existing files while rendering (default True)" and "Create empty placeholder files while rendering frames (similar to Unix ‘touch’) (default False)" and frame_path "Return the absolute path to the filename to be written for a given frame" — API, https://docs.blender.org/api/5.3/bpy.types.RenderSettings.html
- "Disable Overwrite, enable Placeholders in the Render Output panel. Start as many machines as you wish rendering to that directory." — manual, https://docs.blender.org/manual/en/5.3/render/output/properties/output.html
- "playing back images or a video located at the render output of the current scene." — manual, https://docs.blender.org/manual/en/5.3/render/output/animation_player.html
- **Observed**, Cycles file `operators.py`, class `CYCLES_OT_denoise_animation`, lines 38–41 and 69–87 (read 2026-10-16): "File path for image to denoise. If not specified, uses the render file path and frame range from the scene" and `filepath = scene.render.frame_path(frame=frame)` … `if not os.path.isfile(filepath):` … `"Frame '%s' not found, animation must be complete"`.
- "External — Allows you to read the cache from a drive using a user-specified file path." — manual, https://docs.blender.org/manual/en/5.3/physics/baking.html
- "Offset — Frame offset that is used when loading the simulation from the cache. It is not considered when baking the simulation, only when loading it." — manual, https://docs.blender.org/manual/en/5.3/physics/fluid/type/domain/cache.html
- "The Bake node allows saving and loading intermediate geometries." — manual, https://docs.blender.org/manual/en/5.3/modeling/geometry_nodes/geometry/operations/bake.html
- "File — Allows you to use preexisting proxies." and "Overwrite — Whether to overwrite existing proxy files or keep them." — manual, https://docs.blender.org/manual/en/5.3/editors/video_sequencer/sequencer/sidebar/proxy.html
- "Automatically generate and update .tx files for images when rendering. Files are regenerated when the source image is modified." — manual, https://docs.blender.org/manual/en/5.3/render/cycles/render_settings/performance.html
- "Find the render cache file for the current scene and load it into the Render Result." — manual, https://docs.blender.org/manual/en/5.3/editors/image/editing.html
- "However, if the final file exists already, it will use that file, instead of unpacking it." — manual, https://docs.blender.org/manual/en/5.3/files/blend/packed_data.html

**Not covered.** Exactly how the Overwrite or Placeholder check decides a frame is "done". Whether an empty placeholder counts. Whether the compositor File Output node checks for existing files.

**Where I looked.** Render output, animation player, physics baking, fluid cache, Geometry Nodes bake, proxy, Cycles performance, image editing and packed data pages. The API for RenderSettings and PointCache. Cycles `operators.py`. Searched "overwrite", "placeholder", "existing", "load", "read" and "reuse".

**Outside the interface.** *Brought up to date by the hand tests of 2026-10-17 — see* Observed *below. What follows is what the sources alone said.*
- Render settings, caches and bakes are stored data and act in background renders. That is **inferred**.
- The animation player opens a window. The command line also has an `-a` player mode that is "ignored if -b is set".
- Cycles' denoise operator is Python and can be called from a script.

**Operating system.** *Brought up to date by the hand tests of 2026-10-17 — see* Observed *below.* Not stated as differing. The placeholder wording compares to "Unix ‘touch’" as a description only.

### Observed — hand tests on the Creator's Mac, 2026-10-17

**Label: observed.** Same machine, Blender and way of starting it as under question 1; `<W>` is the work folder.

**Entry 7 — whether a File Output node looks at the files it would write.** The file `fo_script.blend` described under question 10 — Cycles, 16 by 16, one sample, frames 1 to 2, regular output `//renders/frame_`, and a File Output node writing `//fileoutput_new_folder/` with one item — had been rendered once, leaving `renders/frame_0001.png`, `frame_0002.png` and `fileoutput_new_folder/fo_script0001.exr`, `fo_script0002.exr`. Its `use_overwrite` was set to False and saved, and the animation rendered again from the command line with `--render-anim`. Modification times, to the fraction of a second, and sizes, before and after:

```
--- before ---
<W>/e07/fileoutput_new_folder/fo_script0001.exr 1792210884.641494190 5171
<W>/e07/fileoutput_new_folder/fo_script0002.exr 1792210884.744491657 5171
<W>/e07/renders/frame_0001.png 1792210884.703814545 1522
<W>/e07/renders/frame_0002.png 1792210884.745182949 1522
render EXIT 0
00:00.305  render           | Skipping existing frame "<W>/e07/renders/frame_0001.png"
00:00.305  render           | Skipping existing frame "<W>/e07/renders/frame_0002.png"
00:00.305  reports          | No frames rendered, skipped to not overwrite
--- after ---
<W>/e07/fileoutput_new_folder/fo_script0001.exr 1792210884.641494190 5171
<W>/e07/fileoutput_new_folder/fo_script0002.exr 1792210884.744491657 5171
<W>/e07/renders/frame_0001.png 1792210884.703814545 1522
<W>/e07/renders/frame_0002.png 1792210884.745182949 1522
```

Then the two regular frames were moved to another folder, leaving the File Output node's two files where they were, and the animation rendered again:

```
render EXIT 0
00:00.466  render           | Executing compositor
00:00.468  render           | Saved: '<W>/e07/fileoutput_new_folder/fo_script0001.exr'
00:00.529  render           | Saved: '<W>/e07/renders/frame_0001.png'
00:00.566  render           | Executing compositor
00:00.567  render           | Saved: '<W>/e07/fileoutput_new_folder/fo_script0002.exr'
00:00.568  render           | Saved: '<W>/e07/renders/frame_0002.png'
--- after ---
<W>/e07/fileoutput_new_folder/fo_script0001.exr 1792210903.860359744 5171
<W>/e07/fileoutput_new_folder/fo_script0002.exr 1792210903.959558838 5171
<W>/e07/renders/frame_0001.png 1792210903.921781225 1522
<W>/e07/renders/frame_0002.png 1792210903.960210213 1522
```

So with Overwrite off, whether a frame is rendered was decided by the regular output path alone: when those frames existed nothing was rendered and nothing written, and when they did not the File Output node wrote over its own existing files, with nothing said about them.

**Entry 9 — what a render looks at before writing.** Small files built by script (Cycles, 16 by 16, one sample) and rendered from the command line with no window.

*(a) Overwrite off.* `render.blend`, frames 1 to 5, output `//ow/frame_`, was rendered with `--render-anim`; frames 2 and 4 were moved away; `use_overwrite` was set to False and saved; the animation was rendered again, exit status 0:

```
before
<W>/e09/ow/frame_0001.png 1792211054.710700922 1174
<W>/e09/ow/frame_0003.png 1792211054.714295332 1174
<W>/e09/ow/frame_0005.png 1792211054.716968087 1174
00:00.329  render           | Skipping existing frame "<W>/e09/ow/frame_0001.png"
00:00.360  render           | Saved: '<W>/e09/ow/frame_0002.png'
00:00.360  render           | Skipping existing frame "<W>/e09/ow/frame_0003.png"
00:00.362  render           | Saved: '<W>/e09/ow/frame_0004.png'
00:00.362  render           | Skipping existing frame "<W>/e09/ow/frame_0005.png"
after
<W>/e09/ow/frame_0001.png 1792211054.710700922 1174
<W>/e09/ow/frame_0002.png 1792211056.355784012 1174
<W>/e09/ow/frame_0003.png 1792211054.714295332 1174
<W>/e09/ow/frame_0004.png 1792211056.357686443 1174
<W>/e09/ow/frame_0005.png 1792211054.716968087 1174
```

Only the two missing frames carry a new modification time.

*(b) An empty file counts as a frame already done.* `ph.blend`, frames 1 to 5, with `use_overwrite` False and `use_placeholder` True; five zero-length files placed at the five names `frame_path()` gave; the animation rendered:

```
EXIT 0
00:00.310  render           | Skipping existing frame "<W>/e09/ph/frame_0001.png"
00:00.310  render           | Skipping existing frame "<W>/e09/ph/frame_0002.png"
00:00.310  render           | Skipping existing frame "<W>/e09/ph/frame_0003.png"
00:00.310  render           | Skipping existing frame "<W>/e09/ph/frame_0004.png"
00:00.310  render           | Skipping existing frame "<W>/e09/ph/frame_0005.png"
00:00.310  reports          | No frames rendered, skipped to not overwrite
```

All five were still zero bytes afterwards. Nothing was rendered and the exit status was 0.

*(c) A broken template stops a render started from the command line.* As a control, `tmplok.blend` with output `//tmpl/{blend_name}_` printed `FRAMEPATH OK '<W>/e09/tmpl/tmplok_0001.png'` and rendered frame 1 with exit status 0, `Saved: '<W>/e09/tmpl/tmplok_0001.png'`. Then `tmplbad.blend` with `//tmpl/{blend_name.png`:

```
FRAMEPATH RAISED RuntimeError Error: Parse errors in path '//tmpl/{blend_name.png':
- Invalid or incomplete template expression '{blend_name.png'.
bad template single frame EXIT 1
00:00.323  render           | Rendering single frame
00:00.323  render           | Rendering frame 1
00:00.323  reports          | ERROR Parse errors in path '//tmpl/{blend_name.png':
                            | - Invalid or incomplete template expression '{blend_name.png'.
bad template animation EXIT 1
00:00.325  render           | Rendering animation (frames 1..2)
00:00.325  render           | Rendering frame 1
00:00.325  reports          | ERROR Parse errors in path '//tmpl/{blend_name.png':
                            | - Invalid or incomplete template expression '{blend_name.png'.
```

The folder held only `tmplok_0001.png` afterwards.

*(d) The output given on the command line, before and after the file name.* `render.blend` (recorded output `//ow/frame_`, Overwrite off from part (a)) was rendered with `--render-output <W>/e09/before/x_` placed before the file name, then with `--render-output <W>/e09/after/x_` placed after it, both exit status 0:

```
--- output given BEFORE the file ---
00:00.324  render           | Skipping existing frame "<W>/e09/ow/frame_0001.png"
00:00.324  reports          | No frames rendered, skipped to not overwrite
--- output given AFTER the file ---
00:00.356  render           | Saved: '<W>/e09/after/x_0001.png'
```

and no `before` folder was made. So the output given before the file was replaced by the file's own recorded path, which is where the render looked (and, Overwrite being off, found frame 1 already there); the one given after the file was used.

*(e) `frame_path()` against what appears.* `fp2.blend`, frames 1 to 5, output `//fp2/shot_{blend_name}_##_`:

```
PREDICTED frame 1: <W>/e09/fp2/shot_fp2_01_.png
PREDICTED frame 7: <W>/e09/fp2/shot_fp2_07_.png
frame 7 EXIT 0
00:00.359  render           | Saved: '<W>/e09/fp2/shot_fp2_07_.png'
anim EXIT 0
00:00.354  render           | Saved: '<W>/e09/fp2/shot_fp2_01_.png'
… through …
00:00.361  render           | Saved: '<W>/e09/fp2/shot_fp2_05_.png'
```

Then `fp.blend` with the same kind of path, the scene set to frame 7, and `bpy.ops.render.render(write_still=True)` called from a script:

```
PATH '//fp/shot_{blend_name}_##_'
PREDICTED <W>/e09/fp/shot_fp_07_.png
00:00.393  render           | Saved: '<W>/e09/fp/shot_fp_##_.png'
render(write_still=True) -> {'FINISHED'}
ON DISK ['shot_fp_##_.png']
MATCH False
```

So for a frame render and an animation started from the command line the file that appears is the one `frame_path()` names; for a still render from a script the template is filled and the `##` is written literally into the file name. Entry 4 met the same thing: a still render to `<W>/e04/out/gone_` wrote `gone_.png` where `frame_path` named `gone_0001.png`.

**Entry 10 — the file-wide path commands against six recorded places.** One file, `paths.blend`, was given six full paths into the work folder: an image `ReadTex` loaded from `assets/tex.png` and used by a plane's material, the render output path `renders/frame_`, a compositor File Output node's `directory` `fileout`, a cloth modifier's point cache with External on and its path `ptcache`, a geometry-nodes modifier's bake folder `gnbake`, and an ocean modifier's cache path `oceanbake` on an object `Grid`. It was saved — which in this build left all six as they were — and the two commands were run from the script with no window:

```
Info: Total files 2 | Changed 2 | Failed 0
MAKE PATHS RELATIVE -> {'FINISHED'}
--- after Make Paths Relative ---
  image ReadTex   : //assets/tex.png
  render output   : <W>/e10/renders/frame_
  file output node: <W>/e10/fileout
  point cache     : <W>/e10/ptcache
  gn bake dir     : <W>/e10/gnbake
  ocean cache     : //oceanbake
Info: Total files 2 | Changed 2 | Failed 0
MAKE PATHS ABSOLUTE -> {'FINISHED'}
--- after Make Paths Absolute ---
  image ReadTex   : <W>/e10/assets/tex.png
  render output   : <W>/e10/renders/frame_
  file output node: <W>/e10/fileout
  point cache     : <W>/e10/ptcache
  gn bake dir     : <W>/e10/gnbake
  ocean cache     : <W>/e10/oceanbake
```

With all six pointed at a folder `gone` that does not exist:

```
Warning: Path '<W>/e10/gone' not found, from local data-block 'OBGrid'
Warning: Path '<W>/e10/gone/tex.png' not found, from local data-block 'IMReadTex'
REPORT MISSING FILES -> {'FINISHED'}
BLEND PATHS: ['<W>/e10/gone', '<W>/e10/gone/tex.png']
```

`OBGrid` carries the ocean modifier. The render output path, the File Output node, the point cache and the geometry-nodes bake folder were named by nothing. With the six put back, `bpy.data.file_path_foreach` was walked with its default flags and with `EXPAND_CACHES`:

```
FOREACH flags=None visited 2:
    Grid '<W>/e10/oceanbake'
    ReadTex '<W>/e10/assets/tex.png'
FOREACH flags={'EXPAND_CACHES'} visited 2:
    Grid '<W>/e10/oceanbake'
    ReadTex '<W>/e10/assets/tex.png'
```

So the commands, the report, the listing and the walk all reach the image and the ocean cache path and none of the other four places recorded for writing. Which settings this build has, asked directly:

```
RenderSettings.use_texture_cache                True
RenderSettings.use_auto_generate_texture_cache  False
RenderSettings.use_render_cache                 False
RenderSettings.use_overwrite                    True
RenderSettings.use_placeholder                  False
Preferences render_cache_directory       ''
Preferences texture_cache_directory      ''
generate_texture_cache in dir(bpy.ops.render)   True
denoise_animation in dir(bpy.ops.cycles)        True
merge_images in dir(bpy.ops.cycles)             True
hasattr(bpy.ops.render, 'smoozy_no_such_operator') -> True
```

The last line asks for an operator that does not exist and gets `True`, so `hasattr` settles nothing about an operator; the lines above it use `dir()`.

**Entry 11 — Cycles' animation denoiser reads the recorded output place.** `dn.blend`: Cycles, 32 by 32, frames 1 to 5, a plane, the view layer's denoising data passes on, output `//f_`. The output format was set with no window:

```
MEDIA TYPE SET TO MULTI_LAYER_IMAGE
FILE FORMAT OPEN_EXR_MULTILAYER
RENDERED ['dn.blend', 'dn.blend1', 'f_0001.exr', 'f_0002.exr', 'f_0003.exr', 'f_0004.exr', 'f_0005.exr']
AFTER DELETING FRAME 3 ['dn.blend', 'dn.blend1', 'f_0001.exr', 'f_0002.exr', 'f_0004.exr', 'f_0005.exr']
Error: Frame '<W>/e11/f_0003.exr' not found, animation must be complete
DENOISE ANIMATION RAISED RuntimeError Error: Frame '<W>/e11/f_0003.exr' not found, animation must be complete
FOLDER AFTER ['dn.blend', 'dn.blend1', 'f_0001.exr', 'f_0002.exr', 'f_0004.exr', 'f_0005.exr']
```

`bpy.ops.cycles.denoise_animation()` was called with no arguments. It looked for the frames at the file's recorded output place, named the missing one and stopped, writing nothing.

**Entry 14 — inside Blender's window: Overwrite off, the animation player and the viewport render.** Two files were built with no window: `wow.blend` (Cycles, 16 by 16, one sample, frames 1 to 5, output `//wow/frame_`), rendered once, frames 2 and 4 deleted, and `use_overwrite` set to False and saved; and `gl.blend`, the same but with output `//glout/gl_` and that folder empty. Each was then opened with its window from the terminal on the Mac and driven by timers in a script; a first-run setup panel was closed with a simulated Esc key first.

*Overwrite off, rendered inside the window* with `bpy.ops.render.render(animation=True)`:

```
before
   frame_0001.png 1792211553.685269 1516
   frame_0003.png 1792211553.756814 1516
   frame_0005.png 1792211553.832997 1516
00:03.158  render           | Saved: '<W>/e14/wow/frame_0002.png'
00:03.196  render           | Saved: '<W>/e14/wow/frame_0004.png'
render(animation=True) -> {'FINISHED'}
after
   frame_0001.png 1792211553.685269 1516
   frame_0002.png 1792211557.506066 1516
   frame_0003.png 1792211553.756814 1516
   frame_0004.png 1792211557.544307 1516
   frame_0005.png 1792211553.832997 1516
```

Only the two missing frames were written. In the window no `Skipping existing frame` line was printed for the three that were kept, where the runs with no window under entry 9 print one for each.

*The animation player*, called with `bpy.ops.render.play_rendered_anim()` in the same window, and the running processes listed straight after:

```
Executing command:
  '/Applications/Blender 5.3b.app/Contents/MacOS/Blender' --gpu-backend metal -a -f 24 1.0 -s 1 -e 5 -j 1 -c 4096 <W>/e14/wow/frame_0001.png
play_rendered_anim() -> {'FINISHED'}
00:00.135  color_management | Using config from OCIO: "/Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/colormanagement/config.ocio"
    16334 /Applications/Blender 5.3b.app/Contents/MacOS/Blender --gpu-backend metal -a -f 24 1.0 -s 1 -e 5 -j 1 -c 4096 <W>/e14/wow/frame_0001.png
```

So the player is a second Blender started with `-a` and handed the first frame at the file's own recorded output place. (It was ended straight after the listing; the `color_management` line is the player's own.)

*The viewport render*, `bpy.ops.render.opengl(animation=True)` called with the 3D viewport as its context, in `gl.blend`:

```
render output path: '//glout/gl_'
that folder before: []
3D viewport found: True
00:03.106  render           | Saved: '<W>/e14/glout/gl_0003.png'
00:03.106  render           | Saved: '<W>/e14/glout/gl_0005.png'
00:03.106  render           | Saved: '<W>/e14/glout/gl_0004.png'
00:03.106  render           | Saved: '<W>/e14/glout/gl_0001.png'
00:03.106  render           | Saved: '<W>/e14/glout/gl_0002.png'
render.opengl(animation=True) -> {'FINISHED'}
that folder after: ['gl_0001.png', 'gl_0002.png', 'gl_0003.png', 'gl_0004.png', 'gl_0005.png']
```

The viewport render wrote into the recorded output place; the five `Saved` lines came out of frame order.

**Entry 12 — the texture cache folder is read back.** Recorded in full under question 18. With the cache on, a render wrote `checker.png.0-926BDA8569F1F2504902F06ABA51E46D.tx` into a `blender_tx` folder beside the image when generating was on, and later renders with the cache on gave a picture differing from the cache-off one whenever that file was there, generating or not, and the cache-off picture came back exactly when the cache was turned off with the file still there.

**Entry 53 — an output place given on the command line is saved into the file.** Recorded in full under question 20. In short: with `-o <W>/e53/after_` placed after the file name and a script calling `bpy.ops.wm.save_mainfile()`, the render output held in memory read `'<W>/e53/after_'`, and the saved file, opened again, read the same instead of its own `'//out/{blend_name}_####.png'`. With `-o` placed before the file name, memory and the saved file both read `'//out/{blend_name}_####.png'`.

**Standing lines, as the hand tests leave them.** *Driven from outside:* every render here except entry 14's was started from the command line with no window. Entry 14 ran inside Blender's window: there too Overwrite off wrote only the missing frames, but without the `Skipping existing frame` lines the runs with no window print; the animation player and the viewport render were called inside the window, and both used the recorded output place. Entry 53's output place, given on the command line, was saved into the file by a script with no window. *Operating system:* macOS only, on the machine named under question 1.

---

## 18. What does a render do when a cache or a bake that a file records is absent?

**Label:** partly documented, partly **observed**, partly **inferred**, the rest **not in the sources**.

**Answer.**
- **Cycles texture cache (`.tx`).** If "Auto Generate" is on, missing or outdated `.tx` files are created when rendering. In Cycles' own image lookup, a tile that *failed* to load returns the "missing" colour. A tile not yet loaded returns the image's average colour (**observed**). What happens with Auto Generate off and no `.tx` file is not stated.
- **Physics caches (point caches).** The simulation is worked out between the cache's start and end frames "whether you bake or not". Baking before rendering is recommended so the result "remains exactly the same every time". That a missing bake means the simulation is recalculated at render, possibly with a different result, is **inferred**.
- **Geometry Nodes simulation zones.** Baking "allows for the simulation to be rendered in a non-sequential order". That without a bake, rendering frames out of order does not give the simulated result is **inferred**.
- **Volumes (OpenVDB).** A failed load keeps an error message on the volume.
- **Absent fluid caches, Geometry Nodes bake files, Alembic or USD cache files, and Ocean or Dynamic Paint output.** Not in the sources. This includes the case where the file says a bake exists but its files are gone.

**Evidence (2026-10-11 unless marked):**
- "Automatically create tx files from image files when rendering, if the files do not exist or are outdated. The path to store the texture cache files is configured in the preferences (default False)" — API, https://docs.blender.org/api/5.3/bpy.types.RenderSettings.html
- **Observed**, Cycles file `source/kernel/util/image.h`, line 361 (read 2026-10-16): `return (tile_descriptor == KERNEL_TILE_LOAD_FAILED) ? missing_rgba : tex.average_color;`
- "The simulation is only calculated for positive frames in between the Start and End frames of the Cache panel, whether you bake or not." and "It is generally recommended to bake your physics simulations before rendering. Aside from no longer needing to go through the time-consuming process of simulating again, baking can help prevent potential glitches and ensure that the outcome of the simulation remains exactly the same every time." — manual, https://docs.blender.org/manual/en/5.3/physics/baking.html
- "When the result is ready to be sent to a render-farm, it can be baked to disk. This allows for the simulation to be rendered in a non-sequential order." — manual, https://docs.blender.org/manual/en/5.3/modeling/geometry_nodes/simulation/simulation_zone.html
- "If loading grids failed, error message with details" — API, https://docs.blender.org/api/5.3/bpy.types.VolumeGrids.html
- PointCache `info` "Info on current cache status" and `is_baked` "The cache is baked" — API, https://docs.blender.org/api/5.3/bpy.types.PointCache.html

**Not covered.** Whether a render whose recorded bake files are missing recalculates, shows nothing, or reports an error. Whether any message is printed. Behaviour for the fluid, Geometry Nodes bake, Alembic/USD, Ocean and Dynamic Paint cases.

**Where I looked.** Physics baking, particle cache, fluid cache, Geometry Nodes bake, baking and simulation zone pages, the mesh sequence cache page, the Cycles performance page, the API for PointCache, NodesModifierBake, VolumeGrids, CacheFile and RenderSettings, and Cycles' kernel image files. Searched "cache", "bake", "baked", "missing", "absent", "not found", "regenerat" and "recalculat".

**Outside the interface.** *Brought up to date by the hand tests of 2026-10-17 — see* Observed *below. What follows is what the sources alone said.* Render settings and caches are stored data, so they behave the same in a background render. That is **inferred**. The `.tx` files can also be made ahead of time from the command line (`--command maketx`).

**Operating system.** *Brought up to date by the hand tests of 2026-10-17 — see* Observed *below.* Not stated as differing.

### Observed — hand tests on the Creator's Mac, 2026-10-17

**Label: observed.** Same machine, Blender and way of starting it as under question 1; `<W>` is the work folder. Pictures were compared pixel by pixel inside Blender, never file against file.

**Entry 12 — Cycles' texture cache with no `.tx` file, with one made, and with one already there.** A 64 by 64 checked image `assets/checker.png` was used as the emission colour of a plane filling the frame; Cycles on the processor, 16 samples, denoising off, 120 by 120, Standard view transform. Each render was its own run with no window, the two settings set from a script and one still frame written:

```
cacheoff: use_texture_cache=False auto_generate=False -> blender_tx folder: (none)
cacheon_nogen: use_texture_cache=True auto_generate=False -> blender_tx folder: (none)
cacheon_gen: use_texture_cache=True auto_generate=True -> blender_tx folder: ['checker.png.0-926BDA8569F1F2504902F06ABA51E46D.tx']
cacheoff2: blender_tx before: ['checker.png.0-926BDA8569F1F2504902F06ABA51E46D.tx']
cacheon_nogen2: blender_tx before: ['checker.png.0-926BDA8569F1F2504902F06ABA51E46D.tx']
cacheon_gen2a: blender_tx before: ['checker.png.0-926BDA8569F1F2504902F06ABA51E46D.tx']
cacheon_gen2b: blender_tx before: ['checker.png.0-926BDA8569F1F2504902F06ABA51E46D.tx']
```

Every run exited 0 and none printed anything about the cache. Compared:

```
   cache off against cache on with no file made: PICTURES size=(120, 120) largest channel difference=0.000000 channels differing=0 of 57600 — SAME PICTURE
   cache off against cache on with the file made: PICTURES size=(120, 120) largest channel difference=0.003922 channels differing=1 of 57600 — DIFFERENT PICTURE
   cache off against cache off again (file present): PICTURES size=(120, 120) largest channel difference=0.000000 channels differing=0 of 57600 — SAME PICTURE
   cache off against cache on, no generating (file present): PICTURES size=(120, 120) largest channel difference=0.003922 channels differing=1 of 57600 — DIFFERENT PICTURE
   cache on with the file made, against the same again: PICTURES size=(120, 120) largest channel difference=0.000000 channels differing=0 of 57600 — SAME PICTURE
   cache on generating, first against second run: PICTURES size=(120, 120) largest channel difference=0.000000 channels differing=0 of 57600 — SAME PICTURE
```

So with the cache on, generating off and no `.tx` file anywhere, the render finished and gave exactly the picture it gives with the cache off. With generating on, a `.tx` file appeared in a `blender_tx` folder beside the image. Whenever a `.tx` file existed and the cache was on — generating or not — the picture differed from the cache-off picture, here by one channel of one pixel by one step in 255; turning the cache off gave the first picture back exactly, and repeated runs of each setting matched each other exactly. The size of that difference depends on the picture used, and only this one was tried.

A first try of this entry passed the settings wrongly from the terminal, so all seven of its renders ran with the cache off; it is kept in the run record of the list and carries nothing.

**Entries 41 to 47 share a stage.** A grid or cube glowing white against a black world under an orthographic camera, Cycles on the processor, four samples, denoising off, 160 by 160, frames 1 to 60, Standard view transform; each frame was rendered from a script with no window unless an entry says otherwise. As a check that the pictures can be compared, the same stepped-through frame was rendered twice in two runs: `two renders of the same stepped frame: PICTURES size=(160, 160) largest channel difference=0.000000 channels differing=0 of 102400 — SAME PICTURE`.

**Entry 41 — an unbaked cloth, frame 60 reached by jumping and by stepping.** A 4-unit grid standing upright with a cloth modifier, no pinning, nothing baked, the animation never played, saved as `cloth.blend`. It was opened twice: once going straight to frame 60, once stepping through every frame to it; each time the evaluated grid was read and the frame rendered:

```
on open: is_baked False | is_outdated False | is_frame_skip False | use_disk_cache False | info '0 frames in memory (0 B).'
MODE jump FRAME 60  lowest z = -12.2137  highest z = -8.2136  lowest x = -2.0001  highest x = 2.0000
at frame 60: is_baked False | is_outdated False | is_frame_skip True | use_disk_cache False | info '2 frames in memory (12 KiB), not exact since frame 1'
MODE step FRAME 60  lowest z = -11.7782  highest z = -7.7782  lowest x = -2.0000  highest x = 2.0000
at frame 60: is_baked False | is_outdated False | is_frame_skip False | use_disk_cache False | info '60 frames in memory (363 KiB).'
jump against step: PICTURES size=(160, 160) largest channel difference=0.756863 channels differing=372 of 102400 — DIFFERENT PICTURE
```

So the cloth fell whichever way the frame was reached, but not to the same place; jumping left the cache reading `not exact since frame 1`. Nothing was printed about a cache in either run.

**Entry 42 — a cloth disk cache that has been moved away.** The same grid with the cloth's Disk Cache ticked, range 1 to 60, saved as `baked.blend` and baked with `bpy.ops.ptcache.bake_all(bake=True)`, which returned `{'FINISHED'}` and made a folder `blendcache_baked` holding 60 files; saved again. Frame 60 was rendered from the command line with `--render-frame 60`, the folder moved to another folder, and frame 60 rendered again; the cache's flags were read with no window before each render:

```
--- with the cache present ---
FLAGS is_baked True | is_outdated False | is_frame_skip False | use_disk_cache True | info '60 frames on disk.'
EXIT 0
lines naming cache or bake: 2
--- cache folder moved away ---
FLAGS is_baked True | is_outdated False | is_frame_skip False | use_disk_cache True | info '0 frames on disk.'
EXIT 0
lines naming cache or bake: 2
cache-present-against-cache-gone: PICTURES size=(160, 160) largest channel difference=0.756863 channels differing=2961 of 102400 — DIFFERENT PICTURE
cache-gone-against-no-bake-jump: PICTURES size=(160, 160) largest channel difference=0.756863 channels differing=2931 of 102400 — DIFFERENT PICTURE
cache-present-against-no-bake-step: PICTURES size=(160, 160) largest channel difference=0.000000 channels differing=0 of 102400 — SAME PICTURE
cache-gone-against-no-bake-step: PICTURES size=(160, 160) largest channel difference=0.756863 channels differing=2961 of 102400 — DIFFERENT PICTURE
```

The two lines naming a cache or a bake in each render were the file's own name — `Read blend: "<W>/e42/baked.blend"` — and the frame's. So with the cache there, the render was pixel for pixel the stepped-through unbaked one; with it gone, the picture changed, nothing was said, the exit status was 0, `is_baked` still read `True`, and `info` dropped to `'0 frames on disk.'`. (The two unbaked pictures it is compared with are entry 41's.)

**Entry 43 — an External point cache pointing at an empty folder.** The same grid with the cloth cache's External ticked, its path an empty folder `emptycache` and its name `SmoozyCache`, saved as `external.blend`. Read with no window, then frame 60 reached by jumping and rendered:

```
use_external True | filepath '<W>/e43/emptycache' | name 'SmoozyCache'
is_baked False | is_outdated False | info 'No valid data to read!'
MODE jump FRAME 60  lowest z = -12.2137  highest z = -8.2136  lowest x = -2.0001  highest x = 2.0000
at frame 60: is_baked False | is_outdated False | is_frame_skip True | use_disk_cache False | info 'No valid data to read!'
against the no-bake jump render: PICTURES size=(160, 160) largest channel difference=0.000000 channels differing=0 of 102400 — SAME PICTURE
```

Blender printed nothing of its own beyond the `Read blend` and `Saved` lines. So `info` reads `No valid data to read!`, and the render simulated: its picture is pixel for pixel entry 41's unbaked jumped one.

**Entry 44 — a geometry-nodes simulation zone: unbaked by jumping and stepping, baked, and with its bake moved away.** A cube carrying a simulation zone whose one float state grows by 0.005 each frame and moves the cube along x by that amount, so the movement piles up; the cube starts spanning x = −10 to −8; seen from above; saved as `gn.blend`. Unbaked, frame 60 reached two ways:

```
MODE jump FRAME 60  lowest z = -1.0000  highest z = 1.0000  lowest x = -9.9850  highest x = -7.9850
MODE step FRAME 60  lowest z = -1.0000  highest z = 1.0000  lowest x = -0.8500  highest x = 1.1500
jump against step: PICTURES size=(160, 160) largest channel difference=0.756863 channels differing=3399 of 102400 — DIFFERENT PICTURE
```

Stepping moved the cube about 9.1 units; jumping moved it 0.015. Then the modifier's bake target was set to `DISK`, its bake folder to `//gnbake/`, and the zone's bake to `DISK`, `ANIMATION` and a custom range of 1 to 60, and it was baked:

```
BAKE id 462735299 target DISK mode ANIMATION range 1 60
BAKE SINGLE -> {'FINISHED'}
BAKE FOLDER gnbake/462735299/blobs 60 files
BAKE FOLDER gnbake/462735299/meta 60 files
```

Frame 60 was rendered from the command line with `--render-frame 60`, the `gnbake` folder moved to another folder, and frame 60 rendered again. Both exited 0; neither printed anything naming a bake, a cache, a warning or an error. Compared:

```
baked-against-bake-gone: PICTURES size=(160, 160) largest channel difference=0.756863 channels differing=3399 of 102400 — DIFFERENT PICTURE
baked-against-stepped-no-bake: PICTURES size=(160, 160) largest channel difference=0.000000 channels differing=0 of 102400 — SAME PICTURE
bake-gone-against-jumped-no-bake: PICTURES size=(160, 160) largest channel difference=0.513726 channels differing=18 of 102400 — DIFFERENT PICTURE
```

So the bake made a single out-of-order frame match the stepped-through result exactly, and with the bake gone the render fell back near the unbaked jumped picture, silently and with exit status 0.

**Entry 45 — a fluid domain whose cache folder has been moved away.** A small gas simulation — domain resolution 16, cache folder `//fluidcache`, cache type All, frames 1 to 12 — was saved as `fluid.blend` and baked with `bpy.ops.fluid.bake_all()`, with no window:

```
BEFORE BAKE  | cache_directory: //fluidcache | has_cache_baked_any: False | has_cache_baked_data: False
BAKE ALL -> {'FINISHED'}
AFTER BAKE   | cache_directory: //fluidcache | has_cache_baked_any: True | has_cache_baked_data: True
cache folder: ['config', 'data', 'guiding', 'mesh', 'noise', 'particles'] | files: 24
cache folder moved away; exists now: False
SAME SESSION, FOLDER GONE | cache_directory: //fluidcache | has_cache_baked_any: True | has_cache_baked_data: True
```

The file, saved after the bake, was opened again in a fresh run:

```
REOPENED with the cache folder gone (folder exists: False)
  has_cache_baked_any: True | has_cache_baked_data: True | has_cache_baked_noise: True | has_cache_baked_mesh: True | has_cache_baked_particles: True
```

and in a further fresh run, after asking for the evaluated scene and stepping to frames 1, 2 and 3: `REOPENED, folder exists before evaluating: False` and `after evaluating and stepping to frame 3, folder exists: False`. So the domain's flags are held in the file and go on reading `True` with the cache gone; and in these runs opening the file did not make the folder again.

**Entry 46 — an EEVEE light probe volume, unbaked, baked and freed.** A closed room — a 10-unit cube with its faces turned inward — with a sphere inside, a light probe volume scaled to fill it, a strong green world and a camera inside the room; EEVEE (`EEVEE names offered: ['BLENDER_EEVEE']`), 120 by 120, Standard view transform. In one run with no window, a frame was rendered unbaked, then after `bpy.ops.object.lightprobe_cache_bake(subset='ALL')`, then after `bpy.ops.object.lightprobe_cache_free(subset='ALL')`:

```
lightprobe_cache_bake -> {'FINISHED'}
lightprobe_cache_free -> {'FINISHED'}
  unbaked against baked: PICTURES size=(120, 120) largest channel difference=0.811765 channels differing=10634 of 57600 — DIFFERENT PICTURE
  baked against freed: PICTURES size=(120, 120) largest channel difference=0.811765 channels differing=10634 of 57600 — DIFFERENT PICTURE
  unbaked against freed: PICTURES size=(120, 120) largest channel difference=0.000000 channels differing=0 of 57600 — SAME PICTURE
```

So the bake changed the picture, freeing it put the unbaked picture back exactly, and nothing was printed about the missing bake. EEVEE printed no warning of any kind before rendering on this Mac.

**Entry 47 — an ocean modifier's bake.** A 10-unit plane with an ocean modifier (resolution 16, frames 1 to 5, wave scale 3, bake folder `//oceanbake/`), a mid-grey material, a low sun, a camera looking across it, Cycles, 96 by 96, saved as `ocean.blend`. As the control that the picture shows the waves, the same file with the modifier's random seed set to 7 instead of 0, unbaked, rendered differently: `control, no bake: seed 0 against seed 7: PICTURES size=(96, 96) largest channel difference=0.133333 channels differing=26151 of 36864 — DIFFERENT PICTURE`. The bake operator's arguments, read off the running Blender, are `modifier` and `free`.

*Baking with no window:*

```
ocean_bake(modifier='Ocean') -> {'FINISHED'}
AFTER exec BAKE   is_cached: False | files: 0
```

*Baking with a window*, the file opened with its window from the terminal on the Mac and the operator called from a timer as the interface calls it, then the file saved — once for `ocean.blend` and once for a copy `ocean_seed7.blend` with seed 7 and its bake folder `//oceanbake7/`:

```
ocean_bake('INVOKE_DEFAULT', modifier='Ocean') -> {'FINISHED'}
AFTER BAKE  is_cached: True | folder '//oceanbake/' | files: ['disp_0001.exr', 'disp_0002.exr', 'disp_0003.exr', 'disp_0004.exr', 'disp_0005.exr']
ocean_bake('INVOKE_DEFAULT', modifier='Ocean') -> {'FINISHED'}
AFTER BAKE  is_cached: True | folder '//oceanbake7/' | files: ['disp_0001.exr', 'disp_0002.exr', 'disp_0003.exr', 'disp_0004.exr', 'disp_0005.exr']
```

*What a render reads.* `ocean.blend` was then opened with no window and frame 3 rendered from the command line three times: (A) with its own seed-0 bake in `oceanbake`; (B) with that folder moved away and a copy of the seed-7 bake put in its place; (C) with no `oceanbake` folder at all. And (D) `ocean_seed7.blend` was rendered with its own seed-7 bake.

```
(A) opened: is_cached True | files 5 ['disp_0001.exr', 'disp_0002.exr', 'disp_0003.exr']
render A_own_bake EXIT 0 | lines naming ocean, cache, bake, warning or error: 0
(B) opened: is_cached True | files 5 ['disp_0001.exr', 'disp_0002.exr', 'disp_0003.exr']
render B_swapped_bake EXIT 0 | lines naming ocean, cache, bake, warning or error: 0
(C) opened: is_cached True | files 0 []
render C_no_bake_folder EXIT 0 | lines naming ocean, cache, bake, warning or error: 0
A_own_bake       against B_swapped_bake     PICTURES size=(96, 96) largest channel difference=0.125490 channels differing=26079 of 36864 — DIFFERENT PICTURE
B_swapped_bake   against D_seed7_own_bake   PICTURES size=(96, 96) largest channel difference=0.000000 channels differing=0 of 36864 — SAME PICTURE
A_own_bake       against C_no_bake_folder   PICTURES size=(96, 96) largest channel difference=0.113725 channels differing=25689 of 36864 — DIFFERENT PICTURE
C_no_bake_folder against nobake_seed0       PICTURES size=(96, 96) largest channel difference=0.117647 channels differing=25806 of 36864 — DIFFERENT PICTURE
```

So a render reads the files in the bake folder: given another seed's files, it drew that seed's sea exactly. With the bake folder gone, the modifier still read `is_cached True`, the render exited 0 and said nothing, and the picture — looked at, brightened — was an even grey with no waves at all, unlike both the baked sea and the same file never baked. Two earlier tries in this sitting (a scene lit so brightly that every picture was white, and a seed changed after baking, which set `is_cached` to `False`) gave pictures that could not tell these cases apart; they are kept in the list's run record.

**Entry 48 — through the window: the Timeline's cache bar with the cloth cache present and moved away.** `baked.blend` from entry 42 was opened with its window from the terminal on the Mac, once with `blendcache_baked` in place and once with it moved to another folder; a script made the cloth grid the active object, set the frame to 30, and turned on the Timeline's cache display (`show_cache`, `cache_cloth`), and pictures were taken with Blender's own screenshot command after a forced redraw. The script read `point cache: is_baked True | info '60 frames on disk.'` the first time and `point cache: is_baked True | info '0 frames on disk.'` the second. The Timeline strip of the two pictures, compared:

```
timeline strips compared: largest channel difference=0.349020 channels differing=24190 of 847134
present  timeline strip average colour r=0.189 g=0.191 b=0.200
gone     timeline strip average colour r=0.189 g=0.190 b=0.193
```

With the cache there, a solid dark-blue bar runs under the frame numbers from the start of the range to 60. With the folder gone the bar is not drawn at all, leaving the frame numbers and the playhead at 30.

**Entry 40 — an Alembic cache file that is absent.** Recorded in full under question 12. With the `.abc` file a Mesh Sequence Cache modifier reads moved away, a render from the command line exited 0, printed `Could not open as Ogawa file from provided streams.`, `Unable to open <W>/e40c/assets/cube.abc` and `object.modifier  | WARNING Object: "AbcCube", Modifier: "MeshSequenceCache", Could not create cache reader for file //assets/cube.abc` (twice each), and drew the object's own mesh instead of the cached one.

**Standing lines, as the hand tests leave them.** *Driven from outside:* every render here was started with no window, and a missing cache or bake was silent in every case — cloth disk cache, geometry-nodes bake, light probe bake, ocean bake, Alembic aside (entry 40 names it) — with exit status 0. A program can read `PointCache.info` (`0 frames on disk.`, `No valid data to read!`), but `is_baked`, the fluid domain's flags and the ocean's `is_cached` stay `True` with the files gone. The ocean bake could be made only through the window (entry 47); with no window the operator returned `{'FINISHED'}` and wrote nothing. Inside the window the Timeline shows a lost cloth cache by drawing no cache bar (entry 48). *Operating system:* macOS only, on the machine named under question 1.

---

## 19. What decides whether two recorded library locations are treated as the same library or as two different libraries?

**Label:** **not in the sources**.

**Answer.** No source says how Blender decides whether two library locations are the same library. Cases the sources do not cover include a relative path versus an absolute path to the same file, different letter case, a different folder route to the same file, and linked folders. Documented facts nearby:
- A library is itself a named data-block.
- One file can hold "one local and several linked Objects sharing the same name". That several libraries can be told apart by name is **inferred**.
- Packed linked data lives in separate "archive" libraries, each owned by its source library. Each packed item carries a hash of "a unique version of a packed linked data and all of its dependencies".
- A library can be relocated "to one or several others".

**Evidence (2026-10-11):**
- "However, it is possible to have one local and several linked Objects sharing the same name." — manual, https://docs.blender.org/manual/en/5.3/files/data_blocks.html
- "Unique data-block ID name, including library one if any" and "Hash representing a unique version of a packed linked data and all of its dependencies" — API, https://docs.blender.org/api/5.3/bpy.types.ID.html
- "Archive libraries of packed IDs, generated (and owned) by this source library" — API, https://docs.blender.org/api/5.3/bpy.types.Library.html
- "Relocate the given library to one or several others" — API, https://docs.blender.org/api/5.3/bpy.ops.wm.html

**Not covered.** All of the question.

**Where I looked.** Searched the manual, API and release notes for "same library", "same file", "different library", "already linked", "duplicate library", "two libraries", "canonical", "normalize path", "realpath", "symlink", "symbolic link" and "resolve path". Read the Library, ID and BlendDataLibraries API pages, the `bpy.ops.wm` link and relocate entries, the Link & Append page, the data-blocks page and the 5.3 core notes.

**Outside the interface.** *Brought up to date by the hand tests of 2026-10-17 — see* Observed *below. What follows is what the sources alone said.* Not stated.

**Operating system.** *Brought up to date by the hand tests of 2026-10-17 — see* Observed *below.* Not stated. Question 8 notes that letter case in paths is treated differently on some systems, but no source connects that to library identity.

### Observed — hand tests on the Creator's Mac, 2026-10-17

**Label: observed.** Same machine, Blender and way of starting it as under question 1; `<W>` is the work folder.

**How every test below was done.** Library files holding empty objects, and a host file, were built by script with no window. Every link was made twice over, in two separate runs: once with `bpy.ops.wm.link(filepath=<lib>/<kind>/<name>, directory=<lib>/<kind>/, filename=<name>, relative_path=…)`, the operator the interface's File ‣ Link runs, and once with `bpy.data.libraries.load(<lib>, link=True, relative=…)`; the arguments of both were read off the running Blender first. After every step the script printed `len(bpy.data.libraries)`, each library's `name` and `filepath`, and each linked object's `name_full` and library, in lines beginning `HT|`. Each host file was then saved and opened in a fresh Blender, and the same printed again (`after-fresh-reopen`). **The two routes gave the same counts and the same recorded text in every test below**; where they printed something else differently, the test says so; the operator route also prints a `call|returned` line, which is left out of the quotations when it reads `{'FINISHED'}`.

**Entry 24 — the count can say one and can say two; two files that share a name.** *First:* `libA.blend` holding `A1` and `A2`, `libB.blend` holding `B1`, and an empty `host.blend`; `A1` linked, then `A2` from the same file by the same spelling, then `B1`:

```
HT|after-A1|library_count|1
HT|after-A1|library|0|name='libA.blend'|filepath='//libA.blend'
HT|after-A2-same-spelling|library_count|1
HT|after-B1-other-file|library_count|2
HT|after-B1-other-file|library|1|name='libB.blend'|filepath='//libB.blend'
HT|after-fresh-reopen|library_count|2
```

*Second:* `p/lib.blend` holding `P1` and `q/lib.blend` holding `Q1`, linked in that order:

```
HT|after-p|library_count|1
HT|after-p|library|0|name='lib.blend'|filepath='//p/lib.blend'
HT|after-q|library_count|2
HT|after-q|library|1|name='lib.blend.001'|filepath='//q/lib.blend'
HT|after-q|object|name='Q1'|name_full='Q1 [lib.blend.001]'|from='//q/lib.blend'
HT|after-fresh-reopen|library_count|2
HT|after-fresh-reopen|library|1|name='lib.blend.001'|filepath='//q/lib.blend'
```

So the count can come out one or two, which every test below rests on; a library's `name` is the bare file name, and a numbered suffix is added when two would clash.

**Entry 25 — one file recorded relative and then absolute, written with steps that go nowhere, and the file-wide path commands.** *First:* `lib.blend` holding `A1` and `A2` beside `host.blend`; `A1` linked with the relative option on, then `A2` with it off:

```
HT|after-relative|library_count|1
HT|after-relative|library|0|name='lib.blend'|filepath='//lib.blend'
HT|after-absolute|library_count|1
HT|after-absolute|library|0|name='lib.blend'|filepath='//lib.blend'
HT|after-fresh-reopen|library_count|1
HT|after-fresh-reopen|library|0|name='lib.blend'|filepath='//lib.blend'
```

*Second:* three objects linked, relative option off, through three spellings of one file:

```
HT|paths|plain='<W>/q19/t3-ops/lib.blend'|dotted='<W>/q19/t3-ops/./lib.blend'|updown='<W>/q19/t3-ops/sub/../lib.blend'
HT|after-plain|library_count|1
HT|after-dot-step|library_count|1
HT|after-up-down-step|library_count|1
HT|after-fresh-reopen|library|0|name='lib.blend'|filepath='<W>/q19/t3-ops/lib.blend'
```

*Third:* the first host, opened again, put through `bpy.ops.file.make_paths_absolute()`, saved and reopened in the same run, then `bpy.ops.file.make_paths_relative()`, saved and reopened:

```
HT|reopened|library|0|name='lib.blend'|filepath='//lib.blend'
Info: Total files 1 | Changed 1 | Failed 0
HT|make_paths_absolute|returned={'FINISHED'}
HT|after-make-absolute|library|0|name='lib.blend'|filepath='<W>/q19/t2-ops/lib.blend'
Info: Read library: '<W>/q19/t2-ops/lib.blend', '<W>/q19/t2-ops/lib.blend', parent '<direct>'
HT|reopened-after-absolute|library_count|1
Info: Total files 1 | Changed 1 | Failed 0
HT|make_paths_relative|returned={'FINISHED'}
HT|after-make-relative|library|0|name='lib.blend'|filepath='//lib.blend'
Info: Read library: '<W>/q19/t2-ops/lib.blend', '//lib.blend', parent '<direct>'
HT|reopened-after-relative|library_count|1
```

So in each part one entry stood for the file: a link with the relative option off joined the entry already recorded relative and kept its `//` text; the `./` and `sub/../` spellings joined the plain one and were recorded as the plain full path; and the path commands rewrote the one entry's text in place.

**Entry 26 — capitals in a library's location, on the Mac's ordinary disk.** `lib.blend` holding `A1` and `A2`; `A1` linked through `<folder>/lib.blend` and `A2` through `<folder>/LIB.blend`, the relative option off. Python's own check, in the same run, read `HT|exists|lower=True|upper=True`. Then:

```
HT|after-lower|library_count|1
HT|after-lower|library|0|name='lib.blend'|filepath='<W>/q19/t4-ops/lib.blend'
HT|after-upper|library_count|2
HT|after-upper|library|1|name='LIB.blend'|filepath='<W>/q19/t4-ops/LIB.blend'
HT|after-fresh-reopen|library_count|2
HT|after-fresh-reopen|library|1|name='LIB.blend'|filepath='<W>/q19/t4-ops/LIB.blend'
```

and the shell, asked afterwards with `ls -li`, listed both names as one file, the same number and the same size:

```
159176030 -rw-r--r--@ 1 artist     wheel  85863 Oct 17 07:50 <W>/q19/t4-ops/LIB.blend
159176030 -rw-r--r--@ 1 artist     wheel  85863 Oct 17 07:50 <W>/q19/t4-ops/lib.blend
```

So on this Mac two spellings that differ only in capitals reach one file, and Blender recorded them as two library entries. (The shell check was meant to run before the link; it was run before the files existed and only says so, and is kept in the run record of the list.)

**Entry 27 — two names on disk for one file, and two ways into one folder.** `data/lib.blend` holding `A1` to `A4`; beside it `alias.blend`, a symbolic link to it, and `hard.blend`, a hard link to it; and `data-otherway`, a symbolic link to the whole `data` folder. Python read `HT|inodes|lib=159176152|alias->159176152|hard=159176152` — one and the same file — and `ls -li` listed `alias.blend -> <W>/q19/t5-ops/data/lib.blend` and `hard.blend` with the same number and size as `lib.blend`. One object was linked through each route, relative option off:

```
HT|after-real|library_count|1
HT|after-symlink-to-file|library_count|2
HT|after-hardlink|library_count|3
HT|after-symlink-to-folder|library_count|4
HT|after-fresh-reopen|library|0|name='alias.blend'|filepath='<W>/q19/t5-ops/data/alias.blend'
HT|after-fresh-reopen|library|1|name='hard.blend'|filepath='<W>/q19/t5-ops/data/hard.blend'
HT|after-fresh-reopen|library|2|name='lib.blend'|filepath='<W>/q19/t5-ops/data/lib.blend'
HT|after-fresh-reopen|library|3|name='lib.blend.001'|filepath='<W>/q19/t5-ops/data-otherway/lib.blend'
```

So each route to the one file became a library entry of its own, recorded by the route's own text. A second mount of the folder, which the Linux run of 2026-08-09 also tried, cannot be made on this Mac — see the *Cannot be run* section of `Audits/Blender Research/Dependencies/Hand tests — 5.3.md`.

**Entry 28 — two entries made to hold the very same recorded text.** `host.blend` with `P1` linked from `lib1.blend` and `Q1` from `lib2.blend`, relative option off. The first library's `filepath` was then written into the second's from Python, the file saved and opened again in the same run with `bpy.ops.wm.open_mainfile`, and then opened in a fresh Blender:

```
HT|assignment|returned normally
HT|after-assignment|library_count|2
HT|after-assignment|library|0|name='lib1.blend'|filepath='<W>/q19/t6-ops/lib1.blend'
HT|after-assignment|library|1|name='lib2.blend'|filepath='<W>/q19/t6-ops/lib1.blend'
Warning: Library '<W>/q19/t6-ops/lib1.blend', '<W>/q19/t6-ops/lib1.blend' had multiple instances, save and reload!
Warning: 0 libraries and 1 linked data-blocks are missing (including 0 ObjectData), please check the Info and Outliner editors for details
HT|after-reopen-in-process|library_count|1
HT|after-reopen-in-process|object|name='Q1'|name_full='Q1 [lib1.blend]'|from='<W>/q19/t6-ops/lib1.blend'
Error: Not freed memory blocks: 3, total unfreed memory 0.001324 MB
```

and in the fresh Blender:

```
00:00.321  reports          | WARNING Library '<W>/q19/t6-ops/lib1.blend', '<W>/q19/t6-ops/lib1.blend' had multiple instances, save and reload!
00:00.322  reports          | Read library: '<W>/q19/t6-ops/lib1.blend', '<W>/q19/t6-ops/lib1.blend', parent '<direct>'
00:00.325  reports          | LIB: Object: 'Q1' missing from '<W>/q19/t6-ops/lib1.blend', parent '<direct>'
00:00.325  reports          | WARNING 0 libraries and 1 linked data-blocks are missing (including 0 ObjectData), please check the Info and Outliner editors for details
HT|after-fresh-reopen|library_count|1
```

So two entries holding the same text were allowed in memory and saved, and on reading the file back Blender merged them into one, said so, and named `Q1` — which lives in `lib2.blend` — as missing from `lib1.blend`. The `Error: Not freed memory blocks` line was printed by Blender as each run ended.

**Entry 29 — relocating one library onto a file another library already uses, and onto a file nobody uses.** `host.blend` with `P1` linked from `lib1.blend` and `Q1` from `lib2.blend`, relative option off, saved; a `lib3.blend` nobody links sat beside them. The operator's arguments were read off the running Blender first: `library`, `filepath`, `directory`, `filename`, `files` and others, with `relative_path` defaulting to `True`. Then, in the same run, `bpy.ops.wm.lib_relocate(library='lib2.blend', directory=<folder>/, filename='lib1.blend')`:

```
HT|two-libraries|library|0|name='lib1.blend'|filepath='<W>/q19/t8-ops/lib1.blend'
HT|two-libraries|library|1|name='lib2.blend'|filepath='<W>/q19/t8-ops/lib2.blend'
HT|relocate|library='lib2.blend'|onto=lib1.blend|returned={'FINISHED'}
HT|after-relocate|library_count|2
HT|after-relocate|library|0|name='lib1.blend'|filepath='//lib1.blend'
HT|after-relocate|library|1|name='lib2.blend'|filepath='<W>/q19/t8-ops/lib2.blend'
HT|after-relocate|object|name='Q1'|name_full='Q1 [lib2.blend]'|from='<W>/q19/t8-ops/lib2.blend'
```

and, in a fresh run of the saved host, the same call naming `lib2.blend` and pointing at `lib3.blend` with all three of `filepath`, `directory` and `filename` given:

```
HT|calling|bpy.ops.wm.lib_relocate(library='lib2.blend', filepath='<W>/q19/t8-ops/lib3.blend', directory='<W>/q19/t8-ops/', filename='lib3.blend')
HT|relocate|returned={'FINISHED'}
HT|after|library_count|2
HT|after|library|0|name='lib1.blend'|filepath='<W>/q19/t8-ops/lib1.blend'
HT|after|library|1|name='lib2.blend'|filepath='<W>/q19/t8-ops/lib2.blend'
```

So, with no window, the call returned `{'FINISHED'}` both times. Pointed at the file `lib1.blend` already records, it left the named `lib2.blend` where it was and rewrote `lib1.blend`'s own text into the `//` form; pointed at an unrecorded file, it changed nothing. What happens when a library really is moved onto a file another library records is therefore not reached.

**Entry 30 — the same file reached directly and through another library; two libraries whose text would match; two files that record one library differently.**

*(i)* `lib.blend` holding `A1` and `A2`; `mid.blend` linking `A1` into a local collection `MidCol`; `host.blend` linking `MidCol` from `mid.blend` and then `A2` straight from `lib.blend`, relative option off:

```
HT|after-indirect|library_count|2
HT|after-indirect|library|0|name='lib.blend'|filepath='<W>/q19/t9-ops/lib.blend'
HT|after-indirect|library|1|name='mid.blend'|filepath='<W>/q19/t9-ops/mid.blend'
HT|after-direct|library_count|2
HT|after-fresh-reopen|library_count|2
```

`lib.blend` was already an entry after the first link, and the direct link joined it.

*(ii)* Folders `one` and `two`, each with its own `x.blend`; `one/mid.blend` linking its neighbour with the relative option on (`HT|built-mid.blend|library|0|name='x.blend'|filepath='//x.blend'`); `two/host.blend` linking `two/x.blend` with the relative option on, then `MidCol` from `one/mid.blend`:

```
HT|after-own-relative|library_count|1
HT|after-own-relative|library|0|name='x.blend'|filepath='//x.blend'
HT|after-mid|library_count|3
HT|after-mid|library|0|name='mid.blend'|filepath='//../one/mid.blend'
HT|after-mid|library|1|name='x.blend'|filepath='//x.blend'
HT|after-mid|library|2|name='x.blend.001'|filepath='//../one/x.blend'
HT|resolve|name='x.blend'|filepath='//x.blend'|parent=None|abspath='<W>/q19/t10-ops/two/x.blend'
HT|resolve|name='x.blend.001'|filepath='//../one/x.blend'|parent='//../one/mid.blend'|abspath='<W>/q19/t10-ops/two/../one/../one/x.blend'
HT|after-fresh-reopen|library_count|3
```

So the text `mid.blend` holds as `//x.blend` appears in the host as `//../one/x.blend`, and the two identical texts the set-up aimed at did not arise.

*(iii)* `lib.blend`; `a.blend` linking from it with the relative option on and `b.blend` with it off — their own records read `HT|built-a.blend|library|0|name='lib.blend'|filepath='//lib.blend'` and `HT|built-b.blend|library|0|name='lib.blend'|filepath='<W>/q19/t13-ops/lib.blend'` — and an empty `c.blend` linking a collection from `a.blend` and then one from `b.blend`, relative option off:

```
HT|after-a|library_count|2
HT|after-b|library_count|3
HT|after-b|library|0|name='a.blend'|filepath='<W>/q19/t13-ops/a.blend'
HT|after-b|library|1|name='b.blend'|filepath='<W>/q19/t13-ops/b.blend'
HT|after-b|library|2|name='lib.blend'|filepath='//lib.blend'
HT|after-fresh-reopen|library_count|3
```

`lib.blend` is listed once, holding `a.blend`'s spelling of it.

*Where the two routes did not print the same thing.* The counts and recorded texts above were the same for both routes. The `Read library` lines printed on the fresh reopen were not: for the files built through `bpy.ops.wm.link`, `one/x.blend` read `'//x.blend', parent '<W>/q19/t10-ops/one/mid.blend'` and `lib.blend` in (iii) read `parent '<W>/q19/t13-ops/a.blend'`; for the files built through `bpy.data.libraries.load`, the same libraries read `'//../one/x.blend', parent '<direct>'` and `parent '<direct>'`.

**Entry 31 — packing linked data.** The three archive properties were asked for by name, and then `A1` was linked from `lib.blend` with `bpy.data.libraries.load(<lib>, link=True, pack=True)` (this route only; the link operator has no packing setting). Then the host was saved and opened in a fresh run, and then opened again with `lib.blend` moved away:

```
HT|prop|is_archive|present=True
HT|prop|archive_parent_library|present=True
HT|prop|archive_libraries|present=True
HT|pack|returned normally
HT|after-pack|library_count|2
HT|after-pack|library|0|name='lib.blend'|filepath='<W>/q19/t12-data/lib.blend'
HT|after-pack|library|1|name='lib.blend.001'|filepath='<W>/q19/t12-data/lib.blend'
HT|after-pack|object|name='A1'|name_full='A1 [lib.blend.001]'|from='<W>/q19/t12-data/lib.blend'
HT|archive|name='lib.blend'|filepath='<W>/q19/t12-data/lib.blend'|is_archive=False|archive_parent=None|archives=['<W>/q19/t12-data/lib.blend']
HT|archive|name='lib.blend.001'|filepath='<W>/q19/t12-data/lib.blend'|is_archive=True|archive_parent='<W>/q19/t12-data/lib.blend'|archives=[]
REOPEN libraries 2
HT|reopen-object|name_full='A1 [lib.blend.001]'|library='lib.blend.001'|is_linked_packed=True
HT|lib-gone-object|name_full='A1 [lib.blend.001]'|is_missing=False
```

So packing linked data made two library entries holding the same recorded text, one an archive whose parent is the other; both survived a save and reopen; and with `lib.blend` gone the packed object was still there and not missing, with nothing printed about the library.

**Standing lines, as the hand tests leave them.** *Driven from outside:* every test here was run with no window, through both the link operator and the Python load call. What the Outliner shows for two entries holding the same text was not photographed. *Operating system:* macOS only, on the machine named under question 1, on its ordinary disk; the capitals result in entry 26 is a result for this disk. Linux, Windows and a second mount were not run in this sitting — see the *Cannot be run* section of `Audits/Blender Research/Dependencies/Hand tests — 5.3.md`.

---

## 20. When a `.blend` file is opened and saved again, and none of the places it records for outside files is touched while it is open, what does the saved file record for each of those places?

**Label:** **inferred** for the main answer. The pieces it rests on are documented. No source states outright what an untouched entry looks like after a save.

**Answer.**
- Nothing in the sources says Blender rewrites an untouched path when it saves. From the pieces below I **infer** that a plain Save to the same place writes each untouched entry back in the same form (relative stays relative, absolute stays absolute), unless one of the documented exceptions applies:
  - **Save As or Save Copy to a different folder.** Relative paths are remapped so they still point at the same files. This is on by default for Save As. For a plain Save it is off by default, and in any case it acts only "when saving to a different directory".
  - **Automatic packing on.** If the file has "Automatically Pack Resources" turned on, eligible external files are packed on save. Their entries then become packed rather than plain paths. Marks made by "Pack Resources" and "Pack Linked Libraries" also take effect only on the next save.
  - **Unused entries are dropped.** Data-blocks with no users are not written. So an image, font, sound or similar with no users disappears from the saved file, path and all. Linked data cannot be protected from this, and the manual suggests a custom property to keep unused linked data across saves.
  - **5.3 packed linked data.** Only *directly* linked packed items are written. Indirect ones are left out.
  - **Missing libraries.** Placeholders keep "references to the missing data" so relocating can restore them. That those references are written back on save is **inferred**.
- The 5.3 release notes record "Initial support for keeping .blend files the same when opening and saving a .blend file". They do not say what it covers.

**Evidence (2026-10-11):**
- Release note: "Initial support for keeping .blend files the same when opening and saving a .blend file" and "code writing linked data now only writes directly linked packed IDs" — https://developer.blender.org/docs/release_notes/5.3/core/
- `save_mainfile(…, relative_remap=False, …)` and `save_as_mainfile(…, relative_remap=True, …)`, both "Remap Relative, Remap relative paths when saving to a different directory" — API, https://docs.blender.org/api/5.3/bpy.ops.wm.html
- "Remap Relative — This option remaps Relative Paths (such as linked libraries and images) when saving a file in a new location." — manual, https://docs.blender.org/manual/en/5.3/files/blend/open_save.html
- "When enabled, this option will ensure that all eligible external resource files, existing or added later, are systematically marked as packed. As with Pack Resources, the blend-file must be saved to the drive for this to have an effect." and "Actual packing will happen on the next save of the blend-file." — manual, https://docs.blender.org/manual/en/5.3/files/blend/packed_data.html
- `use_autopack` "Automatically pack all external data into .blend file (default False)" (a property of the file's own data) — API, https://docs.blender.org/api/5.3/bpy.types.BlendData.html
- "This works by skipping zero user data-blocks when writing blend-files." — manual, https://docs.blender.org/manual/en/5.3/files/data_blocks.html
- "Since it is not editable, linked data cannot be protected with the Fake User option. Adding a custom property pointing to an otherwise unused linked data-block (e.g. a Text block) is a good way to keep it referenced across saves and reloads." and "That way, references to the missing data are not lost" — manual, https://docs.blender.org/manual/en/5.3/files/linked_libraries/link_append.html
- "Default value for Relative Paths when loading external files such as images, sounds, and linked libraries. It will be ignored if a path is already set." — manual, https://docs.blender.org/manual/en/5.3/editors/preferences/save_load.html

**Not covered.** A direct statement that untouched paths are written back character for character. Whether separators or letter case are ever normalised on save. What "Initial support for keeping .blend files the same" includes. Whether the "Save Modified Images" preference changes any recorded path (the sources describe it writing image files, not changing paths).

**Where I looked.** Open/save page, packed data page, data-blocks page, Link & Append, Save & Load preferences, the topbar External Data menu, the compatibility page, the API for `bpy.ops.wm` (open and save), BlendData, `bpy.ops.file` and BlendDataLibraries (`write`, with its `path_remap` options), and the 5.3 core notes. Searched "remap", "relative_remap", "save", "unchanged", "same when opening", "autopack" and "zero user".

**Outside the interface.** *Brought up to date by the hand tests of 2026-10-17 — see* Observed *below. What follows is what the sources alone said.* From Python or the command line, `bpy.ops.wm.save_mainfile` does *not* remap by default, and `save_as_mainfile` does. `BlendDataLibraries.write` has its own path options: "NONE No path manipulation (default)", "RELATIVE", "RELATIVE_ALL" and "ABSOLUTE" (https://docs.blender.org/api/5.3/bpy.types.BlendDataLibraries.html). The interface default for Remap Relative is not stated in the manual.

**Operating system.** *Brought up to date by the hand tests of 2026-10-17 — see* Observed *below.* Not stated as differing.

### Observed — hand tests on the Creator's Mac, 2026-10-17

**Label: observed.** Same machine, Blender and way of starting it as under question 1; `<W>` is the work folder.

**Entry 49 — every recorded place through a plain save, and the two ways of listing them.** A folder `<W>/q20/proj` was filled with `assets/tex.png`, `assets/two words.png`, `assets/beep.wav`, a copy of the Mac's Arial as `assets/testfont.ttf`, `assets/test.osl`, `assets/test.ies`, `assets/notes.txt`, a folder `Тестовая папка` holding `текстура_日本語.png`, an `outer.png` one folder up, an Alembic cube `assets/cube.abc` exported by Blender, a three-frame movie `assets/clip_0001-0003.mp4` rendered by Blender, and a `lib.blend` holding an object `LibObj` and an image `lib-inside-image` with a fake user. A file `main.blend` was then built by script with no window and saved with `bpy.ops.wm.save_as_mainfile(filepath=…, relative_remap=False, compress=False)`. It holds fourteen images, each set to a different spelling — what was asked for and what was stored, at the moment of setting:

```
SET	rel-plain	wanted='//assets/tex.png'	got_filepath='//assets/tex.png'	got_raw='//assets/tex.png'
SET	abs-plain	wanted='<W>/q20/proj/assets/tex.png'	got_filepath='<W>/q20/proj/assets/tex.png'	got_raw='<W>/q20/proj/assets/tex.png'
SET	rel-dotdot	wanted='//assets/../assets/tex.png'	got_filepath='//assets/../assets/tex.png'	got_raw='//assets/../assets/tex.png'
SET	rel-doubleslash	wanted='//assets//tex.png'	got_filepath='//assets//tex.png'	got_raw='//assets//tex.png'
SET	rel-backslash	wanted='//assets\\tex.png'	got_filepath='//assets\\tex.png'	got_raw='//assets\\tex.png'
SET	wrong-case	wanted='//assets/TEX.PNG'	got_filepath='//assets/TEX.PNG'	got_raw='//assets/TEX.PNG'
SET	nonlatin	wanted='//Тестовая папка/текстура_日本語.png'	got_filepath='//Тестовая папка/текстура_日本語.png'	got_raw='//Тестовая папка/текстура_日本語.png'
SET	space-name	wanted='//assets/two words.png'	got_filepath='//assets/two words.png'	got_raw='//assets/two words.png'
SET	trailing-space	wanted='//assets/tex.png '	got_filepath='//assets/tex.png '	got_raw='//assets/tex.png '
SET	missing	wanted='//nowhere/missing.png'	got_filepath='//nowhere/missing.png'	got_raw='//nowhere/missing.png'
SET	up-one	wanted='//../outer.png'	got_filepath='//../outer.png'	got_raw='//../outer.png'
SET	raw-setter	wanted='//assets\\..\\assets//TEX.PNG '	got_filepath='//assets\\..\\assets//TEX.PNG '	got_raw='//assets\\..\\assets//TEX.PNG '
SET	unused-fake	wanted='//assets/tex.png'	got_filepath='//assets/tex.png'	got_raw='//assets/tex.png'
SET	unused-nofake	wanted='//assets/tex.png'	got_filepath='//assets/tex.png'	got_raw='//assets/tex.png'
```

(`raw-setter` was set through `filepath_raw`, the rest through `filepath`; `unused-fake` has a fake user and no other user, `unused-nofake` neither; the others have a fake user.) It also holds `LibObj` linked from `//lib.blend`; `lib-inside-image` appended from it, left with no users (`appended lib-inside-image | users 0 | weak reference ('//lib.blend', 'IMlib-inside-image')`); a font used by a text object; a sound; a text; a volume at `//assets/smoke.vdb`; a compositor File Output node with folder `//comp_out/{blend_name}/`; a cloth cache with External on at `//cache/`; a geometry-nodes bake folder `//gn_bake/`; an OSL Script node and an IES node; the render output `//out/{blend_name}_####.png`; the bake path `//bake/baked.png`; a sequencer proxy folder `//proxies/` with a movie strip and a sound strip; a movie clip; and the Alembic file imported with a cache reader.

Four runs followed, each with no window: the build (above); a fresh run opening the file, touching nothing and saving nothing; a fresh run opening it and calling `bpy.ops.wm.save_mainfile()` with no settings passed; and a fresh run opening it and reading it again. A copy was kept from before the plain save. Each run printed every recorded place, one per line.

*What changed between the build run's memory and the first reopening:*

```
< image[lib-inside-image].filepath	'//assets/tex.png'
< image[lib-inside-image].filepath_raw	'//assets/tex.png'
< image[lib-inside-image].packed	False
< image[lib-inside-image].users	0
< image[raw-setter].filepath	'//assets\\..\\assets//TEX.PNG '
< image[raw-setter].filepath_raw	'//assets\\..\\assets//TEX.PNG '
---
> image[raw-setter].filepath	'//assets/../assets//TEX.PNG '
> image[raw-setter].filepath_raw	'//assets/../assets//TEX.PNG '
< image[rel-backslash].filepath	'//assets\\tex.png'
< image[rel-backslash].filepath_raw	'//assets\\tex.png'
---
> image[rel-backslash].filepath	'//assets/tex.png'
> image[rel-backslash].filepath_raw	'//assets/tex.png'
< image[unused-nofake].filepath	'//assets/tex.png'
< image[unused-nofake].filepath_raw	'//assets/tex.png'
< image[unused-nofake].packed	False
< image[unused-nofake].users	0
< weakref[images:lib-inside-image].filepath	'//lib.blend'
< weakref[images:lib-inside-image].id_name	'IMlib-inside-image'
```

*What changed between the first reopening and the reopening after the plain save:* nothing — `diff` printed no line.

So in these runs the build's own save already left out the two images with no users and no fake user — the appended one and `unused-nofake` — and with the appended image went its weak reference to `//lib.blend`; and the backslashes in the two spellings that had them read back as forward slashes the moment the file was opened. The plain save then changed no recorded place that was printed: the doubled separator, the `..` segment, the wrong capitals, the trailing space, the non-Latin name, the space in a name, the missing place, the place one folder up, the full path, the templates, the library, the font, the sound, the text, the volume, the OSL and IES nodes, the bake path, the cloth cache, the geometry-nodes bake folder, the proxy folder, the movie strip, the sound strip, the movie clip (recorded as a full path, as it was loaded) and the Alembic file and its object path all read the same.

*What the file itself held.* The build wrote an uncompressed file; both copies start `BLENDER17-01`, and both are 600,713 bytes with different checksums (`84158f04…` before, `97d57b68…` after). The text was pulled out of both with `strings -a`. Before the plain save:

```
lines beginning //assets\ :
2180://assets\..\assets//TEX.PNG 
2186://assets\tex.png
```

After it, no line begins `//assets\`, and the two places read `//assets/../assets//TEX.PNG ` and `//assets/tex.png`. Of the extracted text, seven lines differed in all, two of them those spellings. No `lib-inside-image` text was in either copy. So the plain save wrote the forward-slash spellings into the file.

*The two ways of listing every recorded place*, `bpy.utils.blend_paths()` and `bpy.data.file_path_map(include_libraries=True)`, compared as collections:

```
=== BUILD RUN, BEFORE ANY RELOAD: the two listings ===
blend_paths strings: 27 | file_path_map strings: 27
blend_paths entries holding a backslash: ['//assets\\..\\assets//TEX.PNG ', '//assets\\tex.png']
file_path_map entries holding a backslash: [('raw-setter', '//assets\\..\\assets//TEX.PNG '), ('rel-backslash', '//assets\\tex.png')]
=== OPENED, NOTHING TOUCHED, NOTHING SAVED: the two listings ===
blend_paths strings: 24 | file_path_map strings: 25
blend_paths entries holding a backslash: []
file_path_map entries holding a backslash: []
   only file_path_map: '//lib.blend'
```

and after the plain save the same as the line before. So the two routes spelled each place the same way as each other in both processes: with backslashes in the process that built the file, with forward slashes once it had been opened from disk. They did not return the same collection after reopening: `file_path_map` held one more `//lib.blend`.

**Entry 50 — saving somewhere else, and in the same folder under another name.** The folder of entry 49, after its plain save, was copied to `<W>/e50/proj`, with `outer.png` copied to `<W>/e50`. Each variant below started from that `main.blend`, was run with no window, and its result was opened in a fresh run that printed every recorded place and then each image's size (asking for the size loads the picture). The full path and the movie clip's path still name the entry-49 folder `<W>/q20/proj`, which was still there.

*A — `bpy.ops.wm.save_as_mainfile(filepath='<W>/e50/other/main.blend')`, nothing else passed.* The first try, before the folder `<W>/e50/other` existed, failed and wrote nothing: `RuntimeError: Error: Cannot open file <W>/e50/other/main.blend@ for writing: No such file or directory`. With the folder made first, it returned `{'FINISHED'}`. Every relative place a file is read from came back rewritten, and the ones holding `..` or a doubled separator came back shortened:

```
< cachefile[cube.abc].filepath	'//assets/cube.abc'
> cachefile[cube.abc].filepath	'//../proj/assets/cube.abc'
< font[Arial Regular].filepath	'//assets/testfont.ttf'
> font[Arial Regular].filepath	'//../proj/assets/testfont.ttf'
< iesnode[IESLight].filepath	'//assets/test.ies'
> iesnode[IESLight].filepath	'//../proj/assets/test.ies'
< image[missing].filepath	'//nowhere/missing.png'
> image[missing].filepath	'//../proj/nowhere/missing.png'
< image[nonlatin].filepath	'//Тестовая папка/текстура_日本語.png'
> image[nonlatin].filepath	'//../proj/Тестовая папка/текстура_日本語.png'
< image[raw-setter].filepath	'//assets/../assets//TEX.PNG '
> image[raw-setter].filepath	'//../proj/assets/TEX.PNG '
< image[rel-backslash].filepath	'//assets/tex.png'
> image[rel-backslash].filepath	'//../proj/assets/tex.png'
< image[rel-dotdot].filepath	'//assets/../assets/tex.png'
> image[rel-dotdot].filepath	'//../proj/assets/tex.png'
< image[rel-doubleslash].filepath	'//assets//tex.png'
> image[rel-doubleslash].filepath	'//../proj/assets/tex.png'
< image[rel-plain].filepath	'//assets/tex.png'
> image[rel-plain].filepath	'//../proj/assets/tex.png'
< image[space-name].filepath	'//assets/two words.png'
> image[space-name].filepath	'//../proj/assets/two words.png'
< image[trailing-space].filepath	'//assets/tex.png '
> image[trailing-space].filepath	'//../proj/assets/tex.png '
< image[unused-fake].filepath	'//assets/tex.png'
> image[unused-fake].filepath	'//../proj/assets/tex.png'
< image[wrong-case].filepath	'//assets/TEX.PNG'
> image[wrong-case].filepath	'//../proj/assets/TEX.PNG'
< library[lib.blend].filepath	'//lib.blend'
> library[lib.blend].filepath	'//../proj/lib.blend'
< oslnode[OSLMat].filepath	'//assets/test.osl'
> oslnode[OSLMat].filepath	'//../proj/assets/test.osl'
< sound[beep.wav].filepath	'//assets/beep.wav'
> sound[beep.wav].filepath	'//../proj/assets/beep.wav'
< strip[Beep].sound.filepath	'//assets/beep.wav'
> strip[Beep].sound.filepath	'//../proj/assets/beep.wav'
< strip[Movie].filepath	'//assets/clip_0001-0003.mp4'
> strip[Movie].filepath	'//../proj/assets/clip_0001-0003.mp4'
< text[notes.txt].filepath	'//assets/notes.txt'
> text[notes.txt].filepath	'//../proj/assets/notes.txt'
< volume[Vol].filepath	'//assets/smoke.vdb'
> volume[Vol].filepath	'//../proj/assets/smoke.vdb'
```

(each image's `filepath_raw` moved with its `filepath`, letter for letter). The trailing spaces and the capitals were kept. Not in that list, and so printed with the same text as before: the full path `<W>/q20/proj/assets/tex.png`, the movie clip's full path, `up-one`'s `//../outer.png` (the new folder sits beside `proj`, one level under the same `outer.png`), the Alembic object path `/AbcCube/AbcCubeMesh`, and every place a file is written to — `scene[Scene].render.filepath '//out/{blend_name}_####.png'`, `scene[Scene].bake.filepath '//bake/baked.png'`, `fileoutput[File Output].directory '//comp_out/{blend_name}/'`, `pointcache[ClothObj].filepath '//cache/'`, `gnbake[GNObj].bake_directory '//gn_bake/'` and `scene[Scene].sequencer.proxy_dir '//proxies/'`. Every image size was the same as in the starting file: `(2, 2)` for the ten that load, `(0, 0)` for `missing`, `trailing-space` and `raw-setter`.

*B — the same into `<W>/e50/other_noremap/` with `relative_remap=False`.* No recorded place changed text. Opening it printed `WARNING Unable to open '<W>/e50/other_noremap/lib.blend': No such file or directory`, `Font file doesn't exist: //assets/testfont.ttf`, `Could not create cache reader for file //assets/cube.abc` and `1 libraries and 1 linked data-blocks are missing`, and the relative images that had loaded read `(0, 0)` instead of `(2, 2)`: `nonlatin`, `rel-backslash`, `rel-dotdot`, `rel-doubleslash`, `rel-plain`, `space-name`, `unused-fake` and `wrong-case`. `abs-plain` and `up-one` still read `(2, 2)`.

*C — `save_as_mainfile` into the same folder as `renamed.blend`, nothing else passed.* No recorded place changed, and every size was the same.

*D — `bpy.ops.wm.save_mainfile(relative_remap=True)` on a copy `selfsave.blend` in the same folder.* `{'FINISHED'}`; it wrote a backup `selfsave.blend1`; no recorded place changed, and every size was the same.

*E — `bpy.ops.wm.save_mainfile(incremental=True)` on a copy `inc.blend`.* It printed `Info: Saved incremental as "inc1.blend"`, `bpy.data.filepath` then read `<W>/e50/proj/inc1.blend`, and `inc.blend` kept the same checksum as the `main.blend` it was copied from. No recorded place in `inc1.blend` or `inc.blend` differed from the starting file, and every size was the same. `inc1.blend`, `renamed.blend` and `selfsave.blend` had the same checksum as one another (`d96edd9b…`), different from the starting file's (`97d57b68…`).

**Entry 51 — Automatically Pack Resources switched on, then a plain save.** The folder of entry 49 was copied to `<W>/e51/proj`.

*The copy with every place that reaches nothing taken out.* A script opened `main.blend`, removed each image, volume, sound and font whose recorded place does not reach a file, and saved the rest as `clean.blend`: `removed, reaching nothing: ["images[missing] '//nowhere/missing.png'", "images[raw-setter] '//assets/../assets//TEX.PNG '", "images[trailing-space] '//assets/tex.png '", "volumes[Vol] '//assets/smoke.vdb'"]`. A fresh run opened `clean.blend`, printed `use_autopack before: False`, set `bpy.data.use_autopack = True`, and called `bpy.ops.wm.save_mainfile()`:

```
Info: Packed 12 file(s)
Info: Saved "clean.blend"
save_mainfile() -> {'FINISHED'}
```

Ten images, the font and the sound then read packed. Opened again in a fresh run, the file read `use_autopack: True`, and the only lines that differed from the reading before the save were the ten images' packed marks, `False` to `True`; every recorded text was the same, letter for letter, including `'//assets/../assets/tex.png'`, `'//assets//tex.png'`, `'//assets/TEX.PNG'` and the full path. The movie clip, the text, the Alembic file, the library, and the OSL and IES nodes were not in the packed list.

*The full file.* The same steps on the untouched `main.blend`:

```
Error: Unable to pack file, source path '<W>/e51/proj/nowhere/missing.png' not found
Error: Unable to pack file, source path '<W>/e51/proj/assets/TEX.PNG ' not found
Error: Unable to pack file, source path '<W>/e51/proj/assets/tex.png ' not found
Error: Unable to pack file, source path '<W>/e51/proj/assets/smoke.vdb' not found
Info: Packed 16 file(s)
Info: Saved "main.blend"
save_mainfile() RAISED RuntimeError: Error: Unable to pack file, source path '<W>/e51/proj/nowhere/missing.png' not found
```

followed by the other three lines again; the Blender run itself ended with exit code 0. The command raised, and the file was written all the same: `main.blend` went from 600,713 bytes to 1,400,120 with a new checksum, and a backup `main.blend1` appeared beside it. Opened again in a fresh run, it read `use_autopack: True`, printed no warning or error, and held twelve packed items — the same ten images, the font and the sound — with `missing`, `raw-setter`, `trailing-space` and the volume not packed. Compared with `main.blend1` (the file as it was before this save), the only differences were the ten images' packed marks; every recorded text was the same.

**Entry 52 — the relative-paths preference switched off.** A settings folder of its own, `<W>/res-e52`, was given to Blender through `BLENDER_USER_RESOURCES`, and a run without `--factory-startup` printed `use_relative_paths before: True`, set it off, and saved the preferences: `Writing user preferences: "<W>/res-e52/config/userpref.blend"`, `Info: Preferences saved`, `use_relative_paths now: False`. A copy of entry 49's folder in `<W>/e52/proj` was then opened with that settings folder and without `--factory-startup`; the run printed `use_relative_paths read at start: False`, every recorded place, and `bpy.ops.wm.save_mainfile()` returned `{'FINISHED'}` with `Info: Saved "main.blend"`. A fresh run with the same settings folder read the preference as `False` again and printed every place. The 72 lines printed before the save and after it were the same, and the same as the starting file's in entry 50.

**Entry 53 — the render output given on the command line, then a plain save.** Two copies of entry 49's folder, `<W>/e53/before/proj` and `<W>/e53/after/proj`, each holding a `main.blend` whose render output reads `'//out/{blend_name}_####.png'`. Each was run with no window and a script that printed the render output held in memory and called `bpy.ops.wm.save_mainfile()`; a fresh run then opened the saved file and printed the render output it read.

*`-o` placed before the file name:* `"$B53" --background --factory-startup -o <W>/e53/before_ <W>/e53/before/proj/main.blend --python q20b.py …`

```
render filepath in memory = '//out/{blend_name}_####.png'
Info: Saved "main.blend"
save_mainfile() -> {'FINISHED'}
render filepath read from the saved file = '//out/{blend_name}_####.png'
```

*`-o` placed after the file name:* `"$B53" --background --factory-startup <W>/e53/after/proj/main.blend -o <W>/e53/after_ --python q20b.py …`

```
render filepath in memory = '<W>/e53/after_'
Info: Saved "main.blend"
save_mainfile() -> {'FINISHED'}
render filepath read from the saved file = '<W>/e53/after_'
```

**Entry 54 — through the window: the files Blender writes by itself.** A copy of entry 49's folder in `<W>/e54/proj`, with `TMPDIR` pointed at an empty `<W>/e54/tmp/`. `main.blend` was opened with its window from the terminal, under `--factory-startup` and `--enable-event-simulate`, and a script drove it by timers: it closed the first-run splash with a simulated Escape, moved the text object `Txt` one unit along X and pushed an undo step, set the auto-save interval to one minute, paused 75 seconds, listed the temporary folder, and quit with `bpy.ops.wm.quit_blender()`. The window printed:

```
bpy.app.tempdir: <W>/e54/tmp/blender_3uob0v/
use_auto_save_temporary_files: True | auto_save_time: 2
global undo (use_global_undo): True
temporary folder at start: ['blender_3uob0v']
Txt location before: (0.0, 0.0, 0.0)
undo_push -> {'FINISHED'}
Txt location after: (1.0, 0.0, 0.0) | is_dirty: True
auto_save_time set to: 1 minute(s)
temporary folder after the 75-second pause: ['blender_3uob0v', 'main_23594_autosave.blend']
is_dirty: True
ALL STEPS DONE
01:20.263  blend            | Saved session recovery to "<W>/e54/tmp/quit.blend"
```

Afterwards `<W>/e54/tmp/` held `main_23594_autosave.blend` (92,265 bytes) and `quit.blend` (92,272 bytes); the `blender_3uob0v` folder was gone. Each was opened with no window and every recorded place printed: the 72 lines from `quit.blend` and from the auto-save were the same as the 72 from `<W>/e54/proj/main.blend`, letter for letter, and both read `Txt location (1.0, 0.0, 0.0)` where `main.blend` reads `(0.0, 0.0, 0.0)`. Opening either printed `WARNING Unable to open '<W>/e54/tmp/lib.blend': No such file or directory`, `ERROR Unable to pack file, source path '<W>/e54/tmp/assets/testfont.ttf' not found`, `Font file doesn't exist: //assets/testfont.ttf`, `Could not create cache reader for file //assets/cube.abc` and `1 libraries and 1 linked data-blocks are missing`, and every image read `(0, 0)` except `abs-plain` and `up-one` (`//../outer.png` from `<W>/e54/tmp` reaches the `outer.png` copied into `<W>/e54`).

**Entry 55 — the shipped settings of the save commands, and saving pictures changed in memory.** A run with no window printed every setting of `bpy.ops.wm.save_mainfile` and `bpy.ops.wm.save_as_mainfile` with its name, default and description. Besides the file-browser settings (`filepath`, `hide_props_region`, `check_existing`, the `filter_…` settings, `filemode`, `display_type`, `sort_method`), the two commands hold:

```
save_mainfile	compress	| name Compress | type BOOLEAN | default False | Write compressed .blend file
save_mainfile	relative_remap	| name Remap Relative | type BOOLEAN | default False | Remap relative paths when saving to a different directory
save_mainfile	exit	| name Exit | type BOOLEAN | default False | Exit Blender after saving
save_mainfile	incremental	| name Incremental | type BOOLEAN | default False | Save the current Blender file with a numerically incremented name that does not overwrite any existing files
save_mainfile	show_save_modified_images_dialog	| name Show Save Modified Images Dialog | type BOOLEAN | default False | Show a popup dialog to save modified images before saving the blend file
save_as_mainfile	compress	| name Compress | type BOOLEAN | default False | Write compressed .blend file
save_as_mainfile	relative_remap	| name Remap Relative | type BOOLEAN | default True | Remap relative paths when saving to a different directory
save_as_mainfile	copy	| name Save Copy | type BOOLEAN | default False | Save a copy of the actual working state but does not make saved file active
save_as_mainfile	show_save_modified_images_dialog	| name Show Save Modified Images Dialog | type BOOLEAN | default False | Show a popup dialog to save modified images before saving the blend file
```

The preferences, under `--factory-startup`, read:

```
preference filepaths.use_relative_paths	True
preference filepaths.use_file_compression	True
preference filepaths.save_version	1 | type INT | name 'Save Versions' | description 'The number of old versions to maintain in the current directory, when manually saving'
preference filepaths.save_modified_images	'ASK' | type ENUM | name 'Save Modified Images' | description 'How modified images should be handled when saving the .blend file' | choices [('ASK', 'Ask Every Time', 'Show dialog to save modified images when saving the .blend file'), ('ALWAYS_SAVE', 'Always Save', 'Always save modified images when saving the .blend file'), ('NEVER_SAVE', 'Never Save', 'Never save modified images when saving the .blend file')]
```

(The first try at this listing stopped on `AttributeError: 'Preferences' object has no attribute 'use_preferences_save'` after the two `filepaths` lines; the script was changed to read each setting inside a guard and run again.)

Then, three times over in three separate folders, a file `paint.blend` was built holding one image recorded as `//assets/paint.png`, a 4 by 4 grey PNG. A fresh run opened it with no window, turned every pixel red in memory (`is_dirty: True`), set `save_modified_images`, paused 1.2 seconds, called `bpy.ops.wm.save_mainfile()`, and compared the PNG's modification time and checksum from before and after; a further run reopened the file and read the first pixel.

| `save_modified_images` | the save | image recorded after | `is_dirty` after | `paint.png` time changed / contents changed | first pixel on reopening |
|---|---|---|---|---|---|
| `ASK` | `{'FINISHED'}` | `'//assets/paint.png'` | `True` | `False` / `False` | `(0.502, 0.502, 0.502, 1.0)` |
| `ALWAYS_SAVE` | `{'FINISHED'}` | `'//assets/paint.png'` | `False` | `True` / `True` | `(1.0, 0.0, 0.0, 1.0)` |
| `NEVER_SAVE` | `{'FINISHED'}` | `'//assets/paint.png'` | `True` | `False` / `False` | `(0.502, 0.502, 0.502, 1.0)` |

Nothing stopped or waited at `ASK`, and none of the three left the image packed.

**Entry 56 — a plain save of a file, read against the library it links.** A copy of entry 49's folder in `<W>/e56/proj`. The size, modification time and checksum of `lib.blend` and `main.blend` were read; after a two-second pause a run with no window opened `main.blend` (`Read library: '<W>/e56/proj/lib.blend', '//lib.blend', parent '<direct>'`) and called `bpy.ops.wm.save_mainfile()` (`Info: Saved "main.blend"`, `{'FINISHED'}`); then they were read again.

```
BEFORE
<W>/e56/proj/lib.blend | size 85950 | modified 2026-10-17 08:17:28 (1792214248) | changed 2026-10-17 08:33:23
<W>/e56/proj/main.blend | size 600713 | modified 2026-10-17 08:17:49 (1792214269) | changed 2026-10-17 08:33:23
b123fcffece68271456c645821a09d28471d5236d6eedbe96b1e30bb40a5e0f8  <W>/e56/proj/lib.blend
97d57b688c43c576a0369c15229f265ee889b80aeeb85f9ceb0380b01adb7e35  <W>/e56/proj/main.blend
AFTER
<W>/e56/proj/lib.blend | size 85950 | modified 2026-10-17 08:17:28 (1792214248) | changed 2026-10-17 08:33:23
<W>/e56/proj/main.blend | size 600713 | modified 2026-10-17 08:33:25 (1792215205) | changed 2026-10-17 08:33:25
b123fcffece68271456c645821a09d28471d5236d6eedbe96b1e30bb40a5e0f8  <W>/e56/proj/lib.blend
336d903f59b504d83df2fdfa7e0d064f7594fde5f6adca2fb1fafa30a7b0ab7f  <W>/e56/proj/main.blend
```

`lib.blend`'s size, times and checksum were the same after the save; `main.blend` had a new time and checksum, the same size, and a backup `main.blend1` beside it.

**Standing lines, as the hand tests leave them.** *Driven from outside:* every run here except entry 54 was made with no window, and the plain save, the saves to other places, the autopack save, the command-line output and the saving of a changed picture all behaved as above from a script; entry 54 needed a window, because the quit file and the auto-save were written by the window's own session. *Operating system:* macOS only, on the machine named under question 1. What a Windows or Linux Blender writes was not run in this sitting — see the* Cannot be run *section of `Audits/Blender Research/Dependencies/Hand tests — 5.3.md`.

---

## Tests only a running Blender 5.3 can settle

Each test needs a scratch folder with a few small files. Use copies only. Driven via terminal or Python console.

4. **Exit code with missing files (Q12).** After each render above, check the command's exit code: `echo %ERRORLEVEL%` on Windows. *(Pruned 2026-10-17: the macOS exit codes were read for a missing image, font, library and indirect library, and for the kinds in hand-test entry 40, and are recorded under question 12. Windows and Linux are open: no machine running either was available — see the* Cannot be run *section of `Audits/Blender Research/Dependencies/Hand tests — 5.3.md`.)*
6. **Letter case of a recorded path (Q8).** On a Linux machine, set an image path to the right file name but with different capital letters. Render and see whether the image is found. Repeat on Windows. *(Pruned 2026-10-17: the macOS half was run as hand-test entry 16, on the Mac's ordinary disk and on a case-sensitive disk image, and is recorded under question 8. The Linux and Windows halves are open: no machine running either was available — see the* Cannot be run *section of `Audits/Blender Research/Dependencies/Hand tests — 5.3.md`.)*
7. **Windows path opened elsewhere (Q6).** On Windows, set one image to an absolute path with a drive letter and one to a relative `//` path. Save. Open the file on macOS or Linux with the image copied next to it. Run `print(bpy.utils.blend_paths())`, then render, and note which image is found and how each path is shown. *(Left standing 2026-10-17: it needs a file saved by a Windows Blender, and no Windows machine was available — see the* Cannot be run *section of `Audits/Blender Research/Dependencies/Hand tests — 5.3.md`.)*
8. **Non-Latin file names (Q9).** Put an image named with Cyrillic or Japanese letters next to a `.blend` file and use it with a relative path. Save, then copy the folder to a computer running a different operating system. Render there and see whether the image is found. *(Pruned 2026-10-17: the same file was read, rendered and moved on the Mac as hand-test entry 19, recorded under question 9. The move to another operating system is open: no second computer was available — see the* Cannot be run *section of `Audits/Blender Research/Dependencies/Hand tests — 5.3.md`.)*
13. **Network and UNC path resolution (Q7).** Set an image path to a UNC path (`\\server\share\image.png`) on Windows and check whether Blender normalises it, flags it, or converts it to `//`. On Linux, test whether a path starting with `//` is always resolved relative to the `.blend` file or can reach a network share. *(Pruned 2026-10-17: the macOS half was run as hand-test entry 5 and is recorded under question 7. The Windows and Linux halves are open: no machine running either was available — see the* Cannot be run *section of `Audits/Blender Research/Dependencies/Hand tests — 5.3.md`.)*

---

2026-10-16 — Claude Opus 5.5 (Claude Code) — Dependencies, Blender 5.3

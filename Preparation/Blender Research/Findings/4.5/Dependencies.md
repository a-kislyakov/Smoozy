# Dependencies — Blender 4.5

**Sources consulted, and when.** Blender's manual for 4.5 (`manual_4.5`, including its `_images` folder), Blender's Python API documentation for 4.5 (`api_4.5`), Blender's release notes for 4.5 (`release_notes_4.5`), and Cycles' own files for 4.5 (`cycles_4.5`). All of these are the snapshot taken on **2026-07-30**, and that is the date read for every finding below.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

No helper agents were used. Everything below I read myself.

---

## 1. What kinds of thing can a `.blend` file point at outside itself? Give the full list.

**Label: documented** for entries carrying quoted definitions below. Entries named without quoted definitions and the full list asked for are not established.

The Python API documentation marks many properties that hold a file or directory location with the phrase `blend relative // prefix supported`. Searching the whole of `api_4.5` for that phrase returns 41 pages. Taken together with the manual's data-block table, this is the list found by that search:

**Media and content files**

- **Image / movie file** — `Image.filepath`: *"Image/Movie file name"* (`bpy.types.Image`).
- **Sound file** — `Sound.filepath`: *"Sound sample file used by this Sound data-block"* (`bpy.types.Sound`).
- **Movie clip** — `MovieClip.filepath`: *"Filename of the movie or sequence file"* (`bpy.types.MovieClip`).
- **Font file** — `VectorFont.filepath` (`bpy.types.VectorFont`, described as *"Vector font for Text objects"*).
- **Volume file (OpenVDB)** — `Volume.filepath`: *"Volume file used by this Volume data-block"* (`bpy.types.Volume`); and `VolumeGrids.frame_filepath`: *"Volume file used for loading the volume at the current frame…"*.
- **Cache archive (Alembic / USD)** — `CacheFile.filepath`: *"Path to external displacements file"* (the same wording the two modifier paths carry; the same page's `object_paths` is *"Paths of the objects inside the Alembic archive"*); `CacheFileLayer.filepath`: *"Path to the archive"*. *(Citation check 2026-08-05: the quote "Path to the archive" had been attached to both properties; it belongs only to `CacheFileLayer`.)*
- **Text file** — `bpy.data.texts.load(filepath, …)`, *"Add a new text to the main database from a file"*; the `Text` data-block itself has `filepath`: *"Filename of the text file"* (note: the `Text.filepath` entry does **not** carry the `blend relative` marker).

**Another `.blend` file**

- **Library** — `Library.filepath`: *"Path to the library .blend file"*; the class is described as *"External .blend file from which data is linked"* (`bpy.types.Library`).
- **Weak reference to a library** — `LibraryWeakReference.filepath`: *"Path to the library .blend file"*, the class being a *"Weak reference to a data-block in another library .blend file (used to re-use already appended data instead of appending new copies)"* (`bpy.types.ID.library_weak_reference`).

**Shading and lighting side files**

- **OSL shader script** — `ShaderNodeScript.filepath`: *"Shader script path"*.
- **IES light profile** — `ShaderNodeTexIES.filepath`: *"IES light path"*.

**Modifier / simulation external data**

- **Multiresolution external displacements** — `MultiresModifier.filepath`: *"Path to external displacements file"*.
- **Mesh Cache modifier** — `MeshCacheModifier.filepath`: *"Path to external displacements file"*.
- **Ocean modifier bake folder** — `OceanModifier.filepath`: *"Path to a folder to store external baked images"*.
- **Physics point cache** — `PointCache.filepath` and `PointCacheItem.filepath`: *"Cache file path"*.
- **Fluid cache** — `FluidDomainSettings.cache_directory`: *"Directory that contains fluid cache files"*.
- **Geometry-nodes bakes** — `NodesModifier.bake_directory` and `NodesModifierBake.directory`: *"Location on disk where the bake data is stored"*.
- **Dynamic Paint image output** — `DynamicPaintSurface.image_output_path`: *"Directory to save the textures"*.

**Sequencer / clip side files**

- **Image strip folder** — `ImageStrip.directory`; **movie strip file** — `MovieStrip.filepath`.
- **Proxies** — `SequenceEditor.proxy_dir`, `StripProxy.directory` (*"Location to store the proxy files"*), `StripProxy.filepath` (*"Location of custom proxy file"*), `MovieClipProxy.directory` (*"Location to store the proxy files"*).

**Output destinations recorded in the file** (see also question 10)

- **Render output** — `RenderSettings.filepath`: *"Directory/name to save animations, # characters define the position and padding of frame numbers"*.
- **Bake output** — `BakeSettings.filepath`: *"Image filepath to use when saving externally"*.
- **Compositor File Output node** — `CompositorNodeOutputFile.base_path`: *"Base output path for the image"*.
- **Collection exporter** — `CollectionExport.filepath`: *"The file path used for exporting"*.

**Other**

- **Brush icon** — `Brush.icon_filepath`: *"File path to brush icon"*.
- **A file-path node socket** — `NodeSocketStringFilePath.default_value`.
- **Packed-file records** — `ImagePackedFile.filepath` (the API page gives this property no description at all, only its type). *(Citation check 2026-08-05: a gloss reading "the remembered original location of packed image data" was removed — the source does not say it.)*

The manual's data-block table names which of these have a file reference and whether that reference can be pulled inside the file. The "Pack" column is *"File Packing, supports file contents being packed into the blend-file (not applicable for most data-blocks which have no file reference)"*. In that table: **Font** *"References font files"* (Pack ✓); **Image** *"Image files"* (Pack ✓); **Sounds** *"Reference to sound files"* (Pack ✓); **Library** *"References to an external blend-file"* (Pack ✓); **Movie Clip** *"Reference to an image sequence or video file"* (Pack ✗); **Text** *"Text data. Used by Python scripts and OSL shaders"* (Pack ✗); **Cache File** *"Used by Mesh Cache modifiers"* (Pack —).

**Not covered.** This is not the full list the question asks for. It covers properties the API documentation marks with the `blend relative // prefix supported` phrase, plus the manual's data-block table. A path property that omits that marker, as `Text.filepath` does, would not appear in the search. It also does not cover things Blender itself looks up outside the file that are not recorded in the file. Colour-management configuration and the OSL module search path are covered in questions 15 and 5.

**Quoted words, source, address, date.** The quotes above are from Blender's Python API documentation: `https://docs.blender.org/api/4.5/bpy.types.Image.html`, `https://docs.blender.org/api/4.5/bpy.types.Sound.html`, `https://docs.blender.org/api/4.5/bpy.types.MovieClip.html`, `https://docs.blender.org/api/4.5/bpy.types.VectorFont.html`, `https://docs.blender.org/api/4.5/bpy.types.Volume.html`, `https://docs.blender.org/api/4.5/bpy.types.VolumeGrids.html`, `https://docs.blender.org/api/4.5/bpy.types.CacheFile.html`, `https://docs.blender.org/api/4.5/bpy.types.CacheFileLayer.html`, `https://docs.blender.org/api/4.5/bpy.types.Text.html`, `https://docs.blender.org/api/4.5/bpy.types.BlendDataTexts.html`, `https://docs.blender.org/api/4.5/bpy.types.Library.html`, `https://docs.blender.org/api/4.5/bpy.types.LibraryWeakReference.html`, `https://docs.blender.org/api/4.5/bpy.types.ID.html`, `https://docs.blender.org/api/4.5/bpy.types.ShaderNodeScript.html`, `https://docs.blender.org/api/4.5/bpy.types.ShaderNodeTexIES.html`, `https://docs.blender.org/api/4.5/bpy.types.MultiresModifier.html`, `https://docs.blender.org/api/4.5/bpy.types.MeshCacheModifier.html`, `https://docs.blender.org/api/4.5/bpy.types.OceanModifier.html`, `https://docs.blender.org/api/4.5/bpy.types.PointCache.html`, `https://docs.blender.org/api/4.5/bpy.types.PointCacheItem.html`, `https://docs.blender.org/api/4.5/bpy.types.FluidDomainSettings.html`, `https://docs.blender.org/api/4.5/bpy.types.NodesModifier.html`, `https://docs.blender.org/api/4.5/bpy.types.NodesModifierBake.html`, `https://docs.blender.org/api/4.5/bpy.types.DynamicPaintSurface.html`, `https://docs.blender.org/api/4.5/bpy.types.ImageStrip.html`, `https://docs.blender.org/api/4.5/bpy.types.MovieStrip.html`, `https://docs.blender.org/api/4.5/bpy.types.SequenceEditor.html`, `https://docs.blender.org/api/4.5/bpy.types.StripProxy.html`, `https://docs.blender.org/api/4.5/bpy.types.MovieClipProxy.html`, `https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html`, `https://docs.blender.org/api/4.5/bpy.types.BakeSettings.html`, `https://docs.blender.org/api/4.5/bpy.types.CompositorNodeOutputFile.html`, `https://docs.blender.org/api/4.5/bpy.types.CollectionExport.html`, `https://docs.blender.org/api/4.5/bpy.types.Brush.html`, `https://docs.blender.org/api/4.5/bpy.types.NodeSocketStringFilePath.html`, and `https://docs.blender.org/api/4.5/bpy.types.ImagePackedFile.html`. The data-block table quotes are from Blender's manual, `https://docs.blender.org/manual/en/4.5/files/data_blocks.html`. Date read: 2026-07-30.

**Outside the interface.** The sources do not say that the list differs when Blender is driven from outside its own window. The one place the sources directly cover is the render output path, which the command line can set: *"-o, --render-output <path> … Set the render path and file name. Use // at the start of the path to render relative to the blend-file."* (manual, `https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html`).

**Operating system.** The sources do not say this list differs by operating system.


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

**Both standing lines, brought up to date by the Linux tests.** *Driven from outside:* every reading above was taken with Blender running with no window unless it says otherwise, so it holds outside the interface. *Operating system:* **now settled on two operating systems, macOS and Linux, and each is its own result** — what they agree on and where they differ is said above. **Windows is still untried: no Windows machine has run it.** **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

---

## 2. What kinds of library can a file point at, including one that does not sit on the same computer?

**Ambiguity.** "Library" has two meanings in Blender's own words, and the question does not say which. I answer the reading I think is meant — a *library* in the sense of the `Library` data-block, the thing a `.blend` actually points at — and I also report what the sources say about *asset libraries*, which is the other reading.

**Label: documented** for what a library is; **not in the sources** for a library that does not sit on the same computer.

A file points at exactly one kind of library: another `.blend` file. `bpy.types.Library` is *"External .blend file from which data is linked"*, and its `filepath` is *"Path to the library .blend file"*. The manual: *"Link creates a reference to data in a source file such that changes made there will be reflected in the current file the next time it is reloaded."* Libraries are listed in the Outliner: *"They're also listed in the Outliner's Blender File Display Mode, along with the path of the blend-file they originate from."*

A second, weaker kind of pointer at a `.blend` exists: `ID.library_weak_reference`, *"Weak reference to a data-block in another library .blend file (used to re-use already appended data instead of appending new copies)"*.

The other reading — *asset library* — is not something a `.blend` points at. It is a setting in Preferences: *"An asset library is a directory on your drive that is registered in the Preferences as an asset library. Registering it means that you give the library a name (like "Sprite Fright") and the location on drive (like /home/sybren/projects/sprite-fright/assets)."* In the Asset Browser the choices are *"All Libraries"*, *"Current File"*, *"Essentials"* (*"Show the catalogs that come bundled with Blender."*), plus *"Any libraries that you added in the File Path Preferences"*. The word "drive" is used throughout; nothing describes a library reached over a network, by address, or on another machine.

**Not in the sources: a library that does not sit on the same computer.** I searched `manual_4.5`, `api_4.5` and `release_notes_4.5` for: `UNC`, `drive letter`, `network path`, `network drive`, `network file system`, `file server`, `shared directory`, `remote`, `URL`, `smb`, `nfs`, `mounted`, `\\server`, `C:\`, `D:\`. The only hits with any bearing are two, and neither is about a library a `.blend` points at:

- The render-output page's render-farm hint: *"Set up a shared directory over a network file system."* (about where renders are written, not where libraries are read from).
- The deployment page, about bundling *extensions* — not libraries: *"This repository could be located on a read-only network drive or in a system directory."*

I also read `files/linked_libraries/index.txt`, `files/linked_libraries/link_append.txt`, `files/asset_libraries/introduction.txt`, `files/blend/open_save.txt`, `bpy.types.Library`, `bpy.types.UserAssetLibrary`, `bpy.types.AssetLibraryReference`, `bpy.types.BlendDataLibraries`. None mentions a location off the machine. `UserAssetLibrary.path` is only *"Path to a directory with .blend files to use as an asset library"*.

**Not covered.** Whether a library path that happens to name a network share works is not settled here. The sources describe only "a path"; they neither permit nor forbid a network location. Nothing in the sources says a library can be an address (http and the like), and nothing says it cannot.

**Quoted words, source, address, date.** `https://docs.blender.org/api/4.5/bpy.types.Library.html`, `https://docs.blender.org/api/4.5/bpy.types.ID.html`, `https://docs.blender.org/api/4.5/bpy.types.UserAssetLibrary.html` (API documentation); `https://docs.blender.org/manual/en/4.5/files/linked_libraries/link_append.html`, `https://docs.blender.org/manual/en/4.5/files/asset_libraries/introduction.html`, `https://docs.blender.org/manual/en/4.5/editors/asset_browser.html`, `https://docs.blender.org/manual/en/4.5/render/output/properties/output.html`, `https://docs.blender.org/manual/en/4.5/advanced/deploying_blender.html` (manual). Date read: 2026-07-30.

**Outside the interface.** The sources do not say. Linking has a Python route — `bpy.data.libraries.load(filepath, link=…)`, documented at `https://docs.blender.org/api/4.5/bpy.types.BlendDataLibraries.html` — which takes *"The path to a blend file"* and is not tied to the interface. Whether that path may be anything other than a local path, the sources do not say.

**Operating system.** The sources do not say.


**Observed — hand test entry 1, run 2026-08-05 on the Creator's own computer** (MacBook Pro, M1 Pro chip, macOS 26.5.2, Apple's own graphics), with Blender 4.5.12 LTS started from a terminal with no window, using a throwaway settings folder set through `BLENDER_USER_RESOURCES`. The full list of tests is at `Audits/Blender Research/Dependencies/Hand tests.md`.

**What was done.** `Library.is_archive` and `Library.archive_libraries` were looked for by name, in all four versions.

**What came back.** Neither exists in 4.5.12. Both exist in 5.0.1, 5.1.2 and 5.2.0. So the archive arrangement the newer versions carry is not a thing this version has at all.

**Both standing lines, brought up to date by this test.** *Driven from outside:* the reading was taken with no window. *Operating system:* macOS for this test. The same test was run on a rented Linux machine on 2026-08-06 and that result is recorded below; Windows stays untested.

**What this does not settle, and cannot be settled here.** Whether a library that does not sit on the same computer can be reached. That needs a second computer and a shared place both can reach; the Creator has one computer and no share, so it is in the *cannot be run* section of `Audits/Blender Research/Dependencies/Hand tests.md` and stays unanswered. The one adjacent thing that was tested is in question 7 below: a path naming a volume that is not mounted is stored and resolved unchanged, and the file is simply not found.


**Observed — run again on Linux, 2026-08-06.** The two archive-library names were asked for again in this version on the rented Linux machine, and came back exactly as on the Creator's Mac: absent in 4.5, present in 5.0, 5.1 and 5.2. Nothing about which kinds of library exist differs between the two machines.

**The half about a library that does not sit on the same computer is still not settled**, and now for a narrower reason: there are two machines but no folder on network storage that both can reach. What the Linux machine does with a path naming a volume that is not mounted is the same as the Mac: `/Volumes/NoSuchShare/tex.png` is kept exactly as written, resolved exactly as written, and the file is not found. Report Missing Files names it.

**Both standing lines, brought up to date by the Linux tests.** *Driven from outside:* every reading above was taken with Blender running with no window unless it says otherwise, so it holds outside the interface. *Operating system:* **now settled on two operating systems, macOS and Linux, and each is its own result** — what they agree on and where they differ is said above. **Windows is still untried: no Windows machine has run it.**

---

## 3. Can a file hold the things it depends on inside itself partly rather than wholly, and what does a partly-held file report when it is read?

**Label: documented** for the first half; **not in the sources** for the second half.

Yes — packing is per-file, not all-or-nothing. The manual: *"A single file can be packed by clicking on the little "gift box" icon to the left of its file-path UI widget."* and, for the reverse, *"A single file can be unpacked by clicking on the little "gift box" icon to the left of its file-path UI widget."* The whole-file operators exist alongside it: *"Mark all eligible external resource files used by the blend-file as packed. Actual packing will happen on the next save of the blend-file."*

Two limits are stated. First, some things cannot be packed at all: *"Not all external files can be packed — Some typically heavy external files, like videos from the Sequence Editor or Movie Clips, cannot be packed in a blend-file."* Second, the manual's data-block table marks Movie Clip and Text with ✗ in its "Pack" column while Font, Image, Sounds and Library carry ✓. **Inferred:** a file can therefore combine references held inside it with references that remain outside it.

Packing does not happen at the moment you ask for it: *"Actual packing will happen on the next save of the blend-file."*

**What a partly-held file reports when it is read — not in the sources.** Nothing I found describes any report, message or status a partly-packed file produces at load time. What the sources do describe are three things a reader can ask for or see:

- A per-item marker in the interface: *"You know that a data is packed when you see a little "gift box" icon displayed next to its path."*
- An on-demand check: *"Report Missing Files — This option is useful to check if there are links to unpacked files that no longer exist. After selecting this option, a warning message will appear in the Info editor's header. If no warning is shown, there are no missing external files."* The same operator in the API is `bpy.ops.file.report_missing_files()`, *"Report all missing external files"*.
- Per-item state readable from Python: every packable type exposes `packed_file` (for images, *"First packed file of the image"*, and `packed_files`, *"Collection of packed images"*), and `bpy.data.use_autopack` is *"Automatically pack all external data into .blend file"*.

Where I looked to establish the silence: `files/blend/packed_data.txt`, `interface/window_system/topbar.txt`, `files/blend/open_save.txt`, `editors/info_editor.txt`, `files/data_blocks.txt`, `bpy.types.PackedFile`, `bpy.types.ImagePackedFile`, `bpy.types.BlendData`, `bpy.ops.file`. Searches run across all three documentation sets: `packed`, `Pack Resources`, `unpack`, `missing`, `Report Missing`, `placeholder`, `does not exist`, `no longer exist`, `cannot be loaded`, `failed to`.

**Ambiguity.** "Reported when it is read" could also mean "reported by a person or tool reading the file's contents from outside Blender". Nothing in the sources describes the on-disk layout of a partly-packed file, so that reading is equally unanswered.

**Not covered.** Whether a partly-packed file prints anything to a console at load, whether the render treats a packed and an unpacked copy of the same image identically, and what happens if a file is packed *and* its original still exists but differs — the unpack options mention *"(differs) or (identical) show difference status between the packed version and the one on-drive"*, but only in the context of unpacking, not of loading.

**Quoted words, source, address, date.** Manual, `https://docs.blender.org/manual/en/4.5/files/blend/packed_data.html`, `https://docs.blender.org/manual/en/4.5/interface/window_system/topbar.html`, `https://docs.blender.org/manual/en/4.5/files/data_blocks.html`; API documentation, `https://docs.blender.org/api/4.5/bpy.ops.file.html`, `https://docs.blender.org/api/4.5/bpy.types.Image.html`, `https://docs.blender.org/api/4.5/bpy.types.BlendData.html`. Date read: 2026-07-30.

**Outside the interface.** The packing operators are ordinary operators (`bpy.ops.file.pack_all()`, *"Pack all used external files into this .blend"*; `bpy.ops.file.autopack_toggle()`, *"Automatically pack all external files into the .blend file"*; `bpy.ops.file.unpack_all(method=…)`), so they can be driven from Python. But the two ways of *seeing* partial packing that the manual describes — the gift-box icon and the Info editor warning — are interface things. The sources do not say what, if anything, takes their place when there is no window.

**Operating system.** The sources do not say.


**Observed — hand test entries 1 and 5, run 2026-08-05 on the Creator's own computer** (MacBook Pro, M1 Pro chip, macOS 26.5.2, Apple's own graphics), with Blender 4.5.12 LTS started from a terminal with no window, using a throwaway settings folder set through `BLENDER_USER_RESOURCES`. The full list of tests is at `Audits/Blender Research/Dependencies/Hand tests.md`.

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

`WindowManager.reports` does not exist in this version: reading it raised `AttributeError: 'WindowManager' object has no attribute 'reports'`. It exists in 5.2.0, where its length after the same call was 0 — the warning goes to the terminal, not into that collection.

A Cycles render of the same file finished with exit status 0.

**Both standing lines, brought up to date by this test.** *Driven from outside:* settled. A partly-packed file says nothing of its own accord when opened with no window; Report Missing Files does work with no window and its warning is printed to the terminal, naming the path and the data-block. *Operating system:* macOS for this test. The same test was run on a rented Linux machine on 2026-08-06 and that result is recorded below; Windows stays untested.

**Still open.** What the interface shows — the gift-box icons and where the message lands on screen — is entry 21 on the hand-test list and is the Creator's to answer.


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

**Label: documented.**

Yes, in at least five documented ways.

**Image sequences and UDIM tiles.** `Image.source` — *"Where the image comes from"* — is an enum whose members include: *"SEQUENCE — Image Sequence – Multiple image files, as a sequence."*, *"MOVIE — Movie – Movie file."*, and *"TILED — UDIM Tiles – Tiled UDIM image texture."* For UDIM, one recorded path stands for many files by way of substitution tokens: *"Substitution tokens are special sequences of characters in a filename that can be replaced with more meaningful and context aware information… `<UDIM>`: A 4-digit notation calculated as 1001 + u-tile + v-tile \* 10. `<UVTILE>`: A notation defined as u(u-tile + 1)\_v(v-tile + 1)."* with the worked example *"monster-basecolor.<UDIM>.png will load/save files like monster-basecolor.1021.png etc."*

For sequences, the manual states the naming convention plainly in the sequencer section: *"The option detects the frame range of opened images using Blender's frame naming scheme (filename + frame number + .extension) and makes an image sequence with all files in between even if they are missing."*

**Volume (OpenVDB) sequences.** *"Sequence — Loads further VDB files, one for each frame in an animation. Much like with image sequences, all the files should have a numerical suffix in their name; so if you selected smoke-000.vdb in the File Path, there should be a smoke-001.vdb, a smoke-002.vdb and so on."*

**Cache archives.** `CacheFile.is_sequence` is *"Whether the cache is separated in a series of files"*, and `CacheFile.frame` is *"The time to use for looking up the data in the cache file, or to determine which file to use in a file sequence"*. Separately, a single `CacheFile` holds `layers`, each a `CacheFileLayer` with its own *"Path to the archive"* — so one cache reference can mean several archives at once, not only over time.

**Render output.** One recorded output path stands for a whole frame range: *"When rendering an animation, the frame number is appended at the end of the file name with four padded zeros (e.g. image0001.png). You can set a custom padding size by adding the appropriate number of # anywhere in the file name (e.g. image\_##\_test.png translates to image\_01\_test.png)."*

**Multiple views.** With stereo/multi-view, *"By default the system saves each view as an individual file, thus generating as many files as views to be rendered."* Each view carries `SceneRenderView.file_suffix`, *"Suffix added to the render images for this view"*, and `RenderSettings.frame_path()` takes a `view` argument, *"The name of the view to use to replace the "%" chars"*.

**Not covered.** How Blender decides the numeric part of an image-sequence filename in the general (non-sequencer) case; whether a `<UDIM>` token may be used anywhere other than image paths; and what happens if only some members of a sequence exist (except in the sequencer, where placeholders are documented — see question 12).

**Quoted words, source, address, date.** API documentation: `https://docs.blender.org/api/4.5/bpy.types.Image.html`, `https://docs.blender.org/api/4.5/bpy.types.CacheFile.html`, `https://docs.blender.org/api/4.5/bpy.types.CacheFileLayer.html`, `https://docs.blender.org/api/4.5/bpy.types.SceneRenderView.html`, `https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html`. Manual: `https://docs.blender.org/manual/en/4.5/modeling/meshes/uv/workflows/udims.html`, `https://docs.blender.org/manual/en/4.5/video_editing/edit/montage/strips/image.html`, `https://docs.blender.org/manual/en/4.5/modeling/volumes/properties.html`, `https://docs.blender.org/manual/en/4.5/render/output/properties/output.html`, `https://docs.blender.org/manual/en/4.5/render/output/properties/stereoscopy/usage.html`. Date read: 2026-07-30.

**Outside the interface.** The `#` padding behaviour is documented for the command line too: *"The # characters are replaced by the frame number, and used to define zero padding. animation\_##\_test.png becomes animation\_01\_test.png … When the filename does not contain #, the suffix #### is added to the filename."* (manual, command line arguments). The sources do not say whether sequence or UDIM expansion differs when there is no window.

**Operating system.** The sources do not say.


**Observed — hand test entries 4, 15 and 16, run 2026-08-05 on the Creator's own computer** (MacBook Pro, M1 Pro chip, macOS 26.5.2, Apple's own graphics), with Blender 4.5.12 LTS started from a terminal with no window, using a throwaway settings folder set through `BLENDER_USER_RESOURCES`. The full list of tests is at `Audits/Blender Research/Dependencies/Hand tests.md`.

**What was done.** Two UDIM tiles `tiles/tile.1001.png` and `tiles/tile.1002.png`, and a three-frame sequence `seq/shot_0001.png` to `shot_0003.png`, were written to disk. One image data-block was loaded from `tile.1001.png` with `source` set to `TILED`, another from `shot_0001.png` with `source` set to `SEQUENCE`. Their stored paths and tile numbers were printed, then `bpy.utils.blend_paths()`, then `bpy.data.file_path_map()`, then a walk of `bpy.data.file_path_foreach`.

**What came back.**

```
   image=shot_0001.png  source=SEQUENCE  stored='//seq/shot_0001.png' tiles=[1001]
   image=tile.1001.png  source=TILED     stored='//tiles/tile.<UDIM>.png' tiles=[1001, 1002]
```

Blender rewrote the tiled image's path into the `<UDIM>` token form itself, and one recorded path stood for two files. `blend_paths()` and `file_path_map()` both returned the un-expanded text.

**Expanding one path into the files it stands for cannot be asked for in this version.** `bpy.data.file_path_foreach` does not exist: the call raised `AttributeError: 'BlendData' object has no attribute 'file_path_foreach'`. It exists in 5.0.1, 5.1.2 and 5.2.0, and only 5.2.0 can expand.

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

`blend_paths()` and `file_path_map()` both returned the un-expanded text. In this version `bpy.data.file_path_foreach` does not exist and the call raised `AttributeError: 'BlendData' object has no attribute 'file_path_foreach'`, the same as on the Mac.

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

**Label: documented** for the seven forms below; **not in the sources** for `~`, environment-variable expansion and path separators.

Six ways are documented, and they can combine.

**1. Absolute.** *"Make Paths Absolute — Make all paths to external files absolute (= full path from the system's root)."*

**2. Relative to the blend-file, written with a double slash.** *"When you specify one of these external files, the default option is to make the path relative. Blender stores a partial path evaluated relative to the directory location of the referencing blend-file."* and *"When relative paths are supported, the File Browser provides a Relative Path checkbox, when entering the path into a text field, use a double slash prefix (//) to make it so."* The Preferences page states the same shorthand: *"The default path // refers to the folder of the currently open blend-file"*. There is one stated restriction: *"You cannot use relative paths into a new untitled blend-file. Save it before linking to external files."*

**3. With `#` frame padding.** *"You can set a custom padding size by adding the appropriate number of # anywhere in the file name (e.g. image\_##\_test.png translates to image\_01\_test.png)."*

**4. With `<UDIM>` / `<UVTILE>` tile tokens.** *"tokens are identified by being text wrapped in angle bracket characters"* — see question 4 for the full quote.

**5. With `%` for view names.** `RenderSettings.frame_path(… view='')`: *"View, The name of the view to use to replace the "%" chars"*.

**6. With template expressions in curly braces (new in 4.5).** *"Path templates substitute template expressions (written {abcd}, where "abcd" is a variable name) in a filepath when the path is used."* The available variables are `blend_name` (*"The current blend file's name (without the .blend)"*), `fps`, `resolution_x` / `resolution_y`. Format specifiers may follow a colon — *"dance\_{fps:###}.png: format as an integer with at least 3 digits."* — and literal braces are escaped by doubling: *"{{ translates to a single { in the final path."* Substitution is late: *"This substitution happens internally at time of use (e.g. while rendering), so the template syntax will remain as-is in the path field."* The reach is narrow: *"Currently path templates are only supported for the render output path in Scene properties and the output paths in the compositor's File Output node."*

The 4.5 release notes record this as an addition: *"Some file paths in Blender can now contain "template expressions" like `{blend_name}` and `{resolution_x}`, which get replaced by the values they reference when the path is used."* and *"Currently this is only enabled on the Scene's render output path and the output paths of the compositor's File Output node."*

**A seventh, for one specific case only: a bare module name.** For OSL scripts, *"The third option is to specify just the module name, which will be looked up in the shader search path."* Cycles' own code confirms this branch: `elif os.path.dirname(filepath) == "": # module in search path` (`cycles_4.5/osl.py`, line 187–190). **Label for this sub-point: documented (manual) and observed (Cycles file).**

**Length limits.** The manual's working limits page gives, under "Text Fields": `directory: 767`, `file-name: 255`, `file-path: 1023`, `identifier: 63`, with the note *"Multi-byte encoding means some Unicode characters use more than a single ASCII character."*

**Not covered.** Whether `~` (home directory) or environment variables are expanded inside a recorded path — I found nothing either way (see below). Whether backslashes may be used as separators in a recorded path. Which separator character is written into the file.

**Where I looked and found nothing** on `~`, environment-variable expansion and separators inside recorded paths: searched all of `manual_4.5` and `api_4.5` for `double slash`, `backslash`, `forward slash`, `path separator`, `os.sep`, `tilde`, `~/`. The only relevant hits were the double-slash sentence quoted above and `bpy.path.native_pathsep(path)`, *"Replace the path separator with the systems native os.sep"* — a Python helper, not a statement about what Blender stores.

**Quoted words, source, address, date.** Manual: `https://docs.blender.org/manual/en/4.5/files/file_paths.html`, `https://docs.blender.org/manual/en/4.5/files/blend/open_save.html`, `https://docs.blender.org/manual/en/4.5/interface/window_system/topbar.html`, `https://docs.blender.org/manual/en/4.5/editors/preferences/file_paths.html`, `https://docs.blender.org/manual/en/4.5/render/output/properties/output.html`, `https://docs.blender.org/manual/en/4.5/modeling/meshes/uv/workflows/udims.html`, `https://docs.blender.org/manual/en/4.5/render/shader_nodes/script.html`, `https://docs.blender.org/manual/en/4.5/advanced/limits.html`. API documentation: `https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html`, `https://docs.blender.org/api/4.5/bpy.path.html`. Release notes: `https://developer.blender.org/docs/release_notes/4.5/core/`. Cycles' own file: `cycles_4.5/osl.py`, function `update_external_script`, lines 187–190. Date read: 2026-07-30.

**Outside the interface.** The `//` prefix and templates both work from the command line: *"-o, --render-output <path> … Use // at the start of the path to render relative to the blend-file. You can use path templating features such as {blend_name} in the path."* One consequence is recorded in the 4.5 Python release notes: *"`scene.render.frame_path()` used to be infallible but can now fail if there are errors processing Path Templates in the render output path."* (`https://developer.blender.org/docs/release_notes/4.5/python_api/`). Also from 4.5: *"`bpy.props.StringProperty` with subtype set to `DIR_PATH` or `FILE_PATH` will warn when paths use a blend-file relative `//` prefix unless support is explicitly set, example: `options={'PATH_SUPPORTS_BLEND_RELATIVE'}`."* Note that the *error display* for a bad template is described only for the interface: *"When there are template errors in a path, the path field will be highlighted red in the UI"* — though the manual adds *"if there are errors in the render output path, then rendering an animation will fail with an error message indicating the path errors."*

**Operating system.** The sources do not say the writing forms differ by operating system. The one API note pointing at a difference is `bpy.path.basename(path)`: *"Equivalent to os.path.basename, but skips a "//" prefix. Use for Windows compatibility."*


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

## 6. What does a location recorded on one kind of computer mean on another, including a kind that names its disks?

**Label: not in the sources** for the specific question (drive letters, and what an absolute path from one system means on another). Two things nearby *are* documented.

**What is documented.** The manual says plainly that absolute paths are tied to the machine that made them, and that relative paths are the portable form. From the OBJ and FBX exporter pages, in identical words: *"When referencing paths in exported files you may want some control as to the method used since absolute paths may only be correct on your own system. Relative paths, on the other hand, are more portable but mean that you have to keep your files grouped when moving about on your local file system."* And from the relative-paths section: *"With a relative path, you can move the blend-file to a new location provided the externally linked files are moved along with it. For example, you could send someone a folder that contains a blend-file and a subfolder of external images that it references."*

**What is not documented.** Nothing states what a Windows-style path with a drive letter means when the file is opened on macOS or Linux, or the reverse; nothing states whether Blender rewrites separators on load; nothing names drive letters at all in connection with recorded paths.

**Where I looked.** Searched all of `manual_4.5`, `api_4.5` and `release_notes_4.5` for: `drive letter`, `C:\`, `D:\`, `UNC`, `\\server`, `cross-platform`, `different operating system`, `portable`, `path separator`, `backslash`, `native_pathsep`. Read in full: `files/blend/open_save.txt` (the Relative Paths section), `files/file_paths.txt`, `editors/preferences/file_paths.txt`, `editors/file_browser.txt`, `interface/window_system/topbar.txt`, `advanced/limits.txt`, `api_4.5/bpy.path.txt`, `api_4.5/info_gotchas_file_paths_and_encoding.txt`. The only hits on `C:\` are in unrelated places — the Windows launch instructions (`C:\Program Files\Blender Foundation\Blender\blender.exe`) and the OSL shader search path (`C:\Users\$user\AppData\Roaming\Blender Foundation\Blender\4.5\shaders\`) — neither about a path stored in a `.blend`.

The closest the API comes is `bpy.path.native_pathsep(path)`, *"Replace the path separator with the systems native os.sep"*, and `bpy.path.basename(path)`, *"Equivalent to os.path.basename, but skips a "//" prefix. Use for Windows compatibility."* Both are tools offered to a script writer. Neither says what Blender itself does with a foreign path.

**Not covered.** Everything the question actually asks: the meaning of a drive-lettered path elsewhere, separator translation, and whether Blender fails, ignores, or reinterprets such a path.

**Quoted words, source, address, date.** Manual: `https://docs.blender.org/manual/en/4.5/files/import_export/obj.html`, `https://docs.blender.org/manual/en/4.5/addons/import_export/scene_fbx.html`, `https://docs.blender.org/manual/en/4.5/files/blend/open_save.html`, `https://docs.blender.org/manual/en/4.5/advanced/command_line/launch/windows.html`, `https://docs.blender.org/manual/en/4.5/render/shader_nodes/script.html`. API documentation: `https://docs.blender.org/api/4.5/bpy.path.html`. Date read: 2026-07-30.

**Outside the interface.** The sources do not say.

**Operating system.** This question *is* the operating-system question, and the sources do not answer it. The only operating-system-specific path statements I found anywhere are about Blender's own folders, not about paths stored in a `.blend`: the OSL shader search path (`$HOME/.config/blender/4.5/shaders/` on Linux, `C:\Users\$user\AppData\Roaming\Blender Foundation\Blender\4.5\shaders\` on Windows, `/Users/$USER/Library/Application Support/Blender/4.5/shaders/` on macOS), the note that fonts on Unix are *"typically located under /usr/lib/fonts, or some variant like /usr/lib/X11/fonts"*, and the temporary-directory environment variables *"TEMP: Store temporary files here (MS-Windows)"* and *"TMPDIR: Store temporary files here (UNIX Systems)."*


**Observed — hand test entry 3, run 2026-08-05 on the Creator's own computer** (MacBook Pro, M1 Pro chip, macOS 26.5.2, Apple's own graphics), with Blender 4.5.12 LTS started from a terminal with no window, using a throwaway settings folder set through `BLENDER_USER_RESOURCES`. The full list of tests is at `Audits/Blender Research/Dependencies/Hand tests.md`.

**What was done.** An image data-block was given the location `C:\textures\tex.png` on the Creator's Mac, the file was saved, reopened, and the stored text, `bpy.path.abspath` of it, and the image size after `reload()` were printed. The same was done for `/Volumes/NoSuchShare/tex.png`.

**What came back**, the same in all four versions:

```
NAME=drive-letter   STORED='C:/textures/tex.png'                 ABSPATH='C:/textures/tex.png' SIZE=(0, 0)
NAME=no-such-mount  STORED='/Volumes/NoSuchShare/tex.png'        ABSPATH='/Volumes/NoSuchShare/tex.png' SIZE=(0, 0)
```

So a Mac keeps a drive-lettered path, turns its backslashes into forward slashes when the file is saved, treats what is left as an absolute path, and simply does not find the file. Nothing was printed about it.

**Both standing lines, brought up to date by this test.** *Driven from outside:* settled — all of the above was done with no window. *Operating system:* a second operating system is now reachable and the crossing is recorded below. What is still untried is Windows. The macOS-to-Linux half was made on 2026-08-06 and is recorded below. The Windows half needs a Windows machine and none was rented for it. It is in the *cannot be run* section of `Audits/Blender Research/Dependencies/Hand tests.md` and stays unanswered.


**Observed — the crossing between two computers, run for the first time on 2026-08-06.** This is what the question asks and what no earlier test could reach. A `crossing.blend` was built in this version **on the Creator's Mac** at `/Users/artist/…/crossing/4.5/proj/` and saved with `relative_remap=False` so every path stayed exactly as typed. It was then carried whole to the rented Linux machine and opened there in the same version. Both machines were read with the same script.

**What survived the crossing, and what did not.** The result was the same in all four versions.

```
IMAGE rel          stored='//assets/tex.png'
             abspath='/root/…/crossing/4.5/proj/assets/tex.png'
             exists=True size=(32, 32) has_data=True
IMAGE rel-up       stored='//../outer/tex.png'
             abspath='/root/…/crossing/4.5/proj/../outer/tex.png'
             exists=True size=(32, 32) has_data=True
IMAGE abs-mac      stored='/Users/artist/…/crossing/4.5/proj/assets/tex.png'
             abspath='/Users/artist/…/crossing/4.5/proj/assets/tex.png'
             exists=False size=(0, 0) has_data=False
IMAGE abs-volumes  stored='/Volumes/NoSuchShare/tex.png'
             abspath='/Volumes/NoSuchShare/tex.png'
             exists=False size=(0, 0) has_data=False
IMAGE drive        stored='C:/textures/tex.png'
             abspath='C:/textures/tex.png'
             exists=False size=(0, 0) has_data=False
```

On the Mac, before the file travelled, `abs-mac` read `exists=True size=(32, 32)`.

So: **a path written relative to the blend-file survives the crossing whole** and resolves against the file's new folder on the other computer, including one that climbs above that folder with `//../`. **An absolute path written on a Mac does not survive**: Linux keeps it exactly as written, resolves it exactly as written, does not find the file, and says nothing until asked. Blender does not rewrite it, does not translate it, and does not fail on it — it simply resolves to a place that is not there.

**A path that names a disk survives as text and not as a place.** `/Volumes/NoSuchShare/tex.png` is kept and resolved unchanged on both machines and found on neither. A drive-lettered path typed as `C:\textures\tex.png` is stored `C:/textures/tex.png` on both machines, treated as an ordinary absolute path, and found on neither — though Report Missing Files prints it as `/c/textures/tex.png`, which neither the stored text nor `bpy.path.abspath` agrees with.

**And a render does not refuse a foreign output path.** The same file's render output path was a full Mac path. Rendering it on the Linux machine printed `Saved: '/Users/artist/…/crossing/4.5/renders/frame_0001.png'`, finished with exit status 0, and **created the whole Mac-shaped folder tree at the root of the Linux disk** to do it. Nothing warned. That run was made as the root user, who owns the root of that disk, so what happens for a render user who cannot write there is not established.

**Still not established.** What a Windows Blender writes into a path, and what a path written on Windows means on macOS or Linux, or the reverse. No Windows machine has run it.

**Both standing lines, brought up to date by the Linux tests.** *Driven from outside:* every reading above was taken with Blender running with no window unless it says otherwise, so it holds outside the interface. *Operating system:* **now settled on two operating systems, macOS and Linux, and each is its own result** — what they agree on and where they differ is said above. **Windows is still untried: no Windows machine has run it.**

---

## 7. What does a location pointing somewhere other than the computer the file sits on mean, at either end?

**Label: not in the sources.**

Nothing in the manual, the API documentation, the release notes or Cycles' own files describes a recorded location that points at another computer — no network shares, no UNC paths, no addresses, no statement of what such a path means when read on the machine that hosts it versus on any other.

**Where I looked.** Searched all of `manual_4.5`, `api_4.5` and `release_notes_4.5` for: `UNC`, `network path`, `network drive`, `network file system`, `file server`, `shared directory`, `remote`, `smb`, `nfs`, `mounted`, `URL`, `\\server`, `drive letter`. Read in full: `troubleshooting/network.txt` (it is entirely about a 403 error reaching `extensions.blender.org`, nothing to do with file paths), `advanced/deploying_blender.txt`, `render/output/properties/output.txt`, `files/blend/open_save.txt`, `files/linked_libraries/link_append.txt`, `editors/preferences/file_paths.txt`, `api_4.5/bpy.path.txt`.

**The two adjacent facts I did find**, neither of which answers the question:

- The render-output page's render-farm hint, about *writing*, not about a path recorded in the file: *"Primitive Render Farm — An easy way to get multiple machines to share the rendering workload is to: Set up a shared directory over a network file system. Disable Overwrite, enable Placeholders in the Render Output panel. Start as many machines as you wish rendering to that directory."*
- The deployment page, about *extension repositories*, not about anything a `.blend` points at: *"This repository could be located on a read-only network drive or in a system directory."*

**Ambiguity.** "At either end" I read as: what the path means on the machine holding the file, and what it means on the machine holding the target. The sources answer neither. A second reading — what it means for the file and for the render — is equally unanswered.

**Not covered.** Everything the question asks.

**Quoted words, source, address, date.** Manual: `https://docs.blender.org/manual/en/4.5/render/output/properties/output.html`, `https://docs.blender.org/manual/en/4.5/advanced/deploying_blender.html`, `https://docs.blender.org/manual/en/4.5/troubleshooting/network.html`. Date read: 2026-07-30.

**Outside the interface.** The sources do not say.

**Operating system.** The sources do not say.


**Observed — hand test entry 3, run 2026-08-05 on the Creator's own computer** (MacBook Pro, M1 Pro chip, macOS 26.5.2, Apple's own graphics), with Blender 4.5.12 LTS started from a terminal with no window, using a throwaway settings folder set through `BLENDER_USER_RESOURCES`. The full list of tests is at `Audits/Blender Research/Dependencies/Hand tests.md`.

**What was done.** An image data-block was given the location `/Volumes/NoSuchShare/tex.png` — a path naming a volume that is not mounted on the Creator's Mac — the file was saved, reopened, and the stored text, `bpy.path.abspath` of it, and the image size after `reload()` were printed.

**What came back**, the same in all four versions:

```
NAME=no-such-mount  STORED='/Volumes/NoSuchShare/tex.png'        ABSPATH='/Volumes/NoSuchShare/tex.png' SIZE=(0, 0)
```

The text is kept exactly as written, resolved exactly as written, and the file is not found. Nothing was printed about it.

**Both standing lines, brought up to date by this test.** *Driven from outside:* settled for this narrow case, which was done with no window. *Operating system:* macOS for this test. The same test was run on a rented Linux machine on 2026-08-06 and that result is recorded below; Windows stays untested.

**What this does not settle, and cannot be settled here.** What such a location means at either end when it really does point at another computer. That needs a second computer and a place both can reach; the Creator has one computer and no share. It is in the *cannot be run* section of `Audits/Blender Research/Dependencies/Hand tests.md` and stays unanswered.


**Observed — run again on Linux, 2026-08-06.** A path naming a volume that is not mounted was read again in this version on the rented Linux machine: `/Volumes/NoSuchShare/tex.png` was kept exactly as written, resolved exactly as written, and the file was not found, with nothing printed until Report Missing Files was asked. **The same as on the Creator's Mac.**

**The question itself is still not settled, and the reason has narrowed.** There are now two computers, but no folder on network storage that both can reach, so what a location pointing at the other machine means at either end could not be tried. What a Mac and a Linux machine each do with a path naming a place that is not there is now known and is the same; what either does with a place that *is* there over a network is not.

**Both standing lines, brought up to date by the Linux tests.** *Driven from outside:* every reading above was taken with Blender running with no window unless it says otherwise, so it holds outside the interface. *Operating system:* **now settled on two operating systems, macOS and Linux, and each is its own result** — what they agree on and where they differ is said above. **Windows is still untried: no Windows machine has run it.**

---

## 8. Does the difference between capital and small letters in a name matter, and does it matter the same on every kind of computer?

**Label: inferred**, from one sentence in the API documentation. There is no direct statement in the manual.

The one place the sources speak to this is `bpy.path.resolve_ncase(path)`: *"Resolve a case insensitive path on a case sensitive system, returning a string with the path if found else return the original path."*

**The step I took.** The sentence names two kinds of system — case-sensitive and case-insensitive — and offers a helper to bridge between them. From that I infer that whether capitals matter in a filename depends on the operating system Blender is running on, not on Blender. I also infer that Blender does not always do this bridging on its own, since it would not need to hand a script writer a function to do it if it did. Both steps require the reader to supply something; neither is stated. Anyone wanting a documented answer will not find one.

**What is documented and is a different thing.** Case matters for command-line arguments — *"Arguments are case sensitive! -F and -f are not the same."* — and there is a case-comparison option for file extensions, `bpy.path.ensure_ext(filepath, ext, *, case_sensitive=False)`, *"case_sensitive (bool) – Check for matching case when comparing extensions."* Neither of these is about a filename recorded in a `.blend`.

**Where I looked.** Searched all of `manual_4.5`, `api_4.5` and `release_notes_4.5` for `case sensitive`, `case-sensitive`, `case insensitive`, `case-insensitive`, `sensitiv`. Every other hit is about text search, list sorting, brush pressure, or selection — nothing about file paths. Read: `api_4.5/bpy.path.txt` in full, `files/blend/open_save.txt`, `files/blend/rename.txt`, `interface/window_system/topbar.txt`, `advanced/limits.txt`.

**Not covered.** Whether Blender attempts case-insensitive resolution by itself when a path fails to resolve; whether "Find Missing Files" matches case-insensitively; which operating systems are which (the sources never say macOS, Windows or Linux in this connection); whether data-block names (as opposed to file names) are compared case-sensitively.

**Quoted words, source, address, date.** API documentation, `https://docs.blender.org/api/4.5/bpy.path.html`; manual, `https://docs.blender.org/manual/en/4.5/advanced/command_line/render.html`. Date read: 2026-07-30.

**Outside the interface.** The sources do not say. `bpy.path.resolve_ncase` is a plain Python function, so it is available with or without a window; that is about the function, not about how a render resolves a path.

**Operating system.** The quoted sentence implies a difference between operating systems but names none. So: the sources say it differs, and do not say where.


**Observed — hand test entry 10, run 2026-08-05 on the Creator's own computer** (MacBook Pro, M1 Pro chip, macOS 26.5.2, Apple's own graphics), with Blender 4.5.12 LTS started from a terminal with no window, using a throwaway settings folder set through `BLENDER_USER_RESOURCES`. The full list of tests is at `Audits/Blender Research/Dependencies/Hand tests.md`.

**What was done.** Two places to work were used on the one Mac: its ordinary disk, which does not mind capitals, and a case-sensitive disk image made with `hdiutil create -size 64m -fs "Case-sensitive APFS" -volname SmoozyCS cs.dmg` and attached at a folder of its own with `hdiutil attach cs.dmg -mountpoint <folder> -nobrowse`. On each, a file was saved referencing `assets/Texture.PNG`. The file on disk was then renamed to `texture.png`, the blend-file reopened with no window, and the stored path, `bpy.path.abspath` of it, the image size after `reload()`, `os.path.exists` of the resolved path and `bpy.path.resolve_ncase` of it were printed, followed by `bpy.ops.file.report_missing_files()`.

**What came back**, the same in all four versions. On the ordinary Mac disk:

```
IMAGE stored='//assets/Texture.PNG' abspath='<path>/assets/Texture.PNG' size=(32, 32) has_data=True
      os.path.exists(abspath) = True
      bpy.path.resolve_ncase(abspath) = '<path>/assets/Texture.PNG'
```

with nothing printed by Report Missing Files. On the case-sensitive disk:

```
IMAGE stored='//assets/Texture.PNG' abspath='<path>/assets/Texture.PNG' size=(0, 0) has_data=False
      os.path.exists(abspath) = False
      bpy.path.resolve_ncase(abspath) = '<path>/assets/texture.png'
Warning: Path '<path>/assets/Texture.PNG' not found, from local data-block 'IMTexture.PNG'
```

So capitals matter or not according to the disk, not according to Blender, and Blender does not do the case-insensitive resolution itself when loading — even though `bpy.path.resolve_ncase` finds the file when it is asked directly.

**Both standing lines, brought up to date by this test.** *Driven from outside:* settled — all of it was done with no window. *Operating system:* both kinds of disk were reached on macOS, and a second operating system with a case-sensitive disk was reached on 2026-08-06 and is recorded below. Linux was reached on 2026-08-06 and that result is recorded below. Whether Windows behaves the same is still untested: no Windows machine has run it.


**Observed — the case of a file name, settled on a second kind of disk, 2026-08-06.** This version was run again on the rented Linux machine, whose ordinary disk is ext4 and does tell `Texture.PNG` and `texture.png` apart.

A file built **on the Creator's Mac** referencing `//assets/Texture.PNG`, with the file on disk named `texture.png`, was carried to the Linux machine and opened there. On the Mac it resolves; on Linux it does not:

```
on the Mac    IMAGE case-caps  abspath='…/assets/Texture.PNG'  exists=True  size=(32, 32) has_data=True
                               resolve_ncase='…/assets/Texture.PNG'
on Linux      IMAGE case-caps  abspath='…/assets/Texture.PNG'  exists=False size=(0, 0)   has_data=False
                               resolve_ncase='…/assets/texture.png'
Warning: Path '…/assets/Texture.PNG' not found, from local data-block 'IMcase-caps'
```

So the disk decides and Blender does not, on Linux exactly as on the Mac's own case-sensitive disk image. **Blender does not attempt case-insensitive resolution when it loads a dependency**, even though `bpy.path.resolve_ncase` finds the real small-letter file the moment it is asked directly. **This is a way a file that works on an artist's Mac stops working on a rented Linux machine and nowhere else.**

**And this settles something no version's sources answered and no earlier test reached: whether Find Missing Files matches case-insensitively.** It does not. With a search folder holding a file named `texture.png` and the reference written `Texture.PNG`, `bpy.ops.file.find_missing_files(directory=…, find_all=False)` printed

```
Warning: Could not find 'Texture.PNG' in '/root/…/rescue2/4.5/'
```

and changed nothing. With a search folder holding the right name, the same call repaired every other missing path and **rewrote each one as a full absolute path into the search folder**, leaving paths that already resolved alone. The same in all four versions. The Mac cannot answer this question at all, because on its disk the capitals reference is not missing in the first place.

**Still not established.** Whether Windows behaves like either machine. No Windows machine has run it.

**Both standing lines, brought up to date by the Linux tests.** *Driven from outside:* every reading above was taken with Blender running with no window unless it says otherwise, so it holds outside the interface. *Operating system:* **now settled on two operating systems, macOS and Linux, and each is its own result** — what they agree on and where they differ is said above. **Windows is still untried: no Windows machine has run it.**


**Observed — added by this round's hand tests, run 2026-08-07 on a rented Linux machine** (Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB on driver 580.95.05, an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory, and no display server at all), with Blender 4.5.12 LTS started from a terminal with no window, `--factory-startup`, and a settings folder of its own made first and set through `BLENDER_USER_RESOURCES`. **This is an addition to a document written in an earlier round; nothing above it has been changed.** The full list of tests is at `Audits/Blender Research/Dependencies/Hand tests — added questions.md`.

**What was asked.** The earlier tests on this question all asked about the case of a **file name on disk**. This one asks about the case of a name **inside** the file, which the earlier answer listed as not covered.

**What was done.** A library file was saved holding one object named `Cube`. A second file was built holding three objects named `Cube`, `cube` and `CUBE`, three images named `Texture`, `texture` and `TEXTURE`, and three materials named `Mat`, `mat` and `MAT`, each given a fake user so it would survive being saved. That file was saved, opened again, and every name printed. Each was then looked up by the right spelling, by the wrong capitals, and by a mixed spelling. Finally, from a fresh empty file each time, the library's object was asked for by each of the three spellings, two ways: through `bpy.data.libraries.load(<library>, link=True)` naming `data_to.objects = [<spelling>]`, and through `bpy.ops.wm.link(filepath=<library>/Object/<spelling>, directory=<library>/Object, filename=<spelling>)`.

**What came back**, the same in all four versions:

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
LINK asked 'cube' -> got [None]
LINK asked 'CUBE' -> got [None]
wm.link 'Cube' -> {'FINISHED'} ; objects before [] after ['Cube']
wm.link 'cube' -> {'FINISHED'} ; objects before [] after []
wm.link 'CUBE' -> {'FINISHED'} ; objects before [] after []
```

The `libraries.load` route also printed, for each wrong spelling:

```
UserWarning: load: '/root/dependencies/work/q8/4.5/lib.blend' does not contain objects["cube"]
```

So capitals are told apart exactly inside a file: three names differing only in case all survive being saved and reopened, and a lookup with the wrong capitals finds nothing. Asking a library for a name with the wrong capitals through `libraries.load` gives `None` and a warning naming the file. Asking through `bpy.ops.wm.link` gives `{'FINISHED'}`, prints nothing at all, and links nothing.

**Both standing lines, brought up to date by this test.** *Driven from outside:* settled — all of it was done with no window. *Operating system:* run on Linux here; **the same test was run on the Creator's Mac on 2026-08-07 and that result, together with what the Mac's own disk does with capitals, is recorded below.** Windows is untested because no Windows machine has run it.

**Observed — the Mac half of this round's hand tests, run 2026-08-07 on the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card.** Blender 4.5.12 LTS was started from a terminal with no window, with `--factory-startup` and a settings folder of its own, made before it was used and set through `BLENDER_USER_RESOURCES`; that this took effect was checked rather than assumed, and this Blender answered `RESOURCES …/depwork/res-4.5` and `VERSION 4.5.12 LTS`. **Other topics were running Blender on this same Mac throughout, so nothing was timed and no memory was measured.** **This is an addition; nothing above it has been changed.** The full list of tests is at `Audits/Blender Research/Dependencies/Hand tests — added questions.md`. In the quotations below the working folder's long path is written `…/depwork`; nothing else is altered.

**First, the same test the Linux half ran — the case of a name *inside* the file — run again here.** The library file, the three objects `Cube`, `cube` and `CUBE`, the three images, the three materials, the lookups and the two ways of linking were all built exactly as before. **The Mac answered exactly what Linux answered**, in all four versions:

```
LIB objects: ['Cube']
BEFORE SAVE objects: ['CUBE', 'Cube', 'cube']
AFTER RELOAD objects: ['CUBE', 'Cube', 'cube']
AFTER RELOAD images: ['TEXTURE', 'Texture', 'texture']
AFTER RELOAD mats  : ['MAT', 'Mat', 'mat']
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
   warning: UserWarning: load: '…/depwork/q8/4.5/lib.blend' does not contain objects["cube"]
LINK asked 'cube' -> got [None]
   warning: UserWarning: load: '…/depwork/q8/4.5/lib.blend' does not contain objects["CUBE"]
LINK asked 'CUBE' -> got [None]
wm.link 'Cube' -> {'FINISHED'} ; objects before [] after ['Cube']
wm.link 'cube' -> {'FINISHED'} ; objects before [] after []
wm.link 'CUBE' -> {'FINISHED'} ; objects before [] after []
```

**So inside a file, capitals are told apart exactly on macOS as well.** Three names differing only in case survive being saved and reopened; a lookup with the wrong capitals finds nothing; `libraries.load` asked for the wrong capitals gives `None` and a warning naming the file; and `bpy.ops.wm.link` asked for the wrong capitals answers `{'FINISHED'}`, prints nothing at all, and links nothing. **This is the reading to keep for macOS**, since the two machines were asked separately and each answers only for itself.

**Second, something the earlier tests could not settle from this side, and it is the thing the two machines differ on.** The earlier round showed on a **case-sensitive disk image** made on this Mac, and on the Linux machine's own disk, that a reference with the wrong capitals fails. What had not been asked plainly is what the Mac's **ordinary** disk does when Blender is asked for a name by the wrong capitals — with a control that could fail. One file, `tex.png`, was written into a folder, and then asked for by four spellings from a fresh empty session each time; and one library file, `lib.blend`, likewise:

```
on disk: ['tex.png']
--- an IMAGE asked for by three spellings of a name that exists once, as tex.png ---
  asked tex.png      -> size (32, 32) | has_data True  | filepath 'tex.png'
  asked TEX.PNG      -> size (32, 32) | has_data True  | filepath 'TEX.PNG'
  asked Tex.Png      -> size (32, 32) | has_data True  | filepath 'Tex.Png'
  asked nosuch.png   -> RAISED RuntimeError: nosuch.png': No such file or directory
--- the same LIBRARY asked for by three spellings of lib.blend ---
  asked lib.blend      -> available ['LibCube'] | linked ['LibCube']
  asked LIB.BLEND      -> available ['LibCube'] | linked ['LibCube']
  asked Lib.Blend      -> available ['LibCube'] | linked ['LibCube']
  asked nosuch.blend   -> RAISED OSError: load: …/depwork/q8disk/4.5/assets/nosuch.blend failed to open blend file
```

the same in all four versions. **On the Creator's Mac's own disk a recorded file name is found whatever its capitals**, for an image and for a linked library alike, and the recorded spelling is kept as written. The fourth line of each block is the control that makes the other three mean something: a name that genuinely is not there fails, and says so.

**So the answer to this question really does depend on the disk and not on Blender, and this project's two machines sit on opposite sides of it.** A file written on the Creator's Mac can record `Texture.PNG` for a file named `texture.png` and work perfectly, and the same file on the rented Linux machine will not find it. Nothing inside the file behaves that way: names inside a `.blend` are exact on both machines.

**Both standing lines, brought up to date by these tests.** *Driven from outside:* settled — every reading here was taken with Blender running with no window. *Operating system:* **settled on both of this project's machines, and each is its own result.** macOS, on its ordinary disk, ignores capitals in a file name on disk; Linux does not; and both tell names inside the file apart exactly. **Windows is untried: no Windows machine has run it.**




---

## 9. Do names written in characters outside the Latin alphabet survive being read out of a file, moved to another computer, and found again by a render?

**Label: documented** for how Blender stores strings and paths; **not in the sources** for the round trip the question describes.

**What Blender stores.** The API documentation is explicit and, unusually, warns about the exact trouble spot: *"However, this complicates matters for Blender's Python API because .blend files don't have an explicit encoding. To avoid the problem for Python integration and script authors we have decided that all strings in blend-files must be UTF-8, ASCII compatible. This means assigning strings with different encodings to an object name, for instance, will raise an error."*

**Paths are deliberately exempted from that rule.** *"Paths are an exception to this rule since the existence of non-UTF-8 paths on the user's file system cannot be ignored. This means seemingly harmless expressions can raise errors, e.g:"* followed by the worked example

> `>>> print(bpy.data.filepath)`
> `UnicodeEncodeError: 'ascii' codec can't encode characters in position 10-21: ordinal not in range(128)`

and the advice *"Use os.fsencode() or os.fsdecode() instead of built-in string decoding functions when operating on paths."*

So: a path recorded in a `.blend` is **not** guaranteed to be UTF-8. It is whatever the file system gave, and reading it back out as text can fail.

**Data-block names** (as opposed to paths) do survive as UTF-8, but with a byte budget: *"Data-block names have a fixed length of 63 bytes, i.e. 63 basic ASCII characters, or less when using diacritics or non-latin glyphs (the UTF8 encoding will then typically use more than a byte per character)."* The manual's limits page repeats the caution: *"Multi-byte encoding means some Unicode characters use more than a single ASCII character."*

There is even a repair operator for files that predate the rule: `bpy.ops.wm.blend_strings_utf8_validate()`, *"Check and fix all strings in current .blend file to be valid UTF-8 Unicode (needed for some old, 2.4x area files)"*.

Elsewhere the sources show non-Latin text being handled without comment — the asset catalog example file contains `Ružena` in catalog paths and simple names, and states *"Paths are always interpreted as UTF-8."* (this is about catalog paths, which are not file-system paths).

**Not in the sources: the round trip itself.** Nothing states what happens when a file carrying a non-Latin path is moved to a machine with a different file-system encoding, and nothing states what a render does at that point. The sources describe the storage rule and warn that non-UTF-8 paths exist; they do not follow the story to another computer.

**Where I looked.** Searched all of `manual_4.5`, `api_4.5` and `release_notes_4.5` for `UTF-8`, `utf8`, `Unicode`, `non-ASCII`, `ASCII`, `encoding`, `codec`. Read in full: `api_4.5/info_gotchas_file_paths_and_encoding.txt`, `manual_4.5/advanced/limits.txt`, `manual_4.5/files/data_blocks.txt` (Name & Rename), `manual_4.5/files/asset_libraries/catalogs.txt`.

**Not covered.** Whether a render succeeds or fails on such a path; whether Blender normalises paths on save; what happens on a file system whose encoding differs from the one that wrote the path.

**Quoted words, source, address, date.** API documentation: `https://docs.blender.org/api/4.5/info_gotchas_file_paths_and_encoding.html`, `https://docs.blender.org/api/4.5/bpy.ops.wm.html`. Manual: `https://docs.blender.org/manual/en/4.5/files/data_blocks.html`, `https://docs.blender.org/manual/en/4.5/advanced/limits.html`, `https://docs.blender.org/manual/en/4.5/files/asset_libraries/catalogs.html`. Date read: 2026-07-30.

**Outside the interface.** The sources do not say whether reading or resolving these paths differs when Blender is driven from outside its own window.

**Operating system.** The sources do not say whether the round trip differs by operating system.


**Observed — hand test entry 11, run 2026-08-05 on the Creator's own computer** (MacBook Pro, M1 Pro chip, macOS 26.5.2, Apple's own graphics), with Blender 4.5.12 LTS started from a terminal with no window, using a throwaway settings folder set through `BLENDER_USER_RESOURCES`. The full list of tests is at `Audits/Blender Research/Dependencies/Hand tests.md`.

**What was done.** A folder named `Тестовая папка` holding `текстура_日本語.png` was put beside a blend-file whose plane uses that image through a relative path. The file was opened with no window and `repr(image.filepath)`, `os.fsencode(image.filepath)`, `repr(bpy.path.abspath(...))` and the image size were printed; one frame was rendered with Cycles and its centre pixel read. The blend-file and the folder were then moved together to another place on the same Mac and the whole thing repeated.

**What came back**, the same in all four versions and the same before and after the move:

```
  repr(filepath)      = '//Тестовая папка/текстура_日本語.png'
  os.fsencode(...)    = b'//\xd0\xa2\xd0\xb5\xd1\x81\xd1\x82\xd0\xbe\xd0\xb2\xd0\xb0\xd1\x8f \xd0\xbf\xd0\xb0\xd0\xbf\xd0\xba\xd0\xb0/\xd1\x82\xd0\xb5\xd0\xba\xd1\x81\xd1\x82\xd1\x83\xd1\x80\xd0\xb0_\xe6\x97\xa5\xe6\x9c\xac\xe8\xaa\x9e.png'
  size                = (32, 32)
```

The rendered centre pixel was `(1.0, 0.1373, 0.1373, 1.0)` — the texture's own red — both before and after the move, and nothing was printed about the path. The bytes are UTF-8 throughout.

**Both standing lines, brought up to date by this test.** *Driven from outside:* settled — the path was read, resolved and rendered from with no window, and reading it out through Python raised nothing. *Operating system:* the move to a second computer running a different system was made on 2026-08-06 and is recorded below; the earlier move was from one place on that Mac to another. The part of the question about another computer was reached on 2026-08-06 and is recorded below; it is in the *cannot be run* section of `Audits/Blender Research/Dependencies/Hand tests.md` and stays unanswered.


**Observed — the move to another computer, run for the first time on 2026-08-06.** This is the half the Creator's one Mac could not reach. A blend-file referencing `//Тестовая папка/текстура_日本語.png` was built in this version **on the Creator's Mac**, carried whole to the rented Linux machine, and opened there in the same version.

```
IMAGE nonlatin  stored='//Тестовая папка/текстура_日本語.png'
                exists=True size=(32, 32) has_data=True
```

**The name survived the crossing and the image was found**, on both machines, in all four versions, with nothing printed about the path on either. The Linux machine's locale was `C.UTF-8`.

**One limit on that, stated plainly.** The two names were checked on the Mac's own disk before the copy and are the same bytes under both Unicode normalisations, because neither uses a combining accent. **So this does not settle what happens to a name the two systems would store differently**, which is the case that would actually break.

**Still not established.** The same move to Windows. No Windows machine has run it.

**Both standing lines, brought up to date by the Linux tests.** *Driven from outside:* every reading above was taken with Blender running with no window unless it says otherwise, so it holds outside the interface. *Operating system:* **now settled on two operating systems, macOS and Linux, and each is its own result** — what they agree on and where they differ is said above. **Windows is still untried: no Windows machine has run it.**

---

## 10. Are some of the things a file points at places it writes to rather than reads from, and where does such a write land?

**Label: documented.**

Yes. Several recorded locations are write destinations.

**Render output.** `RenderSettings.filepath`: *"Directory/name to save animations, # characters define the position and padding of frame numbers"*. The manual: *"File Path — Choose the location to save rendered frames."* and *"This setting expands Relative Paths where a // prefix represents the directory of the current blend-file."* So a relative render path lands beside the blend-file.

**Compositor File Output node.** `CompositorNodeOutputFile.base_path`, *"Base output path for the image"*, and it is one of only two places where template expressions are allowed.

**Bake output.** `BakeSettings.filepath`, *"Image filepath to use when saving externally"*.

**Physics caches.** These have a documented default landing place: *"When Disk Cache is checked, Blender will save the cache separately to the drive in a folder named blendcache\_[filename] alongside the blend-file. (The blend-file must be saved first.)"* Filenames within it are also documented: *"a cache named MyCache will be stored in MyCache\_xxxxxx\_yy.bphys"*, and for an unnamed cache *"a cache on an object Cube will be stored in 43756265\_xxxxxx\_yy.bphys, where 43756265 is determined by the object name."* An explicit path may be given instead: *"External — Allows you to read the cache from a drive using a user-specified file path."*

**Geometry-nodes bakes.** *"In order to bake the simulation, the blend-file must be saved to your computer. The location the file is saved determines where the baked data is also saved. The directory the baked data is saved to can be changed per modifier"*. The choice of destination is a setting: *"Packed: The baked data is packed into the .blend file. So no separate file is necessary. Disk: The baked data is stored in a separate directory on disk."* The property is `NodesModifier.bake_directory`, *"Location on disk where the bake data is stored"*.

**Ocean modifier.** `OceanModifier.filepath`, *"Path to a folder to store external baked images"*; and the manual notes the folder is made if absent: *"If the cache path folder does not exist, it will be created."*

**Fluid cache.** `FluidDomainSettings.cache_directory`, *"Directory that contains fluid cache files"*.

**Dynamic Paint.** `DynamicPaintSurface.image_output_path`, *"Directory to save the textures"*.

**Proxies.** `StripProxy.directory` and `MovieClipProxy.directory`, both *"Location to store the proxy files"*; `SequenceEditor.proxy_dir`.

**Collection exporters.** `CollectionExport.filepath`, *"The file path used for exporting"*.

**A write that lands next to something the file only reads.** The OSL Script node writes beside the script it points at: *"Used to specify a .osl file from a drive, and this will then be automatically compiled into a .oso file in the same directory."* Cycles' own code shows exactly this, and shows what it does when it cannot:

```
        if ok:
            # copy .oso from temporary path to .osl directory
            dst_path = script_path_noext + ".oso"
            try:
                shutil.copy2(oso_path, dst_path)
            except:
                report({'ERROR'}, "Failed to write .oso file next to external .osl file at " + dst_path)
```

**Label for that sub-point: observed** (from a supplied file), for the fact that the code contains this branch and this message. **Inferred**, that a render using an external `.osl` therefore attempts a write into the script's own folder — the step being that this function is called when a Script node is updated, and that `dst_path` is built by replacing the script's extension, so it sits in the same directory.

**Not covered.** Where a render output path lands when the blend-file has never been saved (the sources say relative paths cannot be used in an unsaved file, but not what an already-relative path does). Whether a failed write stops a render. What the "Render Cache" preference folder defaults to.

**Quoted words, source, address, date.** API documentation: `https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html`, `https://docs.blender.org/api/4.5/bpy.types.CompositorNodeOutputFile.html`, `https://docs.blender.org/api/4.5/bpy.types.BakeSettings.html`, `https://docs.blender.org/api/4.5/bpy.types.NodesModifier.html`, `https://docs.blender.org/api/4.5/bpy.types.OceanModifier.html`, `https://docs.blender.org/api/4.5/bpy.types.FluidDomainSettings.html`, `https://docs.blender.org/api/4.5/bpy.types.DynamicPaintSurface.html`, `https://docs.blender.org/api/4.5/bpy.types.StripProxy.html`, `https://docs.blender.org/api/4.5/bpy.types.MovieClipProxy.html`, `https://docs.blender.org/api/4.5/bpy.types.CollectionExport.html`. Manual: `https://docs.blender.org/manual/en/4.5/render/output/properties/output.html`, `https://docs.blender.org/manual/en/4.5/physics/baking.html`, `https://docs.blender.org/manual/en/4.5/physics/simulation_nodes.html`, `https://docs.blender.org/manual/en/4.5/modeling/modifiers/generate/geometry_nodes.html`, `https://docs.blender.org/manual/en/4.5/modeling/modifiers/physics/ocean.html`, `https://docs.blender.org/manual/en/4.5/render/shader_nodes/script.html`. Cycles' own file: `cycles_4.5/osl.py`, function `update_external_script`, lines 180–186. Date read: 2026-07-30.

**Outside the interface.** The render output destination can be set from outside entirely: *"-o, --render-output <path> — Set the render path and file name."*, with the order caution *"blender --background --render-output /tmp test.blend --render-frame 1 …will not render to /tmp because loading the blend-file overwrites the render output that was set."* So a path set before the file is loaded is discarded. The sources do not say whether the other write destinations behave differently without a window.

**Operating system.** The temporary directory is the one place the sources split by system: *"TEMP: Store temporary files here (MS-Windows)."* and *"TMPDIR: Store temporary files here (UNIX Systems). The path must reference an existing directory or it will be ignored."* The sources do not say whether the other write destinations differ by operating system.


**Observed — hand test entries 13, 18 and 20, run 2026-08-05 on the Creator's own computer** (MacBook Pro, M1 Pro chip, macOS 26.5.2, Apple's own graphics), with Blender 4.5.12 LTS started from a terminal with no window, using a throwaway settings folder set through `BLENDER_USER_RESOURCES`. The full list of tests is at `Audits/Blender Research/Dependencies/Hand tests.md`.

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

In this version, after rendering, the writable folder held `['simple_col.osl', 'simple_col.oso']` — the compiled file did land beside its source. With the folder set to read-only no `.oso` appeared and the terminal carried `ERROR: error: Could not open "<path>/readonly/simple_col.oso"` and `Cycles shader graph: failed to read file <path>/readonly/simple_col.oso`; the render still finished and wrote a frame. In 5.0.1, 5.1.2 and 5.2.0 the same test produced no `.oso` in either folder and printed nothing about the script.

**Part four — what the compositor's File Output node calls its folder.** The properties of `bpy.types.CompositorNodeOutputFile` that are not already on `bpy.types.Node`, in this version:

```
['base_path', 'active_input_index', 'format', 'save_as_render', 'file_slots', 'layer_slots']
```

In 4.5 the folder is `base_path`; in 5.0, 5.1 and 5.2 it is `directory`, with a separate `file_name`. Setting `base_path` in 5.x raises `AttributeError: 'CompositorNodeOutputFile' object has no attribute 'base_path'`, and `Scene.node_tree` does not exist there either — the compositor tree is reached through `Scene.compositing_node_group`, which is `None` until something is put there.

Whether that node creates a folder that does not exist was settled here: with the node's `base_path` set to `//fileoutput_new_folder/` and the two-frame animation rendered, `folder created? True   contents: ['Image0001.png', 'Image0002.png']`.

**Both standing lines, brought up to date by this test.** *Driven from outside:* settled for the render output path, for folder creation and for the unsaved-file case — all of it was done with no window. One thing is settled the other way: the operator that updates an external shader script cannot be called with no window, and says so. *Operating system:* macOS for this test. The same test was run on a rented Linux machine on 2026-08-06 and that result is recorded below; Windows stays untested.


**Observed — hand test entry 26, run by the Creator on 2026-08-05 on their own computer** (MacBook Pro, M1 Pro chip, macOS 26.5.2, Apple's own graphics), in Blender 4.5.12 LTS with its window open. This is the half of the list that needs eyes on the screen; the whole list is at `Audits/Blender Research/Dependencies/Hand tests.md`. Because it was run on a Mac, it settles macOS and leaves Windows and Linux open.

**What was done.** A plain two-frame file was opened, Use Nodes was ticked in the Compositing workspace, a File Output node was added and connected to the Render Layers node's Image output, and its folder was set to `//fileoutput_new_folder/` — a folder that did not exist. Then Render ‣ Render Animation.

**What came back**, in the Creator's words: "It all renders. You can check out the files yourself." The folder beside the blend-file was then read from a terminal:

```
$ ls "/Users/artist/Documents/Smoozy hand tests/Dependencies/4.5/26/fileoutput_new_folder/"
Image0001.png
Image0002.png
```

**So the node does create a folder that does not exist, in this version**, and it wrote two files into it: `Image0001.png` and `Image0002.png`.

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

**The compiled OSL file, and the trap in testing it.** After a render, a writable folder holding `simple_col.osl` also held `simple_col.oso` — the same as on the Mac. The read-only folder had to be run **twice**, because the Linux machine is used as the root user and root walks straight through file permissions: run as plain root, Blender wrote the `.oso` into a folder set to `chmod 555` anyway. Run again with the two file-permission capabilities dropped (`capsh --drop=cap_dac_override,cap_dac_read_search`) and with nothing left over from the first run, no `.oso` appeared and the terminal carried

```
ERROR: error: Could not open "/root/…/e18/readonly/simple_col.oso"
Cycles shader graph: failed to read file /root/…/e18/readonly/simple_col.oso
```

and the render still finished and wrote a frame. That is word for word the Mac's result. **The second run is the real answer; the first is recorded because it shows how easily this test writes down the opposite of the truth.**

**And this closes what the Mac could not settle: whether a File Output node writes when the file is rendered with no window.** It does. A File Output node was built by script, its folder set to `//fileoutput_new_folder/`, connected to the Render Layers node's image output, **saved**, and then rendered in a separate run with no window at all. It created the folder that was not there and wrote:

```
Saved: '/root/…/e26/fileoutput_new_folder/Image0001.png'
Saved: '/root/…/e26/fileoutput_new_folder/Image0002.png'
```

PNG files named after the input, which is exactly what the Creator saw through the window on the Mac.

**One thing about where a write lands that only the crossing could show.** A file whose render output path is a full path written on a Mac does not make a Linux render refuse. It creates the whole Mac-shaped folder tree at the root of the Linux disk and writes the frames into it, saying nothing. That run was made as the root user, who owns the root of that disk, so **what happens for a render user who cannot write there is not established.**

**Both standing lines, brought up to date by the Linux tests.** *Driven from outside:* every reading above was taken with Blender running with no window unless it says otherwise, so it holds outside the interface. *Operating system:* **now settled on two operating systems, macOS and Linux, and each is its own result** — what they agree on and where they differ is said above. **Windows is still untried: no Windows machine has run it.**

---

## 11. Can a reference point at a named thing inside another file, and what happens when that thing is renamed or is gone?

**Label: documented** for name-based references, missing libraries and data-blocks, and the repair operation; **inferred** for a rename breaking the reference.

**Yes, references are by name.** The manual: *"Data-blocks names are unique within their namespace. A data-block namespace is defined by its type, and the blendfile it is stored in."* and *"it is possible to have one local and several linked Objects sharing the same name."* The API reflects this in `ID.name_full`, *"Unique data-block ID name, including library one is any"*. The Python route makes the name-based selection explicit:

> `# Load a single scene we know the name of.`
> `with bpy.data.libraries.load(filepath) as (data_from, data_to):`
> `    data_to.scenes = ["Scene"]`

**When it is gone.** *"Broken Libraries — If Blender cannot find a library while loading a blend-file, it will create placeholder data-blocks to replace missing linked ones. That way, references to the missing data are not lost, and by relocating the missing library, the lost data can be automatically restored."* The placeholder is visible from Python: `ID.is_missing` is *"This data-block is a place-holder for missing linked data (i.e. it is [an override of] a linked data that could not be found anymore)"*.

Note the wording covers a missing *library*. For a missing individual data-block, the import options include `USE_PLACEHOLDERS`, *"Generate a placeholder (empty ID) if not found in any library files."*

From the Python loader, a name that cannot be loaded comes back as nothing at all: *"The loaded objects can be accessed from 'data_to' outside of the context since loading the data replaces the strings for the data-blocks or None if the data-block could not be loaded."*

**When it is renamed.** There is a repair operation aimed squarely at this: *"Relocate Linked ID … A right-click a linked ID will show the Relocate option in the ID Data sub-menu. It allows to relocate a directly linked ID by another one, from the same library or from a different one. This can be used to either fix a broken linked data-block (e.g. because the ID was renamed), or to switch to a variation of the same data."* It is limited: *"This operation is only available on data-blocks that are directly linked, and are not a dependency of any other linked data."*

The 4.5 release notes record this as new in this version: *"Linked IDs can now also be relocated, if they are not a dependency of any other linked data"*.

**Inferred, one step.** Since references are by name and a renamed target must be re-pointed by hand with Relocate, a rename in the source file breaks the reference exactly as a deletion would. The sources do not say this outright; the step is that Relocate is offered *because* the ID was renamed, which only makes sense if the rename broke the link.

**Not covered.** Whether the placeholder for a renamed (as opposed to missing) ID looks the same; whether anything is written to a log at load time; what a render does with a placeholder (see question 12).

**Quoted words, source, address, date.** Manual: `https://docs.blender.org/manual/en/4.5/files/data_blocks.html`, `https://docs.blender.org/manual/en/4.5/files/linked_libraries/link_append.html`. API documentation: `https://docs.blender.org/api/4.5/bpy.types.ID.html`, `https://docs.blender.org/api/4.5/bpy.types.BlendDataLibraries.html`, `https://docs.blender.org/api/4.5/bpy.types.BlendImportContext.html`. Release notes: `https://developer.blender.org/docs/release_notes/4.5/core/`. Date read: 2026-07-30.

**Outside the interface.** Loading by name is a Python operation (`bpy.data.libraries.load`) and needs no window. Relocate and Reload are described only as Outliner context-menu items in the manual; the sources do not name a command-line or Python equivalent for Relocate. `Library.reload()`, *"Reload this library and all its linked data-blocks"*, is available from Python.

**Operating system.** The sources do not say.


**Observed — hand test entries 7 and 8, run 2026-08-05 on the Creator's own computer** (MacBook Pro, M1 Pro chip, macOS 26.5.2, Apple's own graphics), with Blender 4.5.12 LTS started from a terminal with no window, using a throwaway settings folder set through `BLENDER_USER_RESOURCES`. The full list of tests is at `Audits/Blender Research/Dependencies/Hand tests.md`.

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

**Label: not in the sources** for the general case; **documented** for two specific cases.

**The general case is not answered.** Nothing states what Cycles, EEVEE or Workbench render when a texture, a volume file, a cache archive or a font is absent, and nothing states what is written to a log or console at render time.

**Where I looked.** Searched all of `manual_4.5` and `api_4.5` for: `magenta`, `pink`, `missing texture`, `not found`, `cannot be found`, `could not be found`, `Missing Data`, `broken`, `placeholder`, `can't be loaded`, `cannot be loaded`, `fails to load`, `unable to open`, `does not exist`, `no longer exist`. Read in full: `interface/window_system/topbar.txt` (External Data menu), `files/linked_libraries/link_append.txt`, `files/blend/packed_data.txt`, `editors/info_editor.txt`, `advanced/command_line/render.txt`, `advanced/command_line/arguments.txt`, `render/output/properties/output.txt`, `video_editing/setup/directory_structure.txt`. Searched `cycles_4.5` (`engine.py`, `properties.py`, `osl.py`, `ui.py`, `operators.py`) for path and error handling; the only missing-file error message there is the OSL one quoted in question 10.

**The two cases that are documented.**

*Video sequencer.* There is a switch for it, and both of its settings are stated: *"Show Missing Media — Render missing images/movies with a solid magenta color. When disabled, missing content will render fully transparent."* And there is a timeline signal: *"Strips with missing content will be displayed as red in the timeline."* For image sequences with gaps: *"This allows you to render an image sequence with a few frames missing and still the image strip will have the correct range to account for the missing frames displayed as black."*

*Missing linked libraries.* Placeholders are made rather than the reference being dropped — see question 11 for the quote.

**Does it say so anywhere — what is documented.** Three reporting routes, all on demand or in the interface:

- *"Report Missing Files — This option is useful to check if there are links to unpacked files that no longer exist. After selecting this option, a warning message will appear in the Info editor's header. If no warning is shown, there are no missing external files."* In the API: `bpy.ops.file.report_missing_files()`, *"Report all missing external files"*.
- The Info editor itself: *"The Info editor logs the executed operators as well as errors, warnings, and informational messages."*
- The Outliner: linked data-blocks *"are also listed in the Outliner's Blender File Display Mode, along with the path of the blend-file they originate from."*

And a repair route: *"Find Missing Files — In case you have broken links in a blend-file, this can help you to fix the problem. A File Browser will show up. Select the desired directory (or a file within that directory), and a search will be performed in it, recursively in all contained directories. Every missing file found in the search will be recovered. Those recoveries will be done as absolute paths, so if you want to have relative paths you will need to select Make Paths Relative."* With a caution: *"Recovered files might need to be reloaded."*

One indirect signal readable from Python: `Image.size` is *"Width and height of the image buffer in pixels, zero when image data can't be loaded"*, and `Image.has_data` is *"True if the image data is loaded into memory"*.

**Not covered.** What the renderer substitutes for a missing image outside the sequencer; whether a render aborts, warns, or proceeds silently; whether anything reaches standard output in background mode; whether the exit status reflects it.

**Quoted words, source, address, date.** Manual: `https://docs.blender.org/manual/en/4.5/editors/video_sequencer/preview/sidebar.html`, `https://docs.blender.org/manual/en/4.5/video_editing/edit/montage/strips/image.html`, `https://docs.blender.org/manual/en/4.5/interface/window_system/topbar.html`, `https://docs.blender.org/manual/en/4.5/editors/info_editor.html`, `https://docs.blender.org/manual/en/4.5/files/linked_libraries/link_append.html`. API documentation: `https://docs.blender.org/api/4.5/bpy.ops.file.html`, `https://docs.blender.org/api/4.5/bpy.types.Image.html`. Date read: 2026-07-30.

**Outside the interface.** This is exactly where the sources fall short. Both documented reporting routes — the Info editor header and the Outliner — are parts of the window. `bpy.ops.file.report_missing_files()` can be called from Python, but the sources do not say where its report goes when there is no Info editor. There is a general logging facility (`--log <match>`, `--log-level <level>`, `--log-file <filepath>` in the command-line arguments), but nothing ties it to missing files.

**Operating system.** The sources do not say.


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

**Label: documented** for indirect dependencies and library-relative paths; **inferred** that a chain can exceed two files; **not in the sources** for a hard depth limit.

**Yes, dependencies come along.** The manual describes the two sides of it. On appending: *"Localize All — Also copy all indirectly linked data, instead of maintaining the links."* — the existence of "indirectly linked data" being the point. The same option in the API is `use_recursive`, *"Localize All, Localize all appended data, including those indirectly linked from other libraries"*. On the asset side: *"All of the asset and all its dependencies will be appended to the current file… "Dependencies" in this case means everything the asset refers to. For an object, this can be its mesh and materials, but also other objects used by modifiers, constraints, or drivers."*

A data-block knows whether it arrived directly or in tow: `ID.is_library_indirect`, *"Is this ID block linked indirectly"*.

**How deep.** Libraries form a chain: `Library.parent` is itself of type `Library` (`bpy.types.Library`, read-only). The append options distinguish two depths explicitly: `APPEND_RECURSIVE`, *"Append (make local) also indirect dependencies of appended IDs coming from other libraries. NOTE: All IDs (including indirectly linked ones) coming from the same initial library are always made local."*

**Inferred, one step:** because a `Library` has a `parent` that is a `Library`, and because the append option speaks of dependencies "coming from other libraries" as a distinct case, the chain can be more than two files deep. The sources do not put a number on it and do not say "unlimited". The step a reader must supply is that a parent-of-a-parent is possible; the documentation only gives the field.

Writing a library out follows the same rule: *"Note — Indirectly referenced data-blocks will be expanded and written too."* (`bpy.data.libraries.write`).

**Can they be reached the same way — no, and this is the important part.** A path stored inside a library is resolved against *that library*, not against the file you opened. The API documentation says so, and warns that it catches people:

> *"When using Blender data from linked libraries there is an unfortunate complication since the path will be relative to the library rather than the open blend-file. When the data block may be from an external blend-file pass the library argument from the bpy.types.ID."*
> `>>> bpy.path.abspath(image.filepath, library=image.library)`

`bpy.path.abspath` states the same in its parameters: *"library (bpy.types.Library) – The library this path is from. This is only included for convenience, when the library is not None its path replaces start."*

Cycles' own code does exactly this when resolving an OSL script: `script_path = bpy.path.abspath(filepath, library=library)` (`cycles_4.5/osl.py`, line 169), with the same pattern at line 209 for internal scripts. **Label for that sub-point: observed.**

**A stated limit.** *"Circular Dependencies — In general, dependencies should not go in both directions. Attempting to link or append data which links back to the current file will likely result in missing links."*

**Also stated, on memory.** *"Referencing compressed blend-files may need a lot of memory because they have to be loaded in their entirety, even if you only link/append a small part of them. Once the data-blocks are loaded, however, memory usage is the same."*

**Not covered.** Any hard depth limit. Whether a broken link two levels down is reported differently from one level down. Whether packing a library also packs that library's own external files (`Pack Linked Libraries` is described only as *"Mark all linked library files in the current blend-file as packed"*).

**Quoted words, source, address, date.** Manual: `https://docs.blender.org/manual/en/4.5/files/linked_libraries/link_append.html`, `https://docs.blender.org/manual/en/4.5/editors/preferences/file_paths.html`, `https://docs.blender.org/manual/en/4.5/files/blend/packed_data.html`. API documentation: `https://docs.blender.org/api/4.5/info_gotchas_file_paths_and_encoding.html`, `https://docs.blender.org/api/4.5/bpy.path.html`, `https://docs.blender.org/api/4.5/bpy.types.Library.html`, `https://docs.blender.org/api/4.5/bpy.types.ID.html`, `https://docs.blender.org/api/4.5/bpy.types.BlendImportContext.html`, `https://docs.blender.org/api/4.5/bpy.types.BlendDataLibraries.html`, `https://docs.blender.org/api/4.5/bpy.ops.wm.html`. Cycles' own file: `cycles_4.5/osl.py`, lines 169 and 209. Date read: 2026-07-30.

**Outside the interface.** The Python helper for library-relative resolution (`bpy.path.abspath(…, library=…)`) is not described as requiring a window. The sources do not say whether a render without a window reaches the Cycles code quoted above or whether the resolution rule changes.

**Operating system.** The sources do not say.


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

**Label: not in the sources** for the question as asked; **documented** for three ways one scene reaches into another, which bear directly on it.

**What is not answered.** Nothing states what a render loads or evaluates when a file holds several scenes — whether the other scenes' images, caches and libraries are read, ignored, or loaded and unused. The command-line argument that picks a scene says only what it does, not what it costs: *"-S, --scene <name> — Set the active scene <name> for rendering."*

**Where I looked.** Read `advanced/command_line/arguments.txt` and `advanced/command_line/render.txt` in full; `files/data_blocks.txt` (Life Time, Protected); `files/blend/open_save.txt`; `files/linked_libraries/link_append.txt`; `render/output/properties/output.txt`; `scene_layout/scene/properties.txt`. Searched for `entire`, `whole file`, `in memory`, `loaded in their entirety`, `background scene`. The only statement about loading a whole file is about *libraries*, not about scenes: *"Referencing compressed blend-files may need a lot of memory because they have to be loaded in their entirety, even if you only link/append a small part of them."*

**Three documented ways one scene reaches beyond itself**, which mean "only that scene's references" is not a simple boundary:

- **Background scenes**, and recursively: *"Background Scene — Allows you to use a scene as a background… Background Scenes can themselves have a Background Scene (they're recursively included)."*
- **The compositor's Render Layers node**, which names a scene: *"Scene — The scene for which to render a view layer."* (`CompositorNodeRLayers.scene` is of type `Scene`).
- **Sequencer scene strips**: `SceneStrip.scene`, *"Scene that this sequence uses"*.

**Inferred, one step:** since a rendered scene may name a background scene, a compositor Render Layers node pointing at another scene, or a sequencer strip holding another scene, rendering "one scene" can require data belonging to other scenes in the same file. The step is that requiring a scene implies requiring the things that scene points at; the sources do not spell it out.

**Not covered.** Whether unreferenced scenes' external files are opened at load. Whether they are opened at render. Whether `--scene` changes any of that.

**Quoted words, source, address, date.** Manual: `https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html`, `https://docs.blender.org/manual/en/4.5/scene_layout/scene/properties.html`, `https://docs.blender.org/manual/en/4.5/compositing/types/input/scene/render_layers.html`, `https://docs.blender.org/manual/en/4.5/files/linked_libraries/link_append.html`. API documentation: `https://docs.blender.org/api/4.5/bpy.types.CompositorNodeRLayers.html`, `https://docs.blender.org/api/4.5/bpy.types.SceneStrip.html`. Date read: 2026-07-30.

**Outside the interface.** `--scene` is itself an outside-the-interface control, and the order caution applies to it as to everything: *"Arguments are executed in the order they are given."* Beyond that, the sources do not say.

**Operating system.** The sources do not say.


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

The evidence is one-sided on Linux for the same reason it was on the Mac: it shows that a render of one scene does not complain about the other scene's missing file, not that the file was never opened. **Nothing about this question is settled by the second machine.** **That was true of the test described here, and is no longer true of the question: a test that could come out either way was run on 2026-08-07 and is recorded further down this answer.**

**Both standing lines, brought up to date by the Linux tests.** *Driven from outside:* every reading above was taken with Blender running with no window unless it says otherwise, so it holds outside the interface. *Operating system:* **now settled on two operating systems, macOS and Linux, and each is its own result** — what they agree on and where they differ is said above. **Windows is still untried: no Windows machine has run it.**


**Observed — added by this round's hand tests, run 2026-08-07 on a rented Linux machine** (Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB on driver 580.95.05, an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory, and no display server at all), with Blender 4.5.12 LTS started from a terminal with no window, `--factory-startup`, and a settings folder of its own made first and set through `BLENDER_USER_RESOURCES`. **This is an addition to a document written in an earlier round; nothing above it has been changed.** The full list of tests is at `Audits/Blender Research/Dependencies/Hand tests — added questions.md`.

**Why this test is not the earlier one.** The earlier test on this question watched whether Blender complained about the other scene's missing image. It never did — and in Blender 4.5 the renderer never names a missing image under any circumstances, so that test could not have come out the other way. What follows uses two readings that can each come out either way, and each has a control that worked.

**What was done.** One file was built with two scenes. SceneA holds a camera, a sun and a plane whose material uses `assets/a.png`. SceneB holds the same with `assets/b_small.png`, and also an object linked from `assets/lib.blend`. Both scenes are Cycles on the processor, one sample, 64 by 64. Beside them sits `assets/b_big.png`, a 6000 by 6000 image.

*The first reading — what is read from disk.* Each render was run under `strace`, which records every file the program opens:

```
strace -f -e trace=openat,newfstatat -o <trace file> \
  /opt/blender/4.5/blender --background --factory-startup <file> \
  --scene SceneA --render-output <folder>/SceneA_ --render-frame 1
```

and the same with `--scene SceneB`. Each trace was then searched for `a.png`, `b_small.png` and `lib.blend`.

*The second reading — how much memory the run needs.* `b_big.png` was copied over `b_small.png`, so the image only SceneB uses became 144 MB of picture, and the SceneA render was run three times under `/usr/bin/time -f "maxRSS=%MkB"`; then the small one was put back and it was run three times again. The same was done with `--scene SceneB` as the control.

**What came back.** The trace, with the assets folder trimmed off the front of each name and identical lines counted:

```
--- SceneA : trace lines naming the three assets ---
      4 openat(AT_FDCWD, "a.png", O_RDONLY) = 3
      1 openat(AT_FDCWD, "lib.blend", O_RDONLY) = 3
--- SceneB : trace lines naming the three assets ---
      4 openat(AT_FDCWD, "b_small.png", O_RDONLY) = 3
      1 openat(AT_FDCWD, "lib.blend", O_RDONLY) = 3
```

The image belonging to the scene that was **not** being rendered was never opened at all, in either direction. The linked library was opened in both.

The memory, peak resident size of the whole run:

```
SceneA, where the big image is NOT used : 402/417/402 MB with the big one, 410/408/418 MB with the small one
SceneB, where the big image IS used     : 612/616 MB with the big one, 426/417 MB with the small one
```

Swapping a 32 by 32 image for a 6000 by 6000 one changed the runs of the scene that does not use it by nothing, and the control by well over a hundred megabytes. The machine was shared with other work while this ran, which is why three runs of each were taken; the variation between runs of the same thing is far smaller than the control's jump.

So, on this machine and in this version: **rendering one scene does not read the other scene's image from disk at all, and does not carry it in memory.** A `.blend` **linked as a library is opened whichever scene is rendered**, since it is read when the file is loaded rather than when a scene is drawn.

**Not settled by this.** It covers an image used by a scene's material and a linked library, and nothing else a file can point at. It does not cover a scene reached from the rendered one — as a background scene, through a compositor Render Layers node, or through a sequencer strip — which is a different case the answer above already describes.

**Both standing lines, brought up to date by this test.** *Driven from outside:* settled — the whole test was done with no window, and `--scene` chose the scene there. *Operating system:* Linux, on the machine named above. **The Creator's Mac was reached on 2026-08-07 by a third reading, and it agrees; that is recorded below.** Neither of the two readings used here could be taken on the Mac — the trace needs an administrator password, and the memory reading needs a machine nobody else is using. Windows is untested because no Windows machine has run it.

**Observed — the Mac half of this round's hand tests, run 2026-08-07 on the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card.** Blender 4.5.12 LTS was started from a terminal with no window, with `--factory-startup` and a settings folder of its own, made before it was used and set through `BLENDER_USER_RESOURCES`; that this took effect was checked rather than assumed, and this Blender answered `RESOURCES …/depwork/res-4.5` and `VERSION 4.5.12 LTS`. **Other topics were running Blender on this same Mac throughout, so nothing was timed and no memory was measured.** **This is an addition; nothing above it has been changed.** The full list of tests is at `Audits/Blender Research/Dependencies/Hand tests — added questions.md`. In the quotations below the working folder's long path is written `…/depwork`; nothing else is altered.

**Why there is a Mac reading at all, and what it is not.** The Linux half settled this question two ways: a trace of every file each render opened, and the peak memory of a run with a large image only one scene uses. **Neither of those two readings could be taken here.** A file-activity trace on macOS needs an administrator password. And this Mac was shared with other topics' Blender runs all the way through this sitting, so a memory measurement would not have been a measurement of anything. A third reading was tried and thrown away because it could not have come out either way: after rendering SceneA, `image.has_data` reads `False` for **both** images, including the scene's own, so it says nothing at all. That is recorded here so nobody runs it again expecting an answer.

**The reading that did work: the time the disk records for when each file was last read.** This Mac's disk keeps an access time for every file, which was checked first on an ordinary text file before it was leaned on. The three things SceneA and SceneB point at — `assets/a.png`, `assets/b.png` and `assets/lib.blend`, the last being a `.blend` linked into SceneB only — were all set to the same old access time, one scene was rendered, and the three times read back. Then the same the other way round, as the control:

```
--- rendering SceneA ---
    access times set to 1577833200 for all three
    EXIT 0  |  1 frame saved
    a.png      access time now 1786068601  (moved — opened)
    b.png      access time now 1577833200  (unchanged — never opened)
    lib.blend  access time now 1786068601  (moved — opened)
--- rendering SceneB ---
    access times set to 1577833200 for all three
    EXIT 0  |  1 frame saved
    a.png      access time now 1577833200  (unchanged — never opened)
    b.png      access time now 1786068601  (moved — opened)
    lib.blend  access time now 1786068601  (moved — opened)
```

the same in all four versions, with only the clock readings differing. **The image belonging to the scene that was not being rendered was never opened, in either direction; the image belonging to the scene being rendered was; and the linked library was opened whichever scene was chosen.** Both controls worked: each run moved the time on the file it should have moved, so the unchanged time on the other means something.

**This agrees exactly with what the trace found on the rented Linux machine**, and the two are kept as two results rather than folded into one: Linux was read by watching every file the program opened, macOS by the time the disk recorded, and each answers for its own machine.

**Not settled by this.** It covers an image used by a scene's material and a linked library, and nothing else a file can point at. It does not cover a scene reached from the rendered one — as a background scene, through a compositor Render Layers node, or through a sequencer strip. It also says nothing about **how much memory** a render of one scene needs on macOS: that reading was not taken, because the machine was shared.

**Both standing lines, brought up to date by these tests.** *Driven from outside:* settled — the whole test was done with no window, and `--scene` chose the scene there. *Operating system:* **now settled on both of this project's machines, by a different reading on each, and they agree.** **Windows is untried: no Windows machine has run it.**




---

## 15. Can a file rely on a colour management profile that lives on the computer rather than inside the file, and how does Blender find it?

**Label: documented** for loading an external configuration through `OCIO`; **inferred** for a `.blend` relying on names supplied by that configuration.

**Yes.** Blender ships one configuration and will use a different one if the environment points at it: *"Blender comes with a standard OpenColorIO configuration that contains a number of useful display devices and view transforms."* and *"However, OpenColorIO is also designed to give a consistent user experience across multiple applications, and for this, a single shared configuration file can be used. Blender will use the standard OCIO environment variable to read an OpenColorIO configuration other than the default Blender one."*

**How it finds it.** By environment variable, listed among the command-line documentation's environment variables: *"OCIO: Path to override the OpenColorIO configuration file."* The bundled default lives in Blender's own folders: *"./datafiles/colormanagement — Default OpenColorIO configuration."*, and that folder is *"Located in both user and system directories. User data files either override or add to system data files."*

**What the setting exposes is a name.** `ColorManagedInputColorspaceSettings.name` is *"Color space in the image file, to convert to and from when saving and loading the image"*, typed as an enum. The manual repeats in several places: *"The list of color spaces depends on the active OCIO config."* — on the image settings page, the colour management page, the environment texture node page, the Convert Colorspace node page, the image formats page, the sequencer strip sidebar, and the glossary (*"The color spaces supported by Blender depend on the active OCIO config."*).

**Inferred, one step:** the file can rely on an external configuration because the colour-space setting is a name and the available names come from the active configuration. The sources do not state that the configuration or profile itself is stored in the `.blend`.

**Inferred, one step:** because the file records a colour space *name* and the set of valid names comes from the active configuration, a file made against one configuration and rendered against another can name a colour space the new configuration does not have. The sources do not say what happens then. The step is mine; the documentation only establishes the two halves.

**New in 4.5.** *"Color Management: Support OpenColorIO file rules to auto detect color space. The Blender configuration does not contain such rules, but other configurations set through the `OCIO` environment variable can."*

**Also documented as a caution about configurations.** A known problem is recorded in the 4.5 bug-fix notes: *"ERROR (gpu.shader): OCIO_Display FragShader:4.4.3 and 4.5.0 have some error with ACEs 2.0 OCIO Configs from Academy Software Foundation But ACES 1.3 worked fine."*

**Not covered.** What Blender does when the `OCIO` variable points at nothing, or at a configuration missing a colour space the file names. Whether the configuration path is ever recorded in the `.blend`. Whether the variable is read once at start-up or re-read.

**Quoted words, source, address, date.** Manual: `https://docs.blender.org/manual/en/4.5/render/color_management.html`, `https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html`, `https://docs.blender.org/manual/en/4.5/advanced/blender_directory_layout.html`, `https://docs.blender.org/manual/en/4.5/editors/image/image_settings.html`, `https://docs.blender.org/manual/en/4.5/glossary/index.html`. API documentation: `https://docs.blender.org/api/4.5/bpy.types.ColorManagedInputColorspaceSettings.html`. Release notes: `https://developer.blender.org/docs/release_notes/4.5/pipeline_assets_io/` and `https://developer.blender.org/docs/release_notes/4.5/bugfixes/`. Date read: 2026-07-30.

**Outside the interface.** This is one of the few answers that clearly holds outside the interface: the mechanism *is* an environment variable, documented in the command-line section of the manual. The sources do not say it behaves differently in background mode.

**Operating system.** The sources do not say the `OCIO` variable differs by operating system. The bundled configuration's location is given as a path relative to Blender's own directory layout, which the same page describes per system.


**Observed — hand test entry 19, run 2026-08-05 on the Creator's own computer** (MacBook Pro, M1 Pro chip, macOS 26.5.2, Apple's own graphics), with Blender 4.5.12 LTS started from a terminal with no window, using a throwaway settings folder set through `BLENDER_USER_RESOURCES`. The full list of tests is at `Audits/Blender Research/Dependencies/Hand tests.md`.

**What was done.** A small OpenColorIO configuration was written holding one colour space `TinyLinear`, one display `TinyDisplay` and one view `TinyView`, saved as `tiny.ocio`. Each version was then started four ways with no window: with nothing set; with `OCIO` pointing at `tiny.ocio`; with `BLENDER_OCIO` pointing at it; and with `BLENDER_OCIO` pointing at a path that does not exist. Each time both environment variables as Blender sees them, `bpy.data.colorspace.is_missing_opencolorio_config`, the list of colour spaces offered, the scene view transform and a new image's colour space were printed. Separately, a file was saved with the bundled configuration in which an image's colour space is `ACEScg` and the scene's view transform is `Filmic`, then reopened under `tiny.ocio`, which has neither name.

**Which variable this version honours.** `OCIO` works. `BLENDER_OCIO` was ignored entirely: with it set, the bundled colour spaces were still offered and the view transform was still `AgX`. Blender does not put `OCIO` into its own environment at start-up. In 5.0.1, 5.1.2 and 5.2.0 both variables work.

**How much the configuration decides.** The bundled configuration offers 21 colour spaces in this version (21 in 4.5.12, 25 in 5.0.1 and 5.1.2, 63 in 5.2.0). With `tiny.ocio` active the list dropped to `['TinyLinear']` and the view transform became `TinyView`.

**Pointing at a configuration that is not there.** All four fell back to the bundled configuration, offered the full list again, and printed an error naming the file:

```
ERROR (imbuf.color_management): source/blender/imbuf/opencolorio/intern/libocio/error_handling.cc:19 report_exception: OpenColorIO Error: Error could not read '<path>' OCIO profile.
```

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


**Observed — hand test entry 25, run by the Creator on 2026-08-05 on their own computer** (MacBook Pro, M1 Pro chip, macOS 26.5.2, Apple's own graphics), in Blender 4.5.12 LTS with its window open. This is the half of the list that needs eyes on the screen; the whole list is at `Audits/Blender Research/Dependencies/Hand tests.md`. Because it was run on a Mac, it settles macOS and leaves Windows and Linux open.

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

**So in this version nothing on screen says anything happened.** The two fields show the substitutes and there is no warning anywhere. In 5.0, 5.1 and 5.2 a yellow warning appears in the corner of the window, and a second one appears in the dialog shown when Blender is closed.

**Read alongside the terminal test.** With no window the same file under the same configuration reported `image colorspace name now = ''` — an empty string — and `scene view transform now = 'TinyView'`, with the substitutions named in lines beginning `Color management:`. On screen the Color Space field does not read as empty: it reads `TinyLinear`, the configuration's own default. The two readings are of different things — what the property returns to a script, and what the field displays — and both are recorded here as they were seen.

**Both standing lines, brought up to date by this test.** *Driven from outside:* the substitution happens either way, and the messages reach the terminal either way. What is new here is that from 5.0 onward the interface keeps a standing warning and warns again before the file is saved, and that 4.5 does neither. *Operating system:* macOS for this test. The same test was run on a rented Linux machine on 2026-08-06 and that result is recorded below; Windows stays untested.


**Observed — run again on Linux, 2026-08-06.** Every part of this was run again in this version on the rented Linux machine, and **every part agreed with the Creator's Mac.**

**Which variable is honoured.** `OCIO` works. With `BLENDER_OCIO` set instead, the bundled 21 colour spaces were still offered, the view transform was still `AgX`, and `OCIO` in Blender's own environment read `None`. **So `BLENDER_OCIO` is ignored, and Blender does not put `OCIO` into its own environment at start-up.** The same as on the Mac.

**How many colour spaces the bundled configuration offers: 21**, the same as on the Mac. With the small configuration active the list dropped to `['TinyLinear']`.

Pointing `OCIO` at a configuration that does not exist fell back to the bundled one, offered the full list again, and printed:

```
ERROR (imbuf.color_management): source/blender/imbuf/opencolorio/intern/libocio/error_handling.cc:19 report_exception: OpenColorIO Error: Error could not read '/root/…/no_such.ocio' OCIO profile.
```

**With a window open, nothing on screen said anything had happened.** The status bar held the network icon and the version number and no warning of any kind. That is the difference the Creator saw on the Mac, seen again here.

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

**Label: documented** for how it is recorded; **not in the sources** for what happens when it is absent.

**How it is recorded: as a file path, like any other external file.** `bpy.types.VectorFont` is *"Vector font for Text objects"* and carries `filepath`, typed *"string, default "", (never None, blend relative // prefix supported)"*. There is nothing family-name-shaped: no family, no style, no system-font lookup key. A font is a file at a location.

The manual describes loading one as ordinary file browsing: *"To load a different Font, click one of the Load buttons in the Font panel and navigate to a font file. The File Browser will give all valid fonts a capital "F" icon."* and *"If you select a font that is unsupported by Blender, you will get the error Not a valid font."* Supported kinds: *"This font system has its own built-in font, but it can use external fonts too, including PostScript Type 1, OpenType and TrueType fonts."*

The operator that loads one defaults to a relative path: `bpy.ops.font.open(*, filepath='', … relative_path=True, …)`, *"Load a new font from a file"*.

**The one place the sources mention where a system keeps its fonts** is a note about Unix, and it is explicitly vague: *"Location of Fonts on Unix — Fonts are typically located under /usr/lib/fonts, or some variant like /usr/lib/X11/fonts, but not always. They may be in other locations as well, such as /usr/share/local or /usr/local/share, and possibly related sub-trees."* There is also a Preferences default: *"Fonts — Default location to browse for text object font files."* In the Preferences screenshot in the manual (`_images/editors_preferences_section_file-paths.png`, on the File Paths preferences page) the Fonts field is visibly filled in with `//usr/share/fonts` while Textures and Sounds show `//` and Temporary Files is empty. **Label for that observation: documented** (it is part of the manual), and I take from the picture only what is visible: that Fonts, Textures, Sounds, Temporary Files, Render Output and Render Cache are each a single path field.

**A font can be carried inside the file.** The data-block table marks Font with ✓ in the Pack column and describes it as *"References font files."* The API gives `VectorFont.pack()`, *"Pack the font into the current blend file"*, and `VectorFont.unpack(method=…)`, *"Unpack the font to the samples filename"*.

**What is not in the sources: what happens when the font is absent.** No fallback is described. The only nearby fact is that Blender always has one font of its own: *"Blender comes with a built-in font by default that is displayed in each of the four font style data-block menus. The built-in font is always present and shows in this list as "Bfont"."* Whether a text object whose font file is missing falls back to Bfont, renders nothing, or something else, the sources do not say. Nor does anything say a missing font is reported as such. **Inferred:** the general "Report Missing Files" check may include a font because it covers *"links to unpacked files that no longer exist"*, but the quoted words do not name fonts.

**Where I looked.** Read `modeling/texts/introduction.txt`, `modeling/texts/properties.txt`, `modeling/texts/editing.txt`, `editors/preferences/file_paths.txt`, `files/data_blocks.txt`, `api_4.5/bpy.types.VectorFont.txt`, `api_4.5/bpy.ops.font.txt`, `api_4.5/bpy.types.BlendDataFonts.txt`, and viewed `_images/editors_preferences_section_file-paths.png`. Searched all three documentation sets for `bfont`, `built-in font`, `builtin font`, `Not a valid font`, `font` combined with `missing`.

**Not covered.** Any fallback behaviour. Whether Blender ever resolves a font by family name rather than path. Whether the four style slots (Regular, Bold, Italic, Bold-Italic) behave differently when one of them is missing.

**Quoted words, source, address, date.** Manual: `https://docs.blender.org/manual/en/4.5/modeling/texts/properties.html`, `https://docs.blender.org/manual/en/4.5/modeling/texts/introduction.html`, `https://docs.blender.org/manual/en/4.5/editors/preferences/file_paths.html` (text and its File Paths screenshot), `https://docs.blender.org/manual/en/4.5/files/data_blocks.html`, `https://docs.blender.org/manual/en/4.5/interface/window_system/topbar.html`. API documentation: `https://docs.blender.org/api/4.5/bpy.types.VectorFont.html`, `https://docs.blender.org/api/4.5/bpy.ops.font.html`. Date read: 2026-07-30.

**Outside the interface.** The recording form — a path in `VectorFont.filepath` — is a property of the data and does not depend on a window. The error *"Not a valid font"* and the "F" icon are described only for the File Browser. The sources do not say what a background render does about a missing font.

**Operating system.** Only the Unix note above, and it is hedged (*"but not always"*). Nothing is said about where Windows or macOS keep fonts, and nothing about whether a font path recorded on one system means anything on another.


**Observed — hand test entry 6, run 2026-08-05 on the Creator's own computer** (MacBook Pro, M1 Pro chip, macOS 26.5.2, Apple's own graphics), with Blender 4.5.12 LTS started from a terminal with no window, using a throwaway settings folder set through `BLENDER_USER_RESOURCES`. The full list of tests is at `Audits/Blender Research/Dependencies/Hand tests.md`.

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
WARN (bke.vfont): source/blender/blenkernel/intern/vfont.cc:225 BKE_vfont_data_ensure: Font file doesn't exist: //assets/testfont.ttf
```

four times, each preceded by two lines of `Error: Unable to pack file, source path '<path>/assets/testfont.ttf' not found`. In this version those `Unable to pack file` lines also appear when the file is merely opened; in 5.0, 5.1 and 5.2 they appear only on the render, and the warning there reads `geom.vfont       | WARNING Font file doesn't exist: //assets/testfont.ttf` with no source file or line number.

`bpy.ops.file.report_missing_files()` does name the font, in all four versions: `Warning: Path '<path>/assets/testfont.ttf' not found, from local data-block 'VFArial Regular'`.

**Both standing lines, brought up to date by this test.** *Driven from outside:* settled. All of this was done with no window; the warning reaches the terminal and the render is not stopped. *Operating system:* macOS for this test, with the same test run on a rented Linux machine on 2026-08-06 and recorded below; the font came from the Mac's own font folder. Where Windows and Linux keep their fonts, and what a font path written on one means on another, is untested and was not tried here.

**Still open.** What the text object looks like on screen and what the Font panel says is entry 22 on the hand-test list and is the Creator's to answer.


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

**Closed on 2026-08-07, by this round's hand tests.** Under question 8, whether capitals matter in a name **inside** a file, which the answer above had listed as not covered: they do, exactly, and asking a library for a name with the wrong capitals through `bpy.ops.wm.link` reports success and links nothing. Under question 14, whether rendering one scene reads only that scene's references: the earlier test could not have come out the other way, and a new one that could — a record of every file each run opened, and the memory a run needs when the other scene's image is made 144 MB — was run on the rented Linux machine and settles it. Both results are written into the answers above, and the list is at `Audits/Blender Research/Dependencies/Hand tests — added questions.md`.

**Not closed by any test.** Whether the list under question 1 is the whole list: the test file was built
to hold seven kinds, so a kind nobody thought to include would not have appeared.

---

2026-08-04 — Opus 5 (Claude Code) — Dependencies, Blender 4.5

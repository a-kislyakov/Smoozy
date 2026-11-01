# Dependencies — Blender 5.2

**Sources consulted, and when.** Blender 5.2 manual (`Blender Sources/manual_5.2`, including its `_images` folder), Blender 5.2 Python API documentation (`Blender Sources/api_5.2`), Blender 5.2 release notes (`Blender Sources/release_notes_5.2`), and Cycles' own files (`Blender Sources/cycles_5.2`). All were taken on **2026-07-30**, and that is the date read for everything below. I also read `Blender Sources/About this folder.md` and `Blender Sources/requirements/requirements_2026-08-01.txt`; neither is one of the four sources I was told to answer from, so I have not used them as the basis of any answer.

**I read only the question sheet and files inside `Blender Sources/`. I did not open any other part of this project.** One disclosure, so the line above is exact: to create the output folder I ran `mkdir -p` on `Preparation/Blender Research/Findings/5.2/`, and the same shell command printed the names of the files already sitting there. I saw a list of file names. I did not open any of them, and nothing from that list is used anywhere below.

**A note on the three address forms used below.** Manual pages are cited as `https://docs.blender.org/manual/en/5.2/…`, API pages as `https://docs.blender.org/api/5.2/…`, release notes as `https://developer.blender.org/docs/release_notes/5.2/…/`. Cycles' own files have no address and are cited by file and line.

---

## 1. What kinds of thing can a `.blend` file point at outside itself? Give the full list.

**Answer.** The sources expose two groups of outside locations. They do not establish the full list asked for.

**(a) Whole data-blocks whose content is a file.** The manual's table of data-block types has a Pack column whose legend reads *"File Packing, supports file contents being packed into the blend-file (not applicable for most data-blocks which have no file reference)"*. Six rows carry a ✓ or ✗ there — Font, Image, Library, Movie Clip, Sound, Text. Cache File carries a "—" and is listed below because the API gives it a `filepath`, not because the table marks it. *(Citation check 2026-08-05: the rows had been introduced as ones the table "marks as having a file reference", which is not what the table does for Cache File.)* The relevant rows, quoted from the table:

> "Cache File … Used by Mesh Cache modifiers."
> "Font … References font files. Used by curve object-data of text objects."
> "Image … Image files. Used by shader nodes and textures."
> "Library … References to an external blend-file. Access from the Outliner's Blender File view."
> "Movie Clip … Reference to an image sequence or video file. Used in the Movie Clip editor."
> "Sound … Reference to sound files. Used as data of speaker objects."
> "Text … Text data. Used by Python scripts and OSL shaders."

— Manual, *Data-Blocks*, https://docs.blender.org/manual/en/5.2/files/data_blocks.html — read 2026-07-30.

Volume is a further one; the manual's table does not mark it as packable, but the API does list it (see the packing list under question 3).

**(b) Individual path settings scattered across the data.** The API marks every path-typed property with the phrase "blend relative // prefix supported". Searching the whole of the 5.2 API documentation for that phrase returns these types (excluding the search index pages): `bpy.ops.view3d`, `bpy.ops.workspace`, `BakeSettings`, `BlendDataFonts`, `BlendDataImages`, `BlendDataMovieClips`, `BlendDataSounds`, `BlendDataTexts`, `CacheFile`, `CacheFileLayer`, `CollectionExport`, `CompositorNodeOutputFile`, `DynamicPaintSurface`, `FluidDomainSettings`, `Image`, `ImagePackedFile`, `ImageStrip`, `Library`, `LibraryWeakReference`, `MeshCacheModifier`, `MovieClip`, `MovieClipProxy`, `MovieStrip`, `MultiresModifier`, `NodeSocketStringFilePath`, `NodeTreeInterfaceSocketStringFilePath`, `NodesModifier`, `NodesModifierBake`, `OceanModifier`, `PointCache`, `PointCacheItem`, `PreferencesFilePaths`, `RenderSettings`, `SequenceEditor`, `ShaderNodeScript`, `ShaderNodeTexIES`, `Sound`, `StripProxy`, `VectorFont`, `Volume`, `VolumeGrids`.

The exact wording of a representative sample, each quoted from its API page (all read 2026-07-30):

- `Library.filepath` — "Path to the library .blend file (default "", never None, blend relative // prefix supported)" — https://docs.blender.org/api/5.2/bpy.types.Library.html
- `Image.filepath` — "Image/Movie file name (default "", never None, blend relative // prefix supported)" — https://docs.blender.org/api/5.2/bpy.types.Image.html
- `Sound.filepath` — "Sound sample file used by this Sound data-block" — https://docs.blender.org/api/5.2/bpy.types.Sound.html
- `Volume.filepath` — "Volume file used by this Volume data-block" — https://docs.blender.org/api/5.2/bpy.types.Volume.html
- `MovieClip.filepath` — "Filename of the movie or sequence file" — https://docs.blender.org/api/5.2/bpy.types.MovieClip.html
- `VectorFont.filepath` — "(default "", never None, blend relative // prefix supported)" — https://docs.blender.org/api/5.2/bpy.types.VectorFont.html
- `CacheFile.filepath` and `MeshCacheModifier.filepath` and `MultiresModifier.filepath` — "Path to external displacements file" — https://docs.blender.org/api/5.2/bpy.types.CacheFile.html, …`bpy.types.MeshCacheModifier.html`, …`bpy.types.MultiresModifier.html`
- `CacheFileLayer.filepath` — "Path to the archive" — https://docs.blender.org/api/5.2/bpy.types.CacheFileLayer.html
- `ShaderNodeScript.filepath` — "Shader script path" — https://docs.blender.org/api/5.2/bpy.types.ShaderNodeScript.html
- `ShaderNodeTexIES.filepath` — "IES light path" — https://docs.blender.org/api/5.2/bpy.types.ShaderNodeTexIES.html
- `PointCache.filepath` — "Cache file path" — https://docs.blender.org/api/5.2/bpy.types.PointCache.html
- `FluidDomainSettings.cache_directory` — "Directory that contains fluid cache files" — https://docs.blender.org/api/5.2/bpy.types.FluidDomainSettings.html
- `NodesModifier.bake_directory` — "Location on disk where the bake data is stored" — https://docs.blender.org/api/5.2/bpy.types.NodesModifier.html
- `OceanModifier.filepath` — "Path to a folder to store external baked images" — https://docs.blender.org/api/5.2/bpy.types.OceanModifier.html
- `DynamicPaintSurface.image_output_path` — "Directory to save the textures" — https://docs.blender.org/api/5.2/bpy.types.DynamicPaintSurface.html
- `RenderSettings.filepath` — "Directory/name to save animations, # characters define the position and padding of frame numbers (default "//", never None, blend relative // prefix supported, Supports template expressions)" — https://docs.blender.org/api/5.2/bpy.types.RenderSettings.html
- `CompositorNodeOutputFile.directory` — "The directory where the image will be written (default "", never None, blend relative // prefix supported, Supports template expressions)" — https://docs.blender.org/api/5.2/bpy.types.CompositorNodeOutputFile.html
- `BakeSettings.filepath` — "Image filepath to use when saving externally (default "//", …)" — https://docs.blender.org/api/5.2/bpy.types.BakeSettings.html
- `StripProxy.directory` / `MovieClipProxy.directory` — "Location to store the proxy files" — https://docs.blender.org/api/5.2/bpy.types.StripProxy.html, …`bpy.types.MovieClipProxy.html`
- `StripProxy.filepath` — "Location of custom proxy file" — same page
- `SequenceEditor.proxy_dir` — "(default "", never None, blend relative // prefix supported)" — https://docs.blender.org/api/5.2/bpy.types.SequenceEditor.html
- `ImageStrip.directory` plus `ImageStrip.elements` — https://docs.blender.org/api/5.2/bpy.types.ImageStrip.html
- `CollectionExport.filepath` — "The file path used for exporting" — https://docs.blender.org/api/5.2/bpy.types.CollectionExport.html
- `VolumeGrids.frame_filepath` — "Volume file used for loading the volume at the current frame. Empty if the volume has not be loaded or the frame only exists in memory." — https://docs.blender.org/api/5.2/bpy.types.VolumeGrids.html
- `LibraryWeakReference.filepath` — "Path to the library .blend file" — https://docs.blender.org/api/5.2/bpy.types.LibraryWeakReference.html
- `Text.filepath` — "Filename of the text file (default "", never None)". Note this one is *not* marked as supporting the `//` prefix — https://docs.blender.org/api/5.2/bpy.types.Text.html

Blender itself provides the authoritative way to walk this list rather than reconstructing it: `BlendData.file_path_foreach` — "Call visit_path_fn for the file paths used by all ID data-blocks in current bpy.data" — and `BlendData.file_path_map` — "Returns a mapping of all ID data-blocks in current bpy.data to a set of all file paths used by them." (https://docs.blender.org/api/5.2/bpy.types.BlendData.html, read 2026-07-30). The 5.2 release notes say plainly what that is for:

> "Three new options for `bpy.data.file_path_foreach` so render farms and other scripts can find all relevant image files for rendering, including the new Cycles texture cache."

— Release notes, *Python API*, https://developer.blender.org/docs/release_notes/5.2/python_api/ — read 2026-07-30.

**LABEL.** Each individually quoted entry above: **documented**. The list as a whole is **not in the sources**. The search for one fixed phrase cannot establish completeness, because `Text.filepath` is an example of a path property not marked with that phrase. The document records no broader search for unmarked file paths, directory paths, URLs, or stored locations.

**Not covered.** It does not settle what a path property means when it is empty, nor which of these are read from as against written to (see question 10). It does not cover things Blender points at that are not file paths — network URLs for asset libraries are covered separately under question 2. It does not settle whether a path is followed during a render or only when a person asks.

**Outside the interface.** **inferred.** The release-note quote says render farms and scripts can use `file_path_foreach`, so that enumeration route is available outside the interface. The sources quoted here do not establish that every listed setting behaves identically in a background run.

**Operating system.** **not in the sources.** Nothing quoted in this answer marks an entry as platform-specific, but no targeted operating-system search is recorded.


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

**Answer.** The sources describe three library mechanisms, but only the first is documented as an outside file recorded by the `.blend`. They do not establish the full answer asked for.

**(a) A linked `.blend` library.** The `Library` data-block is described as:

> "External .blend file from which data is linked"

— API, `bpy.types.Library`, https://docs.blender.org/api/5.2/bpy.types.Library.html — read 2026-07-30. The manual's own description of what it is for:

> "Link creates a reference to data in a source file such that changes made there will be reflected in the current file the next time it is reloaded."

— Manual, *Link & Append*, https://docs.blender.org/manual/en/5.2/files/linked_libraries/link_append.html — read 2026-07-30.

**(b) An "archive" library — internal storage generated when linked IDs are packed into the current file, not a second outside location.** `Library` carries:

> "archive_libraries — Archive libraries of packed IDs, generated (and owned) by this source library (default None, readonly)"
> "archive_parent_library — Source library from which this archive of packed IDs was generated (readonly)"
> "is_archive — This library is an 'archive' storage for packed linked IDs originally linked from its 'archive parent' library. (default False, readonly)"

— API, `bpy.types.Library`, same page and date.

**(c) An asset library registered in Preferences, which may be local or reached over a network.** The manual:

> "Asset libraries can point to directories on the local file system or to online asset repositories."

and, for the two settings:

> "Path — Directory containing the asset library. Only available for local asset libraries."
> "URL — URL of the online asset library. Only available for online asset libraries."

— Manual, *Asset Libraries* preferences, https://docs.blender.org/manual/en/5.2/editors/preferences/asset_libraries.html — read 2026-07-30.

For the one that does not sit on the same computer, the manual is explicit:

> "A Remote Asset Library is an Asset Library with a few extra files, stored on a server so that it can be accessed remotely."
> "In this case, "Remote" means "reached through a network connection". It can be served from your own computer, it doesn't have to be far away."

— Manual, *Remote Asset Libraries*, https://docs.blender.org/manual/en/5.2/files/asset_libraries/remote_asset_libraries.html — read 2026-07-30.

Two limits on a remote one matter here. First, it is not linked, it is downloaded:

> "Linking is not supported for online asset libraries."

— Manual, *Asset Libraries* preferences, address above.

Second, a remote asset must carry its own dependencies:

> "Each .blend file should be independent. Any external dependency, such as .png files for textures, must be packed into the blend file."

— Manual, *Remote Asset Libraries*, address above.

The 5.2 release notes record this as new in this version:

> "It is now possible to register remotely hosted asset libraries, browse them from within Blender, and download assets as needed. This requires _Allow Internet Access_ to be enabled in the Preferences."

— Release notes, *Assets*, https://developer.blender.org/docs/release_notes/5.2/assets/ — read 2026-07-30.

The matching API is `UserAssetLibrary`, with `path` ("Path to a directory with .blend files to use as an asset library"), `remote_url` ("Remote URL to the asset library"), `use_remote_url` ("Synchronize the asset library with a remote URL (default False, readonly)") and `use_relative_path` ("Use relative path when linking assets from this asset library (default True)") — https://docs.blender.org/api/5.2/bpy.types.UserAssetLibrary.html — read 2026-07-30.

**LABEL.** **documented** for the three mechanisms and their quoted limits. **not in the sources** for whether a saved `.blend` can itself point to a network or remote library. The document records no targeted search for accepted `Library.filepath` schemes or network-path forms.

**Not covered.** The quoted material does not settle whether a `Library.filepath` may itself be a mounted share or URL, what schemes it accepts, or what happens when a remote asset server is unreachable. The remote passages concern asset libraries, which are places to find files rather than the recorded pointer inside a saved `.blend`.

**Outside the interface.** **documented** that the remote-library listing can be generated by the quoted background command. **inferred** that a linked `.blend` can be loaded without the interface because its path is recorded in the file. The quoted sources do not establish how remote-library registration, internet permission, or download behave in a background run.

**Operating system.** **not in the sources.** The remote-library section names web servers and HTTP requirements but no operating-system restriction. No targeted operating-system search is recorded.


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

**Answer.** Yes, partly. Packing is per-file, not only all-or-nothing.

> "A single file can be packed by clicking on the little "gift box" icon to the left of its file-path UI widget."

and, for the reverse:

> "A single file can be unpacked by clicking on the little "gift box" icon to the left of its file-path UI widget."

— Manual, *Packed Data*, https://docs.blender.org/manual/en/5.2/files/blend/packed_data.html — read 2026-07-30.

The same page names the sweeping operations that do it wholesale — "Pack Resources … Mark all eligible external resource files used by the blend-file as packed. Actual packing will happen on the next save of the blend-file." and "Automatically Pack Resources" — and separately "Pack Linked Libraries … Mark all linked library files in the current blend-file as packed."

Not everything can be packed at all, so a fully-packed file is not always achievable:

> "Not all external files can be packed
> Some typically heavy external files, like videos from the Sequence Editor or Movie Clips, cannot be packed in a blend-file."

— same page.

What is packable is fixed by the API. `PackedFile` is described as "External file packed into the .blend file", and its "References" section lists exactly where a packed file can hang: `Image.packed_file`, `ImagePackedFile.packed_file`, `Library.packed_file`, `Sound.packed_file`, `VectorFont.packed_file`, `Volume.packed_file` — https://docs.blender.org/api/5.2/bpy.types.PackedFile.html — read 2026-07-30.

**What a partly-held file reports when it is read.** Three separate reports, all quoted:

- Per data-block, whether its content is inside: `Image.packed_file` — "First packed file of the image (readonly) — Type: PackedFile | None" and `Image.packed_files` — "Collection of packed images (default None, readonly)" (https://docs.blender.org/api/5.2/bpy.types.Image.html). Reading `None` as “not packed” is **inferred** from the stated return type.
- For linked data specifically: `ID.is_linked_packed` — "This data-block is linked and packed into the .blend file (default False, readonly)" (https://docs.blender.org/api/5.2/bpy.types.ID.html).
- When walking the file's paths, the packed ones are left out **by default**. `BlendData.file_path_foreach` has the signature `file_path_foreach(visit_path_fn, *, subset=None, visit_types=None, flags={'SKIP_PACKED', 'SKIP_WEAK_REFERENCES'})`, and the flag means "Skip Packed. Skip paths when their matching data is packed." (https://docs.blender.org/api/5.2/bpy.types.BlendData.html and the *File Path Foreach Flag Items* list). The conclusion that a no-argument walk reports only paths still living outside is **inferred** by combining the default flags with that definition.

In the interface the report is a picture: "You know that a data is packed when you see a little "gift box" icon displayed next to its path." — Manual, *Packed Data*, address above.

**LABEL.** **documented**, except for the two steps marked **inferred** above.

**Not covered.** It does not settle whether a packed copy and the on-disk original are compared automatically on load — the manual mentions a "(differs) or (identical)" status only in the context of the unpack dialog. It does not say what happens if a file is packed and its original path is also still valid but different. It does not say what a partly-packed file reports to a *renderer* as against to a script.

**Outside the interface.** **inferred.** The reporting properties, path walk, and pack/unpack operators are exposed through Python. The quoted sources do not state whether they can all be used in a background run or whether packing then behaves identically there. The gift-box report is documented only in the interface.

**Operating system.** **not in the sources.** No targeted operating-system search is recorded.


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

**Answer.** Yes, in several distinct ways, and 5.2's own API says so in as many words.

**(a) A single directory with an array of files.** The flag list for `file_path_foreach` includes:

> "SKIP_MULTIFILE: Skip Multi-file. Skip paths where a single dir is used with an array of files, eg. sequence strip images or point-caches. In this case only the first file path is processed."

— API, *File Path Foreach Flag Items*, reached from https://docs.blender.org/api/5.2/bpy.types.BlendData.html — read 2026-07-30. `ImageStrip` matches that shape exactly: it carries a `directory` and an `elements` collection (https://docs.blender.org/api/5.2/bpy.types.ImageStrip.html).

**(b) UDIM tiles behind one path with a token in it.**

> "<UDIM>: A 4-digit notation calculated as 1001 + u-tile + v-tile * 10."
> "<UVTILE>: A notation defined as u(u-tile + 1)_v(v-tile + 1)."
> "monster-basecolor.<UDIM>.png will load/save files like monster-basecolor.1021.png etc."

— Manual, *UDIMs*, https://docs.blender.org/manual/en/5.2/modeling/meshes/uv/workflows/udims.html — read 2026-07-30.

The API calls such a path a "virtual filepath" and gives two ways to deal with it:

> "RESOLVE_TOKEN: Resolve Tokens. Resolve tokens within a virtual filepath to a single, concrete, filepath. Currently only used for UDIM tiles."
> "EXPAND_TOKENS: Expand Tokens. Expand template tokens in virtual file paths and invoke the callback once for each concrete file. Currently only used for UDIM tiles."

— *File Path Foreach Flag Items*, address above.

**(c) Image and volume sequences.**

> "EXPAND_SEQUENCES: Expand Sequences. Expand image and volume sequences, invoking the callback once per file on disk."

— same list. `Image.source` confirms the kinds: "SEQUENCE — Image Sequence – Multiple image files, as a sequence." and "TILED — UDIM Tiles – Tiled UDIM image texture." (https://docs.blender.org/api/5.2/bpy.types.Image.html).

**(d) A cache file that accompanies an image.**

> "EXPAND_CACHES: Expand Caches. Visit cache files, like texture cache files associated with images."

— same list, and see question 10 for the Cycles `.tx` files this refers to.

**(e) A render output path that yields one file per frame and, in multi-view, per view.** "When rendering an animation, the frame number is appended at the end of the file name with four padded zeros (e.g. image0001.png)." — Manual, *Output*, https://docs.blender.org/manual/en/5.2/render/output/properties/output.html — read 2026-07-30. And "By default the system saves each view as an individual file, thus generating as many files as views to be rendered." — Manual, *Stereoscopy usage*, https://docs.blender.org/manual/en/5.2/render/output/properties/stereoscopy/usage.html — read 2026-07-30.

The metadata object handed to a `file_path_foreach` callback distinguishes two of these cases: `BlendDataPathMeta.is_expanded` — "True when the path was expanded from a UDIM tile or sequence frame. These paths can not be edited." and `is_cache` — "True when the path is a cache file, like the image texture cache. These paths can not be edited." (https://docs.blender.org/api/5.2/bpy.types.BlendDataPathMeta.html — read 2026-07-30). The quote does not say it identifies the directory-array or render-output cases.

**LABEL.** **documented.**

**Not covered.** It does not settle what Blender does when only some tiles or some frames of such a set are present. For sequences, the quoted words say the callback is invoked “once per file on disk”; they do not say what other recorded information, if any, limits that list. For UDIMs the manual describes a stored tile list, but the relation between that list and expansion is not stated here.

**Outside the interface.** **documented** that the expansion machinery was provided so render farms and scripts can find relevant image files. The sources do not compare its result inside and outside the interface.

**Operating system.** **not in the sources.** No targeted operating-system search is recorded.


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

**Answer.** Five forms of recorded location are documented below, followed by one further form used by a preference rather than stored in the `.blend`. The sources do not establish that every form can be combined with every other one.

**(a) Absolute.**

> "Make Paths Absolute — Make all paths to external files absolute (= full path from the system's root)."

— Manual, *Topbar*, https://docs.blender.org/manual/en/5.2/interface/window_system/topbar.html — read 2026-07-30.

**(b) Relative, with a `//` prefix.**

> "When you specify one of these external files, the default option is to make the path relative. Blender stores a partial path evaluated relative to the directory location of the referencing blend-file."
> "When relative paths are supported, the File Browser provides a Relative Path checkbox, when entering the path into a text field, use a double slash prefix (//) to make it so."
> "You cannot use relative paths into a new untitled blend-file. Save it before linking to external files."

— Manual, *Opening & Saving*, https://docs.blender.org/manual/en/5.2/files/blend/open_save.html — read 2026-07-30.

**(c) With `#` characters standing for a frame number.**

> "When rendering an animation, the frame number is appended at the end of the file name with four padded zeros (e.g. image0001.png). You can set a custom padding size by adding the appropriate number of # anywhere in the file name (e.g. image_##_test.png translates to image_01_test.png)."

— Manual, *Output*, https://docs.blender.org/manual/en/5.2/render/output/properties/output.html — read 2026-07-30. The API repeats it on the property: "# characters define the position and padding of frame numbers" (`RenderSettings.filepath`, https://docs.blender.org/api/5.2/bpy.types.RenderSettings.html).

**(d) With angle-bracket tile tokens `<UDIM>` and `<UVTILE>`** — quoted in full under question 4.

**(e) With curly-brace template expressions.** This is a whole documented syntax:

> "Path templates substitute template expressions (written {abcd}, where "abcd" is a variable name) in a filepath when the path is used."
> "This substitution happens internally at time of use (e.g. while rendering), so the template syntax will remain as-is in the path field."

The variables available in all supporting paths are `blend_name` ("The currently open blend file's name (without the .blend)"), `blend_dir` ("Path up to (but not including) the currently open blend file"), `blend_name_lib` and `blend_dir_lib` ("Like blend_name, except that for path properties on library-linked data-blocks it will be the name of the library blend file that the data-block comes from"). In render output paths only: `fps`, `resolution_x` / `resolution_y`, `scene_name`, `camera_name`. On a node only: `node_name`.

Format specifiers exist — "dance_{fps:###}.png: format as an integer with at least 3 digits." — and literal braces are escaped by doubling — "{{ translates to a single { in the final path."

Where this may be used is restricted:

> "Currently path templates are only supported for the render output path in Scene properties and the output paths in the compositor's File Output node. More filepaths will support templates in future releases."

— all from Manual, *File Paths*, https://docs.blender.org/manual/en/5.2/files/file_paths.html — read 2026-07-30. The API marks the two supporting properties with the words "Supports template expressions" (`RenderSettings.filepath`, `CompositorNodeOutputFile.directory`).

A malformed template is a recorded state too: "Paths that support templates can have template errors, which prevent the path from being processed." (same page).

**(f) Relative to the source image, or blank, for the Cycles texture-cache preference.** This is a preference rather than a location recorded in the `.blend`: `PreferencesFilePaths.texture_cache_directory` — "The directory for storing tx files generated from image files, for more efficient rendering. Paths may be absolute, or relative to the image file. Leave blank to store tx files in the same directory as image files" (https://docs.blender.org/api/5.2/bpy.types.PreferencesFilePaths.html — read 2026-07-30).

**LABEL.** **documented.**

**Not covered.** The quoted material does not settle the stored separator, whether both slash forms are accepted, whether environment variables or `~` are expanded, or what happens if `#` and a `{…}` template appear together.

**Outside the interface.** **documented** that a command-line render accepts a `//` output path, through the quoted example. **inferred** that template substitution also applies there because it happens “at time of use” and rendering is the example. The relative-path default is documented as an interface preference. The claim that Python sets a path literally, and the claim that every template-error report is available without a window, were not established by the quoted words and are removed.

**Operating system.** **not in the sources.** The quoted sources do not describe platform differences for these forms, and no targeted operating-system search is recorded.


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

## 6. What does a location recorded on one kind of computer mean on another, including a kind that names its disks?

*This question is ambiguous. I read it as: what does a path stored in a `.blend` resolve to when the file is opened on a different sort of computer, and in particular on one that identifies volumes by a drive letter. The other reading — what a path means on a different machine of the same sort — is covered by the relative/absolute answer below.*

**Answer.** The sources settle the relative case and are silent on the drive-letter case.

**What is settled.** A relative path is anchored to the referencing file, and that anchoring is what makes it travel:

> "Blender stores a partial path evaluated relative to the directory location of the referencing blend-file. This choice helps when you need to reorganize folders or move your files."
> "With a relative path, you can move the blend-file to a new location provided the externally linked files are moved along with it. For example, you could send someone a folder that contains a blend-file and a subfolder of external images that it references."
> "If it is necessary to relocate a blend-file relative to its linked resources, use Blender's File Save As function which has an option to Remap Relative file links."

— Manual, *Opening & Saving*, https://docs.blender.org/manual/en/5.2/files/blend/open_save.html — read 2026-07-30. And on the Save As option itself: "Remap Relative — This option remaps Relative Paths (such as linked libraries and images) when saving a file in a new location." (same page).

An absolute path is defined as a full path from the system's root: "Make Paths Absolute — Make all paths to external files absolute (= full path from the system's root)." — Manual, *Topbar*, https://docs.blender.org/manual/en/5.2/interface/window_system/topbar.html — read 2026-07-30. The stronger claim that this necessarily ties it to one machine is **inferred**, and the cross-system result remains unestablished below.

**What is not settled.** I found nothing in the 5.2 manual, API documentation, release notes or Cycles files stating what a stored absolute path containing a drive letter (or a stored POSIX absolute path) resolves to when the file is opened on a different kind of computer, nor whether Blender rewrites, normalises, or fails on such a path.

Two adjacent statements exist and are worth having, but neither is about paths stored in a `.blend`:

- The OBJ exporter's Path Mode options say: "When referencing paths in exported files you may want some control as to the method used since absolute paths may only be correct on your own system. Relative paths, on the other hand, are more portable but mean that you have to keep your files grouped when moving about on your local file system." and, for the Relative option, "Uses relative paths in every case (except when on a different drive on Windows)." — Manual, *Wavefront OBJ*, https://docs.blender.org/manual/en/5.2/files/import_export/obj.html — read 2026-07-30. That is documentation of the OBJ *exporter*, not of `.blend` path resolution.
- `bpy.path.basename(path)` — "Equivalent to os.path.basename, but skips a "//" prefix. Use for Windows compatibility." and `bpy.path.native_pathsep(path)` — "Replace the path separator with the system's native os.sep." — API, *Path Utilities*, https://docs.blender.org/api/5.2/bpy.path.html — read 2026-07-30. These show Blender is aware of separator differences and gives scripts a conversion, but say nothing about what Blender does on its own with a stored path.

**LABEL.** **documented** for the quoted relative and absolute definitions. **inferred** for the machine-tied conclusion. **not in the sources** for the drive-letter question and cross-kind resolution generally.

**Where I looked, for the "not in the sources" part.** I searched the whole 5.2 manual and API documentation (text files only) for: `UNC`, `drive letter`, `network path`, `network share`, `network drive`, `shared drive`, `C:\`, `D:\`, `/home/`, `/Users/`, `backslash`, `slash`, `path separator`, `os.sep`, `cross-platform`, `portable`, `another computer`, `different computer`, `different operating system`, `another machine`, `path remap`, `remap`, `Windows and Linux`. The only hits relevant to paths are the two quoted above plus installation-location examples and unrelated keyboard-shortcut names ("NumpadSlash"). I read in full: `files/blend/open_save.txt`, `files/file_paths.txt`, `files/blend/packed_data.txt`, `interface/window_system/topbar.txt`, `api_5.2/bpy.path.txt` and `api_5.2/info_gotchas_file_paths_and_encoding.txt`. The 5.2 release notes contain no path-portability entry (I read `index.md`, `core.md`, `pipeline_io.md`, `python_api.md`, `assets.md`, `cycles.md` and grepped all of them for `path`, `relative`, `librar`, `pack`, `link`).

**Not covered.** Nothing here settles what happens on load when an absolute path is unreachable — see question 12.

**Outside the interface.** “Remap Relative” is documented as a Save As file-browser option. `bpy.ops.file.make_paths_relative()` and `bpy.ops.file.make_paths_absolute()` exist as Python operators (https://docs.blender.org/api/5.2/bpy.ops.file.html — read 2026-07-30). It is **inferred**, not documented, that those operators are usable in a background run. The sources do not state whether a Python save applies the same remapping.

**Operating system.** **not in the sources**, under the search record above. The single platform-named statement found is the OBJ exporter's "except when on a different drive on Windows", which is about export rather than `.blend` storage.


**Observed — hand test entry 3, run 2026-08-05 on the Creator's own computer** (MacBook Pro, M1 Pro chip, macOS 26.5.2, Apple's own graphics), with Blender 5.2.0 LTS started from a terminal with no window, using a throwaway settings folder set through `BLENDER_USER_RESOURCES`. The full list of tests is at `Audits/Blender Research/Dependencies/Hand tests.md`.

**What was done.** An image data-block was given the location `C:\textures\tex.png` on the Creator's Mac, the file was saved, reopened, and the stored text, `bpy.path.abspath` of it, and the image size after `reload()` were printed. The same was done for `/Volumes/NoSuchShare/tex.png`.

**What came back**, the same in all four versions:

```
NAME=drive-letter   STORED='C:/textures/tex.png'                 ABSPATH='C:/textures/tex.png' SIZE=(0, 0)
NAME=no-such-mount  STORED='/Volumes/NoSuchShare/tex.png'        ABSPATH='/Volumes/NoSuchShare/tex.png' SIZE=(0, 0)
```

So a Mac keeps a drive-lettered path, turns its backslashes into forward slashes when the file is saved, treats what is left as an absolute path, and simply does not find the file. Nothing was printed about it.

**Both standing lines, brought up to date by this test.** *Driven from outside:* settled — all of the above was done with no window. *Operating system:* a second operating system is now reachable and the crossing is recorded below. What is still untried is Windows. The macOS-to-Linux half was made on 2026-08-06 and is recorded below. The Windows half needs a Windows machine and none was rented for it. It is in the *cannot be run* section of `Audits/Blender Research/Dependencies/Hand tests.md` and stays unanswered.


**Observed — the crossing between two computers, run for the first time on 2026-08-06.** This is what the question asks and what no earlier test could reach. A `crossing.blend` was built in this version **on the Creator's Mac** at `/Users/artist/…/crossing/5.2/proj/` and saved with `relative_remap=False` so every path stayed exactly as typed. It was then carried whole to the rented Linux machine and opened there in the same version. Both machines were read with the same script.

**What survived the crossing, and what did not.** The result was the same in all four versions.

```
IMAGE rel          stored='//assets/tex.png'
             abspath='/root/…/crossing/5.2/proj/assets/tex.png'
             exists=True size=(32, 32) has_data=True
IMAGE rel-up       stored='//../outer/tex.png'
             abspath='/root/…/crossing/5.2/proj/../outer/tex.png'
             exists=True size=(32, 32) has_data=True
IMAGE abs-mac      stored='/Users/artist/…/crossing/5.2/proj/assets/tex.png'
             abspath='/Users/artist/…/crossing/5.2/proj/assets/tex.png'
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

**And a render does not refuse a foreign output path.** The same file's render output path was a full Mac path. Rendering it on the Linux machine printed `Saved: '/Users/artist/…/crossing/5.2/renders/frame_0001.png'`, finished with exit status 0, and **created the whole Mac-shaped folder tree at the root of the Linux disk** to do it. Nothing warned. That run was made as the root user, who owns the root of that disk, so what happens for a render user who cannot write there is not established.

**Still not established.** What a Windows Blender writes into a path, and what a path written on Windows means on macOS or Linux, or the reverse. No Windows machine has run it.

**Both standing lines, brought up to date by the Linux tests.** *Driven from outside:* every reading above was taken with Blender running with no window unless it says otherwise, so it holds outside the interface. *Operating system:* **now settled on two operating systems, macOS and Linux, and each is its own result** — what they agree on and where they differ is said above. **Windows is still untried: no Windows machine has run it.**

---

## 7. What does a location pointing somewhere other than the computer the file sits on mean, at either end?

**Answer.** Not settled in the sources for paths recorded in a `.blend`. The sources describe network locations only in three places, none of which is about a stored dependency path.

**(a) Render output onto a shared directory — stated as a working method, not as path semantics.**

> "Primitive Render Farm
> An easy way to get multiple machines to share the rendering workload is to:
> Set up a shared directory over a network file system.
> Disable Overwrite, enable Placeholders in the Render Output panel.
> Start as many machines as you wish rendering to that directory."

— Manual, *Output*, https://docs.blender.org/manual/en/5.2/render/output/properties/output.html — read 2026-07-30. This is the only place in the 5.2 manual that describes several computers writing to one location. It says nothing about how the location is *written* in the file, only that a shared directory over a network file system is the arrangement.

**(b) A remote asset library, which is reached by URL rather than by path** — quoted in full under question 2. Note the asymmetry: the remote end is a web server, and what crosses is a downloaded `.blend` that must be self-contained.

**(c) A URL form that Blender does accept, in an unrelated setting.** For extension repositories the manual gives: "file://HOST/share/path/to/packages/index.json for network shares on Windows." — Manual, *Static repository*, https://docs.blender.org/manual/en/5.2/advanced/extensions/creating_repository/static_repository.html — read 2026-07-30. This is a repository index, not a dependency of a `.blend`.

There is also one passing mention of Blender itself being installed on a network location: "read-only network drive or in a system directory" — Manual, *Deploying Blender*, https://docs.blender.org/manual/en/5.2/advanced/deploying_blender.html — read 2026-07-30. Again, not a dependency path.

**LABEL.** **not in the sources**, for what the question asks. **documented** for the three adjacent facts quoted.

**Where I looked.** The same exhaustive search listed under question 6 (`UNC`, `network path`, `network share`, `network drive`, `shared drive`, `NFS`, `SMB`, `mounted`, plus `render farm`, `another machine`) across the whole 5.2 manual and API documentation, restricted to text files. Hits: the four passages above and nothing else. I also read in full `files/blend/open_save.html`'s Relative Paths section, `files/file_paths`, `files/linked_libraries/link_append`, `files/asset_libraries/remote_asset_libraries`, and the `Library` and `PreferencesFilePaths` API pages. The release notes have no entry on network locations for dependencies.

**Not covered.** Nothing here settles whether Blender treats a path on a mounted network volume any differently from a local one; whether it retries or times out; or what the "other end" is expected to provide.

**Outside the interface.** **not in the sources.** The Primitive Render Farm passage describes several machines writing to one shared directory, but it does not say whether Blender is driven with or without its interface. The claim that both settings travel in the file is not carried by that quote.

**Operating system.** **not in the sources** for recorded dependency paths, under the search record above. The adjacent `file://HOST/share/…` form is named specifically for extension repositories on Windows.


**Observed — hand test entry 3, run 2026-08-05 on the Creator's own computer** (MacBook Pro, M1 Pro chip, macOS 26.5.2, Apple's own graphics), with Blender 5.2.0 LTS started from a terminal with no window, using a throwaway settings folder set through `BLENDER_USER_RESOURCES`. The full list of tests is at `Audits/Blender Research/Dependencies/Hand tests.md`.

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

**Answer.** The sources do not describe how Blender resolves a stored path whose letter case does not match what is on disk. They do contain one clear acknowledgement that computers differ on this, in the form of a helper Blender provides to scripts:

> "bpy.path.resolve_ncase(path)
> Resolve a case insensitive path on a case sensitive system, returning a string with the path if found else return the original path."

— API, *Path Utilities*, https://docs.blender.org/api/5.2/bpy.path.html — read 2026-07-30.

A second, narrower one on the same page:

> "bpy.path.ensure_ext(filepath, ext, *, case_sensitive=False) … case_sensitive (bool) – Check for matching case when comparing extensions."

— same page and date.

Data-block *names* are a separate matter and are not paths; the manual states their rules ("A data-block's name is unique within its namespace") without mentioning case — Manual, *Data-Blocks*, https://docs.blender.org/manual/en/5.2/files/data_blocks.html — read 2026-07-30.

**LABEL.** **documented** that the helper exists with that description. **inferred** that Blender treats "a case sensitive system" as a real and different thing from a case-insensitive one — the step is that a function whose entire purpose is to resolve a case-insensitive path on a case-sensitive system would be pointless otherwise. **not in the sources** for whether Blender applies that resolution itself when loading a dependency, and for whether it does so on every operating system.

**Where I looked.** Searched the whole 5.2 manual and API documentation (text files) for `case sensitiv`, `case-sensitiv`, `case insensitiv`, `case-insensitiv`. Every hit is either the two `bpy.path` entries above, a UI list-filter or search option (Outliner search "Case Sensitive", pose/object select by pattern "The matching can be chosen to be case sensitive or not.", `UIList` sorting), the batch-rename tool, or the command-line note "Arguments are case sensitive! -F and -f are not the same." (Manual, *Rendering From The Command Line*, https://docs.blender.org/manual/en/5.2/advanced/command_line/render.html). None of those is about resolving a dependency path. I also read `files/blend/open_save.txt`, `files/file_paths.txt`, `interface/window_system/topbar.txt` (the Find Missing Files description) and `api_5.2/info_gotchas_file_paths_and_encoding.txt` in full, and none mentions letter case.

**Not covered.** It does not settle what Find Missing Files does about case when it searches a directory tree. It does not settle whether a case mismatch produces the same outcome as an absent file.

**Outside the interface.** **inferred** that a script can call `bpy.path.resolve_ncase` outside the interface because it is exposed through Python. Whether it is available in every background context, and whether Blender itself calls it during a render, are **not in the sources**.

**Operating system.** **not in the sources**, under the search record above. The one relevant phrase — "on a case sensitive system" — names no operating system.


**Observed — hand test entry 10, run 2026-08-05 on the Creator's own computer** (MacBook Pro, M1 Pro chip, macOS 26.5.2, Apple's own graphics), with Blender 5.2.0 LTS started from a terminal with no window, using a throwaway settings folder set through `BLENDER_USER_RESOURCES`. The full list of tests is at `Audits/Blender Research/Dependencies/Hand tests.md`.

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
Warning: Could not find 'Texture.PNG' in '/root/…/rescue2/5.2/'
```

and changed nothing. With a search folder holding the right name, the same call repaired every other missing path and **rewrote each one as a full absolute path into the search folder**, leaving paths that already resolved alone. The same in all four versions. The Mac cannot answer this question at all, because on its disk the capitals reference is not missing in the first place.

**Still not established.** Whether Windows behaves like either machine. No Windows machine has run it.

**Both standing lines, brought up to date by the Linux tests.** *Driven from outside:* every reading above was taken with Blender running with no window unless it says otherwise, so it holds outside the interface. *Operating system:* **now settled on two operating systems, macOS and Linux, and each is its own result** — what they agree on and where they differ is said above. **Windows is still untried: no Windows machine has run it.**


**Observed — added by this round's hand tests, run 2026-08-07 on a rented Linux machine** (Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB on driver 580.95.05, an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory, and no display server at all), with Blender 5.2.0 LTS started from a terminal with no window, `--factory-startup`, and a settings folder of its own made first and set through `BLENDER_USER_RESOURCES`. **This is an addition to a document written in an earlier round; nothing above it has been changed.** The full list of tests is at `Audits/Blender Research/Dependencies/Hand tests — added questions.md`.

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
UserWarning: load: '/root/dependencies/work/q8/5.2/lib.blend' does not contain objects["cube"]
```

So capitals are told apart exactly inside a file: three names differing only in case all survive being saved and reopened, and a lookup with the wrong capitals finds nothing. Asking a library for a name with the wrong capitals through `libraries.load` gives `None` and a warning naming the file. Asking through `bpy.ops.wm.link` gives `{'FINISHED'}`, prints nothing at all, and links nothing.

**Both standing lines, brought up to date by this test.** *Driven from outside:* settled — all of it was done with no window. *Operating system:* run on Linux here; **the same test was run on the Creator's Mac on 2026-08-07 and that result, together with what the Mac's own disk does with capitals, is recorded below.** Windows is untested because no Windows machine has run it.

**Observed — the Mac half of this round's hand tests, run 2026-08-07 on the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card.** Blender 5.2.0 LTS was started from a terminal with no window, with `--factory-startup` and a settings folder of its own, made before it was used and set through `BLENDER_USER_RESOURCES`; that this took effect was checked rather than assumed, and this Blender answered `RESOURCES …/depwork/res-5.2` and `VERSION 5.2.0 LTS`. **Other topics were running Blender on this same Mac throughout, so nothing was timed and no memory was measured.** **This is an addition; nothing above it has been changed.** The full list of tests is at `Audits/Blender Research/Dependencies/Hand tests — added questions.md`. In the quotations below the working folder's long path is written `…/depwork`; nothing else is altered.

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
   warning: UserWarning: load: '…/depwork/q8/5.2/lib.blend' does not contain objects["cube"]
LINK asked 'cube' -> got [None]
   warning: UserWarning: load: '…/depwork/q8/5.2/lib.blend' does not contain objects["CUBE"]
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
  asked nosuch.blend   -> RAISED OSError: load: …/depwork/q8disk/5.2/assets/nosuch.blend failed to open blend file
```

the same in all four versions. **On the Creator's Mac's own disk a recorded file name is found whatever its capitals**, for an image and for a linked library alike, and the recorded spelling is kept as written. The fourth line of each block is the control that makes the other three mean something: a name that genuinely is not there fails, and says so.

**So the answer to this question really does depend on the disk and not on Blender, and this project's two machines sit on opposite sides of it.** A file written on the Creator's Mac can record `Texture.PNG` for a file named `texture.png` and work perfectly, and the same file on the rented Linux machine will not find it. Nothing inside the file behaves that way: names inside a `.blend` are exact on both machines.

**Both standing lines, brought up to date by these tests.** *Driven from outside:* settled — every reading here was taken with Blender running with no window. *Operating system:* **settled on both of this project's machines, and each is its own result.** macOS, on its ordinary disk, ignores capitals in a file name on disk; Linux does not; and both tell names inside the file apart exactly. **Windows is untried: no Windows machine has run it.**




---

## 9. Do names written in characters outside the Latin alphabet survive being read out of a file, moved to another computer, and found again by a render?

**Answer.** The sources settle the first part — how such text is stored and read out — and are silent on the second and third.

**Storage and reading out.** The rule for a `.blend` and the deliberate exception for paths:

> "However, this complicates matters for Blender's Python API because .blend files don't have an explicit encoding. To avoid the problem for Python integration and script authors we have decided that all strings in blend-files must be UTF-8, ASCII compatible. This means assigning strings with different encodings to an object name, for instance, will raise an error."
> "Paths are an exception to this rule since the existence of non-UTF-8 paths on the user's file system cannot be ignored. This means seemingly harmless expressions can raise errors, e.g:
> `>>> print(bpy.data.filepath)`
> `UnicodeEncodeError: 'ascii' codec can't encode characters in position 10-21: ordinal not in range(128)`"

and the recommended handling:

> "Use os.fsencode() or os.fsdecode() instead of built-in string decoding functions when operating on paths."
> "To print paths or to include them in the user interface use repr(path) first or "%r" % path with string formatting."

— API, *File Paths & String Encoding*, https://docs.blender.org/api/5.2/info_gotchas_file_paths_and_encoding.html — read 2026-07-30.

For data-block names as against paths, the manual is exact about the cost of non-Latin characters:

> "Data-block names have a fixed length of 255 bytes and are UTF-8 encoded. This is room for 255 basic ASCII characters, or less when using diacritics or non-latin glyphs, which use more than one byte per character. Data-block names cannot contain a Null character (U+0000)."

and, on truncation:

> "In case adding the numeric suffix would make the data-block name too long, the root name part will be shortened as needed."

— Manual, *Data-Blocks*, https://docs.blender.org/manual/en/5.2/files/data_blocks.html — read 2026-07-30.

A related helper strips such characters when a name is to become a filename: `bpy.path.clean_name(name, *, replace='_')` — "Returns a name with characters replaced that may cause problems under various circumstances, such as writing to a file. All characters besides A-Z/a-z, 0-9 are replaced with "_" or the replace argument if defined." — API, *Path Utilities*, https://docs.blender.org/api/5.2/bpy.path.html — read 2026-07-30.

**What is not settled.** Nothing in the 5.2 sources says whether a non-Latin path survives being carried to another computer, nor whether a render finds a file whose name is non-Latin. The encoding page is explicitly a warning to script authors about reading such paths *out*, not a statement about resolving them.

**LABEL.** **documented** for the storage and read-out behaviour quoted. **not in the sources** for surviving a move and being found again by a render.

**Where I looked.** I read `api_5.2/info_gotchas_file_paths_and_encoding.txt` and `api_5.2/bpy.path.txt` in full, and the *Data-Blocks* manual page in full. I searched the whole manual and API documentation for `UTF-8`, `unicode`, `encoding`, `non-latin`, `diacritic`, `fsencode`, `fsdecode`. Hits outside the pages above are about translations of the interface and font glyph coverage, not about resolving paths. The release notes carry no encoding entry.

**Not covered.** It does not settle what happens when a path is valid on the machine that wrote it but not representable on the machine that reads it. It does not settle whether the render error, if any, names the file.

**Outside the interface.** **not in the sources** for the question asked. The quoted Python example and `repr()` advice describe reading and printing a path from Python. They do not establish that the same error occurs in a background render, or that a non-Latin file is found there.

**Operating system.** **not in the sources**, under the encoding search record above. The phrase "the existence of non-UTF-8 paths on the user's file system cannot be ignored" names no operating system.


**Observed — hand test entry 11, run 2026-08-05 on the Creator's own computer** (MacBook Pro, M1 Pro chip, macOS 26.5.2, Apple's own graphics), with Blender 5.2.0 LTS started from a terminal with no window, using a throwaway settings folder set through `BLENDER_USER_RESOURCES`. The full list of tests is at `Audits/Blender Research/Dependencies/Hand tests.md`.

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

**Answer.** Yes. The sections below distinguish paths recorded in the file from destinations chosen by Preferences or a separate command. Those latter destinations are not themselves places the `.blend` points at.

**Render output.**

> "File Path — Choose the location to save rendered frames."
> "This setting expands Relative Paths where a // prefix represents the directory of the current blend-file."

— Manual, *Output*, https://docs.blender.org/manual/en/5.2/render/output/properties/output.html — read 2026-07-30. The API default is `"//"` (`RenderSettings.filepath`, https://docs.blender.org/api/5.2/bpy.types.RenderSettings.html). Combining that default with the manual's definition of `//` supports the **inferred** conclusion that, without a changed setting, the render output is beside the blend-file. The same panel notes that automatic saving can be turned off entirely, in which case "if compositing is enabled, active File Output Node nodes are used to save the result. If no such nodes exist, an error is reported."

Two settings on that panel are the ones the manual's own render-farm hint depends on: "Overwrite — Overwrite existing files when rendering." and "Placeholders — Create empty placeholder frames while rendering." (same page; the manual gives both headings the same run-in title, which reads as an error in the page, but the two descriptions are distinct).

**Compositor File Output node.** `CompositorNodeOutputFile.directory` — "The directory where the image will be written" — https://docs.blender.org/api/5.2/bpy.types.CompositorNodeOutputFile.html — read 2026-07-30.

**Render cache.**

> "Cache Result — Saves the rendered view layers and their passes to a multi-layer OpenEXR image. … The image is stored in the Render Cache folder as specified in the File Paths Preferences."

— Manual, *Output*, address above. The preference: `PreferencesFilePaths.render_cache_directory` — https://docs.blender.org/api/5.2/bpy.types.PreferencesFilePaths.html — read 2026-07-30.

**Cycles texture cache (`.tx` files) — new in 5.2.** Where the write lands is stated twice:

> "Uses .tx files that store images in a tiled, mipmapped format optimized for loading just the tiles and resolutions needed for rendering. By default, files are stored in the blender_tx/ folder next to the image file. This can be changed with the Texture Cache file path preference."
> "Auto Generate — Automatically generate and update .tx files for images when rendering. Files are regenerated when the source image is modified."

— Manual, *Cycles Performance settings*, https://docs.blender.org/manual/en/5.2/render/cycles/render_settings/performance.html — read 2026-07-30.

The release note says the same and adds the command-line route:

> "For every image, a corresponding tx file will be generated in the `blender_tx/` folder next to the image file."

— Release notes, *Cycles*, https://developer.blender.org/docs/release_notes/5.2/cycles/ — read 2026-07-30.

The controlling setting is on the render settings, not on Cycles' own property group. Cycles' UI file draws it from `context.scene.render`:

```
    def draw_header(self, context):
        rd = context.scene.render
        self.layout.prop(rd, "use_texture_cache", text="")
```

— Cycles' own files, `cycles_5.2/ui.py`, lines 892–894 (class `CYCLES_RENDER_PT_performance_texture_cache`, `bl_label = "Texture Cache"`, line 888–889; the Auto Generate row at line 906 and the `render.generate_texture_cache` / `render.clear_texture_cache` buttons at lines 911–912). **observed.**

The matching API descriptions: `RenderSettings.use_auto_generate_texture_cache` — "Automatically create tx files from image files when rendering, if the files do not exist or are outdated. The path to store the texture cache files is configured in the preferences (default False)" — and `PreferencesFilePaths.texture_cache_directory` — "The directory for storing tx files generated from image files, for more efficient rendering. Paths may be absolute, or relative to the image file. Leave blank to store tx files in the same directory as image files" — https://docs.blender.org/api/5.2/bpy.types.RenderSettings.html and https://docs.blender.org/api/5.2/bpy.types.PreferencesFilePaths.html — read 2026-07-30.

The command-line generator is a Cycles file you can read directly:

```
        description=(
            "Generate Cycles texture cache (.tx) files.\n"
            "\n"
            "When a file path is given, generate a tx file for that image.\n"
            "When no file path is given, a blend file must be loaded and tx\n"
            "files are generated for all images used by shader node trees."
        ),
```

and the cache directory argument:

```
        help="Output directory for tx files, relative to source image or absolute.",
```

— Cycles' own files, `cycles_5.2/maketx.py`, lines 14–20 and line 54. It is registered as a command at `cycles_5.2/__init__.py` line 163: `cli_commands.append(bpy.utils.register_cli_command("maketx", maketx_command))`. **observed.**

**Physics disk cache.**

> "When Disk Cache is checked, Blender will save the cache separately to the drive in a folder named blendcache_[filename] alongside the blend-file. (The blend-file must be saved first.)"

and the naming, which matters when several caches share an object:

> "For example, a cache named MyCache will be stored in MyCache_xxxxxx_yy.bphys."
> "For example, a cache on an object Cube will be stored in 43756265_xxxxxx_yy.bphys, where 43756265 is determined by the object name."

— Manual, *Baking*, https://docs.blender.org/manual/en/5.2/physics/baking.html — read 2026-07-30. The same page documents an external, read-only mode: "External — Allows you to read the cache from a drive using a user-specified file path."

**Geometry Nodes bake.**

> "Bake Target … Packed: The baked data is packed into the .blend file. So no separate file is necessary. Disk: The baked data is stored in a separate directory on disk."
> "Bake Path — Location on disk where the baked data for Simulation Zones and Bake Nodes are stored."

— Manual, *Geometry Nodes modifier*, https://docs.blender.org/manual/en/5.2/modeling/modifiers/geometry_nodes.html — read 2026-07-30.

**Others, from their API descriptions** (all https://docs.blender.org/api/5.2/… , read 2026-07-30): `FluidDomainSettings.cache_directory` "Directory that contains fluid cache files"; `OceanModifier.filepath` "Path to a folder to store external baked images"; `DynamicPaintSurface.image_output_path` "Directory to save the textures"; `BakeSettings.filepath` "Image filepath to use when saving externally"; `StripProxy.directory` and `MovieClipProxy.directory` "Location to store the proxy files"; `CollectionExport.filepath` "The file path used for exporting".

**LABEL.** **documented**, except the two Cycles items marked **observed** and the default-output conclusion marked **inferred**.

**Not covered.** The quoted material does not settle what Blender does when most write destinations do not exist or are not writable, nor whether the render output directory is created if absent. It also does not settle which destinations are written during a plain render except where stated: Auto Generate for `.tx` happens “when rendering,” while the physics disk cache and Geometry Nodes bake are bake operations.

**Outside the interface.** **documented** for the `--command maketx` route. **inferred** for other stored write paths: their settings exist in file data, but the quoted sources do not establish that every write behaves identically in a background run. The texture-cache directory is a preference rather than a file setting, and the sources quoted here do not say which preferences a background run uses.

**Operating system.** **not in the sources** for the file-recorded destinations above. No targeted operating-system search is recorded.


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

**Answer.** Yes — a reference into a library is by name, and the name is recorded with its type.

> "id_name — Full ID name in the library .blend file (including the two leading 'id type' chars) (default "", never None)"

— API, `bpy.types.LibraryWeakReference` ("Read-only external reference to a linked data-block and its library file"), https://docs.blender.org/api/5.2/bpy.types.LibraryWeakReference.html — read 2026-07-30.

Names are unique per type per file, which is what makes them usable as a reference:

> "A data-block's name is unique within its namespace. A data-block namespace is defined by its type, and the blendfile it is stored in."
> "This means that there can be for example an Object and a Mesh named the same, but there cannot be two local objects named the same in a blendfile. However, it is possible to have one local and several linked Objects sharing the same name."

— Manual, *Data-Blocks*, https://docs.blender.org/manual/en/5.2/files/data_blocks.html — read 2026-07-30.

**When it is renamed.** The manual provides a repair, and names renaming as the reason:

> "Relocate Linked ID … A right-click a linked ID will show the Relocate option in the ID Data sub-menu. It allows to relocate a directly linked ID by another one, from the same library or from a different one. This can be used to either fix a broken linked data-block (e.g. because the ID was renamed), or to switch to a variation of the same data."
> "This operation is only available on data-blocks that are directly linked, and are not a dependency of any other linked data."

— Manual, *Link & Append*, https://docs.blender.org/manual/en/5.2/files/linked_libraries/link_append.html — read 2026-07-30.

That the repair exists, and cites renaming as the cause, means a rename in the library breaks the reference — Blender does not follow the thing through its rename. **inferred**; the step is from "this fixes a broken linked data-block, e.g. because the ID was renamed" to "a rename breaks it". The sources do not say this outright.

**When it is gone.** The sources directly establish placeholder creation when the whole library is missing:

> "If Blender cannot find a library while loading a blend-file, it will create placeholder data-blocks to replace missing linked ones. That way, references to the missing data are not lost, and by relocating the missing library, the lost data can be automatically restored."

— same page. That passage is about a missing *library*. A per-item placeholder is also offered as an import option:

> "USE_PLACEHOLDERS — Generate a placeholder (empty ID) if not found in any library files."

— API, `bpy.types.BlendImportContext`, https://docs.blender.org/api/5.2/bpy.types.BlendImportContext.html — read 2026-07-30.

And the placeholder is readable as such afterwards:

> "is_missing — This data-block is a place-holder for missing linked data (i.e. it is [an override of] a linked data that could not be found anymore) (default False, readonly)"

— API, `bpy.types.ID`, https://docs.blender.org/api/5.2/bpy.types.ID.html — read 2026-07-30.

**LABEL.** **documented** for the by-name reference, placeholder behaviour when a library is absent, the optional per-item import placeholder, and the `is_missing` report. **inferred** for “a rename breaks the reference,” with the step written out above. **not in the sources** for what a normal file load does when the library exists but the named item has been renamed or removed; no targeted search for that distinction is recorded.

**Not covered.** It does not settle whether a renamed data-block that no longer exists under its old name produces the same placeholder as a wholly absent one, or a different report. It does not settle whether `USE_PLACEHOLDERS` is on for a normal file load — the manual's wording ("it will create placeholder data-blocks") suggests it is for missing libraries, but `BlendImportContext` presents it as an option. It does not say whether a placeholder is silent or reported.

**Outside the interface.** Relocate Library and Relocate Linked ID are documented only as Outliner commands. **inferred** that a script can read `ID.is_missing` and use `BlendImportContext` because they are exposed through Python. The quoted sources do not establish the normal background-load result for a missing named item.

**Operating system.** **not in the sources.** No targeted operating-system search is recorded.


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

**Answer.** The sources cover four specific cases and are silent on the general one.

**(a) A missing library, on load.** Placeholders are made and the reference survives — quoted in full under question 11.

**(b) Missing media in the Video Sequencer.** This is the only place the sources state what a *render* draws:

> "Show Missing Media
> Render missing images/movies with a solid magenta color. When disabled, missing content will render fully transparent."
> "Strips with missing content will be displayed as red in the timeline."

— Manual, *Sequencer preview sidebar*, https://docs.blender.org/manual/en/5.2/editors/video_sequencer/preview/sidebar.html — read 2026-07-30.

**(c) A broken output path template.** Here the render is stopped and does say so:

> "Depending on the path, template errors may prevent certain actions. For example, if there are errors in the render output path, then rendering an animation will fail with an error message indicating the path errors."

— Manual, *File Paths*, https://docs.blender.org/manual/en/5.2/files/file_paths.html — read 2026-07-30.

**(d) An image that cannot be read.** There is a readable signal, though not a rendering behaviour:

> "size — Width and height of the image buffer in pixels, zero when image data cannot be loaded (array of 2 items, in [-inf, inf], default (0, 0), readonly)"

— API, `bpy.types.Image`, https://docs.blender.org/api/5.2/bpy.types.Image.html — read 2026-07-30. The same wording appears on `UDIMTile.size` and `MovieClip.size`. Alongside it, `has_data` — "True if the image data is loaded into memory (default False, readonly)" (same page).

**Does it say so anywhere?** There is a command that answers the question on demand, but it is a command, not something a render does:

> "Report Missing Files — This option is useful to check if there are links to unpacked files that no longer exist. After selecting this option, a warning message will appear in the Info editor's header. If no warning is shown, there are no missing external files."

— Manual, *Topbar*, https://docs.blender.org/manual/en/5.2/interface/window_system/topbar.html — read 2026-07-30. That editor is described as: "The Info editor logs the executed operators as well as errors, warnings, and informational messages." — Manual, *Info Editor*, https://docs.blender.org/manual/en/5.2/editors/info_editor.html — read 2026-07-30.

New in 5.2, those same reports are readable from Python:

> "Blender reports are now exposed as a read-only list in WindowManager ID types. They also now have a session-wide unique identifier integer"

— Release notes, *Python API*, https://developer.blender.org/docs/release_notes/5.2/python_api/ — read 2026-07-30. The property: `WindowManager.reports` — "Collection of reports (default None, readonly)" — https://docs.blender.org/api/5.2/bpy.types.WindowManager.html — read 2026-07-30.

**What is not settled.** What a Cycles or EEVEE render does when an image texture, a volume file, an IES file, a cache file or a font is absent — whether it renders black, renders a substitute, warns, or fails — is not stated anywhere I could find, and neither is whether such an event reaches the standard output of a background run.

**LABEL.** **documented** for (a)–(d) and for the reporting mechanisms. **not in the sources** for the general behaviour of a render on a missing dependency.

**Where I looked.** Searched the whole 5.2 manual for `magenta`, `pink`, `not found`, `cannot be found`, `could not be found`, `unable to open`, `Cannot read`, `fails to load`, `failed to load`, `cannot be loaded`, `invalid image`, `missing file`, `Missing File`, `placeholder`. The only render-behaviour hit is the Sequencer passage quoted. Searched the API documentation for `is_missing`, `has_data`, `cannot be loaded`. Read in full: `editors/info_editor.txt`, `interface/window_system/topbar.txt`, `files/linked_libraries/link_append.txt`, `files/file_paths.txt`, `render/output/properties/output.txt`, `advanced/command_line/render.txt`, `advanced/command_line/arguments.txt` (the environment-variable and argument sections). I also read the 5.2 release notes in full (`index`, `core`, `cycles`, `rendering`, `pipeline_io`, `python_api`, `assets`) and grepped the whole release-notes folder; the closest entries are bug-fix lines such as "Crash when cant find linked files." and "Crash unloading blend file with missing linked sound" in https://developer.blender.org/docs/release_notes/5.2/bugfixes/ , which record that crashes in these situations were fixed but do not describe the resulting behaviour.

**Not covered.** It does not settle whether a render exits with a failure status when something is missing, nor whether the frame is still written.

**Outside the interface.** **not in the sources** for the general missing-dependency result. Report Missing Files is documented as showing a warning in the Info editor, while the API exposes `bpy.ops.file.report_missing_files()` and `WindowManager.reports`. The quoted sources do not say whether those two connect in a background run. The earlier claim that the Info editor does not exist there relied on outside knowledge and is removed.

**Operating system.** **not in the sources.** No targeted operating-system search is recorded.


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

**Answer.** Yes, it does, and the chain can be more than one link long.

**That it happens.** Blender distinguishes what you linked from what came along with it:

> "is_library_indirect — Is this ID block linked indirectly (default False, readonly)"

— API, `bpy.types.ID`, https://docs.blender.org/api/5.2/bpy.types.ID.html — read 2026-07-30.

The append side confirms indirect data is carried and gives the option to flatten it:

> "Localize All — Also copy all indirectly linked data, instead of maintaining the links."

— Manual, *Link & Append*, https://docs.blender.org/manual/en/5.2/files/linked_libraries/link_append.html — read 2026-07-30. And in the API:

> "APPEND_RECURSIVE — Append (make local) also indirect dependencies of appended IDs coming from other libraries. NOTE: All IDs (including indirectly linked ones) coming from the same initial library are always made local."

— API, `bpy.types.BlendImportContext`, https://docs.blender.org/api/5.2/bpy.types.BlendImportContext.html — read 2026-07-30.

**How deep.** A library records a parent library:

> "parent — (readonly) — Type: Library | None"

— API, `bpy.types.Library`, https://docs.blender.org/api/5.2/bpy.types.Library.html — read 2026-07-30.

The 5.2 release notes show a real chain being read, and its members are recorded relative:

> "Some limited library info is now exposed to the `bpy.data.libraries.load()` input context manager, giving in particular access to the blend-file paths of the libraries used by the loaded one"

with the printed example including entries such as `bpy_lib.libraries(filepath='//../../../assets/props/singularity/singularity.blend', is_archive=False)`.

— Release notes, *Python API*, https://developer.blender.org/docs/release_notes/5.2/python_api/ — read 2026-07-30.

Reading the `parent` property as a chain is **inferred**, because its type is another `Library`. No maximum depth is established. The document records no targeted search for nesting, recursion, or depth limits. There is one stated limitation on shape rather than depth:

> "Circular Dependencies — In general, dependencies should not go in both directions. Attempting to link or append data which links back to the current file will likely result in missing links."

— Manual, *Link & Append*, address above.

**Can they be reached the same way — and the trap.** They can be enumerated: `BlendData.file_path_map(*, subset=None, key_types=None, include_libraries=False)` — "include_libraries (bool) – Include library file paths of linked data. False by default." — https://docs.blender.org/api/5.2/bpy.types.BlendData.html — read 2026-07-30 — and `file_path_foreach` has a matching "SKIP_LINKED: Skip Linked. Skip paths of linked IDs." flag (*File Path Foreach Flag Items*, same address).

But they are **not** resolved the same way, and the API says so plainly:

> "When using Blender data from linked libraries there is an unfortunate complication since the path will be relative to the library rather than the open blend-file. When the data block may be from an external blend-file pass the library argument from the bpy.types.ID.
> `>>> bpy.path.abspath(image.filepath, library=image.library)`"

— API, *File Paths & String Encoding*, https://docs.blender.org/api/5.2/info_gotchas_file_paths_and_encoding.html — read 2026-07-30. The matching parameter: "library (bpy.types.Library | None) – The library this path is from. This is only included for convenience, when the library is not None its path replaces start." — API, *Path Utilities*, https://docs.blender.org/api/5.2/bpy.path.html — read 2026-07-30.

The same distinction shows up in the path-template variables, which have library-aware twins: "blend_name_lib: Like blend_name, except that for path properties on library-linked data-blocks it will be the name of the library blend file that the data-block comes from." — Manual, *File Paths*, https://docs.blender.org/manual/en/5.2/files/file_paths.html — read 2026-07-30.

One further place the same question arises, for physics caches: `PointCache.use_library_path` — "Use this file's path for the disk cache when library linked into another file (for local bakes per scene file, disable this option) (default True)" — https://docs.blender.org/api/5.2/bpy.types.PointCache.html — read 2026-07-30; the manual: "Use Library Path — Share the disk cache when the physics object is linked into another blend-file. When this option is enabled, linked versions of the object will reference the same disk cache. Otherwise linked versions of the object will use independent caches." — Manual, *Baking*, https://docs.blender.org/manual/en/5.2/physics/baking.html — read 2026-07-30.

**LABEL.** **documented** for indirect linked data, the `parent` property, enumeration options, library-relative path resolution, and the circular-dependency warning. **inferred** that `parent` establishes a chain. The maximum depth is **not in the sources**, with no targeted search recorded.

**Not covered.** It does not settle how deep the chain may actually go, nor whether Blender detects or reports a very deep one. It does not settle whether a `//` path inside a library that itself came from another library resolves against its own library or the one above it. It does not settle whether indirectly linked data is loaded eagerly or on demand.

**Outside the interface.** **inferred.** The enumeration and path-resolution routes are exposed through Python, and the API tells a script to pass the `library` argument. The quoted sources do not state that every linked-library load behaves identically in a background run.

**Operating system.** **not in the sources.** No targeted operating-system search is recorded.


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

**Answer.** Not in the sources. Nothing in the 5.2 manual, API documentation, release notes or Cycles files states what subset of a file's dependencies is loaded, resolved, or read when one scene of a multi-scene file is rendered.

**Where I looked.** I read `advanced/command_line/render.txt` in full, which documents rendering one scene from outside — "-S, --scene <name> — Set the active scene <name> for rendering." (Manual, *Command Line Arguments*, https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html — read 2026-07-30) — and says nothing about what that loads. I read `files/data_blocks.txt` in full, including its "Life Time" section on reference counting, which describes when data is *kept*, not when it is *read*. I read `files/linked_libraries/link_append.txt` in full, including "Scene-Level Settings — Scene-level settings such as the Rigid Body World will not be copied when linking objects. As an alternative, you can link the entire scene and use it as a Background Scene." I searched the manual and API documentation for `on demand`, `lazily`, `only loads`, `loaded when needed`, `per scene`, `background scene`. I read `render/output/properties/output.txt` and `render/cycles/render_settings/performance.txt` in full.

Two things bear on the question without settling it, and are worth recording:

- The whole of a library file is read even for a small part of it: "Referencing compressed blend-files may need a lot of memory because they have to be loaded in their entirety, even if you only link/append a small part of them. Once the data-blocks are loaded, however, memory usage is the same." — Manual, *Link & Append*, https://docs.blender.org/manual/en/5.2/files/linked_libraries/link_append.html — read 2026-07-30. That is about linking, not about rendering one scene.
- The API's own dependency enumerators are file-wide by default, with an explicit opt-in to narrow them: `file_path_foreach` visits "the file paths used by all ID data-blocks in current bpy.data", and both it and `file_path_map` take a `subset` parameter — "When given, only these data-blocks and their used file paths will be visited." — https://docs.blender.org/api/5.2/bpy.types.BlendData.html — read 2026-07-30. **inferred**, and the step is that Blender's own answer to "what does this file depend on" is framed per-file with narrowing as an option, not per-scene; that is a statement about the enumeration API, not about what a render reads.

**LABEL.** **not in the sources.**

**Not covered.** It settles nothing about texture loading, library loading, or cache reading during a scene render. Anyone needing this must measure it (see the by-hand list).

**Outside the interface.** **documented** that `-S`/`--scene` selects a scene from the command line. Whether only that scene's references are pulled is **not in the sources**, under the search record above.

**Operating system.** **not in the sources.** No targeted operating-system search is recorded.


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

The evidence is one-sided on Linux for the same reason it was on the Mac: it shows that a render of one scene does not complain about the other scene's missing file, not that the file was never opened. **Nothing about this question is settled by the second machine.** **That was true of the test described here, and is no longer true of the question: a test that could come out either way was run on 2026-08-07 and is recorded further down this answer.**

**Both standing lines, brought up to date by the Linux tests.** *Driven from outside:* every reading above was taken with Blender running with no window unless it says otherwise, so it holds outside the interface. *Operating system:* **now settled on two operating systems, macOS and Linux, and each is its own result** — what they agree on and where they differ is said above. **Windows is still untried: no Windows machine has run it.**


**Observed — added by this round's hand tests, run 2026-08-07 on a rented Linux machine** (Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB on driver 580.95.05, an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory, and no display server at all), with Blender 5.2.0 LTS started from a terminal with no window, `--factory-startup`, and a settings folder of its own made first and set through `BLENDER_USER_RESOURCES`. **This is an addition to a document written in an earlier round; nothing above it has been changed.** The full list of tests is at `Audits/Blender Research/Dependencies/Hand tests — added questions.md`.

**Why this test is not the earlier one.** The earlier test on this question watched whether Blender complained about the other scene's missing image. It never did — and in Blender 4.5 the renderer never names a missing image under any circumstances, so that test could not have come out the other way. What follows uses two readings that can each come out either way, and each has a control that worked.

**What was done.** One file was built with two scenes. SceneA holds a camera, a sun and a plane whose material uses `assets/a.png`. SceneB holds the same with `assets/b_small.png`, and also an object linked from `assets/lib.blend`. Both scenes are Cycles on the processor, one sample, 64 by 64. Beside them sits `assets/b_big.png`, a 6000 by 6000 image.

*The first reading — what is read from disk.* Each render was run under `strace`, which records every file the program opens:

```
strace -f -e trace=openat,newfstatat -o <trace file> \
  /opt/blender/5.2/blender --background --factory-startup <file> \
  --scene SceneA --render-output <folder>/SceneA_ --render-frame 1
```

and the same with `--scene SceneB`. Each trace was then searched for `a.png`, `b_small.png` and `lib.blend`.

*The second reading — how much memory the run needs.* `b_big.png` was copied over `b_small.png`, so the image only SceneB uses became 144 MB of picture, and the SceneA render was run three times under `/usr/bin/time -f "maxRSS=%MkB"`; then the small one was put back and it was run three times again. The same was done with `--scene SceneB` as the control.

**What came back.** The trace, with the assets folder trimmed off the front of each name and identical lines counted:

```
--- SceneA : trace lines naming the three assets ---
      6 openat(AT_FDCWD, "a.png", O_RDONLY) = 3
      1 openat(AT_FDCWD, "lib.blend", O_RDONLY) = 3
--- SceneB : trace lines naming the three assets ---
      6 openat(AT_FDCWD, "b_small.png", O_RDONLY) = 3
      1 openat(AT_FDCWD, "lib.blend", O_RDONLY) = 3
```

The image belonging to the scene that was **not** being rendered was never opened at all, in either direction. The linked library was opened in both.

The memory, peak resident size of the whole run:

```
SceneA, where the big image is NOT used : 382/383/382 MB with the big one, 382/382/383 MB with the small one
SceneB, where the big image IS used     : 530/530 MB with the big one, 383/383 MB with the small one
```

Swapping a 32 by 32 image for a 6000 by 6000 one changed the runs of the scene that does not use it by nothing, and the control by well over a hundred megabytes. The machine was shared with other work while this ran, which is why three runs of each were taken; the variation between runs of the same thing is far smaller than the control's jump.

So, on this machine and in this version: **rendering one scene does not read the other scene's image from disk at all, and does not carry it in memory.** A `.blend` **linked as a library is opened whichever scene is rendered**, since it is read when the file is loaded rather than when a scene is drawn.

**Not settled by this.** It covers an image used by a scene's material and a linked library, and nothing else a file can point at. It does not cover a scene reached from the rendered one — as a background scene, through a compositor Render Layers node, or through a sequencer strip — which is a different case the answer above already describes.

**Both standing lines, brought up to date by this test.** *Driven from outside:* settled — the whole test was done with no window, and `--scene` chose the scene there. *Operating system:* Linux, on the machine named above. **The Creator's Mac was reached on 2026-08-07 by a third reading, and it agrees; that is recorded below.** Neither of the two readings used here could be taken on the Mac — the trace needs an administrator password, and the memory reading needs a machine nobody else is using. Windows is untested because no Windows machine has run it.

**Observed — the Mac half of this round's hand tests, run 2026-08-07 on the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card.** Blender 5.2.0 LTS was started from a terminal with no window, with `--factory-startup` and a settings folder of its own, made before it was used and set through `BLENDER_USER_RESOURCES`; that this took effect was checked rather than assumed, and this Blender answered `RESOURCES …/depwork/res-5.2` and `VERSION 5.2.0 LTS`. **Other topics were running Blender on this same Mac throughout, so nothing was timed and no memory was measured.** **This is an addition; nothing above it has been changed.** The full list of tests is at `Audits/Blender Research/Dependencies/Hand tests — added questions.md`. In the quotations below the working folder's long path is written `…/depwork`; nothing else is altered.

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

**Answer.** Yes. The active list of colour spaces depends on an external configuration. The claim that the selected colour-space name is stored in the `.blend` is **inferred** from the per-image setting; the quoted words do not say where that setting is stored.

**The dependency.** The manual states the dependency directly, in two places:

> "The list of color spaces depends on the active OCIO config."

— Manual, *Image settings*, https://docs.blender.org/manual/en/5.2/editors/image/image_settings.html — read 2026-07-30, and again on the Environment Texture node page, https://docs.blender.org/manual/en/5.2/render/shader_nodes/textures/environment.html.

The setting itself is per-image and stored in the file: "Color Space — Specifies the Color Space that the image file was saved in. This information is used to correctly convert the image to Blender's internal linear color space, which is used for all color computations and rendering." (Image settings page, same date).

**How Blender finds the configuration.** Three steps, all documented on one page:

> "Blender comes with a standard OpenColorIO configuration that contains a number of useful display devices and view transforms."
> "Blender will use the standard OCIO environment variable to read an OpenColorIO configuration other than the default Blender one."
> "There is also a BLENDER_OCIO environment variable to change the configuration for Blender only. It is recommended to use OCIO when possible for compatibility with other software and pipelines that may not be aware of BLENDER_OCIO."

— Manual, *OpenColorIO*, https://docs.blender.org/manual/en/5.2/render/color_management/opencolorio.html — read 2026-07-30.

The precedence between the two variables is stated in the command-line documentation:

> "BLENDER_OCIO: Path to override the OpenColorIO configuration file. If not set, the OCIO environment variable is used."

— Manual, *Command Line Arguments*, https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html — read 2026-07-30.

The bundled default lives at a known place in Blender's own layout:

> "./datafiles/colormanagement — Default OpenColorIO configuration."

— Manual, *Blender's Directory Layout*, https://docs.blender.org/manual/en/5.2/advanced/blender_directory_layout.html — read 2026-07-30.

The same page notes that datafiles are layered: "Located in both user and system directories. User data files either override or add to system data files."

**What a differing configuration costs.** The manual states consequences of a configuration that lacks certain features, which is the practical face of this dependency:

> "In older configurations without this, display emulation will be disabled and wide gamut and HDR display will not work well. Saving colorspace metadata for images and video also depends on this."
> "For every display, include a view transform without tone mapping. Blender will look for a view transform named Standard or Un-tone-mapped or the config wide default_view_transform. If not found, the first view transform of the display will be used."

— Manual, *OpenColorIO*, address above.

5.2 changed how the list is presented, per the release notes:

> "Color space are now organized into menus, based on the `Family` attribute in OpenColorIO configurations."

— Release notes, *Rendering*, https://developer.blender.org/docs/release_notes/5.2/rendering/ — read 2026-07-30.

**LABEL.** **documented** for dependence on the active configuration and for how Blender selects that configuration. **inferred** for storage of the selected colour-space name in the `.blend`.

**Not covered.** It does not settle what happens when a colour space named in the file is absent from the active configuration — the "not found" fallback quoted above is about *view transforms*, not about a named colour space on an image. It does not settle whether the configuration used is recorded in the file. It does not say whether the environment variable is read once at start or re-read.

**Outside the interface.** **documented** that both variables are part of the command-line environment. **inferred** that the same configuration-selection rule therefore holds in a background run. The quoted words do not directly compare runs with and without the interface.

**Operating system.** **inferred** that the variables are intended without platform qualification because the same page qualifies other variables by platform. No targeted operating-system search is recorded, so identical behaviour on every operating system is **not in the sources**.


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

**Answer.** A font is recorded exactly as any other external file: as a path on a data-block, which may be relative, and which may be packed.

**How it is recorded.**

> "filepath — (default "", never None, blend relative // prefix supported)"

— API, `bpy.types.VectorFont` ("Vector font for Text objects"), https://docs.blender.org/api/5.2/bpy.types.VectorFont.html — read 2026-07-30. The same page gives it its own pack and unpack methods: "pack() — Pack the font into the current blend file" and "unpack(*, method='USE_LOCAL') — Unpack the font to the samples filename".

The manual's data-block table confirms both linking and packing for the type: "Font ✓ ✓ — References font files. Used by curve object-data of text objects." — Manual, *Data-Blocks*, https://docs.blender.org/manual/en/5.2/files/data_blocks.html — read 2026-07-30.

The quoted API establishes a path to a file. It does not establish the stronger negative claim that no other system-font reference exists. For Unix, the manual says where font files are typically found:

> "Location of Fonts on Unix
> Fonts are typically located under /usr/lib/fonts, or some variant like /usr/lib/X11/fonts, but not always. They may be in other locations as well, such as /usr/share/local or /usr/local/share, and possibly related sub-trees."

— Manual, *Text properties*, https://docs.blender.org/manual/en/5.2/modeling/texts/properties.html — read 2026-07-30.

The starting point for browsing is a preference: `PreferencesFilePaths.font_directory` — "The default directory to search for loading fonts (default "//", blend relative // prefix supported)" — https://docs.blender.org/api/5.2/bpy.types.PreferencesFilePaths.html — read 2026-07-30. The manual's screenshot of that preferences section shows the Fonts field filled with `/usr/share/fonts/` (image `_images/editors_preferences_section_file-paths.webp`, on https://docs.blender.org/manual/en/5.2/editors/preferences/file_paths.html — viewed 2026-07-30). **documented**, and only that: what the field contained in the screenshot. I note the picture also does not show the Texture Cache field that the 5.2 text on the same page describes, so I would not read a full list of fields off it.

**What Blender does when a font is missing.** Not stated. The nearest documented facts are two, and neither answers it:

> "Blender comes with a built-in font by default that is displayed in each of the four font style data-block menus. The built-in font is always present and shows in this list as "Bfont"."

and

> "If you select a font that is unsupported by Blender, you will get the error Not a valid font."

— Manual, *Text properties*, address above. The second is about an unsupported file that *is* present, not an absent one.

**LABEL.** **documented** for how a font is recorded. **not in the sources** for what Blender does when the font file is absent.

**Where I looked, for the missing-font part.** Read `modeling/texts/properties.txt` and `modeling/texts/editing.txt` in full, and `modeling/texts/introduction.txt`. Read `api_5.2/bpy.types.VectorFont.txt` in full. Searched the whole manual and API documentation for `Bfont`, `Not a valid font`, `font` combined with `missing`, and the general missing-file terms listed under question 12. The only file-absence machinery that names fonts is the generic one: Find Missing Files, whose file-browser filter arguments include `filter_font` (`bpy.ops.file.find_missing_files`, https://docs.blender.org/api/5.2/bpy.ops.file.html — read 2026-07-30), which tells us fonts are within its scope but not what happens before it is run. The release notes contain no font entry.

**Not covered.** It does not settle whether a missing font falls back to Bfont, renders nothing, or errors. It does not settle whether the recorded path for a system font is written absolute or relative by default — the general default is relative (question 5), but a system font directory will rarely sit under the blend-file, and the sources do not say what happens then.

**Outside the interface.** **inferred** that the recorded path remains part of the file in a background run. The quoted “Not a valid font” words do not say whether that error is tied to the interface. Missing-font behaviour and reporting in a background render are **not in the sources** under the search record above.

**Operating system.** Partly answered: the manual gives typical font locations for Unix only. The document records no targeted search for equivalent locations or missing-font behaviour on other operating systems.


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

**Closed on 2026-08-07, by this round's hand tests.** Under question 8, whether capitals matter in a name **inside** a file, which the answer above had listed as not covered: they do, exactly, and asking a library for a name with the wrong capitals through `bpy.ops.wm.link` reports success and links nothing. Under question 14, whether rendering one scene reads only that scene's references: the earlier test could not have come out the other way, and a new one that could — a record of every file each run opened, and the memory a run needs when the other scene's image is made 144 MB — was run on the rented Linux machine and settles it. Both results are written into the answers above, and the list is at `Audits/Blender Research/Dependencies/Hand tests — added questions.md`.

**Not closed by any test.** Whether the list under question 1 is the whole list: the test file was built
to hold seven kinds, so a kind nobody thought to include would not have appeared.

---

2026-08-04 — Opus 5 (Claude Code) — Dependencies, Blender 5.2

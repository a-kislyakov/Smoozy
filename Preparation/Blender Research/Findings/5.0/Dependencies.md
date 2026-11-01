# Dependencies — Blender 5.0

**Sources consulted, and when.** Blender 5.0 manual (plain-text copy, including its `_images` pictures), Blender 5.0 Python API documentation, Blender 5.0 release notes, and Cycles' own files as shipped with Blender 5.0 — all as supplied in `Blender Sources/`, snapshot taken **2026-07-30**, which is the date read for every finding below. I also read `Blender Sources/requirements/requirements_2026-08-01.txt`, which is not per-version but names Blender 5.0 in so many words; it is cited only where it does.

**I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.**

I worked alone — no helper agents were started, so nothing below depends on anyone I cannot vouch for.

A note on how the labels are used here. Where I write **documented**, the quoted words carry the claim on their own. Where a reader would have to supply anything to get from the quote to the claim, I wrote **inferred** and set out the step. Several answers are split, because part of a question is answered outright and part is not.

---

## 1. What kinds of thing can a `.blend` file point at outside itself? Give the full list.

**Answer.** Blender 5.0's own documentation does not print a single closing list of "everything a blend-file can point at". What it does give is (a) a table of data-block types with a column saying which ones reference a file, (b) a Python call whose whole purpose is to walk every file path in a blend-file, and (c) per-property documentation for each path. Assembling those gives the list below.

The general statement:

> "External data, like texture images and other resources, can be stored inside the blend-file (packed) or as separate files (unpacked). Blender keeps track of all unpacked resources via a relative or absolute path."
> — Manual, https://docs.blender.org/manual/en/5.0/interface/window_system/topbar.html (read 2026-07-30)

The data-block table marks a "Pack" column, defined as:

> "Pack
> File Packing, supports file contents being packed into the blend-file
> (not applicable for most data-blocks which have no file reference)."
> — Manual, https://docs.blender.org/manual/en/5.0/files/data_blocks.html (read 2026-07-30)

Six types carry a ✓ or ✗ in that column: **Font** ("References font files.") ✓, **Image** ("Image files.") ✓, **Library** ("References to an external blend-file.") ✓, **Sounds** ("Reference to sound files.") ✓, **Movie Clip** ("Reference to an image sequence or video file.") ✗, and **Text** ("Text data.") ✗. Same page, same address, read 2026-07-30. Reading a "—" as meaning "this type has no file reference" is a step past the legend, which says only that packing is "not applicable for *most* data-blocks which have no file reference".

Worth flagging as a discrepancy rather than smoothing over: **Cache File** is marked "—" in the Pack column and described only as "Used by Mesh Cache modifiers.", yet the API documents `CacheFile.filepath`; and **Volume** does not appear in the table at all, even though the API documents `Volume.filepath` and `Volume.packed_file`. So the table is not a complete inventory of file-referencing types in 5.0. *(Citation check 2026-08-05: the table does not describe Cache File as referencing an external archive — that description was not in the source — and the legend's "most" was reinstated.)*

The picture on that page, `_images/files_data-blocks_id-types.png` (Manual, https://docs.blender.org/manual/en/5.0/files/data_blocks.html, read 2026-07-30), shows the ID-type list with icons. Visible in it, among others: Library, Font, Image, Sound, Movie Clip, Cache File, Text. **documented** (from a picture, so: what is visible in it is what I report). The picture does not show a Volume type even though the API documents one — I report that as an absence in the picture, nothing more.

Blender 5.0 also added a call that exists precisely because there is a set of "all file paths used by a blend-file":

> "file_path_map(*, subset=None, key_types=None, include_libraries=False)
> Returns a mapping of all ID data-blocks in current bpy.data to a set of all file paths used by them."
> — Python API, https://docs.blender.org/api/5.0/bpy.types.BlendData.html (read 2026-07-30)

> "Add `bpy.data.file_path_foreach()`, which calls a callback function for each file path used in the blend file"
> — Release notes, https://developer.blender.org/docs/release_notes/5.0/python_api/ (read 2026-07-30)

**The list.** Searching the whole of the 5.0 API documentation for the exact phrase Blender uses to mark a stored, blend-relative path — `blend relative // prefix supported` — returns these types. Each entry below gives its full address and date read.

*Things read from:*
- `Library.filepath` — "Path to the library .blend file" (Python API, https://docs.blender.org/api/5.0/bpy.types.Library.html, read 2026-07-30)
- `Image.filepath` / `Image.filepath_raw` — "Image/Movie file name" (Python API, https://docs.blender.org/api/5.0/bpy.types.Image.html, read 2026-07-30)
- `VectorFont.filepath` (Python API, https://docs.blender.org/api/5.0/bpy.types.VectorFont.html, read 2026-07-30)
- `Sound.filepath` — "Sound sample file used by this Sound data-block" (Python API, https://docs.blender.org/api/5.0/bpy.types.Sound.html, read 2026-07-30)
- `MovieClip.filepath` — "Filename of the movie or sequence file" (Python API, https://docs.blender.org/api/5.0/bpy.types.MovieClip.html, read 2026-07-30)
- `Volume.filepath` — "Volume file used by this Volume data-block"; `VolumeGrids.frame_filepath` (Python API, https://docs.blender.org/api/5.0/bpy.types.Volume.html and https://docs.blender.org/api/5.0/bpy.types.VolumeGrids.html, read 2026-07-30)
- `CacheFile.filepath` and `CacheFileLayer.filepath` — "Path to the archive" (Python API, https://docs.blender.org/api/5.0/bpy.types.CacheFile.html and https://docs.blender.org/api/5.0/bpy.types.CacheFileLayer.html, read 2026-07-30)
- `MeshCacheModifier.filepath` and `MultiresModifier.filepath` — "Path to external displacements file" (Python API, https://docs.blender.org/api/5.0/bpy.types.MeshCacheModifier.html and https://docs.blender.org/api/5.0/bpy.types.MultiresModifier.html, read 2026-07-30)
- `ShaderNodeScript.filepath` — "Shader script path" (Python API, https://docs.blender.org/api/5.0/bpy.types.ShaderNodeScript.html, read 2026-07-30)
- `ShaderNodeTexIES.filepath` — "IES light path" (Python API, https://docs.blender.org/api/5.0/bpy.types.ShaderNodeTexIES.html, read 2026-07-30)
- `ImageStrip.directory`, `MovieStrip.filepath`, `StripProxy.filepath` / `StripProxy.directory`, `SequenceEditor.proxy_dir`, and `MovieClipProxy` (Python API, https://docs.blender.org/api/5.0/bpy.types.ImageStrip.html, https://docs.blender.org/api/5.0/bpy.types.MovieStrip.html, https://docs.blender.org/api/5.0/bpy.types.StripProxy.html, https://docs.blender.org/api/5.0/bpy.types.SequenceEditor.html, and https://docs.blender.org/api/5.0/bpy.types.MovieClipProxy.html, read 2026-07-30)
- `PointCache.filepath` / `PointCacheItem` — "Cache file path" (Python API, https://docs.blender.org/api/5.0/bpy.types.PointCache.html and https://docs.blender.org/api/5.0/bpy.types.PointCacheItem.html, read 2026-07-30)
- `FluidDomainSettings.cache_directory` — "Directory that contains fluid cache files" (Python API, https://docs.blender.org/api/5.0/bpy.types.FluidDomainSettings.html, read 2026-07-30)
- `LibraryWeakReference.filepath` — "Path to the library .blend file" (Python API, https://docs.blender.org/api/5.0/bpy.types.LibraryWeakReference.html, read 2026-07-30)
- `Text.filepath` — "Filename of the text file" (Python API, https://docs.blender.org/api/5.0/bpy.types.Text.html, read 2026-07-30; this one is documented **without** the "blend relative // prefix supported" marker)

*Things written to (see Q10):*
- `RenderSettings.filepath`, `BakeSettings.filepath`, `CompositorNodeOutputFile` base path, `OceanModifier.filepath`, `DynamicPaintSurface.image_output_path`, `NodesModifier.bake_directory` / `NodesModifierBake.directory`, and `CollectionExport` (Python API, https://docs.blender.org/api/5.0/bpy.types.RenderSettings.html, https://docs.blender.org/api/5.0/bpy.types.BakeSettings.html, https://docs.blender.org/api/5.0/bpy.types.CompositorNodeOutputFile.html, https://docs.blender.org/api/5.0/bpy.types.OceanModifier.html, https://docs.blender.org/api/5.0/bpy.types.DynamicPaintSurface.html, https://docs.blender.org/api/5.0/bpy.types.NodesModifier.html, https://docs.blender.org/api/5.0/bpy.types.NodesModifierBake.html, and https://docs.blender.org/api/5.0/bpy.types.CollectionExport.html, read 2026-07-30).

A second, independent corroboration of the kinds of external file Blender knows about is the filter set on its own "find missing files" operator:

> "filter_blender … filter_backup … filter_image … filter_movie … filter_python … filter_font … filter_sound … filter_text … filter_archive … filter_btx … filter_alembic … filter_usd … filter_obj … filter_volume … filter_folder … filter_blenlib"
> — Python API, `bpy.ops.file.find_missing_files`, https://docs.blender.org/api/5.0/bpy.ops.file.html (read 2026-07-30)

**Label:** entries carrying quoted property words are **documented**. Names listed without quoted property words, and the claim that this is the *full* list, are **inferred** from the recorded whole-document search. The step: Blender marks stored, blend-relative paths in its API documentation with the fixed phrase "blend relative // prefix supported"; I searched the whole 5.0 API doc set for that phrase and took the results as members of the list. Treating the results as complete assumes the marker is applied without omission, which the documentation nowhere promises.

**Not covered.** Whether add-ons can register further path properties that carry the same marker (they can define properties, but nothing I read says whether those participate in `file_path_map`). Also not covered: paths that Blender reads at startup rather than from the blend-file — the OpenColorIO config (Q15), user script directories, extension repositories. Those are pointed at by the *installation*, not by the file. `Text.filepath` lacking the relative-path marker is a documented difference I cannot explain from the sources.

**Outside the interface.** The sources do not say that this list differs when Blender is driven from outside its window. Two documented differences touch it: "The audio device is disabled in background-mode by default" (Manual, https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html, read 2026-07-30), and Python auto-execution is off by default — "-Y, --disable-autoexec: Disable automatic Python script execution (Python-drivers & startup scripts), (default)." (same page). Neither says the *paths* stop being recorded.

**Operating system.** The sources do not say this list differs by operating system.


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

**Both standing lines, brought up to date by the Linux tests.** *Driven from outside:* every reading above was taken with Blender running with no window unless it says otherwise, so it holds outside the interface. *Operating system:* **now settled on two operating systems, macOS and Linux, and each is its own result** — what they agree on and where they differ is said above. **Windows is still untried: no Windows machine has run it.** **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

---

## 2. What kinds of library can a file point at, including one that does not sit on the same computer?

**Answer.** In Blender 5.0 a "library" in the blend-file sense is one thing — another `.blend` file:

> "External .blend file from which data is linked"
> — Python API, `bpy.types.Library`, https://docs.blender.org/api/5.0/bpy.types.Library.html (read 2026-07-30)

Blender 5.0 also introduced an *archive* library data-block, but the quoted description says it is owned and managed by the ordinary library. It does not establish another kind of external library that the file points at:

> "Name collisions are averted by storing these packed IDs into a new type of 'archive' libraries (and their namespaces). These libraries:
> * Only contain packed IDs.
> * Are owned and managed by their 'real' library data-block, called an 'archive parent'."
> — Release notes, https://developer.blender.org/docs/release_notes/5.0/core/ (read 2026-07-30)

and the API exposes both sides of that:

> "archive_libraries — Archive libraries of packed IDs, generated (and owned) by this source library"
> "is_archive — This library is an 'archive' storage for packed linked IDs originally linked from its 'archive parent' library."
> — Python API, https://docs.blender.org/api/5.0/bpy.types.Library.html (read 2026-07-30)

Separately, and *not* a thing the file points at, there is the **asset library**, which is a registered folder in the Preferences:

> "An asset library is a directory on your drive that is registered in the Preferences as an asset library."
> — Manual, https://docs.blender.org/manual/en/5.0/files/asset_libraries/introduction.html (read 2026-07-30)

> "path — Path to a directory with .blend files to use as an asset library"
> — Python API, `bpy.types.UserAssetLibrary`, https://docs.blender.org/api/5.0/bpy.types.UserAssetLibrary.html (read 2026-07-30)

**On "not on the same computer".** The sources contain no notion of a library reached by a network address, a URL, or any protocol. A library is a file path, and the only way the sources describe reaching another machine is an ordinary mounted path. The two places that acknowledge this:

> "Volumes
> Drives and network mounts."
> — Manual, File Browser bookmarks, https://docs.blender.org/manual/en/5.0/editors/file_browser.html (read 2026-07-30)

> "Set up a shared directory over a network file system."
> — Manual, "Primitive Render Farm" hint, https://docs.blender.org/manual/en/5.0/render/output/properties/output.html (read 2026-07-30)

Blender's only documented internet-facing setting is about add-ons, not file references:

> "Allow Online Access — Allow Blender to access the internet. Add-ons that follow this setting will only connect to the internet if enabled."
> — Manual, https://docs.blender.org/manual/en/5.0/editors/preferences/system.html (read 2026-07-30)

**Label:** **documented** that an external library is another `.blend` file, that archive library data-blocks exist inside that arrangement, and that an asset library is a preference-registered directory rather than a dependency. **Not in the sources** for how a library on another computer is reached. The mounted-path explanation is a possible reading of the fragments, not a finding they carry.

**Where I looked, for the silent part.** Searched the whole manual, API docs and release notes for: `network drive`, `network share`, `network path`, `network location`, `networked`, `remote server`, `shared drive`, `over the network`, `mount`, `NAS`, `URI`, `file://`, `smb://`, `http`, `drive letter`. The only hits are the three quoted above plus `manual_5.0/advanced/extensions/creating_repository/static_repository.txt` (which uses `file://` URLs for *extension repositories*, an add-on distribution mechanism, not a blend-file dependency) and a 5.0 bugfix note: "Reading mmap'ed images from network drives may crash Blender when the file is not valid" (Release notes, https://developer.blender.org/docs/release_notes/5.0/bugfixes/, read 2026-07-30).

**Not covered.** Whether a library path may be a Windows UNC path (`\\server\share\...`) — the sources never write one. Whether locking, latency or partial reads over a network mount change behaviour. Whether an archive library can itself be relocated.

**Outside the interface.** The sources do not say. Nothing in the library documentation is described as interface-only, but nothing states it holds in background mode either.

**Operating system.** "Drives and network mounts" is the only OS-touching phrase, and it does not distinguish systems. The sources do not say.


**Observed — hand test entry 1, run 2026-08-05 on the Creator's own computer** (MacBook Pro, M1 Pro chip, macOS 26.5.2, Apple's own graphics), with Blender 5.0.1 started from a terminal with no window, using a throwaway settings folder set through `BLENDER_USER_RESOURCES`. The full list of tests is at `Audits/Blender Research/Dependencies/Hand tests.md`.

**What was done.** `Library.is_archive` and `Library.archive_libraries` were looked for by name, in all four versions.

**What came back.** Both exist here, and in the other 5.x versions. Neither exists in 4.5.12.

**Both standing lines, brought up to date by this test.** *Driven from outside:* the reading was taken with no window. *Operating system:* macOS for this test. The same test was run on a rented Linux machine on 2026-08-06 and that result is recorded below; Windows stays untested.

**What this does not settle, and cannot be settled here.** Whether a library that does not sit on the same computer can be reached. That needs a second computer and a shared place both can reach; the Creator has one computer and no share, so it is in the *cannot be run* section of `Audits/Blender Research/Dependencies/Hand tests.md` and stays unanswered. The one adjacent thing that was tested is in question 7 below: a path naming a volume that is not mounted is stored and resolved unchanged, and the file is simply not found.


**Observed — run again on Linux, 2026-08-06.** The two archive-library names were asked for again in this version on the rented Linux machine, and came back exactly as on the Creator's Mac: absent in 4.5, present in 5.0, 5.1 and 5.2. Nothing about which kinds of library exist differs between the two machines.

**The half about a library that does not sit on the same computer is still not settled**, and now for a narrower reason: there are two machines but no folder on network storage that both can reach. What the Linux machine does with a path naming a volume that is not mounted is the same as the Mac: `/Volumes/NoSuchShare/tex.png` is kept exactly as written, resolved exactly as written, and the file is not found. Report Missing Files names it.

**Both standing lines, brought up to date by the Linux tests.** *Driven from outside:* every reading above was taken with Blender running with no window unless it says otherwise, so it holds outside the interface. *Operating system:* **now settled on two operating systems, macOS and Linux, and each is its own result** — what they agree on and where they differ is said above. **Windows is still untried: no Windows machine has run it.**

---

## 3. Can a file hold the things it depends on inside itself partly rather than wholly, and what does a partly-held file report when it is read?

**Ambiguity.** "Partly rather than wholly" reads two ways. (a) *Some* of the dependencies are held inside and the rest are still outside. (b) A single dependency is held inside only in part. I answer (a), which I think is meant, because Blender has an explicit feature for it. Reading (b) I answer at the end: nothing in the sources describes a half-stored single file.

**Answer (a): yes, packing is per-item.**

> "Selective Packing
> A single file can be packed by clicking on the little "gift box" icon to the left of its file-path UI widget."
> — Manual, https://docs.blender.org/manual/en/5.0/files/blend/packed_data.html (read 2026-07-30)

and the whole-file operation is opt-in and deferred:

> "Mark all eligible external resource files used by the blend-file as packed. Actual packing will happen on the next save of the blend-file."
> — same page and address, read 2026-07-30

Some things *cannot* be packed at all, so a fully-packed file is not always achievable:

> "Not all external files can be packed
> Some typically heavy external files, like videos from the Sequence Editor or Movie Clips, cannot be packed in a blend-file."
> — same page and address, read 2026-07-30

The data-block table backs that: Movie Clip and Text are marked "✗" in the Pack column, against "✓" for Font, Image, Library and Sounds (Manual, https://docs.blender.org/manual/en/5.0/files/data_blocks.html, read 2026-07-30).

Linked data can be packed independently of resources, which is a second axis of partial holding:

> "Pack Linked Libraries — Mark all linked library files in the current blend-file as packed."
> — Manual, https://docs.blender.org/manual/en/5.0/files/blend/packed_data.html (read 2026-07-30)

> "Store all data-blocks linked from other .blend files in the current .blend file. Library references are preserved so the linked data-blocks can be unpacked again"
> — Python API, `bpy.ops.file.pack_libraries`, https://docs.blender.org/api/5.0/bpy.ops.file.html (read 2026-07-30)

**What it reports when read.** Three separate reports exist.

*Per data-block, in the API:*
> "is_linked_packed — This data-block is linked and packed into the .blend file"
> "is_missing — This data-block is a place-holder for missing linked data (i.e. it is [an override of] a linked data that could not be found anymore)"
> — Python API, `bpy.types.ID`, https://docs.blender.org/api/5.0/bpy.types.ID.html (read 2026-07-30)

*Per image, the packed content itself:*
> "packed_file — First packed file of the image"
> "packed_files — Collection of packed images"
> — Python API, https://docs.blender.org/api/5.0/bpy.types.Image.html (read 2026-07-30)

*Across the file, on demand:*
> "This option is useful to check if there are links to unpacked files that no longer exist. After selecting this option, a warning message will appear in the Info editor's header. If no warning is shown, there are no missing external files."
> — Manual, "Report Missing Files", https://docs.blender.org/manual/en/5.0/interface/window_system/topbar.html (read 2026-07-30)

Note what that last quote settles: the report is about **unpacked** files only, and it is produced when a person asks for it, not automatically on read. Separately, the path-walking API has an option to skip paths whose matching data is packed:

> "SKIP_PACKED: Skip Packed. Skip paths when their matching data is packed."
> — Python API, https://docs.blender.org/api/5.0/bpy_types_enum_items/file_path_foreach_flag_items.html (read 2026-07-30)

**Label:** **documented** throughout. No claim is made that Report Missing Files uses the path-walking option; the sources do not join those two features.

**Answer (b), the other reading.** **Not in the sources.** Nothing describes a single external file being stored inside a blend-file in part. `PackedFile.data` is documented as "Raw data (bytes, exact content of the embedded file)" (Python API, https://docs.blender.org/api/5.0/bpy.types.PackedFile.html, read 2026-07-30) — "exact content" is a whole-file phrase, but it is a description of the property, not a statement that partial storage is impossible. *Where I looked:* searched manual, API and release notes for `partial`, `partially`, `packed`, `pack`, and read `files/blend/packed_data.txt`, `bpy.types.PackedFile`, `bpy.types.ImagePackedFile`, `bpy.ops.file.*` in full.

**Not covered.** Whether a packed item that later changes on disk is detected. Whether "Report Missing Files" runs at all without a window (the quote names the Info editor's header, which is an interface place). Whether the packed copy or the on-disk file wins when both exist and differ — the unpack options mention "(differs)|(identical)" but that is about unpacking, not about reading.

**Outside the interface.** Partly answered. `bpy.ops.file.report_missing_files` ("Report all missing external files") and `bpy.ops.file.pack_all` exist as operators, so they are reachable from a script; but the manual describes the *result* as appearing in the Info editor's header, and the sources do not say where that result goes when there is no Info editor. `ID.is_missing`, `ID.is_linked_packed` and `Image.packed_files` are plain properties and nothing marks them interface-only.

**Operating system.** The sources do not say packing differs by operating system.


**Observed — hand test entries 1 and 5, run 2026-08-05 on the Creator's own computer** (MacBook Pro, M1 Pro chip, macOS 26.5.2, Apple's own graphics), with Blender 5.0.1 started from a terminal with no window, using a throwaway settings folder set through `BLENDER_USER_RESOURCES`. The full list of tests is at `Audits/Blender Research/Dependencies/Hand tests.md`.

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


**Observed — hand test entry 21, run by the Creator on 2026-08-05 on their own computer** (MacBook Pro, M1 Pro chip, macOS 26.5.2, Apple's own graphics), in Blender 5.0.1 with its window open. This is the half of the list that needs eyes on the screen; the whole list is at `Audits/Blender Research/Dependencies/Hand tests.md`. Because it was run on a Mac, it settles macOS and leaves Windows and Linux open.

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

**With a window open on the Linux machine, and this closes a gap the Mac left.** The Mac's screenshots all had the unpacked image selected, so what sits beside the *packed* image's file-path field was never seen. Photographed here, the Image editor header reads, with the packed image selected: the image icon, the name field `packed.png`, a shield, a two-sheets copy button, **a stack of boxes**, and an X. With the unpacked image selected the row is identical except that where the stack of boxes was there is **a plain folder** — the ordinary open-a-file button. So the packed one carries a parcel and the unpacked one carries a folder. *(Narrowed by the citation check of 2026-08-06: the whole window was photographed in all four versions, but the enlargement of that header row was made only from the 4.5 and 5.2 pictures — the Linux hand-test list records the reading as "the same in 4.5 and 5.2", and no enlarged picture of this version's header exists. What is written here for this version is read across from those two.)*

The rest of the window picture matched the Mac: the plane using the packed image rendered in its own colour and the plane using the unpacked one bright magenta, and Report Missing Files put the same warning on screen that it prints to a terminal.

**Both standing lines, brought up to date by the Linux tests.** *Driven from outside:* every reading above was taken with Blender running with no window unless it says otherwise, so it holds outside the interface. *Operating system:* **now settled on two operating systems, macOS and Linux, and each is its own result** — what they agree on and where they differ is said above. **Windows is still untried: no Windows machine has run it.**

---

## 4. Can one thing a file points at mean several files on disk?

**Answer.** Yes, in several distinct ways, and Blender has a name for it — a "virtual filepath" that resolves to concrete ones, and a "single dir … with an array of files".

The clearest statements are in the flags of the path-walking API:

> "RESOLVE_TOKEN: Resolve Token. Resolve tokens within a virtual filepath to a single, concrete, filepath. Currently only used for UDIM tiles."
> "SKIP_MULTIFILE: Skip Multi-file. Skip paths where a single dir is used with an array of files, eg. sequence strip images or point-caches. In this case only the first file path is processed. This is needed for directory manipulation callbacks which might otherwise modify the same directory multiple times."
> — Python API, https://docs.blender.org/api/5.0/bpy_types_enum_items/file_path_foreach_flag_items.html (read 2026-07-30)

The individual cases:

*Image sequences and UDIM tiles.*
> "SEQUENCE — Image Sequence – Multiple image files, as a sequence."
> "TILED — UDIM Tiles – Tiled UDIM image texture."
> — Python API, `Image.source`, https://docs.blender.org/api/5.0/bpy.types.Image.html (read 2026-07-30)

and the class description says so directly:
> "The Image data-block is a shallow wrapper around image or video file(s) (on disk, as packed data, or generated)."
> — same page and address, read 2026-07-30

*UDIM tokens — one written path, many files.*
> "The following tokens are supported:
> <UDIM>: A 4-digit notation calculated as 1001 + u-tile + v-tile * 10.
> <UVTILE>: A notation defined as u(u-tile + 1)_v(v-tile + 1).
> Examples:
> monster-basecolor.<UDIM>.png will load/save files like monster-basecolor.1021.png etc."
> — Manual, https://docs.blender.org/manual/en/5.0/modeling/meshes/uv/workflows/udims.html (read 2026-07-30)

*Volumes and caches split across frames.*
> "is_sequence — Whether the cache is separated in a series of files"
> — Python API, `bpy.types.Volume`, https://docs.blender.org/api/5.0/bpy.types.Volume.html and `bpy.types.CacheFile`, https://docs.blender.org/api/5.0/bpy.types.CacheFile.html (read 2026-07-30)

> "Sequence — Loads further VDB files, one for each frame in an animation. Much like with image sequences, …"
> — Manual, https://docs.blender.org/manual/en/5.0/modeling/volumes/properties.html (read 2026-07-30)

*Video Sequencer image strips: a directory plus a list.* `ImageStrip` has both `directory` and `elements` (Python API, https://docs.blender.org/api/5.0/bpy.types.ImageStrip.html, read 2026-07-30).

*Physics point caches: a directory plus a filename pattern.*
> "The cache files name format is name_frame_index.bphys."
> — Manual, https://docs.blender.org/manual/en/5.0/physics/baking.html (read 2026-07-30)

*Cache files with layers.* `CacheFile` holds a collection of `CacheFileLayer`, each with its own `filepath` — "Layer of the cache, used to load or override data from the first the first layer" (Python API, https://docs.blender.org/api/5.0/bpy.types.CacheFileLayer.html, read 2026-07-30). **Inferred:** because the collection can hold layers and each layer has a path, one Cache File data-block may name more than one archive.

*Render output.* One written path becomes one file per frame — see Q10.

**Label:** **documented** except for the Cache File layer conclusion, which is **inferred** with its step stated above.

**Not covered.** How Blender decides which files on disk belong to a sequence (numbering rules, gaps, padding) — the VSE page mentions rendering "an image sequence with a few frames missing" (Manual, https://docs.blender.org/manual/en/5.0/video_editing/edit/montage/strips/image.html, read 2026-07-30) but the matching rule itself is not stated. Whether `<UDIM>` and `<UVTILE>` work in every path field or only in image paths.

**Outside the interface.** `RESOLVE_TOKEN` and `SKIP_MULTIFILE` are flags on a Python call, so this multiplicity is visible to a script with no window. The sources do not say whether resolution differs in background mode.

**Operating system.** The sources do not say.


**Observed — hand test entries 4, 15 and 16, run 2026-08-05 on the Creator's own computer** (MacBook Pro, M1 Pro chip, macOS 26.5.2, Apple's own graphics), with Blender 5.0.1 started from a terminal with no window, using a throwaway settings folder set through `BLENDER_USER_RESOURCES`. The full list of tests is at `Audits/Blender Research/Dependencies/Hand tests.md`.

**What was done.** Two UDIM tiles `tiles/tile.1001.png` and `tiles/tile.1002.png`, and a three-frame sequence `seq/shot_0001.png` to `shot_0003.png`, were written to disk. One image data-block was loaded from `tile.1001.png` with `source` set to `TILED`, another from `shot_0001.png` with `source` set to `SEQUENCE`. Their stored paths and tile numbers were printed, then `bpy.utils.blend_paths()`, then `bpy.data.file_path_map()`, then a walk of `bpy.data.file_path_foreach`.

**What came back.**

```
   image=shot_0001.png  source=SEQUENCE  stored='//seq/shot_0001.png' tiles=[1001]
   image=tile.1001.png  source=TILED     stored='//tiles/tile.<UDIM>.png' tiles=[1001, 1002]
```

Blender rewrote the tiled image's path into the `<UDIM>` token form itself, and one recorded path stood for two files. `blend_paths()` and `file_path_map()` both returned the un-expanded text.

**Expanding one path into the files it stands for cannot be asked for in this version either.** `bpy.data.file_path_foreach` exists and visited the three un-expanded paths. Passing a deliberately invalid flag to learn the valid ones returned `ValueError: flags: 'NOT_A_REAL_FLAG' not found in ('SKIP_LINKED', 'SKIP_PACKED', 'RESOLVE_TOKEN', 'SKIP_WEAK_REFERENCES', 'SKIP_MULTIFILE', 'RELOAD_EDITED')` — there is no expand flag here. The metadata object handed to the callback has neither `is_expanded` nor `is_cache`. In 5.2.0 all three expand flags exist and both fields do. In 4.5.12 the function does not exist at all.

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

`blend_paths()` and `file_path_map()` both returned the un-expanded text. The invalid-flag probe returned `ValueError: flags: 'NOT_A_REAL_FLAG' not found in ('SKIP_LINKED', 'SKIP_PACKED', 'RESOLVE_TOKEN', 'SKIP_WEAK_REFERENCES', 'SKIP_MULTIFILE', 'RELOAD_EDITED')` — no expand option, the same as on the Mac — and asking for one gave `ValueError: flags: 'EXPAND_TOKENS' not found in (…)`.

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

**Answer.** Four ways, which can combine.

**(i) Absolute.**
> "Make Paths Absolute — Make all paths to external files absolute (= full path from the system's root)."
> — Manual, https://docs.blender.org/manual/en/5.0/interface/window_system/topbar.html (read 2026-07-30)

**(ii) Relative, with a `//` prefix meaning the folder of the blend-file.**
> "When you specify one of these external files, the default option is to make the path relative. Blender stores a partial path evaluated relative to the directory location of the referencing blend-file."
> "When relative paths are supported, the File Browser provides a Relative Path checkbox, when entering the path into a text field, use a double slash prefix (//) to make it so."
> "You cannot use relative paths into a new untitled blend-file. Save it before linking to external files."
> — Manual, https://docs.blender.org/manual/en/5.0/files/blend/open_save.html (read 2026-07-30)

> "Blender's relative file paths are not compatible with standard Python modules such as sys and os. Built-in Python functions don't understand Blender's // prefix which denotes the blend-file path."
> — Python API, https://docs.blender.org/api/5.0/info_gotchas_file_paths_and_encoding.html (read 2026-07-30)

Which default applies is a preference: "Relative paths are the default but this can be changed in the File tab of the Preferences." (Manual, open_save address above, read 2026-07-30), and "Default To – Relative Paths: Default value for Relative Paths when loading external files" (Manual, https://docs.blender.org/manual/en/5.0/editors/preferences/save_load.html, read 2026-07-30).

**(iii) With template expressions in curly braces** — new in 5.0 for some paths.
> "Path templates substitute template expressions (written {abcd}, where "abcd" is a variable name) in a filepath when the path is used."
> "This substitution happens internally at time of use (e.g. while rendering), so the template syntax will remain as-is in the path field."
> "Currently path templates are only supported for the render output path in Scene properties and the output paths in the compositor's File Output node. More filepaths will support templates in future releases."
> — Manual, https://docs.blender.org/manual/en/5.0/files/file_paths.html (read 2026-07-30)

The variables are `blend_name`, `blend_dir`, `blend_name_lib`, `blend_dir_lib` everywhere templates work, plus `fps`, `resolution_x`/`resolution_y`, `scene_name`, `camera_name` in render output paths, plus `node_name` on node path properties. Format specifiers (`{fps:###}`) and escapes (`{{`, `}}`) are documented on the same page. The 5.0 release notes confirm six of these are new:

> "The following new path template variables have been introduced … `scene_name` … `camera_name` … `node_name` … `blend_name_lib` … `blend_dir` … `blend_dir_lib`"
> — Release notes, https://developer.blender.org/docs/release_notes/5.0/core/ (read 2026-07-30)

**(iv) With substitution tokens that stand for many files** — `#` for frame numbers and `<UDIM>` / `<UVTILE>` for tiles.
> "The # characters are replaced by the frame number, and used to define zero padding."
> — Manual, https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html (read 2026-07-30)

> "Substitution tokens are special sequences of characters in a filename that can be replaced with more meaningful and context aware information. In this case, tokens are identified by being text wrapped in angle bracket characters."
> — Manual, https://docs.blender.org/manual/en/5.0/modeling/meshes/uv/workflows/udims.html (read 2026-07-30)

**How long a path may be.**
> "directory: 767 … file-name: 255 … file-path: 1023"
> — Manual, "Working Limits", https://docs.blender.org/manual/en/5.0/advanced/limits.html (read 2026-07-30)

with the caution on the same page: "Multi-byte encoding means some Unicode characters use more than a single ASCII character."

**Converting between the forms** is `bpy.path.abspath(path, *, start=None, library=None)` — "Returns the absolute path relative to the current blend file using the "//" prefix" — and `bpy.path.relpath` (Python API, https://docs.blender.org/api/5.0/bpy.path.html, read 2026-07-30). Cycles uses exactly this, which I can see in its own file:

> `script_path = bpy.path.abspath(filepath, library=library)`
> — `Blender Sources/cycles_5.0/osl.py`, line 197, inside `update_external_script` (read 2026-07-30). **observed**

**Label:** **documented** for (i)–(iv) and the limits; **observed** for the Cycles line.

**Not covered.** Whether `~` for a home folder, environment variables inside a path, or symbolic links are honoured — nothing in the sources mentions any of the three. Whether the 1023-character limit is bytes or characters (the note about multi-byte encoding implies bytes but does not say). Which of the four forms is stored for paths that support templates when the file is saved — the manual says the template "will remain as-is in the path field", which settles what is stored, not what is written to disk elsewhere.

**Outside the interface.** The `-o` command-line argument explicitly accepts a `//` relative path and path templates — "Use // at the start of the path to render relative to the blend-file. You can use path templating features such as {blend_name} in the path." (Manual, command-line arguments address above, read 2026-07-30). The earlier command-line quote documents `#` frame-number substitution there as well. This settles those forms for the regular render output path. The sources do not say that every path form listed above works in every stored path or every no-window use.

**Operating system.** The sources do not say the writing forms differ by operating system. One helper acknowledges that separators differ: "native_pathsep(path) — Replace the path separator with the systems native os.sep." (Python API, bpy.path address above, read 2026-07-30).


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

## 6. What does a location recorded on one kind of computer mean on another, including a kind that names its disks?

**Answer: not in the sources**, for the question as asked. Blender 5.0's documentation nowhere states what an absolute path written on one operating system resolves to on another, and it nowhere describes what happens to a Windows drive letter when the file is opened on macOS or Linux.

What *is* documented, and is as close as the sources get:

> "Volumes
> Drives and network mounts."
> — Manual, File Browser bookmark categories, https://docs.blender.org/manual/en/5.0/editors/file_browser.html (read 2026-07-30)

That is the only place the sources acknowledge that a computer may name its disks. It describes a browsing convenience, not path resolution.

> "native_pathsep(path) — Replace the path separator with the systems native os.sep."
> — Python API, https://docs.blender.org/api/5.0/bpy.path.html (read 2026-07-30)

That documents a helper for converting separators — it does not say Blender applies it to stored paths.

The one place the sources admit that drive letters defeat relative paths is in the exporters, not in blend-file paths:

> "Uses relative paths in every case (except when on a different drive on Windows)."
> — Manual, Wavefront OBJ exporter "Path Mode: Relative", https://docs.blender.org/manual/en/5.0/files/import_export/obj.html (read 2026-07-30); the same sentence appears for the FBX exporter at https://docs.blender.org/manual/en/5.0/addons/import_export/scene_fbx.html

And the whole point of relative paths is stated as portability, without naming operating systems:

> "With a relative path, you can move the blend-file to a new location provided the externally linked files are moved along with it. For example, you could send someone a folder that contains a blend-file and a subfolder of external images that it references."
> — Manual, https://docs.blender.org/manual/en/5.0/files/blend/open_save.html (read 2026-07-30)

**Label:** **not in the sources** for the question. **Documented** for the four quoted pieces around it.

**Where I looked.** Searched the whole manual, API documentation and release notes for: `drive letter`, `different drive`, `C:\`, `backslash`, `forward slash`, `path separator`, `os.sep`, `UNC`, `volume`, `mount`. Read in full: `files/blend/open_save.txt` (Relative Paths), `files/file_paths.txt`, `interface/window_system/topbar.txt` (External Data), `editors/file_browser.txt`, `editors/preferences/save_load.txt`, `bpy.path`, `info_gotchas_file_paths_and_encoding`, `advanced/limits.txt`. Also checked the 5.0 release notes for `core`, `pipeline_io` and `python_api` for any path-portability change; the only path change in 5.0 is the new template variables. Nothing in any of these answers the cross-system question.

**Not covered.** Everything the question asks beyond the four quoted pieces: whether Blender rewrites separators on load, whether a drive-lettered path is treated as broken or as a literal name on a system without drive letters, whether a relative path is stored with a forward slash regardless of the writing system.

**Outside the interface.** The sources do not say. Nothing suggests the interface changes path meaning; the File Browser "Volumes" list is interface-only by nature, but it is a browsing aid, not resolution.

**Operating system.** This question *is* the operating-system question, and the answer is that the sources do not say.


**Observed — hand test entry 3, run 2026-08-05 on the Creator's own computer** (MacBook Pro, M1 Pro chip, macOS 26.5.2, Apple's own graphics), with Blender 5.0.1 started from a terminal with no window, using a throwaway settings folder set through `BLENDER_USER_RESOURCES`. The full list of tests is at `Audits/Blender Research/Dependencies/Hand tests.md`.

**What was done.** An image data-block was given the location `C:\textures\tex.png` on the Creator's Mac, the file was saved, reopened, and the stored text, `bpy.path.abspath` of it, and the image size after `reload()` were printed. The same was done for `/Volumes/NoSuchShare/tex.png`.

**What came back**, the same in all four versions:

```
NAME=drive-letter   STORED='C:/textures/tex.png'                 ABSPATH='C:/textures/tex.png' SIZE=(0, 0)
NAME=no-such-mount  STORED='/Volumes/NoSuchShare/tex.png'        ABSPATH='/Volumes/NoSuchShare/tex.png' SIZE=(0, 0)
```

So a Mac keeps a drive-lettered path, turns its backslashes into forward slashes when the file is saved, treats what is left as an absolute path, and simply does not find the file. Nothing was printed about it.

**Both standing lines, brought up to date by this test.** *Driven from outside:* settled — all of the above was done with no window. *Operating system:* a second operating system is now reachable and the crossing is recorded below. What is still untried is Windows. The macOS-to-Linux half was made on 2026-08-06 and is recorded below. The Windows half needs a Windows machine and none was rented for it. It is in the *cannot be run* section of `Audits/Blender Research/Dependencies/Hand tests.md` and stays unanswered.


**Observed — the crossing between two computers, run for the first time on 2026-08-06.** This is what the question asks and what no earlier test could reach. A `crossing.blend` was built in this version **on the Creator's Mac** at `/Users/artist/…/crossing/5.0/proj/` and saved with `relative_remap=False` so every path stayed exactly as typed. It was then carried whole to the rented Linux machine and opened there in the same version. Both machines were read with the same script.

**What survived the crossing, and what did not.** The result was the same in all four versions.

```
IMAGE rel          stored='//assets/tex.png'
             abspath='/root/…/crossing/5.0/proj/assets/tex.png'
             exists=True size=(32, 32) has_data=True
IMAGE rel-up       stored='//../outer/tex.png'
             abspath='/root/…/crossing/5.0/proj/../outer/tex.png'
             exists=True size=(32, 32) has_data=True
IMAGE abs-mac      stored='/Users/artist/…/crossing/5.0/proj/assets/tex.png'
             abspath='/Users/artist/…/crossing/5.0/proj/assets/tex.png'
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

**And a render does not refuse a foreign output path.** The same file's render output path was a full Mac path. Rendering it on the Linux machine printed `Saved: '/Users/artist/…/crossing/5.0/renders/frame_0001.png'`, finished with exit status 0, and **created the whole Mac-shaped folder tree at the root of the Linux disk** to do it. Nothing warned. That run was made as the root user, who owns the root of that disk, so what happens for a render user who cannot write there is not established.

**Still not established.** What a Windows Blender writes into a path, and what a path written on Windows means on macOS or Linux, or the reverse. No Windows machine has run it.

**Both standing lines, brought up to date by the Linux tests.** *Driven from outside:* every reading above was taken with Blender running with no window unless it says otherwise, so it holds outside the interface. *Operating system:* **now settled on two operating systems, macOS and Linux, and each is its own result** — what they agree on and where they differ is said above. **Windows is still untried: no Windows machine has run it.**

---

## 7. What does a location pointing somewhere other than the computer the file sits on mean, at either end?

**Answer: not in the sources.** Blender 5.0's documentation does not describe what a path to another machine means, at the writing end or the reading end. It does not describe a remote reference as distinct from an ordinary path.

Three fragments acknowledge that other machines get involved, all treating the remote place as an ordinary folder:

> "Hint
> Primitive Render Farm
> An easy way to get multiple machines to share the rendering workload is to:
> Set up a shared directory over a network file system.
> Disable Overwrite, enable Placeholders in the Render Output panel.
> Start as many machines as you wish rendering to that directory."
> — Manual, https://docs.blender.org/manual/en/5.0/render/output/properties/output.html (read 2026-07-30)

That is the only passage in the whole manual describing several computers sharing files, and note what it settles: the remote place is a directory, reached as a directory, and the coordination between machines is done with two render-output options — not with anything about the path.

> "Volumes — Drives and network mounts."
> — Manual, https://docs.blender.org/manual/en/5.0/editors/file_browser.html (read 2026-07-30)

> "Reading mmap'ed images from network drives may crash Blender when the file is not valid"
> — Release notes, 5.0 bug fixes, https://developer.blender.org/docs/release_notes/5.0/bugfixes/ (read 2026-07-30)

That last one is a documented statement that Blender reads images from network drives, and that a fix was needed there in 5.0.

**Label:** **not in the sources** for the question; **documented** for the three fragments.

**Where I looked.** Same searches as Q2 and Q6: `network drive`, `network share`, `network path`, `network location`, `networked`, `remote server`, `shared drive`, `over the network`, `mount`, `NAS`, `UNC`, `URI`, `file://`, `smb://`, `http`, `ftp`, `offline`, `online`. Read in full: `render/output/properties/output.txt`, `editors/file_browser.txt`, `editors/preferences/system.txt` (Network section), `advanced/deploying_blender.txt`, `video_editing/setup/directory_structure.txt`, `files/blend/open_save.txt`, `bpy.path`. The only internet-facing feature documented anywhere is add-on access ("Allow Online Access — Allow Blender to access the internet."), and `--offline-mode` on the command line — neither concerns file references.

**Not covered.** Everything asked: whether Blender distinguishes a remote path at all; what a machine at the far end sees; permissions, locking, latency, partial reads; whether a library or texture on a mount is retried or given up on.

**Outside the interface.** The render-farm hint describes several machines rendering to one directory, but does not say whether they are driven with or without a window. The sources do not settle the standing line.

**Operating system.** The sources do not say. "Drives and network mounts" is written without distinguishing systems.


**Observed — hand test entry 3, run 2026-08-05 on the Creator's own computer** (MacBook Pro, M1 Pro chip, macOS 26.5.2, Apple's own graphics), with Blender 5.0.1 started from a terminal with no window, using a throwaway settings folder set through `BLENDER_USER_RESOURCES`. The full list of tests is at `Audits/Blender Research/Dependencies/Hand tests.md`.

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

**Ambiguity.** "A name" could mean a data-block's name inside the file, or a file's name on disk. The question sits in a sheet about what a file points at, so I read it as **file names on disk**. The other reading is answered briefly at the end.

**Answer (file names).** The sources do not state a rule. They do contain one thing that only makes sense if case matters on some systems and not others — a helper built for exactly that mismatch:

> "bpy.path.resolve_ncase(path)
> Resolve a case insensitive path on a case sensitive system, returning a string with the path if found else return the original path."
> — Python API, https://docs.blender.org/api/5.0/bpy.path.html (read 2026-07-30)

A related helper takes case as an option when comparing extensions:

> "ensure_ext(filepath, ext, *, case_sensitive=False) … case_sensitive (bool) – Check for matching case when comparing extensions."
> — same page and address, read 2026-07-30

**Label:** **documented** that Blender 5.0 ships a helper described as resolving a case-insensitively-written path on a case-sensitive system, and that its extension helper defaults to ignoring case. **Not in the sources** for the question itself. The helper's description does not establish which operating systems behave which way, or that Blender calls it when opening a referenced file.

**Answer (data-block names inside the file).** The sources say names are unique per type per file and are compared for collision, but say nothing about case in that comparison:

> "Data-blocks names are unique within their namespace. A data-block namespace is defined by its type, and the blendfile it is stored in."
> — Manual, https://docs.blender.org/manual/en/5.0/files/data_blocks.html (read 2026-07-30)

Whether `Cube` and `cube` count as a collision is **not in the sources**.

**Where I looked.** Searched manual, API documentation and release notes for `case sensitiv`, `case-sensitiv`, `case insensitiv`, `case-insensitiv`, `uppercase`, `lowercase`. Every hit outside `bpy.path` is about something else: search fields in the Outliner, Text Editor and object/bone selection; the "To Uppercase"/"To Lowercase" text-object commands; a small-caps typography setting; and the warning that *command-line arguments* are case sensitive ("Arguments are case sensitive! -F and -f are not the same." — Manual, https://docs.blender.org/manual/en/5.0/advanced/command_line/render.html, read 2026-07-30). Read in full: `bpy.path`, `files/data_blocks.txt`, `files/blend/open_save.txt`, `info_gotchas_file_paths_and_encoding`.

**Not covered.** Whether Blender applies `resolve_ncase` on its own when loading a referenced file. What happens when two files differing only in case exist in the same folder. Whether case matters when Blender matches an image sequence or UDIM tiles by name.

**Outside the interface.** `resolve_ncase` is exposed through Python, but the sources do not say whether Blender uses it when opening a referenced file with no window. Beyond that the sources do not say.

**Operating system.** The sources do not name which operating systems are case-sensitive. The phrase "a case sensitive system" is the only acknowledgement that they differ.


**Observed — hand test entry 10, run 2026-08-05 on the Creator's own computer** (MacBook Pro, M1 Pro chip, macOS 26.5.2, Apple's own graphics), with Blender 5.0.1 started from a terminal with no window, using a throwaway settings folder set through `BLENDER_USER_RESOURCES`. The full list of tests is at `Audits/Blender Research/Dependencies/Hand tests.md`.

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
Warning: Could not find 'Texture.PNG' in '/root/…/rescue2/5.0/'
```

and changed nothing. With a search folder holding the right name, the same call repaired every other missing path and **rewrote each one as a full absolute path into the search folder**, leaving paths that already resolved alone. The same in all four versions. The Mac cannot answer this question at all, because on its disk the capitals reference is not missing in the first place.

**Still not established.** Whether Windows behaves like either machine. No Windows machine has run it.

**Both standing lines, brought up to date by the Linux tests.** *Driven from outside:* every reading above was taken with Blender running with no window unless it says otherwise, so it holds outside the interface. *Operating system:* **now settled on two operating systems, macOS and Linux, and each is its own result** — what they agree on and where they differ is said above. **Windows is still untried: no Windows machine has run it.**


**Observed — added by this round's hand tests, run 2026-08-07 on a rented Linux machine** (Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB on driver 580.95.05, an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory, and no display server at all), with Blender 5.0.1 started from a terminal with no window, `--factory-startup`, and a settings folder of its own made first and set through `BLENDER_USER_RESOURCES`. **This is an addition to a document written in an earlier round; nothing above it has been changed.** The full list of tests is at `Audits/Blender Research/Dependencies/Hand tests — added questions.md`.

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
UserWarning: load: '/root/dependencies/work/q8/5.0/lib.blend' does not contain objects["cube"]
```

So capitals are told apart exactly inside a file: three names differing only in case all survive being saved and reopened, and a lookup with the wrong capitals finds nothing. Asking a library for a name with the wrong capitals through `libraries.load` gives `None` and a warning naming the file. Asking through `bpy.ops.wm.link` gives `{'FINISHED'}`, prints nothing at all, and links nothing.

**Both standing lines, brought up to date by this test.** *Driven from outside:* settled — all of it was done with no window. *Operating system:* run on Linux here; **the same test was run on the Creator's Mac on 2026-08-07 and that result, together with what the Mac's own disk does with capitals, is recorded below.** Windows is untested because no Windows machine has run it.

**Observed — the Mac half of this round's hand tests, run 2026-08-07 on the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card.** Blender 5.0.1 was started from a terminal with no window, with `--factory-startup` and a settings folder of its own, made before it was used and set through `BLENDER_USER_RESOURCES`; that this took effect was checked rather than assumed, and this Blender answered `RESOURCES …/depwork/res-5.0` and `VERSION 5.0.1`. **Other topics were running Blender on this same Mac throughout, so nothing was timed and no memory was measured.** **This is an addition; nothing above it has been changed.** The full list of tests is at `Audits/Blender Research/Dependencies/Hand tests — added questions.md`. In the quotations below the working folder's long path is written `…/depwork`; nothing else is altered.

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
   warning: UserWarning: load: '…/depwork/q8/5.0/lib.blend' does not contain objects["cube"]
LINK asked 'cube' -> got [None]
   warning: UserWarning: load: '…/depwork/q8/5.0/lib.blend' does not contain objects["CUBE"]
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
  asked nosuch.blend   -> RAISED OSError: load: …/depwork/q8disk/5.0/assets/nosuch.blend failed to open blend file
```

the same in all four versions. **On the Creator's Mac's own disk a recorded file name is found whatever its capitals**, for an image and for a linked library alike, and the recorded spelling is kept as written. The fourth line of each block is the control that makes the other three mean something: a name that genuinely is not there fails, and says so.

**So the answer to this question really does depend on the disk and not on Blender, and this project's two machines sit on opposite sides of it.** A file written on the Creator's Mac can record `Texture.PNG` for a file named `texture.png` and work perfectly, and the same file on the rented Linux machine will not find it. Nothing inside the file behaves that way: names inside a `.blend` are exact on both machines.

**Both standing lines, brought up to date by these tests.** *Driven from outside:* settled — every reading here was taken with Blender running with no window. *Operating system:* **settled on both of this project's machines, and each is its own result.** macOS, on its ordinary disk, ignores capitals in a file name on disk; Linux does not; and both tell names inside the file apart exactly. **Windows is untried: no Windows machine has run it.**




---

## 9. Do names written in characters outside the Latin alphabet survive being read out of a file, moved to another computer, and found again by a render?

**Answer.** The sources answer the first part squarely and are silent on the rest.

**Paths are deliberately exempted from Blender's UTF-8 rule**, which is the key fact:

> "However, this complicates matters for Blender's Python API because .blend files don't have an explicit encoding. To avoid the problem for Python integration and script authors we have decided that all strings in blend-files must be UTF-8, ASCII compatible. This means assigning strings with different encodings to an object name, for instance, will raise an error.
> Paths are an exception to this rule since the existence of non-UTF-8 paths on the user's file system cannot be ignored."
> — Python API, https://docs.blender.org/api/5.0/info_gotchas_file_paths_and_encoding.html (read 2026-07-30)

And the practical consequence, stated with the error it produces:

> "This means seemingly harmless expressions can raise errors, e.g:
> >>> print(bpy.data.filepath)
> UnicodeEncodeError: 'ascii' codec can't encode characters in position 10-21: ordinal not in range(128)"
> — same page and address, read 2026-07-30

with advice on the same page: "Always use UTF-8 encoding or convert to UTF-8 where the input is unknown", "Use os.fsencode() or os.fsdecode() instead of built-in string decoding functions when operating on paths", and "To print paths or to include them in the user interface use repr(path) first".

**Data-block names** are a different matter and are documented:

> "Data-block names have a fixed length of 255 bytes, i.e. 255 basic ASCII characters, or less when using diacritics or non-latin glyphs (the UTF8 encoding will then typically use more than one byte per character)."
> — Manual, https://docs.blender.org/manual/en/5.0/files/data_blocks.html (read 2026-07-30)

Blender 5.0 raised that limit, and warned about the file-name consequence:

> "The data-blocks can now have names up to 255 bytes, increased from previous limit of 63 bytes."
> "Potential issues on Windows regarding file names. Windows (NTFS) only supports up to 255 chars in its file names. This limit can now more easily be reached with longer data-block names, when they are used as part of a generated file name."
> — Release notes, https://developer.blender.org/docs/release_notes/5.0/core/ (read 2026-07-30)

The same byte-versus-character caution attaches to the path limits: "file-path: 1023 … Multi-byte encoding means some Unicode characters use more than a single ASCII character." (Manual, https://docs.blender.org/manual/en/5.0/advanced/limits.html, read 2026-07-30).

Blender also offers a helper for stripping such characters out when a name is to become a file name:

> "clean_name(name, *, replace='_') — Returns a name with characters replaced that may cause problems under various circumstances, such as writing to a file. All characters besides A-Z/a-z, 0-9 are replaced with "_" or the replace argument if defined."
> — Python API, https://docs.blender.org/api/5.0/bpy.path.html (read 2026-07-30)

**Label:** **documented** that a stored path may contain characters that are not UTF-8 at all, and that reading such a path out through Python can raise an encoding error unless handled. **Documented** that data-block names are UTF-8 and measured in bytes. **Not in the sources** for the rest of the question — whether such a path survives being moved to another computer, and whether a render then finds the file.

**Where I looked, for the silent part.** Searched manual and release notes for `utf-8`, `utf8`, `unicode`, `ascii`, `non-latin`, `diacritic`; searched the API documentation for the same. Read in full: `info_gotchas_file_paths_and_encoding`, `files/data_blocks.txt` (Name & Rename), `advanced/limits.txt`, `bpy.path`, `files/asset_libraries/catalogs.txt` ("Paths are always interpreted as UTF-8." — but that is about *asset catalog* paths inside `blender_assets.cats.txt`, which are not filesystem paths: "think less filesystem path and more URL"). Nothing anywhere discusses a path's encoding travelling between machines.

**Not covered.** What a path containing non-UTF-8 bytes becomes when the blend-file is opened on a system with a different filesystem encoding. Whether Blender normalises Unicode (the decomposed-versus-composed difference that bites when files move between systems) — nothing in the sources mentions normalisation at all. Whether a render reports an encoding failure differently from a missing file.

**Outside the interface.** The encoding gotcha is written for Python, but the quoted words do not distinguish a windowed run from a no-window run. The standing line is not settled.

**Operating system.** Partly answered, and only for names: the 5.0 release note names Windows and NTFS's 255-character file-name limit as a specific hazard. Beyond that, the sources do not say.


**Observed — hand test entry 11, run 2026-08-05 on the Creator's own computer** (MacBook Pro, M1 Pro chip, macOS 26.5.2, Apple's own graphics), with Blender 5.0.1 started from a terminal with no window, using a throwaway settings folder set through `BLENDER_USER_RESOURCES`. The full list of tests is at `Audits/Blender Research/Dependencies/Hand tests.md`.

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

**Answer.** Yes. Blender 5.0 records a substantial number of write destinations, and they use the same path forms as read paths, so `//` means the blend-file's folder for a write just as for a read.

**Render output.**
> "File Path — Choose the location to save rendered frames.
> When rendering an animation, the frame number is appended at the end of the file name with four padded zeros (e.g. image0001.png). You can set a custom padding size by adding the appropriate number of # anywhere in the file name (e.g. image_##_test.png translates to image_01_test.png).
> This setting expands Relative Paths where a // prefix represents the directory of the current blend-file."
> — Manual, https://docs.blender.org/manual/en/5.0/render/output/properties/output.html (read 2026-07-30)

> "filepath — Directory/name to save animations, # characters define the position and padding of frame numbers … string, default "", (never None, blend relative // prefix supported, Supports template expressions)"
> — Python API, `bpy.types.RenderSettings`, https://docs.blender.org/api/5.0/bpy.types.RenderSettings.html (read 2026-07-30)

Two options on that panel govern collisions between writers: "Overwrite existing files when rendering." and "Create empty placeholder frames while rendering." (Manual, output address above, read 2026-07-30).

**Compositor File Output node** — a second render-time write path, and one of only two places templates work: "Currently path templates are only supported for the render output path in Scene properties and the output paths in the compositor's File Output node." (Manual, https://docs.blender.org/manual/en/5.0/files/file_paths.html, read 2026-07-30).

**Physics caches, and this one names where it lands.**
> "The cache of a baked simulation will be stored inside the blend-file when you save it. When Disk Cache is checked, Blender will save the cache separately to the drive in a folder named blendcache_[filename] alongside the blend-file. (The blend-file must be saved first.)"
> — Manual, https://docs.blender.org/manual/en/5.0/physics/baking.html (read 2026-07-30)

That is the clearest "where does the write land" statement in the whole documentation: a folder named after the blend-file, beside it. The same page documents an override — "External — Allows you to read the cache from a drive using a user-specified file path." and "Path — Select the directory path to the cache files."

**Ocean modifier**, which states that it creates the folder:
> "If the cache path folder does not exist, it will be created."
> — Manual, https://docs.blender.org/manual/en/5.0/modeling/modifiers/physics/ocean.html (read 2026-07-30)

**Other write destinations**, each marked "blend relative // prefix supported":
- `BakeSettings.filepath` — "Image filepath to use when saving externally" (Python API, https://docs.blender.org/api/5.0/bpy.types.BakeSettings.html, read 2026-07-30)
- `OceanModifier.filepath` — "Path to a folder to store external baked images" (Python API, https://docs.blender.org/api/5.0/bpy.types.OceanModifier.html, read 2026-07-30)
- `DynamicPaintSurface.image_output_path` — "Directory to save the textures" (Python API, https://docs.blender.org/api/5.0/bpy.types.DynamicPaintSurface.html, read 2026-07-30)
- `FluidDomainSettings.cache_directory` — "Directory that contains fluid cache files" (Python API, https://docs.blender.org/api/5.0/bpy.types.FluidDomainSettings.html, read 2026-07-30)
- `NodesModifier.bake_directory` and `NodesModifierBake.directory` — "Location on disk where the bake data is stored" (Python API, https://docs.blender.org/api/5.0/bpy.types.NodesModifier.html and https://docs.blender.org/api/5.0/bpy.types.NodesModifierBake.html, read 2026-07-30)
- `StripProxy.directory` — "Location to store the proxy files"; `SequenceEditor.proxy_dir` (Python API, https://docs.blender.org/api/5.0/bpy.types.StripProxy.html and https://docs.blender.org/api/5.0/bpy.types.SequenceEditor.html, read 2026-07-30)
- `PointCache.filepath` — "Cache file path" (Python API, https://docs.blender.org/api/5.0/bpy.types.PointCache.html, read 2026-07-30)

**Unpacking is a write too**, and its options are entirely about where the write lands:
> "Use files in current directory (create when necessary) — Unpacks all files in the same directory // as the blend-file, grouping them in proper folders (like ''textures'' for instance). However, if the final file exists already, it will use that file, instead of unpacking it."
> "Use files in original location (create when necessary) — Unpacks all files in their original location."
> — Manual, https://docs.blender.org/manual/en/5.0/files/blend/packed_data.html (read 2026-07-30)

**Label:** **documented** where the quoted words name the destination. An unquoted field name in the compressed API list is **inferred** to be another write destination from the path-property search recorded in Q1.

**Not covered.** Whether Blender creates missing folders for the render output path (it says so only for the ocean cache). What happens when the write destination is not writable. Whether a relative write path on a linked object resolves against the library or the open file — see Q13, where the point cache has an explicit "Use Library Path" option, but the general rule is not stated for write paths.

**Outside the interface.** The regular render output path is settable from the command line and the manual gives a worked example with a relative path: `blender -b animation.blend -o //render_ -F PNG -x 1 -a` (Manual, https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html, read 2026-07-30), with the warning that "Arguments are executed in the order they are given!" and "Always position -f or -a as the last arguments." (Manual, https://docs.blender.org/manual/en/5.0/advanced/command_line/render.html, read 2026-07-30). The sources do not settle no-window behaviour for every other write destination listed above.

**Operating system.** The sources do not say that the in-scope write destinations differ by operating system.


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

**Answer.** Yes — that is exactly how linking works. A link names a data-block inside a library file, by name.

> "Link creates a reference to data in a source file such that changes made there will be reflected in the current file the next time it is reloaded. In the File Browser, navigate to the external source blend-file and select the data-blocks you want to reuse."
> — Manual, https://docs.blender.org/manual/en/5.0/files/linked_libraries/link_append.html (read 2026-07-30)

The API exposes the name held by a library weak reference:

> "id_name — Full ID name in the library .blend file (including the two leading 'id type' chars)"
> — Python API, `bpy.types.LibraryWeakReference`, https://docs.blender.org/api/5.0/bpy.types.LibraryWeakReference.html (read 2026-07-30)

The Python loading interface is name-based end to end:

> "Returns a context manager which exposes 2 library objects on entering. Each object has attributes matching bpy.data which are lists of strings to be linked."
> and in its example: `data_dst.scenes = ["Scene"]`
> — Python API, `bpy.types.BlendDataLibraries.load`, https://docs.blender.org/api/5.0/bpy.types.BlendDataLibraries.html (read 2026-07-30)

**When the named thing is gone.** Two documented behaviours, at two levels.

*The whole library missing:*
> "Broken Libraries
> If Blender cannot find a library while loading a blend-file, it will create placeholder data-blocks to replace missing linked ones. That way, references to the missing data are not lost, and by relocating the missing library, the lost data can be automatically restored."
> — Manual, https://docs.blender.org/manual/en/5.0/files/linked_libraries/link_append.html (read 2026-07-30)

*The single data-block missing, or renamed:*
> "Relocate Linked ID … A right-click a linked ID will show the Relocate option in the ID Data sub-menu. It allows to relocate a directly linked ID by another one, from the same library or from a different one. This can be used to either fix a broken linked data-block (e.g. because the ID was renamed), or to switch to a variation of the same data."
> — same page and address, read 2026-07-30

That quote answers the "renamed" half directly: a rename breaks the link, and the repair is a manual re-pointing. There is a documented limit on the repair:

> "Note
> This operation is only available on data-blocks that are directly linked, and are not a dependency of any other linked data."
> — same page and address, read 2026-07-30

*What the placeholder reports:*
> "is_missing — This data-block is a place-holder for missing linked data (i.e. it is [an override of] a linked data that could not be found anymore)"
> — Python API, https://docs.blender.org/api/5.0/bpy.types.ID.html (read 2026-07-30)

*And through Python, a failed load is reported as nothing at all:*
> "The loaded objects can be accessed from `data_dst` outside of the context since loading the data replaces the strings for the data-blocks or None if the data-block could not be loaded."
> — Python API, BlendDataLibraries address above, read 2026-07-30

**Label:** **documented**.

**Not covered.** Whether the placeholder is created for a *renamed* data-block in an otherwise-present library, or only when the whole library is missing — the "Broken Libraries" paragraph is written about a missing library, and the Relocate Linked ID paragraph about a renamed ID, and the sources never join the two. What the file looks like after a save while a placeholder is standing in. Whether name matching is case-sensitive (see Q8).

**Outside the interface.** `Library.reload()`, `ID.is_missing` and `bpy.data.libraries.load` are exposed through Python. The sources do not explicitly state their no-window behaviour. Relocate Library and Relocate Linked ID are documented only as Outliner context-menu items, and the sources do not give a scripted equivalent.

**Operating system.** The sources do not say.


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

## 12. What does a render do when something the file points at is not there, and does it say so anywhere?

**Answer.** Mostly **not in the sources** for a general answer. Blender 5.0's documentation does not state what a render does when, say, an image texture a material uses is missing. It does document four specific cases and one general reporting mechanism.

**(a) A missing library, on load — placeholders, and the references survive.**
> "If Blender cannot find a library while loading a blend-file, it will create placeholder data-blocks to replace missing linked ones. That way, references to the missing data are not lost, and by relocating the missing library, the lost data can be automatically restored."
> — Manual, https://docs.blender.org/manual/en/5.0/files/linked_libraries/link_append.html (read 2026-07-30)

**(b) Missing media in the Video Sequencer — a chosen colour, and a chosen silence.**
> "Show Missing Media
> Render missing images/movies with a solid magenta color. When disabled, missing content will render fully transparent.
> Tip
> Strips with missing content will be displayed as red in the timeline."
> — Manual, https://docs.blender.org/manual/en/5.0/editors/video_sequencer/preview/sidebar.html (read 2026-07-30)

This is the only place in the whole manual that says what a *render* puts in the picture when a file is missing, and it is confined to the Sequencer.

**(c) A volume that failed to load — an error string on the data.**
> "error_message — If loading grids failed, error message with details"
> — Python API, `bpy.types.VolumeGrids`, https://docs.blender.org/api/5.0/bpy.types.VolumeGrids.html (read 2026-07-30)

and the neighbouring property makes the failure inspectable: "frame_filepath — Volume file used for loading the volume at the current frame. Empty if the volume has not be loaded or the frame only exists in memory." (same page and address).

**(d) A bad output path — the render fails outright.**
> "Depending on the path, template errors may prevent certain actions. For example, if there are errors in the render output path, then rendering an animation will fail with an error message indicating the path errors."
> — Manual, https://docs.blender.org/manual/en/5.0/files/file_paths.html (read 2026-07-30)

**What can be asked, rather than what is volunteered.** The general check is a command a person runs:

> "Report Missing Files — This option is useful to check if there are links to unpacked files that no longer exist. After selecting this option, a warning message will appear in the Info editor's header. If no warning is shown, there are no missing external files."
> — Manual, https://docs.blender.org/manual/en/5.0/interface/window_system/topbar.html (read 2026-07-30)

and per-image, whether the data actually loaded:
> "has_data — True if the image data is loaded into memory"
> — Python API, https://docs.blender.org/api/5.0/bpy.types.Image.html (read 2026-07-30)

**A possible place for messages.** Blender 5.0 unified its logging:

> "Most of Blender now uses the same logging system, including operators reports and background render progress. The output format has changed, and log levels are easier to control."
> — Release notes, https://developer.blender.org/docs/release_notes/5.0/core/ (read 2026-07-30)

with the controls documented as: `--log <match>`, `--log-level <level>` (whose levels are "fatal: Fatal errors only / error: Errors only / warning: Warnings / info: Information about devices, files, configuration, operations / debug … trace …"), and `--log-file <filepath>` — "Set a file to output the log to." (Manual, https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html, read 2026-07-30). The release-note example shows the shape of render log lines, including `00:28.631  render           | Saved: '/tmp/0001.png'`. Neither source says that missing-dependency messages enter this log.

**Label:** **documented** for (a)–(d), for Report Missing Files, for `Image.has_data`, and for the logging controls. **Not in the sources** for the general question — what a render does, and whether it says anything, when an ordinary texture, font, cache or sound file referenced by a scene is missing.

**Where I looked, for the silent part.** Searched the whole manual, API documentation and release notes for: `missing`, `file not found`, `could not be found`, `cannot be found`, `unable to open`, `failed to load`, `does not exist`, `magenta`, `pink`, `placeholder`, `error_message`, `has_data`. Read in full: `interface/window_system/topbar.txt` (External Data), `files/linked_libraries/link_append.txt`, `files/blend/packed_data.txt`, `editors/video_sequencer/preview/sidebar.txt`, `video_editing/setup/directory_structure.txt`, `advanced/command_line/render.txt`, `advanced/command_line/arguments.txt` (Render, Logging and Debug options in full), `bpy.ops.file.txt`, `bpy.types.Image`, `bpy.types.VolumeGrids`, `bpy.types.ID`, and the 5.0 release notes for core, rendering, cycles and bugfixes. The nearest thing to a general statement is a passing mention in a Video Sequencer tutorial page — "It will lower the probability that you accidentally delete a file (which will result in a 'file not found' error) or that you forget to include a necessary file when transferring the project ('file is missing' error)." (Manual, https://docs.blender.org/manual/en/5.0/video_editing/setup/directory_structure.html, read 2026-07-30) — which names two error strings but neither says where they appear nor what the render then contains.

**Not covered.** Whether a render continues or stops on a missing texture. Whether the missing file appears in the log, and at what level. What colour or value a missing texture contributes outside the Sequencer. Whether Blender's exit code reflects it — the only documented exit-code control is `--python-exit-code <code>`, which is about Python exceptions.

**Outside the interface.** This is where the gap bites hardest. The one general check, Report Missing Files, is documented as putting its warning "in the Info editor's header", which is an interface place; the sources do not say where that warning goes with no window. Against that, `bpy.ops.file.report_missing_files` exists as an operator, and 5.0's logging explicitly covers "operators reports and background render progress", with `--log-file` to capture it. Whether a missing-file warning is one of the messages that reaches the log is **not in the sources**.

**Operating system.** The sources do not say.


**Observed — hand test entries 4, 5, 6, 7, 16 and 17, run 2026-08-05 on the Creator's own computer** (MacBook Pro, M1 Pro chip, macOS 26.5.2, Apple's own graphics), with Blender 5.0.1 started from a terminal with no window, using a throwaway settings folder set through `BLENDER_USER_RESOURCES`. The full list of tests is at `Audits/Blender Research/Dependencies/Hand tests.md`.

**What was done.** A file was built with a plane whose Base Color comes from an Image Texture node pointing at a solid green 32 by 32 PNG beside the blend-file, a camera, a sun, 32 by 32 output, Standard view transform and one Cycles sample. The PNG was moved aside and the file rendered from a terminal with

```
"<application>" --background --factory-startup <file>.blend --engine CYCLES --render-output <folder>/miss_CYCLES_ --render-frame 1
```

and again with `--engine BLENDER_EEVEE`. The centre pixel of each written frame was then read by loading it into a fresh Blender.

**What came back.** The render finished, a frame was written, the exit status was 0, and the plane came out magenta — in every version and both engines. Centre pixel with the texture missing: Cycles `(1.0, 0.1373, 1.0, 1.0)`, Eevee `(1.0, 0.1294, 1.0, 1.0)`. With the texture present the same pixel was `(0.1373, 1.0, 0.1373, 1.0)` and `(0.1294, 1.0, 0.1294, 1.0)`.

**What was said about it is where the versions part company.**

```
00:00.294  cycles           | ERROR Image file '<path>/assets/green.png' does not exist.
```

and EEVEE printed `gpu.texture      | ERROR Failed to create GPU texture from Blender image`.

In 4.5.12 the same test printed nothing at all from Cycles. 5.1.2 prints the same line without the quotation marks; 5.2.0 calls it a WARNING and adds `ERROR Failed to load 1 image files`.

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


**Observed — hand test entry 21, run by the Creator on 2026-08-05 on their own computer** (MacBook Pro, M1 Pro chip, macOS 26.5.2, Apple's own graphics), in Blender 5.0.1 with its window open. This is the half of the list that needs eyes on the screen; the whole list is at `Audits/Blender Research/Dependencies/Hand tests.md`. Because it was run on a Mac, it settles macOS and leaves Windows and Linux open.

**What was done.** A file prepared for this test was opened. It holds two planes: the left one uses an image that was packed into the file, the right one uses an image that was not, and neither image file exists on disk any more. The Creator looked at the two images in the Image editor and at the Image Texture node in the Shading workspace, then used File ‣ External Data ‣ Report Missing Files.

**What came back.** Four screenshots, one per version, and these words from the Creator:

> "Report Missing Files says this:
> Path '/Users/artist/Documents/Smoozy hand tests/Dependencies/4.5/21/assets/loose.png' not found, from local data-block 'IMloose.png'
> Obviously 4.5 is for version 4.5 here, but the message is the same in all versions."

So the version's own folder appears in the path and the rest of the message is the same in all four. It is the same wording as the warning printed to the terminal when the same operator is called with no window.

The screenshots show the same three things in all four versions. In the image browse list, `packed.png` carries a small blue preview of itself and `loose.png` carries none. In the 3D viewport the plane using the packed image is blue — its own colour — and the plane using the unpacked one is bright magenta. In the Image editor the unpacked image shows an empty checkerboard.

**What the answer does not cover.** The entry also asked what sits to the left of each image's file-path field, to see whether the packed one carries a marker the unpacked one does not. In all four screenshots the image selected is `loose.png`, the unpacked one, and its row carries no such marker; what the packed image's row looks like was not photographed and is not reported. That part stays open.

**Both standing lines, brought up to date by this test.** *Driven from outside:* the wording of the report is now known to be the same with a window as without one, which the sources did not say. What the sources describe as landing in the Info editor is a window-only route; with no window the same text goes to the terminal. *Operating system:* macOS for this test. The same test was run on a rented Linux machine on 2026-08-06 and that result is recorded below; Windows stays untested.

**Observed — hand test entry 22, run by the Creator on 2026-08-05 on their own computer** (MacBook Pro, M1 Pro chip, macOS 26.5.2, Apple's own graphics), in Blender 5.0.1 with its window open. This is the half of the list that needs eyes on the screen; the whole list is at `Audits/Blender Research/Dependencies/Hand tests.md`. Because it was run on a Mac, it settles macOS and leaves Windows and Linux open.

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


**Observed — run again on Linux, 2026-08-06.** Every missing-thing case was built and run again in this version on the rented Linux machine.

**What a render does is the same on both machines.** With a missing image texture, a missing font, a missing linked library or a missing volume, the render finished, a frame was written, and the exit status was 0 in every case. The plane whose texture was missing came out magenta.

**What it says is also the same on both machines.** Cycles printed `00:00.278  cycles           | ERROR Image file '/root/…/assets/green.png' does not exist.` — with the path in quotation marks — and EEVEE printed `gpu.texture      | ERROR Failed to create GPU texture from Blender image`. The engine's own name on the command line is `BLENDER_EEVEE` here. The same as on the Mac.

The rest, all matching the Mac: a missing font still renders with the letterforms lost rather than falling back to the built-in font; a missing linked library still renders a frame with placeholders in place of what is gone; a missing volume answers `grids.load() = False` and `grids.error_message = 'smoke.vdb not found'`; a partly-packed file whose files are gone prints nothing at all when opened; and on a sequence editor made from a script the missing-media switch reads **on** while the property's declared default is **off**.

Report Missing Files works with no window on Linux and prints the same warning naming the path and the data-block. `WindowManager.reports` raised `AttributeError: 'WindowManager' object has no attribute 'reports'` here, with a window open as well as without one.

**Find Missing Files, which no Mac test reached.** It works with no window, returns `{'FINISHED'}`, **rewrites every repaired path as a full absolute path into the search folder**, and leaves alone the paths that already resolve. It does **not** match a name case-insensitively: with a file named `texture.png` in the search folder and the reference written `Texture.PNG` it printed `Warning: Could not find 'Texture.PNG' in '…'` and changed nothing.

**A broken output-path template, with a window open on the Linux machine.** In 5.0, 5.1 and 5.2 the File Path field is filled **solid red** with the text in white on the red, and the terminal carries `operator | ERROR Parse errors in path '//out_{blend_name.png': - Invalid or incomplete template expression '{blend_name.png'.` In 4.5 the Output panel sat below the visible part of the Properties editor and was not photographed on Linux.

**Both standing lines, brought up to date by the Linux tests.** *Driven from outside:* every reading above was taken with Blender running with no window unless it says otherwise, so it holds outside the interface. *Operating system:* **now settled on two operating systems, macOS and Linux, and each is its own result** — what they agree on and where they differ is said above. **Windows is still untried: no Windows machine has run it.**

---

## 13. Does a file pointing at another `.blend` file bring that file's own references with it, how deep does that go, and can they be reached the same way?

**Answer.** **Inferred:** linked data can bring indirect dependencies. The source pages expose indirectly linked IDs and parent libraries, while the asset-import passage says dependencies come along when an asset is appended. No quoted sentence states the full rule for an ordinary linked library.

**For appended assets, dependencies come along** in the plainest terms:

> "All of the asset and all its dependencies will be appended to the current file."
> ""Dependencies" in this case means everything the asset refers to. For an object, this can be its mesh and materials, but also other objects used by modifiers, constraints, or drivers."
> — Manual, https://docs.blender.org/manual/en/5.0/editors/preferences/file_paths.html (read 2026-07-30)

**The API separately exposes indirect linked data:**
> "is_library_indirect — Is this ID block linked indirectly"
> — Python API, https://docs.blender.org/api/5.0/bpy.types.ID.html (read 2026-07-30)

**And libraries themselves form a chain**, because a library data-block has a parent:
> "parent — Type: Library, (readonly)"
> — Python API, https://docs.blender.org/api/5.0/bpy.types.Library.html (read 2026-07-30)

**How deep.** The sources never state a depth limit, and two options are written in terms that assume arbitrary depth:

> "Localize All — Also copy all indirectly linked data, instead of maintaining the links."
> — Manual, https://docs.blender.org/manual/en/5.0/files/linked_libraries/link_append.html (read 2026-07-30)

> "use_recursive (boolean, (optional)) – Localize All, Localize all appended data, including those indirectly linked from other libraries"
> — Python API, `bpy.ops.wm.append`, https://docs.blender.org/api/5.0/bpy.ops.wm.html (read 2026-07-30)

> "Append (make local) also indirect dependencies of appended IDs coming from other libraries. NOTE: All IDs (including indirectly linked ones) coming from the same initial library are always made local."
> — Python API, `bpy.types.BlendImportContext`, https://docs.blender.org/api/5.0/bpy.types.BlendImportContext.html (read 2026-07-30)

The one direction that is documented as *not* working is backwards:
> "Circular Dependencies
> In general, dependencies should not go in both directions. Attempting to link or append data which links back to the current file will likely result in missing links."
> — Manual, link_append address above, read 2026-07-30

**Label for depth:** **inferred**, with the step written out. The sources state that indirect dependencies exist, that they come from "other libraries" (plural, and other than the first), and that a library has a `parent` library — but no sentence states how many levels are followed. My step is: a per-library `parent` pointer plus wording about dependencies "coming from other libraries" describes a chain rather than a single hop, so the chain is followed beyond one level. Anything stronger than "beyond one level" — in particular "unlimited" — I cannot get from the sources.

**Can they be reached the same way? Partly, and the limits are documented.**

*A relative path inside a library resolves against the library, not against the open file.* This is the single most consequential fact in this answer:

> "When using Blender data from linked libraries there is an unfortunate complication since the path will be relative to the library rather than the open blend-file. When the data block may be from an external blend-file pass the library argument from the bpy.types.ID.
> >>> bpy.path.abspath(image.filepath, library=image.library)"
> — Python API, https://docs.blender.org/api/5.0/info_gotchas_file_paths_and_encoding.html (read 2026-07-30)

Cycles does exactly this in its own code, which I can see rather than infer:

> `script_path = bpy.path.abspath(filepath, library=library)`
> — `Blender Sources/cycles_5.0/osl.py`, line 197 (read 2026-07-30). **observed**

> `osl_path = bpy.path.abspath(script.filepath, library=script.library)`
> — `Blender Sources/cycles_5.0/osl.py`, line 237 (read 2026-07-30). **observed**

Blender 5.0 also added path-template variables for exactly this distinction:
> "blend_name_lib: Like blend_name, except that for path properties on library-linked data-blocks it will be the name of the library blend file that the data-block comes from."
> "blend_dir_lib: Like blend_dir, except that for path properties on library-linked data-blocks it will be the path to the library blend file that the data-block comes from."
> — Manual, https://docs.blender.org/manual/en/5.0/files/file_paths.html (read 2026-07-30)

*Repair operations are restricted for indirect data.*
> "This operation is only available on data-blocks that are directly linked, and are not a dependency of any other linked data."
> — Manual, on Relocate Linked ID, link_append address above, read 2026-07-30

*Caches can be shared or not across the link, deliberately.*
> "Use Library Path — Share the disk cache when the physics object is linked into another blend-file. When this option is enabled, linked versions of the object will reference the same disk cache. Otherwise linked versions of the object will use independent caches."
> — Manual, https://docs.blender.org/manual/en/5.0/physics/baking.html (read 2026-07-30)

*And there is a cost documented for reaching into a library at all:*
> "Referencing compressed blend-files may need a lot of memory because they have to be loaded in their entirety, even if you only link/append a small part of them. Once the data-blocks are loaded, however, memory usage is the same."
> — Manual, link_append address above, read 2026-07-30

That matters here because 5.0 made compression the default: "Blendfile Compression on saving is now enabled by default" (Release notes, https://developer.blender.org/docs/release_notes/5.0/core/, read 2026-07-30).

**Label:** **inferred** that an ordinary linked library brings its own references and that the chain goes beyond one level, with the steps stated above. The path-resolution rules and repair limits are **documented**. The two Cycles lines are **observed** as lines in the supplied files; they establish only that those calls exist there.

**Not covered.** Whether a library's *own* library-relative paths chain correctly two or more levels deep — `abspath` takes one `library` argument, and the sources do not describe the multi-level case. Whether all libraries in a chain, or only the directly-linked ones, appear in the Outliner's Blender File view. Whether Reload and Relocate on a library also refresh the libraries beneath it.

**Outside the interface.** `bpy.path.abspath(..., library=...)` is exposed through Python, and the supplied Cycles file contains calls to it. Reading those lines does not establish what a running no-window render does with them. Relocate is documented only as an Outliner action (see Q11). The standing line is therefore only partly settled.

**Operating system.** The sources do not say.


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

## 14. Does rendering one scene of a file pull only that scene's references?

**Answer: not in the sources.** Blender 5.0's documentation says how to choose the scene, and says nothing about what that choice does or does not load.

What is documented is the choosing:

> "-S, --scene <name>
> Set the active scene <name> for rendering."
> — Manual, https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html (read 2026-07-30)

and that the scene is the top of the data tree:

> "Scene … Primary store of all data displayed and animated. Used as top-level storage for objects & animation."
> — Manual, https://docs.blender.org/manual/en/5.0/files/data_blocks.html (read 2026-07-30)

Two adjacent facts, which I set out because they bear on the question without answering it:

*Blend-files are documented as being read whole when they are a library* — "Referencing compressed blend-files may need a lot of memory because they have to be loaded in their entirety, even if you only link/append a small part of them." (Manual, https://docs.blender.org/manual/en/5.0/files/linked_libraries/link_append.html, read 2026-07-30). That is about a *library* being read into the open file, not about opening a file for rendering, so it does not settle the question.

*Selective loading exists, but only through Python* — `bpy.data.libraries.load` lets a script name exactly which data-blocks to bring in, per type, with the documented example `data_dst.scenes = ["Scene"]` (Python API, https://docs.blender.org/api/5.0/bpy.types.BlendDataLibraries.html, read 2026-07-30). Again, that is a script linking from a file, not Blender opening a file to render it.

**Label:** **not in the sources.**

**Where I looked.** Read in full: `advanced/command_line/arguments.txt` (Render Options and Python Options), `advanced/command_line/render.txt`, `files/blend/open_save.txt`, `files/data_blocks.txt`, `files/linked_libraries/link_append.txt`, `bpy.types.BlendDataLibraries`, `bpy.types.BlendData`, `bpy.ops.wm.txt` (open_mainfile and related). Searched manual, API documentation and release notes for `entirety`, `whole file`, `only that scene`, `active scene`, `--scene`, `unused data`, `orphan`. Checked the 5.0 release notes for core, rendering and python_api for any change to what loading pulls in. Nothing states whether opening or rendering a file loads all scenes' references or only the rendered scene's.

**Not covered.** Everything asked. Also not covered: whether memory use differs between rendering scene A and scene B of the same file; whether a missing file used only by an unrendered scene is reported. The nearest related rule is about *saving*, not loading — "This works by skipping zero user data-blocks when writing blend-files." (Manual, data_blocks address above, read 2026-07-30) — which concerns unused data, not other scenes' data.

**Outside the interface.** The `-S` argument is itself an outside-the-interface mechanism, so the choosing works there. What it pulls is not stated either way.

**Operating system.** The sources do not say.


**Observed — hand test entry 12, run 2026-08-05 on the Creator's own computer** (MacBook Pro, M1 Pro chip, macOS 26.5.2, Apple's own graphics), with Blender 5.0.1 started from a terminal with no window, using a throwaway settings folder set through `BLENDER_USER_RESOURCES`. The full list of tests is at `Audits/Blender Research/Dependencies/Hand tests.md`.

**What was done.** One file was built with two scenes. SceneA uses only `a.png`, SceneB uses only `b.png`; each has its own camera, sun and material, and both scenes were set to Cycles so no engine had to be named on the command line. `b.png` was moved out of the way, and the file rendered twice:

```
"<application>" --background --factory-startup <file>.blend --scene SceneA --render-output <folder>/onlyA_ --render-frame 1
"<application>" --background --factory-startup <file>.blend --scene SceneB --render-output <folder>/onlyB_ --render-frame 1
```

**What came back.** Rendering SceneA said nothing about `b.png`, in any version.

The control worked: rendering SceneB in the same file did name it — `cycles           | ERROR Image file '<path>/assets/b.png' does not exist.` — so the silence when SceneA is rendered is a real result and not a version that keeps quiet about everything. The same control in 4.5.12 said nothing either, so the test settles nothing there.

This is one-sided evidence: it shows that a render of one scene does not complain about the other scene's missing file. It does not show that the file was never opened. A second attempt at a stronger reading was made and abandoned — reading `image.has_data` and `image.size` after rendering SceneA reported `has_data=False` for both images, and reading `size` returned `(32, 32)` for both because reading `size` itself loads the file. Neither can tell whether a render opened a file. A file-activity trace on macOS needs an administrator password, so it was not used.

**Both standing lines, brought up to date by this test.** *Driven from outside:* the whole test was done with no window, and `--scene` chose the scene there. *Operating system:* macOS for this test. The same test was run on a rented Linux machine on 2026-08-06 and that result is recorded below; Windows stays untested.


**Observed — run again on Linux, 2026-08-06.** A two-scene file was built and rendered again in this version on the rented Linux machine. **Rendering SceneA said nothing about SceneB's missing image**, the same as on the Creator's Mac. The control — rendering the *other* scene, where the missing file certainly is reached — did name it: `cycles | ERROR Image file '/root/…/assets/b.png' does not exist.` So the silence in the first run means something here.

The evidence is one-sided on Linux for the same reason it was on the Mac: it shows that a render of one scene does not complain about the other scene's missing file, not that the file was never opened. **Nothing about this question is settled by the second machine.** **That was true of the test described here, and is no longer true of the question: a test that could come out either way was run on 2026-08-07 and is recorded further down this answer.**

**Both standing lines, brought up to date by the Linux tests.** *Driven from outside:* every reading above was taken with Blender running with no window unless it says otherwise, so it holds outside the interface. *Operating system:* **now settled on two operating systems, macOS and Linux, and each is its own result** — what they agree on and where they differ is said above. **Windows is still untried: no Windows machine has run it.**


**Observed — added by this round's hand tests, run 2026-08-07 on a rented Linux machine** (Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB on driver 580.95.05, an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory, and no display server at all), with Blender 5.0.1 started from a terminal with no window, `--factory-startup`, and a settings folder of its own made first and set through `BLENDER_USER_RESOURCES`. **This is an addition to a document written in an earlier round; nothing above it has been changed.** The full list of tests is at `Audits/Blender Research/Dependencies/Hand tests — added questions.md`.

**Why this test is not the earlier one.** The earlier test on this question watched whether Blender complained about the other scene's missing image. It never did — and in Blender 4.5 the renderer never names a missing image under any circumstances, so that test could not have come out the other way. What follows uses two readings that can each come out either way, and each has a control that worked.

**What was done.** One file was built with two scenes. SceneA holds a camera, a sun and a plane whose material uses `assets/a.png`. SceneB holds the same with `assets/b_small.png`, and also an object linked from `assets/lib.blend`. Both scenes are Cycles on the processor, one sample, 64 by 64. Beside them sits `assets/b_big.png`, a 6000 by 6000 image.

*The first reading — what is read from disk.* Each render was run under `strace`, which records every file the program opens:

```
strace -f -e trace=openat,newfstatat -o <trace file> \
  /opt/blender/5.0/blender --background --factory-startup <file> \
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
SceneA, where the big image is NOT used : 443/429/437 MB with the big one, 439/426/431 MB with the small one
SceneB, where the big image IS used     : 642/645 MB with the big one, 420/434 MB with the small one
```

Swapping a 32 by 32 image for a 6000 by 6000 one changed the runs of the scene that does not use it by nothing, and the control by well over a hundred megabytes. The machine was shared with other work while this ran, which is why three runs of each were taken; the variation between runs of the same thing is far smaller than the control's jump.

So, on this machine and in this version: **rendering one scene does not read the other scene's image from disk at all, and does not carry it in memory.** A `.blend` **linked as a library is opened whichever scene is rendered**, since it is read when the file is loaded rather than when a scene is drawn.

**Not settled by this.** It covers an image used by a scene's material and a linked library, and nothing else a file can point at. It does not cover a scene reached from the rendered one — as a background scene, through a compositor Render Layers node, or through a sequencer strip — which is a different case the answer above already describes.

**Both standing lines, brought up to date by this test.** *Driven from outside:* settled — the whole test was done with no window, and `--scene` chose the scene there. *Operating system:* Linux, on the machine named above. **The Creator's Mac was reached on 2026-08-07 by a third reading, and it agrees; that is recorded below.** Neither of the two readings used here could be taken on the Mac — the trace needs an administrator password, and the memory reading needs a machine nobody else is using. Windows is untested because no Windows machine has run it.

**Observed — the Mac half of this round's hand tests, run 2026-08-07 on the Creator's own Mac — a MacBook Pro with an M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card.** Blender 5.0.1 was started from a terminal with no window, with `--factory-startup` and a settings folder of its own, made before it was used and set through `BLENDER_USER_RESOURCES`; that this took effect was checked rather than assumed, and this Blender answered `RESOURCES …/depwork/res-5.0` and `VERSION 5.0.1`. **Other topics were running Blender on this same Mac throughout, so nothing was timed and no memory was measured.** **This is an addition; nothing above it has been changed.** The full list of tests is at `Audits/Blender Research/Dependencies/Hand tests — added questions.md`. In the quotations below the working folder's long path is written `…/depwork`; nothing else is altered.

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

**Answer.** Yes, and the finding mechanism is documented in full: environment variables, with a bundled default as the fallback.

**The default lives with the installation, not the file:**
> "Blender comes with a standard OpenColorIO configuration that contains a number of useful display devices and view transforms."
> — Manual, https://docs.blender.org/manual/en/5.0/render/color_management/opencolorio.html (read 2026-07-30)

and its place in the installation is named:
> "./datafiles/colormanagement
> Default OpenColorIO configuration."
> — Manual, https://docs.blender.org/manual/en/5.0/advanced/blender_directory_layout.html (read 2026-07-30)

**How Blender finds a different one:**
> "However, OpenColorIO is also designed to give a consistent user experience across multiple applications, and for this, a single shared configuration file can be used. Blender will use the standard OCIO environment variable to read an OpenColorIO configuration other than the default Blender one."
> "There is also a BLENDER_OCIO environment variable to change the configuration for Blender only. It is recommended to use OCIO when possible for compatibility with other software and pipelines that may not be aware of BLENDER_OCIO."
> — Manual, opencolorio address above, read 2026-07-30

with the precedence stated on the command-line page:
> "BLENDER_OCIO:
> Path to override the OpenColorIO configuration file.
> If not set, the OCIO environment variable is used."
> — Manual, https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html (read 2026-07-30)

**What the file holds is a name, not the profile.** The blend-file stores a colour-space *name* per image, and that name is resolved against whichever config is active:

> "name — Color space in the image file, to convert to and from when saving and loading the image … Type: enum in Color Space Convert Default Items, default 'NONE'"
> — Python API, `bpy.types.ColorManagedInputColorspaceSettings`, https://docs.blender.org/api/5.0/bpy.types.ColorManagedInputColorspaceSettings.html (read 2026-07-30)

> "The list of color spaces depends on the active OCIO config."
> — Manual, https://docs.blender.org/manual/en/5.0/editors/image/image_settings.html (read 2026-07-30) — the same sentence appears at https://docs.blender.org/manual/en/5.0/render/shader_nodes/textures/environment.html and https://docs.blender.org/manual/en/5.0/editors/video_sequencer/sequencer/sidebar/strip.html

**The config is expected to satisfy certain requirements**, and the manual says what happens when it does not:
> "Use OpenColorIO v2 display_colorspaces and view_transforms with cie_xyz_d65_interchange intermediate display linear space. This is used to emulate the chosen display on the actual display. In older configs without this, display emulation will be disabled and wide gamut and HDR display will not work well."
> "For every display, include a view transform without tone mapping. Blender will look for a view transform named Standard or Un-tone-mapped or the config wide default_view_transform. If not found, the first view transform of the display will be used."
> — Manual, opencolorio address above, read 2026-07-30

For ACES specifically: "for more complete support, the official ACES configurations can be manually installed and set through the OCIO environment variable." (same page and address).

**Label:** **documented** for the mechanism and the fallback order. **Inferred** for "the file therefore relies on the computer's config": the step is that the file stores a colour-space name, the available names come from the active config, and the active config is chosen by environment variables outside the file — no single sentence puts those three together, but each is stated.

**Not covered.** What happens when a name stored in the file does not exist in the active config — the sources describe a fallback for *view transforms* but not for input colour spaces. Whether a config path can be relative or must be absolute. Whether the config can be packed into the blend-file (nothing suggests it can; there is no colour-management entry in the packable data-block table).

**Outside the interface.** Both `OCIO` and `BLENDER_OCIO` are environment variables and are documented on the command-line arguments page. This establishes a way to select the configuration before Blender starts. The sources do not distinguish a windowed run from a no-window run for the resulting colour management.

**Operating system.** The manual does not distinguish operating systems for these variables. The bundled config is described relative to Blender's resource folders. The sources do not say whether the mechanism differs.


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

## 16. How is a font installed on one kind of computer recorded in the file, and what does Blender do when a font it points at is not on the computer?

**Answer, first half.** A font is recorded exactly like any other external file: a path on a Font data-block, with the option to hold the file inside the blend instead.

> "Font … References font files. Used by curve object-data of text objects."
> and, in the same table, Font is marked "✓" in both the Link and the Pack columns.
> — Manual, https://docs.blender.org/manual/en/5.0/files/data_blocks.html (read 2026-07-30)

> "filepath — Type: string, default "", (never None, blend relative // prefix supported)"
> "packed_file — Type: PackedFile, (readonly)"
> "pack()"
> — Python API, `bpy.types.VectorFont`, https://docs.blender.org/api/5.0/bpy.types.VectorFont.html (read 2026-07-30)

So a system-installed font is recorded by **where the font file sits**, not by a family name — the property is a file path, and Blender loads it by browsing to a file:

> "To load a different Font, click one of the Load buttons in the Font panel and navigate to a font file. The File Browser will give all valid fonts a capital "F" icon."
> "If you select a font that is unsupported by Blender, you will get the error Not a valid font."
> — Manual, https://docs.blender.org/manual/en/5.0/modeling/texts/properties.html (read 2026-07-30)

The manual tells the reader where such files typically live on one family of systems, which is the closest it comes to naming "a font installed on one kind of computer":

> "Location of Fonts on Unix
> Fonts are typically located under /usr/lib/fonts, or some variant like /usr/lib/X11/fonts, but not always. They may be in other locations as well, such as /usr/share/local or /usr/local/share, and possibly related sub-trees."
> — same page and address, read 2026-07-30

There is a preference for where the browser starts: "Fonts — Default location to browse for text object font files." (Manual, https://docs.blender.org/manual/en/5.0/editors/preferences/file_paths.html, read 2026-07-30).

**There is one font that is never a file reference:**
> "Blender comes with a built-in font by default that is displayed in each of the four font style data-block menus. The built-in font is always present and shows in this list as "Bfont"."
> — Manual, modeling/texts/properties address above, read 2026-07-30

**Answer, second half: not in the sources.** Nothing states what Blender does when a font a file points at is absent from the computer. It does not say the text disappears, nor that it falls back to Bfont, nor that it reports anything specific. The only documented recourses are the general ones that apply to every external file: Report Missing Files, Find Missing Files, and packing the font into the blend beforehand (Manual, https://docs.blender.org/manual/en/5.0/interface/window_system/topbar.html and https://docs.blender.org/manual/en/5.0/files/blend/packed_data.html, both read 2026-07-30). That the Font type is packable at all is the documented way to make the question moot.

**Label:** **documented** for the recording, the built-in font, the packability, and the Unix locations. **Not in the sources** for what happens when the font is missing.

**Where I looked, for the silent part.** Searched the whole manual, API documentation and release notes for `bfont`, `built-in font`, `default font`, `font file`, `missing`, `fallback`, `substitute`. Read in full: `modeling/texts/properties.txt`, `modeling/texts/introduction.txt`, `modeling/texts/editing.txt`, `bpy.types.VectorFont`, `bpy.ops.font.txt`, `bpy.types.BlendDataFonts`, `editors/preferences/file_paths.txt`, `files/data_blocks.txt`, `files/blend/packed_data.txt`, `interface/window_system/topbar.txt`. Also checked the 5.0 release notes for user_interface, core and pipeline_io. The only font-related failure message anywhere is "Not a valid font", and that is documented for *choosing* an unsupported file, not for a file that has gone.

**Not covered.** What a render containing a text object shows when the font is gone. Whether Blender consults the operating system's installed-font list at all (nothing suggests it does — every mechanism documented is a file path). Whether the interface font and the text-object font behave the same (the directory layout page lists "./datafiles/fonts — User interface fonts", which is a separate thing from a Font data-block).

**Outside the interface.** `VectorFont.filepath`, `packed_file` and `pack()` are exposed through Python, but the sources do not explicitly state their no-window behaviour. The loading side as documented is a File Browser action, and the "Not a valid font" error is described as something "you will get"; where it goes in background mode is not stated. What a background render does with a missing font is exactly the part that is not in the sources.

**Operating system.** Partly. The manual gives Unix font locations only, and gives them as typical rather than fixed ("but not always"). It gives no equivalent for Windows or macOS. Since fonts are recorded as file paths, an absolute path to a system font folder is inherently system-specific — but the sources do not say that, so I do not claim it as a finding.


**Observed — hand test entry 6, run 2026-08-05 on the Creator's own computer** (MacBook Pro, M1 Pro chip, macOS 26.5.2, Apple's own graphics), with Blender 5.0.1 started from a terminal with no window, using a throwaway settings folder set through `BLENDER_USER_RESOURCES`. The full list of tests is at `Audits/Blender Research/Dependencies/Hand tests.md`.

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


**Observed — hand test entry 22, run by the Creator on 2026-08-05 on their own computer** (MacBook Pro, M1 Pro chip, macOS 26.5.2, Apple's own graphics), in Blender 5.0.1 with its window open. This is the half of the list that needs eyes on the screen; the whole list is at `Audits/Blender Research/Dependencies/Hand tests.md`. Because it was run on a Mac, it settles macOS and leaves Windows and Linux open.

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

2026-08-04 — Opus 5 (Claude Code) — Dependencies, Blender 5.0

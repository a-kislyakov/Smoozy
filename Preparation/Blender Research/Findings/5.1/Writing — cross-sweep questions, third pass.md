# Writing — cross-sweep questions, third pass (Blender 5.1)

**Sources consulted, and when.** The Blender 5.1 manual, the Blender 5.1 Python API documentation, the Blender 5.1 release notes, and the Cycles add-on files supplied in `Blender Sources/cycles_5.1` — all as supplied in this folder, taken on **2026-07-30**, which is the date read for everything cited below. One manual picture was opened: `_images/render_output_properties_output_panel.png`.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

I answered question 11 only. I read the whole sheet for context, as instructed, and answered nothing else. No helper agents were used; every search and every reading below is my own.

---

## 11. While a render is still working, what is present in the places it writes to, and how does that change with the size of the picture?

### Reading of the question

I read "the places it writes to" as the folders a render puts files into — the output path, the folders a File Output node uses, and the working folders Blender uses during a render. I read "the size of the picture" as the pixel size of the rendered image (width × height, after the resolution percentage).

**Ambiguity, flagged.** "The size of the picture" could instead mean the size of the finished file on disk. Under that other reading the only thing the sources offer is the video autosplit rule quoted below. I answer the pixel-size reading, which is the one the sources actually speak to.

### Short answer

The sources describe four things that can be sitting in a render's folders while the render is still going:

1. **The frames that have already finished.** For an animation to an image format, each frame is written out as soon as it is done, so at any moment the output folder holds every earlier frame.
2. **Empty placeholder files — only if you switch that on.** Off by default.
3. **The video file being written**, if the output is a video rather than an image sequence.
4. **Working files elsewhere**: the temporary directory holds render layers while Blender is running, and Cycles caches tiles to disk during the render. If Cache Result is on, a multi-layer OpenEXR goes to the Render Cache folder.

The pixel size of the picture is what decides whether the Cycles tile cache on disk happens at all: tiling is for "high resolution images", and the Cycles add-on's own arithmetic says it kicks in once the rendered pixel area is larger than one tile's area (2048 × 2048 by default). The placeholder files do not change with picture size — they are empty whatever the resolution.

The one thing the sources do **not** say is the most operationally interesting: whether the frame *currently* being rendered exists as a half-written file in the output folder before it is finished. Nothing in the manual, the API documentation, the 5.1 release notes, or the supplied Cycles files describes a partial file, a temporary suffix, a write-then-rename, or any naming for work in progress. See the record of looking below.

---

### (a) Finished frames are already on disk — **documented**

> "While rendering stills will allow you to view and save the image from the render buffer when it is complete, animations are a series of images, or frames, and are automatically saved directly out to a drive after being rendered."

> "This allows you an easy recovery if there is a problem and you have to re-start the rendering, since the frames you have already rendered will still be in the output directory."

Source: manual — https://docs.blender.org/manual/en/5.1/render/output/animation.html — read 2026-07-30.

And from the same manual chapter's introduction:

> "For example, when rendering animations to image file formats the render job can be canceled and resumed at the last rendered frame by changing the frame range."

Source: manual — https://docs.blender.org/manual/en/5.1/render/output/introduction.html — read 2026-07-30.

Note the contrast the first quote draws: a *still* render is held in the render buffer and saved by hand; an *animation* render writes each frame out as it finishes. So during an animation render the folder is filling up frame by frame.

### (b) Empty placeholder files, if switched on — **documented**

> "Create empty placeholder frames while rendering."

Source: manual — https://docs.blender.org/manual/en/5.1/render/output/properties/output.html — read 2026-07-30.

> "use_placeholder
>
> Create empty placeholder files while rendering frames (similar to Unix 'touch') (default False)"

> "use_overwrite
>
> Overwrite existing files while rendering (default True)"

Source: Python API — https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html — read 2026-07-30.

The manual gives the reason this exists, in a hint on the same Output page:

> "An easy way to get multiple machines to share the rendering workload is to:
>
> Set up a shared directory over a network file system.
>
> Disable Overwrite, enable Placeholders in the Render Output panel.
>
> Start as many machines as you wish rendering to that directory."

Source: manual — https://docs.blender.org/manual/en/5.1/render/output/properties/output.html — read 2026-07-30.

**Picture size:** the files are described as *empty*, in both the manual and the API, and "similar to Unix 'touch'" — so their content does not vary with the resolution. That is documented wording, not my inference.

**Defaults, from the picture** — **documented**. I opened `_images/render_output_properties_output_panel.png` (the "Output panel." illustration on the Output properties page). It shows the panel with the file path `/tmp/`, **Overwrite ticked**, **Placeholders unticked**, **File Extensions ticked**, **Cache Result unticked**, format PNG. That is what is visible in the image; I read nothing further into it.

### (c) A video output file is present and can be left broken — **documented quote, inferred conclusion**

> "It's not recommended to render directly to a video format in the first instance. If a problem occurs while rendering, the file might become unplayable and you will have to re-render all frames from the beginning."

Source: manual — https://docs.blender.org/manual/en/5.1/render/output/properties/output.html — read 2026-07-30.

**Inferred step:** for a file to "become unplayable" when a render is interrupted, the video file must already exist on disk while the render is still working, holding the frames encoded so far. The source does not say that outright; I supplied it. Label: **inferred**.

Related, and about size — **documented**:

> "If your video is huge and exceeds 2GiB, enable Autosplit Output. This will automatically split the output into multiple files after the first file is 2GiB in size."

Source: manual — https://docs.blender.org/manual/en/5.1/render/output/properties/output.html — read 2026-07-30.

**Inferred step:** a larger picture makes each encoded frame bigger, so a 2GiB boundary is reached after fewer frames, so extra files appear sooner during the render. The source says nothing about picture size here; that connection is mine. Label: **inferred**.

### (d) File Output nodes write during the render — **documented**

> "The File Output node writes one or more images to disk during rendering or compositing."

> "Any missing directories are created automatically during output."

Source: manual — https://docs.blender.org/manual/en/5.1/compositing/types/output/file_output.html — read 2026-07-30.

So a node's folders are also filling during the render, and folders that did not exist appear as part of it. The page does not say at what point within a frame the write happens.

### (e) Working files away from the output folder — **documented**

The temporary directory:

> "The temporary directory is used to store various files at run-time (including render layers, physics cache, copy-paste buffer and crash logs)."

Source: manual — https://docs.blender.org/manual/en/5.1/advanced/blender_directory_layout.html — read 2026-07-30.

> "The Temporary Directory sets where to store files such as temporary renders and auto-saves."

Source: manual — https://docs.blender.org/manual/en/5.1/getting_started/configuration/introduction.html — read 2026-07-30.

The Render Cache folder, if Cache Result is enabled:

> "Saves the rendered view layers and their passes to a multi-layer OpenEXR image. The Compositor can then use this file to improve performance, especially for heavy compositing."
>
> "The image is stored in the Render Cache folder as specified in the File Paths Preferences."

Source: manual — https://docs.blender.org/manual/en/5.1/render/output/properties/output.html — read 2026-07-30.

> "use_render_cache
>
> Save render cache to EXR files (useful for heavy compositing, Note: affects indirectly rendered scenes) (default False)"

Source: Python API — https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html — read 2026-07-30.

There is one hint that this cache file exists before a render has succeeded, in the Read View Layers operator:

> "Reads all the current scene's render layers from cache, as needed. This can be used to save RAM while rendering because the render layers do not have to be saved in RAM. And also for recovering some information from a failed render."
>
> "For this to work, Cache Result must be enabled."

Source: manual — https://docs.blender.org/manual/en/5.1/interface/controls/nodes/editing.html — read 2026-07-30.

**Inferred step:** "recovering some information from a failed render" only makes sense if the cache file is written before the render finishes. The source does not say when it is written. Label: **inferred**, and weakly — it could equally be describing recovery of a *previous* completed render.

The API also exposes a temporary directory to a render engine, with no description at all:

> "temporary_directory
>
> (default "", readonly, never None)"

Source: Python API — https://docs.blender.org/api/5.1/bpy.types.RenderEngine.html — read 2026-07-30. The documentation gives no explanation of what goes in it.

### (f) How this changes with the size of the picture — the tile cache

**Documented**, from the manual:

> "Tile Size
>
> Render high resolution images in tiles of this size, to reduce memory usage. Tiles are cached to disk while rendering to save memory."

Source: manual — https://docs.blender.org/manual/en/5.1/render/cycles/render_settings/performance.html — read 2026-07-30.

That is the whole of what the manual says: high resolution images are rendered in tiles, and those tiles are cached to disk *while rendering*. It does not say which folder, what the files are called, how many there are, or how big.

**Observed**, from the Cycles files supplied. In `Blender Sources/cycles_5.1/properties.py`, lines 1051–1062:

```
    use_auto_tile: BoolProperty(
        name="Auto Tile",
        description="Deprecated, tiling is always enabled",
        default=True,
    )
    tile_size: IntProperty(
        name="Tile Size",
        default=2048,
        description="Render high resolution images in tiles of this size, to reduce memory usage. Tiles are cached to disk while rendering to save memory",
        min=8,
        max=8192,
    )
```

Two things are in that text: the tile size defaults to **2048** and is limited to 8–8192; and the old switch that turned tiling off is marked "Deprecated, tiling is always enabled".

The same point is made in `Blender Sources/cycles_5.1/version_update.py`, lines 114–119, where old files that had tiling switched off are converted:

```
            # Auto tiling is always enabled now
            if version <= (5, 0, 77):
                cscene = scene.cycles
                if not cscene.use_auto_tile:
                    cscene.use_auto_tile = True
                    cscene.tile_size = 8192
```

And `Blender Sources/cycles_5.1/ui.py`, lines 389–405, contains the arithmetic for when a render is actually split into tiles. Its own comment says the calculation matches the renderer's:

```
        if cscene.use_guiding:
            # Calculation matches TileManager::compute_render_tile_size and
            # Session::get_effective_tile_size
            if cscene.tile_size < 128:
                tile_size = cscene.tile_size
            else:
                tile_size = (cscene.tile_size + 128 - 1) & ~(128 - 1)
                tile_size = min(tile_size, 8192)
            tile_area = tile_size ** 2

            render_scale = scene.render.resolution_percentage / 100.0
            render_size_x = int(scene.render.resolution_x * render_scale)
            render_size_y = int(scene.render.resolution_y * render_scale)
            render_area = render_size_x * render_size_y

            if render_area > tile_area and render_size_x <= 8192 and render_size_y <= 8192:
                layout.label(text="May work poorly with render tiling", icon='INFO')
```

Label for the three blocks above: **observed** — that is what the supplied files say, quoted exactly.

**Inferred**, and here is the step I took. The add-on treats "the rendered pixel area is larger than one tile's area" as the condition under which the render is tiled, and its comment says that arithmetic matches the renderer's own. Put together with the manual's "Tiles are cached to disk while rendering", the conclusion is: **a render whose pixel area is at or below the tile area is not split into tiles and so writes no tile cache; a render whose pixel area exceeds it is split, and tile data goes to disk during the render.** With the default tile size of 2048, one tile's area is 4,194,304 pixels, so:

- 1920 × 1080 = 2,073,600 pixels — below the threshold.
- 3840 × 2160 = 8,294,400 pixels — above it.

The resolution percentage counts, because the code scales the resolution by it before comparing. I did not run anything; this is reasoning from the code and the manual sentence, so the label is **inferred**, not observed. A reader has to accept both that the comment is accurate and that "rendered in tiles" is the same condition as "tiles are cached to disk".

Two cautions on that inference. The code path quoted sits inside a path-guiding panel and its purpose there is to warn about guiding, not to document tiling. And the extra conditions `render_size_x <= 8192 and render_size_y <= 8192` in that line are about when the warning is shown; I cannot tell from the supplied files what they mean for tiling itself, and I have not guessed.

### (g) What the sources do not say

**Not in the sources**, each with the record of looking in the section below:

- Whether the frame being rendered right now exists in the output folder as an incomplete file before it is finished. No source describes a partial write, a `.tmp` or `.part` name, or a write-to-temp-then-rename.
- Where the Cycles tile cache is written, what those files are called, how many there are, or how large they are.
- Whether the Render Cache EXR is written during a render or only once it is done.
- Whether EEVEE, or the compositor, put anything on disk during a render.
- What the render engine's `temporary_directory` holds.

### Not covered

This answer does not settle the moment-by-moment state of the file for the frame in flight — the single thing most worth knowing — because the sources are silent on it. It does not settle where the tile cache lands on disk, nor the size or count of those files. It does not settle whether the tiling threshold I inferred is exactly the renderer's rule, only that the add-on shipped with Blender computes it that way. It does not cover render engines other than Cycles and EEVEE, nor rendering through the Video Sequencer. It says nothing about timing — how long a finished frame's file takes to appear after the frame is done.

### Outside the interface

Everything above is a scene setting stored in the file (Overwrite, Placeholders, Cache Result, the output path, Cycles' tile size) or a preference (Temporary Directory, Render Cache), not a property of the window, so nothing in the sources ties any of it to a person clicking. The manual documents that a render started from the command line writes files: "-f, --render-frame <frame> — Render frame <frame> and save it." and "-a, --render-anim — Render frames from start to end (inclusive)." and "-o, --render-output <path> — Set the render path and file name." (manual — https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html — read 2026-07-30). The background switch is described as "Run in background (often used for UI-less rendering)." on the same page. Worth noting: the quotes in (a) say a *still* render inside the interface is kept in the buffer for you to save, whereas `--render-frame` "and save it" writes the file — so for a single frame the two ways of starting a render differ in whether a file appears at all. Beyond that, **the sources do not say** whether anything about what sits in the folders mid-render differs when Blender is driven from outside its window.

### Operating system

The sources give no operating-system difference for placeholders, overwrite, tiling, or the tile cache. The placeholder description says "similar to Unix 'touch'", but that is a comparison for the reader, not a restriction — the property carries no platform note.

The one documented operating-system difference in this area is **where** the temporary directory is: "The temporary directory is selected based on the following priority: User Preference (see File Paths). Environment variables (TEMP on Windows, TMP & TMP_DIR on other platforms). The /tmp/ directory." (manual — https://docs.blender.org/manual/en/5.1/advanced/blender_directory_layout.html — read 2026-07-30). Note that the command-line page names a different variable for the same job: "TEMP: Store temporary files here (MS-Windows)." and "TMPDIR: Store temporary files here (UNIX Systems). The path must reference an existing directory or it will be ignored." (manual — https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html — read 2026-07-30). The two pages disagree on the non-Windows variable names; I report both and settle neither.

---

### Record of looking

Everything below was run over `Blender Sources/manual_5.1`, `Blender Sources/api_5.1`, `Blender Sources/release_notes_5.1` and `Blender Sources/cycles_5.1`, whole-folder, case-insensitive, before opening any page.

**Terms searched across the manual and the API documentation:** `placeholder`, `overwrite`, `while rendering`, `during rendering`, `during render`, `during the render`, `partial`, `incomplete`, `unfinished`, `still rendering`, `in progress`, `to disk`, `written directly`, `saved directly`, `.tmp`, `atomic`, `rename`, `truncat`, `corrupt`, `empty file`, `empty placeholder`, `touch`, `swap`, `spool`, `lock file`, `temporary file`, `temporary files`, `temp directory`, `temp folder`, `temporary render`, `/tmp`, `temporary directory`, `tile`, `tiles`, `tiling`, `tile size`, `Use Tiling`, `cached to disk`, `render cache`, `cache result`, `cached render`, `save_buffers`, `use_save_buffers`, `cachedir`, `tempdir`, `temp_dir`, `high resolution`, `large image`, `big image`, `8192`, `shader cache`, `compilation cache`, `kernel cache`, `out of memory`, `memory usage`.

**Terms searched across the 5.1 release notes:** `tile`, `cache`, `temp`, `placeholder`, `overwrite`, `partial`.

**Terms searched across the supplied Cycles files:** `tile` (all files), `fopen`, `ofstream`, `tmpnam`, `mkstemp`.

**Pages opened and read in full or in the relevant part:**

- https://docs.blender.org/manual/en/5.1/render/output/properties/output.html — the Output panel, Placeholders, Overwrite, Cache Result, Autosplit, the video warning, the render-farm hint. Read in full.
- https://docs.blender.org/manual/en/5.1/render/output/animation.html — read in full; source of the "saved directly out to a drive after being rendered" and "frames you have already rendered will still be in the output directory" quotes.
- https://docs.blender.org/manual/en/5.1/render/output/introduction.html — read in full.
- https://docs.blender.org/manual/en/5.1/render/cycles/render_settings/performance.html — read in full; the only manual sentence linking picture size to disk during a render.
- https://docs.blender.org/manual/en/5.1/advanced/blender_directory_layout.html — Temporary Directory section.
- https://docs.blender.org/manual/en/5.1/editors/preferences/file_paths.html — Temporary Files, Render, Render Cache entries.
- https://docs.blender.org/manual/en/5.1/getting_started/configuration/introduction.html — File and Paths section.
- https://docs.blender.org/manual/en/5.1/compositing/types/output/file_output.html — first sixty lines, covering when the node writes and directory creation.
- https://docs.blender.org/manual/en/5.1/interface/controls/nodes/editing.html — Read View Layers operator.
- https://docs.blender.org/manual/en/5.1/editors/image/editing.html — Open Cached Render.
- https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html — the render arguments, argument order, and the environment-variable list.
- https://docs.blender.org/manual/en/5.1/interface/window_system/topbar.html — the Render menu.
- https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html — `use_placeholder`, `use_overwrite`, `use_render_cache`, `use_persistent_data`, `resolution_x`, `resolution_percentage`.
- https://docs.blender.org/api/5.1/bpy.types.RenderEngine.html — `bl_use_image_save`, `temporary_directory`, `use_highlight_tiles`, `end_result`.
- https://docs.blender.org/api/5.1/bpy.app.html — `bpy.app.tempdir`, `bpy.app.cachedir`.
- Release notes read in full: https://developer.blender.org/docs/release_notes/5.1/rendering/ , https://developer.blender.org/docs/release_notes/5.1/cycles/ , https://developer.blender.org/docs/release_notes/5.1/pipeline_io/ , https://developer.blender.org/docs/release_notes/5.1/core/ , https://developer.blender.org/docs/release_notes/5.1/compositor/ . None of them mentions tiling, the tile cache, placeholders, or partial output.
- Picture opened: `_images/render_output_properties_output_panel.png`, the "Output panel." illustration on the Output properties page.
- Cycles files opened: `properties.py` (tile properties), `ui.py` (the tiling arithmetic and the Memory panel), `version_update.py` (the tiling conversion). I also listed the whole of `Blender Sources/cycles_5.1` (406 files): it contains only the add-on's Python, the render kernel under `source/kernel`, kernel utilities under `source/util`, and compiled shaders. **The session and tile-manager code that the `ui.py` comment names — `TileManager` and `Session` — is not in the supplied files**, which is why the tile cache's location and file naming cannot be settled from them. I checked this by searching the whole folder for `fopen`, `ofstream`, `tmpnam` and `mkstemp`; the only hit is in an unrelated ray-tracing header.

**Ruled out, and why:**

- EEVEE pages under `render/eevee` — searched for `disk`, `memory usage`, `out of memory`; the only hits are about video memory for volumes and light probes, nothing about writing to disk during a render.
- Video Sequencer and proxy pages — the `incomplete` hits there are about reading footage, not writing renders.
- Geometry-node and physics baking pages — they write to disk, but on a Bake operation, not during a render, so they are not "the places a render writes to".
- The `placeholder` hits in the Video Sequencer, glTF exporter, link/append, geometry-node string formatting, and UI-layout pages — all a different sense of the word, checked and discarded.
- `genindex` files were excluded from every search as they are index listings, not content.

---

### Things a person with Blender open could settle in a few minutes

Each of these is written so it can be carried out without seeing any of my reasoning. Use Blender 5.1 throughout.

1. **Does a half-written file appear for the frame in flight?**
   Open Blender. In Output properties set the file path to a fresh empty folder, format PNG, resolution 1920 × 1080. In Render properties set Cycles and raise the sample count until a single frame takes at least a minute (for example 4096 samples with Denoise off). Set the frame range to 1–3. Press Ctrl-F12. While frame 1 is still rendering, look in the output folder with the system file browser, with file sizes showing, and refresh it a few times. Write down: is there any file for frame 1 before it completes, what is it called, and what size is it? Repeat the look during frame 2, when frame 1's file already exists.

2. **The same, with Placeholders on.**
   Same setup, but in Output properties tick Placeholders and untick Overwrite. Render the same three frames and watch the folder. Write down when each file appears, what it is called, and its size in bytes at the moment it appears versus when the frame finishes.

3. **The same, at two picture sizes, to test the tiling threshold.**
   Keep Cycles' Tile Size at its default of 2048 (Render properties, Performance, Memory). Render one frame at 1920 × 1080, then one at 3840 × 2160, with a long enough sample count that each takes a couple of minutes. During each render, watch the Temporary Directory (Preferences, File Paths, Temporary Files — if blank, watch the system temporary folder) sorted by modification time. Write down which files appear during the 4K render that do not appear during the HD one, their names, and their sizes. Then repeat the 4K render with Tile Size set to 8192 and note whether those files stop appearing.

4. **Where the tile cache actually lives.**
   During the 4K render from step 3, note the full path of every file Blender creates or grows in the temporary directory, and whether they are removed when the render finishes or when Blender quits.

5. **Video output mid-render.**
   Set Media Type to Video, container Matroska, codec H.264, and render frames 1–50 of anything slow. While it runs, check whether the output video file exists and whether it is growing. Then cancel the render with Esc and try to open the resulting file in a video player. Write down whether it plays and how many frames it contains.

6. **Render Cache timing.**
   Tick Cache Result in Output properties and set a Render Cache folder you can watch (Preferences, File Paths, Render Cache). Start a slow single-frame render and watch that folder. Write down whether the EXR appears while the render is running or only when it finishes, and its size at each point.

7. **Still render started from the command line versus F12.**
   In a terminal, run Blender with `--background yourfile.blend --render-output /some/empty/folder/test_ --render-frame 1` and confirm a file appears. Then, in the interface, press F12 on the same file and confirm no file appears in that folder until you save it by hand. This settles whether the two ways of starting a still render differ in what lands on disk.

8. **File Output node timing.**
   Add a File Output node in the compositor pointed at an empty folder, with two inputs whose names contain a slash (for example `passes/diffuse`). Render three frames and note when each file and each created sub-folder appears relative to each frame finishing.

---

2026-08-11 — Claude Opus 5 (Claude Code) — Writing, Blender 5.1

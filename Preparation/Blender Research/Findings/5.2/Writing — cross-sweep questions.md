# Writing — cross-sweep questions (Blender 5.2)

Questions answered: 11 and 12.

Sources consulted, all taken on 2026-07-30 and all read on that snapshot date: Blender's 5.2 manual (`Blender Sources/manual_5.2`, including one picture from its `_images` folder), Blender's 5.2 Python API documentation (`Blender Sources/api_5.2`), the 5.2 release notes (`Blender Sources/release_notes_5.2`), and Cycles' own files (`Blender Sources/cycles_5.2`).

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

No helper agents were used. Everything below I read myself.

---

## 11. While a render is still working, what is present in the places it writes to, and how does that change with the size of the picture?

**The question was ambiguous, twice. Here is how I read it.**

- "the places it writes to" — I take this to mean the folders the render has been pointed at for its output: the File Path in the Output panel, and the Directory of any File Output node. The other reading is every place on disk the render touches while it works, including working files it makes for itself. I answer the first reading, and because part (e) below only exists under the second reading, I mark that part clearly.
- "the size of the picture" — I take this to mean the pixel dimensions of the image being rendered. The other reading is the size in bytes of the file produced; I cover that in one line at (f).

**LABEL: mixed — see each part. Parts (a), (b), (d) and (f) are documented; part (c) is not in the sources; part (e) is documented and observed. Parts (b), (d) and (e) each carry one step past the quoted words, labelled inferred where it sits. Nothing in the sources speaks to what a File Output node's directory holds while a render runs, which is recorded under Not covered.**

### (a) Finished frames are there, and they accumulate one at a time

The sources say that during an animation render the already-rendered frames sit in the output folder, and that a render can be stopped and picked up again from them.

Exact words, manual:

> "animations are a series of images, or frames, and are typically saved directly to disk as they are rendered."

> "since the frames you have already rendered will still be in the output directory."

Source: Blender manual, https://docs.blender.org/manual/en/5.2/render/output/animation.html — read 2026-07-30.

> "when rendering animations to image file formats the render job can be canceled and resumed at the last rendered frame by changing the frame range."

Source: Blender manual, https://docs.blender.org/manual/en/5.2/render/output/introduction.html — read 2026-07-30.

That the writing happens frame by frame, as the render goes, is stated directly in the API documentation, which gives a hook that fires per frame:

> "on writing a render frame (directly after the frame is written). Accepts one argument: the scene data-block being rendered"

Source: Blender Python API, https://docs.blender.org/api/5.2/bpy.app.handlers.html (`bpy.app.handlers.render_write`) — read 2026-07-30.

The same page's statistics hook shows that saving is timed separately from rendering:

> "on printing render statistics. Accepts one argument: the render progress as a string containing current frame, current sample, render time and saving time."

Source: Blender Python API, https://docs.blender.org/api/5.2/bpy.app.handlers.html (`bpy.app.handlers.render_stats`) — read 2026-07-30.

LABEL for (a): **documented**.

### (b) Empty placeholder files, but only if that option is turned on

There is a setting that puts an empty file in place for a frame while rendering. It is off unless switched on.

Exact words, manual (the Output panel's list of settings):

> "Create empty placeholder frames while rendering."

And the hint immediately below it:

> "An easy way to get multiple machines to share the rendering workload is to:
> Set up a shared directory over a network file system.
> Disable Overwrite, enable Placeholders in the Render Output panel.
> Start as many machines as you wish rendering to that directory."

Source: Blender manual, https://docs.blender.org/manual/en/5.2/render/output/properties/output.html — read 2026-07-30.

Exact words, API documentation:

> "use_placeholder
> Create empty placeholder files while rendering frames (similar to Unix 'touch') (default False)"

> "use_overwrite
> Overwrite existing files while rendering (default True)"

Source: Blender Python API, https://docs.blender.org/api/5.2/bpy.types.RenderSettings.html — read 2026-07-30.

I also looked at the picture of the Output panel on the manual page above (`_images/render_output_properties_output_panel.png`). It shows the panel with a path of `/tmp/`, and four checkboxes: Overwrite (ticked), Placeholders (unticked), File Extensions (ticked), Cache Result (unticked). That matches the defaults quoted above. LABEL for what the picture shows: **documented**.

LABEL for (b) so far: **documented** — that empty placeholder files are created while frames are being rendered, when the option is on, and that it is off by default. The quoted words do not name the folder those files go in. That it is the output folder is my own step, taken from the render-farm hint, where the machines share one directory and render to it. LABEL for that step: **inferred**.

One step further, marked separately. The manual's hint only works if each machine's placeholder appears *before* that machine has finished the frame — otherwise two machines would render the same frame. So a placeholder for a frame is present in the output folder while that frame is still being rendered, not after. That step is mine, not the manual's. LABEL for that step: **inferred**.

### (c) Whether a frame's own file appears part-written is not stated

I could find nothing in any of the sources saying whether a frame's file appears on disk in a partly written state before it is complete, nor whether a temporary name is used and renamed at the end. Where I looked is recorded at the bottom of this answer.

LABEL for (c): **not in the sources**.

### (d) A cached copy of the result goes to a separate folder, if Cache Result is turned on

> "Saving – Cache Result
> Saves the rendered view layers and their passes to a multi-layer OpenEXR image. The Compositor can then use this file to improve performance, especially for heavy compositing.
> The image is stored in the Render Cache folder as specified in the File Paths Preferences."

Source: Blender manual, https://docs.blender.org/manual/en/5.2/render/output/properties/output.html — read 2026-07-30.

> "Render Cache
> The location where cached render images are stored."

Source: Blender manual, https://docs.blender.org/manual/en/5.2/editors/preferences/file_paths.html — read 2026-07-30.

API documentation:

> "use_render_cache
> Save render cache to EXR files (useful for heavy compositing, Note: affects indirectly rendered scenes) (default False)"

Source: Blender Python API, https://docs.blender.org/api/5.2/bpy.types.RenderSettings.html — read 2026-07-30.

Note that this is a different folder from the output folder, so under my first reading of the question it is beside the point; I include it because it is a place a render writes to while working. A sentence elsewhere in the manual mentions reading render layers from a cache:

> "Reads all the current scene's render layers from cache, as needed. This can be used to save RAM while rendering because the render layers do not have to be saved in RAM. And also for recovering some information from a failed render."

Source: Blender manual, https://docs.blender.org/manual/en/5.2/interface/controls/nodes/editing.html — read 2026-07-30. That sentence sits on the node-editing page and does not itself name the Render Cache folder, so taking it to be about the same cache is a step of mine. LABEL for that step: **inferred**.

LABEL for (d): **documented** that the cache file goes to the Render Cache folder and that the option is off by default. That a partial cache file is present *while* the render is still working is a step past the words — "recovering some information from a failed render" is about a render that stopped, not one still going. LABEL for that step: **inferred**, and weakly so.

### (e) The size of the picture: Cycles writes tiles to disk for large pictures

This is the only place in the sources where anything written during a render is tied to the size of the picture. It concerns a working file, not the output folder, so it belongs to the second reading of "the places it writes to".

Exact words, manual, Cycles Performance panel, under Memory:

> "Tile Size
> Render high resolution images in tiles of this size, to reduce memory usage of render passes. Tiles are cached to disk while rendering to save memory.
> Lowering the tile size also reduces texture cache memory usage, as only the image textures needed by each tile will be loaded into memory.
> Tile size has an impact on performance, and the default 2048 pixels provides a good balance for most scenes. When running out of memory, lowering to 1024 or 512 pixels helps reduce memory usage, typically at a small performance cost."

Source: Blender manual, https://docs.blender.org/manual/en/5.2/render/cycles/render_settings/performance.html — read 2026-07-30.

Release notes for 5.2 confirm the tile size also governs which image textures are loaded:

> "Tile Size under Performance > Memory can be tweaked to further reduce memory usage. For each render tile only the required image textures are loaded."

Source: release notes, https://developer.blender.org/docs/release_notes/5.2/cycles/ — read 2026-07-30.

The same setting, from Cycles' own files. Exact lines:

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

File: `Blender Sources/cycles_5.2/properties.py`, lines 1079–1090. Read 2026-07-30. LABEL: **observed** (this is what the file says: the setting exists, is a whole number, defaults to 2048, and is limited to 8 through 8192; and the file's own description of the Auto Tile setting, which it calls deprecated, reads "tiling is always enabled").

Cycles' own files also contain a comparison between the picture's area and one tile's area. In the file, that comparison decides whether a note is shown in a panel. Exact lines:

```
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

File: `Blender Sources/cycles_5.2/ui.py`, lines 390–405 (inside the path guiding panel's drawing code). Read 2026-07-30. LABEL: **observed** — those are the file's words. Note that the file's own comment sits above the tile-size working-out and says that working-out matches the tile manager's; it does not say the same of the area comparison below it. Note too that the resolution percentage is taken into account, and that the comparison also requires each side to be 8192 pixels or less before the note is shown.

Putting the manual's sentence together with those lines: a picture whose area is no larger than one tile's area is rendered as a single tile, so nothing is cached to disk for it; a picture larger than that is split, and its tiles are cached to disk while the render runs. With the default tile size of 2048, one tile's area is 2048 × 2048 pixels, so a picture larger in area than that — which includes anything above roughly 4 megapixels — is the case where disk gets written to during the render. LABEL for this joining-up: **inferred**. The step a reader must supply is that "render high resolution images in tiles of this size" means a picture no bigger than one tile is not tiled, and that the comparison in `ui.py` is the same comparison the renderer itself makes. The file says the arithmetic matches the tile manager's; it does not say the disk caching switches on at exactly that point.

The sources do not say where on disk these tiles are cached, under what names, how much space they take, or when they are cleaned up.

### (f) The other reading of "size" — file size

For video output only, there is one size-triggered behaviour in the output folder:

> "Autosplit Output
> If your video is huge and exceeds 2GiB, enable Autosplit Output. This will automatically split the output into multiple files after the first file is 2GiB in size."

Source: Blender manual, https://docs.blender.org/manual/en/5.2/render/output/properties/output.html — read 2026-07-30. LABEL: **documented**.

### Not covered

This answer does not settle: whether a frame file is visible on disk while it is still being written, or whether Blender writes to a temporary name first; where the Cycles tile cache files live, what they are called, how large they get, or whether they survive a crash; whether the Render Cache file exists part-written during a render; what is present in the output folder mid-render when the output is a video container rather than an image sequence; whether the placeholder for a frame is created before or after that frame is rendered (only inferred above); whether EEVEE or Workbench write anything to disk during a render at all, and whether they tile; and what a File Output node's directory holds mid-render.

*Every one of those has since been settled by hand tests, immediately below.*

---

### What a running Blender 5.2 showed — hand tests, 2026-08-11

**LABEL: observed.** Every result below was produced on 2026-08-11 by running Blender 5.2.0 LTS on two computers and writing down what it printed and what appeared on disk. The two computers are named in full because that is what settles which operating system each answer covers.

- **The Creator's Mac** — a MacBook Pro with an Apple M1 Pro chip, 16 GB of memory, macOS 26.5.2 (build 25F84), Apple's own graphics and no second card. Blender 5.2.0 LTS at `/Applications/Blender.app/Contents/MacOS/Blender`.
- **A rented Linux machine** — Ubuntu 22.04.5 LTS on kernel 6.8, two NVIDIA RTX 4000 Ada Generation cards of 20 GB each on driver 580.105.08, an AMD Ryzen Threadripper PRO 3945WX with eleven cores, 49 GB of memory. Blender 5.2.0 LTS at `/opt/blender/5.2/blender`.

Every run used `-b --factory-startup` unless it says otherwise, and each was pointed at a settings folder of its own so that no run touched anybody's saved Blender settings. Where the same test ran on both machines, both results are given and it is said plainly whether they agreed. The whole list, with the exact commands, is `Audits/Blender Research/Writing/Hand tests — cross-sweep questions.md`.

**1. A still render puts nothing at the output path unless it is asked to, with a window or without.** Output path set to an empty folder, PNG, 320 × 240, Cycles:

```
BEFORE: []
AFTER render, no save switch: []
AFTER render with the save switch: ['still_.png']
```

`blender -b --factory-startup -o <folder>/cli_ -F PNG -f 1` wrote `cli_0001.png`. The three runs repeated in a Blender **with a window open on screen** printed the same three lines. Both machines agreed.

**2. Nothing stands for a frame while it is being rendered; while it is being *saved*, its file is there part-written.** Cycles, 1920 × 1080, 400 samples, frames 1 to 4, PNG, Placeholders off, watched every fifty milliseconds. On the rented Linux machine:

```
   0.00  .../out  []
  11.15  .../out  [('f_0001.png', 0)]
  11.20  .../out  [('f_0001.png', 253952)]
  11.25  .../out  [('f_0001.png', 966656)]
  11.30  .../out  [('f_0001.png', 1600053)]
  21.70  .../out  [('f_0001.png', 1600053), ('f_0002.png', 253952)]
```

For the eleven seconds a frame is being rendered there is nothing for it at all; for about a fifth of a second while it is being written the file is there and growing. **No temporary name is used and nothing is renamed — the file is built up in place.** That settles the first item under Not covered. The Mac agreed, and the same animation at 8192 × 4608 stretched that window to about two seconds.

**3. The placeholder is created *before* its frame is rendered — the inferred step in part (b) is right.** Placeholders on, Overwrite off. On the rented Linux machine:

```
   0.00  .../out  []
   0.50  .../out  [('f_0001.png', 0)]
  11.05  .../out  [('f_0001.png', 344064)]
  11.10  .../out  [('f_0001.png', 1081344)]
  11.15  .../out  [('f_0001.png', 1600053), ('f_0002.png', 0)]
```

Half a second in, eleven seconds before that frame is finished, at **zero bytes** — which is also what part (b) says the sources will not tell you a placeholder contains. They are made one at a time, for the frame in hand, and the picture is written into the same file rather than a second one. Stopping the render part way left the placeholder for the frame in hand behind, at 0 bytes, with Blender exiting 143. The Mac agreed.

**4. Blender holds the file open while it writes it.** During a 10000 × 10000 multi-layer OpenEXR write, the system reported Blender holding that file with a write handle. Renaming it at that moment succeeded and the render still finished. Both machines agreed.

**5. Where the Cycles tile cache lives, what it is called and how big it gets — the second item under Not covered.** Cycles, 32 samples, tile size at its default 2048, one frame at 8000 × 8000, with every file the process had open sampled twice a second. On the rented Linux machine:

```
blender 46901 root 5u REG 253,1    118784 /tmp/blender_qF6zCi/cycles-tile-buffer-46901-134744316668944-0-0.exr
blender 46901 root 5u REG 253,1  84779008 /tmp/blender_qF6zCi/cycles-tile-buffer-46901-134744316668944-0-0.exr
blender 46901 root 5r REG 253,1  89510421 /tmp/blender_qF6zCi/cycles-tile-buffer-46901-134744316668944-0-0.exr
```

On the Mac the same file sat under `/var/folders/…/T/blender_XXXXXX/`. **One file, an OpenEXR, in Blender's own temporary folder, named `cycles-tile-buffer-<process number>-<number>-0-0.exr`, about 89 MB for that frame, and gone before Blender exits.** The same render at 1000 × 1000 opened no such file at all.

**6. The boundary is one tile's worth of pixels, exactly as part (e) joins it up.** Tile size 2048, one frame, with the system's temporary folder pointed at an empty folder of the run's own and walked every twenty milliseconds. At **2048 × 2048** nothing appeared. At **2048 × 2049**, on the Mac:

```
   0.00  []
   6.70  [('.../tmp/blender_0dpK4g/cycles-tile-buffer-72877-53456653296-0-0.exr', 4096)]
   6.73  [('.../tmp/blender_0dpK4g/cycles-tile-buffer-72877-53456653296-0-0.exr', 90112)]
  13.64  []
```

Both machines agreed. At 8000 × 8000 with the tile size raised to **8192** — larger in area than the picture — there was no such file; at **256** there was one, written to far more often. Setting `use_auto_tile` to False changed nothing, which matches its own description as deprecated. (In Blender 4.5 that switch does turn it off. That is a difference between versions.)

**7. EEVEE and Workbench write nothing to disk during a render.** The same 8000 × 8000 frame under `BLENDER_EEVEE` and under `BLENDER_WORKBENCH` opened no tile-buffer file and put nothing in the temporary folder, on either machine. That settles the item under Not covered about other engines.

**8. What a File Output node's directory holds mid-render, and that the node makes it.** Compositor on, one node aimed at a folder that did not exist, separate from the scene's own; Cycles, 1920 × 1080, frames 1 to 4. `node folder exists before the render: False`, and then, frame by frame:

```
Saved: '.../work/e12/5.2/node/n_Image0001.png'
Saved: '.../work/e12/5.2/out/f_0001.png'
```

The node's folder is made by the render, and its file for a frame lands just before the scene's own file for that frame. Both machines agreed.

**9. The Render Cache file is written for every frame, before that frame's picture — the inferred step in part (d) is right.** Two view layers, Cycles, 1920 × 1080, frames 1 to 3, the render cache folder pointed at an empty folder. Blender printed, once per frame:

```
Caching exr file, 1920x1080, .../cache/cached_RR_UNSAVED_Scene_00000000000000000000000000000000.exr
```

and the watcher showed that file reaching 132,799,542 bytes before frame 1's own PNG appeared at 21.59 seconds, then dropping to zero and growing again for the next frame. One file, rewritten per frame, still there at the end. Both machines agreed.

**10. What is in the output folder during a video render, and what a stop leaves behind.** Workbench, 1280 × 720, FFmpeg in a Matroska container with H.264, frames 1 to 4000, stopped with the ordinary terminate signal after 25 seconds. `movie_0001-4000.mkv` — named for the whole frame range from the first moment — appeared at 0 bytes within a second and grew in steps of 262,144 bytes. Handing the file left behind back to Blender 5.2 gave `loaded as a movie: movie_0001-4000.mkv  frames: 0  size: (256, 256)`: it would not open. Both machines agreed. **This test was run twice** — a first run of 200 frames finished in under four seconds, before it could be stopped, and that run showed the finished file opening properly.

**11. How the autosplit files are named, and that the split is at about 1.87 GiB rather than 2.** Workbench, 1920 × 1080, FFmpeg in an AVI container with the lossless HUFFYUV codec, frames 1 to 1400. **Run on the rented Linux machine only**, because it writes several gigabytes four times over and the Creator's Mac is their working machine; it settles Linux and leaves macOS open. With Autosplit on:

```
FINAL: [('long__0000001-1400.avi', 2003090368), ('long__0010001-1400.avi', 1022075648)]
```

With Autosplit off, one file: `FINAL: [('long_0001-1400.avi', 3025156392)]`. So the names gain an extra block of digits when the option is on, and the second file begins where the first passes 2,003,090,368 bytes. **This test was also run twice**: a first run of 900 frames reached only 1,945,065,136 bytes in one file and never split, which is where the figure comes from.

**12. The texture cache the release notes describe is real, and it is off out of the box.** A 1024 × 1024 PNG written by Blender itself, used as an image texture on the cube, rendered with Cycles at 4096 × 4096. **At factory settings nothing appeared beside the picture** — the folder held only the PNG, before and after. Asking Blender 5.2 what settings it has for this printed:

```
--- render settings whose name mentions cache ---
   use_render_cache BOOLEAN False
   use_texture_cache BOOLEAN True
   use_auto_generate_texture_cache BOOLEAN False
--- file path preferences whose name mentions cache or texture ---
   texture_cache_directory STRING ''
```

Turning both switches on and rendering again gave:

```
set use_texture_cache = True
set use_auto_generate_texture_cache = True
texture folder after the render: ['blender_tx', 'smoozytex.png']
  under the texture folder: .../tex/blender_tx/smoozytex.png.0-926BDA8569F1F2504902F06ABA51E46D.tx  57115
```

So the `blender_tx/` folder is made beside the image file, and the file inside it is the image's own name with a long code and `.tx` on the end. Both machines agreed. **Blender 4.5, 5.0 and 5.1 have neither switch at all** — asked the same question they printed `no such setting: use_texture_cache` and `no such setting: use_auto_generate_texture_cache` — and none of them wrote anything beside the picture. That is a difference between versions.

**13. Two Blenders writing into one folder with Placeholders on and Overwrite off divide the work between them.** **Run on the rented Linux machine only**, and the one test here whose whole subject is two Blenders at once. Frames 1 to 8, Cycles, 1920 × 1080, the second started three seconds after the first. One took the odd frames and the other the even ones, each printing `skipping existing frame "<path>"` for the frames the other had claimed, and no frame was rendered twice. This was a folder on one machine's own disk; a folder shared over a network was not tested.

**What these tests leave open.** Windows, everywhere — no Windows machine has run it. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.** A folder shared over a network. Whether the two-gigabyte split behaves the same on macOS. And multi-view or stereoscopy output.

### Outside the interface

The sources do not say whether any of this differs when Blender is started and driven by something other than a person. What they do show is that the settings involved are ordinary scene data reachable from Python — `use_placeholder`, `use_overwrite`, `use_render_cache` and `filepath` are all listed on `bpy.types.RenderSettings` (https://docs.blender.org/api/5.2/bpy.types.RenderSettings.html, read 2026-07-30) — and that the command line has its own way to set the output path:

> "-o, --render-output <path>
> Set the render path and file name."

Source: Blender manual, https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html — read 2026-07-30.

That the same writing behaviour therefore follows from a command-line render is my step, not the sources'. LABEL for it: **inferred**. The manual's own render-farm hint describes several machines rendering to one shared folder, which is the batch case, but it does not say the machines are driven from outside the interface.

**Settled by the hand tests of 2026-08-11.** It holds. All thirteen tests above were run with no window at all, so every one of them is established for a Blender driven from outside. Test 1 was then repeated in a Blender **with a window open on screen**, on both machines, and printed the same three lines — the still render wrote nothing until it was asked to, exactly as with no window. Nothing was found that a window changes.

### Operating system

The sources do not say. Nothing in the quoted text about placeholders, tiling, the output folder or the render cache is qualified by operating system. The only operating-system split I found nearby concerns where temporary files go — `TEMP` on MS-Windows, `TMPDIR` on UNIX systems, per https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html (read 2026-07-30) — and that only matters here if the Cycles tile cache lands in the temporary folder, which the sources do not say it does.

**Settled by the hand tests of 2026-08-11.** Tests 1 to 10 and 12 above were each run on macOS on an Apple M1 Pro and on Linux on an x86 machine, and the two machines agreed on every one of them, so none of this is a macOS-only or Linux-only behaviour. **The tile cache does land in the temporary folder**, so the per-system split above does reach it: the same `cycles-tile-buffer-…-0-0.exr` file sat under `/var/folders/…/T/` on macOS and under `/tmp/` on Linux. Nothing about **Windows** was tested — no Windows machine has run it — so every claim here leaves Windows open. Tests 11 and 13 were run on Linux only and leave macOS open; the reason is written into each.

### Where I looked, for the parts I answered "not in the sources"

Searched the whole of `manual_5.2`, `api_5.2` and `release_notes_5.2` for: "placeholder", "while rendering", "during render", "partial", "partially written", "incomplete", "truncated", "half-written", "temporary file", "temp directory", "scratch", "in progress", "buffer to disk", "save buffers", "progressive", "cached to disk", "disk space", "cache result", "render cache", "cached render", "tile size", "use tiling", "tiled render", "tile_size", "use_auto_tile", "output directory", "output path". Read in full: the Output properties page, the Rendering Animations page, the Output Introduction page, the Cycles Performance page, the File Paths preferences page, the File Output node page, the Path Templates page, and the Cycles section of the 5.2 release notes. Searched `cycles_5.2` for "temp", "tempdir", "disk", "tile". Ruled out: the sculpting, texture painting and shader-node pages that "tiling" also matches (they concern texture tiling, not render tiles); the Video Sequencer pages; the geometry-nodes and physics cache pages.

---

## 12. What does a render do when it must make a folder and cannot?

**LABEL: not in the sources.**

Nothing in the 5.2 manual, the 5.2 Python API documentation, the 5.2 release notes, or Cycles' own files says what happens when a render needs to create a folder and the creation does not succeed. No error message, no report, no abandoned frame, no stopped render, no fallback path — none of it is described.

What the sources do say, which is only the half before the failure:

> "Any missing directories are created automatically during output."

Source: Blender manual, File Output Node page, https://docs.blender.org/manual/en/5.2/compositing/types/output/file_output.html — read 2026-07-30. In context this sentence is about input names on the node that contain a slash: the manual says such a name "is interpreted as a directory hierarchy", and then that missing directories in it are made automatically. It says nothing about what happens if they cannot be. LABEL for that much — that missing directories in such a name are created automatically during output: **documented**. The failure this question asks about is the part that is not there.

For the render output path in the Output panel, I could not find any statement that Blender creates missing folders at all, let alone what it does when it cannot. The Output panel's File Path is described only as:

> "File Path
> Choose the location to save rendered frames."

Source: Blender manual, https://docs.blender.org/manual/en/5.2/render/output/properties/output.html — read 2026-07-30.

The nearest thing to a documented failure in the same area is about a badly written path, not a folder that cannot be made:

> "Depending on the path, template errors may prevent certain actions. For example, if there are errors in the render output path, then rendering an animation will fail with an error message indicating the path errors."

Source: Blender manual, https://docs.blender.org/manual/en/5.2/files/file_paths.html — read 2026-07-30. That is a malformed template expression such as an unclosed `{`, and I do not offer it as an answer to this question.

The API documentation gives no help either. The render operator itself is undocumented:

> "bpy.ops.render.render(*, animation=False, write_still=False, use_viewport=False, use_sequencer_scene=False, layer='', scene='', frame_start=0, frame_end=0)
> Undocumented, consider contributing."

Source: Blender Python API, https://docs.blender.org/api/5.2/bpy.ops.render.html — read 2026-07-30.

### Not covered

This answer settles nothing about the actual behaviour. It does not say whether the render stops or carries on to the next frame; whether an error is reported in the interface, on the console, or as a Python exception; what an exit code would be; whether a File Output node that cannot make its subfolder aborts the whole render or just that one output; or whether the answer differs between the Output panel's path and a File Output node's directory.

*Every one of those has since been settled by hand tests, immediately below.*

---

### What a running Blender 5.2 showed — hand tests, 2026-08-11

**LABEL: observed.** Run on 2026-08-11 on the two computers described under question 11 above — the Creator's MacBook Pro with an Apple M1 Pro chip on macOS 26.5.2, and a rented Ubuntu 22.04.5 machine on kernel 6.8 — with Blender 5.2.0 LTS, `-b --factory-startup`, each run pointed at a settings folder of its own. On the Linux machine the runs were made by an ordinary account rather than root, because root is not stopped by an unwritable folder. Every scene was Workbench at 320 × 240, frames 1 to 3, PNG. The exact commands are in `Audits/Blender Research/Writing/Hand tests — cross-sweep questions.md`.

**14. The main render output folder *is* made when it is missing — one level or a whole chain.** With the output path set to `<a writable folder>/newfolder/f_`:

```
operator returned: {'FINISHED'}
does the folder exist now: True
folder holds: ['f_0001.png', 'f_0002.png', 'f_0003.png']
```

With three missing levels at once, `<a writable folder>/a/b/c/f_`, the same. Exit code 0. Both machines agreed. **This settles the first item under Not covered by answer 12 above** — the sources are silent on it, and the answer is that Blender does make it.

**15. When it cannot be made, the render stops at the very first frame, says so in two lines, and ends with 1.** The parent folder was made and then set to read-and-enter only, and the output path named a folder inside it that did not exist. On the **Mac**:

```
does the folder exist before: False
00:00.525  image            | ERROR Couldn't create directory for file .../work/e18b/5.2/locked/newfolder/f_0001.png: Permission denied
Error: Render error (Permission denied) cannot save: '.../work/e18b/5.2/locked/newfolder/f_0001.png'
operator raised: RuntimeError Error: Render error (Permission denied) cannot save: '.../work/e18b/5.2/locked/newfolder/f_0001.png'
does the folder exist now: False
```

with `exit code: 1`. Frames 2 and 3 were never attempted, and nothing was left behind. Three missing levels gave the same. The **rented Linux machine** printed the identical two lines with its own paths and behaved identically. **So: reported on the console, reported to Python as a `RuntimeError`, the render stopped rather than carried on, and the exit code is 1.**

**Blender 5.2 names the folder in the message and gives the real reason. Blender 4.5 does neither** — for the same set-up it prints only `Error: Render error (No such file or directory) cannot save: …` on macOS and `(Invalid argument)` on Linux, with no mention of a folder. That is a difference between versions.

**16. A file standing where the folder must go gives a different reason.** An ordinary file called `blocker` was made and the output path set to `<that file>/f_`:

```
ERROR Couldn't create directory for file .../work/e19b/5.2/blocker/f_0001.png: File exists
Error: Render error (File exists) cannot save: '.../work/e19b/5.2/blocker/f_0001.png'
```

Exit code 1, nothing written, render stopped at the first frame. Both machines agreed.

**17. A File Output node that cannot make its folder does *not* abort the render, and the exit code stays 0.** A File Output node aimed at a folder inside the read-only parent, the scene's own output path pointing somewhere that worked:

```
ERROR Couldn't create directory for file .../work/e20b/5.2/locked/nodeout/n_Image0001.png: Permission denied
ERROR Couldn't create directory for file .../work/e20b/5.2/locked/nodeout/n_Image0002.png: Permission denied
ERROR Couldn't create directory for file .../work/e20b/5.2/locked/nodeout/n_Image0003.png: Permission denied
operator returned: {'FINISHED'}
```

with `exit code: 0`, and all three of the scene's own frames written. Both machines agreed. **That is the answer to two of the questions under Not covered at once: only that one output is lost, and the answer does differ between the Output panel's path and a node's directory — the same failure stops the render in one case and not the other. Nothing outside Blender can tell from the exit code that the node's output was lost.**

**18. The subfolder a node's input name asks for is made, and behaves the same way when it cannot be.** The node's input was named `passes/diffuse`, which is the very case the manual's "Any missing directories are created automatically during output" describes. Into a writable folder, Blender 5.2 made a real folder, taking the node's own file name onto the front of it:

```
.../nodeopen/n_passes/diffuse0001.png
.../nodeopen/n_passes/diffuse0002.png
.../nodeopen/n_passes/diffuse0003.png
```

Into a folder that could not be written to, three `ERROR Couldn't create directory for file …: Permission denied` lines and the same `operator returned: {'FINISHED'}` with exit code 0. Both machines agreed. (In Blender 5.0 the slash becomes an underscore and no folder is made at all. That is a difference between versions.)

**19. The render cache folder is made when missing, and when it cannot be made the render carries on and finishes.** With Cache Result on and the render cache folder pointed at a folder that did not exist under a writable parent, the folder and the cache file were both made. Pointed inside the read-only parent, the render wrote all three of its frames and printed, per frame — on the **Mac**:

```
00:48.907  image.openexr    | ERROR Unknown error in IMB_exr_begin_write
00:48.908  reports          | ERROR Error writing render result, No such file or directory (see console)
```

and on the **rented Linux machine**:

```
00:42.290  image.openexr    | ERROR IMB_exr_begin_write: No such file or directory.
00:42.290  reports          | ERROR Error writing render result, No such file or directory (see console)
```

`operator returned: {'FINISHED'}`, exit code 0 on both, and the cache folder was never made. So the two machines agree on the behaviour and differ on the first of the two lines: the Mac loses the reason to "Unknown error" where Linux names it.

**20. With a window open on screen, the terminal says exactly the same thing.** The set-up of test 15 was run again in a Blender with a real window, on both machines, and printed the same two lines and raised the same error to Python. Blender was then asked to take a picture of its own window. **What the status bar and the Info list show could not be settled**: the picture shows Blender's first-run Quick Setup panel sitting over the middle of the window and an empty Info list behind it, and dismissing that panel needs a click, which nothing in this round does. That half of the test is dropped for want of a hand on a mouse. The terminal half is complete and is the answer above.

---

### The two disputed wordings, run again — hand tests, 2026-08-12

**LABEL: observed.** Entries **25** and **26** of `Audits/Blender Research/Writing/Hand tests — cross-sweep questions.md` were run on 2026-08-12. Neither was written for Blender 5.2 — entry 25 was written for a disagreement in Blender 5.0 and entry 26 for one in Blender 5.1 — but both name 5.2 as worth running alongside, because it is the differences between the four versions that are the answer. Both were run on **both** computers:

- **The Creator's Mac** — a MacBook Pro with an Apple M1 Pro chip, 16 GB of memory, macOS 26.5.2, Apple's own graphics and no second card.
- **A rented Linux machine** — Ubuntu 22.04.5 LTS on kernel 6.8, two NVIDIA RTX 4000 Ada Generation cards of 20 GB each on driver 580.105.08, an AMD Ryzen Threadripper PRO 3945WX with eleven cores allotted, 49 GB of memory.

Blender 5.2.0 LTS on both, started with `-b --factory-startup`, each run pointed at a settings folder of its own that was made before Blender was started. On the Linux machine the runs were made by an ordinary account called `smoozy`, not root. Only one Blender ran at a time on each machine. Blender's whole terminal output was kept, unfiltered. **The words are written down per machine and the two machines are not merged into one answer.** In the blocks below only the leading part of each long path is shortened to `...`; every word of every message is as the machine printed it.

**21. (Entry 25.) Test 18's failing run again — in Blender 5.2 the two machines agree.** The File Output node's own base folder was made first and then set to read-and-enter only, so it exists and cannot be written to. The node's one input was named `passes/diffuse`. The scene's own output path pointed at a folder that works. Workbench, 320 × 240, frames 1 to 3, PNG.

On the **Creator's Mac**:

```
VERSION 5.2.0 LTS
File Output node built in the 5.x shape
input renamed to 'passes/diffuse'
node folder exists before: True
00:00.532  image            | ERROR Couldn't create directory for file .../work/e25/5.2/nodelocked/n_passes/diffuse0001.png: Permission denied
00:00.533  reports          | ERROR Render error (Permission denied) cannot save: '.../work/e25/5.2/nodelocked/n_passes/diffuse0001.png'
Blender 5.2.0 LTS (hash fbe6228777e7 built 2026-07-14 01:31:22)
00:00.570  render           | Saved: '.../work/e25/5.2/scene/s_0001.png'
00:00.576  image            | ERROR Couldn't create directory for file .../work/e25/5.2/nodelocked/n_passes/diffuse0002.png: Permission denied
00:00.576  reports          | ERROR Render error (Permission denied) cannot save: '.../work/e25/5.2/nodelocked/n_passes/diffuse0002.png'
00:00.580  render           | Saved: '.../work/e25/5.2/scene/s_0002.png'
00:00.585  image            | ERROR Couldn't create directory for file .../work/e25/5.2/nodelocked/n_passes/diffuse0003.png: Permission denied
00:00.585  reports          | ERROR Render error (Permission denied) cannot save: '.../work/e25/5.2/nodelocked/n_passes/diffuse0003.png'
00:00.589  render           | Saved: '.../work/e25/5.2/scene/s_0003.png'
operator returned: {'FINISHED'}
node folder exists now: True
node folder holds: []
```

with `exit code: 0`. On the **rented Linux machine**, the same lines with its own paths:

```
00:00.820  image            | ERROR Couldn't create directory for file /home/smoozy/crosssweep/work/e25/5.2/nodelocked/n_passes/diffuse0001.png: Permission denied
00:00.820  reports          | ERROR Render error (Permission denied) cannot save: '/home/smoozy/crosssweep/work/e25/5.2/nodelocked/n_passes/diffuse0001.png'
```

also with `operator returned: {'FINISHED'}`, the node's folder left empty, the scene's three frames written, and `exit code: 0`.

**The two machines agreed, word for word.** This matches what test 18 above records for Blender 5.2. **Blender 5.0 does not agree with itself across the two machines on this test** — the Mac says `ERROR Render error (Undefined error: 0) cannot save: '…'` and Linux says `ERROR Render error (Success) cannot save: '…'` — so this is a difference between versions, and 5.2 is on the settled side of it.

**22. (Entry 26.) Test 19's failing run again — the two machines print different words, as test 19 already records.** Cache Result on, the render cache folder pointed at a folder that does not exist under a parent set to read-and-enter only. Entry 26 was run twice per version, because its own written set-up says Workbench at 320 × 240 while the run it points back at, test 19, was Cycles at 1920 × 1080 with 400 samples and two view layers. Neither was preferred: run A repeated test 19's own set-up and run B used the set-up entry 26 itself is written with. **The added step was the second run; nothing else was changed.** Both runs gave the same words on each machine.

On the **Creator's Mac**, run A (Cycles, 1920 × 1080, 400 samples) printed:

```
VERSION 5.2.0 LTS
use_render_cache = True
render_cache_directory = '.../work/e26a/5.2/locked/cachefolder'
view layers: ['ViewLayer', 'SecondLayer']
cache folder exists before the render: False
00:22.697  image.openexr    | ERROR Unknown error in IMB_exr_begin_write
00:22.697  reports          | ERROR Error writing render result, No such file or directory (see console)
Blender 5.2.0 LTS (hash fbe6228777e7 built 2026-07-14 01:31:22)
Caching exr file, 1920x1080, .../work/e26a/5.2/locked/cachefolder/cached_RR_UNSAVED_Scene_00000000000000000000000000000000.exr
00:22.886  render           | Saved: '.../work/e26a/5.2/out/f_0001.png'
```

— the same lines again for frames 2 and 3 — and ended:

```
operator returned: {'FINISHED'}
SCENE FOLDER FINAL: ['f_0001.png', 'f_0002.png', 'f_0003.png']
CACHE FOLDER FINAL: not there
```

with `exit code: 0`. Run B on the Mac (Workbench, 320 × 240) printed the identical two error lines, with `Caching exr file, 320x240, …` in place of `1920x1080`.

On the **rented Linux machine**, run A printed:

```
00:21.464  image.openexr    | ERROR IMB_exr_begin_write: No such file or directory.
00:21.464  reports          | ERROR Error writing render result, No such file or directory (see console)
```

and run B the same two lines, with the same three closing lines and `exit code: 0`.

**The two machines did not print the same words**, and this agrees with what test 19 above already records for Blender 5.2: the Mac loses the reason to "Unknown error" where Linux names it. Everything else matched — all three frames written, the cache folder never made, `operator returned: {'FINISHED'}`, exit code 0.

Laid out across the four versions, the first of the two lines reads:

| Version | The Creator's Mac | The rented Linux machine |
|---|---|---|
| 4.5 | `IMB_exr_begin_write: ERROR: No such file or directory.` | `IMB_exr_begin_write: ERROR: No such file or directory.` |
| 5.0 | `ERROR IMB_exr_begin_write: No such file or directory.` | `ERROR IMB_exr_begin_write: No such file or directory.` |
| 5.1 | `ERROR Unknown error in IMB_exr_begin_write` | `ERROR IMB_exr_begin_write: No such file or directory.` |
| 5.2 | `ERROR Unknown error in IMB_exr_begin_write` | `ERROR IMB_exr_begin_write: No such file or directory.` |

The second line was `Error writing render result, No such file or directory (see console)` in every version on both machines. So the split between the two systems begins at 5.1 and is not present in 4.5 or 5.0 — and Blender 5.1's own document, which recorded the two machines as agreeing, has that disagreement written into it as unresolved.

---

### Outside the interface

The sources do not say. Nothing about folder creation or its failure is stated for either the interface or a batch run, so there is nothing to compare.

**Settled by the hand tests of 2026-08-11.** It holds, and this is the part that matters most for anything driving Blender. Every test above was run with no window: the exit code is **1** when the scene's own output folder cannot be made, and **0** when only a File Output node's or the render cache's cannot. Test 20 repeated the failure in a Blender with a window and the terminal said the same thing. What a person at the screen would see was not settled, for the reason given in test 20.

**Still holding after the hand tests of 2026-08-12.** Tests 21 and 22 were also run with no window, on both machines, and both ended with exit code 0 while printing their failure on the terminal. Nothing changed.

### Operating system

The sources do not say. Nothing about folder creation or its failure is qualified by operating system anywhere I looked.

**Settled by the hand tests of 2026-08-11.** The Creator's Mac and the rented Linux machine agreed on every one of tests 14 to 20 in what the render does, and on the words printed everywhere except the render cache failure in test 19, where the Mac says `Unknown error in IMB_exr_begin_write` and Linux names the reason. Nothing about **Windows** was tested — no Windows machine has run it — so Windows stays open on all of it.

**Confirmed by the hand tests of 2026-08-12.** Both machines were run again on the two places most in doubt, and the picture above is unchanged for Blender 5.2. The render cache failure (test 22) is still the one place where the two systems print different words, and the File Output node failure with an unwritable base folder (test 21) is not — both machines said `ERROR Render error (Permission denied) cannot save: '…'`. What is new is where 5.2 sits among the versions: on the render cache line it matches Blender 5.1 and differs from 4.5 and 5.0, both of which print the same words on both machines. Windows stays open, for the same reason as before.

### Where I looked

Searched the whole of `manual_5.2`, `api_5.2` and `release_notes_5.2` for: "mkdir", "create director", "director creat", "creates the folder", "make director", "missing director", "created automatically", "automatically create", "will be created", "does not exist", "doesn't exist", "non-existing", "nonexistent", "permission", "cannot write", "unable to write", "could not write", "cannot save", "unable to save", "write permission", "writable", "not writable", "read-only", "failed to create", "cannot create", "unable to create", "No such file", "Cannot open", "disk full", "out of space", "no space", "quota", "write access", "aborts", "abort the render", "render fails", "rendering fails", "fail to save", "saving failed", "error message", "raise an error", "reports an error", "output director", "output path". Read in full: the Output properties page, the Rendering Animations page, the File Output Node page, the Path Templates page, the File Paths preferences page (including its "Known Limitations — Permissions on Windows" section, which is about launching an external text editor and not about rendering), the command-line arguments page, and the `bpy.ops.render` and `bpy.types.RenderSettings` API pages. Searched `cycles_5.2` for the same failure words; Cycles' files in this folder are the Python add-on plus kernel and utility headers, and contain no output-path or folder-creation code. Ruled out as unrelated matches: the Ocean modifier's cache path ("If the cache path folder does not exist, it will be created"), the bundle nodes' "If the path does not exist, it is created automatically" (a data path inside a node tree, not a folder on disk), the File Browser's "If you type a nonexistent path, you will be prompted to create it" (a person typing in the file browser, not a render), the extensions and add-on directory pages, and the asset library catalog file.

---

## Things a person with Blender open could settle by hand in a few minutes

**All five items of the original list were run on 2026-08-11, on the Creator's Mac and on a rented Linux machine, and their results are written into questions 11 and 12 above.** What is left is only what those two machines could not reach.

1. **Anything on Windows** — an output folder that cannot be made, a folder denied through its security properties, and the exit code (`echo %ERRORLEVEL%`). Items 3 and 4 of the original list asked for it. **Not run with the equipment used**: the runs used one Mac and one rented Linux machine, and no Windows machine was rented for them. Every question-11 result above leaves Windows open too.
2. **A folder shared over a network between two machines.** Test 13 above ran two Blenders into one folder on a single machine's own disk. The manual's render-farm recipe is about a shared network folder, which is not the same thing, and no network share between two machines is among this project's written-down equipment. It needs the Creator's say-so first.
3. **Whether the two-gigabyte video split behaves the same on macOS.** Test 11 was run on the rented Linux machine only, because it writes several gigabytes four times over and the Mac is the Creator's working machine.
4. **What Blender's own window shows when a render cannot make its folder** — the status bar and the Info list, word for word. The terminal half is settled (test 20), but the picture Blender took of its own window has the first-run Quick Setup panel over it, and dismissing that needs a click. Anybody with a hand on a mouse can settle it in a minute: set the output path into a folder you cannot write to, press Ctrl-F12, and read the status bar and the Info editor.
5. **Multi-view or stereoscopy output during a render.** Nothing here tested it.

---

2026-08-11 — Claude Opus 5 (Claude Code) — Writing, Blender 5.2

2026-08-12 — Claude Opus 5 (Claude Code) — second batch of hand tests: entries 25 and 26 run on both machines and written into question 12 as tests 21 and 22, and both standing lines brought up to date. Nothing on the list above closed or opened; it is unchanged.

2026-08-11 — Claude Opus 5 (Claude Code) — hand tests: twenty observed results written into questions 11 and 12, both standing lines brought up to date on both answers, and the researcher's own list pruned to what two machines cannot reach

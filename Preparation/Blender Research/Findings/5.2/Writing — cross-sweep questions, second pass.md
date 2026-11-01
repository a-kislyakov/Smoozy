# Writing — cross-sweep questions, second pass (Blender 5.2)

**Sources consulted, and when.** Blender's manual for 5.2 (`Blender Sources/manual_5.2`, including its pictures in `_images`), Blender's Python API documentation for 5.2 (`Blender Sources/api_5.2`), the Blender 5.2 release notes (`Blender Sources/release_notes_5.2`), and Cycles' own files as shipped with Blender 5.2 (`Blender Sources/cycles_5.2`). All four were taken on **2026-07-30**, and that is the date read for everything quoted below. The work was done on **2026-08-11**.

`I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.`

Questions answered here: **11 and 12** only.

---

## 11. While a render is still working, what is present in the places it writes to, and how does that change with the size of the picture?

**LABEL: mixed — partly documented, partly inferred, partly observed, and partly not in the sources.** The pieces are labelled one by one below.

**How I read the question.** It is ambiguous in two ways, and I say plainly which reading I answered.

- *"the places it writes to"* — I read this as the places a render puts its **output**: the Output panel's File Path, and the directory of any File Output node. The other reading is "every place on disk Blender touches during a render", which would also take in the temporary directory, the render cache and the texture cache. I answer the first reading, and then cover the second separately under (F) because part of the size-of-picture answer lives there.
- *"the size of the picture"* — I read this as the **pixel resolution** of the render. The other reading is the size of the file in bytes; I cover the one documented thing that turns on file size (the 2 GiB autosplit) under (D), so neither reading is left empty.

---

### (A) What sits in the output folder part-way through an animation render — the completed frames so far

**Label: documented**, for the two statements; **inferred** for the joining of them.

Exact words, all from the manual page *Rendering Animations*:

> "While rendering stills will allow you to view and save the image from the render buffer when it is complete, animations are a series of images, or frames, and are typically saved directly to disk as they are rendered."

> "Blender creates a file for each frame of the animation when Save Output is enabled. If it is disabled, frames will only be written if a Compositor File Output node is configured."

> "This allows you an easy recovery if there is a problem and you have to re-start the rendering, since the frames you have already rendered will still be in the output directory."

> "Just disable the Overwrite option to start rendering where you left off."

Source: Blender manual — https://docs.blender.org/manual/en/5.2/render/output/animation.html — read 2026-07-30.

**The inferred step.** The source does not say, in one sentence, "at any moment during an animation render the output folder holds the frames finished so far". It says frames are saved to disk as they are rendered, and it says that after a render is broken off the already-rendered frames are still in the output directory. A reader has to put those two together to get the mid-render picture. That joining is mine, so the claim is inferred, not documented.

### (B) Placeholders — empty files that appear ahead of the picture

**Label: documented.**

> "Create empty placeholder frames while rendering."

Source: Blender manual — https://docs.blender.org/manual/en/5.2/render/output/properties/output.html — read 2026-07-30.

The same option in the Python API, which also gives its default:

> "use_placeholder
>
> Create empty placeholder files while rendering frames (similar to Unix ‘touch’) (default False)"

Source: Blender Python API — https://docs.blender.org/api/5.2/bpy.types.RenderSettings.html — read 2026-07-30.

And the use the manual puts it to:

> "Primitive Render Farm
>
> An easy way to get multiple machines to share the rendering workload is to:
>
> Set up a shared directory over a network file system.
>
> Disable Overwrite, enable Placeholders in the Render Output panel.
>
> Start as many machines as you wish rendering to that directory."

Source: Blender manual — https://docs.blender.org/manual/en/5.2/render/output/properties/output.html — read 2026-07-30.

**A flaw in the source, reported as found.** On that manual page the Placeholders option is printed under a heading that reads "Image Sequence – Overwrite Image Multi-Layer EXR", which is the *same* heading printed above the Overwrite option immediately before it. The heading is wrong; the description under it ("Create empty placeholder frames while rendering.") is the Placeholders one. I did not correct the source, I am naming the defect.

**Label: documented (from a picture).** The manual's own screenshot of the Output panel shows Overwrite and Placeholders as two separate checkboxes, with Overwrite ticked and Placeholders unticked; File Extensions ticked and Cache Result unticked; the path field showing `/tmp/`. Picture: `_images/render_output_properties_output_panel.png`, shown on https://docs.blender.org/manual/en/5.2/render/output/properties/output.html — read 2026-07-30. What is visible there is only what I have just listed; anything about behaviour is not visible in it.

Overwrite's default is stated outright in the API:

> "use_overwrite
>
> Overwrite existing files while rendering (default True)"

Source: Blender Python API — https://docs.blender.org/api/5.2/bpy.types.RenderSettings.html — read 2026-07-30.

**What this does *not* say.** Neither source says what a placeholder file contains, how large it is, when in the frame's life it is created, or whether it is created for the frame currently being worked on or for all frames at once. "Similar to Unix ‘touch’" is the nearest the sources come, and reading a size of zero bytes out of that phrase would be me supplying the meaning. Not in the sources.

### (C) Rendering straight to a video container

**Label: documented.**

> "Blender creates one file that holds all the frames of your animation."

Source: Blender manual — https://docs.blender.org/manual/en/5.2/render/output/animation.html — read 2026-07-30.

> "It’s not recommended to render directly to a video format in the first instance. If a problem occurs while rendering, the file might become unplayable and you will have to re-render all frames from the beginning."

Source: Blender manual — https://docs.blender.org/manual/en/5.2/render/output/properties/output.html — read 2026-07-30.

So the sources do say that a video file exists on disk before the render has finished, and that if the render breaks off that file may be unplayable. They do not describe what a part-written video file looks like beyond that.

### (D) More than one video file can appear, and that turns on file size

**Label: documented.**

> "If your video is huge and exceeds 2GiB, enable Autosplit Output. This will automatically split the output into multiple files after the first file is 2GiB in size."

Source: Blender manual — https://docs.blender.org/manual/en/5.2/render/output/properties/output.html — read 2026-07-30.

This is the only thing I found in any of the four sources that makes what is present in the output folder depend on a *size*, and the size it depends on is the size of the file in bytes, not the size of the picture in pixels. The sources do not say how the split files are named.

### (E) Whether a single output file is visible while it is still being written

**Label: not in the sources.**

Nothing in the manual, the API documentation, the release notes or Cycles' own files says whether the file for one frame appears on disk in a partly-written state, or only appears complete. The two nearest things are both about *when* writing has finished, not about what is on disk before it has:

> "bpy.app.handlers.render_write
>
> on writing a render frame (directly after the frame is written). Accepts one argument: the scene data-block being rendered"

> "bpy.app.handlers.render_stats
>
> on printing render statistics. Accepts one argument: the render progress as a string containing current frame, current sample, render time and saving time."

Source, both: Blender Python API — https://docs.blender.org/api/5.2/bpy.app.handlers.html — read 2026-07-30.

"Saving time" being reported separately from render time tells you saving is its own step; it does not tell you what the file looks like during that step. I am not going further than that.

*Where I looked for this.* Searched the whole of `manual_5.2`, `api_5.2`, `release_notes_5.2` and `cycles_5.2` for: "partially", "partial file", "incomplete", "in progress", "as it renders", "written to disk", "saved to disk", "flush", "truncated", "corrupt", "unplayable", "empty file", "zero byte", "0 byte", "touch", "each frame is", "after each frame", "frame is written", "while rendering", "during rendering". Opened and read in full: the Output properties page, the Rendering Animations page, the render Output introduction page. Ruled out the whole `manual_5.2/files/import_export/` tree (it is about importers and exporters, not renders), the video-editing tree, and the `contribute/` tree (instructions for writing the manual itself).

### (F) The size of the picture — Cycles caches render tiles to disk, and only for high-resolution images

**Label: documented**, for the fact and its trigger.

> "Tile Size
>
> Render high resolution images in tiles of this size, to reduce memory usage of render passes. Tiles are cached to disk while rendering to save memory.
>
> Lowering the tile size also reduces texture cache memory usage, as only the image textures needed by each tile will be loaded into memory.
>
> Tile size has an impact on performance, and the default 2048 pixels provides a good balance for most scenes. When running out of memory, lowering to 1024 or 512 pixels helps reduce memory usage, typically at a small performance cost."

Source: Blender manual — https://docs.blender.org/manual/en/5.2/render/cycles/render_settings/performance.html — read 2026-07-30.

**Label: observed**, from Cycles' own files, for the setting's exact name, kind, default and range:

In `Blender Sources/cycles_5.2/properties.py`, lines 1079–1090:

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

Two things are on the page there. The tile size is a whole number, default 2048, lowest 8, highest 8192. And the old on/off switch carries the description "Deprecated, tiling is always enabled".

**Label: observed**, for when tiling actually engages, from `Blender Sources/cycles_5.2/ui.py`, lines 389–405:

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

What is observed here is that Cycles' own interface code compares the render's area — width times height, after the resolution percentage has been applied — against the square of the effective tile size, and that its comment says this calculation matches the tile manager's. The plain reading is that a render whose area is not greater than one tile is not split into tiles at all, and one whose area is greater is. **That reading is inferred, not observed**: the code I quoted decides whether to show a warning, not whether to tile, and the step from "the warning uses this comparison, and its comment says the comparison matches the tile manager" to "this is the condition under which tiling happens" is one I took.

**So, the answer to the size part.** Below roughly one tile's worth of pixels (2048 × 2048 by default, after the resolution percentage), nothing is cached to disk because the picture is not split. Above it, Cycles caches tiles to disk while the render runs. That is the one documented way the size of the picture changes what a working render puts on disk.

**Label: not in the sources**, for *where* those cached tiles land. I could not find any statement of the folder or file names Cycles uses for tile caching. The manual says only "cached to disk". The nearest documented thing is the temporary directory:

> "The temporary directory is used to store various files at run-time (including render layers, physics cache, copy-paste buffer and crash logs)."

Source: Blender manual — https://docs.blender.org/manual/en/5.2/advanced/blender_directory_layout.html — read 2026-07-30.

That says render layers go to the temporary directory. It does not say Cycles' cached tiles do. I am not making that leap.

**Nothing in the sources says the render output folder itself gains anything extra because the picture is large.** Under my first reading of "the places it writes to", the answer to the size part is therefore: not in the sources. The size-driven behaviour I did find is in a different place on disk.

*Where I looked for this.* Searched all four sources for "tile", "Save Buffers", "save_buffers", "cached to disk", "to disk", "disk space", "temporary directory", "tempdir", "temp_dir", "temporary_directory", "scratch", "TileManager", "tile_manager". Opened the Cycles Performance page and the EEVEE Performance page in full (EEVEE's has a Memory section, but it is about GPU pools — Shadow Pool and Light Probes Volume Pool — with no disk in it). Read `properties.py` and `ui.py` from `cycles_5.2` around every hit for "tile". Confirmed that `cycles_5.2/source` holds only kernel and utility headers — there is no session or tile-manager code in what I was given, so the file that would name the cache location is not present. Also confirmed that "Save Buffers" appears **nowhere** in any of the four 5.2 sources.

### (G) The other places a running render writes, under the wider reading

Given only because part (F) depends on the distinction. All documented.

- **Texture cache.** > "For every image, a corresponding tx file will be generated in the `blender_tx/` folder next to the image file. This tx file optimized for loading just the tiles and resolutions needed for rendering." Source: release notes — https://developer.blender.org/docs/release_notes/5.2/cycles/ — read 2026-07-30. The manual adds: > "By default, files are stored in the blender_tx/ folder next to the image file. This can be changed with the Texture Cache file path preference." Source: https://docs.blender.org/manual/en/5.2/render/cycles/render_settings/performance.html — read 2026-07-30. These sit beside the *input* images, not in the output folder.
- **Render cache.** > "Saves the rendered view layers and their passes to a multi-layer OpenEXR image. ... The image is stored in the Render Cache folder as specified in the File Paths Preferences." Source: https://docs.blender.org/manual/en/5.2/render/output/properties/output.html — read 2026-07-30. Off by default: > "use_render_cache — Save render cache to EXR files (useful for heavy compositing, Note: affects indirectly rendered scenes) (default False)" — https://docs.blender.org/api/5.2/bpy.types.RenderSettings.html — read 2026-07-30. The manual also says of reading it back: > "And also for recovering some information from a failed render." (https://docs.blender.org/manual/en/5.2/interface/controls/nodes/editing.html, read 2026-07-30), which hints these files exist before a render finishes — but that is a hint, not a statement, and I stop there.

---

### (H) What a running Blender 5.2 showed — hand tests, 2026-08-11

**LABEL: observed.** Every result below was produced on 2026-08-11 by running Blender 5.2.0 LTS on two computers and writing down what it printed and what appeared on disk. The two computers are named in full because that is what settles which operating system each answer covers.

- **The Creator's Mac** — a MacBook Pro with an Apple M1 Pro chip, 16 GB of memory, macOS 26.5.2 (build 25F84), Apple's own graphics and no second card. Blender 5.2.0 LTS at `/Applications/Blender.app/Contents/MacOS/Blender`.
- **A rented Linux machine** — Ubuntu 22.04.5 LTS on kernel 6.8, two NVIDIA RTX 4000 Ada Generation cards of 20 GB each on driver 580.105.08, an AMD Ryzen Threadripper PRO 3945WX with eleven cores, 49 GB of memory. Blender 5.2.0 LTS at `/opt/blender/5.2/blender`.

Every run used `-b --factory-startup` unless it says otherwise, and each was pointed at a settings folder of its own so that no run touched anybody's saved Blender settings. Where the same test ran on both machines, both results are given and it is said plainly whether they agreed. The whole list, with the exact commands, is `Audits/Blender Research/Writing/Hand tests — cross-sweep questions.md`.

**1. Section (A)'s inferred joining-up is right, and section (E) is now settled.** Cycles, 1920 × 1080, 400 samples, frames 1 to 4, PNG, Placeholders off, watched every fifty milliseconds. On the rented Linux machine:

```
   0.00  .../out  []
  11.15  .../out  [('f_0001.png', 0)]
  11.20  .../out  [('f_0001.png', 253952)]
  11.25  .../out  [('f_0001.png', 966656)]
  11.30  .../out  [('f_0001.png', 1600053)]
  21.70  .../out  [('f_0001.png', 1600053), ('f_0002.png', 253952)]
```

At any moment during the render the output folder holds the frames finished so far — and for about a fifth of a second while a frame is being *written* it also holds that frame, part-written and growing. **This is the heart of the question for anyone watching a folder, which section (E) marks not in the sources**: the file is built up in place, with no temporary name and no rename. The Mac agreed. The same animation at 8192 × 4608 stretched that window to about two seconds.

**2. What a placeholder contains and when it appears — the two things section (B) says the sources will not tell you.** Placeholders on, Overwrite off. On the rented Linux machine:

```
   0.00  .../out  []
   0.50  .../out  [('f_0001.png', 0)]
  11.05  .../out  [('f_0001.png', 344064)]
  11.10  .../out  [('f_0001.png', 1081344)]
  11.15  .../out  [('f_0001.png', 1600053), ('f_0002.png', 0)]
```

**Zero bytes**, made half a second in — eleven seconds before that frame finishes — and made **for the frame in hand only**, not for all frames at once. The picture is written into that same file rather than into a second one. Stopping the render part way (frames 1 to 6, the ordinary terminate signal once frame 3's placeholder had appeared) left the placeholder behind at 0 bytes, with Blender exiting 143. The Mac agreed.

**3. Blender holds the file open while it writes it.** During a 10000 × 10000 multi-layer OpenEXR write the system reported Blender holding that file with a write handle. Renaming it at that moment succeeded and the render still finished. Both machines agreed.

**4. Section (C) is right about the video file, and the file left after a stop will not play.** Workbench, 1280 × 720, FFmpeg in a Matroska container with H.264, frames 1 to 4000, stopped after 25 seconds. `movie_0001-4000.mkv` — named for the whole frame range from the first moment — appeared at 0 bytes within a second and grew in steps of 262,144 bytes. Handing the file left behind back to Blender 5.2 gave `loaded as a movie: movie_0001-4000.mkv  frames: 0  size: (256, 256)`. Both machines agreed. **Run twice**: a first run of 200 frames finished before it could be stopped, and showed the finished file opening properly.

**5. How the autosplit files are named — the item under NOT COVERED — and where the split really falls.** Workbench, 1920 × 1080, FFmpeg in an AVI container with the lossless HUFFYUV codec, 1400 frames. **Run on the rented Linux machine only**, because it writes several gigabytes four times over and the Mac is the Creator's working machine; it settles Linux and leaves macOS open. With Autosplit on:

```
FINAL: [('long__0000001-1400.avi', 2003090368), ('long__0010001-1400.avi', 1022075648)]
```

With it off, one file: `FINAL: [('long_0001-1400.avi', 3025156392)]`. So the names gain an extra block of seven digits when the option is on, and the first file stops at 2,003,090,368 bytes — about 1.87 GiB, not 2. **Also run twice**: a first run of 900 frames reached 1,945,065,136 bytes and never split.

**6. Where the cached tiles land — the "not in the sources" in section (F).** Cycles, 32 samples, tile size at its default 2048, one frame at 8000 × 8000, with every file the process had open sampled twice a second. On the rented Linux machine:

```
blender 46901 root 5u REG 253,1    118784 /tmp/blender_qF6zCi/cycles-tile-buffer-46901-134744316668944-0-0.exr
blender 46901 root 5u REG 253,1  84779008 /tmp/blender_qF6zCi/cycles-tile-buffer-46901-134744316668944-0-0.exr
blender 46901 root 5r REG 253,1  89510421 /tmp/blender_qF6zCi/cycles-tile-buffer-46901-134744316668944-0-0.exr
```

On the Mac the same file sat under `/var/folders/…/T/blender_XXXXXX/`. **One file, an OpenEXR, in Blender's own temporary folder, named `cycles-tile-buffer-<process number>-<number>-0-0.exr`, about 89 MB for that frame, and gone before Blender exits.** So the leap this pass refuses to make — that the tile cache and the manual's "render layers" are the same thing — was right to be refused: they share a folder and the tile cache is a file of its own with its own name. The same render at 1000 × 1000 opened no such file at all.

**7. The inferred reading of the tile-area comparison is right.** Tile size 2048, one frame, with the system's temporary folder pointed at an empty folder of the run's own and walked every twenty milliseconds. At **2048 × 2048** nothing appeared. At **2048 × 2049**, on the Mac:

```
   0.00  []
   6.70  [('.../tmp/blender_0dpK4g/cycles-tile-buffer-72877-53456653296-0-0.exr', 4096)]
   6.73  [('.../tmp/blender_0dpK4g/cycles-tile-buffer-72877-53456653296-0-0.exr', 90112)]
  13.64  []
```

Both machines agreed. So the comparison the panel's warning uses is also the comparison the disk cache uses. At 8000 × 8000 with the tile size raised to **8192** — larger in area than the picture — there was no such file; at **256** there was one, written to far more often. Setting `use_auto_tile` to False changed nothing, which matches its own description as deprecated. (In Blender 4.5 that switch does turn it off. That is a difference between versions.)

**8. EEVEE and Workbench put nothing on disk during a render.** The same 8000 × 8000 frame under `BLENDER_EEVEE` and under `BLENDER_WORKBENCH` opened no tile-buffer file and put nothing in the temporary folder, on either machine.

**9. The texture cache in section (G) is real, and it is off out of the box.** A 1024 × 1024 PNG written by Blender itself, used as an image texture on the cube, rendered with Cycles at 4096 × 4096. **At factory settings nothing appeared beside the picture.** Asking Blender 5.2 what settings it has for this printed:

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
texture folder after the render: ['blender_tx', 'smoozytex.png']
  under the texture folder: .../tex/blender_tx/smoozytex.png.0-926BDA8569F1F2504902F06ABA51E46D.tx  57115
```

So the `blender_tx/` folder does sit beside the image file, and the file in it is the image's own name with a long code and `.tx` on the end — but only once `use_auto_generate_texture_cache` is turned on, which it is not by default. Both machines agreed. **Blender 4.5, 5.0 and 5.1 have neither switch at all**, and none of them wrote anything beside the picture. That is a difference between versions.

**10. What is in the output folder while a File Output node render runs — the item under NOT COVERED.** Compositor on, one node aimed at a folder that did not exist, separate from the scene's own; Cycles, 1920 × 1080, frames 1 to 4. `node folder exists before the render: False`, and then, frame by frame:

```
Saved: '.../work/e12/5.2/node/n_Image0001.png'
Saved: '.../work/e12/5.2/out/f_0001.png'
```

The node's folder is made by the render, and its file for a frame lands just before the scene's own file for that frame. Both machines agreed.

**11. The render cache file, and that it is one file rewritten per frame.** Two view layers, Cycles, 1920 × 1080, frames 1 to 3. Blender printed, once per frame, `Caching exr file, 1920x1080, .../cache/cached_RR_UNSAVED_Scene_00000000000000000000000000000000.exr`, and the watcher showed it reaching 132,799,542 bytes before that frame's own PNG appeared, then dropping to zero and growing again. Still there at the end. So the hint section (G) stops short of drawing is right: the cache file does exist before a render finishes. Both machines agreed.

**12. Two Blenders writing into one folder divide the work between them.** **Run on the rented Linux machine only**, and the one test here whose whole subject is two Blenders at once. Placeholders on, Overwrite off, frames 1 to 8, the second started three seconds after the first. One took the odd frames and the other the even ones, each printing `skipping existing frame "<path>"` for the frames the other had claimed. No frame was rendered twice. This was a folder on one machine's own disk; a folder shared over a network was not tested.

**What these tests leave open.** Windows, everywhere — no Windows machine has run it. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.** A folder shared over a network. Whether the two-gigabyte split behaves the same on macOS. And multi-view output, which nothing here tested.

---

**NOT COVERED by answer 11.**

*Most of this list has since been settled by the hand tests in section (H) above; the note after it says which parts stand.*

- What a placeholder file actually contains or how big it is, and at what moment in the render it appears.
- Whether one frame's output file is visible on disk in a half-written state, or appears only once complete. This is the heart of the question for anyone watching a folder, and the sources are silent on it.
- Where Cycles puts its cached tiles, what those files are called, and whether they are cleaned up.
- Whether EEVEE or Workbench put anything on disk during a render. The sources describe tile caching for Cycles only.
- How the autosplit video files are named.
- What is present in the output folder while a File Output node render is running — the node's page describes what it writes, not when.
- Multi-view renders: the manual notes the output Views Format "options include individual files per view, top-bottom, anaglyph among others" (https://docs.blender.org/manual/en/5.2/render/output/properties/stereoscopy/usage.html, read 2026-07-30), so the count of files in the folder can differ, but nothing describes the timing.

**What the hand tests of 2026-08-11 settled from that list, and what is left.** Settled: what a placeholder contains (zero bytes), when it appears (before its frame, one at a time) and what becomes of it after a stop; that one frame's file *is* visible half-written and is built up in place; where the tile cache lives, what it is called and when it goes; that EEVEE and Workbench put nothing on disk; how the autosplit files are named; and what a File Output node's folder holds while a render runs. **Still open**: multi-view output, everything on Windows, a folder shared over a network, and whether the two-gigabyte split behaves the same on macOS.

**Outside the interface.** The sources do not draw a line here, and I found no statement that any of the above behaves differently when Blender is driven from outside its own window. Placeholders, Overwrite, Cache Result and Cycles' Tile Size are all scene render settings — they live on `bpy.types.RenderSettings` and on Cycles' scene properties — and the manual says of a command-line animation render: "Render the whole animation using all the settings saved in the blend-file" (https://docs.blender.org/manual/en/5.2/advanced/command_line/render.html, read 2026-07-30). **Inferred**, with the step written out: from "these are settings saved in the blend-file" plus "a command-line animation render uses all the settings saved in the blend-file", it follows that they apply to a render started from outside. What does *not* follow, and what the sources do not say, is whether the moment-to-moment state of files on disk is the same — for instance whether a background render flushes differently. That is not in the sources.

**Settled by the hand tests of 2026-08-11.** It holds. All twelve tests in section (H) were run with no window at all, so every one of them is established for a Blender driven from outside. One test was then repeated in a Blender **with a window open on screen**, on both machines — a still render with the output path set to an empty folder — and it printed exactly the same thing as with no window: `AFTER render, no save switch: []`, then `AFTER render with the save switch: ['still_.png']`. Nothing was found that a window changes about what lands on disk or when.

**Operating system.** The output-folder behaviour, Placeholders, Overwrite, Autosplit and Cycles tiling are all described without any operating-system qualification, and I found no page saying they differ. The one thing here that *is* per-system is where the temporary directory sits:

> "The temporary directory is selected based on the following priority:
>
> User Preference (see File Paths).
>
> Environment variables (TEMP on Windows, TMP & TMP_DIR on other platforms).
>
> The /tmp/ directory."

Source: Blender manual — https://docs.blender.org/manual/en/5.2/advanced/blender_directory_layout.html — read 2026-07-30. So if cached tiles or render layers go to the temporary directory, the *place* differs by operating system even though the behaviour is described the same. Beyond that, the sources do not say.

**Settled by the hand tests of 2026-08-11.** Tests 1 to 4 and 6 to 11 in section (H) were each run on macOS on an Apple M1 Pro and on Linux on an x86 machine, and the two machines agreed on every one of them, so none of this is a macOS-only or Linux-only behaviour. **Cached tiles do go to the temporary directory**, so the *place* does differ by system exactly as the quote above predicts: the same `cycles-tile-buffer-…-0-0.exr` file sat under `/var/folders/…/T/` on macOS and under `/tmp/` on Linux. Nothing about **Windows** was tested — no Windows machine has run it — so every claim here leaves Windows open. Tests 5 and 12 were run on Linux only and leave macOS open; the reason is written into each.

---

## 12. What does a render do when it must make a folder and cannot?

**LABEL: not in the sources.**

I could not find, in the Blender 5.2 manual, the Blender 5.2 Python API documentation, the Blender 5.2 release notes, or Cycles' own files from Blender 5.2, any statement of what happens when a render needs a folder that is not there and cannot create it. Not an error message, not a report, not a stop-or-continue, not an exit status, not a difference between a render started by a person and one started from outside.

### What the sources do say, in the neighbourhood

**One place, and only one, states that a render makes folders at all. Label: documented.**

> "When the Media Type is set to Image, input names are also used to construct subdirectories:
>
> If an input name contains a slash character (/ or \), the path is interpreted as a directory hierarchy.
>
> Any missing directories are created automatically during output.
>
> This allows organizing outputs into folders directly from the node setup, for example: passes/diffuse or masks/object_01."

Source: Blender manual — https://docs.blender.org/manual/en/5.2/compositing/types/output/file_output.html — read 2026-07-30.

That is the File Output node, and it is about the subdirectories implied by an input socket's name. It says the folders are made. It says nothing about what happens if making one fails.

**For the main render output path, the sources do not even say Blender makes the folder.** The Output panel page describes the File Path field and the relative-path prefix; the command-line page describes `-o`; neither says a missing folder is created, and neither says it is not. So the question's premise — that a render "must make a folder" — is only documented for the File Output node's socket-name subdirectories.

**The nearest documented thing to "a folder that is not there" is a fallback, not a failure. Label: documented.**

> "Temporary Files
>
> The directory for storing temporary save files. The path must reference an existing directory or it will be ignored and the systems temporary directory will be used instead. When left blank, the systems temporary directory will be used (see Temporary Directory for details)."

Source: Blender manual — https://docs.blender.org/manual/en/5.2/editors/preferences/file_paths.html — read 2026-07-30.

This is worth having because it shows the pattern Blender uses in the one directory-missing case it does document: it does **not** create the folder, it ignores the setting and falls back. That is about the temporary-files preference, not about a render's output folder, and reading it across to renders would be me speaking, not the source.

**A different render failure that *is* documented, so that it is not mistaken for this one. Label: documented.**

> "Depending on the path, template errors may prevent certain actions. For example, if there are errors in the render output path, then rendering an animation will fail with an error message indicating the path errors."

Source: Blender manual — https://docs.blender.org/manual/en/5.2/files/file_paths.html — read 2026-07-30.

That is a malformed `{…}` template expression in the path, caught before anything is written. It is not a folder that could not be made.

**And one more nearby, also not this. Label: documented.**

> "If you type a nonexistent path, you will be prompted to create it."

Source: Blender manual — https://docs.blender.org/manual/en/5.2/editors/file_browser.html — read 2026-07-30. That is the File Browser and a person clicking, not a render.

### Where I looked, and what I ruled out

Searched the whole of `manual_5.2`, `api_5.2`, `release_notes_5.2` and `cycles_5.2` for each of these, in both plain and industry wording:

*Making a folder* — "mkdir", "makedirs", "create director", "create the directory", "creates the director", "creating director", "create a folder", "create folder", "create any missing", "missing director", "created automatically", "will be created", "is created", "creates missing", "folder will", "subdirector", "directory".

*Failing to* — "unable to create", "cannot create", "Cannot save", "cannot write", "not writable", "writable", "could not", "unable to", "failed to", "Permission denied", "permission", "write permission", "read-only", "read only", "Cannot open", "Unable to open", "No such file", "does not exist", "non-existent", "nonexistent".

*What a failure would look like* — "error", "exit code", "return code", "exit status", "nonzero", "non-zero", "quits", "aborts", "disk full", "out of disk", "no space".

Pages opened and read in full, to be sure the answer was not sitting in prose that my search terms would miss: the Output properties page, the Rendering Animations page, the render Output introduction page, the File Output node page, the Path Templates page, the File Paths preferences page, the Blender directory layout page, the command-line Rendering page, and the render section of the command-line Arguments page.

Ruled out and why: the `troubleshooting/` tree (its pages are startup, crash, recovery, Python and graphics-driver problems — I listed every file in it and none concerns writing output); the `contribute/` tree (instructions for writing the manual, not about Blender's behaviour); the import/export pages (about importers and exporters, not renders); Cycles' `source/` folder (it holds only kernel and utility headers — the code that opens output files is not among the files I was given, so it could not answer this either way).

**NOT COVERED by answer 12.**

- Whether the main render output folder is created by Blender at all. The sources are silent, so I cannot say either that it is or that it is not.
- What a File Output node does when a subdirectory it must create cannot be created.
- Whether such a failure stops the whole render, skips the frame, or is only reported.
- Where any such report appears — the status bar, the Info editor, the system console, or the process's exit status.
- Whether a partly-written frame or an empty placeholder is left behind after such a failure.
- Whether the cause of the failure (folder missing versus folder not writable versus disk full) changes what happens.

*Every one of those but the last has since been settled by hand tests, immediately below.*

---

### What a running Blender 5.2 showed — hand tests, 2026-08-11

**LABEL: observed.** Run on 2026-08-11 on the two computers described in section (H) above, with Blender 5.2.0 LTS, `-b --factory-startup`, each run pointed at a settings folder of its own. On the Linux machine the runs were made by an ordinary account rather than root, because root is not stopped by an unwritable folder. Every scene was Workbench at 320 × 240, frames 1 to 3, PNG. The exact commands are in `Audits/Blender Research/Writing/Hand tests — cross-sweep questions.md`.

**13. Blender does create the main render output folder — one level or a whole chain.** With the output path set to `<a writable folder>/newfolder/f_`:

```
operator returned: {'FINISHED'}
does the folder exist now: True
folder holds: ['f_0001.png', 'f_0002.png', 'f_0003.png']
```

With three missing levels at once, `<a writable folder>/a/b/c/f_`, the same. Exit code 0. Both machines agreed. **So the first item under NOT COVERED is answered: it does, and it makes the whole chain.**

**14. When the folder cannot be made, the render stops at the very first frame, says so in two lines, and ends with 1.** The parent folder was made and then set to read-and-enter only, and the output path named a folder inside it that did not exist. On the **Mac**:

```
does the folder exist before: False
00:00.525  image            | ERROR Couldn't create directory for file .../work/e18b/5.2/locked/newfolder/f_0001.png: Permission denied
Error: Render error (Permission denied) cannot save: '.../work/e18b/5.2/locked/newfolder/f_0001.png'
operator raised: RuntimeError Error: Render error (Permission denied) cannot save: '.../work/e18b/5.2/locked/newfolder/f_0001.png'
does the folder exist now: False
```

with `exit code: 1`. Frames 2 and 3 were never attempted, and **nothing was left behind** — no part-written frame, no placeholder, the locked folder still empty. Three missing levels gave the same. The **rented Linux machine** printed the identical two lines with its own paths and behaved identically. **That answers three more items at once: it stops the whole render rather than skipping the frame; the report appears on the console and as a Python `RuntimeError`; and the exit status is 1.**

**This is the plain answer to the paragraph under *Outside the interface* below.** There is a message and there is an exit status, and both are now on record.

**Blender 5.2 names the folder in the message and gives the real reason. Blender 4.5 does neither** — for the same set-up it prints only `Error: Render error (No such file or directory) cannot save: …` on macOS and `(Invalid argument)` on Linux, with no mention of a folder. That is a difference between versions.

**15. The cause does change the wording.** An ordinary file standing where the folder must go gave `ERROR Couldn't create directory for file …: File exists` and `Error: Render error (File exists) cannot save: …`, with exit code 1 and the render stopped at the first frame. Both machines agreed. So of the three causes the last item under NOT COVERED names, two were tested and they differ in the reason printed while behaving the same way. Running out of room was not tested here; it was settled for this topic in an earlier round, under question 5.

**16. A File Output node that cannot make its folder does *not* stop the render, and the exit code stays 0.** A File Output node aimed at a folder inside the read-only parent, the scene's own output path pointing somewhere that worked:

```
ERROR Couldn't create directory for file .../work/e20b/5.2/locked/nodeout/n_Image0001.png: Permission denied
ERROR Couldn't create directory for file .../work/e20b/5.2/locked/nodeout/n_Image0002.png: Permission denied
ERROR Couldn't create directory for file .../work/e20b/5.2/locked/nodeout/n_Image0003.png: Permission denied
operator returned: {'FINISHED'}
```

with `exit code: 0`, and all three of the scene's own frames written. Both machines agreed. **So only that one output is lost, and nothing outside Blender can tell from the exit code that anything went wrong.**

**17. The subfolder that an input's name asks for — the case the manual's sentence describes — is made, and behaves the same way when it cannot be.** The node's input was named `passes/diffuse`. Into a writable folder, Blender 5.2 made a real folder, taking the node's own file name onto the front of it:

```
.../nodeopen/n_passes/diffuse0001.png
.../nodeopen/n_passes/diffuse0002.png
.../nodeopen/n_passes/diffuse0003.png
```

Into a folder that could not be written to, three `ERROR Couldn't create directory for file …: Permission denied` lines and the same `operator returned: {'FINISHED'}` with exit code 0. Both machines agreed. (In Blender 5.0 the slash becomes an underscore and no folder is made at all. That is a difference between versions.)

**18. The render cache folder is made when missing, and when it cannot be made the render carries on and finishes.** With Cache Result on and the render cache folder pointed at a folder that did not exist under a writable parent, the folder and the cache file were both made. Pointed inside the read-only parent, the render wrote all three of its frames and printed, per frame — on the **Mac**:

```
00:48.907  image.openexr    | ERROR Unknown error in IMB_exr_begin_write
00:48.908  reports          | ERROR Error writing render result, No such file or directory (see console)
```

and on the **rented Linux machine**:

```
00:42.290  image.openexr    | ERROR IMB_exr_begin_write: No such file or directory.
00:42.290  reports          | ERROR Error writing render result, No such file or directory (see console)
```

`operator returned: {'FINISHED'}`, exit code 0 on both, and the cache folder was never made. The two machines agree on the behaviour and differ on the first line: the Mac loses the reason to "Unknown error" where Linux names it.

**19. With a window open on screen, the terminal says exactly the same thing.** The set-up of test 14 was run again in a Blender with a real window, on both machines, and printed the same two lines and raised the same error to Python. Blender was then asked to take a picture of its own window. **What the status bar and the Info list show could not be settled**: the picture shows Blender's first-run Quick Setup panel sitting over the middle of the window and an empty Info list behind it, and dismissing that panel needs a click, which nothing in this round does. That half of the test is dropped for want of a hand on a mouse. The terminal half is complete and is the answer above.

---

### The two disputed wordings, run again — hand tests, 2026-08-12

**LABEL: observed.** Entries **25** and **26** of `Audits/Blender Research/Writing/Hand tests — cross-sweep questions.md` were run on 2026-08-12. Neither was written for Blender 5.2 — entry 25 was written for a disagreement in Blender 5.0 and entry 26 for one in Blender 5.1 — but both name 5.2 as worth running alongside, because it is the differences between the four versions that are the answer. Both were run on **both** computers:

- **The Creator's Mac** — a MacBook Pro with an Apple M1 Pro chip, 16 GB of memory, macOS 26.5.2, Apple's own graphics and no second card.
- **A rented Linux machine** — Ubuntu 22.04.5 LTS on kernel 6.8, two NVIDIA RTX 4000 Ada Generation cards of 20 GB each on driver 580.105.08, an AMD Ryzen Threadripper PRO 3945WX with eleven cores allotted, 49 GB of memory.

Blender 5.2.0 LTS on both, started with `-b --factory-startup`, each run pointed at a settings folder of its own that was made before Blender was started. On the Linux machine the runs were made by an ordinary account called `smoozy`, not root. Only one Blender ran at a time on each machine. Blender's whole terminal output was kept, unfiltered. **The words are written down per machine and the two machines are not merged into one answer.** In the blocks below only the leading part of each long path is shortened to `...`; every word of every message is as the machine printed it.

**20. (Entry 25.) Test 17's failing run again — in Blender 5.2 the two machines agree.** The File Output node's own base folder was made first and then set to read-and-enter only, so it exists and cannot be written to. The node's one input was named `passes/diffuse`. The scene's own output path pointed at a folder that works. Workbench, 320 × 240, frames 1 to 3, PNG.

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

**The two machines agreed, word for word.** This matches what test 17 above records for Blender 5.2. **Blender 5.0 does not agree with itself across the two machines on this test** — the Mac says `ERROR Render error (Undefined error: 0) cannot save: '…'` and Linux says `ERROR Render error (Success) cannot save: '…'` — so this is a difference between versions, and 5.2 is on the settled side of it.

**21. (Entry 26.) Test 18's failing run again — the two machines print different words, as test 18 already records.** Cache Result on, the render cache folder pointed at a folder that does not exist under a parent set to read-and-enter only. Entry 26 was run twice per version, because its own written set-up says Workbench at 320 × 240 while the run it points back at, test 18, was Cycles at 1920 × 1080 with 400 samples and two view layers. Neither was preferred: run A repeated test 18's own set-up and run B used the set-up entry 26 itself is written with. **The added step was the second run; nothing else was changed.** Both runs gave the same words on each machine.

On the **Creator's Mac**, run B (Workbench, 320 × 240) printed:

```
VERSION 5.2.0 LTS
use_render_cache = True
render_cache_directory = '.../work/e26b/5.2/locked/cachefolder'
view layers: ['ViewLayer', 'SecondLayer']
cache folder exists before the render: False
00:00.545  image.openexr    | ERROR Unknown error in IMB_exr_begin_write
00:00.545  reports          | ERROR Error writing render result, No such file or directory (see console)
Blender 5.2.0 LTS (hash fbe6228777e7 built 2026-07-14 01:31:22)
Caching exr file, 320x240, .../work/e26b/5.2/locked/cachefolder/cached_RR_UNSAVED_Scene_00000000000000000000000000000000.exr
00:00.617  render           | Saved: '.../work/e26b/5.2/out/f_0001.png'
```

— the same lines again for frames 2 and 3 — and ended:

```
operator returned: {'FINISHED'}
SCENE FOLDER FINAL: ['f_0001.png', 'f_0002.png', 'f_0003.png']
CACHE FOLDER FINAL: not there
```

with `exit code: 0`. Run A on the Mac (Cycles, 1920 × 1080) printed the identical two error lines.

On the **rented Linux machine**, run B printed:

```
00:00.768  image.openexr    | ERROR IMB_exr_begin_write: No such file or directory.
00:00.768  reports          | ERROR Error writing render result, No such file or directory (see console)
```

and run A the same two lines, with the same three closing lines and `exit code: 0`.

**The two machines did not print the same words**, and this agrees with what test 18 above already records for Blender 5.2: the Mac loses the reason to "Unknown error" where Linux names it. Everything else matched — all three frames written, the cache folder never made, `operator returned: {'FINISHED'}`, exit code 0.

Laid out across the four versions, the first of the two lines reads:

| Version | The Creator's Mac | The rented Linux machine |
|---|---|---|
| 4.5 | `IMB_exr_begin_write: ERROR: No such file or directory.` | `IMB_exr_begin_write: ERROR: No such file or directory.` |
| 5.0 | `ERROR IMB_exr_begin_write: No such file or directory.` | `ERROR IMB_exr_begin_write: No such file or directory.` |
| 5.1 | `ERROR Unknown error in IMB_exr_begin_write` | `ERROR IMB_exr_begin_write: No such file or directory.` |
| 5.2 | `ERROR Unknown error in IMB_exr_begin_write` | `ERROR IMB_exr_begin_write: No such file or directory.` |

The second line was `Error writing render result, No such file or directory (see console)` in every version on both machines. So the split between the two systems begins at 5.1 and is not present in 4.5 or 5.0 — and Blender 5.1's own document, which recorded the two machines as agreeing, has that disagreement written into it as unresolved.

---

**Outside the interface.** The sources do not say. Since they do not describe the behaviour inside the interface either, there is nothing to compare. I want to state plainly what this means for anyone relying on it: **there is no documented answer for what a `blender -b` render does when it cannot make a folder — not the message, not the exit status.** That has to be settled by hand; see the list below.

**Settled by the hand tests of 2026-08-11.** It is settled now, and the answer is in tests 13 to 19 above. Every one of them was run with `-b` and no window. The message is `ERROR Couldn't create directory for file <path>: <reason>` followed by `Error: Render error (<reason>) cannot save: '<path>'`; the exit status is **1** when it is the scene's own output folder and **0** when it is only a File Output node's or the render cache's. Test 19 repeated the failure in a Blender with a window and the terminal said the same thing.

**Still holding after the hand tests of 2026-08-12.** Tests 20 and 21 were also run with `-b` and no window, on both machines, and both ended with exit status 0 while printing their failure on the terminal. Nothing changed.

**Operating system.** The sources do not say. Nothing I found about folder creation or its failure is qualified by operating system, and nothing describes it in the first place, so I have no basis for saying it holds everywhere or that it does not.

**Settled by the hand tests of 2026-08-11.** The Creator's Mac and the rented Linux machine agreed on every one of tests 13 to 19 in what the render does, and on the words printed everywhere except the render cache failure in test 18, where the Mac says `Unknown error in IMB_exr_begin_write` and Linux names the reason. Nothing about **Windows** was tested — no Windows machine has run it — so Windows stays open on all of it.

**Confirmed by the hand tests of 2026-08-12.** Both machines were run again on the two places most in doubt, and the picture above is unchanged for Blender 5.2. The render cache failure (test 21) is still the one place where the two systems print different words, and the File Output node failure with an unwritable base folder (test 20) is not — both machines said `ERROR Render error (Permission denied) cannot save: '…'`. What is new is where 5.2 sits among the versions: on the render cache line it matches Blender 5.1 and differs from 4.5 and 5.0, both of which print the same words on both machines. Windows stays open, for the same reason as before.

---

## Things a person with Blender 5.2 open could settle by hand in a few minutes

**All ten items of the original list were run on 2026-08-11, on the Creator's Mac and on a rented Linux machine, and their results are written into questions 11 and 12 above.** What is left is only what those two machines could not reach.

1. **Anything on Windows** — an output folder that cannot be made, a folder whose permissions deny writes, and the exit code (`echo %ERRORLEVEL%`). Items 3, 7 and 8 of the original list asked for it. **Not run with the equipment used**: the runs used one Mac and one rented Linux machine, and no Windows machine was rented for them. Every question-11 result above leaves Windows open too.
2. **A folder shared over a network between two machines.** Test 12 in section (H) ran two Blenders into one folder on a single machine's own disk. The manual's render-farm recipe is about a shared network folder, which is not the same thing, and no network share between two machines is among this project's written-down equipment. It needs the Creator's say-so first.
3. **Whether the two-gigabyte video split behaves the same on macOS.** Test 5 in section (H) was run on the rented Linux machine only, because it writes several gigabytes four times over and the Mac is the Creator's working machine.
4. **What Blender's own window shows when a render cannot make its folder** — the status bar and the Info list, word for word. The terminal half is settled (test 19), but the picture Blender took of its own window has the first-run Quick Setup panel over it, and dismissing that needs a click. Anybody with a hand on a mouse can settle it in a minute: set the output path into a folder you cannot write to, press Ctrl-F12, and read the status bar and the Info editor.
5. **Multi-view output during a render.** Nothing here tested it.

---

2026-08-11 — Claude Opus 5 (Claude Code) — Writing, Blender 5.2

2026-08-11 — Claude Opus 5 (Claude Code) — hand tests: nineteen observed results written into questions 11 and 12, both standing lines brought up to date on both answers, and the researcher's own list pruned to what two machines cannot reach

2026-08-12 — Claude Opus 5 (Claude Code) — second batch of hand tests: entries 25 and 26 run on both machines and written into question 12 as tests 20 and 21, and both standing lines brought up to date. Nothing on the list above closed or opened; it is unchanged.

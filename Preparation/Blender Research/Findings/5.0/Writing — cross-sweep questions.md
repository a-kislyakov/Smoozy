# Writing — cross-sweep questions (Blender 5.0)

Sources consulted, all read on 2026-07-30 (the snapshot date): Blender's manual for 5.0 (`Blender Sources/manual_5.0`, including one picture from its `_images` folder), Blender's Python API documentation for 5.0 (`Blender Sources/api_5.0`), the 5.0 release notes (`Blender Sources/release_notes_5.0`), and Cycles' own files (`Blender Sources/cycles_5.0`).

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

Questions answered here: 11 and 12.

---

## 11. While a render is still working, what is present in the places it writes to, and how does that change with the size of the picture?

**Reading taken.** "The size of the picture" is ambiguous. I read it as the resolution of the image being rendered — how many pixels wide and tall. The other reading is the size in bytes of the file being produced. The first reading is answered here, and saying so is better than choosing silently. The second reading is not answered, and no search for it is recorded, so it stands as a hole rather than as silence in the sources.

The question also covers more than one place, so the answer is split by place. Each part carries its own label.

### 11a. The render output path, when an animation is being rendered

**Label: documented.**

Frames that have finished are already on disk, in the output folder, while the rest of the animation is still being rendered.

Exact words:

> "While rendering stills will allow you to view and save the image from the render buffer when it is complete, animations are a series of images, or frames, and are automatically saved directly out to a drive after being rendered."

> "This allows you an easy recovery if there is a problem and you have to re-start the rendering, since the frames you have already rendered will still be in the output directory."

Source: the manual, https://docs.blender.org/manual/en/5.0/render/output/animation.html. Read 2026-07-30.

And:

> "and the files saved directly to disk with the Render Animation operator."

Source: the manual, https://docs.blender.org/manual/en/5.0/render/color_management/displays_views.html. Read 2026-07-30.

### 11b. The render output path, when a single still is being rendered from inside Blender

**Label: documented** for what the quote says, with one **inferred** step marked below.

The picture sits in the render buffer when it is complete, and the person views and saves it from there.

Exact words:

> "While rendering stills will allow you to view and save the image from the render buffer when it is complete"

Source: the manual, https://docs.blender.org/manual/en/5.0/render/output/animation.html. Read 2026-07-30.

**Inferred, with the step written out:** nothing is written to the output path by the render itself. The step: the quoted words say the person may view and save the picture from the buffer, and the rest of the same sentence sets that against animations, which "are automatically saved directly out to a drive after being rendered". A reader has to take that contrast to mean the still is not written anywhere by the render. The sources nowhere say outright that nothing is written, so this is inference, not a documented fact.

### 11c. Empty placeholder files

**Label: documented**, with one **inferred** step marked below.

There is a setting that makes Blender put empty files in the output folder while frames are being rendered.

Exact words, from the manual:

> "Create empty placeholder frames while rendering."

Source: the manual, https://docs.blender.org/manual/en/5.0/render/output/properties/output.html. Read 2026-07-30.

Exact words, from the API documentation, for the same setting:

> "use_placeholder
>
> Create empty placeholder files while rendering frames (similar to Unix 'touch')
>
> Type:
>
> boolean, default False"

Source: the API documentation, https://docs.blender.org/api/5.0/bpy.types.RenderSettings.html. Read 2026-07-30. Note the default: this is off unless someone turns it on.

The manual's picture of the Output panel confirms the same: a checkbox labelled "Placeholders", unchecked, sitting beside a checked "Overwrite". Picture: `_images/render_output_properties_output_panel.png`, shown on https://docs.blender.org/manual/en/5.0/render/output/properties/output.html. Read 2026-07-30. **Label for the picture: documented.**

The manual also gives the reason the setting exists:

> "Primitive Render Farm
>
> An easy way to get multiple machines to share the rendering workload is to:
>
> Set up a shared directory over a network file system.
>
> Disable Overwrite, enable Placeholders in the Render Output panel.
>
> Start as many machines as you wish rendering to that directory."

Source: the manual, https://docs.blender.org/manual/en/5.0/render/output/properties/output.html. Read 2026-07-30.

**Inferred, with the step written out:** the empty file for a frame must appear *before* that frame's picture has been rendered, not after. The step: the quoted recipe only works if a second machine can see that a frame is already being handled and skip it; if the empty file only appeared once the frame was finished, two machines would both start the same frame. The sources do not say this outright, so it is inference, not a documented fact.

So, carrying that inferred step: with Placeholders on, a frame that is currently being rendered would be present in the output folder as an empty file. That the empty file is then replaced by the real picture is a further step the sources do not state either.

### 11d. Video output

**Label: inferred**, from two documented statements.

Documented words:

> "If a problem occurs while rendering, the file might become unplayable and you will have to re-render all frames from the beginning."

> "If your video is huge and exceeds 2GiB, enable Autosplit Output. This will automatically split the output into multiple files after the first file is 2GiB in size."

Source, both: the manual, https://docs.blender.org/manual/en/5.0/render/output/properties/output.html. Read 2026-07-30.

**The step:** for a file to "become unplayable" when a render is interrupted, and for a split to happen "after the first file is 2GiB in size", the video file must already exist on disk and be growing while the render runs. The sources do not state that in so many words. So during a video render, the output place holds one incomplete, growing, possibly unplayable file — inferred, not documented.

### 11e. Files written by File Output nodes in the compositor

**Label: documented.**

> "The File Output node writes one or more images to disk during rendering or compositing."

Source: the manual, https://docs.blender.org/manual/en/5.0/compositing/types/output/file_output.html. Read 2026-07-30.

The sources do not say at what point during a render each such file appears, nor whether a partly written one is ever visible.

### 11f. The render cache folder

**Label: documented** for what it is, **inferred** for when it appears.

Documented words:

> "Saving – Cache Result
>
> Saves the rendered view layers and their passes to a multi-layer OpenEXR image. The Compositor can then use this file to improve performance, especially for heavy compositing.
>
> The image is stored in the Render Cache folder as specified in the File Paths Preferences."

Source: the manual, https://docs.blender.org/manual/en/5.0/render/output/properties/output.html. Read 2026-07-30.

The API documentation gives its default:

> "use_render_cache
>
> Save render cache to EXR files (useful for heavy compositing, Note: affects indirectly rendered scenes)
>
> Type:
>
> boolean, default False"

Source: the API documentation, https://docs.blender.org/api/5.0/bpy.types.RenderSettings.html. Read 2026-07-30. So this is off unless someone turns it on.

Where that folder is:

> "Render Cache
>
> The location where cached render images are stored."

Source: the manual, https://docs.blender.org/manual/en/5.0/editors/preferences/file_paths.html. Read 2026-07-30.

**Inferred, with the step written out:** the cache file is on disk before the render as a whole has finished. The step: the manual says of the operator that reads these files, "Reads all the current scene's render layers from cache, as needed. This can be used to save RAM while rendering because the render layers do not have to be saved in RAM. And also for recovering some information from a failed render. For this to work, Cache Result must be enabled." (Source: the manual, https://docs.blender.org/manual/en/5.0/interface/controls/nodes/editing.html. Read 2026-07-30.) Something can only be recovered from a *failed* render if it was written before the render finished. The sources do not say this directly.

### 11g. The temporary directory

**Label: documented.**

> "The temporary directory is used to store various files at run-time (including render layers, physics cache, copy-paste buffer and crash logs)."

Source: the manual, https://docs.blender.org/manual/en/5.0/advanced/blender_directory_layout.html. Read 2026-07-30.

The same page gives how that directory is chosen; it is quoted under the operating system line below.

### 11h. How this changes with the size of the picture

**Label: documented** for the fact, **observed** for the exact settings, **inferred** for the threshold.

Documented words, in the Cycles Performance panel:

> "Tile Size
>
> Render high resolution images in tiles of this size, to reduce memory usage. Tiles are cached to disk while rendering to save memory."

Source: the manual, https://docs.blender.org/manual/en/5.0/render/cycles/render_settings/performance.html. Read 2026-07-30.

So: the bigger the picture, the more it is rendered in pieces, and those pieces are cached to disk while the render is still working. That is the one place in the sources where what is on disk during a render depends on the size of the picture.

The release notes state a change in this version:

> "* Remove Use Tiling option, it is now always assumed to be on. The tile size can be increased to disabled tiling. (blender/blender!146031)"

Source: the release notes, https://developer.blender.org/docs/release_notes/5.0/cycles/. Read 2026-07-30. (Quoted exactly, including the typo in the original.)

**Observed**, from Cycles' own files:

> ```
> use_auto_tile: BoolProperty(
>     name="Auto Tile",
>     description="Deprecated, tiling is always enabled",
>     default=True,
> )
> tile_size: IntProperty(
>     name="Tile Size",
>     default=2048,
>     description="Render high resolution images in tiles of this size, to reduce memory usage. Tiles are cached to disk while rendering to save memory",
>     min=8,
>     max=8192,
> )
> ```

File: `Blender Sources/cycles_5.0/properties.py`, lines 1051–1062. Read 2026-07-30. So the tile size is a whole number, its default is 2048, its smallest allowed value is 8 and its largest is 8192; and the older switch that turned tiling on and off is marked as no longer meaningful.

**Observed**, the matching versioning step for files made before this change:

> ```
> # Auto tiling is always enabled now
> if version <= (5, 0, 77):
>     cscene = scene.cycles
>     if not cscene.use_auto_tile:
>         cscene.use_auto_tile = True
>         cscene.tile_size = 8192
> ```

File: `Blender Sources/cycles_5.0/version_update.py`, lines 114–119. Read 2026-07-30. The file holds a step for scenes saved before this change: where the old switch was off, it is turned on and the tile size is set to its largest value. That is what the file says. That this step runs when such a file is opened is one step beyond reading it, and the sources do not say so.

**Observed**, the calculation Cycles' own interface uses to work out whether a render will actually be split into tiles:

> ```
> # Calculation matches TileManager::compute_render_tile_size and
> # Session::get_effective_tile_size
> if cscene.tile_size < 128:
>     tile_size = cscene.tile_size
> else:
>     tile_size = (cscene.tile_size + 128 - 1) & ~(128 - 1)
>     tile_size = min(tile_size, 8192)
> tile_area = tile_size ** 2
>
> render_scale = scene.render.resolution_percentage / 100.0
> render_size_x = int(scene.render.resolution_x * render_scale)
> render_size_y = int(scene.render.resolution_y * render_scale)
> render_area = render_size_x * render_size_y
>
> if render_area > tile_area and render_size_x <= 8192 and render_size_y <= 8192:
>     layout.label(text="May work poorly with render tiling", icon='INFO')
> ```

File: `Blender Sources/cycles_5.0/ui.py`, lines 390–405. Read 2026-07-30.

**Inferred, with the step written out:** a render is split into tiles — and therefore caches tiles to disk — when the picture's area in pixels is larger than the tile's area, where the tile's area comes from the tile size rounded up to a multiple of 128 (unless the tile size is below 128, in which case it is used as-is) and capped at 8192. The step: the quoted lines are the interface working out whether tiling will happen, so that it can warn about it, and the comment in them says the calculation matches the parts of Cycles that actually decide the tile size. Reading a warning's own test as the real rule is one step beyond what the file states. Note also that at the default settings a picture must be larger than 2048 by 2048 in area — over four million pixels — before this applies, and that a picture wider or taller than 8192 falls outside the condition quoted, which the sources do not explain.

**Not in the sources, with the record of where I looked:** the sources never say *where* the cached tiles are written, what those files are called, what format they are in, whether they are one file or many, or whether they are removed when the render finishes or is cancelled. I searched the whole manual and API documentation for "cached to disk" (one hit, the sentence quoted above), "tile cache", "Tile Size", "Auto Tile", "auto_tile", "Save Buffers", "save_buffers", "scratch", "swap file", "spool", and searched the Cycles files for "tile" and "temp". I read the Cycles Performance page, the Temporary Directory section of the directory layout page, and the File Paths preferences page in full. Nothing names the location or the file. The nearest thing is the temporary directory sentence quoted in 11g, which lists "render layers" among the run-time files kept there; connecting that phrase to Cycles' tile cache would be a guess, and I do not make it.

### 11i. What the sources do not describe at all

**Label: not in the sources, with the record of where I looked.**

Nothing in the manual, the API documentation, the release notes or the Cycles files says whether a still image file at the output path is written all at once when the frame finishes or built up on disk as the render proceeds; nothing says whether a half-written picture file is ever visible to another program; and nothing says anything about EEVEE writing to disk during a render. Searches run over the whole manual and API documentation: "while rendering", "during rendering", "during the render", "as it renders", "partially written", "incomplete file", "partial file", "half-written", "incrementally", "progressively", "as each frame", "each frame is saved", "saved after each", "flush", and "disk" restricted to the EEVEE, Cycles and Output sections of the manual. Pages read in full: Rendering Animations, Output properties, Output Introduction, Cycles Performance, the File Output node, Blender's Directory Layout, and the File Paths preferences.

### Not covered by answer 11

This answer does not settle: whether a still rendered inside the interface writes anything at all to the output path; how long a placeholder file stays empty; whether the frame currently being rendered is ever visible as a partly written picture; where the tile cache files live or what they are named; what is left behind if a render is cancelled part way through a frame; anything about EEVEE; and the second reading of the question (the size in bytes of the output), for which no search is recorded, so the sources cannot be called silent on it.

*Every item on that list has since been settled by hand tests, immediately below. What is left is named at the end of that section.*

---

### What a running Blender 5.0 showed — hand tests, 2026-08-11

**LABEL: observed.** Every result below was produced on 2026-08-11 by running Blender 5.0.1 on two computers and writing down what it printed and what appeared on disk. The two computers are named in full because that is what settles which operating system each answer covers.

- **The Creator's Mac** — a MacBook Pro with an Apple M1 Pro chip, 16 GB of memory, macOS 26.5.2 (build 25F84), Apple's own graphics and no second card. Blender 5.0.1 at `/Applications/Blender 5.0.1.app/Contents/MacOS/Blender`.
- **A rented Linux machine** — Ubuntu 22.04.5 LTS on kernel 6.8, two NVIDIA RTX 4000 Ada Generation cards of 20 GB each on driver 580.105.08, an AMD Ryzen Threadripper PRO 3945WX with eleven cores, 49 GB of memory. Blender 5.0.1 at `/opt/blender/5.0/blender`.

Every run used `-b --factory-startup` unless it says otherwise, and each was pointed at a settings folder of its own so that no run touched anybody's saved Blender settings. Where the same test ran on both machines, both results are given and it is said plainly whether they agreed. The whole list of tests, with the exact commands, is `Audits/Blender Research/Writing/Hand tests — cross-sweep questions.md`.

**1. A still render puts nothing at the output path unless it is asked to — and that is as true with a window as without.** Output path set to an empty folder, PNG, 320 × 240, Cycles. Running `bpy.ops.render.render(animation=False)` and then the same with `write_still=True`, listing the folder after each, printed:

```
BEFORE: []
AFTER render, no save switch: []
AFTER render with the save switch: ['still_.png']
```

`blender -b --factory-startup -o <folder>/cli_ -F PNG -f 1` wrote `cli_0001.png`. The three runs were then repeated in a Blender **with a window open on screen**, which printed the same three lines. Both machines agreed, and the window made no difference. **This settles section 11b, which the answer above could only infer.**

**2. While a frame is being rendered the output folder holds only the frames already finished — but while a frame is being *saved*, its file is on disk part-written.** Cycles, 1920 × 1080, 400 samples, frames 1 to 4, PNG, Overwrite on, Placeholders off, with a second program listing the folder every fifty milliseconds. On the rented Linux machine:

```
   0.00  .../out  []
   9.96  .../out  [('f_0001.png', 180224)]
  10.01  .../out  [('f_0001.png', 892928)]
  10.06  .../out  [('f_0001.png', 1598467)]
  19.55  .../out  [('f_0001.png', 1598467), ('f_0002.png', 253952)]
  19.60  .../out  [('f_0001.png', 1598467), ('f_0002.png', 950272)]
  19.66  .../out  [('f_0001.png', 1598467), ('f_0002.png', 1458176)]
  19.71  .../out  [('f_0001.png', 1598467), ('f_0002.png', 1598467)]
```

Blender's own line was `Saved: '.../f_0001.png'` at ten seconds. So nothing at all stands for a frame for the ten seconds it is being rendered, and then the file appears and grows to full size inside about a fifth of a second. The Mac agreed.

**3. With Placeholders on, an empty file for a frame appears before that frame is rendered, and is then filled in place.** Same scene, Placeholders on and Overwrite off. On the rented Linux machine:

```
   0.00  .../out  []
   0.30  .../out  [('f_0001.png', 0)]
   9.85  .../out  [('f_0001.png', 516096)]
   9.90  .../out  [('f_0001.png', 1220608)]
   9.95  .../out  [('f_0001.png', 1598467), ('f_0002.png', 0)]
```

The empty file for frame 1 is there three tenths of a second in, ten seconds before that frame is finished; the empty file for frame 2 appears the moment frame 1's picture is complete. So they are made one at a time, for the frame in hand, and the picture is written into the same file rather than into a second one. **This settles the inferred step in section 11c both ways.** The Mac agreed.

**4. A render stopped part way leaves the empty file for the frame in hand behind.** Same as test 3, frames 1 to 6, stopped with the ordinary terminate signal once frame 3's placeholder had appeared. Blender ended with exit code 143 and the folder held:

```
-rw-r--r-- 1 root root 1598467 f_0001.png
-rw-r--r-- 1 root root 1598467 f_0002.png
-rw-r--r-- 1 root root       0 f_0003.png
```

The Mac agreed.

**5. A part-written file is visible for as long as the writing takes, and that grows with the picture.** Workbench, one frame at 10000 × 10000, watched every fifty milliseconds, in PNG, OpenEXR and multi-layer OpenEXR. In every case the file appeared small and climbed to its final size; nothing appeared first under another name and nothing was renamed. The same animation as test 2 run at 8192 × 4608 showed the same thing over about two seconds instead of a fifth of one. Both machines agreed.

**6. Blender holds the file open while it writes it.** While a 10000 × 10000 multi-layer OpenEXR was being written, asking the system which program had that file open returned a line naming Blender with a write handle on it. Renaming the file at that moment succeeded, and the render still finished. Both machines agreed.

**7. Where the cached tiles go: Blender's own temporary folder, in a file named `cycles-tile-buffer`.** Cycles, 32 samples, tile size left at its default of 2048, one frame at 8000 × 8000, with every file the Blender process had open sampled twice a second. On the rented Linux machine:

```
blender 31616 root 3u REG 253,1    118784 /tmp/blender_2lDt1E/cycles-tile-buffer-31616-137632401757888-0-0.exr
blender 31616 root 3u REG 253,1  84697088 /tmp/blender_2lDt1E/cycles-tile-buffer-31616-137632401757888-0-0.exr
blender 31616 root 3r REG 253,1  89265784 /tmp/blender_2lDt1E/cycles-tile-buffer-31616-137632401757888-0-0.exr
```

On the Mac the same file sat under `/var/folders/…/T/blender_XXXXXX/`. The script printed `tempdir after: []` — it is gone when the render ends. The same render at **1000 × 1000** opened no such file at all, on either machine. **So: one file, an OpenEXR, in Blender's own temporary folder, named `cycles-tile-buffer-<process number>-<number>-0-0.exr`, and cleaned up.**

**8. The tile size decides whether it happens; the deprecated switch does not.** Same set-up at 8000 × 8000 with the tile size changed:

- Tile size **8192**: no tile-buffer file at all, and no tiling line. Eight thousand by eight thousand is 64 million pixels, fewer than 8192 × 8192.
- Tile size **256**: the file appears and is written to far more often — many times as many distinct entries as at 2048, on both machines. (The run recorded here counted 151 on Linux and 160 on the Mac.)
- Tile size **2048 with `use_auto_tile` set to False**: the tile-buffer file **still appears**. **The switch the release notes describe as removed has no effect on what reaches the disk.** (The run recorded here counted 18 entries on Linux and 18 on the Mac.) (In Blender 4.5 the same switch does turn it off. That is a difference between versions.)

*Corrected by the citation check of 2026-08-11: those counts were written as though they were figures Blender produces. They are not: the instrument samples which files the process has open twice a second over a render whose length varies. Repeating these runs on the rented Linux machine gave 123 rather than 151, and 16 rather than 18. Both claims the counts support — far more often at 256, and the file still appearing with the switch off — reproduced exactly; the numbers themselves belong to that run.*

**9. The boundary is one tile's worth of pixels — the inferred threshold in section 11h is right.** Tile size 2048, one frame, with the system's temporary folder pointed at an empty folder of the run's own and walked every twenty milliseconds. At **2048 × 2048** nothing appeared in it at all. At **2048 × 2049**, on the Mac:

```
   0.00  []
   5.09  [('.../tmp/blender_4W9vU6/cycles-tile-buffer-72781-49472579296-0-0.exr', 4096)]
   5.16  [('.../tmp/blender_4W9vU6/cycles-tile-buffer-72781-49472579296-0-0.exr', 839680)]
  11.83  []
```

One row of pixels more than a single tile is enough, and the file is removed before Blender even exits. Both machines agreed.

**10. EEVEE and Workbench write no such file.** The same 8000 × 8000 frame rendered with `BLENDER_EEVEE` and with `BLENDER_WORKBENCH` opened no tile-buffer file and put nothing in the temporary folder, on either machine. The tile cache is Cycles' alone.

**11. No texture cache is written beside the pictures a render reads.** A 1024 × 1024 PNG written by Blender itself, used as an image texture on the cube, rendered with Cycles at 4096 × 4096. The folder holding that PNG contained only the PNG, before and after. Asking Blender 5.0 what settings it has for this printed `no such setting: use_texture_cache` and `no such setting: use_auto_generate_texture_cache`. Both machines agreed. (Blender 5.2 has both of those switches and, with them on, writes a `blender_tx/` folder beside the picture. 5.0 has neither.)

**12. A File Output node's folder is made by the render and fills as the render goes.** Compositor on, one File Output node aimed at a folder that did not exist, separate from the scene's own output folder; Cycles, 1920 × 1080, frames 1 to 4. The script printed `node folder exists before the render: False`, and the render then printed, frame by frame:

```
Saved: '.../work/e12/5.0/node/n_Image0001.png'
Saved: '.../work/e12/5.0/out/f_0001.png'
```

The node's file for a frame lands just before the scene's own file for that frame. Both machines agreed. **This settles what section 11e leaves open.**

**13. With Cache Result on, a cache file is written for every frame, before that frame's picture.** Two view layers, Cycles, 1920 × 1080, frames 1 to 3, the render cache folder pointed at an empty folder. Blender printed, once per frame:

```
Caching exr file, 1920x1080, .../cache/cached_RR_UNSAVED_Scene_00000000000000000000000000000000.exr
```

and the watcher showed that file appearing and reaching 132,799,542 bytes before the frame's own PNG appeared. There is one file, not one per frame: it drops back to zero and grows again for each frame, and it is still there when the render ends. **This settles the inferred step in section 11f.** Both machines agreed.

**14. A video render puts one file in the folder from the start, and stopping it leaves something that will not play.** Workbench, 1280 × 720, FFmpeg in a Matroska container with H.264, frames 1 to 4000, stopped with the ordinary terminate signal after 25 seconds. The file `movie_0001-4000.mkv` — named for the whole frame range from the first moment — appeared at 0 bytes within a second and grew in steps of 262,144 bytes. Blender exited 143, and handing the file left behind back to Blender 5.0 gave `loaded as a movie: movie_0001-4000.mkv  frames: 0  size: (256, 256)`: it would not open. Both machines agreed. **This test was run twice** — the first run used 200 frames and finished in under four seconds, before it could be stopped; that run is recorded because it shows the same file appearing at zero bytes and the finished file opening properly.

**15. Past two gigabytes with Autosplit on, a second file appears — and the split is at about 1.87 GiB, not 2.** Workbench, 1920 × 1080, FFmpeg in an AVI container with the lossless HUFFYUV codec, frames 1 to 1400. **Run on the rented Linux machine only**, because it writes several gigabytes four times over and the Creator's Mac is their working machine; it settles Linux and leaves macOS open. With Autosplit on:

```
FINAL: [('long__0000001-1400.avi', 2003090368), ('long__0010001-1400.avi', 1022075648)]
```

With Autosplit off, one file: `FINAL: [('long_0001-1400.avi', 3025156392)]`. So the first file stops at 2,003,090,368 bytes and a second one starts, and the names gain an extra block of digits when the option is on. **This test was also run twice**: a first run of 900 frames reached only 1,945,065,136 bytes in one file and never split, which is where the figure comes from. **This is the answer to the second reading of the question — the size in bytes rather than the size in pixels — which the answer above leaves as a hole.**

**16. Two Blenders writing into one folder with Placeholders on and Overwrite off divide the work between them.** **Run on the rented Linux machine only**, and the one test here whose whole subject is two Blenders at once. Frames 1 to 8, Cycles, 1920 × 1080, the second started three seconds after the first. One took the odd frames and the other the even ones, each printing `skipping existing frame "<path>"` for the frames the other had claimed, and no frame was rendered twice. This was a folder on one machine's own disk; a folder shared over a network was not tested.

**What these tests leave open.** Windows, everywhere — no Windows machine has run it. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.** A folder shared over a network between two machines. Whether the two-gigabyte split behaves the same on macOS. And multi-view or stereoscopy output, which nothing here tested.

### Outside the interface

Partly documented, partly silent.

Documented: driven from outside Blender's own window, a render still writes frames to the output path, and — unlike a still rendered inside the interface — a single frame is saved rather than left in the buffer.

> "-f, --render-frame <frame>
>
> Render frame <frame> and save it."

> "-a, --render-anim
>
> Render frames from start to end (inclusive)."

> "blender -b animation.blend -o //render_ -F PNG -x 1 -a
>
> //render_ becomes //render_####, writing frames as //render_0001.png"

Source: the manual, https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html. Read 2026-07-30.

That is the one real difference the sources support: from the command line a single frame is written to disk, whereas 11b says a still rendered inside the interface stays in the render buffer until someone saves it.

Silent: the sources say nothing about placeholders, the tile cache, the render cache or File Output node files behaving differently when Blender is driven from outside its window. Searched the command line arguments page in full for output-related switches; the only ones there are the output path, the file format, the extension switch, the frame switches and the animation switch. There is no command line switch for placeholders or overwrite listed on that page.

**Settled by the hand tests of 2026-08-11.** It holds. Every one of the sixteen tests above was run with no window at all, so all of it is established for a Blender driven from outside. Test 1 was then repeated in a Blender **with a window open on screen**, on both machines, and printed the same three lines — the still render wrote nothing until it was asked to, exactly as with no window. Nothing was found that a window changes.

### Operating system

Mostly the same everywhere; one documented difference, and it is about the temporary directory.

> "The temporary directory is selected based on the following priority:
>
> User Preference (see File Paths).
>
> Environment variables (TEMP on Windows, TMP & TMP_DIR on other platforms).
>
> The /tmp/ directory."

Source: the manual, https://docs.blender.org/manual/en/5.0/advanced/blender_directory_layout.html. Read 2026-07-30.

The command line arguments page carries the matching pair of switches:

> "Store temporary files here (MS-Windows)."
>
> "Store temporary files here (UNIX Systems)."

Source: the manual, https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html, lines describing the temporary-directory switches. Read 2026-07-30.

For everything else in this answer — placeholders, the growing video file, tiles cached to disk, the render cache — no statement about the operating system was found, and no search for one is recorded. So that part of this line is a hole, not an established silence.

**Settled by the hand tests of 2026-08-11, and this closes that hole.** Tests 1 to 14 above were each run on macOS on an Apple M1 Pro and on Linux on an x86 machine, and the two machines agreed on every one of them — placeholders, the growing video file, the tile cache and the render cache included. So none of it is a macOS-only or Linux-only behaviour. The per-system difference the sources describe is real and it does reach the tile cache: the same `cycles-tile-buffer-…-0-0.exr` file sat under `/var/folders/…/T/` on macOS and under `/tmp/` on Linux — the same file in the place each system keeps temporary files. Nothing about **Windows** was tested — no Windows machine has run it — so every claim here leaves Windows open. Tests 15 and 16 were run on Linux only and leave macOS open; the reason is written into each.

---

## 12. What does a render do when it must make a folder and cannot?

**Label: not in the sources.**

Nothing in Blender's 5.0 manual, its 5.0 Python API documentation, the 5.0 release notes, or Cycles' own files says what happens when a render needs to create a folder and the folder cannot be created. In fact the sources never state that a render creates folders at all — the step the question asks about is itself undocumented, so what happens when it fails is doubly so.

### Where I looked

Searched the whole manual and the whole API documentation for each of these, and read every hit that was about writing or rendering:

- Folder creation: "mkdir", "makedirs", "make directory", "create the directory", "creates a directory", "creates a folder", "creates the folder", "create a folder", "created automatically", "automatically create", "will be created", "does not exist", "doesn't exist", "non-existent", "nonexistent", "missing directory", "missing folder", "parent directory", "if the directory", "if the folder", "output directory", "output folder".
- Failing to write: "unable to create", "failed to create", "could not create", "cannot create", "cannot write", "unable to write", "write permission", "not writable", "read-only", "no permission", "Permission denied", "access denied", "disk full", "no space", "write error", "cannot save", "unable to save", "invalid path", "path is invalid".

Searched the release notes and the Cycles files for "directory", "folder", "mkdir", "permission" and "fail".

Read in full: Rendering Animations, the Output properties page, Output Introduction, the File Output node page, Path Templates, the File Paths preferences page, Blender's Directory Layout, and the command line arguments page's render section.

### The nearest things the sources do say, none of which answers the question

Each of these is documented, and each is about something other than a render making a folder. I list them so the gap is visible rather than assumed.

1. Blender's file browser will offer to make a folder, but this is a person typing a path, not a render:

   > "Directory Ctrl-L
   >
   > The current folder path. Tab will auto-complete an existing path. If you type a nonexistent path, you will be prompted to create it."

   Source: the manual, https://docs.blender.org/manual/en/5.0/editors/file_browser.html. Read 2026-07-30. It says a person is prompted; it says nothing about what happens if creating fails, and nothing about renders.

2. One unrelated part of Blender does document making a missing folder — the Ocean modifier's cache:

   > "If the cache path folder does not exist, it will be created."

   Source: the manual, https://docs.blender.org/manual/en/5.0/modeling/modifiers/physics/ocean.html. Read 2026-07-30. This is a different feature with a different path, and even there the manual does not say what happens if the folder cannot be created. It is worth noting only because it shows the manual does describe folder creation when it wants to — and it never does so for render output.

3. For one specific preference, a missing folder is documented as being ignored rather than created:

   > "Temporary Files
   >
   > The directory for storing temporary save files. The path must reference an existing directory or it will be ignored and the systems temporary directory will be used instead. When left blank, the systems temporary directory will be used"

   Source: the manual, https://docs.blender.org/manual/en/5.0/editors/preferences/file_paths.html. Read 2026-07-30. This is the temporary files preference, not the render output path, and the behaviour described is falling back, not creating.

4. There is one documented case of a render failing because of its output path, but it is about the path's own text being malformed, not about a folder:

   > "Depending on the path, template errors may prevent certain actions. For example, if there are errors in the render output path, then rendering an animation will fail with an error message indicating the path errors."

   Source: the manual, https://docs.blender.org/manual/en/5.0/files/file_paths.html. Read 2026-07-30. This concerns unclosed curly braces in a path template. It does not concern folders that are absent or cannot be made.

### Not covered by answer 12

This answer does not settle whether Blender creates render output folders at all, what it does when it cannot, whether it reports an error, whether it stops the render or carries on to the next frame, or whether it behaves the same for the render output path, File Output node paths, the render cache folder and the temporary directory. All of those are open. It also does not settle whether the answer is the same for a folder that is missing and one that exists but cannot be written to.

*Every one of those has since been settled by hand tests, immediately below.*

---

### What a running Blender 5.0 showed — hand tests, 2026-08-11

**LABEL: observed.** Run on 2026-08-11 on the two computers described under question 11 above — the Creator's MacBook Pro with an Apple M1 Pro chip on macOS 26.5.2, and a rented Ubuntu 22.04.5 machine on kernel 6.8 — with Blender 5.0.1, `-b --factory-startup`, each run pointed at a settings folder of its own. On the Linux machine the runs were made by an ordinary account rather than root, because root is not stopped by an unwritable folder. Every scene was Workbench at 320 × 240, frames 1 to 3, PNG. The exact commands are in `Audits/Blender Research/Writing/Hand tests — cross-sweep questions.md`.

**17. A missing output folder is made — one level or a whole chain.** With the output path set to `<a writable folder>/newfolder/f_`, the render printed `Saved:` three times and the script printed:

```
operator returned: {'FINISHED'}
does the folder exist now: True
folder holds: ['f_0001.png', 'f_0002.png', 'f_0003.png']
```

With three missing levels at once, `<a writable folder>/a/b/c/f_`, the same. Exit code 0. Both machines agreed. **So a render does create folders, which this answer could not establish either way.**

**18. When the folder cannot be made, the render stops at the very first frame, says so in two lines, and ends with 1.** The parent folder was made and then set to read-and-enter only, and the output path named a folder inside it that did not exist. On the **Mac**:

```
does the folder exist before: False
00:00.385  image            | ERROR Couldn't create directory for file .../work/e18b/5.0/locked/newfolder/f_0001.png: Permission denied
Error: Render error (Permission denied) cannot save: '.../work/e18b/5.0/locked/newfolder/f_0001.png'
operator raised: RuntimeError Error: Render error (Permission denied) cannot save: '.../work/e18b/5.0/locked/newfolder/f_0001.png'
does the folder exist now: False
```

with `exit code: 1`. Frames 2 and 3 were never attempted, and nothing at all was left behind. Three missing levels gave the same. The **rented Linux machine** printed the identical two lines with its own paths, and behaved identically.

**Blender 5.0 names the folder in the message and gives the real reason. Blender 4.5 does neither** — for the same set-up it prints only `Error: Render error (No such file or directory) cannot save: …` with no mention of a folder. That is a difference between versions, and it matters to anything reading Blender's output.

**19. A file standing where the folder must go gives a different reason.** An ordinary file called `blocker` was made and the output path set to `<that file>/f_`:

```
ERROR Couldn't create directory for file .../work/e19b/5.0/blocker/f_0001.png: File exists
Error: Render error (File exists) cannot save: '.../work/e19b/5.0/blocker/f_0001.png'
```

Exit code 1, nothing written, render stopped at the first frame. Both machines agreed.

**20. A File Output node that cannot make its folder does *not* stop the render, and the exit code stays 0.** A File Output node was aimed at a folder inside the read-only parent, while the scene's own output path pointed somewhere that worked:

```
ERROR Couldn't create directory for file .../work/e20b/5.0/locked/nodeout/n_Image0001.png: Permission denied
ERROR Couldn't create directory for file .../work/e20b/5.0/locked/nodeout/n_Image0002.png: Permission denied
ERROR Couldn't create directory for file .../work/e20b/5.0/locked/nodeout/n_Image0003.png: Permission denied
operator returned: {'FINISHED'}
```

with `exit code: 0`, and all three of the scene's own frames written. Both machines agreed. **So the same failure stops the render when it is the scene's own path and does not stop it when it is a node's — and in the second case nothing outside Blender can tell from the exit code that anything went wrong.**

**21. A slash in the name of a node's input does not make a folder in 5.0, and the failure it gives is worded differently again.** The node's input was named `passes/diffuse`. Into a writable folder, Blender 5.0 wrote flat files with the slash turned into an underscore:

```
.../nodeopen/n_passes_diffuse0001.png
.../nodeopen/n_passes_diffuse0002.png
.../nodeopen/n_passes_diffuse0003.png
```

No `passes` folder was made. Into a folder that could not be written to, the message was not about a folder at all:

```
ERROR OpenImageIO write failed: Could not open file ".../nodelocked/n_passes_diffuse0001.png"
ERROR Render error (Success) cannot save: '.../nodelocked/n_passes_diffuse0001.png'
```

— note that Blender gives the reason as `(Success)`. `operator returned: {'FINISHED'}`, exit code 0, and the scene's own three frames were all written. Both machines agreed. (In 4.5, 5.1 and 5.2 the same name makes a real folder. That is a difference between versions.)

> **A second run disagrees with the second line, and this is not settled.** The citation check of 2026-08-11 repeated this test on the Creator's Mac with Blender 5.0.1, the node's own base folder made and then set to read-and-enter only. The first line came back the same. The second did not:
>
> ```
> 00:00.385  image.write      | ERROR OpenImageIO write failed: Could not open file ".../nodelocked/n_passes_diffuse0001.png"
> 00:00.385  reports          | ERROR Render error (Undefined error: 0) cannot save: '.../nodelocked/n_passes_diffuse0001.png'
> ```
>
> The same repeat on the rented Linux machine gave `ERROR Render error (Success) cannot save: '…'` — the wording above. So in that pair of runs the two machines did **not** agree on this line: Linux said `(Success)` and the Mac said `(Undefined error: 0)`. Both are what a system says when asked to name error number zero, and the two systems word it differently — so the claim that both machines agreed on this line is the part in doubt. Neither pair of runs is preferred over the other. The test goes back to the hand-test list as entry 25, to be run again on both machines with the wording recorded per machine. The rest of this result — the flat file names, the underscore, `{'FINISHED'}`, exit code 0, the scene's own frames written — reproduced exactly.

**22. The render cache folder is made when missing, and when it cannot be made the render carries on and finishes.** With Cache Result on and the render cache folder pointed at a folder that did not exist under a writable parent, the folder and the cache file were both made. Pointed inside the read-only parent, the render wrote all three of its frames and printed, per frame:

```
ERROR IMB_exr_begin_write: No such file or directory.
ERROR Error writing render result, No such file or directory (see console)
```

`operator returned: {'FINISHED'}`, exit code 0, and the cache folder was never made. Both machines agreed.

**23. With a window open on screen, the terminal says exactly the same thing.** The set-up of test 18 was run again in a Blender with a real window, on both machines, and printed the same two lines and raised the same error to Python. Blender was then asked to take a picture of its own window. **What the status bar and the Info list show could not be settled**: the picture shows Blender's first-run Quick Setup panel sitting over the middle of the window and an empty Info list behind it, and dismissing that panel needs a click, which nothing in this round does. That half of the test is dropped for want of a hand on a mouse. The terminal half is complete and is the answer above.

---

### The two disputed wordings, run again — hand tests, 2026-08-12

**LABEL: observed.** Entries **25** and **26** of `Audits/Blender Research/Writing/Hand tests — cross-sweep questions.md` were run on 2026-08-12. Entry 25 exists because of the disagreement recorded in the block under test 21 above; entry 26 exists for the same reason in Blender 5.1. Both were run on **both** computers:

- **The Creator's Mac** — a MacBook Pro with an Apple M1 Pro chip, 16 GB of memory, macOS 26.5.2, Apple's own graphics and no second card.
- **A rented Linux machine** — Ubuntu 22.04.5 LTS on kernel 6.8, two NVIDIA RTX 4000 Ada Generation cards of 20 GB each on driver 580.105.08, an AMD Ryzen Threadripper PRO 3945WX with eleven cores allotted, 49 GB of memory.

Blender 5.0.1 on both, started with `-b --factory-startup`, each run pointed at a settings folder of its own that was made before Blender was started. On the Linux machine the runs were made by an ordinary account called `smoozy`, not root. Only one Blender ran at a time on each machine. Blender's whole terminal output was kept, unfiltered. **The words are written down per machine and the two machines are not merged into one answer.** In the blocks below only the leading part of each long path is shortened to `...`; every word of every message is as the machine printed it.

**24. (Entry 25.) Test 21's failing run again — and the two machines print different words.** The File Output node's own base folder was made first and then set to read-and-enter only, so it exists and cannot be written to. That is the shape that produces this wording rather than a folder-creation error. The node's one input was named `passes/diffuse`. The scene's own output path pointed at a folder that works. Workbench, 320 × 240, frames 1 to 3, PNG.

On the **Creator's Mac**, Blender printed:

```
VERSION 5.0.1
File Output node built in the 5.x shape
input renamed to 'passes_diffuse'
node folder exists before: True
00:00.524  image.write      | ERROR OpenImageIO write failed: Could not open file ".../work/e25/5.0/nodelocked/n_passes_diffuse0001.png"
00:00.524  reports          | ERROR Render error (Undefined error: 0) cannot save: '.../work/e25/5.0/nodelocked/n_passes_diffuse0001.png'
Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:51:40)
00:00.537  render           | Saved: '.../work/e25/5.0/scene/s_0001.png'
00:00.543  image.write      | ERROR OpenImageIO write failed: Could not open file ".../work/e25/5.0/nodelocked/n_passes_diffuse0002.png"
00:00.543  reports          | ERROR Render error (Undefined error: 0) cannot save: '.../work/e25/5.0/nodelocked/n_passes_diffuse0002.png'
00:00.547  render           | Saved: '.../work/e25/5.0/scene/s_0002.png'
00:00.553  image.write      | ERROR OpenImageIO write failed: Could not open file ".../work/e25/5.0/nodelocked/n_passes_diffuse0003.png"
00:00.553  reports          | ERROR Render error (Undefined error: 0) cannot save: '.../work/e25/5.0/nodelocked/n_passes_diffuse0003.png'
00:00.557  render           | Saved: '.../work/e25/5.0/scene/s_0003.png'
operator returned: {'FINISHED'}
node folder exists now: True
node folder holds: []
```

with `exit code: 0`.

On the **rented Linux machine**, the same run printed:

```
VERSION 5.0.1
File Output node built in the 5.x shape
input renamed to 'passes_diffuse'
node folder exists before: True
00:00.806  image.write      | ERROR OpenImageIO write failed: Could not open file "/home/smoozy/crosssweep/work/e25/5.0/nodelocked/n_passes_diffuse0001.png"
00:00.806  reports          | ERROR Render error (Success) cannot save: '/home/smoozy/crosssweep/work/e25/5.0/nodelocked/n_passes_diffuse0001.png'
Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:30:59)
00:00.828  render           | Saved: '/home/smoozy/crosssweep/work/e25/5.0/scene/s_0001.png'
00:00.836  image.write      | ERROR OpenImageIO write failed: Could not open file "/home/smoozy/crosssweep/work/e25/5.0/nodelocked/n_passes_diffuse0002.png"
00:00.836  reports          | ERROR Render error (Success) cannot save: '/home/smoozy/crosssweep/work/e25/5.0/nodelocked/n_passes_diffuse0002.png'
00:00.841  render           | Saved: '/home/smoozy/crosssweep/work/e25/5.0/scene/s_0002.png'
00:00.846  image.write      | ERROR OpenImageIO write failed: Could not open file "/home/smoozy/crosssweep/work/e25/5.0/nodelocked/n_passes_diffuse0003.png"
00:00.846  reports          | ERROR Render error (Success) cannot save: '/home/smoozy/crosssweep/work/e25/5.0/nodelocked/n_passes_diffuse0003.png'
00:00.852  render           | Saved: '/home/smoozy/crosssweep/work/e25/5.0/scene/s_0003.png'
operator returned: {'FINISHED'}
node folder exists now: True
node folder holds: []
```

with `exit code: 0`.

**The two machines did not print the same words.** The first line was identical on both. Inside the brackets on the second line, the **Mac said `(Undefined error: 0)`** and the **rented Linux machine said `(Success)`**, on all three frames on each machine. Everything else matched on both: the slash in `passes/diffuse` became an underscore, so the name Blender tried to write was `n_passes_diffuse0001.png` and no `passes` folder was attempted; the render carried on to all three frames; the scene's own three frames were written; the node's folder was left empty; `operator returned: {'FINISHED'}`; exit code 0.

> **What this leaves standing.** Test 21 above records this line as `(Success)` on both machines and says both machines agreed. The citation check of 2026-08-11 got `(Undefined error: 0)` on the Mac and `(Success)` on Linux. The run of 2026-08-12 above got `(Undefined error: 0)` on the Mac and `(Success)` on Linux. All three records now stand in this document. Neither of the two earlier runs is chosen over the other, and this one does not choose between them either — it is a third record of what the machines printed on the day it was run.

**25. (Entry 26.) Test 22's failing run again — the two machines agree in Blender 5.0.** Cache Result on, the render cache folder pointed at a folder that does not exist under a parent set to read-and-enter only. Entry 26 was run twice per version, because its own written set-up says Workbench at 320 × 240 while the run it points back at, test 22, was Cycles at 1920 × 1080 with 400 samples and two view layers. Neither was preferred: run A repeated test 22's own set-up and run B used the set-up entry 26 itself is written with. **The added step was the second run; nothing else was changed.** Both runs gave the same words in Blender 5.0.

On the **Creator's Mac**, run B (Workbench, 320 × 240) printed:

```
VERSION 5.0.1
use_render_cache = True
render_cache_directory = '.../work/e26b/5.0/locked/cachefolder'
view layers: ['ViewLayer', 'SecondLayer']
cache folder exists before the render: False
00:00.399  image.openexr    | ERROR IMB_exr_begin_write: No such file or directory.
00:00.399  reports          | ERROR Error writing render result, No such file or directory (see console)
Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:51:40)
Caching exr file, 320x240, .../work/e26b/5.0/locked/cachefolder/cached_RR_UNSAVED_Scene_00000000000000000000000000000000.exr
00:00.451  render           | Saved: '.../work/e26b/5.0/out/f_0001.png'
```

— the same lines again for frames 2 and 3 — and ended:

```
operator returned: {'FINISHED'}
SCENE FOLDER FINAL: ['f_0001.png', 'f_0002.png', 'f_0003.png']
CACHE FOLDER FINAL: not there
```

with `exit code: 0`. Run A (Cycles, 1920 × 1080) printed the same two error lines, with `Caching exr file, 1920x1080, …` in place of `320x240`.

On the **rented Linux machine**, both runs printed the same two error lines, word for word:

```
00:19.230  image.openexr    | ERROR IMB_exr_begin_write: No such file or directory.
00:19.230  reports          | ERROR Error writing render result, No such file or directory (see console)
```

with `operator returned: {'FINISHED'}`, `SCENE FOLDER FINAL: ['f_0001.png', 'f_0002.png', 'f_0003.png']`, `CACHE FOLDER FINAL: not there` and `exit code: 0`.

**The two machines agreed, word for word, in all four runs.** This matches what test 22 above records for Blender 5.0. The disagreement entry 26 was written to chase is in Blender 5.1, not here — but the run was made in 5.0 as well, so that the four versions can be laid side by side. Laid out, the first of the two lines reads:

| Version | The Creator's Mac | The rented Linux machine |
|---|---|---|
| 4.5 | `IMB_exr_begin_write: ERROR: No such file or directory.` | `IMB_exr_begin_write: ERROR: No such file or directory.` |
| 5.0 | `ERROR IMB_exr_begin_write: No such file or directory.` | `ERROR IMB_exr_begin_write: No such file or directory.` |
| 5.1 | `ERROR Unknown error in IMB_exr_begin_write` | `ERROR IMB_exr_begin_write: No such file or directory.` |
| 5.2 | `ERROR Unknown error in IMB_exr_begin_write` | `ERROR IMB_exr_begin_write: No such file or directory.` |

The second line was `Error writing render result, No such file or directory (see console)` in every version on both machines.

---

### Outside the interface

The sources do not say. There is no statement anywhere I searched about folder creation during a render, so there is likewise no statement about whether it differs when Blender is driven from outside its own window. The command line arguments page describes what the output path switch does with frame numbers and relative paths, and says nothing about folders.

**Settled by the hand tests of 2026-08-11.** It holds. Every test above was run with no window, so all of it is established for a Blender driven from outside — including the exit codes, which are 1 when the scene's own output folder cannot be made and 0 when only a File Output node's or the render cache's cannot. Test 23 repeated the failure in a Blender with a window and the terminal said the same thing. What a person at the screen would see was not settled, for the reason given in test 23.

**Still holding after the hand tests of 2026-08-12.** Tests 24 and 25 were also run with no window, on both machines, and both ended with exit code 0 while printing their failure on the terminal. Nothing changed.

### Operating system

The sources do not say. No page I searched raises folder creation for renders on any operating system, so no difference is described and none is ruled out.

**Settled by the hand tests of 2026-08-11.** The Creator's Mac and the rented Linux machine agreed on every one of tests 17 to 23, in what the render does *and* in the words it prints — unlike Blender 4.5, where the two machines print different reasons for the same failure. Nothing about **Windows** was tested — no Windows machine has run it — so Windows stays open on all of it.

**Corrected by the hand tests of 2026-08-12, in one place.** The sentence above says the two machines agreed on the words printed in every one of tests 17 to 23. **Test 24 found one line where they do not.** When a File Output node's own base folder exists and cannot be written to, the Creator's Mac printed `ERROR Render error (Undefined error: 0) cannot save: '…'` and the rented Linux machine printed `ERROR Render error (Success) cannot save: '…'`. That is the one line, in test 21's failing run, where Blender 5.0 does differ between the two systems. Everywhere else in tests 17 to 25 the two machines printed the same words, including the render cache failure of test 25, where 5.0 agrees on both machines while 5.1 and 5.2 do not.

The earlier record is not overwritten by this: test 21 above records the same line as `(Success)` on both machines and says both agreed, and that record stands beside the two later runs that found them differing. Neither is chosen. Nothing about **Windows** was tested in any of them — no Windows machine has run it — so Windows stays open on all of it.

---

## Things only a running Blender 5.0 can settle

**All eight items of the original list were run on 2026-08-11, on the Creator's Mac and on a rented Linux machine, and their results are written into questions 11 and 12 above.** What is left is only what those two machines could not reach.

1. **Anything on Windows** — an output folder that cannot be made, a folder denied by the account's permissions, a drive letter that does not exist, and the exit code (`echo %ERRORLEVEL%`). **Not run with the equipment used**: the runs used one Mac and one rented Linux machine, and no Windows machine was rented for them. Every question-11 result above leaves Windows open too.
2. **A folder shared over a network between two machines.** Test 16 above ran two Blenders into one folder on a single machine's own disk. The manual's render-farm recipe is about a shared network folder, which is not the same thing, and no network share between two machines is among this project's written-down equipment. It needs the Creator's say-so first.
3. **Whether the two-gigabyte video split behaves the same on macOS.** Test 15 was run on the rented Linux machine only, because it writes several gigabytes four times over and the Mac is the Creator's working machine.
4. **What Blender's own window shows when a render cannot make its folder** — the status bar and the Info list, word for word. The terminal half is settled (test 23), but the picture Blender took of its own window has the first-run Quick Setup panel over it, and dismissing that needs a click. Anybody with a hand on a mouse can settle it in a minute: set the output path into a folder you cannot write to, press Ctrl-F12, and read the status bar and the Info editor.
5. **Multi-view or stereoscopy output during a render.** Nothing here tested it.

---

2026-08-11 — Claude Opus 5 (Claude Code) — Writing, Blender 5.0

2026-08-11 — Claude Opus 5 (Claude Code) — hand tests: twenty-three observed results written into questions 11 and 12, both standing lines brought up to date on both answers, and the researcher's own list pruned to what two machines cannot reach

2026-08-12 — Claude Opus 5 (Claude Code) — second batch of hand tests: entries 25 and 26 run on both machines and written into question 12 as tests 24 and 25. The operating-system standing line on question 12 is corrected: the two machines do **not** print the same words when a File Output node's own base folder cannot be written to. The earlier record that says they agreed stands beside it and is not overwritten. Nothing on the list above closed or opened; it is unchanged.

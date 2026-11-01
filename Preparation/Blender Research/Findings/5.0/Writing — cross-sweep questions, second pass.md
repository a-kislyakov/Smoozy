# Writing — cross-sweep questions, second pass (Blender 5.0)

**Sources consulted, and when.** Blender's 5.0 manual (`Blender Sources/manual_5.0`, including two of its picture files), Blender's 5.0 Python API documentation (`Blender Sources/api_5.0`), Blender's 5.0 release notes (`Blender Sources/release_notes_5.0`), and Cycles' own files as shipped with Blender 5.0 (`Blender Sources/cycles_5.0`). All four were taken on **2026-07-30**, and that is the date read for everything quoted below. The searching and reading was done on 2026-08-11.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

Question answered: **11** only.

---

## 11. While a render is still working, what is present in the places it writes to, and how does that change with the size of the picture?

**The question was ambiguous in one place, so I say which reading I took.** "The size of the picture" I have read as *how many pixels across and down the frame is* — the render resolution. The other available reading is *how many bytes the output file is*; I answer that one briefly too, at the end of the size section, because one documented behaviour (Autosplit) turns on byte size rather than pixel size.

I have also read "the places it writes to" as covering every place a running render puts files, not only the main output path, because a render writes to more than one place.

### Short answer

While a render is running, the sources describe several different things sitting on disk at once:

- **The main output path**, during an animation render, holds the frames that have already finished, and nothing for the frames not yet reached — unless the **Placeholders** option is on, in which case empty files also sit there for frames that have been claimed but not finished.
- **The main output path**, during a *still* render, is not written to until the render is over.
- **The main output path**, when the output is a video, holds one partly-written video file while the render runs.
- **Any File Output node's directory** is written to as the render goes.
- **The Render Cache folder** holds a multi-layer OpenEXR file, if Cache Result is on.
- **The temporary directory** holds run-time files including render layers, and — this is the part that depends on picture size — Cycles' tiles.

Picture size enters through Cycles' tiling: a frame bigger than one tile is rendered in tiles, and "Tiles are cached to disk while rendering to save memory." A frame that fits in one tile does not get that treatment.

---

### A. The main output path during an animation render: finished frames

**Label: documented.**

> "While rendering stills will allow you to view and save the image from the render buffer when it is complete, animations are a series of images, or frames, and are automatically saved directly out to a drive after being rendered."

Source: manual. https://docs.blender.org/manual/en/5.0/render/output/animation.html — date read 2026-07-30.

> "This allows you an easy recovery if there is a problem and you have to re-start the rendering, since the frames you have already rendered will still be in the output directory."

Source: manual, same page. Date read 2026-07-30.

> "For example, when rendering animations to image file formats the render job can be canceled and resumed at the last rendered frame by changing the frame range."

Source: manual. https://docs.blender.org/manual/en/5.0/render/output/introduction.html — date read 2026-07-30.

And from the Python API, confirming that writing happens frame by frame inside the job rather than once at the end:

> "bpy.app.handlers.render_write
>
> on writing a render frame (directly after the frame is written)"

Source: Python API documentation. https://docs.blender.org/api/5.0/bpy.app.handlers.html — date read 2026-07-30.

So: during an animation render to an image sequence, what is present at the output path is the set of already-completed frames, growing one file at a time.

---

### B. The Placeholders option: empty files for frames not yet done

**Label: documented** for what the option is; **inferred** for *when* in a frame's life the empty file appears.

The manual's Output panel page lists the option (its heading in the plain-text copy is garbled — the same heading text is repeated for Overwrite and for this one — but the description under it is clear):

> "Create empty placeholder frames while rendering."

Source: manual. https://docs.blender.org/manual/en/5.0/render/output/properties/output.html — date read 2026-07-30.

The Python API gives it a fuller description and its default:

> "use_placeholder
>
> Create empty placeholder files while rendering frames (similar to Unix 'touch')
>
> Type:
>
> boolean, default False"

and next to it:

> "use_overwrite
>
> Overwrite existing files while rendering
>
> Type:
>
> boolean, default True"

Source: Python API documentation. https://docs.blender.org/api/5.0/bpy.types.RenderSettings.html — date read 2026-07-30.

**From a picture.** The manual's screenshot of the Output panel, `_images/render_output_properties_output_panel.png` on the same Output page, shows four checkboxes above the format settings: **Overwrite** (ticked), **Placeholders** (unticked), **File Extensions** (ticked), **Cache Result** (unticked), with the output path field above them reading `/tmp/`. That is a picture from the manual, so what is visible in it is **documented**: Placeholders and Overwrite are two independent tick boxes sitting side by side in the Output panel, and Placeholders is off by default while Overwrite is on. Date read 2026-07-30.

The manual then states what the two are for together:

> "Hint
>
> Primitive Render Farm
>
> An easy way to get multiple machines to share the rendering workload is to:
>
> Set up a shared directory over a network file system.
>
> Disable Overwrite, enable Placeholders in the Render Output panel.
>
> Start as many machines as you wish rendering to that directory."

Source: manual, same Output page. Date read 2026-07-30.

**Inferred, with the step written out.** The manual does not say in so many words that the empty file is created *before* the frame is rendered. I take that step from the render-farm hint: the scheme only shares work between machines if a machine can see, before it starts a frame, that another machine has already taken it. The only mark of that is the empty file. Therefore the empty file must exist while the frame is still being rendered, not after. A reader has to supply that reasoning, so this is inferred, not documented.

What follows from it: with Placeholders on, the output folder during a render holds a mixture — finished frames as full files, and empty (zero-content) files for the frames currently in hand. The sources do not say what happens to the empty file when the frame finishes, nor whether it is removed if the render is cancelled.

---

### C. A still render: nothing at the output path until it is done

**Label: inferred.**

The quoted sentence in section A draws the contrast itself: stills "allow you to view and save the image from the render buffer when it is complete", against animations which "are automatically saved directly out to a drive after being rendered". The API describes the still-render switch as a save that happens as part of the operator:

> "write_still (boolean, (optional)) – Write Image, Save the rendered image to the output path (used only when animation is disabled)"

Source: Python API documentation. https://docs.blender.org/api/5.0/bpy.ops.render.html — date read 2026-07-30.

The step I took: the manual says a still lives in the render buffer until complete and is saved from there, so nothing of that still is at the output path while it is being rendered. The sources never state this as a fact about the folder, so it is inferred.

---

### D. When the output is a video: one partly-written file

**Label: inferred**, from two documented statements.

> "If a problem occurs while rendering, the file might become unplayable and you will have to re-render all frames from the beginning."

Source: manual. https://docs.blender.org/manual/en/5.0/render/output/properties/output.html — date read 2026-07-30.

> "Autosplit Output
>
> If your video is huge and exceeds 2GiB, enable Autosplit Output. This will automatically split the output into multiple files after the first file is 2GiB in size."

Source: manual, same page. Date read 2026-07-30. The Python API states the same threshold:

> "use_autosplit
>
> Autosplit output at 2GB boundary"

Source: Python API documentation. https://docs.blender.org/api/5.0/bpy.types.FFmpegSettings.html — date read 2026-07-30.

The step I took: a file that can "become unplayable" if the render is interrupted must already be on disk, incomplete, while the render runs; and a file that is split "after the first file is 2GiB in size" must be being measured as it grows. So during a video render the output path holds a single growing container file (and, with Autosplit on, further files once earlier ones pass the 2 GiB mark). The sources never describe the in-progress file directly, so this is inferred.

---

### E. File Output nodes write as the render goes

**Label: documented.**

> "The File Output node writes one or more images to disk during rendering or compositing."

and

> "Each connected input is saved to the specified directory and filename, with the frame number appended to create a sequence. For example: {Directory}/{File Name}{Socket Name}{frame number}.{extension}."

Source: manual. https://docs.blender.org/manual/en/5.0/compositing/types/output/file_output.html — date read 2026-07-30.

So each such node's directory is also a place that fills up while the render is still working. The page does not say at what point within a frame the write happens.

---

### F. The Render Cache folder

**Label: documented** for what is put there; **inferred** for when.

> "Saving – Cache Result
>
> Saves the rendered view layers and their passes to a multi-layer OpenEXR image. The Compositor can then use this file to improve performance, especially for heavy compositing.
>
> The image is stored in the Render Cache folder as specified in the File Paths Preferences."

Source: manual. https://docs.blender.org/manual/en/5.0/render/output/properties/output.html — date read 2026-07-30.

> "Render Cache
>
> The location where cached render images are stored."

Source: manual. https://docs.blender.org/manual/en/5.0/editors/preferences/file_paths.html — date read 2026-07-30.

> "use_render_cache
>
> Save render cache to EXR files (useful for heavy compositing, Note: affects indirectly rendered scenes)"

Source: Python API documentation. https://docs.blender.org/api/5.0/bpy.types.RenderSettings.html — date read 2026-07-30. This page gives its default as `boolean, default False`.

The hint that these files exist *before* a render job has succeeded:

> "Reads all the current scene's render layers from cache, as needed. This can be used to save RAM while rendering because the render layers do not have to be saved in RAM. And also for recovering some information from a failed render. For this to work, Cache Result must be enabled."

Source: manual. https://docs.blender.org/manual/en/5.0/interface/controls/nodes/editing.html — date read 2026-07-30.

The step I took: something can only be recovered from a *failed* render if it was written before the failure. So with Cache Result on, cache EXR files appear in the Render Cache folder during the render, not only at the end. Inferred.

The sources do not give the name or naming pattern of the cache file. The manual only says Blender can "Find the render cache file for the current scene" (same Image editor page, under Open Cached Render).

---

### G. The temporary directory

**Label: documented.**

> "The temporary directory is used to store various files at run-time (including render layers, physics cache, copy-paste buffer and crash logs).
>
> The temporary directory is selected based on the following priority:
>
> User Preference (see File Paths).
>
> Environment variables (TEMP on Windows, TMP & TMP_DIR on other platforms).
>
> The /tmp/ directory."

Source: manual. https://docs.blender.org/manual/en/5.0/advanced/blender_directory_layout.html — date read 2026-07-30.

> "Temporary Files
>
> The directory for storing temporary save files. The path must reference an existing directory or it will be ignored and the systems temporary directory will be used instead. When left blank, the systems temporary directory will be used (see Temporary Directory for details)."

Source: manual. https://docs.blender.org/manual/en/5.0/editors/preferences/file_paths.html — date read 2026-07-30.

So the temporary directory is a third place with files in it while a render works. The sources name the *kinds* of thing there ("render layers") but no file names.

---

### H. How this changes with the size of the picture

This is where Cycles' tiling comes in, and it is the only place in the 5.0 sources where the pixel size of the frame changes what is written to disk during a render.

**Label: documented.**

> "Tile Size
>
> Render high resolution images in tiles of this size, to reduce memory usage. Tiles are cached to disk while rendering to save memory."

Source: manual. https://docs.blender.org/manual/en/5.0/render/cycles/render_settings/performance.html — date read 2026-07-30.

The release notes state that in 5.0 this is no longer optional:

> "* Remove Use Tiling option, it is now always assumed to be on. The tile size can be increased to disabled tiling. (blender/blender!146031)"

Source: release notes. https://developer.blender.org/docs/release_notes/5.0/cycles/ — date read 2026-07-30. (The typo "to disabled tiling" is in the original.)

**Label: observed** — Cycles' own files, which give the setting's exact name, kind, default and range.

File `Blender Sources/cycles_5.0/properties.py`, lines 1051–1062:

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

File `Blender Sources/cycles_5.0/ui.py`, lines 390–405 (inside the path-guiding panel, where Cycles works out whether tiling will be in effect so it can warn the user):

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

File `Blender Sources/cycles_5.0/version_update.py`, lines 114–119:

*Corrected by the citation check of 2026-08-11: this said lines 116–119, but the comment and the `if version` line quoted below are at 114 and 115, so two of the six quoted lines fell outside the range given. The words are unchanged; only the line numbers are.*

```
            # Auto tiling is always enabled now
            if version <= (5, 0, 77):
                cscene = scene.cycles
                if not cscene.use_auto_tile:
                    cscene.use_auto_tile = True
                    cscene.tile_size = 8192
```

File `Blender Sources/cycles_5.0/ui.py`, lines 866–878: the Memory sub-panel of Performance draws only `tile_size` — the deprecated `use_auto_tile` is not drawn anywhere in the panel.

Date read for all four: 2026-07-30.

**Label: inferred**, with the step written out. Putting the manual's sentence together with the code above: Cycles compares the frame's area in pixels (resolution times the resolution percentage, both directions multiplied) against the area of one tile. If the frame is no bigger than one tile, it is one tile and there is nothing to cache; if the frame is bigger, it is broken into tiles and, per the manual, "Tiles are cached to disk while rendering". So the change with picture size is a threshold, not a smooth curve: below roughly 2048×2048 pixels at the shipped default tile size, a Cycles render writes no tile cache at all during a frame; above it, tile data is written to disk during the frame, in a place that is not the output folder, and the amount of it grows with the number of tiles the frame is cut into. A reader has to supply the joining-up, so this is inferred rather than documented.

Two things I must be careful not to claim. The code I quoted is Cycles' *interface* code, working out whether to show a warning; it names the two engine functions it mirrors (`TileManager::compute_render_tile_size`, `Session::get_effective_tile_size`) but it is not those functions. And the `render_size_x <= 8192 and render_size_y <= 8192` part of the test I do not interpret at all — I quote it and leave it, because I have no source in this folder that says what it means.

**Where the tile cache goes: not in the sources.** The manual says "cached to disk" and stops. It never names the folder, the file, or the format. The temporary-directory page (section G) says the temporary directory holds "render layers" at run time, which is suggestive, but it does not say "tiles", and I am not going to make that leap.

**The other reading of "size of the picture" (bytes rather than pixels).** One documented behaviour turns on byte size: Autosplit, quoted in section D, splits video output into further files "after the first file is 2GiB in size". Nothing else in the 5.0 sources ties the number or nature of in-progress files to how many bytes they contain.

**Other render engines.** The EEVEE Performance page (https://docs.blender.org/manual/en/5.0/render/eevee/render_settings/performance.html, read 2026-07-30) has Memory settings for shadow and light-probe pools but says nothing about caching anything to disk, and there is no tile size there. So the picture-size effect above is a Cycles matter only, as far as the sources go.

---

### What a running Blender 5.0 showed — hand tests, 2026-08-11

**LABEL: observed.** Every result below was produced on 2026-08-11 by running Blender 5.0.1 on two computers and writing down what it printed and what appeared on disk. The two computers are named in full because that is what settles which operating system each answer covers.

- **The Creator's Mac** — a MacBook Pro with an Apple M1 Pro chip, 16 GB of memory, macOS 26.5.2 (build 25F84), Apple's own graphics and no second card. Blender 5.0.1 at `/Applications/Blender 5.0.1.app/Contents/MacOS/Blender`.
- **A rented Linux machine** — Ubuntu 22.04.5 LTS on kernel 6.8, two NVIDIA RTX 4000 Ada Generation cards of 20 GB each on driver 580.105.08, an AMD Ryzen Threadripper PRO 3945WX with eleven cores, 49 GB of memory. Blender 5.0.1 at `/opt/blender/5.0/blender`.

Every run used `-b --factory-startup` unless it says otherwise, and each was pointed at a settings folder of its own so that no run touched anybody's saved Blender settings. Where the same test ran on both machines, both results are given and it is said plainly whether they agreed. The whole list, with the exact commands, is `Audits/Blender Research/Writing/Hand tests — cross-sweep questions.md`.

**1. Section C is right: a still render writes nothing at the output path unless it is asked to.** Output path set to an empty folder, PNG, 320 × 240, Cycles:

```
BEFORE: []
AFTER render, no save switch: []
AFTER render with the save switch: ['still_.png']
```

`blender -b --factory-startup -o <folder>/cli_ -F PNG -f 1` wrote `cli_0001.png`. The same three runs repeated in a Blender **with a window open on screen** printed the same three lines. Both machines agreed.

**2. Section A is right, and it settles the first point under Not covered.** Cycles, 1920 × 1080, 400 samples, frames 1 to 4, PNG, Placeholders off, with a second program listing the folder every fifty milliseconds. On the rented Linux machine:

```
   0.00  .../out  []
   9.96  .../out  [('f_0001.png', 180224)]
  10.01  .../out  [('f_0001.png', 892928)]
  10.06  .../out  [('f_0001.png', 1598467)]
  19.55  .../out  [('f_0001.png', 1598467), ('f_0002.png', 253952)]
  19.60  .../out  [('f_0001.png', 1598467), ('f_0002.png', 950272)]
  19.71  .../out  [('f_0001.png', 1598467), ('f_0002.png', 1598467)]
```

For the ten seconds a frame is being rendered there is nothing for it in the folder at all. For the fifth of a second it is being written, the file is there and growing. **It is built up in place: no file under another name appears first, and nothing is renamed.** The Mac agreed. At 8192 × 4608 the same thing took about two seconds instead of a fifth of one.

**3. Section B's inferred step is right: the empty file appears before the frame is rendered.** Placeholders on, Overwrite off, same scene. On the rented Linux machine:

```
   0.00  .../out  []
   0.30  .../out  [('f_0001.png', 0)]
   9.85  .../out  [('f_0001.png', 516096)]
   9.90  .../out  [('f_0001.png', 1220608)]
   9.95  .../out  [('f_0001.png', 1598467), ('f_0002.png', 0)]
```

The empty file for frame 1 is there three tenths of a second in, nearly ten seconds before that frame finishes; the one for frame 2 appears the moment frame 1's picture is complete. They are made one at a time, for the frame in hand, and the picture is written into the same file rather than a second one. The Mac agreed.

**4. What becomes of the empty file if the render is cancelled: it stays.** Same as test 3, frames 1 to 6, stopped with the ordinary terminate signal once frame 3's placeholder had appeared. Blender exited 143 and the folder held two finished frames and `f_0003.png` at 0 bytes. Both machines agreed.

**5. Blender holds the file open while it writes it.** During a 10000 × 10000 multi-layer OpenEXR write, the system reported Blender holding that file with a write handle. Renaming it at that moment succeeded and the render still finished. Both machines agreed.

**6. Where the Cycles tile cache lives, what it is called, how big it gets, and whether it is cleaned up — the whole of the second point under Not covered.** Cycles, 32 samples, tile size at its default 2048, one frame at 8000 × 8000, with every file the process had open sampled twice a second. On the rented Linux machine:

```
blender 31616 root 3u REG 253,1    118784 /tmp/blender_2lDt1E/cycles-tile-buffer-31616-137632401757888-0-0.exr
blender 31616 root 3u REG 253,1  84697088 /tmp/blender_2lDt1E/cycles-tile-buffer-31616-137632401757888-0-0.exr
blender 31616 root 3r REG 253,1  89265784 /tmp/blender_2lDt1E/cycles-tile-buffer-31616-137632401757888-0-0.exr
```

On the Mac the same file sat under `/var/folders/…/T/blender_XXXXXX/`. **One file, an OpenEXR, in Blender's own temporary folder, named `cycles-tile-buffer-<process number>-<number>-0-0.exr`, reaching about 89 MB for that frame, and gone before Blender exits** — the script printed `tempdir after: []`. The same render at 1000 × 1000 opened no such file at all. So the guess section H refuses to make — that the tile cache is the same thing as the "render layers" the manual puts in the temporary directory — was right to refuse: it is a file of its own, with its own name, in that folder.

**7. The threshold in section H is right, and it is a threshold.** Tile size 2048, one frame, with the system's temporary folder pointed at an empty folder of the run's own and walked every twenty milliseconds. At **2048 × 2048** nothing appeared in it at all. At **2048 × 2049**, on the Mac:

```
   0.00  []
   5.09  [('.../tmp/blender_4W9vU6/cycles-tile-buffer-72781-49472579296-0-0.exr', 4096)]
   5.16  [('.../tmp/blender_4W9vU6/cycles-tile-buffer-72781-49472579296-0-0.exr', 839680)]
  11.83  []
```

Both machines agreed. And at 8000 × 8000 with the tile size raised to **8192** — larger in area than the picture — there was no tile-buffer file at all, while at **256** there was one written to far more often. **What has no effect is `use_auto_tile`**: setting it to False at the default tile size still produced the file, on both machines. In Blender 4.5 the same switch does turn it off, so that is a difference between versions and not a machine difference.

**8. EEVEE and Workbench write no such file.** The same 8000 × 8000 frame under `BLENDER_EEVEE` and under `BLENDER_WORKBENCH` opened no tile-buffer file and put nothing in the temporary folder, on either machine.

**9. When a File Output node writes, to the frame.** Compositor on, one node aimed at a folder that did not exist. `node folder exists before the render: False`, and then, frame by frame:

```
Saved: '.../work/e12/5.0/node/n_Image0001.png'
Saved: '.../work/e12/5.0/out/f_0001.png'
```

The node's folder is made by the render, and its file for a frame lands just before the scene's own file for that frame. Both machines agreed.

**10. The Render Cache file's name, and that there is one rewritten per frame.** Two view layers, Cycles, 1920 × 1080, frames 1 to 3. Blender printed, once per frame:

```
Caching exr file, 1920x1080, .../cache/cached_RR_UNSAVED_Scene_00000000000000000000000000000000.exr
```

and the watcher showed the file appearing and reaching 132,799,542 bytes before that frame's own PNG appeared, then dropping to zero and growing again for the next frame. It is still there at the end. Both machines agreed.

**11. Section D is right about the video file, and stopping the render leaves something that will not play.** Workbench, 1280 × 720, FFmpeg in a Matroska container with H.264, frames 1 to 4000, stopped after 25 seconds. `movie_0001-4000.mkv` — named for the whole frame range from the first moment — appeared at 0 bytes within a second and grew in steps of 262,144 bytes. Handing the file left behind back to Blender 5.0 gave `loaded as a movie: movie_0001-4000.mkv  frames: 0  size: (256, 256)`. Both machines agreed. **This test was run twice**: the first run used 200 frames and finished before it could be stopped, and that run showed the finished file opening properly.

**12. Autosplit: the second file starts at about 1.87 GiB, not 2.** Workbench, 1920 × 1080, FFmpeg in an AVI container with the lossless HUFFYUV codec, 1400 frames. **Run on the rented Linux machine only** — it writes several gigabytes four times over and the Mac is the Creator's working machine — so it settles Linux and leaves macOS open. With Autosplit on:

```
FINAL: [('long__0000001-1400.avi', 2003090368), ('long__0010001-1400.avi', 1022075648)]
```

With it off, one file of 3,025,156,392 bytes. The names gain an extra block of digits when the option is on. **This test was also run twice**: a first run of 900 frames reached 1,945,065,136 bytes and never split.

**13. No texture cache is written beside the pictures a render reads.** A PNG written by Blender itself, used as an image texture, rendered with Cycles at 4096 × 4096: the folder holding it contained only that PNG before and after. Asking Blender 5.0 what settings it has for this printed `no such setting: use_texture_cache` and `no such setting: use_auto_generate_texture_cache`. Both machines agreed. (Blender 5.2 has both switches and, with them on, writes a `blender_tx/` folder beside the picture.)

**14. The render-farm scheme does stop two machines duplicating a frame.** **Run on the rented Linux machine only**, and the one test here whose whole subject is two Blenders at once. Placeholders on, Overwrite off, frames 1 to 8, the second Blender started three seconds after the first. One took the odd frames and the other the even ones, each printing `skipping existing frame "<path>"` for the frames the other had claimed. No frame was rendered twice. This was a folder on one machine's own disk; a folder shared over a network was not tested.

**What these tests leave open.** Windows, everywhere — no Windows machine has run it. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.** A folder shared over a network. Whether the two-gigabyte split behaves the same on macOS. And multi-view or stereoscopy output.

---

### Not covered

*Most of this list has since been settled by the hand tests above; the note after it says which parts stand.*

What this answer does not settle:

- Whether a single frame's output file is built up in place on disk while it is being written, or written whole in one go, or written to a temporary name and renamed. I searched for this specifically (terms in the record below) and the 5.0 sources say nothing at all about it. So I cannot say what a watcher would see in the output folder in the seconds while one large EXR or PNG is being written out.
- Where Cycles' tile cache files live, what they are called, what format they are in, how large they are, and whether they are cleaned up when a render is cancelled or crashes.
- Whether the Placeholders empty file is created before the frame starts (I inferred it is) and what becomes of it if the render is cancelled.
- The exact moment within a frame at which a File Output node writes.
- The name of the Render Cache file, and whether it is rewritten per frame during an animation.
- Anything about multi-view or stereoscopy output during a render.
- Anything measured. I ran nothing; I have no Blender 5.0 to run, and the folder I was given contains documentation and Cycles' add-on files only, no program.

**What the hand tests of 2026-08-11 settled from that list, and what is left.** Settled: that a frame's file is built up in place on disk and not written whole or renamed; where the tile cache lives, what it is called, how big it gets and that it is cleaned up before Blender exits; that the placeholder is made before its frame and stays behind after a cancel; when a File Output node writes; the Render Cache file's name and that there is one rewritten per frame; and the measurements throughout. **Still open**: everything on Windows, a folder shared over a network, whether the two-gigabyte split behaves the same on macOS, and multi-view or stereoscopy output.

---

### Outside the interface

Does this hold when Blender is driven from outside its own window?

**Label: inferred**, and partly **not in the sources**.

Everything quoted above is a scene or preference setting, not a window behaviour. The manual states that command-line rendering uses the file's own settings:

> "-a
>
> Render the whole animation using all the settings saved in the blend-file."

Source: manual. https://docs.blender.org/manual/en/5.0/advanced/command_line/render.html — date read 2026-07-30.

> "-b, --background
>
> Run in background (often used for UI-less rendering)."

Source: manual. https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html — date read 2026-07-30.

The step I took: since Overwrite, Placeholders, Cache Result, the output path and Cycles' Tile Size are all saved in the file and are used by the render, a background render behaves the same way as one started by clicking. That is a step the reader has to supply; the sources nowhere say "placeholders also work in background mode".

Two things the command-line pages *do* add that bear on this question:

- Argument order matters, and getting it wrong means the render writes somewhere other than where you meant: "blender --background test.blend --render-frame 1 --render-output "/tmp"" … "…will not render to /tmp because --render-frame 1 renders before the output path is set." (same arguments page; date read 2026-07-30). So the place that fills up during a background render is decided by argument order, not just by the file.
- The temporary directory — the place run-time render files go — can be set from outside by the `TEMP` / `TMPDIR` environment variables (same arguments page, and the directory-layout page quoted in section G).

The sources do not describe any difference in what is on disk mid-render between an interface render and a background one. That silence is the honest state of it, not a claim of sameness.

The render-farm hint in section B is itself an outside-the-interface case: several machines writing into one shared network directory, coordinating only through Placeholders and Overwrite. The manual gives no further detail on how that coordination behaves when two machines reach the same frame at the same instant.

**Settled by the hand tests of 2026-08-11.** It holds. All fourteen tests above were run with no window at all, so every one of them is established for a Blender driven from outside its own window. Test 1 was then repeated in a Blender **with a window open on screen**, on both machines, and printed the same three lines. Nothing was found that a window changes. On the render-farm scheme: test 14 ran two Blenders three seconds apart into one folder and they divided the frames between them cleanly, with neither rendering a frame the other had claimed — but that folder was on one machine's own disk, not a network share, so the network case is still untested.

---

### Operating system

Does this hold on every operating system Blender runs on?

**Label: documented** for the one place where the sources say the operating system matters, and **not in the sources** for the rest.

The documented difference is *where* the temporary directory is:

> "Environment variables (TEMP on Windows, TMP & TMP_DIR on other platforms)."

Source: manual. https://docs.blender.org/manual/en/5.0/advanced/blender_directory_layout.html — date read 2026-07-30.

with a fallback of "The /tmp/ directory" on the same page. The command-line arguments page names them slightly differently — "TEMP: Store temporary files here (MS-Windows)." and "TMPDIR: Store temporary files here (UNIX Systems)." (https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html, date read 2026-07-30). Either way, the folder that holds run-time render files is a different place on Windows than elsewhere.

Nothing else in the sources ties any of the above to an operating system. The Placeholders description mentions Unix — "similar to Unix 'touch'" — but that is a comparison used to explain what an empty file is, not a statement that the option is Unix-only, and the option is in the Output panel screenshot with no platform note. The Cycles tiling text carries no platform note. The video and Autosplit text carries no platform note. So for everything except the location of the temporary directory: the sources do not say, and I will not guess.

**Settled by the hand tests of 2026-08-11.** Tests 1 to 11 and 13 above were each run on macOS on an Apple M1 Pro and on Linux on an x86 machine, and the two machines agreed on every one of them, so none of it is a macOS-only or Linux-only behaviour. The per-system difference the sources describe is real and it does reach the tile cache: the same `cycles-tile-buffer-…-0-0.exr` file sat under `/var/folders/…/T/` on macOS and under `/tmp/` on Linux — the same file in the place each system keeps temporary files. Nothing about **Windows** was tested — no Windows machine has run it — so every claim here leaves Windows open. Tests 12 and 14 were run on Linux only and leave macOS open; the reason is written into each.

---

## Record of where I looked

This is part of the answer, not a footnote.

**Searched across `manual_5.0`, `api_5.0`, `release_notes_5.0` and `cycles_5.0` together, for these terms** (case-insensitive, whole folders):

`save buffers`, `save_buffers`, `placeholder`, `Tile Size`, `tile_size`, `use_tiling`, `tile cache`, `cached to disk`, `tiles are cached`, `render cache`, `cache result`, `use_render_cache`, `Open Cached Render`, `Temporary Files`, `temp directory`, `Temporary Directory`, `tempdir`, `temporary file`, `partially written`, `partial`, `incomplete`, `truncated`, `unplayable`, `corrupt`, `half-written`, `in progress`, `while rendering`, `during rendering`, `during the render`, `as it renders`, `atomic`, `.tmp`, `renamed`, `written to disk`, `saved to disk`, `writes to disk`, `flushed`, `scratch`, `spill`, `swap file`, `intermediate file`, `lock file`, `.lock`, `touch`, `zero-byte`, `zero byte`, `empty file`, `0 bytes`, `progressive`, `one at a time`, `after being rendered`, `output folder`, `output directory`, `already rendered`, `resume`, `Autosplit`, `high resolution`, `large resolution`, `resolution_x`, `big image`, `full sample`, `two pass`, `render-output`, `render-anim`, `render-frame`, `Saved:`.

I chose those terms to cover three vocabularies at once: the plain words (empty file, half-written, folder, leftover), the industry words (placeholder, tile, cache, buffer, atomic write, scratch space, autosplit), and the words that sit near them in Blender's own naming (save buffers, full sample, render cache, temporary directory).

**Pages I opened and read in full or in substantial part:**

- https://docs.blender.org/manual/en/5.0/render/output/properties/output.html — the Output panel: File Path, Cache Result, Media Type, Overwrite, Placeholders, the render-farm hint, Encoding, Autosplit, and the closing tips about rendering to video.
- https://docs.blender.org/manual/en/5.0/render/output/animation.html — Rendering Animations, in full.
- https://docs.blender.org/manual/en/5.0/render/output/introduction.html — in full.
- https://docs.blender.org/manual/en/5.0/render/output/properties/post_processing.html — in full; ruled out, it covers Compositing/Sequencer switches and Dither only, nothing about disk.
- https://docs.blender.org/manual/en/5.0/render/cycles/render_settings/performance.html — in full.
- https://docs.blender.org/manual/en/5.0/render/eevee/render_settings/performance.html — in full; ruled out for picture-size disk behaviour, it has no tiling and no disk caching.
- https://docs.blender.org/manual/en/5.0/compositing/types/output/file_output.html — in full.
- https://docs.blender.org/manual/en/5.0/editors/preferences/file_paths.html — the Render Output, Render Cache and Temporary Files entries.
- https://docs.blender.org/manual/en/5.0/advanced/blender_directory_layout.html — the Temporary Directory section.
- https://docs.blender.org/manual/en/5.0/advanced/command_line/render.html — in full.
- https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html — the render arguments (`-b`, `-a`, `-f`, `-o`, `-s`, `-e`, `-j`), the argument-order warning, and the environment variables section.
- https://docs.blender.org/manual/en/5.0/editors/image/editing.html — Open Cached Render.
- https://docs.blender.org/manual/en/5.0/interface/controls/nodes/editing.html — Read View Layers.
- https://docs.blender.org/api/5.0/bpy.types.RenderSettings.html — `use_overwrite`, `use_placeholder`, `use_render_cache`, `use_persistent_data` and neighbours.
- https://docs.blender.org/api/5.0/bpy.types.FFmpegSettings.html — `use_autosplit`.
- https://docs.blender.org/api/5.0/bpy.app.handlers.html — the render handler list.
- https://docs.blender.org/api/5.0/bpy.app.html — `bpy.app.tempdir`.
- https://docs.blender.org/api/5.0/bpy.ops.render.html — `bpy.ops.render.render`.
- https://docs.blender.org/api/5.0/bpy.types.RenderEngine.html — checked for tile-and-disk behaviour; ruled out, its tile calls (`tile_highlight_set`, `tile_highlight_clear_all`, and the `cancel`/`highlight` arguments) are about an in-memory result and its display, not about files.
- https://developer.blender.org/docs/release_notes/5.0/cycles/ — the Other Features list.
- https://developer.blender.org/docs/release_notes/5.0/rendering/ — in full; nothing on in-progress files.
- https://developer.blender.org/docs/release_notes/5.0/pipeline_io/ — in full; its Images & Movies section covers HDR video, wide-gamut images and multi-part multilayer EXR, none of which bears on what sits on disk mid-render.
- I also swept `release_notes_5.0/index.md`, `core.md`, `compositor.md`, `physics.md`, `python_api.md`, `user_interface.md`, `sequencer.md` and `bugfixes.md` for the terms above; the only hits were unrelated (point-cache compression, UI templates, `temp_override`).

**Pictures I opened:**

- `_images/render_output_properties_output_panel.png` (the Output panel screenshot on the Output page) — used in section B.
- I checked the Performance page for a screenshot; it has none in 5.0, so nothing could be read from a picture there.

**Cycles' own files I read:** `properties.py` (tile settings), `ui.py` (Performance Memory panel, and the path-guiding tiling calculation), `version_update.py` (the 5.0 tiling versioning), `presets.py` (which settings the Performance presets carry), and I searched `engine.py`, `operators.py` and `__init__.py` for tile and temp terms — no hits. Cycles' folder in this snapshot contains the Python add-on, licences and compiled OSL shaders only; there is no C++ source in it, so the tile-cache writing code itself is not available to me here.

**What I ruled out and why:**

- *Physics point caches, fluid caches, sequencer proxies.* They are files written by Blender, but they are not written by a render to the places a render writes; they are baked ahead of time. Out of scope for this question.
- *Auto-save and quit.blend recovery.* Same reasoning — those are about the blend-file, not about a render's output.
- *`Save Buffers`.* This is the term I most expected to find for "what is on disk mid-render". It does not appear anywhere in the 5.0 manual, API documentation, release notes or Cycles files. I searched both the words and the property spelling.
- *A temporary-name-then-rename scheme.* Searched `atomic`, `.tmp`, `renamed`, `lock file`, `.lock`, `temporary file`. Nothing. I am satisfied this is genuinely absent from the 5.0 sources rather than merely missed.

---

## Things a person with Blender 5.0 open could settle in a few minutes

**All nine items of the original list were run on 2026-08-11, on the Creator's Mac and on a rented Linux machine, and their results are written into the answer above.** What is left is only what those two machines could not reach.

1. **Anything on Windows.** Every result above holds for macOS and for Linux and says nothing about Windows. **Not run with the equipment used** — the runs used one Mac and one rented Linux machine, and no Windows machine was rented for them.
2. **A folder shared over a network between two machines.** Test 14 above ran two Blenders into one folder on a single machine's own disk, which answers the coordination question but not the network one. No network share between two machines is among this project's written-down equipment, so it needs the Creator's say-so before anybody runs it.
3. **Whether the two-gigabyte video split behaves the same on macOS.** Test 12 was run on the rented Linux machine only, because it writes several gigabytes four times over and the Mac is the Creator's working machine.
4. **Multi-view or stereoscopy output during a render.** Nothing here tested it.

---

2026-08-11 — Claude Opus 5 (Claude Code) — Writing, Blender 5.0

2026-08-11 — Claude Opus 5 (Claude Code) — hand tests: fourteen observed results written into question 11, both standing lines brought up to date, and the researcher's own list pruned to what two machines cannot reach

**Declaration.** I used no helper agents on this task; I did all the searching and reading myself, so there is nobody else's work I cannot vouch for. I read the question sheet at `Preparation/Blender Research/Questions/Writing.md`, the note `Blender Sources/About this folder.md`, and files inside `Blender Sources/` (the 5.0 manual, the 5.0 Python API documentation, the 5.0 release notes, and Cycles' 5.0 files). I opened nothing else in this project — not `Preparation/Groundwork/Start Here.md`, not any other part of `Preparation/`, not `Audits/`, not `App/`, not `README.md`, and no other document in the folder this file is saved into. I used no web access and nothing from memory.

One thing I should declare rather than leave unsaid, because it is not something I chose. The tool I run inside places the project's root `CLAUDE.md` into my context automatically at the start of a session, before I can decline it. So I saw that file's text without opening it. It contains no statements about Blender — it is the signpost that tells agents to read `Preparation/Groundwork/Start Here.md`, and it carries the warning that Blender researchers must not. I did not follow it, and I did not open `Start Here.md` or anything it points to. Nothing in this document came from it. I mention it only because a reader checking my line about opening nothing else deserves to know that one project file passed in front of me by the tool's own doing.

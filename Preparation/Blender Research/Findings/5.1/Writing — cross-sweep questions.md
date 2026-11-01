# Writing — cross-sweep questions (Blender 5.1)

Questions answered: 11 and 12 from `Preparation/Blender Research/Questions/Writing.md`.

**Sources consulted, and when.** Blender 5.1 manual (`Blender Sources/manual_5.1`, including one picture from its `_images` folder), Blender 5.1 Python API documentation (`Blender Sources/api_5.1`), Blender 5.1 release notes (`Blender Sources/release_notes_5.1`), and the Cycles add-on's own files (`Blender Sources/cycles_5.1`). All four were taken on 2026-07-30, and that is the date read for every quotation below.

**I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.**

One thing I must declare so the line above is not misleading. Before I was given my instructions, the tool I run inside had already pasted the text of this project's `CLAUDE.md` into my context by itself. I did not open that file and I did not follow it. It is a signpost pointing at `Preparation/Groundwork/Start Here.md`, which I did not open, and it contains no statement about how Blender behaves, so nothing in it could have coloured an answer. Apart from that, the only files I opened are the question sheet, `Blender Sources/About this folder.md`, and files under `Blender Sources/manual_5.1`, `api_5.1`, `release_notes_5.1` and `cycles_5.1`.

---

## 11. While a render is still working, what is present in the places it writes to, and how does that change with the size of the picture?

**The question was ambiguous, and I have answered one reading.** I read "the size of the picture" as the pixel dimensions of the frame being rendered — Resolution X and Y together with the percentage scale. The other reading is the number of bytes a finished frame takes up on disk; I say what the sources give for that reading too, at the end of the answer, and mark it separately.

I also read "the places it writes to" as the places the render is set to write its output. The sources say a render puts things in one other place as well — a temporary directory — and I report that separately rather than folding it in.

### 11a. What is in the output place while an animation of still images is being rendered

**Label: documented.**

Frames are written out one at a time as each is finished, so while the render is working the output place holds the frames finished so far.

> "While rendering stills will allow you to view and save the image from the render buffer when it is complete, animations are a series of images, or frames, and are automatically saved directly out to a drive after being rendered."

Source: the manual, https://docs.blender.org/manual/en/5.1/render/output/animation.html — read 2026-07-30.

The same page says the finished frames stay where they are and can be picked up again:

> "This allows you an easy recovery if there is a problem and you have to re-start the rendering, since the frames you have already rendered will still be in the output directory."

> "Just disable the Overwrite option to start rendering where you left off."

Source: the manual, https://docs.blender.org/manual/en/5.1/render/output/animation.html — read 2026-07-30.

### 11b. Empty files can also be there, for frames that are not finished

**Label: documented** that the setting exists, that it makes empty files, and that it is off by default; **inferred** for when in a frame's life the empty file appears.

*Corrected by the citation check of 2026-08-11: this section was labelled documented throughout. The quoted words say only "Create empty placeholder frames while rendering" and "Create empty placeholder files while rendering frames (similar to Unix 'touch') (default False)". Neither says the file stands for the frame currently being worked on rather than appearing at some other moment, so that part is inferred. The step, written out: the render-farm hint below only shares work between machines if a machine can see before it starts a frame that another has taken it, and the only mark of that is the empty file — so the empty file must be there while the frame is still being rendered. A reader has to supply that, so it is inference, not a documented fact.*

There is a setting that makes Blender put an empty file in place for a frame while that frame is being rendered.

The manual's Output panel page says:

> "Create empty placeholder frames while rendering."

Source: the manual, https://docs.blender.org/manual/en/5.1/render/output/properties/output.html — read 2026-07-30.

A caution about that quotation. On that page the heading above those words reads "Image Sequence – Overwrite Image Multi-Layer EXR", which is the same heading that sits above the entry before it ("Overwrite existing files when rendering."). Two entries carry the same heading, so the heading cannot be telling the truth for both. The picture on that same page shows what controls the panel actually offers: the Output panel shows two tick-boxes side by side on the top row, labelled **Overwrite** (ticked) and **Placeholders** (not ticked), with **File Extensions** (ticked) and **Cache Result** (not ticked) on the row below. Picture looked at: `_images/render_output_properties_output_panel.png`, on https://docs.blender.org/manual/en/5.1/render/output/properties/output.html — read 2026-07-30. Label for what the picture shows: **documented**. That the tick-box named Placeholders is the one the second entry describes is a step a reader supplies from the wording — **inferred** — and the claim above does not rest on it, because the API entry quoted next carries it on its own.

The API documentation gives the same setting more plainly, and gives its default:

> "use_placeholder
>
> Create empty placeholder files while rendering frames (similar to Unix 'touch') (default False)"

and, next to it:

> "use_overwrite
>
> Overwrite existing files while rendering (default True)"

Source: the API documentation, https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html — read 2026-07-30.

So by default this does not happen: placeholders are off unless somebody turns them on. When they are on, the output place holds, while the render works, the finished frames plus an empty file standing in for a frame that is not finished.

The manual states outright what the two settings together are for:

> "Primitive Render Farm
>
> An easy way to get multiple machines to share the rendering workload is to:
>
> Set up a shared directory over a network file system.
>
> Disable Overwrite, enable Placeholders in the Render Output panel.
>
> Start as many machines as you wish rendering to that directory."

Source: the manual, https://docs.blender.org/manual/en/5.1/render/output/properties/output.html — read 2026-07-30.

### 11c. A single still render puts nothing there at all until it is asked to

**Label: inferred.**

*Corrected by the citation check of 2026-08-11: this was labelled documented. The quoted words say a still may be viewed and saved from the render buffer, and that there is a switch which saves it to the output path. They nowhere say that nothing reaches the output place otherwise. The step, written out: the same sentence sets stills against animations, which "are automatically saved directly out to a drive after being rendered", and a reader has to take that contrast to mean the still is not written by the render itself. The 5.0 answer and this topic's own second pass both draw that contrast and both call it inference.*

For a single frame, the result is held and is not written on its own.

> "While rendering stills will allow you to view and save the image from the render buffer when it is complete…"

Source: the manual, https://docs.blender.org/manual/en/5.1/render/output/animation.html — read 2026-07-30.

The write has to be asked for. The render command carries a switch for it:

> "write_still (bool) – Write Image, Save the rendered image to the output path (used only when animation is disabled) (optional)"

Source: the API documentation, https://docs.blender.org/api/5.1/bpy.ops.render.html — read 2026-07-30.

### 11d. If the output is a video, there is one file and it is being written into

**Label: documented** that Blender makes one file holding all the frames, that it may be left unplayable, and that Autosplit adds a second file past 2GiB; **inferred** that the file is already on disk and being written into while the render runs.

*Corrected by the citation check of 2026-08-11: this section was labelled documented throughout. The step, written out: for a problem occurring "while rendering" to leave a file unplayable, the file must already exist and already hold something before the render is done. The sources do not say that outright. The 4.5 and 5.0 answers and this topic's own second pass all mark the same step inferred.*

> "The Direct Approach, which is highly not recommended and not a standard practice, is where you set your output format to an AVI or MOV format, and click Animation to render your scene directly out to a movie file. Blender creates one file that holds all the frames of your animation."

Source: the manual, https://docs.blender.org/manual/en/5.1/render/output/animation.html — read 2026-07-30.

That file is in a state that may not survive an interruption:

> "If a problem occurs while rendering, the file might become unplayable and you will have to re-render all frames from the beginning."

Source: the manual, https://docs.blender.org/manual/en/5.1/render/output/properties/output.html — read 2026-07-30.

And a second file can appear part-way through, once the first passes a size:

> "If your video is huge and exceeds 2GiB, enable Autosplit Output. This will automatically split the output into multiple files after the first file is 2GiB in size."

Source: the manual, https://docs.blender.org/manual/en/5.1/render/output/properties/output.html — read 2026-07-30.

### 11e. How this changes with the size of the picture

**Nothing in the sources ties what is in the output place to the size of the picture.** Everything in 11a to 11d is described without any mention of picture size. No list of search terms was kept for that, so this silence is not evidenced here.

**Label: documented, for the one thing the sources do tie to picture size.**

The only place the sources connect the size of the picture to something being put on disk during a render is Cycles' tiling — and where that lands is not stated, as set out further down:

> "Tile Size
>
> Render high resolution images in tiles of this size, to reduce memory usage. Tiles are cached to disk while rendering to save memory."

Source: the manual, https://docs.blender.org/manual/en/5.1/render/cycles/render_settings/performance.html — read 2026-07-30.

**Label: observed** — the same words, and the numbers behind them, are in Cycles' own files:

> ```
>     use_auto_tile: BoolProperty(
>         name="Auto Tile",
>         description="Deprecated, tiling is always enabled",
>         default=True,
>     )
>     tile_size: IntProperty(
>         name="Tile Size",
>         default=2048,
>         description="Render high resolution images in tiles of this size, to reduce memory usage. Tiles are cached to disk while rendering to save memory",
>         min=8,
>         max=8192,
>     )
> ```

File: `Blender Sources/cycles_5.1/properties.py`, lines 1051–1062 — read 2026-07-30.

So there is a setting called Tile Size, it is a whole number, its default is 2048, its smallest allowed value is 8 and its largest is 8192; and the switch that used to turn tiling on and off is marked deprecated with the note that tiling is always on.

**Label: inferred** — a picture bigger than one tile is rendered as more than one tile, and the crossing point is the tile's area against the picture's area. The step I took: Cycles' own interface file works out that comparison, and its comment says the working-out is the same one the renderer itself uses.

> ```
>             # Calculation matches TileManager::compute_render_tile_size and
>             # Session::get_effective_tile_size
>             if cscene.tile_size < 128:
>                 tile_size = cscene.tile_size
>             else:
>                 tile_size = (cscene.tile_size + 128 - 1) & ~(128 - 1)
>                 tile_size = min(tile_size, 8192)
>             tile_area = tile_size ** 2
>
>             render_scale = scene.render.resolution_percentage / 100.0
>             render_size_x = int(scene.render.resolution_x * render_scale)
>             render_size_y = int(scene.render.resolution_y * render_scale)
>             render_area = render_size_x * render_size_y
>
>             if render_area > tile_area and render_size_x <= 8192 and render_size_y <= 8192:
>                 layout.label(text="May work poorly with render tiling", icon='INFO')
> ```

File: `Blender Sources/cycles_5.1/ui.py`, lines 390–405 — read 2026-07-30.

Read plainly: the picture's area is Resolution X times Resolution Y, each first multiplied by the percentage scale; the tile's area is the effective tile size squared, where the effective tile size is the setting itself if it is under 128, and otherwise the setting rounded up to a multiple of 128 and capped at 8192. With the default of 2048, the effective tile is 2048 and its area is 4,194,304 pixels — so a picture with more pixels than that is more than one tile. I have marked this inferred because those lines exist to decide whether to show a warning — the one they end in reads "May work poorly with render tiling" — not to decide how the picture is cut up; the claim rests on the comment saying the working-out matches the renderer's own.

Cycles also carries a note that older files are brought forward to always-on tiling:

**Label: observed.**

> ```
>             # Auto tiling is always enabled now
>             if version <= (5, 0, 77):
>                 cscene = scene.cycles
>                 if not cscene.use_auto_tile:
>                     cscene.use_auto_tile = True
>                     cscene.tile_size = 8192
> ```

File: `Blender Sources/cycles_5.1/version_update.py`, lines 114–119 — read 2026-07-30.

**Where those cached tiles are put is not stated.** The nearest the sources come is a general statement about a temporary directory:

> "The temporary directory is used to store various files at run-time (including render layers, physics cache, copy-paste buffer and crash logs)."

Source: the manual, https://docs.blender.org/manual/en/5.1/advanced/blender_directory_layout.html — read 2026-07-30.

That says render layers go to the temporary directory at run time. It does not say the cached tiles are the same thing, and I have not treated it as saying so.

**Label: inferred** — an empty placeholder file does not get bigger with a bigger picture. The step: the API documentation calls the placeholder file "empty" and likens making it to Unix `touch`; a file with no content in it has no pixels in it to grow.

**The other reading of "size of the picture". Label: not in the sources — with no record of the search.** If the question means the number of bytes a finished frame takes, then what the sources give is only the video case above: the accumulated file is split once it passes 2GiB, and until then it is one growing file. Nothing was found in the 5.1 manual, API documentation, release notes or Cycles files about how big a still-image frame is while it is being written, or whether it appears on disk part-written. No list of search terms was kept for that, so this silence is not evidenced here.

### 11f. Not covered

This answer does not settle:

- Whether a frame's own file appears on disk in a part-written state while that frame is being written, or only appears whole. Nothing was found on it, but no list of search terms was kept for this, so the silence is not evidenced here. The search record further down belongs to question 12 — making a folder and failing to — and does not cover this.
- Where the cached tiles go — which folder, under what names, and whether they are cleaned up.
- Whether EEVEE or Workbench put anything on disk during a render. Everything above about tiles is Cycles only, and question 11 names no render engine, so this part of it is unanswered.
- What the compositor's File Output node puts in its folders while a render is working. Those folders are places a render writes to, so this is part of question 11 and it is unanswered here. It was set aside as belonging to another question on the sheet, and no other question on the sheet asks it.
- What becomes of the held result a still render offers for viewing — the render buffer — when Blender is driven from outside its own window.
- Whether more than one placeholder can be present at once, and whether a placeholder is removed if the render is stopped before that frame finishes.
- Whether anything changes when several machines write to the same shared folder at once, beyond the manual's statement that this is what the two settings are for.

### 11f-bis. What a running Blender 5.1 showed — hand tests, 2026-08-11

**LABEL: observed.** Every result below was produced on 2026-08-11 by running Blender 5.1.2 on two computers and writing down what it printed and what appeared on disk. The two computers are named in full because that is what settles which operating system each answer covers.

- **The Creator's Mac** — a MacBook Pro with an Apple M1 Pro chip, 16 GB of memory, macOS 26.5.2 (build 25F84), Apple's own graphics and no second card. Blender 5.1.2 at `/Applications/Blender 5.1.2.app/Contents/MacOS/Blender`.
- **A rented Linux machine** — Ubuntu 22.04.5 LTS on kernel 6.8, two NVIDIA RTX 4000 Ada Generation cards of 20 GB each on driver 580.105.08, an AMD Ryzen Threadripper PRO 3945WX with eleven cores, 49 GB of memory. Blender 5.1.2 at `/opt/blender/5.1/blender`.

Every run used `-b --factory-startup` unless it says otherwise, and each was pointed at a settings folder of its own so that no run touched anybody's saved Blender settings. Where the same test ran on both machines, both results are given and it is said plainly whether they agreed. The whole list, with the exact commands, is `Audits/Blender Research/Writing/Hand tests — cross-sweep questions.md`.

**1. Section 11c is right: a still render writes nothing at the output path unless it is asked to, with a window or without.** Output path set to an empty folder, PNG, 320 × 240, Cycles:

```
BEFORE: []
AFTER render, no save switch: []
AFTER render with the save switch: ['still_.png']
```

`blender -b --factory-startup -o <folder>/cli_ -F PNG -f 1` wrote `cli_0001.png`. The three runs repeated in a Blender **with a window open on screen** printed the same three lines. Both machines agreed.

**2. Nothing stands for a frame while it is being rendered; while it is being *saved*, its file is there part-written.** Cycles, 1920 × 1080, 400 samples, frames 1 to 4, PNG, Placeholders off, watched every fifty milliseconds. On the rented Linux machine:

```
   0.00  .../out  []
  10.25  .../out  [('f_0001.png', 0)]
  10.30  .../out  [('f_0001.png', 712704)]
  10.35  .../out  [('f_0001.png', 1433600)]
  10.40  .../out  [('f_0001.png', 1598617)]
  20.05  .../out  [('f_0001.png', 1598617), ('f_0002.png', 0)]
```

For the ten seconds a frame renders there is nothing for it at all; for about a fifth of a second while it is written the file is there and growing. No file under another name appears first, and nothing is renamed. **This settles the first item under 11f, which had no evidence behind it.** The Mac agreed, and at 8192 × 4608 the same thing took about two seconds.

**3. With Placeholders on, the empty file appears before the frame is rendered.** Placeholders on, Overwrite off. On the rented Linux machine:

```
   0.00  .../out  []
   0.35  .../out  [('f_0001.png', 0)]
  10.15  .../out  [('f_0001.png', 40960)]
  10.20  .../out  [('f_0001.png', 753664)]
  10.25  .../out  [('f_0001.png', 1482752)]
  10.30  .../out  [('f_0001.png', 1598617), ('f_0002.png', 0)]
```

Three tenths of a second in, ten seconds before frame 1 is finished. The one for frame 2 appears the moment frame 1's picture is complete, so they are made one at a time, for the frame in hand — **only one placeholder is present at once**, which 11f leaves open — and the picture is written into the same file rather than a second one. The Mac agreed.

**4. A render stopped part way leaves the empty file behind.** Same as test 3, frames 1 to 6, stopped with the ordinary terminate signal once frame 3's placeholder had appeared. Blender exited 143 and the folder held two finished frames and `f_0003.png` at 0 bytes. **This settles the other half of that item in 11f.** Both machines agreed.

**5. Blender holds the file open while it writes it.** During a 10000 × 10000 multi-layer OpenEXR write, the system reported Blender holding that file with a write handle. Renaming it at that moment succeeded and the render still finished. Both machines agreed.

**6. Where the cached tiles go — the gap 11e names.** Cycles, 32 samples, tile size at its default 2048, one frame at 8000 × 8000, with every file the process had open sampled twice a second. On the rented Linux machine:

```
blender 38829 root 3u REG 253,1    118784 /tmp/blender_MnaQp0/cycles-tile-buffer-38829-134997896366912-0-0.exr
blender 38829 root 3u REG 253,1  84815872 /tmp/blender_MnaQp0/cycles-tile-buffer-38829-134997896366912-0-0.exr
blender 38829 root 3r REG 253,1  89547247 /tmp/blender_MnaQp0/cycles-tile-buffer-38829-134997896366912-0-0.exr
```

On the Mac the same file sat under `/var/folders/…/T/blender_XXXXXX/`. **One file, an OpenEXR, in Blender's own temporary folder, named `cycles-tile-buffer-<process number>-<number>-0-0.exr`, about 89 MB for that frame, and gone before Blender exits.** The same render at 1000 × 1000 opened no such file at all. So the two things 11e is careful not to join — the tile cache and the "render layers" the manual puts in the temporary directory — turn out to share a folder but not a name: the tile cache is a file of its own.

**7. The boundary is one tile's worth of pixels, exactly as inferred.** Tile size 2048, one frame, with the system's temporary folder pointed at an empty folder of the run's own and walked every twenty milliseconds. At **2048 × 2048** nothing appeared. At **2048 × 2049**, on the Mac:

```
   0.00  []
   5.39  [('.../tmp/blender_I8A972/cycles-tile-buffer-72831-40589232864-0-0.exr', 4096)]
   5.45  [('.../tmp/blender_I8A972/cycles-tile-buffer-72831-40589232864-0-0.exr', 913408)]
  12.68  []
```

Both machines agreed. At 8000 × 8000 with the tile size raised to **8192** — larger in area than the picture — there was no such file at all; at **256** there was one, written to far more often. **Setting `use_auto_tile` to False changes nothing**: the file still appeared, on both machines. (In Blender 4.5 that switch does turn it off. That is a difference between versions.)

**8. EEVEE and Workbench write no such file.** The same 8000 × 8000 frame under `BLENDER_EEVEE` and under `BLENDER_WORKBENCH` opened no tile-buffer file and put nothing in the temporary folder, on either machine. **This settles the third item under 11f.**

**9. What a File Output node's folder holds while a render works — the fourth item under 11f.** Compositor on, one node aimed at a folder that did not exist, separate from the scene's own; Cycles, 1920 × 1080, frames 1 to 4. `node folder exists before the render: False`, and then, frame by frame:

```
Saved: '.../work/e12/5.1/node/n_Image0001.png'
Saved: '.../work/e12/5.1/out/f_0001.png'
```

The node's folder is made by the render, and its file for a frame lands just before the scene's own file for that frame. Both machines agreed.

**10. With Cache Result on, a cache file is written for every frame, before that frame's picture.** Two view layers, Cycles, 1920 × 1080, frames 1 to 3, the render cache folder pointed at an empty folder. Blender printed, once per frame, `Caching exr file, 1920x1080, .../cache/cached_RR_UNSAVED_Scene_00000000000000000000000000000000.exr`, and the watcher showed that file reaching about 133 MB before the frame's own PNG appeared, then dropping to zero and growing again for the next frame. One file, rewritten per frame, still there at the end. Both machines agreed.

**11. A video render puts one file in the folder from the start; stopping it leaves something that will not play.** Workbench, 1280 × 720, FFmpeg in a Matroska container with H.264, frames 1 to 4000, stopped after 25 seconds. `movie_0001-4000.mkv` — named for the whole frame range from the first moment — appeared at 0 bytes within a second and grew in steps of 262,144 bytes. Handing the file left behind back to Blender 5.1 gave `loaded as a movie: movie_0001-4000.mkv  frames: 0  size: (256, 256)`. Both machines agreed. **This test was run twice**: a first run of 200 frames finished before it could be stopped, and showed the finished file opening properly.

**12. Autosplit: the second file starts at about 1.87 GiB, not 2 — and this is the other reading of the question, which 11e leaves unevidenced.** Workbench, 1920 × 1080, FFmpeg in an AVI container with the lossless HUFFYUV codec, 1400 frames. **Run on the rented Linux machine only** — it writes several gigabytes four times over and the Mac is the Creator's working machine — so it settles Linux and leaves macOS open. With Autosplit on:

```
FINAL: [('long__0000001-1400.avi', 2003090368), ('long__0010001-1400.avi', 1022075648)]
```

With it off, one file of 3,025,156,392 bytes. The names gain an extra block of digits when the option is on. **Also run twice**: a first run of 900 frames reached 1,945,065,136 bytes and never split.

**13. No texture cache is written beside the pictures a render reads.** A PNG written by Blender itself, used as an image texture, rendered with Cycles at 4096 × 4096: the folder holding it contained only that PNG before and after. Asking Blender 5.1 what settings it has for this printed `no such setting: use_texture_cache` and `no such setting: use_auto_generate_texture_cache`. Both machines agreed. (Blender 5.2 has both switches and, with them on, writes a `blender_tx/` folder beside the picture. 5.1 has neither.)

**14. Two Blenders writing into one folder divide the work — the last item under 11f.** **Run on the rented Linux machine only**, and the one test here whose whole subject is two Blenders at once. Placeholders on, Overwrite off, frames 1 to 8, the second started three seconds after the first. One took the odd frames and the other the even ones, each printing `skipping existing frame "<path>"` for the frames the other had claimed. No frame was rendered twice. This was a folder on one machine's own disk; a folder shared over a network was not tested.

**What these tests leave open.** Windows, everywhere — no Windows machine has run it. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.** A folder shared over a network. Whether the two-gigabyte split behaves the same on macOS. Multi-view or stereoscopy output. And what becomes of the held result a still render offers for viewing when Blender has no window, which no test here reached.

### 11g. Outside the interface

It holds. What changes from outside is only how the write is asked for.

A single still render writes nothing unless the write is asked for, and that is as true from outside as inside. From Python the asking is a switch on the render command, `write_still` (quoted above). From the command line the manual gives an instruction that renders one frame and saves it in the same step:

> "-f, --render-frame <frame>
>
> Render frame <frame> and save it."

and one for a range of frames:

> "-a, --render-anim
>
> Render frames from start to end (inclusive)."

Source: the manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html — read 2026-07-30. The second of those says nothing about saving; what an animation render saves is covered in 11a.

Everything else above is a scene setting, not an interface behaviour: Overwrite, Placeholders and Tile Size are all properties readable and settable without the window (`bpy.types.RenderSettings.use_overwrite`, `use_placeholder`, and Cycles' `tile_size`), so nothing in them depends on a person clicking. The render engine documentation also states plainly that saving during an animation is the engine's own job, not the window's:

> "bl_use_image_save
>
> Save images/movie to disk while rendering an animation. Disabling image saving is only supported when bl_use_postprocess is also disabled. (default True)"

Source: the API documentation, https://docs.blender.org/api/5.1/bpy.types.RenderEngine.html — read 2026-07-30.

Beyond that, the sources do not say whether anything about what lands on disk differs when Blender is driven from outside its own window.

**Settled by the hand tests of 2026-08-11.** It holds. All fourteen tests above were run with no window at all, so every one of them is established for a Blender driven from outside. Test 1 was then repeated in a Blender **with a window open on screen**, on both machines, and printed the same three lines — the still render wrote nothing until it was asked to, exactly as with no window. Nothing was found that a window changes.

### 11h. Operating system

The sources name no operating system limit on any of it. Overwrite, Placeholders and Tile Size are described without qualification. The API's phrase "similar to Unix 'touch'" is a comparison drawn to explain what an empty file is, not a statement that the setting is Unix-only.

One thing does differ by operating system: the temporary directory, which is where the manual says render layers are stored at run time.

> "The temporary directory is selected based on the following priority:
>
> User Preference (see File Paths).
>
> Environment variables (TEMP on Windows, TMP & TMP_DIR on other platforms).
>
> The /tmp/ directory."

Source: the manual, https://docs.blender.org/manual/en/5.1/advanced/blender_directory_layout.html — read 2026-07-30.

The command-line page names the variables slightly differently:

> "TEMP:
>
> Store temporary files here (MS-Windows).
>
> TMPDIR:
>
> Store temporary files here (UNIX Systems). The path must reference an existing directory or it will be ignored."

Source: the manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html — read 2026-07-30.

So the *place* differs by operating system. Whether what is put there differs, the sources do not say.

**Settled by the hand tests of 2026-08-11.** Tests 1 to 11 and 13 above were each run on macOS on an Apple M1 Pro and on Linux on an x86 machine, and the two machines agreed on every one of them, so none of this is a macOS-only or Linux-only behaviour. What is put in the temporary directory is the same on both: the very same `cycles-tile-buffer-…-0-0.exr` file sat under `/var/folders/…/T/` on macOS and under `/tmp/` on Linux. Nothing about **Windows** was tested — no Windows machine has run it — so every claim here leaves Windows open. Tests 12 and 14 were run on Linux only and leave macOS open; the reason is written into each.

---

## 12. What does a render do when it must make a folder and cannot?

**Label: not in the sources.**

The Blender 5.1 manual, the Blender 5.1 Python API documentation, the Blender 5.1 release notes and the Cycles add-on's own files say nothing about a render creating a folder that is not there, and therefore nothing about what happens when it tries and fails. They do not say whether it stops, carries on, writes an error, skips the frame, or leaves the render running with nothing landing.

They do not even settle the step before it — whether a render makes a missing folder at all. Nothing in any of the four sources says it does or does not.

### 12a. Where I looked, and what I ruled out

Searched across all four sources for the act of making a folder: `mkdir`, `makedirs`, `create directory`, `create the directory`, `creates the directory`, `create folder`, `creating folder`, `creating directory`, `create subdirectory`, `subdirectory`, `missing directory`, `directory does not exist`, `does not exist`, `no such`. The only hits anywhere were about unrelated things — a bundle node path, a vertex group, an add-on's own install path — and none about render output.

Searched for the failure: `cannot create`, `can't create`, `unable to create`, `failed to write`, `cannot write`, `unable to write`, `permission denied`, `permission`, `not writable`, `writable`, `write access`, `read-only directory`, `write protect`, `disk full`, `out of disk`, `no space`. Nothing about render output in any of the four sources. The `permission` hits in the manual are three, and none is about rendering: extension manifests, running an external text editor as administrator on Windows, and one line of startup troubleshooting — "Ensure that you have appropriate user permissions on the computer."

*Corrected by the citation check of 2026-08-11: this said "the only `permission` hits in the manual are about extension manifests, and about running an external text editor as administrator on Windows". There is a third, at `manual_5.1/troubleshooting/startup.txt` line 12, quoted above. It is about Blender failing to start, so it does not change the answer, but the record was not true as written.*

Searched for what a failure would look like: `error message`, `Cannot save`, `Could not`, `Unable to`, `render fails`, `fails with`, `abort`, `aborts`, `exit code`, `exit status`, `return code`, `non-zero`. Nothing on render output failure. The command-line page has `--python-exit-code <code>`, described as "Set the exit-code in [0..255] to exit if a Python exception is raised" — that is about Python exceptions, not about a render that cannot write, so it does not answer this.

Pages I opened in full and ruled out, because they are where such a statement would most naturally sit and it is not in any of them:

- Output panel — https://docs.blender.org/manual/en/5.1/render/output/properties/output.html
- Rendering Animations — https://docs.blender.org/manual/en/5.1/render/output/animation.html
- Output introduction — https://docs.blender.org/manual/en/5.1/render/output/introduction.html
- Format panel — https://docs.blender.org/manual/en/5.1/render/output/properties/format.html
- File Paths (path templates and relative paths) — https://docs.blender.org/manual/en/5.1/files/file_paths.html
- Preferences ▸ File Paths — https://docs.blender.org/manual/en/5.1/editors/preferences/file_paths.html
- Blender's Directory Layout — https://docs.blender.org/manual/en/5.1/advanced/blender_directory_layout.html
- Command Line Arguments — https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html
- `bpy.types.RenderSettings` — https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html
- `bpy.ops.render` — https://docs.blender.org/api/5.1/bpy.ops.render.html
- `bpy.types.RenderEngine` — https://docs.blender.org/api/5.1/bpy.types.RenderEngine.html
- The whole of the 5.1 release notes, subject by subject, including the fixed-bugs list.
- Cycles' own files: every one of its Python files (`properties.py`, `ui.py`, `engine.py`, `operators.py`, `osl.py`, `__init__.py`, `camera.py`, `presets.py`, `version_update.py`) searched for `tile`, `temp`, `tmp`, `mkdir`, `makedirs`, `output` and `write`, and `properties.py`, `ui.py` and `version_update.py` read at the places that hit. None of them touches output folders. The only folder-making anywhere in them is in `osl.py`, which uses Python's own temporary-file machinery to compile a shader, and has nothing to do with render output.

The `bpy.ops.render.render` entry in the API documentation, which is the one place an answer would be expected, says only:

> "Undocumented, consider contributing."

Source: the API documentation, https://docs.blender.org/api/5.1/bpy.ops.render.html — read 2026-07-30.

### 12b. Two documented things nearby, which are not the answer

I record these because they are the closest the sources come, and because a reader might otherwise mistake one of them for an answer. Neither is about a render making an output folder.

One. When Blender is told to use a temporary-files folder that is not there, it does not make it — it ignores the setting and falls back.

> "Temporary Files
>
> The directory for storing temporary save files. The path must reference an existing directory or it will be ignored and the systems temporary directory will be used instead. When left blank, the systems temporary directory will be used (see Temporary Directory for details)."

Source: the manual, https://docs.blender.org/manual/en/5.1/editors/preferences/file_paths.html — read 2026-07-30. **Label: documented** — for the temporary-files preference only. It says nothing about the render output path.

Two. A render *can* fail on the output path, but for a different reason — a malformed path template, not a folder that cannot be made.

> "Depending on the path, template errors may prevent certain actions. For example, if there are errors in the render output path, then rendering an animation will fail with an error message indicating the path errors."

Source: the manual, https://docs.blender.org/manual/en/5.1/files/file_paths.html — read 2026-07-30. **Label: documented** — for template errors only. It is the one documented case of a render failing because of its output path, and it is not this case.

### 12c. Not covered

This answer does not settle anything about the behaviour itself. In particular it leaves open: whether a render attempts to make a missing folder; whether it makes only the last level or a whole chain of missing levels; whether a failure stops the whole animation or only loses one frame; whether the render carries on producing nothing; what message, if any, appears and where; and whether the failure is reported in a way another program could detect.

*Every one of those has since been settled by hand tests, immediately below.*

### 12c-bis. What a running Blender 5.1 showed — hand tests, 2026-08-11

**LABEL: observed.** Run on 2026-08-11 on the two computers described under question 11 above — the Creator's MacBook Pro with an Apple M1 Pro chip on macOS 26.5.2, and a rented Ubuntu 22.04.5 machine on kernel 6.8 — with Blender 5.1.2, `-b --factory-startup`, each run pointed at a settings folder of its own. On the Linux machine the runs were made by an ordinary account rather than root, because root is not stopped by an unwritable folder. Every scene was Workbench at 320 × 240, frames 1 to 3, PNG. The exact commands are in `Audits/Blender Research/Writing/Hand tests — cross-sweep questions.md`.

**15. A missing output folder is made — one level or a whole chain.** With the output path set to `<a writable folder>/newfolder/f_`:

```
operator returned: {'FINISHED'}
does the folder exist now: True
folder holds: ['f_0001.png', 'f_0002.png', 'f_0003.png']
```

With three missing levels at once, `<a writable folder>/a/b/c/f_`, the same. Exit code 0. Both machines agreed. **So a render does make missing folders, and it makes the whole chain, which this answer could not settle either way.**

**16. When the folder cannot be made, the render stops at the very first frame, says so in two lines, and ends with 1.** The parent folder was made and then set to read-and-enter only, and the output path named a folder inside it that did not exist. On the **Mac**:

```
does the folder exist before: False
00:00.469  image            | ERROR Couldn't create directory for file .../work/e18b/5.1/locked/newfolder/f_0001.png: Permission denied
Error: Render error (Permission denied) cannot save: '.../work/e18b/5.1/locked/newfolder/f_0001.png'
operator raised: RuntimeError Error: Render error (Permission denied) cannot save: '.../work/e18b/5.1/locked/newfolder/f_0001.png'
does the folder exist now: False
```

with `exit code: 1`. Frames 2 and 3 were never attempted, and nothing at all was left behind. Three missing levels gave the same. The **rented Linux machine** printed the identical two lines with its own paths, and behaved identically.

**Blender 5.1 names the folder in the message and gives the real reason. Blender 4.5 does neither** — for the same set-up it prints only `Error: Render error (No such file or directory) cannot save: …` on macOS and `(Invalid argument)` on Linux, with no mention of a folder. That is a difference between versions.

**17. A file standing where the folder must go gives a different reason.** An ordinary file called `blocker` was made and the output path set to `<that file>/f_`:

```
ERROR Couldn't create directory for file .../work/e19b/5.1/blocker/f_0001.png: File exists
Error: Render error (File exists) cannot save: '.../work/e19b/5.1/blocker/f_0001.png'
```

Exit code 1, nothing written, render stopped at the first frame. Both machines agreed.

**18. A File Output node that cannot make its folder does *not* stop the render, and the exit code stays 0.** A File Output node aimed at a folder inside the read-only parent, the scene's own output path pointing somewhere that worked:

```
ERROR Couldn't create directory for file .../work/e20b/5.1/locked/nodeout/n_Image0001.png: Permission denied
ERROR Couldn't create directory for file .../work/e20b/5.1/locked/nodeout/n_Image0002.png: Permission denied
ERROR Couldn't create directory for file .../work/e20b/5.1/locked/nodeout/n_Image0003.png: Permission denied
operator returned: {'FINISHED'}
```

with `exit code: 0`, and all three of the scene's own frames written. Both machines agreed. **So the same failure stops the render when it is the scene's own path and does not stop it when it is a node's, and in the second case nothing outside Blender can tell from the exit code that anything went wrong.**

**19. A folder asked for by the name of a node's input is made, and behaves the same way when it cannot be.** The node's input was named `passes/diffuse`. Into a writable folder, Blender 5.1 made a real folder, taking the node's own file name onto the front of it:

```
.../nodeopen/n_passes/diffuse0001.png
.../nodeopen/n_passes/diffuse0002.png
.../nodeopen/n_passes/diffuse0003.png
```

Into a folder that could not be written to, three `ERROR Couldn't create directory for file …: Permission denied` lines and the same `operator returned: {'FINISHED'}` with exit code 0. Both machines agreed. (In Blender 5.0 the slash becomes an underscore and no folder is made at all. That is a difference between versions.)

**20. The render cache folder is made when missing, and when it cannot be made the render carries on and finishes.** With Cache Result on and the render cache folder pointed at a folder that did not exist under a writable parent, the folder and the cache file were both made. Pointed inside the read-only parent, the render wrote all three of its frames and printed, per frame:

```
ERROR IMB_exr_begin_write: No such file or directory.
ERROR Error writing render result, No such file or directory (see console)
```

`operator returned: {'FINISHED'}`, exit code 0, and the cache folder was never made. Both machines agreed.

> **A second run disagrees with the first of those two lines, and this is not settled.** The citation check of 2026-08-11 repeated this test on the Creator's Mac with Blender 5.1.2, same set-up. What came back was:
>
> ```
> 00:00.400  image.openexr    | ERROR Unknown error in IMB_exr_begin_write
> 00:00.400  reports          | ERROR Error writing render result, No such file or directory (see console)
> ```
>
> The second line matches. The first does not: where the run above has `IMB_exr_begin_write: No such file or directory.`, that run has `Unknown error in IMB_exr_begin_write` — which is the wording 5.2's own answer records for the Mac and sets against Linux's. The same repeat on the rented Linux machine gave `ERROR IMB_exr_begin_write: No such file or directory.`, the wording above. So in that pair of runs the two machines did **not** agree on this line. The claim that they did is the part in doubt. Neither pair of runs is preferred over the other. The test goes back to the hand-test list as entry 26, to be run again on both machines with the wording recorded per machine. Everything else about this result — the folder made under a writable parent, all three frames written, `{'FINISHED'}`, exit code 0, the cache folder never made — reproduced exactly.

**21. With a window open on screen, the terminal says exactly the same thing.** The set-up of test 16 was run again in a Blender with a real window, on both machines, and printed the same two lines and raised the same error to Python. Blender was then asked to take a picture of its own window. **What the status bar and the Info list show could not be settled**: the picture shows Blender's first-run Quick Setup panel sitting over the middle of the window and an empty Info list behind it, and dismissing that panel needs a click, which nothing in this round does. That half of the test is dropped for want of a hand on a mouse. The terminal half is complete and is the answer above.

---

### 12c-ter. The two disputed wordings, run again — hand tests, 2026-08-12

**LABEL: observed.** Entries **25** and **26** of `Audits/Blender Research/Writing/Hand tests — cross-sweep questions.md` were run on 2026-08-12. Entry 26 exists because of the disagreement recorded in the block under test 20 above; entry 25 exists for the same reason in Blender 5.0. Both were run on **both** computers:

- **The Creator's Mac** — a MacBook Pro with an Apple M1 Pro chip, 16 GB of memory, macOS 26.5.2, Apple's own graphics and no second card.
- **A rented Linux machine** — Ubuntu 22.04.5 LTS on kernel 6.8, two NVIDIA RTX 4000 Ada Generation cards of 20 GB each on driver 580.105.08, an AMD Ryzen Threadripper PRO 3945WX with eleven cores allotted, 49 GB of memory.

Blender 5.1.2 on both, started with `-b --factory-startup`, each run pointed at a settings folder of its own that was made before Blender was started. On the Linux machine the runs were made by an ordinary account called `smoozy`, not root. Only one Blender ran at a time on each machine. Blender's whole terminal output was kept, unfiltered. **The words are written down per machine and the two machines are not merged into one answer.** In the blocks below only the leading part of each long path is shortened to `...`; every word of every message is as the machine printed it.

**22. (Entry 26.) Test 20's failing run again — and the two machines print different words.** Cache Result on, the render cache folder pointed at a folder that does not exist under a parent set to read-and-enter only. Entry 26 was run twice per version, because its own written set-up says Workbench at 320 × 240 while the run it points back at, test 20, was Cycles at 1920 × 1080 with 400 samples and two view layers. Neither was preferred: run A repeated test 20's own set-up and run B used the set-up entry 26 itself is written with. **The added step was the second run; nothing else was changed.** Both runs gave the same words on each machine.

On the **Creator's Mac**, run A (Cycles, 1920 × 1080, 400 samples) printed:

```
VERSION 5.1.2
use_render_cache = True
render_cache_directory = '.../work/e26a/5.1/locked/cachefolder'
view layers: ['ViewLayer', 'SecondLayer']
cache folder exists before the render: False
00:19.536  image.openexr    | ERROR Unknown error in IMB_exr_begin_write
00:19.536  reports          | ERROR Error writing render result, No such file or directory (see console)
Blender 5.1.2 (hash ec6e62d40fa9 built 2026-05-19 02:18:37)
Caching exr file, 1920x1080, .../work/e26a/5.1/locked/cachefolder/cached_RR_UNSAVED_Scene_00000000000000000000000000000000.exr
00:19.709  render           | Saved: '.../work/e26a/5.1/out/f_0001.png'
```

— the same lines again for frames 2 and 3 — and ended:

```
operator returned: {'FINISHED'}
SCENE FOLDER FINAL: ['f_0001.png', 'f_0002.png', 'f_0003.png']
CACHE FOLDER FINAL: not there
```

with `exit code: 0`. Run B on the Mac (Workbench, 320 × 240) printed the identical two error lines:

```
00:00.468  image.openexr    | ERROR Unknown error in IMB_exr_begin_write
00:00.468  reports          | ERROR Error writing render result, No such file or directory (see console)
```

On the **rented Linux machine**, run A printed:

```
VERSION 5.1.2
use_render_cache = True
render_cache_directory = '/home/smoozy/crosssweep/work/e26a/5.1/locked/cachefolder'
view layers: ['ViewLayer', 'SecondLayer']
cache folder exists before the render: False
00:19.551  image.openexr    | ERROR IMB_exr_begin_write: No such file or directory.
00:19.551  reports          | ERROR Error writing render result, No such file or directory (see console)
Blender 5.1.2 (hash ec6e62d40fa9 built 2026-05-19 02:18:37)
Caching exr file, 1920x1080, /home/smoozy/crosssweep/work/e26a/5.1/locked/cachefolder/cached_RR_UNSAVED_Scene_00000000000000000000000000000000.exr
00:19.816  render           | Saved: '/home/smoozy/crosssweep/work/e26a/5.1/out/f_0001.png'
```

— the same lines again for frames 2 and 3 — and ended with the same three closing lines and `exit code: 0`. Run B on Linux (Workbench, 320 × 240) printed the identical two error lines:

```
00:00.651  image.openexr    | ERROR IMB_exr_begin_write: No such file or directory.
00:00.651  reports          | ERROR Error writing render result, No such file or directory (see console)
```

**The two machines did not print the same words.** On the first of the two lines the **Mac said `ERROR Unknown error in IMB_exr_begin_write`** and the **rented Linux machine said `ERROR IMB_exr_begin_write: No such file or directory.`**, in both runs on each machine. The second line was identical on both: `ERROR Error writing render result, No such file or directory (see console)`. Everything else matched: the render carried on and wrote all three of its frames, the cache folder was never made, `operator returned: {'FINISHED'}`, exit code 0.

> **What this leaves standing.** Test 20 above records the first line as `IMB_exr_begin_write: No such file or directory.` on both machines and says both agreed. The citation check of 2026-08-11 got `Unknown error in IMB_exr_begin_write` on the Mac and `IMB_exr_begin_write: No such file or directory.` on Linux. The run of 2026-08-12 above got the same split. All three records now stand in this document. Neither of the two earlier runs is chosen over the other, and this one does not choose between them either — it is a third record of what the machines printed on the day it was run.

Laid out across the four versions, the first of the two lines reads:

| Version | The Creator's Mac | The rented Linux machine |
|---|---|---|
| 4.5 | `IMB_exr_begin_write: ERROR: No such file or directory.` | `IMB_exr_begin_write: ERROR: No such file or directory.` |
| 5.0 | `ERROR IMB_exr_begin_write: No such file or directory.` | `ERROR IMB_exr_begin_write: No such file or directory.` |
| 5.1 | `ERROR Unknown error in IMB_exr_begin_write` | `ERROR IMB_exr_begin_write: No such file or directory.` |
| 5.2 | `ERROR Unknown error in IMB_exr_begin_write` | `ERROR IMB_exr_begin_write: No such file or directory.` |

The second line was `Error writing render result, No such file or directory (see console)` in every version on both machines.

**23. (Entry 25.) Test 19's failing run again — in Blender 5.1 the two machines agree.** The File Output node's own base folder was made first and then set to read-and-enter only, so it exists and cannot be written to. The node's one input was named `passes/diffuse`. The scene's own output path pointed at a folder that works. Workbench, 320 × 240, frames 1 to 3, PNG.

On the **Creator's Mac**:

```
VERSION 5.1.2
File Output node built in the 5.x shape
input renamed to 'passes/diffuse'
node folder exists before: True
00:00.630  image            | ERROR Couldn't create directory for file .../work/e25/5.1/nodelocked/n_passes/diffuse0001.png: Permission denied
00:00.630  reports          | ERROR Render error (Permission denied) cannot save: '.../work/e25/5.1/nodelocked/n_passes/diffuse0001.png'
Blender 5.1.2 (hash ec6e62d40fa9 built 2026-05-19 02:18:37)
00:00.653  render           | Saved: '.../work/e25/5.1/scene/s_0001.png'
00:00.658  image            | ERROR Couldn't create directory for file .../work/e25/5.1/nodelocked/n_passes/diffuse0002.png: Permission denied
00:00.658  reports          | ERROR Render error (Permission denied) cannot save: '.../work/e25/5.1/nodelocked/n_passes/diffuse0002.png'
00:00.663  render           | Saved: '.../work/e25/5.1/scene/s_0002.png'
00:00.667  image            | ERROR Couldn't create directory for file .../work/e25/5.1/nodelocked/n_passes/diffuse0003.png: Permission denied
00:00.667  reports          | ERROR Render error (Permission denied) cannot save: '.../work/e25/5.1/nodelocked/n_passes/diffuse0003.png'
00:00.671  render           | Saved: '.../work/e25/5.1/scene/s_0003.png'
operator returned: {'FINISHED'}
node folder exists now: True
node folder holds: []
```

with `exit code: 0`. On the **rented Linux machine** the same lines, with its own paths:

```
00:00.920  image            | ERROR Couldn't create directory for file /home/smoozy/crosssweep/work/e25/5.1/nodelocked/n_passes/diffuse0001.png: Permission denied
00:00.920  reports          | ERROR Render error (Permission denied) cannot save: '/home/smoozy/crosssweep/work/e25/5.1/nodelocked/n_passes/diffuse0001.png'
```

also with `operator returned: {'FINISHED'}`, the node's folder left empty, the scene's three frames written, and `exit code: 0`.

**The two machines agreed, word for word.** This matches what test 19 above records for Blender 5.1. This run was made because Blender 5.0 was in doubt on the same test; 5.1 was run alongside so that the versions can be compared, and 5.1 shows no difference between the two systems here.

---

### 12d. Outside the interface

The sources do not say. They say nothing about the behaviour inside the interface either, so there is nothing to compare. I specifically checked the command-line page for an exit code or a logged failure that would let a program driving Blender detect this, and found nothing covering it.

**Settled by the hand tests of 2026-08-11.** It holds, and the exit code the question asks about is now on record. Every test above was run with no window: the exit code is **1** when the scene's own output folder cannot be made, and **0** when only a File Output node's or the render cache's cannot. Test 21 repeated the failure in a Blender with a window and the terminal said the same thing. What a person at the screen would see was not settled, for the reason given in test 21.

**Still holding after the hand tests of 2026-08-12.** Tests 22 and 23 were also run with no window, on both machines, and both ended with exit code 0 while printing their failure on the terminal. Nothing changed.

### 12e. Operating system

The sources do not say. Nothing in the four sources names an operating system in connection with making, or failing to make, a render output folder.

**Settled by the hand tests of 2026-08-11.** The Creator's Mac and the rented Linux machine agreed on every one of tests 15 to 21, in what the render does *and* in the words it prints — unlike Blender 4.5, where the two machines print different reasons for the same failure. Nothing about **Windows** was tested — no Windows machine has run it — so Windows stays open on all of it.

**Corrected by the hand tests of 2026-08-12, in one place.** The sentence above says the two machines agreed on the words printed in every one of tests 15 to 21. **Test 22 found one line where they do not.** When the render cache folder cannot be made, the Creator's Mac printed `ERROR Unknown error in IMB_exr_begin_write` and the rented Linux machine printed `ERROR IMB_exr_begin_write: No such file or directory.` The second line of that pair, and everything the render actually did, were the same on both. That is the one line, in test 20's failing run, where Blender 5.1 does differ between the two systems — the same split Blender 5.2 already records, and one that Blender 4.5 and 5.0 do not show.

The earlier record is not overwritten by this: test 20 above records the same line as `IMB_exr_begin_write: No such file or directory.` on both machines and says both agreed, and that record stands beside the two later runs that found them differing. Neither is chosen. Everywhere else in tests 15 to 23 the two machines printed the same words, including the File Output node failure of test 23. Nothing about **Windows** was tested in any of them — no Windows machine has run it — so Windows stays open on all of it.

---

## Things a person sitting at a computer with Blender 5.1 open could settle

**All four items of the original list — A, B, C and D — were run on 2026-08-11, on the Creator's Mac and on a rented Linux machine, and their results are written into questions 11 and 12 above.** What is left is only what those two machines could not reach.

1. **Anything on Windows** — an output folder that cannot be made, a folder denied through its security properties, and the exit code (`echo %ERRORLEVEL%`). Step 6 of the original test B asked for exactly this. **Not run with the equipment used**: the runs used one Mac and one rented Linux machine, and no Windows machine was rented for them. Every question-11 result above leaves Windows open too.
2. **A folder shared over a network between two machines.** Test 14 above ran two Blenders into one folder on a single machine's own disk. The manual's render-farm recipe is about a shared network folder, which is not the same thing, and no network share between two machines is among this project's written-down equipment. It needs the Creator's say-so first.
3. **Whether the two-gigabyte video split behaves the same on macOS.** Test 12 was run on the rented Linux machine only, because it writes several gigabytes four times over and the Mac is the Creator's working machine.
4. **What Blender's own window shows when a render cannot make its folder** — the status bar and the Info list, word for word. The terminal half is settled (test 21), but the picture Blender took of its own window has the first-run Quick Setup panel over it, and dismissing that needs a click. Anybody with a hand on a mouse can settle it in a minute: set the output path into a folder you cannot write to, press Ctrl-F12, and read the status bar and the Info editor.
5. **Multi-view or stereoscopy output during a render**, and **what becomes of the held result a still render offers for viewing** when Blender has no window. Neither was tested.


---

2026-08-11 — Claude Opus 5 (Claude Code) — Writing, Blender 5.1

2026-08-11 — Claude Opus 5 (Claude Code) — hand tests: twenty-one observed results written into questions 11 and 12, both standing lines brought up to date on both answers, and the researcher's own list pruned to what two machines cannot reach

2026-08-12 — Claude Opus 5 (Claude Code) — second batch of hand tests: entries 26 and 25 run on both machines and written into question 12 as tests 22 and 23. The operating-system standing line on question 12 is corrected: the two machines do **not** print the same words when the render cache folder cannot be made. The earlier record that says they agreed stands beside it and is not overwritten. Nothing on the list above closed or opened; it is unchanged.

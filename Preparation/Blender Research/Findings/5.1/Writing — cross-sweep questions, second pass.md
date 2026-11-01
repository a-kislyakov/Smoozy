# Writing — cross-sweep questions, second pass (Blender 5.1)

Sources consulted, all inside `Blender Sources/`, all read on **2026-07-30** (the snapshot date):

- The Blender 5.1 manual (`manual_5.1`), including its pictures in `_images`
- The Blender 5.1 Python API documentation (`api_5.1`)
- The Blender 5.1 release notes (`release_notes_5.1`)
- Cycles' own files (`cycles_5.1`)

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

Question answered: **11** only.

---

## 11. While a render is still working, what is present in the places it writes to, and how does that change with the size of the picture?

### The question was ambiguous in two places, and I say so before answering

**"The places it writes to."** I read this as the folders a render is aimed at — the Output path, and any folder a File Output node names. But a render also writes to places the person never named, above all Blender's own temporary directory, and the one size-related fact in the sources lives there. So I answer the first reading and then cover the second separately, marked as such.

**"The size of the picture."** I read this as the picture's dimensions in pixels — the render resolution. The other reading is the size of the finished file in bytes; under that reading the answer would be about Autosplit, which I also give below so both readings are served.

### Short answer

The sources describe five things that can be in a render's output folders while the render is still going, and only one of them is tied to the size of the picture — and that one is not in the output folder at all.

---

### (a) An animation render: the frames already finished are there, the one being worked on is not

**LABEL: documented** (that finished frames are on disk) / **inferred** (that the frame in progress is absent)

Exact words, manual, *Rendering Animations*:

> "While rendering stills will allow you to view and save the image from the render buffer when it is complete, animations are a series of images, or frames, and are automatically saved directly out to a drive after being rendered."

> "This allows you an easy recovery if there is a problem and you have to re-start the rendering, since the frames you have already rendered will still be in the output directory."

And the manual, *Output ‣ Introduction*:

> "when rendering animations to image file formats the render job can be canceled and resumed at the last rendered frame by changing the frame range."

Source: the manual, https://docs.blender.org/manual/en/5.1/render/output/animation.html and https://docs.blender.org/manual/en/5.1/render/output/introduction.html
Date read: 2026-07-30

The inferred step: the source says a frame is saved out "after being rendered", and that recovery works because "the frames you have already rendered will still be in the output directory". From "after being rendered" I take it that a frame not yet finished has not yet been written, so mid-render the folder holds the finished frames and nothing for the frame currently being worked on. The source never states that outright.

### (b) With Placeholders on, an empty file for the frame stands in the folder while it renders

**LABEL: documented** that the setting exists, that it makes empty files while rendering, and that it is off by default; **inferred** that the empty file is there while *that* frame renders.

*Corrected by the citation check of 2026-08-11: this was labelled documented throughout. The quoted words say "Create empty placeholder frames while rendering" and "(default False)"; they do not say when in a frame's life the file appears. The step, written out: the render-farm hint quoted below only shares work if a machine can see, before it starts a frame, that another has taken it — so the empty file must be down before the frame is worked on. A reader supplies that.*

Exact words, manual, *Output* panel:

> "Create empty placeholder frames while rendering."

Exact words, Python API, `bpy.types.RenderSettings.use_placeholder`:

> "Create empty placeholder files while rendering frames (similar to Unix 'touch') (default False)"

Exact words, manual, the hint under the same panel:

> "Primitive Render Farm
> An easy way to get multiple machines to share the rendering workload is to:
> Set up a shared directory over a network file system.
> Disable Overwrite, enable Placeholders in the Render Output panel.
> Start as many machines as you wish rendering to that directory."

Sources: the manual, https://docs.blender.org/manual/en/5.1/render/output/properties/output.html ; the API, https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html
Date read: 2026-07-30

A note on the manual page, because it matters for anyone checking my quote. In the text of that page the heading above the placeholder line is garbled — it reads "Image Sequence – Overwrite Image Multi-Layer EXR", the same heading as the Overwrite setting just above it. The label **Placeholders** comes from the panel picture on that same page.

**From a picture — LABEL: documented.** I opened `_images/render_output_properties_output_panel.png`, the "Output panel" picture on the manual page above. Visible in it: a file path field reading `/tmp/`, and then two columns of checkboxes — left column "Overwrite" (ticked) and "File Extensions" (ticked), right column "Placeholders" (unticked) and "Cache Result" (unticked) — then File Format PNG, Color BW/RGB/RGBA with RGBA selected, Color Depth 8/16 with 8 selected, and Compression 15%. That is what is visible. That the unticked state in the picture is the default is not something the picture proves; the API's "(default False)" is what proves it.

The default is off, so by default there is no placeholder.

### (c) A render aimed at video: one file, growing, and possibly already more than one

**LABEL: documented** (the file exists and can be left unplayable; the splitting) / **inferred** (that it is present and incomplete part-way through)

Exact words, manual, *Rendering Animations*, the Direct Approach:

> "Blender creates one file that holds all the frames of your animation."

Exact words, manual, *Output ‣ Encoding*:

> "If your video is huge and exceeds 2GiB, enable Autosplit Output. This will automatically split the output into multiple files after the first file is 2GiB in size."

Exact words, manual, *Output* tips:

> "If a problem occurs while rendering, the file might become unplayable and you will have to re-render all frames from the beginning."

Sources: the manual, https://docs.blender.org/manual/en/5.1/render/output/animation.html and https://docs.blender.org/manual/en/5.1/render/output/properties/output.html
Date read: 2026-07-30

The inferred step: for a problem *while rendering* to leave a file unplayable, the file must already be on disk before the render is done. So mid-render the folder holds one partly-written video file. And with Autosplit on, once the render has put 2 GiB into that file the folder holds more than one file from then on. This is the one place the byte-size reading of "size of the picture" bites: a bigger picture makes a bigger file, so the 2 GiB point arrives sooner and the extra files appear earlier. The sources do not say that; it is my step from the quoted 2 GiB rule.

### (d) File Output nodes write into their folders during the render, and make the folders as they go

**LABEL: documented**

Exact words, manual, *File Output Node*:

> "The File Output node writes one or more images to disk during rendering or compositing."

> "Any missing directories are created automatically during output."

Source: the manual, https://docs.blender.org/manual/en/5.1/compositing/types/output/file_output.html
Date read: 2026-07-30

So the sub-folders an input's *name* asks for can come into existence during the render, not before it.

*Narrowed by the citation check of 2026-08-11: this said "a File Output node's folders can come into existence during the render". On that page the "Any missing directories are created automatically during output." sentence sits three lines below "When the Media Type is set to Image, input names are also used to construct subdirectories:" and "If an input name contains a slash character (/ or \\), the path is interpreted as a directory hierarchy." So what the source carries is the sub-folders an input name asks for, not the node's own base folder. The heading of this section still reads "and make the folders as they go", and that half now rests on the hand test below rather than on the quote.*

### (e) A single still rendered from inside Blender puts nothing there at all

**LABEL: inferred**

Exact words, manual, *Rendering Animations*:

> "While rendering stills will allow you to view and save the image from the render buffer when it is complete"

Source: the manual, https://docs.blender.org/manual/en/5.1/render/output/animation.html
Date read: 2026-07-30

The step: the source says a still lets you "save the image from the render buffer" yourself, and contrasts that with animations being "automatically saved directly out to a drive". From that I take it that a still render started from inside Blender writes nothing to the output folder, during or after. The source does not say so in as many words. See the two standing lines below — from outside the interface this is different, and documented.

---

### Now the size of the picture

**LABEL: documented** (that big pictures are cut into tiles and the tiles go to disk)

Exact words, manual, Cycles *Performance ‣ Memory ‣ Tile Size*:

> "Render high resolution images in tiles of this size, to reduce memory usage. Tiles are cached to disk while rendering to save memory."

Source: the manual, https://docs.blender.org/manual/en/5.1/render/cycles/render_settings/performance.html
Date read: 2026-07-30

**LABEL: observed** — from Cycles' own files.

File `cycles_5.1/properties.py`, lines 1051–1063:

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

File `cycles_5.1/ui.py`, lines 390–405:

*Corrected by the citation check of 2026-08-11: this said lines 390–404, but the last line quoted below — the `layout.label` call — is at line 405, outside the range given. The words are unchanged; only the line numbers are.*

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

Date read: 2026-07-30

What is observed and nothing more: the setting is called Tile Size, it is a whole number, its default is 2048, its lowest value is 8 and its highest is 8192. There is a second setting, Auto Tile, whose own description says "Deprecated, tiling is always enabled". And this piece of interface code computes an effective tile size from Tile Size — used as-is below 128, otherwise rounded up to the next multiple of 128 and capped at 8192 — and compares the tile's area against the render's area, the render's area being width times height times the resolution percentage.

**LABEL: inferred** — that the disk tiles start appearing once the picture is bigger than one tile.

The step, written out: the manual says tiles are used to render "high resolution images" and that tiles are cached to disk. Cycles' own interface code computes an effective tile size the same way the tile manager does — the comment says so — and treats the picture as tiled when `render_area > tile_area`. Putting those together, a picture whose area is no bigger than the effective tile is covered by a single tile and there is nothing to cache out; a picture bigger than that is split, and the pieces go to disk. With the default Tile Size of 2048 that boundary sits at about 4.19 million pixels, which a 1920×1080 picture is well under and a 4096×2160 picture is over. A reader has to supply the link between "tiled" and "cached to disk begins", so this is inferred, not documented.

**And here is what the sources will not tell me: where those tiles go.**

I could not find, anywhere in the manual, the API documentation, the release notes, or the Cycles files I was given, a statement of which folder the tile cache is written to, what the files are called, how many there are, how big they are, or when they are deleted. The phrase "cached to disk" appears exactly once in the whole 5.1 manual — the Tile Size line above — and it names no location. The Cycles files supplied here are the add-on's Python and the render kernel and its utilities only; the tile manager and session code that the `ui.py` comment refers to (`TileManager::compute_render_tile_size`, `Session::get_effective_tile_size`) are not among them, so there was nothing to observe. Searching the whole of `cycles_5.1` for `TileManager` returns one hit, and it is that comment.

**One thing I will not do is guess that they go to Blender's temporary directory.** The manual does document that the temporary directory holds render layers while Blender runs:

> "The temporary directory is used to store various files at run-time (including render layers, physics cache, copy-paste buffer and crash logs)."

Source: the manual, https://docs.blender.org/manual/en/5.1/advanced/blender_directory_layout.html
Date read: 2026-07-30

That says render layers. It does not say Cycles tiles. Treating the two as the same thing would be me joining them, and the join is not in any source I was given, so I leave it open.

### One more thing that can be present, unrelated to picture size: the render cache

**LABEL: documented** (what it is and where it goes) / **inferred** (that it is there before the render finishes)

Exact words, manual, *Output ‣ Saving – Cache Result*:

> "Saves the rendered view layers and their passes to a multi-layer OpenEXR image. The Compositor can then use this file to improve performance, especially for heavy compositing."

> "The image is stored in the Render Cache folder as specified in the File Paths Preferences."

Exact words, Python API, `bpy.types.RenderSettings.use_render_cache`:

> "Save render cache to EXR files (useful for heavy compositing, Note: affects indirectly rendered scenes) (default False)"

Exact words, manual, Preferences *File Paths ‣ Render ‣ Render Cache*:

> "The location where cached render images are stored."

Exact words, Python API, `bpy.types.PreferencesFilePaths.render_cache_directory`:

> "Where to cache raw render results (default "", never None, blend relative // prefix supported)"

Exact words, manual, *Read View Layers*:

> "Reads all the current scene's render layers from cache, as needed. This can be used to save RAM while rendering because the render layers do not have to be saved in RAM. And also for recovering some information from a failed render. For this to work, Cache Result must be enabled."

Sources: the manual, https://docs.blender.org/manual/en/5.1/render/output/properties/output.html , https://docs.blender.org/manual/en/5.1/editors/preferences/file_paths.html , https://docs.blender.org/manual/en/5.1/interface/controls/nodes/editing.html ; the API, https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html and https://docs.blender.org/api/5.1/bpy.types.PreferencesFilePaths.html
Date read: 2026-07-30

The inferred step: recovering information from a *failed* render, and saving RAM *while rendering*, both require the cache file to already be on disk before the render is finished. So with Cache Result on, a multi-layer OpenEXR is present in the Render Cache folder while the render is still working. The sources never say when it is first written or whether it grows. Default is off.

### What a running Blender 5.1 showed — hand tests, 2026-08-11

**LABEL: observed.** Every result below was produced on 2026-08-11 by running Blender 5.1.2 on two computers and writing down what it printed and what appeared on disk. The two computers are named in full because that is what settles which operating system each answer covers.

- **The Creator's Mac** — a MacBook Pro with an Apple M1 Pro chip, 16 GB of memory, macOS 26.5.2 (build 25F84), Apple's own graphics and no second card. Blender 5.1.2 at `/Applications/Blender 5.1.2.app/Contents/MacOS/Blender`.
- **A rented Linux machine** — Ubuntu 22.04.5 LTS on kernel 6.8, two NVIDIA RTX 4000 Ada Generation cards of 20 GB each on driver 580.105.08, an AMD Ryzen Threadripper PRO 3945WX with eleven cores, 49 GB of memory. Blender 5.1.2 at `/opt/blender/5.1/blender`.

Every run used `-b --factory-startup` unless it says otherwise, and each was pointed at a settings folder of its own so that no run touched anybody's saved Blender settings. Where the same test ran on both machines, both results are given and it is said plainly whether they agreed. The whole list, with the exact commands, is `Audits/Blender Research/Writing/Hand tests — cross-sweep questions.md`.

**1. Section (e) is right: a still render writes nothing at the output path unless it is asked to — and that holds with a window too.** Output path set to an empty folder, PNG, 320 × 240, Cycles:

```
BEFORE: []
AFTER render, no save switch: []
AFTER render with the save switch: ['still_.png']
```

`blender -b --factory-startup -o <folder>/cli_ -F PNG -f 1` wrote `cli_0001.png`. The three runs repeated in a Blender **with a window open on screen** printed the same three lines. So the difference the *Outside the interface* section draws — a still saved from the command line, not saved from inside — is a difference between **being asked to save and not being asked**, not between having a window and not. Both machines agreed.

**2. Section (a)'s inferred step is right, with one thing it does not cover.** Cycles, 1920 × 1080, 400 samples, frames 1 to 4, PNG, Placeholders off, watched every fifty milliseconds. On the rented Linux machine:

```
   0.00  .../out  []
  10.25  .../out  [('f_0001.png', 0)]
  10.30  .../out  [('f_0001.png', 712704)]
  10.35  .../out  [('f_0001.png', 1433600)]
  10.40  .../out  [('f_0001.png', 1598617)]
  20.05  .../out  [('f_0001.png', 1598617), ('f_0002.png', 0)]
```

For the ten seconds a frame is being rendered there is nothing for it in the folder — the inferred step holds. For about a fifth of a second while it is being *written*, the file is there and growing. **So a half-written frame is briefly visible**, which the first item under NOT COVERED says nothing describes: no temporary name, no lock file, no rename — the file is built up in place. The Mac agreed, and the same animation at 8192 × 4608 stretched that window to about two seconds.

**3. What a placeholder contains, when it is made, and what becomes of it after a cancel — the second item under NOT COVERED.** Placeholders on, Overwrite off, same scene. On the rented Linux machine:

```
   0.00  .../out  []
   0.35  .../out  [('f_0001.png', 0)]
  10.15  .../out  [('f_0001.png', 40960)]
  10.20  .../out  [('f_0001.png', 753664)]
  10.25  .../out  [('f_0001.png', 1482752)]
  10.30  .../out  [('f_0001.png', 1598617), ('f_0002.png', 0)]
```

**Zero bytes**, made three tenths of a second in — ten seconds before the frame is finished — one at a time for the frame in hand, and replaced in place by the picture rather than by a second file. Stopping the render part way (frames 1 to 6, the ordinary terminate signal once frame 3's placeholder had appeared) left `f_0003.png` behind at 0 bytes, with Blender exiting 143. Both machines agreed.

**4. Blender holds the file open while it writes it.** During a 10000 × 10000 multi-layer OpenEXR write the system reported Blender holding that file with a write handle. Renaming it at that moment succeeded and the render still finished. Both machines agreed.

**5. Where Cycles' tile cache lives, what it is named, how big it gets and when it is removed — the whole of the third item under NOT COVERED.** Cycles, 32 samples, tile size at its default 2048, one frame at 8000 × 8000, with every file the process had open sampled twice a second. On the rented Linux machine:

```
blender 38829 root 3u REG 253,1    118784 /tmp/blender_MnaQp0/cycles-tile-buffer-38829-134997896366912-0-0.exr
blender 38829 root 3u REG 253,1  84815872 /tmp/blender_MnaQp0/cycles-tile-buffer-38829-134997896366912-0-0.exr
blender 38829 root 3r REG 253,1  89547247 /tmp/blender_MnaQp0/cycles-tile-buffer-38829-134997896366912-0-0.exr
```

On the Mac the same file sat under `/var/folders/…/T/blender_XXXXXX/`. **One file at a time, an OpenEXR, in Blender's own temporary folder, named `cycles-tile-buffer-<process number>-<number>-0-0.exr`, about 89 MB for that frame, and gone before Blender exits.** So the leap this pass refuses to make — that the tile cache and the manual's "render layers" are the same thing — was right to be refused: they share a folder, and the tile cache is a file of its own with its own name.

**6. The boundary is exactly where it was inferred to be.** Tile size 2048, one frame, with the system's temporary folder pointed at an empty folder of the run's own and walked every twenty milliseconds. At **2048 × 2048** nothing appeared at all. At **2048 × 2049**, on the Mac:

```
   0.00  []
   5.39  [('.../tmp/blender_I8A972/cycles-tile-buffer-72831-40589232864-0-0.exr', 4096)]
   5.45  [('.../tmp/blender_I8A972/cycles-tile-buffer-72831-40589232864-0-0.exr', 913408)]
  12.68  []
```

Both machines agreed. **This settles the fourth item under NOT COVERED**: the boundary the interface code's warning uses is the boundary the disk cache uses. At 8000 × 8000 with the tile size raised to **8192** — larger in area than the picture — there was no such file; at **256** there was one, written to far more often. And `use_auto_tile` set to False changed nothing, which matches its own description. (In Blender 4.5 the same switch does turn it off; that is a difference between versions.)

**7. EEVEE and Workbench write nothing to disk during a render — the fifth item under NOT COVERED.** The same 8000 × 8000 frame under `BLENDER_EEVEE` and under `BLENDER_WORKBENCH` opened no tile-buffer file and put nothing in the temporary folder, on either machine.

**8. When a File Output node writes, and that it makes its own folder.** Compositor on, one node aimed at a folder that did not exist. `node folder exists before the render: False`, and then, frame by frame:

```
Saved: '.../work/e12/5.1/node/n_Image0001.png'
Saved: '.../work/e12/5.1/out/f_0001.png'
```

The node's folder is made by the render, and its file for a frame lands just before the scene's own file for that frame. Both machines agreed.

*Corrected by the citation check of 2026-08-11: this sentence said the node's own base folder being made "is what the manual's 'Any missing directories are created automatically during output' describes". It is not — that sentence is about the sub-folders an input's name asks for, as section (d) above now says. That the node's base folder is made is settled by this test and by nothing in the sources.*

**9. The render cache file is there before the render ends, and this is what it is called.** Two view layers, Cycles, 1920 × 1080, frames 1 to 3. Blender printed, once per frame, `Caching exr file, 1920x1080, .../cache/cached_RR_UNSAVED_Scene_00000000000000000000000000000000.exr`, and the watcher showed it reaching about 133 MB before that frame's own PNG appeared, then dropping to zero and growing again for the next frame. One file, rewritten per frame, still there at the end. **The inferred step in the render-cache section is right.** Both machines agreed.

**10. Section (c) is right about the video file, and the file left after a stop will not play.** Workbench, 1280 × 720, FFmpeg in a Matroska container with H.264, frames 1 to 4000, stopped after 25 seconds. `movie_0001-4000.mkv` — named for the whole frame range from the first moment — appeared at 0 bytes within a second and grew in steps of 262,144 bytes. Handing the file left behind back to Blender 5.1 gave `loaded as a movie: movie_0001-4000.mkv  frames: 0  size: (256, 256)`. Both machines agreed. **Run twice**: a first run of 200 frames finished before it could be stopped, and showed the finished file opening properly.

**11. Autosplit: the second file starts at about 1.87 GiB, not 2 — the byte-size reading of the question.** Workbench, 1920 × 1080, FFmpeg in an AVI container with the lossless HUFFYUV codec, 1400 frames. **Run on the rented Linux machine only** — it writes several gigabytes four times over and the Mac is the Creator's working machine — so it settles Linux and leaves macOS open. With Autosplit on:

```
FINAL: [('long__0000001-1400.avi', 2003090368), ('long__0010001-1400.avi', 1022075648)]
```

With it off, one file of 3,025,156,392 bytes. The names gain an extra block of digits when the option is on. **Also run twice**: a first run of 900 frames reached 1,945,065,136 bytes and never split.

**12. No texture cache is written beside the pictures a render reads.** A PNG written by Blender itself, used as an image texture, rendered with Cycles at 4096 × 4096: the folder holding it contained only that PNG before and after. Asking Blender 5.1 what settings it has for this printed `no such setting: use_texture_cache` and `no such setting: use_auto_generate_texture_cache`. Both machines agreed. (Blender 5.2 has both switches and, with them on, writes a `blender_tx/` folder beside the picture. 5.1 has neither.)

**13. Two Blenders writing into one folder divide the work — the last item under NOT COVERED.** **Run on the rented Linux machine only**, and the one test here whose whole subject is two Blenders at once. Placeholders on, Overwrite off, frames 1 to 8, the second started three seconds after the first. One took the odd frames and the other the even ones, each printing `skipping existing frame "<path>"` for the frames the other had claimed. No frame was rendered twice. This was a folder on one machine's own disk; a folder shared over a network was not tested, so the network file system the render-farm hint suggests is still untried.

**What these tests leave open.** Windows, everywhere — no Windows machine has run it. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.** A folder shared over a network. Whether the two-gigabyte split behaves the same on macOS. And multi-view or stereoscopy output.

---

### NOT COVERED

*Most of this list has since been settled by the hand tests above; the note after it says which parts stand.*

This answer does not settle:

- Whether an image file being written is briefly visible on disk in a half-written state, or appears whole. Nothing in the sources describes a temporary name, a partial file, a lock file, or a rename-on-completion for render output.
- What a placeholder file actually contains or how big it is. "Empty" and "similar to Unix 'touch'" is all the sources say. Whether a placeholder is replaced, left behind after a cancel, or cleaned up after a crash — not stated.
- Where Cycles' tile cache lives, what the files are named, how many exist at once, and when they are removed. Covered above; it is simply absent.
- The exact resolution at which tiling and its disk cache begin. My boundary is inferred from interface code that draws a warning label, not from the tiling decision itself.
- Whether EEVEE, or any engine other than Cycles, writes anything to disk while a picture renders. I found nothing either way.
- Whether anything else in the output folder changes with the size of the picture. Beyond the Cycles tile line and the 2 GiB Autosplit rule, I found no statement linking picture size to what is on disk mid-render.
- Anything about several renders writing into one folder at once, beyond the Primitive Render Farm hint quoted above.

**What the hand tests of 2026-08-11 settled from that list, and what is left.** Settled: that a frame's file is briefly visible half-written and is built up in place, with no temporary name and no rename; that a placeholder is zero bytes, made before its frame, one at a time, and left behind after a cancel; where the tile cache lives, what it is named, how big it gets and when it goes; the exact resolution at which it begins; that EEVEE and Workbench write nothing; and that two Blenders sharing one local folder divide the frames between them. **Still open**: everything on Windows, a folder shared over a network rather than on one machine's own disk, whether the two-gigabyte split behaves the same on macOS, and multi-view or stereoscopy output.

### Outside the interface

Partly documented, partly not.

One difference is documented and real. From the command line a still frame is written to disk, where inside Blender it is not:

> "-f, --render-frame <frame>
> Render frame <frame> and save it."

And the animation switch is documented as using the file's own settings:

> "-a
> Render the whole animation using all the settings saved in the blend-file."

Source: the manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html and https://docs.blender.org/manual/en/5.1/advanced/command_line/render.html
Date read: 2026-07-30

Overwrite, Placeholders, Cache Result and Cycles' Tile Size are all settings held in the blend-file, so on the quoted words above they apply to a render driven from outside just as they do inside — that last step is mine, so it is inferred. Beyond that, the sources say nothing about background mode changing what appears on disk while a render runs. The Primitive Render Farm hint describes many machines rendering into one shared directory using Placeholders, which is the shape of an outside-driven job, but the manual does not call it that. Anything more precise — whether the tile cache, the render cache, or the video file behave differently with no window open — the sources do not say.

**Settled by the hand tests of 2026-08-11.** It holds, and the documented difference above turns out to be narrower than it reads. All thirteen tests were run with no window at all, so every one of them is established for a Blender driven from outside. Test 1 was then repeated in a Blender **with a window open on screen**, on both machines, and printed the same three lines: a still render wrote nothing until it was asked to, window or no window. So what decides whether a still lands on disk is **whether the write was asked for**, not whether Blender has a window — the command-line switch simply asks for it. Nothing was found that a window changes.

### Operating system

Mostly not said, with one documented exception.

The one documented difference is where the temporary directory is, which matters because the manual puts render layers there:

> "The temporary directory is selected based on the following priority:
> User Preference (see File Paths).
> Environment variables (TEMP on Windows, TMP & TMP_DIR on other platforms).
> The /tmp/ directory."

Source: the manual, https://docs.blender.org/manual/en/5.1/advanced/blender_directory_layout.html
Date read: 2026-07-30

Everything else in this answer — placeholders, the growing video file, File Output node writing, the render cache location, Cycles tiling — is described in the sources with no operating system attached, and I found no page saying any of it differs by platform. The API's phrase "similar to Unix 'touch'" is a description of what a placeholder is, not a statement that placeholders are a Unix-only feature. The sources do not say whether a network file system, which the render farm hint explicitly suggests using, changes any of this.

**Settled by the hand tests of 2026-08-11.** Tests 1 to 10 and 12 above were each run on macOS on an Apple M1 Pro and on Linux on an x86 machine, and the two machines agreed on every one of them, so none of this is a macOS-only or Linux-only behaviour. What is put in the temporary directory is the same on both: the very same `cycles-tile-buffer-…-0-0.exr` file sat under `/var/folders/…/T/` on macOS and under `/tmp/` on Linux. Nothing about **Windows** was tested — no Windows machine has run it — so every claim here leaves Windows open. Tests 11 and 13 were run on Linux only and leave macOS open; the reason is written into each. **A network file system was not tested at all** — test 13's two Blenders shared a folder on one machine's own disk — so that line stands exactly as the sources leave it.

---

## Where I looked

Everything below was searched inside `Blender Sources/`, in the 5.1 folders only, on 2026-07-30.

**Whole-manual searches (`manual_5.1`, all pages at once).** Terms: `placeholder`; `save buffers`; `Cache Result`; `tile size`; `auto tile`; `tiling`; `tile`; `Temporary Files`; `temporary directory`; `Render Cache`; `cached to disk`; `cache to disk`; `written to disk`; `saved to disk`; `while rendering`; `partially written`; `incomplete file`; `partial file`; `half-written`; `empty file`; `zero byte`; `0 bytes`; `unix 'touch'`; `resume rendering`; `already rendered`; `skip frames that`; `render farm`; `overwrite`; `.tmp`; `lock file`; `scratch file`; `spool`; `in-progress`; `intermediate file`; `disk cache`; `swap file`; `written at`; `written when`; `written only`; `file is created`; `created before`; `appears in the output`; `output folder will/contains`; `high resolution`; `large resolution`; `bigger image`; `bigger resolution`; `resolution_x`; `temp` (within `render/` and the preferences pages).

**Manual pages opened in full or in part.** `render/output/properties/output.txt`; `render/output/animation.txt`; `render/output/introduction.txt`; `render/cycles/render_settings/performance.txt`; `render/cycles/gpu_rendering.txt` (the Tile Size passage); `compositing/types/output/file_output.txt`; `advanced/blender_directory_layout.txt`; `editors/preferences/file_paths.txt`; `interface/controls/nodes/editing.txt` (Read View Layers); `editors/image/editing.txt` (Open Cached Render); `interface/window_system/topbar.txt` (the Render menu); `advanced/command_line/render.txt`; `advanced/command_line/arguments.txt`.

**Pictures opened.** `manual_5.1/_images/render_output_properties_output_panel.png`, the Output panel picture on the Output page. Reported above.

**Whole-API searches (`api_5.1`).** Terms: `placeholder`; `tile_size`; `use_auto_tile`; `auto_tile`; `tile cache`; `cached to disk`; `temp file`; `temporary file`. Pages opened: `bpy.types.RenderSettings.txt` (the `use_overwrite`, `use_placeholder`, `use_render_cache`, `use_persistent_data` block); `bpy.types.PreferencesFilePaths.txt` (`render_cache_directory`, `temporary_directory`, `use_auto_save_temporary_files`); `bpy.app.handlers.txt` (the seven render handlers); `bpy.types.RenderEngine.txt` (searched for `tile`).

**Release notes (`release_notes_5.1`), all subject files.** Searched for `tile`, `placeholder`, `render cache`, `output`, `memory`, `disk`. Opened `cycles.md`, `rendering.md`, `pipeline_io.md`. Nothing in the 5.1 notes touches what is on disk during a render. The hits that came back were about other things entirely — a "Select Tile" modelling operator, a Grid to Points node, placeholders in text input boxes in the interface — and I ruled each out on reading it.

**Cycles' own files (`cycles_5.1`).** Searched the whole folder for `tile_size`, `tile`, `TileManager`, `disk`, `temp`, `write`. Opened `properties.py` around the tile settings, `ui.py` at lines 375–412 and 868–885, `engine.py`, and listed the folder tree. The folder holds the add-on's Python, an OSL shader set, and `source/kernel` plus `source/util`. It does not hold the session or tile-manager code, which is why the tile cache's location could not be observed.

**Ruled out, and why.**

- `bpy.types.RenderEngine`'s `tile_highlight_set`, `tile_highlight_clear_all` and `use_highlight_tiles` — these are about drawing a highlight over a tile in the interface, not about writing files.
- `bpy.app.handlers.render_write` — "on writing a render frame (directly after the frame is written)". It confirms frames are written one at a time as a render proceeds, but it says nothing about what is in the folder, so I did not lean on it.
- `use_auto_save_temporary_files` — "Automatic saving of temporary files in temp directory, uses process ID". This is the automatic saving of blend-files, not render output.
- The Video Sequencer's disk cache, the physics disk cache, and the Local Cache Directory used for asset indexing — all named "cache" and all found by my searches, none of them render output.
- The Sequencer's `use_placeholders` options — those reserve placeholders for *missing* frames of an image strip being read in, the opposite direction from render output.

---

## Things a person with Blender open could settle in a few minutes

**All eight items of the original list were run on 2026-08-11, on the Creator's Mac and on a rented Linux machine, and their results are written into the answer above.** What is left is only what those two machines could not reach.

1. **Anything on Windows.** Every result above holds for macOS and for Linux and says nothing about Windows. **Not run with the equipment used** — the runs used one Mac and one rented Linux machine, and no Windows machine was rented for them.
2. **A folder shared over a network between two machines.** Test 13 above ran two Blenders into one folder on a single machine's own disk, which settles the coordination but not the network. No network share between two machines is among this project's written-down equipment, so it needs the Creator's say-so first.
3. **Whether the two-gigabyte video split behaves the same on macOS.** Test 11 was run on the rented Linux machine only, because it writes several gigabytes four times over and the Mac is the Creator's working machine.
4. **Multi-view or stereoscopy output during a render.** Nothing here tested it.

---

2026-08-11 — Claude Opus 5 (Claude Code) — Writing, Blender 5.1

2026-08-11 — Claude Opus 5 (Claude Code) — hand tests: thirteen observed results written into question 11, both standing lines brought up to date, and the researcher's own list pruned to what two machines cannot reach

No helper agents were used for this pass. Everything above was read and written by me, from the question sheet and `Blender Sources/` alone.

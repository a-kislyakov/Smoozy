# Writing — cross-sweep questions (Blender 4.5)

Sources consulted: Blender's 4.5 manual, Blender's 4.5 Python API documentation, the Blender 4.5 release notes, and Cycles' own files as shipped with Blender 4.5 — all as held in `Blender Sources/`, snapshot taken 2026-07-30. That snapshot date is the date read for everything quoted below. Pictures from the manual's `_images` folder were also opened where noted.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

Questions answered: 11 and 12.

---

## 11. While a render is still working, what is present in the places it writes to, and how does that change with the size of the picture?

**A note on the reading.** "The size of the picture" is ambiguous. I have answered it as the number of pixels in the rendered image (its resolution), because that is what Blender itself calls the size of the image. The other reading is the size in bytes of the finished file on disk; where anything in the sources bears on that reading, I say so.

**LABEL: mixed — documented for what a finished frame and a placeholder file are; observed for the Cycles tiling setting; inferred for the growing video file and for what happens below the tile size; not in the sources for what a part-written file looks like on disk.**

### What is present in the output folder while an animation render is running

Finished frames are there, one file per frame already done.

> "While rendering stills will allow you to view and save the image from the render buffer when it is complete, animations are a series of images, or frames, and are automatically saved directly out to a drive after being rendered."

> "This allows you an easy recovery if there is a problem and you have to re-start the rendering, since the frames you have already rendered will still be in the output directory."

Source: manual — Rendering Animations. Address: https://docs.blender.org/manual/en/4.5/render/output/animation.html Date read: 2026-07-30. Label: **documented**.

The Python API confirms writing happens frame by frame, as discrete events:

> "bpy.app.handlers.render_write"
> "on writing a render frame (directly after the frame is written)"

Source: API documentation — Application Handlers. Address: https://docs.blender.org/api/4.5/bpy.app.handlers.html Date read: 2026-07-30. Label: **documented**.

### The frame currently being worked on: an empty file, if Placeholders is on

> "Placeholders
>
> Create empty placeholder frames while rendering."

Source: manual — Output panel (Properties ‣ Output ‣ Output). Address: https://docs.blender.org/manual/en/4.5/render/output/properties/output.html Date read: 2026-07-30. Label: **documented**.

The API states the same setting more exactly, and gives its default:

> "use_placeholder"
> "Create empty placeholder files while rendering frames (similar to Unix 'touch')"
> "boolean, default False"

Source: API documentation — RenderSettings. Address: https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html Date read: 2026-07-30. Label: **documented**.

So the file this option makes is an *empty* file, and the option is off unless somebody turns it on. The quoted words do not say when in a frame's life that file appears, and they do not say what, if anything, stands for the frame in flight when the option is off. The neighbouring option is on by default:

> "use_overwrite"
> "Overwrite existing files while rendering"
> "boolean, default True"

Same page and date. Label: **documented**.

The manual states the purpose these two serve together, which is the clearest statement in the sources of what an in-progress output folder is meant to look like to a second machine:

> "Primitive Render Farm
>
> An easy way to get multiple machines to share the rendering workload is to:
>
> Set up a shared directory over a network file system.
>
> Disable Overwrite, enable Placeholders in the Render Output panel.
>
> Start as many machines as you wish rendering to that directory."

Source: manual — Output panel. Address: https://docs.blender.org/manual/en/4.5/render/output/properties/output.html Date read: 2026-07-30. Label: **documented**.

The picture on that same manual page shows the panel with the two boxes: Overwrite ticked, Placeholders unticked, File Extensions ticked, Cache Result unticked, path `/tmp/`. Picture looked at: `_images/render_output_properties_output_panel.png`, the "Output panel" figure on https://docs.blender.org/manual/en/4.5/render/output/properties/output.html Date read: 2026-07-30. Label: **documented** (what is visible in the picture only).

The 4.5 release notes record a fix in this exact area:

> " * Placeholder output files wrong name"

listed under "Render & Cycles: 38". Source: release notes — Bugfixes. Address: https://developer.blender.org/docs/release_notes/4.5/bugfixes/ Date read: 2026-07-30. Label: **documented**.

### If the output is a video rather than a set of images

> "The Direct Approach, which is highly not recommended and not a standard practice, is where you set your output format to an AVI or MOV format, and click Animation to render your scene directly out to a movie file. Blender creates one file that holds all the frames of your animation."

Source: manual — Rendering Animations. Address: https://docs.blender.org/manual/en/4.5/render/output/animation.html Date read: 2026-07-30. Label: **documented**.

> "It's not recommended to render directly to a video format in the first instance. If a problem occurs while rendering, the file might become unplayable and you will have to re-render all frames from the beginning."

Source: manual — Output panel, Tips. Address: https://docs.blender.org/manual/en/4.5/render/output/properties/output.html Date read: 2026-07-30. Label: **documented** for the quoted words. That a single, growing, not-yet-playable video file therefore sits in the output folder for the whole length of the render is **inferred** — the step is: the file holds all frames, the render can be interrupted part way, and what is left is unplayable, so the file must already exist on disk before the render is done.

One documented case where what is present depends on how many bytes have been written, rather than on the picture's resolution:

> "If your video is huge and exceeds 2GiB, enable Autosplit Output. This will automatically split the output into multiple files after the first file is 2GiB in size."

Source: manual — Output panel, Encoding. Address: https://docs.blender.org/manual/en/4.5/render/output/properties/output.html Date read: 2026-07-30. Label: **documented**.

### The compositor's File Output node writes at the same time

> "The image(s) will be saved on rendering, writing to the current frame. An entire sequence of images will be saved, when an animation is rendered."

Source: manual — File Output Node. Address: https://docs.blender.org/manual/en/4.5/compositing/types/output/file_output.html Date read: 2026-07-30. Label: **documented**.

### Places other than the output folder that the sources tie to a running Blender

The temporary directory:

> "The temporary directory is used to store various files at run-time (including render layers, physics cache, copy-paste buffer and crash logs)."

Source: manual — Blender's Directory Layout. Address: https://docs.blender.org/manual/en/4.5/advanced/blender_directory_layout.html Date read: 2026-07-30. Label: **documented** for those words. They say "at run-time" and name render layers among what is stored; that a render is what puts them there, and puts them there while it runs, is **inferred**.

The render cache folder, but only if the option is switched on:

> "Cache Result
>
> Saves the rendered view layers and their passes to a multi-layer OpenEXR image. The Compositor can then use this file to improve performance, especially for heavy compositing.
>
> The image is stored in the Render Cache folder as specified in the File Paths Preferences."

Source: manual — Output panel. Address: https://docs.blender.org/manual/en/4.5/render/output/properties/output.html Date read: 2026-07-30. Label: **documented**. The API gives its default: "use_render_cache" — "Save render cache to EXR files (useful for heavy compositing, Note: affects indirectly rendered scenes)" — "boolean, default False" (https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html, read 2026-07-30). Label: **documented**.

And the manual says these cache files are not made unless asked for:

> "Note that Blender doesn't create these cache files by default. You have to enable Cache Result in the scene's Output options and then render it at least once."

Source: manual — Image editor, Open Cached Render. Address: https://docs.blender.org/manual/en/4.5/editors/image/editing.html Date read: 2026-07-30. Label: **documented**.

### How this changes with the size of the picture

This is the one place where the sources tie mid-render writing to the picture's resolution, and it is a Cycles setting.

> "Use Tiling
>
> Render high resolution images in tiles to reduce memory usage. Tiles are cached to disk while rendering to save memory.
>
> Tile Size
>
> This value is used to control the size of the tile used for rendering; decreasing the size reduces memory usage."

Source: manual — Cycles Performance (Panel: Render ‣ Performance). Address: https://docs.blender.org/manual/en/4.5/render/cycles/render_settings/performance.html Date read: 2026-07-30. Label: **documented**.

Cycles' own file carries the same setting with its kind, default and range:

> ```
>     use_auto_tile: BoolProperty(
>         name="Use Tiling",
>         description="Render high resolution images in tiles to reduce memory usage, using the specified tile size. Tiles are cached to disk while rendering to save memory",
>         default=True,
>     )
>     tile_size: IntProperty(
>         name="Tile Size",
>         default=2048,
>         description="",
>         min=8, max=8192,
>     )
> ```

File: `Blender Sources/cycles_4.5/properties.py`, lines 1051–1061. Date read: 2026-07-30. Label: **observed** (this is what the file says the setting is; it is not a statement about what a render does).

The panel it appears in is `Render ‣ Performance ‣ Memory`, and the file ties the state of the part holding Tile Size to Use Tiling — `Blender Sources/cycles_4.5/ui.py`, lines 850–865 (`col.prop(cscene, "use_auto_tile")`, then `sub.active = cscene.use_auto_tile`, then `sub.prop(cscene, "tile_size")`). That the file says this is **observed**. That Tile Size therefore shows greyed out when Use Tiling is off is **inferred** — the step is what the interface makes of that setting, which the file does not state.

Putting those together: with Cycles, tiling is on out of the box, the tile size is 2048 by default (the file gives no unit), and the sources say tiles are cached to disk while the render runs. So a bigger picture means tile data written to disk mid-render that a small picture does not produce. That last sentence — that a picture at or under the tile size produces no such tile writing while a larger one does — is **inferred**; the step is that the setting exists to break "high resolution images" into tiles "using the specified tile size", so a picture inside one tile has nothing to break up. The sources never state a threshold in so many words, and they never say **where** on disk those tiles are cached.

The other side of the resolution question: the API calls a placeholder file empty — "Create empty placeholder files while rendering frames (similar to Unix 'touch')" — which is **documented**. That a placeholder is therefore the same size whatever the resolution is one step from those words, since they say nothing about resolution: **inferred**.

For completeness on what "size of the picture" means in Blender's own words:

> "Resolution X, Y
>
> The number of pixels horizontally and vertically in the image."

Source: manual — Format panel. Address: https://docs.blender.org/manual/en/4.5/render/output/properties/format.html Date read: 2026-07-30. Label: **documented**.

### What the sources do not say

Whether a finished frame's file appears on disk complete in one go, or grows while it is written, is **not in the sources**. I searched the manual for "partially written", "partial file", "incomplete file", "half-written", "truncated", "atomic", "flush", ".tmp", "temporary file", "in progress", "during rendering" and "while rendering", and the only mid-render writing the manual describes is the four things quoted above. Nothing states a write-then-rename scheme, a lock file, or a part-file suffix. This record of looking covers the manual. It does not record a search of the API documentation, the release notes or Cycles' own files for the same thing, nor a search of any of them for where the cached tiles go.

**NOT COVERED.** This answer does not settle: what a partly-written image file contains or how long it exists; where Cycles writes its cached tiles; the exact resolution at which tiling starts producing disk writes; whether a placeholder is created before the frame starts or after it finishes; whether anything holds a lock on the file being written; what EEVEE or Workbench write mid-render, as opposed to Cycles; whether a network file system changes any of it; and the second reading of the question named at the top — how what is present changes with the size in bytes of the file being written — for which the only thing found in the sources is the 2GiB autosplit above.

*Most of that list has since been settled by hand tests, immediately below. What is left of it is named at the end of that section.*

---

### What a running Blender 4.5 showed — hand tests, 2026-08-11

**LABEL: observed.** Every result below was produced on 2026-08-11 by running Blender 4.5.12 LTS on two computers and writing down what it printed and what appeared on disk. The two computers are named in full because that is what settles which operating system each answer covers.

- **The Creator's Mac** — a MacBook Pro with an Apple M1 Pro chip, 16 GB of memory, macOS 26.5.2 (build 25F84), Apple's own graphics and no second card. Blender 4.5.12 LTS at `/Applications/Blender 4.5.12.app/Contents/MacOS/Blender`.
- **A rented Linux machine** — Ubuntu 22.04.5 LTS on kernel 6.8, two NVIDIA RTX 4000 Ada Generation cards of 20 GB each on driver 580.105.08, an AMD Ryzen Threadripper PRO 3945WX with eleven cores, 49 GB of memory. Blender 4.5.12 LTS at `/opt/blender/4.5/blender`.

Every run used `-b --factory-startup` unless it says otherwise, and each was pointed at a settings folder of its own so that no run touched anybody's saved Blender settings. Where the same test ran on both machines, both results are given and it is said plainly whether they agreed. The whole list of tests, with the exact commands, is `Audits/Blender Research/Writing/Hand tests — cross-sweep questions.md`.

**1. A still render puts nothing at the output path unless it is asked to.** Output path set to an empty folder, PNG, 320 × 240, Cycles. Running `bpy.ops.render.render(animation=False)` and then the same with `write_still=True`, listing the folder after each, printed:

```
BEFORE: []
AFTER render, no save switch: []
AFTER render with the save switch: ['still_.png']
```

`blender -b --factory-startup -o <folder>/cli_ -F PNG -f 1` wrote `cli_0001.png`. The same three runs were then done again in a Blender **with a window open on screen**, which printed the same three lines. Both machines gave the same answer, and the window made no difference.

**2. While a frame is being rendered, the output folder holds only the frames already finished — but while a frame is being *saved*, its file is on disk part-written.** Cycles, 1920 × 1080, 400 samples, frames 1 to 4, PNG, Overwrite on, Placeholders off, with a second program listing the folder every fifty milliseconds. On the rented Linux machine:

```
   0.00  .../out  []
  10.94  .../out  [('f_0001.png', 0)]
  10.99  .../out  [('f_0001.png', 712704)]
  11.04  .../out  [('f_0001.png', 1417216)]
  11.10  .../out  [('f_0001.png', 1584041)]
  21.55  .../out  [('f_0001.png', 1584041), ('f_0002.png', 0)]
  21.60  .../out  [('f_0001.png', 1584041), ('f_0002.png', 638976)]
  21.65  .../out  [('f_0001.png', 1584041), ('f_0002.png', 1368064)]
  21.70  .../out  [('f_0001.png', 1584041), ('f_0002.png', 1584041)]
```

Blender's own line for the first frame was `Time: 00:10.77 (Saving: 00:00.21)`. So nothing at all stands for a frame for the ten seconds it is being rendered, and then the file appears at zero bytes and grows to its full size inside about a fifth of a second. The Mac agreed, with the same shape and its own timings.

**3. With Placeholders on, an empty file for a frame appears before that frame is rendered, and is then filled in place.** Same scene, Placeholders on and Overwrite off. On the rented Linux machine:

```
   0.00  .../out  []
   0.30  .../out  [('f_0001.png', 0)]
  10.85  .../out  [('f_0001.png', 40960)]
  10.90  .../out  [('f_0001.png', 737280)]
  10.95  .../out  [('f_0001.png', 1433600)]
  11.00  .../out  [('f_0001.png', 1584041), ('f_0002.png', 0)]
```

The empty file for frame 1 is there three tenths of a second after the render starts, ten seconds before that frame is finished; the empty file for frame 2 appears at the moment frame 1's picture is complete. So the placeholders are made one at a time, for the frame in hand, not all at once. The Mac agreed.

**4. A render stopped part way leaves the empty file for the frame in hand behind.** Same as test 3, frames 1 to 6, stopped with the ordinary terminate signal once frame 3's placeholder had appeared. Blender ended with exit code 143 and the folder held:

```
-rw-r--r-- 1 root root 1584041 f_0001.png
-rw-r--r-- 1 root root 1584041 f_0002.png
-rw-r--r-- 1 root root       0 f_0003.png
```

The Mac agreed.

**5. A part-written file is visible for as long as the writing takes, and that grows with the picture.** Workbench, one frame at 10000 × 10000, watched every fifty milliseconds. On the Mac, PNG went from `('big_.png', 0)` at 3.96 seconds to `('big_.png', 60377290)` at about 7.5 seconds, climbing through roughly seventy recorded sizes; Blender's own line was `Time: 00:07.15 (Saving: 00:04.96)`. OpenEXR and multi-layer OpenEXR behaved the same way over about three quarters of a second, from `('big_.exr', 4634)` to `('big_.exr', 2705766)`. No file under any other name appeared first, and nothing was renamed. The rented Linux machine agreed.

**6. Blender holds the file open while it writes it.** While the 10000 × 10000 multi-layer OpenEXR above was being written, asking the system which program had that file open returned, on the Mac:

```
COMMAND   PID      USER   FD   TYPE DEVICE SIZE/OFF      NODE NAME
Blender 10209 artist       3w   REG   1,14   206318 103656546 .../work/e06/4.5/out/big_.exr
```

`3w` is a write handle. Renaming the file at that moment succeeded, and the render still finished. The rented Linux machine agreed.

**7. Where the cached tiles go: Blender's own temporary folder, in a file named `cycles-tile-buffer`.** Cycles, 32 samples, tile size left at its default of 2048, one frame at 8000 × 8000, with every file the Blender process had open sampled twice a second. On the Mac the list held:

```
Blender 11500 artist 3u REG 1,14    118784 .../T/blender_IRniBY/cycles-tile-buffer-11500-50886739680-0-0.exr
Blender 11500 artist 3u REG 1,14   2650112 .../T/blender_IRniBY/cycles-tile-buffer-11500-50886739680-0-0.exr
Blender 11500 artist 3r REG 1,14  88768733 .../T/blender_IRniBY/cycles-tile-buffer-11500-50886739680-0-0.exr
```

On the rented Linux machine the same file appeared as `/tmp/blender_VOCB9C/cycles-tile-buffer-23390-133458313353920-0-0.exr`, reaching 89,127,413 bytes. Blender's own progress line said `Rendered 15/16 Tiles` and later `Reading full buffer from disk`. The script printed `tempdir after: []` — the file is gone when the render ends. The same render at **1000 × 1000** opened no such file at all, on either machine.

**8. The tile size decides whether it happens, and in 4.5 the Use Tiling switch still turns it off.** Same set-up at 8000 × 8000 with the tile size changed:

- Tile size **8192**: no tile-buffer file, and no `Rendered n/m Tiles` line at all. Eight thousand by eight thousand is 64 million pixels, which is fewer than 8192 × 8192.
- Tile size **256**: the tile-buffer file appears and is written to far more often — many times as many distinct entries as at 2048, on both machines — and Blender said `Rendered 1023/1024 Tiles`. (The run recorded here counted 175 distinct entries on Linux and 208 on the Mac.)

*Corrected by the citation check of 2026-08-11: those two counts were written as though they were figures Blender produces. They are not: the instrument samples which files the process has open twice a second over a render whose length varies, so the count varies with it. Repeating this run on the rented Linux machine gave 144 rather than 175. The claim the counts support — far more often than at the default tile size — reproduced; the numbers themselves are a property of the sampling and are now marked as belonging to that run.*
- Tile size **2048 with `use_auto_tile` set to False**: **no tile-buffer file at all**, on either machine, and no tiling line. In 4.5 that switch still works.

**9. The boundary is one tile's worth of pixels.** Tile size 2048, one frame, and the system's temporary folder pointed at an empty folder of the run's own so that Blender's own temporary folder could be walked every twenty milliseconds. At **2048 × 2048** nothing appeared in it at all, on either machine, and Blender printed no tiling line. At **2048 × 2049**, on the Mac:

```
   6.61  [('.../tmp/blender_qerlZo/cycles-tile-buffer-72738-52480534240-0-0.exr', 4096)]
   6.64  [('.../tmp/blender_qerlZo/cycles-tile-buffer-72738-52480534240-0-0.exr', 98304)]
   6.67  [('.../tmp/blender_qerlZo/cycles-tile-buffer-72738-52480534240-0-0.exr', 1589248)]
   6.70  [('.../tmp/blender_qerlZo/cycles-tile-buffer-72738-52480534240-0-0.exr', 5734400)]
   6.73  [('.../tmp/blender_qerlZo/cycles-tile-buffer-72738-52480534240-0-0.exr', 6123520)]
   6.75  [('.../tmp/blender_qerlZo/cycles-tile-buffer-72738-52480534240-0-0.exr', 6127934)]
```

and Blender printed `Rendered 1/2 Tiles`. The rented Linux machine agreed. One row of pixels more than a single tile is enough.

**10. EEVEE and Workbench write no such file.** The same 8000 × 8000 frame rendered with `BLENDER_EEVEE_NEXT` and with `BLENDER_WORKBENCH` opened no tile-buffer file and nothing else in the temporary folder, on either machine. The tile cache is Cycles' alone.

**11. No texture cache is written beside the pictures a render reads.** A 1024 × 1024 PNG written by Blender itself, used as an image texture on the cube, rendered with Cycles at 4096 × 4096. Before and after, the folder holding that PNG contained only the PNG:

```
texture folder before the render: ['smoozytex.png']
texture folder after the render: ['smoozytex.png']
```

Asking Blender 4.5 what settings it even has for this printed `no such setting: use_texture_cache` and `no such setting: use_auto_generate_texture_cache`, and the file-path preferences hold `texture_directory` and `render_cache_directory` but no texture-cache folder. Both machines agreed. **In Blender 5.2 the same test writes a `blender_tx/` folder beside the picture once two switches are turned on; 4.5 has no such switches.**

**12. A File Output node's folder is made by the render and fills as the render goes.** Compositor on, one File Output node aimed at a folder that did not exist, separate from the scene's own output folder; Cycles, 1920 × 1080, frames 1 to 4. The script printed `node folder exists before the render: False`, and the render then printed, frame by frame:

```
Saved: '.../work/e12/4.5/node/n_0001.png'
Saved: '.../work/e12/4.5/out/f_0001.png'
Saved: '.../work/e12/4.5/node/n_0002.png'
Saved: '.../work/e12/4.5/out/f_0002.png'
```

The node's file for a frame lands just before the scene's own file for the same frame. Both machines agreed.

**13. With Cache Result on, a cache file is written for every frame, before that frame's picture.** Two view layers, Cycles, 1920 × 1080, frames 1 to 3, the render cache folder pointed at an empty folder. On the rented Linux machine Blender printed, once per frame:

```
Caching exr file, 1920x1080, .../cache/cached_RR_UNSAVED_Scene_00000000000000000000000000000000.exr
```

and the watcher showed the file appearing at 21.20 seconds and reaching 132,729,040 bytes before frame 1's own PNG appeared at 21.40 seconds. There is one file, not one per frame: it drops back to zero and grows again for each frame. It is still there when the render ends. The Mac agreed.

**14. A video render puts one file in the folder from the start, and stopping it leaves something that will not play.** Workbench, 1280 × 720, FFmpeg in a Matroska container with H.264, frames 1 to 4000, stopped with the ordinary terminate signal after 25 seconds. On the Mac:

```
   0.00  .../out  []
   0.80  .../out  [('movie_0001-4000.mkv', 0)]
   5.94  .../out  [('movie_0001-4000.mkv', 262144)]
  10.22  .../out  [('movie_0001-4000.mkv', 524288)]
  14.59  .../out  [('movie_0001-4000.mkv', 786432)]
  18.88  .../out  [('movie_0001-4000.mkv', 1048576)]
  23.14  .../out  [('movie_0001-4000.mkv', 1310720)]
```

The name carries the whole frame range from the first moment, and the file grows in steps of 262,144 bytes. Blender had reached `Append frame 1499` when it was stopped, and exited 143. Handing the 1,310,720-byte file left behind back to Blender 4.5 gave:

```
loaded as a movie: movie_0001-4000.mkv  frames: 0  size: (256, 256)
```

— that is, it would not open as a movie. The rented Linux machine agreed. **This test was run twice.** The first run used 200 frames, which finished in under four seconds, so the render was over before it could be stopped; that run is recorded here because it shows the same file appearing at zero bytes at 0.57 seconds and reaching its final 205,412 bytes at 3.93 seconds, and the finished file did open, `frames: 200  size: (1280, 720)`.

**15. Past two gigabytes with Autosplit on, a second file appears — and the split is at about 1.87 GiB, not 2.** Workbench, 1920 × 1080, FFmpeg in an AVI container with the lossless HUFFYUV codec, frames 1 to 1400. **This test was run on the rented Linux machine only**, because it writes several gigabytes four times over and the Creator's Mac is their working machine; it therefore settles Linux and leaves macOS open. With Autosplit on:

```
FINAL: [('long__0000001-1400.avi', 2003416672), ('long__0010001-1400.avi', 1022242144)]
```

With Autosplit off, one file: `FINAL: [('long_0001-1400.avi', 3025649192)]`. So the first file stops at 2,003,416,672 bytes — about 1.87 GiB — and a second one starts, and the names gain an extra block of digits when the option is on. **This test was also run twice**: the first run used 900 frames and reached only 1,945,065,136 bytes in one file, so no split happened. That is consistent with the threshold above and is recorded because it is where the number came from.

**16. A much larger picture changes nothing but how long the part-written file is visible.** The same animation as test 2 at 8192 × 4608 with 16 samples. On the Mac the file for frame 1 appeared at 105.37 seconds at zero bytes and reached its final 27,913,696 bytes at about 107.5 seconds, climbing through some forty recorded sizes; Blender's own line was `Time: 01:46.35 (Saving: 00:02.13)`. Nothing else appeared in the output folder that was not there at 1920 × 1080. The rented Linux machine agreed.

**17. Two Blenders writing into one folder with Placeholders on and Overwrite off divide the work between them.** **This test was run on the rented Linux machine only**, and it is the one test here whose whole subject is two Blenders at once. Frames 1 to 8, Cycles, 1920 × 1080, the second started three seconds after the first. The first printed:

```
[A] starting frame 1
Saved: '.../out/f_0001.png'
[A] starting frame 3
skipping existing frame ".../out/f_0002.png"
Saved: '.../out/f_0003.png'
[A] starting frame 5
skipping existing frame ".../out/f_0004.png"
Saved: '.../out/f_0005.png'
```

and the second:

```
[B] starting frame 2
skipping existing frame ".../out/f_0001.png"
Saved: '.../out/f_0002.png'
[B] starting frame 4
skipping existing frame ".../out/f_0003.png"
Saved: '.../out/f_0004.png'
```

One took the odd frames and the other the even ones, and no frame was rendered twice. This was a folder on one machine's own disk; a folder shared over a network was not tested.

**What these tests leave open.** Whether any of it differs on Windows — no Windows machine has run it. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.** Whether a folder shared over a network between two machines behaves the same way as the local folder in test 17. Whether the autosplit behaviour in test 15 is the same on macOS, which was not asked. And what a person watching Blender's own window sees when a render fails, which is covered under question 12 below.

**Outside the interface.** The things above are scene settings (`RenderSettings.use_placeholder`, `use_overwrite`, `use_render_cache`, and Cycles' `use_auto_tile`) rather than window settings, and the command-line documentation describes a background render writing frames to the output path in the same way: "`//render_` becomes `//render_####`, writing frames as `//render_0001.png`" (manual — Command Line Arguments, https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html, read 2026-07-30). But the sources nowhere state whether what appears on disk mid-render is the same when Blender is driven from outside its window. On that exact point: **the sources do not say**. The one difference the command-line page does state for background mode is unrelated to writing: "The audio device is disabled in background-mode by default and can be re-enabled by passing in `-setaudio Default` afterwards."

**Settled by the hand tests of 2026-08-11.** It holds. Every test above but one was run with no window at all, so all of it is established for a Blender driven from outside. Test 1 was then run a second time in a Blender **with a window open on screen**, on both machines, and printed the same three lines — the still render wrote nothing until it was asked to, exactly as with no window. Nothing was found that a window changes.

**Operating system.** The sources do not say that any of the output-folder behaviour above differs by operating system. They do say the temporary directory differs: "The temporary directory is selected based on the following priority: User Preference (see File Paths). Environment variables (TEMP on Windows, TMP & TMP_DIR on other platforms). The /tmp/ directory." (manual — Blender's Directory Layout, https://docs.blender.org/manual/en/4.5/advanced/blender_directory_layout.html, read 2026-07-30). Since the sources never say where cached tiles are written, I cannot say whether that per-system difference reaches them.

**Settled by the hand tests of 2026-08-11, and this overtakes the two paragraphs above.** Every one of tests 1 to 14 and 16 above was run on macOS on an Apple M1 Pro and on Linux on an x86 machine, and the two agreed on all of them. So none of it is a macOS-only or Linux-only behaviour. The per-system difference the sources describe is real and it does reach the tile cache: the same `cycles-tile-buffer-…-0-0.exr` file sat under `/var/folders/…/T/blender_IRniBY/` on macOS and under `/tmp/blender_VOCB9C/` on Linux — the same file in the place each system keeps temporary files. Nothing about **Windows** was tested — no Windows machine has run it — so every claim here leaves Windows open. Test 15, the two-gigabyte video split, and test 17, two Blenders sharing a folder, were run on Linux only and leave macOS open; the reason is written into each.

---

## 12. What does a render do when it must make a folder and cannot?

**LABEL: not in the sources.**

The 4.5 manual, the 4.5 Python API documentation, the 4.5 release notes, and Cycles' own files do not say what a render does when a folder it needs cannot be made. They do not even say whether a render makes a missing output folder in the first place. There is no documented error message, no documented exit code, no documented statement of whether the render stops, skips the frame, or carries on.

### Where I looked

Searched across all four sources (`manual_4.5`, `api_4.5`, `release_notes_4.5`, `cycles_4.5`) for: `mkdir`, "make directory", "make the directory", "create folder", "create the folder", "creates the directory", "create the directory", "creating the directory", "folder does not exist", "does not exist", "missing directory", "nonexistent", "non-existent", "if the directory", "existing directory", "will be created", "created automatically", "is created automatically", "automatically create", "cannot create", "unable to create", "not writable", "write permission", "read-only", "permission", "cannot write", "unable to open", "Cannot save", "failed to", "fails to write", "will fail", "exit code", "return code", "error code", "abort", "aborts".

Pages read in full and ruled out:

- Output panel — https://docs.blender.org/manual/en/4.5/render/output/properties/output.html — describes the File Path, Overwrite, Placeholders, Cache Result, formats and encoding; says nothing about folders being made.
- Rendering Animations — https://docs.blender.org/manual/en/4.5/render/output/animation.html — the workflow page; assumes the output folder exists ("use your file manager to navigate to the output folder").
- Output Introduction — https://docs.blender.org/manual/en/4.5/render/output/introduction.html
- File Paths (path templates) — https://docs.blender.org/manual/en/4.5/files/file_paths.html
- File Output Node — https://docs.blender.org/manual/en/4.5/compositing/types/output/file_output.html — describes the Base Path but not what happens if it is not there.
- Blender's Directory Layout — https://docs.blender.org/manual/en/4.5/advanced/blender_directory_layout.html
- File Paths preferences — https://docs.blender.org/manual/en/4.5/editors/preferences/file_paths.html
- Command Line Arguments — https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html — documents `-o`, `-a`, `-f`, `-b`; no folder creation, no exit codes.
- API: RenderSettings (https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html), including `filepath` and `frame_path()`, which is documented only as "Return the absolute path to the filename to be written for a given frame" — it returns a path, and the documentation says nothing about creating anything.
- API: Render Operators (https://docs.blender.org/api/4.5/bpy.ops.render.html) — `bpy.ops.render.render` and `bpy.ops.render.opengl` are documented with their parameters only; no failure behaviour.
- API: Application Handlers (https://docs.blender.org/api/4.5/bpy.app.handlers.html) — there is `render_cancel` "on canceling a render job" and `render_complete` "on completion of render job", but nothing tied to a write or folder failing.
- Release notes 4.5: Rendering, Core, Python API and the Render & Cycles section of Bugfixes — nothing about folder creation. (https://developer.blender.org/docs/release_notes/4.5/rendering/, /core/, /python_api/, /bugfixes/)
- Cycles' own files: searched every Python file for `mkdir`, `makedirs`, `os.path`, "temp" and "cache". The only file-writing code there is the OSL shader compiler's use of temporary files; nothing about render output folders.

### The nearest things the sources do say — none of which answers the question

These are here so the silence can be checked, not as an answer.

1. The manual does state folder creation where a feature does it. For the Ocean modifier: "If the cache path folder does not exist, it will be created." Source: manual — Ocean Modifier. Address: https://docs.blender.org/manual/en/4.5/modeling/modifiers/physics/ocean.html Date read: 2026-07-30. Label: **documented** — about the Ocean modifier's cache, not about rendering.

2. The one place the manual states Blender refusing to make a folder is the temporary files preference: "The directory for storing temporary save files. The path must reference an existing directory or it will be ignored and the systems temporary directory will be used instead." Source: manual — File Paths preferences. Address: https://docs.blender.org/manual/en/4.5/editors/preferences/file_paths.html Date read: 2026-07-30. The command-line page says the same of `TMPDIR`: "The path must reference an existing directory or it will be ignored." (https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html). Label: **documented** — about the temporary directory, not the render output path.

3. Inside Blender's own file browser, a missing folder is offered to the person: "If you type a nonexistent path, you will be prompted to create it." Source: manual — File Browser. Address: https://docs.blender.org/manual/en/4.5/editors/file_browser.html Date read: 2026-07-30. Label: **documented** — this is a person typing in a file browser, not a render.

4. The only documented way a bad render output path stops a render in 4.5 is a path-template error, which is a different fault: "Depending on the path, template errors may prevent certain actions. For example, if there are errors in the render output path, then rendering an animation will fail with an error message indicating the path errors." Source: manual — File Paths. Address: https://docs.blender.org/manual/en/4.5/files/file_paths.html Date read: 2026-07-30. Label: **documented**. The release notes record the matching API change: "`scene.render.frame_path()` used to be infallible but can now fail if there are errors processing Path Templates in the render output path." Source: release notes — Python API. Address: https://developer.blender.org/docs/release_notes/4.5/python_api/ Date read: 2026-07-30. Label: **documented**. Neither is about a folder that cannot be made.

**NOT COVERED.** This answer settles nothing about the actual behaviour: not whether the folder is made at all, not whether several missing levels are made at once, not what is printed or reported, not whether the frame is skipped or the whole render stops, not what a run started from outside gives back, and not whether a File Output node behaves like the main output path.

*All of that has since been settled by hand tests, immediately below.*

---

### What a running Blender 4.5 showed — hand tests, 2026-08-11

**LABEL: observed.** Run on 2026-08-11 on the two computers described under question 11 above — the Creator's MacBook Pro with an Apple M1 Pro chip on macOS 26.5.2, and a rented Ubuntu 22.04.5 machine on kernel 6.8 — with Blender 4.5.12 LTS, `-b --factory-startup`, each run pointed at a settings folder of its own. On the Linux machine the runs were made by an ordinary account rather than root, because root is not stopped by an unwritable folder. Every scene was Workbench at 320 × 240, frames 1 to 3, PNG. The exact commands are in `Audits/Blender Research/Writing/Hand tests — cross-sweep questions.md`.

**18. A missing output folder is made — one level or a whole chain.** With the output path set to `<a writable folder>/newfolder/f_`, the render printed `Saved:` three times and the script printed:

```
operator returned: {'FINISHED'}
does the folder exist now: True
folder holds: ['f_0001.png', 'f_0002.png', 'f_0003.png']
```

With three missing levels at once, `<a writable folder>/a/b/c/f_`, the same. Exit code 0. Both machines agreed.

**19. When the folder cannot be made, the render stops at the very first frame, says so, and ends with 1.** The parent folder was made and then set to read-and-enter only, and the output path named a folder inside it that did not exist. On the **Mac**:

```
does the folder exist before: False
.../work/e18b/4.5/locked/newfolder/f_0001.png: No such file or directory
Time: 00:00.11

Error: Render error (No such file or directory) cannot save: '.../work/e18b/4.5/locked/newfolder/f_0001.png'
operator raised: RuntimeError Error: Render error (No such file or directory) cannot save: '.../work/e18b/4.5/locked/newfolder/f_0001.png'

does the folder exist now: False
```

with `exit code: 1`. Frames 2 and 3 were never attempted. Nothing at all was left behind — the locked folder was still empty afterwards. Three missing levels gave the same. On the **rented Linux machine** the behaviour was identical but the wording was not:

```
Error: Render error (Invalid argument) cannot save: '/home/smoozy/crosssweep/work/e18b/4.5/locked/newfolder/f_0001.png'
```

So the two machines agree on what the render does and differ on the reason printed in the brackets. **Neither wording mentions a folder or a permission.** That matters to anything watching Blender's output for this: a program looking for the word "permission" would miss it on both.

**20. A file standing where the folder must go gives a third wording.** An ordinary file called `blocker` was made, and the output path set to `<that file>/f_`. The Mac printed `Error: Render error (Not a directory) cannot save: …`; the rented Linux machine printed `Error: Render error (Invalid argument) cannot save: …`. Exit code 1 in both, nothing written, render stopped at the first frame.

**21. A File Output node that cannot make its folder does *not* stop the render, and the exit code stays 0.** A File Output node was aimed at a folder inside the read-only parent, while the scene's own output path pointed somewhere that worked. On the Mac:

```
Error: Render error (No such file or directory) cannot save: '.../work/e20b/4.5/locked/nodeout/n_0001.png'
Error: Render error (No such file or directory) cannot save: '.../work/e20b/4.5/locked/nodeout/n_0002.png'
Error: Render error (No such file or directory) cannot save: '.../work/e20b/4.5/locked/nodeout/n_0003.png'
operator returned: {'FINISHED'}
```

with `exit code: 0`, and all three of the scene's own frames written. The rented Linux machine agreed on the behaviour, and printed `(Invalid argument)` for the first frame and `(No such file or directory)` for the second and third. **So the same failure stops the render when it is the scene's own path and does not stop it when it is a node's, and in the second case nothing outside Blender can tell from the exit code that anything went wrong.**

**22. A folder asked for by the name of a node's input is made, and behaves the same way when it cannot be.** The node's input was named `passes/diffuse`. Into a writable folder, 4.5 made a real `passes` folder:

```
.../nodeopen/passes/diffuse0001.png
.../nodeopen/passes/diffuse0002.png
.../nodeopen/passes/diffuse0003.png
```

Into a folder that could not be written to, the same three errors as test 21 and the same `operator returned: {'FINISHED'}` with exit code 0. Both machines agreed. (In Blender 5.0 the slash in that name becomes an underscore and no folder is made; in 5.1 and 5.2 a folder is made but under a different name. That is a difference between versions, not between machines.)

**23. The render cache folder is made when missing, and when it cannot be made the render carries on and finishes.** With Cache Result on and the render cache folder pointed at a folder that did not exist under a writable parent, the folder and the cache file were both made. Pointed inside the read-only parent, the render wrote all three of its frames and printed, per frame:

```
IMB_exr_begin_write: ERROR: No such file or directory.
Error: Error writing render result, No such file or directory (see console)
```

`operator returned: {'FINISHED'}`, exit code 0, and the cache folder was never made. Both machines agreed.

**24. With a window open on screen, the terminal says exactly the same thing.** The first set-up of test 19 was run again in a Blender with a real window, on both machines, and printed the same two lines and raised the same error to Python. Blender was then asked to take a picture of its own window. **What the status bar and the Info list show could not be settled**: the picture shows Blender's first-run Quick Setup panel sitting over the middle of the window and an empty Info list behind it, and dismissing that panel needs a click, which nothing in this round does. That half of the test is dropped for want of a hand on a mouse. The terminal half is complete and is the answer above.

---

### The two disputed wordings, run again — hand tests, 2026-08-12

**LABEL: observed.** Entries **25** and **26** of `Audits/Blender Research/Writing/Hand tests — cross-sweep questions.md` were run on 2026-08-12. They exist because a repeat of two earlier runs came back with different words from what was written down, and neither run was preferred over the other. Both entries were run on **both** computers:

- **The Creator's Mac** — a MacBook Pro with an Apple M1 Pro chip, 16 GB of memory, macOS 26.5.2, Apple's own graphics and no second card.
- **A rented Linux machine** — Ubuntu 22.04.5 LTS on kernel 6.8, two NVIDIA RTX 4000 Ada Generation cards of 20 GB each on driver 580.105.08, an AMD Ryzen Threadripper PRO 3945WX with eleven cores allotted, 49 GB of memory.

Blender 4.5.12 LTS on both, started with `-b --factory-startup`, each run pointed at a settings folder of its own that was made before Blender was started. On the Linux machine the runs were made by an ordinary account called `smoozy`, not root. Only one Blender ran at a time on each machine. Blender's whole terminal output was kept, unfiltered. **The words are written down per machine and the two machines are not merged into one answer.** In the blocks below only the leading part of each long path is shortened to `...`; every word of every message is as the machine printed it.

**25. (Entry 25.) Test 22's failing run again — and the two machines print different words.** The File Output node's own base folder was made first and then set to read-and-enter only, so it exists and cannot be written to. The node's one input was named `passes/diffuse`. The scene's own output path pointed at a folder that works. Workbench, 320 × 240, frames 1 to 3, PNG.

On the **Creator's Mac**, Blender printed:

```
VERSION 4.5.12 LTS
File Output node built in the 4.5 shape
slot path set to 'passes/diffuse'
node folder exists before: True
Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:41:35)
Fra:1 Mem:11.63M (Peak 13.05M) | Time:00:00.10 | Compositing
.../work/e25/4.5/nodelocked/passes/diffuse0001.png: No such file or directory
Error: Render error (No such file or directory) cannot save: '.../work/e25/4.5/nodelocked/passes/diffuse0001.png'
Saved: '.../work/e25/4.5/scene/s_0001.png'
Time: 00:00.16 (Saving: 00:00.01)

Fra:2 Mem:13.70M (Peak 18.33M) | Time:00:00.00 | Compositing
.../work/e25/4.5/nodelocked/passes/diffuse0002.png: No such file or directory
Error: Render error (No such file or directory) cannot save: '.../work/e25/4.5/nodelocked/passes/diffuse0002.png'
Saved: '.../work/e25/4.5/scene/s_0002.png'
Time: 00:00.00 (Saving: 00:00.00)

Fra:3 Mem:13.71M (Peak 18.40M) | Time:00:00.00 | Compositing
.../work/e25/4.5/nodelocked/passes/diffuse0003.png: No such file or directory
Error: Render error (No such file or directory) cannot save: '.../work/e25/4.5/nodelocked/passes/diffuse0003.png'
Saved: '.../work/e25/4.5/scene/s_0003.png'
Time: 00:00.00 (Saving: 00:00.00)

operator returned: {'FINISHED'}
node folder exists now: True
scene folder holds: [(.../scene/s_0001.png, 52290), (.../scene/s_0002.png, 52290), (.../scene/s_0003.png, 52290)]
node folder holds: []
```

with `exit code: 0`.

On the **rented Linux machine**, the same run printed the same shape with one word different, on the first frame only:

```
VERSION 4.5.12 LTS
File Output node built in the 4.5 shape
slot path set to 'passes/diffuse'
node folder exists before: True
Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:31:22)
Fra:1 Mem:11.07M (Peak 12.49M) | Time:00:00.38 | Compositing
/home/smoozy/crosssweep/work/e25/4.5/nodelocked/passes/diffuse0001.png: No such file or directory
Error: Render error (Invalid argument) cannot save: '/home/smoozy/crosssweep/work/e25/4.5/nodelocked/passes/diffuse0001.png'
Saved: '/home/smoozy/crosssweep/work/e25/4.5/scene/s_0001.png'
Time: 00:00.49 (Saving: 00:00.01)

Fra:2 Mem:13.07M (Peak 17.77M) | Time:00:00.00 | Compositing
/home/smoozy/crosssweep/work/e25/4.5/nodelocked/passes/diffuse0002.png: No such file or directory
Error: Render error (No such file or directory) cannot save: '/home/smoozy/crosssweep/work/e25/4.5/nodelocked/passes/diffuse0002.png'
Saved: '/home/smoozy/crosssweep/work/e25/4.5/scene/s_0002.png'
Time: 00:00.01 (Saving: 00:00.00)

Fra:3 Mem:13.07M (Peak 17.77M) | Time:00:00.00 | Compositing
/home/smoozy/crosssweep/work/e25/4.5/nodelocked/passes/diffuse0003.png: No such file or directory
Error: Render error (No such file or directory) cannot save: '/home/smoozy/crosssweep/work/e25/4.5/nodelocked/passes/diffuse0003.png'
Saved: '/home/smoozy/crosssweep/work/e25/4.5/scene/s_0003.png'
Time: 00:00.01 (Saving: 00:00.00)

operator returned: {'FINISHED'}
node folder exists now: True
node folder holds: []
```

with `exit code: 0`.

**The two machines did not agree on the words printed.** On the first frame the Mac said `(No such file or directory)` and the rented Linux machine said `(Invalid argument)`. On the second and third frames both said `(No such file or directory)`. The line above the error — the bare `<path>: No such file or directory` — was identical on both machines for all three frames. Everything else matched: the render carried on to all three frames, the scene's own three frames were written, the node's folder was left empty, `operator returned: {'FINISHED'}` and the exit code was 0 on both.

> **This does not agree with test 22 above, and neither run is chosen.** Test 22 records this failing case as "the same three errors as test 21 … Both machines agreed." Test 21, recorded on the same day, has the Mac printing `(No such file or directory)` three times and Linux printing `(Invalid argument)` for the first frame and `(No such file or directory)` for the second and third — which is exactly the pattern the 2026-08-12 run produced. So the sentence "Both machines agreed" in test 22 stands against the 2026-08-12 run, which found them printing different words on the first frame. Both records stand side by side. Nobody in this round was in the room for the earlier one.

**26. (Entry 26.) Test 23's failing run again — the two machines agree in Blender 4.5.** Cache Result on, the render cache folder pointed at a folder that does not exist under a parent set to read-and-enter only. Entry 26 was run twice per version, because its own written set-up says Workbench at 320 × 240 while the run it points back at, test 23, was Cycles at 1920 × 1080 with 400 samples and two view layers. Neither was preferred: run A repeated test 23's own set-up and run B used the set-up entry 26 itself is written with. **The added step was the second run; nothing else was changed.** Both runs gave the same words in Blender 4.5.

On the **Creator's Mac**, run B (Workbench, 320 × 240) printed:

```
VERSION 4.5.12 LTS
use_render_cache = True
render_cache_directory = '.../work/e26b/4.5/locked/cachefolder'
view layers: ['ViewLayer', 'SecondLayer']
cache folder exists before the render: False
Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:41:35)
Caching exr file, 320x240, .../work/e26b/4.5/locked/cachefolder/cached_RR_UNSAVED_Scene_00000000000000000000000000000000.exr
IMB_exr_begin_write: ERROR: No such file or directory.
Error: Error writing render result, No such file or directory (see console)
Saved: '.../work/e26b/4.5/out/f_0001.png'
Time: 00:00.14 (Saving: 00:00.05)
```

— the same three lines again for frames 2 and 3 — and ended:

```
operator returned: {'FINISHED'}
SCENE FOLDER FINAL: ['f_0001.png', 'f_0002.png', 'f_0003.png']
CACHE FOLDER FINAL: not there
```

with `exit code: 0`. Run A (Cycles, 1920 × 1080) printed the same lines with `Caching exr file, 1920x1080, …` in place of `320x240`.

On the **rented Linux machine**, both runs printed the same two error lines, word for word:

```
IMB_exr_begin_write: ERROR: No such file or directory.
Error: Error writing render result, No such file or directory (see console)
```

with `operator returned: {'FINISHED'}`, `SCENE FOLDER FINAL: ['f_0001.png', 'f_0002.png', 'f_0003.png']`, `CACHE FOLDER FINAL: not there` and `exit code: 0`.

**The two machines agreed, word for word, in all four runs.** This matches what test 23 above records for Blender 4.5. The disagreement entry 26 was written to chase is in Blender 5.1, not here — but the run was made in 4.5 as well, so that the four versions can be laid side by side. Laid out, the first of the two lines reads:

| Version | The Creator's Mac | The rented Linux machine |
|---|---|---|
| 4.5 | `IMB_exr_begin_write: ERROR: No such file or directory.` | `IMB_exr_begin_write: ERROR: No such file or directory.` |
| 5.0 | `ERROR IMB_exr_begin_write: No such file or directory.` | `ERROR IMB_exr_begin_write: No such file or directory.` |
| 5.1 | `ERROR Unknown error in IMB_exr_begin_write` | `ERROR IMB_exr_begin_write: No such file or directory.` |
| 5.2 | `ERROR Unknown error in IMB_exr_begin_write` | `ERROR IMB_exr_begin_write: No such file or directory.` |

The second line was `Error writing render result, No such file or directory (see console)` in every version on both machines.

---

**Outside the interface.** The sources do not say. Nothing in the command-line documentation covers a render whose output folder cannot be made, and no exit code or printed message is documented anywhere for it.

**Settled by the hand tests of 2026-08-11.** It holds. Every test above was run with no window, so all of it is established for a Blender driven from outside — including the exit codes, which are 1 when the scene's own output folder cannot be made and 0 when only a File Output node's or the render cache's cannot. Test 24 repeated the failure in a Blender with a window and the terminal said the same thing. What a person at the screen would see was not settled, for the reason given in test 24.

**Still holding after the hand tests of 2026-08-12.** Tests 25 and 26 were also run with no window, on both machines, and both ended with exit code 0 while printing their failure on the terminal. Nothing changed.

**Operating system.** The sources do not say. The only per-operating-system difference documented anywhere near this ground is where the temporary directory lives, which is a different thing.

**Settled by the hand tests of 2026-08-11, and this overtakes the paragraph above.** What the render *does* is the same on macOS and on Linux in every test: the folder is made when it can be, the render stops at the first frame and ends with 1 when the scene's own folder cannot be made, and it carries on and ends with 0 when a node's or the cache's cannot. **What differs between the two systems is the reason printed in the brackets** — the Mac said `(No such file or directory)` and `(Not a directory)` where Linux said `(Invalid argument)` for both. Nothing about **Windows** was tested — no Windows machine has run it — so Windows stays open on all of it.

**Brought up to date by the hand tests of 2026-08-12.** Two more places on this ground are now on record, both on both machines:

- **A File Output node whose own base folder cannot be written to (test 25): the two machines print different words.** The Mac said `(No such file or directory)` on the first frame, the rented Linux machine said `(Invalid argument)` on the first frame, and both said `(No such file or directory)` on the second and third. That adds a case to the pattern already recorded above rather than changing it. It also stands against test 22's sentence "Both machines agreed"; both records stand, and neither is chosen.
- **The render cache folder that cannot be made (test 26): the two machines agree, word for word.** In Blender 4.5 both printed `IMB_exr_begin_write: ERROR: No such file or directory.` This is the one place on this ground where 4.5 does *not* differ between the two systems. In 5.1 and 5.2 the same test does differ between them; the table in test 26 lays out all four versions.

Windows is still untested on every one of these, for the same reason: the runs used one Mac and one rented Linux machine, and no Windows machine was rented for them.

---

## Things a person with Blender open could settle by hand

**Items 1 to 13 of the original list were all run on 2026-08-11, on the Creator's Mac and on a rented Linux machine, and their results are written into questions 11 and 12 above.** What is left is only what those two machines could not reach.

1. **On Windows**, everything under question 12: an output folder that cannot be made, a folder denied by the account's permissions, and a drive letter that does not exist (`Z:\out\`). Record the exact message and the exit code (`echo %ERRORLEVEL%`). **Not run with the equipment used** — the runs used one Mac and one rented Linux machine, and no Windows machine was rented for them. The same goes for every question-11 result above: all of them leave Windows open.
2. **A folder shared over a network between two machines.** The manual's render-farm recipe is about several machines writing into one shared network folder. Test 17 above ran two Blenders into one folder on a single machine's own disk, which is not the same thing. Joining two machines over a network share is not among the equipment written down for this project, so it needs the Creator's say-so before anybody runs it.
3. **What Blender's own window shows when a render cannot make its folder** — the status bar and the Info list, word for word. The terminal half of this was settled (question 12, test 24) but the picture Blender took of its own window has the first-run Quick Setup panel over it and an empty Info list behind, and dismissing that panel needs a click. Anybody with a hand on a mouse can settle it in a minute: set the output path into a folder you cannot write to, press Ctrl-F12, and read the status bar and the Info editor.
4. **Whether the two-gigabyte video split behaves the same on macOS.** Test 15 above was run on the rented Linux machine only, because it writes several gigabytes four times over and the Creator's Mac is their working machine.

---

2026-08-11 — Claude Opus 5 (Claude Code) — Writing, Blender 4.5

2026-08-11 — Claude Opus 5 (Claude Code) — hand tests: twenty-four observed results written into questions 11 and 12, both standing lines brought up to date, and the researcher's own list pruned to what two machines cannot reach

2026-08-12 — Claude Opus 5 (Claude Code) — second batch of hand tests: entries 25 and 26 run on both machines and written into question 12 as tests 25 and 26, both standing lines brought up to date, and one disagreement with test 22 recorded side by side rather than resolved. Nothing on the list above closed or opened; it is unchanged.

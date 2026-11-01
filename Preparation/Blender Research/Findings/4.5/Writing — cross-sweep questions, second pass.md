# Writing — cross-sweep questions, second pass (Blender 4.5)

Sources consulted, and the date: Blender's 4.5 manual, Blender's 4.5 Python API documentation, Blender's 4.5 release notes, and Cycles' own files for 4.5 — all as supplied in `Blender Sources/`, snapshot taken 2026-07-30, which is the date read for everything quoted below. One picture from the manual was opened and is named where it is used.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

Question answered here: 11 only.

---

## 11. While a render is still working, what is present in the places it writes to, and how does that change with the size of the picture?

### The question was ambiguous in two places, and here is the reading I answered

**"The places it writes to."** I answered it as the places the scene names for output — the render output path, and the base paths of any File Output nodes. The other reading is wider: every place on disk a running render touches, which would also take in Blender's temporary directory and the render cache folder. Because the only thing the sources say about picture size sits in that wider circle, I have answered the narrow reading first and then covered the wider one separately, marked as such. If the wider reading was meant, part B is the answer.

**"The size of the picture."** I answered it as the number of pixels across and down — what the manual calls the resolution. The other reading is the size of the produced file in bytes. I have covered that reading too, at the end, because one documented behaviour turns on it.

For the first reading, the manual defines the term I am using:

> "Resolution X, Y
>
> The number of pixels horizontally and vertically in the image."

Manual, https://docs.blender.org/manual/en/4.5/render/output/properties/format.html, read 2026-07-30.

---

### Part A — what is in the output place while the render is still working

**LABEL: mixed.** Each piece below carries its own label. The short version: what finished frames are doing there is **documented**; what the frame currently being worked on looks like is **not in the sources**.

#### A1. Finished frames are already there — **documented**

> "While rendering stills will allow you to view and save the image from the render buffer when it is complete, animations are a series of images, or frames, and are automatically saved directly out to a drive after being rendered."

Manual, https://docs.blender.org/manual/en/4.5/render/output/animation.html, read 2026-07-30.

The same page says the frames already done stay put if the render stops:

> "since the frames you have already rendered will still be in the output directory."

Manual, same page and address, read 2026-07-30.

So partway through an animation render, the output place holds the frames that have finished, as ordinary finished image files.

The API documentation names an event tied to that moment:

> "bpy.app.handlers.render_write"
>
> "on writing a render frame (directly after the frame is written)"

Python API documentation, https://docs.blender.org/api/4.5/bpy.app.handlers.html, read 2026-07-30.

#### A2. Empty files can also be there, one per frame, if Placeholders is turned on — **documented**

> "Placeholders
>
> Create empty placeholder frames while rendering."

Manual, https://docs.blender.org/manual/en/4.5/render/output/properties/output.html, read 2026-07-30.

The API documentation says the same thing with more detail, and gives the default:

> "use_placeholder"
>
> "Create empty placeholder files while rendering frames (similar to Unix ‘touch’)"
>
> "boolean, default False"

Python API documentation, https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html, read 2026-07-30.

The setting is off unless someone turns it on. The manual shows it off in the picture of the panel:

Picture opened: `_images/render_output_properties_output_panel.png`, which is the "Output panel." picture on https://docs.blender.org/manual/en/4.5/render/output/properties/output.html. **Documented** from the picture: the panel shows a path of `/tmp/`, with Overwrite ticked and File Extensions ticked, and Placeholders unticked and Cache Result unticked. Nothing else about mid-render contents is visible in it.

The manual gives the reason the feature exists, which is the clearest statement of what the output place looks like mid-render when several machines share it:

> "Primitive Render Farm
>
> An easy way to get multiple machines to share the rendering workload is to:
>
> Set up a shared directory over a network file system.
>
> Disable Overwrite, enable Placeholders in the Render Output panel.
>
> Start as many machines as you wish rendering to that directory."

Manual, https://docs.blender.org/manual/en/4.5/render/output/properties/output.html, read 2026-07-30.

The companion setting, quoted for completeness:

> "Overwrite
>
> Overwrite existing files when rendering."

Manual, same page and address, read 2026-07-30. And in the API documentation:

> "use_overwrite"
>
> "Overwrite existing files while rendering"
>
> "boolean, default True"

Python API documentation, https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html, read 2026-07-30.

**Inferred, and here is the step:** the manual's render-farm recipe only works if the empty file for a frame appears *before* that frame is rendered, so that another machine seeing it will skip that frame. Getting there needs a reader to supply that the empty file is created first and the frame data written into it afterwards. The sources never say that in so many words, so this is inferred, not documented.

The 4.5 release notes record a fix in this area, which I quote as a documented statement of change and nothing more:

> " * Placeholder output files wrong name"

Release notes, https://developer.blender.org/docs/release_notes/4.5/bugfixes/, read 2026-07-30. This is one line in the "Render & Cycles" list of that page. It says a name was wrong and is now fixed; it does not say what the right name is.

#### A3. What the file for the frame *currently* being rendered looks like — **not in the sources**

I could not find any statement about whether a file exists for the frame being worked on when Placeholders is off, whether such a file grows as the render proceeds, whether it is written all at once at the end, or whether a half-finished file is left behind if the render is stopped. Where I looked is in the record at the bottom.

#### A4. Rendering straight to a movie file — **documented**, with one **inferred** step

> "The Direct Approach, which is highly not recommended and not a standard practice, is where you set your output format to an AVI or MOV format, and click Animation to render your scene directly out to a movie file. Blender creates one file that holds all the frames of your animation."

Manual, https://docs.blender.org/manual/en/4.5/render/output/animation.html, read 2026-07-30.

> "It’s not recommended to render directly to a video format in the first instance. If a problem occurs while rendering, the file might become unplayable and you will have to re-render all frames from the beginning."

Manual, https://docs.blender.org/manual/en/4.5/render/output/properties/output.html, read 2026-07-30.

**Inferred, and here is the step:** for a file to be left unplayable by a problem *during* the render, the single movie file must already exist and already hold something while the render is still working. The source does not say that outright; a reader has to supply it. So: one movie file, present and incomplete, is inferred, not documented.

---

### Part B — what changes with the size of the picture

**LABEL: documented for the mechanism, not in the sources for where the data lands.**

There is exactly one place in the whole of the 4.5 manual where the size of the picture is tied to something being written to disk during a render, and it is a Cycles setting.

> "Use Tiling
>
> Render high resolution images in tiles to reduce memory usage. Tiles are cached to disk while rendering to save memory.
>
> Tile Size
>
> This value is used to control the size of the tile used for rendering; decreasing the size reduces memory usage."

Manual, https://docs.blender.org/manual/en/4.5/render/cycles/render_settings/performance.html, read 2026-07-30.

The same wording sits in Cycles' own files, together with the default and the range, which the manual does not give. **Observed** by reading the file:

File: `Blender Sources/cycles_4.5/properties.py`, lines 1051 to 1061.

```
    use_auto_tile: BoolProperty(
        name="Use Tiling",
        description="Render high resolution images in tiles to reduce memory usage, using the specified tile size. Tiles are cached to disk while rendering to save memory",
        default=True,
    )
    tile_size: IntProperty(
        name="Tile Size",
        default=2048,
        description="",
        min=8, max=8192,
    )
```

Read 2026-07-30.

So: the setting is a true/false switch called Use Tiling, on by default, and a whole number called Tile Size, 2048 by default, allowed between 8 and 8192.

**Inferred, and here is the step:** the manual says tiles are for "high resolution images" and that Tile Size sets "the size of the tile used for rendering". From those two together, a picture wider or taller than the tile size must be covered by more than one tile, and the number of tiles rises as the picture grows. So the amount of tile data cached to disk during a render grows with the size of the picture. The sources do not state that; the reader supplies the arithmetic. Inferred.

**Not in the sources — and this is the important gap.** Nowhere do the sources say *where* those cached tiles are written, what the file is called, how many files there are, or whether they are removed when the render finishes. The manual does say what the temporary directory holds:

> "The temporary directory is used to store various files at run-time (including render layers, physics cache, copy-paste buffer and crash logs)."

Manual, https://docs.blender.org/manual/en/4.5/advanced/blender_directory_layout.html, read 2026-07-30.

That names "render layers", not Cycles tiles. I am deliberately not joining those two statements: nothing in the sources says the Cycles tile cache is the same thing as the "render layers" in the temporary directory, and guessing that it is would be exactly the kind of step the labels here exist to stop.

The same page gives where the temporary directory is:

> "The temporary directory is selected based on the following priority:
>
> User Preference (see File Paths).
>
> Environment variables (TEMP on Windows, TMP & TMP_DIR on other platforms).
>
> The /tmp/ directory."

Manual, same page and address, read 2026-07-30.

**A note on picture size and the output place itself, not the tile cache: not in the sources.** Nothing I found says the size of the picture changes what is present in the *output* place while a render is working. Placeholder files are described as "empty", which does not vary with the picture. Finished frames are finished frames.

---

### Part C — the other reading of "size", meaning the size of the file in bytes

**LABEL: documented.**

One documented behaviour turns on the size of the growing file rather than the picture:

> "Autosplit Output
>
> If your video is huge and exceeds 2GiB, enable Autosplit Output. This will automatically split the output into multiple files after the first file is 2GiB in size."

Manual, https://docs.blender.org/manual/en/4.5/render/output/properties/output.html, read 2026-07-30.

So with that setting on, a long movie render can have more than one file present in the output place before it is done. **Inferred, and here is the step:** a bigger picture means more data per frame, so the 2GiB mark is reached after fewer frames, so the split happens earlier in the render. The source does not say that; it follows from the two facts together. Inferred, and it is a chain of two steps, so treat it as weak.

---

### Part D — one more place a render can leave something, if asked to

**LABEL: documented.**

> "Cache Result
>
> Saves the rendered view layers and their passes to a multi-layer OpenEXR image. The Compositor can then use this file to improve performance, especially for heavy compositing.
>
> The image is stored in the Render Cache folder as specified in the File Paths Preferences."

Manual, https://docs.blender.org/manual/en/4.5/render/output/properties/output.html, read 2026-07-30.

It is off unless turned on:

> "use_render_cache"
>
> "Save render cache to EXR files (useful for heavy compositing, Note: affects indirectly rendered scenes)"
>
> "boolean, default False"

Python API documentation, https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html, read 2026-07-30.

> "Note that Blender doesn’t create these cache files by default. You have to enable Cache Result in the scene’s Output options and then render it at least once."

Manual, https://docs.blender.org/manual/en/4.5/editors/image/editing.html, read 2026-07-30.

Where they go:

> "Render Cache
>
> The location where cached render images are stored."

Manual, https://docs.blender.org/manual/en/4.5/editors/preferences/file_paths.html, read 2026-07-30. And in the API documentation:

> "render_cache_directory"
>
> "Where to cache raw render results"
>
> "string, default “”, (never None, blend relative // prefix supported)"

Python API documentation, https://docs.blender.org/api/4.5/bpy.types.PreferencesFilePaths.html, read 2026-07-30.

One line hints that these cache files are useful part-way through, and I quote it because it is the nearest the sources come to describing a render that did not finish:

> "Reads all the current scene’s render layers from cache, as needed. This can be used to save RAM while rendering because the render layers do not have to be saved in RAM. And also for recovering some information from a failed render. For this to work, Cache Result must be enabled."

Manual, https://docs.blender.org/manual/en/4.5/interface/controls/nodes/editing.html, read 2026-07-30.

**Inferred, and here is the step:** "recovering some information from a failed render" only makes sense if something was written to the cache before the render failed. The source does not say when during a render the cache file is written. Inferred, and it does not tell us how much is there at any given moment.

---

### What a running Blender 4.5 showed — hand tests, 2026-08-11

**LABEL: observed.** Every result below was produced on 2026-08-11 by running Blender 4.5.12 LTS on two computers and writing down what it printed and what appeared on disk. The two computers are named in full because that is what settles which operating system each answer covers.

- **The Creator's Mac** — a MacBook Pro with an Apple M1 Pro chip, 16 GB of memory, macOS 26.5.2 (build 25F84), Apple's own graphics and no second card. Blender 4.5.12 LTS at `/Applications/Blender 4.5.12.app/Contents/MacOS/Blender`.
- **A rented Linux machine** — Ubuntu 22.04.5 LTS on kernel 6.8, two NVIDIA RTX 4000 Ada Generation cards of 20 GB each on driver 580.105.08, an AMD Ryzen Threadripper PRO 3945WX with eleven cores, 49 GB of memory. Blender 4.5.12 LTS at `/opt/blender/4.5/blender`.

Every run used `-b --factory-startup` unless it says otherwise, and each was pointed at a settings folder of its own so that no run touched anybody's saved Blender settings. Where the same test ran on both machines, both results are given and it is said plainly whether they agreed. The whole list of tests, with the exact commands, is `Audits/Blender Research/Writing/Hand tests — cross-sweep questions.md`.

**1. Nothing stands in the output folder for a frame while it is being rendered; while it is being *saved*, its file is there part-written.** Cycles, 1920 × 1080, 400 samples, frames 1 to 4, PNG, Overwrite on, Placeholders off, with a second program listing the folder every fifty milliseconds. On the rented Linux machine:

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

Blender's own line for the first frame was `Time: 00:10.77 (Saving: 00:00.21)`. The Mac agreed, with the same shape and its own timings. **This settles what section A3 above marks *not in the sources*, and it settles it both ways**: for the ten seconds a frame is being rendered there is nothing for it at all, and for the fifth of a second it is being written there is a file that grows. No file under another name appears first, and nothing is renamed.

**2. With Placeholders on, the empty file appears before the frame is rendered — the step section A2 marks inferred is correct.** Same scene, Placeholders on and Overwrite off. On the rented Linux machine:

```
   0.00  .../out  []
   0.30  .../out  [('f_0001.png', 0)]
  10.85  .../out  [('f_0001.png', 40960)]
  10.90  .../out  [('f_0001.png', 737280)]
  10.95  .../out  [('f_0001.png', 1433600)]
  11.00  .../out  [('f_0001.png', 1584041), ('f_0002.png', 0)]
```

The empty file for frame 1 is there three tenths of a second after the render starts, ten seconds before that frame is finished; the empty file for frame 2 appears at the moment frame 1's picture is complete. The placeholders are made one at a time, for the frame in hand, not all at once, and the picture is written into the same file rather than into a second one. The Mac agreed.

**3. A render stopped part way leaves the empty file behind.** Same as test 2, frames 1 to 6, stopped with the ordinary terminate signal once frame 3's placeholder had appeared. Blender ended with exit code 143 and the folder held `f_0001.png` and `f_0002.png` at 1,584,041 bytes each and `f_0003.png` at 0 bytes. The Mac agreed.

**4. A part-written file is visible for as long as the writing takes, and that grows with the picture.** Workbench, one frame at 10000 × 10000, watched every fifty milliseconds. On the Mac, PNG went from `('big_.png', 0)` at 3.96 seconds to `('big_.png', 60377290)` at about 7.5 seconds, through some seventy recorded sizes; Blender's own line was `Time: 00:07.15 (Saving: 00:04.96)`. OpenEXR and multi-layer OpenEXR behaved the same way over about three quarters of a second. The rented Linux machine agreed. The same animation as test 1 run at 8192 × 4608 showed the same thing over about two seconds instead of a fifth of one.

**5. Blender holds the file open while it writes it.** While that multi-layer OpenEXR was being written, asking the system which program had it open returned, on the Mac:

```
COMMAND   PID      USER   FD   TYPE DEVICE SIZE/OFF      NODE NAME
Blender 10209 artist       3w   REG   1,14   206318 103656546 .../work/e06/4.5/out/big_.exr
```

`3w` is a write handle. Renaming the file at that moment succeeded and the render still finished. Linux agreed.

**6. Where the cached tiles go — this settles the gap section B calls the important one.** Cycles, 32 samples, tile size at its default of 2048, one frame at 8000 × 8000, with every file the Blender process had open sampled twice a second. On the Mac:

```
Blender 11500 artist 3u REG 1,14    118784 .../T/blender_IRniBY/cycles-tile-buffer-11500-50886739680-0-0.exr
Blender 11500 artist 3u REG 1,14   2650112 .../T/blender_IRniBY/cycles-tile-buffer-11500-50886739680-0-0.exr
Blender 11500 artist 3r REG 1,14  88768733 .../T/blender_IRniBY/cycles-tile-buffer-11500-50886739680-0-0.exr
```

On the rented Linux machine the same file was `/tmp/blender_VOCB9C/cycles-tile-buffer-23390-133458313353920-0-0.exr`, reaching 89,127,413 bytes. Blender's own progress line said `Rendered 15/16 Tiles` and later `Reading full buffer from disk`. The script printed `tempdir after: []` — the file is gone once the render ends. So: **it is Blender's own temporary folder**, one file, an OpenEXR, named `cycles-tile-buffer-<process number>-<number>-0-0.exr`, and it is cleaned up. The same render at 1000 × 1000 opened no such file at all.

**7. The boundary is one tile's worth of pixels, and in 4.5 the Use Tiling switch still turns the whole thing off.** With the system's temporary folder pointed at an empty folder of the run's own and walked every twenty milliseconds: at **2048 × 2048** nothing appeared and Blender printed no tiling line; at **2048 × 2049** the tile-buffer file appeared and grew to 6,127,934 bytes and Blender printed `Rendered 1/2 Tiles`. Both machines agreed. At 8000 × 8000 with the tile size set to **8192** there was no tile-buffer file and no tiling line at all, because 64 million pixels is fewer than 8192 × 8192. With the tile size at **256** the file appeared and was written to far more often, and Blender said `Rendered 1023/1024 Tiles`. And with `use_auto_tile` set to **False** at the default tile size, **no tile-buffer file appeared at all** — in Blender 4.5 that switch works. (In 5.0, 5.1 and 5.2 the same switch changes nothing.)

**8. EEVEE and Workbench write no such file.** The same 8000 × 8000 frame under `BLENDER_EEVEE_NEXT` and under `BLENDER_WORKBENCH` opened no tile-buffer file and put nothing in the temporary folder, on either machine.

**9. What a File Output node's folder holds while a render works.** Compositor on, one node aimed at a folder that did not exist, separate from the scene's own; Cycles, 1920 × 1080, frames 1 to 4. The script printed `node folder exists before the render: False`, and the render then printed, frame by frame, `Saved: '.../node/n_0001.png'` immediately followed by `Saved: '.../out/f_0001.png'`, and so on. The node's folder is made by the render, and its file for a frame lands just before the scene's own file for that frame. Both machines agreed.

**10. With Cache Result on, a cache file is written for every frame, before that frame's picture — the step section D marks inferred is correct.** Two view layers, Cycles, 1920 × 1080, frames 1 to 3, the render cache folder pointed at an empty folder. Blender printed, once per frame:

```
Caching exr file, 1920x1080, .../cache/cached_RR_UNSAVED_Scene_00000000000000000000000000000000.exr
```

and the watcher showed the file appearing at 21.20 seconds and reaching 132,729,040 bytes before frame 1's own PNG appeared at 21.40 seconds. There is one file, not one per frame: it drops to zero and grows again for each frame, and it is still there at the end. Both machines agreed.

**11. A video render puts one file in the folder from the start; stopping it leaves something that will not play.** Workbench, 1280 × 720, FFmpeg in a Matroska container with H.264, frames 1 to 4000, stopped after 25 seconds. On the Mac the file `movie_0001-4000.mkv` — named for the whole frame range from the first moment — appeared at 0 bytes at 0.80 seconds and grew in steps of 262,144 bytes to 1,310,720 bytes. Blender had reached `Append frame 1499` and exited 143. Handing that file back to Blender 4.5 gave `loaded as a movie: movie_0001-4000.mkv  frames: 0  size: (256, 256)` — it would not open. Linux agreed. **This test was run twice**: the first run used only 200 frames and finished in under four seconds, before it could be stopped, so it was run again longer; the short run is recorded because it shows the finished file opening properly, `frames: 200  size: (1280, 720)`.

**12. Past two gigabytes with Autosplit on, a second file appears — at about 1.87 GiB, not 2.** Workbench, 1920 × 1080, FFmpeg in an AVI container with the lossless HUFFYUV codec, 1400 frames. **Run on the rented Linux machine only**, because it writes several gigabytes four times over and the Mac is the Creator's working machine; it settles Linux and leaves macOS open. With Autosplit on: `FINAL: [('long__0000001-1400.avi', 2003416672), ('long__0010001-1400.avi', 1022242144)]`. With it off, one file of 3,025,649,192 bytes. **This test was also run twice**: a first run of 900 frames reached only 1,945,065,136 bytes and never split, which is where the figure above comes from.

**13. No texture cache is written beside the pictures a render reads.** A PNG written by Blender itself, used as an image texture, rendered with Cycles at 4096 × 4096: the folder holding that PNG contained only the PNG before and after. Asking Blender 4.5 what settings it has for this printed `no such setting: use_texture_cache` and `no such setting: use_auto_generate_texture_cache`. Both machines agreed. (Blender 5.2 has both of those switches and, with them on, writes a `blender_tx/` folder beside the picture. 4.5 has neither.)

**14. Two Blenders writing into one folder with Placeholders on and Overwrite off divide the work.** **Run on the rented Linux machine only**, and the one test here whose whole subject is two Blenders at once. Frames 1 to 8, the second started three seconds after the first. One took the odd frames and the other the even ones, each printing `skipping existing frame "<path>"` for the frames the other had claimed, and no frame was rendered twice. This was a folder on one machine's own disk; a folder shared over a network was not tested.

**What these tests leave open.** Windows, everywhere — no Windows machine has run it. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.** A folder shared over a network between two machines. And whether the two-gigabyte split behaves the same on macOS.

---

### NOT COVERED

*Most of this list has since been settled by the hand tests above. The parts of it that stand are named in the note after it.*

This answer does not settle any of the following.

- Whether a file exists in the output place for the frame currently being rendered, when Placeholders is off. Nothing found either way.
- Whether an image file is built up on disk as the frame renders, or written in one go when the frame is finished.
- What is left in the output place if a render is stopped or crashes part-way through a single frame.
- Where the Cycles tile cache is written, what it is called, how big it is, and whether it is cleaned up.
- The exact picture size at which Cycles tiling starts splitting a picture into more than one tile. The default tile size of 2048 is observed from Cycles' files, but nothing says the rule that compares it to the picture.
- Whether a placeholder file is created before its frame is rendered or after. The render-farm recipe only makes sense the first way, but that is my inference.
- What a File Output node leaves in its base path while a render is working. The node's page describes what it writes, not when. See the record below.
- Anything about EEVEE or Workbench and disk use during a render. I searched and found nothing comparable to the Cycles tiling statement.
- File sizes, in bytes, of anything.

**What the hand tests of 2026-08-11 settled from that list, and what is left.** Settled: whether a file exists for the frame being rendered with Placeholders off (nothing while it renders, a growing file while it saves); whether an image file is built up on disk (it is); what is left after a stop; where the Cycles tile cache is, what it is called, how big it gets and whether it is cleaned up; the exact picture size at which tiling begins; whether the placeholder is made before its frame (it is) and what becomes of it after a cancel (it stays, at zero bytes); when a File Output node writes; the name of the Render Cache file and that it is one file rewritten per frame; whether EEVEE and Workbench write anything (they do not); and file sizes in bytes throughout. **Still open**: everything on Windows, a folder shared over a network, whether the two-gigabyte video split behaves the same on macOS, and anything about multi-view or stereoscopy output, which nothing here tested.

### Outside the interface

Does this hold when Blender is driven from outside its own window, started and controlled by something other than a person clicking? **The sources do not say, and here is what they do and do not give.**

Every setting above is a property of the scene, not of a window: Overwrite, Placeholders and Cache Result are listed in the API documentation as attributes of `bpy.types.RenderSettings` (https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html, read 2026-07-30), and Use Tiling and Tile Size are properties declared in Cycles' own `properties.py` (lines 1051 to 1061). **Inferred, and here is the step:** because they live in the scene, they travel with the file and apply to a render started any way. The sources never say that these particular settings behave the same in background mode, so this is inferred.

I checked the command-line arguments page for any statement that these differ when Blender is run headless. The page lists `-b, --background` ("Run in background (often used for UI-less rendering)."), `-a, --render-anim`, `-f, --render-frame`, and `-o, --render-output`, and says nothing at all about placeholders, overwriting, tiling, or the render cache. Manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html, read 2026-07-30.

One difference is documented, and it is about the render statistics rather than the files: the API documentation says the render statistics handler reports "the render stats (render/saving time plus in background mode frame/used [peak] memory)" — that is, extra numbers appear in background mode. Python API documentation, https://docs.blender.org/api/4.5/bpy.app.handlers.html, read 2026-07-30. It tells us nothing about what is on disk.

**Settled by the hand tests of 2026-08-11.** It holds. Every test above was run with no window at all, so all fourteen results are established for a Blender driven from outside its own window. One test was then repeated in a Blender **with a window open on screen**, on both machines — a still render with the output path set to an empty folder — and it printed exactly the same thing as with no window: `AFTER render, no save switch: []`, then `AFTER render with the save switch: ['still_.png']`. Nothing was found that a window changes.

### Operating system

Does this hold on every operating system Blender runs on? **Partly documented, partly not said.**

Nothing in any of the quotes above about the output place is marked as belonging to one operating system. Placeholders, Overwrite, Cache Result, Use Tiling and Tile Size carry no operating-system note anywhere I looked.

Where the sources *do* record a difference is in where the temporary directory sits — which matters only for the wider reading of "the places it writes to":

> "Environment variables (TEMP on Windows, TMP & TMP_DIR on other platforms)."

Manual, https://docs.blender.org/manual/en/4.5/advanced/blender_directory_layout.html, read 2026-07-30.

The command-line page names these slightly differently, and I report both rather than choosing:

> "TEMP:
>
> Store temporary files here (MS-Windows).
>
> TMPDIR:
>
> Store temporary files here (UNIX Systems). The path must reference an existing directory or it will be ignored."

Manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html, read 2026-07-30.

The API description of Placeholders reaches for a Unix comparison — "similar to Unix ‘touch’" — but that is a description of the effect, not a statement that it only happens on Unix. I am not reading anything more into it.

**Settled by the hand tests of 2026-08-11, and this overtakes the paragraphs above.** Tests 1 to 11 and 13 were each run on macOS on an Apple M1 Pro and on Linux on an x86 machine, and the two machines agreed on every one of them, so none of this is a macOS-only or Linux-only behaviour. The per-system difference the sources describe is real and it does reach the tile cache: the very same `cycles-tile-buffer-…-0-0.exr` file sat under `/var/folders/…/T/blender_IRniBY/` on macOS and under `/tmp/blender_VOCB9C/` on Linux — the same file, in the place each system keeps temporary files. Nothing about **Windows** was tested — no Windows machine has run it — so every claim here leaves Windows open. Test 12, the two-gigabyte split, and test 14, two Blenders sharing a folder, were run on Linux only and leave macOS open; the reason is written into each.

---

### Record of where I looked

All searching was done over the plain-text copies in `Blender Sources/manual_4.5`, `Blender Sources/api_4.5`, `Blender Sources/release_notes_4.5` and `Blender Sources/cycles_4.5`, searching each whole folder at once rather than opening files and hoping.

**Terms searched across the whole manual:** placeholder; save buffers; temporary file; temp directory; temporary directory; while rendering; during rendering; during the render; partially written; incomplete file; empty file; zero-byte; 0 bytes; touch; progressiv; incrementally; as it renders; in progress; unfinished; half-written; truncated; flush; streamed; cached to disk; to disk; on disk; swap; tile; tiles; tile size; tiling; tiled; high resolution; large image; larger image; image size; resolution near memory; memory near resolution; output folder; output directory; render cache; render animation.

**Terms searched across the whole API documentation:** use_placeholder; use_overwrite; save_buffers; render_cache; use_render_cache; render_write; render_post; render_stats; render_cancel; tile.

**Terms searched across the whole release notes:** tile; placeholder; output director; temporary; partial; output; render cache; file path; write.

**Terms searched across Cycles' own files:** tile; auto_tile; .exr; temp; tmp; and, in the kernel and util folders, tile near file, disk or cache.

**Pages opened and read in full:**

- Output — https://docs.blender.org/manual/en/4.5/render/output/properties/output.html
- Rendering Animations — https://docs.blender.org/manual/en/4.5/render/output/animation.html
- Performance (Cycles) — https://docs.blender.org/manual/en/4.5/render/cycles/render_settings/performance.html
- Format — https://docs.blender.org/manual/en/4.5/render/output/properties/format.html
- Introduction (render output) — https://docs.blender.org/manual/en/4.5/render/output/introduction.html
- File Output Node — https://docs.blender.org/manual/en/4.5/compositing/types/output/file_output.html
- Render Operators — https://docs.blender.org/api/4.5/bpy.ops.render.html

**Pages opened and read in part, at the places the searches pointed to:**

- Directory Layout, the Temporary Directory section — https://docs.blender.org/manual/en/4.5/advanced/blender_directory_layout.html
- Command Line Arguments, the render options and the environment variables — https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html
- File Paths preferences, the Render section — https://docs.blender.org/manual/en/4.5/editors/preferences/file_paths.html
- Image editor editing, the Open Cached Render section — https://docs.blender.org/manual/en/4.5/editors/image/editing.html
- Node editing, the Layers section — https://docs.blender.org/manual/en/4.5/interface/controls/nodes/editing.html
- Topbar, the Render menu — https://docs.blender.org/manual/en/4.5/interface/window_system/topbar.html
- GPU Rendering, the error messages section — https://docs.blender.org/manual/en/4.5/render/cycles/gpu_rendering.html
- Color Management, the section on saved renders — https://docs.blender.org/manual/en/4.5/render/color_management.html
- bpy.types.RenderSettings — https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html
- bpy.app.handlers — https://docs.blender.org/api/4.5/bpy.app.handlers.html
- bpy.types.PreferencesFilePaths — https://docs.blender.org/api/4.5/bpy.types.PreferencesFilePaths.html
- Release notes, Bugfixes, the Render & Cycles list — https://developer.blender.org/docs/release_notes/4.5/bugfixes/
- Release notes, Rendering — https://developer.blender.org/docs/release_notes/4.5/rendering/
- Release notes, Pipeline, Assets & I/O — https://developer.blender.org/docs/release_notes/4.5/pipeline_assets_io/
- Release notes, Core — https://developer.blender.org/docs/release_notes/4.5/core/
- Release notes, Cycles — https://developer.blender.org/docs/release_notes/4.5/cycles/

**Picture opened:** `_images/render_output_properties_output_panel.png`, the "Output panel." picture on the Output page above.

**Cycles files opened:** `Blender Sources/cycles_4.5/properties.py` (lines 1040 to 1069). I also listed everything in `Blender Sources/cycles_4.5` to see what was there. What is supplied is the add-on's Python files (`__init__.py`, `camera.py`, `engine.py`, `operators.py`, `osl.py`, `presets.py`, `properties.py`, `ui.py`, `version_update.py`), the compiled shader files, the licences, and a `source` folder holding only the render kernel and its utilities — header files for cameras, closures, geometry, integrator, light, sampling and so on. There is nothing there that manages tiles, output files or paths, which is why the tile cache's location cannot be settled from what was supplied.

**What I ruled out, and why:**

- **"Save Buffers".** I looked for it by name across the manual, the API documentation and the release notes, in both spellings (`save buffers` and `save_buffers`). No match anywhere in 4.5. I mention it only because it is the obvious name for a mid-render disk buffer and someone will ask; in 4.5 there is nothing under that name.
- **"Use Placeholders" in the Video Sequencer.** Search for "placeholder" turns up `video_editing/edit/montage/strips/image.txt` and two sequencer operators in the API documentation. That is a different feature — standing in for missing frames when *adding* an image strip — and has nothing to do with what a render writes. Ruled out.
- **"Placeholder" in the glTF exporter and in link/append.** Also turned up by the same search, also unrelated. Ruled out.
- **The File Output node page.** I read it in full hoping for a statement about when it writes. It says "The image(s) will be saved on rendering, writing to the current frame" and "An entire sequence of images will be saved, when an animation is rendered", and nothing about what is in the base path part-way through. Nothing usable for this question.
- **The render operators page.** Read in full. `bpy.ops.render.render` and `bpy.ops.render.opengl` document `animation` and `write_still` and nothing about intermediate files. Nothing usable.
- **Geometry-nodes baking, physics caches, asset library paths.** These came up in the "on disk" and "cache" searches and are not renders. Ruled out.
- **The Autosplit 2GiB behaviour** is included above, but only under the byte-size reading, because it is about the size of the file and not the size of the picture.

---

### Things a person sitting at a computer with Blender open could settle in a few minutes

**All eight items of the original list were run on 2026-08-11, on the Creator's Mac and on a rented Linux machine, and their results are written into the answer above.** What is left is only what those two machines could not reach.

1. **Anything on Windows.** Every result above holds for macOS and for Linux and says nothing about Windows. **Not run with the equipment used** — the runs used one Mac and one rented Linux machine, and no Windows machine was rented for them.
2. **A folder shared over a network between two machines.** Test 14 above ran two Blenders into one folder on a single machine's own disk. The manual's render-farm recipe is about a shared network folder, which is not the same thing, and no network share between two machines is among this project's written-down equipment. It needs the Creator's say-so before anybody runs it.
3. **Whether the two-gigabyte video split behaves the same on macOS.** Test 12 was run on the rented Linux machine only, because it writes several gigabytes four times over and the Mac is the Creator's working machine.
4. **Multi-view or stereoscopy output during a render.** Nothing here tested it, and the answer above still does not cover it.

---

2026-08-11 — Claude Opus 5 (Claude Code) — Writing, Blender 4.5

2026-08-11 — Claude Opus 5 (Claude Code) — hand tests: fourteen observed results written into question 11, both standing lines brought up to date, and the researcher's own list pruned to what two machines cannot reach

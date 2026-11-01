# Writing — Open Threads questions — Blender 5.1

This document holds one research pass over question 14 of the Writing question sheet, for Blender 5.1, and answers no other question on that sheet.

Sources consulted, all on 2026-07-30 (snapshot date): the Blender 5.1 manual (`Blender Sources/manual_5.1`, including one image from its `_images` folder), the Blender 5.1 Python API documentation (`Blender Sources/api_5.1`), the Blender 5.1 release notes (`Blender Sources/release_notes_5.1`), and Cycles' own files (`Blender Sources/cycles_5.1`).

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

---

## Question 14

**When a render works through a range of frames, how do the writes belonging to one frame stand in time against the writes belonging to the frame after it?**

### Short answer

The sources say a frame is saved to disk after that frame has been rendered, and that the range is stepped forward after each rendered frame. Taken together those two statements put the saved image of frame N ahead of the render — and so ahead of the saved image — of frame N+1. That is as far as it goes. The sources do not place a frame's *other* writes (a placeholder, a JPG preview, a File Output node's own file) against the next frame's at all. They do not state that frame N's file is finished, flushed or closed before any work on frame N+1 starts, they do not rule out that part of saving frame N overlaps the next frame, and they give no timings. So the ordering claim is an inference, it covers the saved image only, and the "no overlap" claim is not in the sources at all.

### LABEL

**Mixed.** Broken out by claim:

| Claim | Label |
|---|---|
| Each frame of an animation is saved to disk after that frame is rendered | documented |
| The render steps forward through the range after each rendered frame | documented |
| That the step is one frame — that a plain animation render skips nothing | **not carried by the quoted words** (they give the setting and say a value above 1 skips frames; they do not give its default, and the record of looking below holds no search for the setting's default) |
| There is a per-frame "the frame has been written" moment a script can hook | documented |
| Frame N's saved image therefore precedes frame N+1's saved image | **inferred** (step written out below) |
| Where a frame's other writes — a placeholder, a JPG preview, a File Output node's file — fall against the next frame's | **not in the sources** |
| Frame N's file is complete/closed before any of frame N+1's writing begins — i.e. the two never overlap | **not in the sources** |
| Whether placeholder files are made for all frames up front or one per frame just before it renders | **not in the sources** (an inference is offered, labelled) |
| Any timing, gap or duration between one frame's write and the next | **not in the sources** |

### What the sources do say

**1. A frame is written after it is rendered.**

Exact words:

> "animations are a series of images, or frames, and are automatically saved directly out to a drive after being rendered."

- Source: manual, *Rendering Animations*, https://docs.blender.org/manual/en/5.1/render/output/animation.html
- Read: 2026-07-30

And, for the still-image case:

> "Blender creates a file for each frame of the animation."

- Same page and address, read 2026-07-30.

**2. The range is stepped forward after each rendered frame.**

Exact words:

> "-j, --frame-jump <frames>
>
> Set number of frames to step forward after each rendered frame."

- Source: manual, *Command Line Arguments*, https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html
- Read: 2026-07-30

The matching setting in the interface says the same thing:

> "Controls the number of frames to advance for each frame rendered, a value above 1 skips frames."

- Source: manual, *Frame Range*, https://docs.blender.org/manual/en/5.1/render/output/properties/frame_range.html
- Read: 2026-07-30

Neither quote gives the setting's default, so "one frame at a time" is not carried by them. What they carry is that the range moves forward by some number of frames after each rendered frame, and that number can be more than one.

**3. There is a per-frame write event.**

Exact words:

> "bpy.app.handlers.render_write
>
> on writing a render frame (directly after the frame is written). Accepts one argument: the scene data-block being rendered"

- Source: Python API, *Application Handlers (bpy.app.handlers)*, https://docs.blender.org/api/5.1/bpy.app.handlers.html
- Read: 2026-07-30

The frame is also a boundary for the render engine itself:

> "render_frame_finish()
>
> Perform finishing operations after all view layers in a frame were rendered"

- Source: Python API, *RenderEngine*, https://docs.blender.org/api/5.1/bpy.types.RenderEngine.html
- Read: 2026-07-30

Cycles has that entry point in its own files:

> ```
> def render_frame_finish(engine):
>     if not engine.session:
>         return
>
>     import _cycles
>     _cycles.render_frame_finish(engine.session)
> ```

- Source: Cycles' own files, `Blender Sources/cycles_5.1/engine.py`, lines 95–100.
- Label for this line alone: **observed** (this is what the file contains).
- Read: 2026-07-30

**4. Frames already rendered are still in the output folder when a stopped render is picked up again.**

Exact words:

> "since the frames you have already rendered will still be in the output directory."

and

> "Just disable the Overwrite option to start rendering where you left off."

- Source: manual, *Rendering Animations*, https://docs.blender.org/manual/en/5.1/render/output/animation.html
- Read: 2026-07-30

> "when rendering animations to image file formats the render job can be canceled and resumed at the last rendered frame by changing the frame range."

- Source: manual, *Introduction* (Output), https://docs.blender.org/manual/en/5.1/render/output/introduction.html
- Read: 2026-07-30

**5. Which writes belong to a frame (this affects what "one frame's writes" means).**

A frame can carry more than one write. The sources name these:

- The main image or the frame's slice of the video file. For video: "Encodes each frame into a video container." — manual, *Output*, https://docs.blender.org/manual/en/5.1/render/output/properties/output.html, read 2026-07-30. In that case the writes for successive frames go into the *same* file, except that "Autosplit Output" will "automatically split the output into multiple files after the first file is 2GiB in size." (same page).
- An empty placeholder file, if enabled: "Create empty placeholder files while rendering frames (similar to Unix 'touch')" — Python API, *RenderSettings*, https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html, read 2026-07-30. The manual's wording for the same option is "Create empty placeholder frames while rendering." (manual *Output* page, same address as above).
- A JPG preview alongside the frame, if enabled: "When rendering animations, save JPG preview images in same directory" — Python API, *ImageFormatSettings*, https://docs.blender.org/api/5.1/bpy.types.ImageFormatSettings.html, read 2026-07-30.
- A File Output node's own files: "The File Output node writes one or more images to disk during rendering or compositing." — manual, *File Output Node*, https://docs.blender.org/manual/en/5.1/compositing/types/output/file_output.html, read 2026-07-30.
- Cycles' own files carry a Tile Size setting whose description says tiles are cached to disk while rendering. Exact lines:

> ```
>     tile_size: IntProperty(
>         name="Tile Size",
>         default=2048,
>         description="Render high resolution images in tiles of this size, to reduce memory usage. Tiles are cached to disk while rendering to save memory",
>         min=8,
>         max=8192,
>     )
> ```

- Source: Cycles' own files, `Blender Sources/cycles_5.1/properties.py`, lines 1056–1062.
- Label: **observed** that the file contains this setting, with that name, default and range, and that description. That a render actually writes tile data to disk, and that it does so during that frame's own render and so before that frame's saved output, is **inferred** — the step is that the description says the caching is done "while rendering" and its stated purpose is to hold parts of the picture being rendered, which cannot be after the picture is finished and saved. A settings file says a setting exists; it does not say what a running render does with it.

**6. The interface labels, from the picture.**

The manual's Output panel screenshot shows "Overwrite" (ticked) and "Placeholders" (unticked) side by side, above "File Extensions" and "Cache Result".

- Source: manual, image `_images/render_output_properties_output_panel.png`, shown on https://docs.blender.org/manual/en/5.1/render/output/properties/output.html
- Label for this: **documented** (it is part of the manual).
- Read: 2026-07-30
- Note: the text of that page prints the heading "Image Sequence – Overwrite" twice, once for each of the two options. That the second of those headings belongs to the Placeholders checkbox is **inferred** — the step is that the picture shows exactly two checkboxes there, Overwrite and Placeholders, and the page's two blocks of text describe two different options, so the second block is read onto the second checkbox. Nothing on the page says so.

### The inference, written out

From (1) "saved directly out to a drive after being rendered", and (2) the range advancing "after each rendered frame", the chain for a two-frame stretch is: frame N is rendered → frame N is saved → the range steps forward → frame N+1 is rendered → frame N+1 is saved. Frame N's saved image therefore falls before frame N+1's saved image. The reader has to supply one thing the sources do not print: that the "step forward" happens after the save and not before it. Statement (3) supports that supply — `render_write` fires "directly after the frame is written" and is a per-frame event, and `render_frame_finish` marks a per-frame boundary — but neither of them says the step comes after the write in so many words. So: **inferred**, not documented.

**How far that reaches, and no further.** The chain is about the frame's saved image, which is the only write any of those quoted words names. A frame can carry other writes — the ones listed at (5) — and nothing quoted places those in the chain. So this does not say that *every* write belonging to frame N falls before *every* write belonging to frame N+1; it says the saved images fall in that order.

The render-farm hint carries a second, weaker inference about placeholders:

> "Set up a shared directory over a network file system.
>
> Disable Overwrite, enable Placeholders in the Render Output panel.
>
> Start as many machines as you wish rendering to that directory."

- Source: manual, *Output*, https://docs.blender.org/manual/en/5.1/render/output/properties/output.html, read 2026-07-30.

For several machines to divide a range this way, each machine must be able to see which frames another machine has already claimed. That only works if a frame's placeholder appears before that frame's long render, and if placeholders are *not* all created at the start (if they were, every machine would find every frame taken and none would render anything). That reasoning gives: placeholder for frame N → render of frame N → saved file for frame N → placeholder for frame N+1. It is **inferred**; the sources nowhere state when placeholders are created, only that it happens "while rendering frames".

### Not covered

- Where a frame's writes other than its saved image fall against the next frame's: a placeholder file, a JPG preview, a File Output node's own file. Nothing quoted places any of them in the frame-by-frame chain.
- Whether frame N's file is fully written, flushed and closed on disk before *any* byte belonging to frame N+1 is written. The sources are silent on overlap. Nothing found says saving is deferred, queued, or handed to a background worker; nothing found says it is not.
- Any timing: how long a frame's write takes, whether there is a measurable gap between the end of one frame's write and the start of the next, or whether the gap is the render time of the next frame.
- Whether the ordering is guaranteed or merely usual — nothing in the sources frames it as a promise.
- What happens when the frame order is not a plain forward range. The command line accepts "A comma separated list of frames" and relative frames (`+`/`-`) for `--render-frame`; the sources do not say in what order such a list is worked through.
- Multiple machines writing to one directory: the ordering above is per render job. The sources do not say how two jobs' writes to the same folder stand against each other beyond the render-farm hint quoted above.
- The video case is only partly covered: the frames go into one file, but the sources do not say when each frame's data actually reaches disk, nor when the container is finalised. **The record of looking below holds no search of the sources for the words this would be written in**, so this is a gap in the searching rather than an established silence.
- Multi-view / stereoscopy: the order of the two views' writes within one frame was not established. **The record of looking below holds no search of the sources for the words this would be written in**, so this too is a gap in the searching rather than an established silence.
- The File Output node: its page was opened at its opening section only, so nothing here rests on a full reading of it.

### Outside the interface

Nothing quoted above is stated as being limited to Blender's own window, and most of it is written for exactly the outside case. The command-line page states `-a, --render-anim` "Render frames from start to end (inclusive)" and `-j, --frame-jump` steps "after each rendered frame" (manual, *Command Line Arguments*, https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html, read 2026-07-30) — these are command-line arguments, so the stepping statement is made about the outside-the-interface case directly.

That the Python API handlers (`render_write`, `render_pre`, `render_post`, `render_stats`) behave the same way for a script handed in at launch is **inferred** — the reader has to supply that a handler is unchanged by how Blender was started. No quoted words say so.

One documented difference in what background mode reports, not in what it writes:

> "on printing render statistics. Accepts one argument: the render stats (render/saving time plus in background mode frame/used [peak] memory)."

- Source: Python API, *Application Handlers*, https://docs.blender.org/api/5.1/bpy.app.handlers.html, read 2026-07-30. So the statistics named there include a saving time, and in background mode also a frame number and a memory figure. Whether the saving time is reported apart from the render time, or as one combined figure, is not settled by those words — the slash carries either reading.

I found nothing in the sources saying the *ordering* of writes differs between a person clicking Render Animation and a job started from the command line.

**Brought up to date by the hand tests of 2026-08-30.** This is now settled by test rather than left to the sources. The same five-frame render was run in this version on two machines, once with no window at all and once with Blender's own window open, the render started the way the Render Animation menu item starts it — as a job the interface keeps running alongside. **The ordering was the same in all four runs**: for every frame, the write event for frame N came before the render event for frame N+1, and on disk no file was still growing when the next frame's file appeared. See *What the hand tests settled — question 14* at the foot of this document, tests 1 and 13.

### Operating system

The sources say nothing that ties this ordering to any operating system. I found no operating-system caveat on any of the pages quoted above. The only near-miss is the API's description of placeholders, which reaches for a Unix comparison — "similar to Unix 'touch'" (Python API, *RenderSettings*, read 2026-07-30) — but that is a description of what the file is, not a statement that the behaviour is limited to Unix-like systems. Whether the platform's own file system buffering makes a completed write visible to another machine or another program at a different moment is not addressed anywhere I searched.

**Brought up to date by the hand tests of 2026-08-30.** Two operating systems are now settled by test: **macOS**, on the Creator's own Mac, and **Linux**, on a rented machine whose Blender matched the Mac's to the exact build hash. The two agreed on every test. **Windows is still not settled** — not by the sources and not by any test — and four Windows entries are written out ready to run in `Audits/Blender Research/Writing/Hand tests — Open Threads questions.md`. A Windows result must name the edition it was taken on. See *What the hand tests settled — question 14* at the foot of this document.

---

## Record of where I looked

**Files and folders searched:** the whole of `Blender Sources/manual_5.1`, the whole of `Blender Sources/api_5.1`, the whole of `Blender Sources/release_notes_5.1`, and `Blender Sources/cycles_5.1`. I did not read the 4.5, 5.0 or 5.2 copies.

**Search terms used across the manual and/or the API** (case-insensitive): `one frame at a time`, `frame by frame`, `sequentially`, `in sequence`, `next frame`, `previous frame is`, `before moving on`, `after each frame`, `each frame is`, `each frame`, `frame is written`, `frame is saved`, `saved directly`, `already rendered`, `left off`, `resume`, `skip`, `ascending`, `in order`, `order of`, `one after`, `simultaneous`, `in parallel`, `at the same time`, `background thread`, `separate thread`, `while the next`, `queue`, `pipelin`, `atomic`, `flush`, `truncat`, `incomplete file`, `partial`, `overlap`, `Saving:`, `Saving `, `file is closed`, `touch`, `Placeholder`, `Overwrite`, `render farm`, `shared directory`, `network file system`, `Render Animation`, `render-anim`, `render-frame`, `frame-jump`, `background mode`, `while rendering an animation`, `animation render`, `render_write`, `render_pre`, `render_post`, `render_init`, `render_complete`, `render_cancel`, `render_stats`, `render_frame_finish`, `use_placeholder`, `use_overwrite`.

**Pages opened in full:**
- manual `render/output/animation.txt` → https://docs.blender.org/manual/en/5.1/render/output/animation.html
- manual `render/output/introduction.txt` → https://docs.blender.org/manual/en/5.1/render/output/introduction.html
- manual `render/output/properties/output.txt` → https://docs.blender.org/manual/en/5.1/render/output/properties/output.html
- manual `render/output/properties/frame_range.txt` → https://docs.blender.org/manual/en/5.1/render/output/properties/frame_range.html
- manual `render/output/properties/post_processing.txt` → https://docs.blender.org/manual/en/5.1/render/output/properties/post_processing.html
- manual `advanced/command_line/render.txt` → https://docs.blender.org/manual/en/5.1/advanced/command_line/render.html
- manual `advanced/command_line/arguments.txt` (relevant parts) → https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html
- manual `compositing/types/output/file_output.txt` (opening section) → https://docs.blender.org/manual/en/5.1/compositing/types/output/file_output.html
- manual image `_images/render_output_properties_output_panel.png`
- API `bpy.app.handlers.txt` → https://docs.blender.org/api/5.1/bpy.app.handlers.html
- API `bpy.types.RenderEngine.txt` (relevant parts) → https://docs.blender.org/api/5.1/bpy.types.RenderEngine.html
- API `bpy.types.RenderSettings.txt` (relevant parts) → https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html
- API `bpy.types.ImageFormatSettings.txt` (relevant part) → https://docs.blender.org/api/5.1/bpy.types.ImageFormatSettings.html
- API `bpy.ops.render.txt` (the `render` and `opengl` operators) → https://docs.blender.org/api/5.1/bpy.ops.render.html
- release notes `rendering.md` → https://developer.blender.org/docs/release_notes/5.1/rendering/
- release notes `pipeline_io.md` → https://developer.blender.org/docs/release_notes/5.1/pipeline_io/
- release notes `core.md` and `python_api.md`, searched → https://developer.blender.org/docs/release_notes/5.1/core/ and https://developer.blender.org/docs/release_notes/5.1/python_api/
- Cycles' own files: `engine.py` (whole), `properties.py` (searched, tile section read), plus a search of every `.py` in `cycles_5.1` for `write`, `save`, `output`, `frame`.

**What I ruled out, and why:**
- The 5.1 release notes: searched for `background thread`, `asynchronous`, `async`, `in parallel`, `concurrently`, `while rendering`, `saving of`, `save.*thread` across all twenty-one files — no matches. The rendering and pipeline/IO notes contain nothing about when frames are written relative to one another. The only saving-related note is about a single format's internal threading, not about frame order.
- Cycles' own files: they contain the add-on's settings and its entry points into the render session. Nothing in them writes the animation's output files or controls the frame loop, so they cannot settle the ordering. The one write they do describe is the within-frame tile cache, quoted above.
- The Video Sequencer, Grease Pencil, physics and modelling parts of the manual: matches for my terms there were about editing and playback, not about a render writing files.
- EEVEE and Cycles render-settings pages: searched for `simultaneous`, `in parallel`, `at the same time` — matches were about devices and CPU cores inside a single render, not about frames or files.

---

## What the hand tests settled — question 14

Fourteen tests were run for this question on **2026-08-30** by Claude Opus 5 in Claude Code, in all four researched versions of Blender, on two machines. Everything below is what **Blender 5.1.2** did. The full list, with every script written out in full and every entry's negative result stated, is `Audits/Blender Research/Writing/Hand tests — Open Threads questions.md`.

### The two machines

**The Creator's own Mac.** A MacBook Pro with an Apple M1 Pro chip, macOS **26.6.2** (build 25G83, kernel `Darwin 25.6.0`, arm64), Apple's own graphics and no second card. Blender **5.1.2**, hash `ec6e62d40fa9`, started from `/Applications/Blender 5.1.2.app/Contents/MacOS/Blender`. No system setting on that machine was changed.

**A rented Linux machine, in the United Kingdom.** Vast.ai instance `49255001`, machine `143739`, rented and destroyed on 2026-08-30. **A container, not a whole machine**, established by reading rather than assumed: `/.dockerenv` is present, `systemd-detect-virt` says `docker`, `/proc/1/cgroup` prints `0::/`, and process 1 is the provider's own launch script. Ubuntu **22.04.4 LTS**, kernel `7.0.0-28-generic`, glibc 2.35, x86-64. An **AMD Ryzen 9 3900X** with 24 visible threads against a quota of **11.52 cores**; **the container misreports its own size** — the ordinary memory reading says 50,423,779,328 bytes while its own ceiling is 34,227,617,792. A 40 GB overlay root. One **NVIDIA GeForce RTX 5060 with 8151 MiB on driver 595.71.05**. **No display server as rented**; `Xvfb` was added to it for the window test, where the graphics are software (`llvmpipe`). Blender **5.1.2**, hash `ec6e62d40fa9`, at `/opt/blender/5.1/blender`, taken from Blender's own download server as the Linux x64 build — **its hash matches the Mac's exactly**, so any difference between the two machines is a difference of operating system and not of build.

**Every render on both machines was a processor render.** Cycles was set to `CPU` throughout, except in the one test that used EEVEE.

**The two results are never merged.** The Mac settles macOS; the Linux machine settles Linux. **Windows is settled by neither.** Four Windows entries are written out ready to run in the list named above, for the runner's separate Windows batch, and no Windows answer is claimed here.

### How fine the instrument was

This question can be answered wrongly by a slow instrument: a watcher reading the folder once a second sees every frame already finished and calls that "one after the other" whatever Blender did. So the folder was read **every two milliseconds** — on the Mac a reading completed every 3.2 milliseconds on average — and the write was made slow on purpose, a 3000 by 3000 OpenEXR at 32 bits with no compression, 144 megabytes a frame. One frame's write took about 190 milliseconds on the Mac and about 60 on Linux, and the watcher caught **63 separate readings of one file growing** on the Mac and **31** on Linux. Two frames' writes running at the same time could not have been missed at that rate. Blender was also asked, not only watched: a script printed a stamped line at every render event. And on Linux **every file call Blender made was traced**, thread by thread, so one result rests on no polling at all.

Every run used `--factory-startup` and Blender's own default scene, so no add-on and no saved preference could reach any of it, and no `.blend` file was used.

### 1. The order Blender itself reports, driven from outside its own window

Five frames to PNG, 512 by 512, one sample. For every frame in the range, `render_write` for frame N was printed **before** `render_pre` for frame N+1.

- On the **Mac**, the four gaps were +0.000027, +0.000030, +0.000024, +0.000032 seconds. Frames out of order: **0**.
- On the **rented Linux machine**, +0.000031, +0.000032, +0.000027, +0.000023 seconds. Frames out of order: **0**.

The two machines agreed.

### 2. The disk itself, with a write slow enough to catch in the act

Five frames, 3000 by 3000 OpenEXR, uncompressed. Each file's first appearance, its last change of size, and its final size — on the **Mac**:

```
frame_0001.exr                     1788091631.172598  1560845 1788091631.172598 1788091631.408915    144048845
frame_0002.exr                     1788091631.962022  2076941 1788091631.962022 1788091632.296256    144048845
frame_0003.exr                     1788091632.831003   770317 1788091632.831003 1788091633.068709    144048845
frame_0004.exr                     1788091633.967852     4941 1788091633.967852 1788091634.233056    144048845
frame_0005.exr                     1788091634.848801   815373 1788091634.848801 1788091635.100956    144048845
```

On the **rented Linux machine**:

```
frame_0001.exr                     1788091539.000307  1851392 1788091539.000307 1788091539.061520    144048845
frame_0002.exr                     1788091540.644417        0 1788091540.646561 1788091540.707732    144048845
frame_0003.exr                     1788091542.291143   120861 1788091542.291143 1788091542.350857    144048845
frame_0004.exr                     1788091543.992617  2146304 1788091543.992617 1788091544.053928    144048845
frame_0005.exr                     1788091545.688756  1851392 1788091545.688756 1788091545.748256    144048845
```

Whether any file's size changed **after** a later file first appeared: **none** on the Mac, **none** on Linux. No name ever appeared in the folder that was not one of the final frame names, so no file was written under a temporary name and moved into place. The two machines agreed.

### 3. Whether the file is finished at the moment Blender says it is written

The write handler asked Blender for the frame's path with `scene.render.frame_path(frame=scene.frame_current)`, then read the whole file and printed the byte count beside the size the file system reported. On the **Mac**:

```
EVT 1788091638.220072 readback frame=1 stat=144048845 read=144048845 path=…/frame_0001.exr
EVT 1788091639.043779 readback frame=2 stat=144048845 read=144048845 path=…/frame_0002.exr
EVT 1788091640.015358 readback frame=3 stat=144048845 read=144048845 path=…/frame_0003.exr
EVT 1788091641.084575 readback frame=4 stat=144048845 read=144048845 path=…/frame_0004.exr
EVT 1788091642.053003 readback frame=5 stat=144048845 read=144048845 path=…/frame_0005.exr
```

On the **rented Linux machine**:

```
EVT 1788091549.375444 readback frame=1 stat=144048845 read=144048845 path=…/frame_0001.exr
EVT 1788091551.124030 readback frame=2 stat=144048845 read=144048845 path=…/frame_0002.exr
EVT 1788091552.875304 readback frame=3 stat=144048845 read=144048845 path=…/frame_0003.exr
EVT 1788091554.657070 readback frame=4 stat=144048845 read=144048845 path=…/frame_0004.exr
EVT 1788091556.412523 readback frame=5 stat=144048845 read=144048845 path=…/frame_0005.exr
```

Those counts are the same numbers test 2 recorded as the finished size of a frame. The two machines agreed.

### 4. Placeholders on, overwrite off

Five frames, the same large EXR, with `use_placeholder` on and `use_overwrite` off. The empty files appear **one at a time, not all at the start**. On the **Mac**:

```
frame_0001.exr                     1788091644.335089        0 1788091645.100876 1788091645.340931    144048845
frame_0002.exr                     1788091645.340931        0 1788091646.058877 1788091646.359156    144048845
frame_0003.exr                     1788091646.359156        0 1788091647.096824 1788091647.313646    144048845
frame_0004.exr                     1788091647.313646        0 1788091648.061639 1788091648.290275    144048845
frame_0005.exr                     1788091648.290275        0 1788091649.054748 1788091649.286094    144048845
```

On the **rented Linux machine**:

```
frame_0001.exr                     1788091558.276257        0 1788091559.909725 1788091559.970654    144048845
frame_0002.exr                     1788091559.970654        0 1788091561.618659 1788091561.684020    144048845
frame_0003.exr                     1788091561.684020        0 1788091563.318113 1788091563.379730    144048845
frame_0004.exr                     1788091563.379730        0 1788091565.008299 1788091565.067932    144048845
frame_0005.exr                     1788091565.067932        0 1788091566.709512 1788091566.773255    144048845
```

The `at_size` column is zero for every file at first sight, and `first_nonzero` is when the picture began to be written into it. The empty file for a frame appears within a few milliseconds of that frame's render beginning, sits at zero for the whole of that frame's render, and then grows to full size; the next frame's empty file appears within one reading of that. Only one empty file is ever waiting at a time, and the write event fires once per frame, not twice. The two machines agreed.

### 5. A File Output node's file against the main output file

Three frames, the same large EXR, with a File Output node in the compositor writing into a second folder and compositing switched on. Both folders were watched together. On the **Mac**:

```
file_name0001.exr                  1788092123.777357  1126580 1788092123.777357 1788092123.999409    144048756
frame_0001.exr                     1788092123.999409   282893 1788092123.999409 1788092124.263649    144048845
file_name0002.exr                  1788092124.972515   970932 1788092124.972515 1788092125.360290    144048756
frame_0002.exr                     1788092125.360290   512269 1788092125.360290 1788092125.751433    144048845
file_name0003.exr                  1788092126.722636    21236 1788092126.722636 1788092127.056801    144048756
frame_0003.exr                     1788092127.059106   286989 1788092127.059106 1788092127.360745    144048845
```

On the **rented Linux machine**:

```
file_name0001.exr                  1788092162.718782  1523712 1788092162.718782 1788092162.780102    144048756
frame_0001.exr                     1788092162.794755  1605632 1788092162.794755 1788092162.858298    144048845
file_name0002.exr                  1788092164.572350   122880 1788092164.572350 1788092164.636645    144048756
frame_0002.exr                     1788092164.651426   163840 1788092164.651426 1788092164.715885    144048845
file_name0003.exr                  1788092166.433896  1658880 1788092166.433896 1788092166.498085    144048756
frame_0003.exr                     1788092166.512915  2134016 1788092166.512915 1788092166.577199    144048845
```

The node's file for a frame is written first, the scene's own output file second, and both are finished before the next frame's node file appears. The two machines agreed.

### 6. The preview picture saved beside a frame

Three frames, the same large EXR, with `image_settings.use_preview` on. On the **Mac**:

```
frame_0001.exr                     1788091656.992422        0 1788091656.995552 1788091657.213705    144048845
frame_0001.jpg                     1788091657.303840        0 1788091657.307083 1788091657.332210       236589
frame_0002.exr                     1788091657.811802      845 1788091657.811802 1788091658.024363    144048845
frame_0002.jpg                     1788091658.074556    12288 1788091658.074556 1788091658.098359       236589
frame_0003.exr                     1788091658.610309   864525 1788091658.610309 1788091658.818938    144048845
frame_0003.jpg                     1788091658.871036        0 1788091658.873775 1788091658.897921       236589
```

On the **rented Linux machine**:

```
frame_0001.exr                     1788091577.311622  1671168 1788091577.311622 1788091577.383070    144048845
frame_0001.jpg                     1788091577.614945        0 1788091577.617088 1788091577.642267       236838
frame_0002.exr                     1788091579.344838  2473253 1788091579.344838 1788091579.412624    144048845
frame_0002.jpg                     1788091579.562344     4096 1788091579.562344 1788091579.589800       236838
frame_0003.exr                     1788091581.237018  2039808 1788091581.237018 1788091581.305093    144048845
frame_0003.jpg                     1788091581.454163     4096 1788091581.454163 1788091581.481868       236838
```

The frame's own file is written first and the preview second, and both are finished before the next frame's file appears. The two machines agreed.

### 7. Two views in one frame

Three frames with `use_multiview` on, `views_format` `STEREO_3D`, and the image settings' `views_format` `INDIVIDUAL`, so each view gets a file. On the **Mac**:

```
frame_0001_L.exr                   1788091662.049758   176397 1788091662.049758 1788091662.268074    144048845
frame_0001_R.exr                   1788091662.268074  1700109 1788091662.268074 1788091662.475771    144048845
frame_0002_L.exr                   1788091663.440954  1831181 1788091663.440954 1788091663.650886    144048845
frame_0002_R.exr                   1788091663.650886  1347853 1788091663.650886 1788091663.845126    144048845
frame_0003_L.exr                   1788091664.814464      845 1788091664.814464 1788091665.052789    144048845
frame_0003_R.exr                   1788091665.055384  1593613 1788091665.055384 1788091665.278297    144048845
```

On the **rented Linux machine**:

```
frame_0001_L.exr                   1788091586.609332    24581 1788091586.609332 1788091586.681193    144048845
frame_0001_R.exr                   1788091586.681193    24581 1788091586.681193 1788091586.755580    144048845
frame_0002_L.exr                   1788091589.926324   131072 1788091589.926324 1788091589.996228    144048845
frame_0002_R.exr                   1788091589.998305  3678208 1788091589.998305 1788091590.064102    144048845
frame_0003_L.exr                   1788091593.194260   114688 1788091593.194260 1788091593.256382    144048845
frame_0003_R.exr                   1788091593.256382   456917 1788091593.256382 1788091593.318646    144048845
```

The left view is written first and the right second, and both are finished before the next frame's left view appears. In several runs the left view's last change and the right view's first appearance fall in the **same** reading; at two milliseconds those two events cannot be told apart and nothing is claimed about them. What is claimed is only the gap of one to three seconds before the next frame's files.

### 8. A video container: does the one file grow as the run goes

Forty frames at 1920 by 1920 into a video container, written losslessly with the `HUFFYUV` codec so that the finished file is about 153 megabytes — far larger than any write buffer. At the default settings the same scene compresses down to about twenty kilobytes for a short run, which is small enough to sit inside a write buffer and settle nothing.

| | Mac | rented Linux machine |
|---|---|---|
| size changes seen during the run | 58 | 59 |
| bytes on disk at the half-way frame | 68,681,728 | 72,089,600 |
| bytes on disk when the last frame was written | 142,606,336 | 141,557,760 |
| final size | 153,142,267 | 153,143,547 |
| size changes after the last frame's write | 5 | 5 |

The file grows steadily through the run: about half of it is on disk by the half-way frame and about nine tenths by the time the last frame is written, with the rest added afterwards. The two machines agreed.

### 9. The same video render stopped part way

Run only on the **rented Linux machine**, because a media reader was needed and `ffprobe` was installed there; nothing was installed on the Creator's Mac. The same forty-frame lossless video render was interrupted after six seconds and the part-finished file handed to `ffprobe`.

A file was left behind, **68,914,985 bytes**. `ffprobe` read it as `matroska,webm` with codec `huffyuv` and a duration of 0.750000 seconds, and a decoder walking it with `-count_frames` read **18 frames**. Blender had announced **18** frames written before the interrupt. *(The frame counts differ between versions because the interrupt landed at a fixed six seconds into renders of different speeds, not because Blender differs.)*

**This settles macOS not at all.** It is a Linux result only.

### 10. A list of frames given out of order instead of a range

Started with Blender's own frame argument rather than from a script — `-o <folder>/f_#### -F OPEN_EXR -x 1 -f 5,1,3` — with a script that only set the engine, samples and resolution and rendered nothing. Blender's own lines on the **Mac**:

```
00:00.459  render           | Saved: '…/f_0005.exr'
00:00.506  render           | Saved: '…/f_0001.exr'
00:00.555  render           | Saved: '…/f_0003.exr'
```

and the watcher's order of first appearance:

```
1788091813.228732  f_0005.exr
1788091813.274988  f_0001.exr
1788091813.325719  f_0003.exr
```

On the **rented Linux machine**:

```
1788091627.847998  f_0005.exr
1788091627.930203  f_0001.exr
1788091627.998434  f_0003.exr
```

The frames are written **in the order the list gives them**, not in numeric order. The two machines agreed.

### 11. The other thing a frame writes: Cycles' tile cache

Three frames at 4000 by 4000 with `use_auto_tile` on and `tile_size` 256, with Blender's temporary folder pointed at a folder of the test's own and watched alongside the output folder. On the **Mac**:

```
cycles-tile-buffer-71793-35976127200-0-0.exr 1788091815.510842     8192 1788091815.510842 1788091817.837882      9563900
frame_0001.exr                     1788091817.837882   389197 1788091817.837882 1788091818.297656    256064845
cycles-tile-buffer-71793-35976127200-1-0.exr 1788091818.306347    12288 1788091818.306347 1788091820.482686      9563900
frame_0002.exr                     1788091820.482686      845 1788091820.482686 1788091820.979134    256064845
cycles-tile-buffer-71793-35976127200-2-0.exr 1788091820.982310        0 1788091820.986212 1788091822.817474      9563900
frame_0003.exr                     1788091822.817474   110669 1788091822.817474 1788091823.258140    256064845
```

and the watcher's record of those files going away:

```
1788091817.837882 GONE  …/cycles-tile-buffer-71793-35976127200-0-0.exr
1788091820.482686 GONE  …/cycles-tile-buffer-71793-35976127200-1-0.exr
1788091822.817474 GONE  …/cycles-tile-buffer-71793-35976127200-2-0.exr
```

On the **rented Linux machine**:

```
cycles-tile-buffer-5351-137455105977088-0-0.exr 1788091629.866133        0 1788091629.868296 1788091632.716947      9563759
frame_0001.exr                     1788091632.719112   622592 1788091632.719112 1788091632.815895    256064845
cycles-tile-buffer-5351-137455105968960-1-0.exr 1788091632.845559        0 1788091632.847724 1788091635.663497      9563759
frame_0002.exr                     1788091635.667772  1825069 1788091635.667772 1788091635.768300    256064845
cycles-tile-buffer-5351-137455106059008-2-0.exr 1788091635.796614     8192 1788091635.796614 1788091638.624223      9563759
frame_0003.exr                     1788091638.628521    24581 1788091638.628521 1788091638.727150    256064845
```

One tile file per frame, written in the temporary folder during that frame's render, finished and **removed** no later than the reading in which that frame's own output file first appears. The two machines agreed. *(Narrowed by the citation check of 2026-08-30: the watcher stamps every event in one sweep with a single time, so where a removal and the next file's first appearance carry the same stamp — two of the three frames in the Mac run above — the log cannot show which came first. Test 14's system-call trace does show it, and shows the removal about two milliseconds before the frame's own file is opened, but that trace was taken on the Linux machine only and does not speak for the Mac.)*

### 12. The same run with EEVEE instead of Cycles

Five frames, the same large EXR, with the engine set to EEVEE. This version offers the engine under the name **`BLENDER_EEVEE`**, which the script printed. On the **Mac**:

```
frame_0001.exr                     1788091673.307877   307280 1788091673.307877 1788091673.514971    144048656
frame_0002.exr                     1788091675.688987   806992 1788091675.688987 1788091675.925124    144048656
frame_0003.exr                     1788091678.074583    21136 1788091678.074583 1788091678.361863    144048656
frame_0004.exr                     1788091680.540016  1605712 1788091680.540016 1788091680.788298    144048656
frame_0005.exr                     1788091682.850670   790608 1788091682.850670 1788091683.199438    144048656
```

On the **rented Linux machine**:

```
frame_0001.exr                     1788091614.017084    24576 1788091614.017084 1788091614.080240    144048656
frame_0002.exr                     1788091614.752417   504736 1788091614.752417 1788091614.819828    144048656
frame_0003.exr                     1788091615.496402        0 1788091615.498548 1788091615.560270    144048656
frame_0004.exr                     1788091616.235608     8192 1788091616.235608 1788091616.303998    144048656
frame_0005.exr                     1788091616.977897   229376 1788091616.977897 1788091617.044638    144048656
```

Whether any file's size changed after a later file appeared: **none** on the Mac, **none** on Linux. The answer is the same as Cycles', so the ordering is not a property of the render engine. The two machines agreed.

### 13. The same render with Blender's own window open

Five frames, the same large EXR, with Blender started **without** `-b`, so it opened its own window, and the render started with `bpy.ops.render.render('INVOKE_DEFAULT', animation=True)` — which is how the Render Animation menu item starts it, as a job the interface keeps running alongside. A script quit Blender once the job reported itself finished, so no hand touched a mouse. On the Mac this was Blender's own window on the Creator's screen; on Linux it was a window on an `Xvfb` display server added to the machine for this test, where the graphics are software.

The script printed `background False` and `WINDOWS 1`, and `INVOKE-RETURNED` printed about twenty milliseconds after the render was started, so the render really was a job running alongside rather than a call that blocks.

For every frame in the range, `render_write` for frame N was printed **before** `render_pre` for frame N+1.

- On the **Mac**, the four gaps were +0.000030, +0.000030, +0.000031, +0.000031 seconds. Frames out of order: **0**.
- On the **rented Linux machine**, +0.000052, +0.000054, +0.000054, +0.000272 seconds. Frames out of order: **0**.

And the disk agreed. On the **Mac**:

```
frame_0001.exr                     1788092874.683105   483597 1788092874.683105 1788092874.919549    144048845
frame_0002.exr                     1788092876.062580    21325 1788092876.062580 1788092876.312195    144048845
frame_0003.exr                     1788092877.419747   373005 1788092877.419747 1788092877.654361    144048845
frame_0004.exr                     1788092878.774057   135437 1788092878.774057 1788092879.046823    144048845
frame_0005.exr                     1788092880.125965   815373 1788092880.125965 1788092880.385090    144048845
```

On the **rented Linux machine**:

```
frame_0001.exr                     1788092606.499887    24581 1788092606.499887 1788092606.565364    144048845
frame_0002.exr                     1788092608.852302    24581 1788092608.852302 1788092608.944611    144048845
frame_0003.exr                     1788092611.065199   270336 1788092611.065199 1788092611.129095    144048845
frame_0004.exr                     1788092612.893977   180224 1788092612.893977 1788092612.960928    144048845
frame_0005.exr                     1788092614.827575   311296 1788092614.827575 1788092614.889566    144048845
```

Whether any file's size changed after a later file appeared: **none** on the Mac, **none** on Linux.

### 14. The file calls themselves, traced thread by thread

Run only on the **rented Linux machine**. `strace` has no equivalent on the Creator's Mac that does not need a system setting changed, and no system setting on that machine was touched. Three frames of the same large EXR, traced with:

```
strace -f -tt -T -s 0 -e trace=openat,creat,write,pwrite64,writev,close,rename,renameat,renameat2,unlink,ftruncate -o trace.txt <blender> -b --factory-startup --python <the script>
```

For each output file: which thread opened it, when it was first and last written, when it was closed, how many write calls it took, and from which threads. The times are seconds since midnight, as `strace` prints them.

```
file                           opened opentid    first_w     last_w     closed   writes  writing threads
cycles-tile-buffer-6232-128041204591360-0-0.exr 43930.212325   6269 43930.213776 43930.980860 43931.349063      461  6269:461
frame_0001.exr             43931.674633   6232 43931.674896 43931.914467 43931.914571     3006  6232:3006
cycles-tile-buffer-6232-128041204583232-1-0.exr 43932.218978   6270 43932.220009 43932.978484 43933.360999      461  6270:461
frame_0002.exr             43933.691395   6232 43933.691653 43933.937639 43933.937804     3006  6232:3006
cycles-tile-buffer-6232-128041204722432-2-0.exr 43934.256826   6271 43934.258053 43935.020537 43935.385810      461  6271:461
frame_0003.exr             43935.711216   6232 43935.711480 43935.962492 43935.962589     3006  6232:3006

Did any write to one frame's file happen after the NEXT frame's file was opened?
  no
```

Three things this shows that no polling could.

- **The frame's own file is opened, written and closed by the process's first thread** — the thread number is the process number itself — with a little over three thousand write calls each, all from that one thread, and the close following the last write within a tenth of a millisecond.
- **Cycles' tile buffer is written by a different thread each frame**, and each one is finished and closed before that frame's own file is opened. The first thread then removes it, a couple of milliseconds before the frame's file is opened. There are exactly three unlink calls in the whole trace, one per frame.
- **There is no rename call anywhere in the trace.** Every file is written under its final name from the first byte.

**This settles Linux only.** The Mac's answer to the same question rests on tests 2 and 3, which cannot see threads.

### What these tests do not settle

- **Windows.** Neither machine speaks for it. The Windows entries are written and waiting.
- **Several machines writing into one shared folder** — the "primitive render farm" the manual describes — and **an output folder on a network share**. Both need a second computer of the same kind and a file system shared between them, which this project's equipment does not provide, and nothing here established that a rented container can mount a network share at all. This is recorded in the list named above under *Cannot be run*.
- **How long any of this took on the rented machine.** Its processor is shared and it misreports its own size. Nothing above turns on a duration; every claim is about what came before what, read from one process's own clock.
- **What a viewport draws, or how fast.** The Linux display server's graphics are software.

---

## Things only a running Blender could settle — what is left

The fourteen tests recorded above settled everything that stood on this list. Two things remain.

1. **Everything on Windows.** Neither machine used above speaks for it. Four entries are written out in full and ready to run in `Audits/Blender Research/Writing/Hand tests — Open Threads questions.md`, under *Windows — for the runner's separate batch*: the order Blender reports with no window, the disk watched with a write slow enough to catch in the act, placeholders, and whether the file is finished at the moment Blender says it is written. **A Windows result must name the edition it was taken on** — every Windows machine this project has read so far turned out to be a Server edition, and a finding that says only "Windows" is not yet an answer.
2. **Several machines writing into one shared folder**, which the manual's "primitive render farm" hint describes, and **an output folder on a network share**. Both need a second computer of the same kind and a file system shared between them, which this project's equipment does not provide. They are recorded as *cannot be run*, with that reason, in the list named above.

---

2026-08-29 — Claude Opus 5 (Claude Code) — Writing (Open Threads questions), Blender 5.1

# Writing — Open Threads questions, second pass (Blender 5.1)

**Sources consulted, and when.** Blender's manual for 5.1, Blender's Python API documentation for 5.1, the 5.1 release notes, and Cycles' own files for 5.1 — all as the plain-file copies held in `Blender Sources/`, taken on 2026-07-30. That snapshot date is my date read throughout. I did not use the web.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

Answering question 14 only, as instructed.

---

## 14. When a render works through a range of frames, how do the writes belonging to one frame stand in time against the writes belonging to the frame after it?

### Short answer

The sources never state the timing relationship. Nowhere in the 5.1 manual, the 5.1 Python API documentation, the 5.1 release notes or Cycles' own 5.1 files is there a sentence saying whether a frame's files are finished on disk before the next frame's writing starts, or whether the two can overlap.

What the sources do carry is a set of statements that a frame is rendered, then saved, and that the render then advances to the next frame. From those it follows that the frames are worked through one after another and that each frame's save belongs to that frame's turn. That is a step I took, not a sentence I read, so it is labelled inferred below and the step is written out. The stronger claim — that frame N's bytes have all reached the disk before any of frame N+1's writing begins — does not follow from anything I found, and I am not making it.

**LABEL: not in the sources** for the timing relationship as asked. Within the answer, the supporting pieces are labelled individually: **documented** for the quoted statements, **inferred** for the ordering that follows from them.

---

### (a) What the sources say outright — documented

**Frames are saved after being rendered.** The manual's page on rendering animations:

> "While rendering stills will allow you to view and save the image from the render buffer when it is complete, animations are a series of images, or frames, and are automatically saved directly out to a drive after being rendered."

Source: manual — https://docs.blender.org/manual/en/5.1/render/output/animation.html — read 2026-07-30.

**The render advances a frame at a time.** The Frame Range panel:

> "Step
>
> Controls the number of frames to advance for each frame rendered, a value above 1 skips frames."

Source: manual — https://docs.blender.org/manual/en/5.1/render/output/properties/frame_range.html — read 2026-07-30.

The command-line list says the same in its own words:

> "-a, --render-anim
>
> Render frames from start to end (inclusive)."

and

> "-j, --frame-jump <frames>
>
> Set number of frames to step forward after each rendered frame."

Source: manual — https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html — read 2026-07-30.

**Frames already done are on disk while the job is still running.** The same animations page, under Hints:

> "This allows you an easy recovery if there is a problem and you have to re-start the rendering, since the frames you have already rendered will still be in the output directory."

and

> "Just disable the Overwrite option to start rendering where you left off."

Source: manual — https://docs.blender.org/manual/en/5.1/render/output/animation.html — read 2026-07-30.

The output introduction puts it as a property of image-sequence output:

> "For example, when rendering animations to image file formats the render job can be canceled and resumed at the last rendered frame by changing the frame range."

Source: manual — https://docs.blender.org/manual/en/5.1/render/output/introduction.html — read 2026-07-30.

**There is a per-frame write event, and it fires directly after that frame is written.** The Python API:

> "bpy.app.handlers.render_write
>
> on writing a render frame (directly after the frame is written). Accepts one argument: the scene data-block being rendered"

Source: Python API — https://docs.blender.org/api/5.1/bpy.app.handlers.html — read 2026-07-30.

The neighbouring entries on the same page show the same page-level granularity for the render itself — `render_pre` "on render (before)", `render_post` "on render (after)" — and `render_stats` names saving as its own timed thing:

> "on printing render statistics. Accepts one argument: the render stats (render/saving time plus in background mode frame/used [peak] memory)."

Source: Python API — https://docs.blender.org/api/5.1/bpy.app.handlers.html — read 2026-07-30.

**A frame has a closing step of its own.** The render engine interface has a per-frame finish call:

> "render_frame_finish()
>
> Perform finishing operations after all view layers in a frame were rendered"

Source: Python API — https://docs.blender.org/api/5.1/bpy.types.RenderEngine.html — read 2026-07-30.

Cycles supplies that call. In `Blender Sources/cycles_5.1/__init__.py`, in the `CyclesRender` class:

> ```
>     def render_frame_finish(self):
>         engine.render_frame_finish(self)
> ```

and in `Blender Sources/cycles_5.1/engine.py`:

> ```
> def render_frame_finish(engine):
>     if not engine.session:
>         return
>
>     import _cycles
>     _cycles.render_frame_finish(engine.session)
> ```

Source: Cycles' own files — `cycles_5.1/__init__.py` (method of `CyclesRender`, immediately after `render`) and `cycles_5.1/engine.py` (function `render_frame_finish`, immediately after `render`) — read 2026-07-30. **LABEL for these two blocks: observed.** They show the call exists and is wired through to Cycles. They say nothing about file writing.

**Compositor output writes happen as part of a render, one image at a time.** The File Output node:

> "The File Output node writes one or more images to disk during rendering or compositing."

Source: manual — https://docs.blender.org/manual/en/5.1/compositing/types/output/file_output.html — read 2026-07-30.

And the compositing introduction:

> "The Compositor processes one image at a time, no matter what kind of input you provide."

Source: manual — https://docs.blender.org/manual/en/5.1/compositing/introduction.html — read 2026-07-30.

**There is an option that puts an empty file down while rendering.** The Python API:

> "use_placeholder
>
> Create empty placeholder files while rendering frames (similar to Unix 'touch') (default False)"

Source: Python API — https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html — read 2026-07-30.

The manual's wording of the same option, and the use it names:

> "Create empty placeholder frames while rendering."

and, in the hint beside it:

> "Primitive Render Farm
>
> An easy way to get multiple machines to share the rendering workload is to:
>
> Set up a shared directory over a network file system.
>
> Disable Overwrite, enable Placeholders in the Render Output panel.
>
> Start as many machines as you wish rendering to that directory."

Source: manual — https://docs.blender.org/manual/en/5.1/render/output/properties/output.html — read 2026-07-30.

**When the output is a video, the frames do not get separate files.** The animations page:

> "Blender creates one file that holds all the frames of your animation."

Source: manual — https://docs.blender.org/manual/en/5.1/render/output/animation.html — read 2026-07-30.

The Output panel, on Media Type:

> "Video:
>
> Encodes each frame into a video container."

and, on splitting:

> "If your video is huge and exceeds 2GiB, enable Autosplit Output. This will automatically split the output into multiple files after the first file is 2GiB in size."

and, in the tips at the foot of the same page:

> "If a problem occurs while rendering, the file might become unplayable and you will have to re-render all frames from the beginning."

Source: manual — https://docs.blender.org/manual/en/5.1/render/output/properties/output.html — read 2026-07-30.

---

### (b) What follows from those — inferred

**LABEL: inferred.**

Claim: a range render works through its frames one at a time in the order start to end, and the saving of a frame's image belongs to that frame's turn rather than being gathered up and done at the end.

The step, written out. The manual says animations are "automatically saved directly out to a drive after being rendered" — so saving follows rendering, not the whole job. It says Step controls "the number of frames to advance for each frame rendered" and the command-line jump argument steps forward "after each rendered frame" — so there is a per-frame advance, one frame after another. It says frames already rendered "will still be in the output directory" if the job is interrupted, and that a job can be "resumed at the last rendered frame" — so files exist on disk before the job as a whole ends. The API names an event that fires "directly after the frame is written", one per frame. Put together, that gives: frame N is rendered, frame N's file is written, the render advances, frame N+1 is rendered, frame N+1's file is written. A reader has to join those five statements to get there; no single one says it. Hence inferred, not documented.

Claim: for a video output, the frames' writes are not separate files at all but successive additions to one file that is only complete when the job ends.

The step. The manual says one file "holds all the frames", that Video "Encodes each frame into a video container", that Autosplit starts a second file only "after the first file is 2GiB in size", and that an interrupted video render "might become unplayable" and must be re-rendered "from the beginning". A file that grows past a size threshold during the job, and that is unusable if the job stops, is being added to as the job proceeds rather than written once per frame. That is my step, not the manual's sentence.

---

### (c) What the sources do not settle — not in the sources

None of the following is answered anywhere I could find:

- Whether frame N's file is fully written and closed before frame N+1's rendering starts, or whether the write of frame N can still be in progress while frame N+1 is being rendered. The manual's "after being rendered" fixes the order of render-then-save within a frame; it says nothing about the boundary between frames.
- Whether writing is done on the same thread of work as rendering, or handed to another. The one threading statement I found in the API documentation is about handlers and the viewport, not about output files: "While rendering the frame_change_pre and frame_change_post handlers are called from one thread and the viewport updates from a different thread" (Python API — https://docs.blender.org/api/5.1/bpy.app.handlers.html — read 2026-07-30). That concerns the viewport, and by its own terms only arises when there is a viewport.
- Where a File Output node's writes for frame N sit in time against the main output file's write for frame N, and whether either can straggle past the start of frame N+1. The File Output page says only "during rendering or compositing".
- When a placeholder file is put down relative to the frame it stands for — before that frame is rendered, or at some other moment. "While rendering frames" does not fix it, and I would be inventing if I said which.
- Whether the order is guaranteed at all, or is simply what happens. The documentation never phrases any of this as a promise.
- For video output, whether the bytes belonging to frame N reach the file before those belonging to frame N+1. The manual describes B-frames and Group of Pictures settings ("Max B-frames — Enables the use of B-frames"; "Keyframe Interval — The number of pictures per Group of Pictures. Set to 0 for 'intra_only', which disables inter-frame video", same Output page) but never says how frames are ordered in the written stream. I am not going to reason from what those settings are named to what the encoder writes when — that would be exactly the over-reading I was warned about.

---

### Not covered

This answer settles that the sources describe a per-frame render-then-save cycle that advances through the range, and that files for finished frames exist on disk while the job continues. It does not settle:

- any guarantee about the completion of one frame's write relative to the start of the next frame's work;
- the moment at which a file becomes complete or safe for another program to pick up;
- the relative order of the several writes a single frame can produce (main output, File Output nodes, render cache, placeholder);
- what happens under Step values above 1, where "the frame after it" means the next rendered frame rather than the next numbered one — I read the question as being about consecutive rendered frames;
- anything about a render spread across several machines, beyond the "primitive render farm" hint quoted above.

**Ambiguity flagged.** "The writes belonging to one frame" could mean the single output image for that frame, or every file that frame causes to be written. I answered the broader reading, and said above where the sources fall silent on each. The narrower reading — just the main output image — would be answered by exactly the same quotes, with the same silence at the frame boundary.

---

### Outside the interface

Does this hold when Blender is driven from outside its own window?

The sources give no separate account of write ordering for that case, so there is nothing to differ from. The command-line documentation describes the same animation render being started without a window — "-b, --background — Run in background (often used for UI-less rendering)" and "-a, --render-anim — Render frames from start to end (inclusive)" (manual — https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html — read 2026-07-30) — and the page on rendering from the command line says "-a — Render the whole animation using all the settings saved in the blend-file" (manual — https://docs.blender.org/manual/en/5.1/advanced/command_line/render.html — read 2026-07-30). The manual does not say the ordering differs.

Two differences are documented, and neither is about ordering. First, what gets reported differs: the render statistics handler is described as carrying "render/saving time plus in background mode frame/used [peak] memory" (Python API — https://docs.blender.org/api/5.1/bpy.app.handlers.html — read 2026-07-30), so background mode adds frame and memory figures. Second, the threading caution quoted above is about the viewport, which is absent when there is no window.

On the strict question — whether one frame's writes stand differently against the next frame's when Blender is driven from outside — **the sources do not say.**

**Brought up to date by the hand tests of 2026-08-30.** This is now settled by test rather than left to the sources. The same five-frame render was run in this version on two machines, once with no window at all and once with Blender's own window open, the render started the way the Render Animation menu item starts it — as a job the interface keeps running alongside. **The ordering was the same in all four runs**: for every frame, the write event for frame N came before the render event for frame N+1, and on disk no file was still growing when the next frame's file appeared. See *What the hand tests settled — question 14* at the foot of this document, tests 1 and 13.

### Operating system

Does this hold on every operating system Blender runs on?

**The sources do not say.** I found no statement anywhere that ties render output write ordering to an operating system. The only operating-system-specific remarks I found in the neighbourhood are unrelated to it: the temporary-directory arguments, which are given separately for MS-Windows and UNIX systems (manual — https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html), and, in the 5.1 release notes, "Support opening windows without decorations with the: `--no-window-frame` argument (Linux only)" (release notes — https://developer.blender.org/docs/release_notes/5.1/core/ — read 2026-07-30). The placeholder property's phrase "similar to Unix 'touch'" is an analogy explaining what an empty file is; it does not say the option is limited to Unix, and I am not reading it as one.

**Brought up to date by the hand tests of 2026-08-30.** Two operating systems are now settled by test: **macOS**, on the Creator's own Mac, and **Linux**, on a rented machine whose Blender matched the Mac's to the exact build hash. The two agreed on every test. **Windows is still not settled** — not by the sources and not by any test — and four Windows entries are written out ready to run in `Audits/Blender Research/Writing/Hand tests — Open Threads questions.md`. A Windows result must name the edition it was taken on. See *What the hand tests settled — question 14* at the foot of this document.

---

### Record of where I looked

All searching was over the plain-file copies in `Blender Sources/`, snapshot 2026-07-30: `manual_5.1`, `api_5.1`, `release_notes_5.1`, `cycles_5.1`. I searched the whole of each rather than opening likely files, then opened pages the searches pointed at.

**Whole-corpus searches, case-insensitive** (manual, API and release notes together unless noted). Terms, and what came back:

- Timing and order between frames: "one frame at a time", "frame by frame", "frame-by-frame", "each frame is", "after each frame", "as each frame", "every frame is", "next frame", "before the next frame", "previous frame is", "frame after", "once the frame", "in order", "in sequence", "sequentially", "ascending", "start to end", "one at a time", "moves on", "before starting". Nothing on the ordering of output writes. The hits were about Grease Pencil drawing, cloth simulation, motion tracking, video strips, NLA tracks, geometry node sorting and similar — all ruled out as unrelated to render output.
- Overlap and concurrency: "simultaneous", "concurrent", "in parallel", "asynchronous", "background thread", "worker thread", "deferred", "pipelin", "overlap", "queue". The only render-adjacent hits were about CPU cores and devices used during rendering (Cycles Performance page), GPU shader compilation in Preferences, and EEVEE "deferred rendering" as a shading term. None concerns writing files. Ruled out.
- Write completion and file state: "written to disk", "saved to disk", "still being written", "is closed", "flush", "buffered", "atomic", "partial file", "incomplete file", "temporary file", "appended", "as soon as", "finished rendering", "while rendering". "still being written", "partial file", "incomplete file", "worker thread", "atomic" and "background thread" returned nothing at all anywhere in the three sources. "buffered" and "flush" returned only audio, console-line and animation-editor hits, ruled out. "temporary file" returned the temp-directory arguments and autosave, ruled out.
- Jobs and reporting: "render job", "background job", "progress", "render stats", "saving time", "status bar". These led me to the four render handlers and `render_stats`, quoted above, and to `RenderEngine.update_progress` ("Update progress percentage of render"), which says nothing about writes.
- Named settings: "use_placeholder", "use_overwrite", "render cache", "use_render_cache", "Open Cached Render", "Render Animation", "frame_path", "Autosplit". These gave the placeholder, overwrite and video quotes above, and `RenderSettings.frame_path` ("Return the absolute path to the filename to be written for a given frame"), which is about the name, not the moment.

**Pages opened in full** (manual, 5.1): `render/output/animation`, `render/output/introduction`, `render/output/properties/output`, `render/output/properties/frame_range`, `render/output/properties/post_processing`, `compositing/introduction`, `compositing/types/output/file_output`, `advanced/command_line/arguments`, `advanced/command_line/render`, `interface/window_system/topbar` (Render menu section). Also listed the whole `render/` tree of the manual to be sure I was not missing an output page.

**Pages opened in full** (Python API, 5.1): `bpy.app.handlers` (introduction and all render entries), `bpy.types.RenderSettings` (output-related properties and `frame_path`), `bpy.types.RenderEngine` (render entry points), `bpy.ops.render` (the `render` and `opengl` operators). `bpy.ops.render.render` is documented as "Undocumented, consider contributing." — its `animation` parameter says only "Render files from the animation range of this scene", which does not touch timing. Ruled out.

**Release notes, 5.1**: read `rendering.md` and `core.md` in full; searched `pipeline_io.md`, `python_api.md`, `compositor.md`, `bugfixes.md`, `corrective_releases.md` and the rest for "write", "writing", "saved", "output", "frame", "placeholder", "overwrite", "sequence", "file path". Nothing about output write ordering or timing. The compositor notes mention File Output changes ("The **Image** and **File Output** nodes now read and write custom meta-data"; "The **File Output** node now allows specifying directories in item names") but say nothing about when writes happen. Ruled out.

**Cycles' own files, 5.1**: listed the folder; searched all Python files for "frame" and read the results. Found `render_frame_finish` in `__init__.py` and `engine.py`, quoted above, and `CYCLES_OT_denoise_animation` in `operators.py`, which loops over `range(scene.frame_start, scene.frame_end + 1)` and checks each frame's file exists, reporting "Frame '%s' not found, animation must be complete". I am not offering that as an answer: it is a denoising operator working on an already-finished render, not the writing done by a render. I checked `source/kernel` and `source/util` — these are the render kernel and its utility headers, with no file-output code — and ruled them out.

**Pictures.** The Output panel screenshot named on the Output properties page (`render_output_properties_output_panel.png`), and the Frame Range panel screenshot, are the two images sitting closest to this question. I did not treat either as evidence, because a screenshot of a panel can show which options exist but cannot show the order of events in time, which is what was asked.

**Conclusion of the search.** The absence is a real one. I searched for the plain words (write, save, after, next, order), the industry words (concurrent, parallel, asynchronous, buffered, flush, pipeline, atomic), and the words around them (job, progress, placeholder, resume, overwrite), across the entire manual, the entire API documentation and the entire release notes for 5.1, and there is no statement of how one frame's writes stand in time against the next frame's.

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

2026-08-30 — Claude Opus 5 (Claude Code) — Writing, Blender 5.1

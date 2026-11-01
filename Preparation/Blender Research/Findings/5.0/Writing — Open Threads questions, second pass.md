# Writing — Open Threads questions, second pass (Blender 5.0)

**Sources consulted, and when.** The Blender 5.0 manual, the Blender 5.0 Python API documentation, the Blender 5.0 release notes, and the Cycles 5.0 files, all as supplied in `Blender Sources/`. Snapshot date, and therefore my date read for everything below: **2026-07-30**.

**I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.**

Answering question 14 only.

---

## 14. When a render works through a range of frames, how do the writes belonging to one frame stand in time against the writes belonging to the frame after it?

### Short answer

The sources say that an animation render moves through its frames from the start frame to the end frame, that each frame's picture is saved to disk after that frame has been rendered, and that there is a definite moment at which "the frame is written" — a moment a script can be notified of. They also say the time spent saving is measured and reported as part of that frame's own timing.

What the sources **do not** say anywhere I could find is the thing the question is really pointing at: whether every write belonging to frame N is finished before any write belonging to frame N+1 begins, or whether saving frame N can still be going on while frame N+1 is being rendered. No page states an ordering guarantee, an overlap, or the absence of one. So the answer is in two parts, with two different labels.

### Part A — what the sources do say

**LABEL: documented.**

Frames are saved as the render goes, not collected up and written at the end:

> "While rendering stills will allow you to view and save the image from the render buffer when it is complete, animations are a series of images, or frames, and are automatically saved directly out to a drive after being rendered."

Source: manual, *Rendering Animations* — https://docs.blender.org/manual/en/5.0/render/output/animation.html — read 2026-07-30.

The frames are worked through from the first to the last:

> "-a, --render-anim"
> "Render frames from start to end (inclusive)."

Source: manual, *Command Line Arguments* — https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html — read 2026-07-30.

And the frame counter moves forward one step per rendered frame:

> "Controls the number of frames to advance for each frame rendered, a value above 1 skips frames."

Source: manual, *Frame Range* — https://docs.blender.org/manual/en/5.0/render/output/properties/frame_range.html — read 2026-07-30.

Earlier frames are already on disk if the job is stopped part way, and a stopped job can be picked up again at the frame it reached:

> "when rendering animations to image file formats the render job can be canceled and resumed at the last rendered frame by changing the frame range."

Source: manual, *Output Introduction* — https://docs.blender.org/manual/en/5.0/render/output/introduction.html — read 2026-07-30.

> "This allows you an easy recovery if there is a problem and you have to re-start the rendering, since the frames you have already rendered will still be in the output directory."

Source: manual, *Rendering Animations* — https://docs.blender.org/manual/en/5.0/render/output/animation.html — read 2026-07-30.

There is a distinct, notifiable moment at which one frame's write is complete:

> "bpy.app.handlers.render_write"
> "on writing a render frame (directly after the frame is written)"

Source: Python API, *Application Handlers* — https://docs.blender.org/api/5.0/bpy.app.handlers.html — read 2026-07-30.

Saving is a measured phase of a frame, reported together with that frame's render time:

> "bpy.app.handlers.render_stats"
> "on printing render statistics. Accepts one argument: the render stats (render/saving time plus in background mode frame/used [peak] memory)."

Source: Python API, *Application Handlers* — https://docs.blender.org/api/5.0/bpy.app.handlers.html — read 2026-07-30.

The 5.0 release notes show what that looks like in the log of a background render. This is a single-frame example, not an animation, but it is the only place in the sources where the saving of a frame is shown against a clock:

> ```
> 00:00.244  render           | Rendering single frame (frame 1)
> 00:00.245  render           | Start rendering: Scene, ViewLayer
> 00:00.245  render           | Engine: Cycles
> 00:26.799  render           | Mem: 2983M | Sample 1/16
> 00:26.799  render           | Mem: 2983M | Sample 16/16
> 00:28.631  render           | Saved: '/tmp/0001.png'
> 00:28.631  render           | Time: 00:28.38 (Saving: 00:00.15)
> ```

Source: release notes, *Blender 5.0: Core* (Logging section) — https://developer.blender.org/docs/release_notes/5.0/core/ — read 2026-07-30.

Two further documented points about what "the writes belonging to one frame" can consist of:

> "The File Output node writes one or more images to disk during rendering or compositing."

Source: manual, *File Output Node* — https://docs.blender.org/manual/en/5.0/compositing/types/output/file_output.html — read 2026-07-30.

> "Create empty placeholder files while rendering frames (similar to Unix 'touch')"

Source: Python API, `RenderSettings.use_placeholder` — https://docs.blender.org/api/5.0/bpy.types.RenderSettings.html — read 2026-07-30. The manual's wording for the same option is "Create empty placeholder frames while rendering." (https://docs.blender.org/manual/en/5.0/render/output/properties/output.html).

And where the output is a video rather than an image sequence, each frame's contribution goes into one growing file rather than a file of its own:

> "Encodes each frame into a video container."

Source: manual, *Output* panel — https://docs.blender.org/manual/en/5.0/render/output/properties/output.html — read 2026-07-30.

> "Blender creates one file that holds all the frames of your animation."

Source: manual, *Rendering Animations* (Direct Approach) — https://docs.blender.org/manual/en/5.0/render/output/animation.html — read 2026-07-30.

### Part B — the ordering the question asks about

**LABEL: inferred** for the weak form; **not in the sources** for the strong form.

*The weak form, inferred.* Frame N's picture is written before frame N+1's picture is written. The step from the quoted words to that claim: the manual says animation frames are saved out "after being rendered"; the command-line documentation says the frames are rendered "from start to end"; and the manual says that when a job is stopped, "the frames you have already rendered will still be in the output directory", so that it can be "resumed at the last rendered frame". Put together, the frames are rendered in increasing order and each is saved after its own render, so the file for frame N is written before the file for frame N+1. A reader has to join those three statements to get there, so this is inferred, not documented.

*The strong form, not in the sources.* Nothing in the manual, the API documentation, the release notes or the Cycles files states:

- whether frame N's write is fully finished — closed on disk, visible to another program watching the folder — before any work on frame N+1 starts;
- whether saving can run at the same time as the next frame's rendering, on another thread or otherwise;
- whether the file that appears for frame N is complete the moment it appears, or grows;
- how long the gap between one frame's write and the next is, or whether there is one.

The only near-miss is the log line "Time: 00:28.38 (Saving: 00:00.15)", which shows that a frame's saving time is counted inside that frame's reported total. That is suggestive of saving finishing within the frame's own step, but it is a report of measured time, not a statement about ordering or about threads, and I am not willing to read a guarantee out of it. I mention it only so the next reader knows it was considered and rejected as insufficient.

*One more inference about writes within a single frame, since the question speaks of "the writes belonging to one frame" in the plural.* Where a File Output node is used, its files are written "during rendering or compositing", and the compositing pass is what produces the picture that the main output then saves — the manual's Post Processing page describes Compositing as processing the render result, and the manual's Rendering Animations page says the frame is saved after being rendered. So for a given frame, the File Output node's files are written before that frame's main output file. **LABEL: inferred**; the reader has to combine the File Output page with the pipeline description to get there, and neither page states the order outright.

*(Narrowed by the citation check of 2026-08-30.)* The quoted words carry less than the paragraph above claims. The File Output page says only "during rendering **or** compositing", which does not fix the node's write to the compositing step; and the Post Processing page says compositing processes the image "after rendering", the same phrase the animations page uses of saving — so neither page orders compositing against the main save. The step therefore needs an assumption no quote supplies. **What the sources carry is only that both writes exist and both belong to that frame's turn; the order between them is not in the sources.** The order was afterwards settled by test rather than by the sources — test 5 below found the node's file written first and the main output second, in all four versions on both machines — and that test, not this inference, is what the ordering now rests on.

### Where I looked

Everything below was searched across the whole of `manual_5.0`, `api_5.0`, `release_notes_5.0` and `cycles_5.0` unless a narrower scope is stated.

Search terms tried, case-insensitive, over all four sources: `sequentially`, `one at a time`, `in parallel`, `simultaneously`, `concurrently`, `frame by frame`, `one after`, `overlap`, `interleav`, `pipelin`, `prefetch`, `non-blocking`, `asynchron`, `deferred writ`, `flush`, `buffered`, `background thread`, `worker thread`, `while the next`, `next frame`, `each frame is`, `after each frame`, `as they are rendered`, `written to disk`, `saved to disk`, `to a drive`, `when the frame is`, `current frame is written`, `atomic`, `temporary file`, `temp file`, `partially written`, `incomplete file`, `render cache`, `ascending`, `descending`, `reverse order`, `chronological`, `timestamp`, `modification time`, `mtime`, `file is closed`, `closed before`, `Saved:`, `Saving:`, `Time: 00`, `render job`, `render thread`, `Rendering single frame`, `Rendering frames`, `animation render`, `appended`, `already exist`, `skip`, `resume`, `placeholder`, `use_overwrite`, `render_write`, `render_post`, `render_pre`, `render_complete`, `render_init`, `render animation`.

Pages opened in full or in the relevant part, and what each settled:

- manual, *Rendering Animations* (https://docs.blender.org/manual/en/5.0/render/output/animation.html) — gives the "saved directly out to a drive after being rendered" statement and the recovery/resume hints. Says nothing about ordering between frames.
- manual, *Output Introduction* (https://docs.blender.org/manual/en/5.0/render/output/introduction.html) — cancel and resume at the last rendered frame. Nothing on ordering of writes.
- manual, *Output* properties panel (https://docs.blender.org/manual/en/5.0/render/output/properties/output.html) — file path, media type, Overwrite, Placeholders, the "Primitive Render Farm" hint. Nothing on timing.
- manual, *Frame Range* (https://docs.blender.org/manual/en/5.0/render/output/properties/frame_range.html) — Start, End, Step. "advance for each frame rendered". Nothing on writes.
- manual, *Format* properties panel (https://docs.blender.org/manual/en/5.0/render/output/properties/format.html) — resolution, aspect, frame rate. Ruled out, nothing about writing.
- manual, *Post Processing* (https://docs.blender.org/manual/en/5.0/render/output/properties/post_processing.html) — Compositing and Sequencer pipeline switches, Dither. No timing statements.
- manual, *Rendering From The Command Line* (https://docs.blender.org/manual/en/5.0/advanced/command_line/render.html) — worked examples only; no sample log, no ordering statement.
- manual, *Command Line Arguments* (https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html) — `-a`, `-f`, `-s`, `-e`, argument-order warnings. Gives "from start to end (inclusive)".
- manual, *File Output Node* (https://docs.blender.org/manual/en/5.0/compositing/types/output/file_output.html) — read in full; says writes happen "during rendering or compositing", nothing about when relative to other frames.
- manual, *Compositing Introduction* (https://docs.blender.org/manual/en/5.0/compositing/introduction.html) — "The Compositor processes one image at a time". About one image, not about the ordering of writes.
- manual, *Render Introduction* (https://docs.blender.org/manual/en/5.0/render/introduction.html) — engine overview only. Ruled out.
- manual, *Topbar* render menu (https://docs.blender.org/manual/en/5.0/interface/window_system/topbar.html) — "Render the animation of the active scene." Nothing more.
- manual, *Video Formats* (https://docs.blender.org/manual/en/5.0/files/media/video_formats.html) — searched for "while rendering", "during render", "written progressively", "interrupt", "resume": no matches. Ruled out.
- Python API, *Application Handlers* (https://docs.blender.org/api/5.0/bpy.app.handlers.html) — read the whole handler list. `render_write`, `render_stats`, `render_pre`, `render_post`, `render_init`, `render_complete`, `frame_change_pre`, `frame_change_post`. This is the closest the API comes to the question. Note that `render_init` and `render_complete` are described in terms of the "render job" while `render_pre` and `render_post` are described only as "on render (before)" / "on render (after)"; I read that difference as pre/post bracketing each individual render inside a job, but the documentation does not say so, so I am not resting anything on it.
- Python API, `bpy.types.RenderSettings` (https://docs.blender.org/api/5.0/bpy.types.RenderSettings.html) — read the block around `use_overwrite`, `use_placeholder`, `use_persistent_data`, `use_render_cache`, `use_sequencer`, `threads`. Descriptions only, no timing.
- Python API, `bpy.types.RenderEngine` (https://docs.blender.org/api/5.0/bpy.types.RenderEngine.html) — `render()`, `render_frame_finish()` ("Perform finishing operations after all view layers in a frame were rendered"). About view layers within a frame, not about writes or about the next frame.
- Python API, `bpy.ops.render` (https://docs.blender.org/api/5.0/bpy.ops.render.html) — `render(animation=...)` "Render files from the animation range of this scene"; `frame_start` / `frame_end` parameters. No ordering statement.
- Python API, threading and tips gotchas pages — searched for render-related text; only the warning that Python threads are unsupported. Ruled out as unrelated to how Blender itself writes.
- Release notes, *Core* (https://developer.blender.org/docs/release_notes/5.0/core/) — the new logging system and the sample log quoted above. This is the only timed evidence in the sources.
- Release notes, *Rendering* (https://developer.blender.org/docs/release_notes/5.0/rendering/) — read in full: shader nodes, baking, pass renaming. Nothing about output writing or frame ordering.
- Release notes, *Pipeline & I/O* (https://developer.blender.org/docs/release_notes/5.0/pipeline_io/) — read in full: importers, HDR/wide-gamut video and images, multi-part EXR. Nothing about write ordering.
- Release notes, *Compositor* (https://developer.blender.org/docs/release_notes/5.0/compositor/) — the File Output redesign, and the note that the node "now only appends the frame number to the image name when doing an animation render". Relevant to naming, not to ordering.
- Release notes, all other subject files were swept by the term searches above (`writ`, `flush`, `asynchron`, `overlap`, `interleav`); the only hits were unrelated (physics caches, asset catalogue writes, blend-file compression, a Cycles debug-log note).
- Cycles files — `engine.py`, `__init__.py`, `properties.py`, `ui.py`, `operators.py`, `version_update.py`, and the `source/` tree. Searched for `write` and `open(`. What is supplied here is the Python add-on plus the kernel and OSL sources; there is no session, tile or output-driver code in this copy, and nothing that writes output files. `engine.py` exposes `render()` and `render_frame_finish()` as thin calls into `_cycles`, with no visible file writing. I therefore ruled the Cycles files out as a source for this question, rather than reading anything into their silence.

I did not open any `_images` file for this question; the question is about timing, and no screenshot in the manual could settle it.

### Not covered

- Whether frame N's file is complete and closed at the moment it appears in the folder. That is a different question and I did not answer it here.
- Whether the operating system's own write caching delays when another machine or program sees frame N's file, relative to frame N+1's.
- What happens to the ordering when a frame fails, is skipped because Overwrite is off, or when Placeholders are on — specifically, at what point in a frame's step the placeholder is touched. The sources say only "while rendering frames". The manual's "Primitive Render Farm" hint only works if a frame's placeholder appears before that frame is rendered, but the manual never says so, and I have not treated it as a finding.
- The ordering of writes when several machines render into the same folder. Nothing here speaks to that.
- Anything about the ordering of audio output (Render Audio), which is a separate operation and was not examined.
- Anything about the Video Sequencer's own render, beyond the fact that it too uses the Animation button.

### Outside the interface

The sources do not say. Nothing in the manual, API documentation or release notes distinguishes the timing or ordering of a frame's writes between a render started by a person clicking and one started from the command line or from a script. The pieces of documented behaviour above are all stated without reference to how the render was started: the manual's animation page, the frame range page and the File Output page describe the feature, and the command-line page describes `-a` as rendering "frames from start to end (inclusive)" without saying anything about writes. The one place where background mode is singled out at all is the `render_stats` handler, which is documented to carry "render/saving time plus **in background mode** frame/used [peak] memory" — that is a difference in what is reported, not in when files are written. The sample log in the 5.0 release notes is a background render, so the "Saved:" and "Time: … (Saving: …)" lines are at least known to be produced outside the interface.

**Brought up to date by the hand tests of 2026-08-30.** This is now settled by test rather than left to the sources. The same five-frame render was run in this version on two machines, once with no window at all and once with Blender's own window open, the render started the way the Render Animation menu item starts it — as a job the interface keeps running alongside. **The ordering was the same in all four runs**: for every frame, the write event for frame N came before the render event for frame N+1, and on disk no file was still growing when the next frame's file appeared. See *What the hand tests settled — question 14* at the foot of this document, tests 1 and 13.

### Operating system

The sources do not say. I searched the render output pages and the file paths page for "Windows", "macOS" and "Linux" and found no mentions at all, and no page anywhere in the sources ties the writing of render output, or its ordering, to a particular operating system. The only operating-system-specific text near this subject is the command-line page's separate temporary-directory variables for Windows and for UNIX systems, which is about where temporary files go, not about when output files are written.

**Brought up to date by the hand tests of 2026-08-30.** Two operating systems are now settled by test: **macOS**, on the Creator's own Mac, and **Linux**, on a rented machine whose Blender matched the Mac's to the exact build hash. The two agreed on every test. **Windows is still not settled** — not by the sources and not by any test — and four Windows entries are written out ready to run in `Audits/Blender Research/Writing/Hand tests — Open Threads questions.md`. A Windows result must name the edition it was taken on. See *What the hand tests settled — question 14* at the foot of this document.

---

## What the hand tests settled — question 14

Fourteen tests were run for this question on **2026-08-30** by Claude Opus 5 in Claude Code, in all four researched versions of Blender, on two machines. Everything below is what **Blender 5.0.1** did. The full list, with every script written out in full and every entry's negative result stated, is `Audits/Blender Research/Writing/Hand tests — Open Threads questions.md`.

### The two machines

**The Creator's own Mac.** A MacBook Pro with an Apple M1 Pro chip, macOS **26.6.2** (build 25G83, kernel `Darwin 25.6.0`, arm64), Apple's own graphics and no second card. Blender **5.0.1**, hash `a3db93c5b259`, started from `/Applications/Blender 5.0.1.app/Contents/MacOS/Blender`. No system setting on that machine was changed.

**A rented Linux machine, in the United Kingdom.** Vast.ai instance `49255001`, machine `143739`, rented and destroyed on 2026-08-30. **A container, not a whole machine**, established by reading rather than assumed: `/.dockerenv` is present, `systemd-detect-virt` says `docker`, `/proc/1/cgroup` prints `0::/`, and process 1 is the provider's own launch script. Ubuntu **22.04.4 LTS**, kernel `7.0.0-28-generic`, glibc 2.35, x86-64. An **AMD Ryzen 9 3900X** with 24 visible threads against a quota of **11.52 cores**; **the container misreports its own size** — the ordinary memory reading says 50,423,779,328 bytes while its own ceiling is 34,227,617,792. A 40 GB overlay root. One **NVIDIA GeForce RTX 5060 with 8151 MiB on driver 595.71.05**. **No display server as rented**; `Xvfb` was added to it for the window test, where the graphics are software (`llvmpipe`). Blender **5.0.1**, hash `a3db93c5b259`, at `/opt/blender/5.0/blender`, taken from Blender's own download server as the Linux x64 build — **its hash matches the Mac's exactly**, so any difference between the two machines is a difference of operating system and not of build.

**Every render on both machines was a processor render.** Cycles was set to `CPU` throughout, except in the one test that used EEVEE.

**The two results are never merged.** The Mac settles macOS; the Linux machine settles Linux. **Windows is settled by neither.** Four Windows entries are written out ready to run in the list named above, for the runner's separate Windows batch, and no Windows answer is claimed here.

### How fine the instrument was

This question can be answered wrongly by a slow instrument: a watcher reading the folder once a second sees every frame already finished and calls that "one after the other" whatever Blender did. So the folder was read **every two milliseconds** — on the Mac a reading completed every 3.2 milliseconds on average — and the write was made slow on purpose, a 3000 by 3000 OpenEXR at 32 bits with no compression, 144 megabytes a frame. One frame's write took about 190 milliseconds on the Mac and about 60 on Linux, and the watcher caught **63 separate readings of one file growing** on the Mac and **31** on Linux. Two frames' writes running at the same time could not have been missed at that rate. Blender was also asked, not only watched: a script printed a stamped line at every render event. And on Linux **every file call Blender made was traced**, thread by thread, so one result rests on no polling at all.

Every run used `--factory-startup` and Blender's own default scene, so no add-on and no saved preference could reach any of it, and no `.blend` file was used.

### 1. The order Blender itself reports, driven from outside its own window

Five frames to PNG, 512 by 512, one sample. For every frame in the range, `render_write` for frame N was printed **before** `render_pre` for frame N+1.

- On the **Mac**, the four gaps were +0.000037, +0.000033, +0.000029, +0.000030 seconds. Frames out of order: **0**.
- On the **rented Linux machine**, +0.000029, +0.000026, +0.000025, +0.000027 seconds. Frames out of order: **0**.

The two machines agreed.

### 2. The disk itself, with a write slow enough to catch in the act

Five frames, 3000 by 3000 OpenEXR, uncompressed. Each file's first appearance, its last change of size, and its final size — on the **Mac**:

```
frame_0001.exr                     1788091563.251120   725261 1788091563.251120 1788091563.542674    144048845
frame_0002.exr                     1788091564.555199    37133 1788091564.555199 1788091564.912932    144048845
frame_0003.exr                     1788091566.072437   119053 1788091566.072437 1788091566.413501    144048845
frame_0004.exr                     1788091567.588916   901389 1788091567.588916 1788091567.923134    144048845
frame_0005.exr                     1788091568.687097   295181 1788091568.687097 1788091568.977054    144048845
```

On the **rented Linux machine**:

```
frame_0001.exr                     1788091451.528869  2342912 1788091451.528869 1788091451.585546    144048845
frame_0002.exr                     1788091452.568118    24581 1788091452.568118 1788091452.638303    144048845
frame_0003.exr                     1788091453.689932    24581 1788091453.689932 1788091453.755406    144048845
frame_0004.exr                     1788091454.847902  2908160 1788091454.847902 1788091454.909643    144048845
frame_0005.exr                     1788091456.007697  1179648 1788091456.007697 1788091456.067580    144048845
```

Whether any file's size changed **after** a later file first appeared: **none** on the Mac, **none** on Linux. No name ever appeared in the folder that was not one of the final frame names, so no file was written under a temporary name and moved into place. The two machines agreed.

### 3. Whether the file is finished at the moment Blender says it is written

The write handler asked Blender for the frame's path with `scene.render.frame_path(frame=scene.frame_current)`, then read the whole file and printed the byte count beside the size the file system reported. On the **Mac**:

```
EVT 1788091572.569185 readback frame=1 stat=144048845 read=144048845 path=…/frame_0001.exr
EVT 1788091574.043730 readback frame=2 stat=144048845 read=144048845 path=…/frame_0002.exr
EVT 1788091575.522654 readback frame=3 stat=144048845 read=144048845 path=…/frame_0003.exr
EVT 1788091576.829712 readback frame=4 stat=144048845 read=144048845 path=…/frame_0004.exr
EVT 1788091578.178235 readback frame=5 stat=144048845 read=144048845 path=…/frame_0005.exr
```

On the **rented Linux machine**:

```
EVT 1788091459.060616 readback frame=1 stat=144048845 read=144048845 path=…/frame_0001.exr
EVT 1788091460.292266 readback frame=2 stat=144048845 read=144048845 path=…/frame_0002.exr
EVT 1788091461.558161 readback frame=3 stat=144048845 read=144048845 path=…/frame_0003.exr
EVT 1788091462.819726 readback frame=4 stat=144048845 read=144048845 path=…/frame_0004.exr
EVT 1788091464.016804 readback frame=5 stat=144048845 read=144048845 path=…/frame_0005.exr
```

Those counts are the same numbers test 2 recorded as the finished size of a frame. The two machines agreed.

### 4. Placeholders on, overwrite off

Five frames, the same large EXR, with `use_placeholder` on and `use_overwrite` off. The empty files appear **one at a time, not all at the start**. On the **Mac**:

```
frame_0001.exr                     1788091580.555199        0 1788091581.644505 1788091581.844051    144048845
frame_0002.exr                     1788091581.844051        0 1788091582.875854 1788091583.073377    144048845
frame_0003.exr                     1788091583.073377        0 1788091584.037991 1788091584.234444    144048845
frame_0004.exr                     1788091584.234444        0 1788091585.241308 1788091585.445632    144048845
frame_0005.exr                     1788091585.445632        0 1788091586.464069 1788091586.657087    144048845
```

On the **rented Linux machine**:

```
frame_0001.exr                     1788091465.805445        0 1788091466.882108 1788091466.945068    144048845
frame_0002.exr                     1788091466.945068        0 1788091468.052760 1788091468.114067    144048845
frame_0003.exr                     1788091468.114067        0 1788091469.308762 1788091469.370299    144048845
frame_0004.exr                     1788091469.370299        0 1788091470.481073 1788091470.540528    144048845
frame_0005.exr                     1788091470.542642        0 1788091471.629593 1788091471.691383    144048845
```

The `at_size` column is zero for every file at first sight, and `first_nonzero` is when the picture began to be written into it. The empty file for a frame appears within a few milliseconds of that frame's render beginning, sits at zero for the whole of that frame's render, and then grows to full size; the next frame's empty file appears within one reading of that. Only one empty file is ever waiting at a time, and the write event fires once per frame, not twice. The two machines agreed.

### 5. A File Output node's file against the main output file

Three frames, the same large EXR, with a File Output node in the compositor writing into a second folder and compositing switched on. Both folders were watched together. On the **Mac**:

```
file_name0001.exr                  1788092116.119905   123060 1788092116.119905 1788092116.379318    144048756
frame_0001.exr                     1788092116.382728  1749261 1788092116.382728 1788092116.729169    144048845
file_name0002.exr                  1788092117.558868  1015988 1788092117.558868 1788092117.811462    144048756
frame_0002.exr                     1788092117.811462   454925 1788092117.811462 1788092118.062853    144048845
file_name0003.exr                  1788092118.907781  2179252 1788092118.907781 1788092119.164364    144048756
frame_0003.exr                     1788092119.167840  1941773 1788092119.167840 1788092119.472132    144048845
```

On the **rented Linux machine**:

```
file_name0001.exr                  1788092156.673009    16388 1788092156.673009 1788092156.734730    144048756
frame_0001.exr                     1788092156.734730    24581 1788092156.734730 1788092156.796428    144048845
file_name0002.exr                  1788092157.829034   196608 1788092157.829034 1788092157.890634    144048756
frame_0002.exr                     1788092157.892748  2564096 1788092157.892748 1788092157.954431    144048845
file_name0003.exr                  1788092158.957788    24580 1788092158.957788 1788092159.020102    144048756
frame_0003.exr                     1788092159.020102        0 1788092159.022265 1788092159.082691    144048845
```

The node's file for a frame is written first, the scene's own output file second, and both are finished before the next frame's node file appears. The two machines agreed.

### 6. The preview picture saved beside a frame

Three frames, the same large EXR, with `image_settings.use_preview` on. On the **Mac**:

```
frame_0001.exr                     1788091595.712052   164109 1788091595.712052 1788091595.940641    144048845
frame_0001.jpg                     1788091596.133076        0 1788091596.136363 1788091596.195738       236624
frame_0002.exr                     1788091597.046194   225549 1788091597.046194 1788091597.269297    144048845
frame_0002.jpg                     1788091597.342775        0 1788091597.350639 1788091597.375500       236624
frame_0003.exr                     1788091598.322707  1130765 1788091598.322707 1788091598.538741    144048845
frame_0003.jpg                     1788091598.620222     8192 1788091598.620222 1788091598.651855       236624
```

On the **rented Linux machine**:

```
frame_0001.exr                     1788091479.898696    24581 1788091479.898696 1788091479.957939    144048845
frame_0001.jpg                     1788091480.104024        0 1788091480.106164 1788091480.131460       236850
frame_0002.exr                     1788091481.269103        0 1788091481.271256 1788091481.330431    144048845
frame_0002.jpg                     1788091481.404995     4096 1788091481.404995 1788091481.432577       236850
frame_0003.exr                     1788091482.536798   936997 1788091482.536798 1788091482.598419    144048845
frame_0003.jpg                     1788091482.671930        0 1788091482.674106 1788091482.699637       236850
```

The frame's own file is written first and the preview second, and both are finished before the next frame's file appears. The two machines agreed.

### 7. Two views in one frame

Three frames with `use_multiview` on, `views_format` `STEREO_3D`, and the image settings' `views_format` `INDIVIDUAL`, so each view gets a file. On the **Mac**:

```
frame_0001_L.exr                   1788091602.309273   688397 1788091602.309273 1788091602.528978    144048845
frame_0001_R.exr                   1788091602.528978  1896717 1788091602.528978 1788091602.784340    144048845
frame_0002_L.exr                   1788091603.834646        0 1788091603.837801 1788091604.059252    144048845
frame_0002_R.exr                   1788091604.059252    21325 1788091604.059252 1788091604.286975    144048845
frame_0003_L.exr                   1788091605.523025    94477 1788091605.523025 1788091605.755139    144048845
frame_0003_R.exr                   1788091605.755139   815373 1788091605.755139 1788091605.971334    144048845
```

On the **rented Linux machine**:

```
frame_0001_L.exr                   1788091486.651033  2233213 1788091486.651033 1788091486.710017    144048845
frame_0001_R.exr                   1788091486.710017   504925 1788091486.710017 1788091486.771418    144048845
frame_0002_L.exr                   1788091489.011804  3526656 1788091489.011804 1788091489.071084    144048845
frame_0002_R.exr                   1788091489.071084    24581 1788091489.071084 1788091489.132663    144048845
frame_0003_L.exr                   1788091491.415693   741376 1788091491.415693 1788091491.475573    144048845
frame_0003_R.exr                   1788091491.475573   360901 1788091491.475573 1788091491.533388    144048845
```

The left view is written first and the right second, and both are finished before the next frame's left view appears. In several runs the left view's last change and the right view's first appearance fall in the **same** reading; at two milliseconds those two events cannot be told apart and nothing is claimed about them. What is claimed is only the gap of one to three seconds before the next frame's files.

### 8. A video container: does the one file grow as the run goes

Forty frames at 1920 by 1920 into a video container, written losslessly with the `HUFFYUV` codec so that the finished file is about 153 megabytes — far larger than any write buffer. At the default settings the same scene compresses down to about twenty kilobytes for a short run, which is small enough to sit inside a write buffer and settle nothing.

| | Mac | rented Linux machine |
|---|---|---|
| size changes seen during the run | 58 | 59 |
| bytes on disk at the half-way frame | 72,613,888 | 72,613,888 |
| bytes on disk when the last frame was written | 141,557,760 | 145,227,776 |
| final size | 153,142,267 | 153,143,547 |
| size changes after the last frame's write | 4 | 2 |

The file grows steadily through the run: about half of it is on disk by the half-way frame and about nine tenths by the time the last frame is written, with the rest added afterwards. The two machines agreed.

### 9. The same video render stopped part way

Run only on the **rented Linux machine**, because a media reader was needed and `ffprobe` was installed there; nothing was installed on the Creator's Mac. The same forty-frame lossless video render was interrupted after six seconds and the part-finished file handed to `ffprobe`.

A file was left behind, **88,057,840 bytes**. `ffprobe` read it as `matroska,webm` with codec `huffyuv` and a duration of 0.959000 seconds, and a decoder walking it with `-count_frames` read **23 frames**. Blender had announced **23** frames written before the interrupt. *(The frame counts differ between versions because the interrupt landed at a fixed six seconds into renders of different speeds, not because Blender differs.)*

**This settles macOS not at all.** It is a Linux result only.

### 10. A list of frames given out of order instead of a range

Started with Blender's own frame argument rather than from a script — `-o <folder>/f_#### -F OPEN_EXR -x 1 -f 5,1,3` — with a script that only set the engine, samples and resolution and rendered nothing. Blender's own lines on the **Mac**:

```
00:00.303  render           | Saved: '…/f_0005.exr'
00:00.337  render           | Saved: '…/f_0001.exr'
00:00.369  render           | Saved: '…/f_0003.exr'
```

and the watcher's order of first appearance:

```
1788091793.124075  f_0005.exr
1788091793.156685  f_0001.exr
1788091793.189253  f_0003.exr
```

On the **rented Linux machine**:

```
1788091522.640343  f_0005.exr
1788091522.707299  f_0001.exr
1788091522.756141  f_0003.exr
```

The frames are written **in the order the list gives them**, not in numeric order. The two machines agreed.

### 11. The other thing a frame writes: Cycles' tile cache

Three frames at 4000 by 4000 with `use_auto_tile` on and `tile_size` 256, with Blender's temporary folder pointed at a folder of the test's own and watched alongside the output folder. On the **Mac**:

```
cycles-tile-buffer-71682-37246166752-0-0.exr 1788091795.175818        0 1788091795.179493 1788091796.416397      9564035
frame_0001.exr                     1788091796.419098  1699917 1788091796.419098 1788091796.834487    256064845
cycles-tile-buffer-71682-37246166752-1-0.exr 1788091796.837686     8192 1788091796.837686 1788091797.964547      9564035
frame_0002.exr                     1788091797.964547   143437 1788091797.964547 1788091798.381512    256064845
cycles-tile-buffer-71682-37246166752-2-0.exr 1788091798.385768        0 1788091798.389401 1788091799.536066      9564035
frame_0003.exr                     1788091799.536066  1912909 1788091799.536066 1788091799.944495    256064845
```

and the watcher's record of those files going away:

```
1788091796.416397 GONE  …/cycles-tile-buffer-71682-37246166752-0-0.exr
1788091797.964547 GONE  …/cycles-tile-buffer-71682-37246166752-1-0.exr
1788091799.536066 GONE  …/cycles-tile-buffer-71682-37246166752-2-0.exr
```

On the **rented Linux machine**:

```
cycles-tile-buffer-4716-129286877374144-0-0.exr 1788091524.561627     8192 1788091524.561627 1788091527.652313      9563900
frame_0001.exr                     1788091527.655902   672925 1788091527.655902 1788091527.754300    256064845
cycles-tile-buffer-4716-129285516954304-1-0.exr 1788091527.773407     8192 1788091527.773407 1788091530.546050      9563900
frame_0002.exr                     1788091530.548394    40960 1788091530.548394 1788091530.647419    256064845
cycles-tile-buffer-4716-129286384192192-2-0.exr 1788091530.664438        0 1788091530.666610 1788091533.277997      9563900
frame_0003.exr                     1788091533.280188   679936 1788091533.280188 1788091533.375002    256064845
```

One tile file per frame, written in the temporary folder during that frame's render, finished and **removed** no later than the reading in which that frame's own output file first appears. The two machines agreed. *(Narrowed by the citation check of 2026-08-30: the watcher stamps every event in one sweep with a single time, so where a removal and the next file's first appearance carry the same stamp — two of the three frames in the Mac run above — the log cannot show which came first. Test 14's system-call trace does show it, and shows the removal about two milliseconds before the frame's own file is opened, but that trace was taken on the Linux machine only and does not speak for the Mac.)*

### 12. The same run with EEVEE instead of Cycles

Five frames, the same large EXR, with the engine set to EEVEE. This version offers the engine under the name **`BLENDER_EEVEE`**, which the script printed. On the **Mac**:

```
frame_0001.exr                     1788091612.223131   790608 1788091612.223131 1788091612.555802    144048656
frame_0002.exr                     1788091614.853098    21136 1788091614.853098 1788091615.081851    144048656
frame_0003.exr                     1788091617.242120   389200 1788091617.242120 1788091617.528619    144048656
frame_0004.exr                     1788091619.679131  1560656 1788091619.679131 1788091619.960126    144048656
frame_0005.exr                     1788091622.245981   651344 1788091622.245981 1788091622.551019    144048656
```

On the **rented Linux machine**:

```
frame_0001.exr                     1788091511.414315    65536 1788091511.414315 1788091511.481315    144048656
frame_0002.exr                     1788091511.854213  1982464 1788091511.854213 1788091511.917328    144048656
frame_0003.exr                     1788091512.312080   888800 1788091512.312080 1788091512.375287    144048656
frame_0004.exr                     1788091512.804596  3575808 1788091512.804596 1788091512.865890    144048656
frame_0005.exr                     1788091513.346973        0 1788091513.349143 1788091513.412908    144048656
```

Whether any file's size changed after a later file appeared: **none** on the Mac, **none** on Linux. The answer is the same as Cycles', so the ordering is not a property of the render engine. The two machines agreed.

### 13. The same render with Blender's own window open

Five frames, the same large EXR, with Blender started **without** `-b`, so it opened its own window, and the render started with `bpy.ops.render.render('INVOKE_DEFAULT', animation=True)` — which is how the Render Animation menu item starts it, as a job the interface keeps running alongside. A script quit Blender once the job reported itself finished, so no hand touched a mouse. On the Mac this was Blender's own window on the Creator's screen; on Linux it was a window on an `Xvfb` display server added to the machine for this test, where the graphics are software.

The script printed `background False` and `WINDOWS 1`, and `INVOKE-RETURNED` printed about twenty milliseconds after the render was started, so the render really was a job running alongside rather than a call that blocks.

For every frame in the range, `render_write` for frame N was printed **before** `render_pre` for frame N+1.

- On the **Mac**, the four gaps were +0.000030, +0.000031, +0.000031, +0.000033 seconds. Frames out of order: **0**.
- On the **rented Linux machine**, +0.000054, +0.000044, +0.000074, +0.000057 seconds. Frames out of order: **0**.

And the disk agreed. On the **Mac**:

```
frame_0001.exr                     1788092864.551624  1052941 1788092864.551624 1788092864.812962    144048845
frame_0002.exr                     1788092865.953791   459021 1788092865.953791 1788092866.213354    144048845
frame_0003.exr                     1788092867.380481   110861 1788092867.380481 1788092867.595520    144048845
frame_0004.exr                     1788092868.724118   692493 1788092868.724118 1788092868.949822    144048845
frame_0005.exr                     1788092870.042461    21325 1788092870.042461 1788092870.266880    144048845
```

On the **rented Linux machine**:

```
frame_0001.exr                     1788092590.562256   120861 1788092590.562256 1788092590.636526    144048845
frame_0002.exr                     1788092594.907864    24581 1788092594.907864 1788092594.975287    144048845
frame_0003.exr                     1788092596.762069  1273053 1788092596.762069 1788092596.844325    144048845
frame_0004.exr                     1788092598.336397  2621440 1788092598.336397 1788092598.402866    144048845
frame_0005.exr                     1788092599.666807  1417077 1788092599.666807 1788092599.724482    144048845
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
cycles-tile-buffer-6166-137063666443968-0-0.exr 43925.378254   6219 43925.380144 43925.872207 43926.201542      462  6219:462
frame_0001.exr             43926.285692   6166 43926.285957 43926.523162 43926.523268     3006  6166:3006
cycles-tile-buffer-6166-137063666436800-1-0.exr 43926.763702   6220 43926.764914 43927.214609 43927.566443      462  6220:462
frame_0002.exr             43927.651219   6166 43927.651486 43927.907452 43927.907556     3006  6166:3006
cycles-tile-buffer-6166-137063666443968-2-0.exr 43928.157444   6221 43928.158783 43928.632954 43928.950140      462  6221:462
frame_0003.exr             43929.034486   6166 43929.034720 43929.277361 43929.277470     3006  6166:3006

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

2026-08-30 — Claude Opus 5 (Claude Code) — Writing, Blender 5.0

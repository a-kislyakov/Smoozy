# Writing — Open Threads questions, second pass (Blender 4.5)

**Sources consulted, and when.** Blender 4.5 manual (plain-text copy plus the `_images` folder), Blender 4.5 Python API documentation, Blender 4.5 release notes, and the Cycles 4.5 add-on files — all as supplied inside `Blender Sources/`, snapshot taken 2026-07-30. That snapshot date is the date read for everything below. I did not use the web.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

Scope of this pass: question 14 only.

---

## 14. When a render works through a range of frames, how do the writes belonging to one frame stand in time against the writes belonging to the frame after it?

**LABEL: not in the sources** — for the question as asked.

**Short answer.** Nothing in the manual, the API documentation, the release notes or the Cycles files states how one frame's writing stands in time against the next frame's writing. There is no sentence anywhere in the supplied sources that says frame N's file is finished before frame N+1 starts, and none that says the two may overlap. The sources describe *that* each frame is saved, and *what* is saved, but they never describe the timing of one frame's write relative to the next frame's.

What the sources do say, and what can and cannot be carried from it, is set out below. I have labelled each piece separately so the documented parts are not confused with the step I take beyond them.

### What is documented

**(a) An animation's frames are saved to disk as they are rendered, without waiting for the run to end.**

> "While rendering stills will allow you to view and save the image from the render buffer when it is complete, animations are a series of images, or frames, and are automatically saved directly out to a drive after being rendered."

Source: manual, https://docs.blender.org/manual/en/4.5/render/output/animation.html (line 5 of the page text). Date read: 2026-07-30.

**(b) There is a per-frame moment "directly after the frame is written", exposed as a handler.**

> "bpy.app.handlers.render_write"
> "on writing a render frame (directly after the frame is written)"

Source: Python API, https://docs.blender.org/api/4.5/bpy.app.handlers.html (lines 170–172). Date read: 2026-07-30.

The neighbouring handlers on the same page are per-job and per-render, not per-frame-pair:

> "bpy.app.handlers.render_init" — "on initialization of a render job. Accepts one argument: the scene datablock being rendered"
> "bpy.app.handlers.render_pre" — "on render (before)"
> "bpy.app.handlers.render_post" — "on render (after)"
> "bpy.app.handlers.render_complete" — "on completion of render job. Accepts one argument: the scene datablock being rendered"

Same page and date. Note that the documentation does not say in which order these fire relative to each other across a frame boundary — for example, it does not say that `render_write` for one frame fires before `render_pre` for the next.

**(c) Rendering time and saving time are counted as two separate things.**

> "bpy.app.handlers.render_stats" — "on printing render statistics. Accepts one argument: the render stats (render/saving time plus in background mode frame/used [peak] memory)."

Source: Python API, https://docs.blender.org/api/4.5/bpy.app.handlers.html (lines 166–168). Date read: 2026-07-30.

**(d) Frame stepping is forward, and it happens after each rendered frame.**

> "-a, --render-anim"
> "Render frames from start to end (inclusive)."

> "-j, --frame-jump <frames>"
> "Set number of frames to step forward after each rendered frame."

Source: manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html (lines 16–18 and 43–44). Date read: 2026-07-30.

**(e) One documented case spells out the output order of a frame range explicitly — but for Viewport Render Keyframes, not for a normal render.**

> "This only renders those frames for which the selected objects have an animation key. The other frames are still written to the output, but will simply repeat the last-rendered frame."
> "For example, when a six-frame animation is rendered, and the selected objects have a key on frames 3 and 5, the following frames will be output:"
> "The 1st frame is always rendered."
> "The 1st frame is repeated because there is no key on this frame."
> "The 3rd frame is rendered."
> "The 3rd frame is repeated because there is no key on this frame."
> "The 5th frame is rendered."
> "The 5th frame is repeated because there is no key on this frame."

Source: manual, https://docs.blender.org/manual/en/4.5/editors/3dview/viewport_render.html (lines 74–91). Date read: 2026-07-30.

This is the closest the supplied sources come to stating an order of writes over a frame range. It lists the outputs in ascending frame order. It still says nothing about whether one output is complete before the next begins.

**(f) Where the output is a movie, all frames go into one file, which is not usable until the run finishes.**

> "Blender creates one file that holds all the frames of your animation."

Source: manual, https://docs.blender.org/manual/en/4.5/render/output/animation.html (line 41). Date read: 2026-07-30.

> "To save a composition as a movie clip (all frames in a single file), use an AVI or QuickTime format, and use the Animation button and its settings."

Source: manual, https://docs.blender.org/manual/en/4.5/compositing/introduction.html (line 82). Date read: 2026-07-30.

> "It's not recommended to render directly to a video format in the first instance. If a problem occurs while rendering, the file might become unplayable and you will have to re-render all frames from the beginning."

Source: manual, https://docs.blender.org/manual/en/4.5/render/output/properties/output.html (lines 332–335). Date read: 2026-07-30.

> "If your video is huge and exceeds 2GiB, enable Autosplit Output. This will automatically split the output into multiple files after the first file is 2GiB in size."

Source: manual, https://docs.blender.org/manual/en/4.5/render/output/properties/output.html (lines 145–148). Date read: 2026-07-30.

**(g) An interrupted image-sequence render leaves the already-rendered frames on disk, and the run can be picked up from there.**

> "For example, when rendering animations to image file formats the render job can be canceled and resumed at the last rendered frame by changing the frame range."

Source: manual, https://docs.blender.org/manual/en/4.5/render/output/introduction.html (lines 10–11). Date read: 2026-07-30.

> "This allows you an easy recovery if there is a problem and you have to re-start the rendering, since the frames you have already rendered will still be in the output directory."
> "Just disable the Overwrite option to start rendering where you left off."

Source: manual, https://docs.blender.org/manual/en/4.5/render/output/animation.html (lines 139–142). Date read: 2026-07-30.

**(h) There are two settings whose whole purpose is to mark frames on disk while a run is in progress.**

> "Overwrite" — "Overwrite existing files when rendering."
> "Placeholders" — "Create empty placeholder frames while rendering."

Source: manual, https://docs.blender.org/manual/en/4.5/render/output/properties/output.html (lines 66–72). Date read: 2026-07-30.

The same panel's picture, `_images/render_output_properties_output_panel.png` (shown on that page), visibly carries an "Overwrite" checkbox, ticked, and a "Placeholders" checkbox, unticked. Documented, from the image.

The API wording for the second is more specific about when:

> "use_placeholder" — "Create empty placeholder files while rendering frames (similar to Unix 'touch')"

Source: Python API, https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html (lines 746–748). Date read: 2026-07-30.

And the manual gives the reason these two exist together:

> "Primitive Render Farm"
> "An easy way to get multiple machines to share the rendering workload is to:"
> "Set up a shared directory over a network file system."
> "Disable Overwrite, enable Placeholders in the Render Output panel."
> "Start as many machines as you wish rendering to that directory."

Source: manual, https://docs.blender.org/manual/en/4.5/render/output/properties/output.html (lines 74–86). Date read: 2026-07-30.

**(i) The compositor's File Output node writes per frame too, and across an animation it writes a whole sequence.**

> "The image(s) will be saved on rendering, writing to the current frame. An entire sequence of images will be saved, when an animation is rendered."

Source: manual, https://docs.blender.org/manual/en/4.5/compositing/types/output/file_output.html (lines 16–17). Date read: 2026-07-30.

The sources give no statement about how this node's write for a frame stands in time against the main output's write for that same frame, nor against either write for the next frame.

**(j) Where the manual does describe write-and-composite ordering, it does so within a frame, between views — not between frames.**

> "The compositing of a view is completed before the remaining views start to be composited."

Source: manual, https://docs.blender.org/manual/en/4.5/render/output/properties/stereoscopy/usage.html (line 318). Date read: 2026-07-30.

I include this only to show that a sentence of exactly the shape the question asks for does exist in this manual — for views inside one frame. No equivalent sentence exists for one frame against the next. That absence is the finding.

**(k) From the Cycles files: there is a per-frame finishing point in the render engine's own interface.**

> `    def render_frame_finish(self):`
> `        engine.render_frame_finish(self)`

File: `cycles_4.5/__init__.py`, lines 75–76 (inside the `CyclesRender` render-engine class, in the block commented `# final render`). Label: **observed**.

> `def render_frame_finish(engine):`
> `    if not engine.session:`
> `        return`
> `
> `    import _cycles`
> `    _cycles.render_frame_finish(engine.session)`

File: `cycles_4.5/engine.py`, lines 95–100. Label: **observed**.

The API documentation describes what this method is for:

> "render_frame_finish()"
> "Perform finishing operations after all view layers in a frame were rendered"

Source: Python API, https://docs.blender.org/api/4.5/bpy.types.RenderEngine.html (lines 382–384). Date read: 2026-07-30. Label: **documented**.

This establishes that a frame has a defined finishing point in the engine's interface. It does not say whether writing has happened by then, nor whether the next frame may begin before it.

**(l) The Cycles files supplied contain no code that writes render output and no frame loop for rendering.** The `cycles_4.5/source` folder contains only two subfolders, `kernel` and `util` — there is no Blender-integration source. The only frame loop anywhere in the supplied Cycles Python is in `cycles_4.5/operators.py`, lines 84–97, and it belongs to the Denoise Animation operator, which reads already-finished frames off disk rather than rendering them:

> `            for frame in range(scene.frame_start, scene.frame_end + 1):`

and, a few lines on:

> `                    err_msg = tip_("Frame '%s' not found, animation must be complete") % filepath`

File: `cycles_4.5/operators.py`, lines 84 and 89. Label: **observed**. I mention it only to record that I checked it and that it is not the render loop; it settles nothing about question 14.

### What I take from that, and where I stop

**Inferred** — that within one run, frames are worked through one at a time in a forward direction, and each frame's file is written during the run rather than at the end. The step from the quoted words: (a) says frames are "automatically saved directly out to a drive after being rendered"; (d) says the frame jump steps "forward after each rendered frame"; (g) says a cancelled run leaves "the frames you have already rendered" in the output directory and can be "resumed at the last rendered frame". Those three together only make sense if frames are taken in ascending order and each is written before the run ends. A reader still has to supply that joining-up, so this is inferred, not documented.

**Not in the sources — and this is the heart of the question.** Whether frame N's write is *finished* before any of frame N+1's work or writes *begin*. The sources never say it. In particular:

- No source says the write is done on the same thread as the render, or on another one.
- No source says whether the render of frame N+1 may start while frame N's file is still being written or closed.
- No source says whether a frame's file is complete at the instant `render_write` fires, or only that the handler runs "directly after the frame is written".
- Where several files belong to one frame — the main output, one or more File Output nodes, several views in a multi-view render — no source says in what order those are written, nor whether all of them are done before the next frame's begin.
- For movie output, no source says at what moment within the run each frame's data reaches the container file. (f) establishes only that all frames land in one file and that the file is at risk if the run is interrupted.

I looked hard for a statement either way and did not find one. Given the condition on this kind of answer, the record of looking is below.

### Where I looked

Everything below was run over the whole of `manual_4.5`, `api_4.5`, `release_notes_4.5` and `cycles_4.5` at once, unless a narrower scope is named.

Whole-tree text searches, case-insensitive, on these terms:
`Placeholder`, `Overwrite`, `render_write`, `render_post`, `render_pre`, `render_complete`, `render_init`, `render_stats`, `render_cancel`, `render_frame_finish`, `sequentially`, `sequential`, `one frame at a time`, `frame by frame`, `frame-by-frame`, `in parallel`, `simultaneously`, `concurrently`, `next frame`, `previous frame`, `each frame is`, `before rendering each frame`, `after each frame`, `in order`, `chronological`, `ascending`, `descending`, `after the frame is written`, `is written`, `are written`, `writing the file`, `saving time`, `before the next`, `until the next`, `starts rendering`, `begins rendering`, `asynchronous`, `async`, `background job`, `frame server`, `batch`, `queue`, `interleav`, `overlap`, `at the same time`, `while the next`, `blocking`, `non-blocking`, `one after`, `serial`, `pipeline`, `look ahead`, `lookahead`, `prefetch`, `deferred`, `delayed`, `lag`, `buffered`, `flushed`, `closed`, `finalize`, `finalized`, `completes`, `completed before`, `finishes`, `still be writing`, `in flight`, `render job`, `render farm`, `network`, `save_buffers`, `use_persistent_data`, `Persistent Data`, `resume`, `frame_step`, `frame_path`, `use_placeholder`, `use_overwrite`, `use_file_extension`, `Render Animation`, `Saving`.

I also ran a combined search for every line anywhere in the sources containing both the word "frame" and either "writ…" or "sav…", and read every hit that was not about keyframes, saving preferences, or file import/export.

Pages opened and read in full, and what each ruled out:

- https://docs.blender.org/manual/en/4.5/render/output/animation.html — the main animation-rendering page. Gave (a), (f), (g). Says nothing about the timing of one frame's write against the next.
- https://docs.blender.org/manual/en/4.5/render/output/introduction.html — gave (g). Nothing on timing.
- https://docs.blender.org/manual/en/4.5/render/output/properties/output.html — the Output panel, in full including the Encoding section. Gave (f), (h). Nothing on timing.
- https://docs.blender.org/manual/en/4.5/render/output/properties/frame_range.html — Start, End, Step, Time Stretching. Nothing on write timing.
- https://docs.blender.org/manual/en/4.5/render/output/properties/post_processing.html — Compositing and Sequencer toggles. Its "Pipeline" heading is an unwritten section: the page reads "Pipeline / Todo / Add this information." So the one place that might have described the render pipeline's ordering is empty in 4.5.
- https://docs.blender.org/manual/en/4.5/render/introduction.html — overview only.
- https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html — the full Render Options block. Gave (d). Nothing on timing.
- https://docs.blender.org/manual/en/4.5/advanced/command_line/render.html — rendering from the command line, read in full. Covers argument order on the command line, not write order on disk.
- https://docs.blender.org/manual/en/4.5/editors/3dview/viewport_render.html — gave (e).
- https://docs.blender.org/manual/en/4.5/interface/window_system/topbar.html — the Render menu entries. Nothing beyond "Render the animation of the active scene."
- https://docs.blender.org/manual/en/4.5/compositing/introduction.html — gave (f).
- https://docs.blender.org/manual/en/4.5/compositing/types/output/file_output.html — gave (i).
- https://docs.blender.org/manual/en/4.5/render/output/properties/stereoscopy/usage.html — gave (j).
- https://docs.blender.org/manual/en/4.5/render/cycles/render_settings/performance.html — Persistent Data. About memory reuse between frames, not about writing.
- https://docs.blender.org/manual/en/4.5/files/media/video_formats.html — containers and codecs, read in full. Describes formats, not when frames reach them.
- https://docs.blender.org/manual/en/4.5/render/eevee/limitations/limitations.html — checked because it was the only hit for "before rendering each frame"; that sentence is about CPU work preparing geometry, not writing.
- https://docs.blender.org/api/4.5/bpy.app.handlers.html — the full handler list. Gave (b), (c).
- https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html — gave (h), plus `frame_path`, `use_overwrite`, `use_file_extension`, `use_persistent_data`.
- https://docs.blender.org/api/4.5/bpy.types.RenderEngine.html — gave (k). Its worked example covers a single render, not a frame range.
- https://docs.blender.org/api/4.5/bpy.ops.render.html — `bpy.ops.render.render(animation=…)`. The parameter is described as "Animation, Render files from the animation range of this scene" and nothing more.
- https://docs.blender.org/api/4.5/info_gotchas_threading.html — read in full, because a threading note is where an overlap between rendering and writing would most likely be mentioned. It is entirely about Python threads crashing Blender and says nothing about Blender's own render or write threads.
- Release notes for 4.5: I searched the whole set and read the Rendering, Pipeline & I/O, Core, Cycles and Python API pages for any mention of frames or output. The only relevant entry anywhere is under https://developer.blender.org/docs/release_notes/4.5/python_api/ — "`scene.render.frame_path()` used to be infallible but can now fail if there are errors processing Path Templates in the render output path." That is about path building, not write timing. No release note in 4.5 mentions changes to the order or timing of per-frame writes.
- Cycles files: gave (k) and (l).

Image checked: `_images/render_output_properties_output_panel.png`, as shown on the Output panel page. It shows the Overwrite and Placeholders checkboxes and their states. No image in the manual depicts render progress or a write sequence, so no image can settle this question; I searched the `_images` listing for names containing `render_output`, `status` and `progress` and found nothing of that kind.

**Ambiguity in the question.** "The writes belonging to one frame" can be read two ways: as the single output file for that frame, or as every file written on that frame's account (main output, each File Output node, each view of a multi-view render). I have answered both, since the sources are silent on both. The reading I treated as primary is the second, wider one, because the question says "writes" in the plural.

**Not covered.** This answer does not settle: whether writing and rendering ever overlap in time; whether a frame's file is fully flushed and closed before the next frame's render begins; on which thread the write happens; where within a frame's work the File Output node writes fall; at what moment a frame's data reaches a movie container; nor whether the answer differs between Cycles, EEVEE and Workbench, which the sources never distinguish on this point.

**1. Outside the interface.** The sources give no reason to think the timing differs when Blender is driven from outside its own window, and no statement that it holds either. The command-line page documents the same forward frame stepping for a background run — "Render frames from start to end (inclusive)" and "Set number of frames to step forward after each rendered frame" (https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html, lines 18 and 44, read 2026-07-30) — and the render-statistics handler is documented as reporting extra figures specifically "in background mode" (https://docs.blender.org/api/4.5/bpy.app.handlers.html, line 168), which shows the same handler runs in background mode. Beyond that, the sources do not say. **Not in the sources.**

**Brought up to date by the hand tests of 2026-08-30.** This is now settled by test rather than left to the sources. The same five-frame render was run in this version on two machines, once with no window at all and once with Blender's own window open, the render started the way the Render Animation menu item starts it — as a job the interface keeps running alongside. **The ordering was the same in all four runs**: for every frame, the write event for frame N came before the render event for frame N+1, and on disk no file was still growing when the next frame's file appeared. See *What the hand tests settled — question 14* at the foot of this document, tests 1 and 13.

**2. Operating system.** The sources say nothing about the timing of per-frame writes differing between operating systems. The only operating-system-specific statement I found anywhere near rendering an animation is about whether a render can run at all, not about write order: "Headless rendering is not supported on headless Windows systems" (manual, https://docs.blender.org/manual/en/4.5/render/eevee/limitations/limitations.html, line 150, read 2026-07-30), which sits under EEVEE's limitations. The placeholder setting's description mentions Unix as a comparison only — "similar to Unix 'touch'" (https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html, line 748) — and does not limit the feature to Unix. **Not in the sources.**

**Brought up to date by the hand tests of 2026-08-30.** Two operating systems are now settled by test: **macOS**, on the Creator's own Mac, and **Linux**, on a rented machine whose Blender matched the Mac's to the exact build hash. The two agreed on every test. **Windows is still not settled** — not by the sources and not by any test — and four Windows entries are written out ready to run in `Audits/Blender Research/Writing/Hand tests — Open Threads questions.md`. A Windows result must name the edition it was taken on. See *What the hand tests settled — question 14* at the foot of this document.

---

## What the hand tests settled — question 14

Fourteen tests were run for this question on **2026-08-30** by Claude Opus 5 in Claude Code, in all four researched versions of Blender, on two machines. Everything below is what **Blender 4.5.12 LTS** did. The full list, with every script written out in full and every entry's negative result stated, is `Audits/Blender Research/Writing/Hand tests — Open Threads questions.md`.

### The two machines

**The Creator's own Mac.** A MacBook Pro with an Apple M1 Pro chip, macOS **26.6.2** (build 25G83, kernel `Darwin 25.6.0`, arm64), Apple's own graphics and no second card. Blender **4.5.12 LTS**, hash `84afd5f785f7`, started from `/Applications/Blender 4.5.12.app/Contents/MacOS/Blender`. No system setting on that machine was changed.

**A rented Linux machine, in the United Kingdom.** Vast.ai instance `49255001`, machine `143739`, rented and destroyed on 2026-08-30. **A container, not a whole machine**, established by reading rather than assumed: `/.dockerenv` is present, `systemd-detect-virt` says `docker`, `/proc/1/cgroup` prints `0::/`, and process 1 is the provider's own launch script. Ubuntu **22.04.4 LTS**, kernel `7.0.0-28-generic`, glibc 2.35, x86-64. An **AMD Ryzen 9 3900X** with 24 visible threads against a quota of **11.52 cores**; **the container misreports its own size** — the ordinary memory reading says 50,423,779,328 bytes while its own ceiling is 34,227,617,792. A 40 GB overlay root. One **NVIDIA GeForce RTX 5060 with 8151 MiB on driver 595.71.05**. **No display server as rented**; `Xvfb` was added to it for the window test, where the graphics are software (`llvmpipe`). Blender **4.5.12 LTS**, hash `84afd5f785f7`, at `/opt/blender/4.5/blender`, taken from Blender's own download server as the Linux x64 build — **its hash matches the Mac's exactly**, so any difference between the two machines is a difference of operating system and not of build.

**Every render on both machines was a processor render.** Cycles was set to `CPU` throughout, except in the one test that used EEVEE.

**The two results are never merged.** The Mac settles macOS; the Linux machine settles Linux. **Windows is settled by neither.** Four Windows entries are written out ready to run in the list named above, for the runner's separate Windows batch, and no Windows answer is claimed here.

### How fine the instrument was

This question can be answered wrongly by a slow instrument: a watcher reading the folder once a second sees every frame already finished and calls that "one after the other" whatever Blender did. So the folder was read **every two milliseconds** — on the Mac a reading completed every 3.2 milliseconds on average — and the write was made slow on purpose, a 3000 by 3000 OpenEXR at 32 bits with no compression, 144 megabytes a frame. One frame's write took about 190 milliseconds on the Mac and about 60 on Linux, and the watcher caught **63 separate readings of one file growing** on the Mac and **31** on Linux. Two frames' writes running at the same time could not have been missed at that rate. Blender was also asked, not only watched: a script printed a stamped line at every render event. And on Linux **every file call Blender made was traced**, thread by thread, so one result rests on no polling at all.

Every run used `--factory-startup` and Blender's own default scene, so no add-on and no saved preference could reach any of it, and no `.blend` file was used.

### 1. The order Blender itself reports, driven from outside its own window

Five frames to PNG, 512 by 512, one sample. For every frame in the range, `render_write` for frame N was printed **before** `render_pre` for frame N+1.

- On the **Mac**, the four gaps were +0.000023, +0.000025, +0.000021, +0.000023 seconds. Frames out of order: **0**.
- On the **rented Linux machine**, +0.000029, +0.000028, +0.000027, +0.000025 seconds. Frames out of order: **0**.

The two machines agreed.

### 2. The disk itself, with a write slow enough to catch in the act

Five frames, 3000 by 3000 OpenEXR, uncompressed. Each file's first appearance, its last change of size, and its final size — on the **Mac**:

```
frame_0001.exr                     1788091500.294537  2306197 1788091500.294537 1788091500.487492    144048725
frame_0002.exr                     1788091501.056115    28821 1788091501.056115 1788091501.274866    144048725
frame_0003.exr                     1788091501.873625   372885 1788091501.873625 1788091502.071154    144048725
frame_0004.exr                     1788091502.629432  1749141 1788091502.629432 1788091502.826767    144048725
frame_0005.exr                     1788091503.398357  1876117 1788091503.398357 1788091503.641074    144048725
```

On the **rented Linux machine**:

```
frame_0001.exr                     1788091371.145853  2281472 1788091371.145853 1788091371.206872    144048725
frame_0002.exr                     1788091372.237051        0 1788091372.239196 1788091372.298145    144048725
frame_0003.exr                     1788091373.323635  3049229 1788091373.323635 1788091373.383216    144048725
frame_0004.exr                     1788091374.408687   456797 1788091374.408687 1788091374.470566    144048725
frame_0005.exr                     1788091375.495755        0 1788091375.497937 1788091375.557424    144048725
```

Whether any file's size changed **after** a later file first appeared: **none** on the Mac, **none** on Linux. No name ever appeared in the folder that was not one of the final frame names, so no file was written under a temporary name and moved into place. The two machines agreed.

### 3. Whether the file is finished at the moment Blender says it is written

The write handler asked Blender for the frame's path with `scene.render.frame_path(frame=scene.frame_current)`, then read the whole file and printed the byte count beside the size the file system reported. On the **Mac**:

```
EVT 1788091506.771208 readback frame=1 stat=144048725 read=144048725 path=…/frame_0001.exr
EVT 1788091507.811302 readback frame=2 stat=144048725 read=144048725 path=…/frame_0002.exr
EVT 1788091508.843073 readback frame=3 stat=144048725 read=144048725 path=…/frame_0003.exr
EVT 1788091509.956194 readback frame=4 stat=144048725 read=144048725 path=…/frame_0004.exr
EVT 1788091510.977494 readback frame=5 stat=144048725 read=144048725 path=…/frame_0005.exr
```

On the **rented Linux machine**:

```
EVT 1788091378.483478 readback frame=1 stat=144048725 read=144048725 path=…/frame_0001.exr
EVT 1788091379.596248 readback frame=2 stat=144048725 read=144048725 path=…/frame_0002.exr
EVT 1788091380.690421 readback frame=3 stat=144048725 read=144048725 path=…/frame_0003.exr
EVT 1788091381.794025 readback frame=4 stat=144048725 read=144048725 path=…/frame_0004.exr
EVT 1788091382.905234 readback frame=5 stat=144048725 read=144048725 path=…/frame_0005.exr
```

Those counts are the same numbers test 2 recorded as the finished size of a frame. The two machines agreed.

### 4. Placeholders on, overwrite off

Five frames, the same large EXR, with `use_placeholder` on and `use_overwrite` off. The empty files appear **one at a time, not all at the start**. On the **Mac**:

```
frame_0001.exr                     1788091513.054935        0 1788091513.887029 1788091514.073103    144048725
frame_0002.exr                     1788091514.076276        0 1788091514.841962 1788091515.079553    144048725
frame_0003.exr                     1788091515.079553        0 1788091515.870960 1788091516.079393    144048725
frame_0004.exr                     1788091516.079393        0 1788091516.818775 1788091517.044490    144048725
frame_0005.exr                     1788091517.044490        0 1788091517.634836 1788091517.866813    144048725
```

On the **rented Linux machine**:

```
frame_0001.exr                     1788091384.678449        0 1788091385.727257 1788091385.788700    144048725
frame_0002.exr                     1788091385.788700        0 1788091386.803180 1788091386.860014    144048725
frame_0003.exr                     1788091386.860014        0 1788091387.855557 1788091387.914738    144048725
frame_0004.exr                     1788091387.914738        0 1788091388.943600 1788091389.003071    144048725
frame_0005.exr                     1788091389.003071        0 1788091390.035984 1788091390.097757    144048725
```

The `at_size` column is zero for every file at first sight, and `first_nonzero` is when the picture began to be written into it. The empty file for a frame appears within a few milliseconds of that frame's render beginning, sits at zero for the whole of that frame's render, and then grows to full size; the next frame's empty file appears within one reading of that. Only one empty file is ever waiting at a time, and the write event fires once per frame, not twice. The two machines agreed.

### 5. A File Output node's file against the main output file

Three frames, the same large EXR, with a File Output node in the compositor writing into a second folder and compositing switched on. Both folders were watched together. On the **Mac**:

```
Image0001.exr                      1788092109.459886   552920 1788092109.459886 1788092109.737541    144048536
frame_0001.exr                     1788092109.737541  1593493 1788092109.737541 1788092109.995838    144048725
Image0002.exr                      1788092110.535540   573400 1788092110.535540 1788092110.836068    144048536
frame_0002.exr                     1788092110.839411  2224277 1788092110.839411 1788092111.108505    144048725
Image0003.exr                      1788092111.831802   196568 1788092111.831802 1788092112.093323    144048536
frame_0003.exr                     1788092112.093323    21205 1788092112.093323 1788092112.453674    144048725
```

On the **rented Linux machine**:

```
Image0001.exr                      1788092151.167307  2330624 1788092151.167307 1788092151.228614    144048536
frame_0001.exr                     1788092151.230720  1802240 1788092151.230720 1788092151.294153    144048725
Image0002.exr                      1788092152.418832    16384 1788092152.418832 1788092152.485228    144048536
frame_0002.exr                     1788092152.485228   312773 1788092152.485228 1788092152.549834    144048725
Image0003.exr                      1788092153.663356    16384 1788092153.663356 1788092153.730116    144048536
frame_0003.exr                     1788092153.730116   131072 1788092153.730116 1788092153.796715    144048725
```

The node's file for a frame is written first, the scene's own output file second, and both are finished before the next frame's node file appears. The two machines agreed.

### 6. The preview picture saved beside a frame

Three frames, the same large EXR, with `image_settings.use_preview` on. On the **Mac**:

```
frame_0001.exr                     1788091526.828737  1523861 1788091526.828737 1788091527.071174    144048725
frame_0001.jpg                     1788091527.178083        0 1788091527.181316 1788091527.206520       232358
frame_0002.exr                     1788091527.894960   798869 1788091527.894960 1788091528.145210    144048725
frame_0002.jpg                     1788091528.207566     4096 1788091528.207566 1788091528.235918       232358
frame_0003.exr                     1788091528.836669   188565 1788091528.836669 1788091529.080792    144048725
frame_0003.jpg                     1788091529.148732     4096 1788091529.148732 1788091529.174877       232358
```

On the **rented Linux machine**:

```
frame_0001.exr                     1788091398.482431   172032 1788091398.482431 1788091398.541299    144048725
frame_0001.jpg                     1788091398.690496     4096 1788091398.690496 1788091398.717804       232508
frame_0002.exr                     1788091399.719300  2383872 1788091399.719300 1788091399.778618    144048725
frame_0002.jpg                     1788091399.856487     4096 1788091399.856487 1788091399.896491       232508
frame_0003.exr                     1788091400.925377   319488 1788091400.925377 1788091400.985226    144048725
frame_0003.jpg                     1788091401.059538        0 1788091401.061698 1788091401.095722       232508
```

The frame's own file is written first and the preview second, and both are finished before the next frame's file appears. The two machines agreed.

### 7. Two views in one frame

Three frames with `use_multiview` on, `views_format` `STEREO_3D`, and the image settings' `views_format` `INDIVIDUAL`, so each view gets a file. On the **Mac**:

```
frame_0001_L.exr                   1788091533.215742   704696 1788091533.215742 1788091533.477392    144048760
frame_0001_R.exr                   1788091533.480663  1470649 1788091533.480663 1788091533.727009    144048761
frame_0002_L.exr                   1788091535.088559   565432 1788091535.088559 1788091535.316534    144048760
frame_0002_R.exr                   1788091535.316534   794809 1788091535.316534 1788091535.535008    144048761
frame_0003_L.exr                   1788091536.786815    21240 1788091536.786815 1788091537.001298    144048760
frame_0003_R.exr                   1788091537.001298  1847481 1788091537.001298 1788091537.221376    144048761
```

On the **rented Linux machine**:

```
frame_0001_L.exr                   1788091404.910087   792888 1788091404.910087 1788091404.973449    144048760
frame_0001_R.exr                   1788091404.973449   131072 1788091404.973449 1788091405.036920    144048761
frame_0002_L.exr                   1788091407.105284   360816 1788091407.105284 1788091407.164454    144048760
frame_0002_R.exr                   1788091407.166875  4112384 1788091407.166875 1788091407.226665    144048761
frame_0003_L.exr                   1788091409.285753  2523136 1788091409.285753 1788091409.347752    144048760
frame_0003_R.exr                   1788091409.347752  1515520 1788091409.347752 1788091409.413913    144048761
```

The left view is written first and the right second, and both are finished before the next frame's left view appears. In several runs the left view's last change and the right view's first appearance fall in the **same** reading; at two milliseconds those two events cannot be told apart and nothing is claimed about them. What is claimed is only the gap of one to three seconds before the next frame's files.

### 8. A video container: does the one file grow as the run goes

Forty frames at 1920 by 1920 into a video container, written losslessly with the `HUFFYUV` codec so that the finished file is about 153 megabytes — far larger than any write buffer. At the default settings the same scene compresses down to about twenty kilobytes for a short run, which is small enough to sit inside a write buffer and settle nothing.

| | Mac | rented Linux machine |
|---|---|---|
| size changes seen during the run | 56 | 62 |
| bytes on disk at the half-way frame | 68,681,728 | 72,613,888 |
| bytes on disk when the last frame was written | 141,557,760 | 145,489,920 |
| final size | 153,196,987 | 153,196,827 |
| size changes after the last frame's write | 4 | 2 |

The file grows steadily through the run: about half of it is on disk by the half-way frame and about nine tenths by the time the last frame is written, with the rest added afterwards. The two machines agreed.

### 9. The same video render stopped part way

Run only on the **rented Linux machine**, because a media reader was needed and `ffprobe` was installed there; nothing was installed on the Creator's Mac. The same forty-frame lossless video render was interrupted after six seconds and the part-finished file handed to `ffprobe`.

A file was left behind, **88,088,476 bytes**. `ffprobe` read it as `matroska,webm` with codec `huffyuv` and a duration of 0.959000 seconds, and a decoder walking it with `-count_frames` read **23 frames**. Blender had announced **23** frames written before the interrupt. *(The frame counts differ between versions because the interrupt landed at a fixed six seconds into renders of different speeds, not because Blender differs.)*

**This settles macOS not at all.** It is a Linux result only.

### 10. A list of frames given out of order instead of a range

Started with Blender's own frame argument rather than from a script — `-o <folder>/f_#### -F OPEN_EXR -x 1 -f 5,1,3` — with a script that only set the engine, samples and resolution and rendered nothing. Blender's own lines on the **Mac**:

```
Saved: '…/f_0005.exr'
Saved: '…/f_0001.exr'
Saved: '…/f_0003.exr'
```

and the watcher's order of first appearance:

```
1788091777.726858  f_0005.exr
1788091777.760890  f_0001.exr
1788091777.791151  f_0003.exr
```

On the **rented Linux machine**:

```
1788091436.918954  f_0005.exr
1788091436.978665  f_0001.exr
1788091437.033647  f_0003.exr
```

The frames are written **in the order the list gives them**, not in numeric order. The two machines agreed.

### 11. The other thing a frame writes: Cycles' tile cache

Three frames at 4000 by 4000 with `use_auto_tile` on and `tile_size` 256, with Blender's temporary folder pointed at a folder of the test's own and watched alongside the output folder. On the **Mac**:

```
cycles-tile-buffer-71629-31687314144-0-0.exr 1788091779.751345     8192 1788091779.751345 1788091780.994685      9562622
frame_0001.exr                     1788091780.994685    29397 1788091780.994685 1788091781.355685    256064725
cycles-tile-buffer-71629-31687314144-1-0.exr 1788091781.359167        0 1788091781.362397 1788091782.549616      9562622
frame_0002.exr                     1788091782.552757  1941461 1788091782.552757 1788091782.912197    256064725
cycles-tile-buffer-71629-31687314144-2-0.exr 1788091782.915372        0 1788091782.919034 1788091784.175287      9562622
frame_0003.exr                     1788091784.175287      725 1788091784.175287 1788091784.542279    256064725
```

and the watcher's record of those files going away:

```
1788091780.994685 GONE  …/cycles-tile-buffer-71629-31687314144-0-0.exr
1788091782.549616 GONE  …/cycles-tile-buffer-71629-31687314144-1-0.exr
1788091784.175287 GONE  …/cycles-tile-buffer-71629-31687314144-2-0.exr
```

On the **rented Linux machine**:

```
cycles-tile-buffer-3873-136120314047168-0-0.exr 1788091438.815803        0 1788091438.817995 1788091441.319149      9562614
frame_0001.exr                     1788091441.321304  1409024 1788091441.321304 1788091441.417305    256064725
cycles-tile-buffer-3873-136119828118208-1-0.exr 1788091441.423666     8192 1788091441.423666 1788091443.899414      9562614
frame_0002.exr                     1788091443.901574  3425149 1788091443.901574 1788091443.993644    256064725
cycles-tile-buffer-3873-136119828125376-2-0.exr 1788091444.000063     8192 1788091444.000063 1788091446.594164      9562614
frame_0003.exr                     1788091446.596299   800821 1788091446.596299 1788091446.693127    256064725
```

One tile file per frame, written in the temporary folder during that frame's render, finished and **removed** no later than the reading in which that frame's own output file first appears. The two machines agreed. *(Narrowed by the citation check of 2026-08-30: the watcher stamps every event in one sweep with a single time, so where a removal and the next file's first appearance carry the same stamp — two of the three frames in the Mac run above — the log cannot show which came first. Test 14's system-call trace does show it, and shows the removal about two milliseconds before the frame's own file is opened, but that trace was taken on the Linux machine only and does not speak for the Mac.)*

### 12. The same run with EEVEE instead of Cycles

Five frames, the same large EXR, with the engine set to EEVEE. This version offers the engine under the name **`BLENDER_EEVEE_NEXT`**, which the script printed. On the **Mac**:

```
frame_0001.exr                     1788091547.821878   651224 1788091547.821878 1788091548.034419    144048536
frame_0002.exr                     1788091550.083414   380888 1788091550.083414 1788091550.343766    144048536
frame_0003.exr                     1788091552.406035    21016 1788091552.406035 1788091552.756631    144048536
frame_0004.exr                     1788091554.894931  1572824 1788091554.894931 1788091555.156508    144048536
frame_0005.exr                     1788091557.228851  1126360 1788091557.228851 1788091557.518307    144048536
```

On the **rented Linux machine**:

```
frame_0001.exr                     1788091415.416457  1507328 1788091415.416457 1788091415.476100    144048536
frame_0002.exr                     1788091415.833729  2260992 1788091415.833729 1788091415.894901    144048536
frame_0003.exr                     1788091416.258368  1320752 1788091416.258368 1788091416.323821    144048536
frame_0004.exr                     1788091416.682275   327680 1788091416.682275 1788091416.746094    144048536
frame_0005.exr                     1788091417.105966  2719744 1788091417.105966 1788091417.165706    144048536
```

Whether any file's size changed after a later file appeared: **none** on the Mac, **none** on Linux. The answer is the same as Cycles', so the ordering is not a property of the render engine. The two machines agreed.

### 13. The same render with Blender's own window open

Five frames, the same large EXR, with Blender started **without** `-b`, so it opened its own window, and the render started with `bpy.ops.render.render('INVOKE_DEFAULT', animation=True)` — which is how the Render Animation menu item starts it, as a job the interface keeps running alongside. A script quit Blender once the job reported itself finished, so no hand touched a mouse. On the Mac this was Blender's own window on the Creator's screen; on Linux it was a window on an `Xvfb` display server added to the machine for this test, where the graphics are software.

The script printed `background False` and `WINDOWS 1`, and `INVOKE-RETURNED` printed about twenty milliseconds after the render was started, so the render really was a job running alongside rather than a call that blocks.

For every frame in the range, `render_write` for frame N was printed **before** `render_pre` for frame N+1.

- On the **Mac**, the four gaps were +0.000028, +0.000029, +0.000027, +0.000032 seconds. Frames out of order: **0**.
- On the **rented Linux machine**, +0.000046, +0.000046, +0.000055, +0.000036 seconds. Frames out of order: **0**.

And the disk agreed. On the **Mac**:

```
frame_0001.exr                     1788092854.165972   323733 1788092854.165972 1788092854.427498    144048725
frame_0002.exr                     1788092855.596438    13013 1788092855.596438 1788092855.843853    144048725
frame_0003.exr                     1788092857.070861    21205 1788092857.070861 1788092857.279882    144048725
frame_0004.exr                     1788092858.496915  2125973 1788092858.496915 1788092858.697168    144048725
frame_0005.exr                     1788092859.885102  1400981 1788092859.885102 1788092860.097684    144048725
```

On the **rented Linux machine**:

```
frame_0001.exr                     1788092571.877672  1179648 1788092571.877672 1788092571.949569    144048725
frame_0002.exr                     1788092576.311124  1507328 1788092576.311124 1788092576.384986    144048725
frame_0003.exr                     1788092578.051620   942080 1788092578.051620 1788092578.124396    144048725
frame_0004.exr                     1788092579.610736  3293184 1788092579.610736 1788092579.672379    144048725
frame_0005.exr                     1788092580.905525  1277952 1788092580.905525 1788092581.004240    144048725
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
cycles-tile-buffer-6075-138195051715264-0-0.exr 43903.458714   6129 43903.460434 43903.981154 43904.361076      461  6129:461
frame_0001.exr             43904.445065   6075 43904.445442 43904.681205 43904.681310     3006  6075:3006
cycles-tile-buffer-6075-138195064839872-1-0.exr 43904.955549   6130 43904.956832 43905.412522 43905.810841      461  6130:461
frame_0002.exr             43905.894159   6075 43905.894487 43906.122261 43906.122363     3006  6075:3006
cycles-tile-buffer-6075-138195051715264-2-0.exr 43906.387886   6131 43906.389037 43906.875441 43907.269573      461  6131:461
frame_0003.exr             43907.353200   6075 43907.353536 43907.597383 43907.597479     3006  6075:3006

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

2026-08-30 — Claude Opus 5 (Claude Code) — Writing, Blender 4.5

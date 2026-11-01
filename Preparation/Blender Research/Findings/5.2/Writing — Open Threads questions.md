# Writing — Open Threads questions (Blender 5.2)

Sources consulted, all read on **2026-07-30** (the snapshot date):

- Blender 5.2 manual (plain text copy), including its `_images` folder
- Blender 5.2 Python API documentation (plain text copy)
- Blender 5.2 release notes (all subject files)
- Cycles' own files as shipped in Blender 5.2 (`__init__.py`, `engine.py`, `properties.py`, `ui.py`, `operators.py`)

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

I started no helper agents. Every search and every file opened below was done by me.

---

## 14. When a render works through a range of frames, how do the writes belonging to one frame stand in time against the writes belonging to the frame after it?

**LABEL: not in the sources** — for the question as asked.

The sources nowhere state the time relationship between the files written for one frame and the files written for the next. In particular, nothing in the 5.2 manual, the 5.2 Python API documentation, the 5.2 release notes, or Cycles' own files says whether frame N's file is completely written and closed before any work on frame N+1 begins, whether the two can overlap, or whether writing happens on a separate thread from rendering. There is no statement about ordering, overlap, buffering, flushing, or atomicity of output files between frames.

What the sources *do* settle, which is less than the question asks, is set out below. I give each piece its own label so nothing here is mistaken for an answer to the timing question.

### What is documented, and what it does not settle

**(a) A render of a range walks the frames from start to end, advancing after each rendered frame by however many frames the step setting says.**

LABEL: **documented**

Exact words:

> "-a, --render-anim
>
> Render frames from start to end (inclusive)."

> "-j, --frame-jump <frames>
>
> Set number of frames to step forward after each rendered frame."

Source: Blender manual, https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html — read 2026-07-30.

> "Step
>
> Controls the number of frames to advance for each frame rendered, a value above 1 skips frames."

Source: Blender manual, https://docs.blender.org/manual/en/5.2/render/output/properties/frame_range.html — read 2026-07-30.

> "frame_start (int) – Start Frame, Frame to start rendering animation at. If not specified, the scene start frame will be assumed. This should only be specified if doing an animation render"

Source: Python API, https://docs.blender.org/api/5.2/bpy.ops.render.html — read 2026-07-30.

This gives the order the *frames* are taken in. It says nothing about when the *files* for those frames are written relative to one another.

**(b) Frames are typically saved to disk as they are rendered, rather than all at the end.**

LABEL: **documented**

Exact words:

> "While rendering stills will allow you to view and save the image from the render buffer when it is complete, animations are a series of images, or frames, and are typically saved directly to disk as they are rendered."

Source: Blender manual, https://docs.blender.org/manual/en/5.2/render/output/animation.html — read 2026-07-30.

> "This allows you an easy recovery if there is a problem and you have to re-start the rendering, since the frames you have already rendered will still be in the output directory."

Source: Blender manual, https://docs.blender.org/manual/en/5.2/render/output/animation.html — read 2026-07-30.

> "For example, when rendering animations to image file formats the render job can be canceled and resumed at the last rendered frame by changing the frame range."

Source: Blender manual, https://docs.blender.org/manual/en/5.2/render/output/introduction.html — read 2026-07-30.

This tells against one possible answer to the question — that the writes for all frames are held back and done together at the end. The manual's word is "typically", so it does not shut that possibility out flatly. It does not settle how close in time one frame's write sits to the next frame's, nor whether they can overlap.

**(c) There is a per-frame write event, and it is announced after the frame is written.**

LABEL: **documented**

Exact words:

> "bpy.app.handlers.render_write
>
> on writing a render frame (directly after the frame is written). Accepts one argument: the scene data-block being rendered"

Source: Python API, https://docs.blender.org/api/5.2/bpy.app.handlers.html — read 2026-07-30.

Also on the same page:

> "bpy.app.handlers.render_stats
>
> on printing render statistics. Accepts one argument: the render progress as a string containing current frame, current sample, render time and saving time."

Source: Python API, https://docs.blender.org/api/5.2/bpy.app.handlers.html — read 2026-07-30.

So writing is a distinct, named, per-frame step with its own reported time, and there is a moment Blender itself calls "directly after the frame is written". The documentation does not say what happens on either side of that moment with respect to the next frame.

**(d) There is a per-frame boundary in the render engine interface, and Cycles implements it.**

LABEL: **documented** (the API wording) and **observed** (Cycles' files)

Exact words, API:

> "render_frame_finish()
>
> Perform finishing operations after all view layers in a frame were rendered"

Source: Python API, https://docs.blender.org/api/5.2/bpy.types.RenderEngine.html — read 2026-07-30.

Exact lines, Cycles' own files:

`Blender Sources/cycles_5.2/__init__.py`, lines 76–77, inside `class CyclesRender(bpy.types.RenderEngine)`:

```
    def render_frame_finish(self):
        engine.render_frame_finish(self)
```

`Blender Sources/cycles_5.2/engine.py`, lines 95–100:

```
def render_frame_finish(engine):
    if not engine.session:
        return

    import _cycles
    _cycles.render_frame_finish(engine.session)
```

Read 2026-07-30.

The API page states what the method is for. Cycles' files show that a method of that name exists on Cycles' render class and hands off to Cycles' own built-in module. What the files do not show is a running render: whether Blender calls it, when it calls it, or what Cycles does inside it. Nor does either source say whether the output file for that frame has been written by the time this boundary is reached, or when the next frame starts relative to it.

**(e) When Placeholders is on, empty placeholder files are created while frames are being rendered.**

LABEL: **documented**

Exact words:

> "use_placeholder
>
> Create empty placeholder files while rendering frames (similar to Unix 'touch') (default False)"

Source: Python API, https://docs.blender.org/api/5.2/bpy.types.RenderSettings.html — read 2026-07-30.

> "Image Sequence – Overwrite Image Multi-Layer EXR
>
> Create empty placeholder frames while rendering."

> "Primitive Render Farm
>
> An easy way to get multiple machines to share the rendering workload is to:
>
> Set up a shared directory over a network file system.
>
> Disable Overwrite, enable Placeholders in the Render Output panel.
>
> Start as many machines as you wish rendering to that directory."

Source: Blender manual, https://docs.blender.org/manual/en/5.2/render/output/properties/output.html — read 2026-07-30.

I also opened the picture the same page names, `_images/render_output_properties_output_panel.png`. It is a screenshot of the Output panel. It shows the panel's controls; it carries no information about when anything is written. Nothing in it bears on this question.

What the quoted words carry is that these empty files are created during the rendering of frames. They do not say which frame a given empty file belongs to, nor that a frame's empty file comes before that same frame's real one.

**(e2) With Placeholders on, a frame has two writes and the empty one comes first.**

LABEL: **inferred**

The step I took: the quoted words call the file a placeholder and say it is empty and is created while frames are rendered, and the render-farm hint has several machines share a folder and rely on these files to divide the work between them. A file that stands in for a result, and that another machine reads to decide what is already taken, has to exist before the result does. So I take it that for one frame the empty file is written first and the real one after. No quoted sentence says this, so it is inferred, not documented.

Whether frame N+1's empty file is created before or after frame N's real file is finished is **not** stated by any source I found, and I did not infer it either. That is the heart of the question, and the sources do not answer it.

**(f) When the output is a video, one frame's bytes go into the same growing file as the next frame's.**

LABEL: **inferred**

Exact words:

> "Blender creates one file that holds all the frames of your animation."

> "Autosplit Output
>
> If your video is huge and exceeds 2GiB, enable Autosplit Output. This will automatically split the output into multiple files after the first file is 2GiB in size."

Sources: Blender manual, https://docs.blender.org/manual/en/5.2/render/output/animation.html and https://docs.blender.org/manual/en/5.2/render/output/properties/output.html — both read 2026-07-30.

The step I took: the manual says the file is split once it *is* 2GiB in size, during the render, which only makes sense if the file grows while the render is running rather than being written once at the end; and it says one file holds all the frames. From that I take it that successive frames are added to the same file as the render proceeds. The manual does not say this in so many words, so this is inferred, not documented. It still does not settle whether one frame's bytes are fully committed before the next frame's are produced.

**(g) A caution against reading (a)–(f) as an answer.**

The natural guess — frame N is rendered, written, closed, and only then does frame N+1 begin — is consistent with everything quoted above, but not one of these sources states it. Nothing here rules out a design in which writing a finished frame is handed off while the next frame's rendering has already begun. I have not found any source in Blender 5.2's manual, API documentation, release notes, or Cycles' files that closes that gap, so I am not going to close it by assumption.

**(h) On the narrower reading of the question, the frames' writes come out in ascending frame order.**

LABEL: **inferred**

The step I took: quote (a) says a render of a range goes from start to end and advances after each rendered frame, and quote (b) says frames are typically saved to disk as they are rendered. Put together, the files land in the order the frames are rendered in, which is ascending. What is documented is the order the frames are **rendered** in; no quoted sentence states the order the **writes** happen in, so this is inferred, not documented. It says nothing about how close the writes are in time or whether they overlap.

### Ambiguity in the question

The question can be read two ways, and both are answered above. The reading I think is meant — **do the writes for one frame stand strictly before the writes for the next frame in time, or can they overlap?** — is the one the answer at the top of this question comes back *not in the sources* on. The narrower reading — **in what order do the frames' writes occur**, ascending or not — is answered at (h), inferred.

### Not covered

- Whether frame N's file is closed, flushed, or complete on disk before any part of frame N+1 is rendered or written.
- Whether Blender ever writes two frames' files at overlapping times, on a separate thread or otherwise.
- Where the automatic output write for a frame sits relative to that same frame's File Output node writes, and whether either can trail into the next frame. (This is the within-frame ordering the question deliberately does not ask about, but it also means I cannot say which of a frame's writes is "the last" one.)
- Whether the setting called "Persistent Data" changes any of this. The page it sits on is in the record below as read, and I found nothing on it about when a frame is written.
- Whether a movie-format render commits a frame's bytes to disk at the same points an image-sequence render does.
- Whether the video sequencer render path, the compositor-only path, or a playblast behaves differently from a normal frame-range render.

### Outside the interface

The sources do not say. Everything quoted above is written without distinguishing a render started by a person clicking from one started on the command line, and the command-line page describes the very same frame-by-frame model ("Set number of frames to step forward after each rendered frame", https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html, read 2026-07-30). But since the timing question is unanswered inside the interface, it is equally unanswered outside it. I found no statement anywhere that write timing differs between the two.

**Brought up to date by the hand tests of 2026-08-30.** This is now settled by test rather than left to the sources. The same five-frame render was run in this version on two machines, once with no window at all and once with Blender's own window open, the render started the way the Render Animation menu item starts it — as a job the interface keeps running alongside. **The ordering was the same in all four runs**: for every frame, the write event for frame N came before the render event for frame N+1, and on disk no file was still growing when the next frame's file appeared. See *What the hand tests settled — question 14* at the foot of this document, tests 1 and 13.

### Operating system

The sources do not say. No page I read attaches any part of this to a particular operating system. The only place an operating system is named at all is the phrase "similar to Unix 'touch'" in the Python API's description of `use_placeholder`, which is an analogy for what the file looks like, not a statement that the behaviour is Unix-only. The render-farm hint on the Output page assumes "a shared directory over a network file system" without naming a system. Nothing states a per-operating-system difference, and nothing states there is none.

**Brought up to date by the hand tests of 2026-08-30.** Two operating systems are now settled by test: **macOS**, on the Creator's own Mac, and **Linux**, on a rented machine whose Blender matched the Mac's to the exact build hash. The two agreed on every test. **Windows is still not settled** — not by the sources and not by any test — and four Windows entries are written out ready to run in `Audits/Blender Research/Writing/Hand tests — Open Threads questions.md`. A Windows result must name the edition it was taken on. See *What the hand tests settled — question 14* at the foot of this document.

---

## Record of where I looked (question 14)

Searched, in every case across the whole of `manual_5.2`, `api_5.2` and `release_notes_5.2` at once, case-insensitively:

- Ordering and concurrency words: `one frame at a time`, `frame at a time`, `sequentially`, `in sequence`, `in parallel`, `simultaneously`, `asynchronous`, `concurrently`, `interleav`, `overlap`, `queue`, `pipeline`, `step forward`, `advance`, `after each`, `before the next`, `once the frame`, `when the frame`, `at the end of the render`, `finished rendering`
- Writing words: `written to disk`, `saved to disk`, `as they are rendered`, `each frame is`, `frame is written`, `write`, `writing`, `saving`, `flush`, `atomic`, `temporary file`, `.tmp`, `partial`, `incomplete`
- Job and progress words: `render job`, `background job`, `Rendering frame`, `Fra:`, `Saving:`, `resume`
- Feature words: `Placeholder`, `use_placeholder`, `use_overwrite`, `render_write`, `render_post`, `render_pre`, `render_frame_finish`, `Render Animation`, `Persistent Data`

Pages and files opened in full or in the relevant part:

- Manual: Rendering Animations (`render/output/animation`); Output introduction (`render/output/introduction`); Output panel and Encoding (`render/output/properties/output`); Frame Range (`render/output/properties/frame_range`); Post Processing (`render/output/properties/post_processing`); Rendering introduction (`render/introduction`); Rendering From The Command Line (`advanced/command_line/render`); Command Line Arguments (`advanced/command_line/arguments`); File Output Node (`compositing/types/output/file_output`); Cycles Performance settings (`render/cycles/render_settings/performance`); the Render menu section of the Topbar page (`interface/window_system/topbar`)
- Manual picture: `_images/render_output_properties_output_panel.png`, named on the Output panel page
- Python API: `bpy.app.handlers` (all of it, including the note on handlers and threads); `bpy.types.RenderSettings` (the `use_*` block); `bpy.types.RenderEngine` (all of it); `bpy.ops.render` (`render()` entry)
- Release notes 5.2: every subject file grepped for `write`, `writing`, `output path`, `saving`, `file output`. The only near hits were the File Output node's new File Extension checkbox (`compositor.md`), "Saving render output can now be disabled" (`rendering.md`), and faster half-precision EXR writing (`pipeline_io.md`). None of the three speaks to when a frame is written relative to the next frame, so I ruled them out.
- Cycles' own files: `__init__.py` (the `CyclesRender` class), `engine.py` (all of the render entry points). Grepped the whole of `cycles_5.2` for `frame`. The only frame-related item is `render_frame_finish`, quoted above. Cycles' files contain no file-writing code for render output; the shipped add-on is Python that hands work to the built-in `_cycles` module, whose source is not in this folder, and the `source/` folder inside `cycles_5.2` holds only `kernel` and `util` shading code.

Ruled out without opening in full, and why: everything under `manual_5.2/sculpt_paint`, `modeling`, `grease_pencil`, `animation`, `physics`, `movie_clip` and `video_editing` came up only on unrelated senses of the search words (grease pencil drawing "one frame at a time", sequencer strips "in sequence", fluid bake "resume"), and none of them concerns render output. `manual_5.2/files/media/image_formats` was checked for a statement about when an image file is committed and has none about render frame ordering.

---

## What the hand tests settled — question 14

Fourteen tests were run for this question on **2026-08-30** by Claude Opus 5 in Claude Code, in all four researched versions of Blender, on two machines. Everything below is what **Blender 5.2.1 LTS** did. The full list, with every script written out in full and every entry's negative result stated, is `Audits/Blender Research/Writing/Hand tests — Open Threads questions.md`.

### The two machines

**The Creator's own Mac.** A MacBook Pro with an Apple M1 Pro chip, macOS **26.6.2** (build 25G83, kernel `Darwin 25.6.0`, arm64), Apple's own graphics and no second card. Blender **5.2.1 LTS**, hash `9e2066aef7ef`, started from `/Applications/Blender.app/Contents/MacOS/Blender`. No system setting on that machine was changed.

**A rented Linux machine, in the United Kingdom.** Vast.ai instance `49255001`, machine `143739`, rented and destroyed on 2026-08-30. **A container, not a whole machine**, established by reading rather than assumed: `/.dockerenv` is present, `systemd-detect-virt` says `docker`, `/proc/1/cgroup` prints `0::/`, and process 1 is the provider's own launch script. Ubuntu **22.04.4 LTS**, kernel `7.0.0-28-generic`, glibc 2.35, x86-64. An **AMD Ryzen 9 3900X** with 24 visible threads against a quota of **11.52 cores**; **the container misreports its own size** — the ordinary memory reading says 50,423,779,328 bytes while its own ceiling is 34,227,617,792. A 40 GB overlay root. One **NVIDIA GeForce RTX 5060 with 8151 MiB on driver 595.71.05**. **No display server as rented**; `Xvfb` was added to it for the window test, where the graphics are software (`llvmpipe`). Blender **5.2.1 LTS**, hash `9e2066aef7ef`, at `/opt/blender/5.2/blender`, taken from Blender's own download server as the Linux x64 build — **its hash matches the Mac's exactly**, so any difference between the two machines is a difference of operating system and not of build.

**Every render on both machines was a processor render.** Cycles was set to `CPU` throughout, except in the one test that used EEVEE.

**The two results are never merged.** The Mac settles macOS; the Linux machine settles Linux. **Windows is settled by neither.** Four Windows entries are written out ready to run in the list named above, for the runner's separate Windows batch, and no Windows answer is claimed here.

### How fine the instrument was

This question can be answered wrongly by a slow instrument: a watcher reading the folder once a second sees every frame already finished and calls that "one after the other" whatever Blender did. So the folder was read **every two milliseconds** — on the Mac a reading completed every 3.2 milliseconds on average — and the write was made slow on purpose, a 3000 by 3000 OpenEXR at 32 bits with no compression, 144 megabytes a frame. One frame's write took about 190 milliseconds on the Mac and about 60 on Linux, and the watcher caught **63 separate readings of one file growing** on the Mac and **31** on Linux. Two frames' writes running at the same time could not have been missed at that rate. Blender was also asked, not only watched: a script printed a stamped line at every render event. And on Linux **every file call Blender made was traced**, thread by thread, so one result rests on no polling at all.

Every run used `--factory-startup` and Blender's own default scene, so no add-on and no saved preference could reach any of it, and no `.blend` file was used.

### 1. The order Blender itself reports, driven from outside its own window

Five frames to PNG, 512 by 512, one sample. For every frame in the range, `render_write` for frame N was printed **before** `render_pre` for frame N+1.

- On the **Mac**, the four gaps were +0.000034, +0.000035, +0.000029, +0.000031 seconds. Frames out of order: **0**.
- On the **rented Linux machine**, +0.000032, +0.000030, +0.000024, +0.000025 seconds. Frames out of order: **0**.

The two machines agreed.

### 2. The disk itself, with a write slow enough to catch in the act

Five frames, 3000 by 3000 OpenEXR, uncompressed. Each file's first appearance, its last change of size, and its final size — on the **Mac**:

```
frame_0001.exr                     1788091691.997322   454929 1788091691.997322 1788091692.214320    144048849
frame_0002.exr                     1788091693.261686  1892625 1788091693.261686 1788091693.485196    144048849
frame_0003.exr                     1788091694.593803   885009 1788091694.593803 1788091694.824474    144048849
frame_0004.exr                     1788091695.951746   782609 1788091695.951746 1788091696.162070    144048849
frame_0005.exr                     1788091697.254324    90385 1788091697.254324 1788091697.503279    144048849
```

On the **rented Linux machine**:

```
frame_0001.exr                     1788091645.985511    24577 1788091645.985511 1788091646.048520    144048849
frame_0002.exr                     1788091647.650492  1441792 1788091647.650492 1788091647.709914    144048849
frame_0003.exr                     1788091649.301992  2768896 1788091649.301992 1788091649.361468    144048849
frame_0004.exr                     1788091650.962587    24577 1788091650.962587 1788091651.022128    144048849
frame_0005.exr                     1788091652.627227  3522560 1788091652.627227 1788091652.689139    144048849
```

Whether any file's size changed **after** a later file first appeared: **none** on the Mac, **none** on Linux. No name ever appeared in the folder that was not one of the final frame names, so no file was written under a temporary name and moved into place. The two machines agreed.

### 3. Whether the file is finished at the moment Blender says it is written

The write handler asked Blender for the frame's path with `scene.render.frame_path(frame=scene.frame_current)`, then read the whole file and printed the byte count beside the size the file system reported. On the **Mac**:

```
EVT 1788091702.241552 readback frame=1 stat=144048849 read=144048849 path=…/frame_0001.exr
EVT 1788091703.742654 readback frame=2 stat=144048849 read=144048849 path=…/frame_0002.exr
EVT 1788091705.091231 readback frame=3 stat=144048849 read=144048849 path=…/frame_0003.exr
EVT 1788091706.445404 readback frame=4 stat=144048849 read=144048849 path=…/frame_0004.exr
EVT 1788091708.153515 readback frame=5 stat=144048849 read=144048849 path=…/frame_0005.exr
```

On the **rented Linux machine**:

```
EVT 1788091656.479509 readback frame=1 stat=144048849 read=144048849 path=…/frame_0001.exr
EVT 1788091658.211752 readback frame=2 stat=144048849 read=144048849 path=…/frame_0002.exr
EVT 1788091659.973440 readback frame=3 stat=144048849 read=144048849 path=…/frame_0003.exr
EVT 1788091661.734075 readback frame=4 stat=144048849 read=144048849 path=…/frame_0004.exr
EVT 1788091663.494269 readback frame=5 stat=144048849 read=144048849 path=…/frame_0005.exr
```

Those counts are the same numbers test 2 recorded as the finished size of a frame. The two machines agreed.

### 4. Placeholders on, overwrite off

Five frames, the same large EXR, with `use_placeholder` on and `use_overwrite` off. The empty files appear **one at a time, not all at the start**. On the **Mac**:

```
frame_0001.exr                     1788091711.580494        0 1788091712.899046 1788091713.051630    144048849
frame_0002.exr                     1788091713.051630        0 1788091714.151643 1788091714.294090    144048849
frame_0003.exr                     1788091714.297289        0 1788091715.397064 1788091715.533487    144048849
frame_0004.exr                     1788091715.533487        0 1788091716.644186 1788091716.771326    144048849
frame_0005.exr                     1788091716.771326        0 1788091717.892997 1788091718.027497    144048849
```

On the **rented Linux machine**:

```
frame_0001.exr                     1788091665.505723        0 1788091667.189860 1788091667.251116    144048849
frame_0002.exr                     1788091667.253210        0 1788091668.853742 1788091668.914983    144048849
frame_0003.exr                     1788091668.914983        0 1788091670.498729 1788091670.560298    144048849
frame_0004.exr                     1788091670.560298        0 1788091672.184731 1788091672.248800    144048849
frame_0005.exr                     1788091672.248800        0 1788091673.859588 1788091673.923373    144048849
```

The `at_size` column is zero for every file at first sight, and `first_nonzero` is when the picture began to be written into it. The empty file for a frame appears within a few milliseconds of that frame's render beginning, sits at zero for the whole of that frame's render, and then grows to full size; the next frame's empty file appears within one reading of that. Only one empty file is ever waiting at a time, and the write event fires once per frame, not twice. The two machines agreed.

### 5. A File Output node's file against the main output file

Three frames, the same large EXR, with a File Output node in the compositor writing into a second folder and compositing switched on. Both folders were watched together. On the **Mac**:

```
Unsaved0001.exr                    1788092131.048146   168120 1788092131.048146 1788092131.257534    144048760
frame_0001.exr                     1788092131.260663  1827089 1788092131.260663 1788092131.521203    144048849
Unsaved0002.exr                    1788092132.289769     4856 1788092132.289769 1788092132.503750    144048760
frame_0002.exr                     1788092132.503750   622865 1788092132.503750 1788092132.769606    144048849
Unsaved0003.exr                    1788092133.784101    32952 1788092133.784101 1788092133.999117    144048760
frame_0003.exr                     1788092133.999117  1683729 1788092133.999117 1788092134.242141    144048849
```

On the **rented Linux machine**:

```
Unsaved0001.exr                    1788092170.782400    24576 1788092170.782400 1788092170.846041    144048760
frame_0001.exr                     1788092170.858619    24577 1788092170.858619 1788092170.920244    144048849
Unsaved0002.exr                    1788092172.753217  2555904 1788092172.753217 1788092172.815109    144048760
frame_0002.exr                     1788092172.827772        0 1788092172.829907 1788092172.891834    144048849
Unsaved0003.exr                    1788092174.696798  2383872 1788092174.696798 1788092174.758844    144048760
frame_0003.exr                     1788092174.771550    98304 1788092174.771550 1788092174.835683    144048849
```

The node's file for a frame is written first, the scene's own output file second, and both are finished before the next frame's node file appears. The two machines agreed.

### 6. The preview picture saved beside a frame

Three frames, the same large EXR, with `image_settings.use_preview` on. On the **Mac**:

```
frame_0001.exr                     1788091728.306635  1130769 1788091728.306635 1788091728.545479    144048849
frame_0001.jpg                     1788091728.664718        0 1788091728.668006 1788091728.705502       235858
frame_0002.exr                     1788091729.605111  1155345 1788091729.605111 1788091729.861249    144048849
frame_0002.jpg                     1788091729.961076        0 1788091729.965136 1788091730.004107       235858
frame_0003.exr                     1788091730.890228   127249 1788091730.890228 1788091731.172777    144048849
frame_0003.jpg                     1788091731.251821        0 1788091731.255248 1788091731.287864       235858
```

On the **rented Linux machine**:

```
frame_0001.exr                     1788091684.648847    72857 1788091684.648847 1788091684.711854    144048849
frame_0001.jpg                     1788091684.937106        0 1788091684.939247 1788091684.966558       236082
frame_0002.exr                     1788091686.565128  1982464 1788091686.565128 1788091686.624299    144048849
frame_0002.jpg                     1788091686.768768     4096 1788091686.768768 1788091686.796247       236082
frame_0003.exr                     1788091688.424757  3817472 1788091688.424757 1788091688.486891    144048849
frame_0003.jpg                     1788091688.632430        0 1788091688.634602 1788091688.664400       236082
```

The frame's own file is written first and the preview second, and both are finished before the next frame's file appears. The two machines agreed.

### 7. Two views in one frame

Three frames with `use_multiview` on, `views_format` `STEREO_3D`, and the image settings' `views_format` `INDIVIDUAL`, so each view gets a file. On the **Mac**:

```
frame_0001_L.exr                   1788091736.053849  1868049 1788091736.053849 1788091736.308679    144048849
frame_0001_R.exr                   1788091736.311162  1179921 1788091736.311162 1788091736.539810    144048849
frame_0002_L.exr                   1788091738.392914  1134865 1788091738.392914 1788091738.611782    144048849
frame_0002_R.exr                   1788091738.611782   860433 1788091738.611782 1788091738.846810    144048849
frame_0003_L.exr                   1788091740.337021  1319185 1788091740.337021 1788091740.564440    144048849
frame_0003_R.exr                   1788091740.571161  1245457 1788091740.571161 1788091740.790316    144048849
```

On the **rented Linux machine**:

```
frame_0001_L.exr                   1788091693.898089  1033017 1788091693.898089 1788091693.961131    144048849
frame_0001_R.exr                   1788091693.961131  2228224 1788091693.961131 1788091694.024364    144048849
frame_0002_L.exr                   1788091697.244362  1572864 1788091697.244362 1788091697.305707    144048849
frame_0002_R.exr                   1788091697.305707   638976 1788091697.305707 1788091697.367149    144048849
frame_0003_L.exr                   1788091700.579551  2490368 1788091700.579551 1788091700.641606    144048849
frame_0003_R.exr                   1788091700.641606  1892352 1788091700.641606 1788091700.701450    144048849
```

The left view is written first and the right second, and both are finished before the next frame's left view appears. In several runs the left view's last change and the right view's first appearance fall in the **same** reading; at two milliseconds those two events cannot be told apart and nothing is claimed about them. What is claimed is only the gap of one to three seconds before the next frame's files.

### 8. A video container: does the one file grow as the run goes

Forty frames at 1920 by 1920 into a video container, written losslessly with the `HUFFYUV` codec so that the finished file is about 153 megabytes — far larger than any write buffer. At the default settings the same scene compresses down to about twenty kilobytes for a short run, which is small enough to sit inside a write buffer and settle nothing.

| | Mac | rented Linux machine |
|---|---|---|
| size changes seen during the run | 69 | 49 |
| bytes on disk at the half-way frame | 72,613,888 | 72,613,888 |
| bytes on disk when the last frame was written | 141,557,760 | 141,557,760 |
| final size | 153,189,630 | 153,190,910 |
| size changes after the last frame's write | 3 | 3 |

The file grows steadily through the run: about half of it is on disk by the half-way frame and about nine tenths by the time the last frame is written, with the rest added afterwards. The two machines agreed.

### 9. The same video render stopped part way

Run only on the **rented Linux machine**, because a media reader was needed and `ffprobe` was installed there; nothing was installed on the Creator's Mac. The same forty-frame lossless video render was interrupted after six seconds and the part-finished file handed to `ffprobe`.

A file was left behind, **68,936,300 bytes**. `ffprobe` read it as `matroska,webm` with codec `huffyuv` and a duration of 0.750000 seconds, and a decoder walking it with `-count_frames` read **18 frames**. Blender had announced **17** frames written before the interrupt. *(The frame counts differ between versions because the interrupt landed at a fixed six seconds into renders of different speeds, not because Blender differs.)*

**This settles macOS not at all.** It is a Linux result only.

### 10. A list of frames given out of order instead of a range

Started with Blender's own frame argument rather than from a script — `-o <folder>/f_#### -F OPEN_EXR -x 1 -f 5,1,3` — with a script that only set the engine, samples and resolution and rendered nothing. Blender's own lines on the **Mac**:

```
00:00.881  render           | Saved: '…/f_0005.exr'
00:00.930  render           | Saved: '…/f_0001.exr'
00:00.969  render           | Saved: '…/f_0003.exr'
```

and the watcher's order of first appearance:

```
1788091834.628797  f_0005.exr
1788091834.679097  f_0001.exr
1788091834.721883  f_0003.exr
```

On the **rented Linux machine**:

```
1788091734.214476  f_0005.exr
1788091734.299316  f_0001.exr
1788091734.371311  f_0003.exr
```

The frames are written **in the order the list gives them**, not in numeric order. The two machines agreed.

### 11. The other thing a frame writes: Cycles' tile cache

Three frames at 4000 by 4000 with `use_auto_tile` on and `tile_size` 256, with Blender's temporary folder pointed at a folder of the test's own and watched alongside the output folder. On the **Mac**:

```
cycles-tile-buffer-71958-53024910320-0-0.exr 1788091837.247828     8192 1788091837.247828 1788091839.511818      9564818
frame_0001.exr                     1788091839.511818   430161 1788091839.511818 1788091840.130259    256064849
cycles-tile-buffer-71958-53024910320-1-0.exr 1788091840.138931    12288 1788091840.138931 1788091842.902228      9564818
frame_0002.exr                     1788091842.902228   241745 1788091842.902228 1788091843.413195    256064849
cycles-tile-buffer-71958-53024910320-2-0.exr 1788091843.423054     8192 1788091843.423054 1788091846.278470      9564818
frame_0003.exr                     1788091846.278470  1286225 1788091846.278470 1788091846.731483    256064849
```

and the watcher's record of those files going away:

```
1788091839.511818 GONE  …/cycles-tile-buffer-71958-53024910320-0-0.exr
1788091842.902228 GONE  …/cycles-tile-buffer-71958-53024910320-1-0.exr
1788091846.278470 GONE  …/cycles-tile-buffer-71958-53024910320-2-0.exr
```

On the **rented Linux machine**:

```
cycles-tile-buffer-5970-125534205381712-0-0.exr 1788091736.409555        0 1788091736.411711 1788091739.307969      9564921
frame_0001.exr                     1788091739.312231    32769 1788091739.312231 1788091739.410535    256064849
cycles-tile-buffer-5970-125534204087376-1-0.exr 1788091739.437956        0 1788091739.440101 1788091742.275862      9564921
frame_0002.exr                     1788091742.280138  3112960 1788091742.280138 1788091742.378503    256064849
cycles-tile-buffer-5970-125534204128272-2-0.exr 1788091742.404423     8192 1788091742.404423 1788091745.281649      9564921
frame_0003.exr                     1788091745.285938    32769 1788091745.285938 1788091745.382482    256064849
```

One tile file per frame, written in the temporary folder during that frame's render, finished and **removed** no later than the reading in which that frame's own output file first appears. The two machines agreed. *(Narrowed by the citation check of 2026-08-30: the watcher stamps every event in one sweep with a single time, so where a removal and the next file's first appearance carry the same stamp — two of the three frames in the Mac run above — the log cannot show which came first. Test 14's system-call trace does show it, and shows the removal about two milliseconds before the frame's own file is opened, but that trace was taken on the Linux machine only and does not speak for the Mac.)*

### 12. The same run with EEVEE instead of Cycles

Five frames, the same large EXR, with the engine set to EEVEE. This version offers the engine under the name **`BLENDER_EEVEE`**, which the script printed. On the **Mac**:

```
frame_0001.exr                     1788091747.769239   491604 1788091747.769239 1788091747.960731    144048660
frame_0002.exr                     1788091749.956017  1388628 1788091749.956017 1788091750.146200    144048660
frame_0003.exr                     1788091752.180402   647252 1788091752.180402 1788091752.384518    144048660
frame_0004.exr                     1788091754.415693   106580 1788091754.415693 1788091754.622374    144048660
frame_0005.exr                     1788091756.677249  2252884 1788091756.677249 1788091756.871392    144048660
```

On the **rented Linux machine**:

```
frame_0001.exr                     1788091719.794583  1261568 1788091719.794583 1788091719.855610    144048660
frame_0002.exr                     1788091720.552601  1224860 1788091720.552601 1788091720.614015    144048660
frame_0003.exr                     1788091721.313661  2097152 1788091721.313661 1788091721.370993    144048660
frame_0004.exr                     1788091722.071373    81920 1788091722.071373 1788091722.129181    144048660
frame_0005.exr                     1788091722.826713    24580 1788091722.826713 1788091722.882664    144048660
```

Whether any file's size changed after a later file appeared: **none** on the Mac, **none** on Linux. The answer is the same as Cycles', so the ordering is not a property of the render engine. The two machines agreed.

### 13. The same render with Blender's own window open

Five frames, the same large EXR, with Blender started **without** `-b`, so it opened its own window, and the render started with `bpy.ops.render.render('INVOKE_DEFAULT', animation=True)` — which is how the Render Animation menu item starts it, as a job the interface keeps running alongside. A script quit Blender once the job reported itself finished, so no hand touched a mouse. On the Mac this was Blender's own window on the Creator's screen; on Linux it was a window on an `Xvfb` display server added to the machine for this test, where the graphics are software.

The script printed `background False` and `WINDOWS 1`, and `INVOKE-RETURNED` printed about twenty milliseconds after the render was started, so the render really was a job running alongside rather than a call that blocks.

For every frame in the range, `render_write` for frame N was printed **before** `render_pre` for frame N+1.

- On the **Mac**, the four gaps were +0.000033, +0.000029, +0.000039, +0.000036 seconds. Frames out of order: **0**.
- On the **rented Linux machine**, +0.000049, +0.000065, +0.000095, +0.000058 seconds. Frames out of order: **0**.

And the disk agreed. On the **Mac**:

```
frame_0001.exr                     1788092885.095446   553233 1788092885.095446 1788092885.368844    144048849
frame_0002.exr                     1788092886.630070  1265937 1788092886.630070 1788092886.917347    144048849
frame_0003.exr                     1788092888.177145   434449 1788092888.177145 1788092888.462291    144048849
frame_0004.exr                     1788092889.695048   442641 1788092889.695048 1788092889.945384    144048849
frame_0005.exr                     1788092891.209940  1798417 1788092891.209940 1788092891.450986    144048849
```

On the **rented Linux machine**:

```
frame_0001.exr                     1788092623.323869   819200 1788092623.323869 1788092623.391124    144048849
frame_0002.exr                     1788092625.974384  2617281 1788092625.974384 1788092626.067685    144048849
frame_0003.exr                     1788092628.008736    24577 1788092628.008736 1788092628.090671    144048849
frame_0004.exr                     1788092629.987761   696961 1788092629.987761 1788092630.064531    144048849
frame_0005.exr                     1788092632.108041    24577 1788092632.108041 1788092632.192564    144048849
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
cycles-tile-buffer-6282-138027233578064-0-0.exr 43937.123920   6319 43937.125217 43937.906622 43938.291315      461  6319:461
frame_0001.exr             43938.620063   6282 43938.620413 43938.855327 43938.855430     3006  6282:3006
cycles-tile-buffer-6282-138027233070160-1-0.exr 43939.166002   6320 43939.167188 43939.927152 43940.311060      461  6320:461
frame_0002.exr             43940.637206   6282 43940.637467 43940.879978 43940.880074     3006  6282:3006
cycles-tile-buffer-6282-138027233553424-2-0.exr 43941.200955   6321 43941.202019 43941.948639 43942.322686      461  6321:461
frame_0003.exr             43942.646487   6282 43942.646768 43942.899954 43942.900059     3006  6282:3006

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

2026-08-29 — Claude Opus 5 (Claude Code) — Writing (Open Threads), Blender 5.2

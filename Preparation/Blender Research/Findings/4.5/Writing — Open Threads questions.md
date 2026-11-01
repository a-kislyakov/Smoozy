# Writing — Open Threads questions (Blender 4.5)

**Sources consulted, and when.** Blender 4.5 manual (plain-text copy in `Blender Sources/manual_4.5`), Blender 4.5 Python API documentation (`Blender Sources/api_4.5`), Blender 4.5 release notes (`Blender Sources/release_notes_4.5`), and Cycles' own files (`Blender Sources/cycles_4.5`). All read on **2026-07-30** (the snapshot date given).

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

**What this document is.** A first research pass on question 14 of the Writing sheet, and on no other question. No findings document for question 14 existed before this one. The sheet's other questions are answered in other documents in this folder.

---

## 14. When a render works through a range of frames, how do the writes belonging to one frame stand in time against the writes belonging to the frame after it?

### How I read the question

I read it as: during a single render over a frame range, where do the file writes for frame N sit in time relative to the file writes for frame N+1 — before them, after them, or overlapping them?

The question is slightly ambiguous in one respect, and I flag it rather than choosing silently. "The writes belonging to one frame" could mean (a) only the write of that frame's finished output file, or (b) every file-system touch attributable to that frame, including a placeholder file created for it. I answer for (b), the wider reading, and mark clearly which parts of the answer rest on placeholder files. Under reading (a) the answer is the same but the placeholder paragraphs drop out.

### Answer

**LABEL: mixed — see each part below.**

**Part 1 — the frames are taken from start to end, and the step to the next frame comes after a frame is rendered. (documented)**

The command-line documentation states the order of frames outright, and states that the step to the next frame happens *after* a frame is rendered:

> "-a, --render-anim
>
> Render frames from start to end (inclusive)."

> "-j, --frame-jump <frames>
>
> Set number of frames to step forward after each rendered frame."

Source: manual, Command Line Arguments — https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html — read 2026-07-30.

These words give the direction the frames are taken in and the fact that the step forward comes after a frame is rendered. They do not say that one frame's work cannot overlap the next frame's; see Part 8.

**Part 2 — a frame's write comes after that frame is rendered, not before. (documented)**

> "While rendering stills will allow you to view and save the image from the render buffer when it is complete, animations are a series of images, or frames, and are automatically saved directly out to a drive after being rendered."

Source: manual, Rendering Animations — https://docs.blender.org/manual/en/4.5/render/output/animation.html — read 2026-07-30.

The Python API names an event for exactly that moment, per frame:

> "bpy.app.handlers.render_write
>
> on writing a render frame (directly after the frame is written)"

Source: Python API, `bpy.app.handlers` — https://docs.blender.org/api/4.5/bpy.app.handlers.html — read 2026-07-30.

And the boundary between one frame and the next is itself a named thing in the render-engine interface:

> "render_frame_finish()
>
> Perform finishing operations after all view layers in a frame were rendered"

Source: Python API, `bpy.types.RenderEngine` — https://docs.blender.org/api/4.5/bpy.types.RenderEngine.html — read 2026-07-30.

That last quote is about the interface a render engine implements, and within that it says all view layers in a frame are rendered before the frame's finishing step.

**Part 3 — frames already rendered stay in the output folder while the run goes on. (documented)**

> "For example, when rendering animations to image file formats the render job can be canceled and resumed at the last rendered frame by changing the frame range."

Source: manual, Output Introduction — https://docs.blender.org/manual/en/4.5/render/output/introduction.html — read 2026-07-30.

> "This allows you an easy recovery if there is a problem and you have to re-start the rendering, since the frames you have already rendered will still be in the output directory."

Source: manual, Rendering Animations — https://docs.blender.org/manual/en/4.5/render/output/animation.html — read 2026-07-30.

What the words themselves carry: there is a "last rendered frame", and the frames rendered before it are in the output directory while the run can still be re-started.

**Inferred, and the step written out:** that those earlier files are *complete* rather than half-written, and that the later frames have not been touched at all, is in neither quote. A reader has to supply it. Being in the folder and being finished are different things, and neither sentence says which is meant. So the documented part is the two sentences above; the picture of "earlier frames complete, later frames unwritten" is inference, on the same footing as Part 4.

**Part 4 — putting Parts 1–3 together: frame N's write precedes frame N+1's write. (inferred)**

The step I took: the sources say frames go from start to end (Part 1), that each frame's save happens after that frame is rendered (Part 2), and that the step to the next frame happens after the current one is rendered (Part 1, the `-j` wording). From those three, frame N's write happens before frame N+1's write. The sources never say this in one sentence, so this is inferred, not documented.

**Part 5 — a movie file is a different shape of answer. (documented for the shape, inferred for the ordering)**

If the output format is a video container rather than still images, the frames do not get separate files:

> "Blender creates one file that holds all the frames of your animation."

versus the still-image case on the same page:

> "Blender creates a file for each frame of the animation."

Source: manual, Rendering Animations — https://docs.blender.org/manual/en/4.5/render/output/animation.html — read 2026-07-30.

Two further statements bear on that one file:

> "If your video is huge and exceeds 2GiB, enable Autosplit Output. This will automatically split the output into multiple files after the first file is 2GiB in size."

> "If a problem occurs while rendering, the file might become unplayable and you will have to re-render all frames from the beginning."

Source: manual, Output Properties — https://docs.blender.org/manual/en/4.5/render/output/properties/output.html — read 2026-07-30.

Inferred from those: with a movie container, frame N's write and frame N+1's write are successive additions to the same file rather than two separate files, so "one frame's writes against the next frame's" is a matter of position within one growing file. The step I took: the manual says one file holds all frames and that the file is split once it passes a size while rendering, which requires the file to be built up during the run rather than all at once at the end; it does not say in so many words that each frame is appended as it finishes.

**Part 6 — placeholder files put an extra write for a frame ahead of that frame's real content. (inferred)**

Two settings exist:

> "Overwrite
>
> Overwrite existing files when rendering.
>
> Placeholders
>
> Create empty placeholder frames while rendering."

Source: manual, Output Properties — https://docs.blender.org/manual/en/4.5/render/output/properties/output.html — read 2026-07-30.

The API gives the same setting more precisely:

> "use_placeholder
>
> Create empty placeholder files while rendering frames (similar to Unix 'touch')
>
> Type:
>
> boolean, default False"

and, just above it:

> "use_overwrite
>
> Overwrite existing files while rendering
>
> Type:
>
> boolean, default True"

Source: Python API, `bpy.types.RenderSettings` — https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html — read 2026-07-30.

The manual then shows what the two are for together:

> "An easy way to get multiple machines to share the rendering workload is to:
>
> Set up a shared directory over a network file system.
>
> Disable Overwrite, enable Placeholders in the Render Output panel.
>
> Start as many machines as you wish rendering to that directory."

Source: manual, Output Properties, "Primitive Render Farm" hint — https://docs.blender.org/manual/en/4.5/render/output/properties/output.html — read 2026-07-30.

Inferred, with the step written out: for several machines writing into one directory to divide the work rather than duplicate it, the empty file for a frame must exist before that frame is rendered, so that the other machines see it and skip that frame. So with Placeholders on, the order of writes attributable to frame N is: empty file for frame N created, then frame N rendered, then frame N's real content written; and because frames go in order (Part 4), the empty file for frame N+1 is created after frame N's content write. The manual does not state the timing of the placeholder within a frame; it only says "while rendering". This is the weakest-supported part of my answer.

**Part 7 — the File Output node's writes for a frame. (documented that they are per-frame; the ordering against the frame's main write is not in the sources)**

> "The image(s) will be saved on rendering, writing to the current frame. An entire sequence of images will be saved, when an animation is rendered."

Source: manual, File Output Node — https://docs.blender.org/manual/en/4.5/compositing/types/output/file_output.html — read 2026-07-30.

The manual describes the Post Processing panel's options as acting after the render:

> "The Post Processing panel is used to control different options used to process your image after rendering."

Source: manual, Post Processing — https://docs.blender.org/manual/en/4.5/render/output/properties/post_processing.html — read 2026-07-30.

**Inferred, and the step written out:** the first quote says the node writes for the current frame and writes a whole sequence when an animation is rendered; the second says the panel's options act on the image after the render. Putting the two together — that the node's write for frame N happens at frame N's turn, after that frame is rendered — is a step neither sentence takes on its own.

Whether the node's file for frame N is written before or after the scene's own output file for frame N is not stated on the pages I read. My record below shows how far I looked: I read the File Output Node page and searched no compositing wording at all, so for this one point the silence is a limit of my searching rather than searching that came back empty.

**Part 8 — what the sources do not settle. (not in the sources)**

None of the sources say anything about whether writes for two frames can overlap in time. Specifically, nothing states:

- whether frame N's file is fully written, flushed and closed before any work on frame N+1 begins, or whether saving could still be finishing while the next frame renders;
- whether a file is written under a temporary name and renamed, or written in place under its final name;
- how much time separates one frame's write from the next, or whether the file modification times are guaranteed to come out in frame order;
- whether Blender ever renders or writes more than one frame of a range at the same time within one process;
- the order frames are taken in when a list of frames is given rather than a range. I found no words stating that order. I have also quoted no words describing the list form itself, so nothing here rests on it.

My record of looking for these is below.

### Not covered

This answer settles the order in which frames are taken and the fact that a frame's write follows that frame's render. It does **not** settle:

- whether the two frames' writes are strictly disjoint in time (Part 8) — the strongest thing I can say is that the sources describe a sequence, not that they forbid overlap;
- what a partly written file looks like on disk at any instant;
- ordering between the several files one frame may produce (scene output, File Output node files, and the separate files that multi-view output produces). I found no ordering on the pages I read, and my searches covered no multi-view or compositing wording, so for those two this is a gap in my looking rather than searching that came back empty;
- anything about a render distributed across several machines beyond the "Primitive Render Farm" hint quoted above, which is a suggested way of working, not a description of a mechanism;
- whether the ordering holds when a render is cancelled part-way through a frame.

### 1. Outside the interface

The strongest documented statements in this answer — "Render frames from start to end (inclusive)" and "Set number of frames to step forward after each rendered frame" — come from the Command Line Arguments page, which is documentation of Blender driven from outside its own window. So the frame ordering is documented for exactly that case. The manual's Rendering Animations page and the API handler descriptions do not distinguish between the two ways of driving Blender at all. I found nothing in any source stating that the ordering of frames or the placing of writes differs between clicking in the window and driving Blender from outside it. Beyond that, the sources do not say.

One thing they do say about the outside case, which bears on when writes happen at all: arguments are acted on in the order they are given, and the manual tells the reader to put the render argument last.

> "Arguments are executed in the order they are given!"

> "Always position -f or -a as the last arguments."

Source: manual, Rendering From The Command Line — https://docs.blender.org/manual/en/4.5/advanced/command_line/render.html — read 2026-07-30.

**Inferred, and the step written out:** applying the first sentence to this particular pair — that a render argument placed before the output-path argument would render before the path was set — is a step the manual does not take in so many words. It states the general rule and gives the instruction; the consequence for that pair is the reader's.

**Brought up to date by the hand tests of 2026-08-30.** This is now settled by test rather than left to the sources. The same five-frame render was run in this version on two machines, once with no window at all and once with Blender's own window open, the render started the way the Render Animation menu item starts it — as a job the interface keeps running alongside. **The ordering was the same in all four runs**: for every frame, the write event for frame N came before the render event for frame N+1, and on disk no file was still growing when the next frame's file appeared. See *What the hand tests settled — question 14* at the foot of this document, tests 1 and 13.

### 2. Operating system

The sources do not say. I found nothing in the manual, the API documentation, the release notes or Cycles' own files that makes the ordering of frames, or the timing of a frame's writes, depend on the operating system. The API's description of placeholder files mentions a Unix command only as a comparison ("similar to Unix 'touch'"), not as a platform restriction. The only operating-system-specific matter I met while searching was where temporary files are stored, which is a different subject.

**Brought up to date by the hand tests of 2026-08-30.** Two operating systems are now settled by test: **macOS**, on the Creator's own Mac, and **Linux**, on a rented machine whose Blender matched the Mac's to the exact build hash. The two agreed on every test. **Windows is still not settled** — not by the sources and not by any test — and four Windows entries are written out ready to run in `Audits/Blender Research/Writing/Hand tests — Open Threads questions.md`. A Windows result must name the edition it was taken on. See *What the hand tests settled — question 14* at the foot of this document.

### Record of where I looked

**Searched, whole-tree, across `manual_4.5`, `api_4.5`, `release_notes_4.5` and `cycles_4.5` unless noted:**

Terms searched (case-insensitive): `placeholder`, `overwrite`, `render_write`, `render_post`, `render_pre`, `render_init`, `render_complete`, `render_stats`, `render_frame_finish`, `use_placeholder`, `use_overwrite`, `frame_path`, `one frame at a time`, `frame at a time`, `sequentially`, `one after another`, `in sequence`, `frame by frame`, `frame-by-frame`, `concurrent`, `simultaneous`, `in parallel`, `atomic`, `partially written`, `incomplete file`, `temporary file`, `before the next frame`, `next frame is`, `after each frame`, `each rendered frame`, `after rendering each`, `when the frame is`, `Save Buffers`, `render job`, `background thread`, `written to disk`, `saved to disk`, `directly out to a drive`, `overlap`, `interleav`, `race condition`, `file lock`, `prefetch`, `look ahead`, `asynchronous`, `async`, `queue`, `serial`, `saving`, `render statistics`, `Render Animation`.

**Pages opened in full and read:**

- manual, Rendering Animations — https://docs.blender.org/manual/en/4.5/render/output/animation.html
- manual, Output Introduction — https://docs.blender.org/manual/en/4.5/render/output/introduction.html
- manual, Output Properties (Output, Saving, Image Sequence, Encoding, Tips) — https://docs.blender.org/manual/en/4.5/render/output/properties/output.html
- manual, Frame Range — https://docs.blender.org/manual/en/4.5/render/output/properties/frame_range.html
- manual, Post Processing — https://docs.blender.org/manual/en/4.5/render/output/properties/post_processing.html
- manual, Rendering From The Command Line — https://docs.blender.org/manual/en/4.5/advanced/command_line/render.html
- manual, Command Line Arguments (Render Options and Format Options sections) — https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html
- manual, Render Introduction — https://docs.blender.org/manual/en/4.5/render/introduction.html
- manual, File Output Node — https://docs.blender.org/manual/en/4.5/compositing/types/output/file_output.html
- manual, Topbar (Render menu entries) — https://docs.blender.org/manual/en/4.5/interface/window_system/topbar.html
- manual, Cycles Performance settings (Persistent Data) — https://docs.blender.org/manual/en/4.5/render/cycles/render_settings/performance.html
- Python API, `bpy.app.handlers` (full handler list) — https://docs.blender.org/api/4.5/bpy.app.handlers.html
- Python API, `bpy.types.RenderSettings` (`use_overwrite`, `use_placeholder`, `use_persistent_data`, `frame_path`) — https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html
- Python API, `bpy.types.RenderEngine` (methods section) — https://docs.blender.org/api/4.5/bpy.types.RenderEngine.html
- Python API, `bpy.ops.render` (`render`, `opengl`) — https://docs.blender.org/api/4.5/bpy.ops.render.html
- release notes, Rendering — https://developer.blender.org/docs/release_notes/4.5/rendering/
- release notes, Core — https://developer.blender.org/docs/release_notes/4.5/core/
- release notes, Pipeline, Assets & IO (searched for write/output/frame/save) — https://developer.blender.org/docs/release_notes/4.5/pipeline_assets_io/
- Cycles' own files: `cycles_4.5/engine.py` (read lines 60–140), `cycles_4.5/__init__.py` (searched for render entry points), `cycles_4.5/operators.py` (read lines 60–115), `cycles_4.5/properties.py` (searched for `frame`)

**Ruled out, and why:**

- **`cycles_4.5` generally.** Cycles' Python files hand rendering to a compiled part (`_cycles.render`, `_cycles.render_frame_finish` in `engine.py`, lines 89–100). The frame loop and the file writing are not in the files supplied, so these files cannot settle when a frame is written. The only per-frame loop present in them is in `operators.py` (lines 84–96), and that is a loop over an already-finished image sequence for denoising, not a render.
- **Release notes.** Searched all 4.5 subject files for write, output, frame and save. The only render-output changes noted for 4.5 are pixel-density metadata, alpha for BMP, higher bit depths for the FFV1 codec, and path templates. None touch write ordering.
- **Video Sequencer and encoding pages.** Read the Encoding sections for anything about when a container is written. They describe codecs, rates and the 2GiB autosplit, but nothing about the moment a frame enters the file.
- **Concurrency vocabulary.** The searches for `concurrent`, `in parallel`, `simultaneous`, `asynchronous`, `queue`, `race condition`, `file lock`, `atomic`, `overlap` and `interleav` inside the render, compositing, files and advanced sections returned only unrelated hits (shader compilation, viewport threads, sculpt tools, interleaved 3D display modes, a Grease Pencil modifier mode named "Concurrent"). `atomic`, `partially written`, `incomplete file`, `race condition`, `file lock`, `prefetch`, `look ahead`, `asynchronous`, `async`, `queue` and `serial` returned nothing at all in those sections. That is the basis for Part 8.

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

2026-08-29 — Claude Opus 5 (Claude Code) — Writing (Open Threads), Blender 4.5

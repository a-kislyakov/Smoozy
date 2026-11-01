# Writing — Open Threads questions, second pass (Blender 5.2)

**Sources consulted, and when.** Blender 5.2 manual (`manual_5.2`), Blender 5.2 Python API documentation (`api_5.2`), Blender 5.2 release notes (`release_notes_5.2`), and the Cycles add-on's own files (`cycles_5.2`) — all inside `Blender Sources/`, all taken on **2026-07-30**, which is the date read for every finding below. Searching and reading was done on 2026-08-30.

**I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.**

Answering question 14 only.

---

## 14. When a render works through a range of frames, how do the writes belonging to one frame stand in time against the writes belonging to the frame after it?

### The question as I read it

I take the question to be about ordering: does everything a render writes for one frame happen before anything it writes for the next frame, or can the two overlap? The question is ambiguous in one respect — it could instead be asking how far apart in time the two sets of writes fall (the gap between them). I answer the ordering reading, because that is what "stand in time against" most naturally asks and because the second reading has an obvious answer (roughly one frame's render time, which varies with the scene). Where the sources touch the gap reading, I say so.

### Short answer

The sources document that an animation render moves through the range one frame at a time, and that each frame's output is written as that frame is rendered rather than held back to the end. They do **not** anywhere state whether one frame's writing is finished before the next frame's rendering or writing begins, and they do not say whether any writing is buffered, deferred, or handed to a separate thread. So the ordering question is partly documented and partly **not in the sources**.

I break the answer into the pieces the sources do settle, then the piece they do not.

---

### 14a. The render moves through the range one frame at a time, in ascending order

**Label: documented.**

Quotes:

> "-a, --render-anim
>
> Render frames from start to end (inclusive)."

> "-j, --frame-jump <frames>
>
> Set number of frames to step forward after each rendered frame."

- Source: manual. https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html
- Date read: 2026-07-30

> "Step
>
> Controls the number of frames to advance for each frame rendered, a value above 1 skips frames."

- Source: manual. https://docs.blender.org/manual/en/5.2/render/output/properties/frame_range.html
- Date read: 2026-07-30

The phrases "from start to end", "step forward after each rendered frame" and "advance for each frame rendered" are the sources' own words for a single ascending pass with one frame rendered at a time.

**Not covered:** these lines describe the frame *rendering* order. They say nothing about the writing that follows each frame.

---

### 14b. Each frame's output is written as that frame is rendered, not collected and written at the end

**Label: documented.**

Quotes:

> "While rendering stills will allow you to view and save the image from the render buffer when it is complete, animations are a series of images, or frames, and are typically saved directly to disk as they are rendered."

- Source: manual. https://docs.blender.org/manual/en/5.2/render/output/animation.html
- Date read: 2026-07-30

> "bl_use_image_save
>
> Save images/movie to disk while rendering an animation. Disabling image saving is only supported when bl_use_postprocess is also disabled. (default True)"

- Source: Python API documentation. https://docs.blender.org/api/5.2/bpy.types.RenderEngine.html
- Date read: 2026-07-30

> "The File Output node writes one or more images to disk during rendering or compositing."

- Source: manual. https://docs.blender.org/manual/en/5.2/compositing/types/output/file_output.html
- Date read: 2026-07-30

There is also a per-frame event a script can hang on, which the API describes in terms of the moment a frame's file is finished:

> "bpy.app.handlers.render_write
>
> on writing a render frame (directly after the frame is written). Accepts one argument: the scene data-block being rendered"

and a statistics line that treats saving as its own reported quantity alongside rendering, per frame:

> "bpy.app.handlers.render_stats
>
> on printing render statistics. Accepts one argument: the render progress as a string containing current frame, current sample, render time and saving time."

- Source: Python API documentation. https://docs.blender.org/api/5.2/bpy.app.handlers.html
- Date read: 2026-07-30

**Not covered:** "as they are rendered" and "while rendering an animation" fix that writing is spread across the render rather than pooled at the end. Neither phrase says whether a given frame's write completes before the next frame starts.

---

### 14c. Frames already passed are on disk while later frames are still to come — an interrupted run leaves the earlier frames present and the later ones absent

**Label: documented.**

Quotes:

> "For example, when rendering animations to image file formats the render job can be canceled and resumed at the last rendered frame by changing the frame range."

- Source: manual. https://docs.blender.org/manual/en/5.2/render/output/introduction.html
- Date read: 2026-07-30

> "This allows you an easy recovery if there is a problem and you have to re-start the rendering, since the frames you have already rendered will still be in the output directory.
>
> Just disable the Overwrite option to start rendering where you left off."

- Source: manual. https://docs.blender.org/manual/en/5.2/render/output/animation.html
- Date read: 2026-07-30

> "Restart the rendering from the place (the frame) where any problem occurred."

- Source: manual. https://docs.blender.org/manual/en/5.2/render/output/properties/output.html
- Date read: 2026-07-30

**Inferred step, from these three:** if at any interruption point the frames already rendered are on disk and the rest are not, then a frame's file is written before the following frames are written, in the order the frames are rendered. The reader has to supply the step "already rendered means already written", so this is **inferred**, not documented.

**Not covered:** none of this says whether the *last* frame touched before the interruption is complete on disk or half-written. The manual's "Just disable the Overwrite option" advice works on the presence of a file, and the sources never say that a present file is a finished file.

---

### 14d. With placeholders on, a frame's file appears on disk before that frame's picture is written into it

**Label: documented for what the setting does; inferred for the ordering it implies.**

Quotes:

> "Image Sequence – Overwrite Image Multi-Layer EXR
>
> Create empty placeholder frames while rendering."

and the hint printed directly beneath it:

> "Primitive Render Farm
>
> An easy way to get multiple machines to share the rendering workload is to:
>
> Set up a shared directory over a network file system.
>
> Disable Overwrite, enable Placeholders in the Render Output panel.
>
> Start as many machines as you wish rendering to that directory."

- Source: manual. https://docs.blender.org/manual/en/5.2/render/output/properties/output.html
- Date read: 2026-07-30

> "use_placeholder
>
> Create empty placeholder files while rendering frames (similar to Unix ‘touch’) (default False)"

> "use_overwrite
>
> Overwrite existing files while rendering (default True)"

- Source: Python API documentation. https://docs.blender.org/api/5.2/bpy.types.RenderSettings.html
- Date read: 2026-07-30

**Inferred step:** an empty file "similar to Unix ‘touch’" carries no picture, so for the file to be useful as a claim on a frame it must exist before that frame's picture is written. And for several machines sharing a directory to divide the work, each machine's placeholder must be laid down as it reaches that frame — if a machine touched every frame in the range at the start, the other machines would find the whole range claimed and render nothing. Both steps are mine; the manual states only the setting and the recipe. So: with placeholders on, one frame produces **two** writes at different moments — an empty file first, the picture later — and the next frame's empty file is not created until the run reaches that frame. Label **inferred**.

**Not covered:** the sources never say at exactly which point in a frame's processing the placeholder is created, nor whether the placeholder is created for a frame whose rendering then fails, nor what happens to a placeholder when a render is cancelled between the two writes.

---

### 14e. When the destination is a video container, the frames go into one file that is only usable once the run finishes

**Label: documented for the failure behaviour; inferred for the ordering.**

Quote:

> "It’s not recommended to render directly to a video format in the first instance. If a problem occurs while rendering, the file might become unplayable and you will have to re-render all frames from the beginning."

- Source: manual. https://docs.blender.org/manual/en/5.2/render/output/properties/output.html
- Date read: 2026-07-30

The manual's description of the video media type:

> "Video:
>
> Encodes each frame into a video container."

- Source: manual. https://docs.blender.org/manual/en/5.2/render/output/properties/output.html
- Date read: 2026-07-30

**Inferred step:** if each frame is encoded into a shared container and an interruption leaves that container unplayable, then the frames' writes are additions to a single growing file made as the run proceeds, in frame order, and the whole is only complete at the end of the range. The step from "unplayable after a problem" to "written progressively into one file" is mine. Label **inferred**.

**Not covered:** the sources do not say whether encoding of one frame can run while the next frame is being rendered, and do not say what part of the container is written at the end rather than per frame.

---

### 14f. Every frame in the range produces its own output, in frame order, even when its picture is a repeat

**Label: documented.**

This is the one place in the sources where a per-frame output sequence is actually listed in order. It is about the playblast operator, not a full render.

> "This only renders those frames for which the selected objects have an animation key. The other frames are still written to the output, but will simply repeat the last-rendered frame.
>
> For example, when a six-frame animation is rendered, and the selected objects have a key on frames 3 and 5, the following frames will be output:
>
> The 1st frame is always rendered.
>
> The 1st frame is repeated because there is no key on this frame.
>
> The 3rd frame is rendered.
>
> The 3rd frame is repeated because there is no key on this frame.
>
> The 5th frame is rendered.
>
> The 5th frame is repeated because there is no key on this frame."

- Source: manual. https://docs.blender.org/manual/en/5.2/editors/3dview/viewport_render.html
- Date read: 2026-07-30

The list runs one entry per frame, in ascending frame order, with an output for each.

**Not covered:** this is Render Playblast on Keyframes in the 3D Viewport, and the manual notes viewport rendering "only works for the Workbench and EEVEE render engines. It’s not supported for Cycles." The passage is about which picture each frame gets, and the order of the list is the order of the frames — it does not state that the writes happen in that order in time, though nothing in it suggests otherwise.

---

### 14g. A frame can also write to disk in ways that are not its output file

**Label: documented.**

> "Tile Size
>
> Render high resolution images in tiles of this size, to reduce memory usage of render passes. Tiles are cached to disk while rendering to save memory."

- Source: manual. https://docs.blender.org/manual/en/5.2/render/cycles/render_settings/performance.html
- Date read: 2026-07-30

So "the writes belonging to one frame" is not only the frame's picture: with Cycles, tiles are also written to disk during that frame's render.

**Not covered:** the manual does not say where these cached tiles are written, whether they are removed when the frame finishes, or whether one frame's tile cache can still be on disk while the next frame is rendering.

---

### 14h. What the sources do not settle — the ordering itself

**Label: not in the sources.**

Nowhere in the 5.2 manual, the 5.2 Python API documentation, the 5.2 release notes, or the Cycles add-on files do I find any statement that:

- frame N's writing is finished before frame N+1's rendering begins;
- frame N's writing is finished before frame N+1's writing begins;
- writing is or is not handed to a separate thread, buffered, deferred, or queued;
- one frame's writes and the next frame's writes can or cannot overlap;
- when a frame has several writes (main output, one or more File Output nodes, the several files of a multi-view render), those writes fall in any stated order among themselves.

The closest the sources come is the wording of the `render_write` handler — "directly after the frame is written" — which fixes a moment relative to one frame's own write, and says nothing about the next frame. The `render_stats` string reporting "render time and saving time" separately, per frame, shows that saving is accounted for as its own quantity within a frame's progress report, but the API does not say whether that saving time is time during which nothing else is happening.

The one place the API talks about threads during a render is about handlers, not about writing:

> "While rendering the frame_change_pre and frame_change_post handlers are called from one thread and the viewport updates from a different thread."

- Source: Python API documentation. https://docs.blender.org/api/5.2/bpy.app.handlers.html
- Date read: 2026-07-30

That establishes that more than one thread is live during a render, but it names the viewport, not the writing of files, so it cannot be stretched into an answer about writes.

**Where I looked, and what I ruled out.** See the record below; it is part of this answer, not a footnote.

---

### Not covered (the whole of question 14)

- Whether a frame's file is complete and closed at the moment the next frame's rendering starts. This is the heart of the question and the sources are silent on it.
- Whether writing overlaps rendering at all — that is, whether the "saving time" the statistics report is time the render is stalled.
- The order among the several writes a single frame can make.
- Anything about how long the gap between one frame's writes and the next frame's writes is, beyond the obvious point that a frame's render sits between them.
- Whether an interrupted render leaves the frame it was working on partly written.
- Whether these answers change with the render engine. The frame-range and output wording is engine-neutral in the manual, but the sources never say so explicitly, and the tile-cache write in 14g is Cycles-specific.

### Outside the interface

Everything quoted in 14a comes from the command-line arguments page, which exists only for driving Blender from outside its own window — `-a`/`--render-anim` and `-j`/`--frame-jump` are the documented way to run a range of frames without a person clicking, and their wording ("from start to end", "after each rendered frame") is the same one-frame-at-a-time picture. The manual's page on rendering from the command line adds nothing about write timing beyond argument order:

> "Arguments are executed in the order they are given!"

- Source: manual. https://docs.blender.org/manual/en/5.2/advanced/command_line/render.html
- Date read: 2026-07-30

The output settings quoted in 14b–14e sit in the blend-file and are not described anywhere as behaving differently in background mode. The one difference the sources do state near this area is unrelated to writing:

> "-b, --background
>
> Run in background (often used for UI-less rendering).
>
> The audio device is disabled in background-mode by default and can be re-enabled by passing in -setaudio Default afterwards."

- Source: manual. https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html
- Date read: 2026-07-30

For 14f specifically, the playblast operators are 3D Viewport menu items and the manual describes them only from inside the interface; the sources do not say whether they can be driven from outside.

Beyond that: **the sources do not say** whether the timing of writes differs when Blender is driven from outside its window. Nothing I found distinguishes the two cases, and I am not willing to infer that silence means sameness.

**Brought up to date by the hand tests of 2026-08-30.** This is now settled by test rather than left to the sources. The same five-frame render was run in this version on two machines, once with no window at all and once with Blender's own window open, the render started the way the Render Animation menu item starts it — as a job the interface keeps running alongside. **The ordering was the same in all four runs**: for every frame, the write event for frame N came before the render event for frame N+1, and on disk no file was still growing when the next frame's file appeared. See *What the hand tests settled — question 14* at the foot of this document, tests 1 and 13.

### Operating system

The sources say nothing about this answer varying by operating system. The pages quoted carry no per-system note. Two nearby, documented system distinctions exist but neither touches write ordering: the file-format list on the command-line page notes formats "that can be compiled into Blender, not available on all systems", and `-con, --start-console` is marked "(Windows only)" — both on https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html, read 2026-07-30. The placeholder description's phrase "similar to Unix ‘touch’" is an explanation by analogy in the API text, not a statement that the feature is limited to one system.

So: **the sources do not say** whether this holds on every operating system Blender runs on. There is no per-system statement to report either way.

**Brought up to date by the hand tests of 2026-08-30.** Two operating systems are now settled by test: **macOS**, on the Creator's own Mac, and **Linux**, on a rented machine whose Blender matched the Mac's to the exact build hash. The two agreed on every test. **Windows is still not settled** — not by the sources and not by any test — and four Windows entries are written out ready to run in `Audits/Blender Research/Writing/Hand tests — Open Threads questions.md`. A Windows result must name the edition it was taken on. See *What the hand tests settled — question 14* at the foot of this document.

---

## Record of where I looked

All searching was done with `grep -rniI` over the whole of `manual_5.2`, `api_5.2` and `release_notes_5.2` at once (and `cycles_5.2` where noted), from `Blender Sources/`.

**Terms searched across the whole manual, API and release notes, with the outcome:**

| Term | Outcome |
| --- | --- |
| `placeholder` | Led to the Output panel page and `RenderSettings.use_placeholder`. Used — 14d. |
| `one frame at a time` | Only Grease Pencil drawing pages. Ruled out. |
| `frame by frame` | Only a Grease Pencil animation page. Ruled out. |
| `sequentially`, `sequential` | Freestyle, IK constraints, particle rendering, strip editing, image import, spreadsheet/simulation pages. Nothing about render output. Ruled out. |
| `in parallel`, `simultaneously`, `concurrently`, `concurrent` | Preferences/System, modifiers, sculpting, Cycles thread count ("The maximum number of CPU cores to use simultaneously while rendering"), extension version guidelines. All about threads within one render or unrelated topics; nothing about frames or writes. Ruled out. |
| `asynchronous`, `asynchronously` | Zero hits anywhere. |
| `write order`, `order of writes`, `one after another`, `serially`, `in frame order`, `finished writing`, `completely written`, `fully written`, `partial file`, `pipelined` | Zero hits anywhere. This is the core of the "not in the sources" claim in 14h. |
| `temporary file`, `queue`, `background thread` | *(Corrected by the citation check of 2026-08-30, which re-ran these searches: this row originally said "zero hits anywhere" alongside the terms above, and that was wrong for these three.)* `temporary file` returns nine hits — the two temporary-directory command-line arguments, the crash-recovery note on the splash page, the Temporary Files preference, the pose library's thumbnail, the paths list in the help page, the autosave preference, and a USD hook. `queue` returns ten — boid rules, an event queue on `WindowManager`, an asset-library download ping, index pages, and `multiprocessing.Queue` in the threading gotcha. `background thread` returns two — preview image generation on `bpy.types.ID`, and downloading a file in the threading gotcha. None of the twenty-one hits is about render output writing, so the conclusion in 14h stands; only the record of what came back was wrong. |
| `ascending`, `chronological`, `interleav`, `frame order` | Hits only in the sequencer ("Reverse frame order"), image-sequence naming, geometry nodes, and unrelated pages. None about render writes. Ruled out. |
| `as they are rendered` | One hit — the Rendering Animations page. Used — 14b. |
| `after each frame`, `each frame is`, `before the next`, `until the next` | No hits about render output; `each frame is` returned only the image-editor format description and motion paths. Ruled out. |
| `written to disk`, `saved to disk` | File Output node page (used — 14b), plus fluid baking, downloads, property groups. Rest ruled out. |
| `while rendering`, `during rendering`, `during the render`, `after rendering` | Full sweep of all hits. Productive ones: the placeholder line, `bl_use_image_save`, the File Output node line, the Cycles tile cache line, Lock Interface. Rest were about material/shader settings, EEVEE, view layers and Freestyle. |
| `resume`, `last rendered frame` | Output introduction and Rendering Animations pages. Used — 14c. Fluid-bake pause/resume hits ruled out as unrelated. |
| `flush`, `atomic`, `finalize`, `finalized`, `append`, `appended` | Nothing about frame writes. `appended` hits are all about the frame number appended to a file name, or about linking/appending data-blocks. Ruled out. |
| `Frame Step` | Frame Range panel (used — 14a), plus baking and NLA/graph-editor pages, ruled out. |
| `render job` | Output introduction (used — 14c) and the three job handlers `render_init` / `render_complete` / `render_cancel`, which say only when they fire relative to the whole job, not to individual frames. |
| `saving time` | One hit — `render_stats`. Used — 14b and 14h. |
| `Saving:` | Zero hits. The sources do not document the render progress text itself, only that `render_stats` receives it. |
| `Persistent Data` | Cycles performance page ("Keep render data in memory after rendering for faster re-renders and animation renders") — about memory kept between frames, not about writes. Ruled out. |
| `Lock Interface` | Topbar page and `RenderSettings.use_lock_interface` — about memory and about handler safety, not about writes. Ruled out. |
| `playblast` | Viewport Render page. Used — 14f. |
| `start to end`, `advance for each frame`, `after each rendered frame` | Command-line arguments and Frame Range pages. Used — 14a. |
| `render_write` | `bpy.app.handlers` only. Used — 14b, 14h. |
| `I/O`, `at the same time`, `overlap`, `in the background` | Swept over `manual_5.2/render`, `manual_5.2/advanced`, `manual_5.2/interface`, `manual_5.2/editors`, the handlers page, `RenderSettings`, `RenderEngine` and the release notes. Nothing about frame writes; hits were stereoscopy I/O, debug flags, sequencer cache, and viewport drawing. Ruled out. |

**Pages opened and read in full or in substantial part:**

- https://docs.blender.org/manual/en/5.2/render/output/properties/output.html — the whole Output panel page including Overwrite, Placeholders, Media Type, the Primitive Render Farm hint, the Encoding panel and its tips.
- https://docs.blender.org/manual/en/5.2/render/output/animation.html — Rendering Animations, in full.
- https://docs.blender.org/manual/en/5.2/render/output/introduction.html — in full.
- https://docs.blender.org/manual/en/5.2/render/output/properties/frame_range.html — in full.
- https://docs.blender.org/manual/en/5.2/render/introduction.html — in full; no bearing, ruled out.
- https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html — the Render Options, Cycles, Format and Animation Playback sections.
- https://docs.blender.org/manual/en/5.2/advanced/command_line/render.html — in full.
- https://docs.blender.org/manual/en/5.2/compositing/types/output/file_output.html — in full.
- https://docs.blender.org/manual/en/5.2/editors/3dview/viewport_render.html — in full.
- https://docs.blender.org/manual/en/5.2/interface/window_system/topbar.html — the Render menu section, in full. It lists Render Image, Render Animation and the sequencer render entries with no timing detail; ruled out.
- https://docs.blender.org/manual/en/5.2/files/media/video_formats.html — containers and codecs, read for any statement about progressive writing. None. Ruled out.
- https://docs.blender.org/manual/en/5.2/files/file_paths.html — path templates, read because it states when substitution happens ("at time of use (e.g. while rendering)"). About path text, not write timing. Ruled out.
- https://docs.blender.org/manual/en/5.2/render/cycles/render_settings/performance.html — Threads, Tile Size, Texture Cache, Persistent Data. Used — 14g.
- https://docs.blender.org/api/5.2/bpy.app.handlers.html — in full, including the "Note on Altering Data" section and every render handler.
- https://docs.blender.org/api/5.2/bpy.types.RenderEngine.html — the example, all `bl_use_*` flags, `is_animation`, `render_frame_finish`, `update_stats`, `update_progress`.
- https://docs.blender.org/api/5.2/bpy.types.RenderSettings.html — the `use_overwrite`, `use_placeholder`, `use_file_extension`, `use_lock_interface` entries.
- https://docs.blender.org/api/5.2/bpy.ops.render.html — the `bpy.ops.render.render` signature and every parameter. It is marked "Undocumented, consider contributing." and its parameters say nothing about write timing. Ruled out.

**Release notes.** I read `index.md`, `rendering.md`, `pipeline_io.md`, `compositor.md` in full, and searched `bugfixes.md` and `python_api.md` for writing-related terms. Nothing in the 5.2 notes announces any change to when frames are written, to threading of output, or to the order of an animation render. The nearest entries are:

> "* Saving render output can now be disabled. (blender/blender@342f37566f)"

- https://developer.blender.org/docs/release_notes/5.2/rendering/

> "- OpenEXR: Faster writing of half-precision (FP16) EXRs. (blender/blender!157874)"

- https://developer.blender.org/docs/release_notes/5.2/pipeline_io/

Neither speaks to ordering. Two fixed-bug lines sit near the topic and settle nothing — "Rendering File Output Anamorph with Mulit-Layer EXR cannot save file" and "Compositor - frame slip bug using \{camera_name}\ Vairable in File Output Node when switching cameras" (both at https://developer.blender.org/docs/release_notes/5.2/bugfixes/, read 2026-07-30). I record them because the second is about a frame mismatch in a File Output node's path, which a reader might expect to bear on frame ordering; the note gives no detail that would let it answer anything here.

**Cycles' own files.** I listed the folder and read `engine.py` in full, `operators.py` around the animation operators, and searched `properties.py`, `__init__.py`, `operators.py` and `ui.py` for `write`, `save`, `filepath` and `output`. Cycles' own files contain nothing that writes render output — the only frame-related entry point is:

> `def render_frame_finish(engine):`

- Cycles file `engine.py`, line 95 (the function body calls `_cycles.render_frame_finish(engine.session)`).
- **Observed**, date read 2026-07-30.

Its counterpart in the API documentation describes the frame boundary but not writing:

> "render_frame_finish()
>
> Perform finishing operations after all view layers in a frame were rendered"

- Source: Python API documentation. https://docs.blender.org/api/5.2/bpy.types.RenderEngine.html
- Date read: 2026-07-30

One nearby observation from Cycles' files, recorded because it is about a range of frames but does **not** answer the question: the Denoise Animation operator walks the range in ascending order and requires every frame's file to already exist.

> ```
>             for frame in range(scene.frame_start, scene.frame_end + 1):
>                 scene.render.filepath = in_filepath
>                 filepath = scene.render.frame_path(frame=frame)
>                 in_filepaths.append(filepath)
>
>                 if not os.path.isfile(filepath):
>                     scene.render.filepath = original_filepath
>                     err_msg = tip_("Frame '%s' not found, animation must be complete") % filepath
> ```

- Cycles file `operators.py`, lines 78–86, inside `CYCLES_OT_denoise_animation.execute`.
- **Observed**, date read 2026-07-30.

This is an operator run after a render, not the render's own writing loop. It shows only that this operator treats the range as start to end inclusive and expects one file per frame. It settles nothing about when those files were written relative to one another.

**Pictures.** I checked the manual's image references on the pages above — `_images/render_output_properties_output_panel.png` (Output panel) and `_images/render_output_properties_frame_range_panel.png` (Frame Range panel). Both are screenshots of settings panels. A screenshot of a panel can show what options exist, but no screenshot in these sources can show the order of events in time, so I did not take any claim from them. I mention them so it is clear they were considered and set aside for that reason, not overlooked.

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

2026-08-30 — Claude Opus 5 (Claude Code) — Writing (Open Threads), Blender 5.2

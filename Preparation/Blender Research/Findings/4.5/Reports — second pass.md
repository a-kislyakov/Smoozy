# Reports — Blender 4.5 — second pass (question 3 only)

**Sources consulted, and when.** Blender 4.5 manual, Blender 4.5 Python API documentation, Blender 4.5 release notes, and the Cycles 4.5 files supplied — all as the plain files in `Blender Sources/`, taken on **2026-07-30**, which is the date read for every finding below. One picture from the manual was opened and is cited where used. Work done 2026-08-05.

**I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.**

I answered question 3 only, as instructed. I did not open the existing findings document for this version. I ran no helper agents. A Blender tool-server offering a live Blender connection and a manual search of its own was available in this session; **I did not use it, or any tool of it**, for any part of this answer — everything below comes from the frozen files in `Blender Sources/` and nothing else. I did not consult any other version's sources, the web, or my own memory of Blender.

---

## 3. What does Blender report about how far through a piece of work it has got?

**How I read the question.** I read "a piece of work" as a task Blender is running — a render, a bake, a simulation — and "report" as anything Blender shows on screen or hands to a script that says how far along that task is. This was ambiguous. The narrower reading is "a render, and nothing else"; I have marked below which parts survive that narrower reading (3a, 3b, 3d, 3f, 3h, 3i) and which do not (the fluid and point-cache bake statements in 3c).

**LABEL: mixed — documented for the parts listed below, with the inferred steps written out where I took them, and "not in the sources" for the numbers a render actually shows. Each claim carries its own label.**

### 3a. In Blender's own window: one progress item in the Status Bar — documented

The manual gives the Status Bar a middle section for running work, and one named item in it.

Exact words:

> The middle of the Status Bar displays information about in-progress operations.

and

> Running Task
>
> Shows the progress of the currently running task (such as rendering or baking).
> Hovering the mouse pointer over the progress bar will display a time estimate.
> The task can be aborted by clicking the cancel button ().

(The empty brackets at the end are where an icon sat in the original page.)

Source: the manual — https://docs.blender.org/manual/en/4.5/interface/window_system/status_bar.html
Date read: 2026-07-30

So: what Blender shows is a single progress bar for one running task, described as covering rendering and baking among other things, and it can be cancelled from there.

### 3b. What the progress bar carries, from the manual's own picture — documented

The manual places a picture immediately after the sentence about in-progress operations. The picture is `_images/interface_window-system_status-bar_ui-center.png`, referenced on the Status Bar page above. I opened it.

What is visible in it: a small icon on the left; the words **Sequence Render**; then a bar, partly filled, with **94%** written inside it; then a square button carrying an **X**.

So the running-task item shows, at least in that picture: a name for the task, a partly filled bar, a figure in percent written inside the bar, and a button to stop it. That matches the wording quoted in 3a about a cancel button.

Source: the manual, picture on https://docs.blender.org/manual/en/4.5/interface/window_system/status_bar.html
Date read: 2026-07-30

**Caveat, labelled inferred.** The full-bar picture on the same page, `_images/interface_window-system_status-bar_ui.png`, has "v2.80.74" printed in its right-hand corner. I read that as evidence that the Status Bar screenshots on this page were taken from an older Blender and carried forward, so the exact look of the bar in 4.5 is not settled by the picture. The step: the version string is visible in one picture on the page, and I am extending "this screenshot is old" to the neighbouring screenshots on the same page, which the manual does not state.

### 3c. Other places the sources say progress appears — documented

Fluid simulation baking. Six separate pages carry the same sentence, in two variants:

> The progress will be displayed in the status bar. Pressing Esc will pause the simulation.

on
https://docs.blender.org/manual/en/4.5/physics/fluid/type/domain/settings.html ,
https://docs.blender.org/manual/en/4.5/physics/fluid/type/domain/guides.html ,
https://docs.blender.org/manual/en/4.5/physics/fluid/type/domain/gas/noise.html ,
https://docs.blender.org/manual/en/4.5/physics/fluid/type/domain/liquid/particles.html

and

> The progress will be displayed in the status bar. Pressing Esc will abort the simulation.

on
https://docs.blender.org/manual/en/4.5/physics/fluid/type/domain/cache.html ,
https://docs.blender.org/manual/en/4.5/physics/fluid/type/domain/liquid/mesh.html

Date read: 2026-07-30

Point-cache baking. A different place entirely — the mouse cursor:

> Blender will become unresponsive during most baking operations.
> The cursor will display as a number representing the progress of the baking.

Source: the manual — https://docs.blender.org/manual/en/4.5/physics/baking.html
Date read: 2026-07-30

These two are the only progress readouts the manual describes anywhere outside the Status Bar page. I found no third.

### 3d. What the Python API offers, for a render engine to report progress — documented

A render engine hands Blender a single fraction, and separately two strings.

> update_progress(progress)
>
> Update progress percentage of render
>
> Parameters:
>
> progress (float in [0, 1]) – Percentage of render that's done

> update_stats(stats, info)
>
> Update and signal to redraw render status text
>
> Parameters:
>
> stats (string, (never None)) – Stats
>
> info (string, (never None)) – Info

Source: the API documentation — https://docs.blender.org/api/4.5/bpy.types.RenderEngine.html
Date read: 2026-07-30

Two things to be exact about. First, the progress figure is one number between 0 and 1 for the whole render — the documentation attaches no unit to it, and names no frame and no sample. Second, `update_stats` takes free text; the documentation does not say what a render engine should put in either string, and does not say where the "render status text" is shown.

The same page also has, for showing which part of the image is being worked on:

> tile_highlight_set(x, y, width, height, highlight)
>
> Set highlighted state of the given tile

and a property with no description at all, only a type:

> use_highlight_tiles
>
> Type:
>
> boolean, default False

Same page and date. This says which region is lit up, not how far through the work is.

The Hydra render engine page repeats `RenderEngine.update_stats`, `RenderEngine.update_progress` and `RenderEngine.update_memory_stats` in its inherited-member list, at https://docs.blender.org/api/4.5/bpy.types.HydraRenderEngine.html — so those are available there too.

### 3e. What the Python API offers, for anything else to report progress — documented

For a script or operator, not a render engine:

> progress_begin(min, max)
>
> Start progress report
>
> Parameters:
>
> min (float in [-inf, inf]) – min, any value in range [0,9999]
>
> max (float in [-inf, inf]) – max, any value in range [min+1,9998]
>
> progress_update(value)
>
> Update the progress feedback
>
> Parameters:
>
> value (float in [-inf, inf]) – value, Any value between min and max as set in progress_begin()
>
> progress_end()
>
> Terminate progress report

Source: the API documentation — https://docs.blender.org/api/4.5/bpy.types.WindowManager.html
Date read: 2026-07-30

Note this one is not a 0-to-1 fraction — the caller sets its own low and high mark, then reports a value between them.

And for drawing a progress readout into a panel:

> progress(*, text='', text_ctxt='', translate=True, factor=0.0, type='BAR')
>
> Progress indicator

with

> factor (float in [0, 1], (optional)) – Factor, Amount of progress from 0.0f to 1.0f

and

> type (enum in ['BAR', 'RING'], (optional)) – Type, The type of progress indicator

Source: the API documentation — https://docs.blender.org/api/4.5/bpy.types.UILayout.html
Date read: 2026-07-30

So there are two shapes of progress indicator Blender can draw, a bar and a ring, and a text can be put on it.

### 3f. What a script can be told about a render in flight — documented

There is a callback list that fires when render statistics are printed:

> bpy.app.handlers.render_stats
>
> on printing render statistics. Accepts one argument: the render stats (render/saving time plus in background mode frame/used [peak] memory).

Source: the API documentation — https://docs.blender.org/api/4.5/bpy.app.handlers.html
Date read: 2026-07-30

This is the only sentence in any of the four sources that says what is actually inside Blender's render statistics. Read plainly, it says: the argument is the render stats, they carry render and saving time, and when Blender is in background mode they also carry the frame and the used (peak) memory. That "frame" is the only mention I found anywhere of a frame number forming part of a progress report.

**This sentence is ambiguous and I am flagging it rather than choosing silently.** My reading is the one above. The other reading available is that "in background mode" governs the whole parenthesis — that the handler carries anything at all only in background mode. Nothing else on the page settles it.

The same page also lists, without further detail:

> bpy.app.handlers.render_pre
>
> on render (before)

> bpy.app.handlers.render_post
>
> on render (after)

> bpy.app.handlers.render_write
>
> on writing a render frame (directly after the frame is written)

Same page and date. These fire around each piece of work, but the page does not say they carry any progress figure.

### 3g. What a script can ask about running work — documented, with one inferred step

> static bpy.app.is_job_running(job_type)
>
> Check whether a job of the given type is running.

with

> Returns:
>
> Whether a job of the given type is currently running.
>
> Return type:
>
> bool.

Source: the API documentation — https://docs.blender.org/api/4.5/bpy.app.html
Date read: 2026-07-30

The job types it accepts are listed as:

> RENDER:
>
> Regular rendering.
>
> RENDER_PREVIEW:
>
> Rendering previews.
>
> OBJECT_BAKE:
>
> Object Baking.
>
> COMPOSITE:
>
> Compositing.
>
> SHADER_COMPILATION:
>
> Shader compilation.

Source: the API documentation — https://docs.blender.org/api/4.5/bpy_types_enum_items/wm_job_type_items.html
Date read: 2026-07-30

**Inferred:** this tells you only whether a job is running, never how far through it is. The step: the documented return type is `bool`, and a true/false value cannot carry a fraction or a count. The documentation does not say this in so many words.

### 3h. On the command line — documented that the switches exist, not what they print

Two switches in the manual touch statistics and timing:

> --cycles-print-stats
>
> Log statistics about render memory and time usage.

and

> --debug-jobs
>
> Enable time profiling for background jobs.

Source: the manual — https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html
Date read: 2026-07-30

Neither entry says what these actually write, in what shape, or how often. Both are about memory, time and profiling rather than a stated position within the work.

### 3i. Cycles' own files — observed, and it settles less than it looks like it does

I searched every file supplied in the Cycles folder for progress reporting. The word "progress" appears four times, and only one of those is about reporting:

> `        # TODO: support cancel and progress reports.`

File: `cycles_4.5/operators.py`, line 102, inside the denoise-animation operator, immediately before it calls `_cycles.denoise(...)`.
Label: **observed** (exact line, from a supplied file that is not documentation).
Date read: 2026-07-30

**Inferred from it:** the bundled Cycles denoise-animation operator does not report progress while it runs. The step: the comment states an intention to add progress reports, and I am reading "still to be supported" as "not supported now". The file does not say that outright.

The other three hits are unrelated to reporting — `MNEE_MIN_PROGRESS_DISTANCE` and a comment about a Newton solve "not making progress" in `cycles_4.5/source/kernel/integrator/mnee.h` (lines 51, 530, 533), and a citation of a paper title containing the word "Progressive" in `cycles_4.5/source/kernel/sample/pattern.h` (line 179).

**This is the honest limit of what these files can say, and it matters.** The Cycles folder holds only the Python add-on layer (nine `.py` files), the GPU kernel and utility headers, the OSL shader objects, and the licences. There is no session, render-scheduling or progress code in it — the `.cpp` and `.cu` files present all sit under `source/kernel/device/`. So the fact that I found no call to `update_progress` or `update_stats` anywhere in these files **does not** mean Cycles never makes those calls. It means the part of Cycles that would make them was not supplied. I draw no conclusion about Cycles' progress reporting from this folder.

### 3j. What is not in the sources

**LABEL for this part: not in the sources.**

After the searches recorded below, none of the four sources states:

- **The numbers a render actually shows while it runs.** No page gives a sample count, a "sample X of Y", a "frame X of Y", a tile count, a percentage figure specific to rendering, or any other running figure for a render. The single documented rendering progress figure is the abstract 0-to-1 one a render engine passes to `update_progress`, and that has no unit attached.
- **Where the render status text goes.** `update_stats` is documented as updating "render status text", but no page says what region, editor or window that text appears in, nor what Blender's own engines put in it.
- **What Blender writes as a render proceeds, outside its own window.** Beyond the one `render_stats` handler sentence quoted in 3f, no page describes any console, terminal or log line reporting position within a render or an animation.
- **Any progress readout in the Image Editor or the render window.** The Image Editor page describes its header, Render Result and slots and says nothing about render progress; the Render menu entries and the "Render In" preference describe where the render appears, not what it reports.
- **Any change to progress reporting in 4.5.** The release notes' Status Bar section lists spacing, text-entry display, a VSE operator, jiggle, proportional editing and key names — nothing about task progress. The Rendering and Cycles notes cover output metadata, lights, shading, subdivision, bump, custom cameras and GPU support — nothing about progress.

**Not covered.** This answer does not settle: what figures Blender's own engines put into the status bar or the status text while rendering; whether the status bar's percentage during a render counts samples, frames, tiles or something else; what the "render stats" string looks like, character for character, in either mode; whether the Status Bar picture reflects 4.5's appearance; whether Cycles calls `update_progress` at all, and how often (the supplied Cycles files cannot answer this — see 3i); and whether anything at all is written for animation renders between frames. It also does not settle the second reading of the `render_stats` sentence flagged in 3f.

### 3k. Observed, 2026-08-06 — what a running Blender 4.5 actually reports, on two machines

This section was added after the source reading, from tests run by hand. It is not part of the reading above and does not change any label there.

**On the Creator's own MacBook Pro** (Apple M1 Pro, macOS 26.5.2) on 2026-08-05, and **on a rented Linux machine** — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH — on 2026-08-06. Both were run with no window (`-b`) on a Cycles scene at 16 samples, 160 by 120 pixels, and both agreed. The rented machine is a shared container, so nothing here says how long anything took.

**The numbers a render actually shows — 3j's first gap, now filled.** Blender 4.5 prints a running line per change of stage, each carrying the frame number, two memory figures, an elapsed time, sometimes a remaining time, the scene and view layer, and then a stage name or a sample count. From the Linux run, word for word:

```
Fra:1 Mem:9.89M (Peak 9.89M) | Time:00:00.00 | Remaining:00:00.03 | Mem:1.48M, Peak:1.48M | Scene, ViewLayer | Sample 1/16
Fra:1 Mem:10.47M (Peak 11.06M) | Time:00:00.11 | Mem:1.48M, Peak:1.48M | Scene, ViewLayer | Sample 16/16
Fra:1 Mem:10.47M (Peak 11.06M) | Time:00:00.11 | Mem:1.48M, Peak:1.48M | Scene, ViewLayer | Finished
```

So there is a "Sample X of Y", and a frame number, and a time estimate — none of which any source states.

**What the render-statistics string is, character for character — 3f's ambiguity, now settled.** A handler attached to `bpy.app.handlers.render_stats` printed its argument with `repr`. With no window it received the whole printed line, frame number included, as one string plus a second value that printed as `None`:

```
RENDER_STATS: ('Fra:2 Mem:9.89M (Peak 11.06M) | Time:00:00.00 | Remaining:00:00.03 | Mem:1.48M, Peak:1.48M | Scene, ViewLayer | Sample 1/16', None)
```

It fired 128 times over three frames on Linux and 127 on the Mac.

**Whether anything is written between frames — 3j's last gap.** In a three-frame animation, each frame ran its own sequence and there was **no** line covering the range. Searching the whole Linux run for `Rendering animation` and `Rendering frame` matched nothing at all in this version.

**What `--cycles-print-stats` prints — 3h's gap.** A block headed `Render statistics:`, holding mesh and image memory totals, a kernel timing table, and per-shader and per-object costs. It arrived on **standard output** on both machines, although the switch's own help text in Cycles' file says stderr. In this version the block sits between the `Finished` line and the `Saved:` line.

**Whether the output can be read while the render runs.** On the Linux machine a two-frame render was piped into a reader that stamped each line with the seconds since the run began. The lines arrived spread across the run: the line Blender stamped `Time:00:06.00` reached the reader at 6.30 seconds. So a program watching Blender's output does see progress before the end.

**What could not be reached.** Whether Cycles calls `update_progress` at all, and how often — 3i says plainly that the supplied Cycles files cannot answer it, and no test here can see inside the call either. The Status Bar picture question in 3b was answered by the Creator's own screenshots on the Mac, recorded in `Preparation/Blender Research/Findings/4.5/Reports.md`.

**Outside the interface.** One documented statement reaches outside: the `render_stats` handler note says the render stats carry the frame and used (peak) memory **in background mode** (quoted in full in 3f) — read plainly, that means what the stats contain differs when Blender runs without its window, and the frame number appears there. Also documented, and relevant: `bpy.app.background` is "Boolean, True when blender is running without a user interface (started with -b)" (https://docs.blender.org/api/4.5/bpy.app.html, read 2026-07-30), so a script can tell which case it is in. Everything else in 3a–3c is **inferred** not to apply outside the window: the Status Bar is documented as "located at the bottom of the Blender window", the baking readout is documented as the mouse cursor, and `UILayout.progress` draws into a panel — none of those exist when Blender runs with no user interface. That is my step, not the documentation's; no page states it. For the Python hooks in 3d, 3e and 3g the sources say nothing either way about whether they behave differently outside the interface. The two command-line switches in 3h are the only progress-adjacent things the manual describes for command-line use, and it does not say what they print.

**Operating system.** The sources do not say. Nothing on any page I read ties the Status Bar progress item, the baking cursor, the fluid-bake status-bar sentence, or any of the Python progress calls to a particular operating system, and nothing names an operating system where they differ. The nearest OS-specific note I found anywhere near this subject is the manual's Windows launch page telling you how to bring the console back (Window ‣ Toggle System Console), and that page says nothing about progress.

The tests in 3k were run on **both** machines the project has — the Creator's MacBook Pro (Apple M1 Pro, macOS 26.5.2) and the rented Linux machine — and they agreed on the printed line shape, the fields, the statistics string and the counts. So what a running Blender 4.5 reports holds on macOS and on Linux. **Windows is untested and no Windows machine has run it.** **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

---

## Where I looked, for question 3

Everything below was run against the four supplied folders only: `manual_4.5`, `api_4.5`, `release_notes_4.5`, `cycles_4.5`. I searched the whole of each before opening anything.

**Searches run across all four sources.** progress; progress bar; progress of; render progress; progress is; progress indicator; progress report; progress feedback; percent; percentage; percentage complete; percent complete; ETA; elapsed; time remaining; remaining time; estimated time; time estimate; status bar; status text; render status; statistics; stats; update_stats; update_progress; update_memory_stats; job; render job; Running Task; how far; so far along; amount of work; work done; frames completed; frames remaining; counter shows; wait cursor; busy cursor; spinner; sample count; samples rendered; current sample; "Sample N/M" and "Sample 1/" patterns; "Rendered … Tiles"; "Path Tracing Sample"; "Frame N of M"; "N of M frames"; "Samples:"; Fra:; Mem:; Time:; during the render; while rendering; during rendering; as it renders; render is in progress; loading render kernels; updating scene; synchronizing; waiting for; initializing; "Rendering…" with ellipsis; Compositing…; Saving…; highlight; shader compilation; compiling; prefetch; background mode; render time; time it took; System Console.

**Manual pages opened and read.** Status Bar (whole page); Window System introduction; Topbar (Render menu and Window menu); Preferences ▸ Interface (Temporary Editors, "Render In", and the Status Bar preferences section); 3D Viewport Overlays (the Statistics section, to check it was scene counts and not progress); Image Editor Introduction (header, Render Result, slots); Info Editor (whole page); Render Introduction; Render Output Introduction; Rendering Animations; Render Output Properties ▸ Output (the System Console note); Cycles Sampling; Command Line Arguments (render options, Cycles options, logging options, debug options); Rendering From The Command Line; Physics Baking; Fluid domain Settings, Cache, Guides, Gas Noise, Liquid Mesh, Liquid Particles.

**Manual pictures opened.** `_images/interface_window-system_status-bar_ui-center.png` (the running-task progress bar) and `_images/interface_window-system_status-bar_ui.png` (the whole bar). I also checked the `_images` folder by name for any picture of a render window or a render in progress and found none — the only match on "render" plus "window" was a stereoscopy illustration.

**API pages opened and read.** `bpy.types.RenderEngine` (whole member list, then the `update_stats`, `update_progress`, `update_memory_stats`, `report`, `error_set`, `tile_highlight_set`, `tile_highlight_clear_all` and `use_highlight_tiles` entries); `bpy.types.WindowManager` (`progress_begin`, `progress_update`, `progress_end`, `is_interface_locked`); `bpy.types.UILayout` (`progress`); `bpy.app.handlers` (whole render handler group, plus the page's opening notes); `bpy.app` (`is_job_running`, `background`); `bpy_types_enum_items/wm_job_type_items`; `bpy.types.HydraRenderEngine` (inherited member list); `bpy.ops.render` (whole page — no progress-related parameter or note on any operator, including `render.render`); `bpy.types.RenderSettings` (searched for progress; only `use_lock_interface` matched anything nearby); `info_gotchas_internal_data_and_python_objects` (the "Can I redraw during script execution?" section); `info_tips_and_tricks` (the background-mode section). I also searched all `genindex` pages for every entry containing "progress", which returned exactly six: `NDOFMotionEventData.progress`, `ThemeUserInterface.wcol_progress`, `UILayout.progress()`, `WindowManager.progress_begin/_end/_update`, `BlendTexture.progression`, and `RenderEngine.update_progress()`. The first two and `progression` are unrelated to task progress (a motion-sensor value, a theme colour set, and a texture setting). That index sweep is what makes me confident there is no seventh progress call hiding in the API.

**Release notes.** Searched every subject file for progress, status bar, percent, stats, remaining, estimate, compilation. Read the Status Bar section of `user_interface.md`, the status-bar items in `sequencer.md`, the whole of `rendering.md`, and the head of `cycles.md`. Nothing in 4.5's notes changes or describes task progress reporting.

**Cycles files.** Searched every supplied file for progress, update_stats, update_progress, set_status, and the literal strings "Rendering", "Rendered ", "Sample ", "Loading render kernels", "Updating Scene", "Synchronizing". Took an inventory of the folder by file type to establish what is and is not present (270 `.h`, 104 `.oso`, 9 `.py`, 6 licence `.txt`, 6 `.cu`, 2 `.cpp`, 1 `.metal`), and listed every `.cpp`/`.cu`/`.metal` to confirm they all sit under `source/kernel/device/`. Read `operators.py` around line 102 and the three unrelated hits in `mnee.h` and `pattern.h`.

**Ruled out, and why.** Compositing, motion tracking, video sequencing and Grease Pencil pages: searched, and their only "progress" hits are the unrelated senses (progressive video, a clip progressing, work-in-progress notices, mip-maps described as progressively lower resolution). Modifier and constraint pages: their percentage hits are all settings values, not progress readouts. The Spreadsheet editor has a thing it calls a status bar showing row and column counts — a count of data, not of work done — so I set it aside. The `requirements/` folder was not consulted: it is not version-specific and describes hardware and driver support, not reporting.

---

## Things a person with Blender 4.5 open could settle by hand in a few minutes

Most of what was on this list has been run. Items 1, 2, 3, 4 and 9 were run by the Creator with Blender's window open on their own MacBook Pro on 2026-08-05, and the results are in `Preparation/Blender Research/Findings/4.5/Reports.md`. Items 5, 6 and 10 were run from a terminal on both machines and are recorded in 3k above. What is left:

1. **The baking cursor.** Add a cloth or particle system, open Physics Properties, press Bake, and look at the mouse cursor while it works. Note whether a number really appears there, and whether it is a percentage or a frame count.

2. **The fluid bake progress.** Add a fluid domain, press Bake Data, and read the Status Bar. Note the task name shown to the left of the bar and whether the figure is a percentage or a frame count.

Both are about baking rather than rendering, which is why neither has been reached: every test run so far has been a render.

---

2026-08-05 — Opus 5 (Claude Code) — Reports, Blender 4.5

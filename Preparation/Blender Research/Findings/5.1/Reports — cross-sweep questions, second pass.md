# Reports — cross-sweep questions, second pass (Blender 5.1)

**Sources consulted, and when.** Blender 5.1 manual (`Blender Sources/manual_5.1`, including its `_images` folder), Blender 5.1 Python API documentation (`Blender Sources/api_5.1`), Blender 5.1 release notes (`Blender Sources/release_notes_5.1`), and Cycles' own files as shipped with Blender 5.1 (`Blender Sources/cycles_5.1`). All four were taken on **2026-07-30**, and that is the date read for everything quoted below. The searching and reading were done on **2026-08-11**.

**I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.** (Two things to declare, for completeness. First, I checked that the folder I was told to save into exists, with a command that prints only the folder's own name. Second, after saving this file I ran a listing in that folder to confirm my own file name had been written exactly as instructed, and that listing printed the names of the neighbouring files as well — I saw a few of their titles, which name other question sets. I did not open any of them, and none of their contents were shown to me. No answer above was written after that listing.)

Only question **7** was assigned to me. It is the only one answered.

---

## 7. What does a render report about its own progress, across the whole range of picture sizes a render will accept?

**How I read the question.** It was ambiguous, and I say so plainly. I answered this reading: *what does a render tell you about how far along it is, and does that reporting change anywhere across the full span of image sizes Blender will let you render?* The other reading would be: *what, size by size, is reported at each particular picture size* — a table of sizes against reports. The sources contain nothing that would fill such a table, so on that reading the answer would be "not in the sources" for the whole of it. I answer the first reading, and it ends in the same place for the size-dependent part.

### 7a. What sizes a render will accept

**Label: documented.**

Exact words, from the Python API documentation:

> "resolution_x
>
> Number of horizontal pixels in the rendered image (in [4, 65536], default 1920)"

> "resolution_y
>
> Number of vertical pixels in the rendered image (in [4, 65536], default 1080)"

> "resolution_percentage
>
> Percentage scale for render resolution (in [1, 32767], default 100)"

Source: Python API documentation, https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html — read 2026-07-30.

The manual's own page for the same three settings gives no numbers:

> "Resolution X, Y
>
> The number of pixels horizontally and vertically in the image."

> "Percentage
>
> Slider to reduce or increase the size of the rendered image relative to the Resolution values.
> This is useful for small test renders that have the same proportions as the final image."

Source: manual, https://docs.blender.org/manual/en/5.1/render/output/properties/format.html — read 2026-07-30.

**Label: inferred** for what follows from those three numbers. The size a render actually works on is the resolution scaled by the percentage and cut down to whole pixels. That formula is written out in the API documentation's own worked example of a render engine:

> "scale = scene.render.resolution_percentage / 100.0
> self.size_x = int(scene.render.resolution_x * scale)
> self.size_y = int(scene.render.resolution_y * scale)"

Source: Python API documentation, https://docs.blender.org/api/5.1/bpy.types.RenderEngine.html — read 2026-07-30.

The step I am taking: combine that formula with the two documented ranges. Doing so, the picture a render is asked for is not confined to 4–65536 pixels a side. At the bottom, 4 pixels at 1 percent gives `int(0.04)` = 0 pixels. At the top, 65536 pixels at 32767 percent gives 21,478,932 pixels. So the span of *asked-for* sizes runs from zero pixels to roughly twenty-one million pixels a side. Nothing in any source says a render succeeds anywhere near either end; the arithmetic is all I am claiming.

### 7b. What a render reports about its progress

**Label: documented.**

The manual describes one progress report, in the status bar:

> "The middle of the Status Bar displays information about in-progress operations."

> "Running Task
>
> Shows the progress of the currently running task (such as rendering or baking).
> Hovering the mouse pointer over the progress bar will display a time estimate.
> The task can be aborted by clicking the cancel button ()."

Source: manual, https://docs.blender.org/manual/en/5.1/interface/window_system/status_bar.html — read 2026-07-30.

**Label: documented**, from the picture on that same page. The manual points at `[image: _images/interface_window-system_status-bar_ui-center.png]`, and I opened it. It shows a single horizontal bar with a small camera icon at its left, the words "Sequence Render" beside the icon, a filled blue bar with **94%** written on it, and an **X** button at the right end. What is visible is therefore: a task name, a completion figure given as a whole-number percentage, a filled bar, and a cancel control. The picture shows a sequence render rather than a Cycles or EEVEE render; I do not read it as showing what a Cycles or EEVEE render displays.

**Label: documented**, from the Python API, for the machinery a render engine uses to report progress:

> "update_stats(stats, info)
>
> Update and signal to redraw render status text
>
> Parameters:
>
> stats (str) – Stats, (never None)
>
> info (str) – Info, (never None)"

> "update_progress(progress)
>
> Update progress percentage of render
>
> Parameters:
>
> progress (float) – Percentage of render that's done (in [0, 1])"

Source: Python API documentation, https://docs.blender.org/api/5.1/bpy.types.RenderEngine.html — read 2026-07-30.

So the progress figure itself is a single fraction between 0 and 1 covering the render, and everything else is free text the engine supplies. The documentation does not say what any engine puts in that text.

One further report, after the fact rather than during:

> "bpy.app.handlers.render_stats
>
> on printing render statistics. Accepts one argument: the render stats (render/saving time plus in background mode frame/used [peak] memory)."

Source: Python API documentation, https://docs.blender.org/api/5.1/bpy.app.handlers.html — read 2026-07-30.

### 7c. Whether any of that changes with picture size

**Label: not in the sources.** This is the heart of the question, and I could not find it answered anywhere.

Nothing in the 5.1 manual, the 5.1 Python API documentation, or the 5.1 release notes connects the progress report — its figure, its units, its wording, or how often it arrives — to how large the image is. The progress figure is documented as one fraction of the whole render at every size, with nothing said about it being counted per tile, per strip, per band, per pass, or per anything else that would come and go as the image grows.

What the sources *do* tie to picture size is one mechanism, and only the mechanism, never its effect on reporting:

**Label: documented.** The manual, on Cycles' Tile Size:

> "Tile Size
>
> Render high resolution images in tiles of this size, to reduce memory usage.
> Tiles are cached to disk while rendering to save memory."

Source: manual, https://docs.blender.org/manual/en/5.1/render/cycles/render_settings/performance.html — read 2026-07-30.

**Label: observed**, from Cycles' own files. In `Blender Sources/cycles_5.1/properties.py`, lines 1051–1062, the two tile settings are declared:

> `    use_auto_tile: BoolProperty(`
> `        name="Auto Tile",`
> `        description="Deprecated, tiling is always enabled",`
> `        default=True,`
> `    )`
> `    tile_size: IntProperty(`
> `        name="Tile Size",`
> `        default=2048,`
> `        description="Render high resolution images in tiles of this size, to reduce memory usage. Tiles are cached to disk while rendering to save memory",`
> `        min=8,`
> `        max=8192,`
> `    )`

So the tile size is a whole number from 8 to 8192 with a default of 2048, and the switch that used to turn tiling on and off is marked deprecated with the words "tiling is always enabled".

**Label: observed**, from `Blender Sources/cycles_5.1/ui.py`, lines 389–405. This is the add-on working out, from the picture size, whether a render will be split into more than one tile — it does so in order to decide whether to show a warning next to the path guiding settings:

> `        if cscene.use_guiding:`
> `            # Calculation matches TileManager::compute_render_tile_size and`
> `            # Session::get_effective_tile_size`
> `            if cscene.tile_size < 128:`
> `                tile_size = cscene.tile_size`
> `            else:`
> `                tile_size = (cscene.tile_size + 128 - 1) & ~(128 - 1)`
> `                tile_size = min(tile_size, 8192)`
> `            tile_area = tile_size ** 2`
> ``
> `            render_scale = scene.render.resolution_percentage / 100.0`
> `            render_size_x = int(scene.render.resolution_x * render_scale)`
> `            render_size_y = int(scene.render.resolution_y * render_scale)`
> `            render_area = render_size_x * render_size_y`
> ``
> `            if render_area > tile_area and render_size_x <= 8192 and render_size_y <= 8192:`
> `                layout.label(text="May work poorly with render tiling", icon='INFO')`

What is solid here is what the file says: this code compares the area of the picture against the area of one tile, and the comment claims the sum matches two things inside the compiled part of Cycles, named `TileManager::compute_render_tile_size` and `Session::get_effective_tile_size`. **Label: inferred**, and the step is mine: if that comparison is really Cycles' own test, then whether a render is tiled turns on picture area against tile area — at the default tile size of 2048 that is a threshold of 4,194,304 pixels, which a 1920×1080 picture (2,073,600 pixels) sits below and a 3840×2160 picture (8,294,400 pixels) sits above. I stress that this is a step beyond the file: the file only decides whether to draw a warning label, and the compiled code it claims to match is not in the folder I was given, so I cannot check it.

**Even granting all of that, no source says the progress report changes when a render is tiled.** That is the gap.

The closest thing I found is a line in the 5.1 fixed-bug list, in the EEVEE and Vulkan section:

> " * Vulkan Wrong Rendering Tiles are displayed in the Blender Render window"

Source: release notes, https://developer.blender.org/docs/release_notes/5.1/bugfixes/ — read 2026-07-30. **Label: documented** for the fact that this bug was fixed in 5.1. **Label: inferred**, and weakly, for anything more: the title only makes sense if rendering tiles are normally shown in the Render window, so something tile-shaped is visible there while a render runs. It says nothing about a number, a percentage, or any text, and it is the title of a bug report rather than a description of correct behaviour.

I also found nothing anywhere describing progress at the small end. There is no statement about a 4×4 render, or an effectively-zero-pixel render, reporting anything different, or reporting at all.

### Observed — what a running Blender 5.1 does (2026-08-11)

Run in Blender 5.1.2 on **the Creator's Mac (a MacBook Pro with an Apple M1 Pro chip, 16 GB of memory, macOS 26.5.2, Apple's own graphics and no second card)** and on **a rented Linux machine (Ubuntu 22.04.5 LTS on kernel 6.8, two NVIDIA RTX 4000 Ada Generation cards of 20 GB each on driver 580.105.08, eleven cores, 49 GB of memory)**, each with a settings folder of its own so nothing touched the Creator's own Blender settings. The scene was built by Blender 5.1 itself: a cube, a sphere, an area light and a camera; Cycles on the processor; 16 samples; adaptive sampling off; output PNG. **LABEL: observed.**

**1. The progress report does change with picture size, and the threshold is where the interface code above says it is.** The same frame was rendered from a terminal with no window at six sizes: 4 by 4, 160 by 120, 1920 by 1080, 2048 by 2048, 2049 by 2049 and 4096 by 4096. At the first four the capture holds **three** lines carrying a sample count and mentions no tile. At 2049 by 2049 and 4096 by 4096 it holds **thirteen**, all mentioning tiles, and the stage becomes `Rendered <n>/<total> Tiles, Sample <s>/<max>`. Two of the thirteen from the 2049 by 2049 run on the Creator's Mac:

```
00:00.702  render           | Fra: 1 | Remaining: 00:02.99 | Mem: 241M | Rendered 0/4 Tiles, Sample 1/16
00:04.055  render           | Fra: 1 | Mem: 1M | Rendered 3/4 Tiles, Sample 16/16
```

**2048 by 2048 is exactly the tile area worked out above, 4,194,304 pixels, and it printed no tile; 2049 by 2049 printed four.** So the comparison in that code is strictly greater than, not greater than or equal to, and the threshold holds when run. All six runs exited 0 and wrote a picture of the size asked for. The rented Linux machine gave the same split at the same sizes and the same tile counts. **The two machines agreed.**

**2. The arithmetic at the small end holds, and a picture of no pixels is refused.** 4 by 4 at 1 per cent — the case that works out to zero pixels a side — wrote nothing, printed

```
00:00.264  reports          | ERROR Image too small
```

and **still exited 0**. At 1920 by 1080 the written picture is **1920 x 1080** at 100 per cent, **960 x 540** at 50 per cent and **633 x 356** at 33 per cent, so the product is cut down to a whole number rather than rounded — rounding would give 634. Both machines.

**3. The settings clamp silently and never refuse.** Set from Python and read back: 0, 3 and 4 all become **4** for the two size settings; 65536, 65537 and 70000 all become **65536**; for the percentage 0 becomes **1** and 32768 and 70000 become **32767**. No error, nothing printed, both machines.

**4. The exact wording of the progress text, which no source quotes.** With a window open, photographed with Blender's own screenshot command every few seconds: there is **exactly one bar**, never two. Beside it, in order: a small camera icon, the words **`Rendering...`**, the bar with a **whole-number percentage** written inside it, and an **X** at the right end. No decimal point in any photograph. Mid-render readings: `Rendering...` beside `23%` on the Creator's Mac, `Rendering...` beside `26%` on the rented Linux machine. **A sequencer render drives the same bar and is labelled `Rendering sequence...`** — not `Sequence Render`, which is the label in the manual's one picture.

**5. What the progress figure spans: one frame, not a whole animation.** During a three-frame animation the label stayed `Rendering...`, never naming a frame, and the percentage **restarted at each frame**. Fourteen photographs of one three-frame animation, in order, read 16, 16, 16, 16, 16, 0, 16, 0, 48, 80, 0, 80, 0 per cent. Both machines.

**6. Picture size makes no difference to the on-screen report.** The same one bar and the same whole-number percentage at 4 by 4 and at 4096 by 4096, so nothing about tiles is counted or named on screen even when the printed stream counts them.

**7. The tiling settings and the interface code, read from the running program.** `use_auto_tile` is named **Auto Tile** and described **"Deprecated, tiling is always enabled"**, default True; `tile_size` default 2048, range `[8, 8192]`. Searching Blender 5.1's own installed Cycles add-on for the words "May work poorly with render tiling" and for the comment naming `compute_render_tile_size` found **both** — the code quoted above is really there. Blender 4.5's add-on carries **neither**. Both machines.

**8. A render engine's own progress calls work with no window.** A tiny engine registered from a script called `update_progress`, `update_stats` and `update_memory_stats`; all three returned without complaint, with a window and without, on both machines, and none produced output of its own.

**9. A render at the very top of the accepted range does not finish.** At 65536 by 65536 with one sample per pixel, on the rented Linux machine, the render divided into **1024 tiles** and printed a line for each — 1026 tile lines — reaching `Rendered 1023/1024 Tiles, Sample 0/1`, with the machine's own memory use at about 3.5 GB of 49. It then stopped at the point of putting the tiles together, printing `reports | ERROR Error reading tiles from file` on standard output and, on standard error:

```
OpenImageIO exited with a pending error message that was never
retrieved via OIIO::geterror(). This was the error message:
Uncompressed image size 245760.0 MB exceeds the 81920 MB limit.
Image claimed to be 65536x65536, 15-channel float. Possible corrupt input?
If this is a valid file, raise the OIIO attribute "limits:imagesize_MB".
```

The run **exited 1 and wrote no file**, and all four versions did exactly the same. So the arithmetic worked out above says what may be asked for; it does not say what a render will produce, and at the top of the range it produces nothing. It was not run on the Creator's Mac, whose 16 GB is not to be filled on purpose.

### Not covered

- **Whether a 65536 by 65536 render would save into a format OpenImageIO's size limit allows, or with that limit raised.** Not tested. What was tested is recorded just below.
- **What the time estimate behind the hover says.** It needs a hover, and neither machine has any way of moving the pointer without a hand on it, so it was dropped rather than run.
- **Whether the Render window fills in as visible rectangles.** The photographs above are of the whole Blender window and its status bar; nothing was recorded about how the picture area filled in.
- Render Region and Crop to Render Region, which were not tested.
- **Anything on Windows.** No Windows machine has run it. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

### Outside the interface

**Label: documented in part, not in the sources for the rest.**

The status bar report is a piece of the Blender window. The manual places it there in its first line: "The Status Bar is located at the bottom of the Blender window" (same page as above). A render started and driven from outside Blender's own window has no such bar, so that report is not available. The manual does not say what stands in for it.

One documented difference does exist, and it is about the after-the-fact statistics rather than live progress: the `render_stats` handler is described as carrying "render/saving time plus **in background mode** frame/used [peak] memory" (Python API, https://docs.blender.org/api/5.1/bpy.app.handlers.html, read 2026-07-30). Those words say the content of that report is not the same in background mode as it is otherwise. The API does not say the same about `update_progress` or `update_stats`; both are listed without any note about mode, and the sources do not say where their values go when there is no window to draw them in.

For the size half of the question: nothing in any source suggests the accepted size range differs when Blender is driven from outside. `resolution_x`, `resolution_y` and `resolution_percentage` are one set of settings with one set of limits.

**The sources do not say** what a render prints, or reports at all, about its progress when there is no interface, at any picture size. I searched for it specifically (terms and pages listed below) and found nothing.

**Observed (2026-08-11), and the gap is now filled.** With no window there is no status bar and no percentage. What stands in for it is a printed stream, captured in Blender 5.1.2 on both machines. For a sixteen-sample frame below the tile threshold it is three lines carrying a sample count, of the shape `00:00.530  render           | Fra: 1 | Remaining: 00:00.09 | Mem: 3M | Sample 1/16` — a frame number, a time estimate called `Remaining`, a memory figure and a sample count against its maximum. Above the tile threshold the same lines gain a tile count and there are thirteen of them. The step marked inferred above is therefore borne out: no bar exists to photograph when there is no window, and the printed lines are what replaces it. The size settings behave identically either way — the same clamping figures came back from a windowless run.

### Operating system

**Label: not in the sources**, with one nearby exception.

Nothing I found ties progress reporting, or the accepted range of picture sizes, to any operating system. The status bar page, the two API pages, and the Cycles tile settings are all written without qualification.

The one operating-system statement I found anywhere near this ground is about whether a render happens at all without a window, not about what it reports:

> "Headless Rendering
>
> Headless rendering is not supported on headless Windows systems."

Source: manual, https://docs.blender.org/manual/en/5.1/render/eevee/limitations/limitations.html — read 2026-07-30. That is EEVEE's page and speaks only for EEVEE.

**Observed (2026-08-11).** Settled for macOS and for Linux, and they agreed on every point in the observed section above: the same three-or-thirteen sample lines at the same six sizes, the same tile counts, the same clamping figures, the same written picture sizes, the same `ERROR Image too small`, the same `Rendering...` and `Rendering sequence...` labels, and the same whole-number percentages. **Windows is untested** — no Windows machine has run it — so the EEVEE line about headless Windows systems stands unchecked.

### Where I looked, and what I ruled out

Searched across the whole of `manual_5.1`, `api_5.1` and `release_notes_5.1` at once, for these terms (case-insensitive unless noted):

- `progress`, `progress bar`, `Progress` — in the manual this returned about forty files. I ruled out, by opening or by reading the matched line in place: the sculpting mesh filter, preferences pages, rigify, EEVEE ray-tracing ("progressively"), legacy blend textures ("progression"), Map Range nodes, Freestyle line style alpha, shape keys, inverse kinematics, data transfer, build and decimate modifiers, geometry-node volume and hair pages, the compositing introduction, the glossary, motion-tracking stabilisation, and the physics baking, fluid domain, cache, noise, guides, mesh, particles and cloth pages. All of these use the word for something other than a render's progress, mostly a shape of interpolation or a simulation bake. The only rendering-relevant hits in the whole manual were the two lines on the Status Bar page, quoted above. In the release notes the only hits were a texture-baking progress bar bug and a sequencer proxy-building progress bar, neither of which is a render.
- `time estimate`, `remaining time`, `time remaining`, `elapsed`, `ETA`, `percent` — one relevant hit only, the Status Bar hover line. `elapsed` and `percent` hit only modifiers, node groups and slider interface pages.
- `Mem:`, `Fra:`, `Time:`, `Sample <number>`, `render status`, `status text`, `render information`, `render statistics`, `render stats`, `samples left`, `samples remaining` — searched for these because they are the shapes a live render report tends to take. `Mem:` and `Fra:` returned nothing at all anywhere in the manual, API or release notes. `Time:` hit only a mesh cache modifier, a custom-properties page, a video-format page and fluid noise settings. `Sample <number>` returned nothing. `render status` / `status text` returned only the API lines for `update_stats`, `WorkSpace` and `Area`, all quoted or dismissed above. `sample count` returned only settings descriptions on the EEVEE and Cycles sampling pages, none of them about reporting.
- `tiled`, `in tiles`, `tiled render`, `render tiling`, `8192`, `tile_size`, `Tile Size` — across manual and API. The only rendering hits in the manual are the two Tile Size lines on the Cycles Performance page. `tile_size` in the API documentation returns only EEVEE's `volumetric_tile_size`, which is about volume shading, not image tiling; Cycles' own `tile_size` is an add-on setting and is not in the API documentation at all, which is why I took it from Cycles' files instead. `8192` in the API returns only audio mixing buffer sizes and a shadow-resolution clamp.
- `65536`, `resolution_x`, `resolution_percentage` — gave the accepted ranges quoted above, and otherwise only image and UDIM tile generated-size limits, which are about image data-blocks rather than renders.
- `high resolution`, `large image`, `too large`, `very large`, `maximum resolution`, `image size` — looking for any statement about behaviour at the big end of the range. Nothing about renders; the hits were display preferences, sculpting, grease pencil tracing, menus, node frames and compositing nodes.
- `background mode`, `headless` — for the outside-the-interface line. Hits were the scripting security page, the add-on tutorial, four lines of the command-line arguments page (none about progress output), and EEVEE's headless note quoted above.

Pages opened in full and ruled out: the Cycles Performance page (has Tile Size, no reporting), the Cycles Sampling page (samples, adaptive sampling, time limit, sample subsets — no reporting), the Render Introduction page, the Render Output introduction and the Rendering Animations page (workflow advice, no reporting), the Format page (the size settings, no reporting), the Topbar Render menu (the four render commands and the Render window shortcut, no reporting), the EEVEE Limitations page, the command-line arguments page's rendering and background sections, the API pages for `RenderEngine`, `RenderSettings`, `WindowManager` progress calls (those are for scripts, not renders) and `bpy.app.handlers`, and the 5.1 release notes for Rendering and for Cycles in full (neither mentions progress reporting or picture sizes).

Cycles' own files: I searched every Python file in `Blender Sources/cycles_5.1` for `progress`, `Rendering`, `Sample`, `Waiting`, `Updating` and `_cycles`. The only occurrence of "progress" in the whole add-on is a comment in `operators.py` line 97, "`# TODO: support cancel and progress reports.`", which is about an add-on operator and not about a render's progress. There are no progress or status strings in the add-on at all, and `engine.py` and `__init__.py` simply hand rendering to the compiled part. I also searched the `cycles_5.1/source` folder, which holds only kernel and utility headers, and found "progress" only inside two unrelated shading files (`kernel/integrator/mnee.h`, `kernel/sample/pattern.h`). The code that produces Cycles' progress text is not in the files I was given, which is why 7c ends where it does.

Pictures: I opened `_images/interface_window-system_status-bar_ui-center.png`, described above. I checked the whole `_images` folder for any file whose name contains "progress" — there are none — and for status bar images, of which there are four; the centre one is the only one the manual attaches to the Running Task text.

I did not need the web and did not use it.

---

## Things a person with Blender 5.1 open could settle in a few minutes

Six of the seven items that stood here have been run, on both machines, and their results are written into the answer above. Item 6 in particular — the tile threshold — came out exactly where the code suggested. What is left open is below.

1. **The time estimate that appears when the pointer rests on the progress bar.** It needs a hover, and neither machine has any way of moving the pointer without a hand on it.

2. **Whether the Render window fills in as visible rectangles (item 7).** The photographs taken were of the whole Blender window and its status bar; nothing was recorded about how the picture area itself filled in.

3. **Whether a 65536 by 65536 render would save into a format OpenImageIO's size limit allows, or with that limit raised (item 4).** What it does as it stands is recorded in the answer above: it renders all 1024 tiles and then fails to save, exiting 1.

4. **Anything on Windows.** No Windows machine has run it.

---

2026-08-11 — Claude Opus 5 (Claude Code) — Reports, Blender 5.1

# Reports — cross-sweep questions (Blender 4.5)

Sources consulted: Blender's 4.5 manual (`Blender Sources/manual_4.5`, including its pictures in `_images`), Blender's 4.5 Python API documentation (`Blender Sources/api_4.5`), Blender's 4.5 release notes (`Blender Sources/release_notes_4.5`), and Cycles' own files (`Blender Sources/cycles_4.5`). All were taken on 2026-07-30, and that is the date read for everything below.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

Questions answered: 7, 8, 9, 10 and 11.

---

## 7. What does a render report about its own progress, across the whole range of picture sizes a render will accept?

**Reading taken.** I read "picture sizes a render will accept" as the range of render resolutions Blender allows. This was ambiguous. The other reading would be the range of sizes the output *file formats* accept, which is a different question and is not answered here.

The answer splits in two: what sizes are accepted, and what progress is reported. The first is documented. The second is documented in general terms, but nothing in the sources connects it to the picture size.

### The sizes the resolution settings accept — documented

> resolution_x
>
> Number of horizontal pixels in the rendered image
>
> Type:
>
> int in [4, 65536], default 1920

> resolution_y
>
> Number of vertical pixels in the rendered image
>
> Type:
>
> int in [4, 65536], default 1080

> resolution_percentage
>
> Percentage scale for render resolution
>
> Type:
>
> int in [1, 32767], default 100

Source: Python API documentation, https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html — read 2026-07-30.

So the two resolution settings each accept a whole number from 4 to 65536, and the percentage scale accepts a whole number from 1 to 32767. **LABEL: documented** for what those three settings accept.

That a render will actually run at every size in that range is a further step: what a setting accepts and what a render does with it are two different claims, and no quoted words here join them. **LABEL: inferred** for that step. The same point is repeated under *Not covered* below.

### What progress is reported — documented

The manual describes a progress bar in the Status Bar:

> Running Task
>
> Shows the progress of the currently running task (such as rendering or baking).
> Hovering the mouse pointer over the progress bar will display a time estimate.
> The task can be aborted by clicking the cancel button ().

Source: manual, https://docs.blender.org/manual/en/4.5/interface/window_system/status_bar.html — read 2026-07-30.

I opened the picture beside that passage, `_images/interface_window-system_status-bar_ui-center.png` (the manual names it at that point in the page). It shows a single horizontal bar with a film-camera icon, the words "Sequence Render", the bar filled about nine tenths of the way, the number "94%" inside it, and an X button at the right end. **LABEL: documented** for exactly that — what is visible in the picture.

Two things a reader has to supply, and neither is visible. That the figure is *always* a whole-number percentage: the picture shows one figure, "94%", on one occasion, which does not settle what is shown at other moments. That the X button is the cancel button the passage names: the passage says a cancel button exists, and the picture shows an X, and joining them is a step. **LABEL: inferred** for both.

The Python API says what a render engine hands over as progress:

> update_progress(progress)
>
> Update progress percentage of render
>
> Parameters:
>
> progress (float in [0, 1]) – Percentage of render that's done

Source: Python API documentation, https://docs.blender.org/api/4.5/bpy.types.RenderEngine.html — read 2026-07-30.

That is a fraction of the whole job. It carries no pixel counts, no row or tile numbers, and no size information of any kind.

**LABEL: documented** for what the three resolution settings accept, for the existence of a progress bar in the Status Bar with a hover time estimate and a cancel button, and for `update_progress` taking a fraction between 0 and 1.

**LABEL: inferred** for treating those as one thing. Nothing quoted here says the fraction a render engine hands to `update_progress` is the number the Status Bar draws, or that one is the other shown as a percentage. The step is that both describe render progress on the same screen; the sources do not join them.

### Whether the progress report changes with picture size — not in the sources

**LABEL: not in the sources.**

Nothing in the manual, the API documentation, the release notes, or Cycles' own files says that what a render reports about its progress differs at one picture size versus another. Nothing says progress is reported per tile, per row, or per pixel band at large sizes.

The nearest thing is Cycles' tiling, which is size-driven. From Cycles' own files:

> ```
> use_auto_tile: BoolProperty(
>     name="Use Tiling",
>     description="Render high resolution images in tiles to reduce memory usage, using the specified tile size. Tiles are cached to disk while rendering to save memory",
>     default=True,
> )
> tile_size: IntProperty(
>     name="Tile Size",
>     default=2048,
>     description="",
>     min=8, max=8192,
> )
> ```

Source: `Blender Sources/cycles_4.5/properties.py`, lines 1051–1061. **LABEL: observed** (this is what the file says: the setting exists, it is on by default, the tile is 2048 by default, and it can be set between 8 and 8192). Read 2026-07-30.

The manual says the same in words:

> Use Tiling
>
> Render high resolution images in tiles to reduce memory usage.
> Tiles are cached to disk while rendering to save memory.

Source: manual, https://docs.blender.org/manual/en/4.5/render/cycles/render_settings/performance.html — read 2026-07-30.

That tiling happens is documented. That tiling changes what progress is *reported* is not stated anywhere I could find, and I am not asserting it.

### Where I looked (for the "not in the sources" part)

Searched the whole of `manual_4.5`, `api_4.5`, `release_notes_4.5` and `cycles_4.5` for: "progress bar", "progress", "time remaining", "remaining", "ETA" (as a whole word), "elapsed", "percent", "percentage", "estimate", "tile size", "Rendering progress", "Path Tracing Sample", "Sample 1/", "sample count", "maximum resolution", "65536", "image size limit", "too large". The only hits bearing on render progress are the Status Bar page, the `RenderEngine` API page and the Cycles Performance page, all quoted above. The 4.5 release notes contain no hit for "progress" at all. Cycles' Python files contain no progress-printing code: the only hit for "progress" in them is a comment in `operators.py` line 102, "# TODO: support cancel and progress reports.", which is about an operator, not about rendering.

### Observed — what a running Blender 4.5 does (2026-08-11)

Every test below was run in Blender 4.5.12 on both of the machines this project can reach: **the Creator's Mac — a MacBook Pro with an Apple M1 Pro chip, 16 GB of memory, macOS 26.5.2, Apple's own graphics and no second card** — and **a rented Linux machine — Ubuntu 22.04.5 LTS on kernel 6.8, two NVIDIA RTX 4000 Ada Generation cards of 20 GB each on driver 580.105.08, an AMD Ryzen Threadripper PRO 3945WX with eleven cores, 49 GB of memory**. Each run was given a settings folder of its own, so nothing touched the Creator's own Blender settings. **LABEL: observed** throughout.

**The scene.** Built by Blender 4.5 itself and saved as a file: a cube, a sphere, an area light and a camera; Cycles on the processor; 16 samples; adaptive sampling off; time limit 0; 160 by 120 pixels; output PNG.

**1. The progress report does change with picture size, and the change is sharp.** The same frame was rendered from a terminal with no window, six times, changing only the two resolution numbers: `blender -b <file>.blend --python-expr "import bpy; r=bpy.context.scene.render; r.resolution_x=<X>; r.resolution_y=<Y>" -o <out> -f 1`, at 4 by 4, 160 by 120, 1920 by 1080, 2048 by 2048, 2049 by 2049 and 4096 by 4096. All six exited 0 and wrote a picture of the size asked for.

At 4 by 4, 160 by 120, 1920 by 1080 and 2048 by 2048 the capture holds **three** lines carrying a sample count and mentions no tile. At 2049 by 2049 and at 4096 by 4096 it holds **thirteen**, and every one of them mentions a tile. The whole of the thirteen, from the 2049 by 2049 run on the Creator's Mac:

```
Fra:1 Mem:8.34M (Peak 9.10M) | Time:00:00.00 | Mem:0.19M, Peak:0.19M | Scene, ViewLayer | Rendered 0/4 Tiles, Sample 0/16
Fra:1 Mem:248.42M (Peak 248.42M) | Time:00:00.18 | Remaining:00:02.66 | Mem:240.19M, Peak:240.19M | Scene, ViewLayer | Rendered 0/4 Tiles, Sample 1/16
Fra:1 Mem:248.42M (Peak 248.42M) | Time:00:03.24 | Remaining:00:00.00 | Mem:240.19M, Peak:240.19M | Scene, ViewLayer | Rendered 0/4 Tiles, Sample 16/16
Fra:1 Mem:248.42M (Peak 248.42M) | Time:00:03.24 | Remaining:00:00.00 | Mem:240.19M, Peak:240.19M | Scene, ViewLayer | Rendered 1/4 Tiles, Sample 16/16
Fra:1 Mem:8.54M (Peak 248.42M) | Time:00:03.24 | Remaining:00:00.00 | Mem:0.30M, Peak:240.19M | Scene, ViewLayer | Rendered 1/4 Tiles, Sample 1/16
Fra:1 Mem:8.54M (Peak 248.42M) | Time:00:03.25 | Remaining:00:00.00 | Mem:0.30M, Peak:240.19M | Scene, ViewLayer | Rendered 1/4 Tiles, Sample 16/16
Fra:1 Mem:8.54M (Peak 248.42M) | Time:00:03.25 | Remaining:00:00.00 | Mem:0.30M, Peak:240.19M | Scene, ViewLayer | Rendered 2/4 Tiles, Sample 16/16
Fra:1 Mem:8.54M (Peak 248.42M) | Time:00:03.25 | Remaining:00:00.00 | Mem:0.30M, Peak:240.19M | Scene, ViewLayer | Rendered 2/4 Tiles, Sample 1/16
Fra:1 Mem:8.54M (Peak 248.42M) | Time:00:03.25 | Remaining:00:00.00 | Mem:0.30M, Peak:240.19M | Scene, ViewLayer | Rendered 2/4 Tiles, Sample 16/16
Fra:1 Mem:8.54M (Peak 248.42M) | Time:00:03.25 | Remaining:00:00.00 | Mem:0.30M, Peak:240.19M | Scene, ViewLayer | Rendered 3/4 Tiles, Sample 16/16
Fra:1 Mem:8.42M (Peak 248.42M) | Time:00:03.25 | Remaining:00:00.00 | Mem:0.19M, Peak:240.19M | Scene, ViewLayer | Rendered 3/4 Tiles, Sample 1/16
Fra:1 Mem:8.42M (Peak 248.42M) | Time:00:03.25 | Mem:0.19M, Peak:240.19M | Scene, ViewLayer | Rendered 3/4 Tiles, Sample 16/16
Fra:1 Mem:8.42M (Peak 248.42M) | Time:00:03.25 | Mem:0.19M, Peak:240.19M | Scene, ViewLayer | Rendered 3/4 Tiles, Sample 0/16
```

So the stage at the end of the line becomes `Rendered <n>/<total> Tiles, Sample <s>/<max>` once the picture crosses one tile's worth of area. 2048 by 2048 is exactly the default tile size squared and printed no tile; 2049 by 2049 printed four. **The rented Linux machine gave the same three-lines-or-thirteen split at the same six sizes, and the same tile counts. The two machines agreed.**

**2. The settings clamp silently, and never refuse.** Values were set from Python and read straight back: 0, 3 and 4 all become **4** for the two size settings; 65536, 65537 and 70000 all become **65536**; for the percentage 0 becomes **1** and 32768 and 70000 become **32767**. Nothing raised an error and nothing was printed. Identical on both machines.

**3. A render at 4 by 4 finishes and writes a 4 by 4 picture**, on both machines. **A render at the top of the range starts, tiles, and then fails to save.** At 65536 by 65536 with one sample per pixel, on the rented Linux machine, the render divided into **1024 tiles** and printed a line for each — 1026 tile lines in all — reaching `Rendered 1023/1024 Tiles, Sample 0/1`, with the machine's own memory use staying at about 3.5 GB of 49 and Blender's own figure at `Mem:248.54M`. It then stopped at the point of putting the tiles together, printing on standard output

```
Error: Error reading tiles from file
```

and on standard error

```
OpenImageIO exited with a pending error message that was never
retrieved via OIIO::geterror(). This was the error message:
Uncompressed image size 245760.0 MB exceeds the 81920 MB limit.
Image claimed to be 65536x65536, 15-channel float. Possible corrupt input?
If this is a valid file, raise the OIIO attribute "limits:imagesize_MB".
```

The run **exited 1 and wrote no file**. All four versions did exactly the same, on this machine. So a size the settings accept is not necessarily a size a render can finish: the step from what a setting will hold to what a render will produce is a real one, and at the top of the range it does not hold. This was not run on the Creator's Mac, whose 16 GB is not to be filled on purpose.

**4. The percentage scale cuts down rather than rounds.** At 1920 by 1080 the written picture is **1920 x 1080** at 100 per cent, **960 x 540** at 50 per cent and **633 x 356** at 33 per cent — 633 is the product cut down to a whole number, where rounding would give 634. Identical on both machines. **A picture of no pixels is refused, and the run still exits 0**: at 4 by 4 with the percentage at 1, nothing was written and the whole capture on the Creator's Mac reads

```
Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:41:35)
Read blend: "…/scenes/base_4.5.blend"
Error: Image too small

Blender quit
```

**5. The status bar during a render, with a window open.** A render was started the way the F12 key starts it and the whole Blender window was photographed with Blender's own screenshot command every few seconds. There is **exactly one bar**, never two. Beside it, in order: a small camera icon, the words **`Render...`**, the bar with a **whole-number percentage** written inside it, and an **X** at the right end. No decimal point appeared in any photograph. Mid-render readings: `Render...` beside `17%` on the Creator's Mac, `Render...` beside `26%` on the rented Linux machine. **No frame is named**, for a still or for an animation; during a three-frame animation the label stayed `Render...` and the percentage restarted at each frame rather than climbing across the range. Picture size made no difference to any of it — the same one bar and the same whole-number percentage at 4 by 4 and at 4096 by 4096. Both machines agreed.

**6. A sequencer render drives the same bar, and it is labelled `Rendering sequence...`** — not `Sequence Render`, which is the label in the manual's one picture of that bar. Caught mid-render at `0%` and `16%`, on both machines.

**7. The two tiling settings, read from the running program.** `use_auto_tile` is named **Use Tiling**, described "Render high resolution images in tiles to reduce memory usage, using the specified tile size. Tiles are cached to disk while rendering to save memory", default True. `tile_size` has **no description at all**, default 2048, range `[8, 8192]`. The three size settings: `resolution_x` and `resolution_y` default 1920 and 1080, range `[4, 65536]`; `resolution_percentage` is named **Resolution Scale**, default 100, range `[1, 32767]`. Identical on both machines.

**8. Searching Blender 4.5's own installed Cycles add-on** for the words "May work poorly with render tiling" and for a comment naming `compute_render_tile_size` found **neither**, on both machines.

**9. A render engine's own progress calls work with no window.** A tiny render engine registered from a script called `update_progress`, `update_stats` and `update_memory_stats`. All three returned without complaint, with a window and without, on both machines, and none of them produced any output of its own.

**Not covered.** This does not settle what the time estimate says when the pointer rests on the progress bar, which needs a hover and could not be reached (see below). It does not settle whether a 65536 by 65536 render would save into a format OpenImageIO's size limit allows, or with that limit raised. It does not settle anything about Windows.

**Outside the interface.** Settled, and it differs. With no window there is no status bar and no percentage; what a render reports instead is the printed lines quoted above, and those carry a sample count, a memory figure and a `Remaining:` time estimate. The size range is the same either way — the same clamping figures came back from a windowless run. Both machines agreed.

**Operating system.** Settled for macOS and for Linux, and they agreed on every point above: the same three-or-thirteen sample lines at the same sizes, the same tile counts, the same clamping, the same written picture sizes, the same `Error: Image too small`, the same `Render...` label and whole-number percentage, and the same settings. **Windows is untested** — no Windows machine has run it. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

---

## 8. What does a render print over the whole of a run, from its first line to its last?

**LABEL: not in the sources.**

Nothing in the manual, the API documentation, the release notes or Cycles' own files sets out the lines a render prints, in order, from the first to the last. There is no sample console transcript of a render anywhere in these sources. What the sources do carry is a handful of statements about the printing, quoted below, plus switches that add to it.

### What the sources do say

That there is a stream, and where it goes:

> The Console Window, also called a Terminal, is an operating system text window that displays
> messages about Blender's operations, status, and internal errors.
>
> When Blender is manually started from a terminal,
> Blender output is shown in the Console Window it is started from.

Source: manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/index.html — read 2026-07-30. **LABEL: documented.**

That statistics can be added, and what they cover:

> --cycles-print-stats
>
> Log statistics about render memory and time usage.

Source: manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html — read 2026-07-30. **LABEL: documented.**

Cycles' own file says which stream those go to:

> ```
> parser.add_argument("--cycles-print-stats",
>                     help="Print rendering statistics to stderr",
>                     action='store_true')
> ```

Source: `Blender Sources/cycles_4.5/engine.py`, lines 13–15. **LABEL: observed** — the file states the stream is stderr, not stdout. Read 2026-07-30.

That Blender itself prints render statistics, and that these differ in background mode:

> bpy.app.handlers.render_stats
>
> on printing render statistics. Accepts one argument: the render stats (render/saving time plus in background mode frame/used [peak] memory).

Source: Python API documentation, https://docs.blender.org/api/4.5/bpy.app.handlers.html — read 2026-07-30. **LABEL: documented** that a handler exists which fires on printing render statistics, and that its one argument is described in those words.

Reading that description as "render time and saving time always, plus the frame number and used (peak) memory when in background mode" is a step. The words are compressed and carry no verb, and nothing else quoted here says what is printed in either case. **LABEL: inferred** for that reading, and for the conclusion drawn from it that background mode prints more than a run with a window. It is the closest the sources come to naming the content of a printed render line, and it is still not a statement of one.

That video encoding also prints, and that a switch adds more:

> Tip
>
> When you view the System Console,
> you can see some of the output of the encoding process.
> You will see even more output if you execute Blender as blender -d.

Source: manual, https://docs.blender.org/manual/en/4.5/render/output/properties/output.html — read 2026-07-30. **LABEL: documented.**

Switches that change how much is printed, all from https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html (read 2026-07-30): `--log <match>` ("Enable logging categories, taking a single comma separated argument"), `--log-level <level>` ("Set the logging verbosity level (higher for more details) defaults to 1, use -1 to log all levels"), `--log-show-timestamp` ("Show a timestamp for each log message in seconds since start"), `--log-file <filepath>` ("Set a file to output the log to"), `-d, --debug` ("Turn debugging on"), and `--debug-cycles` ("Enable debug messages from Cycles").

### The two pictures of a console, and what they do and do not show

The manual's Windows launch page names a picture, `_images/advanced_command-line_launch_windows_console.png`, with the caption "Blender's Console Window on Microsoft Windows." and the text "The screenshot shows the Blender Console Window on Microsoft Windows directly after starting Blender and then a short while later after opening a file along with the relevant messages." (https://docs.blender.org/manual/en/4.5/advanced/command_line/launch/windows.html, read 2026-07-30).

I opened it. It shows a Command Prompt window containing, in order: the command `blender`, then `Read new prefs: C:\Users\Aaron\AppData\Roaming\Blender Foundation\Blender\2.77\config\userpref.blend`, `found bundled python: c:\blender\2.77\python`, `read blend: C:\tmp\Vincent.blend`, a blank line, then `Blender quit`, then the prompt again. **LABEL: documented** for what is visible in the picture.

Two limits on that picture, and they matter. It is not a render — the manual's own caption says it is a start-up and a file being opened. And the paths inside it read `2.77`, so the picture in the 4.5 manual was taken from an older Blender. I am not treating it as showing what a 4.5 render prints.

The Linux launch page names `_images/advanced_command-line_launch_linux_console.png`, captioned "Starting Blender from a Linux terminal window." (https://docs.blender.org/manual/en/4.5/advanced/command_line/launch/linux.html, read 2026-07-30). I opened it. It shows the command `blender` followed by audio-system messages ("Cannot connect to server socket err = No such file or directory", "jack server is not running or cannot be started", and similar). **LABEL: documented** for what is visible in the picture. Again, no render.

### Where I looked

Searched the whole of `manual_4.5`, `api_4.5`, `release_notes_4.5` and `cycles_4.5` for text that would appear in a printed render run: "Fra:", "Mem:", "Peak:", "Peak memory", "Saved:", "Blender quit", "Time:", "Append frame", "stdout", "standard output", "console output", "render statistics", "print statistics", "print_stats", "cycles-print-stats", "Synchronizing", "Compositing", "Denoising", "Loading render kernels", "Updating Scene", "Path Tracing Sample", "Sample 1/". Of these, only "print_stats", "cycles-print-stats", "render statistics" and "Peak memory" produced anything bearing on the question, and all of it is quoted above. "Fra:", "Mem:", "Peak:", "Saved:" and "Blender quit" produced no hits at all in any of the four sources.

Ruled out and why: the 4.5 release notes were searched in full for "progress", "log", "stat", "render" and "memory"; none of the hits concern what a render prints. Cycles' own Python files were ruled out because they contain no printing of render progress — they hand the work to a compiled module (`import _cycles` … `_cycles.render(engine.session, …)` in `engine.py`, lines 89–92), and that module is not among the files supplied. The Cycles files that are supplied beyond the Python add-on are kernel headers under `cycles_4.5/source/kernel` and `cycles_4.5/source/util`, which are the code that runs on the rendering device and contain no console printing.

### Observed — what a Blender 4.5 render actually prints (2026-08-11)

Run in Blender 4.5.12 on **the Creator's Mac (a MacBook Pro with an Apple M1 Pro chip, 16 GB of memory, macOS 26.5.2, Apple's own graphics and no second card)** and on **a rented Linux machine (Ubuntu 22.04.5 LTS on kernel 6.8, two NVIDIA RTX 4000 Ada Generation cards of 20 GB each, eleven cores, 49 GB of memory)**. Each run had a settings folder of its own. The scene was built by Blender 4.5 itself: a cube, a sphere, an area light and a camera; Cycles on the processor; 16 samples; adaptive sampling off; 160 by 120 pixels; output PNG. **LABEL: observed.**

**1. The whole of one run, first line to last.** The command was `blender -b <file>.blend -o <out> -f 1 > out.txt 2> err.txt`. It exited 0, wrote its picture, and put **nothing at all on standard error**. Standard output held fifty lines. The first three, the last of the stage lines, all three sample lines, and the last four, word for word, from the Creator's Mac:

```
Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:41:35)
Read blend: "…/scenes/base_4.5.blend"
Fra:1 Mem:8.30M (Peak 9.10M) | Time:00:00.03 | Mem:0.00M, Peak:0.00M | Scene, ViewLayer | Synchronizing object | Cube
…
Fra:1 Mem:8.34M (Peak 9.10M) | Time:00:00.03 | Mem:0.19M, Peak:0.19M | Scene, ViewLayer | Loading denoising kernels (may take a few minutes the first time)
Fra:1 Mem:8.34M (Peak 9.10M) | Time:00:00.03 | Mem:0.19M, Peak:0.19M | Scene, ViewLayer | Sample 0/16
Fra:1 Mem:9.45M (Peak 9.45M) | Time:00:00.04 | Remaining:00:00.05 | Mem:1.21M, Peak:1.21M | Scene, ViewLayer | Sample 1/16
Fra:1 Mem:10.04M (Peak 10.62M) | Time:00:00.21 | Mem:1.21M, Peak:1.21M | Scene, ViewLayer | Sample 16/16
Fra:1 Mem:10.04M (Peak 10.62M) | Time:00:00.21 | Mem:1.21M, Peak:1.21M | Scene, ViewLayer | Finished
Saved: '…/out/e1_4.5_0001.png'
Time: 00:00.27 (Saving: 00:00.05)


Blender quit
```

So: **the first line is the version banner and the last line is `Blender quit`, with two blank lines before it.** The thirty-eight lines elided above all have the same shape, differing only in the stage named at the end — Synchronizing object, Initializing, Waiting for render to start, Updating Scene, Updating Shaders, Loading render kernels, Updating Procedurals, Updating Background, Updating Camera, Updating Meshes Flags, Updating Objects, Updating Particle Systems, Updating Meshes, Updating Mesh, Updating Scene BVH, Updating Objects Flags, Updating Primitive Offsets, Updating Images, Updating Camera Volume, Updating Lookup Tables, Updating Lights, Updating Integrator, Updating Film, Updating Baking, Updating Device, Loading denoising kernels.

**Every render line carries `Fra:1`, two memory pairs, a `Time:` field and the scene and view-layer names.** Only one line carries a time estimate, called `Remaining`, and it is the `Sample 1/16` line. **Exactly three lines carry a sample count for a sixteen-sample frame** — 0/16, 1/16 and 16/16 — never one line per sample. **The rented Linux machine gave the same fifty lines in the same order, with nothing on standard error. The two machines agreed.**

**2. Quiet mode silences the whole render.** `blender -b -q <file>.blend -o <out> -f 1` exited 0 and wrote its picture, and the whole of standard output was one line:

```
Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:41:35)
```

The same on both machines.

**3. Cycles' own statistics go to standard output, not to standard error, and carry no sample count.** `blender -b <file>.blend -o <out> -f 1 -- --cycles-print-stats` exited 0. **Standard error was empty on both machines**; the statistics were on standard output. They arrive in two blocks, both interleaved among the render lines rather than at the start or the end: one headed `Update statistics:` during the Updating stages, one headed `Render statistics:` after the last sample line and before `Saved:`. The second block in full, from the Creator's Mac:

```
Render statistics:
Mesh statistics:
  Geometry:
    Total memory: 24.03K (24,607)
        Sphere                           23.55K (24,113)
        Cube                             413 (413)
        Area                             81 (81)
Image statistics:
  Textures:
    Total memory: 0 (0)
Kernel statistics:
  Total render time               : Total 100.00% (0.14s), Self 28.68% (0.04s)
    Ray setup                       : Total 31.62% (0.04s), Self 31.62% (0.04s)
    Intersect Closest               : Total 22.06% (0.03s), Self 22.06% (0.03s)
    Shade Surface                   : Total 13.97% (0.02s), Self 0.00% (0.00s)
      Direct Light                    : Total 8.82% (0.01s), Self 8.82% (0.01s)
      Indirect Light                  : Total 2.94% (0.00s), Self 2.94% (0.00s)
      Setup                           : Total 1.47% (0.00s), Self 1.47% (0.00s)
      Shader Evaluation               : Total 0.74% (0.00s), Self 0.74% (0.00s)
      Render Passes                   : Total 0.00% (0.00s), Self 0.00% (0.00s)
      Ambient Occlusion               : Total 0.00% (0.00s), Self 0.00% (0.00s)
    Intersect Shadow                : Total 3.68% (0.01s), Self 3.68% (0.01s)
    Intersect Volume Stack          : Total 0.00% (0.00s), Self 0.00% (0.00s)
    Intersect Blocked Light         : Total 0.00% (0.00s), Self 0.00% (0.00s)
    Intersect Subsurface            : Total 0.00% (0.00s), Self 0.00% (0.00s)
    Shade Volume                    : Total 0.00% (0.00s), Self 0.00% (0.00s)
      Setup                           : Total 0.00% (0.00s), Self 0.00% (0.00s)
      Integrate                       : Total 0.00% (0.00s), Self 0.00% (0.00s)
      Direct Light                    : Total 0.00% (0.00s), Self 0.00% (0.00s)
      Indirect Light                  : Total 0.00% (0.00s), Self 0.00% (0.00s)
    Shade Shadow                    : Total 0.00% (0.00s), Self 0.00% (0.00s)
      Setup                           : Total 0.00% (0.00s), Self 0.00% (0.00s)
      Surface                         : Total 0.00% (0.00s), Self 0.00% (0.00s)
      Volume                          : Total 0.00% (0.00s), Self 0.00% (0.00s)
      Blocked Light                   : Total 0.00% (0.00s), Self 0.00% (0.00s)
    Shade Light                     : Total 0.00% (0.00s), Self 0.00% (0.00s)
      Setup                           : Total 0.00% (0.00s), Self 0.00% (0.00s)
      Shader Evaluation               : Total 0.00% (0.00s), Self 0.00% (0.00s)
Shader statistics:
  default_surface                 : 0.02s (Relative cost: 1.00)
Object statistics:
  Sphere                          : 0.01s (Relative cost: 2.05)
  Cube                            : 0.00s (Relative cost: 0.38)
```

**Without the bare `--` separator the switch is taken as a file name and the run exits 1**, printing `Error: Cannot read file "…/--cycles-print-stats": No such file or directory`. The same on both machines.

**4. Blender 4.5 has neither `--log-level` nor `--log-list-categories`.** `--log-level info` is rejected on standard error with

```
Error: not a number '--log-level info'.
```

and the render then proceeds normally and exits 0. `--log-list-categories` is not a known argument: on the Creator's Mac it printed `unknown argument, loading as file: --log-list-categories`, then two `Error:` lines, and exited 1. On the rented Linux machine the same command, given without `-b`, did not exit at all — Blender tried to open a window, printed `GHOST: unable to initialize, exiting!` and had to be stopped by hand after twenty-one minutes; run again with `-b` it exited 1 as on the Mac. The switch 4.5 does have, `--log "*"`, added four `INFO (wm.files):` lines about the file being read and nothing else.

**5. An animation prints no announcement of any kind in Blender 4.5.** `blender -b <file>.blend -o <out> -a` over a three-frame range exited 0 and wrote three pictures. The capture holds **no line beginning `Rendering`**. Each frame is a repeat of the whole single-frame block, and the only sign of which frame it is sits inside the render lines as `Fra:1`, `Fra:2`, `Fra:3`. Each frame ends with its own `Saved:` and `Time:` pair, and the run ends with `Blender quit`. The same on both machines.

**6. The other two engines print differently.** With `-E BLENDER_EEVEE_NEXT` — which is what the engine is called in 4.5 — the whole capture on the Creator's Mac is:

```
Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:41:35)
Read blend: "…/scenes/base_4.5.blend"
Fra:1 Mem:40.70M (Peak 40.70M) | Time:00:00.29 | Rendering 1 / 64 samples
Fra:1 Mem:40.79M (Peak 40.79M) | Time:00:00.48 | Rendering 25 / 64 samples
Fra:1 Mem:40.79M (Peak 40.79M) | Time:00:00.59 | Rendering 50 / 64 samples
Fra:1 Mem:40.79M (Peak 40.79M) | Time:00:00.65 | Rendering 64 / 64 samples
Saved: '…/out/e6_4.5_BLENDER_EEVEE_NEXT_0001.png'
Time: 00:00.71 (Saving: 00:00.05)


Blender quit
```

So EEVEE's progress line reads `Rendering <n> / <total> samples`, a different shape from Cycles' `Sample <n>/<max>`, and it carries a memory figure and a time. **With `-E BLENDER_WORKBENCH` there is no progress line at all** — the run goes straight from the file-reading line to `Saved:` and `Time:`. The same on both machines.

**7. What the render-statistics handler is handed.** A script attached a handler to `bpy.app.handlers.render_stats` and wrote every call to a file. With no window the handler fired **forty-three times for one frame** — once per stage, not once per render — and was handed two arguments, a string and `None`. The first, a middle and the last, word for word:

```
STATS ('Fra:1 Mem:8.31M (Peak 9.10M) | Time:00:00.00 | Mem:0.00M, Peak:0.00M | Scene, ViewLayer | Synchronizing object | Area', None)
STATS ('Fra:1 Mem:9.45M (Peak 9.45M) | Time:00:00.00 | Remaining:00:00.01 | Mem:1.21M, Peak:1.21M | Scene, ViewLayer | Sample 1/16', None)
STATS ('Time: 00:00.14 (Saving: 00:00.04)', None)
```

**With a window open the same handler fired once**, and was handed only the closing line:

```
STATS ('Time: 00:00.13 (Saving: 00:00.01)', None)
```

The same on both machines.

**Observed again — the Cycles statistics block, and where it lands in a run (2026-08-12).** Two later hand tests repeated this capture in all four versions on both machines: one for the whole of the block, one for its position. Run on **the Creator's Mac (a MacBook Pro with an Apple M1 Pro chip, 16 GB of memory, macOS 26.5.2, Apple's own graphics and no second card)** and on **a rented Linux machine (Ubuntu 22.04.5 LTS on kernel 6.8, two NVIDIA RTX 4000 Ada Generation cards of 20 GB each on driver 580.105.08, an AMD Ryzen Threadripper PRO 3945WX with eleven cores, 49 GB of memory)**. Each version had a settings folder of its own, made before Blender was pointed at it, and built its own copy of the scene: a cube, a sphere, an area light and a camera; Cycles on the processor; 16 samples; adaptive sampling off; 160 by 120; output PNG. The command was `blender -b <file>.blend -o <out> -f 1 -- --cycles-print-stats`, standard output and standard error kept in separate files. Every run exited 0 and wrote its picture. **LABEL: observed.**

**The head of the block in Blender 4.5, from the Creator's Mac, word for word:**

```
Render statistics:
Mesh statistics:
  Geometry:
    Total memory: 24.03K (24,607)
        Sphere                           23.55K (24,113)
        Cube                             413 (413)
        Area                             81 (81)
Image statistics:
  Textures:
    Total memory: 0 (0)
Kernel statistics:
  Total render time               : Total 100.00% (0.14s), Self 13.24% (0.02s)
```

**The rented Linux machine printed the same ten lines above `Kernel statistics:`, figure for figure.** Its own kernel line read `  Total render time               : Total 100.00% (0.24s), Self 6.20% (0.01s)`. **The two machines agreed on the byte counts and differed on the times.** Everything below `Kernel statistics:` is times and percentages, and those changed from one run to the next on the same machine, so they are not set down here as fixed. The block then runs on through `Shader statistics:` and `Object statistics:`, and **no sample count appears anywhere in it**. Standard error was empty on both machines.

**The figures are this version's own.** The same command was run in the other three versions on the same two machines on the same day. Blender 5.0 and Blender 5.1 both printed `Total memory: 24.04K (24,615)`, `Sphere 23.55K (24,117)`, `Cube 417 (417)`, `Area 81 (81)`. Blender 5.2 printed `Total memory: 16.36K (16,750)`, `Sphere 16.02K (16,405)`, `Cube 289B (289)`, `Area 56B (56)`, and wrote its image section as `Image statistics:` then `  Total image memory: 0B (0)`, where this version writes `Image statistics:`, `  Textures:`, `    Total memory: 0 (0)`.

**Where the block lands, counted by line number.** The same command was run again and the captured lines numbered. On the Creator's Mac standard output held 126 lines: `Update statistics:` at line 10, `Sample 16/16` at 76, `Finished` at 77, `Render statistics:` at 78, `Saved:` at 122, `Time:` at 123. On the rented Linux machine it held 127 lines in the same order: 11, 77, 78, 79, 123 and 124. **The two machines agreed. In Blender 4.5 the block comes after the last sample line and before `Saved:`**, which is what this document already says. In 5.0, 5.1 and 5.2 the order on both machines was the other way round — `Saved:`, then `Time:`, then `Render statistics:`.

**Not covered.** This does not settle what a run prints when it fails rather than succeeds, which is a different question on the sheet. It does not settle what is printed when the output is a video file. It does not settle anything about Windows.

**Outside the interface.** Settled by the tests above, and the printed lines quoted there are the outside-the-interface case throughout — every one was captured from a terminal with no window. Two things differ from a run with a window. The progress lines are printed only when there is no window: with a window open, all that reaches the terminal is `Saved:` and `Time:`. And the render-statistics handler is handed forty-three strings with no window and one string with a window. The inferred reading of the `render_stats` documentation above — that more is printed in background mode — is borne out for this version: the strings handed over with no window carry the frame number and a memory pair, and the single string handed over with a window carries neither.

The documented sentence about where output lands still holds: on macOS, launching the application bundle the ordinary way does not give you the output, and the executable inside it must be run instead (manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/launch/macos.html, read 2026-07-30) — that is how every macOS run above was made.

**Operating system.** Settled for macOS and for Linux, and they agreed line for line: the same fifty lines in the same order, the same first and last line, the same three sample lines, the same one-line quiet mode, the same statistics on standard output with an empty standard error, the same animation with no announcement, the same EEVEE and Workbench shapes, and the same handler strings. **One difference between the machines**: given `--log-list-categories` without `-b`, Blender 4.5 on the Linux machine tried to open a window, printed `GHOST: unable to initialize, exiting!` and did not exit, where on the Mac it exited 1 straight away. **Checked again on 2026-08-12 for Cycles' statistics**: the block's byte counts and its place in the run came back the same on both machines, and only the times inside it differed.

**Windows is untested** — no Windows machine has run it — so the manual's Windows console behaviour, quoted below, stands unchecked. On Windows: "the Console Window (called the Command Prompt) is first created as a separate window on the desktop. The main Blender window will also appear and the Console Window will then be toggled off. To display the console again, go to Window ‣ Toggle System Console." (manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/launch/windows.html, read 2026-07-30).

---

## 9. What does a render report about its progress under each of the ways a render can be started?

Six ways of starting a render are each documented separately, on separate pages. No source found here sets out *the* ways a render can be started as a list, so whether these six are all of them is not established — they are the ways these sources name. What progress is reported is documented for one of them, and not in the sources for the rest.

### The ways, as the sources name them — documented

From the Render menu:

> Render Image F12
>
> Render the active scene at the current frame.
>
> Render Animation Ctrl-F12
>
> Render the animation of the active scene.

> View Render F11
>
> Show the Render window. (Press again to switch back to the main Blender window.)

Source: manual, https://docs.blender.org/manual/en/4.5/interface/window_system/topbar.html — read 2026-07-30.

From the 3D Viewport:

> To render a still image, use 3D Viewport ‣ View ‣ Viewport Render Image.

> To render an animation, use 3D Viewport ‣ View ‣ Viewport Render Animation.

> To render an animation, but only those frames that have a keyframe,
> use 3D Viewport ‣ View ‣ Viewport Render Keyframes.

Source: manual, https://docs.blender.org/manual/en/4.5/editors/3dview/viewport_render.html — read 2026-07-30.

From the command line:

> blender -b file.blend -f 10

> blender -b file.blend -a

Source: manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/render.html — read 2026-07-30, with `-b` documented as "Run in background (often used for UI-less rendering).", `-f, --render-frame <frame>` as "Render frame <frame> and save it." and `-a, --render-anim` as "Render frames from start to end (inclusive)." at https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html.

From Python:

> bpy.ops.render.render(*, animation=False, write_still=False, use_viewport=False, layer='', scene='')
>
> Render active scene

> bpy.ops.render.opengl(*, animation=False, render_keyed_only=False, sequencer=False, write_still=False, view_context=True)
>
> Take a snapshot of the active viewport

Source: Python API documentation, https://docs.blender.org/api/4.5/bpy.ops.render.html — read 2026-07-30.

And continuously in the viewport:

> – Rendered
>
> Render the 3D Viewport using the scene's Render Engine, for interactive rendering.

Source: manual, https://docs.blender.org/manual/en/4.5/editors/3dview/display/shading.html — read 2026-07-30.

### What is reported, per way

**Started from a menu or shortcut inside Blender — documented.** The Status Bar shows a progress bar for the running task, a time estimate on hover, and a cancel button. The full passage is quoted under question 7 above, from https://docs.blender.org/manual/en/4.5/interface/window_system/status_bar.html (read 2026-07-30). The manual makes no distinction between Render Image and Render Animation here; it says "the currently running task (such as rendering or baking)".

A related documented point, though it is about where the picture appears rather than about progress:

> Render In
>
> When rendering, the user interface can do any of:
>
> Keep User Interface:
>
> The user interface does not change and the render is computed in the background.
>
> Maximize Area:
>
> A new Image editor is opened as a temporary window in full screen mode.
>
> Image Editor:
>
> The area that is the largest on screen is replaced placed by a temporary Image editor.
>
> New Window:
>
> A new Image editor is opened as a regularly sized temporary window.

Source: manual, https://docs.blender.org/manual/en/4.5/editors/preferences/interface.html — read 2026-07-30.

**Started from the command line, with or without `-b` — not in the sources**, apart from two things. First, documented, that Blender's output goes to the terminal it was started from (quoted under question 8). Second, that the `render_stats` handler's description names background mode: "render/saving time plus in background mode frame/used [peak] memory" (quoted under question 8). Reading that as background mode printing more is **inferred**, as set out there, and either way it is not a statement of what progress is reported as the render proceeds.

**Started from Python — not in the sources.** `bpy.ops.render.render` is documented only by its description and its five parameters. Nothing is said about progress.

**Viewport Render — not in the sources for progress**, and the manual states a hard limit on it:

> Note
>
> Viewport rendering only works for the Workbench and EEVEE render engines.
> It's not supported for Cycles.

and

> As with a normal render, you can abort it with Esc.

Source: manual, https://docs.blender.org/manual/en/4.5/editors/3dview/viewport_render.html — read 2026-07-30. **LABEL: documented** for both quotes. Nothing is said about what it reports while running.

**Rendered viewport shading — not in the sources.** The shading page describes what it draws, not what it reports. I found no documentation of any sample counter, percentage or other progress text drawn in the viewport during interactive rendering.

**LABEL for the answer as a whole: documented** for each of the six ways separately, and for the in-interface Status Bar report; **inferred** for the six being the whole set, since no source quoted here enumerates them; **not in the sources** for everything reported under the command line, Python, Viewport Render and Rendered shading.

### Where I looked

Searched all four sources for "progress bar", "progress", "status bar", "Running Task", "render job", "background mode", "headless", "Text Info", "Statistics overlay", "sample count", "Rendering progress", "abort". The `render job` hits are three `bpy.app.handlers` entries (`render_init` "on initialization of a render job", `render_complete` "on completion of render job", `render_cancel` "on canceling a render job", https://docs.blender.org/api/4.5/bpy.app.handlers.html, read 2026-07-30) — these are hooks a script can attach to, and their descriptions say nothing about what is reported to a person. The "headless" hits are in the EEVEE limitations pages and do not concern progress. The release notes for 4.5 have no hit for "progress".

Note the limit of this record: every term above is a progress term. The ways of starting a render listed at the top of this answer were gathered page by page, and no search was made or recorded for ways this document does not name. That is why the set of six is labelled inferred rather than documented.

### Observed — what Blender 4.5 reports under each way of starting (2026-08-11)

Run in Blender 4.5.12 on **the Creator's Mac (a MacBook Pro with an Apple M1 Pro chip, 16 GB of memory, macOS 26.5.2, Apple's own graphics and no second card)** and on **a rented Linux machine (Ubuntu 22.04.5 LTS, two NVIDIA RTX 4000 Ada Generation cards of 20 GB each, eleven cores, 49 GB of memory)**, each with a settings folder of its own. The same frame from the same file — Cycles on the processor, 16 samples, adaptive sampling off, 160 by 120 — was started five ways and the terminal captured each time. **LABEL: observed.**

**The way of starting changes what is printed.** Counting the lines that carry a sample count:

| how the render was started | sample lines printed |
|---|---|
| `blender -b <file>.blend -o <out> -f 1` | 3 |
| `blender -b <file>.blend -o <out> -a` (three frames) | 3 per frame |
| `blender -b <file>.blend -o <out> --python <script>` calling `bpy.ops.render.render(write_still=True)` | **3** |
| a window open, the same call from a script | 0 |
| a window open, started the way the F12 key starts it — `bpy.ops.render.render('INVOKE_DEFAULT', write_still=True)` | 0 |

So in Blender 4.5 a render started from Python with no window prints its full progress, exactly as `-f 1` does. **With a window open, no way of starting prints any progress at all** — the terminal gets only `Saved:` and `Time:`. Both machines agreed, and the Python case was run twice, once with the call in a `--python-expr` and once in a script file, with the same result both times.

**What the operator hands back.** Called plainly it returns `{'FINISHED'}`; started the way the F12 key starts it, it returns `{'RUNNING_MODAL'}` and the render goes on afterwards. A picture was written in every case.

**What the ways of starting actually are, read from the running program.** The Python render operator in 4.5 takes **five** arguments — `animation`, `write_still`, `use_viewport`, `layer`, `scene` — and every one carries a description. The Render menu draws **two** render commands, `Render Image` and `Render Animation`, then `Render Audio...`, `View Render`, `View Animation` and `Lock Interface`. The 3D viewport's View menu draws `Viewport Render Image`, `Viewport Render Animation` and `Viewport Render Keyframes`. There is **no** `save_output` setting in this version. Identical on both machines.

**With a window open, the status bar shows one bar labelled `Render...`**, carrying a whole-number percentage and an X to cancel — the full reading is under question 7 above. A sequencer render drives the same bar, labelled `Rendering sequence...`. An animation does not name the frame and its percentage restarts at each frame.

**What the render-statistics handler is handed differs by way of starting.** With no window it fires forty-three times per frame; with a window it fires once, carrying only `Time: 00:00.13 (Saving: 00:00.01)`. The strings are quoted in full under question 8.

**A render engine's own progress calls behave the same either way.** `update_progress`, `update_stats` and `update_memory_stats`, called from a tiny engine registered from a script, all returned without complaint with a window and without, on both machines, and none produced output of its own.

**Observed — a windowed run with a render-statistics handler, and how it was closed (2026-08-12).** A later hand test repeated the windowed run in all four versions on both machines. It was asked for because no earlier record says how a windowed Blender was made to end, and the exit status of such a run belongs to whoever ended it. Run on **the Creator's Mac (a MacBook Pro with an Apple M1 Pro chip, 16 GB of memory, macOS 26.5.2, Apple's own graphics and no second card)** and on **a rented Linux machine (Ubuntu 22.04.5 LTS on kernel 6.8, two NVIDIA RTX 4000 Ada Generation cards of 20 GB each on driver 580.105.08, an AMD Ryzen Threadripper PRO 3945WX with eleven cores, 49 GB of memory), whose window was opened on its own Xorg display**. Each version had a settings folder of its own and its own copy of the scene: a cube, a sphere, an area light and a camera; Cycles on the processor; 16 samples; adaptive sampling off; 160 by 120; output PNG. `/tmp` was emptied of `*.crash.txt` before every run. A start-up script attached a handler to `bpy.app.handlers.render_stats` and another to `bpy.app.handlers.render_complete`, called `bpy.ops.render.render(write_still=True)`, printed the counts, and then closed Blender. **LABEL: observed.**

**How a windowed Blender was actually closed — the step earlier records leave out.** Two ways were tried, and the two machines behaved differently.

- **`bpy.ops.wm.quit_blender()` called straight from the start-up script.** On the rented Linux machine it did not close the run. The script printed `STILL HERE AFTER quit_blender()` and Blender was still running when a 120-second cap ended it, and again when a 240-second cap ended it. Both times the process was ended by `timeout`, whose own exit status is 124, so that status belongs to the cap and not to Blender. No crash file appeared either time. On the Creator's Mac the same call did close the run by itself, exiting 0 — but not at once: `Blender quit` came 24.7 seconds after the render in one run and 6.3 seconds after it in another.
- **A timer.** `bpy.app.timers.register` set to call `bpy.ops.wm.quit_blender()` three seconds after the render closed the run on both machines, in all four versions, every time. That is how the runs below were closed.

**Closed by the timer, Blender 4.5 exited 0 on both machines and no crash file appeared.** The whole of standard output from the Creator's Mac, with long paths shortened:

```
BACKGROUND=False
VERSION=4.5.12 LTS
Read blend: "…/scenes/base_4.5.blend"
Saved: '…/out/e21_4.5_timer_.png'
Time: 00:00.15 (Saving: 00:00.04)

HANDLER FIRED 1 TIMES
FIRST=('Time: 00:00.15 (Saving: 00:00.04)', None)
LAST=('Time: 00:00.15 (Saving: 00:00.04)', None)
RENDER_COMPLETE FIRED 1 TIMES
QUITWAY=timer
TIMER REGISTERED FOR 3s
TIMER FIRED, CALLING quit_blender()
STILL HERE AFTER quit_blender() IN TIMER
Saved session recovery to "/var/folders/n3/z3rfjt654rn1k1x6f_1gk4540000gn/T/quit.blend"
```

Everything from `BACKGROUND` to `STILL HERE` is the test script's own printing; the rest is Blender's. `Blender quit` followed, after a blank line. Standard error was empty.

**The rented Linux machine gave the same run and one different line.** It too exited 0, fired the handler once, fired `render_complete` once, and left standard error empty. Where the Mac printed the session-recovery line at quit time, the Linux machine printed

```
Error: Cannot open file /tmp/quit.blend@ for writing: Permission denied
```

and then `Blender quit`.

**The handler fired once with a window in this version, on both machines**, which is what this document already says, and the single string it was handed was the closing `Time:` line. **No run of Blender 4.5 on either machine ended in a segmentation fault, and no crash file was written anywhere in `/tmp`.**

**Not covered.** This does not settle what the time estimate says when the pointer rests on the progress bar — that needs a hover, and neither machine has any way of moving the pointer without a hand on it. It does not settle which key is bound to which render command: with no window the key configuration holds no items, so that could not be read. It does not settle what a viewport render reports, nor what the Rendered viewport shading mode reports. It does not settle anything about Windows, or about Blender loaded as a Python module, which neither machine has.

**Outside the interface.** Settled, and it is the sharpest difference in this answer. With no window a render started from the command line **or from Python** prints its full progress — the stage lines, three sample lines, a memory figure and a `Remaining:` estimate. With a window open none of that reaches the terminal in any way of starting, and what stands in for it is the status bar's one labelled bar with its whole-number percentage. The `render_stats` handler is handed forty-three strings with no window and one with a window. Both machines agreed. **Added 2026-08-12:** a windowed run started from a terminal, with the handler attached, ended cleanly in this version on both machines — exit 0, no crash file — when it was closed by a timer calling `bpy.ops.wm.quit_blender()`. How such a run is closed is itself part of the answer: the same call made straight from the start-up script did not close a windowed Blender on the Linux machine at all.

**Operating system.** Settled for macOS and for Linux, and they agreed on every point above: the same counts of printed sample lines for all five ways, the same operator arguments and menu entries, and the same `Render...` label and whole-number percentage on each machine's own screen. **One difference between the machines, found on 2026-08-12**: `bpy.ops.wm.quit_blender()` called from a start-up script closed a windowed Blender on the Creator's Mac, after six seconds in one run and twenty-five in another, and did not close it on the rented Linux machine within four minutes. A second, smaller difference: at quit time the Mac printed `Saved session recovery to "/var/folders/…/T/quit.blend"` and the Linux machine printed `Error: Cannot open file /tmp/quit.blend@ for writing: Permission denied`. Closed by a timer, both machines exited 0. **Windows is untested**, no Windows machine having run it.

---

## 10. What does a render do when the machine's own memory runs out while it is sampling?

**LABEL: not in the sources.**

Nothing in the manual, the API documentation, the release notes or Cycles' own files describes what a render does when the machine's own memory is exhausted during sampling. There is no statement that it stops, recovers, spills to disk, reports an error, or ends the process.

### What the sources do say nearby, and why none of it answers the question

Running out of memory is named as a cause of Blender crashing, but with no timing and no mention of rendering:

> The most common causes of Blender crashes:
>
> Running out of memory.
>
> Issues with graphics hardware or drivers.
>
> Bugs in Blender.

Source: manual, https://docs.blender.org/manual/en/4.5/troubleshooting/crash.html — read 2026-07-30. **LABEL: documented** that this is listed as a common cause of crashes. It says nothing about a render, and nothing about sampling.

The API documentation says the same in the context of scripts:

> Be aware of memory limits,
> especially when working with large lists since Blender can crash simply by running out of memory.

Source: Python API documentation, https://docs.blender.org/api/4.5/info_gotchas_crashes.html — read 2026-07-30. This is about Python scripts building large lists, not about a render sampling.

Blender has a report category for the condition, so the condition is at least named:

> ERROR_OUT_OF_MEMORY:
>
> Out of Memory.

Source: Python API documentation, https://docs.blender.org/api/4.5/bpy_types_enum_items/wm_report_items.html — read 2026-07-30. **LABEL: documented** that the category exists. That a render emits it when the machine's memory runs out is *not* stated, and I am not claiming it.

The one "Error: Out of memory" the manual explains is about the graphics card, not the machine:

> Error: Out of memory
>
> This usually means there is not enough memory to store the scene for use by the GPU.

and, in the same page:

> With CUDA, OptiX, HIP and Metal devices, if the GPU memory is full Blender will automatically
> try to use system memory. This has a performance impact, but will usually still result in a faster render
> than using CPU rendering.

Source: manual, https://docs.blender.org/manual/en/4.5/render/cycles/gpu_rendering.html — read 2026-07-30. **LABEL: documented.** Note what this says and does not say: it describes the graphics card running out and system memory being used instead. It does not describe system memory itself running out.

Everything else I found on memory in a render is prevention, not behaviour on exhaustion: the Performance presets ("Lower Memory: Uses settings to decrease memory considered at the cost of slower renders."), tiling ("Render high resolution images in tiles to reduce memory usage. Tiles are cached to disk while rendering to save memory."), and Use Compact BVH ("Use a more compact BVH structure, which can reduce RAM usage but render slower.") — all from https://docs.blender.org/manual/en/4.5/render/cycles/render_settings/performance.html, read 2026-07-30. And the Status Bar can display a figure: "System Memory — Shows an estimate of Blender's RAM consumption. On a single-instance single-machine scenario, this estimate provides a measurement against the hardware limit of the machine." (https://docs.blender.org/manual/en/4.5/interface/window_system/status_bar.html, read 2026-07-30). Showing a number is not the same as saying what happens when the number reaches the limit.

### Where I looked

Searched the whole of `manual_4.5`, `api_4.5`, `release_notes_4.5` and `cycles_4.5` for: "out of memory", "ran out of", "not enough memory", "insufficient memory", "memory limit", "memory error", "swap", "host memory", "system memory", "virtual memory", "paging", "allocation fail", "malloc", "killed", "terminated", "OOM". Files ruled out after reading: `manual_4.5/troubleshooting/crash.txt` (crashes in general, with recovery and reporting advice, no render behaviour), `manual_4.5/render/cycles/gpu_rendering.txt` (graphics-card memory only), `api_4.5/info_gotchas_crashes.txt` (Python scripting), `manual_4.5/editors/preferences/system.txt` ("Memory & Limits" is undo steps and an undo memory limit), `manual_4.5/modeling/modifiers/deform/mesh_deform.txt` (a modifier's memory note). "ran out of", "host memory", "virtual memory", "allocation fail", "malloc", "killed" and "OOM" returned no hits anywhere. The 4.5 release notes' only memory-plus-crash hits are bug-fix list entries about Vulkan in the viewport, not about renders running out of system memory.

### Observed — Blender 4.5 fills the machine and is killed, saying nothing (2026-08-11)

Run on **a rented Linux machine only — Ubuntu 22.04.5 LTS on kernel 6.8, two NVIDIA RTX 4000 Ada Generation cards of 20 GB each on driver 580.105.08, an AMD Ryzen Threadripper PRO 3945WX with eleven cores, 49 GB of memory**. It was **not** run on the Creator's Mac: that is their working computer and its memory is not to be filled on purpose. So this answer covers Linux and leaves macOS untested. **LABEL: observed.**

**The scene.** A sphere, a cube, an area light and a camera; Cycles on the processor; 2000 samples; adaptive sampling off; 8192 by 8192 pixels; tile size 8192; every render pass switched on, cryptomatte at sixteen levels, and forty light groups added. Rendered with `blender -b <file>.blend -o <out> -f 1`, with the machine's memory read once a second alongside.

**Getting the memory to run out while sampling took three attempts, and the first two are results of their own.** More pixels do not do it: at 12288 by 12288 the run reached 48.8 GB of the machine's 49 and **kept sampling**, and at 16384 by 16384 Blender's own figure was byte for byte the same, `Mem:45833.29M`. Tiling caps what is held — the tile size cannot exceed 8192 — so the buffers stop growing with the picture. What did it was more passes.

**What happened.** The last line on standard output is a sampling line:

```
Fra:1 Mem:9.20M (Peak 9.20M) | Time:00:00.00 | Mem:0.55M, Peak:0.55M | Scene, ViewLayer | Sample 0/2000
```

**Standard error was empty. Blender printed nothing whatever about memory.** No picture was written and no crash file appeared. The process ended with status **137**. The machine's own memory use reached 50,038 MB and, in a repeat run, 47,346 MB, of 49,152. The kernel log held:

```
Out of memory: Killed process 104632 (blender) total-vm:102079912kB, anon-rss:48492704kB, file-rss:3204kB, shmem-rss:0kB, UID:0 pgtables:958
```

The whole test was run twice, the second time with the four versions taken in the opposite order, and Blender 4.5 behaved the same way both times.

**Not covered.** This does not settle what happens on macOS or on Windows. It does not settle whether the answer differs when the render is on a graphics card rather than the processor. It does not settle what a window-open run would show, since this was run with no window.

**Outside the interface.** Settled for a run with no window, which is what was tested: the process is killed with status 137, nothing is printed, and nothing is written. What a run with a window would show was not tested.

**Operating system.** Settled for Linux only, and macOS is untested by the Creator's own instruction — their working machine's memory is not to be filled on purpose. Windows is untested, no Windows machine having run it. What the manual says about *watching* memory stands unchecked for the two untested systems: "On Windows, use Task Manager and sort by Memory. On macOS, use Activity Monitor.app and open Memory tab. Alternatively, run top -o MEM. On Linux, run top -o %MEM." (manual, https://docs.blender.org/manual/en/4.5/troubleshooting/crash.html, read 2026-07-30).

---

## 11. What, if anything, does a render print that shows whether a frame took the number of samples it was asked for?

**LABEL: not in the sources** for anything printed.

I found no documented printed line, message, log entry or piece of file metadata that states how many samples a frame actually took, or that compares it against the number asked for.

What exists instead is a *picture*, not a print — a render pass — and it is off by default. And the sources document three separate reasons the count can fall short, without documenting any announcement when it does.

### The nearest thing: a render pass, which is image data rather than printed text

> Sample Count Cycles
>
> Number of samples calculated for each pixel, divided by the maximum number of samples.
> Used to analyze adaptive sampling.

Source: manual, https://docs.blender.org/manual/en/4.5/render/layers/passes.html — read 2026-07-30. **LABEL: documented.**

The same thing in Cycles' own files:

> ```
> pass_debug_sample_count: BoolProperty(
>     name="Debug Sample Count",
>     description="Number of samples per pixel taken, divided by the maximum number of samples. To analyze adaptive sampling",
>     default=False,
>     update=update_render_passes,
> )
> ```

Source: `Blender Sources/cycles_4.5/properties.py`, lines 1496–1501. **LABEL: observed** — the setting exists under that name and description, and its default is off. Read 2026-07-30.

And where it is turned into a pass:

> ```
> # Debug passes.
> if crl.pass_debug_sample_count:
>     yield ("Debug Sample Count", "X", 'VALUE')
> ```

Source: `Blender Sources/cycles_4.5/engine.py`, lines 216–218. **LABEL: observed** — when the setting is on, a single-channel pass named "Debug Sample Count" is registered. Read 2026-07-30.

That the pixel counts in that pass are the *actual* counts, rather than the requested ones, is stated in a comment in Cycles' kernel code:

> ```
> /* Count the sample and get an effective sample for this pixel.
>  *
>  * This logic allows to both count actual number of samples per pixel, and to add samples to this
>  * pixel after it was converged and samples were added somewhere else (in which case the
>  * `scheduled_sample` will be different from actual number of samples in this pixel). */
> ```

Source: `Blender Sources/cycles_4.5/source/kernel/integrator/init_from_camera.h`, lines 74–78 (inside `integrator_init_from_camera`). **LABEL: observed** for what the file says. Read 2026-07-30.

One line of care here, because it is easy to cross. The above is what the files *say*. Whether an actual render, with that pass enabled, produces a picture from which you could read off "this frame took fewer samples than asked" is a step beyond the text, and that step is **inferred**, not observed. The step: the pass stores per-pixel actual sample count divided by the maximum, so a value below 1 in that pass would mean that pixel took fewer than the maximum. I have not run a render to confirm it, and the pass is per pixel, not per frame — so even granted the step, it does not directly answer "did the *frame* take the number of samples it was asked for".

### Three documented reasons the count can fall short — none of them with an announcement

Adaptive sampling, which is on by default:

> If the Noise Threshold checkbox is enabled, Cycles will use adaptive sampling,
> cutting short the sampling process in areas that have become less noisy than the
> specified threshold value

Source: manual, https://docs.blender.org/manual/en/4.5/render/cycles/render_settings/sampling.html — read 2026-07-30. **LABEL: documented.**

That it is on by default, from Cycles' own files:

> ```
> use_adaptive_sampling: BoolProperty(
>     name="Use Adaptive Sampling",
>     description="Automatically reduce the number of samples per pixel based on estimated noise level",
>     default=True,
> )
> ```

Source: `Blender Sources/cycles_4.5/properties.py`, lines 569–573. **LABEL: observed.** Read 2026-07-30.

The time limit:

> Time Limit
>
> Stops rendering if the time exceeds the limit, even if the desired sample count
> hasn't been reached yet. Setting this value to zero disables the limit.

Source: manual, https://docs.blender.org/manual/en/4.5/render/cycles/render_settings/sampling.html — read 2026-07-30. **LABEL: documented.** This one names the exact situation the question is about — a frame not reaching its sample count — and says nothing whatever about reporting it.

The sample subset, which deliberately renders fewer than the maximum:

> ```
> use_sample_subset: BoolProperty(
>     name="Use Sample Subset",
>     description="Render a subset of the specified max samples. Typically used for distributed rendering across multiple devices",
>     default=False,
> )
> ```

Source: `Blender Sources/cycles_4.5/properties.py`, lines 497–501. **LABEL: observed.** Read 2026-07-30. The manual adds: "Length — The number of samples in the subset. While this overrides (Max) Samples in terms of the samples that will get rendered, it's still important to set (Max) Samples to the total number of samples that will be rendered across all subsets" (same sampling page, read 2026-07-30).

### What is printed or stamped, and what it does not include

The statistics switch covers memory and time only, not samples: "--cycles-print-stats — Log statistics about render memory and time usage." (manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html, read 2026-07-30).

The render statistics handler describes its content as "render/saving time plus in background mode frame/used [peak] memory" (Python API documentation, https://docs.blender.org/api/4.5/bpy.app.handlers.html, read 2026-07-30). No samples.

The metadata panel offers a list of items, and samples is not among them. "Fixed" would be a step too far: what is quoted below is what the panel offers and what the API exposes, not a statement that nothing else is ever written into an image. The manual's Metadata panel lists: Date, Time, Render Time, Frame, Frame Range, Memory, Hostname, Camera, Lens, Scene, Marker, Filename, Strip Name, Note (https://docs.blender.org/manual/en/4.5/render/output/properties/metadata.html, read 2026-07-30). The API's matching properties are `use_stamp`, `use_stamp_camera`, `use_stamp_date`, `use_stamp_filename`, `use_stamp_frame`, `use_stamp_frame_range`, `use_stamp_hostname`, `use_stamp_labels`, `use_stamp_lens`, `use_stamp_marker`, `use_stamp_memory`, `use_stamp_note`, `use_stamp_render_time`, `use_stamp_scene`, `use_stamp_sequencer_strip`, `use_stamp_time` — with no samples entry (https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html, read 2026-07-30). Two of them are on by default: "use_stamp_memory — Include the peak memory usage in image metadata — boolean, default True" and "use_stamp_render_time — Include the render time in image metadata — boolean, default True". **LABEL: documented** for what the panel lists, for the matching properties, and for those two defaults. That no samples figure reaches an image by some route these two lists do not cover is **not** established here.

### Where I looked

Searched all four sources for: "samples taken", "actual samples", "actual number of samples", "sample count", "Sample 1/", "Path Tracing Sample", "converged", "noise threshold reached", "render statistics", "print statistics", "Fra:", "Mem:", "use_stamp". The hits are all quoted or listed above. "samples taken", "Sample 1/", "Path Tracing Sample", "noise threshold reached", "Fra:" and "Mem:" returned no hits at all. "converged" hits are Cycles kernel headers (`film/adaptive_sampling.h`, `integrator/init_from_camera.h`, `integrator/init_from_bake.h`) and one unrelated stereoscopy page; the kernel files show the per-pixel convergence test and the per-pixel sample counter but contain no printing. The 4.5 release notes have no hit bearing on sample reporting.

### Observed — what Blender 4.5 prints and writes about samples (2026-08-11)

Run in Blender 4.5.12 on **the Creator's Mac (a MacBook Pro with an Apple M1 Pro chip, 16 GB of memory, macOS 26.5.2, Apple's own graphics and no second card)** and on **a rented Linux machine (Ubuntu 22.04.5 LTS, two NVIDIA RTX 4000 Ada Generation cards of 20 GB each, eleven cores, 49 GB of memory)**, each with a settings folder of its own. **LABEL: observed.**

**1. A printed line does show it, and the mark is not the one a reader would expect.** The scene — a cube, a sphere, an area light and a camera; Cycles on the processor; 4096 samples; adaptive sampling off; 480 by 360 — was rendered three ways from a terminal with no window: plain, with a five-second time limit, and with adaptive sampling on at a threshold of 0.1. **All three exited 0 and wrote their picture.**

A frame that finished ends on its full count. The whole of the sample lines from the plain run on the Creator's Mac:

```
Fra:1 Mem:8.34M (Peak 9.09M) | Time:00:00.00 | Mem:0.19M, Peak:0.19M | Scene, ViewLayer | Sample 0/4096
Fra:1 Mem:17.65M (Peak 17.65M) | Time:00:00.01 | Remaining:00:37.91 | Mem:9.41M, Peak:9.41M | Scene, ViewLayer | Sample 1/4096
Fra:1 Mem:22.93M (Peak 30.84M) | Time:00:37.91 | Mem:9.41M, Peak:9.41M | Scene, ViewLayer | Sample 4096/4096
```

A frame cut short by the time limit shows the count it reached and then **resets to zero**. The whole of the sample lines, same machine:

```
Fra:1 Mem:8.34M (Peak 9.09M) | Time:00:00.00 | Mem:0.19M, Peak:0.19M | Scene, ViewLayer | Sample 0/4096
Fra:1 Mem:17.65M (Peak 17.65M) | Time:00:00.01 | Remaining:00:04.99 | Mem:9.41M, Peak:9.41M | Scene, ViewLayer | Sample 1/4096
Fra:1 Mem:17.65M (Peak 17.65M) | Time:00:04.67 | Remaining:00:00.32 | Mem:9.41M, Peak:9.41M | Scene, ViewLayer | Sample 513/4096
Fra:1 Mem:17.65M (Peak 17.65M) | Time:00:05.00 | Mem:9.41M, Peak:9.41M | Scene, ViewLayer | Sample 549/4096
Fra:1 Mem:22.93M (Peak 30.84M) | Time:00:05.28 | Mem:9.41M, Peak:9.41M | Scene, ViewLayer | Sample 0/4096
```

A frame cut short by adaptive sampling does the same, climbing to `Sample 160/4096` and then ending on `Sample 0/4096`. **Nothing else is printed** — no notice, no warning, nothing naming the time limit or the noise threshold. So the only printed mark of a frame that stopped short is that its **last** sample line reads `Sample 0/<maximum>`, where a completed frame's last line reads `Sample <maximum>/<maximum>`, and the count actually reached sits on the line before it. On the rented Linux machine the same three runs behaved identically, reaching 809/4096 under the time limit and 176/4096 under adaptive sampling before ending on `Sample 0/4096`. **The two machines agreed on the shape and differed only in the counts reached, as two different processors must.**

**2. A saved image carries a sample count, and the manual's Metadata panel does not list it.** A frame was rendered to PNG with the stamp settings untouched, and the fields were read back by a plain Python script rather than by Blender. All eleven fields:

```
File = …/scenes/base_4.5.blend
Date = 2026/08/11 21:53:42
Time = 00:00:00:01
Frame = 1
Camera = Camera
Scene = Scene
RenderTime = 00:00.21
cycles.ViewLayer.samples = 16
cycles.ViewLayer.total_time = 00:00.18
cycles.ViewLayer.render_time = 00:00.17
cycles.ViewLayer.synchronization_time = 00:00.01
```

The same eleven came back on both machines, and the same fields appear in an OpenEXR file. **The sample figure is the maximum asked for, not the number reached**: a run whose sample lines showed 160 of 512 reached carried `cycles.ViewLayer.samples (string) = 512`.

**3. The Sample Count pass is written into a multi-layer OpenEXR in this version, as one channel named `X`.** With `pass_debug_sample_count` on, adaptive sampling on at 0.1 and a maximum of 512 samples, the saved file's channel list — read without Blender — is:

```
ViewLayer.Combined.A, ViewLayer.Combined.B, ViewLayer.Combined.G, ViewLayer.Combined.R,
ViewLayer.Debug Sample Count.X,
ViewLayer.Noisy Image.A, ViewLayer.Noisy Image.B, ViewLayer.Noisy Image.G, ViewLayer.Noisy Image.R
```

**Its values do fall below one.** The same render saved with no compression, so the raw floats could be read directly, gave:

```
DEBUG SAMPLE COUNT PASS: min 0.300000  max 1.000000  avg 0.300453  pixels 172800
  how many pixels below 1.0: 172791 of 172800
```

That run's own sample lines ended `Sample 160/512` and then `Sample 0/512`. Nothing was printed about the pass during the render. The same on both machines.

**4. Cycles' own statistics carry no sample count.** With `-- --cycles-print-stats` the two printed blocks — quoted in full under question 8 — hold memory and time only, on both machines.

**5. The render-statistics handler carries a sample count only while a sampling stage is running.** With no window the handler was handed forty-three strings; the sampling ones read like `STATS ('Fra:1 Mem:9.45M (Peak 9.45M) | Time:00:00.00 | Remaining:00:00.01 | Mem:1.21M, Peak:1.21M | Scene, ViewLayer | Sample 1/16', None)`. With a window it was handed one string, `STATS ('Time: 00:00.13 (Saving: 00:00.01)', None)`, carrying no sample count at all. The same on both machines.

**Not covered.** This does not settle what any logging category emits about samples, since Blender 4.5 has no way of listing its categories. It does not settle anything about Windows.

**Outside the interface.** Settled, and it differs. With no window the sample count is printed, in three lines for a completed frame and in a run that ends on `Sample 0/<maximum>` for a frame that stopped short. With a window open **no sample line reaches the terminal at all**, and the render-statistics handler is handed one string carrying no sample count. The Sample Count pass and the image metadata are written either way — both were produced by runs with no window. Both machines agreed.

**Operating system.** Settled for macOS and for Linux, and they agreed on every point: the same printed pattern, the same eleven metadata fields, the same pass written with the same channel name, and the same absence of a sample count from Cycles' statistics. **Windows is untested**, no Windows machine having run it.

---

## Things only a running Blender can settle

The six items that stood here have all been run, on both machines, and their results are written into the answers above. What is left open is below.

1. **The time estimate that appears when the pointer rests on the progress bar (questions 7 and 9).** It needs a hover. Neither machine has any way of moving the pointer without a hand on it — the rented Linux machine has no `xdotool` or equivalent installed — so this was dropped rather than run.

2. **Whether a render at the very top of the size range finishes and writes a file (question 7).** At 65536 by 65536 the render starts, divides into 1024 tiles and climbs steadily with the machine's memory barely touched, but the runs were stopped before the end. Whether such a render completes, and what it writes, is unsettled.

3. **What a render does when the machine's memory runs out, on macOS (question 10).** Answered on Linux only. The Creator's Mac is their working computer and its memory is not to be filled on purpose.

4. **Everything on Windows (questions 7 to 11).** No Windows machine has run it, so every one of these answers covers macOS and Linux and leaves Windows open.

5. **What a render started from Blender loaded as a Python module reports (question 9).** Neither machine has such a build.

6. **Which key is bound to which render command (question 9).** With no window Blender's key configuration holds no items, so it could not be read that way.

---

2026-08-11 — Claude Opus 5 (Claude Code) — Reports, Blender 4.5

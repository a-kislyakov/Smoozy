# Reports — cross-sweep questions (Blender 5.1)

Questions answered: 7, 8, 9, 10, 11.

**Sources consulted, and when.** Blender 5.1 manual (`Blender Sources/manual_5.1`, including three pictures from its `_images` folder), Blender 5.1 Python API documentation (`Blender Sources/api_5.1`), Blender 5.1 release notes (`Blender Sources/release_notes_5.1`), and Cycles' own files (`Blender Sources/cycles_5.1`). All were taken on 2026-07-30, and that is the date read for everything quoted below.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

A general note that applies to all five answers: the Cycles files supplied here are the add-on's Python files plus the render kernel headers. They contain no Cycles session, progress, or logging code. So nothing about what Cycles prints while it runs can be observed from them. Where I say a thing is not in the sources, that absence covers the Cycles files too.

---

## 7. What does a render report about its own progress, across the whole range of picture sizes a render will accept?

**LABEL: documented** for the range of picture sizes a render will accept, and for the general shape of what a render reports about progress. **not in the sources** for any link between the two — nothing says progress reporting changes, or stays the same, as the picture gets bigger or smaller.

**Ambiguity.** I read "picture sizes" as the pixel size of the rendered image — its width and height. That is the reading I answer. The other reading would be the size of the output file on disk, which is a different subject. No search was made for that reading, so nothing here covers it.

### The range of picture sizes a render will accept

Exact words, Python API documentation, `bpy.types.RenderSettings`:

> resolution_x
>
> Number of horizontal pixels in the rendered image (in [4, 65536], default 1920)

> resolution_y
>
> Number of vertical pixels in the rendered image (in [4, 65536], default 1080)

> resolution_percentage
>
> Percentage scale for render resolution (in [1, 32767], default 100)

Address: https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html — read 2026-07-30.

The manual describes the same two settings in words:

> Resolution X, Y
>
> The number of pixels horizontally and vertically in the image.
>
> Percentage
>
> Slider to reduce or increase the size of the rendered image relative to the Resolution values.

Source: manual, https://docs.blender.org/manual/en/5.1/render/output/properties/format.html — read 2026-07-30.

The two combine. The API documentation's own example render engine shows how:

> scale = scene.render.resolution_percentage / 100.0
> self.size_x = int(scene.render.resolution_x * scale)
> self.size_y = int(scene.render.resolution_y * scale)

Source: Python API documentation, https://docs.blender.org/api/5.1/bpy.types.RenderEngine.html — read 2026-07-30.

That the percentage scales the rendered image relative to the resolution values is **documented**, by the manual words above. That the size a render actually produces is the resolution multiplied by the percentage, cut down to a whole number, is a step past both quotes — the lines above come from an example engine written for the documentation, not from a statement about Blender's own engines — so **inferred**. Whether the product is clamped at either end is **not in the sources**.

### What a render reports about progress

Two things, and both are stated without reference to size.

From the manual, on the Status Bar:

> Running Task
>
> Shows the progress of the currently running task (such as rendering or baking).
> Hovering the mouse pointer over the progress bar will display a time estimate.
> The task can be aborted by clicking the cancel button ().

Source: manual, https://docs.blender.org/manual/en/5.1/interface/window_system/status_bar.html — read 2026-07-30.

The picture on that page, `_images/interface_window-system_status-bar_ui-center.png`, shows one progress bar. **Documented**, from the picture: it carries a film-camera icon, the label "Sequence Render", a filled bar reading "94%", and a cross-shaped cancel button. So that one bar, in that one picture, showed a whole-number percentage beside a label naming the task. Reading from it that every progress bar does the same is a step past a single screenshot, so **inferred**. Anything about how often the number changes, or what it counts, is not visible in it at all.

From the Python API documentation, on `bpy.types.RenderEngine`:

> update_progress(progress)
>
> Update progress percentage of render
>
> Parameters:
>
> progress (float) – Percentage of render that's done (in [0, 1])

> update_stats(stats, info)
>
> Update and signal to redraw render status text
>
> Parameters:
>
> stats (str) – Stats, (never None)
>
> info (str) – Info, (never None)

Source: https://docs.blender.org/api/5.1/bpy.types.RenderEngine.html — read 2026-07-30.

So the reported progress is a single fraction between 0 and 1, plus two free-form pieces of text the engine supplies. Neither is documented as depending on picture size.

### The one size-related setting the sources describe, which is not progress

The manual ties picture size to one Cycles setting:

> Tile Size
>
> Render high resolution images in tiles of this size, to reduce memory usage.
> Tiles are cached to disk while rendering to save memory.

Source: manual, https://docs.blender.org/manual/en/5.1/render/cycles/render_settings/performance.html — read 2026-07-30.

**Observed**, from Cycles' own files. In `cycles_5.1/properties.py`, in the `CyclesRenderSettings` class (lines 1051–1062):

```
    use_auto_tile: BoolProperty(
        name="Auto Tile",
        description="Deprecated, tiling is always enabled",
        default=True,
    )
    tile_size: IntProperty(
        name="Tile Size",
        default=2048,
        description="Render high resolution images in tiles of this size, to reduce memory usage. Tiles are cached to disk while rendering to save memory",
        min=8,
        max=8192,
    )
```

What the file says: the setting exists, is a whole number, defaults to 2048, and is limited to 8–8192; and the older switch that turned tiling on and off now describes itself as deprecated because "tiling is always enabled". What a render then *does* — that a picture wider or taller than the tile size is split into more than one tile — is a step beyond the file, so **inferred**. Whether any of that shows up in what the render reports about its progress is **not in the sources**.

**Where I looked, for the silence.** I searched the whole 5.1 manual, the whole 5.1 API documentation, all 5.1 release notes, and all Cycles files for: `progress`, `progress bar`, `percent`, `percentage`, `Remaining`, `Time Remaining`, `time estimate`, `ETA`, `Elapsed`, `Fra:`, `Mem:`, `Peak`, `stats`, `statistics`, `Sample `, `Rendered `, `Path Tracing`, `Synchronizing`, `Updating Scene`, `Loading render kernels`, `Initializing`, `tile`, `65536`, `32767`. The only manual pages that say anything about progress at all are the Status Bar page quoted above and a Video Sequencer proxy note. No page in the manual, and no page in the API documentation, mentions resolution alongside progress reporting. The release notes for 5.1 say nothing about progress reporting; the only progress mentions there are two bug fixes and one sequencer note, none about render size.

### Observed — what a running Blender 5.1 does (2026-08-11)

Every test below was run in Blender 5.1.2 on both machines this project can reach: **the Creator's Mac — a MacBook Pro with an Apple M1 Pro chip, 16 GB of memory, macOS 26.5.2, Apple's own graphics and no second card** — and **a rented Linux machine — Ubuntu 22.04.5 LTS on kernel 6.8, two NVIDIA RTX 4000 Ada Generation cards of 20 GB each on driver 580.105.08, an AMD Ryzen Threadripper PRO 3945WX with eleven cores, 49 GB of memory**. Each run had a settings folder of its own, so nothing touched the Creator's own Blender settings. **LABEL: observed** throughout.

**The scene.** Built by Blender 5.1 itself: a cube, a sphere, an area light and a camera; Cycles on the processor; 16 samples; adaptive sampling off; time limit 0; 160 by 120 pixels; output PNG.

**1. Tiled rendering does report per-tile progress, and the threshold is one tile's worth of area.** The same frame was rendered from a terminal with no window six times, changing only the two resolution numbers, at 4 by 4, 160 by 120, 1920 by 1080, 2048 by 2048, 2049 by 2049 and 4096 by 4096. All six exited 0 and wrote a picture of the size asked for.

At 4 by 4, 160 by 120, 1920 by 1080 and 2048 by 2048 the capture holds **three** lines carrying a sample count and mentions no tile. At 2049 by 2049 and at 4096 by 4096 it holds **thirteen**, and every one mentions a tile. Two of the thirteen from the 2049 by 2049 run on the Creator's Mac:

```
00:00.702  render           | Fra: 1 | Remaining: 00:02.99 | Mem: 241M | Rendered 0/4 Tiles, Sample 1/16
00:04.055  render           | Fra: 1 | Mem: 1M | Rendered 3/4 Tiles, Sample 16/16
```

2048 by 2048 is exactly the default tile size squared and printed no tile; 2049 by 2049 printed four. **This is the threshold the interface code quoted above works out, and it holds when run.** The rented Linux machine gave the same split at the same six sizes, and the same tile counts. **The two machines agreed.**

**2. The settings clamp silently, and never refuse.** Set from Python and read straight back: 0, 3 and 4 all become **4** for the two size settings; 65536, 65537 and 70000 all become **65536**; for the percentage 0 becomes **1** and 32768 and 70000 become **32767**. Nothing raised an error and nothing was printed. Identical on both machines.

**3. The effective size after the percentage is cut down, not rounded, and it is not clamped upward.** At 1920 by 1080 the written picture is **1920 x 1080** at 100 per cent, **960 x 540** at 50 per cent and **633 x 356** at 33 per cent — 633 is the product cut down, where rounding would give 634. Identical on both machines. **A picture of no pixels is refused, and the run still exits 0**: at 4 by 4 with the percentage at 1 nothing was written, and the message read

```
00:00.264  reports          | ERROR Image too small
```

**4. A render at 4 by 4 finishes and writes a 4 by 4 picture**, on both machines. **A render at the top of the range starts, tiles, and then fails to save.** At 65536 by 65536 with one sample per pixel, on the rented Linux machine, the render divided into **1024 tiles** and printed a line for each — 1026 tile lines — reaching `Rendered 1023/1024 Tiles, Sample 0/1`, with the machine's own memory use at about 3.5 GB of 49. It then stopped at the point of putting the tiles together, printing `04:28.859  reports          | ERROR Error reading tiles from file` on standard output and, on standard error:

```
OpenImageIO exited with a pending error message that was never
retrieved via OIIO::geterror(). This was the error message:
Uncompressed image size 245760.0 MB exceeds the 81920 MB limit.
Image claimed to be 65536x65536, 15-channel float. Possible corrupt input?
If this is a valid file, raise the OIIO attribute "limits:imagesize_MB".
```

The run **exited 1 and wrote no file**. All four versions did exactly the same. So a size the settings accept is not necessarily a size a render can finish. It was not run on the Creator's Mac, whose 16 GB is not to be filled on purpose.

**5. The status bar during a render, with a window open.** A render was started the way the F12 key starts it and the whole Blender window photographed with Blender's own screenshot command every few seconds. There is **exactly one bar**, never two. Beside it, in order: a small camera icon, the words **`Rendering...`**, the bar with a **whole-number percentage** written inside it, and an **X** at the right end. No decimal point appeared in any photograph. Mid-render readings: `Rendering...` beside `23%` on the Creator's Mac, `Rendering...` beside `26%` on the rented Linux machine. **No frame is named**, for a still or for an animation, and during a three-frame animation the percentage restarted at each frame rather than climbing across the range. Picture size made no difference — the same one bar and whole-number percentage at 4 by 4 and at 4096 by 4096, so the report does **not** change shape when the render is tiled. Both machines agreed.

**6. A sequencer render drives the same bar, and it is labelled `Rendering sequence...`** — not `Sequence Render`, which is the label in the manual's one picture. Caught mid-render at `0%` and `16%`, on both machines.

**7. The two tiling settings, read from the running program.** `use_auto_tile` is named **Auto Tile** and described **"Deprecated, tiling is always enabled"**, default True. `tile_size` is described "Render high resolution images in tiles of this size, to reduce memory usage. Tiles are cached to disk while rendering to save memory", default 2048, range `[8, 8192]`. The three size settings: `resolution_x` and `resolution_y` default 1920 and 1080, range `[4, 65536]`; `resolution_percentage` is named **Resolution Scale**, default 100, range `[1, 32767]`. Identical on both machines.

**8. Searching Blender 5.1's own installed Cycles add-on** for the words "May work poorly with render tiling" and for a comment naming `compute_render_tile_size` found **both**, on both machines. Blender 4.5's add-on carries neither.

**9. A render engine's own progress calls work with no window.** A tiny render engine registered from a script called `update_progress`, `update_stats` and `update_memory_stats`. All three returned without complaint, with a window and without, on both machines, and none produced any output of its own.

**Not covered.** This does not settle what the time estimate says when the pointer rests on the progress bar, which needs a hover. It does not settle whether the limits are enforced differently by other render engines. It does not settle whether a 65536 by 65536 render would save into a format OpenImageIO's size limit allows, or with that limit raised. It does not settle anything about Windows.

**Outside the interface.** Settled, and it differs. With no window there is no status bar and no percentage; what a render reports instead is the printed lines quoted above, which carry a sample count, a memory figure and a `Remaining:` time estimate, and which gain a tile count once the picture crosses one tile's area. So the step marked inferred above is borne out — no bar exists to photograph — and what stands in for it is now on record. The size range is the same either way, the same clamping figures coming back from a windowless run. Both machines agreed.

**Operating system.** Settled for macOS and for Linux, and they agreed on every point above: the same three-or-thirteen sample lines at the same sizes, the same tile counts, the same clamping, the same written picture sizes, the same `ERROR Image too small`, the same `Rendering...` label and whole-number percentage, and the same settings. **Windows is untested** — no Windows machine has run it. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

---

## 8. What does a render print over the whole of a run, from its first line to its last?

**LABEL: not in the sources** for the actual run of lines — no source anywhere gives a transcript, a first line, a last line, or a line format. **documented** for several specific things a run does print, listed below.

### What is documented about a run's printed output

The manual says plainly that a run prints, and where:

> The Console Window, also called a Terminal, is an operating system text window that displays
> messages about Blender's operations, status, and internal errors.
>
> When Blender is manually started from a terminal,
> Blender output is shown in the Console Window it is started from.

Source: manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/index.html — read 2026-07-30. The same page lists "For rendering animation" first among its use cases, and "For Python development, to see the output of the print() function."

That status printing happens by default, and can be turned off, is documented by the command-line argument:

> -q, --quiet
>
> Suppress status printing (warnings & errors are still printed).

Source: manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html — read 2026-07-30. Reading that in reverse: without `-q`, a run prints status, warnings and errors; with `-q`, warnings and errors only. That reverse reading is **inferred** — the words say what `-q` suppresses, not what is printed when it is left off.

The single most specific statement about what a render prints is in the Python API documentation, on the render handlers:

> bpy.app.handlers.render_stats
>
> on printing render statistics. Accepts one argument: the render stats (render/saving time plus in background mode frame/used [peak] memory).

Source: https://docs.blender.org/api/5.1/bpy.app.handlers.html — read 2026-07-30. So a render prints render statistics, and those statistics carry render time and saving time, and — when Blender was started in background mode — also the frame and the used and peak memory. That the text handed to this handler is the same text that is printed is a step past the wording, so treat the identity as **inferred**; that statistics are printed and contain those items is **documented**.

The other render handlers show the moments a run passes through, which is the closest the sources come to an ordering:

> bpy.app.handlers.render_init — on initialization of a render job.
> bpy.app.handlers.render_pre — on render (before).
> bpy.app.handlers.render_post — on render (after).
> bpy.app.handlers.render_write — on writing a render frame (directly after the frame is written).
> bpy.app.handlers.render_complete — on completion of render job.
> bpy.app.handlers.render_cancel — on canceling a render job.

Same page and date. These are callback points, not printed lines. That a printed line accompanies any of them is **not in the sources**.

Extra printing can be turned on. From the manual's command-line arguments page, same address and date:

> --cycles-print-stats
>
> Log statistics about render memory and time usage.

> --log <match>
>
> Enable logging categories, taking a single comma separated argument.
>
> --log "*": everything
> --log "render,cycles": both render and cycles messages.

> --log-level <level>
>
> Set the logging verbosity level.
>
> fatal: Fatal errors only
> error: Errors only
> warning: Warnings
> info: Information about devices, files, configuration, operations
> debug: Verbose messages for developers
> trace: Very verbose code execution tracing

The page also lists `--log-show-memory` ("Show memory usage for each log message"), `--log-show-source`, `--log-file <filepath>` ("Set a file to output the log to"), and `--log-list-categories` ("List all available logging categories for --log, and exit"). That `render` and `cycles` are real logging categories is documented by the `--log "render,cycles"` example.

Where the Cycles statistics go is **observed** from Cycles' own files. `cycles_5.1/engine.py`, lines 13–15, inside `_configure_argument_parser`:

```
    parser.add_argument("--cycles-print-stats",
                        help="Print rendering statistics to stderr",
                        action='store_true')
```

What the file says: this option's own help text is "Print rendering statistics to stderr". That the statistics really do go to the error stream rather than the normal one is that help text's own claim, not something the file shows happening — the printing itself is inside `_cycles` — so **inferred**. Lines 34–36 of the same file show the option doing nothing but switching the feature on:

```
    if args.cycles_print_stats:
        import _cycles
        _cycles.enable_print_stats()
```

What is printed as a result is inside `_cycles`, which is not among the files supplied, so its content is **not in the sources**.

One more documented touch on printed lines, from the Preferences:

> Console Scroll-back Lines
>
> The number of lines, buffered in memory of the console window.
> Useful for debugging purposes and command-line rendering.

Source: manual, https://docs.blender.org/manual/en/5.1/editors/preferences/system.html — read 2026-07-30.

### The only pictures of Blender's console output in the manual

**Documented**, from `_images/advanced_command-line_launch_windows_console.png`, on the page https://docs.blender.org/manual/en/5.1/advanced/command_line/launch/windows.html. Its caption is "Blender's Console Window on Microsoft Windows", and the page text says: "The screenshot shows the Blender Console Window on Microsoft Windows directly after starting Blender and then a short while later after opening a file along with the relevant messages." Visible in the picture, in full:

```
c:\blender>blender
Read new prefs: C:\Users\Aaron\AppData\Roaming\Blender Foundation\Blender\2.77\config\userpref.blend
found bundled python: c:\blender\2.77\python
read blend: C:\tmp\Vincent.blend

Blender quit

c:\blender>
```

Two things must be said about this. First, no render is taking place in it — the page describes it as starting Blender and opening a file. So it is not a picture of a render's output. Second, the paths visible in it read `2.77`, so the picture was taken from a much older Blender and carried forward into the 5.1 manual. What is visible is visible: this manual shows a full Blender session's console output beginning with the command typed and ending with the line `Blender quit`. Reading from that what Blender 5.1 prints during a render would be reading far more into the picture than it shows, so I do not.

The other console picture, `_images/advanced_command-line_launch_linux_console.png` on https://docs.blender.org/manual/en/5.1/advanced/command_line/launch/linux.html, captioned "Starting Blender from a Linux terminal window", shows only startup messages about the JACK audio server failing to connect. No render, and no last line.

**Where I looked, for the silence.** Searched the whole manual, the whole API documentation, all release notes and all Cycles files for: `Fra:`, `Mem:`, `Peak`, `Time:`, `Saved:`, `Saving:`, `Append frame`, `Blender quit`, `stdout`, `standard output`, `stderr`, `exit code`, `return code`, `console`, `Console`, `terminal`, `statistics`, `stats`, `Rendered `, `Sample `, `Elapsed`, `Remaining`. The manual's whole treatment of the command line is five pages (`index`, `arguments`, the three `launch` pages) plus `render`, and I read all six in full. None of them shows a render's output. The API documentation's only relevant entry is `bpy.app.handlers.render_stats`, quoted above. The 5.1 release notes mention printed output once, in the bug-fix list — "Warnings about freeing memory after the leak detecter has run when cancelling a CLI render" (https://developer.blender.org/docs/release_notes/5.1/bugfixes/, read 2026-07-30) — which names a message that was appearing after a cancelled command-line render, but does not give it.

### Observed — the whole of a Blender 5.1 render run (2026-08-11)

Run in Blender 5.1.2 on **the Creator's Mac (a MacBook Pro with an Apple M1 Pro chip, 16 GB of memory, macOS 26.5.2, Apple's own graphics and no second card)** and on **a rented Linux machine (Ubuntu 22.04.5 LTS on kernel 6.8, two NVIDIA RTX 4000 Ada Generation cards of 20 GB each, eleven cores, 49 GB of memory)**, each with a settings folder of its own. The scene was built by Blender 5.1 itself: a cube, a sphere, an area light and a camera; Cycles on the processor; 16 samples; adaptive sampling off; 160 by 120; output PNG. **LABEL: observed.**

**1. The order and format of every line, first to last.** `blender -b <file>.blend -o <out> -f 1 > out.txt 2> err.txt` exited 0, wrote its picture, and put **nothing at all on standard error**. Standard output held fifty-four lines. The first seven, all three sample lines and the last four, word for word, from the Creator's Mac:

```
00:00.478  blend            | Read blend: "…/scenes/base_5.1.blend"
00:00.484  render           | Rendering single frame
00:00.484  render           | Rendering frame 1
00:00.485  render           | Start rendering: Scene, ViewLayer
00:00.485  render           | Engine: Cycles
00:00.517  render           | Fra: 1 | Mem: 0M | Synchronizing object | Cube
Blender 5.1.2 (hash ec6e62d40fa9 built 2026-05-19 01:30:33)
…
00:00.524  render           | Fra: 1 | Mem: 2M | Sample 0/16
00:00.530  render           | Fra: 1 | Remaining: 00:00.09 | Mem: 3M | Sample 1/16
00:00.723  render           | Fra: 1 | Mem: 3M | Sample 16/16
00:00.723  render           | Fra: 1 | Mem: 3M | Finished
00:00.782  render           | Saved: '…/out/e1_5.1_0001.png'
00:00.782  render           | Time: 00:00.29 (Saving: 00:00.05)

Blender quit
```

**The first line is `blend | Read blend:` and the last is `Blender quit`, with one blank line before it.** The version banner is not first: it arrives seven lines in, in the middle of the run, and it is the one line carrying no timestamp and no category. Every other line has an elapsed time, a category name, a vertical bar, then the message. **Exactly three lines carry a sample count for a sixteen-sample frame** — never one per sample, and nothing is printed per view layer. Only the `Sample 1/16` line carries a time estimate, called `Remaining`. **The rented Linux machine gave the same fifty-three to fifty-four lines in the same order, with nothing on standard error. The two machines agreed.**

**2. What "status printing" turns out to cover.** `-q` left fifty lines — three fewer. Comparing the two captures line by line, it removes exactly three: the `blend | Read blend:` line, the blank line before the end, and `Blender quit`. Every render line survives, including all three sample lines and the `Saved:` and `Time:` lines, and so does the version banner. The picture was still written and the run exited 0. So the reverse reading marked inferred above is only partly right: without `-q` a run prints those three lines as well, not a whole class of status.

**3. Which stream everything goes to: standard output.** Standard error was **empty** in every successful run on both machines, in this version. That includes Cycles' own statistics: with `-- --cycles-print-stats` the statistics arrived on **standard output**, not the error stream its help text names. They come in two blocks — `Update statistics:` during the Updating stages and `Render statistics:` **after the `Saved:` and `Time:` lines**, near the end of the run — and **carry no sample count**. Without the bare `--` the switch is taken as a file name and the run **exits 1**, printing `reports | ERROR Cannot read file "…/--cycles-print-stats": No such file or directory`.

> **Corrected by the citation check (2026-08-11).** This paragraph placed the second block "after the last sample line and before `Saved:`". Repeating the run on both machines, twice each, put it after `Saved:` and after `Time:` in this version. Blender 4.5 does place it before `Saved:`, which is what its own document says. The placing is also sent to the hand tests to be settled by a second pair of hands.

**Observed — the block this version prints, and where it lands, captured by a second pair of hands (2026-08-12).** Run on **the Creator's Mac (a MacBook Pro with an Apple M1 Pro chip, 16 GB of memory, macOS 26.5.2, Apple's own graphics and no second card)** and on **a rented Linux machine (Ubuntu 22.04.5 LTS on kernel 6.8, two NVIDIA RTX 4000 Ada Generation cards of 20 GB each on driver 580.105.08, an AMD Ryzen Threadripper PRO 3945WX with eleven cores, 49 GB of memory)**, each version with a settings folder of its own and its own copy of the scene: a cube, a sphere, an area light and a camera; Cycles on the processor; 16 samples; adaptive sampling off; 160 by 120; output PNG. The command was `blender -b <file>.blend -o <out> -f 1 -- --cycles-print-stats`, standard output and standard error kept apart. Every run exited 0 and standard error was empty. **LABEL: observed.** The head of the block on the Creator's Mac, word for word:

```
Render statistics:
Mesh statistics:
  Geometry:
    Total memory: 24.04K (24,615)
        Sphere                           23.55K (24,117)
        Cube                             417 (417)
        Area                             81 (81)
Image statistics:
  Textures:
    Total memory: 0 (0)
Kernel statistics:
  Total render time               : Total 100.00% (0.13s), Self 22.66% (0.03s)
```

**The rented Linux machine printed the same ten lines above `Kernel statistics:`, figure for figure**; its own kernel line read `  Total render time               : Total 100.00% (0.28s), Self 10.55% (0.03s)`. **The two machines agreed on the byte counts and differed on the times.** The block runs on through the kernel, shader and object figures, all of them times and percentages that changed from run to run, so only the byte counts should be read as this version's own. **No sample count appears anywhere in it.**

**How this version's figures compare with the others', run the same way on the same day.** Blender 5.0 printed the same four byte counts as this version. Blender 4.5 printed `Total memory: 24.03K (24,607)`, `Sphere 23.55K (24,113)`, `Cube 413 (413)`, `Area 81 (81)`. Blender 5.2 printed `Total memory: 16.36K (16,750)`, `Sphere 16.02K (16,405)`, `Cube 289B (289)`, `Area 56B (56)`, and wrote its image section as `Image statistics:` then `  Total image memory: 0B (0)`, where this version writes `Image statistics:`, `  Textures:`, `    Total memory: 0 (0)`.

**Where the block lands, counted by line number.** The lines of a second run were numbered. On the Creator's Mac standard output held 130 lines: `Update statistics:` at 15, `Sample 16/16` at 81, `Finished` at 82, `Saved:` at 83, `Time:` at 84, `Render statistics:` at 85. On the rented Linux machine it also held 130 lines with exactly the same numbers. **The two machines agreed, and both bear out the correction above: in this version the block comes after `Saved:` and after `Time:`.** Blender 4.5 run the same way put `Render statistics:` at line 78 and `Saved:` at 122 on the Mac — the other way round.

**4. What is printed per frame.** A three-frame animation announces the range once and each frame in turn:

```
00:00.311  render           | Rendering animation (frames 1..3)
00:00.311  render           | Rendering frame 1
…
00:00.459  render           | Rendering frame 2
…
00:00.544  render           | Rendering frame 3
```

Each frame carries its own `Start rendering`, `Engine:`, stage lines, three sample lines, `Finished`, `Saved:` and `Time:`. Nothing follows the last frame's `Time:` line but the blank line and `Blender quit`.

**5. What the logging switches add, and the list of categories.** Against the fifty-four lines above, `--log-level info` gives **139** lines and `--log "*"` gives **140** on the Creator's Mac. `--log-list-categories` prints **139** names on the Creator's Mac and **173** on the rented Linux machine — so the list is longer on Linux — and **both `render` and `cycles` are in it** on both machines.

> **Corrected by the citation check (2026-08-11).** The Linux figure read 174. That is the number of *lines* printed, of which the first is the Blender version banner; the count of categories is 173. The Mac figure was already counted the right way.

**6. What a run ends with when it ends normally: `Blender quit`, and status 0.** Every successful run above exited 0 on both machines.

**7. The other two engines.** EEVEE — called `BLENDER_EEVEE` in this version, not `BLENDER_EEVEE_NEXT` — prints four progress lines of the shape `Fra: 1 | Rendering 1 / 64 samples`, at 1, 25, 50 and 64 of 64, with no memory figure and no time. **Workbench prints no progress line at all**, going straight from the announcements to `Saved:` and `Time:`. The same on both machines.

**8. What the render-statistics handler is handed.** A script attached a handler to `bpy.app.handlers.render_stats` and wrote every call down. With no window it fired **forty-four times for one frame** — once per stage, not once per render — and was handed two arguments, a string and `None`:

```
STATS ('Mem: 0M | Synchronizing object | Cube', None)
STATS ('Remaining: 00:00.04 | Mem: 3M | Sample 1/16', None)
STATS ('Time: 00:00.20 (Saving: 00:00.04)', None)
```

**The string carries no frame number in this version.** **With a window open the same handler fired once**, and was handed only the closing line, `STATS ('Time: 00:00.13 (Saving: 00:00.01)', None)`. So the identity marked inferred above — that the text handed to the handler is the text that is printed — holds for a windowless run: the handler's strings are the tails of the printed lines. The same on both machines.

**Not covered.** This does not settle what is printed when a run is cancelled, nor whether the output is line-buffered or overwrites itself in place. It does not settle what a run prints when it fails rather than succeeds, which is a different question on the sheet. It does not settle anything about Windows.

**Outside the interface.** This answer is mostly about the outside-the-interface case already: the printed output is what a caller sees, and the manual points at it for exactly that use ("For rendering animation", "For automation and batch processing"). One documented difference is quoted above — the render statistics carry the frame and used and peak memory *in background mode*. That they therefore do not carry those when there is a window is the reverse reading of the same sentence, and is **inferred**; the wording only says what background mode adds. Answer 9 labels the same step the same way. On Microsoft Windows there is a documented wrinkle even with a window: "When Blender is started on a Microsoft Windows operating system, the Console Window (called the Command Prompt) is first created as a separate window on the desktop. The main Blender window will also appear and the Console Window will then be toggled off. To display the console again, go to Window ‣ Toggle System Console." (same Windows page, read 2026-07-30).

**Operating system.** Settled for macOS and for Linux by the tests above, and the printed content agreed line for line: the same fifty-three to fifty-four lines in the same order, the same first and last line, the same three sample lines, the same three lines removed by `-q`, the same statistics on standard output with an empty standard error, the same animation announcements, the same EEVEE and Workbench shapes, and the same handler strings. **One difference between the machines**: the list of logging categories is 139 entries on the Creator's Mac and 173 on the rented Linux machine (corrected from 174 by the citation check, 2026-08-11 — the Linux figure had counted the version banner as a category). **Checked again on 2026-08-12 for Cycles' statistics**: the block's byte counts and its place in the run came back the same on both machines — the same six line numbers out of 130 lines — and only the times inside the block differed. **Windows is untested** — no Windows machine has run it — so the Windows-only `-con, --start-console` and the hidden Windows console stand unchecked.

---

## 9. What does a render report about its progress under each of the ways a render can be started?

**LABEL: documented** for the ways a render can be started, and for one difference in reporting between them. **not in the sources** for a per-way account of progress reporting — only one way (background) has its reporting described at all.

### The ways a render can be started, as the sources give them

**From the window's menus.** Manual, Topbar, Render Menu:

> Render Image F12
>
> Render the active scene at the current frame.
>
> Render Animation Ctrl-F12
>
> Render the animation of the active scene.
>
> Render Sequencer Image Alt-F12
>
> If a Sequencer Scene exists that differs from the active scene, render that
> scene instead.
>
> Render Sequencer Animation Ctrl-Alt-F12
>
> Render the animation of the Sequencer Scene.

Source: https://docs.blender.org/manual/en/5.1/interface/window_system/topbar.html — read 2026-07-30. The same menu carries "View Render F11 — Show the Render window. (Press again to switch back to the main Blender window.)"

**From the command line.** Manual, command-line arguments:

> -b, --background
>
> Run in background (often used for UI-less rendering).

> -a, --render-anim
>
> Render frames from start to end (inclusive).

> -f, --render-frame <frame>
>
> Render frame <frame> and save it.

Source: https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html — read 2026-07-30. The dedicated page https://docs.blender.org/manual/en/5.1/advanced/command_line/render.html gives the shapes `blender -b file.blend -f 10` and `blender -b file.blend -a`, and warns: "Arguments are executed in the order they are given!" and "Always position -f or -a as the last arguments."

**From Python.** API documentation:

> bpy.ops.render.render(*, animation=False, write_still=False, use_viewport=False, use_sequencer_scene=False, layer='', scene='', frame_start=0, frame_end=0)
>
> Undocumented, consider contributing.

Source: https://docs.blender.org/api/5.1/bpy.ops.render.html — read 2026-07-30. The word "Undocumented" there is the source's own. The same page carries `bpy.ops.render.opengl(...)` for a viewport render, and `bpy.ops.render.view_cancel()` and `bpy.ops.render.view_show()`.

**From the viewport, interactively.** The manual's Cycles sampling page separates "Viewport (Max) Samples — The number of paths to trace per pixel in the 3D Viewport (when using the Rendered shading mode)" from "Render (Max) Samples — The number of paths to trace per pixel in the final render" (https://docs.blender.org/manual/en/5.1/render/cycles/render_settings/sampling.html, read 2026-07-30). The API documentation's example engine says the same thing about which method is called: "This is the method called by Blender for both final renders (F12) and small preview for materials, world and lights." (https://docs.blender.org/api/5.1/bpy.types.RenderEngine.html, read 2026-07-30).

### What is reported, per way

**Common to every way, engine-side.** `RenderEngine.update_progress(progress)` — "Update progress percentage of render", "Percentage of render that's done (in [0, 1])" — and `RenderEngine.update_stats(stats, info)` — "Update and signal to redraw render status text" — and `RenderEngine.update_memory_stats(*, memory_used=0.0, memory_peak=0.0)` — "Update memory usage statistics", "Current memory usage in megabytes", "Peak memory usage in megabytes". Also `RenderEngine.report(type, message)` — "Report info, warning or error messages" — and `RenderEngine.error_set(message)` — "Set error message displaying after the render is finished". Source: https://docs.blender.org/api/5.1/bpy.types.RenderEngine.html — read 2026-07-30. These are documented as calls a render engine can make, and the documentation does not say they behave differently depending on how the render was started. Whether Blender's own engines call them on every render is not stated.

**When there is a window.** The Status Bar's Running Task, quoted in full in answer 7: a progress bar for "the currently running task (such as rendering or baking)", a time estimate on hover, and a cancel button. The picture on that page shows it as a labelled percentage ("Sequence Render", "94%"). Messages also land in the Info editor: "The Info editor logs the executed operators as well as errors, warnings, and informational messages." (https://docs.blender.org/manual/en/5.1/editors/info_editor.html, read 2026-07-30).

**When started in background.** This is the only documented difference between the ways, and it is the render statistics:

> bpy.app.handlers.render_stats
>
> on printing render statistics. Accepts one argument: the render stats (render/saving time plus in background mode frame/used [peak] memory).

Source: https://docs.blender.org/api/5.1/bpy.app.handlers.html — read 2026-07-30. Read plainly: every render's statistics carry render time and saving time; a background-mode render's statistics carry, in addition, the frame and the used and peak memory. **Documented**. That a render started from the window therefore does *not* carry frame and memory in its statistics is the reverse reading of the same sentence — I take it as **inferred**, since the sentence only says what background mode adds.

Background mode is identifiable from inside a run:

> bpy.app.background
>
> Boolean, True when blender is running without a user interface (started with -b)

Source: https://docs.blender.org/api/5.1/bpy.app.html — read 2026-07-30. Cycles is handed that flag when it starts. **Observed**, `cycles_5.1/engine.py`, line 52, in `init()`:

```
    _cycles.init(path, user_path, bpy.app.background)
```

What the file says: whether Blender is in background mode is passed into Cycles at start-up as the third argument to its `init`. What Cycles then does differently is inside `_cycles` and is not in the supplied files.

Whichever way a render was started, the status printing can be turned off with `-q, --quiet` — "Suppress status printing (warnings & errors are still printed)" — though that argument is only available when Blender is started from a command line.

**Where I looked, for the silence.** I read in full: the Topbar page, the command-line index, arguments and render pages, all three launch pages, the Status Bar page, the Info editor page, the Cycles sampling and performance pages, the render output introduction and animation pages, `bpy.ops.render`, `bpy.types.RenderEngine`, `bpy.app.handlers`, and the `background` entry of `bpy.app`. I searched every source for `progress`, `progress bar`, `percent`, `status`, `stats`, `statistics`, `background`, `quiet`, `console`. Nothing anywhere describes what a render prints or shows when started by `bpy.ops.render.render` from a script, by `-f`, or by `-a`, other than through the general statements above. The operator's own documentation says "Undocumented, consider contributing."

### Observed — what Blender 5.1 reports under each way of starting (2026-08-11)

Run in Blender 5.1.2 on **the Creator's Mac (a MacBook Pro with an Apple M1 Pro chip, 16 GB of memory, macOS 26.5.2, Apple's own graphics and no second card)** and on **a rented Linux machine (Ubuntu 22.04.5 LTS, two NVIDIA RTX 4000 Ada Generation cards of 20 GB each, eleven cores, 49 GB of memory)**, each with a settings folder of its own. The same frame from the same file — Cycles on the processor, 16 samples, adaptive sampling off, 160 by 120 — was started five ways and the terminal captured each time. **LABEL: observed.**

**The way of starting changes what is printed, and a render started from a script prints nothing.** Counting the lines that carry a sample count:

| how the render was started | sample lines printed |
|---|---|
| `blender -b <file>.blend -o <out> -f 1` | 3 |
| `blender -b <file>.blend -o <out> -a` (three frames) | 3 per frame |
| `blender -b <file>.blend -o <out> --python <script>` calling `bpy.ops.render.render(write_still=True)` | **0** |
| a window open, the same call from a script | 0 |
| a window open, started the way the F12 key starts it — `bpy.ops.render.render('INVOKE_DEFAULT', write_still=True)` | 0 |

So a render started in background **by a script** rather than by `-f` or `-a` prints only the file it saved. It was run twice, once with the call in a `--python-expr` and once in a script file, with the same result both times, on both machines.

**Whether the operator blocks.** Called plainly it blocks until the render is finished and returns `{'FINISHED'}`. Started the way the F12 key starts it, it returns `{'RUNNING_MODAL'}` at once and the render goes on afterwards — a script that waited and then listed the folder found the picture written. Both machines.

**Whether reporting differs between a single frame and an animation.** In the terminal, no: each frame of an animation prints the same block as a single frame, three sample lines and all, with `Rendering frame N` in front of it. On screen, no: the status bar's label stays `Rendering...` and never names a frame, and its percentage restarts at each frame rather than climbing across the range.

**What the ways of starting actually are, read from the running program.** The Python render operator takes **eight** arguments — `animation`, `write_still`, `use_viewport`, `use_sequencer_scene`, `layer`, `scene`, `frame_start`, `frame_end` — and every one carries a description, contrary to the API page's "Undocumented, consider contributing". The Render menu draws **four** render commands: `Render Image`, `Render Animation`, `Render Sequencer Image`, `Render Sequencer Animation`. The 3D viewport's View menu draws `Render Viewport Preview`, `Render Playblast` and `Render Playblast on Keyframes`. There is **no** `save_output` setting in this version. Identical on both machines.

**A sequencer render drives the status bar's progress bar and is labelled `Rendering sequence...`**, not `Sequence Render` as in the manual's picture; with no window the same render prints `Rendering frame 1` and the ordinary three sample lines. Both machines.

**The one documented difference, checked.** With no window the render-statistics handler fired forty-four times, the strings carrying a memory figure and the stage but **no frame number**. With a window it fired once, carrying only `STATS ('Time: 00:00.13 (Saving: 00:00.01)', None)`. So the reverse reading marked inferred above is borne out in one direction — a windowed render's statistics carry neither frame nor memory — but the windowless strings carry a memory figure and **no** frame either, in this version.

**A render engine's own progress calls behave the same either way.** `update_progress`, `update_stats` and `update_memory_stats`, called from a tiny engine registered from a script, all returned without complaint with a window and without, on both machines, and none produced output of its own. So whether Blender's own engines call them is still open, but the calls themselves are not window-bound.

**Observed — a windowed run with the handler attached, and how it was closed (2026-08-12).** A later hand test repeated the windowed run in all four versions on both machines, because no earlier record says how a windowed Blender was made to end, and the exit status of such a run belongs to whoever ended it. Run on **the Creator's Mac (a MacBook Pro with an Apple M1 Pro chip, 16 GB of memory, macOS 26.5.2, Apple's own graphics and no second card)** and on **a rented Linux machine (Ubuntu 22.04.5 LTS on kernel 6.8, two NVIDIA RTX 4000 Ada Generation cards of 20 GB each on driver 580.105.08, an AMD Ryzen Threadripper PRO 3945WX with eleven cores, 49 GB of memory), whose window was opened on its own Xorg display**. Each version had a settings folder of its own and its own copy of the scene. `/tmp` was emptied of `*.crash.txt` before every run. A start-up script attached a handler to `bpy.app.handlers.render_stats` and another to `bpy.app.handlers.render_complete`, called `bpy.ops.render.render(write_still=True)`, printed the counts and then closed Blender. **LABEL: observed.**

**How a windowed Blender is closed is part of the answer.** `bpy.ops.wm.quit_blender()` called straight from the start-up script did not close the run on the rented Linux machine: the script printed `STILL HERE AFTER quit_blender()` and Blender was still going when a 120-second cap ended it, and again when a 240-second cap ended it — both times the status was 124, which belongs to `timeout` and not to Blender. On the Creator's Mac the same call did close the run, exiting 0, but 24.7 seconds after the render in one run and 6.3 seconds after it in another. A timer registered with `bpy.app.timers.register`, calling `bpy.ops.wm.quit_blender()` three seconds after the render, closed the run on both machines every time, and that is how the run below was closed.

**Closed by the timer, Blender 5.1 exited 0 on both machines, with no crash file.** The whole of standard output from the Creator's Mac, long paths shortened:

```
00:00.572  blend            | Read blend: "…/scenes/base_5.1.blend"
BACKGROUND=False
VERSION=5.1.2
00:00.751  render           | Saved: '…/out/e21_5.1_timer_.png'
HANDLER FIRED 1 TIMES
FIRST=('Time: 00:00.16 (Saving: 00:00.05)', None)
LAST=('Time: 00:00.16 (Saving: 00:00.05)', None)
RENDER_COMPLETE FIRED 1 TIMES
QUITWAY=timer
TIMER REGISTERED FOR 3s
TIMER FIRED, CALLING quit_blender()
STILL HERE AFTER quit_blender() IN TIMER
00:03.768  blend            | Saved session recovery to "/var/folders/n3/z3rfjt654rn1k1x6f_1gk4540000gn/T/quit.blend"
```

The lines with no timestamp are the test script's own printing. `Blender quit` followed after a blank line, and standard error was empty. **The rented Linux machine gave the same run — exit 0, handler once, `render_complete` once, standard error empty** — with one line different: where the Mac saved a session recovery file, the Linux machine printed `00:04.201  reports          | ERROR Cannot open file /tmp/quit.blend@ for writing: Permission denied`.

**So the count this document already gives is borne out: with a window the handler fires once**, carrying only the closing `Time:` line. **No run of this version on either machine ended in a segmentation fault, and no crash file was written in `/tmp`.**

**Not covered.** This does not settle what an interactive viewport render reports. It does not settle what the time estimate says when the pointer rests on the progress bar — that needs a hover, and neither machine can move the pointer without a hand on it. It does not settle which key is bound to which render command: with no window Blender's key configuration holds no items. It does not settle anything about Windows.

**Outside the interface.** Settled, and it is the hinge of this answer. A render started **from the command line with `-f` or `-a`** and no window prints its full progress. A render started **from a script**, with or without a window, prints nothing but the file it saved. A render started with **a window open** prints nothing in any way, and what stands in for it is the status bar's one labelled bar with a whole-number percentage. The `render_stats` handler is handed forty-four strings with no window and one with a window. Both machines agreed. **Added 2026-08-12:** a windowed run started from a terminal, with the handler attached, ended cleanly on both machines — exit 0, no crash file — when closed by a timer calling `bpy.ops.wm.quit_blender()`; the same call made straight from the start-up script did not close a windowed Blender on the Linux machine at all.

**Operating system.** Settled for macOS and for Linux, and they agreed on every point above: the same counts of printed sample lines for all five ways, the same operator arguments and menu entries, the same `Rendering...` and `Rendering sequence...` labels and whole-number percentages on each machine's own screen. **One difference between the machines, found on 2026-08-12**: `bpy.ops.wm.quit_blender()` called from a start-up script closed a windowed Blender on the Creator's Mac, after six seconds in one run and twenty-five in another, and did not close it on the rented Linux machine within four minutes. At quit time the Mac printed `Saved session recovery to "/var/folders/…/T/quit.blend"` and the Linux machine printed `ERROR Cannot open file /tmp/quit.blend@ for writing: Permission denied`. Closed by a timer, both machines exited 0. **Windows is untested**, no Windows machine having run it.

---

## 10. What does a render do when the machine's own memory runs out while it is sampling?

**LABEL: not in the sources.**

Nothing in the 5.1 manual, the 5.1 Python API documentation, the 5.1 release notes, or the supplied Cycles files says what a render does when the machine's own memory is exhausted during sampling. There is no statement that it stops, falls back, spills to disk, reports an error, or ends the process.

### What the sources do say, none of which answers the question

The nearest is a statement about Blender as a whole rather than about a render. The manual's Crashes page opens:

> Crashes
>
> The most common causes of Blender crashes:
>
> Running out of memory.
>
> Issues with graphics hardware or drivers.
>
> Bugs in Blender.

Source: manual, https://docs.blender.org/manual/en/5.1/troubleshooting/crash.html — read 2026-07-30. The rest of that page is about preventing it and about the crash log. It never mentions rendering or sampling.

The same page documents what is written when a crash does happen, and it differs by operating system:

> When Blender crashes, it writes out a text file which contains information that may help identify the cause of the
> crash. Usually, this file is written in the Temporary Directory directory.

> Windows
>
> On a crash, a file is written based on the name of the currently loaded blend-file, so test.blend will create a
> file called test.crash.txt.

> Linux
>
> After a crash, a file named blender.crash.txt is written to the /tmp directory.

For macOS the page points at the system's own Crash Reporter and at `~/Library/Logs/DiagnosticReports/` instead. Same address and date. That last sentence is a paraphrase rather than a quotation — the exact words are not reproduced here, so this one claim cannot be checked from this document alone. Whether a memory exhaustion during sampling counts as a crash for this purpose is not stated.

The API documentation says the same about Python, not about rendering: "Be aware of memory limits, especially when working with large lists since Blender can crash simply by running out of memory." (https://docs.blender.org/api/5.1/info_gotchas_crashes.html, read 2026-07-30).

A report category for this condition exists. Python API documentation, Wm Report Items:

> ERROR_OUT_OF_MEMORY:
>
> Out of Memory.

Source: https://docs.blender.org/api/5.1/bpy_types_enum_items/wm_report_items.html — read 2026-07-30. That is the list of kinds a report can be. Nothing says a render ever issues one.

The one place the sources describe a memory shortage during rendering is about the graphics card, not the machine's own memory, and about storing the scene rather than sampling:

> Error: Out of memory
>
> This usually means there is not enough memory to store the scene for use by the GPU.

> With CUDA, OptiX, HIP and Metal devices, if the GPU memory is full Blender will automatically
> try to use system memory. This has a performance impact, but will usually still result in a faster render
> than using CPU rendering.

Source: manual, https://docs.blender.org/manual/en/5.1/render/cycles/gpu_rendering.html — read 2026-07-30. That documented fallback runs the other way from the question — it moves work *onto* the machine's own memory. What happens if the machine's own memory is then exhausted is not stated.

Two settings are documented as reducing memory use during a render, which is prevention rather than a response: Tile Size ("Render high resolution images in tiles of this size, to reduce memory usage. Tiles are cached to disk while rendering to save memory") and Use Compact BVH ("Use a more compact BVH structure, which can reduce RAM usage but render slower"), both at https://docs.blender.org/manual/en/5.1/render/cycles/render_settings/performance.html, read 2026-07-30. The same page names a whole preset for it: "Lower Memory: Uses settings to decrease memory considered at the cost of slower renders."

One release note touches the machinery, without saying anything about behaviour when memory runs out:

> Linux: Replace JEMALLOC with TBB_MALLOC_PROXY for memory allocation.
> JEMALLOC is no longer maintained.

Source: release notes, https://developer.blender.org/docs/release_notes/5.1/core/ — read 2026-07-30.

**Where I looked, for the silence.** I searched every file of the 5.1 manual, the 5.1 API documentation, the 5.1 release notes, and the Cycles files for, case-insensitively: `out of memory`, `out-of-memory`, `OOM`, `runs out`, `ran out`, `not enough memory`, `insufficient`, `MemoryError`, `memory limit`, `swap`, `killed`, `terminated`, `abort`, `crash`. I then read in full the only pages those searches reached that could plausibly hold the answer: the Crashes page, the GPU Rendering page (its whole "Error Messages" section), the Cycles Performance page, the System Preferences "Memory & Limits" section, and the API's `info_gotchas_crashes` page. None of them describes what happens during sampling. I also checked the Cycles files: they hold the add-on's Python and the render kernel headers only, with no allocation-failure handling of any kind, and no session code.

### Observed — Blender 5.1 refuses the allocation, says so, and aborts (2026-08-11)

Run on **a rented Linux machine only — Ubuntu 22.04.5 LTS on kernel 6.8, two NVIDIA RTX 4000 Ada Generation cards of 20 GB each on driver 580.105.08, an AMD Ryzen Threadripper PRO 3945WX with eleven cores, 49 GB of memory**. It was **not** run on the Creator's Mac: that is their working computer and its memory is not to be filled on purpose. So this answer covers Linux and leaves macOS untested. **LABEL: observed.**

**The scene.** A sphere, a cube, an area light and a camera; Cycles on the processor; 2000 samples; adaptive sampling off; 8192 by 8192 pixels; tile size 8192; every render pass switched on, cryptomatte at sixteen levels, and forty light groups added. Rendered with `blender -b <file>.blend -o <out> -f 1`, with the machine's memory read once a second alongside.

**Getting the memory to run out while sampling took three attempts, and the first two are results of their own.** More pixels do not do it: at 12288 by 12288 the run reached 48.9 GB of the machine's 49 and **kept sampling**, and at 16384 by 16384 Blender's own figure was the same again. Tiling caps what is held — the tile size cannot exceed 8192 — so the buffers stop growing with the picture. What did it was more passes.

**What happened.** The last line on standard output is a sampling line:

```
00:00.477  render           | Fra: 1 | Mem: 9M | Sample 0/2000
```

**Standard error carries Blender's own words**, and they name the size of the request:

```
Malloc returns null: len=80262201344 in Cycles Aligned Alloc, total 20296764
terminate called after throwing an instance of 'std::bad_alloc'
  what():  std::bad_alloc
```

80,262,201,344 bytes is about 80 GB, on a machine with 49. **No picture was written, no partial frame survived, and no crash file appeared** in `/tmp` or anywhere else. The process ended with status **134**. **The machine's own memory use never went above about 2.9 GB of 49** — the request was refused whole rather than filled piece by piece.

**An animation does not go on to the next frame.** A three-frame run of the same scene printed one `Rendering frame` line, aborted the same way on frame one, wrote nothing, and ended with status 134.

The whole test was run twice, the second time with the four versions taken in the opposite order, and Blender 5.1 behaved the same way both times. **Blender 4.5 and 5.0 behave differently on the same machine with the same scene**: they fill the machine to about 49 GB, print nothing at all, are killed by the operating system's out-of-memory killer, and end with status 137. That is recorded in their own documents.

**Not covered.** This does not settle what happens on macOS or on Windows. It does not settle whether the answer differs when the render is on a graphics card rather than the processor. It does not settle what a window-open run would show, since this was run with no window. It does not settle whether a scene that grows more gradually would be killed rather than refused.

**Outside the interface.** Settled for a run with no window, which is what was tested: the allocation is refused, three lines are printed on standard error, nothing is written, and the process ends with status 134. What a run with a window would show was not tested.

**Operating system.** Settled for Linux only. macOS is untested, because the Creator's working machine must not have its memory filled on purpose, and **Windows is untested**, no Windows machine having run it. On Linux, where it was tested, **no crash file was written at all** — so the documented `blender.crash.txt` in `/tmp` did not appear for this kind of ending.

---

## 11. What, if anything, does a render print that shows whether a frame took the number of samples it was asked for?

**LABEL: not in the sources** for anything printed. **documented** for a render pass that records, per pixel, the samples taken divided by the maximum — which is picture data rather than text, and is not a per-frame answer.

Nothing in the 5.1 manual, the 5.1 API documentation, the 5.1 release notes, or the Cycles files describes any printed line that reports how many samples a frame took, or that compares samples taken against samples asked for.

### The two documented ways a frame can end short, neither of which is documented as printing anything

Adaptive sampling:

> If the Noise Threshold checkbox is enabled, Cycles will use adaptive sampling,
> cutting short the sampling process in areas that have become less noisy than the
> specified threshold value – in other words, not wasting time further refining
> areas that already look good enough.

Time limit:

> Time Limit
>
> Stops rendering if the time exceeds the limit, even if the desired sample count
> hasn't been reached yet. Setting this value to zero disables the limit.

Both from the manual, https://docs.blender.org/manual/en/5.1/render/cycles/render_settings/sampling.html — read 2026-07-30. Neither passage, nor anything else on that page, says that a message is printed when either happens.

A third documented way the number rendered differs from the number asked for is Sample Subset, on the same page: "Only render a subset of the samples… While this overrides (Max) Samples in terms of the samples that will get rendered, it's still important to set (Max) Samples to the total number of samples that will be rendered across all subsets". Again, nothing printed.

### What exists instead of a printed line

A render pass carries exactly this figure, per pixel, as picture data rather than as text. Manual, Render Passes, under "Debug":

> Sample Count Cycles
>
> Number of samples calculated for each pixel, divided by the maximum number of samples.
> Used to analyze adaptive sampling.

Source: https://docs.blender.org/manual/en/5.1/render/layers/passes.html — read 2026-07-30.

**Observed**, in Cycles' own files. `cycles_5.1/properties.py`, in `CyclesRenderLayerSettings` (lines 1482–1487):

```
    pass_debug_sample_count: BoolProperty(
        name="Debug Sample Count",
        description="Number of samples per pixel taken, divided by the maximum number of samples. To analyze adaptive sampling",
        default=False,
        update=update_render_passes,
    )
```

What the file says: the switch exists, is off by default, is named "Debug Sample Count", and describes its contents as samples taken divided by maximum samples. `cycles_5.1/engine.py`, lines 220–221, shows the pass it produces and its shape:

```
    if crl.pass_debug_sample_count:
        yield ("Debug Sample Count", "X", 'VALUE')
```

One channel, of value type. A reader who wants a per-frame yes-or-no answer out of this would have to render the pass and inspect its pixels; that is a step past the files, so **inferred**, and it is inspection of an image, not something printed.

The related live indicator is also a picture, not text. **Observed**, `cycles_5.1/properties.py`, lines 1954–1957:

```
    show_active_pixels: BoolProperty(
        name="Show Active Pixels",
        description="When using adaptive sampling highlight pixels which are being sampled",
    )
```

What the file says: a switch exists under that name, and its own description says it highlights the pixels being sampled. That it does so during a render is the description's claim rather than something the file shows. Either way, nothing printed.

### Why the printed statistics do not carry it either

The only documented description of a render's printed statistics is the render_stats handler, quoted in full in answer 8: "the render stats (render/saving time plus in background mode frame/used [peak] memory)". Render time, saving time, frame, used memory, peak memory. That this list names no sample count is **documented**. That the printed statistics therefore contain none is a step past a one-line summary in the documentation, so **inferred**.

The metadata that can be stamped into an output image is likewise listed in full in the manual and contains no sample count: Date, Time, Render Time, Frame, Frame Range, Memory ("Includes the peak memory usage"), Hostname, Camera, Lens, Scene, Marker, Filename, Strip Name, Note. Source: https://docs.blender.org/manual/en/5.1/render/output/properties/metadata.html — read 2026-07-30. The page does offer a free-text Note field — "It can be useful to use the Note field if you are setting up a render farm. Since you can script any information you like into it" — but what goes in it is the user's to supply, not something a render reports.

**Where I looked, for the silence.** Searched every source for: `sample count`, `Sample Count`, `Debug Sample Count`, `samples taken`, `actual samples`, `converged`, `Sample `, `Rendered `, `adaptive`, `noise threshold`, `Time Limit`, `stats`, `statistics`, `print`. I read in full the Cycles Sampling page, the Render Passes page, the Metadata page, `bpy.app.handlers`, and the sampling-related properties in `cycles_5.1/properties.py`. Every hit is either a setting that controls how many samples are taken, or the render pass described above. None is a printed line. The 5.1 release notes say nothing about sample-count reporting.

### Observed — what Blender 5.1 prints and writes about samples (2026-08-11)

Run in Blender 5.1.2 on **the Creator's Mac (a MacBook Pro with an Apple M1 Pro chip, 16 GB of memory, macOS 26.5.2, Apple's own graphics and no second card)** and on **a rented Linux machine (Ubuntu 22.04.5 LTS, two NVIDIA RTX 4000 Ada Generation cards of 20 GB each, eleven cores, 49 GB of memory)**, each with a settings folder of its own. **LABEL: observed.**

**1. A printed line does show it, and the mark is not the one a reader would expect.** The scene — a cube, a sphere, an area light and a camera; Cycles on the processor; 4096 samples; adaptive sampling off; 480 by 360 — was rendered three ways from a terminal with no window: plain, with a five-second time limit, and with adaptive sampling on at a threshold of 0.1. **All three exited 0 and wrote their picture.**

A frame that finished ends on its full count. The whole of the sample lines from the plain run on the Creator's Mac:

```
00:00.404  render           | Fra: 1 | Mem: 3M | Sample 0/4096
00:00.415  render           | Fra: 1 | Remaining: 00:35.99 | Mem: 26M | Sample 1/4096
00:36.024  render           | Fra: 1 | Mem: 26M | Sample 4096/4096
```

A frame cut short by the time limit shows the count it reached and then **resets to zero**:

```
00:05.640  render           | Fra: 1 | Mem: 26M | Sample 0/4096
```

is its last line, after a line reading `Sample 513/4096`. A frame cut short by adaptive sampling does the same, climbing and then ending on `Sample 0/4096`. **Nothing else is printed** — no notice, no warning, nothing naming the time limit or the noise threshold. So the only printed mark of a frame that stopped short is that its **last** sample line reads `Sample 0/<maximum>`, where a completed frame's last line reads `Sample <maximum>/<maximum>`, and the count actually reached sits on the line before it. On the rented Linux machine the same three runs behaved identically, reaching 798/4096 under the time limit and 240/4096 under adaptive sampling before ending on `Sample 0/4096`. **The two machines agreed on the shape and differed only in the counts reached, as two different processors must.**

**2. The printed statistics carry no sample figure the documentation does not list.** With `-- --cycles-print-stats` the two printed blocks hold memory and time only, on both machines — no sample count anywhere. And the render-statistics handler is handed strings that carry a sample count **only while a sampling stage is running**: with no window it received forty-four strings, one of them `STATS ('Remaining: 00:00.04 | Mem: 3M | Sample 1/16', None)`; with a window it received one string, `STATS ('Time: 00:00.13 (Saving: 00:00.01)', None)`, carrying none.

**3. The number reached is *not* recoverable from the saved image, but the maximum is.** A frame rendered to PNG with the stamp settings untouched carries eleven metadata fields, read back by a plain Python script rather than by Blender:

```
File = …/scenes/base_5.1.blend
Date = 2026/08/11 21:53:44
Time = 00:00:00:01
Frame = 1
Camera = Camera
Scene = Scene
RenderTime = 00:00.23
cycles.ViewLayer.samples = 16
cycles.ViewLayer.total_time = 00:00.21
cycles.ViewLayer.render_time = 00:00.19
cycles.ViewLayer.synchronization_time = 00:00.01
```

**`cycles.ViewLayer.samples` is on that list and is not on the manual's Metadata panel.** But it holds the **maximum asked for, not the number reached**: a run whose sample lines showed 144 of 512 reached carried `cycles.ViewLayer.samples (string) = 512`. The same eleven came back on both machines. An OpenEXR file carries the same fields plus `Software` (`Blender 5.1.2`), `colorInteropID`, `name` and `type`.

**4. The Sample Count pass is *not* written into a multi-layer OpenEXR in this version.** With `pass_debug_sample_count` set to `True` — confirmed set before each render — adaptive sampling on at 0.1 and a maximum of 512 samples, saved as a multi-layer OpenEXR, the file's channel list, read without Blender, is:

```
ViewLayer.Combined.A, ViewLayer.Combined.B, ViewLayer.Combined.G, ViewLayer.Combined.R
```

Only Combined. Nothing was printed about it and the run exited 0. **Blender 4.5 writes the pass into the same file** — its channel list carries `ViewLayer.Debug Sample Count.X` and four Noisy Image channels as well, and its values run from 0.300000 to 1.000000 with 172,791 of 172,800 pixels below one. So this differs between the versions. The same on both machines. One note on the set-up: in this version the output must be switched to multi-layer with **two** settings, `image_settings.media_type = 'MULTI_LAYER_IMAGE'` as well as `image_settings.file_format = 'OPEN_EXR_MULTILAYER'`; setting only the second writes a PNG, with no error and an exit status of 0.

**5. Other render engines.** EEVEE prints `Fra: 1 | Rendering <n> / 64 samples` at 1, 25, 50 and 64 — a running count against a maximum, in a different shape from Cycles' — and it used its own count of 64 where the scene asked for 16, printing nothing about the difference. **Workbench prints no sample line at all.** Both machines.

**Not covered.** This does not settle what is printed when Sample Subset is in use. It does not settle anything about Windows.

**Outside the interface.** Settled, and it differs. With no window the sample count is printed, in three lines for a completed frame and in a run that ends on `Sample 0/<maximum>` for a frame that stopped short. With a window open **no sample line reaches the terminal at all**, and the render-statistics handler is handed one string carrying no sample count. The image metadata is written either way — it was produced by runs with no window. The step marked inferred above, about the Sample Count pass landing in the saved file either way, is **not** borne out for this version: the pass did not reach the saved file at all. Both machines agreed.

**Operating system.** Settled for macOS and for Linux, and they agreed on every point: the same printed pattern, the same eleven metadata fields, the same absence of the pass from the saved file, and the same absence of a sample count from Cycles' statistics. **Windows is untested**, no Windows machine having run it.

---

## Things a person sitting at a computer with Blender 5.1 open could settle in a few minutes

The eleven items that stood here have all been run, on both machines, and their results are written into the answers above. What is left open is below.

1. **The time estimate that appears when the pointer rests on the progress bar (answers 7 and 9).** It needs a hover. Neither machine has any way of moving the pointer without a hand on it — the rented Linux machine has no `xdotool` or equivalent installed — so this was dropped rather than run.

2. **What a render does when the machine's memory runs out, on macOS (answer 10).** Answered on Linux only. The Creator's Mac is their working computer and its memory is not to be filled on purpose.

3. **Everything on Windows (answers 7 to 11).** No Windows machine has run it, so every one of these answers covers macOS and Linux and leaves Windows open.

4. **What an interactive viewport render reports (answer 9).** Not tested.

5. **What a render started from Blender loaded as a Python module reports (answer 9).** Neither machine has such a build.

6. **Which key is bound to which render command (answer 9).** With no window Blender's key configuration holds no items, so it could not be read that way.

7. **What is printed when Sample Subset is in use (answer 11).** Not tested.

---

2026-08-11 — Claude Opus 5 (Claude Code) — Reports, Blender 5.1

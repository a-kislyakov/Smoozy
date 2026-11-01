# Reports — cross-sweep questions (Blender 5.0)

Sources consulted, all read on **2026-07-30** (the snapshot date given for the copies I was handed):
Blender 5.0 manual (`Blender Sources/manual_5.0`, including its `_images` pictures), Blender 5.0 Python API documentation (`Blender Sources/api_5.0`), Blender 5.0 release notes (`Blender Sources/release_notes_5.0`), and Cycles' own files (`Blender Sources/cycles_5.0`).

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

I worked alone. No helper agents were started, so there is nobody else's reading to vouch for.

Questions answered here: 7, 8, 9, 10, 11.

---

## 7. What does a render report about its own progress, across the whole range of picture sizes a render will accept?

**Reading taken.** I read this as two things joined: what range of picture sizes a render accepts at all, and whether what it reports about its progress is the same across that whole range. It is ambiguous. The other reading is "does the *accuracy* or *granularity* of the progress report change with picture size" — I could not answer that one either, and say so below.

### The range of picture sizes accepted

**LABEL: documented.**

> resolution_x
>
> Number of horizontal pixels in the rendered image
>
> Type:
>
> int in [4, 65536], default 1920
>
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

Source: Python API documentation, https://docs.blender.org/api/5.0/bpy.types.RenderSettings.html
Date read: 2026-07-30

What the quoted words carry: each of the two size settings holds a whole number from 4 to 65536, and the percentage setting holds a whole number from 1 to 32767.

**LABEL: inferred** for the step from there to "the range of picture sizes a render accepts". The step: the quotes say what values the settings will hold, and that is read as the sizes a render will take. The words do not say a render succeeds at every size in that range, and they do not say what size comes out once the percentage is applied on top. Whether a render actually runs at either extreme is not settled here.

### What is reported about progress

**LABEL: documented**, for the two reports the sources actually describe. Neither is tied to picture size anywhere.

Inside Blender's own window:

> Running Task
>
> Shows the progress of the currently running task (such as rendering or baking).
> Hovering the mouse pointer over the progress bar will display a time estimate.
> The task can be aborted by clicking the cancel button ().

Source: manual, https://docs.blender.org/manual/en/5.0/interface/window_system/status_bar.html
Date read: 2026-07-30

The manual shows a picture of this at `_images/interface_window-system_status-bar_ui-center.png`, named in the page text as `[image: _images/interface_window-system_status-bar_ui-center.png]`. What is visible in it: a small camera icon, the words "Sequence Render", a bar filled to about nine-tenths with the number **94%** written on it, and an **X** button at the right. That is the whole of what the picture shows. **LABEL for the picture: documented** (it is part of the manual). Date read: 2026-07-30.

When Blender is run in the background, the release notes give this example of the output:

> ```
> 00:00.027  ghost.system     | Create Cocoa system
> 00:00.080  translation      | Locale en_NL used for translation
> 00:00.088  blend            | Read prefs: "/Users/blender/Library/Application Support/Blender/5.0/config/userpref.blend"
> 00:00.096  blend            | Read startup: "/Users/blender/Library/Application Support/Blender/5.0/config/startup.blend"
> 00:00.359  gpu.platform     | Using GPU "Apple M3 Max Metal API 1.2"
> 00:00.359  gpu.platform     | Using Backend "Metal"
> 00:01.041  cycles           | Added device "Apple M3 Max (GPU - 40 cores)" with id "METAL_Apple M3 Max (GPU - 40 cores)".
> ...
> 00:00.244  render           | Rendering single frame (frame 1)
> 00:00.245  render           | Start rendering: Scene, ViewLayer
> 00:00.245  render           | Engine: Cycles
> 00:26.799  render           | Mem: 2983M | Sample 1/16 
> 00:26.799  render           | Mem: 2983M | Sample 16/16
> 00:28.631  render           | Saved: '/tmp/0001.png'
> 00:28.631  render           | Time: 00:28.38 (Saving: 00:00.15)
> ```

Source: release notes, https://developer.blender.org/docs/release_notes/5.0/core/
Date read: 2026-07-30

The progress reported there is counted in **samples**, not in pixels, rows or tiles: `Sample 1/16`, `Sample 16/16`.

### Whether the report changes with picture size

**LABEL: not in the sources.**

Nothing in the manual, the API documentation, the release notes or Cycles' own files says that what a render reports about its progress varies with the size of the picture, and nothing says it stays the same either. The sources describe progress in terms of samples and a percentage bar, both of which are described without reference to resolution.

The one place picture size enters the sources at all near this subject is memory, not reporting:

> Tile Size
>
> Render high resolution images in tiles of this size, to reduce memory usage.
> Tiles are cached to disk while rendering to save memory.

Source: manual, https://docs.blender.org/manual/en/5.0/render/cycles/render_settings/performance.html
Date read: 2026-07-30

And, from Cycles' own files, the same wording with the setting's kind, default and range — **LABEL: observed**:

> ```
>     use_auto_tile: BoolProperty(
>         name="Auto Tile",
>         description="Deprecated, tiling is always enabled",
>         default=True,
>     )
>     tile_size: IntProperty(
>         name="Tile Size",
>         default=2048,
>         description="Render high resolution images in tiles of this size, to reduce memory usage. Tiles are cached to disk while rendering to save memory",
>         min=8,
>         max=8192,
>     )
> ```

File: `Blender Sources/cycles_5.0/properties.py`, lines 1051–1062 (inside `class CyclesRenderSettings`).
Date read: 2026-07-30

The release notes confirm tiling is no longer optional:

> * Remove Use Tiling option, it is now always assumed to be on. The tile size can be increased to disabled tiling. (blender/blender!146031)

Source: release notes, https://developer.blender.org/docs/release_notes/5.0/cycles/
Date read: 2026-07-30

**LABEL: inferred** for "a big picture is rendered in tiles". The step: the setting's description says it renders high resolution images in tiles, the release note says tiling is always on, and the two are put together into a statement about what a render does. A settings file, and a setting's own description, establish that the setting exists with that name, default and range; what a render does with it is a step past reading. Whether tiles ever appear in a progress report is **not in the sources**.

**Where I looked, to be able to say "not in the sources":** I searched the whole of the 5.0 manual, the whole of the 5.0 API documentation, the whole of the 5.0 release notes and Cycles' own Python files for: `progress`, `progress bar`, `status bar`, `Remaining`, `Time Remaining`, `elapsed`, `Sample 1/`, `Path Tracing Sample`, `Fra:`, `Mem:`, `Peak`, `Rendering frame`, `resolution limit`, `maximum resolution`, `max resolution`, `65536`, `Resolution X`, `large image`, `tile`, `16k`, `8k`. I read in full: the status bar page, the Cycles Performance page, the Format page (`render/output/properties/format`), the Working Limits page (`advanced/limits`), the Cycles Sampling page, the Render Layers Passes page, and `bpy.types.RenderSettings`. The Working Limits page turned out to cover space, time, and text field lengths only — no picture size — so I ruled it out. I opened the status bar centre picture. Nothing in any of these connects progress reporting to picture size.

### Observed — what a running Blender 5.0 does (2026-08-11)

Every test below was run in Blender 5.0.1 on both machines this project can reach: **the Creator's Mac — a MacBook Pro with an Apple M1 Pro chip, 16 GB of memory, macOS 26.5.2, Apple's own graphics and no second card** — and **a rented Linux machine — Ubuntu 22.04.5 LTS on kernel 6.8, two NVIDIA RTX 4000 Ada Generation cards of 20 GB each on driver 580.105.08, an AMD Ryzen Threadripper PRO 3945WX with eleven cores, 49 GB of memory**. Each run had a settings folder of its own, so nothing touched the Creator's own Blender settings. **LABEL: observed** throughout.

**The scene.** Built by Blender 5.0 itself and saved as a file: a cube, a sphere, an area light and a camera; Cycles on the processor; 16 samples; adaptive sampling off; time limit 0; 160 by 120 pixels; output PNG.

**1. The progress report does change with picture size, and the change is sharp.** The same frame was rendered from a terminal with no window six times, changing only the two resolution numbers: `blender -b <file>.blend --python-expr "import bpy; r=bpy.context.scene.render; r.resolution_x=<X>; r.resolution_y=<Y>" -o <out> -f 1`, at 4 by 4, 160 by 120, 1920 by 1080, 2048 by 2048, 2049 by 2049 and 4096 by 4096. All six exited 0 and wrote a picture of the size asked for.

At 4 by 4, 160 by 120, 1920 by 1080 and 2048 by 2048 the capture holds **three** lines carrying a sample count and mentions no tile. At 2049 by 2049 and at 4096 by 4096 it holds **thirteen**, and every one mentions a tile. Two of the thirteen from the 2049 by 2049 run on the Creator's Mac:

```
00:00.702  render           | Fra: 1 | Remaining: 00:02.99 | Mem: 241M | Rendered 0/4 Tiles, Sample 1/16
00:04.055  render           | Fra: 1 | Mem: 1M | Rendered 3/4 Tiles, Sample 16/16
```

So the stage becomes `Rendered <n>/<total> Tiles, Sample <s>/<max>` once the picture crosses one tile's worth of area. 2048 by 2048 is exactly the default tile size squared and printed no tile; 2049 by 2049 printed four. **The rented Linux machine gave the same split at the same six sizes, and the same tile counts. The two machines agreed.**

**2. The settings clamp silently, and never refuse.** Set from Python and read straight back: 0, 3 and 4 all become **4** for the two size settings; 65536, 65537 and 70000 all become **65536**; for the percentage 0 becomes **1** and 32768 and 70000 become **32767**. Nothing raised an error and nothing was printed. Identical on both machines.

**3. A render at 4 by 4 finishes and writes a 4 by 4 picture**, on both machines. **A render at the top of the range starts, tiles, and then fails to save.** At 65536 by 65536 with one sample per pixel, on the rented Linux machine, the render divided into **1024 tiles** and printed a line for each — 1026 tile lines — reaching `Rendered 1023/1024 Tiles, Sample 0/1`, with the machine's own memory use at about 3.5 GB of 49. It then stopped at the point of putting the tiles together, printing `00:00.000  reports          | ERROR Error reading tiles from file` on standard output and, on standard error:

```
OpenImageIO exited with a pending error message that was never
retrieved via OIIO::geterror(). This was the error message:
Uncompressed image size 245760.0 MB exceeds the 81920 MB limit.
Image claimed to be 65536x65536, 15-channel float. Possible corrupt input?
If this is a valid file, raise the OIIO attribute "limits:imagesize_MB".
```

The run **exited 1 and wrote no file**. All four versions did exactly the same. So a size the settings accept is not necessarily a size a render can finish. It was not run on the Creator's Mac, whose 16 GB is not to be filled on purpose.

**4. The percentage scale cuts down rather than rounds.** At 1920 by 1080 the written picture is **1920 x 1080** at 100 per cent, **960 x 540** at 50 per cent and **633 x 356** at 33 per cent — 633 is the product cut down, where rounding would give 634. Identical on both machines. **A picture of no pixels is refused, and the run still exits 0**: at 4 by 4 with the percentage at 1 nothing was written, and the message read

```
00:00.207  reports          | ERROR Image too small
```

**5. The status bar during a render, with a window open.** A render was started the way the F12 key starts it and the whole Blender window photographed with Blender's own screenshot command every few seconds. There is **exactly one bar**, never two. Beside it, in order: a small camera icon, the words **`Rendering...`**, the bar with a **whole-number percentage** written inside it, and an **X** at the right end. No decimal point appeared in any photograph. Mid-render readings: `Rendering...` beside `17%` on the Creator's Mac, `Rendering...` beside `26%` on the rented Linux machine. **No frame is named**, for a still or for an animation, and during a three-frame animation the percentage restarted at each frame rather than climbing across the range. Picture size made no difference — the same one bar and whole-number percentage at 4 by 4 and at 4096 by 4096. Both machines agreed.

**6. A sequencer render drives the same bar, and it is labelled `Rendering sequence...`** — not `Sequence Render`, which is the label in the manual's one picture. Caught mid-render at `0%`, `16%` and `32%`, on both machines.

**7. The two tiling settings, read from the running program.** `use_auto_tile` is named **Auto Tile** and described **"Deprecated, tiling is always enabled"**, default True. `tile_size` is described "Render high resolution images in tiles of this size, to reduce memory usage. Tiles are cached to disk while rendering to save memory", default 2048, range `[8, 8192]`. The three size settings: `resolution_x` and `resolution_y` default 1920 and 1080, range `[4, 65536]`; `resolution_percentage` is named **Resolution Scale**, default 100, range `[1, 32767]`. Identical on both machines.

**8. Searching Blender 5.0's own installed Cycles add-on** for the words "May work poorly with render tiling" and for a comment naming `compute_render_tile_size` found **both**, on both machines. That code is present in this version.

**9. A render engine's own progress calls work with no window.** A tiny render engine registered from a script called `update_progress`, `update_stats` and `update_memory_stats`. All three returned without complaint, with a window and without, on both machines, and none produced any output of its own.

**Not covered.** This does not settle what the time estimate says when the pointer rests on the progress bar, which needs a hover and could not be reached. It does not settle whether a 65536 by 65536 render would save into a format OpenImageIO's size limit allows, or with that limit raised. It does not settle anything about Windows.

**Outside the interface.** Settled, and it differs. With no window there is no status bar and no percentage; what a render reports instead is the printed lines quoted above, which carry a sample count, a memory figure and a `Remaining:` time estimate. The size range is the same either way — the same clamping figures came back from a windowless run, so the step marked inferred above is borne out. Both machines agreed.

**Operating system.** Settled for macOS and for Linux, and they agreed on every point above: the same three-or-thirteen sample lines at the same sizes, the same tile counts, the same clamping, the same written picture sizes, the same `ERROR Image too small`, the same `Rendering...` label and whole-number percentage, and the same settings. **Windows is untested** — no Windows machine has run it. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

---

## 8. What does a render print over the whole of a run, from its first line to its last?

**LABEL: documented**, for the one worked example the sources give. **Not in the sources** for anything stronger than "this is the example they publish".

The 5.0 release notes changed how this works and show a whole run:

> ## Logging
>
> Most of Blender now uses the same logging system,
> including operators reports and background render progress.
> The output format has changed, and log levels are easier to control.
> (blender/blender#139451)

and then, under the heading "Example output:", the block quoted in full in answer 7 above — beginning `00:00.027  ghost.system     | Create Cocoa system` and ending `00:28.631  render           | Time: 00:28.38 (Saving: 00:00.15)`.

Source: release notes, https://developer.blender.org/docs/release_notes/5.0/core/
Date read: 2026-07-30

Read as a run, that example goes: system startup, translation, preferences file, startup file, graphics card and backend, Cycles device — then (after an elision the release notes themselves print as `...`) which frame is being rendered, which scene and view layer, which engine, the sample progress with memory, the saved file path, and finally the total time with the saving time broken out.

The same release notes name switches that change how much is printed and that list the categories available. Every line in the example above does begin with a category, though the release notes do not state that as a rule:

> * `--log-level info` enables all info level log messages, with a moderate amount of output.
>   This may be useful to enable in production for troubleshooting issues.
>   `--log-level debug` provides additional output aimed at developers, while `--log-level trace`
>   is even more verbose.
> * `--log-show-memory` shows memory usage on every line.
> * `--log-show-source` shows the source code location.
> * `--log-list-categories` shows the full list of available categories.

Source: release notes, https://developer.blender.org/docs/release_notes/5.0/core/
Date read: 2026-07-30

The manual gives the same switches, with the levels spelled out:

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

> --log-file <filepath>
>
> Set a file to output the log to.

> -q, --quiet
>
> Suppress status printing (warnings & errors are still printed).

Source: manual, https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html
Date read: 2026-07-30

That last one matters for this question: a run can be made to print almost nothing.

Cycles can be asked to add its own statistics. Two sources describe the same switch, and they do not describe it identically.

The manual — **LABEL: documented**:

> --cycles-print-stats
>
> Log statistics about render memory and time usage.

Source: manual, https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html
Date read: 2026-07-30

Cycles' own file — **LABEL: observed**:

> ```
>     parser.add_argument("--cycles-print-stats",
>                         help="Print rendering statistics to stderr",
>                         action='store_true')
> ```

File: `Blender Sources/cycles_5.0/engine.py`, lines 13–15, inside `_configure_argument_parser()`.
Date read: 2026-07-30

The file's own help text says these statistics go to **stderr**; the manual's wording does not say where they go. I report both and do not reconcile them.

Where the output is seen at all:

> The Console Window, also called a Terminal, is an operating system text window that displays
> messages about Blender's operations, status, and internal errors.
>
> When Blender is manually started from a terminal,
> Blender output is shown in the Console Window it is started from.

Source: manual, https://docs.blender.org/manual/en/5.0/advanced/command_line/index.html
Date read: 2026-07-30

**One picture, and a warning about it.** The manual's command-line page for Windows names `[image: _images/advanced_command-line_launch_windows_console.png]`. I opened it. It shows a Command Prompt in which `blender` is run and prints four lines about preferences, bundled Python and a read blend-file, then a blank line, then `Blender quit`, then the prompt returns. **LABEL: documented** for what the picture shows. But it must not be used as 5.0 evidence. What is visible is the two paths `C:\blender\2.77` and `Blender\2.77\config`, and no render anywhere in the window. Reading those paths as the mark of a much older Blender is a step a reader supplies, so **LABEL: inferred** for that step — it is still enough to keep the picture out of any 5.0 claim. I flag it rather than lean on it. The Linux equivalent, `_images/advanced_command-line_launch_linux_console.png`, shows a run whose first lines are messages from the JACK audio library and OpenAL, not from Blender — again a plain launch with no render.

**No record of looking.** This answer keeps no record of the searching behind its "not in the sources" half — which pages were opened, which words were searched for, what was ruled out. Every other answer in this document keeps one. Without it, a thorough miss cannot be told from a lazy one, so that half stands unsupported until the search is done again and written down. The same gap covers the two pictures named just above: neither carries the address of the page it sits on, nor a date read.

### Observed — the whole of a Blender 5.0 render run (2026-08-11)

Run in Blender 5.0.1 on **the Creator's Mac (a MacBook Pro with an Apple M1 Pro chip, 16 GB of memory, macOS 26.5.2, Apple's own graphics and no second card)** and on **a rented Linux machine (Ubuntu 22.04.5 LTS on kernel 6.8, two NVIDIA RTX 4000 Ada Generation cards of 20 GB each, eleven cores, 49 GB of memory)**, each with a settings folder of its own. The scene was built by Blender 5.0 itself: a cube, a sphere, an area light and a camera; Cycles on the processor; 16 samples; adaptive sampling off; 160 by 120; output PNG. **LABEL: observed.**

**1. The whole of one run, first line to last.** `blender -b <file>.blend -o <out> -f 1 > out.txt 2> err.txt` exited 0, wrote its picture, and put **nothing at all on standard error**. Standard output held fifty-four lines. The first seven, the last of the stage lines, all three sample lines and the last four, word for word, from the Creator's Mac:

```
00:00.373  blend            | Read blend: "…/scenes/base_5.0.blend"
00:00.378  render           | Rendering single frame
00:00.378  render           | Rendering frame 1
00:00.378  render           | Start rendering: Scene, ViewLayer
00:00.378  render           | Engine: Cycles
00:00.388  render           | Fra: 1 | Mem: 0M | Synchronizing object | Cube
Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:51:40)
…
00:00.395  render           | Fra: 1 | Mem: 2M | Loading denoising kernels (may take a few minutes the first time)
00:00.395  render           | Fra: 1 | Mem: 2M | Sample 0/16
00:00.398  render           | Fra: 1 | Remaining: 00:00.05 | Mem: 3M | Sample 1/16
00:00.564  render           | Fra: 1 | Mem: 3M | Sample 16/16
00:00.564  render           | Fra: 1 | Mem: 3M | Finished
00:00.617  render           | Saved: '…/out/e1_5.0_0001.png'
00:00.617  render           | Time: 00:00.23 (Saving: 00:00.05)

Blender quit
```

So **the first line is `blend | Read blend:` and the last is `Blender quit`, with one blank line before it.** The version banner is not the first line: it arrives seven lines in, in the middle of the run, and it carries no timestamp and no category. Every other line does carry both, separated by a vertical bar. **Exactly three lines carry a sample count for a sixteen-sample frame** — 0/16, 1/16 and 16/16 — never one line per sample. Only the `Sample 1/16` line carries a time estimate, called `Remaining`. **The rented Linux machine gave the same fifty-two to fifty-four lines in the same order, with nothing on standard error. The two machines agreed.**

**How this compares with the example the release notes publish.** The published example's render lines read `Mem: 2983M | Sample 1/16`; what 5.0.1 actually prints is `Fra: 1 | Remaining: 00:00.05 | Mem: 3M | Sample 1/16` — a frame field and a time estimate the example does not show. The example's opening line reads `Rendering single frame (frame 1)` as one line; 5.0.1 prints `Rendering single frame` and `Rendering frame 1` as two. The example's elision hid nothing but stage lines: there is no line per sample in between, only the three quoted above.

**2. Quiet mode removes three lines, not the render.** `blender -b -q <file>.blend -o <out> -f 1` exited 0, wrote its picture, and left fifty lines. Comparing the two captures line by line, `-q` removes exactly three: the `blend | Read blend:` line, the blank line before the end, and `Blender quit`. Every render line survives, including all three sample lines and the `Saved:` and `Time:` lines, and so does the version banner. The same on both machines.

**3. Cycles' own statistics go to standard output, not standard error, and carry no sample count.** With `-- --cycles-print-stats` the run exited 0 and **standard error was empty on both machines**. The statistics arrive in two blocks: one headed `Update statistics:` during the Updating stages, one headed `Render statistics:` **after the `Saved:` and `Time:` lines**, near the end of the run.

> **Corrected by the citation check (2026-08-11).** This paragraph said the second block arrived "after the last sample line and before `Saved:`". Repeating the run on both machines, twice each, put it after `Saved:` and after `Time:` in this version. Blender 4.5 does place it before `Saved:`, which is what its own document says. Corrected to what the runs show; the placing is also sent to the hand tests to be settled by a second pair of hands.
>
> **Also corrected by the citation check (2026-08-11).** A block of `Render statistics:` figures stood here, quoting `Total memory: 24.03K (24,607)`, `Sphere 23.55K (24,113)`, `Cube 413 (413)` and a kernel line reading `Total 100.00% (0.14s), Self 28.68% (0.04s)`. Those are, to the byte and to the hundredth of a second, the figures the Blender 4.5 document quotes, and repeating the run showed that 4.5 and 5.0 do **not** produce the same figures for the same scene — 5.0 gave `24.04K (24,615)`, `Sphere 23.55K (24,117)` and `Cube 417 (417)`. The block has been cut rather than replaced, because a checker's own run must not be written into a findings document as though it were the researcher's. Capturing the real block for this version is a hand test.

**No sample count appears anywhere in either block.** Without the bare `--` the switch is taken as a file name and the run **exits 1**, printing `00:00.561  reports          | ERROR Cannot read file "…/--cycles-print-stats": No such file or directory`.

**Observed — the block this version actually prints, captured by a second pair of hands (2026-08-12).** This fills the gap the citation check left when it cut the block above. Run on **the Creator's Mac (a MacBook Pro with an Apple M1 Pro chip, 16 GB of memory, macOS 26.5.2, Apple's own graphics and no second card)** and on **a rented Linux machine (Ubuntu 22.04.5 LTS on kernel 6.8, two NVIDIA RTX 4000 Ada Generation cards of 20 GB each on driver 580.105.08, an AMD Ryzen Threadripper PRO 3945WX with eleven cores, 49 GB of memory)**, each version with a settings folder of its own and its own copy of the scene: a cube, a sphere, an area light and a camera; Cycles on the processor; 16 samples; adaptive sampling off; 160 by 120; output PNG. The command was `blender -b <file>.blend -o <out> -f 1 -- --cycles-print-stats`, standard output and standard error kept apart. The run exited 0 and standard error was empty. **LABEL: observed.**

**The whole block, from the Creator's Mac, word for word:**

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
  Total render time               : Total 100.00% (0.12s), Self 4.17% (0.01s)
    Shade Surface                   : Total 46.67% (0.06s), Self 0.00% (0.00s)
      Direct Light                    : Total 22.50% (0.03s), Self 22.50% (0.03s)
      Shader Evaluation               : Total 12.50% (0.01s), Self 12.50% (0.01s)
      Indirect Light                  : Total 8.33% (0.01s), Self 8.33% (0.01s)
      Setup                           : Total 2.50% (0.00s), Self 2.50% (0.00s)
      Render Passes                   : Total 0.83% (0.00s), Self 0.83% (0.00s)
      Ambient Occlusion               : Total 0.00% (0.00s), Self 0.00% (0.00s)
    Intersect Closest               : Total 19.17% (0.02s), Self 19.17% (0.02s)
    Ray setup                       : Total 18.33% (0.02s), Self 18.33% (0.02s)
    Intersect Shadow                : Total 10.83% (0.01s), Self 10.83% (0.01s)
    Shade Light                     : Total 0.83% (0.00s), Self 0.00% (0.00s)
      Setup                           : Total 0.83% (0.00s), Self 0.83% (0.00s)
      Shader Evaluation               : Total 0.00% (0.00s), Self 0.00% (0.00s)
    Intersect Subsurface            : Total 0.00% (0.00s), Self 0.00% (0.00s)
    Intersect Volume Stack          : Total 0.00% (0.00s), Self 0.00% (0.00s)
    Intersect Blocked Light         : Total 0.00% (0.00s), Self 0.00% (0.00s)
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
Shader statistics:
  default_surface                 : 0.05s (Relative cost: 1.00)
Object statistics:
  Cube                            : 0.04s (Relative cost: 1.12)
  Sphere                          : 0.02s (Relative cost: 0.80)
```

**The rented Linux machine printed the same ten lines above `Kernel statistics:`, figure for figure.** Its own kernel line read `  Total render time               : Total 100.00% (0.32s), Self 4.70% (0.01s)`. **The two machines agreed on the byte counts and differed on the times.** Everything below `Kernel statistics:` is times and percentages, and those changed from one run to the next on the same machine, so only the byte counts should be read as this version's own. **There is no sample count anywhere in the block.**

**The figures are not Blender 4.5's.** Run the same way on the same two machines on the same day, Blender 4.5 printed `Total memory: 24.03K (24,607)`, `Sphere 23.55K (24,113)`, `Cube 413 (413)`, `Area 81 (81)` — different in three of the four figures from this version's. Blender 5.1 printed the same figures as this version. Blender 5.2 printed `Total memory: 16.36K (16,750)`, `Sphere 16.02K (16,405)`, `Cube 289B (289)`, `Area 56B (56)`, and wrote its image section as `Image statistics:` then `  Total image memory: 0B (0)`, where this version writes `Image statistics:`, `  Textures:`, `    Total memory: 0 (0)`.

**Where the block lands, counted by line number (2026-08-12).** The same command was run again and the captured lines numbered. On the Creator's Mac standard output held 130 lines: `Update statistics:` at 15, `Sample 16/16` at 81, `Finished` at 82, `Saved:` at 83, `Time:` at 84, `Render statistics:` at 85. On the rented Linux machine it also held 130 lines with exactly the same numbers. **The two machines agreed, and both bear out the correction above: in this version the block comes after `Saved:` and after `Time:`.** Blender 4.5 run the same way put `Render statistics:` at line 78 and `Saved:` at 122 on the Mac — before `Saved:`, the other way round.

**4. What the logging switches add, and the list of categories.** Against the fifty-four lines above, `--log-level info` gives **138** lines and `--log "*"` gives **138** on the Creator's Mac. The extra lines name the graphics device, the denoiser, the file-reading times and the translation locale. `--log-list-categories` works and prints **135** category names on the Creator's Mac and **169** on the rented Linux machine — so the list is longer on Linux — and **both `render` and `cycles` are in it** on both machines.

> **Corrected by the citation check (2026-08-11).** The Linux figure read 170. `--log-list-categories` prints 170 *lines* on that machine, of which the first is the Blender version banner, so the count of categories is 169. The Mac figure was already counted the right way.

**5. An animation announces the range and each frame.** `blender -b <file>.blend -o <out> -a` over three frames exited 0 and wrote three pictures. From the capture on the Creator's Mac:

```
00:00.261  render           | Rendering animation (frames 1..3)
00:00.261  render           | Rendering frame 1
…
00:00.408  render           | Rendering frame 2
…
00:00.493  render           | Rendering frame 3
```

Each frame carries its own `Start rendering`, `Engine:`, stage lines, three sample lines, `Finished`, `Saved:` and `Time:`. Nothing is printed after the last frame's `Time:` line except the blank line and `Blender quit`. The same on both machines.

**6. The other two engines print differently.** EEVEE is called `BLENDER_EEVEE` in this version, not `BLENDER_EEVEE_NEXT`. It prints four progress lines of the shape `00:00.790  render           | Fra: 1 | Rendering 1 / 64 samples`, at 1, 25, 50 and 64 of 64 — **with no memory figure and no time figure**. The scene asked for sixteen Cycles samples; EEVEE used its own count of 64 and printed nothing about the difference. **Workbench prints no progress line at all**, going straight from the announcements to `Saved:` and `Time:`. The same on both machines.

**7. What the render-statistics handler is handed.** A script attached a handler to `bpy.app.handlers.render_stats` and wrote every call down. With no window it fired **forty-five times for one frame** — once per stage, not once per render — and was handed two arguments, a string and `None`. The first, a middle and the last:

```
STATS ('Mem: 0M | Synchronizing object | Cube', None)
STATS ('Remaining: 00:00.05 | Mem: 3M | Sample 1/16', None)
STATS ('Time: 00:00.20 (Saving: 00:00.04)', None)
```

**The string carries no frame number in this version** — only a memory figure, an optional `Remaining:` and the stage. **With a window open the same handler fired once**, and was handed only the closing line, `STATS ('Time: 00:00.12 (Saving: 00:00.01)', None)`. The same on both machines.

**Not covered.** This does not settle what a run prints when it fails rather than succeeds, which is a different question on the sheet. It does not settle what is printed when the output is a video file. It does not settle anything about Windows.

**Outside the interface.** Settled by the tests above, and every line quoted there was captured from a terminal with no window. Two things differ from a run with a window open. **No progress line reaches the terminal at all when there is a window** — all that arrives is `Saved:`. And the render-statistics handler is handed forty-five strings with no window and one string with a window. Both machines agreed.

**Operating system.** Settled for macOS and for Linux, and the printed content agreed line for line: the same fifty-two to fifty-four lines in the same order, the same first and last line, the same three sample lines, the same three lines removed by `-q`, the same statistics on standard output with an empty standard error, the same animation announcements, the same EEVEE and Workbench shapes, and the same handler strings. **One difference between the machines**: the list of logging categories is 135 entries on the Mac and 169 on the Linux machine (corrected from 170 by the citation check, 2026-08-11 — the Linux figure had counted the version banner as a category). **Checked again on 2026-08-12 for Cycles' statistics**: the block's byte counts and its place in the run came back the same on both machines — the same six line numbers out of 130 lines — and only the times inside the block differed. **Windows is untested** — no Windows machine has run it — so the Windows-only switch `-con, --start-console` and the hidden Windows console stand unchecked.

---

## 9. What does a render report about its progress under each of the ways a render can be started?

### The ways the sources name

**LABEL: documented.**

From Blender's own Render menu:

> Render Image F12
>
> Render the active scene at the current frame.
>
> Render Animation Ctrl-F12
>
> Render the animation of the active scene.

> Render Sequencer Image Alt-F12
>
> If a Sequencer Scene exists that differs from the active scene, render that scene instead.
>
> Render Sequencer Animation Ctrl-Alt-F12
>
> Render the animation of the Sequencer Scene.

Source: manual, https://docs.blender.org/manual/en/5.0/interface/window_system/topbar.html
Date read: 2026-07-30

From the command line:

> -b, --background
>
> Run in background (often used for UI-less rendering).

> -a, --render-anim
>
> Render frames from start to end (inclusive).

> -f, --render-frame <frame>
>
> Render frame <frame> and save it.

Source: manual, https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html
Date read: 2026-07-30

From Python:

> bpy.ops.render.render(*, animation=False, write_still=False, use_viewport=False, use_sequencer_scene=False, layer='', scene='', frame_start=0, frame_end=0)
>
> Undocumented, consider contributing.

Source: Python API documentation, https://docs.blender.org/api/5.0/bpy.ops.render.html
Date read: 2026-07-30

There is also a viewport render, `bpy.ops.render.opengl`, but the manual limits it:

> Viewport rendering only works for the Workbench and EEVEE render engines.

Source: manual, https://docs.blender.org/manual/en/5.0/editors/3dview/viewport_render.html
Date read: 2026-07-30

### What each reports

**Started by a person inside Blender's window** — **LABEL: documented.** The status bar's Running Task entry, quoted in full in answer 7: a progress bar, a time estimate on hover, and a cancel button. The picture I opened shows it filled to `94%` with the label "Sequence Render".

**Started from the command line in background mode** — **LABEL: documented.** The log lines quoted in answer 7, with the release notes saying in so many words that background render progress now goes through this system:

> Most of Blender now uses the same logging system,
> including operators reports and background render progress.

and, in the Python API release notes:

> ### Logging
>
> - The format of logging output has changed,
>   including background render progress.
>   Add-ons or render farms parsing this output may need to be updated.

Source: release notes, https://developer.blender.org/docs/release_notes/5.0/python_api/
Date read: 2026-07-30

**The one place the sources tie reported content to the way of starting** — **LABEL: documented.** The Python handler that fires when render statistics are printed says the content is different in background mode:

> bpy.app.handlers.render_stats
>
> on printing render statistics. Accepts one argument: the render stats (render/saving time plus in background mode frame/used [peak] memory).

Source: Python API documentation, https://docs.blender.org/api/5.0/bpy.app.handlers.html
Date read: 2026-07-30

Read plainly: render time and saving time are reported either way; the frame number and the used and peak memory are added **in background mode**.

**What a render engine is given to report with** — **LABEL: documented** for the calls themselves:

> update_stats(stats, info)
>
> Update and signal to redraw render status text

> update_progress(progress)
>
> Update progress percentage of render
>
> Parameters:
>
> progress (float in [0, 1]) – Percentage of render that's done

> update_memory_stats(*, memory_used=0.0, memory_peak=0.0)
>
> Update memory usage statistics
>
> Parameters:
>
> memory_used (float in [0, inf], (optional)) – Current memory usage in megabytes
>
> memory_peak (float in [0, inf], (optional)) – Peak memory usage in megabytes

Source: Python API documentation, https://docs.blender.org/api/5.0/bpy.types.RenderEngine.html
Date read: 2026-07-30

**LABEL: inferred** for "the same regardless of how the render was started". The step: these are calls a render engine makes itself, so they are read as available whichever way a render began. The quoted words say nothing about how a render was started.

**Started from Python** — **LABEL: not in the sources.** `bpy.ops.render.render` is marked "Undocumented, consider contributing", and nothing anywhere says whether a render started that way reports through the status bar, the log, both, or neither. Note that Python can be run inside Blender's window or from a background run started with `-P`, and the sources do not separate those.

**Started as a Sequencer render, or as a viewport render** — **LABEL: not in the sources** for progress reporting. What the status bar picture shows is a progress bar carrying the words "Sequence Render" — that much is **documented** by the picture. Reading it as "a sequencer render drives the same progress bar" is a step a reader supplies: the picture does not say what started the task, nor that those words name the task's kind. So **LABEL: inferred** for that reading. Nothing says what a viewport render reports.

**Where I looked:** I searched the whole 5.0 manual, API documentation, release notes and Cycles' Python files for `progress`, `status bar`, `render progress`, `background render`, `Render Result`, `render_stats`, `update_progress`, `update_stats`, `stdout`, `console`, `terminal`, `quiet`. I read in full: the topbar page (Render menu), the status bar page, the command-line arguments page, the command-line rendering page, the command-line index page, the viewport render page, `bpy.ops.render`, `bpy.app.handlers`, `bpy.types.RenderEngine`, and the Core and Python API release notes. Beyond the `render_stats` sentence, the sources never set the ways of starting side by side.

### Observed — what Blender 5.0 reports under each way of starting (2026-08-11)

Run in Blender 5.0.1 on **the Creator's Mac (a MacBook Pro with an Apple M1 Pro chip, 16 GB of memory, macOS 26.5.2, Apple's own graphics and no second card)** and on **a rented Linux machine (Ubuntu 22.04.5 LTS, two NVIDIA RTX 4000 Ada Generation cards of 20 GB each, eleven cores, 49 GB of memory)**, each with a settings folder of its own. The same frame from the same file — Cycles on the processor, 16 samples, adaptive sampling off, 160 by 120 — was started five ways and the terminal captured each time. **LABEL: observed.**

**The way of starting changes what is printed, and a render started from Python prints nothing.** Counting the lines that carry a sample count:

| how the render was started | sample lines printed |
|---|---|
| `blender -b <file>.blend -o <out> -f 1` | 3 |
| `blender -b <file>.blend -o <out> -a` (three frames) | 3 per frame |
| `blender -b <file>.blend -o <out> --python <script>` calling `bpy.ops.render.render(write_still=True)` | **0** |
| a window open, the same call from a script | 0 |
| a window open, started the way the F12 key starts it — `bpy.ops.render.render('INVOKE_DEFAULT', write_still=True)` | 0 |

The whole capture of the Python case with no window, on the Creator's Mac:

```
00:00.285  blend            | Read blend: "…/scenes/base_5.0.blend"
00:00.445  render           | Saved: '…/out/e14c2_5.0_.png'
OPERATOR RETURNED: {'FINISHED'}
Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:51:40)

Blender quit
```

Three lines of Blender's own, and no progress of any kind. It was run twice, once with the call in a `--python-expr` and once in a script file, with the same result both times, on both machines.

**What the operator hands back.** Called plainly it returns `{'FINISHED'}`; started the way the F12 key starts it, `{'RUNNING_MODAL'}`, with the render going on afterwards. A picture was written in every case.

**What the ways of starting actually are, read from the running program.** The Python render operator takes **eight** arguments — `animation`, `write_still`, `use_viewport`, `use_sequencer_scene`, `layer`, `scene`, `frame_start`, `frame_end` — and every one carries a description, contrary to the API page's "Undocumented, consider contributing" for the operator itself. The Render menu draws **four** render commands: `Render Image`, `Render Animation`, `Render Sequencer Image`, `Render Sequencer Animation`, then `Render Audio...`, `View Render`, `View Animation` and `Lock Interface`. The 3D viewport's View menu draws `Render Viewport Preview`, `Render Playblast` and `Render Playblast on Keyframes`. There is **no** `save_output` setting in this version. Identical on both machines.

**A sequencer render does drive the status bar progress bar, and it is labelled `Rendering sequence...`** — not `Sequence Render`, which is the label in the manual's one picture of that bar. A file whose video sequencer held a scene strip was rendered with a window open and photographed with Blender's own screenshot command; the bar was caught at `0%`, `16%` and `32%`, with the same camera icon and X as an ordinary render. So the picture's label cannot be read as showing what a sequencer render is called. The same on both machines. With no window the same render prints `Rendering frame 1` and then the ordinary three sample lines.

**The status bar for an ordinary render is labelled `Rendering...`**, with one bar, a whole-number percentage and an X — the full reading is under question 7 above. No frame is named, and an animation's percentage restarts at each frame.

**What the render-statistics handler is handed differs by way of starting.** With no window it fires forty-five times per frame, the strings carrying a memory figure and the stage but **no frame number**. With a window it fires once, carrying only `STATS ('Time: 00:00.12 (Saving: 00:00.01)', None)`. The same on both machines.

**A render engine's own progress calls behave the same either way.** `update_progress`, `update_stats` and `update_memory_stats`, called from a tiny engine registered from a script, all returned without complaint with a window and without, on both machines, and none produced output of its own.

**Observed — a windowed run with the handler attached, and how it was closed (2026-08-12).** A later hand test repeated the windowed run in all four versions on both machines, because no earlier record says how a windowed Blender was made to end, and the exit status of such a run belongs to whoever ended it. Run on **the Creator's Mac (a MacBook Pro with an Apple M1 Pro chip, 16 GB of memory, macOS 26.5.2, Apple's own graphics and no second card)** and on **a rented Linux machine (Ubuntu 22.04.5 LTS on kernel 6.8, two NVIDIA RTX 4000 Ada Generation cards of 20 GB each on driver 580.105.08, an AMD Ryzen Threadripper PRO 3945WX with eleven cores, 49 GB of memory), whose window was opened on its own Xorg display**. Each version had a settings folder of its own and its own copy of the scene. `/tmp` was emptied of `*.crash.txt` before every run. A start-up script attached a handler to `bpy.app.handlers.render_stats` and another to `bpy.app.handlers.render_complete`, called `bpy.ops.render.render(write_still=True)`, printed the counts and then closed Blender. **LABEL: observed.**

**How a windowed Blender is closed is part of the answer.** `bpy.ops.wm.quit_blender()` called straight from the start-up script did not close the run on the rented Linux machine: the script printed `STILL HERE AFTER quit_blender()` and Blender was still going when a 120-second cap ended it, and again when a 240-second cap ended it — both times the status was 124, which belongs to `timeout` and not to Blender. On the Creator's Mac the same call did close the run, exiting 0, but 24.7 seconds after the render in one run and 6.3 seconds after it in another. A timer registered with `bpy.app.timers.register`, calling `bpy.ops.wm.quit_blender()` three seconds after the render, closed the run on both machines every time, and that is how the run below was closed.

**Closed by the timer, Blender 5.0 exited 0 on both machines, with no crash file.** The whole of standard output from the Creator's Mac, long paths shortened:

```
00:00.690  blend            | Read blend: "…/scenes/base_5.0.blend"
BACKGROUND=False
VERSION=5.0.1
00:00.851  render           | Saved: '…/out/e21_5.0_timer_.png'
HANDLER FIRED 1 TIMES
FIRST=('Time: 00:00.15 (Saving: 00:00.04)', None)
LAST=('Time: 00:00.15 (Saving: 00:00.04)', None)
RENDER_COMPLETE FIRED 1 TIMES
QUITWAY=timer
TIMER REGISTERED FOR 3s
TIMER FIRED, CALLING quit_blender()
STILL HERE AFTER quit_blender() IN TIMER
00:03.873  blend            | Saved session recovery to "/var/folders/n3/z3rfjt654rn1k1x6f_1gk4540000gn/T/quit.blend"
```

The lines with no timestamp are the test script's own printing. `Blender quit` followed after a blank line, and standard error was empty. **The rented Linux machine gave the same run — exit 0, handler once, `render_complete` once, standard error empty** — with one line different: where the Mac saved a session recovery file, the Linux machine printed `00:04.030  reports          | ERROR Cannot open file /tmp/quit.blend@ for writing: Permission denied`.

**So the count this document already gives is borne out: with a window the handler fires once**, carrying only the closing `Time:` line. **No run of this version on either machine ended in a segmentation fault, and no crash file was written in `/tmp`.**

**Not covered.** This does not settle what the time estimate says when the pointer rests on the progress bar — that needs a hover, and neither machine can move the pointer without a hand on it. It does not settle which key is bound to which render command: with no window Blender's key configuration holds no items. It does not settle what a viewport render or the Rendered shading mode reports. It does not settle anything about Windows.

**Outside the interface.** Settled, and it is the heart of this answer. A render started **from the command line with `-f` or `-a`** and no window prints its full progress. A render started **from Python** with no window prints nothing but the file it saved. A render started with **a window open** prints nothing in any way of starting, and what stands in for it is the status bar's one labelled bar with a whole-number percentage. The `render_stats` handler is handed forty-five strings with no window and one with a window. Both machines agreed. **Added 2026-08-12:** a windowed run started from a terminal, with the handler attached, ended cleanly on both machines — exit 0, no crash file — when closed by a timer calling `bpy.ops.wm.quit_blender()`; the same call made straight from the start-up script did not close a windowed Blender on the Linux machine at all.

**Operating system.** Settled for macOS and for Linux, and they agreed on every point above: the same counts of printed sample lines for all five ways, the same operator arguments and menu entries, the same `Rendering...` and `Rendering sequence...` labels and whole-number percentages on each machine's own screen. **One difference between the machines, found on 2026-08-12**: `bpy.ops.wm.quit_blender()` called from a start-up script closed a windowed Blender on the Creator's Mac, after six seconds in one run and twenty-five in another, and did not close it on the rented Linux machine within four minutes. At quit time the Mac printed `Saved session recovery to "/var/folders/…/T/quit.blend"` and the Linux machine printed `ERROR Cannot open file /tmp/quit.blend@ for writing: Permission denied`. Closed by a timer, both machines exited 0. **Windows is untested**, no Windows machine having run it, so the Windows-only `-con, --start-console` stands unchecked.

---

## 10. What does a render do when the machine's own memory runs out while it is sampling?

**LABEL: not in the sources.**

Nothing in the 5.0 manual, the 5.0 API documentation, the 5.0 release notes or Cycles' own files describes what a render does when the machine's own memory is exhausted during sampling. There is no described fallback, no described message, no described abort, no described exit behaviour for that case.

What the sources *do* say, none of which answers the question:

**Running out of memory is named as a cause of crashes, for Blender as a whole, not for a render mid-sampling** — **LABEL: documented**:

> Crashes
>
> The most common causes of Blender crashes:
>
> Running out of memory.
>
> Issues with graphics hardware or drivers.
>
> Bugs in Blender.

and, on the same page:

> When Blender crashes, it writes out a text file which contains information that may help identify the cause of the
> crash. Usually, this file is written in the Temporary Directory directory.

Source: manual, https://docs.blender.org/manual/en/5.0/troubleshooting/crash.html
Date read: 2026-07-30

**A report type for it exists** — **LABEL: documented**:

> ERROR_OUT_OF_MEMORY:
>
> Out of Memory.

Source: Python API documentation, https://docs.blender.org/api/5.0/bpy_types_enum_items/wm_report_items.html
Date read: 2026-07-30

That says such a report *can* be raised. It does not say a render raises it, or when.

**The out-of-memory message the sources do describe is the graphics card's, not the machine's** — **LABEL: documented**:

> Error: Out of memory
>
> This usually means there is not enough memory to store the scene for use by the GPU.

and, on the same page, the fallback runs the *other* way from what this question asks:

> With CUDA, OptiX, HIP and Metal devices, if the GPU memory is full Blender will automatically
> try to use system memory. This has a performance impact, but will usually still result in a faster render
> than using CPU rendering.

Source: manual, https://docs.blender.org/manual/en/5.0/render/cycles/gpu_rendering.html
Date read: 2026-07-30

That is graphics-card memory running out and the machine's memory being borrowed. It says nothing about the machine's own memory then running out too.

**Measures that exist to keep memory down during a render** — **LABEL: documented**: the Tile Size setting quoted in answer 7 ("Render high resolution images in tiles of this size, to reduce memory usage. Tiles are cached to disk while rendering to save memory"), Use Compact BVH ("Use a more compact BVH structure, which can reduce RAM usage but render slower"), and the Lower Memory performance preset ("Uses settings to decrease memory considered at the cost of slower renders") — all from https://docs.blender.org/manual/en/5.0/render/cycles/render_settings/performance.html, read 2026-07-30. Also, from the topbar page, "Lock Interface — Lock interface during rendering in favor of giving more memory to the renderer." None of these say what happens when memory nevertheless runs out.

**Where I looked, to be able to say "not in the sources":** I searched the whole 5.0 manual, the whole API documentation, the whole release notes and Cycles' Python files, case-insensitively, for: `out of memory`, `out-of-memory`, `OOM`, `ran out of memory`, `insufficient memory`, `memory error`, `memory limit`, `low memory`, `system memory`, `RAM`, `swap`, `memory usage`, `Save Buffers`, `exit code`, `return code`. I read in full: the Crashes troubleshooting page, the Cycles GPU Rendering page (including its whole Error Messages section), the Cycles Performance page, `wm_report_items`, `info_gotchas_crashes`, and the Cycles release notes. The only hits that touch running out of memory are the ones quoted above; every one of them is either about Blender crashing in general, about the graphics card, or about avoiding the situation. The Cycles release notes for 5.0 mention memory only in connection with NanoVDB grids and object-space adaptive subdivision, so I ruled them out.

### Observed — Blender 5.0 fills the machine and is killed, saying nothing (2026-08-11)

Run on **a rented Linux machine only — Ubuntu 22.04.5 LTS on kernel 6.8, two NVIDIA RTX 4000 Ada Generation cards of 20 GB each on driver 580.105.08, an AMD Ryzen Threadripper PRO 3945WX with eleven cores, 49 GB of memory**. It was **not** run on the Creator's Mac: that is their working computer and its memory is not to be filled on purpose. So this answer covers Linux and leaves macOS untested. **LABEL: observed.**

**The scene.** A sphere, a cube, an area light and a camera; Cycles on the processor; 2000 samples; adaptive sampling off; 8192 by 8192 pixels; tile size 8192; every render pass switched on, cryptomatte at sixteen levels, and forty light groups added. Rendered with `blender -b <file>.blend -o <out> -f 1`, with the machine's memory read once a second alongside.

**Getting the memory to run out while sampling took three attempts, and the first two are results of their own.** More pixels do not do it: at 12288 by 12288 the run reached 48.9 GB of the machine's 49 and **kept sampling**, and at 16384 by 16384 Blender's own figure was the same again. Tiling caps what is held — the tile size cannot exceed 8192 — so the buffers stop growing with the picture. What did it was more passes.

**What happened.** The last line on standard output is a sampling line:

```
00:00.389  render           | Fra: 1 | Mem: 9M | Sample 0/2000
```

**Standard error was empty. Blender printed nothing whatever about memory.** No picture was written and no crash file appeared. The process ended with status **137**. The machine's own memory use reached 50,036 MB and, in a repeat run, 49,682 MB, of 49,152. The kernel log held:

```
Out of memory: Killed process 104500 (blender) total-vm:102192540kB, anon-rss:48476848kB, file-rss:3124kB, shmem-rss:0kB, UID:0 pgtables:958
```

The whole test was run twice, the second time with the four versions taken in the opposite order, and Blender 5.0 behaved the same way both times. **Blender 5.1 and 5.2 behave differently on the same machine with the same scene** — they refuse the allocation and say so — which is recorded in their own documents.

**Not covered.** This does not settle what happens on macOS or on Windows. It does not settle whether the answer differs when the render is on a graphics card rather than the processor. It does not settle what a window-open run would show, since this was run with no window.

**Outside the interface.** Settled for a run with no window, which is what was tested: the process is killed with status 137, nothing is printed, and nothing is written. What a run with a window would show was not tested.

**Operating system.** Settled for Linux only. macOS is untested, because the Creator's working machine must not have its memory filled on purpose, and **Windows is untested**, no Windows machine having run it. What the manual says about where a crash file lands therefore stands unchecked for those two — and on Linux, where it was tested, **no crash file was written at all**.

---

## 11. What, if anything, does a render print that shows whether a frame took the number of samples it was asked for?

### What is printed

**LABEL: documented**, for the fact that these lines appear in the release notes' example of a background render:

> ```
> 00:26.799  render           | Mem: 2983M | Sample 1/16 
> 00:26.799  render           | Mem: 2983M | Sample 16/16
> ```

Source: release notes, https://developer.blender.org/docs/release_notes/5.0/core/
Date read: 2026-07-30

Those lines carry a pair of numbers: a sample reached and a sample total. In this example the last one reads `16/16`.

**LABEL: inferred**, for the claim that this is what shows whether the frame took the number of samples it was asked for. The step I took: the example's scene was asked for 16 samples (that is the second number), the final printed line reads `Sample 16/16`, so the two numbers being equal is what a reader would compare. The release notes never say the second number is the requested sample count, and never say the line is printed when sampling stops early. A reader has to supply both of those, which is exactly why this is inferred and not documented.

### Two documented reasons the two numbers could differ

**LABEL: documented.**

> Time Limit
>
> Stops rendering if the time exceeds the limit, even if the desired sample count
> hasn't been reached yet. Setting this value to zero disables the limit.

> Adaptive Sampling
>
> If the Noise Threshold checkbox is enabled, Cycles will use adaptive sampling,
> cutting short the sampling process in areas that have become less noisy than the
> specified threshold value

Source: manual, https://docs.blender.org/manual/en/5.0/render/cycles/render_settings/sampling.html
Date read: 2026-07-30

The same two settings from Cycles' own file, with kind, default and range — **LABEL: observed**:

> ```
>     samples: IntProperty(
>         name="Samples",
>         description="Number of samples to render for each pixel",
>         min=1, max=(1 << 24),
>         default=4096,
>     )
> ```

File: `Blender Sources/cycles_5.0/properties.py`, lines 479–484.

> ```
>     time_limit: FloatProperty(
>         name="Time Limit",
>         description="Limit the render time (excluding synchronization time). "
>         "Zero disables the limit",
>         min=0.0,
>         default=0.0,
>         step=100.0,
>         unit='TIME_ABSOLUTE',
>     )
> ```

File: `Blender Sources/cycles_5.0/properties.py`, lines 514–522.

> ```
>     use_adaptive_sampling: BoolProperty(
>         name="Use Adaptive Sampling",
>         description="Automatically reduce the number of samples per pixel based on estimated noise level",
> ```

File: `Blender Sources/cycles_5.0/properties.py`, lines 566–569.
Date read for all three: 2026-07-30

> **Corrected by the citation check (2026-08-11).** The quoted name read `"Adaptive Sampling"`. The file at those lines reads `name="Use Adaptive Sampling"`. Corrected to the file; the line numbers and the description were already right.

Adaptive sampling matters for this question in a particular way: it reduces samples **per pixel**, so different pixels in one frame can end on different numbers. A single printed `Sample n/m` pair is one number for the whole frame and cannot express that. That last sentence is my reading of the quoted descriptions, so **LABEL: inferred**, and the step is: the description says samples are reduced per pixel; a single frame-wide counter has one value; therefore it cannot record a per-pixel spread.

### What is *not* printed

**LABEL: documented.** The per-pixel truth is available, but as an image pass, not as printed text:

> Sample Count Cycles
>
> Number of samples calculated for each pixel, divided by the maximum number of samples.
> Used to analyze adaptive sampling.

Source: manual, https://docs.blender.org/manual/en/5.0/render/layers/passes.html
Date read: 2026-07-30

The same, from Cycles' own file — **LABEL: observed**:

> ```
>     pass_debug_sample_count: BoolProperty(
>         name="Debug Sample Count",
>         description="Number of samples per pixel taken, divided by the maximum number of samples. To analyze adaptive sampling",
>         default=False,
>         update=update_render_passes,
>     )
> ```

File: `Blender Sources/cycles_5.0/properties.py`, lines 1482–1487.
Date read: 2026-07-30

And Cycles' own file lists it among the render passes — **LABEL: observed**:

> ```
>     if crl.pass_debug_sample_count:
>         yield ("Debug Sample Count", "X", 'VALUE')
> ```

File: `Blender Sources/cycles_5.0/engine.py`, lines 220–221, inside `list_render_passes()`.
Date read: 2026-07-30

That it is not *also* printed as a message rests on the record of looking below, not on this file.

**LABEL: documented.** The metadata Blender can stamp into an image does not include a sample count. The Metadata panel's full Include list is: Date, Time, Render Time, Frame, Frame Range, Memory, Hostname, Camera, Lens, Scene, Marker, Filename, Strip Name, Note. Source: manual, https://docs.blender.org/manual/en/5.0/render/output/properties/metadata.html, read 2026-07-30. Nothing there records samples.

**Where I looked:** I searched the whole 5.0 manual, API documentation, release notes and Cycles' Python files for `sample count`, `Sample Count`, `samples taken`, `actual samples`, `number of samples`, `Sample 1/`, `Path Tracing Sample`, and `samples`. I read in full: the Cycles Sampling page, the Render Layers Passes page, the Metadata page, the Core and Python API release notes, `cycles_5.0/properties.py` around every `samples` hit, and `cycles_5.0/engine.py`. The `Sample n/m` line in the release-notes example is the only printed sample count anywhere in the sources.

### Observed — what Blender 5.0 prints and writes about samples (2026-08-11)

Run in Blender 5.0.1 on **the Creator's Mac (a MacBook Pro with an Apple M1 Pro chip, 16 GB of memory, macOS 26.5.2, Apple's own graphics and no second card)** and on **a rented Linux machine (Ubuntu 22.04.5 LTS, two NVIDIA RTX 4000 Ada Generation cards of 20 GB each, eleven cores, 49 GB of memory)**, each with a settings folder of its own. **LABEL: observed.**

**1. A printed line does show it, and the mark is not the one a reader would expect.** The scene — a cube, a sphere, an area light and a camera; Cycles on the processor; 4096 samples; adaptive sampling off; 480 by 360 — was rendered three ways from a terminal with no window: plain, with a five-second time limit, and with adaptive sampling on at a threshold of 0.1. **All three exited 0 and wrote their picture.**

A frame that finished ends on its full count. The whole of the sample lines from the plain run on the Creator's Mac:

```
00:00.359  render           | Fra: 1 | Mem: 3M | Sample 0/4096
00:00.371  render           | Fra: 1 | Remaining: 00:36.05 | Mem: 26M | Sample 1/4096
00:36.054  render           | Fra: 1 | Mem: 26M | Sample 4096/4096
```

A frame cut short by the time limit shows the count it reached and then **resets to zero**:

```
00:00.351  render           | Fra: 1 | Mem: 3M | Sample 0/4096
00:00.362  render           | Fra: 1 | Remaining: 00:04.98 | Mem: 26M | Sample 1/4096
00:05.281  render           | Fra: 1 | Mem: 26M | Sample 513/4096
00:05.573  render           | Fra: 1 | Mem: 26M | Sample 0/4096
```

A frame cut short by adaptive sampling does the same, climbing and then ending on `Sample 0/4096`. **Nothing else is printed** — no notice, no warning, nothing naming the time limit or the noise threshold. So the only printed mark of a frame that stopped short is that its **last** sample line reads `Sample 0/<maximum>`, where a completed frame's last line reads `Sample <maximum>/<maximum>`, and the count actually reached sits on the line before it. On the rented Linux machine the same three runs behaved identically, reaching 771/4096 under the time limit and 240/4096 under adaptive sampling before ending on `Sample 0/4096`. **The two machines agreed on the shape and differed only in the counts reached, as two different processors must.**

**2. A saved image carries a sample count, and the manual's Metadata panel does not list it.** A frame was rendered to PNG with the stamp settings untouched, and the fields read back by a plain Python script rather than by Blender. All eleven:

```
File = …/scenes/base_5.0.blend
Date = 2026/08/11 21:53:43
Time = 00:00:00:01
Frame = 1
Camera = Camera
Scene = Scene
RenderTime = 00:00.18
cycles.ViewLayer.samples = 16
cycles.ViewLayer.total_time = 00:00.18
cycles.ViewLayer.render_time = 00:00.16
cycles.ViewLayer.synchronization_time = 00:00.01
```

The same eleven came back on both machines. **The sample figure is the maximum asked for, not the number reached**: a run whose sample lines showed 144 of 512 reached carried `cycles.ViewLayer.samples (string) = 512`. An OpenEXR file carries the same fields plus four this version has and 4.5 does not — `Software` (`Blender 5.0.1`), `colorInteropID`, `name` and `type`.

**3. The Sample Count pass is *not* written into a multi-layer OpenEXR in this version.** With `pass_debug_sample_count` set to `True` — confirmed set before each render — adaptive sampling on at 0.1 and a maximum of 512 samples, saved as a multi-layer OpenEXR, the file's channel list, read without Blender, is:

```
ViewLayer.Combined.A, ViewLayer.Combined.B, ViewLayer.Combined.G, ViewLayer.Combined.R
```

Only Combined. Nothing was printed about it and the run exited 0. **Blender 4.5 writes the pass into the same file** — its channel list carries `ViewLayer.Debug Sample Count.X` and four Noisy Image channels as well — so this differs between the versions. The same on both machines. One thing to note about the set-up: in this version the output must be switched to multi-layer with **two** settings, `image_settings.media_type = 'MULTI_LAYER_IMAGE'` as well as `image_settings.file_format = 'OPEN_EXR_MULTILAYER'`; setting only the second writes a PNG, with no error and an exit status of 0. Blender 4.5 has no `media_type` setting at all.

**4. Cycles' own statistics carry no sample count.** With `-- --cycles-print-stats` the two printed blocks — described under question 8 — hold memory and time only, on both machines. (The citation check, 2026-08-11, cut the quoted block from question 8 because it held Blender 4.5's figures; the point made here, that neither block carries a sample count, was repeated and holds.)

**5. The render-statistics handler carries a sample count only while a sampling stage is running.** With no window it was handed forty-five strings, the sampling ones reading `STATS ('Remaining: 00:00.05 | Mem: 3M | Sample 1/16', None)`. With a window it was handed one string, `STATS ('Time: 00:00.12 (Saving: 00:00.01)', None)`, carrying no sample count at all. The same on both machines.

**Not covered.** This does not settle what happens with Sample Subset in use. It does not settle anything about Windows.

**Outside the interface.** Settled, and it differs. With no window the sample count is printed, in three lines for a completed frame and in a run that ends on `Sample 0/<maximum>` for a frame that stopped short. With a window open **no sample line reaches the terminal at all**, and the render-statistics handler is handed one string carrying no sample count. The image metadata is written either way — it was produced by runs with no window. The step marked inferred above, about the Sample Count pass reaching the render result either way, is not borne out for this version: the pass did not reach the saved file at all. Both machines agreed.

**Operating system.** Settled for macOS and for Linux, and they agreed on every point: the same printed pattern, the same eleven metadata fields, the same absence of the pass from the saved file, and the same absence of a sample count from Cycles' statistics. **Windows is untested**, no Windows machine having run it.

---

## Things only a running Blender 5.0 can settle

The nine items that stood here have all been run, on both machines, and their results are written into the answers above. What is left open is below.

1. **The time estimate that appears when the pointer rests on the progress bar (questions 7, 9 and 11).** It needs a hover. Neither machine has any way of moving the pointer without a hand on it — the rented Linux machine has no `xdotool` or equivalent installed — so this was dropped rather than run.

2. **Whether a render at the very top of the size range finishes and writes a file (question 7).** At 65536 by 65536 the render starts, divides into 1024 tiles and climbs steadily with the machine's memory barely touched, but the runs were stopped before the end.

3. **What a render does when the machine's memory runs out, on macOS (question 10).** Answered on Linux only. The Creator's Mac is their working computer and its memory is not to be filled on purpose.

4. **Everything on Windows (questions 7 to 11).** No Windows machine has run it, so every one of these answers covers macOS and Linux and leaves Windows open.

5. **What a render started from Blender loaded as a Python module reports (question 9).** Neither machine has such a build.

6. **Which key is bound to which render command (question 9).** With no window Blender's key configuration holds no items, so it could not be read that way.

7. **What is printed when Sample Subset is in use (question 11).** Not tested.

---

2026-08-11 — Claude Opus 5 (Claude Code) — Reports, Blender 5.0

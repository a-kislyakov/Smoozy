# Reports — cross-sweep questions (Blender 5.2)

**Sources consulted, and when.** Blender's 5.2 manual (`Blender Sources/manual_5.2`, including its `_images` pictures), Blender's 5.2 Python API documentation (`Blender Sources/api_5.2`), Blender's 5.2 release notes (`Blender Sources/release_notes_5.2`), and Cycles' own files as shipped with Blender 5.2 (`Blender Sources/cycles_5.2`). All were taken on **2026-07-30**, and that is the date read for every finding below. No web access was used at any point.

**I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.**

Questions answered here: 7, 8, 9, 10 and 11. I worked alone; no helper agents were started, so there is nobody else's reading to vouch for.

A note that applies to all five answers. Blender's manual and API documentation describe *features and settings*. They almost never describe the text a running render puts on a console. Wherever a question asks what a render *prints* or *does*, expect the honest answer to be that the sources name the mechanism but not its output. I have said so plainly rather than filling the gap.

---

## 7. What does a render report about its own progress, across the whole range of picture sizes a render will accept?

**LABEL: not in the sources** (for the progress reporting varying with picture size), with supporting **documented** facts about the picture-size settings and the one progress display the manual describes, and one **observed** reading of Cycles' own files.

### What the sources do settle

**The picture-size settings, and the values they take.** Documented:

> "resolution_x
>
> Number of horizontal pixels in the rendered image (in [4, 65536], default 1920)"

> "resolution_y
>
> Number of vertical pixels in the rendered image (in [4, 65536], default 1080)"

> "resolution_percentage
>
> Percentage scale for render resolution (in [1, 32767], default 100)"

Source: Python API documentation, https://docs.blender.org/api/5.2/bpy.types.RenderSettings.html. Date read 2026-07-30.

So each of the two size settings takes a value from 4 to 65536, and a separate percentage scale takes 1 to 32767. Those are the ranges the settings accept. That a render accepts every value in them is a step past the quoted words, so it is not claimed here. The manual describes the same two controls in words:

> "The number of pixels horizontally and vertically in the image."

> "Slider to reduce or increase the size of the rendered image relative to the Resolution values."

Source: manual, https://docs.blender.org/manual/en/5.2/render/output/properties/format.html. Date read 2026-07-30.

**What progress reporting is described.** The only progress reporting the manual describes for a render is the status bar's Running Task item:

> "Running Task
>
> Shows the progress of the currently running task (such as rendering or baking).
> Hovering the mouse pointer over the progress bar will display a time estimate.
> The task can be aborted by clicking the cancel button ()."

Source: manual, https://docs.blender.org/manual/en/5.2/interface/window_system/status_bar.html. Date read 2026-07-30.

The picture on that same manual page — https://docs.blender.org/manual/en/5.2/interface/window_system/status_bar.html — is `_images/interface_window-system_status-bar_ui-center.png`. It shows a single horizontal bar labelled "Sequence Render", filled to a point marked **94%**, with a cross-shaped cancel button at its right end. That much is **documented**: it is what is visible in the picture. That every render therefore shows its progress in the interface as one bar carrying a whole-number percentage is **inferred** — the step a reader has to supply is that this one picture of one job stands for every job. The picture shows nothing about picture size.

**What the render-engine type carries for progress.** Documented:

> "update_progress(progress)
>
> Update progress percentage of render"
>
> "progress (float) – Percentage of render that's done (in [0, 1])"

Source: Python API documentation, https://docs.blender.org/api/5.2/bpy.types.RenderEngine.html. Date read 2026-07-30.

Those words describe one method on the `RenderEngine` type, and they say the progress it carries is a fraction between 0 and 1. That this is what a render reports, or what a render started from Python reports, is not in them. Nothing in the definition refers to pixels, resolution, or image area.

**The one place picture size appears alongside tiling.** Observed, in Cycles' own files. The add-on's interface code carries a size calculation, with a comment saying it matches the renderer's own:

```
        if cscene.use_guiding:
            # Calculation matches TileManager::compute_render_tile_size and
            # Session::get_effective_tile_size
            if cscene.tile_size < 128:
                tile_size = cscene.tile_size
            else:
                tile_size = (cscene.tile_size + 128 - 1) & ~(128 - 1)
                tile_size = min(tile_size, 8192)
            tile_area = tile_size ** 2

            render_scale = scene.render.resolution_percentage / 100.0
            render_size_x = int(scene.render.resolution_x * render_scale)
            render_size_y = int(scene.render.resolution_y * render_scale)
            render_area = render_size_x * render_size_y

            if render_area > tile_area and render_size_x <= 8192 and render_size_y <= 8192:
                layout.label(text="May work poorly with render tiling", icon='INFO')
```

File: `Blender Sources/cycles_5.2/ui.py`, lines 389–405. Date read 2026-07-30.

And the tile setting itself:

```
    tile_size: IntProperty(
        name="Tile Size",
        default=2048,
        description="Render high resolution images in tiles of this size, to reduce memory usage. Tiles are cached to disk while rendering to save memory",
        min=8,
        max=8192,
    )
```

File: `Blender Sources/cycles_5.2/properties.py`, lines 1084–1090. Date read 2026-07-30.

The manual says the same thing in prose:

> "Render high resolution images in tiles of this size, to reduce memory usage
> of render passes. Tiles are cached to disk while rendering to save memory."

Source: manual, https://docs.blender.org/manual/en/5.2/render/cycles/render_settings/performance.html. Date read 2026-07-30.

What the files establish is this and no more: the tile setting exists under the name Tile Size, with a default of 2048 and a range of 8 to 8192; the add-on's interface shows the words "May work poorly with render tiling" when guiding is switched on and the picture area exceeds the square of the tile size; and a comment in that code says the calculation matches the renderer's own. What a render actually *does* with the tile setting at any picture size is a step past reading the file — that step is **inferred**, not observed, and it is not taken here. Nor is the further step that the *progress report* changes shape at any picture size. The manual's prose, quoted above, is what says tiles are used for high-resolution images at all.

### What is not in the sources

No source in this set states what a render reports about its progress at any particular picture size, and no source states that the report differs between a 4 × 4 picture and a 65536 × 65536 one. There is no sentence anywhere tying progress reporting to resolution.

**Where I looked.** I searched the whole of `manual_5.2`, `api_5.2` and `release_notes_5.2` for: `progress`, `progress bar`, `percentage`, `Running Task`, `render status`, `status text`, `render statistic`, `status bar`, `resolution`, `image size`, `megapixel`, `high resolution`, `tile`, `Tiles`, `8192`, `16384`, `32767`, `65536`. I read in full: the status bar page, the Format page, the Cycles Performance page, the Cycles Sampling page, the Working Limits page (`https://docs.blender.org/manual/en/5.2/advanced/limits.html` — it lists limits for space, time and text fields, and gives no picture-size limit), the render output introduction and animation pages, and the `RenderEngine`, `RenderSettings` and `WindowManager` API pages. I looked at the status bar picture named above. In Cycles' files I searched every `.py`, `.h`, `.cpp`, `.cu` and `.metal` file for `progress`, `remaining`, `elapsed`; the only matches were unrelated (a shadow-intersection helper, a manifold-path constant, comments about texture tiles) — Cycles' session and progress code is not among the files supplied.

### Observed — what a running Blender 5.2 does (2026-08-11)

Every test below was run in Blender 5.2.0 on both machines this project can reach: **the Creator's Mac — a MacBook Pro with an Apple M1 Pro chip, 16 GB of memory, macOS 26.5.2, Apple's own graphics and no second card** — and **a rented Linux machine — Ubuntu 22.04.5 LTS on kernel 6.8, two NVIDIA RTX 4000 Ada Generation cards of 20 GB each on driver 580.105.08, an AMD Ryzen Threadripper PRO 3945WX with eleven cores, 49 GB of memory**. Each run had a settings folder of its own, so nothing touched the Creator's own Blender settings. **LABEL: observed** throughout.

**The scene.** Built by Blender 5.2 itself: a cube, a sphere, an area light and a camera; Cycles on the processor; 16 samples; adaptive sampling off; time limit 0; 160 by 120 pixels; output PNG.

**1. The unit of progress does change with picture size: it gains a tile count.** The same frame was rendered from a terminal with no window six times, changing only the two resolution numbers, at 4 by 4, 160 by 120, 1920 by 1080, 2048 by 2048, 2049 by 2049 and 4096 by 4096. All six exited 0 and wrote a picture of the size asked for.

At 4 by 4, 160 by 120, 1920 by 1080 and 2048 by 2048 the capture holds **three** lines carrying a sample count and mentions no tile. At 2049 by 2049 and at 4096 by 4096 it holds **thirteen**, and every one mentions a tile. The whole thirteen from the 2049 by 2049 run on the Creator's Mac:

```
00:00.502  render           | Fra: 1 | Mem: 1M | Rendered 0/4 Tiles, Sample 0/16
00:00.702  render           | Fra: 1 | Remaining: 00:02.99 | Mem: 241M | Rendered 0/4 Tiles, Sample 1/16
00:04.050  render           | Fra: 1 | Remaining: 00:00.00 | Mem: 241M | Rendered 0/4 Tiles, Sample 16/16
00:04.050  render           | Fra: 1 | Remaining: 00:00.00 | Mem: 241M | Rendered 1/4 Tiles, Sample 16/16
00:04.050  render           | Fra: 1 | Remaining: 00:00.00 | Mem: 1M | Rendered 1/4 Tiles, Sample 1/16
00:04.053  render           | Fra: 1 | Remaining: 00:00.00 | Mem: 1M | Rendered 1/4 Tiles, Sample 16/16
00:04.053  render           | Fra: 1 | Remaining: 00:00.00 | Mem: 1M | Rendered 2/4 Tiles, Sample 16/16
00:04.053  render           | Fra: 1 | Remaining: 00:00.00 | Mem: 1M | Rendered 2/4 Tiles, Sample 1/16
00:04.055  render           | Fra: 1 | Remaining: 00:00.00 | Mem: 1M | Rendered 2/4 Tiles, Sample 16/16
00:04.055  render           | Fra: 1 | Remaining: 00:00.00 | Mem: 1M | Rendered 3/4 Tiles, Sample 16/16
00:04.055  render           | Fra: 1 | Remaining: 00:00.00 | Mem: 1M | Rendered 3/4 Tiles, Sample 1/16
00:04.055  render           | Fra: 1 | Mem: 1M | Rendered 3/4 Tiles, Sample 16/16
00:04.055  render           | Fra: 1 | Mem: 1M | Rendered 3/4 Tiles, Sample 0/16
```

**2048 by 2048 is exactly the square of the default tile size and printed no tile; 2049 by 2049 printed four.** So the threshold the add-on's own interface code works out is the one that shows in the printed stream — and it is strictly greater than, not greater than or equal to. The rented Linux machine gave the same split at the same six sizes and the same tile counts. **The two machines agreed.**

**2. The settings clamp silently, and never refuse.** Set from Python and read straight back: 0, 3 and 4 all become **4** for the two size settings; 65536, 65537 and 70000 all become **65536**; for the percentage 0 becomes **1** and 32768 and 70000 become **32767**. Nothing raised an error and nothing was printed. Identical on both machines.

**3. The two ends of the accepted range.** A render at **4 by 4** finishes and writes a 4 by 4 picture, on both machines. **At 65536 by 65536 a render starts, tiles, and then fails to save.** With one sample per pixel, on the rented Linux machine, it divided into **1024 tiles** and printed a line for each — 1026 tile lines — reaching `Rendered 1023/1024 Tiles, Sample 0/1`, holding about 241 MB of Blender's own memory and about 3.5 GB of the machine's. It then stopped at the point of putting the tiles together, printing on standard output

```
04:36.477  reports          | ERROR Error reading tiles from file
```

and on standard error

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
00:00.358  reports          | ERROR Image too small
```

**5. The status bar during a render, with a window open.** Photographed with Blender's own screenshot command every few seconds. There is **exactly one bar**, never two. Beside it, in order: a small camera icon, the words **`Rendering...`**, the bar with a **whole-number percentage** written inside it, and an **X** at the right end. No decimal point appeared in any photograph, so the one picture in the manual does stand for a render in this respect. Mid-render readings: `Rendering...` beside `1%` early and `48%` later on the Creator's Mac, `Rendering...` beside `26%` on the rented Linux machine. **No frame is named**, for a still or an animation, and during a three-frame animation the percentage restarted at each frame rather than climbing across the range. **Picture size made no difference on screen** — the same one bar and whole-number percentage at 4 by 4 and at 4096 by 4096, even though the printed stream counts tiles at the larger size. Both machines agreed.

**6. A sequencer render drives the same bar, and it is labelled `Rendering sequence...`** — not `Sequence Render`, which is the label in the manual's one picture. Caught mid-render at `0%` and `16%`, on both machines.

**7. EEVEE and Workbench do not behave as Cycles does.** EEVEE prints `Fra: 1 | Rendering <n> / 64 samples` at 1, 25, 50 and 64 of 64, with no memory figure and no time, and it used its own count of 64 where the scene asked for 16. **Workbench prints no progress line at all.** Neither mentions a tile at any size. Both machines.

**8. The settings, read from the running program.** `use_auto_tile` is named **Auto Tile** and described **"Deprecated, tiling is always enabled"**, default True. `tile_size` is described "Render high resolution images in tiles of this size, to reduce memory usage. Tiles are cached to disk while rendering to save memory", default 2048, range `[8, 8192]`. `resolution_x` and `resolution_y` default 1920 and 1080, range `[4, 65536]`; `resolution_percentage` is named **Resolution Scale**, default 100, range `[1, 32767]`. Searching Blender 5.2's own installed Cycles add-on for the words "May work poorly with render tiling" and for the comment naming `compute_render_tile_size` found **both** — the code quoted above is really there — where Blender 4.5's add-on carries **neither**. Identical on both machines.

### Observed — one further run, at 30000 by 30000 with the tiling switch off (2026-08-07)

**LABEL: observed.** One run, on one machine, on one day, in Blender 5.2 alone. It was not made for this question: it belongs to another topic's hand-test list, which was asking what a graphics card running out of memory does. What it printed answers part of this question at a picture size no run made for this question ever reached, so it is written in here.

**The machine.** A rented Linux machine — the fourth rented Linux machine in the equipment section of `Preparation/Blender Research/Method.md`: Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER of 16,376 MiB, an AMD EPYC 7B12 with twenty-four of its cores allotted, 39 GB of memory, no display server. Its graphics driver had been mended earlier that day and both halves read 580.173.02. **It is neither of the two machines behind the runs of 2026-08-11 above**, and it is destroyed, so nothing further can be run on it. Its Blender read itself out as `Blender 5.2.0 LTS (hash fbe6228777e7 built 2026-07-14 01:32:04)` — **5.2.0, not the 5.2.1 the Creator's Mac carries now**.

**What was done.** That topic's shared scene — the factory cube, camera and light, Cycles, 16 samples, denoising off, 160 by 120, PNG — rendered as one frame from a terminal with no window, with a settings folder of its own, with the card selected by a small script that sets Cycles' compute device and turns the scene to GPU, and with three settings changed at the launch. The command as the record holds it, where `$B` is the version's own program, `$S` the shared scene, `$W` the work folder and `$O` the output folder:

```
$B -b $S --python $W/setcard.py --python-expr "
import bpy
sc = bpy.context.scene
sc.cycles.use_auto_tile = False
sc.render.resolution_x = 30000
sc.render.resolution_y = 30000
" --log "*" --log-level info -o $O/oom/big_ -F PNG -f 1
```

with `nvidia-smi --query-gpu=memory.used` sampled every second beside it.

**What came back.** One line is the whole of the printed output the record keeps:

```
Fra: 1 | Mem: 805M | Rendered 0/225 Tiles, Sample 1/1
```

The card's memory peaked at 3,448 MiB of 16,376. **The run then died, and it was the machine that killed it, not the card.** No exit status and no closing message for this run is in the record.

**What it settles.** At 30000 by 30000, with `use_auto_tile` set to `False`, Blender 5.2 still printed a tile count, in the same shape as every tiled line above — `Rendered <n>/<total> Tiles, Sample <s>/<max>` — and the total was 225. **So the tile count in the printed progress line does not depend on that switch being left on, in this version.** That sits beside what the switch says of itself under item 8 — "Deprecated, tiling is always enabled" — without being the same claim: this is what a render printed, not what a setting reads.

**What it does not cover.**

- **4.5, 5.0 and 5.1 were never run with the switch off for what they print.** Nothing above may be read across to them, and nothing above is evidence of a difference between versions.
- **One run, one machine, one day, and that machine is gone.** It was not repeated and it was never tried on the Creator's Mac.
- **The tile size was not read back on this run**, so nothing here establishes what produced the figure 225.
- **The run did not finish**, so nothing here says what a whole tiled stream at that size carries, or whether a picture that size can be written.
- **The kept line reads `Sample 1/1` where the shared scene sets 16 samples and nothing in the run changes it.** The record does not explain that and the machine cannot be asked. It is left as it stands, and nothing above rests on the sample figure.
- **The line is kept without the timestamp and log prefix** the 2026-08-11 lines carry, so which stream it landed on is not established by it.
- Windows, as everywhere else in this answer.

**Not covered.** This does not settle what the time estimate says when the pointer rests on the progress bar, which needs a hover and could not be reached. It does not settle whether a 65536 by 65536 render would save into a format OpenImageIO's size limit allows, or with that limit raised. It does not settle what 4.5, 5.0 or 5.1 print with the tiling switch turned off. It does not settle anything about Windows.

**Outside the interface.** Settled, and it differs. With no window there is no status bar and no percentage — the step marked inferred above is borne out, since there is no bar to photograph. What stands in for it is the printed stream quoted above, carrying a frame number, a `Remaining:` time estimate, a memory figure, a sample count against its maximum, and a tile count once the picture crosses one tile's area. `update_progress` itself works with no window: a tiny render engine registered from a script called it, and `update_stats` and `update_memory_stats` beside it, and all three returned without complaint with a window and without, on both machines, producing no output of their own. The size range is the same either way. Both machines agreed.

**Operating system.** Settled for macOS and for Linux, and they agreed on every point above: the same three-or-thirteen sample lines at the same sizes, the same tile counts, the same clamping, the same written picture sizes, the same `ERROR Image too small`, the same `Rendering...` label and whole-number percentage, and the same settings. **Windows is untested** — no Windows machine has run it. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

---

## 8. What does a render print over the whole of a run, from its first line to its last?

**LABEL: not in the sources** (for the actual run, first line to last), with several **documented** and one **observed** finding about the machinery that produces the printing.

No source in this set contains a transcript, an example, or a description of the sequence of lines a render prints from start to finish. What the sources do establish is the following.

### There is status printing, and it can be switched off

> "-q, --quiet
>
> Suppress status printing (warnings & errors are still printed)."

Source: manual, https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html. Date read 2026-07-30. **Documented.**

That warnings and errors survive `-q` is in the quoted words. That status printing therefore happens by default is **inferred** — the step a reader supplies is that nothing would be offered for suppression unless it were happening.

### Where the printing goes

> "The Console Window, also called a Terminal, is an operating system text window that displays
> messages about Blender's operations, status, and internal errors."

> "When Blender is manually started from a terminal,
> Blender output is shown in the Console Window it is started from."

Source: manual, https://docs.blender.org/manual/en/5.2/advanced/command_line/index.html. Date read 2026-07-30. **Documented.**

### What the render statistics line contains

> "bpy.app.handlers.render_stats
>
> on printing render statistics. Accepts one argument: the render progress as a string containing current frame, current sample, render time and saving time."

Source: Python API documentation, https://docs.blender.org/api/5.2/bpy.app.handlers.html. Date read 2026-07-30. **Documented.**

This is the single most direct statement in the whole source set about what a render prints. It names four things carried in the printed statistics string: current frame, current sample, render time, saving time. It does not give the string's layout, its punctuation, its order, or how often it is emitted.

### Extra printing that can be turned on

> "--cycles-print-stats
>
> Log statistics about render memory and time usage."

Source: manual, https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html. Date read 2026-07-30. **Documented.**

Cycles' own files show the same switch and say where its output goes:

```
    parser.add_argument("--cycles-print-stats",
                        help="Print rendering statistics to stderr",
                        action='store_true')
```

File: `Blender Sources/cycles_5.2/engine.py`, lines 13–15. Date read 2026-07-30. **Observed.** Note the difference in wording between the two: the manual says "Log statistics about render memory and time usage", the add-on's own help string says "Print rendering statistics to stderr". The stream named — standard error — is stated only in the file.

Logging can also be widened or narrowed:

> "--log <match>
>
> Enable logging categories, taking a single comma separated argument."
>
> "--log \"render,cycles\": both render and cycles messages."

and the levels:

> "fatal: Fatal errors only
> error: Errors only
> warning: Warnings
> info: Information about devices, files, configuration, operations
> debug: Verbose messages for developers
> trace: Very verbose code execution tracing"

with `--log-show-memory` ("Show memory usage for each log message"), `--log-file <filepath>` ("Set a file to output the log to"), `--log-list-categories`, `--debug-cycles` ("Enable debug messages from Cycles") and `--debug-ffmpeg` ("Enable debug messages from FFmpeg video input and output") alongside. Source: manual, https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html. Date read 2026-07-30. **Documented.**

For video output specifically:

> "When you view the System Console,
> you can see some of the output of the encoding process.
> You will see even more output if you execute Blender as blender -d."

Source: manual, https://docs.blender.org/manual/en/5.2/render/output/properties/output.html. Date read 2026-07-30. **Documented.**

### One glimpse of a real console, but not of a render

The manual's page on launching from the command line on Windows carries the picture `_images/advanced_command-line_launch_windows_console.png`. In it, a Command Prompt shows a run of Blender whose printed lines are, in order: a line beginning "Read new prefs:" with a path to `userpref.blend`, a line "found bundled python:" with a path, a line "read blend:" with a path to a `.blend` file, a blank line, then **"Blender quit"**, then the shell prompt again. Source: manual, https://docs.blender.org/manual/en/5.2/advanced/command_line/launch/windows.html. Date read 2026-07-30. **Documented** — but for that run only. It is a session that opened a file and quit; it is not a render, and the paths shown are from an old installation. The one thing it shows about a run's *last* line is that this one session ended by printing "Blender quit". Extending that to command-line sessions in general, or to a render run, would be inference, so I have made neither claim.

The companion picture on the Linux launch page, https://docs.blender.org/manual/en/5.2/advanced/command_line/launch/linux.html (read 2026-07-30), `_images/advanced_command-line_launch_linux_console.png`, shows only audio-subsystem warnings at startup ("jack server is not running or cannot be started" and similar) and no render output at all.

> **Filled in by the citation check (2026-08-11).** This sentence carried "[PENDING: the address of that page — it is not recorded here]". The picture sits on the Linux launch page, whose address is now given above.

**Where I looked.** I searched the whole of `manual_5.2` and `api_5.2` for: `stdout`, `stderr`, `console`, `terminal`, `System Console`, `Saved:`, `Fra:`, `Time:`, `Mem:`, `Remaining`, `Elapsed`, `Peak`, `Append Frame`, `print`, `progress`, `render statistic`. Of those, `Saved:`, `Fra:`, `Mem:`, `Remaining`, `Elapsed`, `Append Frame`, `stdout` and `stderr` produce **no hits at all** anywhere in the manual or API documentation. I read the command line index, arguments, and all three launch pages in full, plus the two console pictures. I searched all of `release_notes_5.2` for `print`, `log`, `stats`, `progress`. In Cycles' files I read `engine.py` in full and searched every supplied source file for progress and timing strings; nothing that prints a render's progress is present among them.

### Observed — the whole of a Blender 5.2 render run (2026-08-11)

Run in Blender 5.2.0 on **the Creator's Mac (a MacBook Pro with an Apple M1 Pro chip, 16 GB of memory, macOS 26.5.2, Apple's own graphics and no second card)** and on **a rented Linux machine (Ubuntu 22.04.5 LTS on kernel 6.8, two NVIDIA RTX 4000 Ada Generation cards of 20 GB each, eleven cores, 49 GB of memory)**, each with a settings folder of its own. The scene was built by Blender 5.2 itself: a cube, a sphere, an area light and a camera; Cycles on the processor; 16 samples; adaptive sampling off; 160 by 120; output PNG. **LABEL: observed.**

**1. The actual text and order of every line, first to last.** `blender -b <file>.blend -o <out> -f 1 > out.txt 2> err.txt` exited 0 and wrote its picture. Standard output held fifty-five lines. The first seven, all three sample lines and the last four, word for word, from the Creator's Mac:

```
00:00.633  blend            | Read blend: "…/scenes/base_5.2.blend"
00:00.639  render           | Rendering single frame
00:00.639  render           | Rendering frame 1
00:00.639  render           | Start rendering: Scene, ViewLayer
00:00.639  render           | Engine: Cycles
00:00.651  render           | Fra: 1 | Mem: 0M | Synchronizing object | Cube
Blender 5.2.0 LTS (hash fbe6228777e7 built 2026-07-14 01:31:22)
…
00:00.658  render           | Fra: 1 | Mem: 1M | Sample 0/16
00:00.662  render           | Fra: 1 | Remaining: 00:00.06 | Mem: 2M | Sample 1/16
00:00.844  render           | Fra: 1 | Mem: 2M | Sample 16/16
00:00.844  render           | Fra: 1 | Mem: 2M | Finished
00:00.927  render           | Saved: '…/out/e1_5.2_0001.png'
00:00.927  render           | Time: 00:00.28 (Saving: 00:00.08)

Blender quit
```

**The first line is `blend | Read blend:` and the last is `Blender quit`, with one blank line before it.** The version banner is not first: it arrives seven lines in, and it is the one line with no timestamp and no category. Every other line has an elapsed time, a category name, a vertical bar, then the message. **Exactly three lines carry a sample count for a sixteen-sample frame** — never one per sample. One stage line is this version's own and appears in none of the other three: `Updating Scene Attribute`.

**The exit status of a run that succeeded is 0**, on both machines.

**One difference between the machines.** On the Creator's Mac standard error was **empty**. On the rented Linux machine, which has a display server running, Blender 5.2 put four lines there even though the render succeeded:

```
Authorization required, but no authorization protocol specified
xcb_connection_has_error() returned true
Authorization required, but no authorization protocol specified
xcb_connection_has_error() returned true
```

Those are not Blender's own words. Blender 4.5, 5.0 and 5.1 left standard error empty on the same machine.

**2. `-q` removes three lines and not the render.** Comparing the two captures line by line, it removes the `blend | Read blend:` line, the blank line before the end, and `Blender quit`. Every render line survives, including all three sample lines and the `Saved:` and `Time:` lines, and so does the version banner. The picture was still written and the run exited 0. So the step marked inferred above is only partly right: status printing happens by default, and what `-q` takes away is those three lines.

**3. Which stream Cycles' statistics go to: standard output, not standard error.** With `-- --cycles-print-stats` the statistics arrived on **standard output** on both machines — the add-on's own help string names standard error, and what the machines did was otherwise. They come in two blocks: `Update statistics:` during the Updating stages and `Render statistics:` **after the `Saved:` and `Time:` lines**, near the end of the run. The second block, in this version, opens

> **Corrected by the citation check (2026-08-11).** This paragraph placed the second block "after the last sample line and before `Saved:`". Repeating the run on both machines, twice each, put it after `Saved:` and after `Time:` in this version. Blender 4.5 does place it before `Saved:`, which is what its own document says. The placing is also sent to the hand tests to be settled by a second pair of hands. The figures quoted below were repeated and came back the same.

```
Render statistics:
Mesh statistics:
  Geometry:
    Total memory: 16.36K (16,750)
        Sphere                           16.02K (16,405)
        Cube                             289B (289)
        Area                             56B (56)
Image statistics:
  Total image memory: 0B (0)
Kernel statistics:
  Total render time               : Total 100.00% (0.14s), Self 29.17% (0.04s)
```

and runs on through the kernel, shader and object figures. **No sample count appears anywhere in either block.** Two shape differences from Blender 4.5: 4.5 writes the image section as `Image statistics:` / `  Textures:` / `    Total memory: 0 (0)` where this version writes `Image statistics:` / `  Total image memory: 0B (0)`, and 4.5 writes plain byte counts — `413 (413)` — where this version writes them with a `B`. Without the bare `--` the switch is taken as a file name and the run **exits 1**, printing `00:00.561  reports          | ERROR Cannot read file "…/--cycles-print-stats": No such file or directory`.

**Observed — the block this version prints, and where it lands, captured by a second pair of hands (2026-08-12).** Run on **the Creator's Mac (a MacBook Pro with an Apple M1 Pro chip, 16 GB of memory, macOS 26.5.2, Apple's own graphics and no second card)** and on **a rented Linux machine (Ubuntu 22.04.5 LTS on kernel 6.8, two NVIDIA RTX 4000 Ada Generation cards of 20 GB each on driver 580.105.08, an AMD Ryzen Threadripper PRO 3945WX with eleven cores, 49 GB of memory)**, each version with a settings folder of its own and its own copy of the scene: a cube, a sphere, an area light and a camera; Cycles on the processor; 16 samples; adaptive sampling off; 160 by 120; output PNG. The command was `blender -b <file>.blend -o <out> -f 1 -- --cycles-print-stats`, standard output and standard error kept apart. Every run exited 0. **LABEL: observed.** The head of the block on the Creator's Mac, word for word:

```
Render statistics:
Mesh statistics:
  Geometry:
    Total memory: 16.36K (16,750)
        Sphere                           16.02K (16,405)
        Cube                             289B (289)
        Area                             56B (56)
Image statistics:
  Total image memory: 0B (0)
Kernel statistics:
  Total render time               : Total 100.00% (0.14s), Self 13.89% (0.02s)
```

**The rented Linux machine printed the same nine lines above `Kernel statistics:`, figure for figure**; its own kernel line read `  Total render time               : Total 100.00% (0.31s), Self 16.88% (0.05s)`. **The two machines agreed on the byte counts and differed on the times.** The block runs on through the kernel, shader and object figures, all of them times and percentages that changed from run to run, so only the byte counts should be read as this version's own. **No sample count appears anywhere in it.** On standard error the Mac printed nothing; the Linux machine printed the same four X-authorisation lines quoted earlier in this answer.

**The two shape differences from Blender 4.5, checked again.** Both hold. Run the same way on the same day, 4.5 wrote its image section as `Image statistics:`, `  Textures:`, `    Total memory: 0 (0)` — three lines to this version's two — and wrote plain byte counts, `Cube 413 (413)`, where this version writes `Cube 289B (289)` with a `B`. The byte counts themselves are different in every version but one: 4.5 gave `Total memory: 24.03K (24,607)`, 5.0 and 5.1 both gave `24.04K (24,615)`, and this version gave `16.36K (16,750)`.

**Where the block lands, counted by line number.** The lines of a second run were numbered. On the Creator's Mac standard output held 130 lines: `Update statistics:` at 15, `Sample 16/16` at 82, `Finished` at 83, `Saved:` at 84, `Time:` at 85, `Render statistics:` at 86. On the rented Linux machine it also held 130 lines with exactly the same numbers. **The two machines agreed, and both bear out the correction above: in this version the block comes after `Saved:` and after `Time:`.** Blender 4.5 run the same way put `Render statistics:` at line 78 and `Saved:` at 122 on the Mac — the other way round.

**4. What is printed before the first frame and after the last, for an animation.** A three-frame run announces the range once and each frame in turn:

```
00:00.396  render           | Rendering animation (frames 1..3)
00:00.396  render           | Rendering frame 1
…
00:00.567  render           | Time: 00:00.16 (Saving: 00:00.06)
00:00.567  render           | Rendering frame 2
00:00.567  render           | Start rendering: Scene, ViewLayer
00:00.567  render           | Engine: Cycles
…
00:00.747  render           | Time: 00:00.08 (Saving: 00:00.00)

Blender quit
```

Nothing follows the last frame's `Time:` line but the blank line and `Blender quit`.

**5. What a run prints when it is not a Cycles run.** EEVEE — `BLENDER_EEVEE` in this version — prints four progress lines of the shape `00:00.790  render           | Fra: 1 | Rendering 1 / 64 samples`, at 1, 25, 50 and 64 of 64, **with no memory figure and no time figure**, where Blender 4.5's EEVEE lines carry both. **Workbench prints no progress line at all**, going from `Engine: Workbench` straight to `Saved:`. The same on both machines.

**6. How often the statistics line repeats, and what it actually carries.** A script attached a handler to `bpy.app.handlers.render_stats` and wrote every call down. With no window it fired **forty-five times for one frame** — once per stage, not once per render — and was handed two arguments, a string and `None`. The first, a middle and the last:

```
STATS ('Mem: 0M | Synchronizing object | Cube', None)
STATS ('Remaining: 00:00.04 | Mem: 2M | Sample 1/16', None)
STATS ('Time: 00:00.22 (Saving: 00:00.06)', None)
```

**The string carries no frame number.** The API page for this version describes it as "the render progress as a string containing current frame, current sample, render time and saving time"; what came back carries a memory figure, an optional `Remaining:`, and a stage that names the current sample when a sampling stage is running — and no frame anywhere in the forty-five strings, on either machine. The render time and saving time arrive together in the last string only.

**Not covered.** This does not settle what a run prints when it fails rather than succeeds, which is a different question on the sheet. It does not settle what is printed when the output is a video file. It does not settle anything about Windows.

**Outside the interface.** Settled by the tests above, and every line quoted there was captured from a terminal with no window. **With a window open, no progress line reaches the terminal at all** — a render started from a script with the window up printed only `Saved:`, on both machines. So the implication the manual leaves open runs the other way: a windowed session started from a terminal does print, but during a render it prints only the file it saved. The `render_stats` handler, unlike in the three older versions, **does** fire in both cases in this version — forty-five strings with no window and forty-four with one — which is recorded under question 9. **A later run on 2026-08-12 counted the windowed firings again and got a different number**: forty-five on the rented Linux machine, three runs out of three, and forty-six, forty-five and forty-five in three runs on the Creator's Mac. Both counts stand as they were taken; the later run is written up under question 9.

**Operating system.** Settled for macOS and for Linux by the tests above, and the printed content agreed line for line: the same fifty-five lines in the same order, the same first and last line, the same three sample lines, the same three lines removed by `-q`, the same statistics on standard output, the same animation announcements, the same EEVEE and Workbench shapes, and the same handler strings. **Two differences between the machines**: Blender 5.2 on the Linux machine, which has a display server running, puts four X-authorisation lines on standard error even when the render succeeds, where the Mac leaves it empty; and the list of logging categories is 149 entries on the Mac and 182 on the Linux machine (corrected from 183 by the citation check, 2026-08-11 — the Linux figure had counted the version banner as a category). **Checked again on 2026-08-12 for Cycles' statistics**: the block's byte counts and its place in the run came back the same on both machines — the same six line numbers out of 130 lines — and only the times inside the block differed. **Windows is untested** — no Windows machine has run it — so what follows stands unchecked. On Windows the console exists but is hidden:

> "When Blender is started on a Microsoft Windows operating system, the Console Window
> (called the Command Prompt) is first created as a separate window on the desktop.
> The main Blender window will also appear and the Console Window will then be toggled off.
> To display the console again, go to Window ‣ Toggle System Console."

Source: manual, https://docs.blender.org/manual/en/5.2/advanced/command_line/launch/windows.html. Date read 2026-07-30. The Linux and macOS pages describe no such hiding. Beyond that, the sources state no operating-system differences in what is printed.

---

## 9. What does a render report about its progress under each of the ways a render can be started?

**LABEL: not in the sources** (for a per-method account of progress reporting), with **documented** findings on the ways a render can be started and on the one progress display the manual describes.

### The ways a render can be started, as the sources name them

**From the interface, via the Render menu:**

> "Render Image F12
>
> Render the active scene at the current frame."
>
> "Render Animation Ctrl-F12
>
> Render the animation of the active scene."
>
> "Render Sequencer Image Alt-F12"
>
> "Render Sequencer Animation Ctrl-Alt-F12"

Source: manual, https://docs.blender.org/manual/en/5.2/interface/window_system/topbar.html. Date read 2026-07-30.

**From the 3D viewport, as a viewport render:** "3D Viewport ‣ View ‣ Render Viewport Preview", "3D Viewport ‣ View ‣ Render Playblast", and "3D Viewport ‣ View ‣ Render Playblast on Keyframes", with

> "As with a normal render, you can abort it with Esc."

and the restriction

> "Viewport rendering only works for the Workbench and EEVEE render engines.
> It's not supported for Cycles."

Source: manual, https://docs.blender.org/manual/en/5.2/editors/3dview/viewport_render.html. Date read 2026-07-30.

**From the command line:**

> "-b, --background
>
> Run in background (often used for UI-less rendering)."
>
> "-a, --render-anim
>
> Render frames from start to end (inclusive)."
>
> "-f, --render-frame <frame>
>
> Render frame <frame> and save it."

Source: manual, https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html. Date read 2026-07-30. The worked examples `blender -b file.blend -f 10` and `blender -b file.blend -a` are on https://docs.blender.org/manual/en/5.2/advanced/command_line/render.html.

**From Python:**

> "bpy.ops.render.render(\*, animation=False, write_still=False, use_viewport=False, use_sequencer_scene=False, layer='', scene='', frame_start=0, frame_end=0)
>
> Undocumented, consider contributing."

Source: Python API documentation, https://docs.blender.org/api/5.2/bpy.ops.render.html. Date read 2026-07-30. The operator that starts a render is, in so many words, undocumented; only its arguments are named, with their defaults, and none of them is described.

### The one progress report described, and which way it belongs to

The status bar's Running Task, quoted in full under question 7, is a feature of Blender's window. Its picture shows a bar reading "Sequence Render" filled to 94% with a cancel button. That is the interface way, and the sources describe no equivalent for any other way.

The `RenderEngine` type carries three separate channels — a fraction, a text, and memory figures:

> "update_progress(progress)
>
> Update progress percentage of render"

and a separate text channel:

> "update_stats(stats, info)
>
> Update and signal to redraw render status text"

and a memory channel:

> "update_memory_stats(\*, memory_used=0.0, memory_peak=0.0)
>
> Update memory usage statistics"

Source: Python API documentation, https://docs.blender.org/api/5.2/bpy.types.RenderEngine.html. Date read 2026-07-30. These three are described as members of the `RenderEngine` type. That they are what a render started from Python reports is a step the quoted words do not carry, and it is not taken here. Note that `update_stats` is described as updating *render status text* and signalling a redraw; that a redraw is a window act is **inferred** rather than stated. Whether that text goes anywhere at all when there is no window is not stated.

The `render_stats` handler quoted under question 8 — "on printing render statistics … current frame, current sample, render time and saving time" — is defined with no reference to how the render was started.

Blender knows which way it was started:

> "bpy.app.background
>
> Boolean, True when blender is running without a user interface (started with -b)"

Source: Python API documentation, https://docs.blender.org/api/5.2/bpy.app.html. Date read 2026-07-30.

### A release note that touches this directly

Blender 5.2's bugfix list records, under Core, a fixed issue whose title is:

> " \* Blender 5.1.1: Is Cycles render sample progress still accessible from Python without using -f? [[c065cc46602](https://projects.blender.org/blender/blender/commit/c065cc46602ad3d2dbe13814e1ef27cc8f8b5142)]"

Source: release notes, https://developer.blender.org/docs/release_notes/5.2/bugfixes/. Date read 2026-07-30. **Documented**, and it is a documented statement about a change — the entry is in a list introduced as "There are a total of 560 commits in 5.2 that fixed issues that existed in Blender 5.1 or older."

I report it because it is the only place in the whole source set where progress reporting is tied to a way of starting a render. It is a bug report's title, phrased as a question, and it names Cycles sample progress, Python, and the `-f` argument together. What exactly was wrong and what the fix does is not stated. Reading anything more into it than "this was an issue and 5.2 fixed it" would be invention.

Two other entries in the same list mention progress:

> " \* Render Sequence Animation stopped showing total progress"

> " \* VSE \"Render Audio\" is missing a progress indicator"

Same source and date. And the sequencer release notes record:

> "The \"Render Audio\" operator now shows a realtime progress bar that can be cancelled (previously Blender would freeze until the operation finished). (blender/blender!155623)"

Source: release notes, https://developer.blender.org/docs/release_notes/5.2/sequencer/. Date read 2026-07-30.

### What is not in the sources

There is no page, and no part of any page, that sets out what progress a render reports when started from the interface versus the command line versus Python. The interface case is described; the other cases are not described at all.

**Where I looked.** The Render menu section of the topbar page; the viewport render page; the command-line index, arguments, render, and all three launch pages; the status bar page and its three pictures; `bpy.ops.render`, `bpy.types.RenderEngine`, `bpy.types.WindowManager`, `bpy.app`, `bpy.app.handlers`; the rendering, core, cycles, sequencer, user_interface, python_api and bugfixes release notes for 5.2. Search terms across the manual and API documentation: `background`, `headless`, `batch render`, `render farm`, `command line`, `progress`, `progress bar`, `Running Task`, `percentage`, `update_progress`, `update_stats`, `progress_begin`, `render status`, `status text`.

### Observed — what Blender 5.2 reports under each way of starting (2026-08-11)

Run in Blender 5.2.0 on **the Creator's Mac (a MacBook Pro with an Apple M1 Pro chip, 16 GB of memory, macOS 26.5.2, Apple's own graphics and no second card)** and on **a rented Linux machine (Ubuntu 22.04.5 LTS, two NVIDIA RTX 4000 Ada Generation cards of 20 GB each, eleven cores, 49 GB of memory)**, each with a settings folder of its own. The same frame from the same file — Cycles on the processor, 16 samples, adaptive sampling off, 160 by 120 — was started five ways and the terminal captured each time. **LABEL: observed.**

**The way of starting changes what is printed, and a render started from a script prints nothing.** Counting the lines that carry a sample count:

| how the render was started | sample lines printed |
|---|---|
| `blender -b <file>.blend -o <out> -f 1` | 3 |
| `blender -b <file>.blend -o <out> -a` (three frames) | 3 per frame |
| `blender -b <file>.blend -o <out> --python <script>` calling `bpy.ops.render.render(write_still=True)` | **0** |
| a window open, the same call from a script | 0 |
| a window open, started the way the F12 key starts it — `bpy.ops.render.render('INVOKE_DEFAULT', write_still=True)` | 0 |

It was run twice, once with the call in a `--python-expr` and once in a script file, with the same result both times, on both machines. **Blender 4.5 differs here**: a render started from Python with no window prints its full progress in 4.5 and prints nothing in 5.0, 5.1 and 5.2.

**The `render_stats` handler fires under every route in this version, and that is not true of the three older ones.** A script attached a handler and wrote every call down. With **no window** it fired **forty-five times** for one frame. With **a window open** it fired **forty-four times**, with the same strings:

```
STATS ('Mem: 0M | Synchronizing object | Area', None)
STATS ('Remaining: 00:00.11 | Mem: 2M | Sample 1/16', None)
STATS ('Mem: 2M | Finished', None)
```

In Blender 4.5, 5.0 and 5.1 the same script with a window open produced **one** string, carrying only `Time: … (Saving: …)`. So in this version a script can follow a render's sample progress with the window up, and in the three older versions it cannot. That is the sharpest difference this round found under this question.

**One thing happened on Linux and not on the Mac.** The windowed run with the handler attached ended in a **segmentation fault**, exit 139, after the handler had received all forty-four strings and after `render_complete` had fired. It wrote `/tmp/base_5.2.crash.txt`, and the shell reported `Segmentation fault`. It was repeated twice more and did the same both times. A windowed 5.2 render on that machine with **no** handler attached exited 0, and 4.5, 5.0 and 5.1 with the handler exited 0. On the Creator's Mac all four versions exited 0.

**A second pair of hands ran the same test on 2026-08-12 and did not see any of that. Both runs stand; neither is chosen.** The paragraph above is left exactly as its author wrote it. What follows is a separate run by somebody who was not in the room for it. **LABEL: observed.**

The repeat was asked for because the paragraph above does not say how its windowed Blender was made to end, and the exit status of a windowed run belongs to whoever ended it. Run on **the Creator's Mac (a MacBook Pro with an Apple M1 Pro chip, 16 GB of memory, macOS 26.5.2, Apple's own graphics and no second card)** and on **a rented Linux machine (Ubuntu 22.04.5 LTS on kernel 6.8, two NVIDIA RTX 4000 Ada Generation cards of 20 GB each on driver 580.105.08, an AMD Ryzen Threadripper PRO 3945WX with eleven cores, 49 GB of memory), whose window was opened on its own Xorg display**. Each version had a settings folder of its own, made before Blender was pointed at it, and its own copy of the scene: a cube, a sphere, an area light and a camera; Cycles on the processor; 16 samples; adaptive sampling off; 160 by 120; output PNG. `/tmp` was emptied of `*.crash.txt` before every run. A start-up script attached a handler to `bpy.app.handlers.render_stats` and another to `bpy.app.handlers.render_complete`, called `bpy.ops.render.render(write_still=True)`, printed the counts, and then closed Blender.

**How the run was closed — the step the paragraph above leaves out.** Two ways were tried. `bpy.ops.wm.quit_blender()` called straight from the start-up script did **not** close a windowed Blender on the rented Linux machine: the script printed `STILL HERE AFTER quit_blender()` and Blender was still going when a 120-second cap ended it, and again when a 240-second cap ended it — both times the status was 124, which belongs to `timeout` and not to Blender, and no crash file appeared either time. On the Creator's Mac the same call did close the run by itself, exiting 0, 24.7 seconds after the render in one run and 6.3 seconds after it in another. What closed the run on both machines every time was a timer: `bpy.app.timers.register` calling `bpy.ops.wm.quit_blender()` three seconds after the render.

**Closed by the timer, Blender 5.2 exited 0 on the rented Linux machine, three runs out of three.** No crash file appeared in `/tmp` — not `base_5.2.crash.txt`, not any other — the shell said nothing about a segmentation fault, and standard error was empty. The whole of standard output from the first of the three, long paths shortened:

```
00:00.903  blend            | Read blend: "…/scenes/base_5.2.blend"
BACKGROUND=False
VERSION=5.2.0 LTS
00:01.302  render           | Saved: '…/out/e21_5.2_timer_.png'
HANDLER FIRED 45 TIMES
FIRST=('Mem: 0M | Synchronizing object | Cube', None)
LAST=('Time: 00:00.39 (Saving: 00:00.13)', None)
RENDER_COMPLETE FIRED 1 TIMES
QUITWAY=timer
TIMER REGISTERED FOR 3s
TIMER FIRED, CALLING quit_blender()
STILL HERE AFTER quit_blender() IN TIMER
00:04.314  reports          | ERROR Cannot open file /tmp/quit.blend@ for writing: Permission denied

Blender quit
```

The lines with no timestamp are the test script's own printing. The other two runs printed `HANDLER FIRED 45 TIMES` as well and ended the same way.

**On the Creator's Mac, Blender 5.2 also exited 0, three runs out of three**, with no crash file and an empty standard error. The handler fired **46** times in the first run and **45** in each of the other two. Where the Linux machine printed the permission error at quit time, the Mac printed `00:04.222  blend            | Saved session recovery to "/var/folders/n3/z3rfjt654rn1k1x6f_1gk4540000gn/T/quit.blend"`.

**So the two runs disagree on three things, and both are recorded.** The earlier run: exit 139, a crash file, forty-four handler firings, on Linux only. This run: exit 0 on both machines, no crash file anywhere, forty-five handler firings on Linux and forty-six then forty-five then forty-five on the Mac. Blender 4.5, 5.0 and 5.1 were run the same way on the same day and each exited 0 on both machines with the handler firing once — which agrees with what those versions' own documents say.

**What the operator hands back.** Called plainly it returns `{'FINISHED'}` and blocks until the render is finished; started the way the F12 key starts it, it returns `{'RUNNING_MODAL'}` at once and the render goes on afterwards — a script that waited and then listed the folder found the picture written. Both machines.

**Whether the four Render-menu commands differ.** Read from the running program, the menu draws `Render Image`, `Render Animation`, `Render Sequencer Image` and `Render Sequencer Animation`, all four calling the same operator with different arguments. A **sequencer render drives the status bar's progress bar and is labelled `Rendering sequence...`** — not `Sequence Render`, which is the label in the manual's one picture — caught mid-render at `0%` and `16%` on both machines. An ordinary render is labelled `Rendering...`. With no window a sequencer render prints `Rendering frame 1` and the ordinary three sample lines.

**Whether the status bar appears for a Python-started render.** Yes. Every windowed run above was started by the operator call the F12 key is bound to, and the bar appeared each time: one bar, a camera icon, the label, a whole-number percentage and an X.

**What the ways of starting actually are.** The Python render operator takes **eight** arguments — `animation`, `write_still`, `use_viewport`, `use_sequencer_scene`, `layer`, `scene`, `frame_start`, `frame_end` — and every one carries a description, contrary to the API page's "Undocumented, consider contributing". The 3D viewport's View menu draws `Render Viewport Preview`, `Render Playblast` and `Render Playblast on Keyframes`. **`save_output` exists in this version and in none of the other three**, on the render settings, with the value `True`. Identical on both machines.

**`update_stats` and `update_progress` do something with no window: they return.** A tiny render engine registered from a script called `update_stats`, `update_progress` and `update_memory_stats`. All three returned without complaint, with a window and without, on both machines, and **none of them produced any output of its own** — the only lines in the capture were the plain ones printed beside each call. So they are not window-bound, and they are not a printing channel either.

**Not covered.** This does not settle what the time estimate says when the pointer rests on the progress bar — that needs a hover, and neither machine can move the pointer without a hand on it. It does not settle which key is bound to which render command: with no window Blender's key configuration holds no items. It does not settle what an interactive viewport render or the Rendered shading mode reports, nor what a render started from Blender loaded as a Python module reports, neither machine having such a build. It does not settle which `--log` category carries render progress, though `render` and `cycles` are both in the list this version prints. It does not settle anything about Windows.

**A gap in the record of looking.** Cycles' own supplied files are not among the places this answer records searching, unlike the other four answers here. So the *not in the sources* finding above rests on the manual, the API documentation and the release notes only, and this question has been sent back for a fresh pass over Cycles' files.

**Outside the interface.** Settled, and it is the heart of this answer. With no window there is no status bar — the step marked inferred above is borne out, since there was no bar to photograph — and what stands in for it is a printed stream: three sample lines carrying a frame number, a `Remaining:` estimate and a memory figure, for a render started with `-f` or `-a`. A render started **from a script** prints nothing but the file it saved, with a window or without. `update_stats` and `update_progress` are **not** window-bound: both returned without complaint with no window, and neither produced output of its own. The `render_stats` handler fires in both cases in this version, forty-five times with no window and forty-four with one. Both machines agreed, except that the windowed run on the Linux machine ended in a segmentation fault, recorded above. **Added 2026-08-12, by a second pair of hands:** a windowed run started from a terminal, with the handler attached, ended cleanly on **both** machines — exit 0, no crash file, three runs each — when it was closed by a timer calling `bpy.ops.wm.quit_blender()`, and the handler fired forty-five times on Linux and forty-six then forty-five then forty-five on the Mac. How such a run is closed is itself part of the answer: the same call made straight from the start-up script did not close a windowed Blender on the Linux machine at all, and took between six and twenty-five seconds to close it on the Mac.

**Operating system.** Settled for macOS and for Linux, and they agreed on every point above: the same counts of printed sample lines for all five ways, the same handler counts, the same operator arguments and menu entries, the same `Rendering...` and `Rendering sequence...` labels and whole-number percentages on each machine's own screen. **One difference between the machines**: the windowed run with a `render_stats` handler attached ended in a segmentation fault on the Linux machine, three times out of three, and exited 0 on the Mac. **That difference was not found again on 2026-08-12**: a second pair of hands ran the same test three times on each machine and got exit 0 and no crash file every time, on both. Both runs stand, side by side, and neither is chosen. **A different machine difference did show up in that later run**: `bpy.ops.wm.quit_blender()` called from a start-up script closed a windowed Blender on the Mac, after six seconds in one run and twenty-five in another, and did not close it on the Linux machine within four minutes; and at quit time the Mac printed `Saved session recovery to "/var/folders/…/T/quit.blend"` where the Linux machine printed `ERROR Cannot open file /tmp/quit.blend@ for writing: Permission denied`. **Windows is untested**, no Windows machine having run it, so the EEVEE line about headless Windows systems stands unchecked.

---

## 10. What does a render do when the machine's own memory runs out while it is sampling?

**LABEL: not in the sources.**

Nothing in the manual, the API documentation, the 5.2 release notes, or Cycles' supplied files describes what happens at the moment the machine's own memory is exhausted during sampling. There is no error name, no message, no fallback, no recovery, and no statement of whether the render stops, the frame is abandoned, or the process ends.

### The nearest things the sources do say

**Running out of memory is named as a cause of Blender ending abnormally.** Documented:

> "The most common causes of Blender crashes:
>
> Running out of memory.
>
> Issues with graphics hardware or drivers.
>
> Bugs in Blender."

and

> "When Blender crashes, it writes out a text file which contains information that may help identify the cause of the
> crash. Usually, this file is written in the Temporary Directory directory."

Source: manual, https://docs.blender.org/manual/en/5.2/troubleshooting/crash.html. Date read 2026-07-30.

This page is about Blender as a whole, not about a render, and not about the sampling stage specifically. It does list ways to reduce memory use (fewer undo steps, no multisample anti-aliasing, lighter window manager on Linux) and how to watch memory use per operating system (Task Manager on Windows, Activity Monitor or `top -o MEM` on macOS, `top -o %MEM` on Linux).

**A render can be told to use less memory, with running out named as the reason.** Documented:

> "Tile size has an impact on performance, and the default 2048 pixels provides
> a good balance for most scenes. When running out of memory, lowering to 1024
> or 512 pixels helps reduce memory usage, typically at a small performance cost."

and, on the same page:

> "Render high resolution images in tiles of this size, to reduce memory usage
> of render passes. Tiles are cached to disk while rendering to save memory."

Source: manual, https://docs.blender.org/manual/en/5.2/render/cycles/render_settings/performance.html. Date read 2026-07-30. This is advice for avoiding the situation, not a description of what happens when it arrives.

**There is a documented fallback, but it is for the graphics card's memory, not the machine's.** Documented:

> "With CUDA, OptiX, HIP and Metal devices, if the GPU memory is full Blender will automatically
> try to use system memory. This has a performance impact, but will usually still result in a faster render
> than using CPU rendering."

and, under Error Messages:

> "Error: Out of memory
>
> This usually means there is not enough memory to store the scene for use by the GPU."

Source: manual, https://docs.blender.org/manual/en/5.2/render/cycles/gpu_rendering.html. Date read 2026-07-30.

I flag this because it is the only "out of memory" message named anywhere in the sources, and it is explicitly about the graphics card's memory and about *storing the scene* — which happens before sampling, not during it. It is not an answer to this question, and treating it as one would be exactly the over-read the question invites.

**A note on ambiguity.** "The machine's own memory" I have read as the computer's main memory (RAM), as distinct from a graphics card's memory. That reading is what makes this question different from question 6 on the sheet. The other reading — a graphics card's memory being the memory of the machine — would be answered by the GPU quotations immediately above, and even under that reading the sources describe scene storage rather than sampling.

**Where I looked.** Search terms across the whole of `manual_5.2`, `api_5.2` and `release_notes_5.2`: `out of memory`, `Out of Memory`, `OOM`, `runs out`, `run out of`, `insufficient memory`, `system memory`, `RAM`, `swap`, `paging`, `crash`, `allocation`, `allocate`, `malloc`, `memory limit`, `exit code`, `return code`, `abort`. I read in full: the Crashes page, the Cycles GPU rendering page, the Cycles Performance page, the Memory & Limits section of the System preferences page (it covers undo steps, undo memory limit, and the Video Sequencer's memory cache limit — nothing about renders), and the status bar's System Memory item ("Shows an estimate of Blender's RAM consumption"). In `release_notes_5.2` I read every `memory` hit; all concern reducing memory use (texture cache, EEVEE video memory, geometry memory sharing, USD export flushing) or fixing leaks, none describe exhaustion behaviour. In Cycles' files I searched every supplied `.py`, `.h`, `.cpp`, `.cu` and `.metal` file for `memory`; the matches are all setting descriptions and none handle exhaustion. Cycles' session, device and memory-management code is not among the files supplied.

### Observed — Blender 5.2 refuses the allocation, says so, and aborts (2026-08-11)

Run on **a rented Linux machine only — Ubuntu 22.04.5 LTS on kernel 6.8, two NVIDIA RTX 4000 Ada Generation cards of 20 GB each on driver 580.105.08, an AMD Ryzen Threadripper PRO 3945WX with eleven cores, 49 GB of memory**. It was **not** run on the Creator's Mac: that is their working computer and its memory is not to be filled on purpose. So this answer covers Linux and leaves macOS untested. **LABEL: observed.**

**The scene.** A sphere, a cube, an area light and a camera; Cycles on the processor; 2000 samples; adaptive sampling off; 8192 by 8192 pixels; tile size 8192; every render pass switched on, cryptomatte at sixteen levels, and forty light groups added. Rendered with `blender -b <file>.blend -o <out> -f 1`, with the machine's memory read once a second alongside. The reading of "the machine's own memory" taken here is the one the answer above sets out: the computer's main memory, not a graphics card's.

**Getting the memory to run out while sampling took three attempts, and the first two are results of their own.** More pixels do not do it: at 12288 by 12288 the run reached 49.8 GB of the machine's 49 and **kept sampling**, and at 16384 by 16384 Blender's own figure was the same again. Tiling caps what is held — the tile size cannot exceed 8192 — so the buffers stop growing with the picture. What did it was more passes.

**What happened.** The last line on standard output is a sampling line:

```
00:00.606  render           | Fra: 1 | Mem: 1M | Sample 0/2000
```

**Standard error carries Blender's own words**, and they name the size of the request:

```
Malloc returns null: len=81335943168 in Cycles Aligned Alloc, total 12027448
terminate called after throwing an instance of 'std::bad_alloc'
  what():  std::bad_alloc
```

81,335,943,168 bytes is about 81 GB, on a machine with 49. **No picture was written, no partial frame survived, and no crash file appeared.** The process ended with status **134**. **The machine's own memory use never went above about 2.9 GB of 49** — the request was refused whole rather than filled piece by piece. Nothing came through the ordinary reporting channel: no `reports | ERROR` line of any kind.

**An animation does not go on to the next frame.** A three-frame run of the same scene printed one `Rendering frame` line, aborted the same way on frame one, wrote nothing, and ended with status 134.

The whole test was run twice, the second time with the four versions taken in the opposite order, and Blender 5.2 behaved the same way both times. **Blender 4.5 and 5.0 behave differently on the same machine with the same scene**: they fill the machine to about 49 GB, print nothing at all, are killed by the operating system's out-of-memory killer, and end with status 137. Blender 5.1 behaves as 5.2 does. That is recorded in each version's own document.

**Not covered.** This does not settle what happens on macOS or on Windows. It does not settle whether the answer differs between Cycles and EEVEE, since only Cycles was pushed. It does not settle what a window-open run would show, since this was run with no window. It does not settle whether a scene whose demand grew more gradually would be killed rather than refused.

**Outside the interface.** Settled for a run with no window, which is what was tested: the allocation is refused, three lines are printed on standard error, nothing is written, and the process ends with status 134. What a run with a window would show was not tested.

**Operating system.** Settled for Linux only. macOS is untested, because the Creator's working machine must not have its memory filled on purpose, and **Windows is untested**, no Windows machine having run it. On Linux, where it was tested, **no crash file was written at all**, so the crash log the manual describes did not appear for this kind of ending.

---

## 11. What, if anything, does a render print that shows whether a frame took the number of samples it was asked for?

**LABEL: documented** for the one printed item that carries a sample number; **not in the sources** for a printed statement that compares samples taken against samples asked for.

### What is printed

One sentence in the whole source set describes the content of printed render statistics:

> "bpy.app.handlers.render_stats
>
> on printing render statistics. Accepts one argument: the render progress as a string containing current frame, current sample, render time and saving time."

Source: Python API documentation, https://docs.blender.org/api/5.2/bpy.app.handlers.html. Date read 2026-07-30. **Documented.**

So the printed statistics string contains the **current sample**. It contains no other sample figure by this description — the maximum asked for is not among the four named items. A reader who wanted to know whether a frame took the number of samples it was asked for would have to supply the asked-for number themselves, from the scene's settings, and compare. That supplying step is exactly what makes any stronger claim inference rather than documentation, so I stop here.

### Why the two numbers can legitimately differ

The sources document three separate mechanisms — a time limit, adaptive sampling, and a sample subset — that make a frame stop short of, or depart from, the Max Samples figure. Four quotations cover the three. This matters because it means "did it take the samples it was asked for" is a real question and not a formality.

> "Time Limit
>
> Stops rendering if the time exceeds the limit, even if the desired sample count
> hasn't been reached yet. Setting this value to zero disables the limit."

> "If the Noise Threshold checkbox is enabled, Cycles will use adaptive sampling,
> cutting short the sampling process in areas that have become less noisy than the
> specified threshold value"

> "Min Samples
>
> The minimum number of samples a pixel receives before adaptive sampling is applied.
> When set to 0 (default), Cycles automatically picks a value determined by the Noise Threshold."

and Sample Subset:

> "Only render a subset of the samples."
>
> "Length
>
> The number of samples in the subset. While this overrides (Max) Samples in terms of the
> samples that will get rendered, it's still important to set (Max) Samples to the total number
> of samples that will be rendered across all subsets"

Source, all four: manual, https://docs.blender.org/manual/en/5.2/render/cycles/render_settings/sampling.html. Date read 2026-07-30. **Documented.**

Cycles' own files carry the same settings with their kinds, defaults and ranges. **Observed:**

```
    samples: IntProperty(
        name="Samples",
        description="Number of samples to render for each pixel",
        min=1, max=(1 << 24),
        default=4096,
    )
```
File: `Blender Sources/cycles_5.2/properties.py`, lines 485–490.

```
    time_limit: FloatProperty(
        name="Time Limit",
        description="Limit the render time (excluding synchronization time). "
        "Zero disables the limit",
        min=0.0,
        default=0.0,
```
File: `Blender Sources/cycles_5.2/properties.py`, lines 520–526.

```
    use_adaptive_sampling: BoolProperty(
        name="Use Adaptive Sampling",
        description="Automatically reduce the number of samples per pixel based on estimated noise level",
        default=True,
    )
```
File: `Blender Sources/cycles_5.2/properties.py`, lines 572–576. Note the default: adaptive sampling is on unless turned off.

```
    sample_subset_length: IntProperty(
        name="Sample Subset Length",
        description="The number of samples to render in this subset",
        min=1, max=(1 << 24),
        default=2048,
    )
```
File: `Blender Sources/cycles_5.2/properties.py`, lines 513–518. Date read for all four: 2026-07-30.

### The one place the comparison is made — but it is a picture, not printing

Cycles offers a render pass that holds exactly the ratio the question is about:

> "Sample Count Cycles
>
> Number of samples calculated for each pixel, divided by the maximum number of samples.
> Used to analyze adaptive sampling."

Source: manual, https://docs.blender.org/manual/en/5.2/render/layers/passes.html. Date read 2026-07-30. **Documented.**

Cycles' own files show the setting behind it and where it sits:

```
    pass_debug_sample_count: BoolProperty(
        name="Debug Sample Count",
        description="Number of samples per pixel taken, divided by the maximum number of samples. To analyze adaptive sampling",
        default=False,
        update=update_render_passes,
    )
```
File: `Blender Sources/cycles_5.2/properties.py`, lines 1526–1531.

```
    if crl.pass_debug_sample_count:
        yield (n_("Debug Sample Count"), "X", 'VALUE')
```
File: `Blender Sources/cycles_5.2/engine.py`, lines 221–222.

```
        col = layout.column(heading="Debug", align=True)
        col.prop(cycles_view_layer, "pass_debug_sample_count", text="Sample Count")
```
File: `Blender Sources/cycles_5.2/ui.py`, lines 1092–1093. Date read for all three: 2026-07-30. **Observed.**

The pass is named "Debug Sample Count" in the file, is off by default, and sits under a "Debug" heading in the view layer's passes; the entry in `engine.py` carries `"X"` and `'VALUE'` beside the name. Reading `"X"` as meaning the pass holds a single channel is a step past what the file says, so that is not claimed here. By the manual's own description it holds, for every pixel, the samples taken divided by the maximum — the question's substance — but it is written into the picture, not printed. The question asks what a render *prints*, so I record this as an adjacent finding, not as the answer.

### What is not in the sources

No source states that a render prints the maximum sample count, prints a "samples taken out of samples requested" figure, prints a notice when a frame stops early because of the Time Limit, or prints anything when adaptive sampling ends a pixel's sampling. The image metadata that can be stamped into an output file includes a Render Time field ("Includes the render time.") and a Note field, but no sample field — source: manual, https://docs.blender.org/manual/en/5.2/render/output/properties/metadata.html, date read 2026-07-30.

**Where I looked.** Search terms across the whole of `manual_5.2`, `api_5.2` and `release_notes_5.2`: `sample count`, `current sample`, `samples rendered`, `sample progress`, `adaptive`, `noise threshold`, `time limit`, `Sample Subset`, `Saved:`, `Fra:`, `progress`, `render statistic`, `print`. I read the Cycles Sampling page and the Render Passes page in full, the Metadata page, `bpy.app.handlers` in full, `bpy.types.RenderEngine` in full, and searched all of Cycles' supplied files for sample- and progress-related strings. The only hit anywhere for "current sample" is the `render_stats` sentence quoted above.

### Observed — what Blender 5.2 prints and writes about samples (2026-08-11)

Run in Blender 5.2.0 on **the Creator's Mac (a MacBook Pro with an Apple M1 Pro chip, 16 GB of memory, macOS 26.5.2, Apple's own graphics and no second card)** and on **a rented Linux machine (Ubuntu 22.04.5 LTS, two NVIDIA RTX 4000 Ada Generation cards of 20 GB each, eleven cores, 49 GB of memory)**, each with a settings folder of its own. **LABEL: observed.**

**1. The layout of the printed statistics string, and how often it is emitted.** A script attached a handler to `bpy.app.handlers.render_stats` and wrote every call down. With no window it fired **forty-five times for one frame** — once per stage, not once per render — and was handed **two** arguments, a string and `None`. The first, a middle and the last, word for word:

```
STATS ('Mem: 0M | Synchronizing object | Cube', None)
STATS ('Remaining: 00:00.04 | Mem: 2M | Sample 1/16', None)
STATS ('Time: 00:00.22 (Saving: 00:00.06)', None)
```

The separator is ` | `. **No frame number appears in any of the forty-five strings**, on either machine, although this version's API page describes the string as "containing current frame, current sample, render time and saving time". The current sample appears only while a sampling stage is running, written as `Sample <n>/<max>` — so the maximum **is** in the string after all, beside the count. The render time and the saving time arrive together, in the last string only, as `Time: … (Saving: …)`. **With a window open the handler fired forty-four times with the same strings** — which the three older versions do not do; there it fires once.

**2. Whether the current-sample figure counts the last sample completed.** In a plain sixteen-sample frame the three sampling strings read `Sample 0/16`, `Sample 1/16` and `Sample 16/16`, so the count runs from nought to the maximum.

**3. What the count does when a frame stops short.** The scene — Cycles on the processor, 4096 samples, adaptive sampling off, 480 by 360 — was rendered plain, with a five-second time limit, and with adaptive sampling on at 0.1. **All three exited 0 and wrote their picture.** The whole of the sample lines for the time-limited run on the Creator's Mac:

```
00:00.496  render           | Fra: 1 | Mem: 1M | Sample 0/4096
00:00.507  render           | Fra: 1 | Remaining: 00:04.98 | Mem: 10M | Sample 1/4096
00:05.734  render           | Fra: 1 | Mem: 10M | Sample 513/4096
00:06.017  render           | Fra: 1 | Mem: 10M | Sample 0/4096
```

against the plain run, which ends `00:42.649  render           | Fra: 1 | Mem: 10M | Sample 4096/4096`. So the count reached is printed, and then the **last** line resets to `Sample 0/<maximum>`. Adaptive sampling does the same, climbing 48, 64, 80, 96, 112, 128, 144, 160, 176 and then ending on `Sample 0/4096`. **Nothing else is printed** — no notice, no warning, nothing naming the time limit or the noise threshold. On the rented Linux machine the same three runs behaved identically, reaching 769/4096 under the time limit and 176/4096 under adaptive sampling. **The two machines agreed on the shape and differed only in the counts reached, as two different processors must.**

**4. A saved image carries the maximum, not the count reached.** A frame rendered to PNG with the stamp settings untouched carries eleven metadata fields, read back by a plain Python script rather than by Blender: `File`, `Date`, `Time`, `Frame`, `Camera`, `Scene`, `RenderTime`, `cycles.ViewLayer.samples`, `cycles.ViewLayer.total_time`, `cycles.ViewLayer.render_time`, `cycles.ViewLayer.synchronization_time`. **`cycles.ViewLayer.samples` is not on the manual's Metadata panel list**, and it holds the maximum: a run whose sample lines showed 160 of 512 reached carried `cycles.ViewLayer.samples (string) = 512`. An OpenEXR file carries the same fields plus `Software` (`Blender 5.2.0 LTS`), `colorInteropID`, `name` and `type`. The same on both machines.

**5. The Sample Count pass is *not* written into a multi-layer OpenEXR in this version.** With `pass_debug_sample_count` set to `True` — confirmed set before each render — adaptive sampling on at 0.1 and a maximum of 512 samples, saved as a multi-layer OpenEXR, the file's channel list, read without Blender, is:

```
ViewLayer.Combined.A, ViewLayer.Combined.B, ViewLayer.Combined.G, ViewLayer.Combined.R
```

Only Combined. Nothing was printed about it and the run exited 0. **Blender 4.5 writes the pass into the same file**, as a single channel named `ViewLayer.Debug Sample Count.X`, with values from 0.300000 to 1.000000 and 172,791 of 172,800 pixels below one. So this differs between the versions, and the reading of `"X"` as one channel — which this answer declined to make — is borne out for 4.5. The same on both machines. One note on the set-up: in this version the output must be switched to multi-layer with **two** settings, `image_settings.media_type = 'MULTI_LAYER_IMAGE'` as well as `image_settings.file_format = 'OPEN_EXR_MULTILAYER'`; setting only the second writes a PNG, with no error and an exit status of 0. Blender 4.5 has no `media_type` setting at all.

**6. What a run prints when the engine is not Cycles.** EEVEE prints `Fra: 1 | Rendering <n> / 64 samples` at 1, 25, 50 and 64 of 64 — a running count against a maximum, in a different shape from Cycles' — and it used its own count of 64 where the scene asked for 16, printing nothing about the difference. **Workbench prints no sample line at all.** Both machines.

**7. Cycles' own statistics carry no sample count.** With `-- --cycles-print-stats` the two printed blocks — quoted in full under question 8 — hold memory and time only, on both machines.

**Not covered.** This does not settle whether the current-sample figure is per frame or per view layer, since every scene tested had one view layer. It does not settle what is printed when Sample Subset is in use. It does not settle anything about Windows.

**Outside the interface.** Settled, and it differs. With no window the sample count is printed, in three lines for a completed frame and in a run that ends on `Sample 0/<maximum>` for a frame that stopped short. With a window open **no sample line reaches the terminal at all** — but the `render_stats` handler still receives all forty-four strings, sample counts and all, which is this version's own behaviour and not the older versions'. The image metadata is written either way. The step marked inferred above, about the Sample Count pass reaching a file written by a background run, is **not** borne out for this version: the pass did not reach the saved file at all, with a window or without. Both machines agreed.

**Operating system.** Settled for macOS and for Linux, and they agreed on every point: the same printed pattern, the same eleven metadata fields, the same absence of the pass from the saved file, the same handler strings, and the same absence of a sample count from Cycles' statistics. **Windows is untested**, no Windows machine having run it.

Nothing in this answer carries an operating-system qualification in the sources either. One neighbouring pass does: the Render Time pass is CPU-only — "Reports time per pixel in milliseconds. Supported only on CPU render devices" (`Blender Sources/cycles_5.2/properties.py`, lines 1532–1535, observed; and the manual's "Note, this pass is not supported on GPU rendering", on the Render Passes page, https://docs.blender.org/manual/en/5.2/render/layers/passes.html, read 2026-07-30). That is a device restriction, not an operating-system one, and it does not apply to the Sample Count pass.

> **Filled in by the citation check (2026-08-11).** This sentence carried "[PENDING: which manual page carries that sentence, and its address — not recorded here]". The sentence is on the Render Passes page, directly under the Render Time pass, whose address is now given above.

---

## Things a person with Blender open could settle in a few minutes

The six items that stood here have all been run, on both machines, and their results are written into the answers above. Item 6 was run on the rented Linux machine only. What is left open is below.

1. **The time estimate that appears when the pointer rests on the progress bar (questions 7, 9 and 11).** It needs a hover. Neither machine has any way of moving the pointer without a hand on it — the rented Linux machine has no `xdotool` or equivalent installed — so this was dropped rather than run.

2. **What a render does when the machine's memory runs out, on macOS (question 10).** Answered on Linux only. The Creator's Mac is their working computer and its memory is not to be filled on purpose. The same item's window-open half was also not run.

3. **Everything on Windows (questions 7 to 11).** No Windows machine has run it, so every one of these answers covers macOS and Linux and leaves Windows open.

4. **Whether a 65536 by 65536 render would save into a format OpenImageIO's size limit allows, or with that limit raised (question 7).** What it does as it stands is recorded above: it renders all 1024 tiles and then fails to save, exiting 1.

5. **What an interactive viewport render or the Rendered shading mode reports (question 9).** Not tested.

6. **What a render started from Blender loaded as a Python module reports (question 9).** Neither machine has such a build.

7. **Which key is bound to which render command (question 9).** With no window Blender's key configuration holds no items, so it could not be read that way.

8. **What is printed when Sample Subset is in use, and whether the sample figure is per frame or per view layer (question 11).** Not tested; every scene had one view layer.

---

2026-08-11 — Claude Opus 5 (Claude Code) — Reports, Blender 5.2
